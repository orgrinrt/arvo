// A `Ranged` numeral, exactly as `50_fog_the_float_model.md` section 1 states it,
// re-implemented here so this file's checks do not inherit that file's code.
//
// A value is carried as an exact pair (m, q) denoting m * r^q, with m an i128.
// No float appears anywhere. Every comparison and every sum is exact.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fmt {
    pub r: i128,
    pub p: u32,
    pub emin: i32,
    pub emax: i32,
    pub gradual: bool,
}

impl Fmt {
    pub fn span(&self) -> i32 {
        self.emax - self.emin + 1
    }
    /// The quantum exponent of the grid selected for a value of exponent e.
    /// This is `50_probes/model.rs:39-47`'s `quantum_exp`, restated.
    pub fn quantum_exp(&self, e: i32) -> i32 {
        let unfloored = e - self.p as i32 + 1;
        let floor = self.emin - self.p as i32 + 1;
        if self.gradual && unfloored < floor {
            floor
        } else if !self.gradual && unfloored < floor {
            floor
        } else {
            unfloored
        }
    }
}

pub fn ipow(r: i128, k: u32) -> i128 {
    let mut acc: i128 = 1;
    for _ in 0..k {
        acc *= r;
    }
    acc
}

/// Exact value: m * r^q. Normalised for comparison by scaling to a common q.
#[derive(Clone, Copy, Debug)]
pub struct Val {
    pub m: i128,
    pub q: i32,
}

impl Val {
    pub fn zero() -> Val {
        Val { m: 0, q: 0 }
    }
    /// Rescale to exponent `tq <= self.q`. Exact; panics on overflow rather
    /// than wrapping, so a silently wrong comparison is impossible.
    pub fn at(&self, r: i128, tq: i32) -> i128 {
        assert!(tq <= self.q, "rescale must not lose digits");
        let k = (self.q - tq) as u32;
        self.m.checked_mul(ipow(r, k)).expect("rescale overflow")
    }
    pub fn cmp_exact(&self, other: &Val, r: i128) -> core::cmp::Ordering {
        let tq = self.q.min(other.q);
        self.at(r, tq).cmp(&other.at(r, tq))
    }
    pub fn eq_exact(&self, other: &Val, r: i128) -> bool {
        self.cmp_exact(other, r) == core::cmp::Ordering::Equal
    }
    pub fn add_exact(&self, other: &Val, r: i128) -> Val {
        let tq = self.q.min(other.q);
        Val {
            m: self.at(r, tq) + other.at(r, tq),
            q: tq,
        }
    }
    pub fn mul_exact(&self, other: &Val, _r: i128) -> Val {
        Val {
            m: self.m * other.m,
            q: self.q + other.q,
        }
    }
    pub fn is_zero(&self) -> bool {
        self.m == 0
    }
}

/// Every value of the numeral, non-negative half, ascending, zero first.
pub fn enumerate(f: &Fmt) -> Vec<Val> {
    let mut out = vec![Val::zero()];
    if f.gradual {
        // the bottom grid extended down to zero: m in [1, r^(p-1))
        let q = f.emin - f.p as i32 + 1;
        for m in 1..ipow(f.r, f.p - 1) {
            out.push(Val { m, q });
        }
    }
    for e in f.emin..=f.emax {
        let q = e - f.p as i32 + 1;
        for m in ipow(f.r, f.p - 1)..ipow(f.r, f.p) {
            out.push(Val { m, q });
        }
    }
    out
}

/// floor(log_r |v|) for a nonzero exact value.
pub fn exponent_of(v: &Val, r: i128) -> i32 {
    assert!(!v.is_zero());
    let mut m = v.m.abs();
    let mut e = v.q;
    // shift up while below 1
    while m >= r {
        m /= r;
        e += 1;
    }
    // m is now in [1, r); note the division above floors, which is what
    // floor(log_r) needs for a positive value.
    e
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Value,
    Overflow,
    /// `Underflow = Abrupt` refuses rather than delivering: the exact value's
    /// magnitude fell strictly inside the hole (0, r^emin).
    UnderflowRefused,
}

/// Round-first, classify second, exactly as the design states it.
/// Round-half-even on the selected grid.
pub fn quantise(f: &Fmt, v: &Val) -> (Val, Outcome) {
    if v.is_zero() {
        return (Val::zero(), Outcome::Value);
    }
    let neg = v.m < 0;
    let a = Val {
        m: v.m.abs(),
        q: v.q,
    };
    let e = exponent_of(&a, f.r);
    let qt = f.quantum_exp(e);
    // round a to a multiple of r^qt, half-even
    let rounded_m = if qt <= a.q {
        a.at(f.r, qt) // exact, no rounding needed
    } else {
        let k = (qt - a.q) as u32;
        let d = ipow(f.r, k);
        let n = a.m;
        let lo = n / d;
        let rem = n % d;
        let twice = 2 * rem;
        if twice > d || (twice == d && lo % 2 == 1) {
            lo + 1
        } else {
            lo
        }
    };
    let mut out = Val {
        m: rounded_m,
        q: qt,
    };
    // classify
    if out.is_zero() {
        return (Val::zero(), Outcome::Value);
    }
    let e2 = exponent_of(&out, f.r);
    if e2 > f.emax {
        return (out, Outcome::Overflow);
    }
    if e2 < f.emin {
        if f.gradual {
            // representable as a subnormal; the grid already floored
        } else {
            return (out, Outcome::UnderflowRefused);
        }
    }
    if neg {
        out.m = -out.m;
    }
    (out, Outcome::Value)
}
