# Probes for panel file 35, does Widening collapse

Three artifacts, all compiled against the workspace pin, `rustc 1.98.0-nightly (57d06900f
2026-05-27)` (matches file 34's pin exactly; confirmed with `rustc --version`). Two build shapes are
used throughout, named per file 34's own section 1 distinction, and every measurement below states
which shape produced it:

- **Functional shape** (value correctness): `rustc --edition 2021 --crate-type bin -C opt-level=2
  <file> -o <bin>`, then run the binary and check printed values.
- **Codegen shape** (instruction-level comparison, "shipping-shaped" per file 34): `rustc --edition
  2021 --crate-type lib -C opt-level=3 --emit=asm <file> -o <out>.s`. No LTO flag. This is file 34's
  corrected shape A: without `-C lto=fat`, `--emit=asm` on a lib runs the full normal pre-link pass
  pipeline (inlining, vectorisation, DCE) and is the right build for a codegen-quality question. LTO
  is not exercised here; nothing in this file needed the cross-crate-visibility question file 32/34's
  shape C answered.

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_widening_collapses_into_mul_full.rs` | Does removing `Widening` as an axis and expressing every preset purely via (a) which primitive is called and (b) the target numeral `quantize` narrows into, lose any distinguishing power or any codegen quality, at native register width? | WORKS. All three old `Widening` instances (`None`, `InContainer`, `PerOperation`) are expressed with no axis: `hot_mul_direct` (a native op with no software-visible wide intermediate), `hot_mul_via_full_then_quantize` (the composite: `mul_full` into a real, named product-numeral type, then `quantize_wrap` narrows it back), and `precise_mul_widens` (returns the product numeral unnarrowed, i.e. "per operation" widening is exactly `mul_full`'s own return type). Functional shape: all three agree on value. Codegen shape, `-C opt-level=3`, no LTO: `hot_mul_direct`, `hot_mul_via_full_then_quantize` and `precise_mul_widens` compile to the literal SAME symbol (`_hot_mul_via_full_then_quantize = _hot_mul_direct`, `_precise_mul_widens = _hot_mul_direct`), one instruction (`mul x0, x1, x0; ret`). The composite pays nothing over the direct form; the compiler folds them before the assembler even sees three bodies. |
| `probe_2_growth_is_a_function_of_the_operation.rs` | Is `Growth` derivable from the operation name, or can the two vary independently (which would mean the law key genuinely needs both slots)? | WORKS, and sharper than argued in prose: the OLD shape (`Growth` as a free co-equal parameter alongside an operation tag) type-checks a pairing the design can never build (`MulFull` paired with `Growth::Narrowed`; `IMPOSSIBLE_BUT_EXPRESSIBLE` compiles to `false` at the value level but the call itself is accepted by the type system, which is exactly the "carries too much... discipline not mechanism" gap consolidation section 1.4 already names). The NEW shape (`Growth` as an associated const on the operation marker, `Op::IS_EXACT`) has no slot in which to spell a mismatch at all: the redundancy is not merely unexercised, it is structurally unrepresentable once `Growth` is bound to the marker rather than threaded as a second parameter. Both `const fn` keys agree everywhere the old key can be called truthfully (the two `const _: ()` assertions at the end of the file). |
| `probe_3_multilimb_widen_then_truncate.rs` | At a width beyond one native register, where schoolbook multiplication genuinely needs fewer limb-products for a truncated result than for the full exact product (3 against 4, for two 2-limb operands), does "always compute the exact wide product via `mul_full`, then narrow" cost anything over a dedicated truncating primitive? | WORKS, shipping-shaped (inlinable `mul_full_256`): `warm_mul_via_full_then_quantize_128` (the composite: schoolbook 256-bit product, then discard the high 128 bits) and `hot_128_direct` (native `u128::wrapping_mul`, a genuine 128x128->128 truncating multiply) both compile to 4 instructions (`umulh`, two `madd`, `mul`), the same instruction shape up to commutative operand order. The optimiser recovers the cheaper computation (the `hi_hi` limb product and its carries, which contribute only to bits >= 128, are eliminated) once the composition is visible to it. Also tried and NOT committed as the primary shape: marking `mul_full_256` `#[inline(never)]` (file 34's "axis-legible", check-build-shaped variant). That variant does not fold; it pays a real call, a stack frame, and a spilled return value (24 lines against 7). Both results are consistent with the SAME distinction file 34's section 1 already draws for addition: codegen quality is a shipping-shaped-build question, axis legibility is a check-build question, and asking the wrong one of the other's question gets a wrong answer for a methodological reason, not a design reason. |

## Honesty about what this does and does not establish

Probe 1 and probe 3 measure the optimiser's behaviour on this pin, at these two widths (native 128-bit,
schoolbook 256-bit-to-128-bit). They are not a formal guarantee that every width and every target
combination folds this cleanly, and this file does not claim one. This is the identical epistemic
status consolidation section 1.6 already assigns the multi-limb carry-chain finding ("a dependency on
an optimiser heuristic holding, not a guarantee, and it costs one codegen test to make falsifiable",
`26:452-457`): a positive result on a real compiler, not a proof, worth pinning as a regression test
so a future toolchain bump that stops folding this is caught rather than silently eating a preset's
performance.

Probe 2 is a structural argument about the type system's expressive power (does the key admit a
mismatch), not a numeric measurement; it is exhaustive by construction (two operations, one
associated fact each, checked at both).
