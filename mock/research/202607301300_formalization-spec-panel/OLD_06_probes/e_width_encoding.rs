// PROBE E (section 7): the three ways to carry a width, priced on the axis
// nobody in the panel has priced them on, which is what the consumer reads when
// something fails. Self-contained, no dependencies. Four parts, run separately.
//
// 02 section 8 concludes that "every axis whose value is subtracted, compared or
// otherwise computed from becomes a type carrying its derived facts as members,
// which is uniform, linear, gate-free". That conclusion is about the trait
// solver. It has a diagnostic cost that decides between two shapes of "type"
// which 02 treats as one, and it rests on a premise that E4 below falsifies.

// ---- E1: Peano widths. Arithmetic is linear and clean. -------------------
//
//   pub struct Z; pub struct S<P: Nat>(PhantomData<P>);
//   pub type N13 = S<S<S<S<S<S<S<S<S<S<S<S<S<Z>>>>>>>>>>>>>;
//   pub fn bad() { fold::<Fix<N13, N3, Signed>>() }
//
// RESULT, verbatim:
//
//   error[E0277]: the trait bound `Fix<S<S<S<S<S<S<S<S<S<S<S<S<S<Z>>>>>>>>>>>>>,
//                 S<S<S<Z>>>, Signed>: AddAssoc` is not satisfied
//
// For a 13.3 fixed-point. arvo's widths run to 128 in the native tables and past
// 256 through `WideBits`, so this is not a shape that scales to arvo's range.

// ---- E2: a flat table of named nats, which is 02's h_widthtype shape ------
//
//   nats!(N1 Z 1; N2 N1 2; ... N16 N15 16;);
//   pub fn bad_a() { fold::<Fix<N13, N3, Signed>>() }
//
// RESULT: renders cleanly.
//
//   error[E0277]: the trait bound `Fix<N13, N3, Signed>: AddAssoc` is not satisfied
//
// But the subtraction has to come from somewhere, and when a row is missing the
// consumer is told about the type-level arithmetic rather than about their type:
//
//   error[E0277]: the trait bound `N13: Sub<N3>` is not satisfied
//     |
//   pub fn bad_b() { needs::<Fix<N13, N3, Signed>>() }
//     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
//   help: the trait `Sub<N3>` is not implemented for `N13`
//   note: required for `Fix<N13, N3, Signed>` to implement `Numeral`
//
// h_widthtype only implements `Sub<Z>`, so it never exercised the recursion and
// never saw either of these.

// ---- E3: const parameters, which is what ships ---------------------------
//
// Renders perfectly as `Fix<13, 3, Signed>` when the consumer's crate has no
// `generic_const_exprs` gate, and degenerates to
// `UFixed<arvo::::aliases::Uint::{constant#0}, ...>` when it does. See probe A.

// ---- E4: the premise. Does the derivation need type position at all? ------
//
// Compiles, runs, ZERO feature gates. The full body:

#![allow(dead_code)]
use core::marker::PhantomData;

pub struct Unsigned;
pub struct Signed;
pub trait Signedness {
    const BITS: u16;
}
impl Signedness for Unsigned {
    const BITS: u16 = 0;
}
impl Signedness for Signed {
    const BITS: u16 = 1;
}

pub trait Numeral {
    const LOGICAL_WIDTH: u16;
    const EXPONENT_FIELD: u16;
    const SIGN_BITS: u16;
    /// the derivation the spec says needs a subtraction, at spec:118
    const SIGNIFICAND: u16 = Self::LOGICAL_WIDTH - Self::EXPONENT_FIELD - Self::SIGN_BITS;
    const IS_INTEGRAL: bool;
}

pub struct Fix<const I: u16, const F: u16, S>(PhantomData<S>);
impl<const I: u16, const F: u16, S: Signedness> Numeral for Fix<I, F, S> {
    const LOGICAL_WIDTH: u16 = I + F + S::BITS;
    const EXPONENT_FIELD: u16 = 0;
    const SIGN_BITS: u16 = S::BITS;
    const IS_INTEGRAL: bool = F == 0;
}

pub struct Flt<const E: u16, const M: u16>;
impl<const E: u16, const M: u16> Numeral for Flt<E, M> {
    const LOGICAL_WIDTH: u16 = 1 + E + M;
    const EXPONENT_FIELD: u16 = E;
    const SIGN_BITS: u16 = 1;
    const IS_INTEGRAL: bool = false;
}

pub const fn check<N: Numeral>() -> (u16, u16, bool) {
    (N::LOGICAL_WIDTH, N::SIGNIFICAND, N::IS_INTEGRAL)
}
pub const A: (u16, u16, bool) = check::<Fix<13, 3, Signed>>();
pub const B: (u16, u16, bool) = check::<Flt<8, 23>>();

// RESULT, printed at runtime from the const values:
//
//   (17, 16, false) (32, 23, false)
//
// Two things. The subtraction the spec says needs const-expression bounds is an
// associated const computed in a const-fn body, which is not type position and
// needs no gate; the width can stay a const parameter and render as `13`. And
// the second tuple independently reproduces 01's finding 8: this derivation
// gives binary32 a significand of 23, where IEEE's precision is 24, because the
// hidden bit is not one of the ten axes.
//
// The premise this falsifies is that the derived width must be a TYPE. It must
// be, only where something is generic over it. arvo's one such consumer is the
// container projection, which is already Pattern C taking the width as a
// standalone const argument.
