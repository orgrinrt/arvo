//! The shape's `Constant<const V: i128>` with `const MEMBER: Self`, "refused at
//! build where V is none". The question is where the refusal sits. The impl
//! has to cover every `V` (a const generic cannot be bounded by range on this
//! toolchain without `generic_const_exprs`), so the bound `T: Constant<300>`
//! is satisfied at an eight-bit type for which 300 is no member, and the
//! refusal only arrives when `MEMBER` is evaluated after monomorphisation.
//! `run.sh` builds this crate under `cargo check` and under `cargo build`.

use arvo_format::Adapt;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{DebugCheck, Int, Num, Quiet, Sole};

pub trait Constant<const V: i128>: Sized {
    const MEMBER: Self;
}

impl<S: Sole, C: DebugCheck, const V: i128> Constant<V> for Num<S, C> {
    const MEMBER: Self = Num::constant::<V>();
}

pub type I8 = Int<8, Adapt<TowardZero, Saturate>, Quiet>;

/// The bound typechecks: nothing about 300 is checked here.
pub fn three_hundred<T: Constant<300>>() -> T {
    T::MEMBER
}

/// And it typechecks at `I8` too, where 300 is no member.
pub fn at_i8() -> I8 {
    three_hundred::<I8>()
}
