//! p7 part one. The EXACT error coordinate for the corpus's answer-differing
//! arms, emitted for the bootstrap in `p7_the_constraint_on_real_arms.py`.
//!
//! `102`'s p5 states a constraint on the mechanism: a weighting may include a
//! measured coordinate only where every arm it ranges over computes the same
//! answer, because otherwise a bench rerun changes the program's output. Its
//! own demonstration of that constraint is explicitly synthetic on the time
//! column, and it says why: "no committed family has arms that disagree".
//!
//! p2, p3, p4 and p6 establish that premise is false. So the constraint can be
//! tested on real committed arms instead of a constructed table, which is a
//! strictly stronger test of the same claim, and that is what this probe is
//! for. Nothing here is synthetic: the error column is computed exactly, and
//! the time column, supplied by the Python half, is the committed CSV samples.
//!
//! What this half emits, in a form the Python half parses:
//!
//! For each arm of `decimal-quantiser-radix-sweep`, the mean relative error
//! against the exact sum, over the controlled input band where both radices
//! denote the identical real number (operands in `[10^6, 10^7)` at `exp = 0`,
//! which is p3's setting). That band is the only one on which a cross-arm
//! error is defined at all, because on the family's own committed band the two
//! arms are handed triples denoting different reals. That substitution is a
//! real limitation of the result and is stated in the deliverable rather than
//! buried here.
//!
//! For each arm of `quantiser-vs-fadd-subnormal-sweep`, the same quantity over
//! that family's own committed input distribution at every committed `PCT`,
//! which needs no controlling because both arms consume identical `f32` values.
//!
//! Error is computed in exact rational arithmetic (radix family) or in `f64`
//! where `f64` is exact for the quantity (fadd family: the sum of two `f32`
//! values in the generated bands is exactly representable in `f64`, since 24
//! significand bits plus a carry fit inside 53). It is converted to a printed
//! decimal only at the end, so the probe's own arithmetic cannot manufacture a
//! difference between two arms that in fact agree.

use bench_quantiser_fadd_shared::{hardware_add, software_add, AddSweep, N as FN};
use bench_quantiser_radix_shared::rmodel::Scaled;
use mockspace_bench_core::Routine;
use bench_quantiser_radix_shared::{
    quantised_add, BIN_EMAX, BIN_EMIN, BIN_P, BIN_R, DEC_EMAX, DEC_EMIN, DEC_P, DEC_R,
};

struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 { 1 } else { a }
}

/// `|mag * r^exp - exact| / exact`, computed as an exact rational and returned
/// as the pair (numerator, denominator) reduced, so the caller decides when to
/// leave exact arithmetic.
fn rel_err(mag: u64, exp: i32, r: i128, exact: i128) -> (i128, i128) {
    // value = mag * r^exp = vn / vd
    let m = mag as i128;
    let (vn, vd) = if exp >= 0 {
        let mut acc: i128 = 1;
        for _ in 0 .. exp {
            acc = acc.checked_mul(r).expect("overflow");
        }
        (m.checked_mul(acc).expect("overflow"), 1i128)
    } else {
        let mut acc: i128 = 1;
        for _ in 0 .. (-exp) {
            acc = acc.checked_mul(r).expect("overflow");
        }
        (m, acc)
    };
    // |vn/vd - exact| / exact = |vn - exact*vd| / (vd * exact)
    let num = (vn - exact.checked_mul(vd).expect("overflow")).abs();
    let den = vd.checked_mul(exact).expect("overflow");
    let g = gcd(num, den);
    (num / g, den / g)
}

fn radix_errors(trials: usize) -> (f64, f64) {
    let mut rng = SplitMix64(0xC0FF_EE12_3456_789A);
    let mut sum_b = 0.0f64;
    let mut sum_d = 0.0f64;

    for _ in 0 .. trials {
        let ma = 1_000_000 + (rng.next() % 9_000_000);
        let mb = 1_000_000 + (rng.next() % 9_000_000);
        let a = Scaled {
            neg: false,
            mag:  ma as u128,
            scale: 0,
        };
        let b = Scaled {
            neg: false,
            mag:  mb as u128,
            scale: 0,
        };
        let exact = ma as i128 + mb as i128;

        let (bm, be, _) = quantised_add::<BIN_R, BIN_P, BIN_EMIN, BIN_EMAX>(a, b);
        let (dm, de, _) = quantised_add::<DEC_R, DEC_P, DEC_EMIN, DEC_EMAX>(a, b);

        let (n1, d1) = rel_err(bm, be, 2, exact);
        let (n2, d2) = rel_err(dm, de, 10, exact);
        sum_b += n1 as f64 / d1 as f64;
        sum_d += n2 as f64 / d2 as f64;
    }
    (sum_b / trials as f64, sum_d / trials as f64)
}

fn fadd_errors<const PCT: usize>(seeds: u64) -> (f64, f64) {
    let mut sum_h = 0.0f64;
    let mut sum_s = 0.0f64;
    let mut counted = 0u64;
    for seed in 0 .. seeds {
        let input = AddSweep::<PCT>::build_input(seed);
        for i in 0 .. FN {
            let (a, b) = (input.a[i], input.b[i]);
            // exact: the sum of two f32 values fits a f64 significand exactly
            // for every band this Routine generates.
            let exact = a as f64 + b as f64;
            if exact == 0.0 {
                continue;
            }
            let hw = hardware_add(a, b) as f64;
            let sw = software_add(a, b) as f64;
            sum_h += ((hw - exact) / exact).abs();
            sum_s += ((sw - exact) / exact).abs();
            counted += 1;
        }
    }
    (sum_h / counted as f64, sum_s / counted as f64)
}

fn main() {
    println!("# p7 part one: exact error coordinates for the answer-differing arms");
    println!("# format: REGION<TAB>ARM<TAB>mean_relative_error");
    println!();

    let (eb, ed) = radix_errors(200_000);
    println!("# decimal-quantiser-radix-sweep");
    println!("# controlled band: both operands (mag in [10^6,10^7), exp = 0), so");
    println!("# both radices denote the identical real. 200000 trials, exact rational.");
    println!("decimal-quantiser-radix-sweep\tquantiser-radix2\t{eb:.12e}");
    println!("decimal-quantiser-radix-sweep\tquantiser-radix10\t{ed:.12e}");
    println!();

    println!("# quantiser-vs-fadd-subnormal-sweep");
    println!("# the family's own committed input distribution, 64 seeds per PCT.");
    macro_rules! fadd {
        ($p:literal) => {{
            let (h, s) = fadd_errors::<$p>(64);
            println!("quantiser-vs-fadd-subnormal-sweep-n{}\tquantiser-hardware\t{h:.12e}", $p);
            println!("quantiser-vs-fadd-subnormal-sweep-n{}\tquantiser-software\t{s:.12e}", $p);
        }};
    }
    fadd!(0);
    fadd!(10);
    fadd!(25);
    fadd!(50);
    fadd!(75);
    fadd!(100);
}
