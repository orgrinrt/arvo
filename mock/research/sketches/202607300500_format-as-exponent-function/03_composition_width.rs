// Probe 03: does the full composition put arithmetic back into type position?
//
// Probes 01 and 02 showed fexp itself is safe, because it applies to a value's
// magnitude exponent. The remaining risk is elsewhere: a fixed-point format with
// I integer bits and F fractional bits needs a container of I + F bits, and
// `Bits<{I + F}, S>` is a const expression in type position, which is exactly the
// forbidden shape.
//
// Part A confirms the naive form is refused (it is commented out; uncommenting it
// reproduces the error quoted in FINDINGS).
// Part B tests the cure: the format carries its width as an ASSOCIATED TYPE, so
// the container comes from the existing projection and nothing computes in type
// position.
//
// Run: rustc --edition 2021 03_composition_width.rs -o /tmp/p03 && /tmp/p03

#![feature(const_trait_impl)]

use core::marker::PhantomData;

// --- width as typestate, mirroring Dim<N> ---------------------------------

pub trait Width {
    const BITS: u16;
}

pub struct W<const N: u16>;

impl<const N: u16> Width for W<N> {
    const BITS: u16 = N;
}

// --- the container projection, standing in for the migrated one -----------

pub trait Container {
    type T;
}

pub struct Pick<Wd>(PhantomData<Wd>);

impl Container for Pick<W<8>> {
    type T = u8;
}
impl Container for Pick<W<16>> {
    type T = u16;
}
impl Container for Pick<W<32>> {
    type T = u32;
}
impl Container for Pick<W<64>> {
    type T = u64;
}

// --- formats, each naming its own total width as an associated TYPE -------

pub const trait Underflow {
    fn fexp(e: i32, prec: i32, emin: i32) -> i32;
}

pub struct Unbounded;
pub struct Gradual;

const impl Underflow for Unbounded {
    fn fexp(e: i32, prec: i32, _emin: i32) -> i32 {
        e - prec
    }
}
const impl Underflow for Gradual {
    fn fexp(e: i32, prec: i32, emin: i32) -> i32 {
        let x = e - prec;
        if x < emin {
            emin
        } else {
            x
        }
    }
}

pub const trait Format {
    /// Total container width, as a TYPE rather than a const expression. This is
    /// the whole trick: the arithmetic that would be `{I + F}` is discharged by
    /// whoever writes the impl, in value position or by naming the answer.
    type Total: Width;

    fn fexp(e: i32) -> i32;
}

/// FIX with I integer bits and F fractional bits. `Total` is named, not computed
/// in type position.
pub struct Fixed<const I: i32, const F: i32, Total>(PhantomData<Total>);

const impl<const I: i32, const F: i32, Total: Width> Format for Fixed<I, F, Total> {
    type Total = Total;
    fn fexp(_e: i32) -> i32 {
        -F
    }
}

pub struct Floating<const PREC: i32, const EMIN: i32, U, Total>(PhantomData<(U, Total)>);

const impl<const PREC: i32, const EMIN: i32, U: [const] Underflow, Total: Width> Format
    for Floating<PREC, EMIN, U, Total>
{
    type Total = Total;
    fn fexp(e: i32) -> i32 {
        U::fexp(e, PREC, EMIN)
    }
}

// --- the composition, six axes, resolving to a concrete container ---------

pub struct Wrap;
pub struct Saturate;
pub struct Trunc;
pub struct FullPrecision;

/// A concrete numeric type is a composition. Nothing here computes in type
/// position; every parameter is a type or a plain const.
pub struct Num<Fmt, Sign, Round, Over, Grow>(PhantomData<(Fmt, Sign, Round, Over, Grow)>);

pub struct Unsigned;

/// The container falls out of the format's associated width via the projection,
/// with no arithmetic anywhere in a type argument.
type ContainerOf<Fmt> = <Pick<<Fmt as Format>::Total> as Container>::T;

fn width_of<F: Format>() -> u16 {
    <F::Total as Width>::BITS
}

fn size_of_container<F: Format>() -> usize
where
    Pick<F::Total>: Container,
{
    core::mem::size_of::<ContainerOf<F>>()
}

fn main() {
    // A UFixed<13, 3>-shaped format: 16 bits total, named rather than computed.
    type U13_3 = Fixed<13, 3, W<16>>;
    type Q = Num<U13_3, Unsigned, Trunc, Wrap, FullPrecision>;
    let _: Option<Q> = None;

    assert_eq!(width_of::<U13_3>(), 16);
    assert_eq!(size_of_container::<U13_3>(), 2);
    assert_eq!(<U13_3 as Format>::fexp(0), -3);
    assert_eq!(<U13_3 as Format>::fexp(99), -3);

    // A binary32-shaped floating format over the same machinery.
    type F32ish = Floating<24, -149, Gradual, W<32>>;
    assert_eq!(width_of::<F32ish>(), 32);
    assert_eq!(size_of_container::<F32ish>(), 4);
    assert_eq!(<F32ish as Format>::fexp(0), -24);
    assert_eq!(<F32ish as Format>::fexp(-200), -149);

    // And an unbounded-exponent format, which IEEE 754 does not standardise.
    type Flx = Floating<24, -126, Unbounded, W<32>>;
    assert_eq!(<Flx as Format>::fexp(-1000), -1024);

    println!("03 WORKS: six-axis composition, container resolved, no type-position arithmetic");
}

// --- Part A, kept for the record ------------------------------------------
//
// The naive form, which is what ships today inside UFixed:
//
//     pub struct Naive<const I: u16, const F: u16>(Bits<{ I + F }>);
//
// Uncommenting reproduces:
//     error: generic parameters may not be used in const operations
//     = help: const parameters may only be used as standalone arguments here
// which is the shape D1 of 202607282100 forbids the gate for.
