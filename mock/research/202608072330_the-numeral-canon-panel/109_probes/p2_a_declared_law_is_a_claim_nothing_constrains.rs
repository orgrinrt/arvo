//! P2. Can a law set be a component of a primitive?
//!
//! The panel's working assumption lists a "law set" as one of the four things
//! a primitive composes. This probe takes that literally: a primitive that
//! DECLARES which laws it satisfies, and asks three questions.
//!
//!   1. Does a false declaration compile? If yes, the declaration is a claim
//!      nothing constrains, sitting exactly where a real check would be
//!      noticed missing.
//!   2. Does a rewrite gated on the declaration change answers? If yes, the
//!      declaration is not decorative, it is load-bearing and wrong.
//!   3. Can the law be COMPUTED instead, at const time, parameterised by the
//!      completion policy? If yes, the law set is a consequence of the other
//!      components rather than a component beside them, and there is no
//!      position in which a wrong answer can be written.
//!
//! The subject: signed saturating addition. Chosen because it is a completion
//! anybody would reach for, it looks associative, and it is not. The value
//! set is four bits, -8..=7, so an exhaustive census over all 4096 triples
//! runs inside const evaluation without touching the const-eval budget.
//!
//! ## The wall this probe hit, and how it was got round
//!
//! Question 3 was first written with the census taking `fn(i32,i32)->i32`.
//! rustc refuses: "function pointer calls are not allowed in constant
//! functions" (four occurrences, recorded in `p2_blocker_fn_ptr.txt`). So a
//! computed law cannot be parameterised by a function value. It CAN be
//! parameterised by a const trait, which is what the census below does, and
//! that is a fact about how such a law has to be built rather than a
//! detail: the policy has to be a TYPE for its laws to be computable about
//! it. See `p0_const_trait_spelling_on_the_pin.rs` for the spelling.
//!
//! Feature gates: `const_trait_impl` only, which is on the workspace's
//! allowed list. No `generic_const_exprs`, no specialization, no `TypeId`.
//!
//! Build: rustc --test -O p2_a_declared_law_is_a_claim_nothing_constrains.rs

#![allow(dead_code)]
#![feature(const_trait_impl)]

// ---------------------------------------------------------------------------
// The value set and three completions over it.
// ---------------------------------------------------------------------------

const W: i32 = 4;
const MIN: i32 = -(1 << (W - 1)); // -8
const MAX: i32 = (1 << (W - 1)) - 1; // 7
const CARD: i32 = MAX - MIN + 1; // 16
const UMAX: i32 = (1 << W) - 1; // 15, the unsigned value set is 0..=15

/// A completion: the total map that makes a partial operation total on the
/// value set. Being a const trait is what makes its laws computable.
const trait Completion {
    fn add(a: i32, b: i32) -> i32;
    /// The value set this completion is supposed to be closed on.
    const LO: i32;
    const HI: i32;
    /// Human-readable, for the census printout only.
    const NAME: &'static str;
}

/// Two-sided saturation: clamps at both ends of the value set.
struct SatBoth;
impl const Completion for SatBoth {
    const NAME: &'static str = "signed saturate-both-ends";
    const LO: i32 = MIN;
    const HI: i32 = MAX;
    fn add(a: i32, b: i32) -> i32 {
        let e = a + b;
        if e > MAX {
            MAX
        } else if e < MIN {
            MIN
        } else {
            e
        }
    }
}

/// One-sided saturation over the SIGNED value set: clamps at the top only.
/// This is the shape a reader reaches for when generalising from unsigned,
/// and it is not a completion at all, as the closure census shows: the sum
/// can leave the value set through the bottom, which nothing clamps.
struct SatTopSigned;
impl const Completion for SatTopSigned {
    const NAME: &'static str = "signed saturate-top-only";
    const LO: i32 = MIN;
    const HI: i32 = MAX;
    fn add(a: i32, b: i32) -> i32 {
        let e = a + b;
        if e > MAX { MAX } else { e }
    }
}

/// One-sided saturation over the UNSIGNED value set 0..=15, where the bottom
/// is unreachable by addition, so a top clamp alone IS a completion.
struct SatTopUnsigned;
impl const Completion for SatTopUnsigned {
    const NAME: &'static str = "unsigned saturate-top-only";
    const LO: i32 = 0;
    const HI: i32 = UMAX;
    fn add(a: i32, b: i32) -> i32 {
        let e = a + b;
        if e > UMAX { UMAX } else { e }
    }
}

