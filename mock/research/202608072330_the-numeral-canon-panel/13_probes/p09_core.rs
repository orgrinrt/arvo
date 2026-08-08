// P9 core. Everything the machinery needs, entirely at the type level, with
// ZERO feature gates: nat, addition, comparison, ceil-to-bytes, an exact-size
// byte buffer, and container selection across the native rungs plus the wide
// rung. The only const in the whole design is the literal a consumer writes.
#![no_std]
#![allow(dead_code)]

pub struct Z;
pub struct O<N>(N);
pub struct E<N>(N);

pub trait Add<R> {
    type Out;
}
pub trait AddC<R> {
    type Out;
}
pub type Sum<A, B> = <A as Add<B>>::Out;
pub type SumC<A, B> = <A as AddC<B>>::Out;

impl Add<Z> for Z {
    type Out = Z;
}
impl<B> Add<O<B>> for Z {
    type Out = O<B>;
}
impl<B> Add<E<B>> for Z {
    type Out = E<B>;
}
impl<A> Add<Z> for O<A> {
    type Out = O<A>;
}
impl<A> Add<Z> for E<A> {
    type Out = E<A>;
}
impl<A: AddC<B>, B> Add<O<B>> for O<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: Add<B>, B> Add<E<B>> for O<A> {
    type Out = O<Sum<A, B>>;
}
impl<A: Add<B>, B> Add<O<B>> for E<A> {
    type Out = O<Sum<A, B>>;
}
impl<A: Add<B>, B> Add<E<B>> for E<A> {
    type Out = E<Sum<A, B>>;
}

impl AddC<Z> for Z {
    type Out = O<Z>;
}
impl<B: AddC<Z>> AddC<O<B>> for Z {
    type Out = E<SumC<B, Z>>;
}
impl<B> AddC<E<B>> for Z {
    type Out = O<B>;
}
impl<A: AddC<Z>> AddC<Z> for O<A> {
    type Out = E<SumC<A, Z>>;
}
impl<A> AddC<Z> for E<A> {
    type Out = O<A>;
}
impl<A: AddC<B>, B> AddC<O<B>> for O<A> {
    type Out = O<SumC<A, B>>;
}
impl<A: AddC<B>, B> AddC<E<B>> for O<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: AddC<B>, B> AddC<O<B>> for E<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: Add<B>, B> AddC<E<B>> for E<A> {
    type Out = O<Sum<A, B>>;
}

// ordering, and the two finite selectors that consume it
pub struct Lt;
pub struct Eqq;
pub struct Gt;

pub trait OrElse<D> {
    type Out;
}
impl<D> OrElse<D> for Eqq {
    type Out = D;
}
impl<D> OrElse<D> for Lt {
    type Out = Lt;
}
impl<D> OrElse<D> for Gt {
    type Out = Gt;
}

pub trait IfLe<T, F> {
    type Out;
}
impl<T, F> IfLe<T, F> for Lt {
    type Out = T;
}
impl<T, F> IfLe<T, F> for Eqq {
    type Out = T;
}
impl<T, F> IfLe<T, F> for Gt {
    type Out = F;
}

pub trait Cmp<R> {
    type Out;
}
pub type Ord2<A, B> = <A as Cmp<B>>::Out;

impl Cmp<Z> for Z {
    type Out = Eqq;
}
impl<B> Cmp<O<B>> for Z {
    type Out = Lt;
}
impl<B> Cmp<E<B>> for Z {
    type Out = Lt;
}
impl<A> Cmp<Z> for O<A> {
    type Out = Gt;
}
impl<A> Cmp<Z> for E<A> {
    type Out = Gt;
}
impl<A: Cmp<B>, B> Cmp<O<B>> for O<A> {
    type Out = Ord2<A, B>;
}
impl<A: Cmp<B>, B> Cmp<E<B>> for E<A> {
    type Out = Ord2<A, B>;
}
impl<A: Cmp<B>, B> Cmp<E<B>> for O<A>
where
    Ord2<A, B>: OrElse<Gt>,
{
    type Out = <Ord2<A, B> as OrElse<Gt>>::Out;
}
impl<A: Cmp<B>, B> Cmp<O<B>> for E<A>
where
    Ord2<A, B>: OrElse<Lt>,
{
    type Out = <Ord2<A, B> as OrElse<Lt>>::Out;
}

// ceiling halve, three times, gives ceil(n/8) with no arithmetic anywhere
pub trait CeilHalf {
    type Out;
}
pub type CH<A> = <A as CeilHalf>::Out;
impl CeilHalf for Z {
    type Out = Z;
}
impl<A: AddC<Z>> CeilHalf for O<A> {
    type Out = SumC<A, Z>;
}
impl<A> CeilHalf for E<A> {
    type Out = A;
}

pub type Bytes<W> = CH<CH<CH<W>>>;

// an exact-size byte buffer built from the nat's binary structure, log depth
pub struct Nil;
#[repr(C)]
pub struct One<T>(u8, T, T);
#[repr(C)]
pub struct Two<T>(T, T);

pub trait Buf {
    type Out;
}
pub type BufOf<A> = <A as Buf>::Out;
impl Buf for Z {
    type Out = Nil;
}
impl<A: Buf> Buf for O<A> {
    type Out = One<BufOf<A>>;
}
impl<A: Buf> Buf for E<A> {
    type Out = Two<BufOf<A>>;
}

// container selection: five native rungs then the wide rung. Finite impls.
pub type Rung<W, Bound, Narrow, Wide> = <Ord2<W, Bound> as IfLe<Narrow, Wide>>::Out;
