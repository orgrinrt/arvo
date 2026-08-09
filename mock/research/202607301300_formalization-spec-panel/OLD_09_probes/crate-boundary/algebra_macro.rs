//! variant of algebra.rs that additionally exports a macro generating the
//! ONE forwarding impl body, with no slot for an extra where-clause. Tests
//! whether a macro-closed forwarding site can foreclose the first-party
//! loophole `numeric_dishonest.rs` demonstrates is otherwise open.
#![crate_type = "rlib"]
#![crate_name = "algebra_macro"]

extern crate numeral;
extern crate policy;

use numeral::Numeral;
use policy::Policy;

pub trait IsTrue {}
pub struct True;
pub struct False;
impl IsTrue for True {}

pub trait StableUnderTranslation {
    type Out;
}
impl StableUnderTranslation for policy::ReduceModulo {
    type Out = True;
}
impl StableUnderTranslation for policy::SubstituteZero {
    type Out = False;
}
impl StableUnderTranslation for policy::Refuse {
    type Out = True;
}

pub trait AddAssoc {}
pub struct Fact<N, P>(core::marker::PhantomData<(N, P)>);
impl<N: Numeral, P: Policy> AddAssoc for Fact<N, P>
where
    P::OverRange: StableUnderTranslation,
    <P::OverRange as StableUnderTranslation>::Out: IsTrue,
{
}

// The macro is the closure attempt: its pattern takes exactly a type path
// with three generic idents and always emits the same fixed impl body.
// There is no `$extra:tt` slot, so a caller cannot splice an additional
// where-clause into the emitted impl through this macro's own grammar.
#[macro_export]
macro_rules! derive_add_assoc {
    ($ty:ident < $n:ident, $p:ident, $l:ident >) => {
        impl<$n: $crate::__NumeralReexport, $p: $crate::__PolicyReexport, $l> $crate::AddAssoc
            for $ty<$n, $p, $l>
        where
            $crate::Fact<$n, $p>: $crate::AddAssoc,
        {
        }
    };
}

// re-exported purely so the macro's hygiene can name the bounds without
// numeric_macro.rs needing its own `extern crate numeral;` bound alias.
pub use numeral::Numeral as __NumeralReexport;
pub use policy::Policy as __PolicyReexport;
