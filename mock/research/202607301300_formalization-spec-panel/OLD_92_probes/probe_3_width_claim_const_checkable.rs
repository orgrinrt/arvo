//! The niche construction's headline ("an optional or fallible value the
//! same width as the value itself") rests on discriminant elision. For
//! `core::option::Option` over the NonZero family that is a documented std
//! guarantee (the null pointer optimization list in core::option's docs).
//! For the stack's own `Maybe` (notko/src/maybe.rs:35, `Is(T)`/`Isnt`) it
//! is NOT documented; notko itself says so in the comment directly under
//! the enum ("its layout depends on whether T happens to carry a niche")
//! and ships `MaybeNull<T>` to pin layout "per instantiation via
//! sealed-trait bound + const assertion". This probe shows the width half
//! of the niche entry is const-checkable per instantiation and therefore
//! never needs to enter the trusted base at all.
#![no_std]
use core::mem::size_of;
use core::num::NonZeroU16;

// documented: std's NPO guarantee for Option over NonZero
const _: () = assert!(size_of::<Option<NonZeroU16>>() == 2);

// Maybe-shaped local model of notko's own enum shape
enum MaybeModel<T> {
    Is(T),
    Isnt,
}
// allow the variants to be formally unused; only layout is under test
const _: MaybeModel<u8> = MaybeModel::<u8>::Isnt;
const _: MaybeModel<u8> = MaybeModel::Is(0);

// holds on the pin, but is a per-pin fact, not a documented guarantee for
// a non-Option enum; this assertion IS the pinning mechanism, the same
// discipline notko's MaybeNull already ships
const _: () = assert!(size_of::<MaybeModel<NonZeroU16>>() == 2);

// negative control: without the niche the same shape costs double, so the
// assertion above has content
const _: () = assert!(size_of::<MaybeModel<u16>>() == 4);
