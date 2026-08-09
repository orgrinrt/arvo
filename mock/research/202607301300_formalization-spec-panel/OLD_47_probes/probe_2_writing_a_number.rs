//! Probe 2: three ways for a consumer to write "37", priced against each
//! other on the two things that matter, what gets typed and what gets read
//! back in a diagnostic.
//!
//! (a) The encoding, verbatim. Probe 1's form.
//! (b) A generated alias table, `N37`.
//! (c) A `nat!(37)` macro resolving through the table by name concatenation,
//!     using `macro_metavar_expr_concat`, which arvo already enables
//!     (`crates/arvo/src/lib.rs:26`).
//!
//! CLAIM A: all three denote the same type, so value-uniqueness is untouched
//! by any of them; the alias and the macro are spellings, not new numerals.
//! Asserted by using them interchangeably at a fn boundary that admits only
//! one type.
//!
//! CLAIM B: the macro form costs nothing at the trait solver, because it
//! resolves by name, not by type-level arithmetic. Contrast with a
//! digit-munching macro that builds the number through the tower's own
//! `Dbl`/`DblInc`, which is probe 2b.
//!
//! EXPECTED: COMPILES CLEAN.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_2_writing_a_number.rs

#![allow(dead_code)]
#![feature(macro_metavar_expr_concat)]

use tower::nat::{Nat, Pz, H, I, O};

// ---------------------------------------------------------------------------
// (b) The generated alias table. Six rows shown; the real one is emitted for
// 0..=1024 by the same generator that already emits arvo's per-width impls.
// A `type` alias emits no symbol and no metadata of its own.
// ---------------------------------------------------------------------------

pub mod n {
    use tower::nat::{Pz, H, I, O};
    pub type N0 = tower::nat::Z;
    pub type N1 = Pz<H>;
    pub type N15 = Pz<I<I<I<H>>>>;
    pub type N16 = Pz<O<O<O<O<H>>>>>;
    pub type N21 = Pz<I<O<I<O<H>>>>>;
    pub type N24 = Pz<O<O<O<I<H>>>>>;
    pub type N37 = Pz<I<O<I<O<O<H>>>>>>;
    pub type N53 = Pz<I<O<I<O<I<H>>>>>>;
}

// ---------------------------------------------------------------------------
// (c) The macro. One rule, no recursion, no arithmetic: it concatenates the
// literal into the table's own alias name.
// ---------------------------------------------------------------------------

#[macro_export]
macro_rules! nat {
    ($v:literal) => { $crate::n::${concat(N, $v)} };
}

// ---------------------------------------------------------------------------
// CLAIM A: one type, three spellings.
// ---------------------------------------------------------------------------

pub struct Accumulator<P: Nat>(core::marker::PhantomData<P>);

/// Admits exactly one type.
pub fn needs_37(_: Accumulator<Pz<I<O<I<O<O<H>>>>>>>) {}

pub fn all_three_are_one_type(
    verbatim: Accumulator<Pz<I<O<I<O<O<H>>>>>>>,
    aliased: Accumulator<n::N37>,
    macroed: Accumulator<nat!(37)>,
) {
    needs_37(verbatim);
    needs_37(aliased);
    needs_37(macroed);
}

const _: () = assert!(<n::N37 as Nat>::VAL == 37);
const _: () = assert!(<nat!(37) as Nat>::VAL == 37);
const _: () = assert!(<nat!(53) as Nat>::VAL == 53);

// ---------------------------------------------------------------------------
// What probe 1's declarations become. This is the whole of the writing cost
// difference, side by side.
// ---------------------------------------------------------------------------

pub type SamplePrecisionVerbatim = Pz<I<I<I<H>>>>;
pub type SamplePrecisionWritten = nat!(15);

pub type AccumPrecisionVerbatim = Pz<I<O<I<O<O<H>>>>>>;
pub type AccumPrecisionWritten = nat!(37);

pub type Binary64Verbatim = Pz<I<O<I<O<I<H>>>>>>;
pub type Binary64Written = nat!(53);
