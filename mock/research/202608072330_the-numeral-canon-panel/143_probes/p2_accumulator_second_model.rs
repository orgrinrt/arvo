//! p2 (143): the accumulator question, on a deliberately different model.
//!
//! `141` section 2.2 finds that the minimising concern is answer-invisible at
//! the column but answer-VISIBLE at the accumulator, in exactly one cell:
//! `signedness = signed, overflow = saturating`. It says of that finding that it
//! rests on one instrument, that it is the kind of result that is easy to
//! produce by modelling an accumulator in a way no implementation would, and it
//! asks specifically for a second reader to build a DIFFERENT accumulator model
//! and see whether the cell still separates.
//!
//! So I did not open `141_probes/p4_the_concern_reaches_the_accumulator.rs`.
//! This model is derived from what an accumulator is in a column kernel rather
//! than from theirs, and it differs in shape:
//!
//!   * theirs, per its prose, varies an accumulator width across per-operation
//!     evaluation. Mine is a FOLD over a slice of N stored values, so the
//!     accumulator persists across steps and the policy is applied at every
//!     accumulation step at the accumulator's own width, which is what a
//!     fixed-width register actually does. `mock/benches/variants/satfold-*`
//!     and `warm-clamp-*` are kernels of exactly this shape.
//!   * the narrowing back to the declared width happens once, at the end.
//!   * the sweep is over whole input slices rather than operand tuples.
//!
//! Predictions, written before running, derived rather than read:
//!   P2a. The accumulator width is answer-VISIBLE at signed saturating.
//!   P2b. It is INVISIBLE at wrapping, for both signednesses, because reduction
//!        modulo 2^acc followed by reduction modulo 2^W is reduction modulo 2^W
//!        whenever W <= acc.
//!   P2c. It is INVISIBLE at unsigned saturating, because a one-sided clamp of a
//!        monotone accumulation is a congruence, which is the mechanism `139`
//!        found for a different question.
//! Together those reproduce `141`'s cell. If they do, the finding is real on two
//! independent models. If P2a falls, `141`'s F2 is an artifact of its
//! construction and the convergence stands unqualified.
//!
//! THE CASE THAT MUST FAIL. A sweep that cannot see an accumulator at all
//! reports "invisible" everywhere and looks like a clean negative. So a LOSSY
//! accumulator, one bit narrower than the declared width, is swept as a control
//! and MUST be visible in every cell. If it is not, this instrument is blind to
//! the accumulator and every verdict below is void. A duplicate control checks
//! the other direction: the same accumulator width reached by two different
//! constructions must merge.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sign {
    Unsigned,
    Signed,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Overflow {
    Wrap,
    Saturate,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Round {
    TowardZero,
    Floor,
}

/// What the fold does. All three are real column-kernel shapes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kernel {
    /// running sum of the column
    Sum,
    /// running sum of products against a fixed coefficient, i.e. a dot product
    Dot,
    /// alternating sum, which is what makes the low clamp reachable
    AltSum,
}

impl Kernel {
    fn name(&self) -> &'static str {
        match self {
            Kernel::Sum => "sum",
            Kernel::Dot => "dot",
            Kernel::AltSum => "altsum",
        }
    }
}

fn lo(w: u32, s: Sign) -> i128 {
    match s {
        Sign::Unsigned => 0,
        Sign::Signed => -(1i128 << (w - 1)),
    }
}

fn hi(w: u32, s: Sign) -> i128 {
    match s {
        Sign::Unsigned => (1i128 << w) - 1,
        Sign::Signed => (1i128 << (w - 1)) - 1,
    }
}

/// Bring a value into the range of a `w`-bit value of signedness `s`.
fn reduce(v: i128, w: u32, s: Sign, o: Overflow) -> i128 {
    let l = lo(w, s);
    let h = hi(w, s);
    match o {
        Overflow::Saturate => {
            if v < l {
                l
            } else if v > h {
                h
            } else {
                v
            }
        }
        Overflow::Wrap => {
            let m: i128 = 1i128 << w;
            let r = (v - l).rem_euclid(m);
            r + l
        }
    }
}

