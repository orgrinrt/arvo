// p3b: the two routes that DO reach the packed access type, and what each costs.
//
// p3 closes one route: arithmetic on a const-carried width, refused in three positions. This
// file builds the two that are open, so the finding is a composition rather than a wall.
//
//   route A: keep the width as a const, drop the arithmetic, and key a trait DIRECTLY on the
//            width. Legal, because a bare const parameter is a standalone argument. The cost is
//            one impl PER WIDTH, which is the enumeration the design has ratified against
//            (quoted at 16:485-488, "SETTLED.md:97 and :110, no enumerations, refused four
//            times"). So this route is closed by design, not by the compiler, and it is worth
//            knowing which of the two closed it.
//
//   route B: carry the width as a TYPE with the access rung as an associated type. Legal, no
//            arithmetic in type position, and no impl count beyond the width ladder the design
//            already has. This is the same move a-refused-bound-wants-a-trait-not-a-feature.md
//            names, applied to the quantity 16 dismissed.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p3b_access_type_two_routes_that_work.rs -o bin/p3b && ./bin/p3b
//
// No #![feature] gate.

#![no_std]
extern crate std;
use std::println;

pub const fn access_bytes(w: u32) -> u32 {
    (w + 6) / 8 + 1
}

// ================= route A: width as a const, trait keyed on the width itself =================

pub struct Wid<const W: u32>;
pub trait AccessAtWidth {
    type T: Copy;
    const BYTES: u32;
}

macro_rules! access_rows {
    ($($w:literal => $t:ty),* $(,)?) => {
        $( impl AccessAtWidth for Wid<$w> {
            type T = $t;
            const BYTES: u32 = access_bytes($w);
        } )*
        pub const ROUTE_A_IMPL_COUNT: u32 = 0 $( + { let _ = $w; 1 } )*;
    };
}
// One row per width. Eight rows here stand in for the 128 the real range needs.
access_rows! {
    5 => u16, 9 => u32, 10 => u32, 11 => u32, 12 => u32, 13 => u32, 16 => u32, 31 => u64,
}

/// generic over the const width, no arithmetic in type position, so it compiles.
pub fn load_window_a<const W: u32>(_base: *const u8, _k: usize) -> <Wid<W> as AccessAtWidth>::T
where
    Wid<W>: AccessAtWidth,
{
    unimplemented!()
}

/// and the type is genuinely reachable, checked by a const assertion on its size.
pub const fn access_bits_a<const W: u32>() -> u32
where
    Wid<W>: AccessAtWidth,
{
    (core::mem::size_of::<<Wid<W> as AccessAtWidth>::T>() * 8) as u32
}
const _: () = assert!(access_bits_a::<13>() == 32);
const _: () = assert!(access_bits_a::<5>() == 16);
const _: () = assert!(access_bits_a::<31>() == 64);

// ================= route B: width as a type, access as an associated type =================

pub trait Width {
    const BITS: u32;
    type Native: Copy;
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
    W13 = 13 : u16, u32;
    W16 = 16 : u16, u32;
    W31 = 31 : u32, u64;
}

pub struct Cold;
pub struct Warm;

pub trait Representation {
    type Carrier: Copy;
    type Access: Copy;
    const STRIDE_BITS: u32;
}
pub struct Packed<W: Width>(core::marker::PhantomData<W>);
pub struct Padded<W: Width>(core::marker::PhantomData<W>);
impl<W: Width> Representation for Packed<W> {
    type Carrier = W::Native;
    type Access = W::Access;
    const STRIDE_BITS: u32 = W::BITS;
}
impl<W: Width> Representation for Padded<W> {
    type Carrier = W::Native;
    type Access = W::Native;
    const STRIDE_BITS: u32 = (core::mem::size_of::<W::Native>() * 8) as u32;
}
pub trait Derive<S> {
    type Repr: Representation;
}
impl<W: Width> Derive<Cold> for W {
    type Repr = Packed<W>;
}
impl<W: Width> Derive<Warm> for W {
    type Repr = Padded<W>;
}

/// the same site as route A, written once, generic over BOTH width and strategy, and the load
/// type comes out as a type with no arithmetic anywhere in type position.
pub fn load_window_b<W: Derive<S>, S>(
    _base: *const u8,
    _k: usize,
) -> <<W as Derive<S>>::Repr as Representation>::Access {
    unimplemented!()
}

pub const fn access_bits_b<W: Derive<S>, S>() -> u32 {
    (core::mem::size_of::<<<W as Derive<S>>::Repr as Representation>::Access>() * 8) as u32
}
const _: () = assert!(access_bits_b::<W13, Cold>() == 32);
const _: () = assert!(access_bits_b::<W13, Warm>() == 16);
const _: () = assert!(access_bits_b::<W31, Cold>() == 64);

fn main() {
    println!("route A: width as a const, trait keyed directly on the width.");
    println!("  compiles. access type reachable. no arithmetic in type position.");
    println!("  impls needed in this file: {}", ROUTE_A_IMPL_COUNT);
    println!("  impls needed to cover widths 1..=128: 128, one per width.");
    println!("  that is the enumeration shape the design refused (quoted at 16:485-488).");
    println!(
        "  so route A is closed by DESIGN, not by the compiler. p3 was closed by the compiler."
    );
    println!();
    println!("route B: width as a type, access as an associated type on the derivation result.");
    println!("  compiles. access type reachable. one blanket impl per strategy.");
    println!(
        "  W13/Cold access = u{}   W13/Warm access = u{}   W31/Cold access = u{}",
        access_bits_b::<W13, Cold>(),
        access_bits_b::<W13, Warm>(),
        access_bits_b::<W31, Cold>()
    );
    println!();
    println!("the load type is a projection of the derivation result under B, and is not");
    println!("reachable at all from the flat pair (carrier type, stride const) alone.");
}
