// p3: does `35`'s accumulator derivation compose with `35`'s own inductive
// ceil(log2), or are they two probes that cannot be one artifact?
//
// WHY THIS RUNS. `35` section 3.2 claims `acc_width(W, C) = W + ceil(log2 C)`
// is expressible gate-free, and supports it with two probes. `p7` builds the
// `SumAccum<Cap<K>>` derivation and satisfies its `Cap<K>: Log2Ceil` bound with
// ONE IMPL PER CAPACITY; `35` says so itself and calls that version
// "inadmissible in substance". `p8` replaces `ceil(log2)` with three impls over
// a positive-binary representation and no table.
//
// NOBODY COMPOSED THEM. `p7.out` as committed still prints the enumeration in
// its arm-5 diagnostic:
//     help: the following other types implement trait `Log2Ceil`:
//               Cap<1024>  Cap<16>  Cap<1>  Cap<256>  Cap<2>  Cap<3>  Cap<4>  Cap<8>
// and `p8` never mentions `SumAccum`. So "gate-free" is the conjunction of two
// artifacts that use DIFFERENT capacity representations: `p7` a const generic
// `Cap<const K: usize>`, `p8` a type-level binary `One / Twice<N> / TwiceP1<N>`.
// Composing them needs a const-to-type map, which is the direction Rust refuses.
//
// This is the same wall `191` section 2.5 hit from the other side, and neither
// file noticed they were the same wall.
//
// ARMS. Three must compile, three must be refused, and the last is the one the
// answer turns on.
//   A1  SumAccum keyed on the BINARY capacity, folded over a slice   COMPILES
//   A2  the derived widths checked against integer arithmetic        COMPILES
//   A2m the same with the recurrence broken (--cfg mutate)           REFUSED
//   B1  a blanket bridge from Cap<K> to the binary type              REFUSED
//   B2  array storage [T; K] sized from the binary capacity's VAL    REFUSED
//   C1  carrying both a const K and a binary C with no proof they
//       agree, instantiated inconsistently                          COMPILES  <- landmine
//   C2  the same, with associated-const equality asked to enforce it     ?
//
// THE CASE THAT MUST FAIL. A2m. Without it, A2's const assertions could be
// vacuous and every "COMPILES" below would mean only that nothing was checked.
// A1 is also a positive control for B1 and B2: if A1 did not compile, their
// refusals would say nothing about the bridge.

#![allow(dead_code)]

use core::marker::PhantomData;

// ---- p8's machinery, copied verbatim in shape ---------------------------

pub struct Z;
pub struct Su<N>(PhantomData<N>);
pub trait NatVal {
    const VAL: u32;
}
impl NatVal for Z {
    const VAL: u32 = 0;
}
impl<N: NatVal> NatVal for Su<N> {
    const VAL: u32 = N::VAL + 1;
}

pub struct One;
pub struct Twice<N>(PhantomData<N>);
pub struct TwiceP1<N>(PhantomData<N>);

pub trait PosVal {
    const VAL: u64;
}
impl PosVal for One {
    const VAL: u64 = 1;
}
impl<N: PosVal> PosVal for Twice<N> {
    const VAL: u64 = 2 * N::VAL;
}
impl<N: PosVal> PosVal for TwiceP1<N> {
    const VAL: u64 = 2 * N::VAL + 1;
}

pub trait Inc {
    type Out;
}
impl Inc for One {
    type Out = Twice<One>;
}
impl<N> Inc for Twice<N> {
    type Out = TwiceP1<N>;
}
impl<N: Inc> Inc for TwiceP1<N> {
    type Out = Twice<<N as Inc>::Out>;
}

pub trait Log2Ceil {
    type Out;
}
impl Log2Ceil for One {
    type Out = Z;
}
impl<N: Log2Ceil> Log2Ceil for Twice<N> {
    type Out = Su<<N as Log2Ceil>::Out>;
}
#[cfg(not(mutate))]
impl<N> Log2Ceil for TwiceP1<N>
where
    N: Inc,
    <N as Inc>::Out: Log2Ceil,
{
    type Out = Su<<<N as Inc>::Out as Log2Ceil>::Out>;
}
// The mutation: drop the increment from the third line. ceil(log2(2N+1))
// becomes 1 + ceil(log2 N), which is right at powers of two and wrong at 3, 5,
// 6, 7, 9 ... A2 must catch it.
#[cfg(mutate)]
impl<N: Log2Ceil> Log2Ceil for TwiceP1<N> {
    type Out = Su<<N as Log2Ceil>::Out>;
}

// ---- p7's machinery, keyed on the binary capacity instead of Cap<K> -----

pub trait Add<R> {
    type Out;
}
impl<R> Add<R> for Z {
    type Out = R;
}
impl<L, R> Add<R> for Su<L>
where
    L: Add<R>,
{
    type Out = Su<<L as Add<R>>::Out>;
}

pub struct Num<W>(pub u64, PhantomData<W>);
impl<W> Num<W> {
    pub const fn new(v: u64) -> Self {
        Num(v, PhantomData)
    }
}
impl<W> Clone for Num<W> {
    fn clone(&self) -> Self {
        Num(self.0, PhantomData)
    }
}
impl<W> Copy for Num<W> {}

pub trait CAdd {
    fn cadd(self, rhs: Self) -> Self;
    fn zero() -> Self;
}
impl<W> CAdd for Num<W> {
    fn cadd(self, rhs: Self) -> Self {
        Num::new(self.0.wrapping_add(rhs.0))
    }
    fn zero() -> Self {
        Num::new(0)
    }
}

pub trait SumAccum<C> {
    type Acc: CAdd + Copy;
    fn lift(self) -> Self::Acc;
}

