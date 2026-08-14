// v3. The entropy hole: which members of the rounding family survive I14.
//
// THE HOLE
// --------
// Every stochastic member needs a source of randomness. arvo is `#![no_std]`,
// no `alloc`, no platform dependency, and I15 catches invalids at compile time.
// `128` section 5 closes its family map with "what no member escapes: entropy at
// runtime", and `129` reports that the position-keyed deterministic dither needs
// no runtime draw at all. Neither checks the position-keyed scheme against the
// constraints, and the dispatch asks whether it is the only member that survives
// them.
//
// WHAT THE CONSTRAINTS ACTUALLY FORBID, READ RATHER THAN ASSUMED
// ---------------------------------------------------------------
// I14 forbids `std::time`, `std::fs`, `std::net`, `std::thread`. Those are the
// only sources of entropy a library can reach on its own. So the question is not
// whether a draw is expensive; it is whether arvo can produce one at all, and
// the answer is that it cannot. A stochastic member's randomness must therefore
// be supplied from outside arvo.
//
// And that is where I13's const instruction bites, at `INTENTS.md:252`: the
// admissible category is whatever is available at const time, which reaches
// const data from outside the typestate. A seed supplied at const time is
// available. But a const seed makes the scheme a fixed function, which is a
// deterministic member of the family rather than a stochastic one.
//
// So the disjunction is sharp and this file checks the half that can be checked
// by compiling it: a position-keyed dither is integer arithmetic on the position,
// needs no entropy, and is const-evaluable.
//
// PREDICTIONS, RECORDED BEFORE COMPILING
// --------------------------------------
// P1. The position-keyed threshold is computable in a `const fn` with integer
//     arithmetic only: no float, no std, no allocation, no feature gate.
// P2. Its decisions are available at compile time, so a rounding decision keyed
//     on a compile-time-known position is a const, not a runtime branch.
// P3. It decorrelates: consecutive positions do not share a threshold, so a
//     repeated input value does not receive a repeated decision.
// P4. It is NOT monotone in position for a fixed value, which is the price
//     `129` measures and which this file confirms rather than assumes.
//
// CONTROLS
// --------
// C1. A degenerate keying that returns the same threshold at every position must
//     show zero distinct decisions on the repeated-input test, or the
//     decorrelation count is not measuring decorrelation.
// C2. The const-evaluation claim is checked by using the values in a `const`
//     item, which fails to compile if any of it is not const-evaluable.

#![no_std]
#![no_main]

// The golden ratio in Q32: floor(2^32 / phi) = floor(2^32 * 0.6180339887...).
// An odd constant, so the additive sequence visits every residue before
// repeating, which is what makes it low-discrepancy rather than merely periodic.
const GOLDEN_Q32: u64 = 2_654_435_769;

/// The threshold at a position, as a Q32 fraction. Pure integer arithmetic on
/// the position: no entropy, no float, no platform call.
const fn threshold_q32(position: u64) -> u64 {
    position.wrapping_mul(GOLDEN_Q32) & 0xFFFF_FFFF
}

/// C1's degenerate keying: the same threshold everywhere.
const fn threshold_degenerate(_position: u64) -> u64 {
    1 << 31
}

/// The rounding decision for a value whose fractional part is `frac_q32`, at a
/// given position. Rounds up when the fraction exceeds the position's threshold.
const fn rounds_up(frac_q32: u64, position: u64) -> bool {
    frac_q32 > threshold_q32(position)
}

const fn rounds_up_degenerate(frac_q32: u64, position: u64) -> bool {
    frac_q32 > threshold_degenerate(position)
}

/// P3 and C1: how many distinct decisions does one repeated value receive across
/// `n` consecutive positions? Counted at const time.
const fn distinct_decisions(frac_q32: u64, n: u64, degenerate: bool) -> u32 {
    let mut i = 0;
    let mut saw_true = false;
    let mut saw_false = false;
    while i < n {
        let up = if degenerate {
            rounds_up_degenerate(frac_q32, i)
        } else {
            rounds_up(frac_q32, i)
        };
        if up {
            saw_true = true;
        } else {
            saw_false = true;
        }
        i += 1;
    }
    (saw_true as u32) + (saw_false as u32)
}

/// P4: monotonicity in position for a value that varies with position. Counts
/// adjacent pairs where a larger value receives a smaller rounded result.
const fn monotonicity_violations(n: u64) -> u32 {
    let mut i = 1;
    let mut bad = 0;
    while i < n {
        // A ramp: the value at position i has fractional part i/n in Q32.
        let f_prev = ((i - 1) << 32) / n;
        let f_here = (i << 32) / n;
        let up_prev = rounds_up(f_prev, i - 1);
        let up_here = rounds_up(f_here, i);
        // The integer part is equal along the ramp, so a violation is exactly
        // the earlier point rounding up while the later one rounds down.
        if up_prev && !up_here {
            bad += 1;
        }
        i += 1;
    }
    bad
}

// ------------------------------------------------------------------ C2 and P2
// Every value below is a `const`. If any expression above were not
// const-evaluable this file would not compile, which is the check.

/// A tie, the worst case for banding: exactly half a quantum.
const HALF: u64 = 1 << 31;

pub const DISTINCT_AT_40: u32 = distinct_decisions(HALF, 40, false);
pub const DISTINCT_DEGENERATE_AT_40: u32 = distinct_decisions(HALF, 40, true);
pub const DISTINCT_AT_256: u32 = distinct_decisions(HALF, 256, false);
pub const VIOLATIONS_AT_40: u32 = monotonicity_violations(40);
pub const VIOLATIONS_AT_256: u32 = monotonicity_violations(256);

/// P2: a decision at a compile-time-known position is itself a const, so it can
/// gate an arm rather than branch at runtime.
pub const DECISION_AT_POSITION_7: bool = rounds_up(HALF, 7);
pub const DECISION_AT_POSITION_8: bool = rounds_up(HALF, 8);

// The assertions are the finding, and they are checked at compile time. A wrong
// one is a build failure rather than a printed number nobody reads.
const _: () = assert!(DISTINCT_AT_40 == 2, "P3: a repeated value must receive both decisions");
const _: () = assert!(DISTINCT_AT_256 == 2, "P3 at a longer pass");
const _: () = assert!(
    DISTINCT_DEGENERATE_AT_40 == 1,
    "C1: a degenerate keying must decorrelate nothing, or the count means nothing"
);
const _: () = assert!(
    VIOLATIONS_AT_40 > 0,
    "P4: the position keying must cost monotonicity, or it is not paying the price 129 measures"
);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
