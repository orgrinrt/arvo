//! p6 (phase two): two questions Q51 raises against my phase-one probes.
//!
//! Q51 carries two repairs that bear directly on p1 and p3, and both are
//! testable on my own instruments rather than by argument.
//!
//! QUESTION A. Q51 says component one fixes the DENOTED answer rather than the
//! computed one, because otherwise every arm under one assignment computes the
//! same result and no fidelity coordinate can vary. If that is right, then an
//! axis that chooses HOW a chain is realised, rather than WHAT it denotes, sits
//! in component two and not component one. The intermediate-width axis is the
//! candidate: "hold the product exact and round once" against "round at each
//! step" reads as two realisations of one denotation.
//!
//! So: how much of p1's count of 24 is carried by that one axis? If most of it
//! is, then p1 is counting realisations and denotations together and the number
//! belongs to a different question than the one it was cited for.
//!
//! Prediction A, before running: collapsing the intermediate axis leaves
//! strictly fewer than 24 classes. I do not have a prior on how many.
//!
//! QUESTION B. Q51 says observability depends on whether the limit is read at
//! the DECLARED width or the CONTAINER width, and reports 0% against 89.081%
//! across that distinction. p3 applies the range policy at the declared width
//! and stores through the container, so p3's negative result is conditional on
//! a reading p3 never states. If the limit is instead read at the container
//! width, a wider container stops clamping where a narrower one does, and the
//! container should become observable.
//!
//! Prediction B, before running: reading the limit at the container width makes
//! the three lossless containers observable, i.e. the class count rises above
//! 24. If it does, p3's F3 needs the reading written into its predicate, and my
//! phase-one statement of it was incomplete rather than wrong.
//!
//! THE CASE THAT MUST FAIL. Both questions are answered by the same partitioner
//! p1 and p3 use, so the same controls apply: a duplicate configuration must
//! collapse and a corrupted twin must not. Both are carried through every sweep
//! below and the probe exits non-zero if either misbehaves.

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Container {
    Packed,
    Minimum,
    Headroom,
}
const CONTAINERS: [Container; 3] = [Container::Packed, Container::Minimum, Container::Headroom];

/// Where the range policy's limit is read. Q51's distinction, made explicit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LimitAt {
    DeclaredWidth,
    ContainerWidth,
}

fn rung_bits(w: u32) -> u32 {
    let mut r = 8u32;
    while r < w {
        r *= 2;
    }
    r
}

impl Container {
    fn bits(&self, w: u32) -> u32 {
        match self {
            Container::Packed => w,
            Container::Minimum => rung_bits(w),
            Container::Headroom => 2 * rung_bits(w),
        }
    }
    fn round_trip(&self, v: i128, w: u32) -> i128 {
        let b = self.bits(w).min(126);
        v & ((1i128 << b) - 1)
    }
}

#[derive(Clone, Copy, Debug)]
struct Config {
    round: Round,
    overflow: Overflow,
    intermediate: Intermediate,
    container: Container,
    control: u8,
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

/// The limit width is the whole of question B.
fn apply_overflow(r: i128, w: u32, cfg: Config, at: LimitAt) -> i128 {
    let lw = match at {
        LimitAt::DeclaredWidth => w,
        LimitAt::ContainerWidth => cfg.container.bits(w).min(100),
    };
    let modulus: i128 = 1i128 << lw;
    let max = modulus - 1;
    match cfg.overflow {
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

fn eval(op: Op, a: i128, b: i128, c: i128, w: u32, f: u32, cfg: Config, at: LimitAt) -> i128 {
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
                    let p = apply_overflow(p, w, cfg, at);
                    p + sign * c
                }
                Intermediate::ExactThenRoundOnce => {
                    shift_round(a * b + sign * (c << f), f, cfg.round)
                }
            }
        }
    };
    let r = apply_overflow(r, w, cfg, at);
    let r = k.round_trip(r, w);
    if cfg.control == 2 {
        (r + 1).rem_euclid(1i128 << w)
    } else {
        r
    }
}

fn answer_vector(op: Op, w: u32, f: u32, cfg: Config, at: LimitAt) -> Vec<i128> {
    let n: i128 = 1i128 << w;
    let mut out = Vec::new();
    if matches!(op, Op::ChainAdd | Op::ChainSub) {
        for a in 0..n {
            for b in 0..n {
                for c in 0..n {
                    out.push(eval(op, a, b, c, w, f, cfg, at));
                }
            }
        }
    } else {
        for a in 0..n {
            for b in 0..n {
                out.push(eval(op, a, b, 0, w, f, cfg, at));
            }
        }
    }
    out
}

fn partition(vs: &[Vec<i128>]) -> Vec<Vec<usize>> {
    let mut cls: Vec<Vec<usize>> = Vec::new();
    for i in 0..vs.len() {
        let mut placed = false;
        for c in cls.iter_mut() {
            if vs[c[0]] == vs[i] {
                c.push(i);
                placed = true;
                break;
            }
        }
        if !placed {
            cls.push(vec![i]);
        }
    }
    cls
}

/// The comparable witness set p1 pins and p2 reproduces.
fn joint(cfgs: &[Config], at: LimitAt) -> Vec<Vec<usize>> {
    let mut j: Vec<Vec<i128>> = vec![Vec::new(); cfgs.len()];
    for f in [0u32, 1, 2] {
        for op in [Op::Add, Op::Sub, Op::Mul, Op::ChainAdd, Op::ChainSub] {
            for (i, &c) in cfgs.iter().enumerate() {
                j[i].extend(answer_vector(op, 4, f, c, at));
            }
        }
    }
    partition(&j)
}

