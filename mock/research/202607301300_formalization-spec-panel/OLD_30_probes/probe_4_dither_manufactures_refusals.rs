//! Probe 4: file 29's dither entry point and the `Refuse` resolution do not
//! compose, and the failure lands on an exactly representable input.
//!
//! File 29 proposes `quantize_dithered(exact, noise) = quantize(exact + noise)`
//! and says of the range ends only that "where it pushes a value past the top
//! the ordinary `OverRange` resolution takes over exactly as it would for any
//! other value that landed there" (`29:96-97`). For every resolution but one
//! that is fine. For `Refuse`, which is `Precise`'s own out-of-range
//! resolution (`202607301200:250-257`), it means the caller's choice to
//! dither can turn a total computation into a refusing one, including on an
//! input the numeral represents exactly. Nothing in either file says so.
//!
//! Model: unsigned, quantum 2 in scaled units, representable values 0, 2, ...,
//! 30, so the top is exactly representable. Nearest-ties-even in range,
//! `Refuse` out of range, which is `Precise`. Dither amplitude one quantum,
//! the rectangular-PDF amplitude file 29's own section 3 names.
//!
//! Every claim is a `const` assertion; compiling is the check.

#![no_std]

const Q: i32 = 2;
const TOP: i32 = 30;
const BOTTOM: i32 = 0;

/// The sentinel for `Refuse`. In the real design this is the refusing branch
/// of `Quantisation::Fallibility<T>`; here it is a value the assertions can
/// name.
const REFUSED: i32 = i32::MIN;

const fn round_nearest_even(x: i32) -> i32 {
    let down = x.div_euclid(Q) * Q;
    let rem = x - down;
    if rem * 2 < Q {
        down
    } else if rem * 2 > Q {
        down + Q
    } else if (down / Q) % 2 == 0 {
        down
    } else {
        down + Q
    }
}

/// `Precise`: round first per file 28's amendment, then classify the rounded
/// result, and refuse out of range.
const fn quantize_precise(exact: i32) -> i32 {
    let r = round_nearest_even(exact);
    if r > TOP || r < BOTTOM {
        REFUSED
    } else {
        r
    }
}

/// File 29's entry point, verbatim: same rounder, one extra input.
const fn quantize_dithered(exact: i32, noise: i32) -> i32 {
    quantize_precise(exact + noise)
}

// ---- the finding -----------------------------------------------------------

/// The top of the range is exactly representable, so the undithered path is
/// total there and returns the input unchanged.
const _: () = assert!(quantize_precise(TOP) == TOP);

/// One quantum of positive dither on that same exactly representable input
/// refuses. The caller did not ask for a fallible computation; it asked for
/// a decorrelated one.
const _: () = assert!(quantize_dithered(TOP, Q) == REFUSED);

/// The band where this can happen is not a corner. With dither amplitude one
/// quantum, every exact value within one quantum of either end can refuse for
/// some admissible noise draw, so the affected fraction of a `UFixed<4,0>`'s
/// range is two of its sixteen steps, and it grows as the numeral narrows.
const fn refusable_count(amplitude: i32) -> i32 {
    let mut n = 0;
    let mut x = BOTTOM;
    while x <= TOP {
        let mut noise = -amplitude;
        let mut any = false;
        while noise <= amplitude {
            if quantize_dithered(x, noise) == REFUSED {
                any = true;
            }
            noise += 1;
        }
        if any {
            n += 1;
        }
        x += 1;
    }
    n
}

const _: () = assert!(refusable_count(0) == 0); // no dither, no new refusals
const _: () = assert!(refusable_count(Q) > 0);

/// And the same input is fine under every non-refusing resolution, which is
/// why the interaction is specific to `Precise` rather than to dither.
const fn quantize_clamping(exact: i32) -> i32 {
    let r = round_nearest_even(exact);
    if r > TOP {
        TOP
    } else if r < BOTTOM {
        BOTTOM
    } else {
        r
    }
}
const _: () = assert!(quantize_clamping(TOP + Q) == TOP);

// ---- the candidate fix, checked --------------------------------------------
//
// Confine the perturbed value to the numeral's own range before quantising,
// which is what a real converter's input stage does and what keeps the
// dithered path exactly as fallible as the undithered one. It costs the
// dither its uniformity within one quantum of each end, which is a real and
// well-known cost in the field, not a free repair.

const fn quantize_dithered_confined(exact: i32, noise: i32) -> i32 {
    let perturbed = exact + noise;
    let confined = if perturbed > TOP {
        TOP
    } else if perturbed < BOTTOM {
        BOTTOM
    } else {
        perturbed
    };
    quantize_precise(confined)
}

/// Totality is restored: over the whole representable range and every
/// admissible noise draw at one quantum of amplitude, the confined path
/// refuses exactly where the undithered path refuses, which is nowhere.
const fn confined_never_adds_a_refusal(amplitude: i32) -> bool {
    let mut x = BOTTOM;
    while x <= TOP {
        let mut noise = -amplitude;
        while noise <= amplitude {
            let undithered = quantize_precise(x);
            let confined = quantize_dithered_confined(x, noise);
            if (confined == REFUSED) != (undithered == REFUSED) {
                return false;
            }
            noise += 1;
        }
        x += 1;
    }
    true
}

const _: () = assert!(confined_never_adds_a_refusal(Q));
const _: () = assert!(confined_never_adds_a_refusal(2 * Q));

/// And the confinement does not silently disarm the dither in the interior,
/// where the whole point of the mechanism lives. File 29's banding fact
/// restated for a ties-to-even rule: the undithered error is periodic in the
/// exact value, so two inputs a period apart receive the identical error,
/// and two different noise draws on one input do not.
///
/// The period is `2Q` rather than `Q` for a ties-to-even rule, because the
/// tie is broken on the quotient's parity and not on the residue alone. That
/// is a correction to file 29's own statement of the mechanism, which names
/// the residue class modulo the quantum (`29:69-72`); its own probe used
/// nearest-away-from-zero, where the period is `Q`.
const fn error(x: i32, noise: i32) -> i32 {
    quantize_dithered_confined(x, noise) - x
}

const _: () = assert!(error(9, 0) == -1);
const _: () = assert!(error(13, 0) == -1); // one period (2Q) away, identical error
const _: () = assert!(error(11, 0) == 1); // half a period away, and it differs
const _: () = assert!(error(9, 1) == 1); // one input, two noise draws, two errors

// ---- what this does NOT show ----------------------------------------------
//
// It does not decide between confinement and the alternative (offering the
// dithered entry point only where the out-of-range resolution is total), and
// it does not quantify what confinement costs the decorrelation guarantee
// near the ends, which is a question for the literature file 29 cites rather
// than for a const assertion.
