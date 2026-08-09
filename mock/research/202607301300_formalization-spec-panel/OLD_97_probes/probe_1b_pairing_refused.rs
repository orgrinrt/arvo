//! Probe 1b. The compile-fail half: file 95's probe_1 case B (a 2^13-value
//! bounded numeral over NonZeroU16, value-unique) is refused at the FIRST
//! construction, before any door exists to ask about. Same const equation as
//! 95_probes/probe_1b, RELOCATED to the ratified site (every construction
//! path), demonstrating that the door-side copy guards an empty region.
//!
//! Expected: FAILS TO COMPILE, E0080, at `embed`, not at `typed_mut`.
//!
//! Build: rustc --edition 2021 probe_1b_pairing_refused.rs -o out/probe_1b
//!        (expected: error)

use core::num::NonZeroU16;

const NICHE_INHABITANTS: u32 = (1u32 << 16) - 1;

#[repr(transparent)]
struct Bounded<const CARD: u32>(NonZeroU16);

impl<const CARD: u32> Bounded<CARD> {
    const PAIRING_TOTAL: () = assert!(
        CARD == NICHE_INHABITANTS,
        "value-unique decode partial over the carrier's inhabitants: this pairing is refused at declaration"
    );

    fn embed(v: u16) -> Self {
        let () = Self::PAIRING_TOTAL;
        Bounded(NonZeroU16::new(v.wrapping_add(1)).expect("bias"))
    }

    #[allow(dead_code)]
    fn typed_mut(&mut self) -> &mut NonZeroU16 {
        &mut self.0
    }
}

fn main() {
    // File 95 probe_1 case B, verbatim in intent: the bounded numeral it
    // constructed and then orphaned through the door. Here it never
    // constructs.
    let mut b = Bounded::<8192>::embed(0);
    let _ = b.typed_mut();
}
