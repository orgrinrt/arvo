//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

//! The industrial conventions, written as aliases over the declared signatures.
//!
//! A consumer already thinking in a standard's vocabulary keeps thinking in it
//! and gets arvo underneath. There is no adapter and no second implementation of
//! anything: a convention names a width, a scaling and a rounding-and-overflow
//! pair, and each of those is one of `arvo-format`'s coordinates under a
//! different word.
//!
//! So the crate ships no arithmetic, and the interesting thing it produces is a
//! refusal. A convention that cannot be written this way is a gap in the
//! primitives rather than a gap here, which is what makes an alias layer the
//! adequacy test on everything below it.
