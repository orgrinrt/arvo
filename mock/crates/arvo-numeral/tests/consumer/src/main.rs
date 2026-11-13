//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A consumer of `arvo-numeral` that names no feature of it and reaches every
//! item it makes public: each type, trait, alias, associated item, door,
//! operation and derived trait. It builds, runs and asserts what it reads, and
//! it implements each open trait once from outside.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Floor, HalfEven, TowardZero};
use arvo_format::{Adapt, Exact, Policy, Signature, Slot};
use arvo_numeral::{
    Abort,
    Additive,
    Bool,
    DebugCheck,
    ISize,
    Identity,
    Int,
    Multiplicative,
    NamesAdaptation,
    Num,
    Quiet,
    Sole,
    USize,
    Uint,
};

type U13 = Uint<13, Adapt<TowardZero, Wrap>, Quiet>;
type I7 = Int<7, Adapt<Floor, Saturate>, Quiet>;

/// A name for an adaptation, of this crate's own. What a strategy will be.
struct Rounded;

impl NamesAdaptation for Rounded {
    type Adaptation = Adapt<HalfEven, Wrap>;
}

/// A debug check of this crate's own.
struct Never;

impl DebugCheck for Never {
    const ABORTS: Bool = Bool::FALSE;
}

/// Generic over the signature and the check, bounded by nothing more.
const fn sum<S: Sole, C: DebugCheck>(a: Num<S, C>, b: Num<S, C>, c: Num<S, C>) -> Num<S, C> {
    a.add(b).add(c)
}

/// Every associated item of `Sole`, read generically.
fn admission_of<S: Sole>() -> (u32, Policy, bool) {
    let () = S::ADMITTED;
    (S::WIDTH.count(), S::POLICY, S::SIGNED.get())
}

fn aborts<C: DebugCheck>() -> bool {
    C::ABORTS.get()
}

fn hash_of<T: Hash>(t: &T) -> u64 {
    let mut h = DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

const SIX: U13 = sum(
    U13::constant::<1>(),
    U13::constant::<2>(),
    U13::constant::<3>(),
);

/// The ring operations in a constant, wrapping at thirteen bits.
const DIFFERENCE: U13 = U13::constant::<0>()
    .sub(U13::constant::<1>())
    .mul(U13::constant::<3>())
    .neg()
    .mul(U13::constant::<2731>())
    .neg();

fn main() {
    // The doors.
    assert_eq!(SIX, U13::constant::<6>());
    assert_eq!(SIX.slot(), Slot::at(6));
    assert_eq!(U13::adapted(Exact::on_grid(SIX.slot())), SIX);
    assert_eq!(
        U13::adapted(Exact::on_grid(Slot::at(8192))).slot().index(),
        0
    );
    // The carrier is observed through the layout and nothing else.
    assert_eq!(core::mem::size_of::<U13>(), 2);
    assert_eq!(core::mem::size_of::<I7>(), 1);

    // The operations.
    let (a, b) = (I7::constant::<-3>(), I7::constant::<5>());
    assert_eq!(a.add(b).slot().index(), 2);
    assert!(a.fits_add(b).get());
    assert!(!I7::constant::<63>().fits_add(I7::constant::<1>()).get());
    assert_eq!(
        I7::constant::<63>().add(I7::constant::<1>()).slot().index(),
        63
    );
    assert_eq!(a.sub(b).slot().index(), -8);
    assert!(a.fits_sub(b).get());
    assert!(!I7::constant::<-64>().fits_sub(I7::constant::<1>()).get());
    assert_eq!(
        I7::constant::<-64>()
            .sub(I7::constant::<1>())
            .slot()
            .index(),
        -64
    );
    assert_eq!(a.neg().slot().index(), 3);
    assert!(a.fits_neg().get());
    assert!(!I7::constant::<-64>().fits_neg().get());
    assert_eq!(I7::constant::<-64>().neg().slot().index(), 63);
    assert_eq!(a.mul(b).slot().index(), -15);
    assert!(a.fits_mul(b).get());
    assert!(!I7::constant::<-64>().fits_mul(I7::constant::<2>()).get());
    assert_eq!(
        I7::constant::<-64>()
            .mul(I7::constant::<2>())
            .slot()
            .index(),
        -64
    );
    assert_eq!(DIFFERENCE.slot().index(), 8191);
    // A fixed-point product: a half times a half is a quarter, at eighths.
    type Eighths =
        Num<Signature<arvo_format::points::Fixed<6, -3>, Adapt<HalfEven, Saturate>>, Quiet>;
    assert_eq!(
        Eighths::constant::<4>()
            .mul(Eighths::constant::<4>())
            .slot()
            .index(),
        2
    );
    assert_eq!(a.cmp(b), Ordering::Less);
    let eq: Bool = a.eq(a);
    assert!(eq.get());
    assert!(a.lt(b).get() && a.le(b).get() && b.gt(a).get() && b.ge(a).get());
    assert_eq!(a.min(b), a);
    assert_eq!(a.max(b), b);

    // The derived traits.
    let copied = a;
    let cloned = Clone::clone(&a);
    assert!(copied == cloned && copied != b);
    assert_eq!(hash_of(&a), hash_of(&cloned));
    assert!(!format!("{a:?}").is_empty());

    // The identity.
    assert_eq!(<I7 as Identity<Additive>>::IDENTITY.slot().index(), 0);
    assert_eq!(a.add(<I7 as Identity<Additive>>::IDENTITY), a);
    assert_eq!(<I7 as Identity<Multiplicative>>::IDENTITY.slot().index(), 1);
    assert_eq!(a.mul(<I7 as Identity<Multiplicative>>::IDENTITY), a);
    assert_eq!(
        <Eighths as Identity<Multiplicative>>::IDENTITY
            .slot()
            .index(),
        8
    );

    // The pointer-width aliases and their host doors, under the default check.
    type UHost = USize<Adapt<TowardZero, Wrap>>;
    type IHost = ISize<Adapt<TowardZero, Wrap>>;
    assert_eq!(UHost::from_host(usize::MAX).host(), usize::MAX);
    assert_eq!(IHost::from_host(isize::MIN).host(), isize::MIN);
    assert_eq!(UHost::from_host(7).add(UHost::from_host(8)).host(), 15);

    // An alias through an outside name for an adaptation, and an outside check.
    type ByName = Int<8, Rounded, Never>;
    assert_eq!(
        ByName::constant::<127>()
            .add(ByName::constant::<1>())
            .slot()
            .index(),
        -128
    );
    assert_eq!(
        core::mem::size_of::<ByName>(),
        core::mem::size_of::<Int<8, Adapt<HalfEven, Wrap>>>()
    );

    // The admission's items, through the traits.
    type S13 = Signature<arvo_format::points::UFixed<13, 0>, Adapt<TowardZero, Wrap>>;
    assert_eq!(admission_of::<S13>(), (13, Policy::Wrap, false));
    type S7 = Signature<arvo_format::points::Integer<7>, Adapt<Floor, Saturate>>;
    assert_eq!(admission_of::<S7>(), (7, Policy::Saturate, true));
    assert!(aborts::<Abort>() && !aborts::<Quiet>() && !aborts::<Never>());
    assert!(Bool::of(true).get());
}
