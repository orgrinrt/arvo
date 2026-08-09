//! Probe 2. The far-point rule's actual jurisdiction over capacity: not
//! Capacity's own definition, but the resolution for an INDEX that runs past
//! it. Built here as the Nat tower's predecessor operation, because that is
//! the whole content of "the last valid index below a capacity."
//!
//! This is genuinely new construction, not already-built machinery: nothing
//! in the panel's prior probes defines a predecessor on `Pos`/`Nat`. It costs
//! no new sealed vocabulary and no forbidden feature; it is ordinary
//! structural recursion over the closed `H | O<P> | I<P>` grammar, the same
//! shape the tower already uses for `VAL`, `Cmp`, and `Gcd` (`68:654-657`).
//!
//! The derivation, stated once so the impls below read as its transcription
//! rather than as arithmetic pulled from nowhere. The grammar builds a value
//! from its MSB (`H`) outward, each wrapper appending one bit at the LOW end:
//!   I<P> = 2P + 1, so I<P> - 1 = 2P = O<P>. No recursion; the trivial case.
//!   O<P> = 2P >= 2 always (P: Pos means P::VAL >= 1), so O<P> - 1 is ALWAYS
//!     representable as Pos, regardless of P's own shape. That fact is what
//!     lets `PosPred` (predecessor-as-Pos) be total over every `O<_>`.
//!   O<H> = 2, O<H> - 1 = 1 = H. The base case.
//!   O<O<Q>> = 4Q, O<O<Q>> - 1 = 4Q - 1 = 2*(2Q - 1) + 1 = I<(O<Q> - 1)>,
//!     and O<Q> - 1 is Pos by the fact above, so this recurses through the
//!     SAME three cases one level down (a carry chain through trailing zero
//!     bits, exactly as binary decrement works by hand).
//!   O<I<Q>> = 2*(2Q+1) = 4Q+2, O<I<Q>> - 1 = 4Q+1 = I<2Q> = I<O<Q>>. Reduces
//!     to the trivial I-case one level down; no further recursion needed.
//!   H = 1, H - 1 = 0 = Z. The one place the tower's own boundary crosses
//!     from Pos to Nat.
#![no_std]

use core::marker::PhantomData;

mod seal {
    pub trait Sealed {}
}

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl seal::Sealed for H {}
impl<P: Pos> seal::Sealed for O<P> {}
impl<P: Pos> seal::Sealed for I<P> {}
impl seal::Sealed for Z {}
impl<P: Pos> seal::Sealed for Pz<P> {}

