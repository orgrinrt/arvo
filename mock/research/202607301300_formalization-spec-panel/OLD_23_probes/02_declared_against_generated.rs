//! PROBE 2: the model the verifier is pointed at.
//!
//! Two axes, both of which a body can lie about, and one operation per lie.
//!
//! `Fuse` is a Policy-side liberty: `Relaxed` permits the single-rounded fused
//! form, `Strict` does not. This is the one liberty on this axis reachable from
//! ordinary Rust, since `f64::mul_add` lowers to `llvm.fmuladd` and nothing
//! else in safe stable Rust emits a fast-math flag at all.
//!
//! `Layout` is a Lowering claim: `Bitpacked` says loads go through a shift and
//! a mask, `Dense` says they do not.
//!
//! Build twice. Clean, every body does what its axes declare. With `--cfg
//! underclaim`, `dot` fuses regardless of what Policy says, which is exactly
//! `19_probes/01_liberties_disconnected_from_body.rs`'s finding: a declared
//! fact and an executed body, authored independently and checked against
//! nothing. `--cfg swapped` gives the liberty to the wrong instance, which
//! keeps the axis live and is the case the rule-free check cannot see.
//! With `--cfg overclaim`, `load` ignores Layout and reads densely
//! under both, so `Bitpacked` becomes a promise with nothing behind it.
//!
//! Neither cfg changes a single type, a single bound, or a single declared
//! axis. Both compile clean. That is the hole.

#![crate_type = "lib"]
#![allow(dead_code)]

pub trait Fidelity {
    const FUSE: bool;
}
pub trait Layout {
    const PACKED: bool;
}

pub struct Strict;
pub struct Relaxed;
pub struct Dense;
pub struct Bitpacked;

impl Fidelity for Strict {
    const FUSE: bool = false;
}
impl Fidelity for Relaxed {
    const FUSE: bool = true;
}
impl Layout for Dense {
    const PACKED: bool = false;
}
impl Layout for Bitpacked {
    const PACKED: bool = true;
}

/// Policy-visible: fusing changes the answer, so the licence has to be real.
pub fn dot<const I: u16, const F: u16, P: Fidelity, L: Layout>(a: &[f64], b: &[f64]) -> f64 {
    let n = if a.len() < b.len() { a.len() } else { b.len() };
    let mut acc = 0.0f64;
    let mut i = 0;
    while i < n {
        #[cfg(swapped)]
        {
            if P::FUSE {
                acc += a[i] * b[i];
            } else {
                acc = a[i].mul_add(b[i], acc);
            }
        }
        #[cfg(not(any(underclaim, swapped)))]
        {
            if P::FUSE {
                acc = a[i].mul_add(b[i], acc);
            } else {
                acc += a[i] * b[i];
            }
        }
        #[cfg(underclaim)]
        {
            acc = a[i].mul_add(b[i], acc);
        }
        // `--cfg swapped`: the axis still changes generated code, so the
        // rule-free check in probe 5 sees nothing. It changes it the wrong
        // way round, which only a rule about what a liberty IS can catch.
        i += 1;
    }
    acc
}

/// Lowering-visible: the layout claim is about generated loads, nothing else.
pub fn load<const I: u16, const F: u16, P: Fidelity, L: Layout>(words: &[u64], idx: usize) -> u64 {
    #[cfg(not(overclaim))]
    {
        if L::PACKED {
            let bits = (I + F) as usize;
            let at = idx * bits;
            let w = words[at / 64];
            (w >> (at % 64)) & ((1u64 << bits) - 1)
        } else {
            words[idx]
        }
    }
    #[cfg(overclaim)]
    {
        let _ = (I, F);
        words[idx]
    }
}

macro_rules! instantiate {
    ($d:ident, $l:ident, $i:expr, $f:expr, $p:ty, $lay:ty) => {
        #[no_mangle]
        pub extern "C" fn $d(a: &[f64], b: &[f64]) -> f64 {
            dot::<$i, $f, $p, $lay>(a, b)
        }
        #[no_mangle]
        pub extern "C" fn $l(w: &[u64], i: usize) -> u64 {
            load::<$i, $f, $p, $lay>(w, i)
        }
    };
}

instantiate!(d0, l0, 3, 5, Strict, Dense);
instantiate!(d1, l1, 3, 5, Strict, Bitpacked);
instantiate!(d2, l2, 3, 5, Relaxed, Dense);
instantiate!(d3, l3, 3, 5, Relaxed, Bitpacked);
instantiate!(d4, l4, 7, 9, Strict, Dense);
instantiate!(d5, l5, 7, 9, Strict, Bitpacked);
instantiate!(d6, l6, 7, 9, Relaxed, Dense);
instantiate!(d7, l7, 7, 9, Relaxed, Bitpacked);
