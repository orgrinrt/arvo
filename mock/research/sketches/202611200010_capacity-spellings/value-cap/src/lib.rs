//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm B: the count as a value of a stack type at a const generic position.
//!
//! This is the spelling hilavitkutin's api still carries, `Seq<T, const N: Cap>`
//! and `Reach<C, const N: Cap>`. The provider takes `adt_const_params`, which
//! the workspace allows. Whether the consumer has to take it too is what the
//! arm measures; the examples in `consumer/examples/` are the instrument.

#![no_std]
#![feature(adt_const_params)]

use core::marker::ConstParamTy;

/// A count as a value, admitted at a const generic position.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct Cap(usize);

impl Cap {
    /// A count from the host's integer.
    #[must_use]
    pub const fn new(count: usize) -> Self {
        Self(count)
    }

    /// The host's integer, read in a constant.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

/// A container sized by a `Cap` the provider itself declares, the arm's
/// best case: the consumer names it with a constant and never declares a
/// `Cap` parameter of its own. It holds no array, because an array of
/// `N.get()` elements is a const expression over a generic parameter.
pub struct Seq<T, const N: Cap>(core::marker::PhantomData<T>);

impl<T, const N: Cap> Seq<T, N> {
    /// The count, read in a constant.
    pub const COUNT: usize = N.get();
}
