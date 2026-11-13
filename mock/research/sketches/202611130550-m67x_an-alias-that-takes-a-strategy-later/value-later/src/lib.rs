//! The same alias once `Warm` is claimed: a strategy names an adaptation too,
//! and the parameter defaults to `Warm`.

#![no_std]

use core::marker::PhantomData;

use arvo_format::points::UFixed;
use arvo_format::{Adapt, Adaptation, DeclaredSignature, Overflow, Rounding, Signature};

/// Whether a debug build aborts when a result leaves the range.
pub trait DebugCheck {}

/// The default.
pub struct Abort;

impl DebugCheck for Abort {}

/// A stand-in for the value: only its type matters here.
pub struct Num<S: DeclaredSignature, C: DebugCheck = Abort>(PhantomData<(S, C)>);

impl<S: DeclaredSignature, C: DebugCheck> Num<S, C> {
    /// A value.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

/// A type that names an adaptation: an adaptation names itself.
pub trait NamesAdaptation {
    /// The adaptation it names.
    type Adaptation: Adaptation;
}

impl<R: Rounding, O: Overflow> NamesAdaptation for Adapt<R, O> {
    type Adaptation = Self;
}

/// The unsigned integer of `BITS` bits under the adaptation `A` names, `Warm`
/// unless named.
pub type Uint<const BITS: u32, A = Warm, C = Abort> =
    Num<Signature<UFixed<BITS, 0>, <A as NamesAdaptation>::Adaptation>, C>;

use arvo_strategy::Strategy;
use arvo_strategy::presets::Warm;

impl NamesAdaptation for Warm {
    type Adaptation = <Warm as Strategy>::Adaptation;
}