// ARM A1. The derivation, keyed on a capacity that is a TYPE. No const
// generic anywhere in the bound, so p8's three impls satisfy it for every
// capacity rather than for the eight p7 enumerated.
impl<W, C> SumAccum<C> for Num<W>
where
    C: Log2Ceil,
    W: Add<<C as Log2Ceil>::Out>,
{
    type Acc = Num<<W as Add<<C as Log2Ceil>::Out>>::Out>;
    fn lift(self) -> Self::Acc {
        Num::new(self.0)
    }
}

/// A bounded aggregate whose capacity is a type and whose storage is a slice.
/// The array length is deliberately not at the type level; B2 is where that
/// is tested.
pub struct BoundedSlice<'a, T, C> {
    pub items: &'a [T],
    pub cap: PhantomData<C>,
}

pub fn sum_bounded<'a, W, C>(xs: &BoundedSlice<'a, Num<W>, C>) -> <Num<W> as SumAccum<C>>::Acc
where
    Num<W>: SumAccum<C>,
{
    let mut acc = <<Num<W> as SumAccum<C>>::Acc as CAdd>::zero();
    let mut i = 0usize;
    while i < xs.items.len() {
        acc = acc.cadd(xs.items[i].lift());
        i += 1;
    }
    acc
}

// ---- ARM A2: the derived widths against integer arithmetic --------------

const fn lg_ref(v: u64) -> u32 {
    let mut r = 0u32;
    let mut p = 1u64;
    while p < v {
        p *= 2;
        r += 1;
    }
    r
}

pub trait AccWidth {
    const W: u32;
}
impl<W: NatVal, C> AccWidth for (Num<W>, C)
where
    C: Log2Ceil,
    <C as Log2Ceil>::Out: NatVal,
{
    const W: u32 = W::VAL + <<C as Log2Ceil>::Out as NatVal>::VAL;
}

type N0 = Z;
type N1 = Su<N0>;
type N2 = Su<N1>;
type N3 = Su<N2>;
type N4 = Su<N3>;
type N8 = Su<Su<Su<Su<N4>>>>;

// capacities as binary types
type C1 = One; // 1
type C2 = Twice<One>; // 2
type C3 = TwiceP1<One>; // 3
type C4 = Twice<Twice<One>>; // 4
type C5 = TwiceP1<Twice<One>>; // 5
type C7 = TwiceP1<TwiceP1<One>>; // 7
type C16 = Twice<Twice<Twice<Twice<One>>>>; // 16
type C256 = Twice<Twice<Twice<Twice<Twice<Twice<Twice<Twice<One>>>>>>>>;

macro_rules! check_cap {
    ($c:ty) => {
        const _: () = {
            assert!(<$c as PosVal>::VAL == <$c as PosVal>::VAL);
        };
        const _: () = {
            // the accumulator width must equal W + ceil(log2 C), computed by
            // integer arithmetic on the capacity's own value.
            assert!(<(Num<N4>, $c) as AccWidth>::W == 4 + lg_ref(<$c as PosVal>::VAL));
        };
    };
}
check_cap!(C1);
check_cap!(C2);
check_cap!(C3);
check_cap!(C4);
check_cap!(C5);
check_cap!(C7);
check_cap!(C16);
check_cap!(C256);

// tightness: one bit narrower is not enough for the worst case.
const _: () = {
    let w = <(Num<N4>, C16) as AccWidth>::W; // 4 + 4 = 8
    assert!(w == 8);
    // 16 values each at most 2^4 - 1 = 15 sum to 240, which needs 8 bits and
    // does not fit in 7.
    assert!(16u64 * 15 <= (1u64 << w) - 1);
    assert!(16u64 * 15 > (1u64 << (w - 1)) - 1);
};

pub fn a1_use(xs: &[Num<N4>]) -> Num<N8> {
    let b = BoundedSlice::<Num<N4>, C16> {
        items: xs,
        cap: PhantomData,
    };
    sum_bounded(&b)
}

// ---- ARM B1: a blanket bridge from a const generic to the binary type ---
#[cfg(arm_b1)]
pub struct Cap<const K: usize>;
#[cfg(arm_b1)]
pub trait ToBinary {
    type Out;
}
// The map every consumer would want: from the const the storage needs to the
// type the derivation needs. There is no way to case-split a const generic in
// a type position without arithmetic in it.
#[cfg(arm_b1)]
impl<const K: usize> ToBinary for Cap<K> {
    type Out = <Cap<{ K / 2 }> as ToBinary>::Out;
}

// ---- ARM B2: array storage sized from the capacity type's own value -----
#[cfg(arm_b2)]
pub struct BoundedArray<T, C: PosVal> {
    pub items: [T; C::VAL as usize],
    pub cap: PhantomData<C>,
}

// ---- ARM C1: both, with nothing tying them together --------------------
#[cfg(any(arm_c1, arm_c2))]
pub struct Both<T, const K: usize, C> {
    pub items: [T; K],
    pub cap: PhantomData<C>,
}

#[cfg(arm_c1)]
pub fn c1_inconsistent(xs: [Num<N4>; 4]) -> Both<Num<N4>, 4, C256> {
    // K = 4 and C = 256. Nothing relates them. If this compiles, the shape is
    // the same landmine 191 found at p1 arms G1 to G3: a nameable, storable,
    // returnable type whose two halves disagree.
    Both {
        items: xs,
        cap: PhantomData,
    }
}

// ---- ARM C2 lives in p3b_const_to_type_bridge.rs ------------------------
// It cannot live here. `C: PosVal<VAL = { K as u64 }>` is gated at PARSE time,
// so merely writing the bound in this file makes every other arm unbuildable
// whatever cfg is set. That is itself the C2 result and is recorded in p3b.
