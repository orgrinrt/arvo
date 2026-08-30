//! Probe 1: does the type-checker's diagnostic name what the consumer wrote,
//! or what it expands to, and does that depend on whether the notation
//! layer is a type alias or a genuine newtype.
//!
//! File 47 built the decoder-ring finding (probe_1c: an E0308 printing both
//! numerals as full positional nests). The consolidation (49:600-603)
//! generalises it: "rustc expands type aliases in diagnostics, so the
//! intended decimal value never surfaces in an E0308 regardless of the
//! notation layer." That sentence has stood since file 47 on the strength
//! of one probe using type ALIASES specifically. Nobody has compiled the
//! newtype alternative to see whether the same claim holds there. This
//! probe does both, side by side, on the identical mismatch.
//!
//! EXPECTED, alias form: the full nest, reproducing 47's finding.
//! EXPECTED, newtype form: unknown going in. A newtype is not transparent
//! to the type checker (two distinct types even with identical layout), so
//! there is a real chance the diagnostic names the newtype's own path
//! rather than expanding its field.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern
//!   tower_nat=libtower_nat.rlib probe_1_alias_expands_newtype_survives.rs

#![allow(dead_code)]

use core::marker::PhantomData;
use tower_nat::{Nat, Pz, H, I, O};

pub struct Container<P: Nat>(PhantomData<P>);

// ---- Form A: type alias (what file 47's alias-table proposal emits) ----

pub type Face37 = Pz<I<O<I<O<O<H>>>>>>; // 37
pub type Face53 = Pz<I<O<I<O<I<H>>>>>>; // 53

pub fn needs_face37(_: Container<Face37>) {}
pub fn give_face53_alias(c: Container<Face53>) {
    needs_face37(c);
}

// ---- Form B: newtype (a genuine distinct type wrapping the encoding) ----

pub struct NFace<const V: u64>(PhantomData<()>);

pub trait NumeralFace {
    type Encoding: Nat;
    const V: u64;
}
impl NumeralFace for NFace<37> {
    type Encoding = Face37;
    const V: u64 = 37;
}
impl NumeralFace for NFace<53> {
    type Encoding = Face53;
    const V: u64 = 53;
}

pub struct FaceContainer<F: NumeralFace>(PhantomData<F>);

pub fn needs_nface37(_: FaceContainer<NFace<37>>) {}
pub fn give_nface53(c: FaceContainer<NFace<53>>) {
    needs_nface37(c);
}
