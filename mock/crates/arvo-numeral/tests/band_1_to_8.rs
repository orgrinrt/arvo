//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Widths 1 to 8 through the value type, held in the 8-bit carrier.
//!
//! Exhaustive: every signature over both families, both completing markers and
//! every mode, every pair of members, against the oracle. Beside it the two
//! associativity laws the registry states, reproduced through the value type
//! with the registry's own counts.

mod common;

use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature, addition_is_associative};
use arvo_numeral::{Additive, Identity, Int, Num, Saturating, Sole, Uint, Wrapping};
use common::{Tally, exhaustive, range, value};

#[test]
fn every_signature_from_1_to_8_bits_adds_and_orders_as_the_oracle_does_on_every_pair() {
    let mut tally = Tally::default();
    every_signature_at!(1, exhaustive, &mut tally);
    every_signature_at!(2, exhaustive, &mut tally);
    every_signature_at!(3, exhaustive, &mut tally);
    every_signature_at!(4, exhaustive, &mut tally);
    every_signature_at!(5, exhaustive, &mut tally);
    every_signature_at!(6, exhaustive, &mut tally);
    every_signature_at!(7, exhaustive, &mut tally);
    every_signature_at!(8, exhaustive, &mut tally);
    tally.check("widths 1 to 8");
}

/// Triples whose two groupings differ, and the first of them in ascending
/// order, through the value type.
fn associativity<S: Sole>() -> (u64, Option<(i128, i128, i128)>) {
    let (min, max) = range::<S>();
    let mut diverging = 0;
    let mut first = None;
    for a in min ..= max {
        let va = value::<S>(a);
        for b in min ..= max {
            let vb = value::<S>(b);
            let ab = va.add(vb);
            for c in min ..= max {
                let vc = value::<S>(c);
                if ab.add(vc) != va.add(vb.add(vc)) {
                    diverging += 1;
                    first.get_or_insert((a, b, c));
                }
            }
        }
    }
    (diverging, first)
}

type Sig<P, O> = Signature<P, Adapt<TowardZero, O>>;

#[test]
fn additive_associativity_under_wrapping_holds_at_4_and_8_bits_of_both_signs() {
    assert_eq!(associativity::<Sig<Integer<4>, Wrapping>>().0, 0);
    assert_eq!(associativity::<Sig<Integer<8>, Wrapping>>().0, 0);
    assert_eq!(associativity::<Sig<UFixed<4, 0>, Wrapping>>().0, 0);
    assert_eq!(associativity::<Sig<UFixed<8, 0>, Wrapping>>().0, 0);
    assert!(addition_is_associative::<Sig<Integer<8>, Wrapping>>().get());
}

#[test]
fn additive_associativity_under_saturation_holds_unsigned_and_fails_signed_by_the_registry_counts()
{
    assert_eq!(associativity::<Sig<UFixed<4, 0>, Saturating>>().0, 0);
    assert_eq!(associativity::<Sig<UFixed<8, 0>, Saturating>>().0, 0);
    assert_eq!(associativity::<Sig<Integer<4>, Saturating>>().0, 952);
    assert_eq!(
        associativity::<Sig<Integer<8>, Saturating>>(),
        (4_177_792, Some((-128, -128, 1)))
    );
    assert!(!addition_is_associative::<Sig<Integer<8>, Saturating>>().get());
}

#[test]
fn a_constant_is_the_member_it_names() {
    type I = Int<5, Adapt<TowardZero, Wrapping>>;
    assert_eq!(I::constant::<-16>().slot().index(), -16);
    assert_eq!(I::constant::<15>().slot().index(), 15);
    assert_eq!(<I as Identity<Additive>>::IDENTITY, I::constant::<0>());
}

#[test]
fn adapted_rounds_a_position_between_slots_by_the_mode() {
    use arvo_format::rounding::{Ceil, Floor, HalfEven};
    use arvo_format::{Exact, Fraction, Slot};
    let half_past_two = Exact::between(Slot::at(2), Fraction::of(1, 2));
    assert_eq!(
        Int::<8, Adapt<Floor, Wrapping>>::adapted(half_past_two)
            .slot()
            .index(),
        2
    );
    assert_eq!(
        Int::<8, Adapt<Ceil, Wrapping>>::adapted(half_past_two)
            .slot()
            .index(),
        3
    );
    assert_eq!(
        Int::<8, Adapt<HalfEven, Wrapping>>::adapted(half_past_two)
            .slot()
            .index(),
        2
    );
}

#[test]
fn a_value_occupies_the_carrier_the_placement_derives_and_no_more() {
    use core::mem::size_of;
    assert_eq!(size_of::<Uint<1, Adapt<TowardZero, Wrapping>>>(), 1);
    assert_eq!(size_of::<Uint<8, Adapt<TowardZero, Wrapping>>>(), 1);
    assert_eq!(size_of::<Int<9, Adapt<TowardZero, Wrapping>>>(), 2);
    assert_eq!(size_of::<Int<13, Adapt<TowardZero, Wrapping>>>(), 2);
    assert_eq!(size_of::<Int<17, Adapt<TowardZero, Wrapping>>>(), 4);
    assert_eq!(size_of::<Uint<33, Adapt<TowardZero, Wrapping>>>(), 8);
    assert_eq!(size_of::<Uint<64, Adapt<TowardZero, Wrapping>>>(), 8);
    assert_eq!(
        size_of::<Num<Signature<UFixed<12, 4>, Adapt<TowardZero, Wrapping>>>>(),
        2
    );
}

#[test]
fn the_pointer_width_values_carry_every_host_value_through() {
    use arvo_numeral::{ISize, USize};
    for x in [0, 1, usize::MAX / 2, usize::MAX] {
        assert_eq!(USize::from_host(x).host(), x);
    }
    for x in [isize::MIN, -1, 0, 1, isize::MAX] {
        assert_eq!(ISize::from_host(x).host(), x);
    }
    assert_eq!(USize::from_host(40).add(USize::from_host(2)).host(), 42);
}

/// What `Hash` writes for a value, through the standard hasher.
fn hash_of<T: core::hash::Hash>(t: &T) -> u64 {
    use core::hash::Hasher;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

#[test]
fn the_hash_and_the_debug_read_the_slot_and_not_the_carrier() {
    type U = Uint<4, Adapt<TowardZero, Wrapping>>;
    type I = Int<4, Adapt<TowardZero, Wrapping>>;
    let (u, i) = (U::constant::<5>(), I::constant::<-3>());
    assert_eq!(hash_of(&u), hash_of(&u.slot().index()));
    assert_eq!(hash_of(&i), hash_of(&i.slot().index()));
    assert_eq!(format!("{u:?}"), format!("Num({:?})", u.slot()));
    assert_eq!(format!("{i:?}"), format!("Num({:?})", i.slot()));
}
