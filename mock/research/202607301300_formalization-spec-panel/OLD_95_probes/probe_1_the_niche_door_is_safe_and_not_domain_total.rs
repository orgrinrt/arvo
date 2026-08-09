//! Probe 1. File 92 section 2.3's amendment, checked at the instantiation where
//! its distinction is nonvacuous.
//!
//! File 92 adopts a door typed at the niche and concludes it "is *safe*: every
//! store through it is a safely-constructed `NonZeroU16`, which cannot be zero,
//! so the soundness obligation returns to the type system and vanishes from the
//! caller entirely" (92:296-299). That is compiled at `92_probes/probe_4`, whose
//! `Biased` carries no domain bound at all: `value()` is `self.0.get() - 1`,
//! total over every inhabitant, so the numeral's domain IS 2^16 - 1 and the door
//! is trivially domain-total there.
//!
//! File 92 also compiles, in a different section for a different argument, the
//! case where it is not: "a bounded numeral of 2^13 values biased into
//! NonZeroU16 leaves (2^16 - 1) - 2^13 = 57343 inhabitants with no decode"
//! (92_probes/probe_2:40-41), which it files under the audited entry's
//! over-collection (section 1.3) and never carries back to the door.
//!
//! Same model, two instantiations, opposite verdicts. This probe runs both under
//! one type so the divergence is one program.
//!
//! Rust safety is not the property in question. Every store below is safe by the
//! type system, and no store below is UB. What separates the two instantiations
//! is whether the door can place the carrier on an inhabitant the numeral's own
//! decode has no answer for, which is the unenforced domain side-condition
//! statement 0's hardening forbids at the fields level (80:99-102).
//!
//! Build: rustc --edition 2021 -O probe_1_*.rs -o out/probe_1 && ./out/probe_1

use core::num::NonZeroU16;

/// A bounded numeral of `CARD` values, biased by one into a `NonZero` carrier.
/// This is file 84's construction as the consolidation states it: "store a
/// bounded numeral's datum shifted by one in `core::num::NonZero`" (91:581-582).
#[derive(Copy, Clone)]
#[repr(transparent)]
struct Bounded<const CARD: u32>(NonZeroU16);

impl<const CARD: u32> Bounded<CARD> {
    /// The construction door. Refuses out-of-domain input.
    fn embed(v: u16) -> Option<Self> {
        if (v as u32) < CARD {
            Some(Bounded(NonZeroU16::new(v + 1).unwrap()))
        } else {
            None
        }
    }

    /// The decode. Partial exactly where the domain does not fill the niche.
    fn decode(self) -> Option<u16> {
        let raw = self.0.get() - 1;
        if (raw as u32) < CARD {
            Some(raw)
        } else {
            None
        }
    }

    /// File 92 section 2.3's door, verbatim in shape. Safe. No `unsafe` anywhere
    /// in this file.
    fn typed_mut(&mut self) -> &mut NonZeroU16 {
        &mut self.0
    }
}

/// The number of inhabitants a `NonZeroU16` carrier has.
const NICHE_INHABITANTS: u32 = (1u32 << 16) - 1;

fn sweep<const CARD: u32>(label: &str) -> (u32, u32) {
    let mut decoded = 0u32;
    let mut orphaned = 0u32;
    // Walk every inhabitant of the carrier through the door.
    for raw in 1u32..=NICHE_INHABITANTS {
        let mut n = Bounded::<CARD>::embed(0).unwrap();
        *n.typed_mut() = NonZeroU16::new(raw as u16).unwrap();
        match n.decode() {
            Some(_) => decoded += 1,
            None => orphaned += 1,
        }
    }
    println!(
        "{label}: CARD={CARD} inhabitants={NICHE_INHABITANTS} decoded={decoded} orphaned={orphaned} \
         door_domain_total={}",
        orphaned == 0
    );
    (decoded, orphaned)
}

fn main() {
    // Instantiation A: the domain fills the niche exactly. This is
    // 92_probes/probe_4's shape, where the amendment was tested.
    let (dec_a, orph_a) = sweep::<65535>("A (domain fills the niche)");
    assert_eq!(dec_a, NICHE_INHABITANTS);
    assert_eq!(orph_a, 0);

    // Instantiation B: a bounded numeral, which is what file 84's construction
    // is for. Same door, same type, same safety, different verdict.
    let (dec_b, orph_b) = sweep::<8192>("B (bounded numeral, 2^13 values)");
    assert_eq!(dec_b, 8192);
    assert_eq!(orph_b, 57343);
    assert_eq!(orph_b, NICHE_INHABITANTS - 8192);

    // The concrete break, one store, spelled out. Entirely safe code.
    let mut n = Bounded::<8192>::embed(7).unwrap();
    assert_eq!(n.decode(), Some(7));
    *n.typed_mut() = NonZeroU16::new(60000).unwrap(); // safe: a valid NonZeroU16
    println!(
        "B: after one safe store through the niche-typed door, decode() = {:?} \
         (carrier raw = {}, debias = {})",
        n.decode(),
        n.0.get(),
        n.0.get() - 1
    );
    assert_eq!(n.decode(), None);

    // What closes it, and it is a declaration-site const equation over type
    // parameters, exactly the tier file 92 section 1.3 moved the same fact into
    // for the audited entry. It belongs on the door for the same reason.
    println!(
        "door-total const equation (CARD == 2^w - 1): A={} B={}",
        DoorTotal::<65535>::OK,
        // DoorTotal::<8192>::OK would not evaluate; see probe_1b.
        "refused, see probe_1b"
    );
}

struct DoorTotal<const CARD: u32>;
impl<const CARD: u32> DoorTotal<CARD> {
    const OK: bool = {
        assert!(CARD == NICHE_INHABITANTS);
        true
    };
}
