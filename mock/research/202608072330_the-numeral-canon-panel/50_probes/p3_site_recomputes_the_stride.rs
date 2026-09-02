// p3: does a site holding the numeral type actually recompute the stride from a ONE-output
// derivation, and what does that cost when the strategy set opens?
//
// 48:331-336 says stride is recoverable under Reading B and cites 47_probes/p2b as compiling it.
// It does not. p2b's `stride_of::<W13, Cold>()` reads `<W13 as DeriveScalar<Cold>>::REPR`, and
// that REPR is written into the impl as `pack(16, 13, 32)`. The 13 comes OUT OF the derivation;
// the site projects it, it does not recompute it. p2b is a probe about the kind boundary and it
// is correct about that; it is not evidence for the recovery claim.
//
// So this file builds the thing that claim needs: a derivation emitting one output, and a site
// that computes the stride itself from the declaration.
//
//   ARM 1  four strategies. the site's formula works. Reading B's collapse is real.
//   ARM 2  a fifth strategy that packs to a 4-bit grid. the site's formula is WRONG, by 3 bits at
//          W=13, and the wrongness is silent: it type-checks and produces a number.
//   ARM 3  the repair, which does not remove the fact, it relocates it onto the strategy marker.
//   ARM 4  a sixth strategy whose packing is not grid-shaped. the repaired formula is wrong again.
//
// Every disagreement below is a `const _: () = assert!(...)`, so the compiler checked it and the
// binary only prints what was already proved.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p3_site_recomputes_the_stride.rs -o bin/p3 && ./bin/p3
//
// No #![feature] gate.

#![no_std]
extern crate std;
use std::println;

// ---------------------------------------------------------------- the declaration, as typestate

pub trait Width {
    const BITS: u32;
}
pub struct W13;
pub struct W12;
pub struct W16;
impl Width for W13 {
    const BITS: u32 = 13;
}
impl Width for W12 {
    const BITS: u32 = 12;
}
impl Width for W16 {
    const BITS: u32 = 16;
}

// ---------------------------------------------------------- ARM 1: one output, four strategies

/// the strategy marker carries one bit of policy, which is the least a site needs to branch.
pub trait Strategy4 {
    const PACKED: bool;
}
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;
impl Strategy4 for Hot {
    const PACKED: bool = false;
}
impl Strategy4 for Warm {
    const PACKED: bool = false;
}
impl Strategy4 for Cold {
    const PACKED: bool = true;
}
impl Strategy4 for Precise {
    const PACKED: bool = false;
}

/// ONE output. no stride anywhere in the derivation's result.
pub trait Derive1<S> {
    type Carrier: Copy;
}
impl Derive1<Hot> for W13 {
    type Carrier = u16;
}
impl Derive1<Warm> for W13 {
    type Carrier = u16;
}
impl Derive1<Cold> for W13 {
    type Carrier = u16;
}
impl Derive1<Precise> for W13 {
    type Carrier = u16;
}
impl Derive1<Cold> for W12 {
    type Carrier = u16;
}
impl Derive1<Warm> for W16 {
    type Carrier = u16;
}

/// the site's own recomputation. this is the packing rule, written at the site.
pub const fn site_stride<W, S>() -> u32
where
    W: Width + Derive1<S>,
    S: Strategy4,
{
    if S::PACKED {
        W::BITS
    } else {
        (core::mem::size_of::<<W as Derive1<S>>::Carrier>() * 8) as u32
    }
}

const _: () = assert!(site_stride::<W13, Cold>() == 13);
const _: () = assert!(site_stride::<W13, Warm>() == 16);
const _: () = assert!(site_stride::<W13, Hot>() == 16);
const _: () = assert!(site_stride::<W13, Precise>() == 16);
const _: () = assert!(site_stride::<W16, Warm>() == 16);

// ------------------------------------------------- ARM 2: a fifth strategy the formula does not know

/// a strategy that packs, but to a four-bit grid rather than to the bit. this is not exotic: it is
/// what a design picks when it wants packing AND cheap nibble-aligned addressing, and INTENTS I1
/// says the strategy set is open, so a rule that assumes otherwise is betting on a closed set.
pub struct Grid4;
impl Strategy4 for Grid4 {
    const PACKED: bool = true;
}
impl Derive1<Grid4> for W13 {
    type Carrier = u16;
}
impl Derive1<Grid4> for W12 {
    type Carrier = u16;
}

/// the truth for Grid4, which only the strategy's own rule knows.
pub const fn true_stride_grid4(w: u32) -> u32 {
    w.div_ceil(4) * 4
}

