//! p2. Do the two committed arms of the `quantiser-radix` family compute one
//! value?
//!
//! `102` claims every committed region in `mock/benches/` is answer-equivalent,
//! meaning all arms compute one value, so every number the repository holds
//! compares cost at a fixed answer. This probe tests that claim at the one
//! family where the arms are two different number formats.
//!
//! The instrument is execution, not source reading. Both arms are public
//! functions over the identical `Operands` type, and the family's own
//! `Routine::build_input` produces the operand stream both arms are fed in the
//! committed bench. So: build the input the harness builds, run both arms on
//! it, and diff the outputs two ways.
//!
//! Two senses of "same answer" are reported separately, because they are not
//! the same question and only one of them is interesting.
//!
//! **Representation identity.** Whether the emitted `(mag, exp)` pairs are
//! equal. This is expected to fail trivially, since a base-two significand and
//! a base-ten significand denote the same quantity with different digits, and
//! failing it proves nothing.
//!
//! **Denoted-value identity.** Whether `mag * R^exp` is the same rational
//! number under each arm's own radix. This is the question that matters. If the
//! denoted values agree, the two arms are answer-equivalent and only their
//! representations differ. If they disagree, the family holds two arms that
//! round the same exact sum to different results, which is an accuracy
//! difference standing in the committed corpus.
//!
//! The comparison is done in exact rational arithmetic over `i128`
//! numerator/denominator with `u128` powers, so no floating point of the
//! probe's own can manufacture or hide a difference.

use bench_quantiser_radix_shared::{run_binary32, run_decimal32, Operands, RadixAdd, Results, N};
use mockspace_bench_core::Routine;

/// Exact rational, kept unreduced except by gcd on construction. `mag * R^exp`
/// with a possibly negative `exp` becomes `mag / R^-exp`.
#[derive(Clone, Copy, Debug)]
struct Q {
    num: i128,
    den: i128,
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 {
        1
    } else {
        a
    }
}

impl Q {
    /// `mag * r^exp` as an exact rational. Returns `None` when the scaling
    /// would overflow `i128`, so an overflow can never be silently read as an
    /// agreement or a disagreement.
    fn from_scaled(mag: u64, exp: i32, r: u32) -> Option<Q> {
        let m = mag as i128;
        let r = r as i128;
        let (num, den) = if exp >= 0 {
            let mut acc: i128 = 1;
            for _ in 0..exp {
                acc = acc.checked_mul(r)?;
            }
            (m.checked_mul(acc)?, 1i128)
        } else {
            let mut acc: i128 = 1;
            for _ in 0..(-exp) {
                acc = acc.checked_mul(r)?;
            }
            (m, acc)
        };
        let g = gcd(num, den);
        Some(Q {
            num: num / g,
            den: den / g,
        })
    }

    fn eq_exact(self, other: Q) -> bool {
        // cross-multiply in i128; both sides are already reduced and the
        // magnitudes here are far below the overflow bound, but check anyway.
        match (
            self.num.checked_mul(other.den),
            other.num.checked_mul(self.den),
        ) {
            (Some(l), Some(r)) => l == r,
            _ => panic!("cross-multiplication overflowed; comparison is not decidable here"),
        }
    }
}

fn run_spread<const SPREAD: usize>(seeds: &[u64]) -> Report {
    let mut rep = Report::default();
    for &seed in seeds {
        let input: Operands = RadixAdd::<SPREAD>::build_input(seed);
        let mut bin = Results::default();
        let mut dec = Results::default();
        run_binary32(&input, &mut bin);
        run_decimal32(&input, &mut dec);

        for i in 0..N {
            rep.lanes += 1;

            if bin.mag[i] == dec.mag[i] && bin.exp[i] == dec.exp[i] {
                rep.repr_equal += 1;
            }

            let qb = Q::from_scaled(bin.mag[i], bin.exp[i], 2);
            let qd = Q::from_scaled(dec.mag[i], dec.exp[i], 10);
            match (qb, qd) {
                (Some(a), Some(b)) => {
                    if a.eq_exact(b) {
                        rep.value_equal += 1;
                    } else {
                        rep.value_differ += 1;
                        if rep.samples.len() < 4 {
                            rep.samples.push((
                                seed,
                                i,
                                (bin.mag[i], bin.exp[i]),
                                (dec.mag[i], dec.exp[i]),
                                (a.num, a.den),
                                (b.num, b.den),
                            ));
                        }
                    }
                }
                _ => rep.undecidable += 1,
            }

            if bin.flags[i] != dec.flags[i] {
                rep.flags_differ += 1;
            }
        }
    }
    rep
}

#[derive(Default)]
struct Report {
    lanes: usize,
    repr_equal: usize,
    value_equal: usize,
    value_differ: usize,
    undecidable: usize,
    flags_differ: usize,
    #[allow(clippy::type_complexity)]
    samples: Vec<(
        u64,
        usize,
        (u64, i32),
        (u64, i32),
        (i128, i128),
        (i128, i128),
    )>,
}

fn report<const SPREAD: usize>(seeds: &[u64]) {
    let r = run_spread::<SPREAD>(seeds);
    println!("--- SPREAD = {SPREAD} (a committed size for this family) ---");
    println!("  lanes compared            : {}", r.lanes);
    println!(
        "  representation identical  : {} of {}",
        r.repr_equal, r.lanes
    );
    println!(
        "  denoted value identical   : {} of {}",
        r.value_equal, r.lanes
    );
    println!(
        "  denoted value DIFFERENT   : {} of {}  ({:.2}%)",
        r.value_differ,
        r.lanes,
        100.0 * r.value_differ as f64 / r.lanes as f64
    );
    println!("  undecidable (overflow)    : {}", r.undecidable);
    println!("  flag words differing      : {}", r.flags_differ);
    for (seed, i, b, d, qa, qb) in &r.samples {
        println!(
            "    seed {seed} lane {i}: binary32 {b:?} = {}/{}   decimal32 {d:?} = {}/{}",
            qa.0, qa.1, qb.0, qb.1
        );
    }
    println!();
}

fn main() {
    println!("p2. do the quantiser-radix arms compute one value?");
    println!();
    println!("arms   : quantiser-radix2 (run_binary32) and quantiser-radix10 (run_decimal32)");
    println!("input  : RadixAdd::<SPREAD>::build_input(seed), the family's own Routine");
    println!("sizes  : 0, 2, 8, 20, the four committed in bench.toml for this family");
    println!("compare: exact rational, i128 num/den, no float in the comparison path");
    println!();

    let seeds: Vec<u64> = (1u64..=8)
        .map(|k| k.wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .collect();

    report::<0>(&seeds);
    report::<2>(&seeds);
    report::<8>(&seeds);
    report::<20>(&seeds);

    println!("READING");
    println!("  Representation identity failing proves nothing: two radices spell");
    println!("  the same quantity with different digits.");
    println!("  Denoted-value identity is the question. A nonzero DIFFERENT count");
    println!("  means the two committed arms of this family round the same exact");
    println!("  sum to different results, so the family compares cost at TWO");
    println!("  answers rather than at a fixed one.");
}
