//! p1: how many observable-policy assignments are actually distinguishable?
//!
//! The claim under test is section 3's membership criterion: two strategies are
//! semantically distinct only if some (shape, operation) exhibits a disagreement.
//! If that is the criterion, then the ceiling on a meaningful strategy count is
//! the number of equivalence classes of assignments under "computes the same
//! answer function", and that number is computable exhaustively at a small width.
//!
//! Model. An unsigned fixed-point numeral of W total bits with F fraction bits.
//! Raw values are integers in [0, 2^W). The logical value is raw / 2^F.
//!
//! Axes swept:
//!   rounding      5 positions   (how a product's extra fraction bits resolve)
//!   overflow      3 positions   (what happens outside [0, 2^W))
//!   intermediate  2 positions   (round each step, or hold exact and round once)
//! so 30 assignments in the product.
//!
//! Operations: add, sub, mul (pairwise, exhaustive), and the chain a*b + c
//! (triples, exhaustive). The chain is what makes the intermediate axis live;
//! add is what should make the rounding axis dead.
//!
//! THE CASE THAT MUST FAIL. A partitioner that cannot separate two genuinely
//! different answer functions reports collapse everywhere and looks like a
//! finding. So the sweep carries two controls, and the probe aborts if either
//! misbehaves:
//!   (a) a duplicate assignment, identical to a real one under a different
//!       label, MUST land in the same class;
//!   (b) a corrupted assignment, identical to a real one except that it adds one
//!       to every result, MUST land in a class of its own.
//! (b) is the one that matters. If it collapses, every number below is noise.

