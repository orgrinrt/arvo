# File 56 probe outcomes

All builds: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved
from inside `56_probes/` (verified: `rustc --version` from this directory matches
`rustc +nightly-2026-05-28 --version`, confirming file 52's warning about the bare-`rustc`-outside-
the-tree trap does not apply here).

Reused verbatim, not reproduced content, only re-linked under this dispatch's own crate names:
`tower_nat.rs` / `tower_bias.rs` (copies of `50_probes/vu_nat_sealed_adj.rs` /
`vu_bias_sealed_adj.rs`); `tower.rs` / `vu_nat.rs` / `vu_bias.rs` (copies of `47_probes/`'s
unsealed tower, used only for the E0275 wall reproduction where the seal is irrelevant);
`vu_nat_sealed_adj.rs` / `vu_bias_sealed_adj.rs` (copies of `46_probes/`'s originals, used to
rebuild `probe_2_vu_core_lib.rs` and `probe_3_direct_impls_refused.rs`, themselves copies of
`46_probes/`'s own files, to reproduce the seal's diagnostic fresh in this dispatch rather than
trust the prior file's transcription).

| probe | question | outcome |
|---|---|---|
| `probe_2_vu_core_lib.rs` + `probe_3_direct_impls_refused.rs` | Reproduce the seal's own direct-impl refusal fresh, this pin, this dispatch. | FAILS, E0277. rustc's own built-in sealed-trait detection fires with no `#[diagnostic::on_unimplemented]` anywhere in the tower: "this is usually done to force you to use one of the provided types", followed by the actual list (`H`, `O<P>`, `I<P>`). This is the strongest diagnostic this fixture found and nobody built it; it is a stock rustc UX feature for the private-supertrait pattern. |
| `probe_1_alias_expands_newtype_survives.rs` | Does an E0308 on a declaration-site mismatch name the alias, or the newtype, or the raw encoding. | Alias form: FAILS, prints the full raw nest, reproducing `47_probes/probe_1c` fresh. Const-generic newtype form (`NFace<const V: u64>`): FAILS, prints `expected 37, found 53` and `FaceContainer<NFace<37>>` / `FaceContainer<NFace<53>>` in full. The newtype is not transparent to the type checker; the alias is. |
| `probe_2_does_the_face_survive_composition.rs` | Does the face survive one operation, or decay to the raw encoding the moment something computes with it. | Shape 1 (operation generic over the raw `Nat`, face is a call-boundary label only): decays. The downstream E0308 shows the raw nest again, one hop after the readable declaration. Shape 2 (operation defined on the face itself, computing a face whose const parameter is itself computed): survives. The downstream E0308 shows `expected 37, found 90`. |
| `probe_3_multi_field_const_face.rs` | Does `adt_const_params` let a face carry more than one field, including a signed rational, and still print readably. | FAILS, E0308, prints the whole struct literal field-labelled: `expected Spec { precision: 15, bias_num: 0, bias_den: 1 }, found Spec { precision: 15, bias_num: 1, bias_den: 2 }`, and the negative-bias variant prints the negative field with no special handling needed. |
| `probe_4_the_face_is_a_new_carrier_and_needs_its_own_seal.rs` | Does a bare `adt_const_params` face refuse an unreduced pair the way the internal `Adjustment` does. | The check is EVALUATIVE, not structural: a forced-use const assertion panics with `evaluation panicked: bias is not reduced to lowest terms`, printing the offending `Spec` in full, but only when something calls the checking constructor. |
| `probe_4b_unchecked_bad_compiles_silently.rs` | The negative half: does a face nobody forces through the checking constructor compile clean with a bad spec inside it. | COMPILES CLEAN, exit 0. Confirms the hazard: `NFace<BAD>` is a legal, silently-wrong type if nothing calls `.checked()`, reproducing file 46's own "a bare alias defers its bound checks" lesson one layer up. |
| `probe_5_does_on_unimplemented_reach_e0275.rs` | Does `#[diagnostic::on_unimplemented]` on a local reproduction of the composition wall change anything. | The local reproduction did not itself diverge (compiled clean; too shallow a recursion to trip the solver's limit), so this probe is inconclusive on its own; superseded by probe 5b, which annotates the real trait directly. |
| `probe_5b_on_unimplemented_on_the_real_reduce.rs` | Does the attribute change the E0275 the real, ratified `Reduce` trait produces, annotated directly on `tower_annotated.rs`'s copy. | FAILS, E0275, byte-identical to the unannotated baseline (48's probe 1 and this dispatch's own direct reproduction of it): "overflow evaluating the requirement", the same recursion trace, the same "126 redundant requirements hidden". The attribute has zero effect. Definitive negative result: `#[diagnostic::on_unimplemented]` does not reach a solver-overflow diagnostic; it customises "no impl found" (E0277), never "the solver gave up" (E0275). |
| `probe_5c_on_unimplemented_on_the_rigid_non_inhabitant.rs` | `46:6.2`'s own separate residual: does the attribute change the E0275 that fires on a CONCRETE rigid non-inhabitant (`LocalNat`), not only on an abstract type parameter. | FAILS, E0275, byte-identical to probe 5b's abstract case, same "126 redundant requirements hidden" count. Both named shapes of the residual are inert against this instrument. |
| `probe_6_the_face_seal_as_a_real_bound.rs` | Can the face's reducedness obligation be stated as a real bound (`Assert<{S.is_reduced()}>: True`) instead of a forced-use panic. | FAILS to compile at all: `generic parameters may not be used in const operations`, `add #![feature(generic_const_exprs)]`. The spine rule's wall recurs a sixth time, now at the notation layer itself: a boolean computed from a generic const cannot be placed in type position without the forbidden feature. |
| `probe_7_concrete_newtype_per_numeral_has_no_sealing_question.rs` | Does a concrete, non-generic, macro-minted-only newtype per numeral sidestep probe 6's sealing hazard entirely. | FAILS, E0308, `expected Container<Q37>, found Container<Q53>`. No generic parameter, no sealing question to ask: nothing except the macro's own emitted `impl` can associate a name with an encoding, so there is no attacker position. |
| `probe_8_the_strongest_combination.rs` | Combine file 47's bound-not-equality lever with this file's concrete-face lever. | FAILS, E0277, fully custom: `expected accumulator width \`Q37\`, this one is \`Q53\`` with a remediation note. The strongest message this fixture produced, and it costs one bound plus one attribute, nothing structural. |

## Verbatim diagnostics, the five load-bearing ones

**The seal, unprompted, this dispatch (`probe_3_direct_impls_refused.rs`):**

```
error[E0277]: the trait bound `EvilPos: nat::sealed::PosSealed` is not satisfied
  --> probe_3_direct_impls_refused.rs:20:14
   |
20 | impl Pos for EvilPos {
   |              ^^^^^^^ unsatisfied trait bound
   |
help: the trait `nat::sealed::PosSealed` is not implemented for `EvilPos`
   ...
   = note: `Pos` is a "sealed trait", because to implement it you also need to implement
     `vu_core::nat::sealed::PosSealed`, which is not accessible; this is usually done to
     force you to use one of the provided types that already implement it
   = help: the following types implement the trait:
             vu_core::nat::H
             vu_core::nat::O<P>
             vu_core::nat::I<P>
```

**The alias decoder-ring, fresh (`probe_1_alias_expands_newtype_survives.rs`):**

```
error[E0308]: mismatched types
   |
37 |     needs_face37(c);
   |     ------------ ^ expected `Container<Pz<I<O<I<O<O<H>>>>>>>`, found `Container<Pz<I<O<I<O<I<H>>>>>>>`
```

**The const-generic face survives (`probe_1_alias_expands_newtype_survives.rs`, same probe, other half):**

```
error[E0308]: mismatched types
   |
61 |     needs_nface37(c);
   |     ------------- ^ expected `37`, found `53`
   |
   = note: expected struct `FaceContainer<NFace<37>>`
              found struct `FaceContainer<NFace<53>>`
```

**The multi-field face (`probe_3_multi_field_const_face.rs`):**

```
error[E0308]: mismatched types
   |
55 |     declare_q15(x);
   |     ----------- ^ expected `Spec { precision: 15, bias_num: 0, bias_den: 1 }`, found `Spec { precision: 15, bias_num: 1, bias_den: 2 }`
```

**The strongest combination (`probe_8_the_strongest_combination.rs`):**

```
error[E0277]: expected accumulator width `Q37`, this one is `Q53`
   |
42 |     needs_q37(acc);
   |     --------- ^^^ declared with the wrong numeral face
   = note: faces are minted only by the numeral-literal macro; if this is the right VALUE but
     the wrong SPELLING, re-emit it from the macro rather than editing the face by hand
```

## Price sweep (`price/`)

Same discipline as files 41, 42, 53, 54: `--emit=metadata`, min-of-3, difference quotient between
N = 0 and N = 60 distinct items, no codegen.

| variant | min, N=0 | min, N=60 | marginal ms/item |
|---|---|---|---|
| `variant_a` (bare `Tag<const V: u64>`, no seal check) | 61.6 ms | 69.2 ms | **0.127** |
| `variant_b` (const-struct `Spec` face, forced seal check via `.checked()`, reduced pairs) | 84.9 ms | 95.6 ms | **0.178** |

Both are one to two orders of magnitude cheaper than the internal tower's own composition cost
(2.1 ms/item dyadic, 143 ms/item worst-case 16-bit random rational, `53:129-137`), because the
face's own seal check is O(1) const-fn integer arithmetic, not O(depth) recursive trait
resolution. The face layer's own cost is not the review's cost problem; the tower underneath it
remains exactly what file 53 and file 54 priced.

## Gates

`cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9 ignored, re-run for this dispatch,
matching every file since 41. Both canon-gate greps (`Adjustment\|Bias\|Numeral` and
`FullRange\|UTerm\|AddWidth` against `mock/crates/ --include="*.rs"`) exit 1, empty, reproduced
fresh from the repo root.
