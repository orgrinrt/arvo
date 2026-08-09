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

type I0 = D0<D0<D0<D1<D0<D1<D1<D1<D1<D1<Term>>>>>>>>>>;
type F0 = D1<D1<D0<D1<D0<D1<D1<D1<D1<D1<Term>>>>>>>>>>;
const _: () = assert!(<<I0 as Add<F0>>::O as Nat>::V == 2003);
pub fn site0(x: Fixed<I0, F0, Hot>) -> WideNil {
    x.0
}
type I1 = D1<D1<D1<D0<D1<D0<D0<D0<D0<D0<D1<Term>>>>>>>>>>>;
type F1 = D0<D1<D0<D1<D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I1 as Add<F1>>::O as Nat>::V == 2081);
pub fn site1(x: Fixed<I1, F1, Hot>) -> WideNil {
    x.0
}
type I2 = D0<D1<D1<D0<D0<D0<D1<D0<D0<D0<D1<Term>>>>>>>>>>>;
type F2 = D1<D0<D0<D1<D0<D1<D0<D0<D0<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I2 as Add<F2>>::O as Nat>::V == 2159);
pub fn site2(x: Fixed<I2, F2, Hot>) -> WideNil {
    x.0
}
type I3 = D1<D0<D1<D0<D1<D1<D1<D0<D0<D0<D1<Term>>>>>>>>>>>;
type F3 = D0<D0<D0<D1<D0<D0<D1<D0<D0<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I3 as Add<F3>>::O as Nat>::V == 2237);
pub fn site3(x: Fixed<I3, F3, Hot>) -> WideNil {
    x.0
}
type I4 = D0<D0<D1<D0<D0<D1<D0<D1<D0<D0<D1<Term>>>>>>>>>>>;
type F4 = D1<D1<D1<D0<D0<D1<D1<D0<D0<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I4 as Add<F4>>::O as Nat>::V == 2315);
pub fn site4(x: Fixed<I4, F4, Hot>) -> WideNil {
    x.0
}
type I5 = D1<D1<D0<D0<D1<D0<D1<D1<D0<D0<D1<Term>>>>>>>>>>>;
type F5 = D0<D1<D1<D0<D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I5 as Add<F5>>::O as Nat>::V == 2393);
pub fn site5(x: Fixed<I5, F5, Hot>) -> WideNil {
    x.0
}
type I6 = D0<D1<D0<D0<D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>>>>>;
type F6 = D1<D0<D1<D0<D0<D1<D0<D1<D0<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I6 as Add<F6>>::O as Nat>::V == 2471);
pub fn site6(x: Fixed<I6, F6, Hot>) -> WideNil {
    x.0
}
type I7 = D1<D0<D0<D0<D1<D1<D0<D0<D1<D0<D1<Term>>>>>>>>>>>;
type F7 = D0<D0<D1<D0<D0<D0<D1<D1<D0<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I7 as Add<F7>>::O as Nat>::V == 2549);
pub fn site7(x: Fixed<I7, F7, Hot>) -> WideNil {
    x.0
}
type I8 = D0<D0<D0<D0<D0<D1<D1<D0<D1<D0<D1<Term>>>>>>>>>>>;
type F8 = D1<D1<D0<D0<D0<D1<D1<D1<D0<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I8 as Add<F8>>::O as Nat>::V == 2627);
pub fn site8(x: Fixed<I8, F8, Hot>) -> WideNil {
    x.0
}
type I9 = D1<D1<D1<D1<D0<D0<D0<D1<D1<D0<D1<Term>>>>>>>>>>>;
type F9 = D0<D1<D0<D0<D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I9 as Add<F9>>::O as Nat>::V == 2705);
pub fn site9(x: Fixed<I9, F9, Hot>) -> WideNil {
    x.0
}
type I10 = D0<D1<D1<D1<D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>>>>>;
type F10 = D1<D0<D0<D0<D0<D1<D0<D0<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I10 as Add<F10>>::O as Nat>::V == 2783);
pub fn site10(x: Fixed<I10, F10, Hot>) -> WideNil {
    x.0
}
type I11 = D1<D0<D1<D1<D0<D1<D1<D1<D1<D0<D1<Term>>>>>>>>>>>;
type F11 = D0<D0<D0<D0<D0<D0<D1<D0<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I11 as Add<F11>>::O as Nat>::V == 2861);
pub fn site11(x: Fixed<I11, F11, Hot>) -> WideNil {
    x.0
}
type I12 = D0<D0<D1<D1<D1<D0<D0<D0<D0<D1<D1<Term>>>>>>>>>>>;
type F12 = D1<D1<D1<D1<D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I12 as Add<F12>>::O as Nat>::V == 2939);
pub fn site12(x: Fixed<I12, F12, Hot>) -> WideNil {
    x.0
}
type I13 = D1<D1<D0<D1<D0<D0<D1<D0<D0<D1<D1<Term>>>>>>>>>>>;
type F13 = D0<D1<D1<D1<D1<D1<D1<D0<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I13 as Add<F13>>::O as Nat>::V == 3017);
pub fn site13(x: Fixed<I13, F13, Hot>) -> WideNil {
    x.0
}
type I14 = D0<D1<D0<D1<D1<D1<D1<D0<D0<D1<D1<Term>>>>>>>>>>>;
type F14 = D1<D0<D1<D1<D1<D0<D0<D1<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I14 as Add<F14>>::O as Nat>::V == 3095);
pub fn site14(x: Fixed<I14, F14, Hot>) -> WideNil {
    x.0
}
type I15 = D1<D0<D0<D1<D0<D1<D0<D1<D0<D1<D1<Term>>>>>>>>>>>;
type F15 = D0<D0<D1<D1<D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I15 as Add<F15>>::O as Nat>::V == 3173);
pub fn site15(x: Fixed<I15, F15, Hot>) -> WideNil {
    x.0
}
type I16 = D0<D0<D0<D1<D1<D0<D1<D1<D0<D1<D1<Term>>>>>>>>>>>;
type F16 = D1<D1<D0<D1<D1<D0<D1<D1<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I16 as Add<F16>>::O as Nat>::V == 3251);
pub fn site16(x: Fixed<I16, F16, Hot>) -> WideNil {
    x.0
}
type I17 = D1<D1<D1<D0<D0<D0<D0<D0<D1<D1<D1<Term>>>>>>>>>>>;
type F17 = D0<D1<D0<D1<D1<D1<D1<D1<D1<D0<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I17 as Add<F17>>::O as Nat>::V == 3329);
pub fn site17(x: Fixed<I17, F17, Hot>) -> WideNil {
    x.0
}
type I18 = D0<D1<D1<D0<D1<D1<D0<D0<D1<D1<D1<Term>>>>>>>>>>>;
type F18 = D1<D0<D0<D1<D1<D0<D0<D0<D0<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I18 as Add<F18>>::O as Nat>::V == 3407);
pub fn site18(x: Fixed<I18, F18, Hot>) -> WideNil {
    x.0
}
type I19 = D1<D0<D1<D0<D0<D1<D1<D0<D1<D1<D1<Term>>>>>>>>>>>;
type F19 = D0<D0<D0<D1<D1<D1<D0<D0<D0<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I19 as Add<F19>>::O as Nat>::V == 3485);
pub fn site19(x: Fixed<I19, F19, Hot>) -> WideNil {
    x.0
}
type I20 = D0<D0<D1<D0<D1<D0<D0<D1<D1<D1<D1<Term>>>>>>>>>>>;
type F20 = D1<D1<D1<D0<D1<D0<D1<D0<D0<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I20 as Add<F20>>::O as Nat>::V == 3563);
pub fn site20(x: Fixed<I20, F20, Hot>) -> WideNil {
    x.0
}
type I21 = D1<D1<D0<D0<D0<D0<D1<D1<D1<D1<D1<Term>>>>>>>>>>>;
type F21 = D0<D1<D1<D0<D1<D1<D1<D0<D0<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I21 as Add<F21>>::O as Nat>::V == 3641);
pub fn site21(x: Fixed<I21, F21, Hot>) -> WideNil {
    x.0
}
type I22 = D0<D1<D0<D0<D1<D1<D1<D1<D1<D1<D1<Term>>>>>>>>>>>;
type F22 = D1<D0<D1<D0<D1<D0<D0<D1<D0<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I22 as Add<F22>>::O as Nat>::V == 3719);
pub fn site22(x: Fixed<I22, F22, Hot>) -> WideNil {
    x.0
}
type I23 = D1<D0<D0<D0<D0<D1<D0<D0<D0<D0<D0<D1<Term>>>>>>>>>>>>;
type F23 = D0<D0<D1<D0<D1<D1<D0<D1<D0<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I23 as Add<F23>>::O as Nat>::V == 3797);
pub fn site23(x: Fixed<I23, F23, Hot>) -> WideNil {
    x.0
}
type I24 = D0<D0<D0<D0<D1<D0<D1<D0<D0<D0<D0<D1<Term>>>>>>>>>>>>;
type F24 = D1<D1<D0<D0<D1<D0<D1<D1<D0<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I24 as Add<F24>>::O as Nat>::V == 3875);
pub fn site24(x: Fixed<I24, F24, Hot>) -> WideNil {
    x.0
}
type I25 = D1<D1<D1<D1<D1<D1<D1<D0<D0<D0<D0<D1<Term>>>>>>>>>>>>;
type F25 = D0<D1<D0<D0<D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I25 as Add<F25>>::O as Nat>::V == 3953);
pub fn site25(x: Fixed<I25, F25, Hot>) -> WideNil {
    x.0
}
type I26 = D0<D1<D1<D1<D0<D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>>>>>>;
type F26 = D1<D0<D0<D0<D1<D0<D0<D0<D1<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I26 as Add<F26>>::O as Nat>::V == 4031);
pub fn site26(x: Fixed<I26, F26, Hot>) -> WideNil {
    x.0
}
type I27 = D1<D0<D1<D1<D1<D0<D1<D1<D0<D0<D0<D1<Term>>>>>>>>>>>>;
type F27 = D0<D0<D0<D0<D1<D1<D0<D0<D1<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I27 as Add<F27>>::O as Nat>::V == 4109);
pub fn site27(x: Fixed<I27, F27, Hot>) -> WideNil {
    x.0
}
type I28 = D0<D0<D1<D1<D0<D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>;
type F28 = D1<D1<D1<D1<D0<D0<D1<D0<D1<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I28 as Add<F28>>::O as Nat>::V == 4187);
pub fn site28(x: Fixed<I28, F28, Hot>) -> WideNil {
    x.0
}
type I29 = D1<D1<D0<D1<D1<D1<D0<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>;
type F29 = D0<D1<D1<D1<D0<D1<D1<D0<D1<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I29 as Add<F29>>::O as Nat>::V == 4265);
pub fn site29(x: Fixed<I29, F29, Hot>) -> WideNil {
    x.0
}
type I30 = D0<D1<D0<D1<D0<D1<D1<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>;
type F30 = D1<D0<D1<D1<D0<D0<D0<D1<D1<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I30 as Add<F30>>::O as Nat>::V == 4343);
pub fn site30(x: Fixed<I30, F30, Hot>) -> WideNil {
    x.0
}
type I31 = D1<D0<D0<D1<D1<D0<D0<D1<D1<D0<D0<D1<Term>>>>>>>>>>>>;
type F31 = D0<D0<D1<D1<D0<D1<D0<D1<D1<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I31 as Add<F31>>::O as Nat>::V == 4421);
pub fn site31(x: Fixed<I31, F31, Hot>) -> WideNil {
    x.0
}
type I32 = D0<D0<D0<D1<D0<D0<D1<D1<D1<D0<D0<D1<Term>>>>>>>>>>>>;
type F32 = D1<D1<D0<D1<D0<D0<D1<D1<D1<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I32 as Add<F32>>::O as Nat>::V == 4499);
pub fn site32(x: Fixed<I32, F32, Hot>) -> WideNil {
    x.0
}
type I33 = D1<D1<D1<D0<D1<D1<D1<D1<D1<D0<D0<D1<Term>>>>>>>>>>>>;
type F33 = D0<D1<D0<D1<D0<D1<D1<D1<D1<D1<D1<Term>>>>>>>>>>>;
const _: () = assert!(<<I33 as Add<F33>>::O as Nat>::V == 4577);
pub fn site33(x: Fixed<I33, F33, Hot>) -> WideNil {
    x.0
}
type I34 = D0<D1<D1<D0<D0<D1<D0<D0<D0<D1<D0<D1<Term>>>>>>>>>>>>;
type F34 = D1<D0<D0<D1<D0<D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I34 as Add<F34>>::O as Nat>::V == 4655);
pub fn site34(x: Fixed<I34, F34, Hot>) -> WideNil {
    x.0
}
type I35 = D1<D0<D1<D0<D1<D0<D1<D0<D0<D1<D0<D1<Term>>>>>>>>>>>>;
type F35 = D0<D0<D0<D1<D0<D1<D0<D0<D0<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I35 as Add<F35>>::O as Nat>::V == 4733);
pub fn site35(x: Fixed<I35, F35, Hot>) -> WideNil {
    x.0
}
type I36 = D0<D0<D1<D0<D0<D0<D0<D1<D0<D1<D0<D1<Term>>>>>>>>>>>>;
type F36 = D1<D1<D1<D0<D0<D0<D1<D0<D0<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I36 as Add<F36>>::O as Nat>::V == 4811);
pub fn site36(x: Fixed<I36, F36, Hot>) -> WideNil {
    x.0
}
type I37 = D1<D1<D0<D0<D1<D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>>>>>>;
type F37 = D0<D1<D1<D0<D0<D1<D1<D0<D0<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I37 as Add<F37>>::O as Nat>::V == 4889);
pub fn site37(x: Fixed<I37, F37, Hot>) -> WideNil {
    x.0
}
type I38 = D0<D1<D0<D0<D0<D1<D1<D1<D0<D1<D0<D1<Term>>>>>>>>>>>>;
type F38 = D1<D0<D1<D0<D0<D0<D0<D1<D0<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I38 as Add<F38>>::O as Nat>::V == 4967);
pub fn site38(x: Fixed<I38, F38, Hot>) -> WideNil {
    x.0
}
type I39 = D1<D0<D0<D0<D1<D0<D0<D0<D1<D1<D0<D1<Term>>>>>>>>>>>>;
type F39 = D0<D0<D1<D0<D0<D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I39 as Add<F39>>::O as Nat>::V == 5045);
pub fn site39(x: Fixed<I39, F39, Hot>) -> WideNil {
    x.0
}
type I40 = D0<D0<D0<D0<D0<D0<D1<D0<D1<D1<D0<D1<Term>>>>>>>>>>>>;
type F40 = D1<D1<D0<D0<D0<D0<D1<D1<D0<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I40 as Add<F40>>::O as Nat>::V == 5123);
pub fn site40(x: Fixed<I40, F40, Hot>) -> WideNil {
    x.0
}
type I41 = D1<D1<D1<D1<D0<D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>>>>>>;
type F41 = D0<D1<D0<D0<D0<D1<D1<D1<D0<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I41 as Add<F41>>::O as Nat>::V == 5201);
pub fn site41(x: Fixed<I41, F41, Hot>) -> WideNil {
    x.0
}
type I42 = D0<D1<D1<D1<D1<D0<D0<D1<D1<D1<D0<D1<Term>>>>>>>>>>>>;
type F42 = D1<D0<D0<D0<D0<D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I42 as Add<F42>>::O as Nat>::V == 5279);
pub fn site42(x: Fixed<I42, F42, Hot>) -> WideNil {
    x.0
}
type I43 = D1<D0<D1<D1<D0<D0<D1<D1<D1<D1<D0<D1<Term>>>>>>>>>>>>;
type F43 = D0<D0<D0<D0<D0<D1<D0<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I43 as Add<F43>>::O as Nat>::V == 5357);
pub fn site43(x: Fixed<I43, F43, Hot>) -> WideNil {
    x.0
}
type I44 = D0<D0<D1<D1<D1<D1<D1<D1<D1<D1<D0<D1<Term>>>>>>>>>>>>;
type F44 = D1<D1<D1<D1<D1<D1<D0<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I44 as Add<F44>>::O as Nat>::V == 5435);
pub fn site44(x: Fixed<I44, F44, Hot>) -> WideNil {
    x.0
}
type I45 = D1<D1<D0<D1<D0<D1<D0<D0<D0<D0<D1<D1<Term>>>>>>>>>>>>;
type F45 = D0<D1<D1<D1<D1<D0<D1<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I45 as Add<F45>>::O as Nat>::V == 5513);
pub fn site45(x: Fixed<I45, F45, Hot>) -> WideNil {
    x.0
}
type I46 = D0<D1<D0<D1<D1<D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>>>>>>;
type F46 = D1<D0<D1<D1<D1<D1<D1<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I46 as Add<F46>>::O as Nat>::V == 5591);
pub fn site46(x: Fixed<I46, F46, Hot>) -> WideNil {
    x.0
}
type I47 = D1<D0<D0<D1<D0<D0<D0<D1<D0<D0<D1<D1<Term>>>>>>>>>>>>;
type F47 = D0<D0<D1<D1<D1<D0<D0<D1<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I47 as Add<F47>>::O as Nat>::V == 5669);
pub fn site47(x: Fixed<I47, F47, Hot>) -> WideNil {
    x.0
}
type I48 = D0<D0<D0<D1<D1<D1<D0<D1<D0<D0<D1<D1<Term>>>>>>>>>>>>;
type F48 = D1<D1<D0<D1<D1<D1<D0<D1<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I48 as Add<F48>>::O as Nat>::V == 5747);
pub fn site48(x: Fixed<I48, F48, Hot>) -> WideNil {
    x.0
}
type I49 = D1<D1<D1<D0<D0<D1<D1<D1<D0<D0<D1<D1<Term>>>>>>>>>>>>;
type F49 = D0<D1<D0<D1<D1<D0<D1<D1<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I49 as Add<F49>>::O as Nat>::V == 5825);
pub fn site49(x: Fixed<I49, F49, Hot>) -> WideNil {
    x.0
}
type I50 = D0<D1<D1<D0<D1<D0<D0<D0<D1<D0<D1<D1<Term>>>>>>>>>>>>;
type F50 = D1<D0<D0<D1<D1<D1<D1<D1<D1<D0<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I50 as Add<F50>>::O as Nat>::V == 5903);
pub fn site50(x: Fixed<I50, F50, Hot>) -> WideNil {
    x.0
}
type I51 = D1<D0<D1<D0<D0<D0<D1<D0<D1<D0<D1<D1<Term>>>>>>>>>>>>;
type F51 = D0<D0<D0<D1<D1<D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I51 as Add<F51>>::O as Nat>::V == 5981);
pub fn site51(x: Fixed<I51, F51, Hot>) -> WideNil {
    x.0
}
type I52 = D0<D0<D1<D0<D1<D1<D1<D0<D1<D0<D1<D1<Term>>>>>>>>>>>>;
type F52 = D1<D1<D1<D0<D1<D1<D0<D0<D0<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I52 as Add<F52>>::O as Nat>::V == 6059);
pub fn site52(x: Fixed<I52, F52, Hot>) -> WideNil {
    x.0
}
type I53 = D1<D1<D0<D0<D0<D1<D0<D1<D1<D0<D1<D1<Term>>>>>>>>>>>>;
type F53 = D0<D1<D1<D0<D1<D0<D1<D0<D0<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I53 as Add<F53>>::O as Nat>::V == 6137);
pub fn site53(x: Fixed<I53, F53, Hot>) -> WideNil {
    x.0
}
type I54 = D0<D1<D0<D0<D1<D0<D1<D1<D1<D0<D1<D1<Term>>>>>>>>>>>>;
type F54 = D1<D0<D1<D0<D1<D1<D1<D0<D0<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I54 as Add<F54>>::O as Nat>::V == 6215);
pub fn site54(x: Fixed<I54, F54, Hot>) -> WideNil {
    x.0
}
type I55 = D1<D0<D0<D0<D0<D0<D0<D0<D0<D1<D1<D1<Term>>>>>>>>>>>>;
type F55 = D0<D0<D1<D0<D1<D0<D0<D1<D0<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I55 as Add<F55>>::O as Nat>::V == 6293);
pub fn site55(x: Fixed<I55, F55, Hot>) -> WideNil {
    x.0
}
type I56 = D0<D0<D0<D0<D1<D1<D0<D0<D0<D1<D1<D1<Term>>>>>>>>>>>>;
type F56 = D1<D1<D0<D0<D1<D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I56 as Add<F56>>::O as Nat>::V == 6371);
pub fn site56(x: Fixed<I56, F56, Hot>) -> WideNil {
    x.0
}
type I57 = D1<D1<D1<D1<D1<D0<D1<D0<D0<D1<D1<D1<Term>>>>>>>>>>>>;
type F57 = D0<D1<D0<D0<D1<D0<D1<D1<D0<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I57 as Add<F57>>::O as Nat>::V == 6449);
pub fn site57(x: Fixed<I57, F57, Hot>) -> WideNil {
    x.0
}
type I58 = D0<D1<D1<D1<D0<D0<D0<D1<D0<D1<D1<D1<Term>>>>>>>>>>>>;
type F58 = D1<D0<D0<D0<D1<D1<D1<D1<D0<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I58 as Add<F58>>::O as Nat>::V == 6527);
pub fn site58(x: Fixed<I58, F58, Hot>) -> WideNil {
    x.0
}
type I59 = D1<D0<D1<D1<D1<D1<D0<D1<D0<D1<D1<D1<Term>>>>>>>>>>>>;
type F59 = D0<D0<D0<D0<D1<D0<D0<D0<D1<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I59 as Add<F59>>::O as Nat>::V == 6605);
pub fn site59(x: Fixed<I59, F59, Hot>) -> WideNil {
    x.0
}
type I60 = D0<D0<D1<D1<D0<D1<D1<D1<D0<D1<D1<D1<Term>>>>>>>>>>>>;
type F60 = D1<D1<D1<D1<D0<D1<D0<D0<D1<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I60 as Add<F60>>::O as Nat>::V == 6683);
pub fn site60(x: Fixed<I60, F60, Hot>) -> WideNil {
    x.0
}
type I61 = D1<D1<D0<D1<D1<D0<D0<D0<D1<D1<D1<D1<Term>>>>>>>>>>>>;
type F61 = D0<D1<D1<D1<D0<D0<D1<D0<D1<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I61 as Add<F61>>::O as Nat>::V == 6761);
pub fn site61(x: Fixed<I61, F61, Hot>) -> WideNil {
    x.0
}
type I62 = D0<D1<D0<D1<D0<D0<D1<D0<D1<D1<D1<D1<Term>>>>>>>>>>>>;
type F62 = D1<D0<D1<D1<D0<D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I62 as Add<F62>>::O as Nat>::V == 6839);
pub fn site62(x: Fixed<I62, F62, Hot>) -> WideNil {
    x.0
}
type I63 = D1<D0<D0<D1<D1<D1<D1<D0<D1<D1<D1<D1<Term>>>>>>>>>>>>;
type F63 = D0<D0<D1<D1<D0<D0<D0<D1<D1<D1<D0<D1<Term>>>>>>>>>>>>;
const _: () = assert!(<<I63 as Add<F63>>::O as Nat>::V == 6917);
pub fn site63(x: Fixed<I63, F63, Hot>) -> WideNil {
    x.0
}
