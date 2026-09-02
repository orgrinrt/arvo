// surfaces.rs. The four live candidate surfaces as includable definitions, with
// no inner attributes and no inner doc comments so that `include!` works from
// more than one file. Definitions are identical to p04's; p04 keeps its own copy
// because its SITE markers are what `count.sh` measures.

pub struct Hot;
pub struct Warm;
pub struct Arvo;
pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;

#[repr(transparent)]
pub struct Fixed<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(WI, WF, S)>,
}

pub type T5 = D1<D0<D1<Term>>>;
pub type T32 = D0<D0<D0<D0<D0<D1<Term>>>>>>;

// C0. Const surface, const keying. The design as it stands.
pub mod c0 {
    use super::*;
    pub struct Idx<const N: u32>;
    #[diagnostic::on_unimplemented(
        message = "arvo does not ship this width: {Self}",
        label = "this numeral names a width arvo does not ship",
        note = "widths are opt-in per program. Add `impl ToNat<MyWidths> for {Self}` and spell the numeral against `MyWidths`"
    )]
    pub trait ToNat<M> {
        type N;
    }
    macro_rules! d { ($($n:literal => $t:ty),* $(,)?) => { $( #[diagnostic::do_not_recommend] impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
    d! { 0 => T0, 3 => T3, 5 => T5, 13 => T13, 16 => T16, 26 => T26, 6 => T6, 32 => T32 }

    #[repr(transparent)]
    pub struct CFixed<const I: u32, const F: u32, S, M = Arvo>
    where
        Idx<I>: ToNat<M>,
        Idx<F>: ToNat<M>,
        <Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
        Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
    {
        raw: Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>,
        _m: PhantomData<(S, M)>,
    }
    pub type UInt<const N: u32> = CFixed<N, 0, Warm>;
    pub type UFixed<const I: u32, const F: u32, S> = CFixed<I, F, S>;

    pub type StrHandle = UInt<5>;
    pub type Coord = UFixed<13, 3, Hot>;
    pub type Product = UFixed<26, 6, Hot>;
}

// C1. Raw nat surface, no alias layer at all.
pub mod c1 {
    use super::*;
    pub type StrHandle = Fixed<D1<D0<D1<Term>>>, Term, Warm>;
    pub type Coord = Fixed<D1<D0<D1<D1<Term>>>>, D1<D1<Term>>, Hot>;
    pub type Product = Fixed<D0<D1<D0<D1<D1<Term>>>>>, D0<D1<D1<Term>>>, Hot>;
}

// C2. Nat surface with a shipped alias layer of NAMES.
pub mod c2 {
    use super::*;
    pub type N0 = Term;
    pub type N3 = T3;
    pub type N5 = T5;
    pub type N6 = T6;
    pub type N13 = T13;
    pub type N26 = T26;
    pub type UInt<W> = Fixed<W, N0, Warm>;
    pub type UFixed<WI, WF, S> = Fixed<WI, WF, S>;

    pub type StrHandle = UInt<N5>;
    pub type Coord = UFixed<N13, N3, Hot>;
    pub type Product = UFixed<N26, N6, Hot>;
}

// C4. The hybrid: const at the door, nat underneath.
pub mod c4 {
    use super::*;
    pub struct Idx<const N: u32>;
    #[diagnostic::on_unimplemented(
        message = "no width literal `{Self}` is declared in this program",
        label = "this literal width is not declared",
        note = "literal widths are declared per program, one line each. Declare it, or name a width you already have"
    )]
    pub trait ToNat<M> {
        type N;
    }
    macro_rules! d { ($($n:literal => $t:ty),* $(,)?) => { $( #[diagnostic::do_not_recommend] impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
    d! { 0 => T0, 3 => T3, 5 => T5, 13 => T13, 26 => T26, 6 => T6 }
    pub type NatOf<const N: u32> = <Idx<N> as ToNat<Arvo>>::N;
    pub type UInt<const N: u32> = Fixed<NatOf<N>, T0, Warm>;
    pub type UFixed<const I: u32, const F: u32, S> = Fixed<NatOf<I>, NatOf<F>, S>;

    pub type StrHandle = UInt<5>;
    pub type Coord = UFixed<13, 3, Hot>;
    pub type Product = UFixed<26, 6, Hot>;
}
