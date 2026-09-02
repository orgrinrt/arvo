//! Probe 1: the three "relations" are one relation with one parameter, the
//! parameter ranges over quotients of the grade, and its domain is not a chain.
//!
//! File 33 imported the partial-algebra ladder and put graded equality on top,
//! calling the result an ordering of three relations (`33:112-115`). This probe
//! checks a different claim: all three are the SAME relation at three settings
//! of one parameter, a monoid homomorphism out of the grade, and the settings
//! that matter are not totally ordered.
//!
//! MODEL. A signed three-bit numeral, values -4..=3, folded under addition. A
//! composition names a `Resolution` per range end, exactly as `Quantisation`
//! does, so a composition can generate refusal CAUSES at one end and
//! quantisation EVENTS at the other. A result carries a value, a definedness
//! flag, a cause count and an event count. The two counts are the grade.
//!
//! THE PARAMETER. A view is how much of each generator class the relation
//! looks at: Exact (multiplicities agree), Presence (agree on whether any
//! occurred), or Ignore. Nine views. The relation is
//!
//!     t1 ~=[v] t2  iff  v(grade t1) = v(grade t2)
//!                  and  both defined implies values equal
//!
//! and the three names in the literature are three of the nine points:
//!
//!     (Ignore,   Ignore) = the WEAK equation
//!     (Presence, Ignore) = the KLEENE equation
//!     (Exact,    Exact)  = GRADED equality
//!
//! CLAIM A. For every composition and arity measured, the set of views under
//! which the law holds is DOWNWARD CLOSED in detail and closed under JOIN.
//! Downward closure is why a ladder looked plausible; join closure is the
//! stronger fact and it is what gives every law a UNIQUE FINEST view. So a
//! law's content is not a boolean per rung, it is one lattice element.
//!
//! CLAIM B, the headline. Four distinct finest views are realised by
//! compositions the design ships or can spell, and two of them are
//! INCOMPARABLE:
//!
//!     interior-safe Precise  ->  (Exact, Exact)     nothing tolerated
//!     Hot, wrapping          ->  (Exact, Ignore)    events tolerated
//!     Precise, refusing      ->  (Ignore, Exact)    causes tolerated
//!     Refuse-top/wrap-bottom ->  (Ignore, Ignore)   both tolerated
//!     Warm, saturating       ->  no view at all     the law is false
//!
//! Hot's and Precise's finest views are incomparable: neither implies the
//! other. No linear order contains them, so "the relation ladder" names a
//! shape the parameter does not have, and the point the three-name ladder
//! cannot name, (Ignore, Exact), is exactly `Precise`'s, which is the
//! composition the consolidation's open question is about (`26:608-617`).
//!
//! CLAIM C. Event invariance, recorded as asserted and never measured
//! (`33:787-789`, re-flagged at `34:463-464` and `36:468`), is measured here
//! as the event component of every view.
//!
//! CLAIM D. Wrapping addition is Kleene-associative and NOT graded-associative.
//! This refuted my own prediction, which was that a homomorphic recovery map
//! gives event invariance for free by the counting argument that the number of
//! reductions is determined by the exact total. That argument is sound for a
//! ONE-DIRECTIONAL wrap and the design's signed numerals wrap in two
//! directions, so a grouping can wrap up and back down where another does not
//! wrap at all. Probe 3 has the witness and the corrected theorem. The
//! consequence for the key: the event component of a law's view depends on
//! `Domain`. `Domain` is already in every law's key transitively, the operand
//! numeral being a never-elided slot (`33:237`); what no file has said is that
//! a verdict changes with it, and that only one component of the verdict does.
//!
//! CLAIM E. Interior safety sends the finest view to the top, so all three
//! named relations hold at once. This is file 33's section 4.3 theorem restated
//! in the view vocabulary, and it is the one part here that was already known
//! for the refusing composition. What was not measured is that it holds for
//! EVERY resolution shape rather than only that one, which is what makes the
//! at-interior-safety column of a preset table uniform rather than
//! extrapolated: `Warm` saturating and `Hot` wrapping both reach the top too,
//! and `Warm` goes from having no law at any view to having every one.
//!
//! WHAT REFUSED FIRST. Two things. My first parameter domain was "the set of
//! grade generators the consumer tolerates", a subset lattice, and under it the
//! holding family is NOT closed under meet, so a law need not have a unique
//! answer. The fix is that the parameter is a quotient of the grade rather than
//! a subset of its generators: Kleene equality collapses cause multiplicities
//! to a boolean rather than dropping them, which is a quotient and not a
//! projection, and once the domain is quotients the family closes under
//! pullback and the unique finest view exists. CLAIM A is that closure,
//! asserted rather than assumed. Second, CLAIM D above: I predicted wrapping
//! was graded and the model said otherwise before I had written a line of the
//! report.

