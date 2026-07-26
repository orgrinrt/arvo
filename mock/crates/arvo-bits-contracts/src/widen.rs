//! `Widen<T>` trait family and concrete impls.
//!
//! The bit-width counterpart to `Narrow<T>`. `widen_to` promotes a value
//! into a wider bit-width carrier. For primitive sources (full-carrier-
//! width values), unsigned widening zero-extends and signed widening
//! sign-extends through Rust's `as` cast.
//!
//! For sub-carrier-width `Bits<M, S, Sign>` the substrate's contract is
//! bit-pattern-preserving on both `Narrow` and `Widen`. The cross-Bits
//! forwarder calls the carrier-side primitive trait method without
//! explicit M-bit sign-extension; bits above M in the source carrier are
//! forwarded to the destination carrier as-is. For values constructed
//! via `Narrow` (which zero-pads above the logical width via
//! `(source as unsigned) & ((1 << N) - 1)`), this produces zeros above
//! the logical width in the widened result regardless of source sign.
//!
//! `widen_to_unmasked` is the hot-path variant for symmetry with
//! `Narrow`'s unmasked variant; on this trait family it is equivalent
//! to `widen_to` because the carrier-side `as` cast preserves bit
//! pattern unconditionally.

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
/// `Bits<M, S, Sign>`). `T` is the target carrier type. For primitive
/// sources, unsigned widening zero-extends and signed widening sign-
/// extends through Rust's `as` cast. For sub-carrier-width
/// `Bits<M, ...>` sources, the cross-Bits forwarder preserves the
/// carrier bit pattern (no explicit sign-extension at bit M-1); see
/// the module-level doc for the contract.
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
    /// Sign-extends at the carrier's high bit for signed primitive
    /// sources. Zero-extends for unsigned primitive sources. For
    /// sub-carrier-width `Bits<M, ...>` sources, forwards the
    /// carrier bit pattern through the carrier-side widen; bits
    /// above the source's logical width are preserved as-is, not
    /// re-sign-extended at bit M-1.
    fn widen_to(self) -> T
    where
        Self: Sized;

    /// Hot-path widen for symmetry with `Narrow::narrow_to_unmasked`.
    ///
    /// On this trait family `widen_to_unmasked` is equivalent to
    /// `widen_to` because the carrier-side `as` cast preserves bit
    /// pattern unconditionally; the variant exists so consumer
    /// generic code over the refit family can name both directions
    /// uniformly.
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

const impl<const M: u16, const N: u16, S: Strategy, Sign: Signedness> Widen<Bits<N, S, Sign>>
    for Bits<M, S, Sign>
where
    S: BitsContainerFor<M, Sign>,
    S: BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<M, Sign>>::T: [const] Widen<<S as BitsContainerFor<N, Sign>>::T>,
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
