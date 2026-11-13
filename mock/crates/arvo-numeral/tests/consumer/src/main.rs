//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A consumer of `arvo-numeral` that names no feature of it and reaches every
//! item it makes public: each type, trait, alias, associated item, door,
//! operation and derived trait. It builds, runs and asserts what it reads.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::{Adapt, Exact, Policy, Signature, Slot};
use arvo_numeral::{
    Additive,
    Bool,
    ISize,
    Identity,
    Int,
    Num,
    Saturating,
    Sole,
    USize,
    Uint,
    ValuePolicy,
    ValueRounding,
    Wrapping,
};

type U13 = Uint<13, Adapt<TowardZero, Wrapping>>;
type I7 = Int<7, Adapt<Floor, Saturating>>;

/// Generic over the signature, bounded by `Sole` and nothing more.
const fn sum<S: Sole>(a: Num<S>, b: Num<S>, c: Num<S>) -> Num<S> {
    a.add(b).add(c)
}

/// Every associated item of `Sole`, read generically.
fn admission_of<S: Sole>() -> (u32, Policy, bool) {
    let () = S::ADMITTED;
    (S::WIDTH.count(), S::POLICY, S::ASSERTS.get())
}

fn asserts<P: ValuePolicy>() -> bool {
    P::ASSERTS.get()
}

fn admitted<M: ValueRounding>() {}

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

    // The pointer-width aliases and their host doors.
    assert_eq!(USize::from_host(usize::MAX).host(), usize::MAX);
    assert_eq!(ISize::from_host(isize::MIN).host(), isize::MIN);
    assert_eq!(USize::from_host(7).add(USize::from_host(8)).host(), 15);

    // The admission's items, through the traits.
    type S13 = Signature<arvo_format::points::UFixed<13, 0>, Adapt<TowardZero, Wrapping>>;
    assert_eq!(admission_of::<S13>(), (13, Policy::Wrap, false));
    type S7 = Signature<arvo_format::points::Integer<7>, Adapt<Floor, Saturating>>;
    assert_eq!(admission_of::<S7>(), (7, Policy::Saturate, false));
    assert!(asserts::<Wrap>() && asserts::<Saturate>());
    assert!(!asserts::<Wrapping>() && !asserts::<Saturating>());
    admitted::<TowardZero>();
    admitted::<Floor>();
    admitted::<Ceil>();
    admitted::<HalfUp>();
    admitted::<HalfEven>();
    assert!(Bool::of(true).get());
}
