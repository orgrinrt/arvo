#!/usr/bin/env python3
"""Sweep the capacity VALUE, not the count.

After the unification a capacity is a type-level natural, and the binary
encoding's depth is the number of bits in the value. Counts in the census are
small; capacities in a real column store are not. A consumer that declares a
buffer of 65536 records is declaring a seventeen-deep type, and a consumer that
declares one of 16777216 is declaring a twenty-five-deep one.

This is the axis the count sweep cannot see, and it is the one the unification
newly puts under every container in the workspace.

  ./gen_deep.py <count> <magnitude>

emits `count` distinct capacities whose values sit near 2**magnitude, each
built, walked, and pinned to its own value so no fold can be skipped.
"""
import sys

PRE = """#![no_std]
use core::marker::PhantomData;
mod seal { pub trait Sealed {} }
pub struct H; pub struct O<P>(PhantomData<P>); pub struct I<P>(PhantomData<P>);
pub struct Z; pub struct Pz<P>(PhantomData<P>);
impl seal::Sealed for H {}
impl<P: Pos> seal::Sealed for O<P> {}
impl<P: Pos> seal::Sealed for I<P> {}
impl seal::Sealed for Z {}
impl<P: Pos> seal::Sealed for Pz<P> {}
pub trait Pos: seal::Sealed { const VAL: usize; }
impl Pos for H { const VAL: usize = 1; }
impl<P: Pos> Pos for O<P> { const VAL: usize = 2 * P::VAL; }
impl<P: Pos> Pos for I<P> { const VAL: usize = 2 * P::VAL + 1; }
pub trait Nat: seal::Sealed { const VAL: usize; }
impl Nat for Z { const VAL: usize = 0; }
impl<P: Pos> Nat for Pz<P> { const VAL: usize = P::VAL; }
pub trait Cmp<R> { const LE: bool; }
pub struct Slot<N, const K: usize>(PhantomData<N>);
impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}
impl<N: Nat, const K: usize> Nat for Slot<N, K> { const VAL: usize = N::VAL; }
pub const fn agrees<N: Nat, const K: usize>() -> bool { N::VAL == K }
pub trait Capacity: Nat { type Array<T>: AsRef<[T]> + AsMut<[T]>; const CAP: usize; }
impl<N: Nat, const K: usize> Capacity for Slot<N, K> {
    type Array<T> = [T; K];
    const CAP: usize = { assert!(agrees::<N, K>()); K };
}
pub fn walk<C: Capacity>(a: &C::Array<u32>) -> u32 {
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32; let mut j = 0;
    while j < r.len() { acc = acc.wrapping_add(r[j]); j += 1; }
    acc
}
"""


def binenc(n):
    if n == 0:
        return "Z"
    t = "H"
    for b in bin(n)[3:]:
        t = f"I<{t}>" if b == "1" else f"O<{t}>"
    return f"Pz<{t}>"


count, mag = int(sys.argv[1]), int(sys.argv[2])
out = [PRE]
base = 1 << mag
for k in range(count):
    # odd offsets keep every encoding a distinct bit pattern rather than a
    # run of zeros, so the depth is real work and not a shared prefix.
    v = base + 2 * k + 1
    out.append(f"pub type D{k} = Slot<{binenc(v)}, {v}>;")
    out.append(f"pub const V{k}: usize = <D{k} as Capacity>::CAP;")
    out.append(f"const _: () = assert!(V{k} == {v});")
print("\n".join(out))
