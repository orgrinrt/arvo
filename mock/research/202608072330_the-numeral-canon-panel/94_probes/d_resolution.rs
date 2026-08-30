// Probe D. When two operands carry different strategies, what may the output's
// strategy honestly claim?
//
// The shipped design resolved a mixed-strategy operation toward the "more
// conservative" side: Hot + Precise gives Precise. That is a JOIN. This probe
// asks what the joined marker is a claim ABOUT, because there are two readings
// and they point opposite ways:
//
//   operational  the marker says what THIS OPERATION does. Join is sound: the
//                addition really was performed exactly.
//   value-level  the marker says what THIS VALUE is worth. Join is unsound: an
//                operand that already lost accuracy cannot have it restored by
//                the policy of a later operation.
//
// The probe measures the gap between the two readings. It builds a value under
// a lossy path, combines it with an exact one under an exactly-performed
// operation, and asks how far the result is from the exactly-rounded answer.
// Under the operational reading that distance is allowed to be nonzero while
// the type says Precise; under the value-level reading it may not be.
//
// It also measures the DUAL claim, which is the thing a meet would buy: is the
// weaker operand's error bound in fact an upper bound on the result's error?
// If it is, the meet is not merely conservative, it is tight, and there is a
// real lattice to resolve in.
//
// Build and run:
//   rustc --edition 2024 -O -o d_resolution d_resolution.rs && ./d_resolution

/// Declared type: W total bits, F fractional. Values are integers scaled 2^F.
const W: u32 = 16;
const F: u32 = 8;
const N: i64 = 1 << W;
const ONE: i64 = 1 << F;

/// The lossy path. A strategy weighing cycles picks a narrower carrier for an
/// intermediate, which costs fraction bits. `drop` names how many.
fn lossy(x: i64, drop: u32) -> i64 {
    if drop == 0 {
        x
    } else {
        (x >> drop) << drop
    }
}

/// Exactly-rounded reference for the declared type.
fn exact_round(num: i128, den: i128) -> i64 {
    // round-half-up of num/den into the declared scale
    let scaled = num * (ONE as i128);
    ((scaled + den / 2) / den) as i64
}

fn main() {
    println!("probe D: what a resolved marker may claim");
    println!("declared type: W = {W}, F = {F}, one = {ONE}");
    println!();

    // ----------------------------------------------------------------------
    // Case 1. y = a + b, where a came off a lossy path and b is exact, and the
    // addition itself is performed exactly at full width. The addition is
    // faultless. The result is still wrong, and the amount it is wrong by is a
    // property of `a` alone.
    // ----------------------------------------------------------------------
    println!("case 1: y = a + b, addition performed EXACTLY, `a` off a lossy path");
    println!(
        "{:>6} {:>14} {:>14} {:>16} {:>16}",
        "drop", "pairs", "y != exact", "max |err| (ulp)", "max |err| in a"
    );
    for drop in [0u32, 1, 2, 4, 8] {
        let mut differ: u64 = 0;
        let mut total: u64 = 0;
        let mut worst_y: i64 = 0;
        let mut worst_a: i64 = 0;
        // sweep the declared domain on a stride so every residue class of the
        // dropped bits is hit, at every magnitude
        let stride = 37;
        let mut a_raw = 0i64;
        while a_raw < N {
            let a = lossy(a_raw, drop);
            let ea = (a - a_raw).abs();
            if ea > worst_a {
                worst_a = ea;
            }
            let mut b = 0i64;
            while b < N {
                total += 1;
                let y = a + b; // performed exactly, no policy applied
                let y_exact = a_raw + b; // what the value should have been
                if y != y_exact {
                    differ += 1;
                    let d = (y - y_exact).abs();
                    if d > worst_y {
                        worst_y = d;
                    }
                }
                b += stride * 7;
            }
            a_raw += stride;
        }
        println!(
            "{:>6} {:>14} {:>14} {:>16} {:>16}",
            drop, total, differ, worst_y, worst_a
        );
    }

    println!();
    println!("  the two max columns are equal at every row, which is the whole point:");
    println!("  the result's error is exactly the operand's error, and the operation");
    println!("  had no opportunity to introduce or remove any of it.");

    // ----------------------------------------------------------------------
    // Case 2. The same shape through a division, where a strategy really does
    // differ in what the operation does, so the operational reading has
    // something to say. Two questions are separated:
    //    (i)  does the exact division repair a lossy operand?   (it does not)
    //    (ii) does an exact division on exact operands land on the exactly
    //         rounded answer?                                    (it does)
    // ----------------------------------------------------------------------
    println!();
    println!("case 2: y = a / b, division performed EXACTLY, `a` off a lossy path");
    println!(
        "{:>6} {:>14} {:>14} {:>16}",
        "drop", "pairs", "y != exact", "max |err| (ulp)"
    );
    for drop in [0u32, 1, 2, 4, 8] {
        let mut differ: u64 = 0;
        let mut total: u64 = 0;
        let mut worst: i64 = 0;
        let mut a_raw = 1i64;
        while a_raw < N {
            let a = lossy(a_raw, drop);
            let mut b = 1i64;
            while b < N {
                total += 1;
                let y = exact_round(a as i128, b as i128);
                let y_exact = exact_round(a_raw as i128, b as i128);
                if y != y_exact {
                    differ += 1;
                    let d = (y - y_exact).abs();
                    if d > worst {
                        worst = d;
                    }
                }
                b += 401;
            }
            a_raw += 53;
        }
        println!("{:>6} {:>14} {:>14} {:>16}", drop, total, differ, worst);
    }

    // ----------------------------------------------------------------------
    // Case 3. Is the weaker operand's bound an upper bound on the result's
    // bound? If yes, a MEET resolution is tight rather than merely safe, and
    // the guarantees form a usable lattice. If no, the meet is unsound too and
    // neither direction is defensible without a separate error term.
    //
    // Multiplication is the interesting one: an error in an operand is scaled
    // by the OTHER operand, so a bound stated in ulps does not survive.
    // ----------------------------------------------------------------------
    println!();
    println!("case 3: does the operand's ulp bound bound the RESULT's ulp error?");
    println!(
        "{:>10} {:>6} {:>14} {:>18} {:>18} {:>10}",
        "op", "drop", "pairs", "operand bound", "max result err", "meet tight"
    );
    for (opname, is_mul) in [("add", false), ("mul", true)] {
        for drop in [1u32, 2, 4, 8] {
            let bound = (1i64 << drop) - 1;
            let mut worst: i64 = 0;
            let mut total: u64 = 0;
            let mut a_raw = 0i64;
            while a_raw < N {
                let a = lossy(a_raw, drop);
                let mut b = 0i64;
                while b < N {
                    total += 1;
                    let (y, y_exact) = if is_mul {
                        (
                            ((a as i128 * b as i128) >> F) as i64,
                            ((a_raw as i128 * b as i128) >> F) as i64,
                        )
                    } else {
                        (a + b, a_raw + b)
                    };
                    let d = (y - y_exact).abs();
                    if d > worst {
                        worst = d;
                    }
                    b += 271;
                }
                a_raw += 53;
            }
            println!(
                "{:>10} {:>6} {:>14} {:>18} {:>18} {:>10}",
                opname,
                drop,
                total,
                bound,
                worst,
                if worst <= bound { "yes" } else { "NO" }
            );
        }
    }

    println!();
    println!("reading of case 3: an ulp bound survives addition and does not survive");
    println!("multiplication, because the operand's error is scaled by the other");
    println!("operand. A lattice over guarantees stated in ulps is therefore not");
    println!("closed under the operations the type supports.");
}
