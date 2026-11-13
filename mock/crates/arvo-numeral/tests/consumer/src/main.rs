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
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::{Floor, HalfEven, TowardZero};
use arvo_format::{Adapt, Exact, Format, Policy, Signature, Slot, WithOverflow, WithRounding};
use arvo_numeral::{
    Abort,
    Accumulator,
    Additive,
    Bool,
    DebugCheck,
    Divisor,
    Fix,
    HostInteger,
    ISize,
    Identity,
    Int,
    Multiplicative,
    NamesAdaptation,
    Num,
    Quiet,
    Sole,
    UFix,
    USize,
    Uint,
};

type U13 = Uint<13, Adapt<TowardZero, Wrap>, Quiet>;
type I7 = Int<7, Adapt<Floor, Saturate>, Quiet>;

/// A preset of this crate's own, naming an adaptation per format: signed
/// integers wrap under it and unsigned ones saturate. What a strategy will be.
struct Rounded;

impl<const BITS: u32> NamesAdaptation<Integer<BITS>> for Rounded
where
    Integer<BITS>: Format,
{
    type Adaptation = Adapt<HalfEven, Wrap>;
}

impl<const BITS: u32> NamesAdaptation<UFixed<BITS, 0>> for Rounded
where
    UFixed<BITS, 0>: Format,
{
    type Adaptation = Adapt<HalfEven, Saturate>;
}

/// An alias of this crate's own whose adaptation carries a default.
type Count<const BITS: u32, A = Rounded, C = Never> = Uint<BITS, A, C>;

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

type S7 = Signature<Integer<7>, Adapt<Floor, Saturate>>;

/// A quotient and a host integer in a constant: a quarter of ten, floored, and
/// a host integer past the top of seven signed bits.
const QUARTER: I7 = I7::constant::<10>().div(Divisor::constant::<4>());
const DOOR: I7 = I7::from_index(200u8);

/// Generic over both signatures, the capacity and the check: an array summed
/// exactly and finished once, in a constant.
const fn folded<S: Sole, T: Sole, const N: usize, C: DebugCheck>(
    terms: &[Num<S, C>; N],
) -> Num<S, C> {
    Accumulator::<S, T, N, C>::of(terms).finish()
}

const FOLDED: I7 = folded::<S7, Signature<Integer<9>, Adapt<Floor, Wrap>>, 3, Quiet>(&[
    I7::constant::<63>(),
    I7::constant::<7>(),
    I7::constant::<-64>(),
]);

