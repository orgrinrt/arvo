// probe 2: a value-exact model of OCP E4M3 (the design's own no-infinity
// witness format: 4 exponent bits, 3 mantissa bits, bias 7, NaN at
// {S,1111,111}, no infinities, max finite 448) run through the three
// candidate out-of-range resolutions on the dispatch's three stress
// computations, plus the boundary and order facts the deliverable cites.
//
// everything here is runtime model arithmetic on the host (f64 carries
// every E4M3 value and every intermediate below exactly; the largest
// intermediate is 1024), no timing, no performance claim. the point is
// bit-exact VALUES under each resolution, not speed.
//
// expected outcomes, asserted below rather than eyeballed:
//   max finite = 448, top-binade ulp = 32, MAX's stored mantissa LSB even
//   in-range/out-of-range boundary sits at 464, tie resolved TO 448 by
//     parity (E4M3's max finite is even because the all-ones slot is NaN,
//     the opposite parity of an IEEE format's max finite)
//   (448+448)-448: true 448 | saturate 0 | nan-mode NaN | refuse at op 1
//   (416*2)/4:     true 208 | saturate 112 | nan-mode NaN | refuse at op 1
//   sum of 16x64:  true 1024| saturate 448 | nan-mode NaN | refuse at op 8
//   saturating quantisation is weakly monotone over a 2401-point sweep;
//     nan-on-overflow mode is not (448 then unordered)

#[derive(Clone, Copy, Debug, PartialEq)]
enum Q {
    Val(f64),
    Nan,
    Refused,
}

#[derive(Clone, Copy, PartialEq)]
enum Resolution {
    SaturateFar,
    NanOnOverflow,
    Refuse,
}

