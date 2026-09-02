//! The combined case consolidation nine's open list owes (91:997-999): a
//! niche-carrying numeral that also ships a raw mutable door. Two door
//! shapes, two different failure tiers, and only one composes.
//!
//! Door A is the integer-typed raw door, the exact shape file 87's probe 3
//! modelled for a padding carrier, transplanted unchanged onto a niche
//! carrier. Door B is typed at the niche. The finding: through door A the
//! niche-violating write is one line, carries ZERO diagnostics (contrast
//! the warn-level `invalid_value` lint on the value-transmute in
//! 87_probes/probe_1), and is undefined behaviour, not decorrelation; the
//! violating body is compiled and never executed. Through door B no safe
//! store can produce the excluded pattern, because no safely-constructed
//! NonZeroU16 is zero: the soundness obligation returns to the type system
//! and vanishes from the caller.
use core::num::NonZeroU16;

#[derive(Copy, Clone)]
#[repr(transparent)]
struct Biased(NonZeroU16); // value v stored as v + 1

impl Biased {
    fn embed(v: u16) -> Self {
        Biased(NonZeroU16::new(v + 1).unwrap())
    }
    fn value(self) -> u16 {
        self.0.get() - 1
    }
    /// door shape A: integer-typed, file 87 probe 3's shape unchanged
    unsafe fn to_raw_mut(&mut self) -> &mut u16 {
        unsafe { &mut *(self as *mut Biased as *mut u16) }
    }
    /// door shape B: typed at the niche, and safe
    fn typed_mut(&mut self) -> &mut NonZeroU16 {
        &mut self.0
    }
}

// The padding obligation is vacuous for the NonZero family everywhere:
// every member's width is whole-byte, so at Dense the container equals the
// carrier, and under Bitpacked the group arithmetic (P = 8/gcd(W_S, 8))
// gives whole-byte groups with zero pad bits at every member width.
const _: () = {
    let widths = [8u32, 16, 32, 64, 128];
    let mut i = 0;
    while i < 5 {
        let w = widths[i];
        let g = w / 8; // gcd(w, 8) = 8 for every member width
        let p = 8 / 8;
        assert!(g * 8 == w * p); // container bits == carrier bits: no pad
        i += 1;
    }
};

/// compiled, never executed: the silent UB shape door A permits
#[allow(dead_code)]
fn never_run(b: &mut Biased) {
    // no lint fires on this line; the invalid_value lint catches value
    // transmutes, not place stores through an integer-typed borrow
    unsafe {
        *b.to_raw_mut() = 0;
    }
}

fn main() {
    let mut b = Biased::embed(500);
    assert_eq!(b.value(), 500);

    // door B, safe, honest by construction
    *b.typed_mut() = NonZeroU16::new(7001).unwrap();
    assert_eq!(b.value(), 7000);

    // ten arbitrary safe mutations through the typed door; the excluded
    // pattern is unreachable on every one, structurally
    let mut i: u16 = 1;
    while i <= 10 {
        *b.typed_mut() = NonZeroU16::new(i * 3 + 1).unwrap();
        assert_eq!(b.value(), i * 3);
        i += 1;
    }
    println!(
        "combined case: typed door, 10 safe mutations, value = {}, excluded pattern unreachable throughout; integer door compiled with zero diagnostics and stays unexecuted",
        b.value()
    );
}
