//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A value of a declared signature, held alone in the carrier its placement
//! derives.
//!
//! `Num<S>` is one member of the format of `S`, kept in the narrowest ladder
//! carrier `arvo_placement::derive_sole` gives the signature. Every operation is
//! the format's own on the member's slot: the slot of `a.add(b)` is the slot
//! `arvo_format::add::<S>` returns for the two slots, for every pair of members
//! of every admitted signature.
//!
//! `Sole` says which signatures a value is held for: a slot family of 1 to 64
//! bits, an overflow policy that is a `ValuePolicy`, a rounding mode that is a
//! `ValueRounding`, a zero phase and a constant quantum. Each overflow policy
//! has two spellings: under `Wrap` and `Saturate` a result leaving the range
//! asserts in a debug build, under `Wrapping` and `Saturating` nothing asserts.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

mod admit;
mod alias;
mod identity;
mod machine;
mod map;
mod num;
mod rung;

pub use admit::{Saturating, Sole, ValuePolicy, ValueRounding, Wrapping};
pub use alias::{ISize, Int, USize, Uint};
pub use arvo_format::Bool;
pub use identity::{Additive, Identity};
pub use machine::Machine;
pub use map::Carried;
pub use num::Num;

#[cfg(test)]
mod tests;
