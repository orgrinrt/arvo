//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws the design names, asserted over the whole matrix rather than a sample.
//!
//! Split four ways along the same seams the source is split along: what is in the
//! representable set, what an adaptation is, what joins the inventory, and what
//! the coordinate types themselves promise. One file was carrying all four and had
//! grown past what one file should, and the seams were already written into it as
//! section headings.
//!
//! Every coordinate here is written as the type the contract declares it with. A
//! suite reaching for a host integer would be exercising a surface the contract
//! does not have, and it would be the one place in the crate still saying a slot
//! is a number.

mod the_adaptation;
mod the_coordinates;
mod the_inventory;
mod the_predicate;
