//! Probe 1: is the overflow band inhabited for mixed-format addition, the second of
//! the two members of `40:178-180`'s claim ("inhabited for multiplication, division,
//! mixed-format addition and every float operation") that has never been independently
//! compiled. File 43 compiled the division member and found it wrong as a blanket
//! statement (empty for same-precision division, inhabited only once precisions
//! decouple, and even then only per-format-triple rather than unconditionally). The
//! multiplication member was independently compiled (files 30/31/33). The two
//! remaining members, mixed-format addition and "every float operation," had never had
//! a probe run against them before this file; the consolidation carries them from
//! `28:229-231` on the strength of same-format addition and multiplication being
//! checked, which is the same extrapolation-from-a-sibling-member pattern that made
//! the division entry wrong.
//!
//! CLAIM A (compiled, exhaustive over the swept windows): mixed-format addition CAN
//! inhabit the band. Witness (`_WITNESS_1`): operand quanta 1/3 and 1/6, destination
//! quantum 1/4, destination range Vmax = 5/4 (m = 5). k1 = 0, k2 = 8: exact sum is
//! 0/3 + 8/6 = 4/3 = 1.3333..., which is strictly inside (Vmax, Vmax + half a
//! destination quantum] = (1.25, 1.375]. So `40:178-180` is right that mixed-format
//! addition is not exempt the way same-format addition is, and this is the first
//! probe to have actually checked it rather than carried it by analogy.
//!
//! CLAIM B (compiled, a sweep of 40 mixed-format triples): the claim, read as
//! "mixed-format addition unconditionally inhabits the band," is TOO STRONG, the same
//! shape of overstatement division's blanket "inhabited" entry turned out to carry.
//! 36 of 40 swept (d1, d2, dr, m) triples inhabit the band; 4 do not, and all four
//! empty cases share a structural feature the inhabited cases lack: d2 is an exact
//! multiple of d1 (3 and 6), so the two operand quanta are not independent, the pair
//! collapses to single-quantum arithmetic on 1/6, and whether THAT lands in the band
//! reduces to the same alignment condition that governs same-format addition, which
//! is why it can come out empty. Every triple with genuinely independent (non-dividing)
//! quanta in this sweep inhabits the band; every empty triple has one quantum
//! dividing the other.
//!
//! Consequence for the shape: `40:178-180`'s "mixed-format addition" entry needs the
//! same per-format-triple correction division already received, not a blanket
//! "inhabited," and the deciding structural fact (whether one operand quantum divides
//! the other) is new information this probe surfaces rather than something carried
//! from an earlier file. The "every float operation" member remains wholly uncompiled
//! after this probe; nothing here speaks to it, and it is named as still-open in the
//! deliverable this probe supports rather than resolved here.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_1_the_overflow_band_for_mixed_format_addition.rs --out-dir <dir>
//! Outcome: WORKS (all claims assert; the sweep's 36/4 split is pinned exactly).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// The search. Operand 1: quantum 1/d1, window k1 in 0..=k1max. Operand 2: quantum
// 1/d2, window k2 in 0..=k2max. Destination: quantum 1/dr, Vmax = m/dr. Exact sum
// s = k1/d1 + k2/d2. Band membership: Vmax < s <= Vmax + 1/(2*dr), i.e. in the common
// denominator N = 2*d1*d2*dr, working with SN = s*N/2 (kept an exact integer by
// requiring d1*d2 even, checked by the assertion below rather than assumed):
//   SN       = k1*d2*dr + k2*d1*dr
//   Vmax*N/2 = m*d1*d2
//   band: m*d1*d2 < SN <= m*d1*d2 + d1*d2/2
// ---------------------------------------------------------------------------