fn real_classes(cls: &[Vec<usize>], cfgs: &[Config]) -> usize {
    cls.iter()
        .filter(|c| c.iter().any(|&i| cfgs[i].control == 0))
        .count()
}

fn with_controls(mut v: Vec<Config>) -> Vec<Config> {
    let base = v[0];
    v.push(Config { control: 1, ..base });
    v.push(Config { control: 2, ..base });
    v
}

fn check(cls: &[Vec<usize>], cfgs: &[Config], where_: &str) -> usize {
    let find = |pred: u8| {
        cls.iter().position(|c| {
            c.iter()
                .any(|&i| cfgs[i].control == pred && i >= cfgs.len() - 2)
        })
    };
    let base = cls.iter().position(|c| c.contains(&0)).unwrap();
    let d = find(1);
    let c = find(2);
    let mut bad = 0;
    match d {
        Some(x) if x == base => {}
        _ => {
            println!("  !! CONTROL FAIL {where_}: duplicate did not collapse into its twin");
            bad += 1;
        }
    }
    match c {
        Some(x) if x != base => {}
        _ => {
            println!("  !! CONTROL FAIL {where_}: corrupted twin collapsed into its twin");
            bad += 1;
        }
    }
    bad
}

fn main() {
    let mut failures = 0usize;

    println!("p6 (phase two): two questions Q51 raises against p1 and p3");
    println!("witness set throughout: W=4, F in {{0,1,2}}, ops {{add,sub,mul,a*b+c,a*b-c}}\n");

    // ---------- QUESTION A ----------
    println!("=== A. how much of the count does the intermediate axis carry? ===");

    // full: rounding x overflow x intermediate, one container, declared width.
    // this must reproduce p1's 24, or p6 and p1 disagree and one is wrong.
    let mut full = Vec::new();
    for &r in ROUNDS.iter() {
        for &o in OVERFLOWS.iter() {
            for &m in [
                Intermediate::RoundEachStep,
                Intermediate::ExactThenRoundOnce,
            ]
            .iter()
            {
                full.push(Config {
                    round: r,
                    overflow: o,
                    intermediate: m,
                    container: Container::Packed,
                    control: 0,
                });
            }
        }
    }
    let full = with_controls(full);
    let cls = joint(&full, LimitAt::DeclaredWidth);
    let n_full = real_classes(&cls, &full);
    failures += check(&cls, &full, "A/full");
    println!("rounding x overflow x intermediate (30): {n_full} classes");
    println!(
        "  agrees with p1's 24 -> {}",
        if n_full == 24 {
            "YES"
        } else {
            "NO, p1 and p6 disagree"
        }
    );

    // collapsed: intermediate held fixed, so only rounding x overflow varies.
    for &m in [
        Intermediate::RoundEachStep,
        Intermediate::ExactThenRoundOnce,
    ]
    .iter()
    {
        let mut v = Vec::new();
        for &r in ROUNDS.iter() {
            for &o in OVERFLOWS.iter() {
                v.push(Config {
                    round: r,
                    overflow: o,
                    intermediate: m,
                    container: Container::Packed,
                    control: 0,
                });
            }
        }
        let v = with_controls(v);
        let cls = joint(&v, LimitAt::DeclaredWidth);
        let n = real_classes(&cls, &v);
        failures += check(&cls, &v, "A/collapsed");
        println!("rounding x overflow alone, intermediate = {m:?} (15): {n} classes");
    }
    println!(
        "\nprediction A (collapsing the intermediate axis leaves fewer than 24): {}",
        if n_full == 24 {
            "see the two numbers above"
        } else {
            "n/a, full count did not reproduce"
        }
    );

    // ---------- QUESTION B ----------
    println!(
        "\n=== B. is the container observable if the limit is read at the container width? ==="
    );
    let mut wide = Vec::new();
    for &r in ROUNDS.iter() {
        for &o in OVERFLOWS.iter() {
            for &m in [
                Intermediate::RoundEachStep,
                Intermediate::ExactThenRoundOnce,
            ]
            .iter()
            {
                for &k in CONTAINERS.iter() {
                    wide.push(Config {
                        round: r,
                        overflow: o,
                        intermediate: m,
                        container: k,
                        control: 0,
                    });
                }
            }
        }
    }
    let wide = with_controls(wide);

    let cls_decl = joint(&wide, LimitAt::DeclaredWidth);
    let n_decl = real_classes(&cls_decl, &wide);
    failures += check(&cls_decl, &wide, "B/declared");

    let cls_cont = joint(&wide, LimitAt::ContainerWidth);
    let n_cont = real_classes(&cls_cont, &wide);
    failures += check(&cls_cont, &wide, "B/container");

    println!("90 configs (30 assignments x 3 lossless containers)");
    println!("  limit read at the DECLARED width:  {n_decl} classes");
    println!("  limit read at the CONTAINER width: {n_cont} classes");
    println!(
        "\nprediction B (the container-width reading makes the container observable): {}",
        if n_cont > n_decl {
            "CONFIRMED"
        } else {
            "REFUTED"
        }
    );
    if n_cont > n_decl {
        println!("  so p3's F3 holds only under the declared-width reading, and its");
        println!("  predicate must say so. phase one stated it without that dimension.");
    }

    println!("\n=== controls ===");
    if failures > 0 {
        println!("CONTROL FAILURES: {failures}. every number above is void.");
        std::process::exit(1);
    }
    println!("controls behaved in every sweep: the partitioner separates.");
}
