# `12_probes`

Sources and emitted output for `12_muratori_can_the_surface_meet_the_bar.md`. Every claim in that file
that names a probe names one of these, and every compile is reproducible with the command in its own
header comment.

Toolchain: `rustc +nightly-2026-05-28` reporting `1.98.0-nightly (57d06900f 2026-05-27)`, edition 2024,
aarch64-apple-darwin. No probe enables a forbidden feature. Three enable an allowed one
(`min_generic_const_args`) and one enables `lazy_type_alias`, which is **not on the workspace's vetted
list** and is used only to establish what it would buy, never as a proposal.

`ladder.rs` is copied unmodified from `11_probes/`, which copied it from `10_probes/`. Cite it for what
it proved, never for how it was written.

## Foundations nobody had checked

| File | Question | Result |
|---|---|---|
| `p01_nat_canonicity.rs` | is the ladder's addition canonical | yes. `Sum<T13,T13>` **is** `T26`, at three points including a carry that lengthens the tower. A padded tower is a distinct type of the same value |
| `p02_const_door_alias.rs` | may a type alias with a const parameter carry a projection | yes. `pub type UInt<const N: u32> = Fixed<NatOf<N>, T0, Warm>;` compiles and `UInt<5>` resolves |

## The hybrid, and whether the ceiling lifts

| File | Question | Result |
|---|---|---|
| `p03_hybrid_door_closed_algebra.rs` | does a const door over a nat keying lift the ceiling | yes. Three multiply octaves against a **six-row** door containing none of 48, 96 or 192. Containers exact at 4, 8, 16, 32 bytes. `_p03_native16 = _p03_arvo16` in the emitted assembly |
| `p04_five_spellings.rs` | what does a consumer type, in each candidate | all five compile in one crate; C1 through C4 proved the same type by coercion. Measured by `count.sh` |
| `p15_markers_do_not_partition.rs` | do two consumers' markers compose under the door | yes. Three markers, one type. A 4711-bit numeral declared by one of them is an ordinary numeral to all |

## What a consumer reads

| File | Question | Result |
|---|---|---|
| `p05_diag_mismatch.rs` | what does the ordinary mistake print | const keying prints `expected 13, found 26`; every nat keying prints a **truncated** binary digit tower |
| `p06_default_param_elision.rs` | does rustc elide a defaulted type parameter | no, but the construction compiles and the primary label recovers to `expected 13, found 26` |
| `p11_diag_battery.rs` | five keyings, one mistake, side by side | the measurement. Base ten is untruncated and readable; base two is neither |
| `p12_first_day_errors.rs` | what does an undeclared width say at the alias site | **nothing**, under both const-keyed and door-keyed surfaces. Only the name-based surface reports, with a suggestion |
| `p13_where_the_door_error_lands.rs` | where does it land instead | at the first use, sixteen lines later, spanning a name the consumer did not mistype and citing an internal type |
| `p14_lazy_type_alias.rs` | can that silence be closed | yes, at the alias's own line, under `lazy_type_alias` plus a `where` clause. `p14b` records the fifteen library-side bounds the feature then wants |
| `p14b_lazy_over_full_ladder.rs` | what does the feature cost the library | fifteen E0277s, every one an internal projection alias that now wants a bound written on it. `p14` shows writing them is what fixes it |

## The pinning wall, four positions, all refused

Reached from the opposite direction to `10` and `11`: nat to const, rather than const to nat.

| File | Position | Names |
|---|---|---|
| `p07a_pin_const_to_nat_nogate.rs` | `NatIs<{ <W as Nat>::V }>`, no gate | `generic_const_exprs` |
| `p07b_pin_const_to_nat_min_gca.rs` | the same under `min_generic_const_args` | wants `type const V` |
| `p07c_pin_const_to_nat_type_const.rs` | `type const V: u32 = 2 * T::V;` | wants a const block |
| `p07d_pin_const_block.rs` | `type const V: u32 = const { 2 * T::V };` | `generic_const_args` |

## Base ten, built rather than sketched

| File | Question | Result |
|---|---|---|
| `p09_decimal_ladder.rs` | is structural addition available in base ten | yes. Checked against `Nat::V` at ten points including `8+8`, `99+1` and `777+777`. Canonicity at four |
| `p10_decimal_container.rs` | can base ten derive the container | yes. Halving is twenty rows; the native rung is asserted at 8, 13, 16, 17, 32, 33, 64, 65, 128 and the wide payload at 129, 200, 256 and 1636 bits, landing on 208 bytes |
| `p08_does_p06_shape_keep_the_ceiling.rs` | does the consts-in-front shape stay closed | only with two head constructors. Compiles, six-row door, containers exact |

## Reproducing all of it

`./verify.sh` recompiles all nineteen and prints one line each against its expected outcome. Output
committed as `out/verify.txt`, so a later reader compares against a record rather than rerunning to find
out whether they agree. Eight compile, eleven refuse, and every refusal is the result rather than a
failure.

`./count.sh` produces the character counts in section 3 off `p04_five_spellings.rs`, which compiles.
Output committed as `out/count.txt`.

## `out/`

Captured stderr per probe (`*.log`), emitted metadata, and `p03.s`, the assembly the codegen claim in
section 3 is read from. **That reading is an ad-hoc quick spike with no substance as a measurement** and
is named that in the file. Nothing here ran on the bench harness, so every magnitude in this round is
unpriced.
