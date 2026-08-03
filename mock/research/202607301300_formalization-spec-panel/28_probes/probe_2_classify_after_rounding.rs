//! Probe 2: the five positions must be assigned to the rounded result, not
//! the exact one.
//!
//! The spec (`202607301200_topic.the-formalization-spec.md:130-134`) sorts an
//! exact value into five positions and applies `OverRange` when the value is
//! past the top. IEEE 754 defines overflow the other way round: the result is
//! first rounded "as though the exponent range were unbounded", and overflow
//! is signalled only if THAT result exceeds the largest finite number
//! (754-2019, 7.4). SystemC (IEEE 1666, sc_dt) and MATLAB fi likewise apply
//! the quantization mode first and the overflow mode to its output.
//!
//! The two orderings disagree on a band of exact values: past the largest
//! representable but within half a quantum of it. There the exact value is
//! "past the top" positionally, yet every one of the three test standards
//! rounds it back to the largest finite with NO overflow event.
//!
//! Model: quantum 4, grid {0, 4, ..., 28}, top 28, wrap span 32, exact values
//! as integers so the band (28, 30) is inhabited. Round-to-nearest, ties to
//! even grid index, computed on the unbounded grid.
//!
//! Confirmed by compiling:
//!   - Refuse: classify-first refuses at v = 29; round-first returns 28.
//!     IEEE's trap-on-overflow does not trap there; the spec as written does.
//!   - Clamp: the two orderings agree at every value (checked exhaustively),
//!     which is why the defect is invisible in the shipped presets (Warm and
//!     Cold clamp).
//!   - ReduceModulo: classify-first is not even well-defined on the band. The
//!     reduction of v = 29 modulo the span is 29, still past the top, so the
//!     resolution hands back a value in the situation it was meant to
//!     resolve. Round-first has no such case: reduction applies to a grid
//!     value and always lands on the grid in range.
//!
//! For same-format addition the band is empty (exact sums are on-grid), which
//! is consistent with the consolidation's finding that addition's recovery
//! map is a partial identity. The band is inhabited exactly when the exact
//! result carries sub-quantum precision: multiplication, division,
//! mixed-format addition, and every float operation. The ordering defect is
//! therefore invisible to the additive half and live for everything else.

#![no_std]

const Q: u32 = 4;
const TOP: u32 = 28;
const SPAN: u32 = 32;
const REFUSED: u32 = u32::MAX;

/// Round to nearest grid multiple of `Q`, ties to even grid index, on the
/// unbounded grid (no range concern here at all).
const fn rne_unbounded(v: u32) -> u32 {
    let down = (v / Q) * Q;
    let rem = v - down;
    let half = Q / 2;
    if rem < half {
        down
    } else if rem > half {
        down + Q
    } else if (down / Q) % 2 == 0 {
        down
    } else {
        down + Q
    }
}

// The spec as written: position the EXACT value, then resolve.
const fn classify_first_refuse(v: u32) -> u32 {
    if v > TOP {
        REFUSED
    } else {
        rne_unbounded(v)
    }
}

// IEEE 754 7.4 / SystemC / MATLAB: round on the unbounded grid, then
// position the ROUNDED value.
const fn round_first_refuse(v: u32) -> u32 {
    let r = rne_unbounded(v);
    if r > TOP {
        REFUSED
    } else {
        r
    }
}

// The band: exact past the top, rounded within.
const _: () = assert!(classify_first_refuse(29) == REFUSED);
const _: () = assert!(round_first_refuse(29) == 28);

// Past the band the orderings agree again (30 ties up to 32, which is out).
const _: () = assert!(classify_first_refuse(30) == REFUSED);
const _: () = assert!(round_first_refuse(30) == REFUSED);

// Clamp hides the divergence at every exact value: checked exhaustively over
// three spans.
const fn clamp_orderings_agree() -> bool {
    let mut v = 0;
    while v <= 3 * SPAN {
        let cf = if v > TOP { TOP } else { rne_unbounded(v) };
        let r = rne_unbounded(v);
        let rf = if r > TOP { TOP } else { r };
        if cf != rf {
            return false;
        }
        v += 1;
    }
    true
}
const _: () = assert!(clamp_orderings_agree());

// ReduceModulo under classify-first: the reduction of a band value lands
// back in the band. The resolution's output is in the situation the
// resolution was invoked to resolve, so classify-first wrap needs a second
// rule the spec does not state. Round-first needs none.
const _: () = assert!(29 % SPAN > TOP);
const _: () = assert!(rne_unbounded(30) % SPAN == 0); // round-first: 30 -> 32 -> wraps to 0, well-defined
