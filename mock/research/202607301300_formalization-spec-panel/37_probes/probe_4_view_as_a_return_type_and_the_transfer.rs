//! Probe 4: the mechanism. A law returns a lattice element rather than a
//! boolean, and a combinator that regroups publishes exactly what its law does
//! not preserve, in its own result grade.
//!
//! This is the const-fn-is-the-key discipline the design already ratified
//! (`26:174-186`) with exactly one thing changed: the return type. No axis is
//! added, no new trait family is introduced, no unstable feature is reached
//! for, and `generic_const_exprs` is not needed anywhere.
//!
//! WHY THE RETURN TYPE AND NOT THREE MARKERS. File 33's atom list gives three
//! separate derived marker traits for one fold law (`ValueAssociative`,
//! `DefinednessInvariant`, `EventInvariant`, `33:483-485`). Three problems.
//!
//! Three markers cannot name the PRESENCE level. Probe 1 measures nine views,
//! not eight, because a consumer propagating an error bound needs event
//! multiplicities while a consumer asking "did anything round at all" needs
//! only presence, and those are different facts about the same fold. Covering
//! nine views with markers takes five, whose conjunctions span thirty-two
//! combinations of which nine mean anything, which is the
//! rich-index-that-constrains-nothing failure in the markers' own form.
//!
//! Three markers have three coherence surfaces and one invariant with no home:
//! probe 1's CLAIM A, that the holding set is downward closed and join closed,
//! is a relationship AMONG the markers. Copies decorrelate, and this one would
//! decorrelate silently, because a marker impl'd where it should not be looks
//! exactly like one that should be.
//!
//! And three markers cannot express the law's content, which is one object:
//! the unique finest view. A consumer asking how associative a composition is
//! gets three booleans and reconstructs the lattice element itself.
//!
//! THE TRANSFER RULE, which is what stops the lattice from being decoration.
//! A view is coeffect-shaped: it is about what a consumer will put up with,
//! and this review has already measured what happens to a permission-shaped
//! fact that carries no data, which is that a corrupted grant compiles clean
//! with zero diagnostic (`26:213-215`, and the droplist's `ViewC` entry at
//! `26:735-742`). The same disease is available here the moment a consumer is
//! allowed to DECLARE which view it needs: declaring a weak one is a waiver,
//! and nothing checks a waiver.
//!
//! So no consumer declares anything. The combinator publishes exactly the
//! generator classes its law fails to preserve, and the consumer's contract is
//! the ordinary type of the result. A caller that needs a fold whose
//! definedness matches the sequential one asks for `Folded<0>` and cannot be
//! handed a `Precise` regrouping below interior safety, because that
//! combinator's result type is `Folded<1>`. The coeffect discharges into an
//! effect, which is the asymmetry file 17 identified as the single sentence
//! worth putting in the spec (`26:213-215`), used here to make itself
//! unnecessary.
//!
//! WHAT REFUSED FIRST, and it changed the design. My first version had the
//! consumer declare a required view, checked the law against it, and ALSO
//! published a deficit. The compiler refused two of its own call sites:
//!
//!     error[E0080]: evaluation panicked: this composition's fold law does not
//!     hold at the required view: no regrouping of it is licensed at that
//!     detail
//!        --> probe_4...rs:309:5  (PRECISE_TOLERATING_REFUSAL)
//!
//! The two halves were fighting: the licence check refuses exactly the case
//! the transfer exists to handle. Pulling on that found the real error, which
//! is that "the consumer requires a view" is two different things run together.
//! Where the WEAK equation fails the values themselves diverge and no
//! publication rescues anything, so that case is a hard refusal. Everywhere
//! else the regrouping is always sound and the only question is what it must
//! say about itself, which is derived rather than requested. The `REQ`
//! parameters are gone, the mechanism is smaller, and there is no
//! consumer-supplied index left to be too rich.
//!
//! CLAIM A. The whole mechanism compiles with no unstable feature: a const fn
//! per law returning a `View`, a derived deficit, and a const assertion binding
//! the published grade.
//!
//! CLAIM B. The published grade cannot be COMPUTED in return position, because
//! that is an expression over generic const parameters in type position, the
//! wall the droplist records (`26:719-724`) and file 36 re-verified from a
//! fourth direction (`36:88-99`). It is therefore DECLARED and CHECKED, the
//! shape file 35 recommends for `Op::IS_EXACT` (`35:204-214`). Understating is
//! a compile error (probe 4c); overstating compiles and is merely pessimistic,
//! the same safe direction files 31, 33 and 34 all take on lattice
//! containment.
//!
//! CLAIM C. The const fn's body is probe 1's and probe 3's measurements, so the
//! mechanism and the measurement are the same numbers rather than two
//! statements of one thing that will decorrelate.
//!
//! Probes 4b and 4c are committed refusing.

