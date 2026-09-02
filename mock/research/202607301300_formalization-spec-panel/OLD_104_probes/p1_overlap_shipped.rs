// p1: the shipped `bitfield!`'s placement map, against the shipped macro.
//
// Compiled from a scratch crate OUTSIDE the repo (mock/crates is out of the
// panel's scope), path-depending on the shipped facade, with the repo's own
// pinned toolchain. Manifest and commands in OUTCOMES.md.
//
// Two declarations. A. overlapping fields, both contained: compiles, and the
// overlap silently aliases at runtime. B. a field running past the container:
// refuses at declaration with no construction and no use.

#![feature(const_trait_impl)]
#![feature(macro_metavar_expr_concat)]
#![allow(incomplete_features)]
#![allow(non_upper_case_globals)]

use arvo::bitfield;
use arvo_bits::{Bits, Hot};

// ---- A ----------------------------------------------------------------
bitfield! {
    /// a occupies [0,8); b occupies [4,12). Bits 4..8 belong to both.
    pub struct Overlap: 16 {
        /// low byte
        a: 8 at 0,
        /// overlapping byte
        b: 8 at 4,
    }
}

pub fn overlap_witness() -> (u16, u16, u16) {
    let h = Overlap::new()
        .with_a(Bits::<8, Hot>::from_raw(0xFF_u8))
        .with_b(Bits::<8, Hot>::from_raw(0x00_u8));
    (
        h.to_bits().to_raw(),
        h.a().to_raw() as u16,
        h.b().to_raw() as u16,
    )
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn overlap_aliases() {
        let (bits, a, b) = overlap_witness();
        println!("container = {:#018b}, a = {:#x}, b = {:#x}", bits, a, b);
        println!("a still 0xFF? {}", a == 0xFF);
    }
}

// ---- B ----------------------------------------------------------------
// Compiled separately; the two cannot share a file because B refuses.
//
// bitfield! {
//     /// lo = 12, width = 8, N = 16: runs to bit 20
//     pub struct OutOfRange: 16 {
//         /// runs past the container
//         wide: 8 at 12,
//     }
// }
