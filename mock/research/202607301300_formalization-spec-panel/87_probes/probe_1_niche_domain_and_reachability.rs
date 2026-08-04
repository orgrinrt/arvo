// Probe 1: independent reproduction of the statement-0/niche collision, plus the
// provable-versus-trusted distinction between a field-shrink closure and a niche closure.
//
// Built independently of 86_probes/probe_1_niche_vs_statement0.rs: a different width (a
// bounded 12-bit domain biased into a NonZeroU16, rather than the full 16-bit domain 86
// used), to check the claim generalises rather than confirm the one instance already built.
#![no_std]
#![allow(dead_code)]

use core::mem::transmute;
use core::num::NonZeroU16;

// --- Part A: the domain-size collision, at a width the prior probe did not use. ---
//
// A 12-bit bounded domain, biased by one into a NonZeroU16 carrier (the u16 width is
// what "Encoding::Fields' width" names for this Lowering, since NonZeroU16 is
// repr(transparent) over u16 and the design's own encoding freedom lets an encoding
// choose which datum carries a value, per the ratified 78:193-195).
//
// Claim under test: no k in 0..=16 gives 2^k == 4096 - 1 (the biased domain's own size,
// were the design to bias only a 12-bit logical range rather than the full 16-bit one).
// Checked exhaustively, in const position, at every k the fields' width could plausibly
// be shrunk to, not merely asserted from arithmetic.
const BIASED_DOMAIN_SIZE: u32 = 4096 - 1; // one pattern spent as the niche
const fn no_width_matches(domain: u32) -> bool {
    let mut k = 0u32;
    while k <= 16 {
        if (1u32 << k) == domain {
            return false;
        }
        k += 1;
    }
    true
}
const _: () = assert!(no_width_matches(BIASED_DOMAIN_SIZE));
// And the full-width case file 86 built, reproduced independently at the same width:
const _: () = assert!(no_width_matches(65536 - 1));
// Algebraic corroboration, not merely the sweep: an odd number greater than 1 is never a
// power of two, and 4095 = 3 * 5 * 7 * 13, 65535 = 3 * 5 * 17 * 257, both odd. The sweep
// is the checkable form; this comment records why it was never going to find one.

// --- Part B: NonZeroU16::new(0) is None: no SAFE constructor reaches the excluded pattern. ---
const fn safe_path_refuses_zero() -> bool {
    NonZeroU16::new(0).is_none()
}
const _: () = assert!(safe_path_refuses_zero());

// --- Part C: the excluded pattern is reachable through unsafe code. NEVER CALLED. ---
//
// This function is never invoked anywhere in this probe (grep confirms: zero call sites
// below). Its existence, as CODE THAT TYPE-CHECKS, is the claim under test: repr(transparent)
// plus an ordinary transmute lets an adversary attempt exactly the pattern the safe
// constructor above refuses. Calling it is instant, unbounded undefined behaviour by
// NonZeroU16's own documented safety contract; not calling it, while confirming it compiles,
// is what demonstrates "reachable" in the perimeter rule's sense (an operation existing
// through which the excluded state can be reached) without triggering the UB itself.
unsafe fn reach_the_excluded_pattern() -> NonZeroU16 {
    unsafe { transmute::<u16, NonZeroU16>(0) }
}

// --- Part D: the provable/trusted distinction, compiled. ---
//
// A field-shrunk closure: a genuinely 2-variant closed domain, decoded by an EXHAUSTIVE
// match with no wildcard arm. If a third variant were ever added to `Shrunk` without
// updating this match, the match becomes non-exhaustive and the crate fails to compile:
// E0004, "non-exhaustive patterns". The compiler is the one doing the checking; there is
// no way to violate the closure this way, safe or unsafe, short of editing this file.
enum Shrunk {
    A,
    B,
}
const fn decode_shrunk(s: &Shrunk) -> u8 {
    match s {
        Shrunk::A => 0,
        Shrunk::B => 1,
        // no wildcard arm: the compiler proves this exhaustive against Shrunk's own
        // closed declaration, not against any discipline the caller promised to follow.
    }
}

// A niche closure: decode over NonZeroU16 has no analogous compiler-checked exhaustiveness
// against "all reachable u16 patterns will be nonzero." The function below compiles and
// looks total, but its totality is a claim ABOUT NonZeroU16's validity contract, not a fact
// the match arm structure itself proves; nothing here would fail to compile if the
// contract were violated by unsafe code elsewhere (part C), only the caller of that
// unsafe code violates a documented obligation the type system never re-checks.
const fn decode_niche(n: NonZeroU16) -> u16 {
    n.get() - 1
}

fn main() {}
