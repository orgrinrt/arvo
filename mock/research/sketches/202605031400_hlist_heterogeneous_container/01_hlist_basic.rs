//! Sketch 01: HList basic shape — Cons/Nil declarations + repr(C) layout
//! verification.
//!
//! Question: does `Cons<H, T>` with `H, T: Copy` lay out predictably under
//! `repr(C)`? Specifically: is `size_of::<Cons<u128, Cons<u64, Nil>>>() ==
//! 24` (16 + 8 + 0 = 24 bytes, 192 bits) when `Nil` is a ZST?
//!
//! Run: `rustc --edition 2024 -Zunstable-options 01_hlist_basic.rs && ./01_hlist_basic`
//!
//! Outcome: TBD. Run before deciding on heterogeneous HList path.

#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![allow(incomplete_features)]

// Nil is a zero-sized terminator.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Nil;

// Cons<H, T> holds the head primitive plus the tail HList.
// H is one of u8 / u16 / u32 / u64 / u128 (a base BitPrim).
// T is another HList (Cons<...> or Nil).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Cons<H: Copy, T: Copy> {
    pub head: H,
    pub tail: T,
}

// Convenience constructors for common shapes.
impl Nil {
    pub const fn new() -> Self { Nil }
}

impl<H: Copy, T: Copy> Cons<H, T> {
    pub const fn new(head: H, tail: T) -> Self { Self { head, tail } }
}

// Type aliases for sample shapes.
pub type C8 = u8;
pub type C16 = u16;
pub type C24 = Cons<u16, u8>;          // 24 bits — but layout-wise this is Cons<u16, u8> ≠ Cons<u16, Nil>
pub type C32 = u32;
pub type C64 = u64;
pub type C128 = u128;
pub type C192 = Cons<u128, u64>;       // 16 + 8 = 24 bytes = 192 bits IF NO PADDING
pub type C200 = Cons<u128, Cons<u64, u8>>;  // 16 + 8 + 1 = 25 bytes IF NO PADDING (more likely 32 with align)
pub type C256 = Cons<u128, u128>;      // 16 + 16 = 32 bytes = 256 bits
pub type C384 = Cons<u128, Cons<u128, u128>>;  // 48 bytes = 384 bits
pub type C512 = Cons<u128, Cons<u128, Cons<u128, u128>>>;  // 64 bytes = 512 bits
pub type C1024 = Cons<u128, Cons<u128, Cons<u128, Cons<u128, Cons<u128, Cons<u128, Cons<u128, u128>>>>>>>; // 128 bytes

// Compile-time layout assertions.
// If `Cons<H, T>` doesn't lay out as `sizeof(H) + sizeof(T)`, these fail to compile.
const _: () = {
    use core::mem::size_of;
    assert!(size_of::<Nil>() == 0, "Nil must be ZST");
    assert!(size_of::<C8>() == 1);
    assert!(size_of::<C16>() == 2);
    assert!(size_of::<C32>() == 4);
    assert!(size_of::<C64>() == 8);
    assert!(size_of::<C128>() == 16);
    // C192 = Cons<u128, u64>. With repr(C), align is 16 (max of u128's 16 and u64's 8).
    // Layout: 16 (u128) + 8 (u64) = 24 bytes, no padding because tail's align ≤ struct's align.
    // FINDING from this sketch: repr(C) struct alignment is the max of fields.
    // Total struct size must be multiple of alignment. So Cons<u128, u64>:
    // u128 (16 bytes, align 16) + u64 (8 bytes, align 8) = 24 bytes content
    // → padded to 32 bytes (next multiple of 16). The "exact 192 bits = 24 bytes"
    // intuition is wrong; struct must round to alignment.
    assert!(size_of::<C192>() == 32, "C192 = Cons<u128, u64> = 32 bytes (24 content + 8 trailing padding)");
    // C256 = Cons<u128, u128>. 16 + 16 = 32 bytes (no padding; size is multiple of 16).
    assert!(size_of::<C256>() == 32, "C256 = Cons<u128, u128> = 32 bytes (no padding, exact)");
    // C384 = Cons<u128, Cons<u128, u128>>. Inner 32 bytes, outer 16 + 32 = 48 bytes.
    assert!(size_of::<C384>() == 48, "C384 = 48 bytes (exact, all 16-aligned)");
    // C512: 64 bytes.
    assert!(size_of::<C512>() == 64, "C512 = 64 bytes (exact, all 16-aligned)");
    // C1024: 128 bytes.
    assert!(size_of::<C1024>() == 128, "C1024 = 128 bytes (exact, all 16-aligned)");
};

