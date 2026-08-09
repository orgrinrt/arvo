# Probe outcomes, file 41

All probes compiled against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
confirmed with `rustc --version` from inside the repo. `vu_nat.rs` in this directory is a copy of
`36_probes/vu_nat.rs` with the `maxmin` ablation module dropped (unneeded here; that module itself is
not copied, so the trim is load-bearing, not cosmetic). `vu_bias.rs` is the new module this file adds,
built on top of it.

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_bias_is_the_same_construction.rs` | Is `Bias` value-unique by construction, and does it carry the MATLAB witness `Bias = Int` (`36:222`) could not? | WORKS. `BZero \| BPos<N,D> \| BNeg<N,D>` with `N: Pos + Gcd<D, Out = H>, D: Pos`. Slope 1, bias 1/2 (file 39 probe 1's witness) is a representable `BPos<H, O<H>>`. `ReducedBiasPos`/`ReducedBiasNeg` normalise two spellings of one signed rational to one type before anything asks. |
| `probe_1b_an_unreduced_pair_has_no_bias_type.rs` | Does an unreduced pair (6/12) reach a `Bias`-bounded position? | FAILS WITH E0271: `type mismatch resolving <O<I<H>> as Gcd<O<O<I<H>>>>>::Out == H`, committed refusing, on purpose. |
| `probe_2_generic_reduce_composition_is_refused.rs` | Does `Reduce` compose as a bound inside another fully generic trait/function, the way `Adjustment` already composes `Gcd`? | (a) `Gcd` bound alone: WORKS. (b) `Reduce` as a bare bound: FAILS WITH E0275, "overflow evaluating the requirement `Pz<O<_>>: ExactDivOdd<_>`", unused, no call site anywhere in the file. (c) `Reduce` with a concrete associated-type equality (`N = H`): FAILS identically. This is the wall that decided `BiasProduct`'s final shape; committed refusing, on purpose. |
| `probe_2b_bare_alias_stays_lazy.rs` | Does the same `Reduce` reference, as a bare top-level type alias with no trait wrapping it, escape probe 2's wall? | WORKS. This is the shape `vu_bias.rs`'s `ReducedBiasPos`/`ReducedBiasNeg`/`BiasMagN`/`BiasMagD` all use. |
| `probe_3_bias_multiplication_and_closure.rs` | Does `bias = B1 * B2` (`31:399-400`) compute correctly at the type level over the witnesses file 39 checked at the value level (biases 1/2 and 5/2)? | WORKS. `1/2 * 5/2 = 5/4` (all three sign combinations); `2/3 * 3/4`'s raw componentwise product `6/12` renormalises to `1/2`; `3/4 * 4/3 = 1/1`, the identity case, checked both signs. |
| `probe_4_adjustment_is_not_sealed_lib.rs` + `probe_4b_downstream_widens_adjustment.rs` | Does a genuinely separate downstream crate widen `Adjustment` with a fabricated, unreduced pair? | WORKS (the defect: this should have been refused, and was not). A foreign type `Six` implements `Adjustment` directly with `NUM = 6, DEN = 12`, no coprimality check anywhere, because `Adjustment` (unlike `Pos`/`Nat`) carries no seal. Two-step build, both steps clean. |
| `probe_5_bias_sealed_perimeter_lib.rs` + `probe_5b_downstream_cannot_widen_bias.rs` | Does the same attack succeed against `Bias`, which is bounded directly on the reduction condition and carries its own seal? | FAILS on both attempted routes, in the SAME file: (a) implementing the private `bias_sealed::BiasSealed` supertrait directly, FAILS WITH E0603, "module `bias_sealed` is private"; (b) using the exported `BPos` constructor with an unreduced pair, FAILS WITH E0271, the same coprimality failure as probe 1b surviving the crate boundary. Two-step build; both errors are from the second step. |

## Price (measured, `price/`)

`price/gen.py` + `price/sweep.sh`, the same methodology as `36_probes/price/` (a seeded generator,
`rustc --edition 2021 --crate-type lib --emit=metadata`, min-of-N wall time, baseline at count 0
subtracted, every instantiation forced by a const assertion against a Python-computed value so nothing
is elided). Two honestly-stated scope reductions against file 36's own sweep: min-of-1 rather than
min-of-3 (one run per point, not three; the wall-clock budget for one dispatch does not stretch to
`6 counts x 2 kinds x 2 bit-widths x 3 runs`), and two bit-widths chosen for what they compare against
rather than matching file 36's exact 8/16 pair (see below).

Two kinds swept: `bias_mag` (`BiasMagN`/`BiasMagD` alone, the magnitude: `PMul` then `Reduce`, no
sign) and `bias_full` (`BiasMulPP`, the full composition a consumer actually names). Counts 0, 25, 50,
100, 200, 400, least-squares slope over the non-zero counts, baseline (count 0) subtracted.

Operand bit width needs a word, because `BiasMagN`/`BiasMagD` reduces the RAW componentwise product
`N1*N2` / `D1*D2`, not the operands themselves, so an operand width of `w` bits produces a `Reduce`
call over up to `2w`-bit numerals. File 36's own headline reduction number (12.07 ms/composition) is
`Reduce` over 16-bit operands directly. The comparable point for `BiasMagN`/`BiasMagD` is therefore
8-bit RANDOM operands (product up to 16 bits, the same width `Reduce` itself is doing the work at), not
16-bit operands (product up to 32 bits, a harder problem file 36 never swept). Both are run; both are
reported; neither is silently treated as the other.

| shape | operand width | product width | ms/composition (least-squares slope) |
|---|---|---|---|
| `BiasMagN`/`BiasMagD` (magnitude only) | 8-bit | up to 16-bit | 13.61 |
| `BiasMulPP` (magnitude plus sign) | 8-bit | up to 16-bit | 19.10 |
| `BiasMagN`/`BiasMagD` (magnitude only) | 16-bit | up to 32-bit | 102.60 |
| `BiasMulPP` (magnitude plus sign) | 16-bit | up to 32-bit | 159.42 |
| `BiasMulPP`, dyadic magnitudes only (`N = 1`, `D` a power of two up to `2^16`) | mixed | up to 32-bit | ~1.55 (single run, 400 compositions, 700 ms wall total minus ~80 ms baseline) |

Zero symbols emitted at 400 `bias_full` compositions, confirmed with `nm -g` on a
`-C opt-level=2 --emit=link` build (`nm -g out/*.rlib \| grep -c " T \| t "` returns `0`), the same
check file 36 ran and the expected answer for the same reason: every type here is a `PhantomData`-only
zero-sized type with const-only content.

Metadata debit, `--emit=metadata`, empty-crate baseline 1267 bytes: `BiasMagN`/`BiasMagD` at 400
compositions (16-bit operands) reaches 1,686,904 bytes, roughly 4.21 KB per composition; `BiasMulPP`
reaches 1,770,870 bytes, roughly 4.42 KB per composition. Both are larger than file 36's own reduction
debit (1.9 KB per composition), consistent with the wider (up to 32-bit) product this operand width
forces the reduction to name in its type parameters.
