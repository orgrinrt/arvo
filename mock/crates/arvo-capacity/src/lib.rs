//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! A count fixed when the consumer builds, carried as a type.
//!
//! `obligation::a_count_fixed_at_build_that_sizes_arrays_from_generic_code`
//! asks for a count that sizes real arrays from code generic over it, many of
//! them bundled under one parameter, nested with no product computed, readable
//! in a constant, and none of it needing a feature line in the consumer.
//!
//! [`Capacity`] projects an array of exactly its count, [`Dim`] is the capacity
//! of `N` elements, and [`count`] reads a capacity's count in a constant.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

mod capacity;
mod count;

pub use capacity::{Capacity, Dim};
pub use count::count;
