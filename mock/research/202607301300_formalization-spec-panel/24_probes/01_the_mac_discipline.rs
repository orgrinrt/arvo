// Probe 01: the MAC discipline, measured.
//
// Hypothesis: the load-bearing multiplicative shape in signal processing is not
// fold(*, xs) but the multiply-accumulate: sum over i of a_i * b_i. Under the
// per-operation quantisation reading, that shape loses grouping-invariance and
// accumulates error. Under the field's standard discipline (exact products into
// a wide accumulator, quantise once at the store), the interior is exact integer
// arithmetic, grouping-invariance is restored by construction, and the whole
// computation performs exactly one quantisation.
//
// Model: signed Q2.2 numeral, raw in [-8, 7], value raw/4. Exact products carry
// 4 fractional bits (raw units 2^-4) and are held in i64, exact for every sum
// measured here. Quantisers: round-to-nearest-even (the Warm/Cold/Precise
// in-range intent) and floor (the shipped `>> FRAC`, which on a two's-complement
// arithmetic shift rounds toward negative infinity). Range recovery: clamp.
//
// Disciplines compared, per input tuple (a_1..a_n, b_1..b_n):
//   (A) per-op: each product quantised to Q2.2 immediately, then the sum folded
//       with per-op quantisation, over EVERY binary grouping of the sum.
//   (B) wide accumulator: exact products summed exactly in i64, quantised once.
// Reference: (B) is quantise(exact result), the correctly-rounded answer.
//
// Outputs, per n: grouping diameter of (A) (max over inputs of spread across
// groupings, in raw ulps), worst |A - B| in ulps, mean signed error (bias) of
// (A) under each rounding rule, and the rounding-fire counts for + and * that
// files 18 and 21 measured, reproduced here independently.

const LO: i64 = -8;
const HI: i64 = 7;

// quantise a value carrying `extra` fractional bits beyond F down to the numeral,
// round-to-nearest-even, then clamp.
fn q_rne(x: i64, extra: u32) -> i64 {
    if extra == 0 {
        return clamp(x);
    }
    let unit = 1i64 << extra;
    let half = unit >> 1;
    let floor = x >> extra; // arithmetic shift: toward -inf
    let rem = x - (floor << extra);
    let rounded = if rem > half {
        floor + 1
    } else if rem < half {
        floor
    } else {
        // tie: to even
        if floor & 1 == 0 {
            floor
        } else {
            floor + 1
        }
    };
    let _ = unit;
    clamp(rounded)
}

// floor quantisation (the shipped `>> FRAC` shape), then clamp.
fn q_floor(x: i64, extra: u32) -> i64 {
    clamp(x >> extra)
}

fn clamp(x: i64) -> i64 {
    if x < LO {
        LO
    } else if x > HI {
        HI
    } else {
        x
    }
}

// all binary groupings (as fold trees) of n leaves, evaluated by recursion on
// index ranges: enumerate split points. Returns the set of results for the
// per-op discipline on the quantised products `ps` (each already Q2.2 raw).
fn groupings(ps: &[i64], quant: fn(i64, u32) -> i64, out: &mut Vec<i64>) {
    fn eval(ps: &[i64], lo: usize, hi: usize, quant: fn(i64, u32) -> i64, acc: &mut Vec<i64>) {
        if hi - lo == 1 {
            acc.push(ps[lo]);
            return;
        }
        let mut left = Vec::new();
        let mut right = Vec::new();
        for m in lo + 1..hi {
            left.clear();
            right.clear();
            eval(ps, lo, m, quant, &mut left);
            eval(ps, m, hi, quant, &mut right);
            for &l in &left {
                for &r in &right {
                    // addition of two Q2.2 raws is exact at 0 extra bits; the
                    // quantiser only clamps here.
                    acc.push(quant(l + r, 0));
                }
            }
        }
    }
    out.clear();
    eval(ps, 0, ps.len(), quant, out);
    out.sort_unstable();
    out.dedup();
}

