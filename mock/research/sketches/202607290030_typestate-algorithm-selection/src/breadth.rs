#[derive(Clone, Copy, PartialEq)]
pub struct Bool(pub bool);

pub trait Pred2<A, B> {
    const MONOTONE: bool = false;
    const SORTED_OK: bool = false; // input assumed pre-sorted for this predicate
    const CHEAP: bool = true; // test is cheap enough to call per element
    fn test(&self, a: &A, b: &B) -> Bool;
}
impl<A, B, F: Fn(&A, &B) -> Bool> Pred2<A, B> for F {
    #[inline(always)]
    fn test(&self, a: &A, b: &B) -> Bool {
        self(a, b)
    }
}

pub struct Budget {
    pub cap: u32,
}
impl Pred2<u32, u32> for Budget {
    const MONOTONE: bool = true;
    const SORTED_OK: bool = true;
    #[inline(always)]
    fn test(&self, s: &u32, x: &u32) -> Bool {
        Bool(s + x <= self.cap)
    }
}
pub struct Parity;
impl Pred2<u32, u32> for Parity {
    #[inline(always)]
    fn test(&self, s: &u32, x: &u32) -> Bool {
        Bool((s + x).is_multiple_of(2))
    }
}
pub struct Costly {
    pub cap: u32,
}
impl Pred2<u32, u32> for Costly {
    const MONOTONE: bool = true;
    const CHEAP: bool = false;
    #[inline(always)]
    fn test(&self, s: &u32, x: &u32) -> Bool {
        let mut h = *s;
        for _ in 0..8 {
            h = h.wrapping_mul(31).wrapping_add(*x);
        }
        Bool(h % 97 < 50 && s + x <= self.cap)
    }
}

#[inline(always)]
fn galloping(v: &[u32], c: u32) -> usize {
    let mut i = 1;
    let mut s = 0u32;
    while i < v.len() && s + v[i] <= c {
        s += v[i];
        i *= 2;
    }
    i.min(v.len())
}
#[inline(always)]
fn bisect(v: &[u32], c: u32) -> usize {
    let (mut lo, mut hi) = (0, v.len());
    while lo < hi {
        let m = (lo + hi) / 2;
        let mut s = 0u32;
        for x in &v[..m] {
            s += x;
        }
        if s + v[m] <= c {
            lo = m + 1
        } else {
            hi = m
        }
    }
    lo
}
#[inline(always)]
fn scan(v: &[u32], c: u32) -> usize {
    let mut s = 0u32;
    for (i, x) in v.iter().enumerate() {
        if s + x > c {
            return i;
        }
        s += x;
    }
    v.len()
}
#[inline(always)]
fn batched(v: &[u32], c: u32) -> usize {
    let mut s = 0u32;
    let mut i = 0;
    while i + 4 <= v.len() {
        let b: u32 = v[i..i + 4].iter().sum();
        if s + b > c {
            break;
        }
        s += b;
        i += 4;
    }
    while i < v.len() {
        if s + v[i] > c {
            return i;
        }
        s += v[i];
        i += 1;
    }
    v.len()
}

/// THREE nested static branches, four algorithms.
#[inline(always)]
pub fn select<P: Pred2<u32, u32>>(v: &[u32], _p: &P, c: u32) -> usize {
    if P::MONOTONE {
        if P::SORTED_OK {
            galloping(v, c)
        } else {
            bisect(v, c)
        }
    } else {
        if P::CHEAP {
            scan(v, c)
        } else {
            batched(v, c)
        }
    }
}

pub fn p_budget(v: &[u32]) -> usize {
    select(v, &Budget { cap: 10 }, 10)
}
pub fn p_parity(v: &[u32]) -> usize {
    select(v, &Parity, 10)
}
pub fn p_costly(v: &[u32]) -> usize {
    select(v, &Costly { cap: 10 }, 10)
}
