//! Probe 5 (the compiling half): `mul_full`'s associativity is a TYPE identity,
//! not only a value identity, and it holds only for a value-unique numeral
//! encoding. The normalised encoding unifies; the unnormalised one refuses,
//! and the refusal is committed as `probe_5b_unreduced_refuses.rs` with its
//! error recorded in OUTCOMES.md.
//!
//! File 33's probe 3 verified `mulnum(mulnum(N1,N2),N3) = mulnum(N1,mulnum(N2,N3))`
//! as an arithmetic fact over integers (`33:299-308`). But `mul_full` is a
//! family of maps `N1 x N2 -> mulnum(N1, N2)` where the numerals are TYPES
//! (`33:681-690`), so "the same numeral" in a generic signature means "the
//! same type", judged by rustc's type equality, not by mathematical equality
//! of what the types denote. Two consequences, one per half of this probe:
//!
//! CLAIM A (this file). With adjustments carried as reduced rational const
//! parameters (`adt_const_params`, reduction applied at construction), the
//! two bracketings of a triple product produce the SAME type, and a function
//! demanding type equality accepts them. Type-level associativity holds
//! because the encoding is value-unique: one type per numeral.
//!
//! CLAIM B (`probe_5b`). With the same rational adjustment carried UNREDUCED
//! (file 28's rational-pair proposal as literally stated, which file 31
//! carries into the settled contract at `31:204-205` with no normalisation
//! requirement anywhere in files 28, 30, 31 or 33), "the same numeral" is
//! spelling-dependent: the product of two reduced numerals (2/3 times 3/4)
//! is spelled 6/12 and does not inhabit the type of the numeral 1/2 a
//! consumer writes directly, refused with E0308. Componentwise pair
//! multiplication is itself associative, so pure product chains still unify
//! with each other; the crack is between a product's spelling and the
//! numeral it denotes, which is exactly where a generic consumer of
//! `mul_full`'s result stands. Value-level facts (file 33's probe 3) are
//! intact; the type-level statement of any law about "the" product numeral
//! is not well formed until the encoding is value-unique.
//!
//! The general form of the obligation, stated for the spec: **the numeral
//! encoding must be value-unique (a normal form per denoted numeral), or
//! every law about a numeral-producing operation splits into a value half
//! (true) and a type half (false).** For the width chain the shipped
//! typenum-style binary encoding already has this property (no leading
//! zeros = one type per width). For the rational adjustment the settled gcd
//! formula makes normalisation REACHABLE (gcd is exactly the reduction), but
//! nothing in the settled contract STATES it, and the type-level gcd this
//! requires (typenum ships one, `typenum::type_operators::Gcd`, so
//! feasibility has prior art on stable Rust) is unbuilt and unpriced in this
//! design. Recorded as open.
//!
//! Why const generics cannot dodge this: computing the product adjustment in
//! type position from GENERIC const parameters is the exact
//! `generic parameters may not be used in const operations` wall the
//! droplist already records (`26:719-724`), so the computation must be
//! trait-level, where value-uniqueness of the encoding is a design
//! obligation, not a given.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_5_unreduced_adjustment_breaks_type_level_associativity.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]
#![feature(adt_const_params)]

use core::marker::ConstParamTy;
use core::marker::PhantomData;

/// A rational adjustment carried as a const parameter.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct Adj {
    pub num: u64,
    pub den: u64,
}

/// A numeral keyed on its adjustment.
pub struct Numeral<const A: Adj>;

const fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// The normal form: reduce the fraction. This is the value-unique encoding.
const fn reduce(a: Adj) -> Adj {
    let g = gcd(a.num, a.den);
    Adj {
        num: a.num / g,
        den: a.den / g,
    }
}

/// The product adjustment of two exact (bias-zero) numerals: multiply and
/// reduce. At concrete arguments this is const-evaluable; at GENERIC
/// arguments it cannot appear in type position (the droplist wall), which is
/// why the real design computes it at the trait level and why the encoding's
/// normal form is load-bearing there.
const fn mul_adj(a: Adj, b: Adj) -> Adj {
    reduce(Adj {
        num: a.num * b.num,
        den: a.den * b.den,
    })
}

/// Type equality, demanded rather than assumed.
pub fn same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

// Three operand numerals: 3/4, 2/3, 5/6.
const A1: Adj = Adj { num: 3, den: 4 };
const A2: Adj = Adj { num: 2, den: 3 };
const A3: Adj = Adj { num: 5, den: 6 };

// The two bracketings, each reduced at every step.
const LEFT: Adj = mul_adj(mul_adj(A1, A2), A3);
const RIGHT: Adj = mul_adj(A1, mul_adj(A2, A3));

// CLAIM A: the two bracketings are the same type. This function type-checks,
// which is the claim; there is nothing to run.
pub fn bracketings_unify() {
    same_type(PhantomData::<Numeral<LEFT>>, PhantomData::<Numeral<RIGHT>>);
}

// The value-level fact behind it, checked so the type identity is not
// mistaken for a coincidence of these operands: (3/4)(2/3)(5/6) = 5/12.
const _: () = assert!(LEFT.num == 5 && LEFT.den == 12);
const _: () = assert!(RIGHT.num == 5 && RIGHT.den == 12);

// And the two value-equal spellings that CLAIM B (probe_5b) uses (the
// product spelling 6/12 against the directly-written 1/2), shown here to be
// value-equal under reduction, so the 5b refusal is unambiguously a
// statement about the encoding, not about the values:
const HALF_DIRECT: Adj = Adj { num: 1, den: 2 };
const HALF_AS_PRODUCT_SPELLING: Adj = Adj { num: 6, den: 12 };
const _: () = assert!({
    let r = reduce(HALF_AS_PRODUCT_SPELLING);
    r.num == HALF_DIRECT.num && r.den == HALF_DIRECT.den
});