const _: () = assert!(true_stride_grid4(13) == 16);
const _: () = assert!(site_stride::<W13, Grid4>() == 13);
// the site's answer and the truth disagree, silently, by three bits per element.
const _: () = assert!(site_stride::<W13, Grid4>() != true_stride_grid4(13));
// and they agree at W=12, which is exactly the shape that makes a sampled test report green.
const _: () = assert!(site_stride::<W12, Grid4>() == true_stride_grid4(12));

// ------------------------------------------------------- ARM 3: the repair relocates the fact

/// the repair: give the strategy the grid it packs to. 0 means "does not pack".
pub trait Strategy5 {
    const GRID_BITS: u32;
}
impl Strategy5 for Hot {
    const GRID_BITS: u32 = 0;
}
impl Strategy5 for Warm {
    const GRID_BITS: u32 = 0;
}
impl Strategy5 for Precise {
    const GRID_BITS: u32 = 0;
}
impl Strategy5 for Cold {
    const GRID_BITS: u32 = 1;
}
impl Strategy5 for Grid4 {
    const GRID_BITS: u32 = 4;
}

pub const fn site_stride_v2<W, S>() -> u32
where
    W: Width + Derive1<S>,
    S: Strategy5,
{
    if S::GRID_BITS == 0 {
        (core::mem::size_of::<<W as Derive1<S>>::Carrier>() * 8) as u32
    } else {
        W::BITS.div_ceil(S::GRID_BITS) * S::GRID_BITS
    }
}

const _: () = assert!(site_stride_v2::<W13, Cold>() == 13);
const _: () = assert!(site_stride_v2::<W13, Grid4>() == 16);
const _: () = assert!(site_stride_v2::<W13, Warm>() == 16);

// ------------------------------------- ARM 4: a sixth strategy whose packing is not grid-shaped

/// a strategy that pads each element by one bit so a run can carry a per-element validity flag
/// inline. nothing about this is unreasonable and nothing about it is a grid.
pub struct Tagged;
impl Strategy4 for Tagged {
    const PACKED: bool = true;
}
impl Strategy5 for Tagged {
    const GRID_BITS: u32 = 1;
}
impl Derive1<Tagged> for W13 {
    type Carrier = u16;
}

pub const fn true_stride_tagged(w: u32) -> u32 {
    w + 1
}

const _: () = assert!(true_stride_tagged(13) == 14);
const _: () = assert!(site_stride_v2::<W13, Tagged>() == 13);
const _: () = assert!(site_stride_v2::<W13, Tagged>() != true_stride_tagged(13));

fn main() {
    println!("ARM 1  a ONE-output derivation, and the site computing the stride itself");
    println!("       (this is the construction 48:331-336 needs, and 47_probes/p2b is not it:");
    println!("        p2b projects a stride an impl wrote down, it does not recompute one)");
    println!(
        "       W13 Cold    site says {:>3}",
        site_stride::<W13, Cold>()
    );
    println!(
        "       W13 Warm    site says {:>3}",
        site_stride::<W13, Warm>()
    );
    println!("       compiles gate-free. so under Reading B, stride is recoverable and the");
    println!("       criterion demotes it. the collapse 48 reports is real.");
    println!();
    println!("ARM 2  what the recomputation actually is: the packing rule, at the site");
    println!(
        "       W13 Grid4   site says {:>3}   truth {:>3}   WRONG by {}",
        site_stride::<W13, Grid4>(),
        true_stride_grid4(13),
        true_stride_grid4(13) - site_stride::<W13, Grid4>()
    );
    println!(
        "       W12 Grid4   site says {:>3}   truth {:>3}   agrees, which is how a sampled test",
        site_stride::<W12, Grid4>(),
        true_stride_grid4(12)
    );
    println!("                                              reports green over a broken rule");
    println!();
    println!("ARM 3  the repair: put the grid on the strategy marker");
    println!(
        "       W13 Grid4   site says {:>3}   truth {:>3}   correct",
        site_stride_v2::<W13, Grid4>(),
        true_stride_grid4(13)
    );
    println!("       the fact did not disappear. it moved from the derivation's result onto the");
    println!("       strategy, where it is still a fact the derivation supplies.");
    println!();
    println!("ARM 4  a sixth strategy whose packing is not a grid");
    println!(
        "       W13 Tagged  site says {:>3}   truth {:>3}   WRONG again",
        site_stride_v2::<W13, Tagged>(),
        true_stride_tagged(13)
    );
    println!("       so the repair in ARM 3 was a bet on the shape of the next strategy, not a");
    println!("       removal of the fact. under an open strategy set (INTENTS I1) every such bet");
    println!("       is a fact the derivation owes and the site is guessing at.");
}
