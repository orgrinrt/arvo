// PROBE p1. Does the composed law `(a+b)-c == a+(b-c)` (u8, saturating) admit
// a nameable const predicate that exactly carves its holding region, the way
// 62/57b found for signed saturating multiplicative associativity (symmetric
// range restores it exactly)? And does that predicate follow from the
// per-operation facts already established (76_probes/probe1b: saturating add
// alone is associative UNIVERSALLY, predicate = any), or is it a genuinely new
// fact that per-operation predicates cannot supply?
//
// 76_probes/probe1b_output.txt already measured the failure rate exhaustively:
// 13,882,880 of 16,777,216 triples fail (82.7484%). This probe is independently
// re-run here (section 1) and then goes further: it hunts for the exact
// predicate (section 2), evaluated against every candidate the H1/H2 style of
// this panel's format-concept unit would suggest, and reports which candidate
// (if any) reaches zero residue in both directions (sufficiency and
// necessity), the same bar 57b's p9 held its own frame to.

fn lhs(a: u8, b: u8, c: u8) -> u8 {
    a.saturating_add(b).saturating_sub(c)
}
fn rhs(a: u8, b: u8, c: u8) -> u8 {
    a.saturating_add(b.saturating_sub(c))
}
fn holds(a: u8, b: u8, c: u8) -> bool {
    lhs(a, b, c) == rhs(a, b, c)
}

// Candidate predicates, each a pure function of (a, b, c) as exact integers.
// Named for what they test, not for a claimed mechanism; the mechanism is
// decided by which one matches, not asserted going in.

// P0: "neither elementary saturating op, applied to its own two operands in
// isolation, actually saturates." I.e. a+b <= 255 and b >= c. This is the
// naive "no clamp event anywhere" reading.
fn p0_no_elementary_clamp(a: u8, b: u8, c: u8) -> bool {
    let ab = a as i32 + b as i32;
    let bc = b as i32 - c as i32;
    ab <= 255 && bc >= 0
}

// P1: the EXACT (unbounded-integer) result a+b-c lands inside [0, 255].
fn p1_exact_result_in_range(a: u8, b: u8, c: u8) -> bool {
    let exact = a as i32 + b as i32 - c as i32;
    (0..=255).contains(&exact)
}

// P2: the "clamp-then-pullback" mechanism from 62/57b, restated for this pair
// of operations. LHS clamps a+b down to 255 (an over-range clamp) and then c
// pulls the result back toward the interior (c > 0 after the clamp fires).
// P2 names the sufficient condition for HOLDING as the negation of that
// pullback event, covering only the ceiling side (there is no floor clamp on
// a+b since both are unsigned).
fn p2_no_ceiling_pullback(a: u8, b: u8, c: u8) -> bool {
    let ab = a as i32 + b as i32;
    let clamped_high = ab > 255;
    !(clamped_high && c > 0)
}

// P3: P2 plus a guard against the mirrored underflow clamp on b - c.
fn p3_no_ceiling_pullback_and_no_underflow(a: u8, b: u8, c: u8) -> bool {
    let ab = a as i32 + b as i32;
    let bc = b as i32 - c as i32;
    let clamped_high = ab > 255;
    let clamped_low = bc < 0;
    !(clamped_high && c > 0) && !clamped_low
}

// P4: derived by hand after P0..P3 each missed one direction (see
// p1_output.txt), from the full four-way case split on (ceiling, underflow):
//   no ceiling, no underflow  -> always equal
//   ceiling, no underflow     -> equal iff c == 0
//   no ceiling, underflow     -> equal iff a == 0
//   ceiling, underflow        -> never equal
fn p4_case_split(a: u8, b: u8, c: u8) -> bool {
    let ab = a as i32 + b as i32;
    let ceiling = ab > 255;
    let underflow = b < c;
    match (ceiling, underflow) {
        (false, false) => true,
        (true, false) => c == 0,
        (false, true) => a == 0,
        (true, true) => false,
    }
}

