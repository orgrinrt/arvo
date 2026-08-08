//! p4. Does a two-endpoint numeral erase, and does it stay in registers.
//!
//! The dispatch says an interval-valued numeral that cannot erase is a real exclusion and
//! should be reported as one. That is a claim to test rather than assume, and it splits in two.
//!
//!   ERASURE. Does a typed pair of endpoints lower to the same instructions as a hand-written
//!   pair of the underlying containers? Checked by symbol folding in the assembler's own
//!   output, which is the instrument `17` section 0 identifies as the real one.
//!
//!   CALLING CONVENTION. A scalar numeral is passed in a register. A pair of endpoints is a
//!   two-field aggregate, and above some width an aggregate stops being passed in registers.
//!   That is a cost the erasure check cannot see, because both sides of a folded comparison
//!   would pay it equally.
//!
//! ZERO feature gates, and the check has to be anchored to the line start, because these two
//! comment lines mention the attribute and an unanchored grep counts them. The honest command
//! is `grep -c '^#!\[feature' p4_interval_erasure.rs`, which returns 0. The unanchored form
//! returns 2, and that is a probe self-check reporting its own prose.
//!
//! Build:
//!   rustc +nightly-2026-05-28 --edition 2021 -O --emit asm --crate-type lib \
//!     p4_interval_erasure.rs --out-dir asm

#![no_std]

/// Stand-in for the design's opaque storage primitive at one width.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Nat16(u16);

/// Stand-in for a numeral: a container plus a phantom shape the codegen must not see.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Num<const I: u8, const F: u8>(Nat16);

/// A two-endpoint datum. This is the shape the denotation clause excludes.
#[derive(Clone, Copy)]
pub struct Iv<const I: u8, const F: u8> {
    lo: Num<I, F>,
    hi: Num<I, F>,
}

// ---- the typed path -------------------------------------------------------

impl<const I: u8, const F: u8> Num<I, F> {
    #[inline]
    const fn add_down(self, other: Self) -> Self {
        Num(Nat16(self.0 .0.wrapping_add(other.0 .0)))
    }
    #[inline]
    const fn add_up(self, other: Self) -> Self {
        Num(Nat16(self.0 .0.wrapping_add(other.0 .0)))
    }
}

#[no_mangle]
pub fn p4_typed_interval_add(a: Iv<3, 13>, b: Iv<3, 13>) -> Iv<3, 13> {
    Iv {
        lo: a.lo.add_down(b.lo),
        hi: a.hi.add_up(b.hi),
    }
}

// ---- the hand-written path ------------------------------------------------

#[no_mangle]
pub fn p4_raw_pair_add(a: (u16, u16), b: (u16, u16)) -> (u16, u16) {
    (a.0.wrapping_add(b.0), a.1.wrapping_add(b.1))
}

// ---- the scalar control ---------------------------------------------------

#[no_mangle]
pub fn p4_typed_scalar_add(a: Num<3, 13>, b: Num<3, 13>) -> Num<3, 13> {
    a.add_down(b)
}

#[no_mangle]
pub fn p4_raw_scalar_add(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}

// ---- the wide case, where the aggregate stops fitting ---------------------

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Nat128(u128);

#[derive(Clone, Copy)]
pub struct IvWide {
    lo: Nat128,
    hi: Nat128,
}

#[no_mangle]
pub fn p4_wide_interval_add(a: IvWide, b: IvWide) -> IvWide {
    IvWide {
        lo: Nat128(a.lo.0.wrapping_add(b.lo.0)),
        hi: Nat128(a.hi.0.wrapping_add(b.hi.0)),
    }
}

#[no_mangle]
pub fn p4_wide_scalar_add(a: Nat128, b: Nat128) -> Nat128 {
    Nat128(a.0.wrapping_add(b.0))
}

// ---- size assertions, comparing a type against ANOTHER TYPE ---------------
//
// `17` section 7 class D warns that an assertion comparing a type's size against a number the
// same derivation produced is a tautology. These compare two independently declared types, so
// each is a real assertion.

const _: () = assert!(core::mem::size_of::<Iv<3, 13>>() == core::mem::size_of::<(u16, u16)>());
const _: () = assert!(core::mem::size_of::<Num<3, 13>>() == core::mem::size_of::<u16>());
const _: () = assert!(core::mem::size_of::<IvWide>() == core::mem::size_of::<(u128, u128)>());
const _: () = assert!(core::mem::align_of::<Iv<3, 13>>() == core::mem::align_of::<(u16, u16)>());

/// Raw counterpart to the wide interval, to separate the newtype question from the ABI
/// question. If these two fold, the newtype erases and the indirect return is purely a
/// consequence of the aggregate's size.
#[no_mangle]
pub fn p4_raw_wide_pair_add(a: (u128, u128), b: (u128, u128)) -> (u128, u128) {
    (a.0.wrapping_add(b.0), a.1.wrapping_add(b.1))
}
