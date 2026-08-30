// p2: a Rust array's stride IS size_of, so no carrier-typed array can express a packed extent.
//
// Checks ONE thing, in the type system rather than in arithmetic: that the packed layout the
// Cold strategy promises has no expression as [Carrier; N], for any Carrier, at any width that
// is not a whole number of bytes. If that holds, the second output cannot be a type.
//
//   rustc +nightly-2026-05-28 -O p2_stride_is_not_size_of.rs -o /tmp/p2 && /tmp/p2
//
// no_std is not asserted here; this probe prints, so it needs a host main. The claim it checks
// is about layout, which core::mem answers identically under no_std.
//
// Spike. Presume it flawed.

use core::mem::{align_of, size_of};

// The one-output derivation, as a type-level map, for the widths this probe uses.
#[repr(transparent)]
struct Num13Cold(u16);
#[repr(transparent)]
struct Num16Cold(u16);
#[repr(transparent)]
struct Num13Warm(u16);

// The two-output derivation cannot be a newtype, so here is what it has to be instead:
// a bit stream plus a stride, with the element recovered by shifting. Concrete widths only,
// because a generic length would put an expression in type position and that needs
// generic_const_exprs, which is forbidden.
const N: usize = 1000;

// ceil(13 * 1000 / 8) = 1625, written out because the expression may not be generic.
struct Packed13x1000([u8; 1625]);
// ceil(16 * 1000 / 8) = 2000
struct Packed16x1000([u8; 2000]);

const fn assert_eq_usize(a: usize, b: usize) -> bool {
    a == b
}

// Compile-time assertions. If any of these is false the build fails, which is the point:
// a claim about layout should be checked by the compiler, not printed and eyeballed.
const _: () = assert!(assert_eq_usize(size_of::<Num13Cold>(), 2));
const _: () = assert!(assert_eq_usize(size_of::<Num16Cold>(), 2));
const _: () = assert!(assert_eq_usize(size_of::<Num13Warm>(), 2));

// The collapse, as a compile-time fact: the two Cold declarations are layout-identical,
// and the 13-bit Cold declaration is layout-identical to the 13-bit Warm one.
const _: () = assert!(size_of::<Num13Cold>() == size_of::<Num16Cold>());
const _: () = assert!(size_of::<Num13Cold>() == size_of::<Num13Warm>());
const _: () = assert!(align_of::<Num13Cold>() == align_of::<Num16Cold>());

// The array stride identity. There is no way to make [T; N] pack.
const _: () = assert!(size_of::<[Num13Cold; N]>() == size_of::<Num13Cold>() * N);
const _: () = assert!(size_of::<[Num13Cold; N]>() == 2000);

// What the strategy promised.
const _: () = assert!(size_of::<Packed13x1000>() == 1625);
const _: () = assert!(size_of::<Packed16x1000>() == 2000);

// And the two-output form agrees with the one-output form exactly where the width is a whole
// number of bytes, which is why a power-of-two test matrix reports green.
const _: () = assert!(size_of::<Packed16x1000>() == size_of::<[Num16Cold; N]>());
const _: () = assert!(size_of::<Packed13x1000>() != size_of::<[Num13Cold; N]>());

fn main() {
    println!("all layout assertions are const and were checked at compile time");
    println!();
    println!("one declaration, one carrier, one size:");
    println!(
        "  UFixed<13,0,Cold> -> carrier u16, size_of = {}",
        size_of::<Num13Cold>()
    );
    println!(
        "  UFixed<16,0,Cold> -> carrier u16, size_of = {}",
        size_of::<Num16Cold>()
    );
    println!(
        "  UFixed<13,0,Warm> -> carrier u16, size_of = {}",
        size_of::<Num13Warm>()
    );
    println!("  the three are indistinguishable by every layout question a type can answer");
    println!();
    println!("N = {N} contiguous:");
    println!(
        "  [carrier; N] for the 13-bit declaration : {} bytes",
        size_of::<[Num13Cold; N]>()
    );
    println!(
        "  packed extent for the 13-bit declaration: {} bytes",
        size_of::<Packed13x1000>()
    );
    println!(
        "  packed extent for the 16-bit declaration: {} bytes",
        size_of::<Packed16x1000>()
    );
    println!();
    println!("the 16-bit rows agree, the 13-bit rows do not. a matrix of power-of-two widths");
    println!("reports green over a derivation that has lost the strategy entirely.");
}
