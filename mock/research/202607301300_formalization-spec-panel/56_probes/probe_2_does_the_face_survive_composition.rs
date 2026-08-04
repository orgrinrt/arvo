//! Probe 2: probe 1 found a newtype face keeps the readable const through a
//! DECLARATION mismatch. File 04's own residue, carried forward at
//! `53:295-304`, is that operations are keyed on the numeral type, so a
//! face reappears as the raw encoding one operation into any expression.
//! This probe builds the smallest real operation (numeral addition,
//! following the design's own `mulnum`/`foldnum` shape: a computed result
//! numeral) over two faces and asks what the SUM's own face looks like, and
//! what a THIRD, mismatched operand's error names.
//!
//! Two designs are compared. Shape 1 computes the sum only in the raw
//! encoding (`Nat`), which is what a literal transcription of `mulnum`
//! would do: the operation is generic over `Nat`, not over the face, so
//! the face type is not in scope to preserve. Shape 2 makes the face
//! itself carry the operation, so the result is again a face whose const
//! parameter is computed, which costs one more layer of plumbing per
//! operation the design defines.
//!
//! EXPECTED: unknown going in. This is the empirical question the
//! checkpoint asks the fixture to answer before the notation macro's
//! vehicle is chosen.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern
//!   tower_nat=libtower_nat.rlib probe_2_does_the_face_survive_composition.rs

#![allow(dead_code)]

use core::marker::PhantomData;
use tower_nat::{Nat, Pz, H, I, O};

// The raw encoding for 37, 53, and their sum, 90.
pub type Enc37 = Pz<I<O<I<O<O<H>>>>>>; // 37
pub type Enc53 = Pz<I<O<I<O<I<H>>>>>>; // 53
pub type Enc90 = Pz<I<O<I<I<O<I<H>>>>>>>; // 90

// A trivial `Nat`-level addition, exactly the shape a real one would take:
// generic over the raw encoding, computing a raw encoding out. This is
// deliberately NOT the real gcd/reduce machinery; it exists only to have
// something that composes two `Nat`s into a third, so the diagnostic
// question can be asked without the tower's own complexity in the way.
pub trait NAdd<Rhs: Nat>: Nat {
    type Out: Nat;
}
// Hand-instantiate only the two sums this probe needs, standing in for a
// real recursive `NAdd` (which exists in file 50's `ESum`/`NAdd` shape and
// is orthogonal to the diagnostic question this probe asks).
impl NAdd<Enc53> for Enc37 {
    type Out = Enc90;
}

pub struct Container<P: Nat>(PhantomData<P>);

pub fn sum<A: Nat + NAdd<B>, B: Nat>(_: Container<A>, _: Container<B>) -> Container<A::Out> {
    Container(PhantomData)
}

// ---- Shape 1: the face is a label on entry; the operation is generic
// over the raw Nat, so its return type is a raw Nat, not a face. ----

pub struct NFace<const V: u64>(PhantomData<()>);
pub trait NumeralFace {
    type Encoding: Nat;
    const V: u64;
}
impl NumeralFace for NFace<37> {
    type Encoding = Enc37;
    const V: u64 = 37;
}
impl NumeralFace for NFace<53> {
    type Encoding = Enc53;
    const V: u64 = 53;
}

pub fn shape1_sum_decays_to_raw(a: Container<Enc37>, b: Container<Enc53>) -> Container<Enc90> {
    // A face-labelled caller feeds `sum` after unwrapping to the raw
    // encoding at the call boundary. The RETURN type is `Container<Enc90>`,
    // the raw type, not a face: nothing carried the face past the call.
    sum(a, b)
}

// Force a downstream mismatch whose error is about the SUM's own type,
// to see what it names now that the sum has decayed to raw.
pub fn shape1_needs_the_wrong_sum(_: Container<Enc37>) {}
pub fn shape1_consumer(a: Container<Enc37>, b: Container<Enc53>) {
    let s = shape1_sum_decays_to_raw(a, b);
    shape1_needs_the_wrong_sum(s); // 90 where 37 was wanted
}

// ---- Shape 2: the operation is defined ON the face, computing a face
// whose const parameter is itself computed, so the result is again a
// face and the diagnostic question is asked one hop later. ----

pub struct FaceContainer<F: NumeralFace>(PhantomData<F>);

pub trait FaceAdd<Rhs: NumeralFace>: NumeralFace {
    type Out: NumeralFace;
}
impl FaceAdd<NFace<53>> for NFace<37> {
    type Out = NFace<90>;
}
impl NumeralFace for NFace<90> {
    type Encoding = Enc90;
    const V: u64 = 90;
}

pub fn face_sum<A: NumeralFace + FaceAdd<B>, B: NumeralFace>(
    _: FaceContainer<A>,
    _: FaceContainer<B>,
) -> FaceContainer<A::Out> {
    FaceContainer(PhantomData)
}

pub fn shape2_needs_the_wrong_sum(_: FaceContainer<NFace<37>>) {}
pub fn shape2_consumer(a: FaceContainer<NFace<37>>, b: FaceContainer<NFace<53>>) {
    let s = face_sum(a, b);
    shape2_needs_the_wrong_sum(s); // 90 where 37 was wanted, but now the SUM's own type is a face
}
