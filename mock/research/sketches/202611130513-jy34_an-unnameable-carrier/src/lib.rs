//! The encoding under test. The carrier map is a trait in a module no consumer
//! can reach, and the public `Sole` reaches it only through an associated type
//! bound in its own supertrait list, which the compiler elaborates wherever
//! `S: Sole` holds. `Sole` declares no associated type of its own.

#![no_std]

use arvo_format::{DeclaredSignature, Format};

mod sealed {
    pub trait Machine: Copy + PartialEq + core::fmt::Debug {}
    impl Machine for u8 {}
    impl Machine for u16 {}

    pub trait Carried: arvo_format::Slots {
        type Repr: Machine;
    }
    impl Carried for arvo_format::slots::Unsigned<4> {
        type Repr = u8;
    }
    impl Carried for arvo_format::slots::Unsigned<13> {
        type Repr = u16;
    }
}

type SlotsOf<S> = <<S as DeclaredSignature>::Format as Format>::Slots;
type ReprOf<S> = <SlotsOf<S> as sealed::Carried>::Repr;

/// A signature a value can hold, with no associated type to project.
pub trait Sole: DeclaredSignature<Format: Format<Slots: sealed::Carried>> {}

impl<S> Sole for S
where
    S: DeclaredSignature,
    SlotsOf<S>: sealed::Carried,
{
}

/// The value, storing the carrier.
#[derive(PartialEq, Debug)]
pub struct Num<S: Sole>(ReprOf<S>);

impl<S: Sole> Num<S> {
    /// The value at slot zero, built without naming the carrier outside.
    pub fn zero() -> Self
    where
        ReprOf<S>: Default,
    {
        Self(Default::default())
    }

    /// An operation over the stored carrier, reachable generically.
    pub fn same(self, rhs: Self) -> bool {
        self.0 == rhs.0
    }
}

// By hand, as the value crate does: a derive would bound `S: Copy`.
impl<S: Sole> Clone for Num<S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<S: Sole> Copy for Num<S> {}
