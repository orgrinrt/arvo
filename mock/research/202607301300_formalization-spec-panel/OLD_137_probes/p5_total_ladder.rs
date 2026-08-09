//! P5: is the ladder TOTAL, gate-free, with no width enumerated anywhere?
//! A little-endian binary structural nat, a rung by digit count for the native
//! arms, and for the wide arm a word count computed by structural ceil-division,
//! feeding P4's word cons. No `#![feature]`, no `-Z` flag, no width listed.
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

// --- the magnitude, little endian binary --------------------------------------
#[derive(Clone, Copy)]
pub struct Term;
#[derive(Clone, Copy)]
pub struct D0<T>(PhantomData<T>);
#[derive(Clone, Copy)]
pub struct D1<T>(PhantomData<T>);

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

// --- unary tally --------------------------------------------------------------
pub struct Z;
pub struct S<T>(PhantomData<T>);
pub trait Tally {
    const N: u32;
}
impl Tally for Z {
    const N: u32 = 0;
}
impl<T: Tally> Tally for S<T> {
    const N: u32 = 1 + T::N;
}

// --- digit count, ignoring leading zeros. this is the native rung selector ----
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

// --- zero classifiers, disjoint and total over towers -------------------------
pub trait AllZero {}
impl AllZero for Term {}
impl<T: AllZero> AllZero for D0<T> {}

pub trait AnyOne {}
impl<T> AnyOne for D1<T> {}
impl<T: AnyOne> AnyOne for D0<T> {}

// --- shift right by six, that is floor(W / 64), with the peeled digits kept ---
pub trait Shr6 {
    type Q;
    type R;
}
impl Shr6 for Term {
    type Q = Term;
    type R = Term;
}
impl<T: Shr5> Shr6 for D0<T> {
    type Q = Q6<T>;
    type R = D0<R5<T>>;
}
impl<T: Shr5> Shr6 for D1<T> {
    type Q = Q6<T>;
    type R = D1<R5<T>>;
}
// the remaining five peels, written as projections so no width is named
pub type Q6<T> = <T as Shr5>::Q;
pub type R5<T> = <T as Shr5>::R;
pub trait Shr5 {
    type Q;
    type R;
}
impl Shr5 for Term {
    type Q = Term;
    type R = Term;
}
impl<T: Shr4> Shr5 for D0<T> {
    type Q = <T as Shr4>::Q;
    type R = D0<<T as Shr4>::R>;
}
impl<T: Shr4> Shr5 for D1<T> {
    type Q = <T as Shr4>::Q;
    type R = D1<<T as Shr4>::R>;
}
pub trait Shr4 {
    type Q;
    type R;
}
impl Shr4 for Term {
    type Q = Term;
    type R = Term;
}
impl<T: Shr3> Shr4 for D0<T> {
    type Q = <T as Shr3>::Q;
    type R = D0<<T as Shr3>::R>;
}
impl<T: Shr3> Shr4 for D1<T> {
    type Q = <T as Shr3>::Q;
    type R = D1<<T as Shr3>::R>;
}
pub trait Shr3 {
    type Q;
    type R;
}
impl Shr3 for Term {
    type Q = Term;
    type R = Term;
}
impl<T: Shr2> Shr3 for D0<T> {
    type Q = <T as Shr2>::Q;
    type R = D0<<T as Shr2>::R>;
}
impl<T: Shr2> Shr3 for D1<T> {
    type Q = <T as Shr2>::Q;
    type R = D1<<T as Shr2>::R>;
}
pub trait Shr2 {
    type Q;
    type R;
}
impl Shr2 for Term {
    type Q = Term;
    type R = Term;
}
impl<T: Shr1> Shr2 for D0<T> {
    type Q = <T as Shr1>::Q;
    type R = D0<<T as Shr1>::R>;
}
impl<T: Shr1> Shr2 for D1<T> {
    type Q = <T as Shr1>::Q;
    type R = D1<<T as Shr1>::R>;
}
pub trait Shr1 {
    type Q;
    type R;
}
impl Shr1 for Term {
    type Q = Term;
    type R = Term;
}
impl<T> Shr1 for D0<T> {
    type Q = T;
    type R = Term;
}
impl<T> Shr1 for D1<T> {
    type Q = T;
    type R = D1<Term>;
}

