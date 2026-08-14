//! p3. Controlling the input, do the two committed `quantiser-radix` arms
//! round the same exact sum to different answers?
//!
//! p2 established that the two arms emit different denoted values on 97 to 99
//! percent of lanes across all four committed sizes. That result on its own is
//! open to a deflationary reading: the family's own `build_input` draws the
//! exponent as a **grid step** rather than an absolute magnitude, so the same
//! `(mag, exp)` triple denotes a different real number under each radix, and
//! the arms are therefore not being fed the same value. On that reading the
//! arms disagree because their inputs disagree, which is a different and less
//! interesting fact than an accuracy difference.
//!
//! This probe removes that confound entirely. Both arms are fed operands with
//! `exp = 0`, where `mag` is drawn from the family's own `[10^6, 10^7)` band.
//! At `exp = 0` the triple denotes the integer `mag` under **either** radix, so
//! the two arms receive the identical pair of exact real numbers and the exact
//! sum they must round is the same integer. Any difference in the emitted
//! result is then rounding and nothing else.
//!
//! What remains is a clean accuracy question: `binary32` carries 24 binary
//! digits and `decimal32` carries 7 decimal digits, and `2^24 = 16777216` while
//! `10^7 = 10000000`, so the two formats have genuinely different precision and
//! the sum of two operands in `[10^6, 10^7)` lands above both thresholds often
//! enough for the difference to show. Whichever way the counts fall, they are a
//! fact about whether the committed corpus holds arms that disagree on the
//! answer for one input.
//!
//! Comparison is exact rational arithmetic in `i128`, so the probe's own
//! arithmetic cannot manufacture or hide a difference. Error magnitudes are
//! reported as an exact rational compared by cross-multiplication, never as a
//! float.

use bench_quantiser_radix_shared::rmodel::Scaled;
use bench_quantiser_radix_shared::{
    quantised_add, BIN_EMAX, BIN_EMIN, BIN_P, BIN_R, DEC_EMAX, DEC_EMIN, DEC_P, DEC_R,
};

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

/// Exact rational `num/den`, reduced, `den > 0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Q {
    num: i128,
    den: i128,
}

impl Q {
    fn new(num: i128, den: i128) -> Q {
        let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
        let g = gcd(num, den);
        Q {
            num: num / g,
            den: den / g,
        }
    }

    fn int(n: i128) -> Q {
        Q { num: n, den: 1 }
    }

    /// `mag * r^exp` exactly. Panics rather than wrapping, so an overflow is
    /// never silently read as agreement.
    fn from_scaled(mag: u64, exp: i32, r: i128) -> Q {
        let m = mag as i128;
        if exp >= 0 {
            let mut acc: i128 = 1;
            for _ in 0..exp {
                acc = acc.checked_mul(r).expect("scale overflowed i128");
            }
            Q::new(m.checked_mul(acc).expect("scale overflowed i128"), 1)
        } else {
            let mut acc: i128 = 1;
            for _ in 0..(-exp) {
                acc = acc.checked_mul(r).expect("scale overflowed i128");
            }
            Q::new(m, acc)
        }
    }

    fn sub(self, o: Q) -> Q {
        Q::new(
            self.num.checked_mul(o.den).expect("sub overflow") - o.num.checked_mul(self.den).expect("sub overflow"),
            self.den.checked_mul(o.den).expect("sub overflow"),
        )
    }

    fn abs(self) -> Q {
        Q {
            num: self.num.abs(),
            den: self.den,
        }
    }

    /// self > o, by cross-multiplication. Both denominators are positive.
    fn gt(self, o: Q) -> bool {
        self.num.checked_mul(o.den).expect("cmp overflow") > o.num.checked_mul(self.den).expect("cmp overflow")
    }
}

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

