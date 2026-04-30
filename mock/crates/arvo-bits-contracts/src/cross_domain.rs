//! Cross-domain primitive ↔ `Bits` direct refit impls.
//!
//! These let consumers narrow a bare primitive directly into a typed
//! `Bits<N, S, Sign>`, or widen a typed `Bits<M, S, Sign>` directly into a
//! bare primitive carrier, without an intermediate `Bits::from_raw` wrap.
//! Underlies the `Bits::from_narrowed` / `Bits::from_widened` ergonomic
//! constructors (housed in `arvo-bits` via the `BitsRefitCtor` extension
//! trait). The impls forward through the carrier-side trait impl and wrap
//! or unwrap as appropriate.

use arvo_storage::Bits;
use arvo_strategy::{BitsContainerFor, Signed, Strategy, Unsigned};

use crate::{Narrow, Widen};

// --- Pri -> Bits<N, S, Unsigned> (Narrow) ------------------------------

macro_rules! impl_narrow_pri_to_bits_unsigned {
    ($($pri:ty),+) => {
        $(
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cross-domain narrow direct impl bridging primitive into Bits; tracked: #290
            impl<const N: u16, S: Strategy>
                const Narrow<Bits<N, S, Unsigned>> for $pri
            where
                S: BitsContainerFor<N, Unsigned>,
                $pri: ~const Narrow<<S as BitsContainerFor<N, Unsigned>>::T>,
            {
                #[inline(always)]
                fn narrow_to<const W: u16>(self) -> Bits<N, S, Unsigned> {
                    Bits::from_raw(<$pri as Narrow<_>>::narrow_to::<W>(self))
                }

                #[inline(always)]
                fn narrow_to_unmasked<const W: u16>(self) -> Bits<N, S, Unsigned> {
                    Bits::from_raw(<$pri as Narrow<_>>::narrow_to_unmasked::<W>(self))
                }
            }
        )+
    };
}

impl_narrow_pri_to_bits_unsigned!(u8, u16, u32, u64, u128);

macro_rules! impl_narrow_pri_to_bits_signed {
    ($($pri:ty),+) => {
        $(
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cross-domain narrow direct impl bridging primitive into signed Bits; tracked: #290
            impl<const N: u16, S: Strategy>
                const Narrow<Bits<N, S, Signed>> for $pri
            where
                S: BitsContainerFor<N, Signed>,
                $pri: ~const Narrow<<S as BitsContainerFor<N, Signed>>::T>,
            {
                #[inline(always)]
                fn narrow_to<const W: u16>(self) -> Bits<N, S, Signed> {
                    Bits::from_raw(<$pri as Narrow<_>>::narrow_to::<W>(self))
                }

                #[inline(always)]
                fn narrow_to_unmasked<const W: u16>(self) -> Bits<N, S, Signed> {
                    Bits::from_raw(<$pri as Narrow<_>>::narrow_to_unmasked::<W>(self))
                }
            }
        )+
    };
}

impl_narrow_pri_to_bits_signed!(i8, i16, i32, i64, i128);

// --- Bits<M, S, Sign> -> Pri (Widen) -----------------------------------

macro_rules! impl_widen_bits_to_pri_unsigned {
    ($($pri:ty),+) => {
        $(
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cross-domain widen direct impl bridging Bits into primitive; tracked: #290
            impl<const M: u16, S: Strategy>
                const Widen<$pri> for Bits<M, S, Unsigned>
            where
                S: BitsContainerFor<M, Unsigned>,
                <S as BitsContainerFor<M, Unsigned>>::T: ~const Widen<$pri>,
            {
                #[inline(always)]
                fn widen_to(self) -> $pri {
                    <_ as Widen<$pri>>::widen_to(self.to_raw())
                }

                #[inline(always)]
                fn widen_to_unmasked(self) -> $pri {
                    <_ as Widen<$pri>>::widen_to_unmasked(self.to_raw())
                }
            }
        )+
    };
}

impl_widen_bits_to_pri_unsigned!(u8, u16, u32, u64, u128);

macro_rules! impl_widen_bits_to_pri_signed {
    ($($pri:ty),+) => {
        $(
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cross-domain widen direct impl bridging signed Bits into primitive; tracked: #290
            impl<const M: u16, S: Strategy>
                const Widen<$pri> for Bits<M, S, Signed>
            where
                S: BitsContainerFor<M, Signed>,
                <S as BitsContainerFor<M, Signed>>::T: ~const Widen<$pri>,
            {
                #[inline(always)]
                fn widen_to(self) -> $pri {
                    <_ as Widen<$pri>>::widen_to(self.to_raw())
                }

                #[inline(always)]
                fn widen_to_unmasked(self) -> $pri {
                    <_ as Widen<$pri>>::widen_to_unmasked(self.to_raw())
                }
            }
        )+
    };
}

impl_widen_bits_to_pri_signed!(i8, i16, i32, i64, i128);