/// Two-sided saturation over the same UNSIGNED set, for the controlled
/// comparison: only the clamp count differs between this and the row above.
struct SatBothUnsigned;
impl const Completion for SatBothUnsigned {
    const NAME: &'static str = "unsigned saturate-both-ends";
    const LO: i32 = 0;
    const HI: i32 = UMAX;
    fn add(a: i32, b: i32) -> i32 {
        let e = a + b;
        if e > UMAX {
            UMAX
        } else if e < 0 {
            0
        } else {
            e
        }
    }
}

/// Wrapping: a group operation on the value set.
struct Wrap;
impl const Completion for Wrap {
    const NAME: &'static str = "signed wrap";
    const LO: i32 = MIN;
    const HI: i32 = MAX;
    fn add(a: i32, b: i32) -> i32 {
        let m = 1 << W;
        let mut e = (a + b - MIN) % m;
        if e < 0 {
            e += m;
        }
        e + MIN
    }
}

// ---------------------------------------------------------------------------
// Question 1 and 2. The declaration shape: a law set as a component.
// ---------------------------------------------------------------------------

trait DeclaredLaws {
    /// The primitive states, about itself, that its addition associates.
    /// Nothing reads this except the rewrite below, and nothing checks it.
    const ADD_ASSOCIATIVE: bool;
    fn add(a: i32, b: i32) -> i32;
}

impl DeclaredLaws for SatBoth {
    // A lie. It compiles. That is question 1 answered.
    const ADD_ASSOCIATIVE: bool = true;
    fn add(a: i32, b: i32) -> i32 {
        <SatBoth as Completion>::add(a, b)
    }
}

impl DeclaredLaws for Wrap {
    // True, and equally unchecked. A reader cannot tell this row from the one
    // above by looking, which is the point: two assertions in the same
    // position with the same weight and opposite truth values.
    const ADD_ASSOCIATIVE: bool = true;
    fn add(a: i32, b: i32) -> i32 {
        <Wrap as Completion>::add(a, b)
    }
}

/// A rewrite that reassociates a three-term sum when the type says it may.
/// This is the shape of every optimisation such a declaration would exist to
/// license: a scheduler shortening a dependency chain, a vectoriser building
/// a tree reduction.
fn sum3_rewritten<T: DeclaredLaws>(a: i32, b: i32, c: i32) -> i32 {
    if T::ADD_ASSOCIATIVE {
        T::add(a, T::add(b, c))
    } else {
        T::add(T::add(a, b), c)
    }
}

fn sum3_reference<T: DeclaredLaws>(a: i32, b: i32, c: i32) -> i32 {
    T::add(T::add(a, b), c)
}

// ---------------------------------------------------------------------------
// Question 3. The law as a computed consequence.
//
// A const function that decides associativity by exhausting the value set,
// parameterised by the completion rather than declared by the implementor.
// There is no position in which a wrong answer could be written.
// ---------------------------------------------------------------------------

/// Closure census: does the completion land inside its own value set for
/// every pair? A law question about an operation that escapes its set is
/// meaningless, so this runs first.
const fn closure_census<T: [const] Completion>() -> (u32, u32) {
    let mut escapes = 0u32;
    let mut pairs = 0u32;
    let mut a = T::LO;
    while a <= T::HI {
        let mut b = T::LO;
        while b <= T::HI {
            let r = T::add(a, b);
            if r < T::LO || r > T::HI {
                escapes += 1;
            }
            pairs += 1;
            b += 1;
        }
        a += 1;
    }
    (escapes, pairs)
}

