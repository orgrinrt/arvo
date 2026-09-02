// d3: carry the magnitude's VALUE as a const on the head of the structural numeral,
// so the value is present in the printed type. Nothing is transformed: the const is
// matched, never computed. Does the E0308 print the number?
#![no_std]
use core::marker::PhantomData;

pub struct Term;
pub struct D0<T>(PhantomData<T>);
pub struct D1<T>(PhantomData<T>);

// the head. V is carried alongside the digits, never computed from them.
pub struct W<const V: u32, D>(PhantomData<D>);

pub trait Nat {
    const V: u32;
}
impl Nat for Term {
    const V: u32 = 0;
}
impl<T: Nat> Nat for D0<T> {
    const V: u32 = 2 * T::V;
}
impl<T: Nat> Nat for D1<T> {
    const V: u32 = 2 * T::V + 1;
}
impl<const V: u32, D: Nat> Nat for W<V, D> {
    const V: u32 = <D as Nat>::V;
}

pub trait Inc {
    type O;
}
impl Inc for Term {
    type O = D1<Term>;
}
impl<T> Inc for D0<T> {
    type O = D1<T>;
}
impl<T: Inc> Inc for D1<T> {
    type O = D0<<T as Inc>::O>;
}

pub trait Dec {
    type O;
}
impl<T> Dec for D1<T> {
    type O = D0<T>;
}
impl<T: Dec> Dec for D0<T> {
    type O = D1<<T as Dec>::O>;
}

pub trait Add<R> {
    type O;
}
impl Add<Term> for Term {
    type O = Term;
}
impl<B> Add<D0<B>> for Term {
    type O = D0<B>;
}
impl<B> Add<D1<B>> for Term {
    type O = D1<B>;
}
impl<A> Add<Term> for D0<A> {
    type O = D0<A>;
}
impl<A> Add<Term> for D1<A> {
    type O = D1<A>;
}
impl<A: Add<B>, B> Add<D0<B>> for D0<A> {
    type O = D0<<A as Add<B>>::O>;
}
impl<A: Add<B>, B> Add<D1<B>> for D0<A> {
    type O = D1<<A as Add<B>>::O>;
}
impl<A: Add<B>, B> Add<D0<B>> for D1<A> {
    type O = D1<<A as Add<B>>::O>;
}
impl<A: Add<B>, B> Add<D1<B>> for D1<A>
where
    <A as Add<B>>::O: Inc,
{
    type O = D0<<<A as Add<B>>::O as Inc>::O>;
}

pub struct Z;
pub struct S<T>(PhantomData<T>);
pub trait Bump {
    type O;
}
impl Bump for Z {
    type O = Z;
}
impl<T> Bump for S<T> {
    type O = S<S<T>>;
}
pub trait Len {
    type L;
}
impl Len for Term {
    type L = Z;
}
impl<T: Len> Len for D0<T>
where
    T::L: Bump,
{
    type L = <T::L as Bump>::O;
}
impl<T: Len> Len for D1<T> {
    type L = S<T::L>;
}

#[derive(Copy, Clone, Default)]
pub struct WideNil;
pub trait Rung {
    type T: Copy;
}
type L0 = Z;
type L1 = S<L0>;
type L2 = S<L1>;
type L3 = S<L2>;
type L4 = S<L3>;
type L5 = S<L4>;
type L6 = S<L5>;
type L7 = S<L6>;
impl Rung for L0 {
    type T = u8;
}
impl Rung for L1 {
    type T = u8;
}
impl Rung for L2 {
    type T = u8;
}
impl Rung for L3 {
    type T = u8;
}
impl Rung for L4 {
    type T = u16;
}
impl Rung for L5 {
    type T = u32;
}
impl Rung for L6 {
    type T = u64;
}
impl Rung for L7 {
    type T = u128;
}
impl<T> Rung for S<S<S<S<S<S<S<S<T>>>>>>>> {
    type T = WideNil;
}

pub struct Hot;
pub trait Store<X> {
    type T: Copy;
}
impl<X: Dec> Store<X> for Hot
where
    <X as Dec>::O: Len,
    <<X as Dec>::O as Len>::L: Rung,
{
    type T = <<<X as Dec>::O as Len>::L as Rung>::T;
}

// the sum of two heads carries the sum's value only where the writer supplied it,
// so Add on heads is defined only through the digits and the head is REBUILT with
// a value the writer names. Here: keep the head only on the leaves and let derived
// coordinates be bare digit strings. Two variants tested below.
pub struct Fixed<I, F, S>(<S as Store<<I as Add<F>>::O>>::T, PhantomData<(I, F)>)
where
    I: Add<F>,
    S: Store<<I as Add<F>>::O>;

// Add through the head: the OUTPUT head's const cannot be computed, so it is
// dropped and the output is a bare digit string. This is the honest shape.
impl<const A: u32, DA: Add<DB>, const B: u32, DB> Add<W<B, DB>> for W<A, DA> {
    type O = <DA as Add<DB>>::O;
}
impl<const A: u32, DA, B> Store<W<A, DA>> for Hot
where
    Hot: Store2<DA>,
{
    type T = <Hot as Store2<DA>>::T;
}
pub trait Store2<X> {
    type T: Copy;
}
impl<X: Dec> Store2<X> for Hot
where
    <X as Dec>::O: Len,
    <<X as Dec>::O as Len>::L: Rung,
{
    type T = <<<X as Dec>::O as Len>::L as Rung>::T;
}

type T3 = D1<D1<Term>>;
type T13 = D1<D0<D1<D1<Term>>>>;
type T16 = D0<D0<D0<D0<D1<Term>>>>>;
type T6 = D0<D1<D1<Term>>>;
type T26 = D0<D1<D0<D1<D1<Term>>>>>;
pub type N3 = W<3, T3>;
pub type N13 = W<13, T13>;
pub type N16 = W<16, T16>;
pub type N6 = W<6, T6>;
pub type N26 = W<26, T26>;

pub fn q13_3(x: Fixed<N13, N3, Hot>) -> u16 {
    x.0
}

pub fn mul<I, F, J, K, S>(
    _a: Fixed<I, F, S>,
    _b: Fixed<J, K, S>,
) -> Fixed<<I as Add<J>>::O, <F as Add<K>>::O, S>
where
    I: Add<F> + Add<J>,
    J: Add<K>,
    F: Add<K>,
    <I as Add<J>>::O: Add<<F as Add<K>>::O>,
    S: Store<<I as Add<F>>::O>
        + Store<<J as Add<K>>::O>
        + Store<<<I as Add<J>>::O as Add<<F as Add<K>>::O>>::O>,
{
    todo!()
}

pub fn wrong(a: Fixed<N13, N3, Hot>, b: Fixed<N13, N3, Hot>) -> Fixed<N16, N6, Hot> {
    mul(a, b)
}
