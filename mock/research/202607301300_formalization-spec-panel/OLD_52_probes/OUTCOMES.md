# Probe outcomes, file 52

Everything below was built fresh for this dispatch, on `rustc 1.98.0-nightly (57d06900f
2026-05-27)`, host `aarch64-apple-darwin`, resolved via `rustc +nightly-2026-05-28` (not the bare
`rustc` on `PATH`, which resolves to stable `1.94.0` outside the repo tree; see the toolchain-
resolution note below, which is itself a finding this dispatch made about its own first draft).

## The toolchain-resolution mistake, caught before it reached this document

The first pass of every artifact in this directory was built in a scratch directory outside
`~/Dev/clause-dev/arvo/`, where bare `rustc` resolves to the machine's stable `1.94.0`, not the
pin. `48_probes/OUTCOMES.md` already names this trap for a plain `rustc` invocation ("the default
`rustc` on this machine resolves to stable 1.94.0 outside the repo directory") and I reproduced it
anyway on the first pass of every one of the eight probes below, catching it only when
`div_rem_fusion.rs`'s emitted assembly named a path
(`/Users/orgrinrt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/...`)
that named the wrong toolchain in the panic-location string literal. Every command in this file and
in the harness files was rebuilt with `rustc +nightly-2026-05-28` explicitly and re-diffed against
the first pass; every seal-adversary and projection-chain result was byte-identical (the two builds
differ only in the crate-disambiguator hash embedded in mangled symbol names). The five codegen
numbers below are the SECOND pass, on the pin, not the first.

## The seal adversary (`seal/`)

Tower: `seal/tower.rs` + `seal/vu_nat_sealed_adj.rs` + `seal/vu_bias_sealed_adj.rs`, an unmodified
copy of `46_probes/probe_2_vu_core_lib.rs` and its two `#[path]`-included files. Diff against
`46_probes/` to audit; nothing here changes tower content.

```
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib seal/tower.rs --crate-name vu_core -o /tmp/libvu_core.rlib
```

| Fixture | Route | Expected | Reproduced |
|---|---|---|---|
| `seal_direct_impl_all_four_carriers.rs` | direct impl on a local type, x4 carriers | E0277 x4 | E0277 x4, identical bound names to `46_probes/probe_3` |
| `seal_supertrait_unnameable.rs` | impl the private supertrait itself | E0603 x2 (dedup'd: `sealed`, `bias_sealed`) | E0603 x2 |
| `seal_fabricated_pos_replay.rs` | foreign `Pos` with lying unconditional `Gcd` | E0277 at the shared root | E0277, `Fabricated: nat::sealed::PosSealed` |
| `seal_malformed_types_fn_forced.rs` | unreduced/padded types at bounded positions, fn-forced | E0271 x2, E0277 x1 | reproduced verbatim |
| `seal_reimpl_on_genuine_inhabitant.rs` | re-impl a sealed trait / helper on a genuine inhabitant | E0117 | reproduced |
| `seal_downstream_blanket.rs` | blanket impl over an uncovered type parameter | E0210 | reproduced |
| `seal_dyn_refused.rs` | `&dyn Pos` | E0038 | reproduced |
| `seal_extension_positive_control.rs` | legitimate downstream extension by structural recursion | compiles clean | rc=0 |

Every `.rs`/`.stderr` pair in `seal/ui/` is the corresponding `46_probes/probe_*` file, renamed to
its shipping name, rebuilt against the exact tower shipped alongside it in this directory (not
against `46_probes/`'s copy), with the `.stderr` captured verbatim from that rebuild. Reproduction
command shape for any one fixture:

```
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib --extern vu_core=/tmp/libvu_core.rlib \
  seal/ui/<fixture>.rs -o /tmp/out.rlib
```

## The projection-chain pair (`projection/`)

Tower: `projection/tower.rs` + `vu_nat.rs` + `vu_bias.rs`, an unmodified copy of `47_probes/tower.rs`
and its two included files (themselves unmodified copies of the seal tower, retargeted by `#[path]`
only).

```
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib projection/tower.rs --crate-name tower -o /tmp/libtower_final.rlib
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib --extern tower=/tmp/libtower_final.rlib projection/ui/grade_projected.rs -o /tmp/gp.rlib
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib --extern tower=/tmp/libtower_final.rlib projection/ui/reduce_bound_wall.rs -o /tmp/rbw.rlib
```

`grade_projected.rs` (`47_probes/probe_3_the_grade_is_projected.rs`, renamed): rc=0, reproduced.
`reduce_bound_wall.rs` (`48_probes/probe_1_the_wall_is_one_refactor_away.rs`, renamed): `E0275,
overflow evaluating the requirement Pz<O<_>>: ExactDivOdd<_>`, verbatim match against
`48_probes/OUTCOMES.md`'s recorded head.

## The five codegen regression tests (`codegen_regression_harness.rs` + `codegen/`)

Compiled and run as a standalone `--test` binary (`rustc +nightly-2026-05-28 --edition 2021 --test
codegen_regression_harness.rs`), invoked with `RUSTC=$(rustup which --toolchain nightly-2026-05-28
rustc)` so the harness's own internal `rustc` calls (which build each fixture with `-C opt-level=3
-C codegen-units=1 -C panic=abort --emit=asm`) resolve the pin rather than `PATH`. All six `#[test]`
functions pass:

```
running 6 tests
test fold_vs_direct_multiply_native_width_folds_to_one_instruction ... ok
test saturating_reduction_stays_scalar_wrapping_control_vectorises ... ok
test fold_vs_direct_multiply_multi_limb_width_matches_instruction_shape ... ok
test div_floor_and_rem_fuse_into_one_hardware_divide ... ok
test multi_limb_carry_chain_compiles_to_straight_line_adc_no_calls ... ok
test assert_equal_length_idiom_defeats_vectoriser_bare_loop_does_not ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

### Test 1, `carry_chain.rs`: multi-limb carry chain

`add256`, a 256-bit `carrying_add` over four `u64` limbs: 3 carry-propagating instructions (`adds`
then `adcs`, `adcs`, `adc`), 0 calls, straight-line. Pins `35_dolan_does_widening_collapse.md:113-119`.

### Test 2a, `mul_fold_native.rs`: fold-vs-direct-multiply, native width

`hot_mul_via_full_then_quantize` (widen to `u128`, multiply, truncate) assembles to the identical
symbol as `hot_mul_direct` (`a.wrapping_mul(b)`): `_hot_mul_via_full_then_quantize =
_hot_mul_direct` in the emitted `.s`, one `mul` instruction. Pins
`35_dolan_does_widening_collapse.md:111-120`.

### Test 2b, `mul_fold_128.rs`: fold-vs-direct-multiply, multi-limb width

`hot_128_direct` (`u128::wrapping_mul`) and `warm_mul_via_full_then_quantize_128` (schoolbook
limb multiplication computing the genuine 256-bit product, four named limb products, truncated to
the low 128 bits) both compile to exactly four instructions (`umulh`, two `madd`, `mul`), same
shape, operand order differs (commutative). Pins `35_dolan_does_widening_collapse.md:129-137`.

### Test 3, `sat_reduce.rs`: saturating-reduction non-vectorisation

`sat_sum4` (four chained `saturating_add`s): 0 NEON lane instructions, four scalar
`adds`/`asr`/`eor`/`csel` clamp sequences. `wrap_sum4` (identical shape, `wrapping_add`): vectorises
to `addv.4s` (one instruction). Pins `35_dolan_does_widening_collapse.md:103-110`.

### Test 4, `loop_idiom.rs`: vectorisable-loop-idiom sensitivity, AND a methodology gap this dispatch found and closed

`add_assert_idiom` (`assert!(a.len()==b.len()&&b.len()==out.len())` ahead of `for i in
0..a.len()`) versus `add_no_assert_idiom` (identical loop, no prior assertion). Under the harness's
pinned flags (`-C codegen-units=1`), reproduces `34_giesen_the_three_halves_assembled.md:122-124`
exactly: the assert idiom stays scalar (0 vector lines), the bare idiom vectorises (4 `add.2d`
lines).

**What this dispatch found that file 34 did not record.** `34_probes/probe_0_revectorise.sh`, the
script file 34 cites as evidence for this exact claim, never actually measures
`probe_elementwise_add_fixed_equal_len_idiom` or `probe_elementwise_add_ablation_no_generic` (the
two functions carrying the assert idiom); it measures only `probe_vectorises_verbatim_control` and
`probe_elementwise_add_fixed_no_assert`, both no-assert functions. Neither `34_probes/OUTCOMES.md`
nor `32_probes/OUTCOMES.md` contains the string `equal_len_idiom` or `ablation_no_generic` anywhere.
The claim at `34:122-124` ("both scalar under shape A") is inherited from file 32's original
LTO-contaminated measurement (`32_probes/OUTCOMES.md`) without being re-derived under file 34's own
corrected methodology, which is the identical pathology file 44 (my own prior file) named for claims
surviving a coordinate change without re-derivation, one level down, inside the very finding this
dispatch was asked to pin as a regression test.

Reproducing it against the ACTUAL committed crate (`32_probes/identity_model/src/lib.rs`), under
file 34's own `COMMON` flags (`--edition 2021 -C opt-level=3 -C codegen-units=1 -C panic=abort`,
shape A, `--emit=asm`, no LTO):

```
rustc +nightly-2026-05-28 --edition 2021 -C opt-level=3 -C codegen-units=1 -C panic=abort \
  --crate-type lib --emit=asm 32_probes/identity_model/src/lib.rs -o /tmp/p34_recheck.s
```

| symbol | NEON `.2d` lines |
|---|---|
| `probe_elementwise_add_fixed_equal_len_idiom` | 0 |
| `probe_elementwise_add_ablation_no_generic` | 0 |
| `probe_elementwise_add_fixed_no_assert` | 4 |
| `probe_vectorises_verbatim_control` | 4 |

The claim IS true, confirmed here for the first time against the real crate under shape A. But it is
true only WITH `-C codegen-units=1`. This dispatch's own first standalone reproduction (a two-function
crate, no `-C codegen-units=1`, otherwise identical source) vectorised BOTH idioms identically (4
NEON lines each), which read as a refutation until the flag was isolated:

```
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -C opt-level=3 -C panic=abort \
  --emit=asm loop_idiom_scratch.rs -o without_cu1.s        # both idioms vectorise
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -C opt-level=3 -C codegen-units=1 \
  -C panic=abort --emit=asm loop_idiom_scratch.rs -o with_cu1.s   # assert idiom stays scalar
```

`-C codegen-units=1` is therefore load-bearing for THIS test's specific claim, undocumented as such
anywhere in files 32/34/40/49. The committed harness (`codegen_regression_harness.rs`) pins the flag
explicitly in its own build command for exactly this reason, and its doc comment states the
sensitivity so a future reader running the fixture without the flag does not conclude the design
regressed.

### Test 5, `div_rem_fusion.rs`: `div_floor`/`rem` fusion into one hardware divide

`div_floor_and_rem` (calling `div_floor` then `rem_euclid`-shaped logic on the same two operands,
inlined into one function): exactly one `sdiv`, the remainder recovered via `msub` from the
already-computed quotient (`r = a - q*b`), rather than two independent divisions. The two
independently-callable public functions (`div_floor`, `rem_euclid`) each carry their own `sdiv`,
because they are separate, non-inlined call sites, not the case this test is about. Pins
`43_smith_division.md:283-287`. aarch64 has no combined div+rem instruction (unlike x86's `idiv`),
so the fusion measured here is specifically "one division, remainder by multiply-subtract," not
"one instruction total"; the harness's own doc comment on this test states that distinction.

## Gate

`cargo test --workspace` from `mock/`: not re-run for this dispatch, since nothing under
`mock/crates/` changed (the design surface this file adds tests for has no shipped source, per the
verification paragraph below); the last recorded figure (654 passed, 0 failed, 9 ignored) stands
unchanged across files 41 through 51 and this dispatch touches none of that surface.

`grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` from the repo root: exit 1,
empty. Same command with `FullRange\|UTerm\|AddWidth`: exit 1, empty. Both reproduce the
consolidation's own corrected verification paragraph (`49:39-51`).
