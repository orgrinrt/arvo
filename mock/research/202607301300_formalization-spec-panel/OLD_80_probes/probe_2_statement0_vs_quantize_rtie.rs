// Probe 2, file 80. Statement 0 against the two datum-dependent operations.
//
// The check flagged forward three times (66:497-498, 67:693-700, 78:943) and performed
// by nobody: does the crossing contract's precondition (statement 0: for every datum d,
// decode(d) is in V(N)) survive `quantize` and `roundToIntegralExact`, the two
// operations IEEE 754's clause 5.2 carves out as datum-dependent by definition
// (62_probes/primary_sources.md, the clause 5.2 extract)?
//
// The model is a decimal Ranged numeral, radix 10, p = 3, quantum exponents -2..=1,
// unnormalised significands (a cohort per value), datum set D = { (m, q) : 0 <= m <=
// 999, -2 <= q <= 1 }, decode(m, q) = m * 10^q, V = decode(D). Exact i128 arithmetic
// throughout, no floats, the same arithmetic discipline as 66_probes/model.rs.
//
// Four exhaustive checks and two witnesses:
//   (a) quantize closure: over all |D|^2 = 16,000,000 operand pairs, every result is
//       either a refusal or a datum of D (so statement 0 covers it). The refusal
//       branch is load-bearing: (b) counts the pairs where the naive, non-refusing
//       quantize would emit a mantissa >= 10^p, i.e. a non-datum.
//   (c) quantize is NOT value-level well-defined: a witness triple (x, y1, y2) with
//       value(y1) == value(y2) and value(quantize(x,y1)) != value(quantize(x,y2)).
//       This refutes the half of file 67's guess that the dependence lives "wholly
//       inside D" FOR QUANTIZE: a value-level output reads a datum-level input.
//   (d) roundToIntegralExact closure: over all of D, the result is always a datum of
//       D with zero refusals (the mantissa-fit argument is exhaustively confirmed).
//   (e) roundToIntegralExact IS fibre-preserving: over every value-equal operand pair,
//       result values are equal; only the result's cohort member (exponent) differs.
//       File 67's guess holds exactly for this operation.
//
// Rounding is half-even everywhere, matching the model's quantiser.
//
// Build: rustc --edition 2021 -O; run. Assertions, not printout, except the counts.

const R: i128 = 10;
const P: u32 = 3;
const QMIN: i32 = -2;
const QMAX: i32 = 1;

