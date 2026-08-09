# Outcomes, 80_probes

Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`,
resolved from the repo's `rust-toolchain.toml`, confirmed by `rustc --version` inside the
tree this session (the identical command outside the tree resolves to stable `1.94.0`).
Runtime probes built `rustc --edition 2021 -O` and run on the host; compile-only probes
built `--edition 2021 --crate-type=lib --emit=metadata`. Nothing under `mock/crates/`
touched.

## probe_1_foldexact_type_level.rs. COMPILES CLEAN, all assertions at compile time

The exact fold-width closed form `foldexact(P, A) = bitlen(A * (2^P - 1))` built at the
type level over the sealed `Nat`/`Pos` grammar: `Succ` (carry chain), `AddP` (nine
constructor pairs), `MulP` (shift-and-add), `DecP` (file 79's predecessor, rebuilt),
`AllOnes` (2^P - 1 as P ones, value recursion through `DecP`), `BitLen` (structural
depth). Zero feature gates. 114 matrix cells (p in {1,2,3,8,11,16} x A in {1,2,3,4,5,7,
8,16,64,100,256,257,513,514,1024,1025,2048,2049,4096}), every cell asserted at compile
time equal to an independent u128 const ground truth, plus `exact <= foldnum` and
`foldnum - exact <= 1` per cell; the six characterised loose cells (p=8: 257, 513, 514,
1025, 2049; p=11: 2049) asserted loose by exactly one; ten power-of-two cells plus
the (p=2, A=3) tight-non-power counterexample asserted exactly tight. The computed width
is consumed in type position (`Acc<W>`, `FoldAcc<P, A>`), no const machinery.

Negative control: appending `assert!(<<P8 as FoldExact<C257>>::Out as Nat>::VAL == 17)`
to a scratch copy fails the build with E0080 (`evaluation panicked`), so the const
assertions are live under `--emit=metadata`, and (p=8, A=257) is genuinely 16 where
`foldnum` says 17.

Pricing, wall clock, three runs: 0.148 / 0.145 / 0.144 s against an empty `#![no_std]`
lib baseline of 0.036 s. The whole machinery plus 131 checked cells costs roughly 0.11 s,
under 1 ms per cell all-in.

## probe_2_statement0_vs_quantize_rtie.rs. RUNS CLEAN, 0.70 s

Model: decimal Ranged numeral, r=10, p=3, quantum exponents -2..=1, unnormalised
significands, |D| = 4000, exact i128 arithmetic (the same discipline as
`66_probes/model.rs`), rounding half-even throughout.

- Quantize closure, exhaustive over 16,000,000 operand pairs: every result is a refusal
  or a datum of D. Refusals: 5,679,000 (35.5% of pairs), exactly equal to the count of
  pairs where the naive non-refusing quantize emits a mantissa >= 10^3 (a non-datum).
- Quantize is not value-level well-defined: witness x = 1.23, y1 = (1, 0), y2 = (10,
  -1) (equal values, different data) give results 1 and 1.2 (different VALUES). Density:
  2,889 of 4,000 operand x's are affected by at least one cohort pair.
- roundToIntegralExact closure, exhaustive over D: total, zero refusals, every result a
  datum of D.
- roundToIntegralExact is fibre-preserving, exhaustive over every value-equal operand
  pair: result values always equal; witness (10, 0) vs (1, 1) shows the result DATUM
  still differs (the dependence is exactly cohort-member selection).

## probe_3_nine_bit_companion.rs. RUNS CLEAN, 0.42 s

The u16-class companion at nine logical bits. Container-class witnesses: u16 dispatched
(size 2); logical wrap and container wrap differ at nine bits and coincide at eight (the
u8-class vacuity fact). Unsigned and signed (two's complement, sign-flip bijection)
order matrices exhaustive over 2^18 pairs each: datum order equals value order, zero
mismatches, injectivity everywhere, exactly one signed zero pattern. Padding hazard,
whole matrix of same-value-different-padding pairs (512 x 127 = 65,024): every pair
misordered by a raw-carrier compare and Equal under the canonical compare; plus a
witness where raw order inverts value order outright (dirty 0 > clean 1).

## probe_4_crosses_unsafe_marking.rs / probe_4b_safe_impl_refused.rs

probe_4 COMPILES CLEAN: `unsafe trait Crosses<N: Numeral>: Lowering` with ONE blanket
`unsafe impl` over tower-generated encodings (obligation discharged by construction,
Send/Sync shape) plus a per-declaration consumer `unsafe impl` for a hand-laid layout,
and a door consuming the bound.

probe_4b REFUSED, E0200: `the trait Crosses<SomeNumeral> requires an unsafe impl
declaration`, on the literal spelling of the consolidated text's "the impl is blanket
and safe" (68:271). The compile-fail file is kept so a later downgrade of `Crosses` to a
safe trait starts compiling it and surfaces.
