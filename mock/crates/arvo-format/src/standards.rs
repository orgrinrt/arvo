//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The industrial conventions, as instances of the open inventory.
//!
//! A consumer already thinking in a standard's vocabulary keeps thinking in it
//! and gets arvo underneath. There is no adapter and no second implementation of
//! anything: a convention names a width, a scaling and a rounding-and-overflow
//! pair, and each of those is one of this crate's coordinates under a different
//! word.
//!
//! Nothing behind the naming is what makes it honest. A convention needing a
//! computation to work would not be one arvo supports badly, it would be a gap in
//! the primitives, which is what the standards bound is for. So the interesting
//! output here is a refusal, and each one in these modules is a claim about this
//! crate rather than about the standard.
//!
//! One module per standard. MATLAB and IEEE 754 are the two the parity suite is
//! built against, and SystemC `sc_fixed`, Vitis `ap_fixed` and Algorithmic C
//! `ac_fixed` are reserved rather than excluded: one joins as one more module
//! here, built from the same coordinates.
//!
//! These sit beside `points` for the same reason those are there: the concept is
//! closed and the inventory is open, so an instance joins by supplying the
//! obligations rather than by amending anything.

pub mod ieee754;
pub mod matlab;