fn main() {
    // Section 1: reproduce 76_probes/probe1b's exhaustive count independently.
    let mut fail_count: u64 = 0;
    let mut hold_count: u64 = 0;
    for a in 0u16..=255 {
        for b in 0u16..=255 {
            for c in 0u16..=255 {
                let (a, b, c) = (a as u8, b as u8, c as u8);
                if holds(a, b, c) {
                    hold_count += 1;
                } else {
                    fail_count += 1;
                }
            }
        }
    }
    println!(
        "section 1: reproduced independently. holds={} fails={} ({:.4}%) total={}",
        hold_count,
        fail_count,
        100.0 * (fail_count as f64) / 16_777_216.0,
        hold_count + fail_count
    );

    // Section 2: evaluate every candidate predicate for exact agreement
    // (sufficiency AND necessity: predicate true iff law holds), the same
    // evaluation shape 57b used for H1/H2.
    struct Candidate {
        name: &'static str,
        f: fn(u8, u8, u8) -> bool,
    }
    let candidates = [
        Candidate {
            name: "P0 no_elementary_clamp",
            f: p0_no_elementary_clamp,
        },
        Candidate {
            name: "P1 exact_result_in_range",
            f: p1_exact_result_in_range,
        },
        Candidate {
            name: "P2 no_ceiling_pullback",
            f: p2_no_ceiling_pullback,
        },
        Candidate {
            name: "P3 no_ceiling_pullback_and_no_underflow",
            f: p3_no_ceiling_pullback_and_no_underflow,
        },
        Candidate {
            name: "P4 case_split",
            f: p4_case_split,
        },
    ];

    for cand in candidates.iter() {
        let mut sufficiency_violations: u64 = 0; // predicate true, law false
        let mut necessity_violations: u64 = 0; // predicate false, law true
        let mut first_suff: Option<(u8, u8, u8)> = None;
        let mut first_nec: Option<(u8, u8, u8)> = None;
        for a in 0u16..=255 {
            for b in 0u16..=255 {
                for c in 0u16..=255 {
                    let (a, b, c) = (a as u8, b as u8, c as u8);
                    let pred = (cand.f)(a, b, c);
                    let law = holds(a, b, c);
                    if pred && !law {
                        sufficiency_violations += 1;
                        if first_suff.is_none() {
                            first_suff = Some((a, b, c));
                        }
                    }
                    if !pred && law {
                        necessity_violations += 1;
                        if first_nec.is_none() {
                            first_nec = Some((a, b, c));
                        }
                    }
                }
            }
        }
        println!(
            "{}: sufficiency_violations={} (first={:?}) necessity_violations={} (first={:?})",
            cand.name, sufficiency_violations, first_suff, necessity_violations, first_nec
        );
    }

    // Section 3: what "add is universally associative" (predicate = any)
    // would have predicted, stated explicitly. Saturating add alone is
    // associative for EVERY (a,b,c) per 76_probes/probe1b_output.txt line 1
    // re-run: predicate `any`. If that predicate were sufficient to license
    // the composed rewrite (a+b)-c == a+(b-c), every triple would agree.
    // Section 1 already shows 82.7% disagree, so this is stated for the
    // record as the negative control the panel's own evidence rule asks for:
    // a hypothesis this weak, checked and refused.
    let mut add_alone_mispredictions: u64 = 0;
    for a in 0u16..=255 {
        for b in 0u16..=255 {
            for c in 0u16..=255 {
                let (a, b, c) = (a as u8, b as u8, c as u8);
                if !holds(a, b, c) {
                    add_alone_mispredictions += 1;
                }
            }
        }
    }
    println!(
        "section 3: 'add associates universally' as a predicate for the composed law: {} mispredictions of {} ({:.4}%)",
        add_alone_mispredictions,
        16_777_216u64,
        100.0 * (add_alone_mispredictions as f64) / 16_777_216.0
    );
}
