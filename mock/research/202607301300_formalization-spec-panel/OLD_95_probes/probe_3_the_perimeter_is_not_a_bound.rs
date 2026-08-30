//! Probe 3. Whether the mutation theorem's domain can be carried by the type
//! system rather than by a stated sentence.
//!
//! This is the constructive question behind file 92 section 2.1. If the
//! perimeter ("no public field, no `DerefMut` to the representation, no
//! foreign-bytes constructor outside statement C's obligation site, no accessor
//! below the level's write granule") could be a bound, the theorem would not
//! need a written domain at all: the design's standing preference is that the
//! type system is the verification layer and a claim that can refuse should
//! refuse (`harness-the-type-system.md`, and the pricing pillar at 91:113-121).
//!
//! Expected: COMPILES CLEAN. The marker asserts the perimeter and checks
//! nothing, so a type with a public field carries it, and a safe write reaches
//! the bits the marker says are unreachable.
//!
//! A clean compile is the finding here, not a pass: it establishes that the
//! perimeter has no expressible form as a bound on the permitted feature set
//! (no `TypeId`, no reflection, no full `specialization`, and `min_specialization`
//! cannot see a field's visibility either), so the theorem's domain is prose by
//! necessity, which is what puts it under the definitional-completeness line
//! rather than under the harness.
//!
//! Build: rustc --edition 2021 -O probe_3_*.rs -o out/probe_3 && ./out/probe_3

/// The claim, as strong a form as the language permits: an unsafe marker whose
/// documented contract is the whole perimeter.
///
/// # Safety
/// The implementing type's safe surface exposes no public field, no `DerefMut`
/// to the representation, no foreign-bytes constructor outside the
/// canonicalising one, and no accessor below `WRITE_GRANULE_BITS`.
unsafe trait PerimeterClosed {
    const WRITE_GRANULE_BITS: u32;
}

/// An honest implementor.
pub struct Honest(u16);

impl Honest {
    pub fn new(v: u16) -> Self {
        Honest(v & 0x1FFF) // 13 fields, 3 padding bits, canonical by construction
    }
    pub fn value(&self) -> u16 {
        self.0 & 0x1FFF
    }
    pub fn raw(&self) -> u16 {
        self.0
    }
}
unsafe impl PerimeterClosed for Honest {
    const WRITE_GRANULE_BITS: u32 = 16;
}

/// A dishonest implementor. Identical declaration cost, identical bound
/// satisfaction, one `pub` on the field.
pub struct Dishonest(pub u16);

impl Dishonest {
    pub fn new(v: u16) -> Self {
        Dishonest(v & 0x1FFF)
    }
    pub fn value(&self) -> u16 {
        self.0 & 0x1FFF
    }
    pub fn raw(&self) -> u16 {
        self.0
    }
}
unsafe impl PerimeterClosed for Dishonest {
    const WRITE_GRANULE_BITS: u32 = 16;
}

/// A generic consumer that relies on the marker, as a real column digest would.
fn raw_image_is_canonical<T: PerimeterClosed>(_witness: &T) -> bool {
    // The whole point of the marker: a consumer may take the raw image without
    // re-canonicalising, because the perimeter is closed. Nothing here can
    // check that, and nothing the language offers can.
    true
}

fn main() {
    let h = Honest::new(7);
    let mut d = Dishonest::new(7);

    println!("granule(Honest)    = {}", Honest::WRITE_GRANULE_BITS);
    println!("granule(Dishonest) = {}", Dishonest::WRITE_GRANULE_BITS);
    println!(
        "marker satisfied by both: {} {}",
        raw_image_is_canonical(&h),
        raw_image_is_canonical(&d)
    );

    // The safe write the marker promises does not exist. One line, no unsafe.
    d.0 |= 0xE000;

    println!("Honest    value={} raw={:#06x}", h.value(), h.raw());
    println!("Dishonest value={} raw={:#06x}", d.value(), d.raw());
    assert_eq!(h.value(), d.value(), "value-keyed reads still agree");
    assert_ne!(h.raw(), d.raw(), "the raw image has decorrelated");

    println!("the marker compiled, the perimeter did not hold, and rustc said nothing");
}
