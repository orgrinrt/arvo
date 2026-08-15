// p1b: my p1 reproduces 139's 42.14% at F=0 and disagrees at every F>0.
// Find the mechanism rather than reporting the disagreement.
//
// 139's table, section 4, signed rows:
//   signed wrapping    0.00  1.64  5.54 12.34 22.22 33.40
//   signed saturating 42.14 39.10 35.67 31.96 29.52 33.41
//
// p1's table (both truncation modes):
//   signed wrapping    0.00  0.00  0.00  0.00  0.00  0.00
//   signed saturating 42.14 37.50 30.22 19.77  7.53  0.01
//
// HYPOTHESIS. A fixed-point multiply has to do three things: form the wide
// product, discard F fraction bits, and bring the result back into the declared
// range. The order of the last two is not fixed by anything either file states.
//
//   Model A (p1's): reduce AFTER the shift.   mul(a,b) = reduce(shift(a*b))
//   Model B:        reduce BEFORE the shift.  mul(a,b) = shift(reduce(a*b))
//
// Under A, at F=0 the two coincide, which is why we agree exactly there and only
// there. Under B, wrapping the product before shifting is not the same map as
// wrapping after, so signed wrapping picks up a nonzero difference that rises
// with F, which is the shape 139 reports and mine does not have.
//
// If B reproduces 139's numbers, the disagreement is a modelling choice neither
// predicate carries, not an error in either instrument, and the design owes an
// answer before either table means anything.
//
// PREDICTIONS, before running:
//   R1. Model B, signed wrapping, reproduces 0.00 / 1.64 / 5.54 / 12.34 / 22.22
//       / 33.40 to two decimal places under at least one truncation mode.
//   R2. Model B, signed saturating, reproduces 42.14 / 39.10 / 35.67 / 31.96 /
//       29.52 / 33.41 under the same mode.
//   R3. Model B, unsigned, stays at 0.00% everywhere, because both files agree
//       there and a mechanism that explains the signed rows must not disturb the
//       unsigned ones.
//   R4. Models A and B are themselves distinguishable as ordinary arithmetic:
//       their stepwise arms disagree at a nonzero rate at F>0. If they did not,
//       the hypothesis would be untestable and this probe would prove nothing.
//
// CONTROL. R4 is the negative control: a run in which A and B agree everywhere
// would mean the two models are the same model and any reproduction is luck.
//
// Run: rustc -O -o /tmp/p1b p1b_which_model_produces_139s_table.rs && /tmp/p1b

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sign {
    U,
    S,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ovf {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Trunc {
    TowardZero,
    Floor,
}

fn lo(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => 0,
        Sign::S => -(1i128 << (w - 1)),
    }
}
fn hi(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => (1i128 << w) - 1,
        Sign::S => (1i128 << (w - 1)) - 1,
    }
}

fn reduce(v: i128, s: Sign, o: Ovf, w: u32) -> i128 {
    match o {
        Ovf::Sat => v.clamp(lo(s, w), hi(s, w)),
        Ovf::Wrap => {
            let m = 1i128 << w;
            let r = v.rem_euclid(m);
            match s {
                Sign::U => r,
                Sign::S => {
                    if r >= (1i128 << (w - 1)) {
                        r - m
                    } else {
                        r
                    }
                }
            }
        }
    }
}

fn shift(p: i128, f: u32, t: Trunc) -> i128 {
    if f == 0 {
        return p;
    }
    match t {
        Trunc::TowardZero => p / (1i128 << f),
        Trunc::Floor => p >> f,
    }
}