pub trait Pos: seal::Sealed {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

pub trait Nat: seal::Sealed {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

/// Predecessor-as-Pos: total over every `O<_>` and `I<_>` (never over `H`,
/// which has no positive predecessor). Four disjoint impls, no
/// specialization: `I<Q>`, `O<H>`, `O<O<Q>>`, `O<I<Q>>` are four structurally
/// distinct types under the sealed grammar, so ordinary coherence accepts
/// all four with no overlap.
pub trait PosPred: Pos {
    type Out: Pos;
}
impl<Q: Pos> PosPred for I<Q> {
    type Out = O<Q>;
}
impl PosPred for O<H> {
    type Out = H;
}
impl<Q: Pos> PosPred for O<O<Q>>
where
    O<Q>: PosPred,
{
    type Out = I<<O<Q> as PosPred>::Out>;
}
impl<Q: Pos> PosPred for O<I<Q>>
where
    I<Q>: PosPred,
{
    type Out = I<<I<Q> as PosPred>::Out>;
}

/// The general predecessor, `Pos -> Nat`, the one place the tower's own
/// boundary is crossed (`H`'s predecessor is `Z`, not a `Pos`).
pub trait Dec: Pos {
    type Out: Nat;
}
impl Dec for H {
    type Out = Z;
}
impl<Q: Pos> Dec for I<Q> {
    type Out = Pz<O<Q>>;
}
impl Dec for O<H> {
    type Out = Pz<H>;
}
impl<Q: Pos> Dec for O<O<Q>>
where
    O<Q>: PosPred,
{
    type Out = Pz<<O<O<Q>> as PosPred>::Out>;
}
impl<Q: Pos> Dec for O<I<Q>>
where
    I<Q>: PosPred,
{
    type Out = Pz<<O<I<Q>> as PosPred>::Out>;
}

/// The far point of a capacity-bounded index space: the last valid index
/// below `C`, i.e. `C - 1`. Defined ONLY for `C: Pos` (a nonzero capacity);
/// an empty capacity has no valid index and so no far point, which the type
/// system states by simply never implementing this for `Z`-rooted counts
/// (see the negative-control probe).
///
/// This is the far-point rule's own shape (`68:275-286`, section 1.16),
/// carried down to the index domain: the supremum of the ordered set of
/// valid indices `{0, ..., C-1}` is `C-1`, taken over a set that, unlike a
/// numeral's value set, can genuinely be empty, and the rule's NaN-exclusion
/// clause (supremum taken over the ordered values only) has a direct analogue
/// here: an empty index set has no supremum, so the operation is not total
/// over `Nat`, only over `Pos`, matching the shape rather than merely
/// resembling it.
pub const fn last_index<C: Pos + Dec>() -> usize
where
    <C as Dec>::Out: Nat,
{
    <C as Dec>::Out::VAL
}

// --- Exhaustive check across the whole matrix of structural cases, not a
// --- sample. Every value 1 through 17, plus 24, 31, 32 (five and six bits:
// --- the trivial I-case, the O<H> base case, carry chains through one to
// --- four trailing zero bits, and O<I<_>> reductions). The grammar builds
// --- MSB-first: H, then each subsequent wrapper appends one LOW bit, so the
// --- OUTERMOST constructor is the least significant bit.

type V1 = H; //     1 =      1
type V2 = O<H>; //     2 =     10
type V3 = I<H>; //     3 =     11
type V4 = O<O<H>>; //     4 =    100
type V5 = I<O<H>>; //     5 =    101
type V6 = O<I<H>>; //     6 =    110
type V7 = I<I<H>>; //     7 =    111
type V8 = O<O<O<H>>>; //     8 =   1000
type V9 = I<O<O<H>>>; //     9 =   1001
type V10 = O<I<O<H>>>; //    10 =   1010
type V11 = I<I<O<H>>>; //    11 =   1011
type V12 = O<O<I<H>>>; //    12 =   1100
type V13 = I<O<I<H>>>; //    13 =   1101
type V14 = O<I<I<H>>>; //    14 =   1110
type V15 = I<I<I<H>>>; //    15 =   1111
type V16 = O<O<O<O<H>>>>; //    16 =  10000
type V17 = I<O<O<O<H>>>>; //    17 =  10001
type V24 = O<O<O<I<H>>>>; //    24 =  11000
type V31 = I<I<I<I<H>>>>; //    31 =  11111
type V32 = O<O<O<O<O<H>>>>>; //    32 = 100000

const _: () = {
    assert!(V1::VAL == 1);
    assert!(V2::VAL == 2);
    assert!(V3::VAL == 3);
    assert!(V4::VAL == 4);
    assert!(V5::VAL == 5);
    assert!(V6::VAL == 6);
    assert!(V7::VAL == 7);
    assert!(V8::VAL == 8);
    assert!(V9::VAL == 9);
    assert!(V10::VAL == 10);
    assert!(V11::VAL == 11);
    assert!(V12::VAL == 12);
    assert!(V13::VAL == 13);
    assert!(V14::VAL == 14);
    assert!(V15::VAL == 15);
    assert!(V16::VAL == 16);
    assert!(V17::VAL == 17);
    assert!(V24::VAL == 24);
    assert!(V31::VAL == 31);
    assert!(V32::VAL == 32);

    // Exhaustive decrement check, VAL - 1 in every case, covering every
    // structural shape the grammar admits at this depth.
    assert!(<H as Dec>::Out::VAL == 0); // 1 -> 0, crosses to Z
    assert!(<V2 as Dec>::Out::VAL == 1); // O<H>, base case
    assert!(<V3 as Dec>::Out::VAL == 2); // I<H>, trivial
    assert!(<V4 as Dec>::Out::VAL == 3); // O<O<H>>, carry chain depth 1
    assert!(<V5 as Dec>::Out::VAL == 4); // I<O<H>>, trivial
    assert!(<V6 as Dec>::Out::VAL == 5); // O<I<H>>, reduces after one step
    assert!(<V7 as Dec>::Out::VAL == 6); // I<I<H>>, trivial
    assert!(<V8 as Dec>::Out::VAL == 7); // O<O<O<H>>>, carry chain depth 2
    assert!(<V9 as Dec>::Out::VAL == 8); // I<O<O<H>>>, trivial
    assert!(<V10 as Dec>::Out::VAL == 9); // O<I<O<H>>>, reduces after one step
    assert!(<V11 as Dec>::Out::VAL == 10); // I<I<O<H>>>, trivial
    assert!(<V12 as Dec>::Out::VAL == 11); // O<O<I<H>>>, carry chain depth 1 through I
    assert!(<V13 as Dec>::Out::VAL == 12); // I<O<I<H>>>, trivial
    assert!(<V14 as Dec>::Out::VAL == 13); // O<I<I<H>>>, reduces after one step
    assert!(<V15 as Dec>::Out::VAL == 14); // I<I<I<H>>>, trivial
    assert!(<V16 as Dec>::Out::VAL == 15); // O<O<O<O<H>>>>, carry chain depth 3
    assert!(<V17 as Dec>::Out::VAL == 16); // I<O<O<O<H>>>>, trivial
    assert!(<V24 as Dec>::Out::VAL == 23); // O<O<O<I<H>>>>, carry chain depth 2 through I
    assert!(<V31 as Dec>::Out::VAL == 30); // I<I<I<I<H>>>>, trivial
    assert!(<V32 as Dec>::Out::VAL == 31); // O<O<O<O<O<H>>>>>, carry chain depth 4

    // The far-point-of-index-space projection, total over Pos, using the
    // identical Dec machinery: the last valid index below capacities V13, H
    // and V32.
    assert!(last_index::<V13>() == 12);
    assert!(last_index::<H>() == 0);
    assert!(last_index::<V32>() == 31);
};

fn main() {}
