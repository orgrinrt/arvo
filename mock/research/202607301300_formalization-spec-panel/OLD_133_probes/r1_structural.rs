// r1: is step A gate-free when the magnitude is a TYPE rather than a const?
// Little-endian binary nat. No feature gates, no -Z flag.
#![no_std]

pub struct Term; // 0
pub struct D0<T>(core::marker::PhantomData<T>); // 2n
pub struct D1<T>(core::marker::PhantomData<T>); // 2n + 1

// ---- value-position read, an ordinary associated const, computes freely ----
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

// ---- type-position addition, structural carry chain ----
pub trait Add<R> {
    type O;
}
impl Add<Term> for Term {
    type O = Term;
}
impl<B: Nat> Add<D0<B>> for Term {
    type O = D0<B>;
}
impl<B: Nat> Add<D1<B>> for Term {
    type O = D1<B>;
}
impl<A: Nat> Add<Term> for D0<A> {
    type O = D0<A>;
}
impl<A: Nat> Add<Term> for D1<A> {
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
    type O = D0<<<A as Add<B>>::O as Inc>::O>; // carry
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

// ---- bit length, as a unary nat: one impl per digit constructor ----
pub struct Z;
pub struct S<T>(core::marker::PhantomData<T>);
pub trait Len {
    type L;
}
impl Len for Term {
    type L = Z;
}
impl<T: Len> Len for D0<T> {
    type L = S<T::L>;
}
impl<T: Len> Len for D1<T> {
    type L = S<T::L>;
}

// ---- the rung, by peeling the unary length eight at a time ----
// Nine base impls (lengths 0..=8 hold in one byte) plus one peel impl.
// No width is enumerated: the peel is total and unbounded.
pub trait Bucket {
    type T: Copy;
}
macro_rules! base { ($($t:ty),*) => { $( impl Bucket for $t { type T = u8; } )* } }
type U0 = Z;
type U1 = S<U0>;
type U2 = S<U1>;
type U3 = S<U2>;
type U4 = S<U3>;
type U5 = S<U4>;
type U6 = S<U5>;
type U7 = S<U6>;
type U8 = S<U7>;
base!(U0, U1, U2, U3, U4, U5, U6, U7, U8);
// peel nine: length 9 or more steps to the next rung
pub trait Bucket1 {
    type T: Copy;
}
impl<T> Bucket for S<S<S<S<S<S<S<S<S<T>>>>>>>>>
where
    S<T>: Bucket1,
{
    type T = <S<T> as Bucket1>::T;
}
macro_rules! base1 { ($($t:ty),*) => { $( impl Bucket1 for $t { type T = u16; } )* } }
base1!(U1, U2, U3, U4, U5, U6, U7, U8);
pub trait Bucket2 {
    type T: Copy;
}
impl<T> Bucket1 for S<S<S<S<S<S<S<S<S<T>>>>>>>>>
where
    S<T>: Bucket2,
{
    type T = <S<T> as Bucket2>::T;
}
macro_rules! base2 { ($($t:ty),*) => { $( impl Bucket2 for $t { type T = u32; } )* } }
base2!(
    U1,
    U2,
    U3,
    U4,
    U5,
    U6,
    U7,
    U8,
    S<U8>,
    S<S<U8>>,
    S<S<S<U8>>>,
    S<S<S<S<U8>>>>,
    S<S<S<S<S<U8>>>>>,
    S<S<S<S<S<S<U8>>>>>>,
    S<S<S<S<S<S<S<U8>>>>>>>,
    S<S<S<S<S<S<S<S<U8>>>>>>>>
);

// ---- the numeral, keyed on structural coordinates ----
pub struct Hot;
pub trait Store<W> {
    type T: Copy;
}
impl<W: Len> Store<W> for Hot
where
    <W as Len>::L: Bucket,
{
    type T = <<W as Len>::L as Bucket>::T;
}

pub struct Fixed<I, F, S: Store<<I as Add<F>>::O>>
where
    I: Add<F>,
{
    raw: <S as Store<<I as Add<F>>::O>>::T,
    _m: core::marker::PhantomData<(I, F)>,
}

// ---- concrete sites: the container falls out with no const anywhere ----
type N3 = D1<D1<Term>>; // 3
type N13 = D1<D0<D1<D1<Term>>>>; // 13
type N16 = D0<D0<D0<D0<D1<Term>>>>>; // 16
const _: () = assert!(<N3 as Nat>::V == 3);
const _: () = assert!(<N13 as Nat>::V == 13);
const _: () = assert!(<<N13 as Add<N3>>::O as Nat>::V == 16);

pub fn q13_3(x: Fixed<N13, N3, Hot>) -> u16 {
    x.raw
} // 16 bits -> u16
pub fn q3_0(x: Fixed<N3, Term, Hot>) -> u8 {
    x.raw
} // 3 bits  -> u8
pub fn q16_16(x: Fixed<N16, N16, Hot>) -> u32 {
    x.raw
} // 32 bits -> u32

// ---- and the law is width-generic with no gate and no flag ----
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
