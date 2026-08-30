//! P5. Can a primitive carry its range, so that the completion is ELIMINATED
//! rather than chosen?
//!
//! The motivation is I15: invalids are caught at compile time and never at
//! runtime. A completion policy (wrap, saturate, panic) is what a primitive
//! does when the exact answer leaves its value set. If the type knows the
//! operands' ranges, and the sum of those ranges fits, then the exact answer
//! CANNOT leave the value set, the completion never fires, and the lowering
//! is the bare machine instruction with no clamp, no branch, no mask.
//!
//! That is the microkernelling shape: the typestate knows something the
//! backend never learns, and supplying it removes an instruction the compiler
//! could not prove dead.
//!
//! The obstacle is well known and is why this is worth checking rather than
//! assuming. Writing the result range as `Ranged<{LO_A + LO_B}, {HI_A +
//! HI_B}>` puts arithmetic in type position, which needs
//! `generic_const_exprs`, which is forbidden. The workspace's standing
//! reflex says a refused bound is usually a trait nobody has named yet.
//!
//! Five questions:
//!
//!   Q1. Does range propagation through a trait's associated consts compile
//!       with no forbidden feature?
//!   Q2. Does it compose to depth, so a chain of operations carries a range
//!       rather than only a single operation?
//!   Q3. Is the derived range CORRECT, checked against an exhaustive census
//!       rather than asserted?
//!   Q4. Does a range that overflows its container get REFUSED at compile
//!       time, per I15, rather than at runtime?
//!   Q5. Does the emitted code actually lose the completion, or does the
//!       proof evaporate at lowering and leave the branch in?
//!
//! Feature gates: none. This file compiles on the pinned nightly with no
//! `#![feature(...)]` line at all, which is the point of Q1.
//!
//! Build: rustc --test -O p5_a_carried_range_eliminates_the_completion.rs

#![allow(dead_code)]

use core::marker::PhantomData;

/// The container: eight bits, unsigned. Chosen small so the census in Q3 is
/// exhaustive rather than sampled.
type Store = u8;
const STORE_MAX: i32 = 255;

// ---------------------------------------------------------------------------
// Q1. Range as a trait, not as an expression in type position.
// ---------------------------------------------------------------------------

/// A carried range. The arithmetic lives in an associated const's BODY,
/// where arbitrary const expressions are legal, rather than in type position,
/// where they are not.
trait Range {
    const LO: i32;
    const HI: i32;
}

/// A literal range, the leaf of the type-level tree.
struct Lit<const L: i32, const H: i32>;
impl<const L: i32, const H: i32> Range for Lit<L, H> {
    const LO: i32 = L;
    const HI: i32 = H;
}

/// The range of a sum. This is the whole trick: `A::LO + B::LO` is a const
/// expression in a const item, not a const expression in a type.
struct RSum<A, B>(PhantomData<(A, B)>);
impl<A: Range, B: Range> Range for RSum<A, B> {
    const LO: i32 = A::LO + B::LO;
    const HI: i32 = A::HI + B::HI;
}

/// The range of a product.
struct RMul<A, B>(PhantomData<(A, B)>);
impl<A: Range, B: Range> Range for RMul<A, B> {
    // Non-negative operands only in this probe, so the extremes are the
    // corner products. A general version takes the min and max over all four.
    const LO: i32 = A::LO * B::LO;
    const HI: i32 = A::HI * B::HI;
}

/// The range of a difference. Present to show the propagation rule is
/// per-operation rather than uniform: the low end of a difference comes from
/// the low end of one operand and the HIGH end of the other.
struct RDiff<A, B>(PhantomData<(A, B)>);
impl<A: Range, B: Range> Range for RDiff<A, B> {
    const LO: i32 = A::LO - B::HI;
    const HI: i32 = A::HI - B::LO;
}

/// A value carrying a compile-time range and nothing at runtime.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
struct Ranged<R: Range>(Store, PhantomData<R>);

