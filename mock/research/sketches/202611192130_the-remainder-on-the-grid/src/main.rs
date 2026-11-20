//! The remainder `r = a - b q`, `q` the exact quotient `a / b` rounded to an
//! integer by a mode, swept over arvo's constant-quantum grid and a toy
//! magnitude-indexed grid, every value an exact rational formed here.
//!
//! Each sweep states a claim and asserts it; each claim has a control in the
//! same run that has to come out the other way, so a sweep that cannot fail is
//! visible as one. The features plant a wrong remainder that a sweep must report.

use std::collections::HashSet;

type I = i128;

const fn gcd(mut a: I, mut b: I) -> I {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// An exact rational in lowest terms, denominator positive.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Q {
    n: I,
    d: I,
}

impl Q {
    fn new(n: I, d: I) -> Q {
        assert!(d != 0);
        let s = if d < 0 { -1 } else { 1 };
        let g = gcd(n, d).max(1);
        Q {
            n: s * n / g,
            d: s * d / g,
        }
    }

    fn int(n: I) -> Q {
        Q {
            n,
            d: 1,
        }
    }

    fn add(self, o: Q) -> Q {
        Q::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }

    fn sub(self, o: Q) -> Q {
        Q::new(self.n * o.d - o.n * self.d, self.d * o.d)
    }

    fn mul(self, o: Q) -> Q {
        Q::new(self.n * o.n, self.d * o.d)
    }

    fn div(self, o: Q) -> Q {
        Q::new(self.n * o.d, self.d * o.n)
    }

    fn floor(self) -> I {
        self.n.div_euclid(self.d)
    }

    fn is_int(self) -> bool {
        self.d == 1
    }

    fn sign(self) -> I {
        self.n.signum()
    }

    fn abs(self) -> Q {
        Q {
            n: self.n.abs(),
            d: self.d,
        }
    }

    fn lt(self, o: Q) -> bool {
        self.n * o.d < o.n * self.d
    }

    fn le(self, o: Q) -> bool {
        self.n * o.d <= o.n * self.d
    }
}

/// The six modes, the sixth as its two outcomes: a stochastic rounding lands on
/// the floor or the ceiling, so each is swept as a mode of its own.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    TowardZero,
    Floor,
    Ceil,
    HalfUp,
    HalfEven,
    StochasticDown,
    StochasticUp,
}

const MODES: [Mode; 7] = [
    Mode::TowardZero,
    Mode::Floor,
    Mode::Ceil,
    Mode::HalfUp,
    Mode::HalfEven,
    Mode::StochasticDown,
    Mode::StochasticUp,
];

/// `x` rounded to an integer by `m`, read from the rational alone.
fn round(x: Q, m: Mode) -> I {
    let f = x.floor();
    let frac = x.sub(Q::int(f));
    let half = Q::new(1, 2);
    match m {
        Mode::Floor | Mode::StochasticDown => f,
        Mode::Ceil | Mode::StochasticUp => {
            if frac.n == 0 {
                f
            } else {
                f + 1
            }
        },
        Mode::TowardZero => {
            if frac.n == 0 || x.n > 0 {
                f
            } else {
                f + 1
            }
        },
        // Ties toward positive infinity, `floor(x + 1/2)`.
        Mode::HalfUp => x.add(half).floor(),
        Mode::HalfEven => {
            if frac.lt(half) {
                f
            } else if half.lt(frac) {
                f + 1
            } else if f % 2 == 0 {
                f
            } else {
                f + 1
            }
        },
    }
}

/// The mode the remainder's quotient is rounded by. A plant swaps it.
fn quotient_mode(m: Mode) -> Mode {
    if cfg!(feature = "plant-sign-of-the-dividend") && m == Mode::Floor {
        Mode::TowardZero
    } else {
        m
    }
}

/// A grid at a constant quantum `r^e`, a phase `pn / pd` in lowest terms and a
/// slot range `[min, max]`: slot `s` is the value `(pn / pd + s) r^e`.
#[derive(Clone, Copy, Debug)]
struct Grid {
    r:   I,
    e:   i32,
    pn:  I,
    pd:  I,
    min: I,
    max: I,
}

