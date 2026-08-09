//! Probe 6. Which way can a derivation run?
//!
//! `77b`'s sentence is a composition: "It's not a numeral itself. But it
//! contains a numeral that expresses this length." Read literally that puts
//! the capacity first and the numeral inside it, which would mean the const
//! parameter is primary and the numeral is projected out of it. Probe 4 built
//! the other direction (numeral primary, storage projected out of it). Only
//! one of the two can be built, and this probe establishes which.
//!
//! Claims:
//!   A. numeral -> value: WORKS, gate-free, ordinary associated const
//!      (probe 2 claim B, restated here for the contrast).
//!   B. numeral -> storage: WORKS, gate-free (probe 4).
//!   C. const parameter -> numeral, GENERICALLY: REFUSED. Projecting a binary
//!      numeral out of a `const N: usize` needs recursion on `N / 2` in type
//!      position, which is the forbidden feature by another door.
//!   D. const parameter -> numeral, PER INSTANCE by emission: works, and is
//!      therefore the only form the composition reading can take. Which means
//!      it is a build-layer contract, not a type-system one.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]

use core::marker::PhantomData;

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

pub trait Pos {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

// CLAIM D. Emission. A macro that is handed the digits writes both spellings
// from one source, so nothing in the emitted text can disagree with anything
// else in it. This is what a notation vehicle already does for this design
// (`102` section 1.18), and it is the shape the composition reading reduces to.
pub struct Dim<const N: usize>;

pub trait Denotes {
    type Numeral: Pos;
    const N: usize;
}

macro_rules! denote {
    ($n:literal => $($d:ident)*) => {
        impl Denotes for Dim<$n> {
            type Numeral = denote!(@build $($d)*);
            const N: usize = $n;
        }
        const _: () = assert!(<Dim<$n> as Denotes>::N == <<Dim<$n> as Denotes>::Numeral as Pos>::VAL);
    };
    (@build h) => { H };
    (@build o $($rest:ident)*) => { O<denote!(@build $($rest)*)> };
    (@build i $($rest:ident)*) => { I<denote!(@build $($rest)*)> };
}

// LSB first. The assertion inside the macro is what makes a wrong emission
// fail at the emission site rather than downstream.
// The first draft of these six lines had five of the six digit strings wrong,
// and every one failed at the emission site rather than downstream. That is
// claim D demonstrated on the author rather than asserted about a consumer.
denote!(1  => h);
denote!(3  => i h);
denote!(5  => i o h);
denote!(7  => i i h);
denote!(13 => i o i h);
denote!(47 => i i i i o h);

// CLAIM C. The same projection written once, generically, over every `N`.
// This is what "the capacity contains a numeral" would have to mean if it
// meant anything in the type system rather than in a build step.
// Compile with `--cfg refuse` to reproduce. Kept behind a cfg so the emission
// half above stands on its own exit code.
#[cfg(refuse)]
mod generic_projection {
    use super::{Dim, Pos, O};
    pub trait Project {
        type Numeral: Pos;
    }
    impl<const N: usize> Project for Dim<N>
    where
        Dim<{ N / 2 }>: Project,
    {
        type Numeral = O<<Dim<{ N / 2 }> as Project>::Numeral>;
    }
}
