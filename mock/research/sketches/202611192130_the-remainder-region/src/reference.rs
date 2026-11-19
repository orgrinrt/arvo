//! The definition under test, in the sketch's own rationals, reading nothing
//! of arvo.
//!
//! A constant-quantum format is a radix `r`, an exponent `e`, a phase `h / d`
//! in units of the quantum, and a slot range `[min, max]`. The member at slot
//! `s` is `(h / d + s) r^e`.
//!
//! The remainder of the member `x` of `F` by the non-zero member `y` of `G`
//! under the rounding mode `m` (and a dither for the stochastic mode) is
//! `R = x - n y` with `n = round_m(x / y)` an integer. Its position on `F`'s
//! grid is `R / q_F - phase_F`; it is exact on the grid when that is whole.

use crate::rat::{Q, pow};

#[derive(Clone, Copy, Debug)]
pub struct Fmt {
    pub radix: i128,
    pub exp:   i32,
    pub ph_n:  i128,
    pub ph_d:  i128,
    pub min:   i128,
    pub max:   i128,
}

impl Fmt {
    pub fn q(self) -> Q {
        pow(self.radix, self.exp)
    }

    pub fn phase(self) -> Q {
        Q::new(self.ph_n, self.ph_d)
    }

    pub fn value(self, s: i128) -> Q {
        self.phase().add(Q::int(s)).mul(self.q())
    }

    pub fn signed(self) -> bool {
        self.min < 0
    }

    pub fn width_signed(w: u32) -> (i128, i128) {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    }

    pub fn width_unsigned(w: u32) -> (i128, i128) {
        (0, (1i128 << w) - 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M {
    TowardZero,
    Floor,
    Ceil,
    HalfUp,
    HalfEven,
    Stochastic,
}

pub const MODES: [M; 6] = [M::TowardZero, M::Floor, M::Ceil, M::HalfUp, M::HalfEven, M::Stochastic];

/// The dithers a mode reads, in quarters: four for the stochastic mode.
pub fn dithers(m: M) -> core::ops::RangeInclusive<i128> {
    if m == M::Stochastic { 0 ..= 3 } else { 0 ..= 0 }
}

/// `t` rounded to an integer by `m`, the stochastic mode as `ceil(t - k/4)`,
/// which is the format oracle's stated rule.
pub fn round(t: Q, m: M, k: i128) -> i128 {
    match m {
        M::Floor => t.floor(),
        M::Ceil => t.ceil(),
        M::TowardZero => {
            if t.n < 0 {
                t.ceil()
            } else {
                t.floor()
            }
        },
        M::HalfUp => t.add(Q::new(1, 2)).floor(),
        M::HalfEven => {
            let f = t.floor();
            let frac = t.sub(Q::int(f));
            let half = Q::new(1, 2);
            if frac.lt(half) {
                f
            } else if half.lt(frac) {
                f + 1
            } else if f.rem_euclid(2) == 0 {
                f
            } else {
                f + 1
            }
        },
        M::Stochastic => t.sub(Q::new(k, 4)).ceil(),
    }
}

/// The count and the remainder's exact position on `F`'s grid, in slot units.
pub fn remainder(f: Fmt, g: Fmt, a: i128, b: i128, m: M, k: i128) -> (i128, Q) {
    let x = f.value(a);
    let y = g.value(b);
    let n = round(x.div(y), m, k);
    let r = x.sub(Q::int(n).mul(y));
    (n, r.div(f.q()).sub(f.phase()))
}

/// The lattice condition: every member of `G` is a whole multiple of `q_F`.
/// With `G`'s phase `h / d` reduced, that is `q_G / q_F` a whole multiple of `d`.
pub fn lattice(f: Fmt, g: Fmt) -> bool {
    let k = g.q().div(f.q());
    let d = g.phase().d;
    k.is_int() && k.n % d == 0
}