// --- tower to unary tally, total by structure --------------------------------
pub trait ToTally {
    type T;
}
impl ToTally for Term {
    type T = Z;
}
pub trait AddT<R> {
    type O;
}
impl<R> AddT<R> for Z {
    type O = R;
}
impl<L: AddT<R>, R> AddT<R> for S<L> {
    type O = S<<L as AddT<R>>::O>;
}
impl<T: ToTally> ToTally for D0<T>
where
    T::T: AddT<T::T>,
{
    type T = <T::T as AddT<T::T>>::O;
}
impl<T: ToTally> ToTally for D1<T>
where
    T::T: AddT<T::T>,
{
    type T = S<<T::T as AddT<T::T>>::O>;
}

// decrement, so both the native rung and the word count are keyed on W - 1.
// That is what separates 8 from 9, 128 from 129, and 64 words from 65.
pub trait Dec {
    type O;
}
impl<T> Dec for D1<T> {
    type O = D0<T>;
}
impl<T: Dec> Dec for D0<T> {
    type O = D1<<T as Dec>::O>;
}

// --- the word count: ceil(W / 64), as ONE total impl -------------------------
// ceil(W/64) == floor((W-1)/64) + 1 for W >= 1, so the round-up needs no
// classifier and there is no overlap to resolve. `Dec` is already on hand for
// the native rung. Note that two blanket impls partitioned by disjoint
// where-clauses do NOT work here: rustc reports E0119 for `impl<T> Tr for T`
// even when the classifiers are structurally disjoint, because coherence does
// not reason negatively. 133:1.2's result holds only for a constrained self
// type.
pub trait WordCount {
    /// unary tally of 64-bit words, at least one
    type W;
}
impl<T> WordCount for T
where
    T: Dec,
    <T as Dec>::O: Shr6,
    <<T as Dec>::O as Shr6>::Q: ToTally,
{
    type W = S<<<<T as Dec>::O as Shr6>::Q as ToTally>::T>;
}

// --- the wide payload, built from the tally. total by structure ---------------
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WNil;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WCons<T> {
    pub w: u64,
    pub rest: T,
}
pub trait Build {
    type P;
}
impl Build for Z {
    type P = WNil;
}
impl<T: Build> Build for S<T> {
    type P = WCons<<T as Build>::P>;
}

pub trait WAdd: Copy {
    fn add_c(self, o: Self, carry: bool) -> (Self, bool);
}
impl WAdd for WNil {
    #[inline]
    fn add_c(self, _o: Self, carry: bool) -> (Self, bool) {
        (WNil, carry)
    }
}
impl<T: WAdd> WAdd for WCons<T> {
    #[inline]
    fn add_c(self, o: Self, carry: bool) -> (Self, bool) {
        let (s, c) = self.w.carrying_add(o.w, carry);
        let (rest, c2) = self.rest.add_c(o.rest, c);
        (WCons { w: s, rest }, c2)
    }
}

// --- the native rungs, by digit count, with a structural catch-all ------------
pub trait Rung {
    type C;
}
impl Rung for Z {
    type C = u8;
}
impl Rung for S<Z> {
    type C = u8;
}
impl Rung for S<S<Z>> {
    type C = u8;
}
impl Rung for S<S<S<Z>>> {
    type C = u8;
}
impl Rung for S<S<S<S<Z>>>> {
    type C = u16;
}
impl Rung for S<S<S<S<S<Z>>>>> {
    type C = u32;
}
impl Rung for S<S<S<S<S<S<Z>>>>>> {
    type C = u64;
}
impl Rung for S<S<S<S<S<S<S<Z>>>>>>> {
    type C = u128;
}
// eight digits or more of `W - 1`: above 128 bits, so the wide payload. The
// catch-all is structural, so there is no largest width and nothing enumerated.
impl<T> Rung for S<S<S<S<S<S<S<S<T>>>>>>>>
where
    T: Unbump,
{
    type C = Wide<T>;
}
pub trait Unbump {}
impl Unbump for Z {}
impl<T> Unbump for S<T> {}
#[derive(Clone, Copy)]
pub struct Wide<T>(PhantomData<T>);

// --- checks: the tally arithmetic is right ------------------------------------
pub type N64 = D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>; // 64
pub type N65 = D1<D0<D0<D0<D0<D0<D1<Term>>>>>>>; // 65
pub type N128 = D0<D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>>; // 128
pub type N129 = D1<D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>>; // 129
pub type N192 = D0<D0<D0<D0<D0<D0<D1<D1<Term>>>>>>>>; // 192
pub type N200 = D0<D0<D0<D1<D0<D0<D1<D1<Term>>>>>>>>; // 200
pub type N256 = D0<D0<D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>>>; // 256
pub type N1024 = D0<D0<D0<D0<D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>>>>>; // 1024
pub type W3 = D1<D1<Term>>; // 3
pub type W13 = D1<D0<D1<D1<Term>>>>; // 13
pub type W16 = D0<D0<D0<D0<D1<Term>>>>>; // 16
pub type W33 = D1<D0<D0<D0<D0<D1<Term>>>>>>; // 33
pub type W128 = D0<D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>>; // 128
pub type W8 = D0<D0<D0<D1<Term>>>>; // 8
pub type W9 = D1<D0<D0<D1<Term>>>>; // 9
pub type W65 = D1<D0<D0<D0<D0<D0<D1<Term>>>>>>>; // 65
pub type W64 = D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>; // 64

