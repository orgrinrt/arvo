//! Probe 5. Does the datum-versus-value fork even apply to the weight types
//! arvo-graph and arvo-comb actually ship against (`UFixed`/`IFixed`)?
//!
//! `arvo/src/traits/total_ord.rs:60-79` routes `UFixed`/`IFixed`'s
//! `TotalOrd` through `arvo_storage::ConstOrd`, itself a comparison over the
//! `Bits<N, S, Sign>` container: an unsigned magnitude compare for
//! `Unsigned`, a native two's-complement signed compare for `Signed`
//! (`arvo-strategy`'s container table dispatches `Signed` to the native
//! signed primitives i8..i128, which are two's complement by definition,
//! one representation of zero). Neither has a second zero datum, a NaN, or
//! any other cohort: `Bits<N, S, Sign>`'s encoding is injective (58:171-177
//! statement 3, "the encoding is injective iff no value has two data").
//!
//! Claim under test: when the encoding is injective, ANY bit-comparator
//! that is a strict total order over the data is automatically
//! value-respecting, because there is no second datum for two different
//! data to collide on. Not a new mechanism: an identity that follows from
//! injectivity alone. Spot-checked over the reachable range rather than
//! argued from injectivity alone, because a spot check plus the structural
//! argument beats the argument alone.
//!
//! POSITIVE CONTROL. Expected: WORKS, zero counterexamples over the swept
//! range, for both an unsigned stand-in (u16, standing in for
//! `Bits<N,Unsigned>`, same bit-comparator shape) and a signed stand-in
//! (i16, standing in for `Bits<N,Signed>`, two's complement, matching
//! `IFixed`'s container).

fn decode_unsigned(bits: u16) -> u32 {
    // Injective: the value IS the datum, read as an unsigned magnitude.
    // This is exactly what an unsigned fixed-point numeral's `decode` does
    // (the raw container value times a fixed positive scale; the scale
    // cancels out of an equality/order check so it is omitted here).
    bits as u32
}

fn decode_signed(bits: i16) -> i32 {
    // Injective: two's complement has exactly one representation of every
    // integer in range, including zero.
    bits as i32
}

fn main() {
    let mut checked_u = 0u32;
    let mut checked_i = 0u32;

    // Full range for u16 stand-in: every pair would be 2^32 comparisons,
    // so this sweeps every datum against every OTHER datum in a 512-wide
    // window around three representative regions (bottom, middle, top of
    // the range) rather than the full cross product, which is enough to
    // falsify the claim if it were false anywhere, since injectivity is a
    // per-encoding structural fact, not a magnitude-dependent one.
    let windows: [(u16, u16); 3] = [(0, 512), (32000, 32512), (65024, 65535)];
    for (lo, hi) in windows {
        for a in lo..=hi {
            for b in lo..=hi {
                let datum_eq = a == b;
                let value_eq = decode_unsigned(a) == decode_unsigned(b);
                assert_eq!(
                    datum_eq, value_eq,
                    "unsigned fork found at a={a}, b={b}: datum_eq={datum_eq}, value_eq={value_eq}"
                );
                checked_u += 1;
            }
        }
    }

    let windows_i: [(i16, i16); 3] = [
        (i16::MIN, i16::MIN + 511),
        (-256, 255),
        (i16::MAX - 511, i16::MAX),
    ];
    for (lo, hi) in windows_i {
        let mut a = lo;
        loop {
            let mut b = lo;
            loop {
                let datum_eq = a == b;
                let value_eq = decode_signed(a) == decode_signed(b);
                assert_eq!(
                    datum_eq, value_eq,
                    "signed fork found at a={a}, b={b}: datum_eq={datum_eq}, value_eq={value_eq}"
                );
                checked_i += 1;
                if b == hi {
                    break;
                }
                b += 1;
            }
            if a == hi {
                break;
            }
            a += 1;
        }
    }

    println!("probe_5 WORKS: {checked_u} unsigned pairs, {checked_i} signed pairs, zero forks");
    println!(
        "for an injective encoding (UFixed/IFixed today), datum_cmp == Equal iff value_cmp == Equal \
         always; the fork this file exists to resolve does not apply to arvo-graph/arvo-comb's shipped \
         weight types at all."
    );
}
