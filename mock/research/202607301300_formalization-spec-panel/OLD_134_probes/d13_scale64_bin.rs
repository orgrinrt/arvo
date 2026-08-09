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

pub type A0 = D0<D0<D1<D0<D1<Term>>>>>;
pub type B0 = D0<D1<D1<D1<Term>>>>;
const _: () = assert!(<A0 as Nat>::V == 20 && <B0 as Nat>::V == 14);
const _: () = assert!(<<A0 as Add<B0>>::O as Nat>::V == 34);
pub fn f0(x: Fixed<A0, B0, Hot>) -> <Hot as Store<<A0 as Add<B0>>::O>>::T {
    x.0
}
pub type A1 = D0<D0<D1<D0<D1<Term>>>>>;
pub type B1 = D1<D0<D0<D1<D1<Term>>>>>;
const _: () = assert!(<A1 as Nat>::V == 20 && <B1 as Nat>::V == 25);
const _: () = assert!(<<A1 as Add<B1>>::O as Nat>::V == 45);
pub fn f1(x: Fixed<A1, B1, Hot>) -> <Hot as Store<<A1 as Add<B1>>::O>>::T {
    x.0
}
pub type A2 = D0<D0<D1<D0<D1<Term>>>>>;
pub type B2 = D0<D0<D1<D0<D0<D1<Term>>>>>>;
const _: () = assert!(<A2 as Nat>::V == 20 && <B2 as Nat>::V == 36);
const _: () = assert!(<<A2 as Add<B2>>::O as Nat>::V == 56);
pub fn f2(x: Fixed<A2, B2, Hot>) -> <Hot as Store<<A2 as Add<B2>>::O>>::T {
    x.0
}
pub type A3 = D0<D0<D1<D0<D1<Term>>>>>;
pub type B3 = D1<D1<D1<D1<D0<D1<Term>>>>>>;
const _: () = assert!(<A3 as Nat>::V == 20 && <B3 as Nat>::V == 47);
const _: () = assert!(<<A3 as Add<B3>>::O as Nat>::V == 67);
pub fn f3(x: Fixed<A3, B3, Hot>) -> <Hot as Store<<A3 as Add<B3>>::O>>::T {
    x.0
}
pub type A4 = D0<D0<D1<D0<D1<Term>>>>>;
pub type B4 = D0<D1<D0<D1<D1<D1<Term>>>>>>;
const _: () = assert!(<A4 as Nat>::V == 20 && <B4 as Nat>::V == 58);
const _: () = assert!(<<A4 as Add<B4>>::O as Nat>::V == 78);
pub fn f4(x: Fixed<A4, B4, Hot>) -> <Hot as Store<<A4 as Add<B4>>::O>>::T {
    x.0
}
pub type A5 = D0<D0<D1<D0<D1<Term>>>>>;
pub type B5 = D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>;
const _: () = assert!(<A5 as Nat>::V == 20 && <B5 as Nat>::V == 69);
const _: () = assert!(<<A5 as Add<B5>>::O as Nat>::V == 89);
pub fn f5(x: Fixed<A5, B5, Hot>) -> <Hot as Store<<A5 as Add<B5>>::O>>::T {
    x.0
}
pub type A6 = D0<D0<D1<D0<D1<Term>>>>>;
pub type B6 = D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A6 as Nat>::V == 20 && <B6 as Nat>::V == 80);
const _: () = assert!(<<A6 as Add<B6>>::O as Nat>::V == 100);
pub fn f6(x: Fixed<A6, B6, Hot>) -> <Hot as Store<<A6 as Add<B6>>::O>>::T {
    x.0
}
pub type A7 = D0<D0<D1<D0<D1<Term>>>>>;
pub type B7 = D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A7 as Nat>::V == 20 && <B7 as Nat>::V == 91);
const _: () = assert!(<<A7 as Add<B7>>::O as Nat>::V == 111);
pub fn f7(x: Fixed<A7, B7, Hot>) -> <Hot as Store<<A7 as Add<B7>>::O>>::T {
    x.0
}
pub type A8 = D1<D0<D0<D0<D0<D1<Term>>>>>>;
pub type B8 = D0<D1<D1<D1<Term>>>>;
const _: () = assert!(<A8 as Nat>::V == 33 && <B8 as Nat>::V == 14);
const _: () = assert!(<<A8 as Add<B8>>::O as Nat>::V == 47);
pub fn f8(x: Fixed<A8, B8, Hot>) -> <Hot as Store<<A8 as Add<B8>>::O>>::T {
    x.0
}
pub type A9 = D1<D0<D0<D0<D0<D1<Term>>>>>>;
pub type B9 = D1<D0<D0<D1<D1<Term>>>>>;
const _: () = assert!(<A9 as Nat>::V == 33 && <B9 as Nat>::V == 25);
const _: () = assert!(<<A9 as Add<B9>>::O as Nat>::V == 58);
pub fn f9(x: Fixed<A9, B9, Hot>) -> <Hot as Store<<A9 as Add<B9>>::O>>::T {
    x.0
}
pub type A10 = D1<D0<D0<D0<D0<D1<Term>>>>>>;
pub type B10 = D0<D0<D1<D0<D0<D1<Term>>>>>>;
const _: () = assert!(<A10 as Nat>::V == 33 && <B10 as Nat>::V == 36);
const _: () = assert!(<<A10 as Add<B10>>::O as Nat>::V == 69);
pub fn f10(x: Fixed<A10, B10, Hot>) -> <Hot as Store<<A10 as Add<B10>>::O>>::T {
    x.0
}
pub type A11 = D1<D0<D0<D0<D0<D1<Term>>>>>>;
pub type B11 = D1<D1<D1<D1<D0<D1<Term>>>>>>;
const _: () = assert!(<A11 as Nat>::V == 33 && <B11 as Nat>::V == 47);
const _: () = assert!(<<A11 as Add<B11>>::O as Nat>::V == 80);
pub fn f11(x: Fixed<A11, B11, Hot>) -> <Hot as Store<<A11 as Add<B11>>::O>>::T {
    x.0
}
pub type A12 = D1<D0<D0<D0<D0<D1<Term>>>>>>;
pub type B12 = D0<D1<D0<D1<D1<D1<Term>>>>>>;
const _: () = assert!(<A12 as Nat>::V == 33 && <B12 as Nat>::V == 58);
const _: () = assert!(<<A12 as Add<B12>>::O as Nat>::V == 91);
pub fn f12(x: Fixed<A12, B12, Hot>) -> <Hot as Store<<A12 as Add<B12>>::O>>::T {
    x.0
}
pub type A13 = D1<D0<D0<D0<D0<D1<Term>>>>>>;
pub type B13 = D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>;
const _: () = assert!(<A13 as Nat>::V == 33 && <B13 as Nat>::V == 69);
const _: () = assert!(<<A13 as Add<B13>>::O as Nat>::V == 102);
pub fn f13(x: Fixed<A13, B13, Hot>) -> <Hot as Store<<A13 as Add<B13>>::O>>::T {
    x.0
}
pub type A14 = D1<D0<D0<D0<D0<D1<Term>>>>>>;
pub type B14 = D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A14 as Nat>::V == 33 && <B14 as Nat>::V == 80);
const _: () = assert!(<<A14 as Add<B14>>::O as Nat>::V == 113);
pub fn f14(x: Fixed<A14, B14, Hot>) -> <Hot as Store<<A14 as Add<B14>>::O>>::T {
    x.0
}
pub type A15 = D1<D0<D0<D0<D0<D1<Term>>>>>>;
pub type B15 = D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A15 as Nat>::V == 33 && <B15 as Nat>::V == 91);
const _: () = assert!(<<A15 as Add<B15>>::O as Nat>::V == 124);
pub fn f15(x: Fixed<A15, B15, Hot>) -> <Hot as Store<<A15 as Add<B15>>::O>>::T {
    x.0
}
pub type A16 = D0<D1<D1<D1<D0<D1<Term>>>>>>;
pub type B16 = D0<D1<D1<D1<Term>>>>;
const _: () = assert!(<A16 as Nat>::V == 46 && <B16 as Nat>::V == 14);
const _: () = assert!(<<A16 as Add<B16>>::O as Nat>::V == 60);
pub fn f16(x: Fixed<A16, B16, Hot>) -> <Hot as Store<<A16 as Add<B16>>::O>>::T {
    x.0
}
pub type A17 = D0<D1<D1<D1<D0<D1<Term>>>>>>;
pub type B17 = D1<D0<D0<D1<D1<Term>>>>>;
const _: () = assert!(<A17 as Nat>::V == 46 && <B17 as Nat>::V == 25);
const _: () = assert!(<<A17 as Add<B17>>::O as Nat>::V == 71);
pub fn f17(x: Fixed<A17, B17, Hot>) -> <Hot as Store<<A17 as Add<B17>>::O>>::T {
    x.0
}
pub type A18 = D0<D1<D1<D1<D0<D1<Term>>>>>>;
pub type B18 = D0<D0<D1<D0<D0<D1<Term>>>>>>;
const _: () = assert!(<A18 as Nat>::V == 46 && <B18 as Nat>::V == 36);
const _: () = assert!(<<A18 as Add<B18>>::O as Nat>::V == 82);
pub fn f18(x: Fixed<A18, B18, Hot>) -> <Hot as Store<<A18 as Add<B18>>::O>>::T {
    x.0
}
pub type A19 = D0<D1<D1<D1<D0<D1<Term>>>>>>;
pub type B19 = D1<D1<D1<D1<D0<D1<Term>>>>>>;
const _: () = assert!(<A19 as Nat>::V == 46 && <B19 as Nat>::V == 47);
const _: () = assert!(<<A19 as Add<B19>>::O as Nat>::V == 93);
pub fn f19(x: Fixed<A19, B19, Hot>) -> <Hot as Store<<A19 as Add<B19>>::O>>::T {
    x.0
}
pub type A20 = D0<D1<D1<D1<D0<D1<Term>>>>>>;
pub type B20 = D0<D1<D0<D1<D1<D1<Term>>>>>>;
const _: () = assert!(<A20 as Nat>::V == 46 && <B20 as Nat>::V == 58);
const _: () = assert!(<<A20 as Add<B20>>::O as Nat>::V == 104);
pub fn f20(x: Fixed<A20, B20, Hot>) -> <Hot as Store<<A20 as Add<B20>>::O>>::T {
    x.0
}
pub type A21 = D0<D1<D1<D1<D0<D1<Term>>>>>>;
pub type B21 = D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>;
const _: () = assert!(<A21 as Nat>::V == 46 && <B21 as Nat>::V == 69);
const _: () = assert!(<<A21 as Add<B21>>::O as Nat>::V == 115);
pub fn f21(x: Fixed<A21, B21, Hot>) -> <Hot as Store<<A21 as Add<B21>>::O>>::T {
    x.0
}
pub type A22 = D0<D1<D1<D1<D0<D1<Term>>>>>>;
pub type B22 = D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A22 as Nat>::V == 46 && <B22 as Nat>::V == 80);
const _: () = assert!(<<A22 as Add<B22>>::O as Nat>::V == 126);
pub fn f22(x: Fixed<A22, B22, Hot>) -> <Hot as Store<<A22 as Add<B22>>::O>>::T {
    x.0
}
pub type A23 = D0<D1<D1<D1<D0<D1<Term>>>>>>;
pub type B23 = D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A23 as Nat>::V == 46 && <B23 as Nat>::V == 91);
const _: () = assert!(<<A23 as Add<B23>>::O as Nat>::V == 137);
pub fn f23(x: Fixed<A23, B23, Hot>) -> <Hot as Store<<A23 as Add<B23>>::O>>::T {
    x.0
}
pub type A24 = D1<D1<D0<D1<D1<D1<Term>>>>>>;
pub type B24 = D0<D1<D1<D1<Term>>>>;
const _: () = assert!(<A24 as Nat>::V == 59 && <B24 as Nat>::V == 14);
const _: () = assert!(<<A24 as Add<B24>>::O as Nat>::V == 73);
pub fn f24(x: Fixed<A24, B24, Hot>) -> <Hot as Store<<A24 as Add<B24>>::O>>::T {
    x.0
}
pub type A25 = D1<D1<D0<D1<D1<D1<Term>>>>>>;
pub type B25 = D1<D0<D0<D1<D1<Term>>>>>;
const _: () = assert!(<A25 as Nat>::V == 59 && <B25 as Nat>::V == 25);
const _: () = assert!(<<A25 as Add<B25>>::O as Nat>::V == 84);
pub fn f25(x: Fixed<A25, B25, Hot>) -> <Hot as Store<<A25 as Add<B25>>::O>>::T {
    x.0
}
pub type A26 = D1<D1<D0<D1<D1<D1<Term>>>>>>;
pub type B26 = D0<D0<D1<D0<D0<D1<Term>>>>>>;
const _: () = assert!(<A26 as Nat>::V == 59 && <B26 as Nat>::V == 36);
const _: () = assert!(<<A26 as Add<B26>>::O as Nat>::V == 95);
pub fn f26(x: Fixed<A26, B26, Hot>) -> <Hot as Store<<A26 as Add<B26>>::O>>::T {
    x.0
}
pub type A27 = D1<D1<D0<D1<D1<D1<Term>>>>>>;
pub type B27 = D1<D1<D1<D1<D0<D1<Term>>>>>>;
const _: () = assert!(<A27 as Nat>::V == 59 && <B27 as Nat>::V == 47);
const _: () = assert!(<<A27 as Add<B27>>::O as Nat>::V == 106);
pub fn f27(x: Fixed<A27, B27, Hot>) -> <Hot as Store<<A27 as Add<B27>>::O>>::T {
    x.0
}
pub type A28 = D1<D1<D0<D1<D1<D1<Term>>>>>>;
pub type B28 = D0<D1<D0<D1<D1<D1<Term>>>>>>;
const _: () = assert!(<A28 as Nat>::V == 59 && <B28 as Nat>::V == 58);
const _: () = assert!(<<A28 as Add<B28>>::O as Nat>::V == 117);
pub fn f28(x: Fixed<A28, B28, Hot>) -> <Hot as Store<<A28 as Add<B28>>::O>>::T {
    x.0
}
pub type A29 = D1<D1<D0<D1<D1<D1<Term>>>>>>;
pub type B29 = D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>;
const _: () = assert!(<A29 as Nat>::V == 59 && <B29 as Nat>::V == 69);
const _: () = assert!(<<A29 as Add<B29>>::O as Nat>::V == 128);
pub fn f29(x: Fixed<A29, B29, Hot>) -> <Hot as Store<<A29 as Add<B29>>::O>>::T {
    x.0
}
pub type A30 = D1<D1<D0<D1<D1<D1<Term>>>>>>;
pub type B30 = D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A30 as Nat>::V == 59 && <B30 as Nat>::V == 80);
const _: () = assert!(<<A30 as Add<B30>>::O as Nat>::V == 139);
pub fn f30(x: Fixed<A30, B30, Hot>) -> <Hot as Store<<A30 as Add<B30>>::O>>::T {
    x.0
}
pub type A31 = D1<D1<D0<D1<D1<D1<Term>>>>>>;
pub type B31 = D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A31 as Nat>::V == 59 && <B31 as Nat>::V == 91);
const _: () = assert!(<<A31 as Add<B31>>::O as Nat>::V == 150);
pub fn f31(x: Fixed<A31, B31, Hot>) -> <Hot as Store<<A31 as Add<B31>>::O>>::T {
    x.0
}
pub type A32 = D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>;
pub type B32 = D0<D1<D1<D1<Term>>>>;
const _: () = assert!(<A32 as Nat>::V == 72 && <B32 as Nat>::V == 14);
const _: () = assert!(<<A32 as Add<B32>>::O as Nat>::V == 86);
pub fn f32(x: Fixed<A32, B32, Hot>) -> <Hot as Store<<A32 as Add<B32>>::O>>::T {
    x.0
}
pub type A33 = D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>;
pub type B33 = D1<D0<D0<D1<D1<Term>>>>>;
const _: () = assert!(<A33 as Nat>::V == 72 && <B33 as Nat>::V == 25);
const _: () = assert!(<<A33 as Add<B33>>::O as Nat>::V == 97);
pub fn f33(x: Fixed<A33, B33, Hot>) -> <Hot as Store<<A33 as Add<B33>>::O>>::T {
    x.0
}
pub type A34 = D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>;
pub type B34 = D0<D0<D1<D0<D0<D1<Term>>>>>>;
const _: () = assert!(<A34 as Nat>::V == 72 && <B34 as Nat>::V == 36);
const _: () = assert!(<<A34 as Add<B34>>::O as Nat>::V == 108);
pub fn f34(x: Fixed<A34, B34, Hot>) -> <Hot as Store<<A34 as Add<B34>>::O>>::T {
    x.0
}
pub type A35 = D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>;
pub type B35 = D1<D1<D1<D1<D0<D1<Term>>>>>>;
const _: () = assert!(<A35 as Nat>::V == 72 && <B35 as Nat>::V == 47);
const _: () = assert!(<<A35 as Add<B35>>::O as Nat>::V == 119);
pub fn f35(x: Fixed<A35, B35, Hot>) -> <Hot as Store<<A35 as Add<B35>>::O>>::T {
    x.0
}
pub type A36 = D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>;
pub type B36 = D0<D1<D0<D1<D1<D1<Term>>>>>>;
const _: () = assert!(<A36 as Nat>::V == 72 && <B36 as Nat>::V == 58);
const _: () = assert!(<<A36 as Add<B36>>::O as Nat>::V == 130);
pub fn f36(x: Fixed<A36, B36, Hot>) -> <Hot as Store<<A36 as Add<B36>>::O>>::T {
    x.0
}
pub type A37 = D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>;
pub type B37 = D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>;
const _: () = assert!(<A37 as Nat>::V == 72 && <B37 as Nat>::V == 69);
const _: () = assert!(<<A37 as Add<B37>>::O as Nat>::V == 141);
pub fn f37(x: Fixed<A37, B37, Hot>) -> <Hot as Store<<A37 as Add<B37>>::O>>::T {
    x.0
}
pub type A38 = D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>;
pub type B38 = D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A38 as Nat>::V == 72 && <B38 as Nat>::V == 80);
const _: () = assert!(<<A38 as Add<B38>>::O as Nat>::V == 152);
pub fn f38(x: Fixed<A38, B38, Hot>) -> <Hot as Store<<A38 as Add<B38>>::O>>::T {
    x.0
}
pub type A39 = D0<D0<D0<D1<D0<D0<D1<Term>>>>>>>;
pub type B39 = D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A39 as Nat>::V == 72 && <B39 as Nat>::V == 91);
const _: () = assert!(<<A39 as Add<B39>>::O as Nat>::V == 163);
pub fn f39(x: Fixed<A39, B39, Hot>) -> <Hot as Store<<A39 as Add<B39>>::O>>::T {
    x.0
}
pub type A40 = D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>;
pub type B40 = D0<D1<D1<D1<Term>>>>;
const _: () = assert!(<A40 as Nat>::V == 85 && <B40 as Nat>::V == 14);
const _: () = assert!(<<A40 as Add<B40>>::O as Nat>::V == 99);
pub fn f40(x: Fixed<A40, B40, Hot>) -> <Hot as Store<<A40 as Add<B40>>::O>>::T {
    x.0
}
pub type A41 = D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>;
pub type B41 = D1<D0<D0<D1<D1<Term>>>>>;
const _: () = assert!(<A41 as Nat>::V == 85 && <B41 as Nat>::V == 25);
const _: () = assert!(<<A41 as Add<B41>>::O as Nat>::V == 110);
pub fn f41(x: Fixed<A41, B41, Hot>) -> <Hot as Store<<A41 as Add<B41>>::O>>::T {
    x.0
}
pub type A42 = D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>;
pub type B42 = D0<D0<D1<D0<D0<D1<Term>>>>>>;
const _: () = assert!(<A42 as Nat>::V == 85 && <B42 as Nat>::V == 36);
const _: () = assert!(<<A42 as Add<B42>>::O as Nat>::V == 121);
pub fn f42(x: Fixed<A42, B42, Hot>) -> <Hot as Store<<A42 as Add<B42>>::O>>::T {
    x.0
}
pub type A43 = D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>;
pub type B43 = D1<D1<D1<D1<D0<D1<Term>>>>>>;
const _: () = assert!(<A43 as Nat>::V == 85 && <B43 as Nat>::V == 47);
const _: () = assert!(<<A43 as Add<B43>>::O as Nat>::V == 132);
pub fn f43(x: Fixed<A43, B43, Hot>) -> <Hot as Store<<A43 as Add<B43>>::O>>::T {
    x.0
}
pub type A44 = D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>;
pub type B44 = D0<D1<D0<D1<D1<D1<Term>>>>>>;
const _: () = assert!(<A44 as Nat>::V == 85 && <B44 as Nat>::V == 58);
const _: () = assert!(<<A44 as Add<B44>>::O as Nat>::V == 143);
pub fn f44(x: Fixed<A44, B44, Hot>) -> <Hot as Store<<A44 as Add<B44>>::O>>::T {
    x.0
}
pub type A45 = D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>;
pub type B45 = D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>;
const _: () = assert!(<A45 as Nat>::V == 85 && <B45 as Nat>::V == 69);
const _: () = assert!(<<A45 as Add<B45>>::O as Nat>::V == 154);
pub fn f45(x: Fixed<A45, B45, Hot>) -> <Hot as Store<<A45 as Add<B45>>::O>>::T {
    x.0
}
pub type A46 = D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>;
pub type B46 = D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A46 as Nat>::V == 85 && <B46 as Nat>::V == 80);
const _: () = assert!(<<A46 as Add<B46>>::O as Nat>::V == 165);
pub fn f46(x: Fixed<A46, B46, Hot>) -> <Hot as Store<<A46 as Add<B46>>::O>>::T {
    x.0
}
pub type A47 = D1<D0<D1<D0<D1<D0<D1<Term>>>>>>>;
pub type B47 = D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A47 as Nat>::V == 85 && <B47 as Nat>::V == 91);
const _: () = assert!(<<A47 as Add<B47>>::O as Nat>::V == 176);
pub fn f47(x: Fixed<A47, B47, Hot>) -> <Hot as Store<<A47 as Add<B47>>::O>>::T {
    x.0
}
pub type A48 = D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>;
pub type B48 = D0<D1<D1<D1<Term>>>>;
const _: () = assert!(<A48 as Nat>::V == 98 && <B48 as Nat>::V == 14);
const _: () = assert!(<<A48 as Add<B48>>::O as Nat>::V == 112);
pub fn f48(x: Fixed<A48, B48, Hot>) -> <Hot as Store<<A48 as Add<B48>>::O>>::T {
    x.0
}
pub type A49 = D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>;
pub type B49 = D1<D0<D0<D1<D1<Term>>>>>;
const _: () = assert!(<A49 as Nat>::V == 98 && <B49 as Nat>::V == 25);
const _: () = assert!(<<A49 as Add<B49>>::O as Nat>::V == 123);
pub fn f49(x: Fixed<A49, B49, Hot>) -> <Hot as Store<<A49 as Add<B49>>::O>>::T {
    x.0
}
pub type A50 = D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>;
pub type B50 = D0<D0<D1<D0<D0<D1<Term>>>>>>;
const _: () = assert!(<A50 as Nat>::V == 98 && <B50 as Nat>::V == 36);
const _: () = assert!(<<A50 as Add<B50>>::O as Nat>::V == 134);
pub fn f50(x: Fixed<A50, B50, Hot>) -> <Hot as Store<<A50 as Add<B50>>::O>>::T {
    x.0
}
pub type A51 = D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>;
pub type B51 = D1<D1<D1<D1<D0<D1<Term>>>>>>;
const _: () = assert!(<A51 as Nat>::V == 98 && <B51 as Nat>::V == 47);
const _: () = assert!(<<A51 as Add<B51>>::O as Nat>::V == 145);
pub fn f51(x: Fixed<A51, B51, Hot>) -> <Hot as Store<<A51 as Add<B51>>::O>>::T {
    x.0
}
pub type A52 = D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>;
pub type B52 = D0<D1<D0<D1<D1<D1<Term>>>>>>;
const _: () = assert!(<A52 as Nat>::V == 98 && <B52 as Nat>::V == 58);
const _: () = assert!(<<A52 as Add<B52>>::O as Nat>::V == 156);
pub fn f52(x: Fixed<A52, B52, Hot>) -> <Hot as Store<<A52 as Add<B52>>::O>>::T {
    x.0
}
pub type A53 = D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>;
pub type B53 = D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>;
const _: () = assert!(<A53 as Nat>::V == 98 && <B53 as Nat>::V == 69);
const _: () = assert!(<<A53 as Add<B53>>::O as Nat>::V == 167);
pub fn f53(x: Fixed<A53, B53, Hot>) -> <Hot as Store<<A53 as Add<B53>>::O>>::T {
    x.0
}
pub type A54 = D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>;
pub type B54 = D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A54 as Nat>::V == 98 && <B54 as Nat>::V == 80);
const _: () = assert!(<<A54 as Add<B54>>::O as Nat>::V == 178);
pub fn f54(x: Fixed<A54, B54, Hot>) -> <Hot as Store<<A54 as Add<B54>>::O>>::T {
    x.0
}
pub type A55 = D0<D1<D0<D0<D0<D1<D1<Term>>>>>>>;
pub type B55 = D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A55 as Nat>::V == 98 && <B55 as Nat>::V == 91);
const _: () = assert!(<<A55 as Add<B55>>::O as Nat>::V == 189);
pub fn f55(x: Fixed<A55, B55, Hot>) -> <Hot as Store<<A55 as Add<B55>>::O>>::T {
    x.0
}
pub type A56 = D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>;
pub type B56 = D0<D1<D1<D1<Term>>>>;
const _: () = assert!(<A56 as Nat>::V == 111 && <B56 as Nat>::V == 14);
const _: () = assert!(<<A56 as Add<B56>>::O as Nat>::V == 125);
pub fn f56(x: Fixed<A56, B56, Hot>) -> <Hot as Store<<A56 as Add<B56>>::O>>::T {
    x.0
}
pub type A57 = D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>;
pub type B57 = D1<D0<D0<D1<D1<Term>>>>>;
const _: () = assert!(<A57 as Nat>::V == 111 && <B57 as Nat>::V == 25);
const _: () = assert!(<<A57 as Add<B57>>::O as Nat>::V == 136);
pub fn f57(x: Fixed<A57, B57, Hot>) -> <Hot as Store<<A57 as Add<B57>>::O>>::T {
    x.0
}
pub type A58 = D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>;
pub type B58 = D0<D0<D1<D0<D0<D1<Term>>>>>>;
const _: () = assert!(<A58 as Nat>::V == 111 && <B58 as Nat>::V == 36);
const _: () = assert!(<<A58 as Add<B58>>::O as Nat>::V == 147);
pub fn f58(x: Fixed<A58, B58, Hot>) -> <Hot as Store<<A58 as Add<B58>>::O>>::T {
    x.0
}
pub type A59 = D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>;
pub type B59 = D1<D1<D1<D1<D0<D1<Term>>>>>>;
const _: () = assert!(<A59 as Nat>::V == 111 && <B59 as Nat>::V == 47);
const _: () = assert!(<<A59 as Add<B59>>::O as Nat>::V == 158);
pub fn f59(x: Fixed<A59, B59, Hot>) -> <Hot as Store<<A59 as Add<B59>>::O>>::T {
    x.0
}
pub type A60 = D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>;
pub type B60 = D0<D1<D0<D1<D1<D1<Term>>>>>>;
const _: () = assert!(<A60 as Nat>::V == 111 && <B60 as Nat>::V == 58);
const _: () = assert!(<<A60 as Add<B60>>::O as Nat>::V == 169);
pub fn f60(x: Fixed<A60, B60, Hot>) -> <Hot as Store<<A60 as Add<B60>>::O>>::T {
    x.0
}
pub type A61 = D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>;
pub type B61 = D1<D0<D1<D0<D0<D0<D1<Term>>>>>>>;
const _: () = assert!(<A61 as Nat>::V == 111 && <B61 as Nat>::V == 69);
const _: () = assert!(<<A61 as Add<B61>>::O as Nat>::V == 180);
pub fn f61(x: Fixed<A61, B61, Hot>) -> <Hot as Store<<A61 as Add<B61>>::O>>::T {
    x.0
}
pub type A62 = D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>;
pub type B62 = D0<D0<D0<D0<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A62 as Nat>::V == 111 && <B62 as Nat>::V == 80);
const _: () = assert!(<<A62 as Add<B62>>::O as Nat>::V == 191);
pub fn f62(x: Fixed<A62, B62, Hot>) -> <Hot as Store<<A62 as Add<B62>>::O>>::T {
    x.0
}
pub type A63 = D1<D1<D1<D1<D0<D1<D1<Term>>>>>>>;
pub type B63 = D1<D1<D0<D1<D1<D0<D1<Term>>>>>>>;
const _: () = assert!(<A63 as Nat>::V == 111 && <B63 as Nat>::V == 91);
const _: () = assert!(<<A63 as Add<B63>>::O as Nat>::V == 202);
pub fn f63(x: Fixed<A63, B63, Hot>) -> <Hot as Store<<A63 as Add<B63>>::O>>::T {
    x.0
}
