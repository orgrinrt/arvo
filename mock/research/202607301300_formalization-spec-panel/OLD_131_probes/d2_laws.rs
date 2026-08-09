//! The strategy picks the container. `UFixed<13, 3, Warm>`, three written
//! parameters, container derived from (strategy, widths, sign).
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

// ---------------------------------------------------------------- sign axis
pub trait Sign: Copy {
    const EXTRA: u32;
}
#[derive(Clone, Copy)]
pub struct Unsigned;
#[derive(Clone, Copy)]
pub struct Signed;
impl Sign for Unsigned {
    const EXTRA: u32 = 0;
}
impl Sign for Signed {
    const EXTRA: u32 = 1;
}

// -------------------------------------------------- the ladder the hw has
#[derive(Clone, Copy)]
pub struct Wide<const BYTES: usize, A>([u8; BYTES], PhantomData<A>);
#[derive(Clone, Copy)]
pub struct A1;
#[derive(Clone, Copy)]
pub struct A16;

pub trait Project<const TAG: usize, G: Sign, const BYTES: usize, S> {
    type T: Copy;
}
pub struct Picker;
pub struct Hot;
pub struct Cold;
pub struct Warm;
pub struct Precise;

macro_rules! native {
    ($s:ty, [$($ut:literal => $u:ty),+], [$($it:literal => $i:ty),+]) => {
        $(impl<const B: usize> Project<$ut, Unsigned, B, $s> for Picker { type T = $u; })+
        $(impl<const B: usize> Project<$it, Signed,   B, $s> for Picker { type T = $i; })+
    };
}
native!(Hot,     [0=>u8,1=>u16,2=>u32,3=>u64,4=>u128], [0=>i8,1=>i16,2=>i32,3=>i64,4=>i128]);
native!(Cold,    [0=>u8,1=>u16,2=>u32,3=>u64,4=>u128], [0=>i8,1=>i16,2=>i32,3=>i64,4=>i128]);
native!(Warm,    [0=>u16,1=>u32,2=>u64,3=>u128],       [0=>i16,1=>i32,2=>i64,3=>i128]);
native!(Precise, [0=>u16,1=>u32,2=>u64,3=>u128],       [0=>i16,1=>i32,2=>i64,3=>i128]);
impl<G: Sign, const B: usize> Project<5, G, B, Hot> for Picker {
    type T = Wide<B, A16>;
}
impl<G: Sign, const B: usize> Project<5, G, B, Cold> for Picker {
    type T = Wide<B, A1>;
}
impl<G: Sign, const B: usize> Project<5, G, B, Warm> for Picker {
    type T = Wide<B, A1>;
}
impl<G: Sign, const B: usize> Project<5, G, B, Precise> for Picker {
    type T = Wide<B, A1>;
}

pub const fn tag_min(w: u32) -> usize {
    if w <= 8 {
        0
    } else if w <= 16 {
        1
    } else if w <= 32 {
        2
    } else if w <= 64 {
        3
    } else if w <= 128 {
        4
    } else {
        5
    }
}
pub const fn tag_headroom(w: u32) -> usize {
    if w <= 8 {
        0
    } else if w <= 16 {
        1
    } else if w <= 32 {
        2
    } else if w <= 64 {
        3
    } else {
        5
    }
}
pub const fn bytes_for(w: u32) -> usize {
    (w as usize).div_ceil(8)
}

pub struct Rung<const I: u32, const F: u32, G: Sign, S>(PhantomData<(G, S)>);
pub trait Tagged {
    type const TAG: usize;
    type const BYTES: usize;
}
macro_rules! tagged {
    ($s:ty, $f:ident) => {
        impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, $s> {
            type const TAG: usize = const { $f(G::EXTRA + I + F) };
            type const BYTES: usize = const { bytes_for(G::EXTRA + I + F) };
        }
    };
}
tagged!(Hot, tag_min);
tagged!(Cold, tag_min);
tagged!(Warm, tag_headroom);
tagged!(Precise, tag_headroom);

/// The container level: derived, never declared as an axis.
pub trait Store<const I: u32, const F: u32, G: Sign> {
    type T: Copy;
}
macro_rules! store {
    ($s:ty) => {
        impl<const I: u32, const F: u32, G: Sign> Store<I, F, G> for $s
        where
            Picker: Project<
                { <Rung<I, F, G, $s> as Tagged>::TAG },
                G,
                { <Rung<I, F, G, $s> as Tagged>::BYTES },
                $s,
            >,
        {
            type T = <Picker as Project<
                { <Rung<I, F, G, $s> as Tagged>::TAG },
                G,
                { <Rung<I, F, G, $s> as Tagged>::BYTES },
                $s,
            >>::T;
        }
    };
}
store!(Hot);
store!(Cold);
store!(Warm);
store!(Precise);

