//! P6: all four parts of the gate at once, on one crate, gate-free.
//! Consumer writes bits. Typestate derives container. It validates. It erases.
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");

// --- strategy markers ---------------------------------------------------------
pub struct Hot;
pub struct Warm;

// --- the const-to-type bridge, carrying a marker so a consumer can extend it --
pub struct Idx<const N: u32>;
pub trait ToNat<M> {
    type N;
}
pub struct Arvo;

macro_rules! bridge { ($($n:literal => $t:ty),* $(,)?) => { $( impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
bridge! {
    0 => T0, 3 => T3, 8 => T8, 13 => T13, 16 => T16, 24 => T24,
    6 => T6, 26 => T26, 30 => T30, 40 => T40, 64 => T64, 100 => T100, 200 => T200,
}

// --- the numeral: one real field, everything else a ZST marker ---------------
#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, S, M = Arvo>
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>,
    <Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
    Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
{
    raw: Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>,
    _m: PhantomData<(S, M)>,
}

pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, S>;

impl<const I: u32, const F: u32, S, M> Clone for Fixed<I, F, S, M>
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>,
    <Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
    Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
    Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>: Wrapping,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, S, M> Copy for Fixed<I, F, S, M>
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>,
    <Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
    Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
    Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>: Wrapping,
{
}

// the operation, written once, over whatever the ladder produced
impl<const I: u32, const F: u32, S, M> Fixed<I, F, S, M>
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>,
    <Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
    Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
    Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>: Wrapping,
{
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Fixed {
            raw: Wrapping::wadd(self.raw, o.raw),
            _m: PhantomData,
        }
    }
}

// --- erasure, asserted at compile time ---------------------------------------
const _: () = assert!(core::mem::size_of::<UFixed<13, 3, Hot>>() == core::mem::size_of::<u16>());
const _: () = assert!(core::mem::align_of::<UFixed<13, 3, Hot>>() == core::mem::align_of::<u16>());
const _: () = assert!(core::mem::size_of::<UFixed<40, 24, Hot>>() == core::mem::size_of::<u64>());
const _: () = assert!(core::mem::size_of::<UFixed<3, 0, Hot>>() == 1);
const _: () = assert!(core::mem::size_of::<UFixed<100, 100, Hot>>() == 32); // 200 bits, 4 words
const _: () = assert!(core::mem::size_of::<UFixed<200, 24, Hot>>() == 32); // 224 bits, 4 words

// --- codegen ------------------------------------------------------------------
#[unsafe(no_mangle)]
pub fn arvo16(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> UFixed<13, 3, Hot> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn native16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub fn arvo64(a: UFixed<40, 24, Hot>, b: UFixed<40, 24, Hot>) -> UFixed<40, 24, Hot> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn native64(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub fn arvo_wide200(a: UFixed<100, 100, Hot>, b: UFixed<100, 100, Hot>) -> UFixed<100, 100, Hot> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn bar_wide256(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let (r0, c) = a[0].carrying_add(b[0], false);
    let (r1, c) = a[1].carrying_add(b[1], c);
    let (r2, c) = a[2].carrying_add(b[2], c);
    let (r3, _) = a[3].carrying_add(b[3], c);
    [r0, r1, r2, r3]
}
#[unsafe(no_mangle)]
pub fn arvo_vec(x: &mut [UFixed<13, 3, Hot>; 1024], y: &[UFixed<13, 3, Hot>; 1024]) {
    for i in 0..1024 {
        x[i] = x[i].add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub fn native_vec(x: &mut [u16; 1024], y: &[u16; 1024]) {
    for i in 0..1024 {
        x[i] = x[i].wrapping_add(y[i]);
    }
}

// --- a consumer bringing its own marker and its own widths, per 134c ---------
pub struct Mine;
impl ToNat<Mine> for Idx<777> {
    type N = T777;
}
impl ToNat<Mine> for Idx<41> {
    type N = T41;
}
#[unsafe(no_mangle)]
pub fn consumer_818(
    a: Fixed<777, 41, Hot, Mine>,
    b: Fixed<777, 41, Hot, Mine>,
) -> Fixed<777, 41, Hot, Mine> {
    a.add(b)
}
const _: () = assert!(core::mem::size_of::<Fixed<777, 41, Hot, Mine>>() == 104); // 818 bits, 13 words

// --- a width-generic law, stated over the CONST coordinates ------------------
// The output coordinates are const parameters too, pinned to the structural sum
// by an associated-type equality bound. No const arithmetic anywhere.
pub fn mul<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const OI: u32,
    const OF: u32,
    S,
    M,
>(
    _a: Fixed<I, F, S, M>,
    _b: Fixed<J, K, S, M>,
) -> Fixed<OI, OF, S, M>
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>,
    Idx<J>: ToNat<M>,
    Idx<K>: ToNat<M>,
    Idx<OI>: ToNat<M>,
    Idx<OF>: ToNat<M>,
    <Idx<I> as ToNat<M>>::N:
        Add<<Idx<F> as ToNat<M>>::N> + Add<<Idx<J> as ToNat<M>>::N, O = <Idx<OI> as ToNat<M>>::N>,
    <Idx<J> as ToNat<M>>::N: Add<<Idx<K> as ToNat<M>>::N>,
    <Idx<F> as ToNat<M>>::N: Add<<Idx<K> as ToNat<M>>::N, O = <Idx<OF> as ToNat<M>>::N>,
    <Idx<OI> as ToNat<M>>::N: Add<<Idx<OF> as ToNat<M>>::N>,
    Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
    Sum<<Idx<J> as ToNat<M>>::N, <Idx<K> as ToNat<M>>::N>: Container,
    Sum<<Idx<OI> as ToNat<M>>::N, <Idx<OF> as ToNat<M>>::N>: Container,
    Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>: Wrapping,
    Cont<Sum<<Idx<J> as ToNat<M>>::N, <Idx<K> as ToNat<M>>::N>>: Wrapping,
    Cont<Sum<<Idx<OI> as ToNat<M>>::N, <Idx<OF> as ToNat<M>>::N>>: Wrapping,
{
    todo!()
}

// concrete site: 13.3 times 13.3 is 26.6, and rustc infers OI and OF
pub fn law_site(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> Fixed<26, 6, Hot> {
    mul(a, b)
}
