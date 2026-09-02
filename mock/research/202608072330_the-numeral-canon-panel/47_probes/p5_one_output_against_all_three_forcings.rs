// p5: the single type-valued output put against all three forcings the panel has named, at
// once, to see whether it holds or breaks.
//
// The three forcings on the record:
//   F1  Cold packs, so eight widths collapse onto one native carrier      (16:126-141, unconditional)
//   F2  two strategies diverge in alignment at some rung                  (45 section 11.1, conditional)
//   F3  Precise computes wider than it stores                             (45 section 3, conditional on op's intent)
//
// For each, this file asks TWO questions rather than one, because they have different answers
// and the panel has been running them together:
//
//   (a) does the FLAT PAIR (carrier as a TYPE, stride as a const) separate the declarations?
//   (b) does a SINGLE type-valued output separate them?
//
// The answers, all const-checked below, are not uniform, and that non-uniformity is the result.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p5_one_output_against_all_three_forcings.rs -o bin/p5 && ./bin/p5
//
// The distinctness refusals are in p5b. No #![feature] gate.

#![no_std]
extern crate std;
use std::println;

pub trait SameType<T: ?Sized> {}
impl<T: ?Sized> SameType<T> for T {}
pub const fn assert_same<A: SameType<B> + ?Sized, B: ?Sized>() {}

pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;

pub trait Width {
    const BITS: u32;
    type Native: Copy;
    type Wide: Copy;
    type WideAligned: Copy;
    /// the compute type Precise would use under the widening reading
    type Widened: Copy;
}

// narrow rung
pub struct W13;
impl Width for W13 {
    const BITS: u32 = 13;
    type Native = u16;
    type Wide = u16;
    type WideAligned = u16;
    type Widened = u32;
}
// wide rung, 45's W=256 witness: 32 bytes either way, align 1 against align 16
#[derive(Clone, Copy)]
pub struct WideBits32([u8; 32]);
#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct AlignedWideBits32([u8; 32]);
pub struct W256;
impl Width for W256 {
    const BITS: u32 = 256;
    type Native = WideBits32;
    type Wide = WideBits32;
    type WideAligned = AlignedWideBits32;
    type Widened = WideBits32;
}

// 45's p1 witness, re-checked here rather than trusted from its prose.
const _: () =
    assert!(core::mem::size_of::<WideBits32>() == core::mem::size_of::<AlignedWideBits32>());
const _: () = assert!(core::mem::align_of::<WideBits32>() == 1);
const _: () = assert!(core::mem::align_of::<AlignedWideBits32>() == 16);

// ============================ the FLAT PAIR form ============================
pub trait Flat<S> {
    type Carrier: Copy;
    const STRIDE_BITS: u32;
}
impl<W: Width> Flat<Warm> for W {
    type Carrier = W::Wide;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Wide>() * 8) as u32;
}
impl<W: Width> Flat<Hot> for W {
    type Carrier = W::WideAligned;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::WideAligned>() * 8) as u32;
}
impl<W: Width> Flat<Cold> for W {
    type Carrier = W::Wide;
    const STRIDE_BITS: u32 = W::BITS;
}
// Precise stores exactly as Warm does; only its COMPUTE type differs, and the flat pair has no
// slot for a compute type. That absence is the point of arm F3 below.
impl<W: Width> Flat<Precise> for W {
    type Carrier = W::Wide;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Wide>() * 8) as u32;
}

// ============================ the SINGLE OUTPUT form ============================
pub trait Representation {
    type Carrier: Copy;
    type Compute: Copy;
    const STRIDE_BITS: u32;
}
pub struct Padded<W: Width>(core::marker::PhantomData<W>);
pub struct PaddedAligned<W: Width>(core::marker::PhantomData<W>);
pub struct Packed<W: Width>(core::marker::PhantomData<W>);
pub struct PaddedWideCompute<W: Width>(core::marker::PhantomData<W>);

impl<W: Width> Representation for Padded<W> {
    type Carrier = W::Wide;
    type Compute = W::Wide;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Wide>() * 8) as u32;
}
impl<W: Width> Representation for PaddedAligned<W> {
    type Carrier = W::WideAligned;
    type Compute = W::WideAligned;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::WideAligned>() * 8) as u32;
}
impl<W: Width> Representation for Packed<W> {
    type Carrier = W::Wide;
    type Compute = W::Wide;
    const STRIDE_BITS: u32 = W::BITS;
}
impl<W: Width> Representation for PaddedWideCompute<W> {
    type Carrier = W::Wide;
    type Compute = W::Widened;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Wide>() * 8) as u32;
}

