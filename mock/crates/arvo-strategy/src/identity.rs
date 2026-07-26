//! The canonical typed-const surfaces: `Bounded`, `Identity<Op>`,
//! `SignedIdentity`.
//!
//! `Bounded` carries a type's minimum and maximum. `Identity<Op>` carries
//! the identity element of an operation, one impl per operation the type
//! actually has one for. Substrate types gain both through blanket impls
//! keyed on the underlying primitive.
//!
//! Split out of `arith.rs`, where these sat behind eight hundred lines of
//! arithmetic macros they have nothing to do with.

use crate::Picker;

/// Per-type bottom/top const surface.
///
/// `pub const trait`. Implemented for u8 / u16 / u32 / u64 / u128 and
/// i8 / i16 / i32 / i64 / i128 by macro impls below. Substrate types
/// that wrap these (Bits, UFixed, IFixed, Mask families) gain Bounded
/// via blanket impls keyed on the underlying primitive.
pub const trait Bounded: Sized {
    /// The minimum representable value of this type.
    const MIN: Self;
    /// The maximum representable value of this type.
    const MAX: Self;
}

/// The operation whose identity is addition.
pub struct Additive;

/// The operation whose identity is multiplication.
pub struct Multiplicative;

/// The identity element of `Op` in this type.
///
/// An identity is only ever an identity with respect to an operation, so
/// the operation is a parameter and there is one constant. The earlier
/// shape carried `ZERO` and `ONE` together, which named two instances of
/// one concept after their values and forced a type to have both or
/// neither.
///
/// **Absence is a statement.** An identity element must be a value of the
/// type it is an identity for, so where an operation has no identity in
/// the type there is no impl, and naming `IDENTITY` for it fails to
/// resolve rather than returning something plausible. `UFixed<0, F, S>`
/// spans `[0, 1)`, which contains zero and does not contain one: it impls
/// `Identity<Additive>` and no `Identity<Multiplicative>`.
///
/// The parameter also generalises. `Identity<Min>`, `Identity<Max>` and
/// `Identity<BitOr>` each have their own correct element and arrive as
/// new impls rather than as a third and fourth constant on this trait.
pub const trait Identity<Op>: Sized {
    /// The identity element of `Op` in this type.
    const IDENTITY: Self;
}

/// Compresses the integer-bit count into the two-element space "one is
/// representable" and "one is not".
///
/// The raw encoding of one in a fixed-point type is `1 << F`, and the
/// container is `I + F` bits wide, so the encoding fits exactly when
/// there is at least one integer bit to hold it. At `I == 0` the final
/// carry leaves the container: the wrapping strategies wrapped it to zero
/// and to the container minimum, and the saturating one clamped it just
/// below one.
///
/// Same shape as `container::tag_hot_cold`: a const fn mapping a large
/// const space onto a small enumerated tag set, so impls can enumerate
/// the tags instead of the space.
#[inline(always)]
pub const fn tag_one_representable(int_bits: u16) -> usize {
    if int_bits >= 1 {
        0
    } else {
        1
    }
}

/// Witness that one is representable at a given tag.
///
/// Sealed via `crate::sealed::Sealed`, so downstream cannot supply the
/// impl this deliberately withholds. `Picker` is the sole implementor,
/// and only at tag 0. There is no `impl OneRepresentable<1>`: that
/// absence is how `Identity<Multiplicative>` stops existing for a purely
/// fractional type.
#[diagnostic::on_unimplemented(
    message = "this type has no multiplicative identity",
    note = "A purely fractional fixed-point type has zero integer bits, so it spans [0, 1) unsigned or [-1, 1) signed and one is not a value of it. Its `Identity<Multiplicative>` impl does not exist, because an identity element must be a value of the type it is an identity for. `Identity<Additive>` is unaffected and available at every width. If you need to multiply by one here, the type needs at least one integer bit."
)]
pub trait OneRepresentable<const TAG: usize>: crate::sealed::Sealed {}

impl OneRepresentable<0> for Picker {}

macro_rules! impl_bounded_identity {
    ($zero:expr, $one:expr, $($ty:ty),+) => {
        $(
            impl const Bounded for $ty {
                const MIN: Self = <$ty>::MIN;
                const MAX: Self = <$ty>::MAX;
            }
            impl const Identity<Additive> for $ty {
                const IDENTITY: Self = $zero;
            }
            impl const Identity<Multiplicative> for $ty {
                const IDENTITY: Self = $one;
            }
        )+
    };
}

impl_bounded_identity!(0, 1, u8, u16, u32, u64, u128, usize);
impl_bounded_identity!(0, 1, i8, i16, i32, i64, i128, isize);

// Float Bounded / Identity. Bottom-out at the language-defined MIN /
// MAX inherents and 0.0 / 1.0 literals. The `Ieee` seal on the public
// surface restricts these traits' use to `f32` / `f64` exposure
// through `FastFloat<F>` / `StrictFloat<F>`, but the substrate impls
// land here so the canonical const surface stays unified.
//
// A float has both identities at every width, so neither is conditional.
impl_bounded_identity!(0.0, 1.0, f32, f64);

/// Per-signed-type negative-one const surface.
///
/// `pub const trait`. Implemented for `i8` / `i16` / `i32` / `i64` /
/// `i128` / `isize` and substrate types that wrap signed primitives
/// (`IFixed`, `Bits<N, S, Signed>`).
///
/// Minus one is not the identity of any operation, so it does not fold
/// into `Identity<Op>` and keeps its own trait. It also does not inherit
/// the representability condition: a signed purely fractional type spans
/// `[-1, 1)`, so minus one **is** representable there, as the container
/// minimum, while one is not. The asymmetry is real, and bounding this
/// on `Identity<Multiplicative>` would have made `NEG_ONE` unreachable on
/// exactly the types that have it.
pub const trait SignedIdentity: Sized {
    /// The additive inverse of the multiplicative identity (signed -1).
    const NEG_ONE: Self;
}

macro_rules! impl_signed_identity {
    ($neg_one:expr, $($ty:ty),+) => {
        $(impl const SignedIdentity for $ty {
            const NEG_ONE: Self = $neg_one;
        })+
    };
}

impl_signed_identity!(-1, i8, i16, i32, i64, i128, isize);

// ML / scientific code uses `-1.0` as a canonical constant; the substrate
// provides it through the same trait surface as signed integers so
// consumers reach for `<F as SignedIdentity>::NEG_ONE` rather than a
// type-specific path.
impl_signed_identity!(-1.0, f32, f64);
