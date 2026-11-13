//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Widths 9 to 16 through the value type, held in the 16-bit carrier: the
//! edge lattice and seeded pairs of every signature, both families, both
//! overflow policies and every mode, against the oracle.

mod common;

use common::{Tally, lattice_and_seeded_products, on_the_lattice, seeded};

fn lattice_and_seeded<S: arvo_numeral::Sole>(tally: &mut Tally) {
    on_the_lattice::<S>(tally);
    seeded::<S>(tally);
}

#[test]
fn every_signature_from_9_to_16_bits_adds_subtracts_negates_and_orders_as_the_oracle_does() {
    let mut tally = Tally::default();
    every_signature_at!(9, lattice_and_seeded, &mut tally);
    every_signature_at!(10, lattice_and_seeded, &mut tally);
    every_signature_at!(11, lattice_and_seeded, &mut tally);
    every_signature_at!(12, lattice_and_seeded, &mut tally);
    every_signature_at!(13, lattice_and_seeded, &mut tally);
    every_signature_at!(14, lattice_and_seeded, &mut tally);
    every_signature_at!(15, lattice_and_seeded, &mut tally);
    every_signature_at!(16, lattice_and_seeded, &mut tally);
    tally.check("widths 9 to 16");
}

#[test]
fn every_signature_from_9_to_16_bits_multiplies_as_the_oracle_does() {
    let mut tally = Tally::default();
    every_signature_at!(9, lattice_and_seeded_products, &mut tally);
    every_signature_at!(10, lattice_and_seeded_products, &mut tally);
    every_signature_at!(11, lattice_and_seeded_products, &mut tally);
    every_signature_at!(12, lattice_and_seeded_products, &mut tally);
    every_signature_at!(13, lattice_and_seeded_products, &mut tally);
    every_signature_at!(14, lattice_and_seeded_products, &mut tally);
    every_signature_at!(15, lattice_and_seeded_products, &mut tally);
    every_signature_at!(16, lattice_and_seeded_products, &mut tally);
    tally.check_products("widths 9 to 16");
}