use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Round {
    TowardZero,
    TiesEven,
    TiesAway,
    TowardNegInf,
    TowardPosInf,
}
const ROUNDS: [Round; 5] = [
    Round::TowardZero,
    Round::TiesEven,
    Round::TiesAway,
    Round::TowardNegInf,
    Round::TowardPosInf,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Overflow {
    Wrap,
    SaturateBoth,
    SaturateHighOnly,
}
const OVERFLOWS: [Overflow; 3] = [
    Overflow::Wrap,
    Overflow::SaturateBoth,
    Overflow::SaturateHighOnly,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Intermediate {
    RoundEachStep,
    ExactThenRoundOnce,
}
const INTERMEDIATES: [Intermediate; 2] =
    [Intermediate::RoundEachStep, Intermediate::ExactThenRoundOnce];

#[derive(Clone, Copy, Debug)]
struct Assignment {
    round: Round,
    overflow: Overflow,
    intermediate: Intermediate,
    /// Control marker. `0` is a real assignment, `1` is the duplicate control,
    /// `2` is the corrupted control that must not collapse into its twin.
    control: u8,
}

impl Assignment {
    fn label(&self) -> String {
        let tag = match self.control {
            0 => "",
            1 => " [dup-control]",
            2 => " [corrupt-control]",
            _ => " [?]",
        };
        format!(
            "{:?}/{:?}/{:?}{}",
            self.round, self.overflow, self.intermediate, tag
        )
    }
}

/// Round `v`, which carries `s` extra fractional bits, down to zero extra bits.
/// `v` may be negative; the modes are defined on the real line, not on
/// magnitudes, so TowardZero and TowardNegInf part company below zero.
fn shift_round(v: i128, s: u32, mode: Round) -> i128 {
    if s == 0 {
        return v;
    }
    let d: i128 = 1i128 << s;
    let q = v.div_euclid(d); // floor
    let r = v.rem_euclid(d); // 0 <= r < d
    match mode {
        Round::TowardNegInf => q,
        Round::TowardPosInf => {
            if r == 0 {
                q
            } else {
                q + 1
            }
        }
        Round::TowardZero => {
            if v >= 0 {
                q
            } else if r == 0 {
                q
            } else {
                q + 1
            }
        }
        Round::TiesAway => {
            let twice = 2 * r;
            if twice > d {
                q + 1
            } else if twice < d {
                q
            } else if v >= 0 {
                q + 1
            } else {
                q
            }
        }
        Round::TiesEven => {
            let twice = 2 * r;
            if twice > d {
                q + 1
            } else if twice < d {
                q
            } else if q % 2 == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

fn apply_overflow(r: i128, w: u32, mode: Overflow) -> i128 {
    let modulus: i128 = 1i128 << w;
    let max = modulus - 1;
    match mode {
        Overflow::Wrap => r.rem_euclid(modulus),
        Overflow::SaturateBoth => {
            if r < 0 {
                0
            } else if r > max {
                max
            } else {
                r
            }
        }
        Overflow::SaturateHighOnly => {
            if r > max {
                max
            } else {
                r.rem_euclid(modulus)
            }
        }
    }
}

fn eval_add(a: i128, b: i128, w: u32, _f: u32, asg: Assignment) -> i128 {
    let r = a + b;
    let r = apply_overflow(r, w, asg.overflow);
    corrupt(r, w, asg)
}

fn eval_sub(a: i128, b: i128, w: u32, _f: u32, asg: Assignment) -> i128 {
    let r = a - b;
    let r = apply_overflow(r, w, asg.overflow);
    corrupt(r, w, asg)
}

fn eval_mul(a: i128, b: i128, w: u32, f: u32, asg: Assignment) -> i128 {
    // product carries 2F fraction bits; bring it back to F.
    let p = a * b;
    let r = shift_round(p, f, asg.round);
    let r = apply_overflow(r, w, asg.overflow);
    corrupt(r, w, asg)
}

/// a*b + c. This is where the intermediate axis is observable: either the
/// product is brought to F fraction bits before the add, or the addend is
/// lifted to 2F and the single rounding happens at the end.
fn eval_chain(a: i128, b: i128, c: i128, w: u32, f: u32, asg: Assignment) -> i128 {
    let r = match asg.intermediate {
        Intermediate::RoundEachStep => {
            let p = shift_round(a * b, f, asg.round);
            // the intermediate is itself subject to the overflow policy, which
            // is what makes "round each step" a different function rather than
            // merely a different order of the same arithmetic.
            let p = apply_overflow(p, w, asg.overflow);
            p + c
        }
        Intermediate::ExactThenRoundOnce => {
            let wide = a * b + (c << f);
            shift_round(wide, f, asg.round)
        }
    };
    let r = apply_overflow(r, w, asg.overflow);
    corrupt(r, w, asg)
}

/// a*b - c. Same two intermediate policies, but the addend is subtractive, so a
/// product that saturated at the top can be pulled back down. That is the case
/// where "saturate the intermediate" and "saturate once at the end" part company,
/// and it is absent from `eval_chain`. Its presence or absence in the witness set
/// is what makes the class count a fact about the witness set.
fn eval_chain_sub(a: i128, b: i128, c: i128, w: u32, f: u32, asg: Assignment) -> i128 {
    let r = match asg.intermediate {
        Intermediate::RoundEachStep => {
            let p = shift_round(a * b, f, asg.round);
            let p = apply_overflow(p, w, asg.overflow);
            p - c
        }
        Intermediate::ExactThenRoundOnce => {
            let wide = a * b - (c << f);
            shift_round(wide, f, asg.round)
        }
    };
    let r = apply_overflow(r, w, asg.overflow);
    corrupt(r, w, asg)
}

/// The corrupted control's whole content: a real assignment plus one. It exists
/// so the partitioner has something it MUST separate.
fn corrupt(r: i128, w: u32, asg: Assignment) -> i128 {
    if asg.control == 2 {
        (r + 1).rem_euclid(1i128 << w)
    } else {
        r
    }
}

fn assignments() -> Vec<Assignment> {
    let mut v = Vec::new();
    for &round in ROUNDS.iter() {
        for &overflow in OVERFLOWS.iter() {
            for &intermediate in INTERMEDIATES.iter() {
                v.push(Assignment {
                    round,
                    overflow,
                    intermediate,
                    control: 0,
                });
            }
        }
    }
    // the two controls, both twins of the first real assignment
    let base = v[0];
    v.push(Assignment { control: 1, ..base });
    v.push(Assignment { control: 2, ..base });
    v
}

/// Partition assignments by equality of their full answer vector over the whole
/// input domain. Returns classes as lists of indices into `asgs`.
fn partition(asgs: &[Assignment], vectors: &[Vec<i128>]) -> Vec<Vec<usize>> {
    let mut classes: Vec<Vec<usize>> = Vec::new();
    for i in 0..asgs.len() {
        let mut placed = false;
        for cls in classes.iter_mut() {
            if vectors[cls[0]] == vectors[i] {
                cls.push(i);
                placed = true;
                break;
            }
        }
        if !placed {
            classes.push(vec![i]);
        }
    }
    classes
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Chain,
    ChainSub,
}

impl Op {
    fn name(&self) -> &'static str {
        match self {
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
            Op::Chain => "chain a*b+c",
            Op::ChainSub => "chain a*b-c",
        }
    }
}

fn answer_vector(op: Op, w: u32, f: u32, asg: Assignment) -> Vec<i128> {
    let n: i128 = 1i128 << w;
    let mut out = Vec::new();
    match op {
        Op::Chain => {
            for a in 0..n {
                for b in 0..n {
                    for c in 0..n {
                        out.push(eval_chain(a, b, c, w, f, asg));
                    }
                }
            }
        }
        Op::ChainSub => {
            for a in 0..n {
                for b in 0..n {
                    for c in 0..n {
                        out.push(eval_chain_sub(a, b, c, w, f, asg));
                    }
                }
            }
        }
        _ => {
            for a in 0..n {
                for b in 0..n {
                    let v = match op {
                        Op::Add => eval_add(a, b, w, f, asg),
                        Op::Sub => eval_sub(a, b, w, f, asg),
                        Op::Mul => eval_mul(a, b, w, f, asg),
                        Op::Chain | Op::ChainSub => unreachable!(),
                    };
                    out.push(v);
                }
            }
        }
    }
    out
}

fn main() {
    let asgs = assignments();
    let real_count = asgs.iter().filter(|a| a.control == 0).count();
    let dup_idx = asgs.iter().position(|a| a.control == 1).unwrap();
    let corrupt_idx = asgs.iter().position(|a| a.control == 2).unwrap();

    println!("p1: distinguishable assignment classes");
    println!(
        "axes: rounding {} x overflow {} x intermediate {} = {} real assignments",
        ROUNDS.len(),
        OVERFLOWS.len(),
        INTERMEDIATES.len(),
        real_count
    );
    println!("plus 2 controls (a duplicate, and a corrupted twin)\n");

    let mut control_failures = 0usize;
    let mut summary: BTreeMap<(u32, u32, &'static str), usize> = BTreeMap::new();

    // pairwise ops at three widths, chain at two (the chain domain is 2^(3W))
    let pair_shapes: Vec<(u32, u32)> = vec![(6, 0), (6, 1), (6, 2), (6, 3), (8, 0), (8, 2), (4, 1)];
    let chain_shapes: Vec<(u32, u32)> = vec![(4, 0), (4, 1), (4, 2), (6, 0), (6, 2)];

    for &(w, f) in pair_shapes.iter() {
        for op in [Op::Add, Op::Sub, Op::Mul] {
            let vectors: Vec<Vec<i128>> = asgs
                .iter()
                .map(|&a| answer_vector(op, w, f, a))
                .collect();
            let classes = partition(&asgs, &vectors);
            let real_classes = count_real_classes(&classes, &asgs);
            summary.insert((w, f, op.name()), real_classes);
            report(w, f, op.name(), real_count, real_classes, &classes, &asgs);
            control_failures += check_controls(&classes, dup_idx, corrupt_idx, w, f, op.name());
        }
    }

    for &(w, f) in chain_shapes.iter() {
      for op in [Op::Chain, Op::ChainSub] {
        let vectors: Vec<Vec<i128>> = asgs
            .iter()
            .map(|&a| answer_vector(op, w, f, a))
            .collect();
        let classes = partition(&asgs, &vectors);
        let real_classes = count_real_classes(&classes, &asgs);
        summary.insert((w, f, op.name()), real_classes);
        report(w, f, op.name(), real_count, real_classes, &classes, &asgs);
        control_failures += check_controls(&classes, dup_idx, corrupt_idx, w, f, op.name());
      }
    }


    // ---- the joint partition: the actual ceiling ----
    //
    // Each single (shape, operation) reveals only part of the axis structure.
    // The number that answers "how many strategies could a design meaningfully
    // name" is the partition under the WHOLE witness set: concatenate every
    // answer vector over every shape and operation, and count classes.
    println!("\n=== joint partition over the whole witness set ===");
    let mut joint: Vec<Vec<i128>> = vec![Vec::new(); asgs.len()];
    for &(w, f) in pair_shapes.iter() {
        for op in [Op::Add, Op::Sub, Op::Mul] {
            for (i, &a) in asgs.iter().enumerate() {
                joint[i].extend(answer_vector(op, w, f, a));
            }
        }
    }
    for &(w, f) in chain_shapes.iter() {
        for (i, &a) in asgs.iter().enumerate() {
            joint[i].extend(answer_vector(Op::Chain, w, f, a));
        }
    }
    let jclasses = partition(&asgs, &joint);
    let jreal = count_real_classes(&jclasses, &asgs);
    let narrow_joint = jreal;
    println!(
        "{} real assignments -> {} distinguishable classes under all shapes and ops together",
        real_count, jreal
    );
    for cls in jclasses.iter() {
        let reals: Vec<String> = cls
            .iter()
            .filter(|&&i| asgs[i].control == 0)
            .map(|&i| asgs[i].label())
            .collect();
        if !reals.is_empty() {
            println!("      {{ {} }}", reals.join(" | "));
        }
    }
    control_failures += check_controls(&jclasses, dup_idx, corrupt_idx, 0, 0, "JOINT");

    // now widen the witness set by one operation and recount.
    println!("\n=== the same joint partition, witness set widened by ONE operation ===");
    let mut joint2 = joint.clone();
    for &(w, f) in chain_shapes.iter() {
        for (i, &a) in asgs.iter().enumerate() {
            joint2[i].extend(answer_vector(Op::ChainSub, w, f, a));
        }
    }
    let j2classes = partition(&asgs, &joint2);
    let j2real = count_real_classes(&j2classes, &asgs);
    println!(
        "adding the subtractive chain a*b-c: {narrow_joint} classes -> {j2real} classes"
    );
    for cls in j2classes.iter() {
        let reals: Vec<String> = cls
            .iter()
            .filter(|&&i| asgs[i].control == 0)
            .map(|&i| asgs[i].label())
            .collect();
        if !reals.is_empty() {
            println!("      {{ {} }}", reals.join(" | "));
        }
    }
    println!(
        "prediction P6 (widening the witness set raises the count): {}",
        if j2real > narrow_joint { "CONFIRMED" } else { "REFUTED" }
    );
    control_failures += check_controls(&j2classes, dup_idx, corrupt_idx, 0, 0, "JOINT-WIDE");

    // The best single witness, against the WIDE joint. Comparing it against the
    // narrow joint would be comparing across two different witness sets, which
    // is exactly the error this probe exists to make visible.
    let mut best = 0usize;
    let mut best_name = String::new();
    for (&(w, f, op), &n) in summary.iter() {
        if n > best {
            best = n;
            best_name = format!("W={w} F={f} {op}");
        }
    }
    println!(
        "\nbest single (shape, operation) witness: {best_name} at {best} classes."
    );
    println!(
        "wide joint is {j2real}. does one witness reveal the whole structure here? {}",
        if best >= j2real { "YES, this one does" } else { "no" }
    );
    println!(
        "redundant assignments under the wide witness set: {} of {}",
        real_count - j2real,
        real_count
    );

    // ---- the comparable witness set ----
    //
    // p2 re-derives this same count from an independent model (exact rationals,
    // no shifts, range policy applied to the logical value). For the two to be
    // comparable they must range over an identical witness set, so it is pinned
    // here rather than left as "the shapes each happened to sweep".
    println!("\n=== comparable witness set (shared with p2) ===");
    let cmp_shapes: Vec<(u32, u32)> = vec![(4, 0), (4, 1), (4, 2)];
    let mut cjoint: Vec<Vec<i128>> = vec![Vec::new(); asgs.len()];
    for &(w, f) in cmp_shapes.iter() {
        for op in [Op::Add, Op::Sub, Op::Mul, Op::Chain, Op::ChainSub] {
            for (i, &a) in asgs.iter().enumerate() {
                cjoint[i].extend(answer_vector(op, w, f, a));
            }
        }
    }
    let cclasses = partition(&asgs, &cjoint);
    let creal = count_real_classes(&cclasses, &asgs);
    println!("W in {{4}}, F in {{0,1,2}}, ops {{add,sub,mul,a*b+c,a*b-c}}");
    println!("COMPARABLE_JOINT_CLASSES={creal}");
    control_failures += check_controls(&cclasses, dup_idx, corrupt_idx, 0, 0, "COMPARABLE");

    println!("\n=== controls ===");
    if control_failures == 0 {
        println!("both controls behaved at every shape: the duplicate always collapsed into");
        println!("its twin, and the corrupted twin never did. the partitioner separates.");
    } else {
        println!("CONTROL FAILURES: {control_failures}. every number above is void.");
        std::process::exit(1);
    }
}

fn count_real_classes(classes: &[Vec<usize>], asgs: &[Assignment]) -> usize {
    classes
        .iter()
        .filter(|cls| cls.iter().any(|&i| asgs[i].control == 0))
        .count()
}

fn report(
    w: u32,
    f: u32,
    op: &str,
    real_count: usize,
    real_classes: usize,
    classes: &[Vec<usize>],
    asgs: &[Assignment],
) {
    println!(
        "W={w:2} F={f} {op:<12}  {real_count} assignments -> {real_classes} distinguishable classes"
    );
    if real_classes <= 8 {
        for cls in classes.iter() {
            let reals: Vec<String> = cls
                .iter()
                .filter(|&&i| asgs[i].control == 0)
                .map(|&i| asgs[i].label())
                .collect();
            if !reals.is_empty() {
                println!("      {{ {} }}", reals.join(" | "));
            }
        }
    }
}

fn check_controls(
    classes: &[Vec<usize>],
    dup: usize,
    corrupt: usize,
    w: u32,
    f: u32,
    op: &str,
) -> usize {
    let find = |x: usize| classes.iter().position(|c| c.contains(&x)).unwrap();
    let c_dup = find(dup);
    let c_cor = find(corrupt);
    let c_base = find(0);
    let mut bad = 0;
    if c_dup != c_base {
        println!("  !! CONTROL FAIL W={w} F={f} {op}: duplicate did not collapse into its twin");
        bad += 1;
    }
    if c_cor == c_base {
        println!("  !! CONTROL FAIL W={w} F={f} {op}: corrupted twin collapsed into its twin");
        bad += 1;
    }
    bad
}
