//! P3. Is a FORMULA over two numerals' members cheaper, in the typestate, than an
//! EXTREMUM over the same members?
//!
//! This is the load-bearing feasibility check under option H. H says every result
//! numeral is produced by a formula rather than by a least upper bound. If a formula
//! is expressible and an extremum is not, that is a design reason to prefer H's
//! reading. If both are equally expressible, then "formula versus extremum" is a
//! SEMANTIC distinction and not a typestate-cost one, and H cannot be argued on
//! feasibility grounds at all.
//!
//! Three projections are declared over a value-unique type-level binary natural:
//!   SUM  (what a product's widths do:      I1+I2, F1+F2)
//!   MAX  (what a coordinatewise join does: max I, max F)
//!   ADDN (what a sum's widths do:          max(I1,I2)+1, max(F1,F2))
//!
//! No `generic_const_exprs`, no `generic_const_args`, no full `specialization`,
//! no `-Znext-solver=globally`, no `TypeId`, no `dyn`, no alloc, `no_std`.
//! MEASURED: this file carries ZERO `#![feature(...)]` gates. It was written
//! expecting to need `min_generic_const_args` and did not need it, because the
//! arithmetic lives entirely in associated types and never in a const argument
//! position. That absence is the result, not the setup.

#![no_std]
#![no_main]

// ---------------------------------------------------------------------------
// A value-unique type-level natural. Little-endian bits, terminated by End.
// Value-unique: End is zero, and a One/Zero cell may not be built over an
// all-zero tail spelled as Zero<End>, so 0 has exactly one spelling. The
// sealing is by construction here (nothing constructs a redundant spelling)
// rather than by a sealed trait, because this is a spike checking arithmetic
// reach, not a design for the encoding.
// ---------------------------------------------------------------------------

pub struct End;
pub struct Zero<T>(core::marker::PhantomData<T>);
pub struct One<T>(core::marker::PhantomData<T>);

pub trait Nat {
    const VAL: u32;
}
impl Nat for End {
    const VAL: u32 = 0;
}
impl<T: Nat> Nat for Zero<T> {
    const VAL: u32 = 2 * T::VAL;
}
impl<T: Nat> Nat for One<T> {
    const VAL: u32 = 2 * T::VAL + 1;
}

// ---------------------------------------------------------------------------
// Successor. Needed by both SUM (through the carry) and ADDN (the +1).
// ---------------------------------------------------------------------------

pub trait Succ {
    type Out: Nat;
}
impl Succ for End {
    type Out = One<End>;
}
impl<T: Nat> Succ for Zero<T> {
    type Out = One<T>;
}
impl<T: Nat + Succ> Succ for One<T> {
    type Out = Zero<<T as Succ>::Out>;
}

// ---------------------------------------------------------------------------
// FORMULA 1: SUM. A ripple-carry fold over the two bit lists.
// This is what a product's integer and fraction widths do.
// ---------------------------------------------------------------------------

pub trait AddN<R> {
    type Out: Nat;
}

impl AddN<End> for End {
    type Out = End;
}
impl<T: Nat> AddN<Zero<T>> for End {
    type Out = Zero<T>;
}
impl<T: Nat> AddN<One<T>> for End {
    type Out = One<T>;
}
impl<T: Nat> AddN<End> for Zero<T> {
    type Out = Zero<T>;
}
impl<T: Nat> AddN<End> for One<T> {
    type Out = One<T>;
}

