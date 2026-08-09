// Probe 2: shortest round-trip printing over the model numeral, exhaustive.
//
// Same model as probe 1: radix 2, p = 8, m in [128, 255], e in [-4, 4],
// value = m * 2^(e-7). 1152 data, the whole matrix, not a sample.
//
// For every datum x: find the smallest k such that rounding x to k significant
// decimal digits (correctly, by exact integer arithmetic) and parsing the result
// back (probe 1's single-rounding quantiser) returns exactly x.
//
// Claims:
//   (a) every datum round-trips at k <= H where H = ceil(p * log10(2)) + 1 = 4
//       (Matula's bound, checked here rather than trusted);
//   (b) the probe reports the measured max k and a witness needing it, so the
//       spec's tightness sentence is a measurement, not a guess;
//   (c) the shortest print is computed from the exact rational alone: the only
//       inputs are (m, e) and integer arithmetic. No table, no float.

const P: u32 = 8;
const EMIN: i32 = -4;
const EMAX: i32 = 4;

fn quantise_rne(num: u128, den: u128) -> (u32, i32) {
    let mut e = EMIN;
    loop {
        let fits = if e + 1 >= 0 {
            num < den << ((e + 1) as u32)
        } else {
            num << ((-(e + 1)) as u32) < den
        };
        if fits || e == EMAX {
            break;
        }
        e += 1;
    }
    let shift = (P as i32 - 1 - e) as u32;
    let scaled = num << shift;
    let q = scaled / den;
    let r = scaled % den;
    let mut m = q as u32;
    let twice = r * 2;
    if twice > den || (twice == den && m % 2 == 1) {
        m += 1;
    }
    if m == (1u32 << P) && e < EMAX {
        return (1u32 << (P - 1), e + 1);
    }
    if m == (1u32 << P) {
        return ((1u32 << P) - 1, e);
    }
    (m, e)
}

// Round value m * 2^(e-7) to k significant decimal digits, exactly.
// Returns (digits, dexp) meaning digits * 10^dexp, with 10^(k-1) <= digits < 10^k.
fn round_to_sig_digits(m: u32, e: i32, k: u32) -> (u128, i32) {
    // value = m / 2^(7-e), with 7-e in [3, 11] for our ranges.
    let shift = (7 - e) as u32;
    let num = m as u128;
    let den = 1u128 << shift;
    // find d10 = floor(log10(value)): largest t with value >= 10^t
    let ge_pow10 = |t: i32| -> bool {
        if t >= 0 {
            num >= den * 10u128.pow(t as u32)
        } else {
            num * 10u128.pow((-t) as u32) >= den
        }
    };
    let mut d10: i32 = -3; // value >= 0.0625 > 10^-3
    while ge_pow10(d10 + 1) {
        d10 += 1;
    }
    // digits = round(value / 10^(d10 - k + 1)) with RNE, exactly.
    let t = d10 - k as i32 + 1;
    // digits = num * 10^(-t) / den   (t may be negative -> multiply num)
    let (dnum, dden) = if t >= 0 {
        (num, den * 10u128.pow(t as u32))
    } else {
        (num * 10u128.pow((-t) as u32), den)
    };
    let q = dnum / dden;
    let r = dnum % dden;
    let mut digits = q;
    let twice = r * 2;
    if twice > dden || (twice == dden && digits % 2 == 1) {
        digits += 1;
    }
    // rounding may carry to k+1 digits (e.g. 999 -> 1000): renormalise
    if digits >= 10u128.pow(k) {
        return (digits / 10, t + 1);
    }
    (digits, t)
}

fn main() {
    let mut max_k = 0u32;
    let mut max_witness = (0u32, 0i32);
    let mut count_at: [u32; 8] = [0; 8];
    let mut total = 0u32;

    for e in EMIN..=EMAX {
        for m in 128u32..=255 {
            let mut needed = 0u32;
            for k in 1..=6u32 {
                let (digits, dexp) = round_to_sig_digits(m, e, k);
                // parse digits * 10^dexp back through the quantiser
                let (num, den) = if dexp >= 0 {
                    (digits * 10u128.pow(dexp as u32), 1u128)
                } else {
                    (digits, 10u128.pow((-dexp) as u32))
                };
                let (pm, pe) = quantise_rne(num, den);
                if (pm, pe) == (m, e) {
                    needed = k;
                    break;
                }
            }
            assert!(
                needed != 0,
                "datum m={m} e={e} failed to round-trip within 6 digits"
            );
            assert!(
                needed <= 4,
                "datum m={m} e={e} needed {needed} digits, above the H bound of 4"
            );
            if needed > max_k {
                max_k = needed;
                max_witness = (m, e);
            }
            count_at[needed as usize] += 1;
            total += 1;
        }
    }

    println!(
        "all {total} data round-trip; digit-count distribution k=1..6: {:?}; \
         measured max k = {max_k}, witness m={} e={}",
        &count_at[1..7],
        max_witness.0,
        max_witness.1
    );
}
