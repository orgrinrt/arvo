// GENERATED: the door and numeral from q09, extracted.
// ------------------------------------------------------------- the door -----
// One impl per literal a program writes. This is the bridge, and it is the
// table every candidate in the second stretch has. Its domain is the literals
// in the source text, per 11, 12 and 13 arriving at that separately.
pub struct L<const K: u32>;
pub trait Lit {
    type N;
}
include!("q09_lit_inc.rs");
pub type NatOf<const K: u32> = <L<K> as Lit>::N;

// --------------------------------------------------------- the numeral ------
// Keyed on (total width, fraction width). The integer width is a derived view,
// never stored, which is exactly why the negative-integer-width case costs
// nothing: see q01 and q06.
pub struct Numeral<W, F, Sn, S>(core::marker::PhantomData<(W, F, Sn, S)>);

pub type UFixed<const I: u32, const F: u32, Sn, S> =
    Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>, Sn, S>;
pub type UInt<const I: u32, Sn, S> = Numeral<NatOf<I>, N0, Sn, S>;

// The whole derivation named once, so the bound avalanche is arvo's cost paid
// in one impl and never reaches a consumer. 13:214-231 is the same repair for a
// two-input map; this is it for three.
pub trait Derived {
    type Container;
    type Stride;
    type Width;
    type Frac;
}
impl<W, F, Sn: Signedness, S> Derived for Numeral<W, F, Sn, S>
where
    S: Realise<Sn, W>,
{
    type Container = <S as Realise<Sn, W>>::Container;
    type Stride = <S as Realise<Sn, W>>::Stride;
    type Width = W;
    type Frac = F;
}

pub type Cont<T> = <T as Derived>::Container;
pub type Strd<T> = <T as Derived>::Stride;
