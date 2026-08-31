//! p2: is "the strategy selects a ladder" a different design from "the strategy
//! keys the ladder", or two spellings of one function?
//!
//! Seat 226. The register records a standing disagreement inside the
//! container-derivation material: whether the strategy is an upstream selector
//! the ladder never sees, or a key of the ladder itself. This compiles both,
//! independently, and asks the compiler whether they resolve to the same
//! carrier.
//!
//!   order A   strategy -> objective, then (width, objective) -> carrier
//!   order B   (width, strategy) -> carrier
//!
//! **The two tables are written out separately, by hand, on purpose.** Generating
//! both from one macro invocation would make the agreement true by construction
//! and the probe would prove nothing; that is the setup-that-helps failure. Each
//! table states its own carriers, and a transcription slip in either shows up as
//! a refusal. p2b and p2c are that file with one cell changed, and both must
//! refuse.
//!
//! Under arvo's constraints: `#![no_std]`, no `alloc`, no `dyn`, no `TypeId`, no
//! feature gate. `rustc --edition 2024 --crate-type=lib`, so a clean exit is the
//! whole result. Selection is by monomorphisation and nothing here runs.
//!
//! **This is a spike and its enumeration is a shortcut.** Ten widths are written
//! out where a design would key structurally; the claim under test is whether
//! the two currying orders agree, which does not depend on the ladder being
//! total. Nothing here says a design should enumerate.

#![no_std]

pub struct W<const N: usize>;

// The three placement objectives, which are what the ladder is really keyed on.
pub struct Packed; // the machine type one packed-column load must touch
pub struct Standalone; // the smallest native type holding a lone value
pub struct Widened; // the next native type above Standalone

// The strategies, as names bound to objectives.
pub struct Cold;
pub struct Warm;
pub struct Hot;

// ---- order A: the strategy selects an objective; the ladder never sees it ----

pub trait Selects {
    type Obj;
}
impl Selects for Cold {
    type Obj = Packed;
}
impl Selects for Warm {
    type Obj = Standalone;
}
impl Selects for Hot {
    type Obj = Widened;
}

pub trait LadderFor<O> {
    type Carrier;
}
pub type OrderA<Wd, S> = <Wd as LadderFor<<S as Selects>::Obj>>::Carrier;

macro_rules! table_a {
    ($($w:literal => $packed:ty, $standalone:ty, $widened:ty;)*) => {$(
        impl LadderFor<Packed>     for W<$w> { type Carrier = $packed; }
        impl LadderFor<Standalone> for W<$w> { type Carrier = $standalone; }
        impl LadderFor<Widened>    for W<$w> { type Carrier = $widened; }
    )*};
}

table_a! {
     3 => u16, u8,  u16;
     4 => u8,  u8,  u16;
     8 => u8,  u8,  u16;
     9 => u16, u16, u32;
    13 => u32, u16, u32;
    16 => u16, u16, u32;
    17 => u32, u32, u64;
    32 => u32, u32, u64;
    33 => u64, u64, u128;
    64 => u64, u64, u128;
}

// ---- order B: one ladder keyed on the width and the strategy together ----

pub trait Ladder<S> {
    type Carrier;
}
pub type OrderB<Wd, S> = <Wd as Ladder<S>>::Carrier;

macro_rules! table_b {
    ($($w:literal => $cold:ty, $warm:ty, $hot:ty;)*) => {$(
        impl Ladder<Cold> for W<$w> { type Carrier = $cold; }
        impl Ladder<Warm> for W<$w> { type Carrier = $warm; }
        impl Ladder<Hot>  for W<$w> { type Carrier = $hot; }
    )*};
}

// Written out again rather than reused. The values are the same because the
// function is the same, which is the thing being tested.
table_b! {
     3 => u16, u8,  u16;
     4 => u8,  u8,  u16;
     8 => u8,  u8,  u16;
     9 => u16, u16, u32;
    13 => u32, u16, u32;
    16 => u16, u16, u32;
    17 => u32, u32, u64;
    32 => u32, u32, u64;
    33 => u64, u64, u128;
    64 => u64, u64, u128;
}

// ---- the question ----

pub trait Same<T> {}
impl<T> Same<T> for T {}
pub fn same<A: Same<B>, B>() {}

macro_rules! agree {
    ($($w:literal),*) => {$(
        same::<OrderA<W<$w>, Cold>, OrderB<W<$w>, Cold>>();
        same::<OrderA<W<$w>, Warm>, OrderB<W<$w>, Warm>>();
        same::<OrderA<W<$w>, Hot>,  OrderB<W<$w>, Hot>>();
    )*};
}

pub fn the_two_orders_agree() {
    agree!(3, 4, 8, 9, 13, 16, 17, 32, 33, 64);
}

/// The positive control that must NOT refuse, in the same compile: the three
/// objectives are genuinely distinguishable, so a file where every carrier were
/// the same type would pass `the_two_orders_agree` vacuously.
pub fn the_objectives_are_distinguishable() {
    // At W = 3 the three objectives give three shapes, two of which coincide.
    same::<OrderA<W<3>, Cold>, u16>();
    same::<OrderA<W<3>, Warm>, u8>();
    same::<OrderA<W<3>, Hot>, u16>();
    // At W = 4 Cold and Warm coincide and Hot does not, which is the other
    // pattern, so the table is not one column wearing three names.
    same::<OrderA<W<4>, Cold>, u8>();
    same::<OrderA<W<4>, Hot>, u16>();
}
