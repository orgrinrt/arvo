// p6: attacking p5b's refusal rather than reporting it.
//
// p5b showed that recovering the carrier from a const extent needs generic_const_exprs, which
// is forbidden. The workspace rule for a refused bound is that it wants a trait, not a feature.
// So: build the trait form and see what it gives.
//
// Checks ONE thing: whether a single trait can carry BOTH components gate-free, with the same
// arity across strategies, and whether every layout quantity a consumer needs falls out of the
// pair. That is the arity question, which is what this dispatch is about.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p6_trait_form_recovers_both.rs -o bin/p6 && ./bin/p6
//
// SCAFFOLDING FLAG, read before citing this file. The width-to-native-carrier ladder here is
// written as one impl per width, which the design has RATIFIED against ("no enumerations",
// SETTLED.md:97 and :110). That is not a proposal and it is not this probe's subject. The
// non-enumerating ladder is the closed panel's `137` result and I am not re-deriving it. What
// this probe checks is what sits ON TOP of a ladder: whether one blanket impl can emit two
// components at once. The blanket impls over `W: Width` are the part to read.
//
// A first version of this file tried to select rungs by a where-clause bound and was refused
// with E0119, conflicting implementations, because where-clauses do not disambiguate impls.
// That refusal is recorded in p6_first_attempt.err and is a real result about rung selection:
// the rung has to be in the type, not in a bound.
//
// No #![feature] gate is enabled. That absence is load-bearing.

#![no_std]
extern crate std;
use std::println;

// ---- the strategies ----
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;

// ---- a declared width, carrying its own native rung. SCAFFOLDING: see the flag above. ----
pub trait Width {
    const BITS: u32;
    type Native: Copy;
}

macro_rules! widths {
    ($($n:ident = $bits:literal : $native:ty;)*) => {
        $( pub struct $n; impl Width for $n { const BITS: u32 = $bits; type Native = $native; } )*
    };
}
widths! {
    W5  = 5  : u8;
    W8  = 8  : u8;
    W13 = 13 : u16;
    W16 = 16 : u16;
    W31 = 31 : u32;
    W47 = 47 : u64;
}

// ---- the derivation: ONE trait, TWO associated items. this is the shape under test. ----
pub trait Derive<S> {
    /// output 1: the machine type an operation lowers to.
    type Carrier: Copy;
    /// output 2: what one value occupies, and at what stride consecutive values repeat.
    const EXTENT_BITS: u32;
    const STRIDE_BITS: u32;
}

// One blanket impl per strategy. Same arity in every one; only the values differ.
impl<W: Width> Derive<Warm> for W {
    type Carrier = W::Native;
    const EXTENT_BITS: u32 = W::BITS;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Native>() * 8) as u32;
}
impl<W: Width> Derive<Hot> for W {
    type Carrier = W::Native;
    const EXTENT_BITS: u32 = W::BITS;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Native>() * 8) as u32;
}
impl<W: Width> Derive<Precise> for W {
    type Carrier = W::Native;
    const EXTENT_BITS: u32 = W::BITS;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Native>() * 8) as u32;
}
impl<W: Width> Derive<Cold> for W {
    type Carrier = W::Native; // compute still happens in a machine type
    const EXTENT_BITS: u32 = W::BITS;
    const STRIDE_BITS: u32 = W::BITS; // packed: the stride IS the width
}

// ---- what a downstream site computes from the pair, with no further derivation ----
pub const fn access_bytes(extent_bits: u32) -> u32 {
    (extent_bits + 6) / 8 + 1
}
pub const fn bytes_for(stride_bits: u32, n: u64) -> u64 {
    (stride_bits as u64 * n).div_ceil(8)
}
pub const fn phase_of(stride_bits: u32, k: u64) -> u32 {
    ((stride_bits as u64 * k) % 8) as u32
}

fn main() {
    println!("one trait, two associated items, no feature gates.");
    println!();
    println!("declaration          carrier  extent  stride  access  bytes/1e6   phase of elem 3");
    macro_rules! row {
        ($label:literal, $w:ty, $s:ty) => {{
            const C: usize = core::mem::size_of::<<$w as Derive<$s>>::Carrier>() * 8;
            const E: u32 = <$w as Derive<$s>>::EXTENT_BITS;
            const S: u32 = <$w as Derive<$s>>::STRIDE_BITS;
            println!(
                "{:<20} u{:<6} {:<7} {:<7} {:<7} {:<11} {}",
                $label,
                C,
                E,
                S,
                access_bytes(E),
                bytes_for(S, 1_000_000),
                phase_of(S, 3)
            );
        }};
    }
    row!("UFixed<5,0,Warm>", W5, Warm);
    row!("UFixed<5,0,Cold>", W5, Cold);
    row!("UFixed<8,0,Cold>", W8, Cold);
    row!("UFixed<13,0,Warm>", W13, Warm);
    row!("UFixed<13,0,Cold>", W13, Cold);
    row!("UFixed<16,0,Cold>", W16, Cold);
    row!("UFixed<31,0,Cold>", W31, Cold);
    row!("UFixed<47,0,Cold>", W47, Cold);

    println!();
    println!("the 13-bit Warm row and the 13-bit Cold row share a carrier and differ in stride.");
    println!("the 13-bit Cold row and the 16-bit Cold row share a carrier and differ in stride.");
    println!("a derivation returning only the first column cannot tell any of them apart.");
    println!();
    println!("every remaining column is computed from the pair by a plain const fn, so no");
    println!("third component is needed for the layout quantities I was able to name.");
}
