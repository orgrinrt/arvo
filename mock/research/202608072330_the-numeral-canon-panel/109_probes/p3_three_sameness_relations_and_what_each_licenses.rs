//! P3. What makes two primitives the same, and is the strategy part of the
//! identity or a selector over it?
//!
//! The question "are these two primitives the same" has three different
//! answers, and this probe's claim is that all three are needed because each
//! licenses a different operation:
//!
//!   NOMINAL: the same name applied to the same arguments. Licenses
//!            assignment without a cast.
//!   REPRESENTATIONAL: the same value set AND the same bits. Licenses
//!            reinterpreting memory: an array of one read as an array of the
//!            other.
//!   DENOTATIONAL: the same value set AND the same answers, whatever the
//!            bits. Licenses a rewrite: a law proved for one holds for the
//!            other.
//!
//! If the three coincided, one relation would do. This probe checks that the
//! implications are STRICT in both directions where they should be, by
//! exhibiting witnesses:
//!
//!   W1. Denotationally same, representationally different.
//!   W2. Representationally same, nominally different.
//!   W3. Nominally different, denotationally different: the ordinary case.
//!
//! And the strategy question. If two strategy markers resolve to the same
//! value set, realisation and completion, are the resulting types the same?
//! Under a design that carries the MARKER in the type, no: they need a cast
//! for a difference that does not exist. Under a design that carries the
//! RESOLUTION, yes: they unify, and the request is no longer recoverable.
//! Both are demonstrated. Neither is proposed.
//!
//! No feature gates. `std` is used only by the test harness.
//!
//! Build: rustc --edition 2021 --test -O p3_three_sameness_relations_and_what_each_licenses.rs

#![allow(dead_code)]

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// The three components, as types, so that a resolution is a type and can be
// compared by the compiler.
// ---------------------------------------------------------------------------

/// Component one: the value set. `I` integer bits, `F` fraction bits.
trait ValueSet {
    const I: u32;
    const F: u32;
    /// The denotation of a stored integer k, as the rational k / DEN.
    const DEN: u64;
}

/// Component two: the realisation. How wide the resting container is, and
/// whether an element is independently addressable.
trait Realisation {
    const CONTAINER_BITS: u32;
    const INDEPENDENTLY_ADDRESSABLE: bool;
}

/// Component three: the completion.
trait Completion {
    const KIND: u8; // 0 wrap, 1 saturate
    fn complete(exact: i64, lo: i64, hi: i64) -> i64;
}

struct V8_0;
impl ValueSet for V8_0 {
    const I: u32 = 8;
    const F: u32 = 0;
    const DEN: u64 = 1;
}

struct V4_4;
impl ValueSet for V4_4 {
    const I: u32 = 4;
    const F: u32 = 4;
    const DEN: u64 = 16;
}

/// Rests in its own byte.
struct ByteRest;
impl Realisation for ByteRest {
    const CONTAINER_BITS: u32 = 8;
    const INDEPENDENTLY_ADDRESSABLE: bool = true;
}

/// Rests in a shared word, eight to a u64. Same value set, different bits at
/// rest, and not independently addressable.
struct PackedRest;
impl Realisation for PackedRest {
    const CONTAINER_BITS: u32 = 8;
    const INDEPENDENTLY_ADDRESSABLE: bool = false;
}

struct Wrap;
impl Completion for Wrap {
    const KIND: u8 = 0;
    fn complete(exact: i64, lo: i64, hi: i64) -> i64 {
        let m = hi - lo + 1;
        let mut e = (exact - lo) % m;
        if e < 0 {
            e += m;
        }
        e + lo
    }
}

struct Sat;
impl Completion for Sat {
    const KIND: u8 = 1;
    fn complete(exact: i64, lo: i64, hi: i64) -> i64 {
        if exact > hi {
            hi
        } else if exact < lo {
            lo
        } else {
            exact
        }
    }
}

/// A RESOLUTION: the triple. This is a type, so two resolutions are the same
/// type exactly when their three components are the same types.
struct Resolved<V, R, C>(PhantomData<(V, R, C)>);

