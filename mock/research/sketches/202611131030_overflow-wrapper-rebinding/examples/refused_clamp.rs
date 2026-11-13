//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A control that must not build: the substitution handed a policy a value
//! cannot carry.
//!
//! `WithOverflow<S, Clamp>` is expressible, since the alias is only a
//! projection. The admission is what has to refuse it.

use arvo_format::adapt::{Adapt, Adaptation, DeclaredSignature, Signature};
use arvo_format::overflow::{Clamp, Wrap};
use arvo_format::points;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Abort, Num};

type WithOverflow<S, O> = Signature<
    <S as DeclaredSignature>::Format,
    Adapt<<<S as DeclaredSignature>::Adaptation as Adaptation>::Rounding, O>,
>;

type U8Wrap = Signature<points::UFixed<8, 0>, Adapt<TowardZero, Wrap>>;

fn main() {
    let value: Num<WithOverflow<U8Wrap, Clamp>, Abort> = Num::constant::<3>();
    let _ = value;
}
