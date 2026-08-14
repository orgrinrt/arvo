//! P1. Are the value set and the bit realisation one thing or two?
//!
//! The panel's working assumption bundles them under "format". This probe
//! asks whether they can vary independently, which is the test for whether
//! they are one component or two.
//!
//! Two directions, and both have to hold for independence:
//!
//!   A. Same value set, two different realisations. If the answers agree and
//!      the footprints differ, the realisation moved while the value set
//!      stood still.
//!   B. Same realisation, two different value sets. If the same bits denote
//!      two different rationals and arithmetic on them differs, the value
//!      set moved while the realisation stood still.
//!
//! Nothing here is a design proposal. The types are scaffolding chosen to
//! reach the check. No feature gates, no alloc; `std` is used only by the
//! test harness.
//!
//! ## A defect in the first version of this file, kept because it is the
//! ## finding rather than an embarrassment
//!
//! v1 (`p1_v1_lane_aligned_pack_was_not_a_pack.rs.rejected`) modelled the
//! packed realisation as four 13-bit lanes inside one u64. That is 52 bits
//! used of 64, so 16 bits per element, which is exactly what the u16
//! realisation costs. The footprint assertion failed, correctly, and what it
//! caught was a setup that helps: a "pack" that does not pack cannot
//! demonstrate anything about packing. A dense stream crossing word
//! boundaries is used below instead, at exactly 13 bits per element, which
//! is also why its decode is not a single shift.
//!
//! Build: rustc --test -O p1_value_set_and_realisation_are_independent.rs

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Direction A. One value set, two realisations.
//
// The value set: the integers 0..=8191, i.e. 13 bits of range, no fraction.
// Realisation A1 rests each element in its own u16 container.
// Realisation A2 rests elements in a dense bit stream at 13 bits each,
// crossing word boundaries, which is what makes it a different realisation
// rather than a renaming of the first.
// ---------------------------------------------------------------------------

const V13_BITS: u32 = 13;
const V13_MAX: u32 = (1 << V13_BITS) - 1;

/// Realisation A1: one element, one container, decode is the identity.
#[derive(Copy, Clone, Debug, PartialEq)]
struct WideRest(u16);

impl WideRest {
    const BITS_PER_ELEM: u32 = 16;

    const fn encode(v: u32) -> Self {
        WideRest(v as u16)
    }
    const fn decode(self) -> u32 {
        self.0 as u32
    }
}

/// Realisation A2: a dense stream. Element i occupies bits
/// `13*i .. 13*i+13` of a flat bit array, with no alignment to any word.
/// Fixed capacity, const size, no growth.
#[derive(Copy, Clone, Debug)]
struct PackedRest<const N: usize> {
    // N elements at 13 bits, rounded up to whole u64 words, plus one spare
    // word so a straddling read at the last element never runs off the end.
    words: [u64; 64],
    len: usize,
}

impl<const N: usize> PackedRest<N> {
    const BITS_PER_ELEM: u32 = V13_BITS;

    const fn new() -> Self {
        PackedRest { words: [0u64; 64], len: 0 }
    }

    fn set(&mut self, i: usize, v: u32) {
        let bit = i * V13_BITS as usize;
        let w = bit / 64;
        let off = (bit % 64) as u32;
        // Clear then write the low part.
        let mask_lo = (V13_MAX as u64) << off;
        self.words[w] = (self.words[w] & !mask_lo) | (((v as u64) << off) & mask_lo);
        // The element straddles when off + 13 > 64.
        if off + V13_BITS > 64 {
            let taken = 64 - off;
            let rest = V13_BITS - taken;
            let mask_hi = (1u64 << rest) - 1;
            self.words[w + 1] = (self.words[w + 1] & !mask_hi) | ((v as u64) >> taken);
        }
        if i + 1 > self.len {
            self.len = i + 1;
        }
    }

    fn get(&self, i: usize) -> u32 {
        let bit = i * V13_BITS as usize;
        let w = bit / 64;
        let off = (bit % 64) as u32;
        let mut v = (self.words[w] >> off) & (V13_MAX as u64);
        if off + V13_BITS > 64 {
            let taken = 64 - off;
            let rest = V13_BITS - taken;
            let mask_hi = (1u64 << rest) - 1;
            v |= (self.words[w + 1] & mask_hi) << taken;
        }
        v as u32
    }

    /// True when element i needs two word reads rather than one.
    const fn straddles(i: usize) -> bool {
        let off = (i * V13_BITS as usize) % 64;
        off as u32 + V13_BITS > 64
    }
}

/// The completion policy, held FIXED across A1 and A2 so that only the
/// realisation varies: saturating at the top of the 13-bit value set.
const fn complete_sat(exact: u32) -> u32 {
    if exact > V13_MAX { V13_MAX } else { exact }
}

// ---------------------------------------------------------------------------
// Direction B. One realisation, two value sets.
//
// The realisation: exactly eight bits, in a u8, no spare, no tag.
// Value set B1: the integers 0..=255.            (I = 8, F = 0)
// Value set B2: the multiples of 1/16 in [0,16). (I = 4, F = 4)
//
// The bits are identical. What they denote is not, and neither is the answer
// to a multiplication, which is the observable that separates them.
// ---------------------------------------------------------------------------

/// Denotation under B1: the bit pattern k denotes the rational k/1.
const fn denote_i8f0(bits: u8) -> (u32, u32) {
    (bits as u32, 1)
}

/// Denotation under B2: the bit pattern k denotes the rational k/16.
const fn denote_i4f4(bits: u8) -> (u32, u32) {
    (bits as u32, 16)
}

