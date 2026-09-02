//! e01. An enumeration-free const-to-type bridge, in one blanket impl.
//!
//! Every route in `10` section 8 treats the bridge as blocked. Section 1 of `11`
//! says the block is narrower than that: a BARE const parameter reaches type
//! position freely, and only an ARITHMETIC image of it is refused.
//!
//! If that is right, a bridge with no table should be writable, provided its
//! codomain is an array type whose length is the bare parameter. This file tests
//! exactly that, and then tests what the resulting type is good for.
//!
//! No `#![feature]`, no `-Z` flag.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata -o out/e01.meta e01_enumeration_free_bridge.rs
#![no_std]
#![crate_type = "lib"]

pub struct Idx<const N: usize>;
pub struct Arvo;
pub trait ToNat<M> {
    type N;
}

// ONE impl. Total over every width. No literal appears anywhere.
impl<const N: usize, M> ToNat<M> for Idx<N> {
    type N = [u8; N];
}

// It is total: no width can be "not shipped".
pub type W13 = <Idx<13> as ToNat<Arvo>>::N;
pub type W4711 = <Idx<4711> as ToNat<Arvo>>::N;
pub type W1636 = <Idx<1636> as ToNat<Arvo>>::N;

// And a container trait blanket-implements over it, also with no table.
pub trait Container {
    type C;
}
impl<const N: usize> Container for [u8; N] {
    type C = [u8; N];
}

pub type Cont<W> = <W as Container>::C;
pub type ContainerFor<const N: usize> = Cont<<Idx<N> as ToNat<Arvo>>::N>;

// A whole numeral, generic over the width, with no enumeration in the crate.
#[repr(transparent)]
pub struct Fixed<const I: usize, const F: usize, S>
where
    Idx<I>: ToNat<Arvo>,
    // the same where-clause soup `10` section 5.3 names: rustc will not
    // normalise the projection to `[u8; I]` in a generic context on its own.
    <Idx<I> as ToNat<Arvo>>::N: Container,
{
    raw: ContainerFor<I>,
    _m: core::marker::PhantomData<(S, [(); F])>,
}

pub struct Hot;

// Every width, including ones nobody wrote:
pub type A = Fixed<13, 3, Hot>;
pub type B = Fixed<4711, 1, Hot>;
pub type C = Fixed<1636, 0, Hot>;

// --- and now the part that decides whether any of it is worth anything ------

// The bridge is total and enumeration-free. Its codomain overshoots by exactly
// a factor of eight: N BYTES are allocated for N BITS.
const _: () = {
    assert!(core::mem::size_of::<ContainerFor<13>>() == 13); // want 2
    assert!(core::mem::size_of::<ContainerFor<64>>() == 64); // want 8
    assert!(core::mem::size_of::<ContainerFor<1636>>() == 1636); // want 208
};

// Closing the overshoot needs ceil(N/8). Uncomment to see which feature rustc
// names. Left commented so this file compiles; the refusal is in e02.
// impl<const N: usize> Container for [u8; N] { type C = [u8; (N + 7) / 8]; }