impl Grid {
    fn quantum(&self) -> Q {
        if self.e >= 0 {
            Q::int(self.r.pow(self.e as u32))
        } else {
            Q::new(1, self.r.pow((-self.e) as u32))
        }
    }

    fn phase(&self) -> Q {
        Q::new(self.pn, self.pd)
    }

    fn value(&self, s: I) -> Q {
        self.phase().add(Q::int(s)).mul(self.quantum())
    }

    /// The position of a value, `v / q - phase`: a slot where it is whole.
    fn position(&self, v: Q) -> Q {
        v.div(self.quantum()).sub(self.phase())
    }

    fn members(&self) -> impl Iterator<Item = I> {
        self.min ..= self.max
    }

    fn signed(w: u32, r: I, e: i32, pn: I, pd: I) -> Grid {
        Grid {
            r,
            e,
            pn,
            pd,
            min: -(1 << (w - 1)),
            max: (1 << (w - 1)) - 1,
        }
    }

    fn unsigned(w: u32, r: I, e: i32, pn: I, pd: I) -> Grid {
        Grid {
            r,
            e,
            pn,
            pd,
            min: 0,
            max: (1 << w) - 1,
        }
    }
}

/// The remainder and its integer quotient for two values.
///
/// The plant `plant-quotient-on-the-grid` rounds the quotient to a multiple of
/// an eighth instead, the quotient on a grid finer than the integers, which is
/// option b's quotient at a negative exponent: the remainder then leaves the grid.
fn remainder(a: Q, b: Q, m: Mode) -> (I, Q) {
    if cfg!(feature = "plant-quotient-on-the-grid") {
        let eighths = round(a.div(b).mul(Q::int(8)), quotient_mode(m));
        let q = Q::new(eighths, 8);
        return (q.floor(), a.sub(b.mul(q)));
    }
    let q = round(a.div(b), quotient_mode(m));
    (q, a.sub(b.mul(Q::int(q))))
}

/// The grids at a whole phase every sweep below runs over.
fn whole_phase_grids() -> Vec<(Grid, &'static str)> {
    let mut out = Vec::new();
    for r in [2, 3, 10] {
        for e in -3 ..= 2 {
            for pn in [0, 1, -2] {
                for w in 1 ..= 5 {
                    out.push((Grid::signed(w, r, e, pn, 1), "signed"));
                    out.push((Grid::unsigned(w, r, e, pn, 1), "unsigned"));
                }
            }
        }
    }
    out
}

#[derive(Default, Debug)]
struct Tally {
    pairs:             u64,
    off_grid:          u64,
    slot_formula:      u64,
    past_bound:        u64,
    wrong_sign:        u64,
    out_of_range:      u64,
    quotient_on_s:     u64,
    b_agrees:          u64,
    b_departs:         u64,
    b_departs_at_zero: u64,
}

