// Probe 02: the same format concept as a `const trait`, callable in const context.
//
// Probe 01 established the shape works with ordinary traits. arvo's contract
// surface is `pub const trait` throughout, and `FromConstant` plus the identity
// machinery are const-callable, so the format has to be too or it cannot sit
// under them.
//
// `const_trait_impl` is gated but ALLOWED (WATCH tier, tracking #143874), and
// arvo already gates it in every contracts crate.
//
// Run: rustc --edition 2021 02_const_trait.rs -o /tmp/p02 && /tmp/p02

#![feature(const_trait_impl)]

use core::marker::PhantomData;

pub const trait Underflow {
    fn fexp(e: i32, prec: i32, emin: i32) -> i32;
}

pub struct Unbounded;
pub struct Gradual;
pub struct Flushed;

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

const impl Underflow for Flushed {
    fn fexp(e: i32, prec: i32, emin: i32) -> i32 {
        let x = e - prec;
        if x < emin {
            emin + prec - 1
        } else {
            x
        }
    }
}

pub const trait Format {
    fn fexp(e: i32) -> i32;
}

pub struct Fixed<const F: i32>;
pub struct Floating<const PREC: i32, const EMIN: i32, U>(PhantomData<U>);

const impl<const F: i32> Format for Fixed<F> {
    fn fexp(_e: i32) -> i32 {
        -F
    }
}

// The interesting one: a const impl that defers to another const trait through
// a generic parameter. This is where a const-trait chain either resolves or does
// not, and it is the shape the whole composition depends on.
const impl<const PREC: i32, const EMIN: i32, U: [const] Underflow> Format
    for Floating<PREC, EMIN, U>
{
    fn fexp(e: i32) -> i32 {
        U::fexp(e, PREC, EMIN)
    }
}

// --- const evaluation, the actual claim being tested -----------------------

const FIX_AT_ZERO: i32 = <Fixed<16> as Format>::fexp(0);
const FIX_AT_HUNDRED: i32 = <Fixed<16> as Format>::fexp(100);

type Flx = Floating<24, -126, Unbounded>;
type Flt = Floating<24, -149, Gradual>;
type Ftz = Floating<24, -149, Flushed>;

const FLX_LOW: i32 = <Flx as Format>::fexp(-1000);
const FLT_LOW: i32 = <Flt as Format>::fexp(-200);
const FTZ_LOW: i32 = <Ftz as Format>::fexp(-200);

// A const fn generic over the format, threading the bound through its own
// generic code rather than naming a concrete type.
const fn canonical<F: [const] Format>(mag: i32) -> i32 {
    F::fexp(mag)
}

const THREADED: i32 = canonical::<Flt>(-200);

fn main() {
    assert_eq!(FIX_AT_ZERO, -16);
    assert_eq!(FIX_AT_HUNDRED, -16);
    assert_eq!(FLX_LOW, -1024);
    assert_eq!(FLT_LOW, -149);
    assert_eq!(FTZ_LOW, -149 + 24 - 1);
    assert_eq!(THREADED, -149);

    println!("02 WORKS: const trait, const-evaluated, one allowed gate");
}
