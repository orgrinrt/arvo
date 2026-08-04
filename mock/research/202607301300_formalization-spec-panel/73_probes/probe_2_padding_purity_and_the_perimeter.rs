// Probe 2: two claims.
//
// (a) A construction of a carrier FROM a datum (a `From`-shaped, one-argument
//     operation) can only be a pure, deterministic function of the datum if
//     its padding is canonical (fixed by the impl, e.g. zero). "Preserve
//     whatever padding was already there" is not a coherent semantics for
//     construction, because construction has no "already there" to preserve;
//     it is only coherent for an UPDATE of an existing carrier, which is a
//     different, strictly more general two-argument operation. Compiled by
//     showing the two shapes have different signatures and the pure one
//     cannot express the update semantics without the extra argument.
//
// (b) `repr(transparent)` grants every consumer, safe-API or not, a route to
//     the raw carrier bytes that bypasses any padding policy the safe API
//     tries to enforce. The safe constructor's padding choice is not a
//     recommendation; it is the only chance the design gets to fix what an
//     unsafe transmute will see, because the transmute route exists whether
//     or not the safe API ever calls it.

const DATUM_MASK: u16 = 0x1FFF; // 13 live bits, 3 padding bits, matches 72_probes

// (a) construction: one argument, must be pure. this is the ONLY shape that
// can be a `From<u16> for Carrier` impl.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Carrier(u16);

impl From<u16> for Carrier {
    // canonical-at-rest: padding is always zero. this is a pure function of
    // the datum bits alone; nothing else could be, because `From::from`
    // takes exactly one argument and has no prior carrier to consult.
    fn from(datum13: u16) -> Self {
        Carrier(datum13 & DATUM_MASK)
    }
}

// the "preserve existing padding" semantics is NOT expressible as a `From`
// impl. it needs the old carrier as a second argument, which is a strictly
// different operation (an update, not a construction).
pub fn embed_preserving_padding(old: Carrier, new_datum13: u16) -> Carrier {
    let padding = old.0 & !DATUM_MASK;
    Carrier((new_datum13 & DATUM_MASK) | padding)
}

// (b) the perimeter: repr(transparent) makes the raw bytes reachable without
// going through any arvo-declared door at all. a bit-cast (not even `unsafe`
// arvo API, just `unsafe` Rust) sees whatever bits are actually there,
// regardless of which construction path built the value.
fn raw_bytes_via_transmute(c: Carrier) -> [u8; 2] {
    // SAFETY: repr(transparent) over u16; this is exactly the kind of access
    // the type's own declared representation already permits to ANY caller,
    // not only to arvo's own `raw()` door.
    unsafe { core::mem::transmute::<Carrier, [u8; 2]>(c) }
}

fn main() {
    // (a) canonical construction is deterministic: same datum in, same carrier out,
    // regardless of how many times it is called, with no dependence on any prior state.
    let c1 = Carrier::from(0x1A5C);
    let c2 = Carrier::from(0x1A5C);
    assert_eq!(
        c1, c2,
        "From<u16> for Carrier is pure: same input, same output, always"
    );
    assert_eq!(
        c1.0 & !DATUM_MASK,
        0,
        "canonical-at-rest: padding bits are zero by construction"
    );

    // preserving padding needs the OLD carrier; it cannot be squeezed into a
    // one-argument `From` impl, demonstrated by simply needing a different signature.
    let old = Carrier(0x1A5C | 0xE000); // some non-zero padding, as if it arrived foreign
    let updated = embed_preserving_padding(old, 0x1A5C);
    assert_eq!(updated.0 & !DATUM_MASK, 0xE000, "update-in-place CAN preserve padding, but only because it took the old carrier as an argument");

    // (b) the perimeter: whichever padding policy `From` picks, a transmute sees it,
    // unconditionally, with zero gate and zero dependence on arvo ever shipping a
    // `to_bytes()` method at all.
    let bytes = raw_bytes_via_transmute(c1);
    assert_eq!(
        bytes,
        c1.0.to_ne_bytes(),
        "transmute observes exactly the carrier's own native bytes, no arvo API involved"
    );
    let reconstructed = u16::from_ne_bytes(bytes);
    assert_eq!(reconstructed & !DATUM_MASK, 0, "the padding bits an unsafe consumer would see are the ones From<u16> chose, not the ones a safe API happens to expose");

    println!(
        "canonical construction is pure ({:#06x} == {:#06x}); \
         update-in-place needed a second argument to preserve padding ({:#06x}); \
         transmute observes {:?} unconditionally, the same bytes From<u16> committed to",
        c1.0, c2.0, updated.0, bytes
    );
}
