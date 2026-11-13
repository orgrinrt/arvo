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
//! Five things are checked, each one a separate way the shape could fail, and
//! each arm asserts something a wrong answer would break.
//!
//! One. Whether the substitution is expressible at all, with ordinary associated
//! types and no feature gate, and whether it re-binds what it says: the same
//! position past the range has to complete differently under the two wrappers.
//!
//! Two. Whether the result is still admitted. `Sole`'s blanket impl asserts
//! things about the slot family and the policy, so a projected signature has to
//! satisfy it the same way a written one does. The two files under `examples/`
//! are the controls: signatures the admission refuses, which do not build.
//!
//! Three. Whether the re-binding works generically, where the compiler holds `S`
//! abstract and has to normalise the projection without seeing a concrete
//! signature.
//!
//! Four. Whether a consumer can write the crossing from the public surface
//! alone, as a move of a value from one signature to its sibling.
//!
//! Five. Whether MATLAB's documented `fimath` default, `Nearest` with
//! `Saturate`, is a signature the value tier admits.

use arvo_format::adapt::{Adapt, Adaptation, DeclaredSignature, Signature};
use arvo_format::overflow::{Overflow, Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_format::{Exact, Slot, points, standards};
use arvo_numeral::{Abort, DebugCheck, Num, Quiet, Sole};

/// The substitution, as a type alias over the two halves of the signature.
///
/// A signature is a format and an adaptation, an adaptation is a rounding and an
/// overflow, and this rebuilds the pair with one coordinate replaced. It is an
/// alias and no trait because every piece of it is already an associated type,
/// so there is nothing for an impl body to compute.
type WithOverflow<S, O> = Signature<
    <S as DeclaredSignature>::Format,
    Adapt<<<S as DeclaredSignature>::Adaptation as Adaptation>::Rounding, O>,
>;

/// The two wrappers op named, as aliases over a signature and no newtypes.
type Saturating<S, C> = Num<WithOverflow<S, Saturate>, C>;
type Wrapping<S, C> = Num<WithOverflow<S, Wrap>, C>;

/// A written signature to start from: an unsigned 8-bit integer that wraps.
type U8Wrap = Signature<points::UFixed<8, 0>, Adapt<TowardZero, Wrap>>;

/// Checks one and two, concretely.
///
/// Slot 300 is past an unsigned 8-bit range. Under the wrapper that names
/// `Wrap` it has to come back as 44, and under the one that names `Saturate` as
/// 255. Were the alias to keep the overflow it was given, both would be 44. The
/// `Quiet` check is named because the default `Abort` aborts a debug build on a
/// position that leaves the range, which is not what is being asked here.
fn the_substitution_rebinds_the_overflow() {
    let past = Exact::on_grid(Slot::at(300));
    let written: Num<U8Wrap, Quiet> = Num::adapted(past);
    let wrapping: Wrapping<U8Wrap, Quiet> = Num::adapted(past);
    let saturating: Saturating<U8Wrap, Quiet> = Num::adapted(past);
    assert_eq!(written.slot().index(), 44);
    assert_eq!(wrapping.slot().index(), 44);
    assert_eq!(saturating.slot().index(), 255);
}

/// Checks three and four. `S` is abstract here, so the compiler has to normalise
/// `WithOverflow<S, O>` and discharge `Sole` for it without seeing which
/// signature it is.
///
/// The caller owes `WithOverflow<S, O>: Sole` explicitly, because `S: Sole` says
/// nothing about any other signature, not even one sharing its format. The
/// projection normalises, since the `DeclaredSignature` impl on `Signature` is
/// concrete, but normalising is not proving the admission.
///
/// The crossing is three public doors in a row: the value's slot, that slot as
/// an exact position already on the grid, and the target signature's adaptation
/// over it. Both signatures share a format, so the position is on the target's
/// grid and inside its range, and the adaptation has nothing to round or
/// complete. Whether it lowers to a move is a codegen question a sketch does not
/// answer.
fn rebind<S, O, C>(value: Num<S, C>) -> Num<WithOverflow<S, O>, C>
where
    S: Sole,
    O: Overflow,
    C: DebugCheck,
    WithOverflow<S, O>: Sole,
{
    Num::adapted(Exact::on_grid(value.slot()))
}

/// The crossing keeps the member, and the value that arrives carries the policy
/// it crossed into: one step further past the end it saturates where the value
/// it came from wraps.
fn the_crossing_moves_the_member_and_changes_its_policy() {
    let wrapping: Num<U8Wrap, Abort> = Num::constant::<201>();
    let saturating: Saturating<U8Wrap, Abort> = rebind::<_, Saturate, _>(wrapping);
    let back: Wrapping<U8Wrap, Abort> = rebind::<_, Wrap, _>(saturating);
    assert_eq!(saturating.slot().index(), 201);
    assert_eq!(back.slot().index(), 201);

    let top: Num<U8Wrap, Quiet> = Num::constant::<255>();
    let one: Num<U8Wrap, Quiet> = Num::constant::<1>();
    let crossed_top: Saturating<U8Wrap, Quiet> = rebind::<_, Saturate, _>(top);
    let crossed_one: Saturating<U8Wrap, Quiet> = rebind::<_, Saturate, _>(one);
    assert_eq!(top.add(one).slot().index(), 0);
    assert_eq!(crossed_top.add(crossed_one).slot().index(), 255);
}

/// Check five. MathWorks documents the default `fimath` as `RoundingMethod:
/// Nearest` and `OverflowAction: Saturate`, and `arvo-format::standards` maps
/// Nearest onto `HalfUp`.
///
/// `Sole` wants the whole range of the declared width, a wrapping or saturating
/// policy, a deterministic rounding mode, a constant quantum family and a zero
/// phase. The declaration below would not build were the signature refused, and
/// the two assertions pin the saturation at both ends of a signed 16-bit range.
fn the_matlab_default_is_admitted_and_saturates() {
    type MatlabDefault<const W: u32, const F: i32> =
        Signature<standards::Fi<W, F>, Adapt<standards::rounding_method::Nearest, Saturate>>;
    let above: Num<MatlabDefault<16, 8>, Quiet> = Num::adapted(Exact::on_grid(Slot::at(40_000)));
    let below: Num<MatlabDefault<16, 8>, Quiet> = Num::adapted(Exact::on_grid(Slot::at(-40_000)));
    assert_eq!(above.slot().index(), 32_767);
    assert_eq!(below.slot().index(), -32_768);
}

fn main() {
    the_substitution_rebinds_the_overflow();
    the_crossing_moves_the_member_and_changes_its_policy();
    the_matlab_default_is_admitted_and_saturates();

    println!("WORKS: the substitution compiles and re-binds the overflow (300 is 44 and 255)");
    println!("WORKS: the projected signature is admitted, concretely and with `S` abstract");
    println!("the generic form needs `WithOverflow<S, O>: Sole` written at the call,");
    println!("because `S: Sole` proves nothing about a sibling signature");
    println!("WORKS: the crossing is `Num::adapted(Exact::on_grid(value.slot()))`,");
    println!("three public doors, and the value that arrives adds under the new policy");
    println!("WORKS: MATLAB's default, `Fi` under `Nearest` and `Saturate`, is admitted");
    println!("the controls under `examples/` do not build: see `out/refused-*.txt`");
}
