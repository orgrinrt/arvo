# Probe outcomes, file 46

All probes compiled against the workspace pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, confirmed with `rustc --version` / `rustc -vV` from inside the repo.

`vu_nat_sealed.rs` and `vu_bias_sealed.rs` are copies of `42_probes/vu_nat_sealed.rs` /
`42_probes/vu_bias_sealed.rs`, unmodified: the tower exactly as file 42 left it. `vu_nat_sealed_adj.rs`
is that file plus ONE diff (the `Adjustment` seal: `AdjustmentSealed` added to the existing private
`sealed` module, `Adjustment` gains it as a supertrait, one blanket `AdjustmentSealed` impl with the
identical bound the `Adjustment` impl already carries); `vu_bias_sealed_adj.rs` is the bias file with
only its `#[path]` retargeted. Diff the pairs to audit.

Build commands are in each probe's header. Library builds first:

```
rustc --edition 2021 --crate-type lib probe_1_tower_as_42_left_it_lib.rs     # 42's tower
rustc --edition 2021 --crate-type lib probe_2_vu_core_lib.rs                 # this file's tower
rustc --edition 2021 --crate-type lib --crate-name vu_nat_layer vu_nat_sealed_adj.rs  # for probe 7
```

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_tower_as_42_left_it_lib.rs` + `probe_1b_foreign_adjustment_still_lands.rs` | Does file 41's ORIGINAL attack (a genuinely separate downstream crate implements `Adjustment` directly on a local type, fabricated `NUM = 6, DEN = 12`, no `Pos`, no `Ratio`, no coprimality) still land against the tower as file 42 left it, and reach an `A: Adjustment`-bounded position (fn-forced, not alias-inert)? | **COMPILES CLEAN (the defect).** File 42's `Pos`/`Nat` seal is never touched by this route; `Adjustment` in `vu_nat_sealed.rs:448-455` has no supertrait. The perimeter file 42's own table (`42:139`) records as "CLOSED after the fix" is closed only against the fabricated-`Pos` route; the direct-impl route file 41 found first (`41_probes/probe_4b`) was never closed in any copy the tower composes with. |
| `probe_2_vu_core_lib.rs` | Does the completed tower (`Pos`/`Nat`/`Adjustment`/`Bias` all sealed) build? | WORKS. |
| `probe_3_direct_impls_refused.rs` | Direct foreign impls of all four sealed traits. | FAILS x4, one E0277 per trait: `EvilPos: PosSealed`, `EvilNat: NatSealed`, `EvilAdj: AdjustmentSealed` (the new one; this exact impl compiles clean against probe 1's tower), `EvilBias: BiasSealed`. |
| `probe_3b_supertrait_unnameable.rs` | Implement the private supertraits themselves. | FAILS x3, E0603 (`module 'sealed' is private` twice, `module 'bias_sealed' is private` once): the seal's own route is unnameable before trait solving starts. |
| `probe_3c_fabricated_pos_replay.rs` | File 42's fabricated-`Pos`-with-lying-`Gcd` attack, replayed against this tower as a regression check that the `Adjustment` diff loosened nothing. Fn-forced. | FAILS at the shared root, one E0277 (`Fabricated: PosSealed`), matching `42_probes`' probe_3b error shape exactly. |
| `probe_3d_malformed_types_refused.rs` | No impls fabricated; malformed TYPES pushed at bounded positions: unreduced `Ratio<P6, P12>` as `Adjustment`, unreduced `BPos<P6, P12>` as `Bias`, padded `O<Evil>` as `Pos`. | FAILS x3: two E0271 (`<O<I<H>> as Gcd<O<O<I<H>>>>>::Out == H` mismatch: the gcd is 6, reported as the type, not `H`), one E0277 (`Evil: Pos`). **First draft of this probe used bare type aliases and COMPILED CLEAN while testing nothing**: a type alias defers its bound checks. Kept on the record as the tautology it was; the committed form forces well-formedness through fn signatures. Any shipped compile-fail suite for this perimeter must force, or it will be green while asserting nothing. |
| `probe_3e_reimpl_on_genuine_inhabitant.rs` | Re-implement `Adjustment` for `Ratio<H, H>` (lying consts), `Gcd<O<H>>` for `I<H>` (lying gcd), `Pos` for `H` (lying `VAL`). | FAILS x3, E0117 (orphan rule), before any seal or overlap check is consulted. This also covers the `min_specialization` worry: the upstream impls are not `default`, and a downstream overlap is refused at orphan/coherence regardless of feature gates. |
| `probe_3f_downstream_blanket_refused.rs` | A downstream blanket (`impl<T: LocalLicence> Adjustment for T`), the route that would mint inhabitants wholesale. | FAILS, E0210 (uncovered type parameter). |
| `probe_4_gcd_local_rhs_coherence.rs` | File 42's argued-not-compiled residual (`42:360-364`), first half: does coherence ADMIT `impl Gcd<LocalRhs> for H` (local type in the trait's parameter position, against the upstream blanket `impl<B: Pos> Gcd<B> for H`)? | **COMPILES CLEAN.** Coherence admits it (rustc concludes no overlap, treating `LocalRhs: Pos` as knowably unsatisfied for a local type). The residual is real, not hypothetical: a downstream CAN hold a lying `Gcd` fact about the genuine inhabitant `H`. |
| `probe_4b_local_rhs_cannot_enter.rs` | Second half: can that lying fact reach any consuming position? Fn-forced `Ratio<H, LocalRhs>` at an `Adjustment` bound, with the evil impl in scope satisfying `H: Gcd<LocalRhs, Out = H>`. | FAILS, E0277 (`LocalRhs: Pos`): the OTHER operand position's own `Pos` bound refuses before the fabricated gcd fact is ever consulted. File 42's completeness argument (`42:146-165`), confirmed by compile in both halves. |
| `probe_4c_unbounded_operand_position.rs` | `BiasProduct<Rhs>` declares no bound on `Rhs`; does the same coherence door admit a downstream impl over a non-`Bias` right operand? | COMPILES CLEAN. Not a breach (the declared `Out: Bias` bound means only genuine inhabitants come out, and the impl fires only on the downstream's own query), but it is the one public trait in the tower whose reachability argument rests on the output bound rather than the input bounds. Hygiene recommendation: `trait BiasProduct<Rhs: Bias>`. |
| `probe_5_helpers_open_and_harmless.rs` | Helper traits carry no seal: a local type implements `Dbl` (expected clean, orphan-legal, overlaps nothing), and then tries to enter the tower through `Reduce` (expected refused). | The impl half is admitted as expected; the entry half FAILS, but with **E0275** (`Pz<O<_>>: ExactDivOdd<_>` overflow), not the crisp E0277 predicted. The refusal is real (the local type cannot enter), and the diagnostic is the composition wall's: forcing `Ratio<LocalNat, H>: Reduce` on a CONCRETE non-inhabitant diverges the same way files 41/42 measured for fully abstract parameters. A new boundary fact for the wall's residual: the eager-confirmation divergence does not require abstract operands, one rigid non-`Pos` operand suffices. |
| `probe_6_extension_positive_control.rs` | Does the seal cost legitimate extension anything? Three shapes: a new operation by structural recursion over the public constructors (`BitLen`, const-asserted), the MATLAB numeral pieces by composition (`Reduced<H, H>`, `ReducedBiasPos<H, O<H>>`, consts asserted 1/1 and 1/2), a local convention contract with sealed-trait-bounded associated types. | COMPILES CLEAN, all three, all const-asserted. |
| `probe_7_bias_as_separate_crate.rs` | Does the design's own layered-crate shape cross the seal: the bias layer as a genuinely separate crate (`--extern vu_nat_layer`) declaring its own sealed carrier over the upstream sealed carriers? | COMPILES CLEAN. One diff from `vu_bias_sealed_adj.rs`: the `#[path]` include becomes `pub use vu_nat_layer as nat;`. |
| `probe_8_dyn_refused.rs` | The type-erasure route, for completeness. | FAILS, E0038: `Pos` is not dyn-compatible (associated const `VAL`). The route does not exist even before the workspace's own no-`dyn` rule applies. |