impl<R: Range> Ranged<R> {
    /// The proof obligation, discharged once at construction. Everything
    /// downstream inherits it.
    fn new(v: Store) -> Self {
        debug_assert!(
            (v as i32) >= R::LO && (v as i32) <= R::HI,
            "construction is the one place a range is established"
        );
        Ranged(v, PhantomData)
    }
    fn raw(self) -> Store {
        self.0
    }

    /// Q4's gate. A post-monomorphisation const assertion: it fires only for
    /// instantiations that actually occur, and it fires at compile time.
    const FITS: () = assert!(
        R::LO >= 0 && R::HI <= STORE_MAX,
        "the carried range does not fit the container"
    );
}

// ---------------------------------------------------------------------------
// The operations. Note what is NOT here: no clamp, no wrap, no branch, no
// checked_add. The completion is absent because the type proves it cannot be
// needed.
// ---------------------------------------------------------------------------

fn add<A: Range, B: Range>(a: Ranged<A>, b: Ranged<B>) -> Ranged<RSum<A, B>>
where
    RSum<A, B>: Range,
{
    let () = Ranged::<RSum<A, B>>::FITS;
    Ranged(a.0 + b.0, PhantomData)
}

fn mul<A: Range, B: Range>(a: Ranged<A>, b: Ranged<B>) -> Ranged<RMul<A, B>>
where
    RMul<A, B>: Range,
{
    let () = Ranged::<RMul<A, B>>::FITS;
    Ranged(a.0 * b.0, PhantomData)
}

// ---------------------------------------------------------------------------
// Q5. Two lowerings of the same computation, for reading the emitted code.
// The proved one carries the range; the unproved one has to complete.
// ---------------------------------------------------------------------------

type Small = Lit<0, 100>;
type Tiny = Lit<0, 50>;

/// Proved: 0..=100 plus 0..=50 is 0..=150, which fits eight bits.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn proved_add(a: Store, b: Store) -> Store {
    let x: Ranged<Small> = Ranged(a, PhantomData);
    let y: Ranged<Tiny> = Ranged(b, PhantomData);
    add(x, y).raw()
}

/// Unproved, saturating: the same addition with no carried range, so the
/// completion has to be emitted.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn unproved_add_saturating(a: Store, b: Store) -> Store {
    a.saturating_add(b)
}

