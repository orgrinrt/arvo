//! Probe 4: the evaluation strategy of a refusing operand's sibling, decided
//! by writing the consumer's own diagnostic path and then checking it
//! exhaustively rather than arguing about it.
//!
//! The design owes one sentence (`40:639-641`). File 37 measured that the
//! choice changes the grade a term publishes and no law's verdict
//! (`37:227-243`). What no file has asked is what a consumer can rely on under
//! either reading, so this probe asks it, over every term of a small shape.
//!
//! MY FIRST HYPOTHESIS WAS WRONG AND THE CHECK KILLED IT, which is why the
//! check is here rather than the argument. I expected the short circuit to make
//! the report grouping-dependent, since a regrouping refuses at a different
//! node. It does not: CLAIM A below is an exhaustive negative over all 81
//! four-leaf terms. Every grouping visits the leaves left to right, so the
//! short circuit always reports the prefix before the first refusing leaf,
//! whatever the tree. The short circuit is grouping-invariant, and any
//! objection to it on those grounds, including the one this probe was written
//! to make, is void.
//!
//! CLAIM B is what survives, and it is worse for the short circuit than the
//! objection it replaces. The report is not invariant under REORDERING. Same
//! multiset of channels, three orders, three different reports, while the
//! delivered value is the same because addition is commutative. Strict gives
//! one report for all of them.
//!
//! CLAIM C. Under the short circuit the report carries less as the data gets
//! worse, so a consumer's diagnostic degrades exactly when it is needed.
//!
//! The scenario throughout: independently scaled telemetry channels combined
//! into one sample. A channel narrowing into a `Precise` numeral may REFUSE;
//! one narrowing into a `Warm` numeral ROUNDS, and a rounding is what tells the
//! consumer that channel is drifting toward its limit. The consumer reads the
//! report off a refused sample to decide which channel to rescale, which is the
//! only reason anyone publishes a grade.
//!
//! EXPECTED: COMPILES CLEAN. Every claim is a const assertion; the finding is
//! the claims, not the outcome.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib probe_4_the_siblings_report.rs

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// One term's grade: causes and events, file 37's two generator classes, as
// multiplicities because "which channel do I rescale" needs counts.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Report {
    pub causes: u32,
    pub events: u32,
}

const NONE: Report = Report {
    causes: 0,
    events: 0,
};
/// A channel that refused: one cause, no event.
const REFUSAL: Report = Report {
    causes: 1,
    events: 0,
};
/// A channel that rounded: no cause, one event.
const ROUNDING: Report = Report {
    causes: 0,
    events: 1,
};

const fn join(a: Report, b: Report) -> Report {
    Report {
        causes: a.causes + b.causes,
        events: a.events + b.events,
    }
}
const fn same(a: Report, b: Report) -> bool {
    a.causes == b.causes && a.events == b.events
}

/// The two readings, side by side, as the only difference in the whole probe.
const fn combine(l: Report, r: Report, strict: bool) -> Report {
    if strict {
        // Both operands evaluated; both reports join, whatever either did.
        join(l, r)
    } else if l.causes > 0 {
        // Left refused; the right operand is never evaluated and contributes
        // nothing.
        l
    } else {
        join(l, r)
    }
}

const STRICT: bool = true;
const SHORT: bool = false;

// ---------------------------------------------------------------------------
// Three groupings of a four-leaf term. The transfer rule licenses a regrouping
// to replace any of these with any other, so the consumer cannot know which one
// runs.
// ---------------------------------------------------------------------------

const fn left_nested(t: [Report; 4], s: bool) -> Report {
    combine(combine(combine(t[0], t[1], s), t[2], s), t[3], s)
}
const fn balanced(t: [Report; 4], s: bool) -> Report {
    combine(combine(t[0], t[1], s), combine(t[2], t[3], s), s)
}
const fn right_nested(t: [Report; 4], s: bool) -> Report {
    combine(t[0], combine(t[1], combine(t[2], t[3], s), s), s)
}

/// Leaf alphabet: refusal, rounding, clean.
const fn leaf(i: u32) -> Report {
    if i == 0 {
        REFUSAL
    } else if i == 1 {
        ROUNDING
    } else {
        NONE
    }
}

// ---------------------------------------------------------------------------
// CLAIM A (exhaustive, and the refutation of this probe's own first idea):
// the report is grouping-invariant under BOTH readings, over all 3^4 = 81
// four-leaf terms.
//
// Every grouping visits leaves left to right, so the short circuit reports the
// prefix before the first refusing leaf regardless of tree shape.
// ---------------------------------------------------------------------------