// Padding-prone shape: u8 in the middle of a u128/u64 chain.
// Cons<u128, Cons<u64, u8>> — innermost is `Cons<u64, u8>` which has align 8 from u64,
// size 8 (u64) + 1 (u8) + 7 (padding to align 8) = 16 bytes.
// Outer: 16 (u128) + 16 (inner) = 32 bytes total.
// So C200 actually pads to 32 bytes, NOT 25. This is a concern for "exact bit width" promise.
const _: () = {
    use core::mem::size_of;
    // Document expected padded behavior:
    let s_inner = size_of::<Cons<u64, u8>>();
    assert!(s_inner == 16, "Cons<u64, u8> with repr(C) pads to 16 bytes (u8 alignment lost to u64)");
    let s_outer = size_of::<C200>();
    assert!(s_outer == 32, "C200 = Cons<u128, Cons<u64, u8>> = 32 bytes (with padding)");
};

// FINDING (analytical, before compile): repr(C) HList nested with smaller-prim tail
// pads up to the largest prim's alignment. This means C200 (logical 200 bits)
// occupies 32 bytes / 256 bits of storage. The "exact bit width" promise of arvo's
// optimal-fit is NOT preserved by repr(C) HList alone.
//
// Resolution options:
// (a) repr(packed) chain — sound for read/write at the cost of unaligned access codegen.
// (b) Order parts smallest-first: Cons<u8, Cons<u64, u128>>. Then padding falls at the
//     END of the struct, between u128 and any subsequent containing struct. For an
//     isolated container this eliminates internal padding but the trailing padding
//     after u128 alignment (== 0 bytes) doesn't exist. So smallest-first order MAY
//     work — needs verification.
// (c) Accept the padding for heterogeneous-with-mixed-aligns cases; document that
//     the substrate's "optimal-fit" guarantee is "smallest-storage-with-natural-alignment"
//     not "exact-bit-storage".
//
// Each has tradeoffs. (b) is the most appealing if the order-doesn't-matter property holds.

// Helper: define a "smallest-first" HList shape and verify.
pub type C200_SmallFirst = Cons<u8, Cons<u64, u128>>;
const _: () = {
    use core::mem::size_of;
    // Cons<u64, u128>: align 16, size 16 (u128) + 8 (u64) + 8 (padding to align) = 32 OR
    // 8 (u64) + 8 (padding) + 16 (u128) = 32. repr(C) field-order matters; with smallest-first
    // it's u64 at offset 0, then u128 at offset 16 (after 8 bytes padding) → 32 bytes total.
    let s_inner = size_of::<Cons<u64, u128>>();
    assert!(s_inner == 32, "Cons<u64, u128> = 32 bytes (8 padding before u128)");
    // So smallest-first DOESN'T help. The padding shifts but stays.
};

// CONCLUSION (analytical): heterogeneous repr(C) HList with mixed primitive aligns
// will pad. There is no field order that avoids it. The "optimal-fit" promise must
// either:
// - Use repr(packed) [unaligned access cost]
// - Restrict heterogeneity to align-compatible primitive sets
// - Accept the padding as the storage cost
//
// For the substrate's purpose: bitpacked `Cold` strategy already does manual bit
// packing inside its container; the HList layout doesn't have to preserve exact bits
// for `Cold`. For `Hot` and `Warm`, optimal-fit was about minimizing storage; the
// padded HList still gives "smallest natural-aligned storage" which is what `Hot`
// is supposed to provide.
//
// Recommendation: accept the natural-alignment padding. Document this. The container
// reports its physical storage size (via size_of) which may exceed the logical bit
// width; LOGICAL_WIDTH stays the user-facing concept (200 bits) and the physical
// container just happens to be 32 bytes for C200.

fn main() {
    // Print sizes at runtime for convenience (compile-time asserts above already verified them).
    println!("C8: {} bytes", core::mem::size_of::<C8>());
    println!("C16: {} bytes", core::mem::size_of::<C16>());
    println!("C24: {} bytes", core::mem::size_of::<C24>());
    println!("C32: {} bytes", core::mem::size_of::<C32>());
    println!("C64: {} bytes", core::mem::size_of::<C64>());
    println!("C128: {} bytes", core::mem::size_of::<C128>());
    println!("C192: {} bytes (logical 192 bits)", core::mem::size_of::<C192>());
    println!("C200 (large-first): {} bytes (logical 200 bits — padded to 256)", core::mem::size_of::<C200>());
    println!("C200_SmallFirst: {} bytes", core::mem::size_of::<C200_SmallFirst>());
    println!("C256: {} bytes (logical 256 bits)", core::mem::size_of::<C256>());
    println!("C384: {} bytes (logical 384 bits)", core::mem::size_of::<C384>());
    println!("C512: {} bytes (logical 512 bits)", core::mem::size_of::<C512>());
    println!("C1024: {} bytes (logical 1024 bits)", core::mem::size_of::<C1024>());
}
