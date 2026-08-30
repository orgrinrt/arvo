// PROBE C: what a fallible return costs in LAYOUT, which is arvo's identity.
// Sizes measured, not reasoned. Run under nightly-2026-05-28.
//
// OUTPUT:
//   Full16 (bare)                                size  2  align  2
//   Just<Full16>                                 size  2  align  2
//   Outcome<Full16, ZST err>                     size  4  align  2
//   Outcome<Full16, 2-byte err>                  size  4  align  2
//   Maybe<Full16>                                size  4  align  2
//   InU32 (bare, 16 spare bits unknown to rustc) size  4  align  4
//   Outcome<InU32, ZST err>                      size  8  align  4
//   Narrow4 (4 valid patterns, niche known)      size  1  align  1
//   Outcome<Narrow4, ZST err>                    size  1  align  1
//   Maybe<Narrow4>                               size  1  align  1
//   [Full16; 8] column of 8                      size 16  align  2
//   [Outcome<Full16, ZST>; 8] column of 8        size 32  align  2
//   u64                                          size  8 ; Outcome<u64, ZST>  size 16
//   u128                                         size 16 ; Outcome<u128, ZST> size 32
//
// The Narrow4 rows are the load-bearing ones: where rustc KNOWS the valid
// pattern range, the refusal costs zero bytes. arvo's exact widths create
// exactly that spare range and repr(transparent) over a full-width primitive
// throws the knowledge away.
#![allow(dead_code)]
use core::mem::{align_of, size_of};
use notko::{Just, Maybe, Outcome};

#[derive(Clone, Copy)]
pub struct OutOfRange;
#[derive(Clone, Copy)]
pub struct Detailed {
    pub over: bool,
    pub at: u8,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Full16(u16);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct InU32(u32);
#[derive(Clone, Copy)]
pub enum Narrow4 {
    A,
    B,
    C,
    D,
}

fn row<T>(name: &str) {
    println!(
        "{:<44} size {:>2}  align {:>2}",
        name,
        size_of::<T>(),
        align_of::<T>()
    );
}

fn main() {
    row::<Full16>("Full16 (bare)");
    row::<Just<Full16>>("Just<Full16>");
    row::<Outcome<Full16, OutOfRange>>("Outcome<Full16, ZST err>");
    row::<Outcome<Full16, Detailed>>("Outcome<Full16, 2-byte err>");
    row::<Maybe<Full16>>("Maybe<Full16>");
    row::<InU32>("InU32 (bare, 16 spare bits unknown to rustc)");
    row::<Outcome<InU32, OutOfRange>>("Outcome<InU32, ZST err>");
    row::<Narrow4>("Narrow4 (4 valid patterns, niche known)");
    row::<Outcome<Narrow4, OutOfRange>>("Outcome<Narrow4, ZST err>");
    row::<Maybe<Narrow4>>("Maybe<Narrow4>");
    row::<[Full16; 8]>("[Full16; 8] column of 8");
    row::<[Outcome<Full16, OutOfRange>; 8]>("[Outcome<Full16, ZST>; 8] column of 8");
    row::<u64>("u64");
    row::<Outcome<u64, OutOfRange>>("Outcome<u64, ZST err>");
    row::<u128>("u128");
    row::<Outcome<u128, OutOfRange>>("Outcome<u128, ZST err>");
}
