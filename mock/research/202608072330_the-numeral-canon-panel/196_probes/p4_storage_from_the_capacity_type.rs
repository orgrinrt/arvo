// p4: if the const-to-type bridge is unavailable, drop the const.
//
// WHY THIS RUNS. p3 established that `acc_width(W,C) = W + ceil(log2 C)`
// composes gate-free when the capacity is a TYPE, that the const-to-type map
// (B1), the type-to-array-length map (B2) and the associated-const-equality
// bridge (p3b) are all refused on the pinned toolchain, and that carrying both
// a const `K` and a capacity type `C` compiles with the two disagreeing (C1),
// which is the landmine `191` found at its own p1 arms G1 to G3.
//
// Every one of those problems exists only because the STORAGE wanted a const.
// So: can the storage be derived from the capacity type as well, by the same
// induction that derives the width, with no const arithmetic anywhere?
//
// The three constructors are already there and are already disjoint:
//     One          -> one slot
//     Twice<N>     -> two N-shaped blocks
//     TwiceP1<N>   -> two N-shaped blocks and one more slot
// which holds exactly `PosVal::VAL` slots of `T` by construction, and needs no
// array length and no arithmetic in a type position.
//
// If this compiles, the const never has to exist, C1's landmine has nothing to
// be inconsistent about, and the answer to "is a capacity a const or a type"
// is that carrying it as a type costs the consumer a spelling and buys back a
// definition-site refusal. That is a composition rather than a winner and both
// halves get stated.
//
// ARMS, and the run does not count without the two controls.
//   S1   the inductive storage exists for eight capacities        COMPILES
//   S2   its slot count equals PosVal::VAL, checked by const
//        assertion against integer arithmetic                     COMPILES
//   S2m  the same with the TwiceP1 arm's extra slot dropped
//        (--cfg mutate). MUST FAIL, or S2 is asserting nothing.    REFUSED
//   S3   a fold over it producing the p3-derived accumulator       COMPILES
//   S4   a capacity whose storage row is missing                   REFUSED
#![allow(dead_code)]

use core::marker::PhantomData;

// ---- the ladder and the binary naturals, as p8 ---------------------------
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
impl<N> Log2Ceil for TwiceP1<N>
where
    N: Inc,
    <N as Inc>::Out: Log2Ceil,
{
    type Out = Su<<<N as Inc>::Out as Log2Ceil>::Out>;
}

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

// ---- ARM S1: the storage, derived by the same induction ------------------
pub struct Slot<T>(pub T);
pub struct Pair<A, B>(pub A, pub B);

/// The shape a capacity's worth of `T` takes. Three impls, pairwise disjoint
/// by the same construction that makes `Log2Ceil` three impls. No array, no
/// length, no arithmetic in a type position.
pub trait Store<T> {
    type Shape;
    /// Slot count, for the S2 assertion only. Not used to build anything.
    const SLOTS: u64;
}
impl<T> Store<T> for One {
    type Shape = Slot<T>;
    const SLOTS: u64 = 1;
}
impl<T, N: Store<T>> Store<T> for Twice<N> {
    type Shape = Pair<<N as Store<T>>::Shape, <N as Store<T>>::Shape>;
    const SLOTS: u64 = 2 * <N as Store<T>>::SLOTS;
}
#[cfg(not(mutate))]
impl<T, N: Store<T>> Store<T> for TwiceP1<N> {
    type Shape = Pair<Pair<<N as Store<T>>::Shape, <N as Store<T>>::Shape>, Slot<T>>;
    const SLOTS: u64 = 2 * <N as Store<T>>::SLOTS + 1;
}
// The mutation: the odd constructor forgets its extra slot. The shape then
// holds 2N where the capacity says 2N+1, silently, for every odd capacity.
#[cfg(mutate)]
impl<T, N: Store<T>> Store<T> for TwiceP1<N> {
    type Shape = Pair<<N as Store<T>>::Shape, <N as Store<T>>::Shape>;
    const SLOTS: u64 = 2 * <N as Store<T>>::SLOTS;
}

// ---- capacities, including the odd ones the mutation breaks -------------
type C1 = One;
type C2 = Twice<One>;
type C3 = TwiceP1<One>;
type C4 = Twice<Twice<One>>;
type C5 = TwiceP1<Twice<One>>;
type C7 = TwiceP1<TwiceP1<One>>;
type C13 = TwiceP1<Twice<TwiceP1<One>>>;
type C16 = Twice<Twice<Twice<Twice<One>>>>;

