//! Probe 4. The container dispatch rebuilt correctly, and its true size.
//!
//! Probe 3 established the target shape compiles; its `Bucket` was a
//! placeholder that returned one tag for everything. This one is the real
//! thing: the bucket a `const fn` computes today
//! (`arvo-strategy/src/container.rs:60-91`, ten lines of `if`) derived
//! structurally from the width type, and checked against the const fn's own
//! answer at every boundary.
//!
//! `bucket(W) = clamp(bitlen(W - 1) - 3)`, so it needs three type-level
//! functions the const fn got for free: predecessor, bit length, and a
//! saturating subtract. That decomposition is the finding, not the code.
#![no_std]

use core::marker::PhantomData;

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct Iv<P>(PhantomData<P>);
pub trait Pos {
    const VAL: u128;
}
impl Pos for H {
    const VAL: u128 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u128 = 2 * P::VAL;
}
impl<P: Pos> Pos for Iv<P> {
    const VAL: u128 = 2 * P::VAL + 1;
}

pub struct Z;
pub struct Pz<P>(PhantomData<P>);
pub trait Nat {
    const VAL: u128;
}
impl Nat for Z {
    const VAL: u128 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u128 = P::VAL;
}

// -- unary counter for lengths (at most 16 for a u16 width) -----------------
pub struct UZ;
pub struct US<N>(PhantomData<N>);
pub trait Un {
    const N: u32;
}
impl Un for UZ {
    const N: u32 = 0;
}
impl<N: Un> Un for US<N> {
    const N: u32 = N::N + 1;
}

// -- predecessor: 4 impls (the P=H case splits) -----------------------------
pub trait Pred {
    type Out: Nat;
}
impl Pred for Pz<H> {
    type Out = Z;
}
impl Pred for Pz<O<H>> {
    type Out = Pz<H>;
}
impl<P: Pos> Pred for Pz<O<O<P>>>
where
    Pz<O<P>>: Pred,
    <Pz<O<P>> as Pred>::Out: NatToPos,
{
    type Out = Pz<Iv<<<Pz<O<P>> as Pred>::Out as NatToPos>::Out>>;
}
impl<P: Pos> Pred for Pz<O<Iv<P>>>
where
    Pz<Iv<P>>: Pred,
    <Pz<Iv<P>> as Pred>::Out: NatToPos,
{
    type Out = Pz<Iv<<<Pz<Iv<P>> as Pred>::Out as NatToPos>::Out>>;
}
impl<P: Pos> Pred for Pz<Iv<P>> {
    type Out = Pz<O<P>>;
}
// the helper the split forces into existence
pub trait NatToPos {
    type Out: Pos;
}
impl<P: Pos> NatToPos for Pz<P> {
    type Out = P;
}

// -- bit length: 4 impls ----------------------------------------------------
pub trait BitLen {
    type Out: Un;
}
impl BitLen for Z {
    type Out = UZ;
}
impl BitLen for Pz<H> {
    type Out = US<UZ>;
}
impl<P: Pos> BitLen for Pz<O<P>>
where
    Pz<P>: BitLen,
{
    type Out = US<<Pz<P> as BitLen>::Out>;
}
impl<P: Pos> BitLen for Pz<Iv<P>>
where
    Pz<P>: BitLen,
{
    type Out = US<<Pz<P> as BitLen>::Out>;
}

// -- bucket from bit length: 9 impls (one per reachable length) -------------
pub struct B0;
pub struct B1;
pub struct B2;
pub struct B3;
pub struct B4;
pub struct B5;
pub trait Tag {
    const IDX: usize;
}
impl Tag for B0 {
    const IDX: usize = 0;
}
impl Tag for B1 {
    const IDX: usize = 1;
}
impl Tag for B2 {
    const IDX: usize = 2;
}
impl Tag for B3 {
    const IDX: usize = 3;
}
impl Tag for B4 {
    const IDX: usize = 4;
}
impl Tag for B5 {
    const IDX: usize = 5;
}

pub trait LenTag {
    type Out: Tag;
}
type U0 = UZ;
type U1 = US<U0>;
type U2 = US<U1>;
type U3 = US<U2>;
type U4 = US<U3>;
type U5 = US<U4>;
type U6 = US<U5>;
type U7 = US<U6>;
type U8 = US<U7>;
impl LenTag for U0 {
    type Out = B0;
}
impl LenTag for U1 {
    type Out = B0;
}
impl LenTag for U2 {
    type Out = B0;
}
impl LenTag for U3 {
    type Out = B0;
}
impl LenTag for U4 {
    type Out = B1;
}
impl LenTag for U5 {
    type Out = B2;
}
impl LenTag for U6 {
    type Out = B3;
}
impl LenTag for U7 {
    type Out = B4;
}
impl<N: Un> LenTag for US<US<US<US<US<US<US<US<N>>>>>>>> {
    type Out = B5;
}

pub trait Bucket {
    type Out: Tag;
}
impl<W> Bucket for W
where
    W: Nat + Pred,
    <W as Pred>::Out: BitLen,
    <<W as Pred>::Out as BitLen>::Out: LenTag,
{
    type Out = <<<W as Pred>::Out as BitLen>::Out as LenTag>::Out;
}

// -- the const fn this replaces, verbatim from container.rs:60-75 -----------
pub const fn tag_hot_cold(n: u16) -> usize {
    let n = n as usize;
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else if n <= 128 {
        4
    } else {
        5
    }
}

// -- differential check at every boundary and its neighbours ----------------
macro_rules! check { ($($t:ty),+ $(,)?) => { const _: () = { $(
    assert!(<<$t as Bucket>::Out as Tag>::IDX == tag_hot_cold(<$t as Nat>::VAL as u16));
)+ }; } }
type N1 = Pz<H>;
type N7 = Pz<Iv<Iv<H>>>;
type N8 = Pz<O<O<O<H>>>>;
type N9 = Pz<Iv<O<O<H>>>>;
type N16 = Pz<O<O<O<O<H>>>>>;
type N17 = Pz<Iv<O<O<O<H>>>>>;
type N32 = Pz<O<O<O<O<O<H>>>>>>;
type N33 = Pz<Iv<O<O<O<O<H>>>>>>;
type N64 = Pz<O<O<O<O<O<O<H>>>>>>>;
type N65 = Pz<Iv<O<O<O<O<O<H>>>>>>>;
type N128 = Pz<O<O<O<O<O<O<O<H>>>>>>>>;
type N129 = Pz<Iv<O<O<O<O<O<O<H>>>>>>>>;
type N256 = Pz<O<O<O<O<O<O<O<O<H>>>>>>>>>;
check!(N1, N7, N8, N9, N16, N17, N32, N33, N64, N65, N128, N129, N256);
