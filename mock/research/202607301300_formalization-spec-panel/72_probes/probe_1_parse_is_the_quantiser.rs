// Probe 1: parsing a digit string is the quantiser applied once to the exact rational.
//
// Model numeral: radix 2, precision p = 8, normalised significand m in [128, 255],
// exponent e in [-4, 4], value = m * 2^(e-7), non-negative domain, no specials.
// 1152 representable values plus zero.
//
// Claims exercised, each over the whole in-range decimal grid (318,126 strings,
// every four-decimal-place string in [0.0625, 31.8750]), not a sample:
//
//   (a) scaled-integer single rounding from the exact rational (the quantiser's own
//       remainder-comparison RNE kernel) equals brute-force nearest-representable
//       with ties-to-even, on every string. parse = quantise(exact rational), one map.
//   (b) staging the parse through a wider intermediate (p = 12) with RNE at both
//       steps DIFFERS from the direct parse on real inputs (double rounding).
//       The probe searches for witnesses and asserts it finds them.
//   (c) the identical staging with round-to-odd at the intermediate step agrees with
//       the direct parse on every string (the ToOdd cure; p_mid = 12 >= p + 2 = 10).
//
// (b)+(c) is the file-01 observation ("round-to-odd IS expressible and is the
// classical cure for double rounding") given its first concrete job in this review.

const P: u32 = 8; // target precision
const EMIN: i32 = -4;
const EMAX: i32 = 4;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Val {
    m: u32, // significand, in [1<<(P-1), (1<<P)-1]
    e: i32, // exponent, value = m * 2^(e - (P-1))
}

// exact value of (m, e, p) as a rational num/2^k against num2/den comparison:
// v = m * 2^(e - (p-1)). All comparisons below are exact integer arithmetic.

// Brute-force nearest with ties-to-even over the whole representable set.
fn nearest_bruteforce(num: u128, den: u128) -> Val {
    let mut best: Option<(Val, (u128, u128))> = None; // (candidate, |v*den*scale - num*scale| as exact pair)
    for e in EMIN..=EMAX {
        for m in (1u32 << (P - 1))..=((1u32 << P) - 1) {
            // candidate value m * 2^(e-7); compare |num/den - m*2^(e-7)| exactly.
            // diff = |num * 2^(7-e) - m * den| / (den * 2^(7-e)); shift = 7 - e in [3, 11].
            let shift = (P as i32 - 1 - e) as u32;
            let lhs = num << shift; // num up to ~3.2e5 * 2^11: fits u128 easily
            let rhs = (m as u128) * den;
            let diff = if lhs > rhs { lhs - rhs } else { rhs - lhs };
            let cand = Val { m, e };
            match &best {
                None => best = Some((cand, (diff, 1))),
                Some((bv, (bd, _))) => {
                    // all candidates share the same denominator per e; to compare
                    // across e, normalise: diff_e / 2^(7-e): compare diff * 2^(e)
                    // Use exact cross-multiplied comparison at max shift 11:
                    let cur = diff << (e - EMIN) as u32; // common denom 2^(7-EMIN)
                    let bst = bd << (bv.e - EMIN) as u32;
                    if cur < bst || (cur == bst && cand.m % 2 == 0 && bv.m % 2 == 1) {
                        best = Some((cand, (diff, 1)));
                    }
                }
            }
        }
    }
    best.unwrap().0
}

