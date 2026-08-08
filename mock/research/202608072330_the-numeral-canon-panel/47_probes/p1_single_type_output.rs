// p1: the "one richer output" form, built, and the injectivity property stated as a
// compile-time check rather than as prose.
//
// The topic asks whether "the derivation needs a carrier and a stride" means two outputs or
// means the carrier was under-specified and one richer output suffices. This probe builds the
// one-richer-output form: the derivation has exactly ONE associated item, a TYPE, and every
// layout fact hangs off that type as a projection.
//
// Checks THREE things and nothing else:
//   1. the form compiles, gate-free, with the same arity in every strategy;
//   2. the eight Cold widths 9..=16, which collapse onto one native carrier (16:126-141),
//      have EIGHT DISTINCT single outputs, so the injectivity failure is repaired;
//   3. that distinctness is stateable and checkable AT THE TYPE LEVEL, via a SameType bridge,
//      which is the property the two-output finding rests on and which a flat (type, const)
//      pair cannot be the subject of, because a pair of a type and a const is not a type.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p1_single_type_output.rs -o bin/p1 && ./bin/p1
//
// SCAFFOLDING FLAG, read before citing this file. The width-to-native ladder is one impl per
// width, which the design has ratified against ("no enumerations"). That is scaffolding copied
// in spirit from 16_probes/p6, which carries the same flag, and it is not a proposal. What this
// probe checks is what sits ON TOP of a ladder: whether the derivation's result can be a single
// named type, and whether injectivity of that result is expressible. The `Derive` impls and the
// SameType checks are the part to read; the `widths!` macro is scaffolding to reach them.
//
// No #![feature] gate is enabled anywhere. That absence is load-bearing.

#![no_std]
extern crate std;
use std::println;

// ---- strategies ----
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;

// ---- a declared width, carrying its native rung and its packed access rung. SCAFFOLDING. ----
pub trait Width {
    const BITS: u32;
    /// smallest native container holding BITS
    type Native: Copy;
    /// smallest native container covering a BITS-wide field at an arbitrary bit phase,
    /// i.e. floor((BITS+6)/8)+1 bytes rounded up to a power of two
    type Access: Copy;
}

macro_rules! widths {
    ($($n:ident = $bits:literal : $native:ty , $access:ty ;)*) => {
        $( pub struct $n; impl Width for $n {
            const BITS: u32 = $bits;
            type Native = $native;
            type Access = $access;
        } )*
    };
}
widths! {
    W5  = 5  : u8,  u16;
    W9  = 9  : u16, u32;
    W10 = 10 : u16, u32;
    W11 = 11 : u16, u32;
    W12 = 12 : u16, u32;
    W13 = 13 : u16, u32;
    W14 = 14 : u16, u32;
    W15 = 15 : u16, u32;
    W16 = 16 : u16, u32;
    W31 = 31 : u32, u64;
    W47 = 47 : u64, u64;
}

// ---- the richer output itself: a TYPE, one per (packing discipline, width) ----
//
// This is the whole proposal under test. `Padded<W>` and `Packed<W>` are the derivation's
// codomain. Neither is a machine type; each PROJECTS one.
pub struct Padded<W: Width>(core::marker::PhantomData<W>);
pub struct Packed<W: Width>(core::marker::PhantomData<W>);

pub trait Representation {
    /// what an operation lowers to
    type Carrier: Copy;
    /// what a load covering one element at an arbitrary phase must be
    type Access: Copy;
    /// bits between consecutive elements of an aggregate
    const STRIDE_BITS: u32;
    /// bits one value actually occupies
    const WIDTH_BITS: u32;
}

impl<W: Width> Representation for Padded<W> {
    type Carrier = W::Native;
    type Access = W::Native;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Native>() * 8) as u32;
    const WIDTH_BITS: u32 = W::BITS;
}

impl<W: Width> Representation for Packed<W> {
    type Carrier = W::Native;
    type Access = W::Access;
    const STRIDE_BITS: u32 = W::BITS;
    const WIDTH_BITS: u32 = W::BITS;
}

// ---- the derivation: ONE associated item, in every strategy ----
pub trait Derive<S> {
    type Repr: Representation;
}