/// A primitive as the panel's working assumption has it: the composition
/// carries a strategy marker alongside the resolved components.
/// `derive` is deliberately NOT used here. A derived `PartialEq` on a
/// generic struct places a bound on every parameter, including the phantom
/// markers, which then demand impls they have no reason to carry. That is an
/// artifact of the derive rather than of the design, and it is recorded
/// because a real primitive parameterised this way has to hand-write these
/// impls for exactly this reason.
#[repr(transparent)]
#[derive(Copy, Clone)]
struct WithMarker<V, R, C, S>(u8, PhantomData<(V, R, C, S)>);

/// A primitive keyed only on the resolution, with the request discarded.
#[repr(transparent)]
#[derive(Copy, Clone)]
struct WithoutMarker<V, R, C>(u8, PhantomData<(V, R, C)>);

// The strategy markers. Two of them, deliberately chosen to RESOLVE
// IDENTICALLY at this width, which is the interesting case.
struct Speed;
struct Space;

/// A strategy is a selector: given a value set, it picks a realisation and a
/// completion. It is not itself one of them.
trait Strategy {
    type PickRealisation: Realisation;
    type PickCompletion: Completion;
}

impl Strategy for Speed {
    type PickRealisation = ByteRest;
    type PickCompletion = Wrap;
}

/// At this width the storage-minimising selector has nothing to gain: eight
/// bits is already the container, so it picks the same realisation. The two
/// strategies COINCIDE here and diverge at other widths, which is the whole
/// point of calling them selectors.
impl Strategy for Space {
    type PickRealisation = ByteRest;
    type PickCompletion = Wrap;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Type equality, decided by the compiler rather than by a runtime
    /// comparison. `same_type::<A, B>()` compiles only when A and B are the
    /// same type, so its ABSENCE at a call site is as informative as its
    /// presence.
    fn same_type<T>(_a: &T, _b: &T) {}