/// One: at a whole phase and a constant quantum the remainder is on the grid
/// at every radix, exponent and mode, it is the value-index integer remainder,
/// and its sign and bound follow the mode. Two controls: the remainder formed
/// from the quotient on the grid (option b's) agrees exactly at exponent zero
/// and phase zero and departs elsewhere; and the range is left by the modes the
/// claim says leave it, so the range check is seen to fire.
fn whole_phase() {
    let mut per_mode: Vec<(Mode, Tally)> = MODES.iter().map(|&m| (m, Tally::default())).collect();
    let mut leaves_by: Vec<(Mode, &str, u64)> = Vec::new();
    for (g, family) in whole_phase_grids() {
        let zero_is_member = (g.min ..= g.max).contains(&(-g.pn));
        for (m, t) in per_mode.iter_mut() {
            let m = *m;
            let mut leaves = 0u64;
            for sa in g.members() {
                for sb in g.members() {
                    let (a, b) = (g.value(sa), g.value(sb));
                    if b.n == 0 {
                        continue;
                    }
                    t.pairs += 1;
                    let (q, r) = remainder(a, b, m);
                    let at = g.position(r);
                    if !at.is_int() {
                        t.off_grid += 1;
                        continue;
                    }
                    // The value-index form: `k = n + s`, and the slot of `r` is
                    // `k_a - k_b q - n`, read with no radix and no exponent.
                    let ka = g.pn + sa;
                    let kb = g.pn + sb;
                    let qk = round(Q::new(ka, kb), quotient_mode(m));
                    if qk != q || at.n != ka - kb * qk - g.pn {
                        t.slot_formula += 1;
                    }
                    let nearest = matches!(m, Mode::HalfUp | Mode::HalfEven);
                    let bound_ok = if nearest {
                        r.abs().le(b.abs().mul(Q::new(1, 2)))
                    } else {
                        r.abs().lt(b.abs())
                    };
                    if !bound_ok {
                        t.past_bound += 1;
                    }
                    let sign_ok = r.n == 0
                        || match m {
                            Mode::TowardZero => r.sign() == a.sign(),
                            Mode::Floor | Mode::StochasticDown => r.sign() == b.sign(),
                            Mode::Ceil | Mode::StochasticUp => r.sign() == -b.sign(),
                            _ => true,
                        };
                    if !sign_ok {
                        t.wrong_sign += 1;
                    }
                    if !(g.min ..= g.max).contains(&at.n) {
                        leaves += 1;
                        // Toward zero lies between 0 and a, the floor between 0
                        // and b: both are members wherever the zero is.
                        if zero_is_member
                            && matches!(m, Mode::TowardZero | Mode::Floor | Mode::StochasticDown)
                        {
                            t.out_of_range += 1;
                        }
                        // At phase zero a two's complement range is closed under
                        // every mode: `|r| < |b|`, and `-MIN` is past the top by one.
                        if g.pn == 0 && family == "signed" {
                            t.out_of_range += 1;
                        }
                    }
                    // The quotient as a member of the grid: is `q` a value of it?
                    let qpos = g.position(Q::int(q));
                    if !(qpos.is_int() && (g.min ..= g.max).contains(&qpos.n)) {
                        t.quotient_on_s += 1;
                    }
                    // Option b's remainder: the quotient rounded onto this grid by
                    // the same mode, then `a - b q`.
                    let qs_pos = round(g.position(a.div(b)), m);
                    let qs = g.value(qs_pos);
                    let rb = a.sub(b.mul(qs));
                    if rb == r {
                        t.b_agrees += 1
                    } else {
                        t.b_departs += 1
                    }
                    if g.e == 0 && g.pn == 0 && rb != r {
                        t.b_departs_at_zero += 1;
                    }
                }
            }
            if leaves > 0 {
                match leaves_by
                    .iter_mut()
                    .find(|(lm, lf, _)| *lm == m && *lf == family)
                {
                    Some(x) => x.2 += leaves,
                    None => leaves_by.push((m, family, leaves)),
                }
            }
        }
    }
    println!(
        "== one: whole phase, constant quantum, radix 2, 3, 10, e -3..=2, phase 0, 1, -2, w 1..=5"
    );
    for (m, t) in &per_mode {
        println!("{m:?}: {t:?}");
    }
    println!(
        "pairs whose remainder leaves the slot range, by mode and family (zero member or not):"
    );
    for (m, f, n) in &leaves_by {
        println!("  {m:?} {f}: {n}");
    }
    for (m, t) in &per_mode {
        assert!(t.pairs > 0);
        assert_eq!(
            t.off_grid, 0,
            "{m:?}: a remainder off the grid at a whole phase"
        );
        assert_eq!(t.slot_formula, 0, "{m:?}: the value-index form disagrees");
        assert_eq!(t.past_bound, 0, "{m:?}: a remainder past its bound");
        assert_eq!(t.wrong_sign, 0, "{m:?}: a remainder of the wrong sign");
        assert_eq!(t.out_of_range, 0, "{m:?}: left a range that holds the zero");
        assert!(
            t.quotient_on_s > 0,
            "{m:?}: the integer quotient was always a member"
        );
        assert!(
            t.b_departs > 0,
            "{m:?}: option b never departed, so the control is blind"
        );
        assert_eq!(
            t.b_departs_at_zero, 0,
            "{m:?}: option b at exponent zero and phase zero is the remainder"
        );
    }
    // The control on the range check: unsigned ranges are left under the
    // ceiling and the nearest modes.
    for m in [Mode::Ceil, Mode::HalfUp, Mode::HalfEven, Mode::StochasticUp] {
        assert!(
            leaves_by
                .iter()
                .any(|(lm, f, _)| *lm == m && *f == "unsigned"),
            "{m:?} never left an unsigned range, so the range check is blind"
        );
    }
}