// ------------------------------------------------------------- the numeral
pub struct Fixed<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> {
    raw: <S as Store<I, F, G>>::T,
    _m: PhantomData<G>,
}
impl<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> Copy for Fixed<I, F, G, S> {}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, S> = Fixed<I, F, Signed, S>;

// ----------------------------------- the mathematical coordinates, as reads
pub trait Format {
    const PRECISION: u32; // significand digits: I + F, sign-free (D69)
    const EXPONENT: i32; // the quantum: -F
    const INTEGER_DIGITS: u32;
    const FRACTION_DIGITS: u32;
    const SIGNED: bool;
    const STORED_WIDTH: u32; // derived on the physical side (D69)
}
impl<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> Format for Fixed<I, F, G, S> {
    const PRECISION: u32 = I + F;
    const EXPONENT: i32 = -(F as i32);
    const INTEGER_DIGITS: u32 = I;
    const FRACTION_DIGITS: u32 = F;
    const SIGNED: bool = G::EXTRA == 1;
    const STORED_WIDTH: u32 = G::EXTRA + I + F;
}

// ============================== the laws, as named items =================
pub struct SumFormat<const I: u32, const J: u32, const F: u32, const M: u32>;
impl<const I: u32, const J: u32, const F: u32, const M: u32> SumFormat<I, J, F, M> {
    pub const HOLDS: () = assert!(
        M == if I > J { I } else { J } + 1,
        "arvo: the sum's format does not follow from its inputs.
         The law: aligned Fixed<I, F> plus Fixed<J, F> has format Fixed<max(I, J) + 1, F>.
         The line above prints SumFormat::<I, J, F, M> with the actual digit counts,
         in that order. Name the output with max of the first two, plus one, and the
         same F. If the call sits inside a function you did not write, that function
         states a format relation that does not hold, and the note below names it.
         Search your own source for the last number printed above to find your call."
    );
}
pub struct ProductFormat<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
>;
impl<const I: u32, const F: u32, const J: u32, const K: u32, const M: u32, const N: u32>
    ProductFormat<I, F, J, K, M, N>
{
    pub const HOLDS: () = assert!(
        M == I + J && N == F + K,
        "arvo: the product's format does not follow from its inputs.
         The law: Fixed<I, F> times Fixed<J, K> has format Fixed<I + J, F + K>.
         The line above prints ProductFormat::<I, F, J, K, M, N> with the actual
         digit counts, in that order. Name the output with the first four added
         pairwise. If the call sits inside a function you did not write, that
         function states a format relation that does not hold, and the note below
         names the function and its line. Search your own source for the last two
         numbers printed above to find which of your calls reached it."
    );
}

// ============================== the operations ===========================
/// Alignment is an equality between coordinates, so it is a bound: both
/// arguments name the same F, checked by unification before any const eval.
pub fn add<const I: u32, const J: u32, const F: u32, const M: u32, G: Sign, S>(
    _a: Fixed<I, F, G, S>,
    _b: Fixed<J, F, G, S>,
) -> Fixed<M, F, G, S>
where
    S: Store<I, F, G> + Store<J, F, G> + Store<M, F, G>,
{
    let () = SumFormat::<I, J, F, M>::HOLDS;
    unimplemented!()
}

pub fn mul<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
    G: Sign,
    S,
>(
    _a: Fixed<I, F, G, S>,
    _b: Fixed<J, K, G, S>,
) -> Fixed<M, N, G, S>
where
    S: Store<I, F, G> + Store<J, K, G> + Store<M, N, G>,
{
    let () = ProductFormat::<I, F, J, K, M, N>::HOLDS;
    unimplemented!()
}

/// The scale change is an operation with a name, because the exponent is in
/// the type. It is never an assignment and never a `From`.
pub fn rescale<const I: u32, const F: u32, const J: u32, const K: u32, G: Sign, S>(
    _a: Fixed<I, F, G, S>,
) -> Fixed<J, K, G, S>
where
    S: Store<I, F, G> + Store<J, K, G>,
{
    unimplemented!()
}

/// Widening the integer part at a fixed exponent is total and value-preserving,
/// so it is the one conversion that may be implicit.
pub fn widen<const I: u32, const F: u32, const J: u32, G: Sign, S>(
    _a: Fixed<I, F, G, S>,
) -> Fixed<J, F, G, S>
where
    S: Store<I, F, G> + Store<J, F, G>,
{
    const { assert!(true) }
    unimplemented!()
}

// ============================== what a consumer writes ===================
pub fn arithmetic(a: UFixed<13, 3, Warm>, b: UFixed<13, 3, Warm>) {
    let _p: UFixed<26, 6, Warm> = mul(a, b);
    let _s: UFixed<14, 3, Warm> = add(a, b);
    let _r: UFixed<8, 8, Warm> = rescale(a);
    let _w: UFixed<20, 3, Warm> = widen(a);
}
pub fn signed(a: IFixed<12, 3, Warm>) {
    let _p: IFixed<24, 6, Warm> = mul(a, a);
}
