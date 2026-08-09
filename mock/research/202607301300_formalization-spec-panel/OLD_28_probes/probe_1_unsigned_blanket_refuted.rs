//! Probe 1: the spec's unsigned faithfulness blanket, checked exhaustively.
//!
//! `202607301200_topic.the-formalization-spec.md:210-216` derives:
//!
//! ```text
//! // unsigned addition can only leave the range above, so one end is
//! // unreachable and the rule is truncated addition whatever it does there
//! impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}
//! ```
//!
//! The quantifier is over every `Resolution`, which includes `SubstituteZero`
//! (SystemC's `SC_SAT_ZERO`, the reason the marker is in the vocabulary at
//! all). This probe checks the claim at a 5-bit model, exhaustively, for the
//! four out-of-range resolutions an unsigned addition can carry at its one
//! reachable end.
//!
//! Model: quantum 1, range 0..=31, same-format operands, so the exact sum is
//! always on-grid and the in-range direction never fires. The out-of-range
//! resolution is the entire behaviour, which is the cleanest possible test of
//! the blanket.
//!
//! Expected and confirmed by compiling:
//!   ReduceModulo    -> associative        (homomorphism onto Z/32Z)
//!   clamp           -> associative        (absorption: min(min(a+b,M)+c, M) = min(a+b+c, M), needs c >= 0)
//!   Refuse (Kleene) -> associative        (defined iff a+b+c <= M on both groupings)
//!   SubstituteZero  -> NOT associative    (witness: (25 + 10) + 5 = 5, 25 + (10 + 5) = 0)
//!
//! So the blanket is false in one cell and the "whatever it does there"
//! reasoning is wrong: the reachable end's rule matters, and the correct
//! condition is per-resolution, not per-signedness.

#![no_std]

const M: u32 = 31;
const SPAN: u32 = 32;
const REFUSED: u32 = u32::MAX;

const fn add_wrap(a: u32, b: u32) -> u32 {
    (a + b) % SPAN
}

const fn add_clamp(a: u32, b: u32) -> u32 {
    let s = a + b;
    if s > M {
        M
    } else {
        s
    }
}

const fn add_zero(a: u32, b: u32) -> u32 {
    let s = a + b;
    if s > M {
        0
    } else {
        s
    }
}

const fn add_refuse(a: u32, b: u32) -> u32 {
    // Kleene composition: a refusal on either side propagates, and two
    // refusals count as agreement.
    if a == REFUSED || b == REFUSED {
        return REFUSED;
    }
    let s = a + b;
    if s > M {
        REFUSED
    } else {
        s
    }
}

const fn apply(sel: u8, a: u32, b: u32) -> u32 {
    match sel {
        0 => add_wrap(a, b),
        1 => add_clamp(a, b),
        2 => add_zero(a, b),
        _ => add_refuse(a, b),
    }
}

const fn associative(sel: u8) -> bool {
    let mut a = 0;
    while a < SPAN {
        let mut b = 0;
        while b < SPAN {
            let mut c = 0;
            while c < SPAN {
                let l = apply(sel, apply(sel, a, b), c);
                let r = apply(sel, a, apply(sel, b, c));
                if l != r {
                    return false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

const _: () = assert!(associative(0)); // ReduceModulo
const _: () = assert!(associative(1)); // clamp (TowardNegative at OverRange)
const _: () = assert!(associative(3)); // Refuse, under Kleene equality
const _: () = assert!(!associative(2)); // SubstituteZero: the blanket's false cell

// The witness, spelled out so the deliverable can quote it.
const _: () = assert!(add_zero(add_zero(25, 10), 5) == 5);
const _: () = assert!(add_zero(25, add_zero(10, 5)) == 0);
