//! P8. What does a bare `T: Add<Output = T>` determine about T?
//!
//! Two halves.
//!
//! (a) Totality is a real inference. A strategy whose arithmetic refuses
//!     cannot satisfy `Add<Output = Self>`, because the operation's result
//!     type is the refusing wrapper rather than Self. So the bound excludes
//!     it, and the exclusion is checked by the compiler rather than asserted.
//!
//! (b) Width, resolution, container and signedness are determined by nothing
//!     in the bound. The second half of the file states the negative by
//!     asking for one of them through the bound and being refused.
//!
//! Expected: (a) refused at the impl-satisfaction site, (b) refused at E0220
//! or equivalent, both naming what is absent.

#![no_std]

use core::ops::Add;

pub struct Refused;

/// A strategy whose out-of-range behaviour is to refuse. Its addition cannot
/// return Self, so it implements Add with a different Output.
#[derive(Clone, Copy)]
pub struct PreciseNum(pub u32);

impl Add for PreciseNum {
    type Output = Result<PreciseNum, Refused>;
    fn add(self, rhs: Self) -> Self::Output {
        self.0.checked_add(rhs.0).map(PreciseNum).ok_or(Refused)
    }
}

/// A strategy whose out-of-range behaviour is total.
#[derive(Clone, Copy)]
pub struct WarmNum(pub u32);

impl Add for WarmNum {
    type Output = WarmNum;
    fn add(self, rhs: Self) -> WarmNum {
        WarmNum(self.0.wrapping_add(rhs.0))
    }
}

/// The public API shape op describes: an algorithm over a contract.
pub fn fold_total<T: Add<Output = T> + Copy>(xs: &[T], init: T) -> T {
    let mut acc = init;
    let mut i = 0;
    while i < xs.len() {
        acc = acc + xs[i];
        i += 1;
    }
    acc
}

/// (a) The total strategy satisfies it.
pub fn warm_is_admitted(xs: &[WarmNum]) -> WarmNum {
    fold_total(xs, WarmNum(0))
}

/// (a) The refusing strategy does not. The bound is what refuses it.
pub fn precise_is_refused(xs: &[PreciseNum]) -> PreciseNum {
    fold_total(xs, PreciseNum(0))
}
