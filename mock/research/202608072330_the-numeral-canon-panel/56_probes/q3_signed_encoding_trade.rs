//! Probe q3: for a signed value set, raw-order agreement and raw-adder
//! correctness are mutually exclusive over bijective encodings, and the
//! trade is forced.
//!
//! Context. `55` phase two refined `08`'s pool-scoped raw-order finding: the
//! excess-K encodings are monotone, so memcmp-sortability is purchasable by
//! encoding for signed sets. This probe completes that refinement into a
//! trade by adding the other pattern-level property that matters, whether
//! the plain binary adder on raw patterns implements the value operation.
//!
//!   1. raw-order agreement: unsigned compare of patterns == value order.
//!      Holds for offset binary (excess-8), fails for two's complement and
//!      sign-magnitude. (Reproduces `55_probes/p3` check 3 independently.)
//!   2. raw-adder correctness: (e(a) + e(b)) mod 16 == e(wrap(a + b)),
//!      exhaustively over Q^2. Holds for two's complement (256 of 256),
//!      fails for offset binary at every single pair (0 of 256), and the
//!      failure is a CONSTANT: the raw sum differs from the correct pattern
//!      by exactly K = 8 (mod 16), every time (measured). So offset-binary
//!      hardware pays one bias correction per add, which is why exponent
//!      fields (compare-heavy) are biased and integer ALUs (add-heavy) are
//!      two's complement.
//!   3. the exclusivity argument, made computational: a monotone bijection
//!      between two finite totally ordered sets is unique, namely the
//!      sorted correspondence. The probe constructs it and confirms it IS
//!      offset binary. Since the unique monotone bijection fails the adder
//!      property by the constant K, NO bijective encoding of a signed
//!      value set has both properties. For the unsigned set the identity
//!      encoding has both, so the exclusivity is a fact about signedness
//!      (more precisely, about K != 0 in the unique monotone encoding).
//!
//! Instrument validation: a mutant offset (K = 7) must break the
//! sorted-correspondence equality, and a bit-scrambled encoding must fail
//! both properties.
//!
//! Exhaustive at 4 bits. Q signed = [-8, 7], Q unsigned = [0, 15].

fn wrap16(x: i64) -> i64 {
    ((x + 8).rem_euclid(16)) - 8
}

fn enc_twos(v: i64) -> u8 {
    (v.rem_euclid(16)) as u8
}

fn enc_offset(v: i64) -> u8 {
    (v + 8) as u8
}

fn enc_offset_mutant(v: i64) -> u8 {
    ((v + 7).rem_euclid(16)) as u8
}

// scrambled: gray-ish permutation, monotone nowhere useful and adder-hostile
fn enc_scrambled(v: i64) -> u8 {
    let b = (v + 8) as u8;
    b ^ (b >> 1)
}

fn order_agrees(enc: fn(i64) -> u8) -> bool {
    let mut agrees = true;
    for a in -8i64..=7 {
        for b in -8i64..=7 {
            if (a < b) != (enc(a) < enc(b)) && a != b {
                agrees = false;
            }
        }
    }
    agrees
}

// count pairs where the raw 4-bit adder computes the encoding of the wrapped sum
fn adder_correct_pairs(enc: fn(i64) -> u8) -> u32 {
    let mut n = 0;
    for a in -8i64..=7 {
        for b in -8i64..=7 {
            let raw = (enc(a).wrapping_add(enc(b))) & 0xF;
            if raw == enc(wrap16(a + b)) {
                n += 1;
            }
        }
    }
    n
}

// is the raw-adder defect a constant offset? returns Some(k) if
// raw_sum - correct_pattern == k (mod 16) for every pair, else None
fn adder_defect_constant(enc: fn(i64) -> u8) -> Option<u8> {
    let mut k: Option<u8> = None;
    for a in -8i64..=7 {
        for b in -8i64..=7 {
            let raw = (enc(a).wrapping_add(enc(b))) & 0xF;
            let correct = enc(wrap16(a + b));
            let d = raw.wrapping_sub(correct) & 0xF;
            match k {
                None => k = Some(d),
                Some(prev) if prev != d => return None,
                _ => {}
            }
        }
    }
    k
}

fn main() {
    let mut ok = true;

    // 1. raw-order agreement
    let ord_twos = order_agrees(enc_twos);
    let ord_off = order_agrees(enc_offset);
    let ord_scr = order_agrees(enc_scrambled);
    println!(
        "raw order agrees: twos {}  offset {}  scrambled {}",
        ord_twos, ord_off, ord_scr
    );
    ok &= !ord_twos && ord_off && !ord_scr;

    // 2. raw-adder correctness
    let add_twos = adder_correct_pairs(enc_twos);
    let add_off = adder_correct_pairs(enc_offset);
    let add_scr = adder_correct_pairs(enc_scrambled);
    println!(
        "raw adder correct pairs of 256: twos {}  offset {}  scrambled {}",
        add_twos, add_off, add_scr
    );
    ok &= add_twos == 256 && add_off == 0;
    ok &= add_scr < 256; // scrambled must not accidentally be an adder encoding

    // the offset defect is the constant K = 8
    let defect = adder_defect_constant(enc_offset);
    println!(
        "offset-binary adder defect is constant: {:?} (expect Some(8))",
        defect
    );
    ok &= defect == Some(8);

    // 3. uniqueness of the monotone bijection: sorted correspondence == offset
    let values: Vec<i64> = (-8..=7).collect(); // already sorted
    let patterns: Vec<u8> = (0..=15).collect(); // already sorted
    let unique_monotone: Vec<(i64, u8)> = values
        .iter()
        .cloned()
        .zip(patterns.iter().cloned())
        .collect();
    let is_offset = unique_monotone.iter().all(|&(v, p)| enc_offset(v) == p);
    println!(
        "the unique monotone bijection is offset binary: {}",
        is_offset
    );
    ok &= is_offset;

    // mutant: K = 7 must break the correspondence
    let mutant_breaks = unique_monotone
        .iter()
        .any(|&(v, p)| enc_offset_mutant(v) != p);
    println!(
        "mutant offset (K=7) breaks the sorted correspondence: {}",
        mutant_breaks
    );
    ok &= mutant_breaks;

    // 4. the unsigned identity encoding has both properties
    let mut uord = true;
    let mut uadd = 0u32;
    for a in 0i64..=15 {
        for b in 0i64..=15 {
            if (a < b) != ((a as u8) < (b as u8)) && a != b {
                uord = false;
            }
            let raw = ((a as u8).wrapping_add(b as u8)) & 0xF;
            if raw == ((a + b).rem_euclid(16)) as u8 {
                uadd += 1;
            }
        }
    }
    println!(
        "unsigned identity encoding: raw order {}  raw adder correct pairs {} of 256",
        uord, uadd
    );
    ok &= uord && uadd == 256;

    println!("{}", if ok { "Q3 WORKS" } else { "Q3 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
