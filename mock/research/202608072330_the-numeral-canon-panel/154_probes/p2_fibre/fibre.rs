//! P2. Is the strategy a component of a product with the format, or a fibre
//! over it?
//!
//! Product reading: a value is a pair (format, strategy), the representation is
//! a function of the format, and the strategy annotates behaviour. Then any two
//! strategies at one format have the same representation.
//!
//! Fibre reading: the representation is a function of the PAIR. Two strategies
//! at one format may have different representations, and a format alone is an
//! index rather than a space of values.
//!
//! The repository already contains the discriminating case, at
//! `mock/benches/variants/bitpack-footprint-shared/src/lib.rs:92` --
//! `LOGICAL_BITS = 13` -- held two ways in one buffer: a dense region at
//! `MAX_N * 2` bytes (`:109`) and a packed region at `(MAX_N * 13) / 8` bytes
//! (`:105`). One logical format, two footprints, both shipped, both measured.
//!
//! This probe asks the structural question that pricing cannot: what happens to
//! a generic algorithm written over "a numeral" when one of its instances is
//! dense and another is packed.
//!
//! NEGATIVE CONTROL, stated before the run. The dense instance MUST satisfy the
//! generic algorithm and produce the right answer. If it does not, any failure
//! at the packed instance is a fact about my trait rather than about packing,
//! and the probe establishes nothing.
#![no_std]
#![allow(dead_code)]

use core::mem::size_of;

pub const W: u32 = 13;
pub const MASK: u16 = (1u16 << W) - 1;

// ---------- Instance one: dense. A value is a type. ----------
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Dense13(u16);

impl Dense13 {
    pub const fn new(v: u16) -> Self {
        Dense13(v & MASK)
    }
    pub const fn get(self) -> u16 {
        self.0
    }
}

// ---------- Instance two: packed. There is no such type. ----------
// A packed 13-bit value has no standalone size: `size_of` cannot be 13 bits.
// It exists only as a position in a column, reached through a read.
pub struct Packed13Col<'a>(pub &'a [u8]);

impl<'a> Packed13Col<'a> {
    pub fn get(&self, i: usize) -> u16 {
        let bit = i * W as usize;
        let byte = bit / 8;
        let off = bit % 8;
        let raw = u32::from_le_bytes([
            self.0[byte],
            self.0[byte + 1],
            self.0[byte + 2],
            self.0[byte + 3],
        ]);
        ((raw >> off) as u16) & MASK
    }
}

// ---------- The signature an algorithm crate would be written against ----------
// This is the shape I11 calls for: "the contracts for things that compose to
// bigger units than just numerals alone."
pub trait Numeral: Copy {
    const WIDTH: u32;
    fn to_u64(self) -> u64;
}

impl Numeral for Dense13 {
    const WIDTH: u32 = W;
    fn to_u64(self) -> u64 {
        self.0 as u64
    }
}

/// A stand-in algorithm crate: sums a slice of numerals. It never names an
/// instance.
pub fn algo_sum<T: Numeral>(xs: &[T]) -> u64 {
    let mut s = 0u64;
    for &x in xs {
        s = s.wrapping_add(x.to_u64());
    }
    s
}

// ---------- The control: the dense instance must work ----------
#[unsafe(no_mangle)]
pub fn control_dense_works() -> u64 {
    let xs = [Dense13::new(5), Dense13::new(9), Dense13::new(8191)];
    algo_sum(&xs)
}

/// Sizes, for the record. `Dense13` is 2 bytes: eight sixteenths of a bit
/// wasted per element is the price of being a type at all.
#[unsafe(no_mangle)]
pub fn dense_size() -> usize {
    size_of::<Dense13>()
}

/// The packed element's footprint, per element, in bits. There is no
/// `size_of` for it: this is arithmetic over the column, not a property of a
/// value.
pub const PACKED_BITS_PER_ELEM: u32 = W;
pub const DENSE_BITS_PER_ELEM: u32 = (size_of::<Dense13>() * 8) as u32;

// ---------- The attempt that must fail, and the point of the probe ----------
// Uncommenting the block below is the whole finding. There is no way to write
// `impl Numeral for <a packed element>`, because there is no packed element:
// the trait requires `Copy`, which requires `Sized`, and 13 bits is not a size.
//
// pub struct Packed13Elem;               // what would this hold?
// impl Numeral for Packed13Elem {
//     const WIDTH: u32 = W;
//     fn to_u64(self) -> u64 { /* from where? */ }
// }
//
// The compilable form of the same intent is below, and it is a DIFFERENT
// signature: it is over the column, not over the element.
pub trait NumeralColumn {
    const WIDTH: u32;
    fn len(&self) -> usize;
    fn get_u64(&self, i: usize) -> u64;
}

impl<'a> NumeralColumn for Packed13Col<'a> {
    const WIDTH: u32 = W;
    fn len(&self) -> usize {
        ((self.0.len().saturating_sub(4)) * 8) / W as usize
    }
    fn get_u64(&self, i: usize) -> u64 {
        self.get(i) as u64
    }
}

impl<'a> NumeralColumn for &'a [Dense13] {
    const WIDTH: u32 = W;
    fn len(&self) -> usize {
        <[Dense13]>::len(self)
    }
    fn get_u64(&self, i: usize) -> u64 {
        self[i].0 as u64
    }
}

/// The same algorithm, written against the column signature. THIS one accepts
/// both instances.
pub fn algo_sum_col<C: NumeralColumn>(c: &C) -> u64 {
    let mut s = 0u64;
    let n = c.len();
    let mut i = 0;
    while i < n {
        s = s.wrapping_add(c.get_u64(i));
        i += 1;
    }
    s
}

#[unsafe(no_mangle)]
pub fn both_instances_via_column(packed: &[u8], dense: &[Dense13]) -> u64 {
    algo_sum_col(&Packed13Col(packed)).wrapping_add(algo_sum_col(&dense))
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
