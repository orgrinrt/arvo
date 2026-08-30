//! Probe 3: the encoding is a separate axis from the denoted value set.
//!
//! Hypothesis: a format's identity is value-level (which numbers it denotes).
//! The encoding, which bit pattern carries which value, is a second choice
//! made after the value set, and it is not innocent: for a FIXED pattern
//! budget, different encodings realise different value sets and different
//! redundancy. So the derivation the acceptance criterion names (bits in,
//! container AND numeral representations out) has to pass through the
//! encoding as its own step, and the plural "representations" is load
//! bearing.
//!
//! Exhaustive at 4 bits, three classical signed encodings of "a signed
//! integer in 4 bits":
//!   - two's complement:   den(b) = b - 16 * [b >= 8]
//!   - offset binary K=8:  den(b) = b - 8
//!   - sign-magnitude:     den(b) = (-1)^(b >> 3) * (b & 7)
//!
//! Checks:
//!   1. two's complement and offset binary denote the SAME value set [-8, 7]
//!      through DIFFERENT pattern maps (so encoding does not touch the value
//!      set when both encodings are bijective onto it).
//!   2. sign-magnitude denotes a DIFFERENT set ([-7, 7], no -8) and is
//!      REDUNDANT (two patterns for 0), so with the same 16 patterns the
//!      value count drops to 15: encoding choice changes what a bit budget
//!      buys.
//!   3. the pattern-level order (unsigned compare of the raw bits) agrees
//!      with the value order for offset binary, and disagrees for two's
//!      complement and sign-magnitude: an operational property (comparability
//!      by raw compare) lives on the encoding axis, not the value axis.
//!
//! Instrument validation: a mutant offset (K=7) must be detected as denoting
//! a different set from two's complement.

use std::collections::BTreeSet;

fn den_twos(b: u8) -> i64 {
    let b = b as i64;
    if b >= 8 {
        b - 16
    } else {
        b
    }
}

fn den_offset(b: u8) -> i64 {
    b as i64 - 8
}

fn den_offset_mutant(b: u8) -> i64 {
    b as i64 - 7
}

fn den_signmag(b: u8) -> i64 {
    let mag = (b & 7) as i64;
    if b & 8 != 0 {
        -mag
    } else {
        mag
    }
}

fn image(den: fn(u8) -> i64) -> BTreeSet<i64> {
    (0u8..16).map(den).collect()
}

fn main() {
    let mut ok = true;

    let twos = image(den_twos);
    let offs = image(den_offset);
    let smag = image(den_signmag);

    // 1. same value set, different maps
    let same_set = twos == offs;
    let maps_differ = (0u8..16).any(|b| den_twos(b) != den_offset(b));
    println!("twos == offset as value sets: {}", same_set);
    println!("twos and offset maps differ pointwise: {}", maps_differ);
    ok &= same_set && maps_differ;
    ok &= twos.len() == 16; // both bijective onto [-8, 7]

    // 2. sign-magnitude: different set, redundant
    let smag_differs = smag != twos;
    let smag_count = smag.len();
    let zero_patterns = (0u8..16).filter(|&b| den_signmag(b) == 0).count();
    println!("sign-magnitude set differs from twos: {}", smag_differs);
    println!(
        "sign-magnitude distinct values: {} (of 16 patterns)",
        smag_count
    );
    println!("sign-magnitude patterns denoting zero: {}", zero_patterns);
    ok &= smag_differs && smag_count == 15 && zero_patterns == 2;

    // 3. raw-compare order vs value order, per encoding
    let order_agrees = |den: fn(u8) -> i64| -> bool {
        let mut agrees = true;
        for a in 0u8..16 {
            for b in 0u8..16 {
                if (a < b) != (den(a) < den(b)) && den(a) != den(b) {
                    agrees = false;
                }
            }
        }
        agrees
    };
    let off_ord = order_agrees(den_offset);
    let twos_ord = order_agrees(den_twos);
    let smag_ord = order_agrees(den_signmag);
    println!(
        "raw compare matches value order: offset {}, twos {}, signmag {}",
        off_ord, twos_ord, smag_ord
    );
    ok &= off_ord && !twos_ord && !smag_ord;

    // instrument validation: mutant offset must be detected
    let mutant = image(den_offset_mutant);
    let mutant_detected = mutant != twos;
    println!("mutant offset (K=7) detected: {}", mutant_detected);
    ok &= mutant_detected;

    println!("{}", if ok { "P3 WORKS" } else { "P3 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
