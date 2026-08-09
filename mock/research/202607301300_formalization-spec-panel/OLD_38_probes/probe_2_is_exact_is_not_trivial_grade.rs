//! Probe 2: `Op::IS_EXACT` alone is not "the grade monoid is trivial".
//!
//! File 37 section 4.1 states: "`Op::IS_EXACT` is the statement that the
//! operation's grade monoid is trivial. An exact operation generates no
//! causes and no events" (`37:299-303`). File 35 defines `IS_EXACT` as
//! whether a quantiser sits between the exact operation and the result
//! (`35:204-214`, from `33:241`). Those are different statements: exactness
//! kills quantisation events and quantiser-generated causes; it says nothing
//! about causes that no quantiser generates. The design already names one
//! such cause with no quantiser behind it, divide-by-zero (`26:322-324`),
//! and file 33's own atom table carries `Total<Op>` as a separate atom keyed
//! on "resolutions only" (`33:492`), so the two facts were already known to
//! be independent one file before they were fused.
//!
//! The model: values 0..=7, grade = (causes, events) multiplicities.
//!   - `mul_full`-shaped: exact AND total. Grade is the unit on every pair.
//!   - wrapping add: total, NOT exact (ReduceModulo quantiser). Grade
//!     carries events on some pair: Total alone does not trivialise.
//!   - `div_exact`-shaped: exact (no quantiser anywhere, no event can ever
//!     fire), NOT total (refuses on b = 0 and on b not dividing a). Grade
//!     carries a cause on some pair: IS_EXACT alone does not trivialise.
//!
//! No shipped operation today is exact-and-partial, so nothing file 37
//! measured is wrong; but file 36 built exact division at the type level
//! (`36:280-301`), so its value-level twin is a plausible future operation,
//! and the spec sentence must be the conjunction: **`IS_EXACT` and
//! `Total<Op>` jointly make the grade monoid trivial; neither alone does.**
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_2_is_exact_is_not_trivial_grade.rs
//! Outcome: WORKS (all const assertions hold).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![no_std]
#![allow(dead_code)]

const M: i32 = 8; // model numeral: [0, 7], modulus 8

#[derive(Clone, Copy, PartialEq, Eq)]
struct Grade {
    causes: u32,
    events: u32,
}
const UNIT: Grade = Grade {
    causes: 0,
    events: 0,
};

#[derive(Clone, Copy)]
struct Out {
    value: i32, // meaningless when causes > 0
    grade: Grade,
}

// exact and total: the mul_full shape. the product numeral holds every
// product, so nothing refuses and nothing rounds.
const fn mul_full_model(a: i32, b: i32) -> Out {
    Out {
        value: a * b,
        grade: UNIT,
    }
}

// total, not exact: wrapping addition. the ReduceModulo quantiser sits
// between the exact sum and the result; each firing is one event.
const fn wrap_add_model(a: i32, b: i32) -> Out {
    let exact = a + b;
    if exact >= M {
        Out {
            value: exact - M,
            grade: Grade {
                causes: 0,
                events: 1,
            },
        }
    } else {
        Out {
            value: exact,
            grade: UNIT,
        }
    }
}

// exact, not total: exact division. no quantiser exists in this operation
// at all (nothing is ever rounded; no event can fire), yet it refuses on
// b = 0 and on any pair the quotient does not divide exactly. IS_EXACT is
// true by file 35's own definition; the grade monoid is not trivial.
const fn div_exact_model(a: i32, b: i32) -> Out {
    if b == 0 || a % b != 0 {
        Out {
            value: 0,
            grade: Grade {
                causes: 1,
                events: 0,
            },
        }
    } else {
        Out {
            value: a / b,
            grade: UNIT,
        }
    }
}

const fn grade_eq(a: Grade, b: Grade) -> bool {
    a.causes == b.causes && a.events == b.events
}

// exhaustive over the model.
const CHECK: () = {
    let mut a = 0;
    let mut exact_total_all_unit = true; // mul_full: unit everywhere
    let mut total_inexact_has_event = false; // wrap_add: some event fires
    let mut exact_partial_has_cause = false; // div_exact: some cause fires
    let mut exact_partial_has_event = false; // div_exact: no event, ever
    while a < M {
        let mut b = 0;
        while b < M {
            let m = mul_full_model(a, b);
            if !grade_eq(m.grade, UNIT) {
                exact_total_all_unit = false;
            }
            let w = wrap_add_model(a, b);
            if w.grade.events > 0 {
                total_inexact_has_event = true;
            }
            let d = div_exact_model(a, b);
            if d.grade.causes > 0 {
                exact_partial_has_cause = true;
            }
            if d.grade.events > 0 {
                exact_partial_has_event = true;
            }
            b += 1;
        }
        a += 1;
    }
    // exact AND total: trivial grade, everywhere.
    assert!(exact_total_all_unit);
    // total alone is not enough: the quantiser generates events.
    assert!(total_inexact_has_event);
    // exact alone is not enough: refusal causes need no quantiser.
    assert!(exact_partial_has_cause);
    // and the exactness half does hold: no event without a quantiser.
    assert!(!exact_partial_has_event);
};

// the view consequence, stated concretely: for div_exact the cause-ignoring
// and cause-exact views disagree on whether two grades are identified, so
// "every view identifies every grade" (37's collapse) is false for it.
const VIEW_SPLIT: () = {
    let g1 = div_exact_model(7, 2).grade; // one cause
    let g2 = div_exact_model(8 - 2, 2).grade; // unit (6/2 exact)
                                              // cause-Ignore view: both map to unit; identified.
                                              // cause-Exact view: (1,0) vs (0,0); distinguished.
    assert!(!grade_eq(g1, g2));
};
