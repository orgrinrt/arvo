// Probe 3: the two-tier repair, compiled. Tier 1, the safe surface, never hands out a
// raw door into padding at all: every mutation is value -> value, re-embedding through
// the same pure constructor every construction already uses, so statement P/C's
// postcondition is re-established on every safe write, structurally, with no tracking
// needed. Tier 2, the unsafe escape hatch every toolbox-not-policer / always-optimal-
// internals door implies will exist, carries the canonicalisation obligation as a named,
// documented, trusted-base postcondition on the door itself, exactly parallel to
// Crosses's own statement 0/P obligations, rather than left implicit.
#![allow(dead_code)]

const FIELDS_MASK: u16 = 0x1FFF;
const PADDING_MASK: u16 = !FIELDS_MASK;

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Carrier(u16);

const fn embed(datum: u16) -> Carrier {
    Carrier(datum & FIELDS_MASK)
}
const fn canonical_read(c: &Carrier) -> u16 {
    c.0 & FIELDS_MASK
}

// --- Tier 1: the safe mutation surface. No raw accessor exists at this tier at all;
// every method is value -> value. Statement P/C hold after every call, structurally,
// because every call re-embeds. This is not a new mechanism: it is ordinary
// construction, called again.
impl Carrier {
    fn set(&mut self, datum: u16) {
        *self = embed(datum);
    }
    fn add_wrapping(&mut self, delta: u16) {
        let v = (canonical_read(self).wrapping_add(delta)) & FIELDS_MASK;
        *self = embed(v);
    }
}

// --- Tier 2: the unsafe escape hatch, `from_raw`/`to_raw`-shaped per
// arvo-always-optimal-internals.md's own stated doors. The safety contract is a
// documented postcondition, not a type-system-enforced one: the same status as
// Vec::set_len's own "you must have initialized these elements" obligation.
impl Carrier {
    /// # Safety
    /// The caller must ensure that, before this borrow's last use, every bit
    /// outside `FIELDS_MASK`'s width is left at the padding law's canonical
    /// value (zero). This is a trusted-base obligation, identical in kind and
    /// identical in enforcement to a hand-laid Lowering's own `unsafe impl
    /// Crosses` statement P: the type system consumes the promise and cannot
    /// check it; violating it does not fail to compile, it produces exactly
    /// the silent decorrelation probe 2 demonstrated.
    unsafe fn to_raw_mut(&mut self) -> &mut u16 {
        &mut self.0
    }
}

fn main() {
    // Tier 1: no dirtying is possible through the safe surface, by construction,
    // not by discipline. Ten arbitrary safe mutations, checked after each one.
    let mut c = embed(100);
    for delta in [7000u16, 1, 9999, 42, 8000, 3, 100, 6000, 5, 200] {
        c.add_wrapping(delta);
        assert_eq!(
            c.0 & PADDING_MASK,
            0,
            "tier 1: padding stays canonical after every safe mutation, unconditionally"
        );
    }
    println!(
        "tier 1: {} safe mutations, padding canonical throughout",
        10
    );

    // Tier 2: the documented obligation, honoured. A caller using the escape
    // hatch and respecting its postcondition sees no divergence either.
    let mut c2 = embed(500);
    unsafe {
        let raw = c2.to_raw_mut();
        *raw = (*raw & FIELDS_MASK) | 0; // an honoured caller: canonical padding restored
    }
    assert_eq!(
        c2.0 & PADDING_MASK,
        0,
        "tier 2, honoured: padding canonical"
    );
    println!(
        "tier 2 (contract honoured): padding canonical, byte image = {:#06x}",
        c2.0
    );

    // Tier 2, violated (the trusted-base contract broken, deliberately, to show
    // the failure is exactly the documented one and nothing else silently saves it):
    let mut c3 = embed(500);
    unsafe {
        let raw = c3.to_raw_mut();
        *raw |= PADDING_MASK; // caller violates the documented postcondition
    }
    assert_eq!(canonical_read(&c3), 500, "value-keyed read still correct");
    assert_ne!(
        c3.0,
        embed(500).0,
        "byte image now decorrelated, exactly as documented"
    );
    println!(
        "tier 2 (contract violated): value-keyed read still correct ({}), \
         byte image decorrelated ({:#06x} != {:#06x}), exactly as the door's own \
         safety contract said would happen",
        canonical_read(&c3),
        c3.0,
        embed(500).0
    );
}
