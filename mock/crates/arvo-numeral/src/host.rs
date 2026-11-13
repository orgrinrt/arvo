//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The door for a host integer, stated over the slot index.
//!
//! A host integer names a slot index, and an index is on the grid, so nothing
//! rounds on the way in and the door needs only the completion. That is done at
//! 64 bits, by the 64-bit rung's own adaptation, and the format's exact step is
//! never on this path.

use core::mem::size_of;

use arvo_format::{Bool, Policy, Slots};

use crate::admit::Sole;
use crate::check::DebugCheck;
use crate::machine::{Machine, copy};
use crate::num::{Num, SlotsOf};
use crate::rung;

/// A host integer a value takes its slot index from and gives it as.
///
/// The host's machine integers of 8 to 64 bits and the two pointer-width ones.
/// Sealed: the door's arithmetic is written for these and no others.
pub trait HostInteger: Machine {}

macro_rules! host_integer {
    ($($t:ty),+ $(,)?) => {
        $( impl HostInteger for $t {} )+
    };
}

host_integer! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize }

/// A host integer at 64 bits: its two's complement pattern, extended by its
/// own sign, and the two facts a 64-bit comparison of the other sign would
/// lose.
#[derive(Clone, Copy)]
struct Wide {
    bits:     u64,
    /// Below zero, which only a signed host integer is.
    negative: bool,
    /// Above the signed 64-bit maximum, which only an unsigned one is.
    above:    bool,
}

const fn signed(x: i64) -> Wide {
    Wide {
        bits:     x as u64,
        negative: x < 0,
        above:    false,
    }
}

const fn unsigned(x: u64) -> Wide {
    Wide {
        bits:     x,
        negative: false,
        above:    x > i64::MAX as u64,
    }
}

/// The host integer widened. One arm survives monomorphisation.
const fn widen<H: HostInteger>(host: H) -> Wide {
    match const { (size_of::<H>(), H::SIGNED) } {
        (1, true) => signed(copy::<H, i8>(host) as i64),
        (1, false) => unsigned(copy::<H, u8>(host) as u64),
        (2, true) => signed(copy::<H, i16>(host) as i64),
        (2, false) => unsigned(copy::<H, u16>(host) as u64),
        (4, true) => signed(copy::<H, i32>(host) as i64),
        (4, false) => unsigned(copy::<H, u32>(host) as u64),
        (8, true) => signed(copy::<H, i64>(host)),
        (8, false) => unsigned(copy::<H, u64>(host)),
        _ => unreachable!(),
    }
}

impl<S: Sole, C: DebugCheck> Num<S, C> {
    /// The member at the slot index a host integer names, completed by the
    /// declared overflow policy where the index is outside the range.
    ///
    /// Equal to `adapted(Exact::on_grid(Slot::at(host)))` for every host value,
    /// and computed at 64 bits. Under the `Abort` check a debug build aborts on
    /// an index outside the range, which `holds_index` asks ahead; under
    /// `Quiet` no build does.
    #[must_use]
    pub const fn from_index<H: HostInteger>(host: H) -> Self {
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                Self::holds_index(host).get(),
                "the host integer names a slot index outside the signature's range, and the `Abort` \
                 check aborts on that in a debug build. Name the `Quiet` check where the completion \
                 the overflow policy gives is what you mean"
            );
        }
        let wide = widen(host);
        let width = const { S::WIDTH.count() };
        // The two cases in which the host's sign and the range's differ are
        // answered before any comparison, since a cast would lose them.
        let index = if const { S::SIGNED.get() } {
            let completed = match S::POLICY {
                Policy::Wrap => rung::adapt_s64(wide.bits as i64, width, Policy::Wrap),
                policy => {
                    let x = if wide.above { i64::MAX } else { wide.bits as i64 };
                    rung::adapt_s64(x, width, policy)
                },
            };
            rung::index_s64(completed)
        } else {
            let completed = match S::POLICY {
                Policy::Wrap => rung::adapt_u64(wide.bits, width, Policy::Wrap),
                policy => {
                    let x = if wide.negative {
                        if cfg!(feature = "mutant-index") { u64::MAX } else { 0 }
                    } else {
                        wide.bits
                    };
                    rung::adapt_u64(x, width, policy)
                },
            };
            rung::index_u64(completed)
        };
        Self::at_index(index)
    }

    /// Whether the slot index a host integer names is a member, so `from_index`
    /// needs no completion.
    #[must_use]
    pub const fn holds_index<H: HostInteger>(host: H) -> Bool {
        let () = S::ADMITTED;
        let wide = widen(host);
        let shift = 64 - const { S::WIDTH.count() };
        Bool::of(if const { S::SIGNED.get() } {
            let x = wide.bits as i64;
            !wide.above && x >= i64::MIN >> shift && x <= i64::MAX >> shift
        } else {
            !wide.negative && wide.bits <= u64::MAX >> shift
        })
    }

    /// The member's slot index as the host integer `H`. An unwrap door.
    ///
    /// Refused at build where the signature's slot range is not inside `H`'s,
    /// so it never truncates.
    #[must_use]
    pub const fn index<H: HostInteger>(self) -> H {
        const {
            let bits = 8 * size_of::<H>() as u32;
            let (lowest, highest) = if H::SIGNED {
                (-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1)
            } else {
                (0, (1i128 << bits) - 1)
            };
            assert!(
                <SlotsOf<S> as Slots>::MIN.index() >= lowest
                    && <SlotsOf<S> as Slots>::MAX.index() <= highest,
                "`index::<H>()` names a host integer that does not hold every slot index of the \
                 signature's range. Name one that does: a wider one, or one of the range's sign"
            );
        }
        let index = self.slot().index();
        match const { (size_of::<H>(), H::SIGNED) } {
            (1, true) => copy::<i8, H>(index as i8),
            (1, false) => copy::<u8, H>(index as u8),
            (2, true) => copy::<i16, H>(index as i16),
            (2, false) => copy::<u16, H>(index as u16),
            (4, true) => copy::<i32, H>(index as i32),
            (4, false) => copy::<u32, H>(index as u32),
            (8, true) => copy::<i64, H>(index as i64),
            (8, false) => copy::<u64, H>(index as u64),
            _ => unreachable!(),
        }
    }
}
