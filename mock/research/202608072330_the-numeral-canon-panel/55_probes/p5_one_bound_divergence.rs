//! Probe 5: one reachable bound does not save associativity, and the mechanism
//! is clamp-then-pullback, not two-bound reachability.
//!
//! Motivation. `56` section 3.3 quotes `42`'s mechanism as: associativity of a
//! clamped operation "holds exactly when at most one of its clamps can be
//! triggered by any association order of the specific fold in question"
//! (`42:315-316` as quoted by `56`; I have not opened `42` and say so). `56`'s
//! own coverage names, as unresolved, whether window coherence is strictly
//! coarser than that per-trajectory condition, with the separating case
//! unconstructed.
//!
//! Hand analysis produced a candidate that separates them in the OTHER
//! direction: signed clamp on Q = [-8, 7], operands (7, 7, -1). Both
//! association orders trigger only the ceiling, never the floor, yet
//! (7 # 7) # -1 = 6 and 7 # (7 # -1) = 7. If that holds up exhaustively, the
//! quoted condition's sufficiency direction ("one bound triggered implies
//! associativity survives") is refuted, and the mechanism wants restating:
//! divergence needs a clamp event FOLLOWED BY MOVEMENT TOWARD THE INTERIOR
//! (an operand pulling a clamped partial sum back into range, making the
//! clamp's information loss observable). One-directional folds cannot do
//! that, which is why add-only unsigned saturation is coherent, and why the
//! Q12 divergence split tracks operand-sign mixture rather than bound count.
//!
//! Measured, exhaustive over Q^3 for signed clamp, both associations:
//!   1. divergent triples where NO clamp fires in either association: must be
//!      0 (no clamp means exact integer arithmetic, associative; this is the
//!      instrument check that the bound-event bookkeeping can fail).
//!   2. divergent triples where ONLY THE CEILING fires across both
//!      associations: predicted > 0, witness (7, 7, -1) confirmed among them.
//!   3. divergent triples where ONLY THE FLOOR fires: predicted > 0 by
//!      symmetry, witness (-8, -8, 1).
//!   4. divergent triples with all operands of one sign (all >= 0 or all
//!      <= 0): predicted 0, the monotone-trajectory case where a clamped
//!      partial sum is never pulled back.
//!   5. every divergent triple has mixed-sign operands and at least one clamp
//!      event: the necessary-conditions check for the restated mechanism.

const LO: i64 = -8;
const HI: i64 = 7;

fn clamp(x: i64) -> i64 {
    x.clamp(LO, HI)
}

// events: (floor_fired, ceiling_fired) for one association order of (a, b, c)
fn events_left(a: i64, b: i64, c: i64) -> (bool, bool, i64) {
    let s1 = a + b;
    let r1 = clamp(s1);
    let s2 = r1 + c;
    let r2 = clamp(s2);
    let floor = s1 < LO || s2 < LO;
    let ceil = s1 > HI || s2 > HI;
    (floor, ceil, r2)
}

fn events_right(a: i64, b: i64, c: i64) -> (bool, bool, i64) {
    let s1 = b + c;
    let r1 = clamp(s1);
    let s2 = a + r1;
    let r2 = clamp(s2);
    let floor = s1 < LO || s2 < LO;
    let ceil = s1 > HI || s2 > HI;
    (floor, ceil, r2)
}

fn main() {
    let mut ok = true;

    let mut divergent = 0u64;
    let mut div_no_clamp = 0u64;
    let mut div_ceiling_only = 0u64;
    let mut div_floor_only = 0u64;
    let mut div_both = 0u64;
    let mut div_same_sign = 0u64;
    let mut witness_ceiling = false;
    let mut witness_floor = false;

    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                let (fl_l, ce_l, left) = events_left(a, b, c);
                let (fl_r, ce_r, right) = events_right(a, b, c);
                if left == right {
                    continue;
                }
                divergent += 1;
                let floor = fl_l || fl_r;
                let ceil = ce_l || ce_r;
                match (floor, ceil) {
                    (false, false) => div_no_clamp += 1,
                    (false, true) => div_ceiling_only += 1,
                    (true, false) => div_floor_only += 1,
                    (true, true) => div_both += 1,
                }
                let all_nonneg = a >= 0 && b >= 0 && c >= 0;
                let all_nonpos = a <= 0 && b <= 0 && c <= 0;
                if all_nonneg || all_nonpos {
                    div_same_sign += 1;
                }
                if (a, b, c) == (7, 7, -1) {
                    witness_ceiling = !floor && ceil;
                }
                if (a, b, c) == (-8, -8, 1) {
                    witness_floor = floor && !ceil;
                }
            }
        }
    }

    println!("divergent triples:            {}", divergent);
    println!("  with no clamp event:        {}", div_no_clamp);
    println!("  ceiling only:               {}", div_ceiling_only);
    println!("  floor only:                 {}", div_floor_only);
    println!("  both bounds:                {}", div_both);
    println!("  all-same-sign operands:     {}", div_same_sign);
    println!(
        "witness (7,7,-1) divergent, ceiling-only: {}",
        witness_ceiling
    );
    println!(
        "witness (-8,-8,1) divergent, floor-only:  {}",
        witness_floor
    );

    // 1. instrument: no divergence without a clamp event
    ok &= div_no_clamp == 0;
    // 2, 3. one-bound divergence exists on both sides, witnesses confirmed
    ok &= div_ceiling_only > 0 && witness_ceiling;
    ok &= div_floor_only > 0 && witness_floor;
    // 4. monotone trajectories never diverge
    ok &= div_same_sign == 0;
    // 5. consistency of the counts
    ok &= div_no_clamp + div_ceiling_only + div_floor_only + div_both == divergent;

    println!("{}", if ok { "P5 WORKS" } else { "P5 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
