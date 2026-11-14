//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A value of a declared signature, held alone in the carrier its placement
//! derives.
//!
//! `Num<S, C>` is one member of the format of `S`, kept in the machine integer
//! of the ladder container `arvo_placement::SoleCarrier` names for the
//! signature's slot family. Every operation is the format's own on the member's
//! slot: the slot of `a.add(b)` is the slot `arvo_format::add::<S>` returns for
//! the two slots, for every pair of members of every admitted signature.
//!
//! `Sole` says which signatures a value is held for, by checking what a
//! signature already declares: a slot family whose range is the whole signed or
//! unsigned range of its width, an overflow policy that wraps or saturates, a
//! rounding mode that reads no dither, a zero phase and a constant quantum. A
//! slot family, a policy or a mode of another crate is held on the same terms.
//!
//! `C` is the debug check, a separate axis the consumer chooses: under `Abort`,
//! the default, a debug build aborts when a result leaves the range, and under
//! `Quiet` no build does.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

mod admit;
mod alias;
mod check;
pub mod host;
mod identity;
mod machine;
mod map;
mod num;
mod rung;

pub use admit::Sole;
pub use alias::{ISize, Int, NamesAdaptation, USize, Uint};
pub use arvo_format::Bool;
pub use check::{Abort, DebugCheck, Quiet};
pub use host::{At, Byte, Bytes, HostWord, Word};
pub use identity::{Additive, Identity};
pub use num::Num;

#[cfg(test)]
mod tests;
