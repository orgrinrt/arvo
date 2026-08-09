// Probe 4b, file 80. The refusing half: a safe impl of `unsafe trait Crosses` is
// refused with E0200. This is the compile-fail artifact pinning the marking fact
// probe_4's header states; if a later edit downgrades `Crosses` to a safe trait,
// this file starts compiling and the refusal this probe documents is gone.
//
// Build: rustc --edition 2021 --crate-type=lib --emit=metadata.
// EXPECTED: error[E0200]: the trait `Crosses<SomeNumeral>` requires an `unsafe impl`
// declaration.
#![no_std]

pub trait Numeral {}
pub trait Lowering {}
pub unsafe trait Crosses<N: Numeral>: Lowering {}

pub struct SomeNumeral;
impl Numeral for SomeNumeral {}

pub struct GeneratedLowering;
impl Lowering for GeneratedLowering {}

// "the impl is blanket and safe" spelled literally:
impl Crosses<SomeNumeral> for GeneratedLowering {}