## Verbatim error heads (key ones; full texts reproduce with the commands above)

Probe 3, the new refusal (the one probe 1b shows was missing):

```
error[E0277]: the trait bound `EvilAdj: nat::sealed::AdjustmentSealed` is not satisfied
```

Probe 3d, the unreduced pair, showing the E0271 carries the actual gcd in the type:

```
error[E0271]: type mismatch resolving `<O<I<H>> as Gcd<O<O<I<H>>>>>::Out == H`
   = note: expected struct `H`
              found struct `O<I<H>>`
   = note: required for `Ratio<O<I<H>>, O<O<I<H>>>>` to implement `Adjustment`
```

Probe 3f:

```
error[E0210]: type parameter `T` must be used as the type parameter for some local type
```

## Price (measured, `price/`)

`price/gen.py` (file 42's generator plus one kind, `alias_sealed_adj`, pointing at the
`Adjustment`-sealed tower) and `price/sweep.sh`. Same shape as files 36/41/42: `--emit=metadata`,
8-bit operands, every instantiation const-asserted against a Python-computed value. Scope honestly
stated: min-of-1, two counts (0 and 400), so the figure is a difference quotient, not a fitted slope.

| kind | ms/composition ((t400 - t0)/400) | metadata B/composition |
|---|---|---|
| `alias_sealed` (file 42's tower, baseline) | 15.91 | 2036.3 |
| `alias_sealed_adj` (this file's tower) | 16.00 | 2036.7 |

Both within noise of each other and of file 42's own 15.486 for the same baseline kind. The
`Adjustment` seal costs nothing measurable in time and under half a byte of metadata per composition.
All figures are `pin + host` facts (`aarch64-apple-darwin`), not benches.