#![allow(dead_code)]

// ---------------------------------------------------------------- the lattice

/// How much of one grade generator class a view looks at.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Detail {
    Ignore = 0,
    Presence = 1,
    Exact = 2,
}

/// A view: one detail level per generator class. The real design has one entry
/// per refusal cause and per quantisation event kind; two classes is what the
/// measurement covers and the shape does not change with more.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct View {
    pub causes: Detail,
    pub events: Detail,
}

impl View {
    pub const fn new(causes: Detail, events: Detail) -> Self {
        View { causes, events }
    }

    /// The three names the literature has, as three points of one lattice.
    pub const WEAK: View = View::new(Detail::Ignore, Detail::Ignore);
    pub const KLEENE: View = View::new(Detail::Presence, Detail::Ignore);
    pub const GRADED: View = View::new(Detail::Exact, Detail::Exact);
    /// The point the three names cannot reach, which is `Precise`'s.
    pub const MODULO_REFUSAL: View = View::new(Detail::Ignore, Detail::Exact);

    /// Is `self` at least as fine as `other`. Pointwise, which is why this is
    /// a lattice and not a chain.
    pub const fn at_least(self, other: View) -> bool {
        (self.causes as u8) >= (other.causes as u8) && (self.events as u8) >= (other.events as u8)
    }

    /// The generator classes a regrouping under this law may disagree with the
    /// sequential fold about. Bit 0 is causes, bit 1 is events.
    pub const fn unpreserved(self) -> u8 {
        let mut d = 0u8;
        if (self.causes as u8) < (Detail::Exact as u8) {
            d |= 1;
        }
        if (self.events as u8) < (Detail::Exact as u8) {
            d |= 2;
        }
        d
    }
}

/// A law either holds under some finest view or holds under none.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LawView {
    Never,
    Finest(View),
}

