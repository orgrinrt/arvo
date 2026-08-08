// p1b: the negative control for p1, so its SameType bridge is not vacuous.
//
// p1 asserts type equalities that HOLD. A bridge that accepted everything would make those
// assertions worthless, exactly the "asserting a value against itself" shape the-test-gate.md
// names. This file asserts equalities that must NOT hold, and is EXPECTED TO FAIL TO COMPILE.
// The committed .err is the result.
//
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p1b_negctl_distinctness.rs
//
// Three refusals expected:
//   (a) Packed<W13> is not Packed<W16>            <- the Cold collapse, repaired
//   (b) Packed<W13> is not Padded<W13>            <- Cold is not Warm at the same width
//   (c) the two CARRIERS are the same (u16 == u16), asserted as DIFFERENT via a
//       deliberately-false claim, to show the bridge distinguishes the two levels
//
// No #![feature] gate.

#![no_std]

pub struct Hot;
pub struct Warm;
pub struct Cold;

pub trait Width {
    const BITS: u32;
    type Native: Copy;
    type Access: Copy;
}
macro_rules! widths {
    ($($n:ident = $bits:literal : $native:ty , $access:ty ;)*) => {
        $( pub struct $n; impl Width for $n {
            const BITS: u32 = $bits;
            type Native = $native;
            type Access = $access;
        } )*
    };
}
widths! {
    W13 = 13 : u16, u32;
    W16 = 16 : u16, u32;
}

pub struct Padded<W: Width>(core::marker::PhantomData<W>);
pub struct Packed<W: Width>(core::marker::PhantomData<W>);

pub trait Representation {
    type Carrier: Copy;
    type Access: Copy;
    const STRIDE_BITS: u32;
    const WIDTH_BITS: u32;
}
impl<W: Width> Representation for Padded<W> {
    type Carrier = W::Native;
    type Access = W::Native;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Native>() * 8) as u32;
    const WIDTH_BITS: u32 = W::BITS;
}
impl<W: Width> Representation for Packed<W> {
    type Carrier = W::Native;
    type Access = W::Access;
    const STRIDE_BITS: u32 = W::BITS;
    const WIDTH_BITS: u32 = W::BITS;
}

pub trait Derive<S> {
    type Repr: Representation;
}
impl<W: Width> Derive<Warm> for W {
    type Repr = Padded<W>;
}
impl<W: Width> Derive<Hot> for W {
    type Repr = Padded<W>;
}
impl<W: Width> Derive<Cold> for W {
    type Repr = Packed<W>;
}

pub trait SameType<T: ?Sized> {}
impl<T: ?Sized> SameType<T> for T {}
pub const fn assert_same<A: SameType<B> + ?Sized, B: ?Sized>() {}

type ReprOf<W, S> = <W as Derive<S>>::Repr;

// (a) two Cold widths that share a carrier must NOT share a single output.
const _: () = assert_same::<ReprOf<W13, Cold>, ReprOf<W16, Cold>>();

// (b) Cold and Warm at the same width must NOT share a single output.
const _: () = assert_same::<ReprOf<W13, Cold>, ReprOf<W13, Warm>>();

// (c) a claim that is false at the carrier level too, so the bridge is shown to be
//     discriminating rather than merely refusing everything with two distinct spellings.
const _: () = assert_same::<<ReprOf<W13, Cold> as Representation>::Carrier, u32>();