const _: () = assert!(<N64 as Nat>::V == 64);
const _: () = assert!(<N65 as Nat>::V == 65);
const _: () = assert!(<N129 as Nat>::V == 129);
const _: () = assert!(<N192 as Nat>::V == 192);
const _: () = assert!(<N200 as Nat>::V == 200);
const _: () = assert!(<N1024 as Nat>::V == 1024);
const _: () = assert!(<W13 as Nat>::V == 13);
const _: () = assert!(<W33 as Nat>::V == 33);

// exact multiples of 64 take the AllZero arm
const _: () = assert!(<<N64 as WordCount>::W as Tally>::N == 1);
const _: () = assert!(<<N128 as WordCount>::W as Tally>::N == 2);
const _: () = assert!(<<N192 as WordCount>::W as Tally>::N == 3);
const _: () = assert!(<<N256 as WordCount>::W as Tally>::N == 4);
const _: () = assert!(<<N1024 as WordCount>::W as Tally>::N == 16);
// non-multiples take the AnyOne arm and round up
const _: () = assert!(<<N65 as WordCount>::W as Tally>::N == 2);
const _: () = assert!(<<N129 as WordCount>::W as Tally>::N == 3);
const _: () = assert!(<<N200 as WordCount>::W as Tally>::N == 4);

pub type P192 = <<N192 as WordCount>::W as Build>::P;
pub type P256 = <<N256 as WordCount>::W as Build>::P;
pub type P1024 = <<N1024 as WordCount>::W as Build>::P;
pub type P200 = <<N200 as WordCount>::W as Build>::P;

const _: () = assert!(core::mem::size_of::<P192>() == 24);
const _: () = assert!(core::mem::size_of::<P256>() == 32);
const _: () = assert!(core::mem::size_of::<P1024>() == 128);
const _: () = assert!(core::mem::size_of::<P200>() == 32);

// --- codegen: the derived payload's add, against the hand-written bar ---------
#[unsafe(no_mangle)]
pub fn bar_1024(a: [u64; 16], b: [u64; 16]) -> [u64; 16] {
    let mut out = [0u64; 16];
    let mut c = false;
    let mut i = 0;
    while i < 16 {
        let (s, cc) = a[i].carrying_add(b[i], c);
        out[i] = s;
        c = cc;
        i += 1;
    }
    out
}
#[unsafe(no_mangle)]
pub fn bar_256(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let (r0, c) = a[0].carrying_add(b[0], false);
    let (r1, c) = a[1].carrying_add(b[1], c);
    let (r2, c) = a[2].carrying_add(b[2], c);
    let (r3, _) = a[3].carrying_add(b[3], c);
    [r0, r1, r2, r3]
}
#[unsafe(no_mangle)]
pub fn derived_192(a: P192, b: P192) -> P192 {
    a.add_c(b, false).0
}
#[unsafe(no_mangle)]
pub fn derived_256(a: P256, b: P256) -> P256 {
    a.add_c(b, false).0
}
#[unsafe(no_mangle)]
pub fn derived_1024(a: P1024, b: P1024) -> P1024 {
    a.add_c(b, false).0
}

// --- the native rungs resolve to the right machine type, as type equalities ---
pub trait Container {
    type C;
}
impl<T> Container for T
where
    T: Dec,
    <T as Dec>::O: Len,
    <<T as Dec>::O as Len>::L: Rung,
{
    type C = <<<T as Dec>::O as Len>::L as Rung>::C;
}
pub fn c3(x: <W3 as Container>::C) -> u8 {
    x
}
pub fn c8(x: <W8 as Container>::C) -> u8 {
    x
}
pub fn c9(x: <W9 as Container>::C) -> u16 {
    x
}
pub fn c13(x: <W13 as Container>::C) -> u16 {
    x
}
pub fn c16(x: <W16 as Container>::C) -> u16 {
    x
}
pub fn c33(x: <W33 as Container>::C) -> u64 {
    x
}
pub fn c64(x: <W64 as Container>::C) -> u64 {
    x
}
pub fn c65(x: <W65 as Container>::C) -> u128 {
    x
}
pub fn c128(x: <W128 as Container>::C) -> u128 {
    x
}
