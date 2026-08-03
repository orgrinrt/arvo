# Probe outcomes, file 43

All probes compiled against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
confirmed with `rustc --version` from inside the repo, each with
`rustc --edition 2021 --crate-type lib <file> --out-dir <dir>`. Every load-bearing number below was
computed independently in Python (math.lcm, fractions-style exact integer arithmetic) BEFORE the
const fn recomputing it was written, per the review's own discipline against the construction and
the check sharing an author's blind spot; the Python values are recorded in this file and the const
assertions pin the Rust recomputation against them. `vu_nat_sealed.rs` and `vu_bias_sealed.rs` are
unmodified copies of `42_probes/`' files of the same name (the sealed tower, file 42's fix), the
same copy-forward precedent file 42 used against file 41.

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_the_prediction_tested.rs` | Does the consolidation's prediction (`26:678-681`, carried into `40:651-655`: "no finite accumulator solution exists for division at all") survive being computed rather than read? | WORKS, and the prediction splits. CLAIM A: in the dyadic coordinates the prediction was written in, it is TRUE (2^F mod 3 is never 0, F exhaustive to 1000: no dyadic grid at any width holds 1/3). CLAIM B: in the RATIFIED coordinates (rational adjustment, files 28/36/41) it is FALSE: the numeral with adjustment (A1/A2)/lcm(1..K) holds every quotient exactly, checked exhaustively at p=3 (K=7, L=420) and p=4 (K=15, L=360360). CLAIM C: lcm is the least denominator (exhaustive at p=3: no b < 420 divisible by every d <= 7). CLAIM D: the accumulator's width is Theta(2^p) bits: 5, 12, 23, 51, 95, 190, 370 bits for p = 2..8 (bits of K*lcm(1..K)); p <= 6 in u128, p = 7, 8 via a [u64; 8] const bignum from prime powers, cross-checked against Python math.lcm (lcm bit lengths 183 and 362) and against the u128 path at p = 6. |
| `probe_2_the_overflow_band_for_division.rs` | Is the overflow band (where round-first and classify-first disagree) inhabited for division, as `40:178-180` states? | WORKS, and the sentence is wrong for division. CLAIM A: for unsigned SAME-PRECISION division, any dyadic scales, the band is EMPTY for every p in 2..=8 (exhaustive over the reduced membership condition, all shifts m <= 2p+2; the m <= p+1 half also has a residue proof: 2Kk2 = -2k2 mod 2^m, which can never reach within k2 of the next multiple). The multiplication contrast in the same probe: same-precision products DO inhabit it (11/8 * 11/8 = 121/64 at p=4, F=3). Division patterns with ADDITION on this axis, not multiplication. CLAIM B: the band IS inhabited once operand and result precisions decouple: 196/13 = 15.077 into the p=4, F=0 numeral lies strictly inside (15, 15.5), round-first delivers in-range 15 while classify-first declares overflow; and 31/2 = 15.5 pins the tie (RNE to 16, past the range: round-first overflow by the tie rule). |
| `probe_3_exact_division_by_a_constant.rs` | Does the design's own machinery carry the exact division subfamily (file 28's divide-by-radix-power, `28:329-336`) without new mechanism? | WORKS, and the subfamily is larger than file 28 named: division by ANY fixed nonzero representable constant. The numeral map is adjustment A1*(cd/cn), bias B1*(cd/cn), both via `PMul` + `Reduce` from the sealed tower, at concrete types (the one shape the toolchain accepts per files 41/42), zero new mechanism. Witnesses (all pre-checked in Python fractions): 3/4 / 4 = 3/16 (radix-power case, exponent shift); 3/4 / (3/2) = 6/12 -> 1/2 (renormalisation); 2/3 / (2/3) = 1/1; bias 1/2 / (3/2) = 1/3. Totality by construction: the constant's numerator is `Pos`, no zero to name (`41:132-141`'s induction). |
| `probe_4_the_euclidean_pair.rs` | Does the multiplicative relocation (file 24) transfer to division, and what is division's finite exact carrier? | WORKS. The carrier is the Euclidean pair (q, r), not a numeral. Exhaustive at mixed quanta A1 = 1/4, A2 = 1/3 (remainder grid gcd = 1/12): a = q*b + r with 0 <= r < b for all 240 pairs; the observed quotient bound equals the identity-axis formula floor(maxV1/minposV2) = 11 exactly; observed remainders < 60 units as predicted. Correct rounding onto the result grid is a function of the pair: the scaled-remainder RNE agrees with a never-dividing argmin-by-cross-multiplication oracle on all pairs. Double-rounding control: rounding via the 1/12 grid first then the 1/4 grid diverges from rounding once on 29 of 240 pairs (Python cross-check: 29, first divergent pair k1=1, k2=2: once=2, twice=1). |
| `probe_5_the_roundtrip_law_and_its_view.rs` | What does a law over division claim under the finest-view lattice? | WORKS. The round-trip law div(mul_full(a, b), b) = a, exhaustive at p=4, F=2: values agree wherever both sides are defined (all 240 defined pairs), definedness disagrees at exactly the 16 zero-divisor pairs, and the event components never agree (1 against 0). Finest view = the weak-equation corner (collapse events and definedness, keep values). No division-specific extension to the lattice vocabulary was needed. One reading assumed and marked in the header: events count per quantiser application, the type-level over-approximating reading `40:279-287`/`40:308-312` commits to. |

## Python pre-computations (run before the Rust was written)

- `math.lcm(1..K)` bit lengths: K=3: 3, K=7: 9 (L=420), K=15: 19 (L=360360), K=31: 47, K=63: 89,
  K=127: 183, K=255: 362. Accumulator (K*L) bit lengths: 5, 12, 23, 51, 95, 190, 370.
- Same-precision band search (all p <= 9, all shifts): EMPTY every p. Decoupled-precision
  witnesses: (k1=31, k2=1, m=0) is the tie at any po >= 5 into pr = 4; 196/13 (po = 8) is a strict
  interior point.
- Double-rounding divergence count at the probe-4 model: 29 of 240, first at (k1=1, k2=2).
- Probe 3 witnesses: Fraction(3,4)/4 = 3/16; Fraction(3,4)/Fraction(3,2) = 1/2;
  Fraction(2,3)/Fraction(2,3) = 1; Fraction(1,2)/Fraction(3,2) = 1/3.

## Scope stated honestly

No timing was measured anywhere in this directory; every number is a const-eval count, an exact
integer computation, or a bit length, per `bench-and-sketch-discipline.md` (a runtime cost claim
for a shipped divider belongs in `mock/benches/` and none is made here). The signed variants of
probes 2, 4 and 5 are not built; the unsigned results are exhaustive and the signed extensions are
reasoned only (flagged in the deliverable). The exhaustive sweeps stay at model widths by design:
the transfer argument to real widths is the forbidden-feature one the consolidation already
records (`40:30-32`).
