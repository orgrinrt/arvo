#!/usr/bin/env python3
# Emits the tower + a NatIndex table of a requested size, with or without
# the per-row agreement assertion. Used to price the bridge.
import sys

TOWER = r'''#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]

use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

// ---- Pos ::= H | O<P> | I<P> ------------------------------------------
pub trait Pos: Sealed {
    const VAL: u64;
    type Succ: Pos;
}
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);
impl Sealed for H {}
impl<P: Pos> Sealed for O<P> {}
impl<P: Pos> Sealed for I<P> {}
impl Pos for H {
    const VAL: u64 = 1;
    type Succ = O<H>;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
    type Succ = I<P>;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
    type Succ = O<P::Succ>;
}

// ---- Pos addition, the carry chain ------------------------------------
pub trait PosAdd<R: Pos>: Pos {
    type Out: Pos;
}
impl PosAdd<H> for H {
    type Out = O<H>;
}
impl<Q: Pos> PosAdd<O<Q>> for H {
    type Out = I<Q>;
}
impl<Q: Pos> PosAdd<I<Q>> for H {
    type Out = O<Q::Succ>;
}
impl<P: Pos> PosAdd<H> for O<P> {
    type Out = I<P>;
}
impl<P: PosAdd<Q>, Q: Pos> PosAdd<O<Q>> for O<P> {
    type Out = O<<P as PosAdd<Q>>::Out>;
}
impl<P: PosAdd<Q>, Q: Pos> PosAdd<I<Q>> for O<P> {
    type Out = I<<P as PosAdd<Q>>::Out>;
}
impl<P: Pos> PosAdd<H> for I<P> {
    type Out = O<P::Succ>;
}
impl<P: PosAdd<Q>, Q: Pos> PosAdd<O<Q>> for I<P> {
    type Out = I<<P as PosAdd<Q>>::Out>;
}
impl<P: PosAdd<Q>, Q: Pos> PosAdd<I<Q>> for I<P> {
    type Out = O<<<P as PosAdd<Q>>::Out as Pos>::Succ>;
}

// ---- Nat ::= Z | Pz<P> ------------------------------------------------
pub trait Nat: Sealed {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P: Pos>(PhantomData<P>);
impl Sealed for Z {}
impl<P: Pos> Sealed for Pz<P> {}
impl Nat for Z {
    const VAL: u64 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

pub trait NatAdd<R: Nat>: Nat {
    type Out: Nat;
}
impl<R: Nat> NatAdd<R> for Z {
    type Out = R;
}
impl<P: Pos> NatAdd<Z> for Pz<P> {
    type Out = Pz<P>;
}
impl<P: PosAdd<Q>, Q: Pos> NatAdd<Pz<Q>> for Pz<P> {
    type Out = Pz<<P as PosAdd<Q>>::Out>;
}

pub trait Precision: Nat {}
impl<T: Nat> Precision for T {}

// ---- the bridge: one impl per admitted width --------------------------
pub struct Idx<const N: u16>;
pub trait NatIndex {
    type Out: Nat;
}
pub type NatOf<const N: u16> = <Idx<N> as NatIndex>::Out;
pub type PrecisionOf<const A: u16, const B: u16> =
    <NatOf<{ A }> as NatAdd<NatOf<{ B }>>>::Out;
'''


def pos_lit(n):
    assert n >= 1
    if n == 1:
        return "H"
    if n % 2 == 0:
        return "O<%s>" % pos_lit(n // 2)
    return "I<%s>" % pos_lit(n // 2)


def nat_lit(n):
    return "Z" if n == 0 else "Pz<%s>" % pos_lit(n)


def main():
    size = int(sys.argv[1])
    check = len(sys.argv) > 2 and sys.argv[2] == "check"
    out = [TOWER]
    for n in range(size):
        out.append("impl NatIndex for Idx<%d> { type Out = %s; }" % (n, nat_lit(n)))
        if check:
            out.append(
                "const _: () = assert!(<<Idx<%d> as NatIndex>::Out as Nat>::VAL == %d);"
                % (n, n)
            )
    out.append("")
    out.append("// the load-bearing claim")
    out.append("const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);")
    print("\n".join(out))


main()
