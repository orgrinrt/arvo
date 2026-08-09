//! Probe 1b. The close for probe 1's finding, and it is a declaration-site
//! refusal rather than an audit obligation, per the pricing pillar's own clause
//! (91:113-121) and per file 92 section 1.3's own move for the audited entry's
//! other three facts.
//!
//! Expected: FAILS TO COMPILE, E0080, evaluation of the const panicked.
//!
//! Build: rustc --edition 2021 probe_1b_*.rs -o out/probe_1b   (expected: error)

use core::num::NonZeroU16;

const NICHE_INHABITANTS: u32 = (1u32 << 16) - 1;

#[repr(transparent)]
struct Bounded<const CARD: u32>(NonZeroU16);

impl<const CARD: u32> Bounded<CARD> {
    /// The door is domain-total exactly when the numeral's domain fills the
    /// carrier's inhabitant set. Anything less and a safe store lands on an
    /// inhabitant the decode has no answer for.
    const DOOR_TOTAL: () = assert!(CARD == NICHE_INHABITANTS);

    fn typed_mut(&mut self) -> &mut NonZeroU16 {
        let () = Self::DOOR_TOTAL;
        &mut self.0
    }
}

fn main() {
    let mut ok = Bounded::<65535>(NonZeroU16::new(1).unwrap());
    let _ = ok.typed_mut(); // fine

    let mut bad = Bounded::<8192>(NonZeroU16::new(1).unwrap());
    let _ = bad.typed_mut(); // the refusal
}
