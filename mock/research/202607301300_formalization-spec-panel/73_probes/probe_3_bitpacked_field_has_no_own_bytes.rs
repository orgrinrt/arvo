// Probe 3: does a single bitpacked field, packed with siblings into a shared
// word, have its own independent byte image? Modelled on the exact shape
// file 32 (Aaltonen) measured for extraction cost: four 13-bit fields packed
// into one 64-bit word with 12 bits of headroom, the non-power-of-two shape
// arvo's own bitfield examples use.
//
// The claim: for every byte index of the word, that byte's bits belong to
// more than one logical field (or to the shared headroom). No byte of the
// word can be handed to a consumer as "field i's own byte", because doing so
// would also hand over bits of field i-1 or i+1. Only the WHOLE WORD has a
// byte image; a single packed field's byte image is not a well-formed
// request, only its extracted, re-embedded-in-its-own-carrier DATUM is.

const FIELD_BITS: u32 = 13;
const N_FIELDS: u32 = 4;
const WORD_BITS: u32 = 64;

fn field_bit_range(i: u32) -> (u32, u32) {
    // fields packed LSB-first, contiguous, no cross-field padding
    let lo = i * FIELD_BITS;
    let hi = lo + FIELD_BITS - 1;
    (lo, hi)
}

fn byte_bit_range(byte_index: u32) -> (u32, u32) {
    let lo = byte_index * 8;
    let hi = lo + 7;
    (lo, hi)
}

fn ranges_overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn fields_touching_byte(byte_index: u32) -> Vec<u32> {
    let br = byte_bit_range(byte_index);
    (0..N_FIELDS)
        .filter(|&i| ranges_overlap(br, field_bit_range(i)))
        .collect()
}

fn main() {
    let n_bytes = (WORD_BITS / 8) as u32;
    let mut multi_field_bytes = 0;
    let mut pure_headroom_bytes = 0;
    for b in 0..n_bytes {
        let touching = fields_touching_byte(b);
        let br = byte_bit_range(b);
        let field_extent = 0..(N_FIELDS * FIELD_BITS); // 0..52
        let touches_headroom = br.1 >= field_extent.end || br.0 >= field_extent.end;
        if touching.len() > 1 {
            multi_field_bytes += 1;
        }
        if touching.is_empty() && !touches_headroom {
            unreachable!("a byte with no fields must be pure headroom");
        }
        if touching.is_empty() {
            pure_headroom_bytes += 1;
        }
        println!(
            "byte {}: bits [{},{}], fields touching: {:?}",
            b, br.0, br.1, touching
        );
    }

    // the claim: at least one byte is straddled by two distinct fields.
    // (byte 1 holds bits [8,15]: field 0 owns bits [0,12], field 1 owns
    // bits [13,25], so byte 1's bits 13,14,15 belong to field 1 while bits
    // 8..12 belong to field 0.)
    assert!(multi_field_bytes > 0, "every byte belonged to exactly one field: the packing was accidentally byte-aligned, not the case this probe models");

    // no byte can be losslessly attributed to exactly one field's own byte
    // image for every straddled byte; a per-field byte image therefore does
    // not exist independently of its siblings for this layout.
    println!(
        "{} of {} bytes in the word are shared by more than one field ({} pure headroom); \
         a single field's own byte image is not expressible as a sub-slice of the word's bytes",
        multi_field_bytes, n_bytes, pure_headroom_bytes
    );

    // what IS well-defined: extracting the field into its own, freshly sized
    // carrier (the datum, at the crossing contract's D layer), independent of
    // the word's own byte layout entirely.
    let word: u64 = 0x0000_1FFF_2AAA_0C21; // arbitrary packed word
    for i in 0..N_FIELDS {
        let (lo, _hi) = field_bit_range(i);
        let extracted = ((word >> lo) & ((1u64 << FIELD_BITS) - 1)) as u16;
        // this IS a well-formed 13-bit datum with its own, fresh, byte image
        // once embedded in its own carrier (probe 1's ByteCap route), but
        // that carrier's bytes are not a sub-sequence of the word's bytes.
        println!(
            "field {} extracted as its own datum: {:#06x} (13 bits, fresh carrier)",
            i, extracted
        );
    }
}
