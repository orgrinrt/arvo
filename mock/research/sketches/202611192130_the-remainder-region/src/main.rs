//! Where the remainder `x - round_m(x / y) y` is exact, where it is completed,
//! what its count needs, and where the alternatives break. Every section
//! asserts a prediction stated before the sweep and runs a planted variant that
//! the same check has to report.

mod arms;
mod rat;
mod reference;

use std::collections::HashSet;

use rat::{Q, pow};
use reference::{Fmt, M, MODES, dithers, lattice, remainder, round};

fn fmt(radix: i128, exp: i32, ph: (i128, i128), signed: bool, w: u32) -> Fmt {
    let (min, max) = if signed { Fmt::width_signed(w) } else { Fmt::width_unsigned(w) };
    Fmt {
        radix,
        exp,
        ph_n: ph.0,
        ph_d: ph.1,
        min,
        max,
    }
}

/// Every pair, every mode and dither: is the remainder's position whole?
fn exact_everywhere(f: Fmt, g: Fmt) -> bool {
    for a in f.min ..= f.max {
        for b in g.min ..= g.max {
            if g.value(b).is_zero() {
                continue;
            }
            for m in MODES {
                for k in dithers(m) {
                    if !remainder(f, g, a, b, m, k).1.is_int() {
                        return false;
                    }
                }
            }
        }
    }
    true
}

const RADIXES: [i128; 3] = [2, 3, 10];
const EXPS: [i32; 5] = [-3, -1, 0, 1, 2];
const PHASES: [(i128, i128); 8] =
    [(0, 1), (1, 1), (-3, 1), (1, 2), (1, 3), (2, 3), (-5, 4), (3, 2)];

/// Section 1: the remainder by a member of the same format is exact on the grid
/// for every pair exactly where the phase is whole; by a member of another
/// format, exactly where the lattice condition holds.
fn section_exactness() {
    let mut cells = 0;
    let mut planted_exponent_rule = 0;
    let mut whole = 0;
    for &r in &RADIXES {
        for &e in &EXPS {
            for &ph in &PHASES {
                for signed in [true, false] {
                    for w in 3 ..= 5 {
                        let f = fmt(r, e, ph, signed, w);
                        let seen = exact_everywhere(f, f);
                        let predicted = lattice(f, f);
                        assert_eq!(
                            predicted,
                            f.phase().d == 1,
                            "the lattice rule at G = F is the whole phase"
                        );
                        assert_eq!(seen, predicted, "{f:?}");
                        cells += 1;
                        whole += u64::from(seen);
                        // Planted: option b's region, the exponent alone.
                        planted_exponent_rule += u64::from((e == 0) != seen);
                    }
                }
            }
        }
    }
    assert!(
        planted_exponent_rule > 0,
        "the planted exponent-only rule went unreported"
    );
    println!(
        "1a. G = F: {cells} formats (radix 2,3,10 x exp -3,-1,0,1,2 x 8 phases x signed/unsigned x widths 3..5); exact at every pair exactly where the phase is whole: {whole} exact, {} not; the exponent-only rule disagrees at {planted_exponent_rule}",
        cells - whole
    );

    // Across formats: dividends over a few grids, divisors over many.
    let fs = [
        fmt(2, -2, (1, 2), true, 4),
        fmt(10, -1, (0, 1), true, 4),
        fmt(2, -1, (0, 1), false, 4),
        fmt(3, 0, (2, 3), true, 3),
    ];
    let mut pairs = 0;
    let mut held = 0;
    let mut planted_no_phase = 0;
    let mut planted_same_grid = 0;
    for f in fs {
        for &r in &RADIXES {
            for &e in &EXPS {
                for &ph in &PHASES {
                    for signed in [true, false] {
                        let g = fmt(r, e, ph, signed, 3);
                        let seen = exact_everywhere(f, g);
                        let predicted = lattice(f, g);
                        assert_eq!(seen, predicted, "F {f:?} G {g:?}");
                        pairs += 1;
                        held += u64::from(seen);
                        // Planted: the quantum ratio alone, forgetting G's phase.
                        let k = g.q().div(f.q());
                        planted_no_phase += u64::from(k.is_int() != seen);
                        // Planted: the member door's grid test, one quantum and one phase.
                        let same = g.q() == f.q() && g.phase() == f.phase();
                        planted_same_grid += u64::from(same != seen);
                    }
                }
            }
        }
    }
    assert!(
        planted_no_phase > 0 && planted_same_grid > 0,
        "a planted lattice rule went unreported"
    );
    println!(
        "1b. across formats: {pairs} (F, G) pairs; the lattice rule (q_G / q_F a whole multiple of G's reduced phase denominator) predicts exactness at all {pairs}, {held} exact; quantum-ratio-only rule wrong at {planted_no_phase}, same-grid rule wrong at {planted_same_grid}"
    );
}