impl<A: Nat + AddN<B>, B: Nat> AddN<Zero<B>> for Zero<A> {
    type Out = Zero<<A as AddN<B>>::Out>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<One<B>> for Zero<A> {
    type Out = One<<A as AddN<B>>::Out>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<Zero<B>> for One<A> {
    type Out = One<<A as AddN<B>>::Out>;
}
// the only carrying case
impl<A: Nat + AddN<B>, B: Nat> AddN<One<B>> for One<A>
where
    <A as AddN<B>>::Out: Succ,
{
    type Out = Zero<<<A as AddN<B>>::Out as Succ>::Out>;
}

// ---------------------------------------------------------------------------
// EXTREMUM: MAX. This is what a coordinatewise JOIN does on the same members.
// Built from a comparison fold, because max is not computable bit-by-bit from
// the low end without knowing the high end first. That asymmetry against SUM
// is exactly the thing this probe exists to measure.
// ---------------------------------------------------------------------------

pub struct Lt;
pub struct Eq;
pub struct Gt;

pub trait Cmp<R> {
    type Out;
}
impl Cmp<End> for End {
    type Out = Eq;
}
impl<T: Nat> Cmp<Zero<T>> for End
where
    End: Cmp<T>,
{
    // End vs Zero<T>: zero against 2*T, decided entirely by T against zero
    type Out = <End as Cmp<T>>::Out;
}
impl<T: Nat> Cmp<One<T>> for End {
    type Out = Lt;
}
impl<T: Nat> Cmp<End> for Zero<T>
where
    T: Cmp<End>,
{
    type Out = <T as Cmp<End>>::Out;
}
impl<T: Nat> Cmp<End> for One<T> {
    type Out = Gt;
}

// the recursive cases: the HIGH end decides, so the tail's answer wins unless
// the tail says Eq, in which case this bit decides.
impl<A: Nat + Cmp<B>, B: Nat> Cmp<Zero<B>> for Zero<A> {
    type Out = <A as Cmp<B>>::Out;
}
impl<A: Nat + Cmp<B>, B: Nat> Cmp<One<B>> for One<A> {
    type Out = <A as Cmp<B>>::Out;
}
impl<A: Nat + Cmp<B>, B: Nat> Cmp<One<B>> for Zero<A>
where
    <A as Cmp<B>>::Out: TieBreak<Lt>,
{
    type Out = <<A as Cmp<B>>::Out as TieBreak<Lt>>::Out;
}
impl<A: Nat + Cmp<B>, B: Nat> Cmp<Zero<B>> for One<A>
where
    <A as Cmp<B>>::Out: TieBreak<Gt>,
{
    type Out = <<A as Cmp<B>>::Out as TieBreak<Gt>>::Out;
}

/// If the tail decided, keep it; if the tail tied, take this bit's verdict.
pub trait TieBreak<D> {
    type Out;
}
impl<D> TieBreak<D> for Lt {
    type Out = Lt;
}
impl<D> TieBreak<D> for Gt {
    type Out = Gt;
}
impl<D> TieBreak<D> for Eq {
    type Out = D;
}

pub trait Select<A, B> {
    type Out: Nat;
}
impl<A: Nat, B: Nat> Select<A, B> for Lt {
    type Out = B;
}
impl<A: Nat, B: Nat> Select<A, B> for Eq {
    type Out = A;
}
impl<A: Nat, B: Nat> Select<A, B> for Gt {
    type Out = A;
}

pub trait MaxN<R> {
    type Out: Nat;
}
impl<A: Nat + Cmp<B>, B: Nat> MaxN<B> for A
where
    <A as Cmp<B>>::Out: Select<A, B>,
{
    type Out = <<A as Cmp<B>>::Out as Select<A, B>>::Out;
}

// ---------------------------------------------------------------------------
// The three result-numeral projections, stated over a numeral's two width
// members. `Num` stands in for the whole identity contract; only the two
// width positions matter to this check.
// ---------------------------------------------------------------------------

pub struct Num<I, F>(core::marker::PhantomData<(I, F)>);

/// The multiplicative result numeral. Pure formula: coordinatewise sum.
pub trait MulNum<R> {
    type Out;
}
impl<I1: Nat + AddN<I2>, F1: Nat + AddN<F2>, I2: Nat, F2: Nat> MulNum<Num<I2, F2>> for Num<I1, F1> {
    type Out = Num<<I1 as AddN<I2>>::Out, <F1 as AddN<F2>>::Out>;
}

/// The coordinatewise JOIN. This is the lattice operation, for comparison.
pub trait JoinNum<R> {
    type Out;
}
impl<I1: Nat + MaxN<I2>, F1: Nat + MaxN<F2>, I2: Nat, F2: Nat> JoinNum<Num<I2, F2>>
    for Num<I1, F1>
{
    type Out = Num<<I1 as MaxN<I2>>::Out, <F1 as MaxN<F2>>::Out>;
}

/// The additive result numeral: join, then one more integer bit.
pub trait AddNum<R> {
    type Out;
}
impl<I1: Nat + MaxN<I2>, F1: Nat + MaxN<F2>, I2: Nat, F2: Nat> AddNum<Num<I2, F2>> for Num<I1, F1>
where
    <I1 as MaxN<I2>>::Out: Succ,
{
    type Out = Num<<<I1 as MaxN<I2>>::Out as Succ>::Out, <F1 as MaxN<F2>>::Out>;
}

// ---------------------------------------------------------------------------
// Readback. Every assertion below is a const assertion, so a wrong answer is a
// compile error rather than a runtime print: the check is that the projections
// are const-evaluable and correct, which is the erasure question in miniature.
// ---------------------------------------------------------------------------

pub trait Widths {
    const I: u32;
    const F: u32;
}
impl<I: Nat, F: Nat> Widths for Num<I, F> {
    const I: u32 = I::VAL;
    const F: u32 = F::VAL;
}

// literals
type N0 = End;
type N1 = One<End>;
type N2 = Zero<One<End>>;
type N3 = One<One<End>>;
type N4 = Zero<Zero<One<End>>>;
type N5 = One<Zero<One<End>>>;
type N7 = One<One<One<End>>>;
type N8 = Zero<Zero<Zero<One<End>>>>;
type N11 = One<One<Zero<One<End>>>>;
type N13 = One<Zero<One<One<End>>>>;
type N24 = Zero<Zero<Zero<One<One<End>>>>>;
type N31 = One<One<One<One<One<End>>>>>;
type N47 = One<One<One<One<Zero<One<End>>>>>>;

macro_rules! ck {
    ($lhs:expr, $rhs:expr, $tag:literal) => {
        const _: () = assert!($lhs == $rhs, $tag);
    };
}

// --- the literals read back ---
ck!(<N0 as Nat>::VAL, 0, "N0");
ck!(<N13 as Nat>::VAL, 13, "N13");
ck!(<N31 as Nat>::VAL, 31, "N31");
ck!(<N47 as Nat>::VAL, 47, "N47");

// --- FORMULA: the product numeral of U<3,5> and U<7,11> is U<10,16> ---
type P1 = <Num<N3, N5> as MulNum<Num<N7, N11>>>::Out;
ck!(<P1 as Widths>::I, 10, "mul I");
ck!(<P1 as Widths>::F, 16, "mul F");

// a wider one, at widths a consumer would actually write
type P2 = <Num<N13, N24> as MulNum<Num<N31, N47>>>::Out;
ck!(<P2 as Widths>::I, 44, "mul I wide");
ck!(<P2 as Widths>::F, 71, "mul F wide");

// carry chains: 31 + 1, 31 + 31, 47 + 47
ck!(<<N31 as AddN<N1>>::Out as Nat>::VAL, 32, "carry 31+1");
ck!(<<N31 as AddN<N31>>::Out as Nat>::VAL, 62, "carry 31+31");
ck!(<<N47 as AddN<N47>>::Out as Nat>::VAL, 94, "carry 47+47");
ck!(<<N0 as AddN<N47>>::Out as Nat>::VAL, 47, "0+47");
ck!(<<N47 as AddN<N0>>::Out as Nat>::VAL, 47, "47+0");

// --- EXTREMUM: the join of U<3,5> and U<7,11> is U<7,11> ---
type J1 = <Num<N3, N5> as JoinNum<Num<N7, N11>>>::Out;
ck!(<J1 as Widths>::I, 7, "join I");
ck!(<J1 as Widths>::F, 11, "join F");

// the join is NOT the product: same operands, different answer
const _: () = assert!(<P1 as Widths>::I != <J1 as Widths>::I, "join =/= mul I");
const _: () = assert!(<P1 as Widths>::F != <J1 as Widths>::F, "join =/= mul F");

// a crossing pair, where each operand wins one coordinate
type J2 = <Num<N13, N5> as JoinNum<Num<N3, N24>>>::Out;
ck!(<J2 as Widths>::I, 13, "join cross I");
ck!(<J2 as Widths>::F, 24, "join cross F");

// max over equal arguments, and over zero
ck!(<<N31 as MaxN<N31>>::Out as Nat>::VAL, 31, "max equal");
ck!(<<N0 as MaxN<N47>>::Out as Nat>::VAL, 47, "max 0");
ck!(<<N47 as MaxN<N0>>::Out as Nat>::VAL, 47, "max 0 rev");
// the case a bit-by-bit max would get wrong: 8 against 7, where the low bits
// all favour 7 and only the high bit decides.
ck!(
    <<N8 as MaxN<N7>>::Out as Nat>::VAL,
    8,
    "max 8v7 high-end decides"
);
ck!(
    <<N7 as MaxN<N8>>::Out as Nat>::VAL,
    8,
    "max 7v8 high-end decides"
);
ck!(<<N11 as MaxN<N13>>::Out as Nat>::VAL, 13, "max 11v13");
ck!(<<N4 as MaxN<N2>>::Out as Nat>::VAL, 4, "max 4v2");

// --- the additive result numeral: join then +1 on the integer side ---
type A1 = <Num<N3, N5> as AddNum<Num<N7, N11>>>::Out;
ck!(<A1 as Widths>::I, 8, "add I");
ck!(<A1 as Widths>::F, 11, "add F");

#[no_mangle]
pub extern "C" fn probe_entry() -> u32 {
    // every projection folded to a constant at compile time; this is the
    // erasure question asked at the smallest scale that can answer it.
    <P2 as Widths>::I + <P2 as Widths>::F + <J1 as Widths>::I + <A1 as Widths>::I
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