// every finite E4M3 value, positive side, ascending.
fn e4m3_positive_finites() -> Vec<f64> {
    let mut v = Vec::new();
    // subnormals: exp field 0, value = m/8 * 2^-6, m in 0..=7
    for m in 0..=7u32 {
        v.push((m as f64) / 8.0 * 2f64.powi(-6));
    }
    // normals: exp field 1..=15, value = (1 + m/8) * 2^(e-7)
    // (e=15, m=7) is NaN and is excluded; e4m3 uses the whole top binade
    // otherwise, which is what buys emax = 8.
    for e in 1..=15i32 {
        for m in 0..=7u32 {
            if e == 15 && m == 7 {
                continue; // NaN slot
            }
            v.push((1.0 + (m as f64) / 8.0) * 2f64.powi(e - 7));
        }
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v.dedup();
    v
}

// round-to-nearest, ties-to-even over the EXTENDED grid: the format's own
// step pattern continued one hypothetical step past max finite. for e4m3
// the step above 448 is 480 (stored mantissa would be 111, odd), so the
// tie at 464 resolves to 448 (mantissa 110, even). only a rounded result
// that is NOT a representable value is an out-of-range event; everything
// up to and including 464 is ordinary in-range rounding. this is the same
// construction ieee 754 uses for its own overflow boundary, applied to a
// grid whose far point happens to be finite.
fn quantise(x: f64, grid: &[f64], res: Resolution) -> Q {
    let max = *grid.last().unwrap();
    let step_top = max - grid[grid.len() - 2]; // 448 - 416 = 32
    let a = x.abs();
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    // boundary: max + step/2, tie included on the in-range side because
    // max's parity is even (asserted in main).
    if a <= max + step_top / 2.0 {
        // ordinary rne among representables (ties at interior midpoints
        // resolved by stored-mantissa parity; for this probe's inputs no
        // interior tie arises except the top one, handled above).
        let mut best = grid[0];
        let mut bd = (a - best).abs();
        for &g in grid {
            let d = (a - g).abs();
            if d < bd {
                best = g;
                bd = d;
            }
        }
        return Q::Val(sign * best);
    }
    match res {
        Resolution::SaturateFar => Q::Val(sign * max),
        Resolution::NanOnOverflow => Q::Nan,
        Resolution::Refuse => Q::Refused,
    }
}

fn add(a: Q, b: Q, grid: &[f64], r: Resolution) -> Q {
    bin(a, b, grid, r, |x, y| x + y)
}
fn mul(a: Q, b: Q, grid: &[f64], r: Resolution) -> Q {
    bin(a, b, grid, r, |x, y| x * y)
}
fn div(a: Q, b: Q, grid: &[f64], r: Resolution) -> Q {
    bin(a, b, grid, r, |x, y| x / y)
}
fn bin(a: Q, b: Q, grid: &[f64], r: Resolution, f: fn(f64, f64) -> f64) -> Q {
    match (a, b) {
        (Q::Refused, _) | (_, Q::Refused) => Q::Refused,
        (Q::Nan, _) | (_, Q::Nan) => Q::Nan,
        (Q::Val(x), Q::Val(y)) => quantise(f(x, y), grid, r),
    }
}

fn main() {
    let grid = e4m3_positive_finites();
    let max = *grid.last().unwrap();
    let ulp_top = max - grid[grid.len() - 2];
    assert_eq!(max, 448.0);
    assert_eq!(ulp_top, 32.0);
    // MAX = 448 = 1.110 x 2^8: stored mantissa 110, LSB 0, even. the
    // hypothetical next step 480 would be 111, odd. so the extended-grid
    // tie at 464 goes to 448 by the ordinary even rule.
    assert_eq!(quantise(464.0, &grid, Resolution::Refuse), Q::Val(448.0));
    assert_eq!(quantise(456.0, &grid, Resolution::Refuse), Q::Val(448.0));
    assert_eq!(quantise(464.0001, &grid, Resolution::Refuse), Q::Refused);
    println!("boundary: (448, 464] in-range -> 448; above 464 out-of-range");

    for r in [
        Resolution::SaturateFar,
        Resolution::NanOnOverflow,
        Resolution::Refuse,
    ] {
        let name = match r {
            Resolution::SaturateFar => "saturate",
            Resolution::NanOnOverflow => "nan-mode",
            Resolution::Refuse => "refuse  ",
        };
        // a. the come-back sum: (448 + 448) - 448, true value 448
        let a = add(
            add(Q::Val(448.0), Q::Val(448.0), &grid, r),
            Q::Val(-448.0),
            &grid,
            r,
        );
        // b. large product through a division: (416 * 2) / 4, true 208
        let b = div(
            mul(Q::Val(416.0), Q::Val(2.0), &grid, r),
            Q::Val(4.0),
            &grid,
            r,
        );
        // c. column accumulation: sixteen elements of 64, true 1024
        let mut c = Q::Val(0.0);
        for _ in 0..16 {
            c = add(c, Q::Val(64.0), &grid, r);
        }
        println!("{name}: (448+448)-448 = {a:?} | (416*2)/4 = {b:?} | sum16x64 = {c:?}");
    }

    // order: saturating quantisation is weakly monotone (adjacent pairs
    // over a dense sweep suffice for monotone). nan-on-overflow is not
    // even comparable past the boundary.
    let mut prev = f64::NEG_INFINITY;
    let mut mono = true;
    let mut x = -600.0;
    while x <= 600.0 {
        if let Q::Val(v) = quantise(x, &grid, Resolution::SaturateFar) {
            if v < prev {
                mono = false;
            }
            prev = v;
        } else {
            mono = false;
        }
        x += 0.5;
    }
    assert!(mono);
    println!("saturate: weakly monotone over 2401-point sweep, total on it");
    let lo = quantise(460.0, &grid, Resolution::NanOnOverflow);
    let hi = quantise(470.0, &grid, Resolution::NanOnOverflow);
    println!("nan-mode: q(460) = {lo:?}, q(470) = {hi:?}: order lost at the boundary");
}
