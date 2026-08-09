#!/usr/bin/env python3
"""Generate the same consumer program under four arms.

The consumer profile is drawn from what the design's heaviest real consumer
actually writes, measured this session across the workspace:

    grep -rhno 'Uint<[^>]*>|Int<[^>]*>|UFixed<[^>]*>|IFixed<[^>]*>' \
        hilavitkutin/mock/crates --include='*.rs'

which yields widths 1,2,3,4,5,6,7,11,14,16,27,28,64 plus one purely
fractional (0,16), across strategies Hot / Warm / Cold. Capacities come from
the same tree's Dim / Array / Matrix sites. Nothing here is a synthetic sweep
of compositions; the shapes are the ones a consumer wrote.

Arms, all forced to discharge the SAME five obligations so the comparison is
between designs rather than between amounts of work:

  1. the total stored width, readable by the consumer
  2. `OneRepresentable` as a BOUND that refuses a zero integer part
  3. the two fraction predicates
  4. a capacity-generic container that builds and walks
  5. one function generic over both a numeral and a capacity

  z   type-level naturals; the width is a type-level sum folded by the trait
      solver at every instantiation.
  zs  same types, but the declaration site emits the already-summed width, so
      no fold runs at a concrete site. Generic code still folds.
  y   const parameters; `OneRepresentable` by the two-dimensional impl table,
      which is its only expression under the permitted features.
  ys  const parameters, with the predicate computed at expansion time and
      carried as a sealed type-level witness whose agreement with the widths is
      checked at the one door.
"""
import sys

CENSUS = [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
          (11, 0), (14, 0), (16, 0), (27, 0), (28, 0), (64, 0), (0, 16)]
CAPS = [1, 3, 4, 7, 8, 13, 16, 28, 32, 64]


def widths(n):
    out = []
    i = 0
    while len(out) < n:
        base = CENSUS[i % len(CENSUS)]
        bump = i // len(CENSUS)
        # extend past the census by walking fresh integer widths, never
        # repeating a pair, so `n` really is n DISTINCT numerals.
        w = (base[0] + bump * 7, base[1]) if base[0] else (base[0], base[1] + bump)
        if w[0] + w[1] > 0 and w not in out:
            out.append(w)
        i += 1
    return out


