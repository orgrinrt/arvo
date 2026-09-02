// p4: the property the whole two-output finding rests on is INJECTIVITY of the derivation, and
// this file checks what it takes to STATE that property at the type level.
//
// 16:126-141 establishes the finding by exhibiting a failure of injectivity: eight Cold widths
// collapse onto one carrier. Under harness-the-type-system.md and
// catalogue-edge-cases-as-tests.md, a property the design turns on should be pinned by
// something that refuses when it breaks, not only argued in prose. So: can it be pinned?
//
// Three arms.
//
//   arm A: componentwise. Assert the carriers differ, assert the strides differ. Compiles, and
//          CERTIFIES NOTHING, because the carriers do NOT differ across the eight widths; that
//          is the collapse itself. A componentwise check on a flat pair is satisfied by a
//          derivation that is not injective.
//
//   arm B: jointly, by reifying the pair into one type. Works. And the type that has to exist
//          for the assertion to be writable is the one-richer-output form.
//
//   arm C: what happens to each when a THIRD projection arrives (a compute carrier distinct
//          from the storage carrier, the Precise-widens reading). The tuple reification's
//          subject changes name and every assertion is rewritten. A named derivation result
//          does not change at all: the projection is added to its trait and the assertions
//          stand verbatim.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p4_stating_injectivity_needs_one_subject.rs -o bin/p4 && ./bin/p4
//
// The negative controls for arms B and C are in p4b, which is expected to fail to compile.
//
// No #![feature] gate.

#![no_std]
extern crate std;
use std::println;

pub trait SameType<T: ?Sized> {}
impl<T: ?Sized> SameType<T> for T {}
pub const fn assert_same<A: SameType<B> + ?Sized, B: ?Sized>() {}

// ---- the eight colliding declarations, reduced to their flat pair ----
// carrier is u16 for every one of widths 9..=16; stride is the width.

// ===================== arm A: componentwise, and it certifies nothing =====================
//
// The carrier component is IDENTICAL across all eight. So the strongest componentwise statement
// available about the carrier is an EQUALITY, and it holds:
const _: () = assert_same::<u16, u16>();
//
// Nothing here distinguishes UFixed<9,0,Cold> from UFixed<16,0,Cold>. A suite that asserted
// "the carrier is what the ladder says" for each of the eight would be eight green assertions
// over a derivation that has already lost the distinction. That is 16's own tautological
// size_of check (16:229-232) wearing a different name.

// ===================== arm B: jointly, by reifying the pair into ONE type =====================

pub struct Pair<C: Copy, const STRIDE: u32>(core::marker::PhantomData<C>);

pub type Pair9 = Pair<u16, 9>;
pub type Pair13 = Pair<u16, 13>;
pub type Pair16 = Pair<u16, 16>;

// positive: a declaration's reified result is itself
const _: () = assert_same::<Pair13, Pair<u16, 13>>();

// The distinctness assertions live in p4b, which must not compile.

// ===================== arm C: a third projection arrives =====================
//
// Reading C1: the tuple reification. The subject's NAME changes, its arity changes, and every
// site naming it is rewritten.
pub struct Triple<C: Copy, const STRIDE: u32, X: Copy>(core::marker::PhantomData<(C, X)>);
pub type Triple13Narrow = Triple<u16, 13, u16>;
pub type Triple13Wide = Triple<u16, 13, u32>;
const _: () = assert_same::<Triple13Narrow, Triple<u16, 13, u16>>();
// note: `Pair13` and `Triple13Narrow` are different types carrying the same design fact. Every
// assertion written against `Pair` is dead the moment the third projection lands.

// Reading C2: a named derivation result. The subject is a type keyed on the DECLARATION, and
// the projections hang off a trait. Adding one changes the trait, and the subject's name, its
// arity, and every assertion about it are untouched.
pub trait Width {
    const BITS: u32;
    type Native: Copy;
}
macro_rules! widths {
    ($($n:ident = $bits:literal : $native:ty;)*) => {
        $( pub struct $n; impl Width for $n { const BITS: u32 = $bits; type Native = $native; } )*
    };
}
widths! { W9 = 9 : u16; W13 = 13 : u16; W16 = 16 : u16; }

pub struct Cold;

pub struct Packed<W: Width>(core::marker::PhantomData<W>);

// The trait carries THREE projections here, where p1's carried two. Compare the two files:
// `Packed<W>` and every `assert_same` about it read identically in both.
pub trait Representation {
    type Carrier: Copy;
    type Compute: Copy;
    const STRIDE_BITS: u32;
}
impl<W: Width> Representation for Packed<W> {
    type Carrier = W::Native;
    type Compute = u32; // the Precise-widens reading, modelled as a wider compute type
    const STRIDE_BITS: u32 = W::BITS;
}
pub trait Derive<S> {
    type Repr: Representation;
}
impl<W: Width> Derive<Cold> for W {
    type Repr = Packed<W>;
}

type ReprOf<W, S> = <W as Derive<S>>::Repr;

// These three lines are character-identical to p1's, which had a two-projection trait.
const _: () = assert_same::<ReprOf<W13, Cold>, Packed<W13>>();
const _: () = assert_same::<ReprOf<W9, Cold>, Packed<W9>>();
const _: () = assert_same::<ReprOf<W16, Cold>, Packed<W16>>();

fn main() {
    println!("arm A, componentwise: `assert_same::<u16, u16>()` compiles.");
    println!("  it is true of all eight colliding widths and certifies nothing about any of them.");
    println!();
    println!("arm B, joint: the assertion needs ONE subject. Reifying the pair supplies one.");
    println!("  Pair<u16, 9> / Pair<u16, 13> / Pair<u16, 16> are three distinct types.");
    println!("  p4b asserts they are the same and is refused; that refusal is the check.");
    println!();
    println!("arm C, a third projection arrives:");
    println!("  tuple reification: Pair<C, S> becomes Triple<C, S, X>. every assertion rewritten.");
    println!("  named result:      Packed<W> is unchanged. the three assert_same lines in this");
    println!("                     file are character-identical to p1's, and p1's trait had two");
    println!("                     projections where this one has three.");
    println!();
    println!("stride of the eight, from the named result, with the compute projection present:");
    println!(
        "  W9={} W13={} W16={}   compute width = u{} in all three",
        <ReprOf<W9, Cold> as Representation>::STRIDE_BITS,
        <ReprOf<W13, Cold> as Representation>::STRIDE_BITS,
        <ReprOf<W16, Cold> as Representation>::STRIDE_BITS,
        core::mem::size_of::<<ReprOf<W13, Cold> as Representation>::Compute>() * 8
    );
}
