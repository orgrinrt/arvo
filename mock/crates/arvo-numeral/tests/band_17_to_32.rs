//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Widths 17 to 32 through the value type, held in the 32-bit carrier: the
//! edge lattice and seeded pairs of every signature, both families, both
//! overflow policies and every mode, against the oracle.

mod common;

use common::{Tally, on_the_lattice, seeded};

fn lattice_and_seeded<S: arvo_numeral::Sole>(tally: &mut Tally) {
    on_the_lattice::<S>(tally);
    seeded::<S>(tally);
}

#[test]
fn every_signature_from_17_to_32_bits_adds_and_orders_as_the_oracle_does() {
    let mut tally = Tally::default();
    every_signature_at!(17, lattice_and_seeded, &mut tally);
    every_signature_at!(18, lattice_and_seeded, &mut tally);
    every_signature_at!(19, lattice_and_seeded, &mut tally);
    every_signature_at!(20, lattice_and_seeded, &mut tally);
    every_signature_at!(21, lattice_and_seeded, &mut tally);
    every_signature_at!(22, lattice_and_seeded, &mut tally);
    every_signature_at!(23, lattice_and_seeded, &mut tally);
    every_signature_at!(24, lattice_and_seeded, &mut tally);
    every_signature_at!(25, lattice_and_seeded, &mut tally);
    every_signature_at!(26, lattice_and_seeded, &mut tally);
    every_signature_at!(27, lattice_and_seeded, &mut tally);
    every_signature_at!(28, lattice_and_seeded, &mut tally);
    every_signature_at!(29, lattice_and_seeded, &mut tally);
    every_signature_at!(30, lattice_and_seeded, &mut tally);
    every_signature_at!(31, lattice_and_seeded, &mut tally);
    every_signature_at!(32, lattice_and_seeded, &mut tally);
    tally.check("widths 17 to 32");
}
