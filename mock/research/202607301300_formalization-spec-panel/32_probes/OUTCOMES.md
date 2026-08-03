# Probes for panel file 32, does identity lower well

Two artifacts, both compiled or run against `nightly-2026-05-28`, the workspace pin
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, matching file 31's own gate). `identity_model/` is a
standalone crate (`[workspace]` with no members, so `cargo`'s auto-discovery cannot splice it into
the real `mock/Cargo.toml`) modelling the settled identity contract from `31:326-359`: `Numeral`
(Radix, Precision, `ExponentForm` nesting `Adjustment`/`Bias`/`Underflow`/`Specials`, `Domain`) and
`Encoding` nested inside `Lowering` (`SignIndexing`, `FieldLayout`, `Canonicalisation`). It carries
`#![feature(const_trait_impl)]` only, WATCH-allowed per `unstable-features.md`, same as file 08's
`union.rs`. `gen_identity_sweep.py` is the generator for the compile-cost sweep, mirroring
`08_probes/i_gen_monomorphisation_sweep.py`'s exact methodology.

To reproduce the compile-time sweep: from `identity_model/`, `cargo build --release --lib`, then for
each `K`, `python3 ../gen_identity_sweep.py K src/bin/swK.rs`, then
`rustc --edition 2021 -O -C lto=fat -C codegen-units=1 -C panic=abort --extern
identity_model=target/release/libidentity_model.rlib -Z time-passes src/bin/swK.rs -o /tmp/swK`, read
`monomorphization_collector_graph_walk` and the final `total` line, and `nm -U /tmp/swK | wc -l` for
the symbol count. The generated `src/bin/swK.rs` files are not committed, matching file 08's own
practice with its sweep outputs; the generator is the artifact.

To reproduce every codegen-inspection result: `rustc --edition 2021 -C opt-level=3 -C lto=fat -C
codegen-units=1 -C panic=abort --crate-type lib --emit=obj,asm identity_model/src/lib.rs -o
identity_model.o`, then `objdump -d --no-show-raw-insn identity_model.o` (or `llvm-objdump`), filtered
to the `probe_*` symbols. Every `probe_*` function is `#[no_mangle] #[inline(never)]` specifically so
it survives to its own disassemblable symbol, per the consolidation's own check-build discipline
(`26_consolidation_two.md` section 1.6: defeat the inliner deliberately so "what did this axis
generate" has an answer). The layout claims (`size_of`/`align_of` on `Number<N, S>` and on every axis
marker type) are `const` assertions at the bottom of `lib.rs`; the crate not compiling **is** the
check, per the same discipline `arvo-storage/src/layout_assertions.rs` already uses in shipped source.

| Question | Method | Outcome |
|---|---|---|
| What does a distinct identity-side composition cost to monomorphise | `gen_identity_sweep.py`, K in {1,10,40,100,200,400}, `-Z time-passes` | sub-millisecond to low-single-digit-millisecond `monomorphization_collector_graph_walk`, not clearly linear in K over this range; symbol count flat at 557 across the full 400x range. Scoped: this measures the type-level axis nesting alone, not the law/witness apparatus file 08 measured through the same call. |
| What do `Specials` cost the `Implicit` (fixed-point) path | `probe_classify_implicit` vs `probe_classify_ranged`, disassembled | Two structurally separate, independently-compiled function bodies (`Implicit` has no `Specials` parameter to name). Both branchless (`csel`); `Implicit` is 5 instructions, `Ranged`/`WithInfNaN` is 6. Cost is proportional to what specials deliver, not a runtime flag on a shared body. |
| What does a canonicalisation obligation cost per op | `add::<Fix...>` (`Canonical = IdentityCanon`) vs `add::<Float...>` (`Canonical = NaNCanon`), disassembled | `IdentityCanon` compiles to nothing: the whole `add` is 2 instructions (`add x0,x1,x0; ret`), identical to a bare `wrapping_add`. `NaNCanon` (a single-representative NaN-band collapse) costs 5 extra branchless instructions (7 total), no branch. Scoped: a richer canonicalisation policy (payload-preserving NaN propagation) was not modelled and may not stay branchless. |
| What does the richer `Encoding::Fields` cost a bitpacked column | `probe_bitpacked_column_sum` (through `extract_field::<PlainFields>`) vs `probe_bitpacked_column_sum_raw_baseline` (hand-written shift and mask), disassembled | Byte-identical function bodies, 14 instructions each, same opcodes at the same relative offsets. |
| Does the type-level machinery leave anything at runtime | `const` assertions on `size_of`/`align_of` of `Number<N,S>` (both `Implicit` and `Ranged` sides) and of every individual axis marker type | Confirmed zero-sized for every marker; `Number<N,S>` is exactly its raw payload width on both sides. The crate compiling is the evidence. |
| Does the shape a consumer writes vectorise | A minimal standalone control (`/tmp/vec_control.rs`, not part of this dir; see file 32 body) vs the same scalar shape reached through the full identity contract, vs the same scalar shape reached through a bare ablation, all disassembled | The minimal standalone control autovectorises (NEON `add.2d`) on this toolchain and target. The identical scalar shape, whether reached through the identity contract or through a bare non-generic ablation, did **not** vectorise once co-located in the `identity_model` crate under identical flags; isolated to a crate-level build-context effect (confirmed by embedding a verbatim copy of the vectorising control inside `identity_model` and finding it, too, stops vectorising there), not a fact about the identity contract. Root cause not pinned within this dispatch's scope; named as an open item. |

Mechanism notes, recorded so they are not rediscovered.

`generic_const_exprs` is forbidden per `unstable-features.md`; deriving `Precision` or the exponent's
negative-`F` value from `I + F` inside an impl (`type Precision = P<{ I + F }>`) hits the identical
`error: generic parameters may not be used in const operations` wall the consolidation's droplist
already names for the multiplicative half's width arithmetic. `identity_model` sidesteps it the same
way the multiplicative half did: the derived quantities (`PBITS`, `NEG_F`) are carried as independent
const parameters supplied at the call site, not computed in const position.

`S: Policy + Lowering` (the fused two-parameter form, `26_consolidation_two.md:34`) means every
`*Lowering` marker used as `S` needs its own (here trivial) `Policy` impl; a `Lowering`-only type
cannot stand in for `S` on its own.

`cargo init --lib` run inside a directory tree under an existing Cargo workspace silently splices the
new crate's relative path into the enclosing workspace's `Cargo.toml` `members` list. This happened to
this probe on first init and modified the real `mock/Cargo.toml`; reverted, and the standalone
`[workspace]` table added to stop it recurring. Worth a general note for any future probe nested under
a repo's `mock/` tree.

A no_std cdylib does not link on macOS arm64 without `libSystem` (`dyld_stub_binder` unresolved); the
codegen-inspection probes use `--crate-type lib --emit=obj,asm` on an unlinked object file instead,
which needs no link step and is sufficient for `objdump`/`nm`. `bl` and `b` targets to non-local
symbols show as self-referential branches in `objdump -d` on an unlinked `.o` (pending
`ARM64_RELOC_BRANCH26` relocations); `objdump -d -r` resolves the symbol name per relocation entry and
was used to confirm every apparent "infinite loop" in the raw disassembly is a tail call or a call
site, not a miscompile.