/// Section 2: at a zero phase, where the exact remainder stays in the range,
/// and the bounds each mode gives it.
fn section_range() {
    let mut cells = 0;
    let mut out_cells = Vec::new();
    for &r in &[2i128, 10] {
        for &e in &[-3, 0, 2] {
            for signed in [true, false] {
                for w in 1 ..= 8 {
                    let f = fmt(r, e, (0, 1), signed, w);
                    for m in MODES {
                        let mut all_in = true;
                        for a in f.min ..= f.max {
                            for b in f.min ..= f.max {
                                if b == 0 {
                                    continue;
                                }
                                for k in dithers(m) {
                                    let (n, s) = remainder(f, f, a, b, m, k);
                                    assert!(s.is_int());
                                    let (x, y) = (f.value(a), f.value(b));
                                    let rr = x.sub(Q::int(n).mul(y));
                                    // |R| < |y| for every mode.
                                    assert!(rr.abs().lt(y.abs()), "{m:?} {a} {b}");
                                    match m {
                                        M::TowardZero => {
                                            assert!(rr.is_zero() || (rr.n < 0) == (x.n < 0))
                                        },
                                        M::Floor => {
                                            assert!(rr.is_zero() || (rr.n < 0) == (y.n < 0))
                                        },
                                        M::Ceil => assert!(rr.is_zero() || (rr.n < 0) != (y.n < 0)),
                                        M::HalfUp | M::HalfEven => {
                                            assert!(rr.abs().le(y.abs().mul(Q::new(1, 2))))
                                        },
                                        M::Stochastic => {},
                                    }
                                    all_in &= s.n >= f.min && s.n <= f.max;
                                }
                            }
                        }
                        let predicted = signed || matches!(m, M::TowardZero | M::Floor) || w == 1;
                        assert_eq!(all_in, predicted, "{f:?} {m:?}");
                        if !all_in && r == 2 && e == 0 && w == 8 {
                            out_cells.push(format!("{m:?}"));
                        }
                        cells += 1;
                    }
                }
            }
        }
    }
    println!(
        "2. zero phase, radix 2 and 10, exp -3/0/2, widths 1..8, both signednesses, six modes: {cells} cells; the exact remainder is in the range at every pair except unsigned under {} (width 2 and up); |R| < |y| everywhere, the sign rules of toward_zero, floor and ceil and |R| <= |y|/2 at the half modes hold everywhere",
        out_cells.join(", ")
    );
}

