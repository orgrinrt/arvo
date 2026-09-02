# 38_probes outcomes

All against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed with
`rustc --version` from inside the repo (file 36's outside-the-repo resolution note observed).

| Probe | Question | Build shape | Outcome |
|---|---|---|---|
| `probe_1_collapse_survives_the_encoding_swap.rs` | Does file 35's Widening-collapse codegen result survive file 36's encoding replacement? The two had never been in one compilation unit; `36:362-364` asserted compatibility without compiling it. | `rustc --edition 2021 -C opt-level=3 --emit=asm` (codegen-quality shape, no LTO, per file 34's one-flag-set-per-question-class rule), plus a `-O` binary for the value check | WORKS. `_hot_mul_via_full_then_quantize = _hot_mul_direct` and `_precise_mul_widens = _hot_mul_direct` (assembler-level symbol aliases, `probe_1.s:190,192`); the one body is `mul x0, x1, x0; ret`. The 16+16=32 width sum is accepted as a type-equality demand on the value-unique spelling, and the runtime value check agrees (`OK: direct = 7006652, composite = 7006652`). Identical result to `35_probes/probe_1` on the replaced encoding. |
| `probe_2_is_exact_is_not_trivial_grade.rs` | Is `Op::IS_EXACT` alone "the grade monoid is trivial" (`37:301-303`)? | `rustc --edition 2021 --crate-type lib` (const eval, exhaustive over the 8-value model) | WORKS, and the answer is no. Exact-and-total: unit grade on all 64 pairs. Total-not-exact (wrapping add): events fire. Exact-not-total (a `div_exact` model, no quantiser anywhere, no event ever): refusal causes fire, so two views disagree on grade identification and the nine-view collapse fails for it. The trivialising condition is the conjunction `IS_EXACT && Total<Op>`. Negative control: negating one assertion in a scratch copy fails compilation with E0080 (`evaluation of CHECK failed`), so the checks demonstrably run. |

No timing claims anywhere in these probes; both are compile-and-inspect artifacts, per
`bench-and-sketch-discipline.md`.
