#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

//! arvo-bits — bit-storage aliases.
//!
//! `Bit` / `Nibble` / `Byte` / `Word` / `DWord` / `QWord` are
//! `arvo_storage::Bits<N, S>` aliases at the common power-of-two
//! widths. Default strategy is `Hot`: bit work typically wants the
//! minimum container and wrapping semantics.
//!
//! The bit-level trait declarations (`HasBitWidth`, `BitAccess`,
//! `BitSequence`, `BitLogic`) live in `arvo-bits-contracts`. The
//! blanket impls on `Bits<N, S>` also live there (orphan rule:
//! trait + foreign-type impls share a crate). This crate is the
//! domain-alias surface, nothing else.

// Convenience re-exports so consumers can pull bit-level traits and
// the bit-storage primitive from one crate. Trait declarations live
// in `arvo-bits-contracts`; storage primitive lives in `arvo-storage`.
pub use arvo_bits_contracts::{
    BitAccess, BitLogic, BitPrim, BitSequence, HasBitWidth, IBitContainer, IBitPrim, Narrow,
    Narrowed, UBitContainer, Widen, Widened,
};
pub use arvo_storage::Bits;
pub use arvo_strategy::{Hot, Strategy};

mod refit_constructors {
    use arvo_bits_contracts::{Narrow, Widen};
    use arvo_storage::Bits;
    use arvo_strategy::{BitsContainerFor, Signedness, Strategy};

    /// Sealed extension trait providing `from_narrowed` and
    /// `from_widened` ergonomic constructors on `Bits`.
    ///
    /// The methods read as inherent-on-Bits at consumer call sites
    /// because the trait is sealed and the blanket impl covers every
    /// `Bits<N, S, Sign>` shape. Source type `Src` is inferred from
    /// the value at the call site; no turbofish needed.
    pub const trait BitsRefitCtor<const N: u16, S: Strategy, Sign: Signedness>
    where
        S: BitsContainerFor<N, Sign>,
    {
        /// Construct from a wider-or-equal-bit-width source via `Narrow`.
        ///
        /// Routes through `Src::narrow_to::<N>(src)` and wraps with
        /// `from_raw`. `Self`'s outer N drives the mask width.
        fn from_narrowed<Src>(src: Src) -> Bits<N, S, Sign>
        where
            Src: ~const Narrow<<S as BitsContainerFor<N, Sign>>::T>;

        /// Construct from a narrower source via `Widen`.
        ///
        /// Routes through `Src::widen_to(src)` and wraps with
        /// `from_raw`. The source's bit-width derives from per-impl
        /// context (carrier `BITS` for primitives, `M` for
        /// `Bits<M, ...>`).
        fn from_widened<Src>(src: Src) -> Bits<N, S, Sign>
        where
            Src: ~const Widen<<S as BitsContainerFor<N, Sign>>::T>;
    }

    impl<const N: u16, S: Strategy, Sign: Signedness>
        const BitsRefitCtor<N, S, Sign> for Bits<N, S, Sign>
    where
        S: BitsContainerFor<N, Sign>,
    {
        #[inline(always)]
        fn from_narrowed<Src>(src: Src) -> Bits<N, S, Sign>
        where
            Src: ~const Narrow<<S as BitsContainerFor<N, Sign>>::T>,
        {
            Bits::from_raw(src.narrow_to::<N>())
        }

        #[inline(always)]
        fn from_widened<Src>(src: Src) -> Bits<N, S, Sign>
        where
            Src: ~const Widen<<S as BitsContainerFor<N, Sign>>::T>,
        {
            Bits::from_raw(src.widen_to())
        }
    }
}

pub use refit_constructors::BitsRefitCtor;

/// 1-bit opaque bit-pattern.
///
/// Use for column-stored flag data. `Bool` (in `arvo-storage`) is
/// the control-flow counterpart.
pub type Bit<S = Hot> = Bits<1, S>;

/// 4-bit opaque bit-pattern (half-byte).
pub type Nibble<S = Hot> = Bits<4, S>;

/// 8-bit opaque bit-pattern.
pub type Byte<S = Hot> = Bits<8, S>;

/// 16-bit opaque bit-pattern (x86 "word").
pub type Word<S = Hot> = Bits<16, S>;

/// 32-bit opaque bit-pattern (x86 "dword").
pub type DWord<S = Hot> = Bits<32, S>;

/// 64-bit opaque bit-pattern (x86 "qword"; arvo's widest logical
/// value).
pub type QWord<S = Hot> = Bits<64, S>;