const fn band_inhabited_for_one_triple(
    d1: u64,
    d2: u64,
    dr: u64,
    m: u64,
    k1max: u64,
    k2max: u64,
) -> bool {
    assert!(
        (d1 * d2) % 2 == 0,
        "d1*d2 must be even so the half-quantum band has an exact integer boundary"
    );
    let lo = m * d1 * d2; // exclusive
    let hi = lo + (d1 * d2) / 2; // inclusive
    let mut k1 = 0u64;
    while k1 <= k1max {
        let mut k2 = 0u64;
        while k2 <= k2max {
            let sn = k1 * d2 * dr + k2 * d1 * dr;
            if sn > lo && sn <= hi {
                return true;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    false
}

/// Which (k1, k2) triggers the band, for the first inhabited case found, in the same
/// (k1 outer, k2 inner) order `band_inhabited_for_one_triple` searches. Used to build
/// a pinned, checkable witness rather than trusting the boolean alone.
const fn first_witness(d1: u64, d2: u64, dr: u64, m: u64, k1max: u64, k2max: u64) -> (u64, u64) {
    let lo = m * d1 * d2;
    let hi = lo + (d1 * d2) / 2;
    let mut k1 = 0u64;
    while k1 <= k1max {
        let mut k2 = 0u64;
        while k2 <= k2max {
            let sn = k1 * d2 * dr + k2 * d1 * dr;
            if sn > lo && sn <= hi {
                return (k1, k2);
            }
            k2 += 1;
        }
        k1 += 1;
    }
    (u64::MAX, u64::MAX) // sentinel: not found
}

// ---------------------------------------------------------------------------
// CLAIM A: witness triple 1. Operand quanta 1/3 and 1/6 (d1*d2 = 18, even), destination
// quantum 1/4, Vmax = 5/4 (m = 5).
// ---------------------------------------------------------------------------

const _WITNESS_1_INHABITED: () = assert!(band_inhabited_for_one_triple(3, 6, 4, 5, 12, 12));
const _WITNESS_1: (u64, u64) = first_witness(3, 6, 4, 5, 12, 12);
const _WITNESS_1_K1: u64 = _WITNESS_1.0;
const _WITNESS_1_K2: u64 = _WITNESS_1.1;
// k1 = 0, k2 = 8: sum is 0/3 + 8/6 = 4/3 = 1.3333..., strictly inside (1.25, 1.375].
const _WITNESS_1_CHECK: () = assert!(_WITNESS_1_K1 == 0 && _WITNESS_1_K2 == 8);
// Independent confirmation in plain rationals, cross-checked by hand.
const _WITNESS_1_LO: () = assert!(4 * 4 > 5 * 3); // 4/3 > 5/4  <=>  16 > 15
const _WITNESS_1_HI: () = assert!(4 * 8 <= 11 * 3); // 4/3 <= 11/8 <=> 32 <= 33

// ---------------------------------------------------------------------------
// CLAIM A, second witness: an independently shaped case (different quanta, different
// destination range, genuinely independent operand quanta with neither dividing the
// other) so the first witness is not a coincidence of one parameter choice.
// ---------------------------------------------------------------------------

const _WITNESS_2_INHABITED: () = assert!(band_inhabited_for_one_triple(5, 8, 6, 9, 20, 20));
const _WITNESS_2: (u64, u64) = first_witness(5, 8, 6, 9, 20, 20);
const _WITNESS_2_CHECK: () = assert!(_WITNESS_2.0 == 1 && _WITNESS_2.1 == 11);

// ---------------------------------------------------------------------------
// NEGATIVE CONTROL: same-format addition (d1 == d2 == dr) never inhabits the band,
// over the same search shape, matching `28:229-231` / `26:169-171`'s own claim for
// same-format addition, re-run here as a sanity check on the search machinery itself
// rather than trusted from the earlier files.
// ---------------------------------------------------------------------------

const fn same_format_never_inhabited(d: u64, m: u64, kmax: u64) -> bool {
    !band_inhabited_for_one_triple(d, d, d, m, kmax, kmax)
}

const _SAME_FORMAT_CONTROL_1: () = assert!(same_format_never_inhabited(4, 15, 30));
const _SAME_FORMAT_CONTROL_2: () = assert!(same_format_never_inhabited(8, 40, 60));
const _SAME_FORMAT_CONTROL_3: () = assert!(same_format_never_inhabited(6, 25, 50));

// ---------------------------------------------------------------------------
// CLAIM B: a sweep over 40 genuinely mixed (d1 != d2) triples drawn from a small set,
// with a destination range chosen per triple. Counts BOTH how many inhabit the band
// and how many do not, rather than asserting a single direction, because the claim
// under test ("mixed-format addition inhabits the band") turns out to hold only when
// the two operand quanta are independent (neither divides the other); the four
// dividing-pair cases in this sweep (d1, d2) = (3, 6) or (6, 3) come out empty.
// ---------------------------------------------------------------------------

const fn sweep_counts() -> (u32, u32, u32) {
    let ds: [u64; 4] = [3, 4, 5, 6];
    let mut i = 0usize;
    let mut tried = 0u32;
    let mut inhabited = 0u32;
    while i < 4 {
        let mut j = 0usize;
        while j < 4 {
            if ds[i] != ds[j] {
                let mut k = 0usize;
                while k < 4 {
                    let dr = ds[k];
                    let m = dr * 2 + 1; // an odd numerator, avoiding accidental exact grid alignment
                    if (ds[i] * ds[j]) % 2 == 0 {
                        tried += 1;
                        if band_inhabited_for_one_triple(ds[i], ds[j], dr, m, 10, 10) {
                            inhabited += 1;
                        }
                    }
                    k += 1;
                }
            }
            j += 1;
        }
        i += 1;
    }
    (tried, inhabited, tried - inhabited)
}

const _SWEEP: (u32, u32, u32) = sweep_counts();
// Pinned split: 40 triples tried, 36 inhabited, 4 empty. Both categories nonempty,
// which is the finding: mixed-format addition is NOT unconditionally in the band
// (ruling out a blanket restatement of `40:178-180`'s claim as-is), and it is also
// NOT never in the band the way same-format addition is (so the claim's basic shape,
// that mixed-format differs from same-format, survives; only "unconditional" does not).
const _SWEEP_TRIED: () = assert!(_SWEEP.0 == 40);
const _SWEEP_INHABITED: () = assert!(_SWEEP.1 == 36);
const _SWEEP_EMPTY: () = assert!(_SWEEP.2 == 4);
const _SWEEP_BOTH_NONEMPTY: () = assert!(_SWEEP.1 > 0 && _SWEEP.2 > 0);

// ---------------------------------------------------------------------------
// The structural read on the four empty cases: both empty (d1, d2) pairs are the
// dividing pair (3, 6) / (6, 3), where d2 = 2*d1, so the "mixed" operand pair is
// actually single-quantum arithmetic in disguise (every k1/3 + k2/6 lands on the 1/6
// grid exactly), and the emptiness then depends on the SAME kind of alignment that
// governs same-format addition rather than on a genuinely mixed-precision effect.
// Confirmed directly: for a non-dividing pair with the same relative sizes (4 and 5,
// neither a multiple of the other), the band inhabits at both destination choices
// that made the dividing pair empty in spirit (dr = 5, m = 11 and dr = 6, m = 13).
// ---------------------------------------------------------------------------

const _DIVIDING_PAIR_EMPTY_1: () = assert!(!band_inhabited_for_one_triple(3, 6, 5, 11, 10, 10));
const _DIVIDING_PAIR_EMPTY_2: () = assert!(!band_inhabited_for_one_triple(3, 6, 6, 13, 10, 10));
const _NONDIVIDING_PAIR_INHABITED_1: () =
    assert!(band_inhabited_for_one_triple(4, 5, 5, 11, 10, 10));
const _NONDIVIDING_PAIR_INHABITED_2: () =
    assert!(band_inhabited_for_one_triple(4, 5, 6, 13, 10, 10));