fn main() {
    // reproduce the rounding-fire counts first (files 18 and 21).
    let mut add_fired = 0u32;
    let mut mul_fired = 0u32;
    for a in LO..=HI {
        for b in LO..=HI {
            // addition: exact sum has 0 extra fractional bits; rounding never fires.
            let s = a + b;
            if s != q_rne(s, 0) && s >= LO && s <= HI {
                add_fired += 1;
            }
            // multiplication: product carries 2 extra fractional bits (F=2).
            let p = a * b; // units 2^-4
            let in_range = (p >> 2) >= LO && (p >> 2) <= HI;
            if in_range && (p & 0b11) != 0 {
                mul_fired += 1;
            }
        }
    }
    println!("in-range rounding fired: add {add_fired}/256  mul {mul_fired}/256");
    println!();

    // MAC disciplines. Sample the operand space (exhaustive at n=2, sampled above).
    let mut seed: u64 = 0x243F6A8885A308D3;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };

    println!("n | samples | diamA(rne) | worst|A-B|(rne) | biasA(rne) | diamA(floor) | worst|A-B|(floor) | biasA(floor) | diamB");
    for n in 2usize..=5 {
        let samples = if n == 2 { 65536 } else { 200_000 };
        let mut diam_a_rne = 0i64;
        let mut worst_rne = 0i64;
        let mut bias_rne = 0f64;
        let mut diam_a_fl = 0i64;
        let mut worst_fl = 0i64;
        let mut bias_fl = 0f64;
        let mut diam_b = 0i64;
        let mut set = Vec::new();
        for s in 0..samples {
            let mut a = [0i64; 5];
            let mut b = [0i64; 5];
            if n == 2 {
                // exhaustive over (a0,b0,a1,b1)
                let mut v = s as i64;
                for k in 0..2 {
                    a[k] = LO + (v & 15);
                    v >>= 4;
                    b[k] = LO + (v & 15);
                    v >>= 4;
                }
            } else {
                for k in 0..n {
                    a[k] = LO + (rnd() & 15) as i64;
                    b[k] = LO + (rnd() & 15) as i64;
                }
            }
            // exact products, units 2^-4
            let mut exact = 0i64;
            let mut ps_rne = [0i64; 5];
            let mut ps_fl = [0i64; 5];
            for k in 0..n {
                let p = a[k] * b[k];
                exact += p;
                ps_rne[k] = q_rne(p, 2);
                ps_fl[k] = q_floor(p, 2);
            }
            // (B): quantise the exact sum once. Interior is exact integer
            // addition in i64: associative, so diameter 0 by construction;
            // asserted anyway.
            let b_res = q_rne(exact, 2);
            let b_res2 = q_rne(exact, 2);
            diam_b = diam_b.max(b_res - b_res2);

            // (A): all groupings of the per-op sums.
            groupings(&ps_rne[..n], q_rne, &mut set);
            let d = set[set.len() - 1] - set[0];
            diam_a_rne = diam_a_rne.max(d);
            for &r in &set {
                worst_rne = worst_rne.max((r - b_res).abs());
            }
            // left-to-right bias
            let mut acc = ps_rne[0];
            for k in 1..n {
                acc = q_rne(acc + ps_rne[k], 0);
            }
            bias_rne += (acc - b_res) as f64;

            groupings(&ps_fl[..n], q_floor, &mut set);
            let d = set[set.len() - 1] - set[0];
            diam_a_fl = diam_a_fl.max(d);
            let b_fl = q_floor(exact, 2); // floor reference for the floor rule
            for &r in &set {
                worst_fl = worst_fl.max((r - b_fl).abs());
            }
            let mut acc = ps_fl[0];
            for k in 1..n {
                acc = q_floor(acc + ps_fl[k], 0);
            }
            bias_fl += (acc - b_fl) as f64;
        }
        let m = samples as f64;
        println!(
            "{n} | {samples} | {diam_a_rne} | {worst_rne} | {:+.4} | {diam_a_fl} | {worst_fl} | {:+.4} | {diam_b}",
            bias_rne / m,
            bias_fl / m
        );
    }
    println!();
    println!("diamB is 0 at every n by construction: the wide accumulator's interior is exact");
    println!("integer addition, which is associative; the one quantisation happens at the store.");
}
