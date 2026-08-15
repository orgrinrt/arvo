//! p3: is the container choice observable in the answer function?
//!
//! Section 4 claims the storage-minimising concern is a weighting and not an
//! observable-policy assignment, i.e. that packing a column tighter changes what
//! it costs and not what it computes. That claim is testable rather than
//! rhetorical, and the test is the same partition p1 runs.
//!
//! Method. Extend the assignment tuple with a CONTAINER choice and re-run the
//! joint partition over p1's comparable witness set. If the container is an
//! observable policy, the class count rises: some pair of containers computes
//! different answers somewhere. If it is a cost coordinate, the count is
//! unchanged, because every container decodes to the same logical column.
//!
//! Containers modelled, all storing a W-bit unsigned value:
//!   Packed    exactly W bits, no padding
//!   Minimum   the smallest whole-byte rung that holds W bits
//!   Headroom  twice that rung
//!   Lossy     W-1 bits, THE NEGATIVE CONTROL
//!
//! THE CASE THAT MUST FAIL. Lossy is a container that genuinely loses
//! information, so it MUST be observable and MUST raise the class count. If the
//! sweep reports "the container is never observable" while Lossy is in it, the
//! sweep is not measuring observability and its result about the other three is
//! worthless. The probe aborts if Lossy fails to separate.
//!
//! Prediction, written before running: Packed, Minimum and Headroom contribute
//! zero additional classes; Lossy contributes some.

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Container {
    Packed,
    Minimum,
    Headroom,
    Lossy,
}
const CONTAINERS: [Container; 4] = [
    Container::Packed,
    Container::Minimum,
    Container::Headroom,
    Container::Lossy,
];

impl Container {
    /// Bits actually retained by a store-then-load round trip.
    fn retained_bits(&self, w: u32) -> u32 {
        match self {
            Container::Packed => w,
            Container::Minimum => rung_bits(w),
            Container::Headroom => 2 * rung_bits(w),
            Container::Lossy => w.saturating_sub(1),
        }
    }

    /// Store a W-bit value and load it back. The whole content of a container
    /// choice, as far as the answer function can see.
    fn round_trip(&self, v: i128, w: u32) -> i128 {
        let bits = self.retained_bits(w).min(126);
        if bits == 0 {
            return 0;
        }
        let mask: i128 = (1i128 << bits) - 1;
        v & mask
    }
}

fn rung_bits(w: u32) -> u32 {
    let mut r = 8u32;
    while r < w {
        r *= 2;
    }
    r
}

#[derive(Clone, Copy, Debug)]
struct Config {
    round: Round,
    overflow: Overflow,
    intermediate: Intermediate,
    container: Container,
}

impl Config {
    fn label(&self) -> String {
        format!(
            "{:?}/{:?}/{:?}/{:?}",
            self.round, self.overflow, self.intermediate, self.container
        )
    }
    fn assignment_label(&self) -> String {
        format!("{:?}/{:?}/{:?}", self.round, self.overflow, self.intermediate)
    }
}

fn shift_round(v: i128, s: u32, mode: Round) -> i128 {
    if s == 0 {
        return v;
    }
    let d: i128 = 1i128 << s;
    let q = v.div_euclid(d);
    let r = v.rem_euclid(d);
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
            if v >= 0 || r == 0 {
                q
            } else {
                q + 1
            }
        }
        Round::TiesAway => {
            let t = 2 * r;
            if t > d {
                q + 1
            } else if t < d {
                q
            } else if v >= 0 {
                q + 1
            } else {
                q
            }
        }
        Round::TiesEven => {
            let t = 2 * r;
            if t > d {
                q + 1
            } else if t < d {
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

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    ChainAdd,
    ChainSub,
}

impl Op {
    fn name(&self) -> &'static str {
        match self {
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
            Op::ChainAdd => "a*b+c",
            Op::ChainSub => "a*b-c",
        }
    }
    fn is_ternary(&self) -> bool {
        matches!(self, Op::ChainAdd | Op::ChainSub)
    }
}

/// Operands are loaded from the container, the arithmetic runs, and the result
/// is stored back through it. That is the whole of what a storage choice does.
fn eval(op: Op, a: i128, b: i128, c: i128, w: u32, f: u32, cfg: Config) -> i128 {
    let k = cfg.container;
    let (a, b, c) = (k.round_trip(a, w), k.round_trip(b, w), k.round_trip(c, w));
    let r = match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => shift_round(a * b, f, cfg.round),
        Op::ChainAdd | Op::ChainSub => {
            let sign: i128 = if matches!(op, Op::ChainAdd) { 1 } else { -1 };
            match cfg.intermediate {
                Intermediate::RoundEachStep => {
                    let p = shift_round(a * b, f, cfg.round);
                    let p = apply_overflow(p, w, cfg.overflow);
                    p + sign * c
                }
                Intermediate::ExactThenRoundOnce => {
                    shift_round(a * b + sign * (c << f), f, cfg.round)
                }
            }
        }
    };
    let r = apply_overflow(r, w, cfg.overflow);
    k.round_trip(r, w)
}

