// p6: is the fused arm a lowering of one policy, or is it a different policy the
// design already has an axis for?
//
// 139 wants fusion on signed saturating types and, because fusion changes the
// answer there at 42.14% of triples, proposes that a policy declare a set of
// acceptable answers so a cost model may choose fusion inside it. p2 measures the
// required set at 32 of 63 raw units and 65.2% of the representable range, which
// is a declaration that determines almost nothing.
//
// But the intermediate axis already exists. 140's p1 partitions assignments over
// rounding, overflow AND intermediate placement, and its phase-two p6 finds that
// the intermediate axis alone carries roughly half of the 24 classes. Its two
// positions are "round and reduce at each step" and "hold the intermediate and
// reduce once at the end".
//
// "Reduce once at the end" is what fusing a multiply-add IS.
//
// If that is an identity rather than a resemblance, then nothing needs to be
// declared: a consumer that wants the fused answer on a signed saturating type
// selects the exact-intermediate position, which is an observable policy position
// with its own declared semantics, and gets determinism and the fast arm at once.
// The slack mechanism would then be buying a capability the design already has,
// at the cost of the property it exists to protect.
//
// PREDICTIONS, before running:
//   X1. The fused arm under the STEPWISE-intermediate assignment is bit-identical
//       to the plain arm under the EXACT-intermediate assignment, at every input,
//       in every cell. Zero mismatches.
//   X2. It is NOT identical to the plain arm under the stepwise assignment in the
//       cells where fusion differs at all. Without this, X1 is vacuous: if all
//       three coincided there would be no axis and no question.
//   X3. The two intermediate positions are therefore distinguishable exactly where
//       139's fusion table is nonzero, cell for cell, which is what makes the
//       identification a claim rather than a definition restated.
//
// CONTROL. X2 is the non-vacuity control and X3 is the cell-for-cell agreement
// control. A probe reporting only X1 would be reporting that I wrote the same
// expression twice.
//
// Run: rustc -O -o /tmp/p6 p6_fusion_is_an_axis_position_already.rs && /tmp/p6

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
fn shift(p: i128, f: u32) -> i128 {
    if f == 0 {
        p
    } else {
        p >> f
    }
}

/// Plain arm under the STEPWISE-intermediate assignment.
fn plain_stepwise(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32) -> i128 {
    reduce(reduce(shift(a * b, f), s, o, w) + c, s, o, w)
}
/// Plain arm under the EXACT-intermediate assignment: the intermediate is held
/// and the reduction happens once.
fn plain_exact(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32) -> i128 {
    reduce(shift(a * b, f) + c, s, o, w)
}
/// The fused LOWERING of the stepwise assignment, which is what 139 wants a cost
/// model to be allowed to choose.
fn fused_lowering_of_stepwise(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32) -> i128 {
    reduce(shift(a * b, f) + c, s, o, w)
}

fn main() {
    let w = 6u32;
    println!("p6: is the fused arm the exact-intermediate policy?");
    println!("W = {w}, exhaustive over all triples per cell, rounding = floor\n");
    println!(
        "{:<22} {:>6} {:>16} {:>18} {:>16}",
        "cell", "F", "X1 mismatches", "X2 stepwise diffs", "X3 rate"
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
            for f in 0..=5u32 {
                let (l, h) = (lo(s, w), hi(s, w));
                let mut x1 = 0u64;
                let mut x2 = 0u64;
                let mut n = 0u64;
                for a in l..=h {
                    for b in l..=h {
                        for c in l..=h {
                            n += 1;
                            let fu = fused_lowering_of_stepwise(a, b, c, s, o, w, f);
                            let ex = plain_exact(a, b, c, s, o, w, f);
                            let st = plain_stepwise(a, b, c, s, o, w, f);
                            if fu != ex {
                                x1 += 1;
                            }
                            if st != ex {
                                x2 += 1;
                            }
                        }
                    }
                }
                println!(
                    "{:<22} {f:>6} {x1:>16} {x2:>18} {:>15.2}%",
                    if f == 0 { name.as_str() } else { "" },
                    100.0 * x2 as f64 / n as f64
                );
            }
        }
    }

    println!("\nX1 holds iff the mismatch column is 0 everywhere: the fused lowering of");
    println!("the stepwise policy and the plain lowering of the exact-intermediate policy");
    println!("are the same function.");
    println!("X2 holds iff the stepwise-diffs column is nonzero wherever fusion differs:");
    println!("the two intermediate positions are genuinely two positions.");
    println!("X3's rate column should reproduce p1's fusion table under floor, which is");
    println!("what makes the identification a measured claim rather than a restatement.");
}
