//! Probe 2: file 34's reification lemma is true of one reifier and false in
//! general, and the hypothesis it needs is about the reifying element, not
//! about the relation.
//!
//! File 34's claim (`34:176-190`): "a refusal reified as an absorbing value
//! turns every definedness split into a value split", and therefore "the one
//! relation invariant under the Refuse-to-special reification is the graded
//! one". That reading is currently the only argument on the table tilting the
//! consolidation's open question (`26:608-617`) toward the graded rung, so it
//! is worth more than the one witness pair it was drawn from.
//!
//! Two reifications of the same `Refuse` composition are measured, both of
//! them shapes the design already has:
//!
//!   NAN  the refusal becomes a distinguished element OUTSIDE the numeral's
//!        value set, absorbing. This is the `Specials` extension file 34 used.
//!
//!   ZERO the refusal becomes `SubstituteZero`, which is one of the design's
//!        own four `Resolution` instances (`26:44-48`). The delivered element
//!        is INSIDE the value set and does NOT absorb.
//!
//! For each of the nine views of probe 1, the probe asks whether the view's
//! verdict AT EVERY POINT is the same under `Refuse` and under the reification.
//!
//! CLAIM A. Under the NAN reification, the views that keep any cause
//! information (Kleene and everything finer) are preserved pointwise; the weak
//! equation is not. That is file 34's finding, confirmed, and it is the
//! narrower half of what it claimed.
//!
//! CLAIM B. Under the ZERO reification, NO view is preserved, the graded one
//! included. Witness at x = 3, y = 3, z = 1: under `Refuse` both groupings
//! refuse with one cause each, so every one of the nine views holds at that
//! point; under `SubstituteZero` the left grouping delivers 1 and the right
//! delivers 3, so the values disagree and every one of the nine views fails.
//! All nine flip together, in the same direction, at one input.
//!
//! So reification stability is not a property that distinguishes the relations.
//! It is a property of the reifying element:
//!
//!   A reification preserves a view's verdict when the reifying element lies
//!   outside the numeral's value set AND absorbs the operation. The first
//!   conjunct is what makes "this term failed" observable in the value; the
//!   second is what stops the continuation from computing anything further.
//!   With either conjunct dropped, no view survives, graded included.
//!
//! The design consequence is not about which relation to name. It is that
//! `SubstituteZero` DESTROYS grade information at the value level while the
//! grade itself still records the event, so a law's verdict cannot be
//! transported across a change in the resolutions, which is exactly why the
//! resolutions are in the key (`33:240`) and is a reason independent of the
//! one already recorded there.
//!
//! WHAT REFUSED FIRST. My first version reified only to NAN and concluded file
//! 34's lemma held, because `SubstituteZero` did not look like a "reification"
//! until I noticed that delivering a refusal as a value is exactly what it
//! does. The CLAIM B assertion was written after that and is the finding; the
//! CLAIM A assertion is the half that survives.

#![allow(dead_code)]

const LO: i32 = -4;
const HI: i32 = 3;

const IGNORE: u8 = 0;
const PRESENCE: u8 = 1;
const EXACT: u8 = 2;

/// A result. `nan` is the out-of-set special; it is a defined result whose
/// value is not a member of the numeral, and it absorbs.
#[derive(Copy, Clone)]
struct R {
    def: bool,
    nan: bool,
    v: i32,
    c: u32,
    e: u32,
}

const fn ok(v: i32) -> R {
    R {
        def: true,
        nan: false,
        v,
        c: 0,
        e: 0,
    }
}

/// Which shape a past-range result takes.
const MODE_REFUSE: u8 = 0;
const MODE_NAN: u8 = 1;
const MODE_ZERO: u8 = 2;

const fn add(a: R, b: R, mode: u8) -> R {
    if !a.def {
        return R {
            def: false,
            nan: false,
            v: 0,
            c: a.c,
            e: a.e,
        };
    }
    if !b.def {
        return R {
            def: false,
            nan: false,
            v: 0,
            c: a.c + b.c,
            e: a.e + b.e,
        };
    }
    let bc = a.c + b.c;
    let be = a.e + b.e;
    // The special absorbs, and absorbing raises nothing further.
    if a.nan || b.nan {
        return R {
            def: true,
            nan: true,
            v: 0,
            c: bc,
            e: be,
        };
    }
    let s = a.v + b.v;
    if s > HI || s < LO {
        if mode == MODE_REFUSE {
            R {
                def: false,
                nan: false,
                v: 0,
                c: bc + 1,
                e: be,
            }
        } else if mode == MODE_NAN {
            R {
                def: true,
                nan: true,
                v: 0,
                c: bc + 1,
                e: be,
            }
        } else {
            R {
                def: true,
                nan: false,
                v: 0,
                c: bc + 1,
                e: be,
            }
        }
    } else {
        R {
            def: true,
            nan: false,
            v: s,
            c: bc,
            e: be,
        }
    }
}