#![allow(dead_code)]

const LO: i32 = -4;
const HI: i32 = 3;
const MODULUS: i32 = HI - LO + 1;

const REFUSE: u8 = 0;
const CLAMP: u8 = 1;
const WRAP: u8 = 2;
const SUBZERO: u8 = 3;

/// Detail levels for one generator class.
const IGNORE: u8 = 0;
const PRESENCE: u8 = 1;
const EXACT: u8 = 2;

#[derive(Copy, Clone)]
struct R {
    def: bool,
    v: i32,
    c: u32,
    e: u32,
}

const fn ok(v: i32) -> R {
    R {
        def: true,
        v,
        c: 0,
        e: 0,
    }
}

#[derive(Copy, Clone)]
struct P {
    rtop: u8,
    rbot: u8,
    ilo: i32,
    ihi: i32,
    strict: bool,
}

const fn resolve(s: i32, high: bool, r: u8) -> R {
    if r == REFUSE {
        R {
            def: false,
            v: 0,
            c: 1,
            e: 0,
        }
    } else if r == CLAMP {
        R {
            def: true,
            v: if high { HI } else { LO },
            c: 0,
            e: 1,
        }
    } else if r == WRAP {
        let mut y = s;
        while y > HI {
            y -= MODULUS;
        }
        while y < LO {
            y += MODULUS;
        }
        R {
            def: true,
            v: y,
            c: 0,
            e: 1,
        }
    } else {
        R {
            def: true,
            v: 0,
            c: 0,
            e: 1,
        }
    }
}

const fn add(a: R, b: R, p: P) -> R {
    if !a.def || !b.def {
        let (c, e) = if p.strict {
            (a.c + b.c, a.e + b.e)
        } else if !a.def {
            (a.c, a.e)
        } else {
            (a.c + b.c, a.e + b.e)
        };
        return R {
            def: false,
            v: 0,
            c,
            e,
        };
    }
    let s = a.v + b.v;
    let bc = a.c + b.c;
    let be = a.e + b.e;
    if s > p.ihi {
        let r = resolve(s, true, p.rtop);
        R {
            def: r.def,
            v: r.v,
            c: bc + r.c,
            e: be + r.e,
        }
    } else if s < p.ilo {
        let r = resolve(s, false, p.rbot);
        R {
            def: r.def,
            v: r.v,
            c: bc + r.c,
            e: be + r.e,
        }
    } else {
        R {
            def: true,
            v: s,
            c: bc,
            e: be,
        }
    }
}

/// Quantise the interior result into the numeral. A no-op when the accumulator
/// is the numeral itself, which is every composition here except the
/// interior-safe one.
const fn store(a: R, p: P) -> R {
    if !a.def {
        return a;
    }
    if a.v > HI {
        let r = resolve(a.v, true, p.rtop);
        R {
            def: r.def,
            v: r.v,
            c: a.c + r.c,
            e: a.e + r.e,
        }
    } else if a.v < LO {
        let r = resolve(a.v, false, p.rbot);
        R {
            def: r.def,
            v: r.v,
            c: a.c + r.c,
            e: a.e + r.e,
        }
    } else {
        a
    }
}

const fn eval4(x: i32, y: i32, z: i32, w: i32, g: usize, p: P) -> R {
    let (a, b, c, d) = (ok(x), ok(y), ok(z), ok(w));
    let t = if g == 0 {
        add(add(add(a, b, p), c, p), d, p)
    } else if g == 1 {
        add(add(a, add(b, c, p), p), d, p)
    } else if g == 2 {
        add(add(a, b, p), add(c, d, p), p)
    } else if g == 3 {
        add(a, add(add(b, c, p), d, p), p)
    } else {
        add(a, add(b, add(c, d, p), p), p)
    };
    store(t, p)
}

/// Does view `(dc, de)` hold at one pair of results.
const fn point_ok(a: R, b: R, dc: u8, de: u8) -> bool {
    let lw = !(a.def && b.def) || a.v == b.v;
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
    lw && cok && eok
}

const fn vindex(dc: u8, de: u8) -> u32 {
    (dc as u32) * 3 + (de as u32)
}