fn shift_round(v: i128, f: u32, r: Round) -> i128 {
    if f == 0 {
        return v;
    }
    let d: i128 = 1i128 << f;
    let q = v.div_euclid(d);
    let rem = v.rem_euclid(d);
    match r {
        Round::Floor => q,
        Round::TowardZero => {
            if v >= 0 || rem == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Config {
    sign: Sign,
    overflow: Overflow,
    round: Round,
    /// The accumulator's width, in bits. The whole subject of this probe.
    acc_w: u32,
    /// Control marker: 0 real, 1 duplicate-by-another-construction, 2 unused.
    control: u8,
}

/// The fold. An accumulator of `acc_w` bits, the policy applied at every step at
/// the accumulator's own width, narrowed once to the declared width at the end.
fn run(k: Kernel, xs: &[i128], w: u32, f: u32, c: Config) -> i128 {
    let mut acc: i128 = 0;
    for (i, &x) in xs.iter().enumerate() {
        let term = match k {
            Kernel::Sum => x,
            // a fixed coefficient, chosen to be representable and not 1
            Kernel::Dot => {
                let coeff: i128 = 3;
                shift_round(x * coeff, f, c.round)
            }
            Kernel::AltSum => {
                if i % 2 == 0 {
                    x
                } else {
                    -x
                }
            }
        };
        acc = reduce(acc + term, c.acc_w, c.sign, c.overflow);
    }
    reduce(acc, w, c.sign, c.overflow)
}

fn values(w: u32, s: Sign) -> Vec<i128> {
    (lo(w, s)..=hi(w, s)).collect()
}

fn answer_vector(k: Kernel, w: u32, f: u32, len: usize, c: Config) -> Vec<i128> {
    let vs = values(w, c.sign);
    let mut out = Vec::new();
    let mut idx = vec![0usize; len];
    loop {
        let xs: Vec<i128> = idx.iter().map(|&i| vs[i]).collect();
        out.push(run(k, &xs, w, f, c));
        let mut p = len;
        loop {
            if p == 0 {
                return out;
            }
            p -= 1;
            idx[p] += 1;
            if idx[p] < vs.len() {
                break;
            }
            idx[p] = 0;
        }
    }
}

fn classes(cfgs: &[Config], k: Kernel, w: u32, f: u32, len: usize) -> Vec<Vec<usize>> {
    let vecs: Vec<Vec<i128>> = cfgs
        .iter()
        .map(|&c| answer_vector(k, w, f, len, c))
        .collect();
    let mut cls: Vec<Vec<usize>> = Vec::new();
    for i in 0..cfgs.len() {
        let mut placed = false;
        for cl in cls.iter_mut() {
            if vecs[cl[0]] == vecs[i] {
                cl.push(i);
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

fn main() {
    let w = 4u32;
    let len = 3usize;
    let mut failures = 0usize;

    println!("p2 (143): is the accumulator width answer-visible?");
    println!("model: a FOLD over a slice, policy applied per step at the accumulator width,");
    println!("narrowed once at the end. W={w}, slice length {len}, exhaustive over all slices.\n");

    println!("{:<10} {:<10} {:<4} {:<8} {}", "sign", "overflow", "F", "kernel", "acc widths -> classes");
    println!("{}", "-".repeat(78));

    let mut visible_cells: Vec<String> = Vec::new();
    let mut invisible_cells: Vec<String> = Vec::new();
    let mut lossy_blind: Vec<String> = Vec::new();

    for &sign in [Sign::Unsigned, Sign::Signed].iter() {
        for &overflow in [Overflow::Wrap, Overflow::Saturate].iter() {
            for &f in [0u32, 1, 2].iter() {
                for &k in [Kernel::Sum, Kernel::Dot, Kernel::AltSum].iter() {
                    // the real arm: three lossless accumulator widths
                    let real: Vec<Config> = [w, w + 2, w + 4]
                        .iter()
                        .map(|&a| Config {
                            sign,
                            overflow,
                            round: Round::TowardZero,
                            acc_w: a,
                            control: 0,
                        })
                        .collect();
                    let n_real = classes(&real, k, w, f, len).len();

                    // duplicate control: the same width twice, must merge
                    let mut dup = real.clone();
                    dup.push(Config { control: 1, ..real[0] });
                    let n_dup = classes(&dup, k, w, f, len).len();
                    if n_dup != n_real {
                        println!("  !! CONTROL FAIL: a duplicate accumulator width did not merge");
                        failures += 1;
                    }

                    // lossy control: one bit NARROWER than the declared width.
                    // this must be visible, or the sweep is blind.
                    let mut lossy = real.clone();
                    lossy.push(Config {
                        acc_w: w - 1,
                        ..real[0]
                    });
                    let n_lossy = classes(&lossy, k, w, f, len).len();

                    let cell = format!(
                        "{:?}/{:?}/F={}/{}",
                        sign,
                        overflow,
                        f,
                        k.name()
                    );
                    if n_real > 1 {
                        visible_cells.push(cell.clone());
                    } else {
                        invisible_cells.push(cell.clone());
                    }
                    if n_lossy <= n_real {
                        lossy_blind.push(cell.clone());
                    }

                    println!(
                        "{:<10} {:<10} {:<4} {:<8} {{W,W+2,W+4}} -> {}   lossy(W-1) -> {}",
                        format!("{sign:?}"),
                        format!("{overflow:?}"),
                        f,
                        k.name(),
                        n_real,
                        n_lossy
                    );
                }
            }
        }
    }

    println!("\n=== verdict by cell ===");
    println!("accumulator VISIBLE in {} cells:", visible_cells.len());
    for c in visible_cells.iter() {
        println!("  {c}");
    }
    println!("accumulator invisible in {} cells", invisible_cells.len());

    let all_visible_are_signed_sat = visible_cells
        .iter()
        .all(|c| c.starts_with("Signed/Saturate"));
    let some_signed_sat_visible = visible_cells
        .iter()
        .any(|c| c.starts_with("Signed/Saturate"));

    println!("\nP2a (visible at signed saturating): {}", if some_signed_sat_visible { "CONFIRMED" } else { "REFUTED" });
    println!(
        "P2b + P2c (invisible everywhere else): {}",
        if all_visible_are_signed_sat { "CONFIRMED" } else { "REFUTED" }
    );
    println!(
        "\n141's cell reproduces on an independent model: {}",
        if some_signed_sat_visible && all_visible_are_signed_sat {
            "YES"
        } else {
            "NO, and the disagreement is the finding"
        }
    );

    println!("\n=== controls ===");
    if !lossy_blind.is_empty() {
        println!("!! CONTROL FAIL: a lossy accumulator was invisible in {} cells:", lossy_blind.len());
        for c in lossy_blind.iter() {
            println!("  {c}");
        }
        println!("the sweep is blind to the accumulator there, so its negatives are void.");
        failures += 1;
    } else {
        println!("the lossy accumulator is visible in every cell, so the sweep can see an");
        println!("accumulator when there is something to see, and the invisible cells are real.");
    }
    if failures > 0 {
        std::process::exit(1);
    }
}
