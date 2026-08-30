//! Probe 8. Whether the truth contract has a second implementor already in the
//! tree, which is what decides whether branch B's genericity is load-bearing or
//! speculative.
//!
//! `MaskOps` (arvo-mask-contracts/src/lib.rs:45-66) is a Boolean algebra under
//! a set-theoretic vocabulary: empty/full/union/intersection/complement are
//! FALSE/TRUE/or/and/not. Modelled here at width 64. If one predicate
//! declaration serves both the scalar and the lane-wise truth type, the fork's
//! second branch buys something rather than costing something.
#![no_std]
#![feature(const_trait_impl)]
#![crate_type = "lib"]
#![allow(dead_code)]

use p1_arvo::Bool;
use p1_foundation::{Cons, Nil, Pred, Truth};

type L2<A, B> = Cons<A, Cons<B, Nil>>;

/// The lane-wise truth type, modelling `Mask<64>`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Mask64(u64);

impl Mask64 {
    #[inline(always)]
    pub const fn new(m: u64) -> Self {
        Mask64(m)
    }
    #[inline(always)]
    pub const fn bits(self) -> u64 {
        self.0
    }
}

const impl Truth for Mask64 {
    const TRUE: Self = Mask64(u64::MAX);
    const FALSE: Self = Mask64(0);
    #[inline(always)]
    fn not(self) -> Self {
        Mask64(!self.0)
    }
    #[inline(always)]
    fn and(self, o: Self) -> Self {
        Mask64(self.0 & o.0)
    }
    #[inline(always)]
    fn or(self, o: Self) -> Self {
        Mask64(self.0 | o.0)
    }
}

/// One declaration, generic over the truth type. The scalar path and the
/// lane-wise path are the same source.
#[inline(always)]
pub fn all_hold<F, B>(items: &[u32], f: &Pred<L2<u32, u32>, F>, acc: &u32) -> B
where
    F: Fn(&u32, &u32) -> B,
    B: Truth,
{
    let mut out = B::TRUE;
    for it in items {
        out = out.and(f(acc, it));
    }
    out
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn scalar_path(items: &[u32], acc: u32) -> bool {
    let p: Pred<L2<u32, u32>, _> = Pred::new(|a: &u32, b: &u32| Bool::new(*a >= *b));
    all_hold::<_, Bool>(items, &p, &acc) == Bool::TRUE
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn lane_path(items: &[u32], acc: u32) -> u64 {
    let p: Pred<L2<u32, u32>, _> = Pred::new(|a: &u32, b: &u32| {
        // a lane-wise comparison producing a full/empty mask per element
        if *a >= *b {
            Mask64::TRUE
        } else {
            Mask64::FALSE
        }
    });
    all_hold::<_, Mask64>(items, &p, &acc).bits()
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
