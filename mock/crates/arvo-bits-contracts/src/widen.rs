//! `Widen<T>` trait family and concrete impls.
//!
//! The bit-width counterpart to `Narrow<T>`. `widen_to` promotes a value
//! into a wider bit-width carrier; unsigned widening zero-extends, signed
//! widening sign-extends. For sub-carrier-width `Bits<M, S, Signed>` the
//! cross-Bits impl masks bits 0..M of the source carrier and sign-extends
//! bit M-1, using M from impl context.
//!
//! `widen_to_unmasked` skips the source-side mask for the hot case where
//! bits above the source's logical width are already zero.

use arvo_storage::Bits;
use arvo_strategy::{BitsContainerFor, Signedness, Strategy};

/// Widened-result expression alias.
///
/// Cosmetic alias for the result of `widen_to` so consumer signatures
/// read `fn foo() -> Widened<u64>` instead of bare `u64`. Documents
/// the value's bit-width promotion at the type level without
/// changing the carrier.
pub type Widened<T> = T;

/// Widen `Self` into a wider bit-width carrier `T`.
///
/// `Self` is a narrower raw value (a bare primitive or a
/// `Bits<M, S, Sign>`). `T` is the target carrier type. Unsigned
/// widening zero-extends through Rust's `as` cast; signed widening
/// sign-extends.
///
/// Asymmetric with `Narrow<T>`: `Widen<T>` does NOT carry a const-N
/// parameter. The source bit-width is determined by impl context
/// (carrier `BITS` for primitives, `M` for `Bits<M, S, Sign>`); a
/// const parameter on `widen_to` would be informational at best and
/// never drives impl behaviour. The asymmetry is deliberate and
/// documented in `DESIGN.md`.
pub const trait Widen<T> {
    /// Widen `Self` into `T`.
    ///
    /// Zero-extends for unsigned source-target. Sign-extends for
    /// signed source-target. For sub-carrier-width `Bits<M, ...>`
    /// source the body masks bits 0..M before the cast.
    fn widen_to(self) -> T
    where
        Self: Sized;

    /// Widen without source-side masking.
    ///
    /// Sound when the caller knows bits above the source's logical
    /// width are already zero (well-formed `Bits<M, ...>`, primitive
    /// source where all bits are valid). Skips the source-side mask
    /// in the body.
    fn widen_to_unmasked(self) -> T
    where
        Self: Sized;
}

// --- Cross-primitive Widen impls ---------------------------------------

macro_rules! impl_widen_u {
    ($src:ty => $($dst:ty),+) => {
        $(
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cross-primitive widen impl on bare primitives that the trait was designed to bridge; tracked: #290
            impl const Widen<$dst> for $src {
                #[inline(always)]
                fn widen_to(self) -> $dst { self as $dst }

                #[inline(always)]
                fn widen_to_unmasked(self) -> $dst { self as $dst }
            }
        )+
    };
}

impl_widen_u!(u8 => u16, u32, u64, u128);
impl_widen_u!(u16 => u32, u64, u128);
impl_widen_u!(u32 => u64, u128);
impl_widen_u!(u64 => u128);

macro_rules! impl_widen_i {
    ($src:ty => $($dst:ty),+) => {
        $(
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cross-primitive signed widen via sign-extending `as` cast; tracked: #290
            impl const Widen<$dst> for $src {
                #[inline(always)]
                fn widen_to(self) -> $dst { self as $dst }

                #[inline(always)]
                fn widen_to_unmasked(self) -> $dst { self as $dst }
            }
        )+
    };
}

impl_widen_i!(i8 => i16, i32, i64, i128);
impl_widen_i!(i16 => i32, i64, i128);
impl_widen_i!(i32 => i64, i128);
impl_widen_i!(i64 => i128);

// --- Typed Bits<M, S, Sign> -> Bits<N, S, Sign> for M < N ---------------
//
// Forwards through the underlying primitive `Widen<T_N>` impl on the
// source's container type. Where M < N is not enforced at the type level
// (parallel to Narrow's M > N non-enforcement); consumers supplying M >= N
// still get a valid result.

impl<const M: u16, const N: u16, S: Strategy, Sign: Signedness>
    const Widen<Bits<N, S, Sign>> for Bits<M, S, Sign>
where
    S: BitsContainerFor<M, Sign>,
    S: BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<M, Sign>>::T:
        ~const Widen<<S as BitsContainerFor<N, Sign>>::T>,
{
    #[inline(always)]
    fn widen_to(self) -> Bits<N, S, Sign> {
        let raw = self.to_raw();
        let widened = raw.widen_to();
        Bits::from_raw(widened)
    }

    #[inline(always)]
    fn widen_to_unmasked(self) -> Bits<N, S, Sign> {
        let raw = self.to_raw();
        let widened = raw.widen_to_unmasked();
        Bits::from_raw(widened)
    }
}
