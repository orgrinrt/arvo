// Four surface spellings over one tower, so consumer call sites can be compiled
// against each without changing anything underneath.
#![no_std]
#![allow(dead_code)]

extern crate tower;
pub use tower::*;

// ============ Spelling A: const parameters + the generated table =========
pub struct Idx<const N: u16>;

#[diagnostic::on_unimplemented(
    message = "width `{Self}` is outside the widths arvo admits",
    note = "a written width must lie in 0 ..= 31"
)]
pub trait AdmittedWidth {
    type Out: Nat;
}

macro_rules! row {
    ($n:ty) => {
        impl AdmittedWidth for Idx<{ <$n as Nat>::VAL as u16 }> {
            type Out = $n;
        }
    };
}
macro_rules! emit {
    ( [] ; $($n:ty),* ) => { $( row!(Pz<$n>); )* };
    ( [ $_x:tt $($d:tt)* ] ; $($n:ty),* ) => {
        $( row!(Pz<$n>); )*
        emit!( [ $($d)* ] ; $( O<$n>, I<$n> ),* );
    };
}
impl AdmittedWidth for Idx<0> {
    type Out = Z;
}
emit!( [x x x x x x x x x x x] ; H );

pub type NatOf<const W: u16> = <Idx<W> as AdmittedWidth>::Out;

// The public alias, exactly as D48 and D31 spell it.
pub type UFixed<const I: u16, const F: u16, S> =
    Number<FixedNumeral<Sum<NatOf<{ I }>, NatOf<{ F }>>, NonNegative>, S>;

// ============ Spelling C: the width is a type parameter ==================
pub type UFixedT<Iw, Fw, S> = Number<FixedNumeral<Sum<Iw, Fw>, NonNegative>, S>;

// A generated alias family, the naming convenience for spelling C.
#[allow(unused_macros)]
macro_rules! alias_row {
    ($nm:ident, $n:ty) => {
        pub type $nm = $n;
    };
}
pub type W0 = Z;
pub type W1 = Pz<H>;
pub type W2 = Pz<O<H>>;
pub type W3 = Pz<I<H>>;
pub type W7 = Pz<I<I<H>>>;
pub type W13 = Pz<I<O<I<H>>>>;
pub type W31 = Pz<I<I<I<I<H>>>>>;
pub type W40 = Pz<O<O<O<I<O<H>>>>>>;
pub type W30 = Pz<O<I<I<I<H>>>>>;
pub type W1000 = Pz<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>;

// ============ Spelling B: a call-site macro over the table ===============
#[macro_export]
macro_rules! ufixed {
    ($i:literal, $f:literal, $s:ty) => {
        $crate::Number<
            $crate::FixedNumeral<
                $crate::Sum<$crate::NatOf<$i>, $crate::NatOf<$f>>,
                $crate::NonNegative,
            >,
            $s,
        >
    };
}

// ============ Spelling D: digits munched into the tower's arithmetic =====
// No table at all past ten digit rows. Decimal is built by the tower's own
// multiplication-by-repeated-addition, which is type-level arithmetic that
// already exists.
pub type D0 = Z;
pub type D1 = Pz<H>;
pub type D2 = Pz<O<H>>;
pub type D3 = Pz<I<H>>;
pub type D4 = Pz<O<O<H>>>;
pub type D5 = Pz<I<O<H>>>;
pub type D6 = Pz<O<I<H>>>;
pub type D7 = Pz<I<I<H>>>;
pub type D8 = Pz<O<O<O<H>>>>;
pub type D9 = Pz<I<O<O<H>>>>;

// x * 10 = ((x+x)+(x+x)) + ((x+x)+(x+x)) + (x+x), four adds via doubling
pub type Dbl<X> = Sum<X, X>;
pub type Times10<X> = Sum<Dbl<Dbl<Dbl<X>>>, Dbl<X>>;

#[macro_export]
macro_rules! digit {
    (0) => {
        $crate::D0
    };
    (1) => {
        $crate::D1
    };
    (2) => {
        $crate::D2
    };
    (3) => {
        $crate::D3
    };
    (4) => {
        $crate::D4
    };
    (5) => {
        $crate::D5
    };
    (6) => {
        $crate::D6
    };
    (7) => {
        $crate::D7
    };
    (8) => {
        $crate::D8
    };
    (9) => {
        $crate::D9
    };
}

#[macro_export]
macro_rules! w {
    // accumulator form: w!(@ acc ; remaining digits)
    (@ $acc:ty ; ) => { $acc };
    (@ $acc:ty ; $d:tt $($rest:tt)* ) => {
        $crate::w!(@ $crate::Sum<$crate::Times10<$acc>, $crate::digit!($d)> ; $($rest)*)
    };
    ( $($d:tt)+ ) => { $crate::w!(@ $crate::D0 ; $($d)+) };
}

#[macro_export]
macro_rules! w_ufixed {
    ( [ $($i:tt)+ ] , [ $($f:tt)+ ] ) => {
        $crate::Number<
            $crate::FixedNumeral<
                $crate::Sum<$crate::w!($($i)+), $crate::w!($($f)+)>,
                $crate::NonNegative,
            >,
            $crate::Warm,
        >
    };
}
