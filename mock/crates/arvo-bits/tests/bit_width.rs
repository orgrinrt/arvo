//! `HasBitWidth::WIDTH` correctness across types and strategies.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::ifixed::IFixed;
use arvo::newtype::{FBits, IBits};
use arvo::strategy::{Cold, Hot, Precise, Warm};
use arvo::ufixed::UFixed;
use arvo_bits::{HasBitWidth, Byte, DWord, Nibble, QWord, Word};

#[test]
fn ufixed_width_equals_i_plus_f() {
    // `UFixed<IBits(3), FBits(5), Hot>` -> 8 logical bits.
    assert_eq!(
        <UFixed<{ IBits(3) }, { FBits(5) }, Hot> as HasBitWidth>::WIDTH.0,
        8
    );
    // `UFixed<IBits(12), FBits(4), Warm>` -> 16 logical bits.
    assert_eq!(
        <UFixed<{ IBits(12) }, { FBits(4) }, Warm> as HasBitWidth>::WIDTH.0,
        16
    );
    // `UFixed<IBits(32), FBits(0), Cold>` -> 32 logical bits.
    assert_eq!(
        <UFixed<{ IBits(32) }, { FBits::ZERO }, Cold> as HasBitWidth>::WIDTH.0,
        32
    );
    // `UFixed<IBits(48), FBits(16), Precise>` -> 64 logical bits.
    assert_eq!(
        <UFixed<{ IBits(48) }, { FBits(16) }, Precise> as HasBitWidth>::WIDTH.0,
        64
    );
}

#[test]
fn ifixed_width_adds_sign_bit() {
    // `IFixed<IBits(3), FBits(4), Hot>` -> 1 + 3 + 4 = 8 logical bits.
    assert_eq!(
        <IFixed<{ IBits(3) }, { FBits(4) }, Hot> as HasBitWidth>::WIDTH.0,
        8
    );
    // `IFixed<IBits(15), FBits(0), Warm>` -> 1 + 15 + 0 = 16.
    assert_eq!(
        <IFixed<{ IBits(15) }, { FBits::ZERO }, Warm> as HasBitWidth>::WIDTH.0,
        16
    );
    // `IFixed<IBits(31), FBits(32), Precise>` -> 1 + 31 + 32 = 64.
    assert_eq!(
        <IFixed<{ IBits(31) }, { FBits(32) }, Precise> as HasBitWidth>::WIDTH.0,
        64
    );
}

#[test]
fn alias_widths() {
    // Aliases default to Hot.
    assert_eq!(<Byte<Hot> as HasBitWidth>::WIDTH.0, 8);
    assert_eq!(<Nibble<Hot> as HasBitWidth>::WIDTH.0, 4);
    assert_eq!(<Word<Hot> as HasBitWidth>::WIDTH.0, 16);
    assert_eq!(<DWord<Hot> as HasBitWidth>::WIDTH.0, 32);
    assert_eq!(<QWord<Hot> as HasBitWidth>::WIDTH.0, 64);
}
