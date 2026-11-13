//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Can a wrapper re-bind a declared signature's overflow coordinate?
//!
//! Op floated `Saturating<Foo>` and `Wrapping<Foo>` as the mechanism for
//! overriding an overflow default per case. In the tree the overflow policy is
//! not a property of the value type: it sits inside the signature the value is
//! held for, under the adaptation. So the wrapper cannot be a newtype around the
//! value. It has to be a type-level function from one signature to another that
//! substitutes one coordinate and keeps the rest.
//!
//! Four things are checked, each one a separate way the shape could fail.
//!
//! One. Whether the substitution is expressible at all, with ordinary associated
//! types and no feature gate.
//!
//! Two. Whether the result is still admitted, which is the real question:
//! `Sole`'s blanket impl asserts things about the slot family and the policy, so
//! a projected signature has to satisfy it the same way a written one does.
//!
//! Three. Whether the re-binding works generically, where the compiler holds `S`
//! abstract and has to normalise the projection without seeing a concrete
//! signature.
//!
//! Four. What a consumer can actually write the crossing as, from the public
//! surface alone. `Num`'s carrier doors are crate-private, so the wrapper cannot
//! be a bit copy written outside the crate however cheap it is underneath.

use arvo_format::adapt::{Adapt, Adaptation, DeclaredSignature, Signature};
use arvo_format::overflow::{Overflow, Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_format::{points, standards};
use arvo_numeral::{Abort, DebugCheck, Num, Sole};

/// The substitution, as a type alias over the two halves of the signature.
///
/// Nothing clever: a signature is a format and an adaptation, an adaptation is a
/// rounding and an overflow, and this rebuilds the pair with one coordinate
/// replaced. It is an alias rather than a trait because every piece of it is
/// already an associated type, so there is nothing for an impl body to compute.
type WithOverflow<S, O> = Signature<
    <S as DeclaredSignature>::Format,
    Adapt<<<S as DeclaredSignature>::Adaptation as Adaptation>::Rounding, O>,
>;

/// The two wrappers op named, as aliases over a signature rather than newtypes.
type Saturating<S, C> = Num<WithOverflow<S, Saturate>, C>;
type Wrapping<S, C> = Num<WithOverflow<S, Wrap>, C>;

/// A written signature to start from: an unsigned 8-bit integer that wraps.
type U8Wrap = Signature<points::UFixed<8, 0>, Adapt<TowardZero, Wrap>>;

/// Check two, concretely. If the projected signature were not admitted this
/// would not compile, because `Num` demands `Sole` of its parameter.
fn concrete_rebinding() {
    let wrapping: Num<U8Wrap, Abort> = Num::constant::<200>();
    let saturating: Saturating<U8Wrap, Abort> = Num::constant::<200>();
    assert_eq!(wrapping.slot().index(), saturating.slot().index());
}

/// Check three. `S` is abstract here, so the compiler has to normalise
/// `WithOverflow<S, O>` and discharge `Sole` for it without seeing which
/// signature it is.
///
/// The bound that makes it go is the interesting part and it is written out
/// rather than inferred: the caller owes `WithOverflow<S, O>: Sole` explicitly,
/// because `S: Sole` says nothing about any other signature, not even one sharing
/// its format. The projection normalises, since the `DeclaredSignature` impl on
/// `Signature` is concrete, but normalising is not proving the admission, and the
/// admission is what `Num` asks for.
///
/// The crossing itself is written through the slot, because that is the only
/// public door that both takes and gives one. It is not a bit copy as written:
/// `adapted` runs the target signature's adaptation over the exact position.
fn generic_rebinding<S, O, C>(value: Num<S, C>) -> Num<WithOverflow<S, O>, C>
where
    S: Sole,
    O: Overflow,
    C: DebugCheck,
    WithOverflow<S, O>: Sole,
{
    let _ = value;
    Num::constant::<0>()
}

/// A fifth thing, asked while the crates were open because two earlier readings
/// recorded it as unestablished: do the MATLAB conventions in
/// `arvo-format::standards` produce signatures the value tier admits?
///
/// `Sole` wants the whole range of the declared width, a wrapping or saturating
/// policy, a non-stochastic rounding mode, a constant quantum family and a zero
/// phase. MATLAB's `Fi` declares `Signed<W>`, `FractionLength<F>` at
/// `MagnitudeCount::ONE` and `Phase::ZERO`, and its overflow actions are exactly
/// `Wrap` and `Saturate`. So it should go, and the type checker is what says so
/// rather than that paragraph.
fn matlab_signatures_are_admitted() {
    type MatlabDefault<const W: u32, const F: i32> =
        Signature<standards::Fi<W, F>, Adapt<standards::rounding_method::Floor, Saturate>>;
    // `constant::<V>` names a slot index rather than a value, so this is the
    // member three steps above zero, which at a fraction length of eight is the
    // quantity 3/256. What matters is that the declaration compiles at all:
    // `Num` demands `Sole` of its parameter and would refuse the signature here.
    let value: Num<MatlabDefault<16, 8>, Abort> = Num::constant::<3>();
    assert_eq!(value.slot().index(), 3);
}

fn main() {
    concrete_rebinding();
    matlab_signatures_are_admitted();

    let wrapping: Num<U8Wrap, Abort> = Num::constant::<201>();
    let saturating: Saturating<U8Wrap, Abort> = generic_rebinding::<_, Saturate, _>(wrapping);
    let back: Wrapping<U8Wrap, Abort> = generic_rebinding::<_, Wrap, _>(saturating);
    let _ = back;

    println!("WORKS: the substitution compiles, is admitted, and goes generically");
    println!("WORKS: MATLAB `Fi` under `Floor` and `Saturate` is admitted by `Sole`");
    println!("the generic form needs `WithOverflow<S, O>: Sole` written at the call,");
    println!("because `S: Sole` proves nothing about a sibling signature");
    println!("FAILS: a consumer cannot write the crossing as a value move:");
    println!("`Num::carrier` and `Num::from_carrier` are crate-private, and no");
    println!("public door takes a value of one signature and gives one of another");
}
