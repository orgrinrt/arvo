//! Where the ratified locus clause actually cuts, run rather than argued.
//!
//! `ruling::the_format_spine_is_canon` stamps
//! `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`,
//! whose sentence reads: "The representable set is a constant of the type: a value
//! set that depends on other data is not a format but storage."
//!
//! Q26's first option reads that clause as placing a platform-width type outside
//! format. This asks the clause's own criterion where it puts each of three
//! declarations, by writing each one as a `Format` and recording which compile.
//!
//! The establishing source names the class the clause excludes:
//! `08_knuth_what_the_one_format_concept_covers.md:278` heads it "The locus" and
//! lists block floating point, the shared-scale microscaling formats,
//! frame-of-reference and delta column encodings, dictionary encodings and
//! run-length encodings, with the reason at `08:283`: "no per-datum type can
//! express a constraint that holds between data." So the excluded thing is a
//! value set that depends on a **sibling datum**, and the question this probe
//! settles is whether a compilation target is one.
//!
//! Three arms. Two of them are here and build. The third is
//! `src/the_block_floating_point_element.rs`, kept out of the build, and its
//! `E0435` is committed beside it.
//!
//! **The case that must fail, stated before the run**: a block floating point
//! element cannot be a `Format`, because its quantum's exponent is a value read
//! from the block it sits in and a value is not a const generic argument. If it
//! compiled, the clause would not discriminate and this probe would establish
//! nothing.

#![no_std]

use core::mem::size_of;

use arvo_format::points::{Integer, UFixed};
use arvo_format::quantum::{Constant, Quantum};
use arvo_format::slots::Slots;
use arvo_format::{Format, Magnitude, Slot, contains};

/// Arm one: an ordinary declared width. The clause's uncontested case.
pub type AnOrdinaryFormat = UFixed<32, 0>;

/// The target's pointer width, less sixteen so every arm lands inside the widths
/// the shipped family admits on every target `cfg(target_pointer_width)` has.
///
/// The subtraction is a probe artifact and carries no meaning. What the arm is
/// about is that the value comes from the compilation rather than from a literal.
#[cfg(target_pointer_width = "16")]
pub const PLATFORM_DERIVED_BITS: u32 = 16 - 8;
#[cfg(target_pointer_width = "32")]
pub const PLATFORM_DERIVED_BITS: u32 = 32 - 8;
#[cfg(target_pointer_width = "64")]
pub const PLATFORM_DERIVED_BITS: u32 = 64 - 8;

/// Arm two: a declaration whose width comes from the compilation target.
///
/// The shape a platform-width numeral has, at a width the shipped family admits.
/// If the locus clause excluded target-derived widths, this arm is where it would
/// show, and it does not: a `cfg`-selected const is a const.
pub type ATargetDerivedFormat = UFixed<{ PLATFORM_DERIVED_BITS }, 0>;

/// Both arms are formats, and every coordinate reads back at const time.
///
/// Reading through `Format` rather than off the struct is what forces the
/// coordinates to be associated items, which is the mechanical content of "a
/// constant of the type".
pub const BOTH_ARMS_ARE_FORMATS: () = {
    assert!(<AnOrdinaryFormat as Format>::Slots::WIDTH.count() == 32);
    assert!(<ATargetDerivedFormat as Format>::Slots::WIDTH.count() == PLATFORM_DERIVED_BITS);

    assert!(<AnOrdinaryFormat as Format>::Quantum::MAGNITUDES.count() == 1);
    assert!(<ATargetDerivedFormat as Format>::Quantum::MAGNITUDES.count() == 1);

    assert!(contains::<AnOrdinaryFormat>(Slot::ZERO, Magnitude::SMALLEST).get());
    assert!(contains::<ATargetDerivedFormat>(Slot::ZERO, Magnitude::SMALLEST).get());
};

/// The target-derived arm holds no value, exactly as the literal one does.
///
/// The control that says arm two did not smuggle a datum in. A declaration that
/// carried its width as a field would be non-zero-sized, and that is what a value
/// set depending on other data would have to look like.
pub const NEITHER_ARM_CARRIES_A_DATUM: () = {
    assert!(size_of::<AnOrdinaryFormat>() == 0);
    assert!(size_of::<ATargetDerivedFormat>() == 0);
    assert!(size_of::<Integer<32>>() == 0);
};

/// The negative control on the whole discrimination, and it is the point.
///
/// A `Constant<EXP>` reached through a const is a format. The block floating
/// point arm differs from this in exactly one way: its `EXP` is a value. So this
/// arm establishes that the const route works, and the refusal beside it is
/// therefore about the value and not about the route.
pub const THE_CONST_ROUTE_WORKS: () = {
    const A_BLOCK_EXPONENT_FIXED_AT_COMPILE_TIME: i32 = -7;
    assert!(<Constant<{ A_BLOCK_EXPONENT_FIXED_AT_COMPILE_TIME }> as Quantum>::BASE.power() == -7);
};
