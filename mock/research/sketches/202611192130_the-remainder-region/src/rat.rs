//! Exact rationals over i128, reduced, positive denominator. Independent of
//! every arvo type: this is the sketch's own arithmetic.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Q {
    pub n: i128,
    pub d: i128,
}

pub fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

impl Q {
    pub fn new(n: i128, d: i128) -> Q {
        assert!(d != 0, "zero denominator");
        let s = if d < 0 { -1 } else { 1 };
        let g = gcd(n, d).max(1);
        Q {
            n: s * n / g,
            d: s * d / g,
        }
    }

    pub fn int(n: i128) -> Q {
        Q {
            n,
            d: 1,
        }
    }

    pub fn add(self, o: Q) -> Q {
        Q::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }

    pub fn sub(self, o: Q) -> Q {
        Q::new(self.n * o.d - o.n * self.d, self.d * o.d)
    }

    pub fn mul(self, o: Q) -> Q {
        Q::new(self.n * o.n, self.d * o.d)
    }

    pub fn div(self, o: Q) -> Q {
        assert!(o.n != 0, "division by zero in the sketch's own arithmetic");
        Q::new(self.n * o.d, self.d * o.n)
    }

    pub fn is_int(self) -> bool {
        self.d == 1
    }

    pub fn floor(self) -> i128 {
        self.n.div_euclid(self.d)
    }

    pub fn ceil(self) -> i128 {
        -(Q {
            n: -self.n,
            d: self.d,
        })
        .floor()
    }

    pub fn abs(self) -> Q {
        Q {
            n: self.n.abs(),
            d: self.d,
        }
    }

    pub fn lt(self, o: Q) -> bool {
        self.n * o.d < o.n * self.d
    }

    pub fn le(self, o: Q) -> bool {
        self.n * o.d <= o.n * self.d
    }

    pub fn is_zero(self) -> bool {
        self.n == 0
    }
}

pub fn pow(r: i128, e: i32) -> Q {
    if e >= 0 { Q::int(r.pow(e as u32)) } else { Q::new(1, r.pow((-e) as u32)) }
}
