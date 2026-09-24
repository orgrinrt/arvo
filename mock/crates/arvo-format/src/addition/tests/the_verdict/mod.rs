//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The associativity verdict against brute force, and the counts the canon
//! records.
//!
//! Brute force here is the triple walk over every member of the range, under one
//! signature and one dither, and a cell is associative when no triple diverges
//! at any dither the sweep carries. The same dither is handed to every addition
//! in a triple, which is the reading the verdict is stated for: the stochastic
//! mode's offset is the dither's, and the verdict holds for every dither at once.
//!
//! The cross runs every interval with both ends between minus four and four, so
//! ranges without zero in them are crossed as often as ranges with it, under the
//! shared phase list, beside the shipped points at the widths the walk affords.
//! A licensed cell never diverges. At a whole phase over two slots or more the
//! verdict is exact both ways. Where the phase is fractional the verdict refuses
//! some cells that turn out associative anyway, and that count is pinned so a
//! change to it is seen rather than absorbed.
//!
//! Four modules, one per section: `counts` reproduces the canon's own numbers
//! and shows they hold at every scale; `brute_force` crosses the verdict against
//! the triple walk directly; `width_walk` carries the same claim past what the
//! walk can afford, by sample and by witness, over every admitted width;
//! `negative_controls` shows two narrower rules disagreeing with brute force
//! where this verdict does not.

use crate::adapt::{Adapt, Signature};
use crate::overflow::{Saturate, Wrap};
use crate::rounding::Floor;

mod brute_force;
mod counts;
mod negative_controls;
mod width_walk;

/// The saturating signature over `F`, under the floor.
type Sat<F> = Signature<F, Adapt<Floor, Saturate>>;

/// The wrapping signature over `F`, under the floor.
type Wr<F> = Signature<F, Adapt<Floor, Wrap>>;