    /// W1. Denotationally the same, representationally different. Same value
    /// set, same completion, different resting bits. Every answer agrees;
    /// the memory does not.
    #[test]
    fn w1_denotationally_same_representationally_different() {
        // The realisations differ in a property nothing about the values can
        // see.
        assert_eq!(
            <ByteRest as Realisation>::CONTAINER_BITS,
            <PackedRest as Realisation>::CONTAINER_BITS
        );
        assert_ne!(
            <ByteRest as Realisation>::INDEPENDENTLY_ADDRESSABLE,
            <PackedRest as Realisation>::INDEPENDENTLY_ADDRESSABLE
        );

        // And the answers agree, exhaustively, when actually routed THROUGH
        // the two realisations rather than compared to themselves. The first
        // version of this block asserted `complete(..) == complete(..)` with
        // both sides the same call, which is a tautology and is not a test.
        // Recorded because it is the failure this workspace's test gate names
        // first, and it was written here by reflex.
        let mut checked = 0u32;
        for a in 0i64..=255 {
            for b in 0i64..=255 {
                let exact = a + b;

                // Route A: rest in an own byte.
                let byte_rest: u8 = <Wrap as Completion>::complete(exact, 0, 255) as u8;
                let via_byte = byte_rest as i64;

                // Route B: rest packed at offset 24 inside a shared u64 whose
                // other lanes are non-zero, so a bleed would be caught.
                let mut word: u64 = 0xDEAD_0000_00BE_EFu64;
                let v = <Wrap as Completion>::complete(exact, 0, 255) as u64;
                word = (word & !(0xFFu64 << 24)) | (v << 24);
                let via_packed = ((word >> 24) & 0xFF) as i64;

                assert_eq!(
                    via_byte, via_packed,
                    "the two realisations must agree at {a}+{b}"
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 65_536);

        // So a rewrite proved for one is valid for the other, and a memory
        // reinterpretation is not. Two different licences, one pair of types.
    }

    /// W2. Representationally the same, nominally different. Two strategy
    /// markers that resolve to the identical triple still produce different
    /// types when the marker is carried, so an assignment needs a cast for a
    /// difference that does not exist.
    #[test]
    fn w2_representationally_same_nominally_different() {
        type Sp = WithMarker<V8_0, ByteRest, Wrap, Speed>;
        type Sc = WithMarker<V8_0, ByteRest, Wrap, Space>;

        let a: Sp = WithMarker(7, PhantomData);
        let b: Sc = WithMarker(7, PhantomData);

        // The resolutions are identical, component for component.
        assert_eq!(
            <<Speed as Strategy>::PickRealisation as Realisation>::CONTAINER_BITS,
            <<Space as Strategy>::PickRealisation as Realisation>::CONTAINER_BITS
        );
        assert_eq!(
            <<Speed as Strategy>::PickCompletion as Completion>::KIND,
            <<Space as Strategy>::PickCompletion as Completion>::KIND
        );

        // The bits are identical, and both are repr(transparent) over u8, so
        // they have the same layout.
        assert_eq!(core::mem::size_of::<Sp>(), core::mem::size_of::<Sc>());
        assert_eq!(core::mem::align_of::<Sp>(), core::mem::align_of::<Sc>());
        assert_eq!(a.0, b.0);
        assert_eq!(core::mem::size_of::<Sp>(), 1, "repr(transparent) over u8");

        // And they are NOT the same type. The line below is what a consumer
        // would have to write, and it does not compile:
        //
        //     let _c: Sp = b;
        //
        // The refusal is recorded as a compile-fail sibling rather than as a
        // comment: see `p3b_marker_makes_identical_resolutions_incompatible.rs`.
    }

    /// W2, second half. With the request discarded and only the resolution
    /// keyed, the two unify: they are literally one type, so `same_type`
    /// accepts them.
    #[test]
    fn w2_without_the_marker_the_two_requests_unify() {
        let a: WithoutMarker<
            V8_0,
            <Speed as Strategy>::PickRealisation,
            <Speed as Strategy>::PickCompletion,
        > = WithoutMarker(7, PhantomData);
        let b: WithoutMarker<
            V8_0,
            <Space as Strategy>::PickRealisation,
            <Space as Strategy>::PickCompletion,
        > = WithoutMarker(7, PhantomData);

        // This call is the assertion. It compiles only because the two are
        // the same type, which is precisely what carrying the marker
        // prevents.
        same_type(&a, &b);
        assert_eq!(a.0, b.0);

        // What is lost: nothing in `a`'s type records that a consumer asked
        // for speed rather than space. If the storage-minimising selector
        // later picks a packed realisation at this width, `a`'s source does
        // not change and `a`'s meaning does. That is the cost of discarding
        // the request, and it is a real cost rather than an argument against.
    }

    /// W3. The ordinary case, kept so the lattice is not established only on
    /// its edges: two primitives differing in the value set differ under all
    /// three relations, and the answers differ too.
    #[test]
    fn w3_different_value_sets_differ_under_every_relation() {
        assert_ne!(<V8_0 as ValueSet>::DEN, <V4_4 as ValueSet>::DEN);
        let mut disagreements = 0u32;
        for k in 0u64..=255 {
            // The same bit pattern k under two value sets.
            let as_i8f0 = (k as f64) / (<V8_0 as ValueSet>::DEN as f64);
            let as_i4f4 = (k as f64) / (<V4_4 as ValueSet>::DEN as f64);
            if as_i8f0 != as_i4f4 {
                disagreements += 1;
            }
        }
        assert_eq!(disagreements, 255, "every non-zero pattern denotes twice");
    }

    /// The completion is a genuine third axis: holding the value set and the
    /// realisation fixed and moving only the completion changes answers.
    /// Without this the three-way decomposition would collapse to two.
    #[test]
    fn the_completion_axis_is_not_redundant() {
        let mut differ = 0u32;
        let mut checked = 0u32;
        for a in 0i64..=255 {
            for b in 0i64..=255 {
                let w = <Wrap as Completion>::complete(a + b, 0, 255);
                let s = <Sat as Completion>::complete(a + b, 0, 255);
                if w != s {
                    differ += 1;
                }
                checked += 1;
            }
        }
        assert_eq!(checked, 65_536);
        assert!(differ > 0);
        println!("wrap and saturate differ on {differ} of {checked} pairs at I=8,F=0");
    }

    /// The strategy is a selector, not a component: it is a function from a
    /// request to a resolution, and two selectors can land on one resolution.
    /// If a strategy were a component of the triple, this could not happen,
    /// because a component is by definition part of what distinguishes.
    #[test]
    fn a_strategy_selects_a_resolution_rather_than_being_one() {
        let r1: Resolved<
            V8_0,
            <Speed as Strategy>::PickRealisation,
            <Speed as Strategy>::PickCompletion,
        > = Resolved(PhantomData);
        let r2: Resolved<
            V8_0,
            <Space as Strategy>::PickRealisation,
            <Space as Strategy>::PickCompletion,
        > = Resolved(PhantomData);
        same_type(&r1, &r2);
    }
}