/// The integer quotient's span is the slots' quotient's, whatever the radix and
/// the exponent: at phase zero, `max |q|` over a signed `w`-bit range is
/// `2^(w-1)`, `MIN / -1`, and over the unsigned one `2^w - 1`.
fn quotient_span() {
    println!("== the integer quotient's span, phase zero");
    for w in 1 ..= 6 {
        for (signed, want) in [(true, 1i128 << (w - 1)), (false, (1i128 << w) - 1)] {
            let mut seen = HashSet::new();
            for r in [2, 3, 10] {
                for e in -3 ..= 2 {
                    let g = if signed {
                        Grid::signed(w, r, e, 0, 1)
                    } else {
                        Grid::unsigned(w, r, e, 0, 1)
                    };
                    let mut top = 0;
                    for sa in g.members() {
                        for sb in g.members() {
                            if sb == 0 {
                                continue;
                            }
                            for m in MODES {
                                let (q, _) = remainder(g.value(sa), g.value(sb), m);
                                top = top.max(q.abs());
                            }
                        }
                    }
                    seen.insert(top);
                }
            }
            println!("  w {w} signed {signed}: max |q| {:?}", seen);
            assert_eq!(seen.len(), 1, "the span moved with the radix or exponent");
            assert!(seen.contains(&want));
        }
    }
    // The ticket's example: `Fix<8, -4>`, 7.9375 by 0.0625.
    let g = Grid::signed(8, 2, -4, 0, 1);
    let (q, r) = remainder(g.value(127), g.value(1), Mode::HalfEven);
    println!(
        "  Fix<8,-4>: 7.9375 / 0.0625 gives q {q}, r {r:?}; q at position {:?}",
        g.position(Q::int(q))
    );
    assert_eq!((q, r), (127, Q::int(0)));
    assert!(g.position(Q::int(q)).n > g.max);
    let (q, _) = remainder(g.value(-128), g.value(-1), Mode::TowardZero);
    assert_eq!(q, 128);
}

/// Two: at a phase whose reduced denominator `d` is past one, the remainder is
/// on the grid exactly where `d` divides `q`, and every format has a pair where
/// it is not.
fn fractional_phase() {
    println!("== two: fractional phase");
    let mut on = 0u64;
    let mut off = 0u64;
    for (pn, pd) in [(1, 2), (-1, 2), (1, 3), (-2, 3), (3, 4), (1, 5)] {
        for r in [2, 10] {
            for e in [-2, 0, 1] {
                for w in 1 ..= 5 {
                    for g in [Grid::signed(w, r, e, pn, pd), Grid::unsigned(w, r, e, pn, pd)] {
                        let mut format_off = 0;
                        for sa in g.members() {
                            for sb in g.members() {
                                let (a, b) = (g.value(sa), g.value(sb));
                                assert!(b.n != 0, "no slot names the zero past a whole phase");
                                for m in MODES {
                                    let (q, rem) = remainder(a, b, m);
                                    let on_grid = g.position(rem).is_int();
                                    assert_eq!(on_grid, q % pd == 0, "{g:?} {sa} {sb} {m:?}");
                                    if on_grid {
                                        on += 1
                                    } else {
                                        off += 1;
                                        format_off += 1
                                    }
                                    // The exact remainder is on the whole-phase grid of a
                                    // quantum `q / d`, the value-index lattice.
                                    let fine = rem.div(g.quantum()).mul(Q::int(pd));
                                    assert!(fine.is_int());
                                }
                            }
                        }
                        assert!(format_off > 0, "{g:?}: every remainder landed on the grid");
                    }
                }
            }
        }
    }
    println!("  on the grid {on}, off it {off}");
}

