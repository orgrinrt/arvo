// p270b: does `probe::rounding_commutes_with_the_overflow_policies` hold under
// both readings of `half_up`, or only one?
//
// The row says: "At W in {3, 4, 5} over exact points at sixteen subquanta per
// quantum, wrapping commutes with floor, ceiling, half-up and half-even and
// fails to commute with toward-zero". Neither seat 229 nor seat 267 names this
// row among those whose truth depends on the reading.
//
// Model. An exact point is k / 16 for integer k, over four whole wraps of the
// signed W-bit integer grid on each side, so every point is reached from the
// container and from outside it. Rounding R takes it to an integer. Wrapping
// takes an integer, or an exact point, to the representative in
// [-2^(W-1), 2^(W-1)). Commutation is compared in the quotient group, the
// comparison the source of the row, `125`, used: wrap(R(x)) against
// wrap(R(wrap(x))), so a rounding that lands one past the top of the range
// after the wrap is compared as the integer it is congruent to.
//
// Controls, stated before the sweep and asserted after:
//   K1 must hold: floor commutes on every row.
//   K2 must fail: toward_zero fails on every row, as the canon row records.
//   K3 must fail somewhere: the two readings of `half_up` must give different
//      counts, or this instrument cannot tell them apart and is void.
//
// Run: rustc --edition 2021 -O p270b_wrap_commutation_under_both_readings.rs -o /tmp/p270b && /tmp/p270b

const SUB: i64 = 16; // subquanta per quantum, as the row states

#[derive(Copy, Clone, PartialEq, Debug)]
enum M {
    Floor,
    Ceil,
    TowardZero,
    HalfEven,
    HalfUpPosInf,
    HalfUpAway,
}

const MODES: [M; 6] =
    [M::Floor, M::Ceil, M::TowardZero, M::HalfEven, M::HalfUpPosInf, M::HalfUpAway];

fn round(m: M, k: i64) -> i64 {
    let q = k.div_euclid(SUB);
    let r = k.rem_euclid(SUB);
    let twice = 2 * r;
    match m {
        M::Floor => q,
        M::Ceil => {
            if r == 0 {
                q
            } else {
                q + 1
            }
        },
        M::TowardZero => {
            if r == 0 || k >= 0 {
                q
            } else {
                q + 1
            }
        },
        M::HalfEven => {
            if twice > SUB {
                q + 1
            } else if twice < SUB {
                q
            } else if q % 2 == 0 {
                q
            } else {
                q + 1
            }
        },
        M::HalfUpPosInf => {
            if twice >= SUB {
                q + 1
            } else {
                q
            }
        },
        M::HalfUpAway => {
            if twice > SUB {
                q + 1
            } else if twice < SUB {
                q
            } else if k < 0 {
                q
            } else {
                q + 1
            }
        },
    }
}

fn wrap_int(n: i64, w: u32) -> i64 {
    let span = 1i64 << w;
    let half = span / 2;
    (n + half).rem_euclid(span) - half
}

fn wrap_exact(k: i64, w: u32) -> i64 {
    let span = (1i64 << w) * SUB;
    let half = span / 2;
    (k + half).rem_euclid(span) - half
}

fn main() {
    println!("p270b: wrapping against rounding, quotient-group comparison");
    println!("exact points k/{SUB}, four wraps either side of the signed W-bit grid");
    println!();
    let mut counts = Vec::new();
    for w in [3u32, 4, 5] {
        let span = (1i64 << w) * SUB;
        let lo = -4 * span;
        let hi = 4 * span;
        print!("W = {w}, {} points:", hi - lo);
        for m in MODES {
            let mut fail = 0u64;
            for k in lo .. hi {
                let a = wrap_int(round(m, k), w);
                let b = wrap_int(round(m, wrap_exact(k, w)), w);
                if a != b {
                    fail += 1;
                }
            }
            print!("  {:?} {}", m, fail);
            counts.push((w, m, fail));
        }
        println!();
    }
    println!();
    let get = |w: u32, m: M| counts.iter().find(|c| c.0 == w && c.1 == m).unwrap().2;
    let k1 = [3, 4, 5].iter().all(|&w| get(w, M::Floor) == 0);
    let k2 = [3, 4, 5].iter().all(|&w| get(w, M::TowardZero) > 0);
    let k3 = [3, 4, 5]
        .iter()
        .any(|&w| get(w, M::HalfUpPosInf) != get(w, M::HalfUpAway));
    println!("K1 floor commutes on every row: {k1}");
    println!("K2 toward_zero fails on every row: {k2}");
    println!("K3 the two readings give different counts somewhere: {k3}");
    let pos_inf_commutes = [3, 4, 5].iter().all(|&w| get(w, M::HalfUpPosInf) == 0);
    let away_commutes = [3, 4, 5].iter().all(|&w| get(w, M::HalfUpAway) == 0);
    println!();
    println!("the row's `half-up commutes` holds under ties toward +inf: {pos_inf_commutes}");
    println!("the row's `half-up commutes` holds under ties away from zero: {away_commutes}");
    assert!(
        k1 && k2 && k3,
        "a control failed; every count above is void"
    );
    println!("instrument: sound");
}
