//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The three law rows the product owes, counted at the cells they state.
//!
//! `multiplicative_associativity_under_wrapping`,
//! `multiplicative_associativity_under_saturation` and
//! `distributivity_of_multiplication_over_addition` in the law registry each
//! name a holds region and a fails region. Every triple of members is counted
//! from tables of the adapted operations, so a cell said to hold counts zero and
//! a cell said to fail counts what its witness measured.

use super::{At, Sat, Wr};
use crate::adapt::DeclaredSignature;
use crate::addition::add;
use crate::apply::Dither;
use crate::format::Format;
use crate::multiplication::mul;
use crate::points::{Integer, UFixed};
use crate::slots::{Signed, Slots};
use crate::tests::table::Table;

/// Every pair of members multiplied under `S`.
fn products<S: DeclaredSignature>() -> Table {
    Table::of(
        <<S::Format as Format>::Slots as Slots>::MIN,
        <<S::Format as Format>::Slots as Slots>::MAX,
        |a, b| mul::<S>(a, b, Dither::UNUSED),
    )
}

/// Every pair of members added under `S`.
fn sums<S: DeclaredSignature>() -> Table {
    Table::of(
        <<S::Format as Format>::Slots as Slots>::MIN,
        <<S::Format as Format>::Slots as Slots>::MAX,
        |a, b| add::<S>(a, b, Dither::UNUSED),
    )
}

/// How many triples `(a * b) * c` and `a * (b * c)` disagree on, out of how
/// many.
fn associativity<S: DeclaredSignature>() -> (u64, u64) {
    let table = products::<S>();
    (table.divergent(), (table.members() as u64).pow(3))
}

/// How many triples `a * (b + c)` and `a * b + a * c` disagree on, out of how
/// many.
fn distributivity<S: DeclaredSignature>() -> (u64, u64) {
    let times = products::<S>();
    let plus = sums::<S>();
    let n = times.members();
    let mut wrong = 0u64;
    for a in 0 .. n {
        for b in 0 .. n {
            let ab = times.at(a, b);
            for c in 0 .. n {
                let left = times.at(a, plus.at(b, c));
                let right = plus.at(ab, times.at(a, c));
                wrong += u64::from(left != right);
            }
        }
    }
    (wrong, (n as u64).pow(3))
}

/// A share of triples in hundredths of a percent, rounded half up.
fn basis_points((wrong, of): (u64, u64)) -> u64 {
    (wrong * 10_000 + of / 2) / of
}

macro_rules! at_widths {
    ($law:ident, $sig:ident, $fmt:ident; $($w:literal),+) => {
        [$( $law::<$sig<$fmt<$w>>>() ),+]
    };
}

/// An unsigned range of `W` bits at the quantum one.
type UInt<const W: u32> = UFixed<W, 0>;

#[test]
fn wrapping_multiplication_is_associative_on_the_integer_grid_at_widths_three_to_eight() {
    for (wrong, of) in at_widths!(associativity, Wr, Integer; 3, 4, 5, 6, 7, 8) {
        assert_eq!(wrong, 0, "signed, out of {of}");
    }
    for (wrong, of) in at_widths!(associativity, Wr, UInt; 3, 4, 5, 6, 7, 8) {
        assert_eq!(wrong, 0, "unsigned, out of {of}");
    }
}

#[test]
fn wrapping_multiplication_is_not_associative_at_any_fraction_width_at_widths_three_to_seven() {
    // The row's fails cell is `F > 0` at widths three to seven, so every
    // fraction width from one to `W` is crossed at each of them, signed and
    // unsigned. Width eight is outside the cell, so nothing is asserted there.
    let mut crossed = 0;
    macro_rules! cross {
        ($w:literal; $($f:literal),+) => {
            $(
                let (wrong, of) = associativity::<Wr<At<Signed<$w>, { -$f }>>>();
                assert!(wrong > 0, "signed {} bits, {} fraction bits held, out of {of}", $w, $f);
                let (wrong, of) = associativity::<Wr<UFixed<$w, { -$f }>>>();
                assert!(wrong > 0, "unsigned {} bits, {} fraction bits held, out of {of}", $w, $f);
                crossed += 2;
            )+
        };
    }
    cross!(3; 1, 2, 3);
    cross!(4; 1, 2, 3, 4);
    cross!(5; 1, 2, 3, 4, 5);
    cross!(6; 1, 2, 3, 4, 5, 6);
    cross!(7; 1, 2, 3, 4, 5, 6, 7);
    assert_eq!(crossed, 2 * (3 + 4 + 5 + 6 + 7));
}

#[test]
fn saturating_multiplication_is_associative_on_the_unsigned_integer_grid() {
    for (wrong, of) in at_widths!(associativity, Sat, UInt; 3, 4, 5, 6, 7, 8) {
        assert_eq!(wrong, 0, "out of {of}");
    }
}

#[test]
fn saturating_multiplication_fails_associativity_on_the_signed_integer_grid_at_the_measured_counts()
{
    let counted = at_widths!(associativity, Sat, Integer; 3, 4, 5, 6).map(|(wrong, _)| wrong);
    assert_eq!(counted, [28, 160, 780, 3516]);
    // The row's witness, at four bits.
    type S = Sat<Integer<4>>;
    let (seven, minus_one) = (crate::slots::Slot::at(7), crate::slots::Slot::at(-1));
    let left = mul::<S>(
        mul::<S>(seven, seven, Dither::UNUSED),
        minus_one,
        Dither::UNUSED,
    );
    let right = mul::<S>(
        seven,
        mul::<S>(seven, minus_one, Dither::UNUSED),
        Dither::UNUSED,
    );
    assert_eq!((left.index(), right.index()), (-7, -8));
}

#[test]
fn multiplication_distributes_over_addition_on_the_unsigned_integer_grid() {
    for (wrong, of) in at_widths!(distributivity, Wr, UInt; 3, 4, 5, 6, 7, 8) {
        assert_eq!(wrong, 0, "wrapping, out of {of}");
    }
    for (wrong, of) in at_widths!(distributivity, Sat, UInt; 3, 4, 5, 6, 7, 8) {
        assert_eq!(wrong, 0, "saturating, out of {of}");
    }
}

#[test]
fn saturating_multiplication_fails_distributivity_on_the_signed_integer_grid_at_the_measured_counts()
 {
    // The row's fails cell names six and seven bits and puts 34.52% at six. The
    // probes it cites measured that share at four bits, 1414 of 4096, and the
    // law failing at every width they ran: `93_probes/p7` lines 15 to 47 and
    // `97_probes/p2` line 209. Ticket
    // `law_row_distributivity_names_w_6_and_7_for_a_failure_measure`.
    let counted = at_widths!(distributivity, Sat, Integer; 3, 4, 5, 6, 7);
    assert_eq!(counted.map(|(wrong, _)| wrong), [
        120, 1414, 13594, 119422, 1000674
    ]);
    assert_eq!(basis_points(counted[1]), 3452, "the share at four bits");
    assert_eq!(basis_points(counted[4]), 4772, "the share at seven bits");
}
