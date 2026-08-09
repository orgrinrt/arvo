// Probe 5: the parse and print kernels are const-callable as written.
// The probe re-declares probe 1's quantiser and probe 2's sig-digit rounding
// as `const fn`, evaluates both in const context, and asserts known values,
// so "everything wants a const-callable form" is a compiled fact rather than
// an intention. No feature gates.

#![no_std]

const P: u32 = 8;
const EMIN: i32 = -4;
const EMAX: i32 = 4;

const fn quantise_rne(num: u128, den: u128) -> (u32, i32) {
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

const fn pow10(t: u32) -> u128 {
    let mut p: u128 = 1;
    let mut i = 0;
    while i < t {
        p *= 10;
        i += 1;
    }
    p
}

const fn round_to_sig_digits(m: u32, e: i32, k: u32) -> (u128, i32) {
    let shift = (7 - e) as u32;
    let num = m as u128;
    let den = 1u128 << shift;
    let mut d10: i32 = -3;
    loop {
        let t = d10 + 1;
        let ge = if t >= 0 {
            num >= den * pow10(t as u32)
        } else {
            num * pow10((-t) as u32) >= den
        };
        if !ge {
            break;
        }
        d10 += 1;
    }
    let t = d10 - k as i32 + 1;
    let (dnum, dden) = if t >= 0 {
        (num, den * pow10(t as u32))
    } else {
        (num * pow10((-t) as u32), den)
    };
    let q = dnum / dden;
    let r = dnum % dden;
    let mut digits = q;
    let twice = r * 2;
    if twice > dden || (twice == dden && digits % 2 == 1) {
        digits += 1;
    }
    if digits >= pow10(k) {
        return (digits / 10, t + 1);
    }
    (digits, t)
}

// parse "0.1" (num = 1, den = 10) at const time: 0.1 * 2^11 = 204.8 in binade
// e = -4 (0.1 in [0.0625, 0.125)); m_exact = 0.1 * 2^11 = 204.8 -> RNE 205.
const PARSED: (u32, i32) = quantise_rne(1, 10);
const _: () = assert!(PARSED.0 == 205 && PARSED.1 == -4);

// print m=205 e=-4 (value 205/2048 = 0.10009765625) to 3 sig digits: 100e-3.
const PRINTED: (u128, i32) = round_to_sig_digits(205, -4, 3);
const _: () = assert!(PRINTED.0 == 100 && PRINTED.1 == -3);

// and the round trip closes at const time: parse(0.100) = (205, -4).
const REPARSED: (u32, i32) = quantise_rne(100, 1000);
const _: () = assert!(REPARSED.0 == 205 && REPARSED.1 == -4);