/// Generic over the host integer, bounded by `HostInteger` and nothing more: a
/// host integer in through the door and out again.
fn round_trip<H: HostInteger>(host: H) -> H {
    Int::<16, Adapt<TowardZero, Wrap>, Quiet>::from_index(host).index::<H>()
}

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

    // The same preset names another adaptation for an unsigned integer, and an
    // alias that defaults to it needs no adaptation written.
    let (top, one) = (Count::<8>::constant::<255>(), Count::<8>::constant::<1>());
    assert_eq!(top.add(one).slot().index(), 255);
    // The default overridden per case, at the type and at a value.
    type Wrapping8 = Count<8, Adapt<HalfEven, Wrap>>;
    assert_eq!(
        Wrapping8::constant::<255>()
            .add(Wrapping8::constant::<1>())
            .slot()
            .index(),
        0
    );
    assert_eq!(
        top.with_overflow::<Wrap>()
            .add(one.with_overflow::<Wrap>())
            .slot()
            .index(),
        0
    );
    assert_eq!(top.with_rounding::<Floor>().slot(), top.slot());

    // A sibling signature by the aliases, and the crossing to it and back.
    type Wrapping13 = Signature<UFixed<13, 0>, Adapt<TowardZero, Wrap>>;
    type Saturating13 = WithOverflow<Wrapping13, Saturate>;
    type Nearest13 = WithRounding<Saturating13, HalfEven>;
    let there: Num<Saturating13, Quiet> = SIX.under();
    assert_eq!(there.slot(), SIX.slot());
    assert_eq!(there.under::<Nearest13>().under::<Wrapping13>(), SIX);
    let top13 = U13::constant::<8191>();
    assert_eq!(top13.add(SIX).slot().index(), 5);
    assert_eq!(
        top13.under::<Saturating13>().add(there).slot().index(),
        8191
    );

    // The quotient, by a divisor that is a constant of the format.
    let seven: Divisor<Integer<7>> = Divisor::constant::<7>();
    assert_eq!(I7::constant::<-50>().div(seven).slot().index(), -8);
    assert!(I7::constant::<-50>().fits_div(seven).get());
    let minus_one = Divisor::constant::<-1>();
    assert!(!I7::constant::<-64>().fits_div(minus_one).get());
    assert_eq!(I7::constant::<-64>().div(minus_one).slot().index(), 63);
    assert_eq!(QUARTER.slot().index(), 2);

    // The conversion onto another signature: three is twenty-four eighths, and
    // a hundred is past the top of the eighths.
    type EighthsSig = Signature<arvo_format::points::Fixed<6, -3>, Adapt<HalfEven, Saturate>>;
    assert_eq!(
        U13::constant::<3>().convert::<EighthsSig>().slot().index(),
        24
    );
    assert!(U13::constant::<3>().fits_convert::<EighthsSig>().get());
    assert!(!U13::constant::<100>().fits_convert::<EighthsSig>().get());
    assert_eq!(
        U13::constant::<100>()
            .convert::<EighthsSig>()
            .slot()
            .index(),
        31
    );
    // One and a half onto the integers, by the target's mode.
    assert_eq!(Eighths::constant::<12>().convert::<S7>().slot().index(), 1);

    // The door for a host integer, in and out at whichever width is asked.
    assert_eq!(U13::from_index(8191u16), top13);
    assert_eq!(U13::from_index(8192u64).slot().index(), 0);
    assert_eq!(I7::from_index(1000i32).slot().index(), 63);
    assert_eq!(I7::from_index(u64::MAX).slot().index(), 63);
    assert!(U13::holds_index(8191i64).get() && !U13::holds_index(-1i8).get());
    assert_eq!(top13.index::<u16>(), 8191);
    assert_eq!(top13.index::<i64>(), 8191);
    assert_eq!(a.index::<i8>(), -3);
    assert_eq!(round_trip::<i16>(-3), -3);
    assert_eq!(round_trip::<isize>(40), 40);
    assert_eq!(DOOR.slot().index(), 63);

    // The fixed-point aliases: a lane at sixteen fraction bits, half to even.
    type Lane = Fix<32, -16, Adapt<HalfEven, Saturate>>;
    assert_eq!(
        Lane::constant::<98304>()
            .mul(Lane::constant::<32768>())
            .slot()
            .index(),
        49152
    );
    type Share = UFix<8, -8, Adapt<Floor, Wrap>, Quiet>;
    assert_eq!(
        Share::constant::<128>()
            .add(Share::constant::<128>())
            .slot()
            .index(),
        0
    );

    // The accumulator: seven bits of terms summed exactly in twelve.
    type Wide = Signature<Integer<12>, Adapt<Floor, Saturate>>;
    let terms = [I7::constant::<63>(), I7::constant::<63>(), b, a];
    let all = Accumulator::<S7, Wide, 8, Quiet>::of(&terms);
    assert_eq!(all.total().slot().index(), 128);
    assert!(!all.fits().get());
    assert_eq!(all.finish().slot().index(), 63);
    let (block, rest) = Accumulator::<S7, Wide, 3, Quiet>::take(&terms);
    assert_eq!(block.total().slot().index(), 131);
    assert_eq!(rest, &[a]);
    let (last, none) = Accumulator::<S7, Wide, 3, Quiet>::take(rest);
    assert!(none.is_empty() && last.fits().get());
    let joined = Accumulator::<S7, Wide, 8, Quiet>::plus(block, last);
    assert_eq!(joined.total(), all.total());
    let copied = joined;
    assert_eq!(Clone::clone(&copied).finish(), all.finish());
    assert_eq!(
        Accumulator::<S7, Wide, 3, Quiet>::minus(last, block)
            .slot()
            .index(),
        -134
    );
    assert_eq!(
        Accumulator::<S7, Wide, 8, Quiet>::plus(Accumulator::<S7, Wide, 0, Quiet>::empty(), last)
            .finish(),
        a
    );
    assert_eq!(FOLDED.slot().index(), 6);

    // The admission's items, through the traits.
    type S13 = Signature<arvo_format::points::UFixed<13, 0>, Adapt<TowardZero, Wrap>>;
    assert_eq!(admission_of::<S13>(), (13, Policy::Wrap, false));
    assert_eq!(admission_of::<S7>(), (7, Policy::Saturate, true));
    assert!(aborts::<Abort>() && !aborts::<Quiet>() && !aborts::<Never>());
    assert!(Bool::of(true).get());
}
