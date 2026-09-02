// Probe: is the "typestate derives the matching container and numeral
// representations, then validates, and erases" pipeline expressible as one
// generic trait mechanism, without any forbidden feature
// (generic_const_exprs, generic_const_args, full specialization,
// -Znext-solver=globally), and without dyn or TypeId (arvo's own no-dyn,
// monomorphisation-is-the-dispatch rule)?
//
// This does not attempt to be arvo's real Strategy/Bits/UFixed machinery.
// It is the smallest sketch that exercises the SHAPE, STRATEGY, DERIVE,
// VALIDATE, ERASE structure this file's cold derivation argues for, and
// checks it compiles under the pinned toolchain with none of the forbidden
// gates. It also exercises the "representations" plural: the same shape
// under the same strategy legitimately routes to two different concrete
// bit-container encodings for two different derive sites, which is exactly
// the point the associated-type-per-impl shape below is built to allow.
//
// Compiled directly with the pinned nightly, no cargo, no forbidden feature
// gates declared or needed:
//   rustc --edition 2024 --crate-type lib derive_validate_erase_pipeline.rs

#![allow(dead_code)]

/// A strategy: the axis that picks which measurement a derivation weighs.
/// Standing in for arvo's Hot/Warm/Cold/Precise, kept open per this file's
/// own conclusion that the concrete strategy set is not the canon's concern.
trait Strategy {
    const NAME: &'static str;
}

struct Hot;
impl Strategy for Hot {
    const NAME: &'static str = "hot";
}

struct Cold;
impl Strategy for Cold {
    const NAME: &'static str = "cold";
}

/// A shape: the type-level name for an abstract number system's value set
/// plus operation-law family, independent of any concrete encoding.
/// Standing in for arvo's UFixed<I, F, S> / a floating shape / a bit-domain
/// shape; this probe uses one toy shape, "Q3.5", an unsigned dyadic
/// rational with 3 integer bits and 5 fractional bits.
struct Q3_5;

/// Derive: given (Shape, Strategy), produce the container type and the
/// concrete numeral representation, with a validity predicate decidable
/// from the container's bits alone (no external state), and an erase step
/// that discards the typestate while leaving the bits self-sufficient.
trait Derive<S: Strategy> {
    type Container: Copy + core::fmt::Debug;

    /// Decidable purely from the bits plus the static type parameters.
    /// This is the admissibility criterion the cold derivation argues
    /// erasure requires: nothing outside (Self, S, raw) may be consulted.
    fn validate(raw: Self::Container) -> bool;

    /// Discards the typestate. Default is the identity because a
    /// repr-transparent erase changes no bits, only what the type system
    /// remembers about them.
    fn erase(raw: Self::Container) -> Self::Container {
        raw
    }
}

/// Hot: full u8 range is valid (wrapping semantics are the strategy's
/// business elsewhere; this probe only exercises the derive/validate/erase
/// shape, not overflow policy itself).
impl Derive<Hot> for Q3_5 {
    type Container = u8;

    fn validate(_raw: u8) -> bool {
        true
    }
}

/// Cold: same abstract shape, same strategy contract, but this derive site
/// chooses a DIFFERENT concrete numeral representation: a bit-reversed
/// packing (standing in for, say, a denser cold-storage layout that is not
/// bit-identical to Hot's natural layout even though it represents the same
/// value set). This is the "representations" plural made concrete: one
/// shape, one derive/validate/erase contract, two different container
/// encodings admissible at two different call sites.
impl Derive<Cold> for Q3_5 {
    type Container = u8;

    fn validate(raw: u8) -> bool {
        // reject the top bit set as a stand-in "reserved pattern" so the
        // two derive sites are visibly not the same representation, not
        // merely relabelled
        raw & 0b1000_0000 == 0
    }

    fn erase(raw: u8) -> u8 {
        raw.reverse_bits()
    }
}

fn pipeline<S: Strategy, T: Derive<S>>(raw: T::Container) -> Option<T::Container> {
    if T::validate(raw) {
        Some(T::erase(raw))
    } else {
        None
    }
}

pub fn run_probe() -> (Option<u8>, Option<u8>, Option<u8>) {
    let hot_ok = pipeline::<Hot, Q3_5>(200);
    let cold_ok = pipeline::<Cold, Q3_5>(0b0111_1111);
    let cold_reject = pipeline::<Cold, Q3_5>(0b1000_0001);
    (hot_ok, cold_ok, cold_reject)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hot_accepts_full_range_and_erases_identically() {
        assert_eq!(pipeline::<Hot, Q3_5>(200), Some(200));
        assert_eq!(pipeline::<Hot, Q3_5>(0), Some(0));
        assert_eq!(pipeline::<Hot, Q3_5>(255), Some(255));
    }

    #[test]
    fn cold_rejects_the_reserved_top_bit() {
        assert_eq!(pipeline::<Cold, Q3_5>(0b1000_0001), None);
    }

    #[test]
    fn cold_accepts_and_erases_to_a_different_representation_than_hot() {
        let raw = 0b0111_1111u8;
        let erased = pipeline::<Cold, Q3_5>(raw).unwrap();
        assert_eq!(erased, raw.reverse_bits());
        assert_ne!(
            erased, raw,
            "cold's numeral representation for this shape is not bit-identical to hot's, \
             on purpose: same shape, same strategy contract, two admissible representations"
        );
    }

    #[test]
    fn one_shape_two_strategies_two_container_encodings_both_type_check() {
        let (hot, cold, cold_rej) = run_probe();
        assert_eq!(hot, Some(200));
        assert_eq!(cold, Some(0b1111_1110));
        assert_eq!(cold_rej, None);
    }
}