/// Three: the composition from the shipped quotient. Rounding `a / b` onto the
/// grid by the mode and then onto the integers by the same mode is the integer
/// rounding for the directed modes at a negative exponent. For the nearest
/// modes it misses exactly where the radix is even and the range holds an odd
/// divisor slot `s` with `|s| >= r^-e`: then `a / b` can sit `1 / (2 |s|)` from
/// a half, within the grid's half quantum, and the first rounding moves it onto
/// the tie, which is a grid point only at an even radix. At a
/// positive exponent the grid's quotient is coarser than the integers and no
/// second rounding recovers `q`, under every mode.
fn double_rounding() {
    println!("== three: the integer quotient through the grid's quotient, phase zero");
    for (r, e) in [(2, -3), (2, -1), (10, -1), (10, -2), (3, -2), (2, 1), (10, 1)] {
        for w in 1 ..= 8 {
            for g in [Grid::signed(w, r, e, 0, 1), Grid::unsigned(w, r, e, 0, 1)] {
                let mut row = Vec::new();
                for m in MODES {
                    let mut wrong = 0u64;
                    for sa in g.members() {
                        for sb in g.members() {
                            if sb == 0 {
                                continue;
                            }
                            let (a, b) = (g.value(sa), g.value(sb));
                            let x = a.div(b);
                            let on_grid = g.value(round(g.position(x), m));
                            if round(on_grid, m) != round(x, m) {
                                wrong += 1;
                            }
                        }
                    }
                    row.push((m, wrong));
                }
                let reach = if e < 0 { r.pow((-e) as u32) } else { 0 };
                // The tie `k + 1/2` is a point of the grid only where `r^-e` is even,
                // which at a negative exponent is an even radix: at an odd radix the
                // half lies between two grid points, the first rounding keeps the
                // side of it, and nothing misses.
                let odd_far = r % 2 == 0 && g.members().any(|s| s % 2 != 0 && s.abs() >= reach);
                if w == 8 || (e < 0 && w == 4) {
                    println!("  radix {r} e {e} {:?}: {row:?}", (g.min, g.max));
                }
                for (m, wrong) in row {
                    let directed = !matches!(m, Mode::HalfUp | Mode::HalfEven);
                    if e < 0 && directed {
                        assert_eq!(wrong, 0, "{m:?} at e {e}");
                    } else if e < 0 {
                        assert_eq!(
                            wrong > 0,
                            odd_far,
                            "{m:?} at radix {r} e {e} over {:?}",
                            (g.min, g.max)
                        );
                    } else if g.max >= 2 {
                        assert!(
                            wrong > 0,
                            "{m:?} at radix {r} e {e}: expected the composition to miss"
                        );
                    }
                }
            }
        }
    }
}

/// Wrap and saturate a slot onto `[min, max]`.
fn wrap(s: I, g: &Grid) -> I {
    let span = g.max - g.min + 1;
    g.min + (s - g.min).rem_euclid(span)
}
fn sat(s: I, g: &Grid) -> I {
    s.clamp(g.min, g.max)
}

/// Four: the composition from the shipped product and difference on the
/// operands' own signature, `a - (b q)` with `b q` landed first. Toward zero
/// never leaves the range in the intermediate; the floor, the ceiling and the
/// nearest modes do, and under saturation the composed answer is then wrong
/// where the true remainder is a member. Under wrapping it is right wherever the
/// true remainder is a member, because the wrap is a ring map on the slots.
fn composed_on_the_signature() {
    println!("== four: a - (b q) composed on the operands' signature, phase zero, every mode");
    for policy in ["saturate", "wrap"] {
        let mut row = Vec::new();
        for m in MODES {
            let mut wrong = 0u64;
            for (r, e) in [(2, -2), (2, 0), (10, -1), (3, 1)] {
                for w in 1 ..= 5 {
                    for g in [Grid::signed(w, r, e, 0, 1), Grid::unsigned(w, r, e, 0, 1)] {
                        for sa in g.members() {
                            for sb in g.members() {
                                if sb == 0 {
                                    continue;
                                }
                                let (q, rem) = remainder(g.value(sa), g.value(sb), m);
                                let true_slot = g.position(rem).n;
                                if !(g.min ..= g.max).contains(&true_slot) {
                                    continue;
                                }
                                let land =
                                    |s: I| if policy == "wrap" { wrap(s, &g) } else { sat(s, &g) };
                                let bq = land(sb * q);
                                let got = land(sa - bq);
                                if got != true_slot {
                                    wrong += 1;
                                }
                            }
                        }
                    }
                }
            }
            row.push((m, wrong));
        }
        println!("  {policy}: {row:?}");
        for (m, wrong) in row {
            if policy == "wrap" || m == Mode::TowardZero {
                assert_eq!(wrong, 0, "{policy} {m:?}");
            } else {
                assert!(
                    wrong > 0,
                    "{policy} {m:?}: expected the saturated intermediate to miss"
                );
            }
        }
    }
}

