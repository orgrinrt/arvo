//! Probe 1b: a mistyped numeral is a well-formed numeral.
//!
//! The value-unique encoding guarantees one type per value. It does not, and
//! cannot, guarantee that the type a consumer typed is the value they meant.
//! Every `Pos` built from `H`/`O`/`I` is a legal `Pos`; a dropped or swapped
//! constructor produces a different legal number, silently.
//!
//! This matters more here than for an ordinary literal because the encoding
//! is positional binary written inside-out, so the edit distance between 37
//! and 21 is one character in the middle of a nest six deep.
//!
//! EXPECTED: COMPILES CLEAN, which is the finding. The consumer wrote the
//! accumulator precision for a three-million-element fold and got 21 instead
//! of 37: sixteen digits of headroom missing, an accumulator that overflows
//! on the real column, and no diagnostic anywhere.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_1b_a_wrong_digit_is_silent.rs

#![allow(dead_code)]

use tower::nat::{Nat, Pz, H, I, O};

/// What probe 1 says the accumulator precision is: 37.
pub type Intended = Pz<I<O<I<O<O<H>>>>>>;

/// One constructor changed: the third `O` became an `I`. Still a `Nat`,
/// still sealed, still unique, still admitted at every bounded position.
pub type Typo = Pz<I<O<I<O<I<H>>>>>>;

const _: () = assert!(<Intended as Nat>::VAL == 37);
const _: () = assert!(<Typo as Nat>::VAL == 53);

/// And the one that is genuinely dangerous: a dropped constructor, which is
/// what an editor's bracket matching invites. `I<O<I<O<H>>>>` is 21.
pub type Dropped = Pz<I<O<I<O<H>>>>>;
const _: () = assert!(<Dropped as Nat>::VAL == 21);

/// Both reach any `Nat`-bounded position, fn-forced so well-formedness is
/// actually checked (the lesson `46_probes/probe_3d` recorded the hard way:
/// a bare type alias defers its bounds and tests nothing).
pub struct Accumulator<P: Nat>(core::marker::PhantomData<P>);
pub fn declare_intended(_: Accumulator<Intended>) {}
pub fn declare_typo(_: Accumulator<Typo>) {}
pub fn declare_dropped(_: Accumulator<Dropped>) {}