fn main() {
    println!("p3. same exact input, do the two committed radix arms answer differently?");
    println!();
    println!("arms  : quantised_add at <R=2,P=24>  and  <R=10,P=7>, the exact");
    println!("        instantiations quantiser-radix2 and quantiser-radix10 run");
    println!("input : both operands (neg=false, mag in [10^6, 10^7), exp = 0)");
    println!("        at exp = 0 the triple denotes the integer `mag` under EITHER");
    println!("        radix, so both arms receive the identical pair of reals and");
    println!("        the exact sum they must round is the same integer");
    println!("cmp   : exact rational, i128, no float anywhere in the comparison");
    println!();

    let trials = 200_000usize;
    let mut rng = SplitMix64(0xC0FF_EE12_3456_789A);

    let mut agree = 0usize;
    let mut differ = 0usize;
    let mut bin_exact = 0usize;
    let mut dec_exact = 0usize;
    let mut bin_closer = 0usize;
    let mut dec_closer = 0usize;
    let mut tie_error = 0usize;
    let mut samples: Vec<(u64, u64, i128, (u64, i32), (u64, i32))> = Vec::new();

    for _ in 0..trials {
        let ma = 1_000_000 + (rng.next() % 9_000_000);
        let mb = 1_000_000 + (rng.next() % 9_000_000);

        let a = Scaled {
            neg: false,
            mag: ma as u128,
            scale: 0,
        };
        let b = Scaled {
            neg: false,
            mag: mb as u128,
            scale: 0,
        };

        let (bm, be, _bf) = quantised_add::<BIN_R, BIN_P, BIN_EMIN, BIN_EMAX>(a, b);
        let (dm, de, _df) = quantised_add::<DEC_R, DEC_P, DEC_EMIN, DEC_EMAX>(a, b);

        let exact = Q::int(ma as i128 + mb as i128);
        let qb = Q::from_scaled(bm, be, 2);
        let qd = Q::from_scaled(dm, de, 10);

        if qb == qd {
            agree += 1;
        } else {
            differ += 1;
            if samples.len() < 6 {
                samples.push((ma, mb, exact.num, (bm, be), (dm, de)));
            }
        }

        let eb = qb.sub(exact).abs();
        let ed = qd.sub(exact).abs();
        if eb == Q::int(0) {
            bin_exact += 1;
        }
        if ed == Q::int(0) {
            dec_exact += 1;
        }
        if ed.gt(eb) {
            bin_closer += 1;
        } else if eb.gt(ed) {
            dec_closer += 1;
        } else {
            tie_error += 1;
        }
    }

    println!("trials                        : {trials}");
    println!(
        "answers IDENTICAL             : {agree}  ({:.4}%)",
        100.0 * agree as f64 / trials as f64
    );
    println!(
        "answers DIFFERENT             : {differ}  ({:.4}%)",
        100.0 * differ as f64 / trials as f64
    );
    println!();
    println!("accuracy against the exact integer sum:");
    println!(
        "  binary32 exact              : {bin_exact}  ({:.4}%)",
        100.0 * bin_exact as f64 / trials as f64
    );
    println!(
        "  decimal32 exact             : {dec_exact}  ({:.4}%)",
        100.0 * dec_exact as f64 / trials as f64
    );
    println!("  binary32 strictly closer    : {bin_closer}");
    println!("  decimal32 strictly closer   : {dec_closer}");
    println!("  equal error                 : {tie_error}");
    println!();
    for (ma, mb, exact, b, d) in &samples {
        println!("  {ma} + {mb} = {exact} exactly;  binary32 -> {b:?},  decimal32 -> {d:?}");
    }
    println!();
    println!("READING");
    println!("  Both arms saw the identical pair of exact integers. Any nonzero");
    println!("  DIFFERENT count is therefore two committed arms of one committed");
    println!("  family returning different answers for one input, which is an");
    println!("  accuracy difference and not an input-interpretation artefact.");
    println!("  The `binary32 strictly closer` and `decimal32 strictly closer`");
    println!("  counts are the ranking a fidelity coordinate would record, and");
    println!("  no column of the committed CSV corpus carries it.");
}
