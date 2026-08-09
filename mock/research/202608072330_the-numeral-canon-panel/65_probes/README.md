# 65_probes

Probes for `65_knuth_number_systems_derived_cold.md`, written during the blind
phase of the cold derivation protocol (premises only, no panel files read).

## derive_validate_erase.rs

One standalone file, compiled on the pinned toolchain with zero feature gates:

```
rustc --edition 2024 --crate-type lib derive_validate_erase.rs
```

Outcome: **WORKS**. `compile_output.txt` records the clean compile and the
toolchain (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the pinned
`nightly-2026-05-28`). Every `const _: () = assert!(...)` in the file evaluated
at compile time; a failure of any would have been a compile error.

What it establishes, and nothing more:

1. Each verb of op's acceptance criterion ("derive the matching container and
   numeral representations, then validate, and erase") has an expressible form
   in plain traits, const fns and `repr(transparent)`, with no feature gates,
   when width is a type-level marker rather than a bare const.
2. A demand window derives two distinct representations (biased storage,
   two's-complement compute) over one container, with coverage and round-trip
   validated exhaustively in const context.
3. A redundant compute representation (carry-save pair) preserves value under
   the 3:2 compressor, exhaustively at the 4-bit model width, and names one
   value by several numerals.
4. The law inventory is a computed property of (window, policy), not of the
   numeral: wrapping addition mod 16 is associative (exhaustive), signed
   saturating addition is not (counterexample found exhaustively and pinned:
   `(7 sat+ 7) sat+ -7 = 0` versus `7 sat+ (7 sat+ -7) = 7`), and unsigned
   saturating addition is associative (exhaustive).
5. A law can be a compile-time contract: `requires_associative::<SatAddI4Op>()`
   is refused by the trait solver. `compile_fail_negative_case.txt` records the
   E0277 from compiling the file with that line enabled.

What it does not establish: the general width-to-container projection for an
arbitrary const `N`, any performance claim, and any transfer beyond the 4-bit
model width other than by the uniformity of the constructions. The probe's
incidental spellings (names, the i64 model carrier, the [-3, 12] window) are
scaffolding, not design.
