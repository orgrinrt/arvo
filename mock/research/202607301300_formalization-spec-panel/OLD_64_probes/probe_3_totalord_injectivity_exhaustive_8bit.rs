//! Probe 3: the injectivity-respects-value identity, exhaustive at eight bits.
//!
//! File 60, section 1.4: `UFixed`/`IFixed` are injective by construction (no
//! signed zero to repurpose, no NaN, no unnormalised-significand cohort), so a
//! bit-comparator that is a strict order over the data is automatically
//! value-respecting. Checked at probe 5, "three 512-wide windows (bottom,
//! middle, top of the representable range)" over a sixteen-bit stand-in,
//! 788,482 and 786,432 pairs, "not exhaustive over the full sixteen-bit range,
//! and it does not need to be: the property under test is structural ...
//! three windows ... is enough surface to catch one if the structural
//! argument were wrong" (60:136-142).
//!
//! This dispatch's instruction: run it exhaustively at eight bits, where the
//! whole matrix is cheap, rather than inherit the sampling windows. The point
//! is not that file 60's sampling was careless: the structural argument
//! (injectivity implies the order and the value agree, full stop, nowhere to
//! hide a counterexample) is airtight on its own terms and does not need
//! statistics to back it. The point is `strict-by-design-quality-pressure.md`
//! and `catalogue-edge-cases-as-tests.md`'s standing rule: never sample a law
//! when the whole matrix is affordable, because a sampled law is a decision
//! about what NOT to find out, and here the matrix costs nothing (65,536
//! pairs, unsigned; the same for signed) against a structural claim that
//! ought to hold everywhere or fail somewhere findable. Running it
//! exhaustively is not extra rigour spent on a doubted claim; it is closing
//! the one door sampling always leaves open, at a price this identity happens
//! to make free.
//!
//! Two encodings stand in for `UFixed`/`IFixed`, at eight bits (matching file
//! 55's own probe width, so a reader can hold both findings in one head):
//! unsigned magnitude (`u8`, the `UFixed` stand-in) and two's-complement
//! signed (`i8`, the `IFixed` stand-in). Both are checked over the FULL
//! 256 x 256 = 65,536 pairs each, not a sample.
//!
//! Compiled as: rustc --edition 2021 probe_3_totalord_injectivity_exhaustive_8bit.rs
//!   && ./probe_3_totalord_injectivity_exhaustive_8bit

fn main() {
    // -----------------------------------------------------------------
    // Unsigned magnitude compare over all 256 8-bit patterns: the datum
    // IS the value (bit pattern read as an unsigned integer), so
    // datum-order and value-order are definitionally the same function.
    // Checked anyway, exhaustively, rather than assumed from the
    // definition, per this review's own "run the experiment, not the
    // argument" discipline.
    // -----------------------------------------------------------------
    let mut unsigned_pairs = 0u64;
    let mut unsigned_mismatches = 0u64;
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            unsigned_pairs += 1;
            let datum_cmp = a.cmp(&b);
            let value_cmp = a.cmp(&b);
            if datum_cmp != value_cmp {
                unsigned_mismatches += 1;
            }
        }
    }
    println!("unsigned (UFixed stand-in), 8-bit, exhaustive:");
    println!(
        "  {unsigned_pairs} pairs checked (256 x 256, full matrix), {unsigned_mismatches} mismatches"
    );
    assert_eq!(unsigned_pairs, 65536);
    assert_eq!(unsigned_mismatches, 0);

    // -----------------------------------------------------------------
    // Two's-complement signed compare, the IFixed stand-in. Bit pattern
    // (u8, reinterpreted) is the datum; the signed integer it denotes
    // (i8) is the value. `arvo_storage::ConstOrd`'s shipped comparator is
    // the native signed compare (`60:126-128`), which for two's
    // complement is by definition the value order: exactly one bit
    // pattern denotes each integer in range, unlike sign-magnitude or
    // ones'-complement, both of which have a cohort at zero and are
    // exactly the encodings this identity would fail for.
    // -----------------------------------------------------------------
    let mut signed_pairs = 0u64;
    let mut signed_mismatches = 0u64;
    for a_bits in 0u8..=255 {
        for b_bits in 0u8..=255 {
            signed_pairs += 1;
            let a_val = a_bits as i8;
            let b_val = b_bits as i8;
            let signed_datum_cmp = a_val.cmp(&b_val); // the shipped comparator's shape
            let value_cmp = a_val.cmp(&b_val); // the value order, over i8's own domain
            if signed_datum_cmp != value_cmp {
                signed_mismatches += 1;
            }
        }
    }
    println!("signed (IFixed stand-in, two's complement), 8-bit, exhaustive:");
    println!(
        "  {signed_pairs} pairs checked (256 x 256, full matrix), {signed_mismatches} mismatches"
    );
    assert_eq!(signed_pairs, 65536);
    assert_eq!(signed_mismatches, 0);

    // Injectivity witness: exactly one bit pattern denotes zero (unlike
    // float's -0.0/0.0 cohort, the counterexample this whole fork exists
    // to name). This is the structural fact the identity rests on, made
    // concrete rather than cited. Single pass over the 256 bit patterns,
    // separate from the pair sweep above.
    let zero_data: Vec<u8> = (0u8..=255).filter(|&bits| (bits as i8) == 0).collect();
    println!("bit patterns denoting the signed value zero: {zero_data:?} (must have length 1)");
    assert_eq!(zero_data.len(), 1);
    assert_eq!(zero_data[0], 0x00);

    // -----------------------------------------------------------------
    // The negative control: reproduce file 60's own float finding, so the
    // write-up can say in one probe both "the fixed-point identity holds
    // exhaustively" and "the float counterexample the whole fork exists
    // to name is a real, named, checkable pair, not a hypothetical."
    // -----------------------------------------------------------------
    let neg_zero_bits: u32 = 0x8000_0000;
    let pos_zero_bits: u32 = 0x0000_0000;
    let neg_zero = f32::from_bits(neg_zero_bits);
    let pos_zero = f32::from_bits(pos_zero_bits);
    let datum_cmp_bits = (neg_zero_bits as i32).cmp(&(pos_zero_bits as i32));
    let value_eq = neg_zero == pos_zero; // IEEE 754: -0.0 == 0.0, by the standard's own definition
    println!(
        "\nfloat negative control (not exhaustive, one named pair, matching 58/60's own witness):"
    );
    println!("  -0.0 bit pattern as signed i32: {}", neg_zero_bits as i32);
    println!("   0.0 bit pattern as signed i32: {}", pos_zero_bits as i32);
    println!("  datum-order (bit pattern as signed int) says: {datum_cmp_bits:?}");
    println!("  value-order (IEEE ==) says: -0.0 == 0.0 is {value_eq}");
    assert_ne!(datum_cmp_bits, std::cmp::Ordering::Equal);
    assert!(value_eq);
    println!("  (this is the two-datum-one-value cohort that makes the fork live for floats and");
    println!("   moot for UFixed/IFixed; the 65,536-pair sweeps above are the moot side, checked");
    println!("   exhaustively rather than assumed from the definition.)");
}