impl<W: Width> Derive<Warm> for W {
    type Repr = Padded<W>;
}
impl<W: Width> Derive<Hot> for W {
    type Repr = Padded<W>;
}
impl<W: Width> Derive<Precise> for W {
    type Repr = Padded<W>;
}
impl<W: Width> Derive<Cold> for W {
    type Repr = Packed<W>;
}

// ---- the type-level distinctness bridge ----
//
// `SameType<T>` holds exactly when Self and T are the same type. A bound `A: SameType<B>` is
// therefore a compile-time assertion of type equality, and its ABSENCE is a compile-time
// assertion of distinctness once the negative control is run (see p1b).
pub trait SameType<T: ?Sized> {}
impl<T: ?Sized> SameType<T> for T {}

pub const fn assert_same<A: SameType<B> + ?Sized, B: ?Sized>() {}

type ReprOf<W, S> = <W as Derive<S>>::Repr;

// POSITIVE controls: a declaration's single output is the same type as itself, and Cold and
// Warm at the SAME width are genuinely different single outputs.
const _: () = assert_same::<ReprOf<W13, Cold>, Packed<W13>>();
const _: () = assert_same::<ReprOf<W13, Warm>, Padded<W13>>();
const _: () = assert_same::<ReprOf<W13, Hot>, ReprOf<W13, Precise>>();

// The carrier, by contrast, IS the same across all four at W=13, which is 16's collapse.
const _: () = assert_same::<
    <ReprOf<W13, Cold> as Representation>::Carrier,
    <ReprOf<W13, Warm> as Representation>::Carrier,
>();

fn main() {
    println!("one output, a TYPE, per (strategy, width). every layout fact is a projection.");
    println!();
    println!("declaration            single output   carrier  access  stride  bytes/1e6");

    macro_rules! row {
        ($label:literal, $rep:literal, $w:ty, $s:ty) => {{
            type R = <$w as Derive<$s>>::Repr;
            const C: usize = core::mem::size_of::<<R as Representation>::Carrier>() * 8;
            const A: usize = core::mem::size_of::<<R as Representation>::Access>() * 8;
            const S: u32 = <R as Representation>::STRIDE_BITS;
            println!(
                "{:<22} {:<15} u{:<7} u{:<6} {:<7} {}",
                $label,
                $rep,
                C,
                A,
                S,
                (S as u64 * 1_000_000).div_ceil(8)
            );
        }};
    }
    row!("UFixed<13,0,Warm>", "Padded<W13>", W13, Warm);
    row!("UFixed<13,0,Cold>", "Packed<W13>", W13, Cold);
    row!("UFixed<16,0,Cold>", "Packed<W16>", W16, Cold);
    row!("UFixed<5,0,Cold>", "Packed<W5>", W5, Cold);
    row!("UFixed<31,0,Cold>", "Packed<W31>", W31, Cold);
    row!("UFixed<47,0,Cold>", "Packed<W47>", W47, Cold);

    println!();
    println!("the collapse 16:126-141 names, and its repair under one output:");
    println!("  width  carrier(collapses)  single output(does not)");
    macro_rules! coldrow {
        ($w:ty, $rep:literal) => {{
            type R = <$w as Derive<Cold>>::Repr;
            const C: usize = core::mem::size_of::<<R as Representation>::Carrier>() * 8;
            const S: u32 = <R as Representation>::STRIDE_BITS;
            println!("  {:<6} u{:<18} {:<12} stride={}", <$w>::BITS, C, $rep, S);
        }};
    }
    coldrow!(W9, "Packed<W9>");
    coldrow!(W10, "Packed<W10>");
    coldrow!(W11, "Packed<W11>");
    coldrow!(W12, "Packed<W12>");
    coldrow!(W13, "Packed<W13>");
    coldrow!(W14, "Packed<W14>");
    coldrow!(W15, "Packed<W15>");
    coldrow!(W16, "Packed<W16>");

    println!();
    println!("eight declarations, one carrier, eight distinct single outputs.");
    println!("the const _ blocks above are compile-time type-equality assertions; the file");
    println!("compiled, so they hold. distinctness is pinned by the negative control in p1b.");
}
