// Probe 2: roundToIntegralExact is quantise-at-a-pinned-quantum plus one extra
// resolution the pinned form is denied: exponent escape.
//
// File 80 established (exhaustively, at the cohort-carrying decimal model):
// roundToIntegralExact is total over D and decomposes into a value-keyed law
// (round to integer) plus a datum-keyed exponent selection; quantize is
// pair-keyed as a whole and can never be a law. File 84 established that
// lifting the quantum to type position makes quantise::<Q> value-keyed and
// law-eligible. Nobody connected the two.
//
// Hypothesis: rTIE's value function IS quantise::<0>'s value function, and
// rTIE's totality is bought exactly where quantise::<0> refuses, by raising
// the result exponent (the standard's preferred-exponent freedom), a
// resolution unavailable once the quantum is pinned. If the refusal set of
// the pinned form equals the escape set of the free form, cell for cell, the
// standard's carve-out and rTIE's totality are one fact seen from two sides.
//
// Model: file 80's shape. r = 10, p = 3, exponents -2..=1, unsigned mantissas
// 0..1000, |D| = 4000. Exact i64 arithmetic throughout, values scaled by 10^2
// so every model value is an integer. Run probe; every count asserted.

const P: i64 = 1000; // 10^p
const SCALE: i64 = 100; // 10^2, makes every value at e >= -2 an integer

fn pow10(e: i64) -> i64 {
    let mut r = 1i64;
    let mut i = 0;
    while i < e {
        r *= 10;
        i += 1;
    }
    r
}

/// Scaled value of datum (m, e): m * 10^e * SCALE, exact.
fn val(m: i64, e: i64) -> i64 {
    m * pow10(e + 2)
}

/// Round scaled value to the nearest multiple of `q_scaled`, ties to even.
fn round_half_even(v: i64, q: i64) -> i64 {
    let quot = v / q;
    let rem = v % q;
    let twice = 2 * rem;
    if twice > q || (twice == q && quot % 2 != 0) {
        quot + 1
    } else {
        quot
    }
}

fn main() {
    let mut cells = 0u32;
    let mut agree = 0u32;
    let mut refusals = 0u32;
    let mut escapes = 0u32;

    for e in -2i64..=1 {
        for m in 0i64..1000 {
            cells += 1;
            let v = val(m, e);

            // quantise::<0>: result value at quantum 10^0, mantissa within p
            // digits or refuse. OverRange on At<N, 0> per file 84 section 2.
            let m0 = round_half_even(v, SCALE); // integer result value
            let pinned = if m0.abs() < P { Some(m0) } else { None };

            // roundToIntegralExact on N: same value function (round to
            // integer, half even), then EXPRESS the result in N, using
            // whatever exponent it takes. Representable iff some e' in
            // -2..=1 and |m'| < 1000 give m' * 10^e' == m0.
            let rtie_value = m0;
            let mut representable = false;
            let mut needs_positive_exp = true;
            for ep in -2i64..=1 {
                let q = pow10(ep + 2) / SCALE; // 10^ep as a scale on integers
                                               // m0 = m' * 10^ep requires 10^ep | m0 when ep >= 0; for
                                               // ep < 0, m' = m0 * 10^-ep always integral.
                let mprime = if ep >= 0 {
                    if q != 0 && rtie_value % q == 0 {
                        Some(rtie_value / q)
                    } else {
                        None
                    }
                } else {
                    Some(rtie_value * (SCALE / pow10(ep + 2)))
                };
                if let Some(mp) = mprime {
                    if mp.abs() < P {
                        representable = true;
                        if ep <= 0 {
                            needs_positive_exp = false;
                        }
                    }
                }
            }
            assert!(representable, "rTIE total on the model: m={m} e={e}");

            match pinned {
                Some(pv) => {
                    // Where the pinned form delivers, the two value
                    // functions agree exactly.
                    assert!(pv == rtie_value, "value split at m={m} e={e}");
                    agree += 1;
                    // And no escape was needed: exponent 0 suffices.
                    assert!(!needs_positive_exp || pv == 0);
                }
                None => {
                    refusals += 1;
                    // Where the pinned form refuses, rTIE survives only by
                    // exponent escape: the integer needs e' > 0 to fit in
                    // p digits.
                    assert!(needs_positive_exp, "refusal without escape at m={m} e={e}");
                    escapes += 1;
                }
            }
        }
    }

    assert!(cells == 4000);
    assert!(refusals == 900); // e = 1, m in 100..=999: value >= 1000
    assert!(agree == 3100);
    assert!(escapes == refusals); // cell for cell, not merely in total
    println!("cells={cells} agree={agree} refusals={refusals} escapes={escapes}");
    println!("OK: rTIE = pinned quantise + exponent escape, cell for cell");
}