/// Five: the Euclidean remainder, `r >= 0` whatever the signs, is the floor's
/// where the divisor is positive and the ceiling's where it is negative. At a
/// whole phase every divisor door fixes that sign at build, so it is a constant
/// choice between two modes, not a branch on a value.
fn euclid() {
    println!("== five: the Euclidean remainder by a divisor of one sign");
    let mut n = 0u64;
    for (r, e) in [(2, -2), (10, 0), (3, 1)] {
        for w in 1 ..= 5 {
            let g = Grid::signed(w, r, e, 0, 1);
            for sa in g.members() {
                for sb in g.members() {
                    if sb == 0 {
                        continue;
                    }
                    let (a, b) = (g.value(sa), g.value(sb));
                    let rq = round(a.div(b.abs()), Mode::Floor);
                    let re = a.sub(b.abs().mul(Q::int(rq)));
                    assert!(re.n >= 0);
                    let by_sign = if sb > 0 { Mode::Floor } else { Mode::Ceil };
                    assert_eq!(remainder(a, b, by_sign).1, re);
                    n += 1;
                }
            }
        }
    }
    println!("  {n} pairs agree");
}

/// A toy floating format: radix `r`, `p` digits, exponents `emin..=emax`, with
/// the subnormals at `emin`. Its finite members as exact rationals.
fn floats(r: I, p: u32, emin: i32, emax: i32) -> Vec<Q> {
    let mut out = vec![Q::int(0)];
    let top = r.pow(p);
    for e in emin ..= emax {
        let lo = if e == emin { 1 } else { r.pow(p - 1) };
        for m in lo .. top {
            let v =
                if e >= 0 { Q::int(m * r.pow(e as u32)) } else { Q::new(m, r.pow((-e) as u32)) };
            out.push(v);
            out.push(Q::new(-v.n, v.d));
        }
    }
    out
}

/// Six: over a magnitude-indexed quantum the remainder is a member for toward
/// zero and the nearest modes at every pair, and not for the floor or the
/// ceiling. The control: the same checker finds sums that are not members.
fn magnitude_indexed() {
    println!("== six: magnitude-indexed quantum, toy floats");
    for (r, p, emin, emax) in [
        (2, 2, -2, 2),
        (2, 3, -3, 2),
        (2, 4, -2, 2),
        (10, 1, -1, 1),
        (10, 2, -1, 1),
        (3, 2, -2, 1),
    ] {
        let set = floats(r, p, emin, emax);
        let members: HashSet<Q> = set.iter().copied().collect();
        let is_member =
            |v: Q| cfg!(feature = "plant-float-checker-accepts-all") || members.contains(&v);
        let mut sums_off = 0u64;
        let mut row = Vec::new();
        for m in MODES {
            let mut missed = 0u64;
            for &a in &set {
                for &b in &set {
                    if b.n == 0 {
                        continue;
                    }
                    let (_, rem) = remainder(a, b, m);
                    if !is_member(rem) {
                        missed += 1;
                    }
                }
            }
            row.push((m, missed));
        }
        for &a in &set {
            for &b in &set {
                if !is_member(a.add(b)) {
                    sums_off += 1;
                }
            }
        }
        println!(
            "  radix {r} p {p} e {emin}..={emax}, {} members: {row:?}; sums not members {sums_off}",
            set.len()
        );
        assert!(
            sums_off > 0,
            "the membership check accepts every sum, so it is blind"
        );
        for (m, missed) in row {
            match m {
                Mode::TowardZero | Mode::HalfUp | Mode::HalfEven => {
                    assert_eq!(missed, 0, "{m:?} radix {r} p {p}")
                },
                _ => {
                    assert!(
                        missed > 0,
                        "{m:?} radix {r} p {p}: expected a remainder off the set"
                    )
                },
            }
        }
    }
}

fn main() {
    whole_phase();
    quotient_span();
    fractional_phase();
    double_rounding();
    composed_on_the_signature();
    euclid();
    magnitude_indexed();
    println!("every claim held, every control fired");
}