// ---- ARM S2: the shape holds exactly the capacity's worth ---------------
// The claim is about the SHAPE, so the assertion has to reach the shape rather
// than compare two declarations to each other. `size_of` is what the shape
// actually lays out, so a shape with a missing slot is a smaller type and the
// arithmetic catches it.
macro_rules! slots_agree {
    ($c:ty) => {
        const _: () = {
            assert!(<$c as Store<u8>>::SLOTS == <$c as PosVal>::VAL);
        };
        const _: () = {
            // and the declaration is tied to something the compiler laid out:
            // one u8 per slot, so the shape's size is the capacity.
            assert!(
                core::mem::size_of::<<$c as Store<u8>>::Shape>() == <$c as PosVal>::VAL as usize
            );
        };
    };
}
slots_agree!(C1);
slots_agree!(C2);
slots_agree!(C3);
slots_agree!(C4);
slots_agree!(C5);
slots_agree!(C7);
slots_agree!(C13);
slots_agree!(C16);

// ---- ARM S3: a fold over the derived shape into the derived accumulator --
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
    fn cadd(self, r: Self) -> Self {
        Num::new(self.0.wrapping_add(r.0))
    }
    fn zero() -> Self {
        Num::new(0)
    }
}

pub trait SumAccum<C> {
    type Acc: CAdd + Copy;
    fn lift(self) -> Self::Acc;
}
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

/// Walking the derived shape is structural, so it is three impls again and it
/// carries no index and no length.
/// Walking the derived shape is structural, so it is three impls again and it
/// carries no index and no length. Parameterised by the CAPACITY, because the
/// leaf's lift is `SumAccum<C>` and a first draft parameterised it by the
/// accumulator instead, which asked the compiler for `Num<..>: Log2Ceil`.
pub trait FoldInto<C, A: CAdd> {
    fn fold_into(&self, acc: A) -> A;
}
impl<W, C, A: CAdd> FoldInto<C, A> for Slot<Num<W>>
where
    Num<W>: SumAccum<C, Acc = A>,
{
    fn fold_into(&self, acc: A) -> A {
        acc.cadd(self.0.lift())
    }
}
impl<C, A: CAdd, X: FoldInto<C, A>, Y: FoldInto<C, A>> FoldInto<C, A> for Pair<X, Y> {
    fn fold_into(&self, acc: A) -> A {
        self.1.fold_into(self.0.fold_into(acc))
    }
}

type N4 = Su<Su<Su<Su<Z>>>>;

pub fn s3_sum(store: &<C16 as Store<Num<N4>>>::Shape) -> <Num<N4> as SumAccum<C16>>::Acc
where
    <Num<N4> as SumAccum<C16>>::Acc: CAdd,
    <C16 as Store<Num<N4>>>::Shape: FoldInto<C16, <Num<N4> as SumAccum<C16>>::Acc>,
{
    // The capacity has to be NAMED here. The shape does not determine it:
    // `Pair<Slot<T>, Slot<T>>` is the storage for capacity 2 and is also a
    // sub-shape of the storage for 4, so inference has nothing to go on and
    // reports E0283. That is a real cost of deriving storage from the type
    // and it is recorded rather than hidden: a consumer writes the capacity
    // once at the fold, not once per element.
    <_ as FoldInto<C16, _>>::fold_into(store, <<Num<N4> as SumAccum<C16>>::Acc as CAdd>::zero())
}

// ---- ARM S4: a capacity with no storage row is refused at the use site ---
#[cfg(arm_s4)]
pub struct NoRow;
#[cfg(arm_s4)]
impl PosVal for NoRow {
    const VAL: u64 = 9;
}
#[cfg(arm_s4)]
pub fn s4_uncovered() -> <NoRow as Store<u8>>::Shape {
    unimplemented!()
}

// ---- ARM S5: does the derived tree lay out like a flat array? -----------
// The shape is a nest of `Pair`s at `repr(Rust)`, so its layout is unspecified
// and an odd capacity nests `Pair<Pair<X, X>, Slot<T>>`, which is where padding
// would appear if it appears at all. A shape that is bigger than the array it
// replaces is a cost a consumer has to know about, so it is measured rather
// than assumed, at an element with real alignment rather than at `u8`.
//
// REQUIRED: every assertion holds, at eight capacities, four of them odd, for
// both a 1-byte and an 8-byte element. If any fails the shape is not a
// drop-in for `[T; K]` on size and the finding says so.
macro_rules! packs_like_an_array {
    ($c:ty) => {
        const _: () = {
            assert!(
                core::mem::size_of::<<$c as Store<u8>>::Shape>()
                    == <$c as PosVal>::VAL as usize * core::mem::size_of::<u8>()
            );
        };
        const _: () = {
            assert!(
                core::mem::size_of::<<$c as Store<u64>>::Shape>()
                    == <$c as PosVal>::VAL as usize * core::mem::size_of::<u64>()
            );
        };
        const _: () = {
            assert!(
                core::mem::align_of::<<$c as Store<u64>>::Shape>() == core::mem::align_of::<u64>()
            );
        };
    };
}
packs_like_an_array!(C1);
packs_like_an_array!(C2);
packs_like_an_array!(C3);
packs_like_an_array!(C4);
packs_like_an_array!(C5);
packs_like_an_array!(C7);
packs_like_an_array!(C13);
packs_like_an_array!(C16);
