# Probes for panel file 10

Four files. Reproduce under `nightly-2026-05-28`, the workspace pin. Build from any
directory; each file is a standalone crate.

```
rustc +nightly-2026-05-28 --edition 2024 a_one_definition.rs -o a_repro && ./a_repro
rustc +nightly-2026-05-28 --edition 2024 b_lying_carrier_caught.rs        # expect E0080
rustc +nightly-2026-05-28 --edition 2024 c_substitution_unwritable.rs     # expect E0425 + E0599
rustc +nightly-2026-05-28 --edition 2024 -C opt-level=3 --emit asm d_reference_path_codegen.rs -o d.s
```

| File | Question | Outcome |
|---|---|---|
| `a_one_definition.rs` | can the witnessed `phi` and the executed arithmetic be one definition, generic over the payload, with a compile-time preservation check at the door | YES. `phi` is a `[const]` trait method generic over a `Payload` const trait; the checker instantiates it at a 3-bit model, the runtime at 16 bits, same body. The preservation equation `observe(pipeline(a,b)) ==_Kleene phi(a+b)` is checked exhaustively over the model at const eval. 09's reproduction case (5+4 over [0,7] under ReduceModulo) returns 1, phi's wrap, where the union returned the hardcoded 7 |
| `b_lying_carrier_caught.rs` | does the preservation check catch a carrier that lies within what parametricity permits (drops the value, claims a refusal) | YES. `error[E0080]: evaluation panicked: executed arithmetic disagrees with its verified recovery map`, at the forcing const |
| `c_substitution_unwritable.rs` | is the union's actual lie (delivery substitutes a caller-chosen clamp value) writable under the one-definition signatures | NO. `refused()` receives no payload, so returning the clamp bound is `E0425: cannot find value max in this scope`, and fabricating a payload from `T: Copy` alone is `E0599`. Caveat recorded in panel file section 3: an implementor can add its own `T: Default`-style bound and fabricate that way; probe B's check is what catches the fabricated value, so the two mechanisms are belt and braces, not either alone |
| `d_reference_path_codegen.rs` | what the reference-semantics hot path costs in emitted code | NOTHING. At `-C opt-level=3`, aarch64: the wrap pipeline is `add` + `and` + `ret`, and LLVM aliased the symbols, `_baseline_wrap = _add_wrap_12bit`, meaning the generic-phi pipeline and the hand-written mask compile to the identical function. The clamp pipeline is the same five-instruction `csel` shape as its hand-written baseline. No timing claimed; the instruction sequence is the artifact |

Two honest shortcuts inside probe A, named so nobody carries them forward as design:
`Poison::refused` uses `core::mem::zeroed` for the unobservable payload slot, which is
unsound for general `T: Copy` (a reference type would be UB); real arvo would use its own
storage primitive for the slot. And both `Payload` impls share `i64` as the wide type,
which sidesteps the per-width widening question; real arvo's `Wide` is per-width and its
adequacy (`max + max` fits) is an O(1) fact checkable at the actual width, per the panel
file's obligation taxonomy.