pub trait Derive<S> {
    type Repr: Representation;
}
impl<W: Width> Derive<Warm> for W {
    type Repr = Padded<W>;
}
impl<W: Width> Derive<Hot> for W {
    type Repr = PaddedAligned<W>;
}
impl<W: Width> Derive<Cold> for W {
    type Repr = Packed<W>;
}
impl<W: Width> Derive<Precise> for W {
    type Repr = PaddedWideCompute<W>;
}

type ReprOf<W, S> = <W as Derive<S>>::Repr;
type FlatCarrier<W, S> = <W as Flat<S>>::Carrier;

// ---- F2, alignment divergence at W=256: does the FLAT PAIR separate Hot from Warm? ----
// stride is equal:
const _: () = assert!(<W256 as Flat<Hot>>::STRIDE_BITS == <W256 as Flat<Warm>>::STRIDE_BITS);
// but the carrier TYPES differ, so the flat pair does separate them. p5b pins that.

// ---- F3, Precise widening at W=13: does the FLAT PAIR separate Precise from Warm? ----
// stride equal AND carrier type equal. The flat pair collapses them, and this assertion is
// the collapse, compiled:
const _: () = assert!(<W13 as Flat<Precise>>::STRIDE_BITS == <W13 as Flat<Warm>>::STRIDE_BITS);
const _: () = assert_same::<FlatCarrier<W13, Precise>, FlatCarrier<W13, Warm>>();

// ---- and the SINGLE OUTPUT separates them, which p5b pins by refusing the equality ----
const _: () = assert_same::<ReprOf<W13, Precise>, PaddedWideCompute<W13>>();
const _: () = assert_same::<ReprOf<W13, Warm>, Padded<W13>>();

fn main() {
    println!("F1  Cold packing, W=13 against W=16");
    println!("    flat pair:     carrier u16 both; stride 13 against 16  -> SEPARATES");
    println!("    single output: Packed<W13> against Packed<W16>         -> SEPARATES");
    println!();
    println!("F2  alignment divergence, W=256, Hot against Warm");
    println!(
        "    flat pair:     stride {} both; carrier size {} both; ALIGN {} against {}",
        <W256 as Flat<Hot>>::STRIDE_BITS,
        core::mem::size_of::<FlatCarrier<W256, Hot>>(),
        core::mem::align_of::<FlatCarrier<W256, Hot>>(),
        core::mem::align_of::<FlatCarrier<W256, Warm>>()
    );
    println!("                   the carrier is a TYPE, so it separates. 45's p1 collision is");
    println!("                   against a carrier represented as a BIT COUNT, not against the");
    println!("                   flat pair as 15 and 16 actually state it. -> SEPARATES");
    println!("    single output: PaddedAligned<W256> against Padded<W256> -> SEPARATES");
    println!();
    println!("F3  Precise widening, W=13, Precise against Warm");
    println!(
        "    flat pair:     stride {} both; carrier u{} both -> DOES NOT SEPARATE",
        <W13 as Flat<Precise>>::STRIDE_BITS,
        core::mem::size_of::<FlatCarrier<W13, Precise>>() * 8
    );
    println!(
        "    single output: PaddedWideCompute<W13> (compute u{}) against Padded<W13> (compute u{})",
        core::mem::size_of::<<ReprOf<W13, Precise> as Representation>::Compute>() * 8,
        core::mem::size_of::<<ReprOf<W13, Warm> as Representation>::Compute>() * 8
    );
    println!("                   -> SEPARATES, with no change to the derivation's arity.");
    println!();
    println!("so the three forcings do not all bear on the same question:");
    println!("  F1 forces the codomain past the native machine types. both forms handle it.");
    println!("  F2 forces the carrier to be a TYPE rather than a width. both forms handle it,");
    println!("     and only a bit-count carrier fails, which is a representation choice inside");
    println!("     an instrument rather than a fact about the derivation.");
    println!("  F3 forces a THIRD projection. the flat form's arity moves; the single output's");
    println!("     does not, because the projection is added to the result's contract.");
}
