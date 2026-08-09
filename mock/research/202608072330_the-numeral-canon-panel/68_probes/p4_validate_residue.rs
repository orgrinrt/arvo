//! 68 probe p4: the two readings of "validate" leave different residue in the
//! object code, and only one of them is erasure.
//!
//! Reading A (65's): validation is compile-time. Membership is discharged at
//! the construction door (here: an inline-const assert over a const generic),
//! and interior operations take the typed value with NO check, because the
//! type carries the invariant. After monomorphisation nothing of the check
//! survives at interior call sites.
//!
//! Reading B (66's pipeline fn): validation is a runtime predicate over the
//! bits. Every ingestion of an untyped byte pays a branch, and the branch is
//! precisely typestate residue: the check did not erase, it lowered.
//!
//! This is a QUALITATIVE existence demonstration read off emitted assembly.
//! It is an ad-hoc quick spike with no substance for any "how much" question:
//! nothing here is a bench, nothing is priced, and the cost of the branch is
//! not measured. What it establishes: `add_trusted` compiles to arithmetic
//! with no conditional branch, `add_runtime_validated` contains conditional
//! branches, on the pinned toolchain at -O. See p4_asm_grep.txt.
//!
//! Both readings are legitimate; they answer different questions (see 68
//! section on the validate fork). The point pinned here is only that they are
//! not the same verb, so a canon sentence using "validate" once must say which
//! it means, or say both and key them by boundary.

#![no_std]

#[repr(transparent)]
pub struct Num(u8);

impl Num {
    /// compile-time-validated door: the membership condition (top bit
    /// reserved, standing in for any derived validity predicate) is
    /// discharged during compilation, per constant. An out-of-range RAW is
    /// a compile error at the use site; no runtime check exists anywhere.
    pub const fn new_const<const RAW: u8>() -> Num {
        const { assert!(RAW & 0x80 == 0, "reserved pattern") };
        Num(RAW)
    }

    #[inline]
    pub const fn raw(&self) -> u8 {
        self.0
    }
}

/// interior operation over already-validated values: the invariant rides on
/// the type, so there is nothing to check here. Reading A's claim is that
/// this function is indistinguishable from the same function over bare u8.
#[unsafe(no_mangle)]
pub fn add_trusted(a: &Num, b: &Num) -> u8 {
    a.raw().wrapping_add(b.raw())
}

/// the same function over bare u8, as the erasure comparator.
#[unsafe(no_mangle)]
pub fn add_bare(a: &u8, b: &u8) -> u8 {
    a.wrapping_add(*b)
}

/// Reading B: 66's pipeline shape. Untyped bytes are validated at runtime,
/// combined only if both pass. The branches in the emitted code are the
/// typestate's runtime residue under this reading.
#[unsafe(no_mangle)]
pub fn add_runtime_validated(x: u8, y: u8) -> u8 {
    let vx = if x & 0x80 == 0 { Some(x) } else { None };
    let vy = if y & 0x80 == 0 { Some(y) } else { None };
    match (vx, vy) {
        (Some(a), Some(b)) => a.wrapping_add(b),
        _ => 0,
    }
}

/// a compile-time-validated constant reaching runtime: folds to a constant,
/// with the assert discharged during compilation and absent from the code.
#[unsafe(no_mangle)]
pub fn trusted_constant() -> u8 {
    let a = Num::new_const::<0x3A>();
    let b = Num::new_const::<0x41>();
    a.raw().wrapping_add(b.raw())
}