const fn grouping_invariant_everywhere(s: bool) -> bool {
    let mut a = 0;
    while a < 3 {
        let mut b = 0;
        while b < 3 {
            let mut c = 0;
            while c < 3 {
                let mut d = 0;
                while d < 3 {
                    let t = [leaf(a), leaf(b), leaf(c), leaf(d)];
                    if !same(left_nested(t, s), balanced(t, s))
                        || !same(balanced(t, s), right_nested(t, s))
                    {
                        return false;
                    }
                    d += 1;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

const _: () = assert!(grouping_invariant_everywhere(STRICT));
const _: () = assert!(grouping_invariant_everywhere(SHORT));

// ---------------------------------------------------------------------------
// CLAIM B: reordering. The value of a sum does not depend on the order of its
// terms. Under the short circuit the report does.
//
// One multiset of four channels: one refusing, two rounding, one clean. Three
// orders, all of which a consumer treats as the same sum, and between which the
// stack's own locality machinery moves (hilavitkutin's RCM renumbering is
// exactly a permutation of a column's traversal order).
// ---------------------------------------------------------------------------

const REFUSING_FIRST: [Report; 4] = [REFUSAL, ROUNDING, ROUNDING, NONE];
const REFUSING_SECOND: [Report; 4] = [ROUNDING, REFUSAL, ROUNDING, NONE];
const REFUSING_THIRD: [Report; 4] = [ROUNDING, ROUNDING, REFUSAL, NONE];

// Short circuit: three orders, three reports. The consumer is told about zero,
// one, or two drifting channels depending on where the refusing one happened to
// sit in the traversal.
const _: () = assert!(balanced(REFUSING_FIRST, SHORT).events == 0);
const _: () = assert!(balanced(REFUSING_SECOND, SHORT).events == 1);
const _: () = assert!(balanced(REFUSING_THIRD, SHORT).events == 2);

// Strict: one report, and it is the term's own leaf multiset.
const _: () = assert!(balanced(REFUSING_FIRST, STRICT).events == 2);
const _: () = assert!(balanced(REFUSING_SECOND, STRICT).events == 2);
const _: () = assert!(balanced(REFUSING_THIRD, STRICT).events == 2);

// Stated as the property, so the next reader sees which invariant is at stake.
const _: () = assert!(same(
    balanced(REFUSING_FIRST, STRICT),
    balanced(REFUSING_THIRD, STRICT)
));
const _: () = assert!(!same(
    balanced(REFUSING_FIRST, SHORT),
    balanced(REFUSING_THIRD, SHORT)
));

// The delivered outcome agrees in every case under both readings, which is what
// makes the divergence a report defect rather than an arithmetic one: the
// refusal is present in all six.
const _: () = assert!(balanced(REFUSING_FIRST, SHORT).causes == 1);
const _: () = assert!(balanced(REFUSING_THIRD, SHORT).causes == 1);
const _: () = assert!(balanced(REFUSING_FIRST, STRICT).causes == 1);
const _: () = assert!(balanced(REFUSING_THIRD, STRICT).causes == 1);

// ---------------------------------------------------------------------------
// CLAIM C: the diagnostic degrades as the data worsens, under the short circuit
// only. Same term shape, three inputs, increasingly bad.
// ---------------------------------------------------------------------------

/// Nothing refuses. Both readings see both drifting channels.
const CLEAN: [Report; 4] = [NONE, ROUNDING, NONE, ROUNDING];
/// One channel refuses, late. The short circuit has lost one drift already.
const RIGHT_BAD: [Report; 4] = [NONE, ROUNDING, REFUSAL, ROUNDING];
/// It refuses early. The short circuit reports no drift at all.
const LEFT_BAD: [Report; 4] = [REFUSAL, ROUNDING, NONE, ROUNDING];

const _: () = assert!(balanced(CLEAN, SHORT).events == 2);
const _: () = assert!(balanced(RIGHT_BAD, SHORT).events == 1);
const _: () = assert!(balanced(LEFT_BAD, SHORT).events == 0);

const _: () = assert!(balanced(CLEAN, STRICT).events == 2);
const _: () = assert!(balanced(RIGHT_BAD, STRICT).events == 2);
const _: () = assert!(balanced(LEFT_BAD, STRICT).events == 2);

// ---------------------------------------------------------------------------
// The consumer's actual code, and what each reading does to it.
// ---------------------------------------------------------------------------

/// Read off a refused sample: are any channels drifting toward their limits?
/// A consumer runs this and rescales.
pub const fn needs_rescale(r: Report) -> bool {
    r.events > 0
}

/// Strict: a refused sample still names the drifting channels, so the consumer
/// fixes the pipeline.
const _: () = assert!(needs_rescale(balanced(LEFT_BAD, STRICT)));
/// Short circuit: the same sample reports nothing to rescale, so the consumer
/// ships a pipeline that quietly keeps refusing.
const _: () = assert!(!needs_rescale(balanced(LEFT_BAD, SHORT)));