const fn associativity_census<T: [const] Completion>() -> (u32, u32) {
    let mut failures = 0u32;
    let mut triples = 0u32;
    let mut a = T::LO;
    while a <= T::HI {
        let mut b = T::LO;
        while b <= T::HI {
            let mut c = T::LO;
            while c <= T::HI {
                let left = T::add(T::add(a, b), c);
                let right = T::add(a, T::add(b, c));
                if left != right {
                    failures += 1;
                }
                triples += 1;
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    (failures, triples)
}

/// The computed laws, evaluated by the compiler, not asserted by anybody.
const SAT_BOTH_ASSOC: (u32, u32) = associativity_census::<SatBoth>();
const SAT_TOP_S_ASSOC: (u32, u32) = associativity_census::<SatTopSigned>();
const SAT_TOP_U_ASSOC: (u32, u32) = associativity_census::<SatTopUnsigned>();
const SAT_BOTH_U_ASSOC: (u32, u32) = associativity_census::<SatBothUnsigned>();
const WRAP_ASSOC: (u32, u32) = associativity_census::<Wrap>();

const SAT_BOTH_CLOSED: (u32, u32) = closure_census::<SatBoth>();
const SAT_TOP_S_CLOSED: (u32, u32) = closure_census::<SatTopSigned>();
const SAT_TOP_U_CLOSED: (u32, u32) = closure_census::<SatTopUnsigned>();
const SAT_BOTH_U_CLOSED: (u32, u32) = closure_census::<SatBothUnsigned>();
const WRAP_CLOSED: (u32, u32) = closure_census::<Wrap>();

#[cfg(test)]
mod tests {
    use super::*;

    /// Question 1. The false declaration is present in a compiled binary,
    /// and the computed truth disagrees with it.
    #[test]
    fn a_false_law_declaration_compiles_and_the_computed_law_refutes_it() {
        assert!(
            <SatBoth as DeclaredLaws>::ADD_ASSOCIATIVE,
            "the impl declares true; if this ever reads false the probe has \
             been edited and the finding is gone"
        );
        assert!(
            SAT_BOTH_ASSOC.0 > 0,
            "two-sided saturating addition must actually fail associativity, \
             or this probe is measuring nothing"
        );
        println!(
            "declared ADD_ASSOCIATIVE = {}, computed failures = {} of {} triples",
            <SatBoth as DeclaredLaws>::ADD_ASSOCIATIVE,
            SAT_BOTH_ASSOC.0,
            SAT_BOTH_ASSOC.1
        );
    }

    /// Question 2. The declaration is load-bearing and wrong: a rewrite that
    /// trusts it changes answers. Census over the whole value set, with a
    /// witness. The same rewrite on a type whose declaration happens to be
    /// true changes nothing, which shows the rewrite is not the defect.
    #[test]
    fn the_rewrite_licensed_by_the_declaration_changes_answers() {
        let mut wrong = 0u32;
        let mut checked = 0u32;
        let mut witness: Option<(i32, i32, i32, i32, i32)> = None;
        for a in MIN..=MAX {
            for b in MIN..=MAX {
                for c in MIN..=MAX {
                    let r = sum3_rewritten::<SatBoth>(a, b, c);
                    let s = sum3_reference::<SatBoth>(a, b, c);
                    if r != s {
                        wrong += 1;
                        if witness.is_none() {
                            witness = Some((a, b, c, s, r));
                        }
                    }
                    checked += 1;
                }
            }
        }
        assert_eq!(checked, (CARD * CARD * CARD) as u32);
        assert!(wrong > 0);
        let w = witness.unwrap();
        println!(
            "rewrite changed {wrong} of {checked} answers; witness a={} b={} c={} \
             reference={} rewritten={}",
            w.0, w.1, w.2, w.3, w.4
        );

        let mut wrong_wrap = 0u32;
        for a in MIN..=MAX {
            for b in MIN..=MAX {
                for c in MIN..=MAX {
                    if sum3_rewritten::<Wrap>(a, b, c) != sum3_reference::<Wrap>(a, b, c) {
                        wrong_wrap += 1;
                    }
                }
            }
        }
        assert_eq!(wrong_wrap, 0, "the rewrite itself is sound where the law holds");
    }

    /// Question 3. The law computes at const time, parameterised by the
    /// completion. These constants were produced by the compiler; the
    /// assertions read them back. There is no position in the source where a
    /// wrong answer could have been written.
    #[test]
    fn the_law_is_computable_at_const_time_from_the_completion() {
        let signed_triples = (CARD * CARD * CARD) as u32;
        let unsigned_triples = ((UMAX + 1) * (UMAX + 1) * (UMAX + 1)) as u32;
        assert_eq!(SAT_BOTH_ASSOC.1, signed_triples);
        assert_eq!(SAT_TOP_S_ASSOC.1, signed_triples);
        assert_eq!(WRAP_ASSOC.1, signed_triples);
        assert_eq!(SAT_TOP_U_ASSOC.1, unsigned_triples);
        assert_eq!(SAT_BOTH_U_ASSOC.1, unsigned_triples);
        assert!(SAT_BOTH_ASSOC.0 > 0);
        assert_eq!(WRAP_ASSOC.0, 0);
        println!(
            "computed associativity: {} {}/{}, {} {}/{}, {} {}/{}, {} {}/{}, {} {}/{}",
            <SatBoth as Completion>::NAME, SAT_BOTH_ASSOC.0, SAT_BOTH_ASSOC.1,
            <SatTopSigned as Completion>::NAME, SAT_TOP_S_ASSOC.0, SAT_TOP_S_ASSOC.1,
            <SatTopUnsigned as Completion>::NAME, SAT_TOP_U_ASSOC.0, SAT_TOP_U_ASSOC.1,
            <SatBothUnsigned as Completion>::NAME, SAT_BOTH_U_ASSOC.0, SAT_BOTH_U_ASSOC.1,
            <Wrap as Completion>::NAME, WRAP_ASSOC.0, WRAP_ASSOC.1
        );
    }

    /// Closure is prior to any law, and it is where my own first prediction
    /// broke. I expected a top-only clamp to associate because it does over
    /// an unsigned set. Over a SIGNED set it fails 448 of 4096 triples, and
    /// the reason is upstream of associativity: the sum escapes the value set
    /// through the bottom, which nothing clamps. A law census over an
    /// operation that is not closed is measuring a function of a different
    /// type than it claims to.
    #[test]
    fn closure_is_prior_to_any_law() {
        assert_eq!(SAT_BOTH_CLOSED.0, 0, "two-sided signed is closed");
        assert_eq!(WRAP_CLOSED.0, 0, "wrap is closed");
        assert_eq!(SAT_TOP_U_CLOSED.0, 0, "one-sided unsigned is closed");
        assert_eq!(SAT_BOTH_U_CLOSED.0, 0, "two-sided unsigned is closed");
        assert!(
            SAT_TOP_S_CLOSED.0 > 0,
            "one-sided SIGNED must escape its own value set, or the \
             correction this test records is not real"
        );
        println!(
            "closure escapes: {} {}/{}, {} {}/{}, {} {}/{}, {} {}/{}, {} {}/{}",
            <SatBoth as Completion>::NAME, SAT_BOTH_CLOSED.0, SAT_BOTH_CLOSED.1,
            <SatTopSigned as Completion>::NAME, SAT_TOP_S_CLOSED.0, SAT_TOP_S_CLOSED.1,
            <SatTopUnsigned as Completion>::NAME, SAT_TOP_U_CLOSED.0, SAT_TOP_U_CLOSED.1,
            <SatBothUnsigned as Completion>::NAME, SAT_BOTH_U_CLOSED.0, SAT_BOTH_U_CLOSED.1,
            <Wrap as Completion>::NAME, WRAP_CLOSED.0, WRAP_CLOSED.1
        );
    }

    /// With closure held, the clamp count IS the discriminator, and the
    /// comparison is controlled: same value set, same operation, one clamp
    /// against two.
    #[test]
    fn among_closed_completions_the_clamp_count_decides_associativity() {
        assert_eq!(SAT_TOP_U_CLOSED.0, 0);
        assert_eq!(SAT_BOTH_U_CLOSED.0, 0);
        assert_eq!(
            SAT_TOP_U_ASSOC.0, 0,
            "one clamp on an unsigned set is a monotone idempotent map and \
             associates"
        );
        // The second clamp is unreachable by addition on an unsigned set, so
        // it changes nothing here. The signed case is where it bites.
        assert_eq!(SAT_BOTH_U_ASSOC.0, 0);
        assert!(
            SAT_BOTH_ASSOC.0 > 0,
            "on a SIGNED set both clamps are reachable and they do not \
             commute with each other"
        );
        println!(
            "associativity failures: unsigned-top {}, unsigned-both {}, \
             signed-both {}, of {} triples each",
            SAT_TOP_U_ASSOC.0, SAT_BOTH_U_ASSOC.0, SAT_BOTH_ASSOC.0, SAT_BOTH_ASSOC.1
        );
    }

    /// The census is not vacuous: the completions really are different
    /// functions rather than several names for one, and every triple was
    /// visited.
    #[test]
    fn the_completions_are_actually_different_functions() {
        let mut both_vs_wrap = 0u32;
        for a in MIN..=MAX {
            for b in MIN..=MAX {
                if <SatBoth as Completion>::add(a, b) != <Wrap as Completion>::add(a, b) {
                    both_vs_wrap += 1;
                }
            }
        }
        assert!(both_vs_wrap > 0, "SatBoth and Wrap must differ somewhere");

        let mut u_top_vs_u_both = 0u32;
        for a in 0..=UMAX {
            for b in 0..=UMAX {
                if <SatTopUnsigned as Completion>::add(a, b)
                    != <SatBothUnsigned as Completion>::add(a, b)
                {
                    u_top_vs_u_both += 1;
                }
            }
        }
        // These two AGREE everywhere, which is the point of the controlled
        // pair: the bottom clamp is unreachable, so the only difference
        // between them is an instruction nobody executes.
        assert_eq!(u_top_vs_u_both, 0);
        assert_eq!(CARD, 16);
        println!("signed both-vs-wrap differ on {both_vs_wrap} of {} pairs", CARD * CARD);
    }
}