/// Section 3: the remainder's slot reads no quantum: at one phase and one slot
/// range it is the same at every radix and exponent.
fn section_quantum_free() {
    let mut checked = 0u64;
    let mut planted = 0u64;
    for &ph in &PHASES {
        for signed in [true, false] {
            for w in 1 ..= 6 {
                let base = fmt(2, 0, ph, signed, w);
                for &r in &RADIXES {
                    for &e in &EXPS {
                        let f = fmt(r, e, ph, signed, w);
                        for a in f.min ..= f.max {
                            for b in f.min ..= f.max {
                                if f.value(b).is_zero() {
                                    continue;
                                }
                                for m in MODES {
                                    for k in dithers(m) {
                                        let s = remainder(f, f, a, b, m, k);
                                        assert_eq!(s, remainder(base, base, a, b, m, k));
                                        checked += 1;
                                        // Planted: option b's count, the quotient on F's grid.
                                        let q = f.q();
                                        let t = f.value(a).div(f.value(b)).div(q).sub(f.phase());
                                        let qb = f.value(round(t, m, k));
                                        let sb = f
                                            .value(a)
                                            .sub(qb.mul(f.value(b)))
                                            .div(q)
                                            .sub(f.phase());
                                        let qb0 = base.value(round(
                                            base.value(a)
                                                .div(base.value(b))
                                                .div(base.q())
                                                .sub(base.phase()),
                                            m,
                                            k,
                                        ));
                                        let sb0 = base
                                            .value(a)
                                            .sub(qb0.mul(base.value(b)))
                                            .div(base.q())
                                            .sub(base.phase());
                                        planted += u64::from(sb != sb0);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(
        planted > 0,
        "option b's remainder was quantum-free too, so the check cannot fail"
    );
    println!(
        "3. {checked} (pair, mode, dither, radix, exponent) points: the definition's count and slot are the same at every radix and exponent; option b's remainder moves with the quantum at {planted} of them"
    );
}

/// Section 4: the count's range.
fn section_count() {
    let mut lines = Vec::new();
    for signed in [true, false] {
        for w in 1 ..= 8u32 {
            let f = fmt(2, -3, (0, 1), signed, w);
            let mut lo = i128::MAX;
            let mut hi = i128::MIN;
            for a in f.min ..= f.max {
                for b in f.min ..= f.max {
                    if b == 0 {
                        continue;
                    }
                    for m in MODES {
                        for k in dithers(m) {
                            let (n, _) = remainder(f, f, a, b, m, k);
                            lo = lo.min(n);
                            hi = hi.max(n);
                        }
                    }
                }
            }
            if signed {
                // At width one the range is {-1, 0}: no positive divisor, so
                // the lowest count is 0 rather than MIN. The first run asserted
                // (MIN, -MIN) here and failed at width one; recorded in FINDINGS.
                let low = if w == 1 { 0 } else { f.min };
                assert_eq!((lo, hi), (low, -f.min), "signed {w}");
            } else {
                assert_eq!((lo, hi), (0, f.max), "unsigned {w}");
            }
            if w == 8 {
                lines.push(format!(
                    "{} 8: count in [{lo}, {hi}]",
                    if signed { "signed" } else { "unsigned" }
                ));
            }
        }
    }
    // With a phase the count reaches past the slot range.
    let f = fmt(2, -1, (1, 2), true, 6);
    let mut hi = 0;
    for a in f.min ..= f.max {
        for b in f.min ..= f.max {
            for m in MODES {
                hi = hi.max(remainder(f, f, a, b, m, 0).0.abs());
            }
        }
    }
    lines.push(format!(
        "signed 6 at phase 1/2: |count| up to {hi}, past the slot range's 32"
    ));
    assert_eq!(hi, 63);
    println!(
        "4. the count at zero phase, widths 1..8, every mode: signed [MIN, -MIN] (one past MAX, at MIN / -1 only), unsigned [0, MAX]: {}",
        lines.join("; ")
    );
}

/// Section 5: the host's own integer remainders, a third instance at zero phase.
fn section_host() {
    let mut checked = 0;
    for w in 1 ..= 8u32 {
        let f = fmt(2, 0, (0, 1), true, w);
        // A divisor format wide enough to hold |b| at MIN.
        let wide = fmt(2, 0, (0, 1), true, w + 1);
        for a in f.min ..= f.max {
            for b in f.min ..= f.max {
                if b == 0 {
                    continue;
                }
                let (a64, b64) = (a as i64, b as i64);
                assert_eq!(
                    remainder(f, f, a, b, M::TowardZero, 0).1.n,
                    (a64 % b64) as i128
                );
                // Rust's Euclidean remainder is the floor remainder by |b|,
                // and the floor one for b > 0, the ceil one for b < 0.
                let eu = a64.rem_euclid(b64) as i128;
                assert_eq!(remainder(f, wide, a, b.abs(), M::Floor, 0).1.n, eu);
                let m = if b > 0 { M::Floor } else { M::Ceil };
                assert_eq!(remainder(f, f, a, b, m, 0).1.n, eu);
                checked += 1;
            }
        }
        // Planted: Euclid read as the floor remainder by b itself.
        if w >= 2 {
            let mut caught = false;
            for a in f.min ..= f.max {
                for b in f.min ..= -1 {
                    caught |= remainder(f, f, a, b, M::Floor, 0).1.n
                        != (a as i64).rem_euclid(b as i64) as i128;
                }
            }
            assert!(
                caught,
                "the planted Euclid-as-floor went unreported at width {w}"
            );
        }
    }
    println!(
        "5. host integers, signed widths 1..8, {checked} pairs: toward_zero is `%`; `rem_euclid` is floor by |b|, and floor for b > 0 with ceil for b < 0; the planted Euclid-as-floor is reported at every width from 2"
    );
}

/// Section 6: a toy binary float with subnormals, every finite pair: where the
/// exact remainder is a member.
fn section_floats() {
    for p in 2 ..= 4u32 {
        let (emin, emax) = (-3i32, 3i32);
        let mut set = HashSet::new();
        set.insert(Q::int(0));
        for e in emin ..= emax {
            for m in 0 .. (1i128 << p) {
                if e > emin && m < (1 << (p - 1)) {
                    continue;
                }
                let v = Q::int(m).mul(pow(2, e - p as i32 + 1));
                set.insert(v);
                set.insert(Q {
                    n: -v.n,
                    d: v.d,
                });
            }
        }
        let vals: Vec<Q> = set.iter().copied().collect();
        let mut line = Vec::new();
        let mut max_count = 0i128;
        for m in MODES {
            let mut miss = 0;
            let mut total = 0;
            let mut example = None;
            for &x in &vals {
                for &y in &vals {
                    if y.is_zero() {
                        continue;
                    }
                    for k in dithers(m) {
                        let n = round(x.div(y), m, k);
                        max_count = max_count.max(n.abs());
                        let r = x.sub(Q::int(n).mul(y));
                        total += 1;
                        if !set.contains(&r) {
                            miss += 1;
                            if example.is_none() {
                                example = Some((x, y, r));
                            }
                        }
                    }
                }
            }
            let exact = miss == 0;
            assert_eq!(
                exact,
                matches!(m, M::TowardZero | M::HalfUp | M::HalfEven),
                "p {p} {m:?}"
            );
            line.push(match example {
                None => format!("{m:?} exact at {total}"),
                Some((x, y, r)) => {
                    format!(
                        "{m:?} off at {miss} of {total}, e.g. {}/{} by {}/{} leaves {}/{}",
                        x.n, x.d, y.n, y.d, r.n, r.d
                    )
                },
            });
        }
        println!(
            "6. binary float, precision {p}, exponents {emin}..{emax} with subnormals: {}; largest count {max_count}, against a significand of {} bits",
            line.join("; "),
            p
        );
    }
}

fn section_arms() {
    let rows = arms::run();
    println!(
        "7. options b and c through arvo's shipped div, mul, sub, convert and adapt, width 6, every mode and policy:"
    );
    println!(
        "   {:<14} {:<11} {:<9} {:>6} {:>7} {:>7} {:>7} {:>8} {:>8} {:>7} {:>7} {:>9}",
        "format",
        "mode",
        "policy",
        "pairs",
        "def-grid",
        "b=def",
        "b-grid",
        "b-bound",
        "b-adapt",
        "c1=def",
        "c2=def",
        "c3=def"
    );
    let mut b_off = 0;
    let mut c_off = 0;
    for row in &rows {
        let t = row.tally;
        println!(
            "   {:<14} {:<11} {:<9} {:>6} {:>7} {:>7} {:>7} {:>8} {:>8} {:>7} {:>7} {:>9}",
            row.format,
            format!("{:?}", row.mode),
            row.policy,
            t.pairs,
            t.def_on_grid,
            t.b_same,
            t.b_on_grid,
            t.b_bounded,
            t.b_adapted,
            t.c1_same,
            t.c2_same,
            if t.c3_run > 0 {
                format!("{}/{}", t.c3_same, t.c3_run)
            } else {
                "n/a".to_string()
            }
        );
        b_off += t.pairs - t.b_adapted;
        c_off += t.pairs - t.c1_same;
        // c3, the reimplementation in a wider integer format, is the definition
        // wherever it runs: that is the composable region.
        assert_eq!(
            t.c3_same, t.c3_run,
            "{} {:?} {}",
            row.format, row.mode, row.policy
        );
    }
    assert!(
        b_off > 0 && c_off > 0,
        "options b and c agreed everywhere, so the arm cannot fail"
    );
    for line in arms::named() {
        println!("   named: {line}");
    }
}

fn main() {
    section_exactness();
    section_range();
    section_quantum_free();
    section_count();
    section_host();
    section_floats();
    section_arms();
    println!("all predictions held and every planted variant was reported");
}