fn configs(containers: &[Container]) -> Vec<Config> {
    let mut v = Vec::new();
    for &round in ROUNDS.iter() {
        for &overflow in OVERFLOWS.iter() {
            for &intermediate in INTERMEDIATES.iter() {
                for &container in containers.iter() {
                    v.push(Config {
                        round,
                        overflow,
                        intermediate,
                        container,
                    });
                }
            }
        }
    }
    v
}

fn answer_vector(op: Op, w: u32, f: u32, cfg: Config) -> Vec<i128> {
    let n: i128 = 1i128 << w;
    let mut out = Vec::new();
    if op.is_ternary() {
        for a in 0..n {
            for b in 0..n {
                for c in 0..n {
                    out.push(eval(op, a, b, c, w, f, cfg));
                }
            }
        }
    } else {
        for a in 0..n {
            for b in 0..n {
                out.push(eval(op, a, b, 0, w, f, cfg));
            }
        }
    }
    out
}

fn partition(vectors: &[Vec<i128>]) -> Vec<Vec<usize>> {
    let mut classes: Vec<Vec<usize>> = Vec::new();
    for i in 0..vectors.len() {
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

/// The joint over p1's comparable witness set: W=4, F in {0,1,2}, five ops.
fn joint_classes(containers: &[Container]) -> (usize, Vec<Config>, Vec<Vec<usize>>) {
    let cfgs = configs(containers);
    let mut joint: Vec<Vec<i128>> = vec![Vec::new(); cfgs.len()];
    for f in [0u32, 1, 2] {
        let w = 4u32;
        for op in [Op::Add, Op::Sub, Op::Mul, Op::ChainAdd, Op::ChainSub] {
            for (i, &cfg) in cfgs.iter().enumerate() {
                joint[i].extend(answer_vector(op, w, f, cfg));
            }
        }
    }
    let classes = partition(&joint);
    (classes.len(), cfgs, classes)
}

fn main() {
    println!("p3: is the container observable in the answer function?");
    println!("witness set: W=4, F in {{0,1,2}}, ops {{add,sub,mul,a*b+c,a*b-c}}");
    println!("(p1 and p2 both report 24 classes for the assignments alone)\n");

    // one container at a time, to see what each contributes on its own
    for k in CONTAINERS.iter() {
        let (n, _, _) = joint_classes(&[*k]);
        println!("container {:?} alone: {} classes over 30 assignments", k, n);
    }

    // the three lossless containers together. if the container is a cost
    // coordinate, this is still 24: three containers times 30 assignments = 90
    // configs collapsing to the same 24 answer functions.
    let lossless = [Container::Packed, Container::Minimum, Container::Headroom];
    let (n_lossless, cfgs_l, classes_l) = joint_classes(&lossless);
    println!(
        "\nPacked + Minimum + Headroom together: {} configs -> {} classes",
        cfgs_l.len(),
        n_lossless
    );

    // does every class contain all three containers for one assignment? that is
    // the precise statement of "the container is not observable".
    let mut container_never_splits = true;
    for cls in classes_l.iter() {
        let assigns: Vec<String> = cls.iter().map(|&i| cfgs_l[i].assignment_label()).collect();
        let first = &assigns[0];
        if !assigns.iter().all(|a| a == first) {
            // a class merging two DIFFERENT assignments is fine and expected.
            continue;
        }
        if cls.len() != lossless.len() {
            container_never_splits = false;
            println!(
                "  container SPLIT an assignment: {} appears in a class of size {}",
                first,
                cls.len()
            );
        }
    }

    println!(
        "\nCLAIM: the three lossless containers add zero classes -> {}",
        if n_lossless == 24 { "CONFIRMED" } else { "REFUTED" }
    );
    println!(
        "CLAIM: no class ever splits one assignment across containers -> {}",
        if container_never_splits { "CONFIRMED" } else { "REFUTED" }
    );

    // the negative control: Lossy MUST be observable.
    let with_lossy = [
        Container::Packed,
        Container::Minimum,
        Container::Headroom,
        Container::Lossy,
    ];
    let (n_lossy, _, _) = joint_classes(&with_lossy);
    println!(
        "\nNEGATIVE CONTROL: adding the Lossy container: {} -> {} classes",
        n_lossless, n_lossy
    );
    if n_lossy <= n_lossless {
        println!("  !! CONTROL FAIL: a container that drops a bit was not observable.");
        println!("  the sweep is not measuring observability. every number above is void.");
        std::process::exit(1);
    }
    println!("  Lossy separates, so the sweep can see a container when there is");
    println!("  something to see. the lossless result above is a real negative.");
}
