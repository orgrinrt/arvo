//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The fixed-point aliases.
//!
//! Each alias is the written type, which a binding of one to the other checks
//! at build. The product under an alias rounds by the mode its parameter names,
//! shown on one tie that two modes send to different slots. A preset of this
//! file names an adaptation for the fixed-point formats, and the alias reads it.

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, UFixed};
use arvo_format::rounding::{Floor, HalfEven};
use arvo_format::{Adapt, Format, Signature};
use arvo_numeral::{Abort, Fix, NamesAdaptation, Num, Quiet, UFix};

#[test]
fn each_alias_is_the_written_type() {
    let lane: Fix<32, -16, Adapt<HalfEven, Saturate>> = Num::constant::<65536>();
    let written: Num<Signature<Fixed<32, -16>, Adapt<HalfEven, Saturate>>, Abort> = lane;
    assert_eq!(written.slot().index(), 65536);

    let unsigned: UFix<16, -8, Adapt<Floor, Wrap>, Quiet> = Num::constant::<256>();
    let written: Num<Signature<UFixed<16, -8>, Adapt<Floor, Wrap>>, Quiet> = unsigned;
    assert_eq!(written.slot().index(), 256);
}

/// A half times three eighths is twelve sixty-fourths, one and a half eighths:
/// a tie that half to even sends up to two and the floor cuts to one.
#[test]
fn the_product_rounds_by_the_mode_the_alias_names() {
    type Even = Fix<6, -3, Adapt<HalfEven, Saturate>>;
    type Cut = Fix<6, -3, Adapt<Floor, Saturate>>;
    assert_eq!(
        Even::constant::<4>()
            .mul(Even::constant::<3>())
            .slot()
            .index(),
        2
    );
    assert_eq!(
        Cut::constant::<4>()
            .mul(Cut::constant::<3>())
            .slot()
            .index(),
        1
    );

    type UEven = UFix<6, -3, Adapt<HalfEven, Saturate>>;
    type UCut = UFix<6, -3, Adapt<Floor, Saturate>>;
    assert_eq!(
        UEven::constant::<4>()
            .mul(UEven::constant::<3>())
            .slot()
            .index(),
        2
    );
    assert_eq!(
        UCut::constant::<4>()
            .mul(UCut::constant::<3>())
            .slot()
            .index(),
        1
    );
}

/// A preset of the consumer's: half to even and saturating, for every signed
/// and unsigned fixed-point format.
struct Lane;

impl<const BITS: u32, const FRAC: i32> NamesAdaptation<Fixed<BITS, FRAC>> for Lane
where
    Fixed<BITS, FRAC>: Format,
{
    type Adaptation = Adapt<HalfEven, Saturate>;
}

impl<const BITS: u32, const FRAC: i32> NamesAdaptation<UFixed<BITS, FRAC>> for Lane
where
    UFixed<BITS, FRAC>: Format,
{
    type Adaptation = Adapt<HalfEven, Saturate>;
}

#[test]
fn a_preset_names_the_adaptation_through_the_alias() {
    let through: Fix<6, -3, Lane> = Num::constant::<4>();
    let written: Fix<6, -3, Adapt<HalfEven, Saturate>> = through;
    assert_eq!(written.mul(Num::constant::<3>()).slot().index(), 2);
    // Thirty-one eighths and one more: the preset's policy holds the top.
    let top: Fix<6, -3, Lane, Quiet> = Num::constant::<31>();
    assert_eq!(top.add(Num::constant::<1>()).slot().index(), 31);

    let unsigned: UFix<6, -3, Lane, Quiet> = Num::constant::<63>();
    let written: UFix<6, -3, Adapt<HalfEven, Saturate>, Quiet> = unsigned;
    assert_eq!(written.add(Num::constant::<1>()).slot().index(), 63);
}

#[test]
fn a_lane_value_is_a_constant() {
    const PRODUCT: Fix<32, -16, Adapt<HalfEven, Saturate>> =
        Fix::<32, -16, Adapt<HalfEven, Saturate>>::constant::<98304>()
            .mul(Num::constant::<32768>());
    // One and a half times a half is three quarters.
    assert_eq!(PRODUCT.slot().index(), 49152);
}
