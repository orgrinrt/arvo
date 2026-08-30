//! Probe 1. Under the entry-level totality refusal RATIFIED at 95b (section 4:
//! "inhabitant-totality ... move to declaration-site const refusals"), the
//! region file 95's door-side domain-preservation equation guards is
//! unreachable: every declarable value-unique niche pairing has a total
//! decode, so the niche-typed door is unconditionally domain-preserving.
//!
//! This file is the RUN half: the one declarable instantiation, swept over
//! every inhabitant, no sampling. The compile-fail half is probe_1b.
//!
//! Separation statement per 86b: this model separates door-total from
//! door-partial across the pairing's legality. The nonvacuous instantiation
//! of that distinction (a LEGAL pairing with a PARTIAL decode) does not exist
//! once the ratified entry refusal is in the model, which is the finding:
//! file 95's probe_1 case B is constructible only in a model that omits a
//! statement ratified at the same checkpoint.
//!
//! Build: rustc --edition 2021 -O probe_1_foreclosed_region.rs -o out/probe_1
//! Run:   ./out/probe_1

use core::num::NonZeroU16;

const NICHE_INHABITANTS: u32 = (1u32 << 16) - 1;

#[repr(transparent)]
struct Bounded<const CARD: u32>(NonZeroU16);

impl<const CARD: u32> Bounded<CARD> {
    /// The RATIFIED site: the pairing itself. A value-unique decode partial
    /// over the carrier's inhabitants is the unenforced side-condition
    /// statement 0's hardening forbids (92 section 1.3 (ii), ratified 95b);
    /// the refusal gates every construction path, not the mutable door.
    const PAIRING_TOTAL: () = assert!(
        CARD == NICHE_INHABITANTS,
        "value-unique decode partial over the carrier's inhabitants: this pairing is refused at declaration"
    );

    fn embed(v: u16) -> Self {
        let () = Self::PAIRING_TOTAL;
        Bounded(NonZeroU16::new(v.wrapping_add(1)).expect("bias"))
    }

    fn value(&self) -> u16 {
        self.0.get() - 1
    }

    /// File 92's niche-typed door, with NO door-side equation. The question:
    /// is it domain-preserving anyway, for every declarable pairing?
    fn typed_mut(&mut self) -> &mut NonZeroU16 {
        &mut self.0
    }
}

fn main() {
    // The only declarable value-unique instantiation. Sweep the whole
    // inhabitant set through the door; count orphans (stores whose decode
    // has no answer). Domain = [0, 65534], inhabitants = [1, 65535].
    let mut orphans: u32 = 0;
    let mut visited: u32 = 0;
    let mut x = Bounded::<65535>::embed(0);
    let mut raw: u32 = 1;
    while raw <= 65535 {
        let store = NonZeroU16::new(raw as u16).unwrap();
        *x.typed_mut() = store; // safe store of an arbitrary inhabitant
        let v = x.value(); // decode
                           // value-unique decode over a full-cardinality domain: every
                           // inhabitant answers, and the answer is the debias.
        if u32::from(v) != raw - 1 {
            orphans += 1;
        }
        visited += 1;
        raw += 1;
    }
    println!("declarable pairing: CARD=65535, inhabitants={NICHE_INHABITANTS}");
    println!("inhabitants stored through the niche-typed door: {visited}");
    println!("orphaned stores (decode has no answer): {orphans}");
    println!(
        "door is domain-preserving with no door-side equation: {}",
        orphans == 0 && visited == NICHE_INHABITANTS
    );
}