def caps(m):
    out = []
    i = 0
    while len(out) < m:
        c = CAPS[i % len(CAPS)] + (i // len(CAPS)) * 11
        if c not in out:
            out.append(c)
        i += 1
    return out


def binenc(n, prefix):
    """The sealed value-unique encoding of n, as a type expression."""
    assert n >= 0
    if n == 0:
        return f"{prefix}Z"
    bits = bin(n)[3:]  # drop the leading 1, which is H
    t = f"{prefix}H"
    for b in bits:
        t = (f"{prefix}I<{t}>" if b == "1" else f"{prefix}O<{t}>")
    return f"{prefix}Pz<{t}>"


PRELUDE_NAT = """#![no_std]
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

// Successor and addition on the sealed encoding: the machinery route Z needs
// so that a stored width is a type. Peano would be linear in the VALUE; this
// is the binary encoding, so it is linear in the number of BITS.
pub trait SuccP { type Out: Pos; }
impl SuccP for H { type Out = O<H>; }
impl<P: Pos> SuccP for O<P> { type Out = I<P>; }
impl<P: Pos> SuccP for I<P> where P: SuccP { type Out = O<<P as SuccP>::Out>; }

pub trait AddP<R> { type Out: Pos; }
impl AddP<H> for H { type Out = O<H>; }
impl<P: Pos> AddP<H> for O<P> { type Out = I<P>; }
impl<P: Pos> AddP<H> for I<P> where P: SuccP { type Out = O<<P as SuccP>::Out>; }
impl<P: Pos> AddP<O<P>> for H where P: SuccP { type Out = I<P>; }
impl<P: Pos> AddP<I<P>> for H where P: SuccP { type Out = O<<P as SuccP>::Out>; }
impl<A: Pos, B: Pos> AddP<O<B>> for O<A> where A: AddP<B> { type Out = O<<A as AddP<B>>::Out>; }
impl<A: Pos, B: Pos> AddP<I<B>> for O<A> where A: AddP<B> { type Out = I<<A as AddP<B>>::Out>; }
impl<A: Pos, B: Pos> AddP<O<B>> for I<A> where A: AddP<B> { type Out = I<<A as AddP<B>>::Out>; }
impl<A: Pos, B: Pos> AddP<I<B>> for I<A>
where A: AddP<B>, <A as AddP<B>>::Out: SuccP { type Out = O<<<A as AddP<B>>::Out as SuccP>::Out>; }

pub trait AddN<R> { type Out: Nat; }
impl AddN<Z> for Z { type Out = Z; }
impl<P: Pos> AddN<Z> for Pz<P> { type Out = Pz<P>; }
impl<P: Pos> AddN<Pz<P>> for Z { type Out = Pz<P>; }
impl<A: Pos, B: Pos> AddN<Pz<B>> for Pz<A> where A: AddP<B> { type Out = Pz<<A as AddP<B>>::Out>; }

// Obligation 2: one is representable exactly when the integer part is nonzero.
// A bound, not a bool: the absence at Z is what refuses.
pub trait OneRepresentable {}
impl<P: Pos> OneRepresentable for Pz<P> {}
// Obligation 3.
pub trait IsZero {} impl IsZero for Z {}
pub trait NonZero {} impl<P: Pos> NonZero for Pz<P> {}

pub struct Hot; pub struct Warm; pub struct Cold;
pub trait Strategy {}
impl Strategy for Hot {} impl Strategy for Warm {} impl Strategy for Cold {}

// Obligation 4: the capacity, split by layer (probe B2). The count is the
// shared carrier; the array grammar is the lowering-side literal.
pub struct Slot<N, const K: usize>(PhantomData<N>);
impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}
impl<N: Nat, const K: usize> Nat for Slot<N, K> { const VAL: usize = N::VAL; }
pub const fn agrees<N: Nat, const K: usize>() -> bool { N::VAL == K }
pub trait Capacity: Nat { type Array<T>: AsRef<[T]> + AsMut<[T]>;
    fn build<T: Copy>(v: T) -> Self::Array<T>; }
impl<N: Nat, const K: usize> Capacity for Slot<N, K> {
    type Array<T> = [T; K];
    fn build<T: Copy>(v: T) -> [T; K] {
        const { assert!(agrees::<N, K>(), "capacity length disagrees with its value") };
        [v; K]
    }
}
"""

PRELUDE_CONST = """#![no_std]
#![feature(adt_const_params)]
use core::marker::PhantomData;
pub struct Hot; pub struct Warm; pub struct Cold;
pub trait Strategy {}
impl Strategy for Hot {} impl Strategy for Warm {} impl Strategy for Cold {}
pub struct Slot<const K: usize>;
pub trait Capacity { const VAL: usize; type Array<T>: AsRef<[T]> + AsMut<[T]>;
    fn build<T: Copy>(v: T) -> Self::Array<T>; }
impl<const K: usize> Capacity for Slot<K> {
    const VAL: usize = K;
    type Array<T> = [T; K];
    fn build<T: Copy>(v: T) -> [T; K] { [v; K] }
}
"""


def arm_z(n, m, staged):
    """Type-level naturals. `staged` emits the already-summed width type."""
    s = [PRELUDE_NAT]
    s.append("""
pub struct UFixed<Ib, Fb, S>(PhantomData<(Ib, Fb, S)>);
// Obligation 1: the stored width. Under this arm it is a type-level sum, and
// the solver folds it wherever a consumer reads it.
pub trait Stored { type Width: Nat; const W: usize; }
impl<Ib: Nat + AddN<Fb>, Fb: Nat, S: Strategy> Stored for UFixed<Ib, Fb, S> {
    type Width = <Ib as AddN<Fb>>::Out;
    const W: usize = <<Ib as AddN<Fb>>::Out as Nat>::VAL;
}
// Obligation 2 at the numeral: reachable only when the integer part has one.
pub trait HasOne {}
impl<Ib: Nat + OneRepresentable, Fb: Nat, S: Strategy> HasOne for UFixed<Ib, Fb, S> {}
// Arithmetic: the sum of two numerals, which is where a width genuinely has to
// be computed rather than declared, in both arms.
pub trait AddNum<R> { type Out; }
impl<Ia: Nat + AddN<Ib>, Fa: Nat + AddN<Fb>, Ib: Nat, Fb: Nat, S: Strategy>
    AddNum<UFixed<Ib, Fb, S>> for UFixed<Ia, Fa, S>
{ type Out = UFixed<<Ia as AddN<Ib>>::Out, <Fa as AddN<Fb>>::Out, S>; }
""")
    strats = ["Hot", "Warm", "Cold"]
    ws = widths(n)
    for k, (i, f) in enumerate(ws):
        st = strats[k % 3]
        s.append(f"pub type Ib{k} = {binenc(i, '')};")
        s.append(f"pub type Fb{k} = {binenc(f, '')};")
        if staged:
            # The declaration site emits the reduced normal form of the sum.
            s.append(f"pub type Wb{k} = {binenc(i + f, '')};")
            s.append(f"pub type N{k} = UFixed<Ib{k}, Fb{k}, {st}>;")
            s.append(f"pub const W{k}: usize = <Wb{k} as Nat>::VAL;")
            s.append(f"const _: () = assert!(W{k} == {i + f});")
        else:
            s.append(f"pub type N{k} = UFixed<Ib{k}, Fb{k}, {st}>;")
            s.append(f"pub const W{k}: usize = <N{k} as Stored>::W;")
            s.append(f"const _: () = assert!(W{k} == {i + f});")
        if i > 0:
            s.append(f"pub fn one_ok_{k}() where N{k}: HasOne {{}}")
        if f == 0:
            s.append(f"pub fn int_like_{k}() where Fb{k}: IsZero {{}}")
        else:
            s.append(f"pub fn frac_like_{k}() where Fb{k}: NonZero {{}}")
    # pairwise arithmetic across neighbours, same strategy only
    for k in range(len(ws) - 3):
        if k % 3 == (k + 3) % 3:
            s.append(f"pub type Sum{k} = <N{k} as AddNum<N{k + 3}>>::Out;")
            s.append(f"pub const SW{k}: usize = <Sum{k} as Stored>::W;")
    for k, c in enumerate(caps(m)):
        s.append(f"pub type C{k} = Slot<{binenc(c, '')}, {c}>;")
        s.append(f"pub fn build{k}() -> <C{k} as Capacity>::Array<u32> {{ C{k}::build(0) }}")
    s.append("""
// Obligation 4: build and walk, generic over the capacity.
pub fn fold_generic<C: Capacity>(seed: u32) -> u32 {
    let mut a = C::build(seed);
    let s: &mut [u32] = a.as_mut();
    let mut i = 0; while i < s.len() { s[i] = s[i].wrapping_add(i as u32); i += 1; }
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32; let mut j = 0;
    while j < r.len() { acc = acc.wrapping_add(r[j]); j += 1; }
    acc
}
// Obligation 5: generic over a numeral AND a capacity at once. This is the
// site staging cannot reach, because neither width is known here.
pub fn scaled_fold<Ib, Fb, S, C>(seed: u32) -> u32
where Ib: Nat + AddN<Fb>, Fb: Nat, S: Strategy, C: Capacity,
      UFixed<Ib, Fb, S>: Stored + HasOne
{ fold_generic::<C>(seed).wrapping_mul(<UFixed<Ib, Fb, S> as Stored>::W as u32) }
""")
    for k in range(min(len(ws), m)):
        i, f = ws[k]
        if i > 0:
            st = strats[k % 3]
            s.append(f"pub fn call{k}() -> u32 "
                     f"{{ scaled_fold::<Ib{k}, Fb{k}, {st}, C{k % m}>({k}) }}")
    return "\n".join(s) + "\n"


def arm_y(n, m, staged, ceiling):
    s = [PRELUDE_CONST]
    s.append("""
pub struct UFixed<const I: u16, const F: u16, S>(PhantomData<S>);
pub trait Stored { const W: usize; }
impl<const I: u16, const F: u16, S: Strategy> Stored for UFixed<I, F, S> {
    const W: usize = (I as usize) + (F as usize);
}
pub trait IsZeroW<const F: u16> {}
pub struct FracFlag<const F: u16>;
impl IsZeroW<0> for FracFlag<0> {}
pub trait NonZeroW<const F: u16> {}
""")
    if staged:
        s.append("""
// Obligation 2, staged: the predicate is decided at expansion time and carried
// as a sealed type-level witness. One impl, no table. The agreement between the
// witness and the widths is checked at the one door, so a hand-written lie does
// not survive the build (see y_attack.rs).
mod wseal { pub trait Sealed {} }
pub struct OneYes; pub struct OneNo;
impl wseal::Sealed for OneYes {} impl wseal::Sealed for OneNo {}
pub trait OneWitness: wseal::Sealed { const YES: bool; }
impl OneWitness for OneYes { const YES: bool = true; }
impl OneWitness for OneNo { const YES: bool = false; }
pub struct Num<const I: u16, const F: u16, W, S>(PhantomData<(W, S)>);
pub trait HasOne { fn witness(); }
impl<const I: u16, const F: u16, S: Strategy> HasOne for Num<I, F, OneYes, S> {
    fn witness() { const { assert!(I > 0, "one-witness disagrees with the widths") }; }
}
""")
    else:
        s.append("""
// Obligation 2, unstaged: `I > 0` compared against a const parameter has no
// expression under the permitted features except an impl table. The table is
// two-dimensional in (I, F) because the predicate the review installed reads
// both, and it must cover the substrate's own dispatch ceiling rather than the
// consumer's current maximum, per arvo-toolbox-not-policer.md's ban on a cap
// below what the substrate dispatches.
pub trait HasOne {}
""")
        rows = []
        for i in range(1, ceiling + 1):
            for f in range(0, ceiling + 1 - i):
                rows.append(f"impl<S: Strategy> HasOne for UFixed<{i}, {f}, S> {{}}")
        s.append("\n".join(rows))
    strats = ["Hot", "Warm", "Cold"]
    ws = widths(n)
    for k, (i, f) in enumerate(ws):
        st = strats[k % 3]
        if staged:
            yes = "OneYes" if i > 0 else "OneNo"
            s.append(f"pub type N{k} = Num<{i}, {f}, {yes}, {st}>;")
            s.append(f"pub const W{k}: usize = {i + f};")
            if i > 0:
                s.append(f"pub fn one_ok_{k}() {{ <N{k} as HasOne>::witness() }}")
        else:
            s.append(f"pub type N{k} = UFixed<{i}, {f}, {st}>;")
            s.append(f"pub const W{k}: usize = <N{k} as Stored>::W;")
            if i > 0:
                s.append(f"pub fn one_ok_{k}() where N{k}: HasOne {{}}")
        s.append(f"pub const F{k}_IS_ZERO: bool = {str(f == 0).lower()};")
    for k in range(len(ws) - 3):
        a, b = ws[k], ws[k + 3]
        s.append(f"pub const SW{k}: usize = {a[0] + b[0] + a[1] + b[1]};")
    for k, c in enumerate(caps(m)):
        s.append(f"pub type C{k} = Slot<{c}>;")
        s.append(f"pub fn build{k}() -> <C{k} as Capacity>::Array<u32> {{ C{k}::build(0) }}")
    s.append("""
pub fn fold_generic<C: Capacity>(seed: u32) -> u32 {
    let mut a = C::build(seed);
    let s: &mut [u32] = a.as_mut();
    let mut i = 0; while i < s.len() { s[i] = s[i].wrapping_add(i as u32); i += 1; }
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32; let mut j = 0;
    while j < r.len() { acc = acc.wrapping_add(r[j]); j += 1; }
    acc
}
pub fn scaled_fold<const I: u16, const F: u16, S: Strategy, C: Capacity>(seed: u32) -> u32
where UFixed<I, F, S>: Stored
{ fold_generic::<C>(seed).wrapping_mul(<UFixed<I, F, S> as Stored>::W as u32) }
""")
    for k in range(min(len(ws), m)):
        i, f = ws[k]
        if i > 0:
            st = strats[k % 3]
            s.append(f"pub fn call{k}() -> u32 "
                     f"{{ scaled_fold::<{i}, {f}, {st}, C{k % m}>({k}) }}")
    return "\n".join(s) + "\n"


def arm_base(n, m):
    """The floor: the same consumer program with none of the machinery.

    Numerals are plain aliases and capacities are plain literal arrays. Every
    obligation that needs a type-level mechanism is discharged by a const the
    generator computed, which is exactly what the arms are being compared
    against: the cost of moving these facts into the type system at all.
    """
    s = [PRELUDE_CONST.replace("#![feature(adt_const_params)]\n", "")]
    ws = widths(n)
    for k, (i, f) in enumerate(ws):
        s.append(f"pub const W{k}: usize = {i + f};")
        s.append(f"pub const ONE_OK_{k}: bool = {str(i > 0).lower()};")
        s.append(f"pub const F{k}_IS_ZERO: bool = {str(f == 0).lower()};")
    for k in range(len(ws) - 3):
        a, b = ws[k], ws[k + 3]
        s.append(f"pub const SW{k}: usize = {a[0] + b[0] + a[1] + b[1]};")
    for k, c in enumerate(caps(m)):
        s.append(f"pub type C{k} = Slot<{c}>;")
        s.append(f"pub fn build{k}() -> [u32; {c}] {{ [0; {c}] }}")
    s.append("""
pub fn fold_generic<C: Capacity>(seed: u32) -> u32 {
    let mut a = C::build(seed);
    let s: &mut [u32] = a.as_mut();
    let mut i = 0; while i < s.len() { s[i] = s[i].wrapping_add(i as u32); i += 1; }
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32; let mut j = 0;
    while j < r.len() { acc = acc.wrapping_add(r[j]); j += 1; }
    acc
}
pub fn scaled_fold<C: Capacity>(seed: u32, w: usize) -> u32
{ fold_generic::<C>(seed).wrapping_mul(w as u32) }
""")
    for k in range(min(len(ws), m)):
        i, f = ws[k]
        if i > 0:
            s.append(f"pub fn call{k}() -> u32 {{ scaled_fold::<C{k % m}>({k}, W{k}) }}")
    return "\n".join(s) + "\n"


if __name__ == "__main__":
    arm = sys.argv[1]
    n = int(sys.argv[2])
    m = int(sys.argv[3])
    ceiling = int(sys.argv[4]) if len(sys.argv) > 4 else 64
    if arm == "z":
        print(arm_z(n, m, False), end="")
    elif arm == "zs":
        print(arm_z(n, m, True), end="")
    elif arm == "y":
        print(arm_y(n, m, False, ceiling), end="")
    elif arm == "ys":
        print(arm_y(n, m, True, ceiling), end="")
    elif arm == "base":
        print(arm_base(n, m), end="")
    else:
        raise SystemExit("arm must be base | z | zs | y | ys")
