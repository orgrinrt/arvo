//! Probe 4: probe 3's `Spec` const struct is readable, but nothing in it
//! refuses an unreduced bias the way the internal `Adjustment` trait does
//! (`N: Pos + Gcd<D, Out = H>`, `49:295-346`). The carrier-at-birth rule
//! (`49:74-87`) says a new closed vocabulary a guarantee quantifies over
//! owes its seal at declaration, not after three passes; a face carrying a
//! spec IS such a vocabulary the moment an operation's law is keyed on it.
//! This probe checks the cheapest available discharge: a blanket impl
//! guarded by a `const` assertion (ordinary integer gcd, not the type-level
//! `Pos`/`Gcd` tower, since `Spec`'s fields are plain integers) that panics
//! at evaluation rather than at the type-check pass a coherent-position
//! caller would reach.
//!
//! EXPECTED: unknown going in, specifically whether the panic fires only
//! when the const is USED (as file 46's own lesson about bare aliases
//! predicts) or at declaration.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   probe_4_the_face_is_a_new_carrier_and_needs_its_own_seal.rs

#![allow(dead_code)]
#![feature(adt_const_params)]
use core::marker::ConstParamTy;
use core::marker::PhantomData;

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Spec {
    pub precision: u16,
    pub bias_num: u64,
    pub bias_den: u64,
}

impl Spec {
    const fn is_reduced(&self) -> bool {
        gcd(self.bias_num, self.bias_den) == 1
    }
}

pub struct NFace<const S: Spec>(PhantomData<()>);

/// The seal: every `NFace<S>` must have its `AGREES` const forced at use,
/// the same discipline file 55 applied to `Capacity`'s two spellings
/// (`55:158-169`) and file 46 learned the hard way for a bare alias
/// (`46_probes/probe_3d`'s own finding, restated at `52:213-221`): an
/// unevaluated const assertion nobody touches is not evaluated.
impl<const S: Spec> NFace<S> {
    pub const REDUCED: () = assert!(S.is_reduced(), "bias is not reduced to lowest terms");
    pub const fn checked() -> Self {
        let () = Self::REDUCED;
        NFace(PhantomData)
    }
}

pub const GOOD: Spec = Spec {
    precision: 15,
    bias_num: 1,
    bias_den: 2,
};
pub const BAD: Spec = Spec {
    precision: 15,
    bias_num: 6,
    bias_den: 12,
}; // MATLAB's own witness, unreduced

pub const fn make_good() -> NFace<GOOD> {
    NFace::<GOOD>::checked()
}

// This line is the actual test: does declaring the BAD type alone compile,
// or does something have to touch REDUCED/checked() before it panics?
pub type UncheckedBad = NFace<BAD>;

#[allow(dead_code)]
fn _unused_make_bad_disabled() -> NFace<BAD> {
    NFace::<BAD>::checked()
}