const fn eval3(x: i32, y: i32, z: i32, g: usize, mode: u8) -> R {
    let (a, b, c) = (ok(x), ok(y), ok(z));
    if g == 0 {
        add(add(a, b, mode), c, mode)
    } else {
        add(a, add(b, c, mode), mode)
    }
}

/// Values agree. Two specials are one value-level class, per the canonical
/// quotient file 34 settled (`34:270-277`); a special and a number are not.
const fn values_agree(a: R, b: R) -> bool {
    if !(a.def && b.def) {
        return true;
    }
    if a.nan || b.nan {
        return a.nan && b.nan;
    }
    a.v == b.v
}

const fn point_ok(a: R, b: R, dc: u8, de: u8) -> bool {
    let cok = if dc == EXACT {
        a.c == b.c
    } else if dc == PRESENCE {
        (a.c > 0) == (b.c > 0)
    } else {
        true
    };
    let eok = if de == EXACT {
        a.e == b.e
    } else if de == PRESENCE {
        (a.e > 0) == (b.e > 0)
    } else {
        true
    };
    values_agree(a, b) && cok && eok
}

/// Is view `(dc, de)`'s verdict the same at every point under `Refuse` and
/// under the reification named by `mode`?
const fn preserved(dc: u8, de: u8, mode: u8) -> bool {
    let mut x = LO;
    while x <= HI {
        let mut y = LO;
        while y <= HI {
            let mut z = LO;
            while z <= HI {
                let r0 = eval3(x, y, z, 0, MODE_REFUSE);
                let r1 = eval3(x, y, z, 1, MODE_REFUSE);
                let s0 = eval3(x, y, z, 0, mode);
                let s1 = eval3(x, y, z, 1, mode);
                if point_ok(r0, r1, dc, de) != point_ok(s0, s1, dc, de) {
                    return false;
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    true
}

// CLAIM A. Under the out-of-set absorbing special, every view that keeps any
// cause information is preserved pointwise, and the weak equation is not.
const _: () = assert!(preserved(PRESENCE, IGNORE, MODE_NAN)); // Kleene
const _: () = assert!(preserved(EXACT, EXACT, MODE_NAN)); // graded
const _: () = assert!(preserved(EXACT, IGNORE, MODE_NAN));
const _: () = assert!(preserved(PRESENCE, EXACT, MODE_NAN));
const _: () = assert!(!preserved(IGNORE, IGNORE, MODE_NAN)); // weak, not preserved
const _: () = assert!(!preserved(IGNORE, EXACT, MODE_NAN));
const _: () = assert!(!preserved(IGNORE, PRESENCE, MODE_NAN));

// CLAIM B. Under `SubstituteZero`, an ordinary member of the design's own
// `Resolution` set, NO view is preserved. All nine.
const fn none_preserved_under_zero() -> bool {
    let mut dc = 0;
    while dc < 3 {
        let mut de = 0;
        while de < 3 {
            if preserved(dc, de, MODE_ZERO) {
                return false;
            }
            de += 1;
        }
        dc += 1;
    }
    true
}
const _: () = assert!(none_preserved_under_zero());

// The witness, stated so a reader does not have to trust the sweep. At
// (3, 3, 1) both groupings refuse with one cause each, so every view holds;
// under `SubstituteZero` the left delivers 1 and the right delivers 3.
const W_L_REFUSE: R = eval3(3, 3, 1, 0, MODE_REFUSE);
const W_R_REFUSE: R = eval3(3, 3, 1, 1, MODE_REFUSE);
const W_L_ZERO: R = eval3(3, 3, 1, 0, MODE_ZERO);
const W_R_ZERO: R = eval3(3, 3, 1, 1, MODE_ZERO);

const _: () = assert!(!W_L_REFUSE.def && !W_R_REFUSE.def);
const _: () = assert!(W_L_REFUSE.c == 1 && W_R_REFUSE.c == 1);
const _: () = assert!(point_ok(W_L_REFUSE, W_R_REFUSE, EXACT, EXACT));
const _: () = assert!(W_L_ZERO.def && W_R_ZERO.def);
const _: () = assert!(W_L_ZERO.v == 1 && W_R_ZERO.v == 3);
const _: () = assert!(!point_ok(W_L_ZERO, W_R_ZERO, IGNORE, IGNORE));

// And the grade is the same on both sides of the witness, which is the sharp
// form of the finding: the graded relation had all the information it needed
// and still flipped, because what changed was the VALUE the resolution
// delivered, not the grade it recorded.
const _: () = assert!(W_L_ZERO.c == W_R_ZERO.c && W_L_ZERO.e == W_R_ZERO.e);