/// Model A: form the wide product, shift, then bring into range.
fn mul_a(a: i128, b: i128, s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> i128 {
    reduce(shift(a * b, f, t), s, o, w)
}
/// Model B: form the wide product, bring into range, then shift.
fn mul_b(a: i128, b: i128, s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> i128 {
    shift(reduce(a * b, s, o, w), f, t)
}

fn rate<F: Fn(i128, i128, i128) -> bool>(s: Sign, w: u32, pred: F) -> f64 {
    let (l, h) = (lo(s, w), hi(s, w));
    let mut d = 0u64;
    let mut n = 0u64;
    for a in l..=h {
        for b in l..=h {
            for c in l..=h {
                n += 1;
                if pred(a, b, c) {
                    d += 1;
                }
            }
        }
    }
    100.0 * d as f64 / n as f64
}

fn main() {
    let w = 6u32;
    println!("p1b: which multiply model produces 139's table\n");

    for t in [Trunc::TowardZero, Trunc::Floor] {
        let tn = match t {
            Trunc::TowardZero => "toward zero",
            Trunc::Floor => "floor",
        };
        println!("=== fusion difference rate, MODEL B (reduce before shift), rounding = {tn} ===");
        println!(
            "{:<22} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
            "cell", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5"
        );
        for s in [Sign::U, Sign::S] {
            for o in [Ovf::Wrap, Ovf::Sat] {
                let name = format!(
                    "{}, {}",
                    if s == Sign::U { "unsigned" } else { "signed" },
                    if o == Ovf::Wrap {
                        "wrapping"
                    } else {
                        "saturating"
                    }
                );
                let mut row = format!("{name:<22}");
                for f in 0..=5u32 {
                    // stepwise under B: the multiply reduces, then the add reduces.
                    // fused: one reduction, at the end, over the unreduced product.
                    let r = rate(s, w, |a, b, c| {
                        let st = reduce(mul_b(a, b, s, o, w, f, t) + c, s, o, w);
                        let fu = reduce(shift(a * b, f, t) + c, s, o, w);
                        st != fu
                    });
                    row.push_str(&format!(" {r:>7.2}%"));
                }
                println!("{row}");
            }
        }
        println!();
    }

    println!("=== 139's reported table, for side by side reading ===");
    println!(
        "{:<22} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
        "cell", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5"
    );
    println!(
        "{:<22} {:>7}% {:>7}% {:>7}% {:>7}% {:>7}% {:>7}%",
        "unsigned, wrapping", "0.00", "0.00", "0.00", "0.00", "0.00", "0.00"
    );
    println!(
        "{:<22} {:>7}% {:>7}% {:>7}% {:>7}% {:>7}% {:>7}%",
        "unsigned, saturating", "0.00", "0.00", "0.00", "0.00", "0.00", "0.00"
    );
    println!(
        "{:<22} {:>7}% {:>7}% {:>7}% {:>7}% {:>7}% {:>7}%",
        "signed, wrapping", "0.00", "1.64", "5.54", "12.34", "22.22", "33.40"
    );
    println!(
        "{:<22} {:>7}% {:>7}% {:>7}% {:>7}% {:>7}% {:>7}%",
        "signed, saturating", "42.14", "39.10", "35.67", "31.96", "29.52", "33.41"
    );
    println!();

    println!("=== R4 CONTROL: are models A and B distinguishable at all? ===");
    println!(
        "(rate at which the two STEPWISE arms disagree; zero everywhere would void this probe)"
    );
    for t in [Trunc::TowardZero, Trunc::Floor] {
        let tn = match t {
            Trunc::TowardZero => "toward zero",
            Trunc::Floor => "floor",
        };
        for s in [Sign::U, Sign::S] {
            for o in [Ovf::Wrap, Ovf::Sat] {
                let name = format!(
                    "{}, {}, {tn}",
                    if s == Sign::U { "unsigned" } else { "signed" },
                    if o == Ovf::Wrap {
                        "wrapping"
                    } else {
                        "saturating"
                    }
                );
                let mut row = format!("{name:<38}");
                for f in 0..=5u32 {
                    let r = rate(s, w, |a, b, c| {
                        let sa = reduce(mul_a(a, b, s, o, w, f, t) + c, s, o, w);
                        let sb = reduce(mul_b(a, b, s, o, w, f, t) + c, s, o, w);
                        sa != sb
                    });
                    row.push_str(&format!(" {r:>7.2}%"));
                }
                println!("{row}");
            }
        }
    }
}
