// p5b: negative controls for p5. EXPECTED TO FAIL TO COMPILE; the .err is the result.
//
// p5 asserts equalities that hold. Without this file those assertions could all be vacuous.
// Here the claims that must NOT hold are asserted, and the refusals are the check. One claim in
// this file DOES hold and is deliberately included: the flat pair's collapse under the
// Precise-widens reading. Its absence from the error list is as much the result as the two
// refusals are.
//
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p5b_negctl_forcings.rs
//
// Expected: exactly two E0277 refusals, and NO error mentioning line 68.
//
// No #![feature] gate.

#![no_std]

pub trait SameType<T: ?Sized> {}
impl<T: ?Sized> SameType<T> for T {}
pub const fn assert_same<A: SameType<B> + ?Sized, B: ?Sized>() {}

pub struct Hot;
pub struct Warm;
pub struct Precise;

#[derive(Clone, Copy)]
pub struct WideBits32(pub [u8; 32]);
#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct AlignedWideBits32(pub [u8; 32]);

pub trait Width {
    type Wide: Copy;
    type WideAligned: Copy;
    type Widened: Copy;
}
pub struct W13;
impl Width for W13 {
    type Wide = u16;
    type WideAligned = u16;
    type Widened = u32;
}
pub struct W256;
impl Width for W256 {
    type Wide = WideBits32;
    type WideAligned = AlignedWideBits32;
    type Widened = WideBits32;
}

pub trait Flat<S> {
    type Carrier: Copy;
}
impl<W: Width> Flat<Warm> for W {
    type Carrier = W::Wide;
}
impl<W: Width> Flat<Hot> for W {
    type Carrier = W::WideAligned;
}
impl<W: Width> Flat<Precise> for W {
    type Carrier = W::Wide;
}

pub trait Representation {
    type Carrier: Copy;
    type Compute: Copy;
}
pub struct Padded<W: Width>(core::marker::PhantomData<W>);
pub struct PaddedAligned<W: Width>(core::marker::PhantomData<W>);
pub struct PaddedWideCompute<W: Width>(core::marker::PhantomData<W>);
impl<W: Width> Representation for Padded<W> {
    type Carrier = W::Wide;
    type Compute = W::Wide;
}
impl<W: Width> Representation for PaddedAligned<W> {
    type Carrier = W::WideAligned;
    type Compute = W::WideAligned;
}
impl<W: Width> Representation for PaddedWideCompute<W> {
    type Carrier = W::Wide;
    type Compute = W::Widened;
}
pub trait Derive<S> {
    type Repr: Representation;
}
impl<W: Width> Derive<Warm> for W {
    type Repr = Padded<W>;
}
impl<W: Width> Derive<Hot> for W {
    type Repr = PaddedAligned<W>;
}
impl<W: Width> Derive<Precise> for W {
    type Repr = PaddedWideCompute<W>;
}

type ReprOf<W, S> = <W as Derive<S>>::Repr;
type FlatCarrier<W, S> = <W as Flat<S>>::Carrier;

// MUST REFUSE: the single output separates Precise from Warm at the same width.
const _: () = assert_same::<ReprOf<W13, Precise>, ReprOf<W13, Warm>>();

// MUST REFUSE: at the wide rung, a TYPE-valued carrier separates Hot from Warm, so 45's
// alignment collision is a collision against a bit-count carrier and not against the flat pair
// as 15 and 16 state it (16:604-616: "it is a reason the carrier must be a type rather than a
// width").
const _: () = assert_same::<FlatCarrier<W256, Hot>, FlatCarrier<W256, Warm>>();

// MUST NOT REFUSE: the flat pair's carrier does NOT separate Precise from Warm at W=13. This
// is the collapse a third projection exists to repair, and its absence from the error list is
// the positive half of this control.
const _: () = assert_same::<FlatCarrier<W13, Precise>, FlatCarrier<W13, Warm>>();
