# 68_probes

Probes for `68_leroy_what_the_pipeline_certifies.md`. All runs on the pinned
`nightly-2026-05-28` (`p0_toolchain.txt`); zero feature gates anywhere.

## Verification reruns (p1, p1b, p2, p2b)

- `p1_rerun_65_clean.txt`: rerun of `65_probes/derive_validate_erase.rs`, clean compile, exit 0.
- `p1b_65_negative_enabled.rs` + `p1b_65_negative.stderr`: `65`'s negative case regenerated from the
  **committed** source (one line enabled; `65`'s own committed transcript was produced from an
  uncommitted `/tmp/negcase.rs`). E0277 reproduces with line numbers matching the committed text.
- `p2_rerun_66.txt`: `66_probes/derive_validate_erase_pipeline.rs` compiled as lib and as test
  binary; four tests pass. `66` had committed no transcript of any of this.
- `p2b_rerun_66_python.txt`: all three of `66`'s Python probes run; every claimed count reproduces
  (1152/4913, 81 strings onto 31 values, the fixed-vs-float error table). `66` had committed no
  outputs.

## New instruments (p3, p4, p5)

- `p3_mutant_overdeclared_window.rs` + `p3_mutant.stderr`: MUTANT of `65`'s probe. The storage
  representation's declared window is widened to a lie ([-100, 100] over maps covering [-3, 12])
  and `REDUNDANT` flipped. **Compiles clean.** Establishes: `65`'s validation constrains
  declarations only from below; the guarantee is carried by the round-trip through the maps, and a
  declared constant nothing reads constrains nothing.
- `p4_validate_residue.rs` + `p4_validate_residue.s` + `p4_asm_grep.txt`: the two readings of
  "validate" at `-O` on the pin. The typed interior op is symbol-aliased to the bare op
  (`_add_trusted = _add_bare`, zero residue); the compile-time-validated constant folds to
  `mov w0, #123`; the runtime-validate shape carries `orr`/`tst`/`csel` residue per ingest.
  Qualitative existence demonstration only; an ad-hoc quick spike with no substance for any
  how-much question. Nothing is priced.
- `p5_const_eval_ceiling.rs` + `p5_w6.txt` + `p5_w9.txt`: the exhaustive signed-saturating
  associativity count in const context, cfg-keyed by width. w=6 compiles in 3.1 s; w=9 is refused
  after 4.7 s by `deny(long_running_const_eval)` (on by default; allowable, so a default refusal
  rather than an absolute wall). Re-establishes the model-width ceiling inside this panel's own
  probe set.

Every incidental spelling here (names, the reserved-top-bit predicate, the i64 model) is
scaffolding, not design.