/// Multiplication that stays in the same eight bits, truncating toward zero,
/// wrapping on range failure. Held FIXED across B1 and B2; only the fraction
/// position, which is a fact about the value set rather than about the bits,
/// differs.
const fn mul_in_place(a: u8, b: u8, frac_bits: u32) -> u8 {
    let prod = (a as u32) * (b as u32);
    ((prod >> frac_bits) & 0xff) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A. Every element of the value set survives both realisations, and the
    /// completed sum agrees, element for element, across the entire value
    /// set against a representative set of addends.
    #[test]
    fn same_value_set_two_realisations_agree_on_every_answer() {
        let mut checked = 0u64;
        let addends = [0u32, 1, 7, 64, 1000, 4095, 8191];
        let mut packed: PackedRest<8> = PackedRest::new();
        for a in 0..=V13_MAX {
            for (slot, &b) in addends.iter().enumerate() {
                let want = complete_sat(a + b);

                // A1: element in its own container.
                let got_wide = WideRest::encode(want).decode();

                // A2: element in a dense stream. Neighbours are deliberately
                // written with a saturated pattern first, so a bleed across
                // the 13-bit boundary is caught rather than masked by zeros.
                for n in 0..8 {
                    packed.set(n, V13_MAX);
                }
                packed.set(slot, want);
                let got_packed = packed.get(slot);

                assert_eq!(got_wide, want, "A1 lost a value at a={a} b={b}");
                assert_eq!(got_packed, want, "A2 lost a value at a={a} b={b}");
                checked += 1;
            }
        }
        // Guard against a setup that silently skips: the loop must have run
        // over the entire value set.
        assert_eq!(checked, (V13_MAX as u64 + 1) * 7);
    }

    /// A, second part. The two realisations differ in footprint. If they did
    /// not, there would be nothing to choose between them and the axis would
    /// be vacuous. This is the assertion that caught v1's fake pack.
    #[test]
    fn the_two_realisations_have_different_footprints() {
        assert_eq!(WideRest::BITS_PER_ELEM, 16);
        assert_eq!(PackedRest::<8>::BITS_PER_ELEM, 13);
        assert!(PackedRest::<8>::BITS_PER_ELEM < WideRest::BITS_PER_ELEM);

        // And the declared constants are tied to what the code actually
        // does, rather than being free-floating numbers. A wrong
        // BITS_PER_ELEM would fail here.
        let mut p: PackedRest<8> = PackedRest::new();
        for i in 0..8 {
            p.set(i, V13_MAX);
        }
        let set_bits: u32 = p.words.iter().map(|w| w.count_ones()).sum();
        assert_eq!(
            set_bits,
            8 * PackedRest::<8>::BITS_PER_ELEM,
            "eight saturated elements must occupy exactly eight times the \
             declared per-element width, with no padding"
        );
    }

    /// A, third part, and the one that carries the claim. The dense
    /// realisation is not independently addressable: some elements need two
    /// word reads. That is a property of the realisation alone, invisible in
    /// the value set, and it is where the decode cost the bitpack benches
    /// measure comes from.
    #[test]
    fn the_dense_realisation_is_not_independently_addressable() {
        // At 13 bits, element 5 starts at bit 65 and element 4 at bit 52,
        // so element 4 straddles the first word boundary.
        assert!(PackedRest::<8>::straddles(4), "element 4 must straddle");
        assert!(!PackedRest::<8>::straddles(0), "element 0 must not");

        // Straddling is real, not a naming: writing a straddling element
        // touches two words.
        let mut p: PackedRest<8> = PackedRest::new();
        p.set(4, V13_MAX);
        assert_ne!(p.words[0], 0, "low word must carry part of element 4");
        assert_ne!(p.words[1], 0, "high word must carry the rest");

        // And a full round trip over every element still holds, so the
        // straddle is handled rather than merely present.
        let mut q: PackedRest<8> = PackedRest::new();
        for i in 0..8 {
            q.set(i, (i as u32 * 977) & V13_MAX);
        }
        for i in 0..8 {
            assert_eq!(q.get(i), (i as u32 * 977) & V13_MAX, "round trip at {i}");
        }
    }

    /// B. The same eight bits denote different rationals under two value
    /// sets, and the multiplication answers differ. Exhaustive over all
    /// 65536 pairs; the count of disagreements is reported rather than
    /// asserted at a magnitude, because this probe establishes the existence
    /// of the disagreement, not its size.
    #[test]
    fn same_realisation_two_value_sets_disagree() {
        let mut disagreements = 0u32;
        let mut checked = 0u32;
        for a in 0u16..=255 {
            for b in 0u16..=255 {
                let (a, b) = (a as u8, b as u8);
                let r1 = mul_in_place(a, b, 0); // I=8,F=0
                let r2 = mul_in_place(a, b, 4); // I=4,F=4
                if r1 != r2 {
                    disagreements += 1;
                }
                checked += 1;
            }
        }
        assert_eq!(checked, 65_536);
        assert!(
            disagreements > 0,
            "if the two value sets never disagreed, the fraction position \
             would not be observable and the axis would be vacuous"
        );
        println!("B: {disagreements} of {checked} pairs disagree between I=8,F=0 and I=4,F=4");
    }

    /// B, second part. The denotations differ as rationals, which is what
    /// makes the disagreement above one about meaning rather than about a
    /// shift.
    #[test]
    fn the_two_value_sets_denote_different_rationals() {
        for bits in 0u16..=255 {
            let bits = bits as u8;
            let (n1, d1) = denote_i8f0(bits);
            let (n2, d2) = denote_i4f4(bits);
            // n1/d1 == n2/d2  iff  n1*d2 == n2*d1
            let same = n1 * d2 == n2 * d1;
            if bits == 0 {
                assert!(same, "zero denotes zero under both");
            } else {
                assert!(!same, "bit pattern {} must denote two values", bits);
            }
        }
    }
}