// The quantiser: single RNE rounding from the exact rational, remainder comparison.
// Generic in precision so the same kernel serves the p = 12 intermediate.
fn quantise_rne(num: u128, den: u128, p: u32, emin: i32, emax: i32) -> (u32, i32) {
    // find e with m_exact = num * 2^(p-1-e) / den in [2^(p-1), 2^p)
    let mut e = emin;
    loop {
        // upper bound of binade e: value < 2^(e+1) equiv num < den * 2^(e+1)
        // handle negative e via shifting num instead
        let fits = if e + 1 >= 0 {
            num < den << ((e + 1) as u32)
        } else {
            num << ((-(e + 1)) as u32) < den
        };
        if fits || e == emax {
            break;
        }
        e += 1;
    }
    // m_exact = num * 2^(p-1-e) / den, rounded RNE
    let shift = (p as i32 - 1 - e) as u32; // p-1-e >= 0 for our ranges
    let scaled = num << shift;
    let q = scaled / den;
    let r = scaled % den;
    let mut m = q as u32;
    let twice = r * 2;
    if twice > den || (twice == den && m % 2 == 1) {
        m += 1;
    }
    // rounding may carry into the next binade
    if m == (1u32 << p) && e < emax {
        return (1u32 << (p - 1), e + 1);
    }
    if m == (1u32 << p) {
        // at emax the carry would overflow the range; out of scope for this probe
        // (range resolution is the preset's Resolution pair); clamp for safety.
        return ((1u32 << p) - 1, e);
    }
    (m, e)
}

// Round-to-odd at precision p: truncate, then set the low bit if inexact.
fn quantise_to_odd(num: u128, den: u128, p: u32, emin: i32, emax: i32) -> (u32, i32) {
    let mut e = emin;
    loop {
        let fits = if e + 1 >= 0 {
            num < den << ((e + 1) as u32)
        } else {
            num << ((-(e + 1)) as u32) < den
        };
        if fits || e == emax {
            break;
        }
        e += 1;
    }
    let shift = (p as i32 - 1 - e) as u32;
    let scaled = num << shift;
    let q = scaled / den;
    let r = scaled % den;
    let mut m = q as u32;
    if r != 0 {
        m |= 1;
    }
    (m, e)
}

fn main() {
    let den: u128 = 10_000;
    let lo: u128 = 625; // 0.0625
    let hi: u128 = 318_750; // 31.8750
    let mut checked = 0u64;
    let mut double_round_witnesses = 0u64;
    let mut first_witness: Option<(u128, Val, Val)> = None;

    for num in lo..=hi {
        // (a) quantiser == brute force
        let (m, e) = quantise_rne(num, den, P, EMIN, EMAX);
        let direct = Val { m, e };
        let brute = nearest_bruteforce(num, den);
        assert_eq!(
            direct, brute,
            "single-rounding quantiser disagrees with nearest-RNE at num={num}"
        );

        // (b) staged RNE at p=12 then RNE at p=8
        let (m12, e12) = quantise_rne(num, den, 12, EMIN, EMAX);
        // re-express intermediate as exact rational: m12 * 2^(e12 - 11)
        let (snum, sden) = if e12 - 11 >= 0 {
            ((m12 as u128) << ((e12 - 11) as u32), 1u128)
        } else {
            (m12 as u128, 1u128 << ((11 - e12) as u32))
        };
        let (ms, es) = quantise_rne(snum, sden, P, EMIN, EMAX);
        let staged = Val { m: ms, e: es };
        if staged != direct {
            double_round_witnesses += 1;
            if first_witness.is_none() {
                first_witness = Some((num, direct, staged));
            }
        }

        // (c) staged with round-to-odd at p=12, then RNE at p=8
        let (mo, eo) = quantise_to_odd(num, den, 12, EMIN, EMAX);
        let (onum, oden) = if eo - 11 >= 0 {
            ((mo as u128) << ((eo - 11) as u32), 1u128)
        } else {
            (mo as u128, 1u128 << ((11 - eo) as u32))
        };
        let (mc, ec) = quantise_rne(onum, oden, P, EMIN, EMAX);
        assert_eq!(
            Val { m: mc, e: ec },
            direct,
            "ToOdd-staged parse diverged from direct parse at num={num}"
        );

        checked += 1;
    }

    assert!(
        double_round_witnesses > 0,
        "expected RNE-staged double rounding to differ somewhere; it never did"
    );
    let (wn, wd, ws) = first_witness.unwrap();
    println!(
        "checked {checked} strings; quantiser==bruteforce on all; \
         RNE-staged double rounding differs on {double_round_witnesses} \
         (first witness: {}.{:04} -> direct m={} e={}, staged m={} e={}); \
         ToOdd staging agrees on all",
        wn / den,
        wn % den,
        wd.m,
        wd.e,
        ws.m,
        ws.e
    );
}
