# Probe outcomes, file 99

Rust probes compiled against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`aarch64-apple-darwin`, confirmed with `rustc --version` from inside the repo, each with
`rustc --edition 2021 --crate-type lib <file> --out-dir <dir>` (probe 2 additionally with `-O
--emit asm` for the emitted-code findings). Every load-bearing count was computed independently in
Python (`precompute.py`, exact integer arithmetic, `math.isqrt`, integer powers, the `decimal`
module at 60 digits) BEFORE the Rust recomputing it was written, per the file-43 discipline; the
Python values are recorded here and the Rust const assertions pin the recomputation against them.
Python probes 3 and 4 are exact-integer and 60-digit-decimal computations respectively; no float
enters any load-bearing comparison (floats only locate search windows in probe 3).

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_the_root_residue_carrier.rs` | Does the carrier-kind move (file 43) extend to `Sqrt`, and what does the root's exact carrier cost? | WORKS, compiled: every claim is exhaustive over the full index set at each (P, F) and evaluated in const position, so compilation is the verification. CLAIM A: correct rounding is a one-comparison function of the pair (m, r) with m = isqrt(k << F), r = (k << F) - m^2: round up iff r > m; agrees with a never-rooting argmin oracle (cross-multiplied squares) at (4,4), (6,6), (8,8), (8,4), exhaustively. CLAIM B: nearest-rounding ties are IMPOSSIBLE (4r = 4m + 1, even = odd); zero ties observed at all nine (P, F) sweeps, matching the two-line parity proof. CLAIM C: the same-numeral overflow band is empty exactly when M >= 2^F - 1 (the far point is at least 1 - q) and inhabited otherwise: 0 overflows at (2,2), (3,3), (4,2), (4,4), (6,6), (8,4), (8,8); 3 of 4 nonzero operands overflow at (2,4) and 7 at (3,6), the numerals whose value set contains no 1. CLAIM C': no nonzero operand ever rounds to zero, every sweep. CLAIM D: the carrier is linear-width: the widest integer the whole decision touches is the scaled operand itself, P + F bits (16 at (8,8)); max residue 508 at (8,8), 28 at (4,4), 124 at (6,6). CLAIM E: the cube root has the identical carrier shape (residue against (2m+1)^3, tie parity-impossible): counts (2, 7, 0) at (4,2) and (4, 30, 0) at (6,3), pinned. |
| `probe_2_sqrt_in_const_position.rs` | Does the correctly-rounded sqrt exist in the fourth rule's required const form, and what does each position emit? | WORKS. Const position: `sqrt_rn(512, 8)` evaluates in a `const` item and the const-position consumer lowers to `mov w0, #362; ret` under `-O` on `aarch64-apple-darwin` (asm inspected, function `_sqrt_rn_const_position`). Value position: the runtime consumer emits a 14-instruction body: a Newton isqrt loop with one `udiv` per iteration, then the entire rounding decision is `msub, cmp, cinc`, three instructions, branchless. Four const pins: sqrt(0.5) -> 181/256, sqrt(2) -> 362/256, sqrt(255/256) -> 255/256 (inside the far point by under 2^-14 of an index), sqrt(3/2^24) -> 7094/2^24. |
| `probe_3_exp2_exact_hits.py` | Is the radix-power exponential decidable, and at what carrier width? | WORKS. CLAIM A: exact grid hits of 2^(k/2^F) occur exactly at integer exponents, exhaustive at F = 1..4 over the stated ranges (hit sets: k in multiples of 2^F only, expected counts matched). CLAIM B: rounding ties never occur (0 at every sweep). CLAIM C: the exact comparison object j^(2^F) doubles in width per fractional bit: max 13, 25, 57, 113 bits at F = 1, 2, 3, 4 (uniform-range series 17, 33, 65, 129). Decidable, and exponential-carrier: division's class over a divisor numeral, not sqrt's linear class. |
| `probe_4_the_hardness_const.py` | Does the transcendental hardness const exist, and does it follow a formula? | WORKS. exp over three model numerals, 60-digit decimal, distance of exp(k/2^F) * 2^F from the nearest rounding boundary, full value set each: zero ties anywhere (the model-width shadow of Lindemann-Weierstrass). Hardness: P=F=8 needs 11 extra bits (hardest operand k = 112, distance 5.565e-4); P=F=6 needs 9 (k = 53); P=8, F=4 needs 10 (k = 218). The const exists, is computable by exhaustion, moves with the type, and follows no visible formula of (P, F). |

## Python pre-computations (`precompute.py`, run before the Rust was written)

- sqrt sweeps (exact, ups, ties, max_r, max_operand_bits, overflow, zero_flush): (2,2): (2,1,0,4,4,0,0); (3,3): (2,4,0,12,6,0,0); (4,2): (4,6,0,12,6,0,0); (4,4): (4,8,0,28,8,0,0); (6,6): (8,34,0,124,12,0,0); (8,4): (16,128,0,124,12,0,0); (8,8): (16,134,0,508,16,0,0); (2,4): (2,2,0,12,6,**3**,0); (3,6): (3,3,0,31,9,**7**,0).
- emptiness criterion M >= 2^F - 1 predicted every band outcome correctly, all nine sweeps.
- cbrt: (4,2) -> (2,7,0); (6,3) -> (4,30,0).
- exp2 hits: F=2: k in {0,4,8}; F=3: k in {0,8,16,24}; ties 0; widths 25 and 57 bits.
- exp hardness at P=F=8: 0 ties, hardest k=112, distance 5.565e-4, ~11 extra bits.