/// A nine-bit mask: bit `vindex(dc, de)` is set when the fold law holds under
/// that view, over every input and every pair of the five groupings of a
/// four-element fold.
const fn holding_mask(p: P) -> u32 {
    let mut mask: u32 = 0b1_1111_1111;
    let mut x = LO;
    while x <= HI {
        let mut y = LO;
        while y <= HI {
            let mut z = LO;
            while z <= HI {
                let mut w = LO;
                while w <= HI {
                    let mut i = 0;
                    while i < 5 {
                        let mut j = i + 1;
                        while j < 5 {
                            let a = eval4(x, y, z, w, i, p);
                            let b = eval4(x, y, z, w, j, p);
                            let mut dc = 0;
                            while dc < 3 {
                                let mut de = 0;
                                while de < 3 {
                                    if !point_ok(a, b, dc, de) {
                                        mask &= !(1 << vindex(dc, de));
                                    }
                                    de += 1;
                                }
                                dc += 1;
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    w += 1;
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    mask
}

const fn held(mask: u32, dc: u8, de: u8) -> bool {
    mask & (1 << vindex(dc, de)) != 0
}

/// CLAIM A, first half: holding is downward closed in detail. If the law holds
/// while looking at more of the grade, it holds while looking at less.
const fn downward_closed(mask: u32) -> bool {
    let mut dc = 0;
    while dc < 3 {
        let mut de = 0;
        while de < 3 {
            if held(mask, dc, de) {
                let mut dc2 = 0;
                while dc2 <= dc {
                    let mut de2 = 0;
                    while de2 <= de {
                        if !held(mask, dc2, de2) {
                            return false;
                        }
                        de2 += 1;
                    }
                    dc2 += 1;
                }
            }
            de += 1;
        }
        dc += 1;
    }
    true
}

/// CLAIM A, second half: holding is closed under join (pointwise maximum
/// detail). This is the pullback of the two quotients, and it is what makes
/// the finest holding view unique. Without it a law would have several
/// incomparable minimal answers and "the law's content" would not be one
/// object.
const fn join_closed(mask: u32) -> bool {
    let mut a = 0;
    while a < 9 {
        let mut b = 0;
        while b < 9 {
            let (ac, ae) = ((a / 3) as u8, (a % 3) as u8);
            let (bc, be) = ((b / 3) as u8, (b % 3) as u8);
            if held(mask, ac, ae) && held(mask, bc, be) {
                let jc = if ac > bc { ac } else { bc };
                let je = if ae > be { ae } else { be };
                if !held(mask, jc, je) {
                    return false;
                }
            }
            b += 1;
        }
        a += 1;
    }
    true
}

/// The unique finest holding view, as `vindex`, or 255 when the law holds
/// under no view at all (not even the weak equation).
const fn finest(mask: u32) -> u32 {
    if mask == 0 {
        return 255;
    }
    let mut best = 255u32;
    let mut v = 0;
    while v < 9 {
        if held(mask, (v / 3) as u8, (v % 3) as u8) {
            let (bc, be) = (best / 3, best % 3);
            if best == 255 || (v / 3 >= bc && v % 3 >= be) {
                best = v;
            }
        }
        v += 1;
    }
    best
}

const fn comp(rtop: u8, rbot: u8, strict: bool) -> P {
    P {
        rtop,
        rbot,
        ilo: LO,
        ihi: HI,
        strict,
    }
}

const HOT: P = comp(WRAP, WRAP, true);
const WARM: P = comp(CLAMP, CLAMP, true);
const PRECISE: P = comp(REFUSE, REFUSE, true);
const SUBZ: P = comp(SUBZERO, SUBZERO, true);
const MIX_RW: P = comp(REFUSE, WRAP, true);
const MIX_RW_SHORT: P = comp(REFUSE, WRAP, false);
const PRECISE_SAFE: P = P {
    rtop: REFUSE,
    rbot: REFUSE,
    ilo: 4 * LO,
    ihi: 4 * HI,
    strict: true,
};
const WARM_SAFE: P = P {
    rtop: CLAMP,
    rbot: CLAMP,
    ilo: 4 * LO,
    ihi: 4 * HI,
    strict: true,
};
const HOT_SAFE: P = P {
    rtop: WRAP,
    rbot: WRAP,
    ilo: 4 * LO,
    ihi: 4 * HI,
    strict: true,
};

const M_HOT: u32 = holding_mask(HOT);
const M_WARM: u32 = holding_mask(WARM);
const M_PRECISE: u32 = holding_mask(PRECISE);
const M_SUBZ: u32 = holding_mask(SUBZ);
const M_RW: u32 = holding_mask(MIX_RW);
const M_RW_SHORT: u32 = holding_mask(MIX_RW_SHORT);
const M_SAFE: u32 = holding_mask(PRECISE_SAFE);
const M_WARM_SAFE: u32 = holding_mask(WARM_SAFE);
const M_HOT_SAFE: u32 = holding_mask(HOT_SAFE);

// CLAIM A, at every composition measured.
const _: () = assert!(downward_closed(M_HOT) && join_closed(M_HOT));
const _: () = assert!(downward_closed(M_WARM) && join_closed(M_WARM));
const _: () = assert!(downward_closed(M_PRECISE) && join_closed(M_PRECISE));
const _: () = assert!(downward_closed(M_SUBZ) && join_closed(M_SUBZ));
const _: () = assert!(downward_closed(M_RW) && join_closed(M_RW));
const _: () = assert!(downward_closed(M_RW_SHORT) && join_closed(M_RW_SHORT));
const _: () = assert!(downward_closed(M_SAFE) && join_closed(M_SAFE));

// CLAIM B. Four distinct finest views, from five compositions.
const _: () = assert!(finest(M_SAFE) == vindex(EXACT, EXACT));
const _: () = assert!(finest(M_HOT) == vindex(EXACT, IGNORE));
const _: () = assert!(finest(M_PRECISE) == vindex(IGNORE, EXACT));
const _: () = assert!(finest(M_RW) == vindex(IGNORE, IGNORE));
const _: () = assert!(finest(M_WARM) == 255 && finest(M_SUBZ) == 255);

// The incomparability, stated directly against the two named relations rather
// than through the index arithmetic. Hot satisfies Kleene and fails the view
// that keeps events; Precise is the reverse. Neither implies the other.
const _: () = assert!(held(M_HOT, PRESENCE, IGNORE) && !held(M_HOT, IGNORE, EXACT));
const _: () = assert!(held(M_PRECISE, IGNORE, EXACT) && !held(M_PRECISE, PRESENCE, IGNORE));

// CLAIM D. Wrapping is Kleene-associative and not graded-associative, and it
// fails even at event PRESENCE, not merely at event multiplicity.
const _: () = assert!(held(M_HOT, PRESENCE, IGNORE));
const _: () = assert!(!held(M_HOT, IGNORE, PRESENCE));

// Saturating fails the weak equation, the measured inversion at `26:126-137`.
const _: () = assert!(!held(M_WARM, IGNORE, IGNORE));

// `Precise` satisfies the weak equation and fails Kleene below the accumulator
// threshold, reproducing `33:121-123` in a second independent model.
const _: () = assert!(held(M_PRECISE, IGNORE, IGNORE) && !held(M_PRECISE, PRESENCE, IGNORE));

// CLAIM E. Interior safety sends the finest view to the top, and it does so
// for EVERY composition rather than only for the refusing one. Measured for
// all three of the shipped resolution shapes, because the design's table of
// what each preset satisfies has an at-interior-safety column and a claim that
// one column is uniform is worth more when it is not extrapolated from one row.
const _: () = assert!(held(M_SAFE, EXACT, EXACT));
const _: () = assert!(finest(M_WARM_SAFE) == vindex(EXACT, EXACT));
const _: () = assert!(finest(M_HOT_SAFE) == vindex(EXACT, EXACT));
const _: () = assert!(downward_closed(M_WARM_SAFE) && join_closed(M_WARM_SAFE));
const _: () = assert!(downward_closed(M_HOT_SAFE) && join_closed(M_HOT_SAFE));

// The evaluation strategy does not change any view's verdict in this model,
// which is the honest negative: the slot is unstated and does not bite here.
// Probe 2 shows it does change the GRADE, which under a design that publishes
// the grade is an obligation even when the verdicts agree.
const _: () = assert!(M_RW == M_RW_SHORT);

pub const MASKS: [u32; 7] = [M_HOT, M_WARM, M_PRECISE, M_SUBZ, M_RW, M_RW_SHORT, M_SAFE];
pub const FINEST: [u32; 7] = [
    finest(M_HOT),
    finest(M_WARM),
    finest(M_PRECISE),
    finest(M_SUBZ),
    finest(M_RW),
    finest(M_RW_SHORT),
    finest(M_SAFE),
];
