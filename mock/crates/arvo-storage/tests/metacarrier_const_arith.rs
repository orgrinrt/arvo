//! Const-context smoke test for `MetaCarrier::as_bits` plus the
//! macro-emitted `IBits` / `FBits` / `Width` parallels.
//!
//! Closes audit Finding 35 (round 202605041051, task #324). The
//! `as_bits` projection bodies use `core::mem::transmute` from
//! `MetaCarrier`/wrapper to `Bits<9, Hot, Unsigned>` (layout-
//! identical via `repr(transparent)` over u16). The transmute is
//! const-stable; this file exercises the projection in const
//! context to assert the layout invariant holds at compile time.

use arvo_storage::{Bits, FBits, IBits, MetaCarrier, Width};
use arvo_strategy::{Hot, Unsigned};

const _META_FROM_RAW: MetaCarrier = MetaCarrier::from_raw(123);
const _META_TO_RAW: u16 = MetaCarrier::from_raw(123).to_raw();
const _META_AS_BITS: Bits<9, Hot, Unsigned> = MetaCarrier::from_raw(123).as_bits();

const _IBITS_CTOR: IBits = IBits(MetaCarrier::from_raw(8));
const _IBITS_AS_BITS: Bits<9, Hot, Unsigned> = IBits(MetaCarrier::from_raw(8)).as_bits();
const _IBITS_RAW: u16 = IBits(MetaCarrier::from_raw(8)).raw();

const _FBITS_CTOR: FBits = FBits(MetaCarrier::from_raw(0));
const _FBITS_AS_BITS: Bits<9, Hot, Unsigned> = FBits(MetaCarrier::from_raw(0)).as_bits();

const _WIDTH_CTOR: Width = Width(MetaCarrier::from_raw(64));
const _WIDTH_AS_BITS: Bits<9, Hot, Unsigned> = Width(MetaCarrier::from_raw(64)).as_bits();

const _ZERO_AND_ONE_PROBE: () = {
    assert!(IBits::ZERO.raw() == 0);
    assert!(IBits::ONE.raw() == 1);
    assert!(FBits::ZERO.raw() == 0);
    assert!(FBits::ONE.raw() == 1);
    assert!(Width::ZERO.raw() == 0);
    assert!(Width::ONE.raw() == 1);
};

const _LAYOUT_EQUIVALENCE_PROBE: () = {
    let raw: u16 = 42;
    let m: MetaCarrier = MetaCarrier::from_raw(raw);
    let b: Bits<9, Hot, Unsigned> = m.as_bits();
    assert!(m.to_raw() == raw);
    let _ = b; // exercise the bound; runtime assertions check round-trip below
};

#[test]
fn metacarrier_layout_round_trip() {
    let raw: u16 = 0b0_1010_1010_u16;
    let m = MetaCarrier::from_raw(raw);
    let b: Bits<9, Hot, Unsigned> = m.as_bits();
    assert_eq!(m.to_raw(), raw);
    assert_eq!(b.to_raw(), raw);

    let i = IBits(MetaCarrier::from_raw(8));
    assert_eq!(i.raw(), 8);
    assert_eq!(i.as_bits().to_raw(), 8);

    let w = Width(MetaCarrier::from_raw(64));
    assert_eq!(w.raw(), 64);
    assert_eq!(w.as_bits().to_raw(), 64);
}
