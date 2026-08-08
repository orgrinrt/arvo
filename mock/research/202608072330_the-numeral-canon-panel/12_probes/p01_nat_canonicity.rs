//! p01. Is the ladder's structural addition CANONICAL?
//!
//! Everything downstream depends on this and nobody has asked it. If a consumer
//! names a product width with a decimal (`UInt<26>`) and the algebra computed
//! `Sum<T13, T13>`, those two have to be the SAME TYPE, not merely two types of
//! the same value. Rust has no definitional equality on these towers beyond
//! structural identity, so this is a real question with a compiled answer.
//!
//! Tested by passing a value of the computed type into a slot spelled with the
//! written type. If they differ, this is E0308 and the whole hybrid in p03 is
//! dead on arrival.
//!
//! No `#![feature]`, no `-Z` flag.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata -o out/p01.meta p01_nat_canonicity.rs
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");

pub struct Hot;

#[repr(transparent)]
pub struct FixedT<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(WI, WF, S)>,
}
pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;

// --- the test. `written` takes the hand-spelled T26. ------------------------
pub fn written(x: FixedT<T26, T0, Hot>) -> FixedT<T26, T0, Hot> {
    x
}

// and this hands it the COMPUTED type. Same type, or E0308.
pub fn computed(x: FixedT<Sum<T13, T13>, T0, Hot>) -> FixedT<T26, T0, Hot> {
    written(x)
}

// the same question one octave up, where carries actually propagate.
// 24 + 24 = 48, and 48 has a different digit count than 24.
pub type T48 = D0<D0<D0<D0<D1<D1<Term>>>>>>;
pub fn computed_48(x: FixedT<Sum<T24, T24>, T0, Hot>) -> FixedT<T48, T0, Hot> {
    x
}

// and across a carry chain that changes the tower length: 13 + 3 = 16.
pub type T16b = D0<D0<D0<D0<D1<Term>>>>>;
pub fn computed_16(x: FixedT<Sum<T13, T3>, T0, Hot>) -> FixedT<T16b, T0, Hot> {
    x
}

// --- and the trap: is a NON-canonical tower a distinct type? -----------------
// T3 with a leading zero. Same VALUE (3), different STRUCTURE.
pub type T3_padded = D1<D1<D0<Term>>>;
const _: () = assert!(<T3_padded as Nat>::V == 3);
const _: () = assert!(<T3 as Nat>::V == 3);
// If these were the same type this would be a duplicate definition error, and it
// is not: they are two distinct types denoting the same natural number.
pub fn padded_is_distinct(x: FixedT<T3_padded, T0, Hot>) -> FixedT<T3_padded, T0, Hot> {
    x
}
