// r2: the container ladder from a structurally-keyed magnitude. No gates, no flag.
// The rung of a width W is the binary digit count of W-1, which is structural.
#![no_std]
use core::marker::PhantomData;

pub struct Term; // 0
pub struct D0<T>(PhantomData<T>); // 2n
pub struct D1<T>(PhantomData<T>); // 2n + 1

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
} // 2n+1 -> 2n
impl<T: Dec> Dec for D0<T> {
    type O = D1<<T as Dec>::O>;
} // 2n -> 2(n-1)+1

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

// digit count, as a unary tally
pub struct Z;
pub struct S<T>(PhantomData<T>);
pub trait Bump {
    type O;
}
impl Bump for Z {
    type O = Z;
} // trailing zeros are not digits
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

// the rung: digit count of W-1. 0..=3 -> u8, 4 -> u16, 5 -> u32, 6 -> u64,
// 7 -> u128, 8 or more -> wide. Nine impls, total, no width enumerated.
#[derive(Copy, Clone, Default)]
pub struct WideCons<T>(pub u8, pub T);
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
} // 8 or more digits

pub struct Hot;
pub trait Store<W> {
    type T: Copy;
}
impl<W: Dec> Store<W> for Hot
where
    <W as Dec>::O: Len,
    <<W as Dec>::O as Len>::L: Rung,
{
    type T = <<<W as Dec>::O as Len>::L as Rung>::T;
}

pub struct Fixed<I, F, S>(<S as Store<<I as Add<F>>::O>>::T, PhantomData<(I, F)>)
where
    I: Add<F>,
    S: Store<<I as Add<F>>::O>;

// concrete sites
type N0 = Term;
type N3 = D1<D1<Term>>;
type N13 = D1<D0<D1<D1<Term>>>>;
type N16 = D0<D0<D0<D0<D1<Term>>>>>;
type N30 = D0<D1<D1<D1<D1<Term>>>>>;
type N100 = D0<D0<D1<D0<D0<D1<D1<Term>>>>>>>;
const _: () = assert!(
    <N13 as Nat>::V == 13
        && <N16 as Nat>::V == 16
        && <N30 as Nat>::V == 30
        && <N100 as Nat>::V == 100
);
const _: () = assert!(<<N13 as Add<N3>>::O as Nat>::V == 16);

pub fn q13_3(x: Fixed<N13, N3, Hot>) -> u16 {
    x.0
} // 16 bits -> u16
pub fn q3_0(x: Fixed<N3, N0, Hot>) -> u8 {
    x.0
} // 3 bits  -> u8
pub fn q16_16(x: Fixed<N16, N16, Hot>) -> u32 {
    x.0
} // 32 bits -> u32
pub fn q30_3(x: Fixed<N30, N3, Hot>) -> u64 {
    x.0
} // 33 bits -> u64
pub fn q100_30(x: Fixed<N100, N30, Hot>) -> WideNil {
    x.0
} // 130 bits -> wide rung

// ---- does the D48 surface survive as an alias over the structural encoding? ----
pub struct Idx<const N: u32>;
pub trait ToNat {
    type N;
}
impl ToNat for Idx<0> {
    type N = Term;
}
impl ToNat for Idx<3> {
    type N = N3;
}
impl ToNat for Idx<13> {
    type N = N13;
}
impl ToNat for Idx<16> {
    type N = N16;
}

pub type UFixed<const I: u32, const F: u32, S> =
    Fixed<<Idx<I> as ToNat>::N, <Idx<F> as ToNat>::N, S>;

pub fn surface(x: UFixed<13, 3, Hot>) -> u16 {
    x.0
}

// ---- and the law, width-generic over the structural coordinates ----
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

pub fn law_site(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> Fixed<N26, N6, Hot> {
    mul(a, b)
}
type N6 = D0<D1<D1<Term>>>;
type N26 = D0<D1<D0<D1<D1<Term>>>>>;
const _: () = assert!(<N6 as Nat>::V == 6 && <N26 as Nat>::V == 26);
