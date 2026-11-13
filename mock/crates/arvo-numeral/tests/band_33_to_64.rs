//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Widths 33 to 64 through the value type, held in the 64-bit carrier: the
//! edge lattice and seeded pairs of every signature, both families, both
//! overflow policies and every mode, against the oracle.

mod common;

use common::{Tally, on_the_lattice, seeded};

fn lattice_and_seeded<S: arvo_numeral::Sole>(tally: &mut Tally) {
    on_the_lattice::<S>(tally);
    seeded::<S>(tally);
}

#[test]
fn every_signature_from_33_to_64_bits_adds_and_orders_as_the_oracle_does() {
    let mut tally = Tally::default();
    every_signature_at!(33, lattice_and_seeded, &mut tally);
    every_signature_at!(34, lattice_and_seeded, &mut tally);
    every_signature_at!(35, lattice_and_seeded, &mut tally);
    every_signature_at!(36, lattice_and_seeded, &mut tally);
    every_signature_at!(37, lattice_and_seeded, &mut tally);
    every_signature_at!(38, lattice_and_seeded, &mut tally);
    every_signature_at!(39, lattice_and_seeded, &mut tally);
    every_signature_at!(40, lattice_and_seeded, &mut tally);
    every_signature_at!(41, lattice_and_seeded, &mut tally);
    every_signature_at!(42, lattice_and_seeded, &mut tally);
    every_signature_at!(43, lattice_and_seeded, &mut tally);
    every_signature_at!(44, lattice_and_seeded, &mut tally);
    every_signature_at!(45, lattice_and_seeded, &mut tally);
    every_signature_at!(46, lattice_and_seeded, &mut tally);
    every_signature_at!(47, lattice_and_seeded, &mut tally);
    every_signature_at!(48, lattice_and_seeded, &mut tally);
    every_signature_at!(49, lattice_and_seeded, &mut tally);
    every_signature_at!(50, lattice_and_seeded, &mut tally);
    every_signature_at!(51, lattice_and_seeded, &mut tally);
    every_signature_at!(52, lattice_and_seeded, &mut tally);
    every_signature_at!(53, lattice_and_seeded, &mut tally);
    every_signature_at!(54, lattice_and_seeded, &mut tally);
    every_signature_at!(55, lattice_and_seeded, &mut tally);
    every_signature_at!(56, lattice_and_seeded, &mut tally);
    every_signature_at!(57, lattice_and_seeded, &mut tally);
    every_signature_at!(58, lattice_and_seeded, &mut tally);
    every_signature_at!(59, lattice_and_seeded, &mut tally);
    every_signature_at!(60, lattice_and_seeded, &mut tally);
    every_signature_at!(61, lattice_and_seeded, &mut tally);
    every_signature_at!(62, lattice_and_seeded, &mut tally);
    every_signature_at!(63, lattice_and_seeded, &mut tally);
    every_signature_at!(64, lattice_and_seeded, &mut tally);
    tally.check("widths 33 to 64");
}
