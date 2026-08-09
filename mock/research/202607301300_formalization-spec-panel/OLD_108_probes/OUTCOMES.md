# 108 probe outcomes

Toolchain for every entry below: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the repo pin, invoked
from inside the tree. Verified this session that a bare `rustc` outside the tree resolves to `rustc
1.94.0 (4a4ef493e 2026-03-02)`, stable, which would silently change several of these results. Target
`aarch64-apple-darwin`. Optimisation flag stated per entry.

| Probe | Claim | Flags | Outcome |
|---|---|---|---|
| `p1_three_way.rs` | A producer declaration bound on the algebra admits both a one-lane and a two-lane truth type; the two-lane one has no exit and reduces only through an inherent named word | `-O`, lib | COMPILES |
| `p2_bound_on_exit.rs` | The same design with the declaration bound on the exit-carrying part instead | none, lib | `E0277`, `Mask2: Branch` not satisfied, at `type Truth = Mask2` |
| `p2b_wrapper_relocates_the_choice.rs` | The repair for p2: the producer names `All<Mask2>` as its truth | `-O`, lib | COMPILES, and the call site reads `a.eq(b).is_true()` while silently meaning all-lanes, with no route to any-lane |
| `p3_exit_cost.rs` | Raw primitive, concrete truth newtype, and generic-over-truth, one branch each | `-O --emit asm` | `_b_concrete = _a_raw`, `_c_generic_at_scalar = _a_raw`. Three spellings, one symbol |
| `p4_default_routes.rs` | Five introduction routes for an unnamed exit: trait default body over an algebra carrying equality, blanket impl, inherent method of the same name, `Deref`, `From`/`Into` | `-O` | ALL FIVE COMPILE. R1 silently means all-lanes, R5 silently means any-lane, both looking canonical |
| `p4b_enforcement.rs` | `impl !Branch for Mask2 {}` under `negative_impls` alone | `-O` | COMPILES; the absence becomes a declaration |
| `p4d_negonly_blanket.rs` | Route R2 written against a declared absence, `negative_impls` only, no `with_negative_coherence` | `-O` | `E0751`, positive and negative implementation both named |
| `p4e_negonly_directimpl.rs` | Route R1 written against a declared absence, same features | `-O` | `E0751` |
| `p5_select_vs_exit.rs` | `max` written once against a selector keyed on the pair, no exit anywhere, at one and two lanes | `-O --emit asm` | COMPILES at both; `_max_scalar = _max_raw` |
| `p5b_exit_max_is_wrong.rs` | Is a lane-wise `max` routed through an exit unavailable, or wrong? | `-O`, bin, executed | lane-wise `[7,9]`; exit with `all` `[7,2]`; exit with `any` `[3,9]`. Two different wrong answers |
| `p6_arity_blanket.rs` | Index the algebra by lane count, blanket the exit at one lane | `-O` | COMPILES; the exit is derived from arity rather than chosen |
| `p6b_mask_cannot_branch.rs` | Does that blanket leak the exit onto the mask? | `-O` | `E0599`, "doesn't satisfy `Mask2: Branch` or `Mask2: TruthAlgebra<1>`" |
| `p6c_manual_override.rs` | Can a one-lane type override its derived exit? | `-O` | `E0119`, conflicting implementations |
| `p6d_cost_under_blanket.rs` | Cost of the arity-blanket shape | `-O --emit asm` | `_b_concrete = _a_raw`, `_c_generic_at_scalar = _a_raw` |
| `p7_const_form.rs` | The whole split in the design's own idiom: `pub const trait`, `const impl`, `[const]` bounds, exit reached in const position | `-O`, `const_trait_impl` | COMPILES; `const _: () = assert!(...)` holds at compile time; the two-lane instance coexists |
| `p8_two_selects.rs` | Is file 103's thunked selector on the truth type an escape from the exit? | `-O` | COMPILES in both directions. `exit_from_select` and `select_from_exit` are total and mutually definable, so the two are the same object |
| `p9_homs.py` | Which functions from the n-lane truth algebra to the one-lane one preserve the Boolean-algebra structure? Exhaustive over all `2^(2^n)` candidates, n = 2 and n = 3 | python3 | n=2: 2 homomorphisms of 16 candidates; n=3: 3 of 256. Both times exactly the coordinate projections. `all` is not one. `any` is not one |

## Notes on two entries

`p3` was rewritten once. The first form returned `x` or `y` from a comparison of `x` against `y`, which
LLVM correctly folded to `return y` for every arm, giving a two-instruction body that measured nothing.
The rewritten form returns two distinct constants.

`p9` is exhaustive rather than sampled: every function from the n-lane carrier to the one-lane one is
enumerated and tested against all five equations at every pair of points. The result is stated for n = 2
and n = 3 because those are the smallest instantiations where the distinction is nonvacuous, and the
finite-Boolean-algebra result it reproduces is standard.