// ------------------------------------------------------------------- the key
//
// The key with `Growth` removed per file 35 and the numeral identified by its
// own members. Every parameter here is one the law's proof actually used; a
// parameter the body does not name cannot be read, which is the completeness
// direction the const-fn shape already buys (`26:174-186`).

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Resolution {
    Refuse,
    Clamp,
    ReduceModulo,
    SubstituteZero,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Domain {
    Unsigned,
    Signed,
}

/// The one derived fact for the fold law. Its body is probe 1's and probe 3's
/// measurements and nothing else.
pub const fn add_assoc_view(
    top: Resolution,
    bot: Resolution,
    domain: Domain,
    arity: u32,
    // Interior headroom, as a multiple of the numeral's own range. Interior
    // safety is `headroom >= arity - 1` (`26:717`, file 34's n-1 contract at
    // `34:203-206`).
    headroom: u32,
) -> LawView {
    if headroom + 1 >= arity {
        // One quantisation fires, at the root, on a grouping-independent
        // argument. Every component agrees at once.
        return LawView::Finest(View::GRADED);
    }
    match (top, bot) {
        (Resolution::ReduceModulo, Resolution::ReduceModulo) => match domain {
            // One-directional reduction: the event count is the exact total
            // over the modulus, hence grouping-independent (probe 3, CLAIM A).
            Domain::Unsigned => LawView::Finest(View::GRADED),
            // Two-directional: reductions cancel in the value and are both
            // counted in the grade (probe 3, CLAIM C).
            Domain::Signed => LawView::Finest(View::new(Detail::Exact, Detail::Ignore)),
        },
        // Refusing at both ends: values agree wherever both groupings return,
        // definedness does not, and there are no events to disagree about.
        (Resolution::Refuse, Resolution::Refuse) => LawView::Finest(View::MODULO_REFUSAL),
        // Refusing at one end and reducing at the other: both generator classes
        // are live and neither is invariant (probe 1, the RW composition).
        (Resolution::Refuse, Resolution::ReduceModulo)
        | (Resolution::ReduceModulo, Resolution::Refuse) => LawView::Finest(View::WEAK),
        // Clamping is a retraction: it preserves order and not the operation,
        // so the values themselves diverge and no view recovers the law
        // (`26:126-137`, reproduced in probe 1).
        _ => LawView::Never,
    }
}

// -------------------------------------------------------------- the transfer
//
// A fold's result carries, in its type, the generator classes in which it may
// disagree with the sequential fold. Bit 0: it may refuse where the sequential
// fold returned, or return where it refused. Bit 1: it may carry a different
// multiset of quantisation events.

pub struct Folded<const GRADE: u8>(pub i32);

pub const fn regroup_fold<
    const TOP: u8,
    const BOT: u8,
    const DOM: u8,
    const ARITY: u32,
    const HEADROOM: u32,
    const PUBLISHED: u8,
>(
    xs: [i32; 4],
) -> Folded<PUBLISHED> {
    let law = add_assoc_view(
        resolution(TOP),
        resolution(BOT),
        domain(DOM),
        ARITY,
        HEADROOM,
    );
    // Where the values themselves diverge, no publication rescues anything.
    const_assert_values_agree(law);
    // And whatever the law does not preserve is published, never waived.
    const_assert_published(law, PUBLISHED);
    let mut acc = 0;
    let mut i = 0;
    while i < 4 {
        acc += xs[i];
        i += 1;
    }
    Folded(acc)
}

const fn resolution(r: u8) -> Resolution {
    if r == 0 {
        Resolution::Refuse
    } else if r == 1 {
        Resolution::Clamp
    } else if r == 2 {
        Resolution::ReduceModulo
    } else {
        Resolution::SubstituteZero
    }
}

const fn domain(d: u8) -> Domain {
    if d == 0 {
        Domain::Unsigned
    } else {
        Domain::Signed
    }
}

const fn const_assert_values_agree(law: LawView) {
    assert!(
        !matches!(law, LawView::Never),
        "this composition's fold has no associativity law at any view: \
         regrouping it changes the delivered value, so no published grade \
         makes the regrouping honest. Widen the accumulator until the fold is \
         interior-safe, or do not regroup."
    );
}

const fn const_assert_published(law: LawView, published: u8) {
    let needed = match law {
        LawView::Never => 3,
        LawView::Finest(v) => v.unpreserved(),
    };
    assert!(
        published & needed == needed,
        "a regrouping must publish every grade generator class its law does \
         not preserve: tolerance is a transfer, never a waiver."
    );
}

// --------------------------------------------------------------- call sites
//
// CLAIMS A and C, compiled: every composition probe 1 measured, each publishing
// exactly what its own law fails to preserve.

/// Unsigned wrapping: graded, so nothing is published.
pub const UNSIGNED_WRAP: Folded<0> = regroup_fold::<2, 2, 0, 4, 0, 0>([1, 2, 3, 4]);

/// Signed wrapping: values and definedness agree, event multiplicities do not,
/// so bit 1 is published. This is the fact probe 3 found and no file had.
pub const SIGNED_WRAP: Folded<2> = regroup_fold::<2, 2, 1, 4, 0, 2>([1, 2, 3, 4]);

/// `Precise` below interior safety: values agree, definedness does not, so bit
/// 0 is published. This is the answer to what `Precise` is: not "associative",
/// not "not associative", but associative with its refusals transferred.
pub const PRECISE_BELOW: Folded<1> = regroup_fold::<0, 0, 1, 4, 0, 1>([1, 2, 3, 4]);

/// `Precise` at interior safety: nothing is published, and this is the same
/// combinator rather than a different one.
pub const PRECISE_SAFE: Folded<0> = regroup_fold::<0, 0, 1, 4, 3, 0>([1, 2, 3, 4]);

/// Refusing at one end and reducing at the other: both classes published.
pub const MIXED: Folded<3> = regroup_fold::<0, 2, 1, 4, 0, 3>([1, 2, 3, 4]);

/// Overstating is sound and merely pessimistic: a combinator may publish more
/// than its law forces.
pub const PRECISE_OVERSTATED: Folded<3> = regroup_fold::<0, 0, 1, 4, 0, 3>([1, 2, 3, 4]);

/// A caller whose own contract is a refusal-faithful fold. It cannot be handed
/// the `Precise` regrouping below interior safety, and probe 4c is that error.
pub const fn needs_faithful_definedness(f: Folded<0>) -> i32 {
    f.0
}
pub const OK_AT_INTERIOR_SAFETY: i32 = needs_faithful_definedness(PRECISE_SAFE);
pub const OK_UNSIGNED: i32 = needs_faithful_definedness(UNSIGNED_WRAP);

// The three named relations as three points of one parameter, and the two
// middle points incomparable, which is probe 1's headline in the mechanism's
// own vocabulary.
const _: () = assert!(View::GRADED.at_least(View::KLEENE));
const _: () = assert!(View::KLEENE.at_least(View::WEAK));
const _: () = assert!(View::GRADED.at_least(View::MODULO_REFUSAL));
const _: () = assert!(!View::KLEENE.at_least(View::MODULO_REFUSAL));
const _: () = assert!(!View::MODULO_REFUSAL.at_least(View::KLEENE));
