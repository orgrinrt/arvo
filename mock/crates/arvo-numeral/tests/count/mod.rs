//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the two counting targets share: the lists, the positions, and the
//! checks that read a count as a call, which both paths answer.
//!
//! Nothing here names a feature of the toolchain. A consumer counts in a value
//! through these bounds and no gate of its own.

#![allow(dead_code)]

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::Integer;
use arvo_format::rounding::{HalfEven, TowardZero};
use arvo_format::{Adapt, Signature, Slot};
use arvo_numeral::{Abort, DebugCheck, Fix, Int, Num, Quiet, Sole, USize, Uint};
use notko_hlist::{Cons, Empty, Here, Length, Position, There};

pub struct A;
pub struct B;
pub struct C;

pub type L0 = Empty;
pub type L1 = Cons<A, L0>;
pub type L2 = Cons<B, L1>;
pub type L3 = Cons<C, L2>;
pub type L4 = Cons<A, L3>;
pub type L5 = Cons<B, L4>;

pub type P0 = Here;
pub type P1 = There<P0>;
pub type P2 = There<P1>;
pub type P3 = There<P2>;

pub type Host = USize<Adapt<TowardZero, Wrap>>;
pub type Byte = Uint<8, Adapt<TowardZero, Saturate>>;
pub type Signed = Int<7, Adapt<TowardZero, Wrap>, Quiet>;
pub type TwoBitsWrap = Uint<2, Adapt<TowardZero, Wrap>, Quiet>;
pub type TwoBitsSaturate = Uint<2, Adapt<TowardZero, Saturate>, Quiet>;
pub type TwoBitsAbort = Uint<2, Adapt<TowardZero, Wrap>>;
/// Eighths, where one is the slot at eight.
pub type Eighths = Fix<8, -3, Adapt<HalfEven, Saturate>>;

/// The slot of a list's length, generic over the list, the signature and the
/// check, bounded by `Length` and nothing more.
pub fn length<L: Length<Num<S, C>>, S: Sole, C: DebugCheck>() -> Slot {
    L::len().slot()
}

/// The slot of a position's index, the same way.
pub fn index<P: Position<Num<S, C>>, S: Sole, C: DebugCheck>() -> Slot {
    P::index().slot()
}

/// Each list's length in one count type, as the slots at `0, 1, 2, 3, 4`
/// scaled by where the format puts one.
pub fn lengths_of<N>() -> [N; 5]
where
    L0: Length<N>,
    L1: Length<N>,
    L2: Length<N>,
    L3: Length<N>,
    L4: Length<N>,
{
    [
        <L0 as Length<N>>::len(),
        <L1 as Length<N>>::len(),
        <L2 as Length<N>>::len(),
        <L3 as Length<N>>::len(),
        <L4 as Length<N>>::len(),
    ]
}

/// Every check that reads a count as a call.
pub fn the_counts_as_calls() {
    let host = lengths_of::<Host>();
    let byte = lengths_of::<Byte>();
    let signed = lengths_of::<Signed>();
    let eighths = lengths_of::<Eighths>();
    for n in 0 .. 5 {
        assert_eq!(host[n].host(), n, "the host-width count of {n} cells");
        assert_eq!(byte[n].slot().index(), n as i128);
        assert_eq!(signed[n].slot().index(), n as i128);
        // A count on a fixed-point grid is the member `n`, at `n` times eight.
        assert_eq!(eighths[n].slot().index(), 8 * n as i128);
    }

    assert_eq!(<P0 as Position<Host>>::index().host(), 0);
    assert_eq!(<P1 as Position<Host>>::index().host(), 1);
    assert_eq!(<P3 as Position<Byte>>::index().slot().index(), 3);
    assert_eq!(<P2 as Position<Eighths>>::index().slot().index(), 16);

    // Through code generic over the signature and the check.
    type S7 = Signature<Integer<7>, Adapt<TowardZero, Wrap>>;
    assert_eq!(length::<L4, S7, Quiet>().index(), 4);
    assert_eq!(length::<L4, S7, Abort>().index(), 4);
    assert_eq!(index::<P3, S7, Quiet>().index(), 3);

    // Past the range the count is the signature's sum: five cells in two bits
    // wrap to one, or stay at the top.
    assert_eq!(<L3 as Length<TwoBitsWrap>>::len().slot().index(), 3);
    assert_eq!(<L4 as Length<TwoBitsWrap>>::len().slot().index(), 0);
    assert_eq!(<L5 as Length<TwoBitsWrap>>::len().slot().index(), 1);
    assert_eq!(<L5 as Length<TwoBitsSaturate>>::len().slot().index(), 3);
    // Three cells fit two bits, and the default check passes what fits.
    assert_eq!(<L3 as Length<TwoBitsAbort>>::len().slot().index(), 3);
}