/// Unproved, checked: the same addition with a runtime validity check, which
/// is the shape I15 forbids.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn unproved_add_checked(a: Store, b: Store) -> Store {
    match a.checked_add(b) {
        Some(v) => v,
        None => 255,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Q1. Range propagation compiles, with no feature gate anywhere in this
    /// file. The absence of `#![feature(generic_const_exprs)]` is the result;
    /// this test reads the propagated values back.
    #[test]
    fn range_propagates_through_associated_consts_with_no_feature_gate() {
        assert_eq!(<RSum<Small, Tiny> as Range>::LO, 0);
        assert_eq!(<RSum<Small, Tiny> as Range>::HI, 150);
        assert_eq!(<RMul<Lit<2, 3>, Lit<4, 5>> as Range>::LO, 8);
        assert_eq!(<RMul<Lit<2, 3>, Lit<4, 5>> as Range>::HI, 15);
        // Per-operation propagation: a difference crosses the endpoints.
        assert_eq!(<RDiff<Lit<10, 20>, Lit<3, 7>> as Range>::LO, 3);
        assert_eq!(<RDiff<Lit<10, 20>, Lit<3, 7>> as Range>::HI, 17);
    }

    /// Q2. It composes to depth. If it did not, the mechanism would serve one
    /// operation and nothing built on it.
    #[test]
    fn range_composes_to_depth() {
        type D2 = RSum<Small, Tiny>; // 0..=150
        type D3 = RSum<D2, Lit<0, 10>>; // 0..=160
        type D4 = RSum<D3, Lit<5, 20>>; // 5..=180
        type D5 = RMul<Lit<1, 2>, D4>; // 5..=360
        type D6 = RDiff<D5, Lit<0, 100>>; // -95..=360
        assert_eq!(<D3 as Range>::HI, 160);
        assert_eq!(<D4 as Range>::LO, 5);
        assert_eq!(<D4 as Range>::HI, 180);
        assert_eq!(<D5 as Range>::HI, 360);
        assert_eq!(<D6 as Range>::LO, -95);
        assert_eq!(<D6 as Range>::HI, 360);
    }

    /// Q3. The derived range is CORRECT, not merely computed. Exhaustive over
    /// every pair in both operand ranges: the true extremes of the sum are
    /// exactly the derived LO and HI, and every result lands inside.
    ///
    /// This is the check that separates a real propagation rule from a
    /// declaration nothing constrains. A wrong `RSum::HI` would pass every
    /// test above and fail here.
    #[test]
    fn the_derived_range_is_tight_and_sound_by_census() {
        let lo_a = <Small as Range>::LO;
        let hi_a = <Small as Range>::HI;
        let lo_b = <Tiny as Range>::LO;
        let hi_b = <Tiny as Range>::HI;
        let derived_lo = <RSum<Small, Tiny> as Range>::LO;
        let derived_hi = <RSum<Small, Tiny> as Range>::HI;

        let mut seen_lo = i32::MAX;
        let mut seen_hi = i32::MIN;
        let mut checked = 0u32;
        let mut outside = 0u32;
        for a in lo_a..=hi_a {
            for b in lo_b..=hi_b {
                let s = a + b;
                if s < seen_lo {
                    seen_lo = s;
                }
                if s > seen_hi {
                    seen_hi = s;
                }
                if s < derived_lo || s > derived_hi {
                    outside += 1;
                }
                checked += 1;
            }
        }
        assert_eq!(checked, ((hi_a - lo_a + 1) * (hi_b - lo_b + 1)) as u32);
        assert_eq!(outside, 0, "SOUND: no result may leave the derived range");
        assert_eq!(seen_lo, derived_lo, "TIGHT: the low end must be attained");
        assert_eq!(seen_hi, derived_hi, "TIGHT: the high end must be attained");

        // And the same for the product rule, over a range where the corner
        // rule is the whole story.
        type PA = Lit<3, 9>;
        type PB = Lit<2, 7>;
        let mut plo = i32::MAX;
        let mut phi = i32::MIN;
        for a in 3..=9 {
            for b in 2..=7 {
                let p = a * b;
                if p < plo {
                    plo = p;
                }
                if p > phi {
                    phi = p;
                }
            }
        }
        assert_eq!(plo, <RMul<PA, PB> as Range>::LO);
        assert_eq!(phi, <RMul<PA, PB> as Range>::HI);
    }

    /// Q5, the runtime half. The proved path computes the same answers as an
    /// ordinary addition over the whole region where the range holds. If it
    /// did not, the elimination would be a bug rather than an optimisation.
    /// The assembly half of Q5 is in `p5_asm.txt`.
    #[test]
    fn the_proved_path_agrees_with_ordinary_addition_on_its_whole_region() {
        let mut checked = 0u32;
        for a in 0u16..=100 {
            for b in 0u16..=50 {
                let got = proved_add(a as u8, b as u8);
                assert_eq!(got as u16, a + b, "proved path wrong at {a}+{b}");
                // And on this region the two unproved paths agree too, which
                // is what makes them a fair comparison rather than a
                // strawman: they differ only outside the proved region.
                assert_eq!(unproved_add_saturating(a as u8, b as u8) as u16, a + b);
                assert_eq!(unproved_add_checked(a as u8, b as u8) as u16, a + b);
                checked += 1;
            }
        }
        assert_eq!(checked, 101 * 51);
    }

    /// And outside the proved region the three disagree, which is exactly why
    /// the completion exists at all and why removing it needs a proof rather
    /// than a preference.
    #[test]
    fn outside_the_proved_region_the_completions_disagree() {
        assert_eq!(unproved_add_saturating(200, 200), 255);
        assert_eq!(unproved_add_checked(200, 200), 255);
        // The proved path is not defined there: constructing a
        // `Ranged<Lit<0,100>>` from 200 violates the obligation `new` states.
        // That is the point. It is not a value the type admits.
    }
}