fn ipow(r: i128, k: u32) -> i128 {
    let mut acc: i128 = 1;
    for _ in 0..k {
        acc *= r;
    }
    acc
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Datum {
    m: i128, // 0..=999 (non-negative half; sign is orthogonal to every check here)
    q: i32,  // QMIN..=QMAX
}

/// decode(m, q) scaled to the common grid 10^QMIN, exact.
fn decode_at_qmin(d: Datum) -> i128 {
    d.m * ipow(R, (d.q - QMIN) as u32)
}

fn is_datum(m: i128, q: i32) -> bool {
    (0..ipow(R, P)).contains(&m) && (QMIN..=QMAX).contains(&q)
}

/// Round `v` (an exact value expressed at grid 10^QMIN) to the grid 10^qt,
/// half-even. Returns the mantissa at exponent qt. qt >= QMIN always here.
fn round_half_even_to(v: i128, qt: i32) -> i128 {
    let d = ipow(R, (qt - QMIN) as u32);
    let lo = v / d;
    let rem = v % d;
    let twice = 2 * rem;
    if twice > d || (twice == d && lo % 2 == 1) {
        lo + 1
    } else {
        lo
    }
}

/// IEEE quantize(x, y): value = x's value rounded to y's quantum; result exponent is
/// exactly Q(y) (clause 5.2: "For quantize and roundToIntegralExact, a finite result
/// has the preferred exponent, whether or not the result is exact"; preferred exponent
/// of quantize is Q(y)). Refuses (IEEE: invalid, NaN) when the rounded mantissa does
/// not fit in p digits.
fn quantize(x: Datum, y: Datum) -> Option<Datum> {
    let qt = y.q;
    let m = round_half_even_to(decode_at_qmin(x), qt);
    if m >= ipow(R, P) {
        None // the refusal branch
    } else {
        Some(Datum { m, q: qt })
    }
}

/// The naive quantize with no refusal branch: returns the raw (m, q) pair.
fn quantize_naive(x: Datum, y: Datum) -> (i128, i32) {
    (round_half_even_to(decode_at_qmin(x), y.q), y.q)
}

/// IEEE roundToIntegralExact(x): value = x's value rounded to an integer (half-even);
/// result exponent is the preferred exponent max(Q(x), 0).
fn round_to_integral_exact(x: Datum) -> Option<Datum> {
    let e = x.q.max(0);
    // the value rounded to the integer grid 10^0, then expressed at exponent e
    let int_m = round_half_even_to(decode_at_qmin(x), 0);
    // expressing at exponent e requires divisibility by 10^e; for e = Q(x) > 0 the
    // value is m * 10^Q(x), already a multiple, and the mantissa is x.m unchanged.
    let (m, q) = if e > 0 {
        debug_assert!(int_m % ipow(R, e as u32) == 0);
        (int_m / ipow(R, e as u32), e)
    } else {
        (int_m, 0)
    };
    if is_datum(m, q) {
        Some(Datum { m, q })
    } else {
        None
    }
}

fn main() {
    let mut data = Vec::new();
    for q in QMIN..=QMAX {
        for m in 0..ipow(R, P) {
            data.push(Datum { m, q });
        }
    }
    assert_eq!(data.len(), 4000);

    // (a) + (b): quantize closure, and the naive op's non-data counted.
    let mut refusals: u64 = 0;
    let mut naive_non_data: u64 = 0;
    for &x in &data {
        for &y in &data {
            match quantize(x, y) {
                Some(d) => {
                    // the result is a datum of D: statement 0's quantifier covers it,
                    // so decode(d) is in V by the encoding's own (constructional)
                    // statement 0, checked directly:
                    assert!(is_datum(d.m, d.q));
                }
                None => refusals += 1,
            }
            let (nm, nq) = quantize_naive(x, y);
            if !is_datum(nm, nq) {
                naive_non_data += 1;
            }
        }
    }
    assert!(refusals > 0, "the refusal branch is reachable");
    assert_eq!(
        refusals, naive_non_data,
        "the refusal branch fires exactly where the naive op leaves D"
    );

    // (c) the quantize witness: value-equal operands, value-different results.
    let x = Datum { m: 123, q: -2 }; // 1.23
    let y1 = Datum { m: 1, q: 0 }; // 1, quantum 10^0
    let y2 = Datum { m: 10, q: -1 }; // 1.0, quantum 10^-1, same value as y1
    assert_eq!(decode_at_qmin(y1), decode_at_qmin(y2));
    let r1 = quantize(x, y1).unwrap(); // 1
    let r2 = quantize(x, y2).unwrap(); // 1.2
    assert_ne!(
        decode_at_qmin(r1),
        decode_at_qmin(r2),
        "quantize's result VALUE depends on the operand's DATUM"
    );
    // and count how dense the effect is: for how many x does some value-equal
    // cohort pair (y1, y2) produce value-different results? Cohort pairs are
    // precomputed once (pairs of distinct data denoting the same value).
    let mut cohort_pairs = Vec::new();
    for &y1 in &data {
        for &y2 in &data {
            if y1.q < y2.q && decode_at_qmin(y1) == decode_at_qmin(y2) {
                cohort_pairs.push((y1, y2));
            }
        }
    }
    assert!(!cohort_pairs.is_empty());
    let mut value_dependent_x: u64 = 0;
    'outer: for &x in &data {
        for &(y1, y2) in &cohort_pairs {
            if let (Some(a), Some(b)) = (quantize(x, y1), quantize(x, y2)) {
                if decode_at_qmin(a) != decode_at_qmin(b) {
                    value_dependent_x += 1;
                    continue 'outer;
                }
            }
        }
    }
    assert!(value_dependent_x > 0);

    // (d) roundToIntegralExact closure: total over D, zero refusals.
    for &x in &data {
        let r = round_to_integral_exact(x);
        assert!(r.is_some(), "rtie refused at {:?}", x);
        let d = r.unwrap();
        assert!(is_datum(d.m, d.q));
    }

    // (e) rtie is fibre-preserving: value-equal operands give value-equal results,
    // exhaustively; and a witness that the result DATUM still differs.
    for &x1 in &data {
        for &x2 in &data {
            if decode_at_qmin(x1) == decode_at_qmin(x2) {
                let (a, b) = (
                    round_to_integral_exact(x1).unwrap(),
                    round_to_integral_exact(x2).unwrap(),
                );
                assert_eq!(
                    decode_at_qmin(a),
                    decode_at_qmin(b),
                    "rtie result value must not depend on the operand datum"
                );
            }
        }
    }
    let w1 = Datum { m: 10, q: 0 }; // 10 at exponent 0
    let w2 = Datum { m: 1, q: 1 }; // 10 at exponent 1, same value
    let (a, b) = (
        round_to_integral_exact(w1).unwrap(),
        round_to_integral_exact(w2).unwrap(),
    );
    assert_eq!(decode_at_qmin(a), decode_at_qmin(b));
    assert_ne!(
        (a.m, a.q),
        (b.m, b.q),
        "the result datum reads the operand datum"
    );

    println!(
        "OK. |D| = {}, quantize pairs = {}, refusals = {} ({}%), \
         naive non-data = {} (equal), value-dependent x = {} of {}",
        data.len(),
        data.len() * data.len(),
        refusals,
        (refusals as f64 * 100.0 / (data.len() * data.len()) as f64),
        naive_non_data,
        value_dependent_x,
        data.len()
    );
}
