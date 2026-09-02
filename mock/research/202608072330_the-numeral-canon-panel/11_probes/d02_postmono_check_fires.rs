//! d02. Control for d01's validation: does the post-monomorphisation assert
//! actually fire, and what does its diagnostic look like?
//! A numeral claiming 2 bytes for 40 bits.
#![no_std]
#![crate_type = "lib"]
//! d01. The one const-to-type family Rust admits generically, taken seriously.
//!
//! Section 1 of `11` establishes that `Idx<N> -> [u8; N]` is a total, uncapped,
//! enumeration-free const-to-type map, because a BARE const parameter reaches
//! type position with no feature. This file asks whether a numeral can be built
//! on that family alone: no bridge, no table, no nats, no marker.
//!
//! The surface carries a byte count as a third coordinate. Everything else is
//! derived from it by array-length substitution, which is the permitted family.
//!
//! No `#![feature]`, no `-Z` flag.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata,asm -o out/d01.meta d01_bare_parameter_carrier.rs

use core::marker::PhantomData;

pub struct Hot;

/// The numeral. `B` is the byte count. No table anywhere in this file.
#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, const B: usize, S> {
    raw: [u8; B],
    _m: PhantomData<S>,
}

impl<const I: u32, const F: u32, const B: usize, S> Fixed<I, F, B, S> {
    /// Validation, post-monomorphisation, with a message of our own choosing.
    /// This is the refinement-type shape from `11` section 3.8: it constrains a
    /// representation and does not choose one.
    const CHECK: () = assert!(
        B * 8 >= (I + F) as usize,
        "this numeral's byte count is too small for its bit width"
    );
    pub const fn new(raw: [u8; B]) -> Self {
        let () = Self::CHECK;
        Self {
            raw,
            _m: PhantomData,
        }
    }
}

// --- the properties the bridge could not give -------------------------------

// arbitrary width, no declaration anywhere
pub type Odd = Fixed<4711, 1, 590, Hot>;
pub type Small = Fixed<13, 3, 2, Hot>;

// closed under the law algebra: no output width can be "not shipped", because
// there is nothing to ship. Three octaves, none of which needs a row.
pub fn mul<
    const I: u32,
    const F: u32,
    const B: usize,
    const OI: u32,
    const OF: u32,
    const OB: usize,
    S,
>(
    _a: Fixed<I, F, B, S>,
    _b: Fixed<I, F, B, S>,
) -> Fixed<OI, OF, OB, S> {
    todo!()
}
pub fn octave_1(a: Small, b: Small) -> Fixed<26, 6, 4, Hot> {
    mul(a, b)
}
pub fn octave_2(a: Fixed<26, 6, 4, Hot>, b: Fixed<26, 6, 4, Hot>) -> Fixed<52, 12, 8, Hot> {
    mul(a, b)
}
pub fn octave_3(a: Fixed<52, 12, 8, Hot>, b: Fixed<52, 12, 8, Hot>) -> Fixed<104, 24, 16, Hot> {
    mul(a, b)
}

// --- and the two things it costs, both measurable ---------------------------

// (1) layout. A 16-bit numeral is two bytes at align 1, not a u16 at align 2.
const _: () = {
    assert!(core::mem::size_of::<Small>() == 2);
    assert!(core::mem::align_of::<Small>() == 1);
    assert!(core::mem::align_of::<u16>() == 2);
};

// (2) codegen. These two must be compared, not assumed.
pub fn d01_arvo16(a: Small, b: Small) -> Small {
    let x = u16::from_le_bytes(a.raw);
    let y = u16::from_le_bytes(b.raw);
    Fixed {
        raw: x.wrapping_add(y).to_le_bytes(),
        _m: PhantomData,
    }
}

pub fn d01_native16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}

// the vectorisation case, which is where a layout difference shows up
pub fn d01_arvo_vec(x: &mut [Small; 1024], y: &[Small; 1024]) {
    let mut i = 0;
    while i < 1024 {
        let a = u16::from_le_bytes(x[i].raw);
        let b = u16::from_le_bytes(y[i].raw);
        x[i].raw = a.wrapping_add(b).to_le_bytes();
        i += 1;
    }
}

pub fn d01_native_vec(x: &mut [u16; 1024], y: &[u16; 1024]) {
    let mut i = 0;
    while i < 1024 {
        x[i] = x[i].wrapping_add(y[i]);
        i += 1;
    }
}

pub const BAD: Fixed<40, 0, 2, Hot> = Fixed::<40, 0, 2, Hot>::new([0; 2]);
