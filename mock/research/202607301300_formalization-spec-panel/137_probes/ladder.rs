// P5: is the ladder TOTAL, gate-free, with no width enumerated anywhere?
// A little-endian binary structural nat, a rung by digit count for the native
// arms, and for the wide arm a word count computed by structural ceil-division,
// feeding P4's word cons. No `#![feature]`, no `-Z` flag, no width listed.

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

// --- structural addition on the binary nat, seven plus nine impls -------------
pub trait Add<R> {
    type O;
}
pub trait AddC<R> {
    type O;
} // self + R + 1

impl<R> Add<R> for Term {
    type O = R;
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
impl<A: AddC<B>, B> Add<D1<B>> for D1<A> {
    type O = D0<<A as AddC<B>>::O>;
}

impl AddC<Term> for Term {
    type O = D1<Term>;
}
impl<B> AddC<D0<B>> for Term {
    type O = D1<B>;
}
impl<B: AddC<Term>> AddC<D1<B>> for Term {
    type O = D0<<B as AddC<Term>>::O>;
}
impl<A> AddC<Term> for D0<A> {
    type O = D1<A>;
}
impl<A: AddC<Term>> AddC<Term> for D1<A> {
    type O = D0<<A as AddC<Term>>::O>;
}
impl<A: Add<B>, B> AddC<D0<B>> for D0<A> {
    type O = D1<<A as Add<B>>::O>;
}
impl<A: AddC<B>, B> AddC<D1<B>> for D0<A> {
    type O = D0<<A as AddC<B>>::O>;
}
impl<A: AddC<B>, B> AddC<D0<B>> for D1<A> {
    type O = D0<<A as AddC<B>>::O>;
}
impl<A: AddC<B>, B> AddC<D1<B>> for D1<A> {
    type O = D1<<A as AddC<B>>::O>;
}

// --- the container: native rung by digit count of W-1, wide payload above ----
pub trait Container {
    type C;
}
impl<T> Container for T
where
    T: Dec,
    <T as Dec>::O: Len,
    <<T as Dec>::O as Len>::L: RungPick,
    T: WordCount,
    <T as WordCount>::W: Build,
{
    type C = <<<T as Dec>::O as Len>::L as RungPick>::Pick<<<T as WordCount>::W as Build>::P>;
}
pub trait RungPick {
    type Pick<W>;
}
impl RungPick for Z {
    type Pick<W> = u8;
}
impl RungPick for S<Z> {
    type Pick<W> = u8;
}
impl RungPick for S<S<Z>> {
    type Pick<W> = u8;
}
impl RungPick for S<S<S<Z>>> {
    type Pick<W> = u8;
}
impl RungPick for S<S<S<S<Z>>>> {
    type Pick<W> = u16;
}
impl RungPick for S<S<S<S<S<Z>>>>> {
    type Pick<W> = u32;
}
impl RungPick for S<S<S<S<S<S<Z>>>>>> {
    type Pick<W> = u64;
}
impl RungPick for S<S<S<S<S<S<S<Z>>>>>>> {
    type Pick<W> = u128;
}
// eight digits or more of W-1: above 128 bits, so the derived wide payload.
// structural catch-all, so there is no largest width.
impl<T> RungPick for S<S<S<S<S<S<S<S<T>>>>>>>> {
    type Pick<W> = W;
}

// --- the one operation over whatever the ladder produced ---------------------
pub trait Wrapping: Copy {
    fn wadd(self, o: Self) -> Self;
}
macro_rules! natw { ($($t:ty),*) => { $( impl Wrapping for $t {
    #[inline] fn wadd(self, o: Self) -> Self { self.wrapping_add(o) } } )* } }
natw!(u8, u16, u32, u64, u128);
impl Wrapping for WNil {
    #[inline]
    fn wadd(self, _o: Self) -> Self {
        WNil
    }
}
impl<T: WAdd> Wrapping for WCons<T> {
    #[inline]
    fn wadd(self, o: Self) -> Self {
        self.add_c(o, false).0
    }
}
pub type T0 = Term;
pub type T3 = D1<D1<Term>>;
pub type T8 = D0<D0<D0<D1<Term>>>>;
pub type T13 = D1<D0<D1<D1<Term>>>>;
pub type T16 = D0<D0<D0<D0<D1<Term>>>>>;
pub type T24 = D0<D0<D0<D1<D1<Term>>>>>;
pub type T30 = D0<D1<D1<D1<D1<Term>>>>>;
pub type T40 = D0<D0<D0<D1<D0<D1<Term>>>>>>;
pub type T41 = D1<D0<D0<D1<D0<D1<Term>>>>>>;
pub type T64 = D0<D0<D0<D0<D0<D0<D1<Term>>>>>>>;
pub type T100 = D0<D0<D1<D0<D0<D1<D1<Term>>>>>>>;
pub type T200 = D0<D0<D0<D1<D0<D0<D1<D1<Term>>>>>>>>;
pub type T777 = D1<D0<D0<D1<D0<D0<D0<D0<D1<D1<Term>>>>>>>>>>;
pub type T6 = D0<D1<D1<Term>>>;
pub type T26 = D0<D1<D0<D1<D1<Term>>>>>;
