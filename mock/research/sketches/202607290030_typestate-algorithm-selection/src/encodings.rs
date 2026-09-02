#[derive(Clone, Copy, PartialEq)]
pub struct Bool(pub bool);
pub trait Pred2<A, B> {
    const MONOTONE: bool = false;
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
    #[inline(always)]
    fn test(&self, s: &u32, x: &u32) -> Bool {
        Bool(s + x <= self.cap)
    }
}

#[inline(always)]
fn binary(items: &[u32], cap: u32) -> usize {
    let (mut lo, mut hi) = (0usize, items.len());
    while lo < hi {
        let m = (lo + hi) / 2;
        let mut s = 0u32;
        for x in &items[..m] {
            s += x;
        }
        if s + items[m] <= cap {
            lo = m + 1
        } else {
            hi = m
        }
    }
    lo
}
#[inline(always)]
fn linear(items: &[u32], cap: u32) -> usize {
    let mut s = 0u32;
    for (i, x) in items.iter().enumerate() {
        if s + x > cap {
            return i;
        }
        s += x;
    }
    items.len()
}

// 1. TYPESTATE: property on the type
pub fn ts(items: &[u32]) -> usize {
    fn inner<P: Pred2<u32, u32>>(it: &[u32], cap: u32) -> usize {
        if P::MONOTONE {
            binary(it, cap)
        } else {
            linear(it, cap)
        }
    }
    inner::<Budget>(items, 10)
}

// 2. CONST GENERIC bool
pub fn cg(items: &[u32]) -> usize {
    fn inner<const M: bool>(it: &[u32], cap: u32) -> usize {
        if M {
            binary(it, cap)
        } else {
            linear(it, cap)
        }
    }
    inner::<true>(items, 10)
}

// 3. CONST FN
const fn is_mono() -> bool {
    true
}
pub fn cf(items: &[u32]) -> usize {
    if is_mono() {
        binary(items, 10)
    } else {
        linear(items, 10)
    }
}

// 4. RUNTIME bool, crossing a real boundary (not inlinable away)
#[inline(never)]
fn inner_rt(it: &[u32], cap: u32, m: bool) -> usize {
    if m {
        binary(it, cap)
    } else {
        linear(it, cap)
    }
}
pub fn rt(items: &[u32]) -> usize {
    inner_rt(items, 10, true)
}
