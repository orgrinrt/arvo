### mock/research/202611130520_datum-a-cold-second-read.md
- date: 202611130520 (file name stamp)
- by: not stated (line 353: "I carry no named persona")
- holds: A second, cold read of `question::what_a_datum_stands_for`, written before and after opening the first expert's proposal file. It sets out the route by which the question closes, its own answer in clauses A to G, a comparison with the first expert's two proposal rows, and what the two reads would and would not supply for a promotion.
- headline: "By the route in 1, the two reads do not close it, and the gate is not ours in any case." (line 323)
- watch: "if two separate experts agree on it heavily and can reason and spell it out, that is a ratification for canon, though distinct from something I've blessed." (line 32)
- watch: "ratified, by op): "Every remaining canon question is the panel's. [...] where his words genuinely do" (line 42)
- watch: "Under wrapping no element but the additive identity absorbs anything" (line 108)
- watch: "Its operations are functions of the declared signature and never of the carrier:" (line 180)
- watch: "panic "for what cannot be caught at compile time through the typestate"." (line 193)

### mock/research/2026_05_05_doc_audit.md
- date: 2026-05-05
- by: not stated
- holds: An audit of the per-crate documentation templates across 16 crates, listing per-crate findings (doc-vs-source drift, internal contradictions, Tier-1 leakage, banned vocabulary), cross-cutting patterns, and a suggested two-round scope for the fixes.
- headline: "Total findings: roughly 95 across 16 crates." (line 11)
- watch: "source `aliases.rs` defines only `Uint<const N: u16, S = Warm>` and `Int<const N: u16, S = Warm>`" (line 19)
- watch: "Fix: rewrite the Warm section to match the post-#316 reality (Warm matches Hot's projection, byte-sequence storage above 128)." (line 29)
- watch: "Source: `BitPrim` is `pub const trait BitPrim` in `arvo-bits-contracts/src/lib.rs:159`; not internal to arvo-bits, not sealed in the standard sense." (line 156)
- watch: "`MultiContainer<HiT, LoT>` was deleted entirely in round 202605031400 / #316 and replaced by `WideBits<BYTES, A>` / `AlignedWideBits16<BYTES, A>`." (line 308)

### mock/research/2026_05_05_doc_audit_entrypoints.md
- date: 2026-05-05
- by: not stated
- holds: An audit of the repo-root README, the three `mock/` root templates and the rendered `docs/*` output against the documentation and writing rules, listing round-number, mockspace and auto-gen-header leakage per file, cross-cutting patterns and a suggested topic-file scope. Companion to the per-crate audit.
- headline: "The arvo entrypoint leg of the workspace doc round wants to address:" (line 68)
- watch: "`arvo-refit | Re-export gateway... Renamed from arvo-narrow in round 202604301700.`" (line 21)
- watch: "`during round 202604300826), arvo-mask-contracts, arvo-numeric-` paragraph wrap." (line 32)
- watch: "Every trait whose methods are sensibly callable in const context is `pub const trait`." (line 19)

### mock/research/2026_05_28_arbitrary_limitations.md
- date: 2026-05-28
- by: not stated
- holds: A catalog of hardcoded width and capacity limits across arvo crates, each marked arbitrary or justified and open or resolved, fulfilling the exit condition of design round `202605111741`. It names the follow-up rounds each open category drives and fixes no signature.
- headline: "Categories 1 through 4 are the arbitrary limitations the foundations should lift so that generic surfaces and named defaults replace the baked literals." (line 61)
- watch: "The principle the round stated: the foundations ship generic surfaces and named defaults, never hardcoded literals masquerading as foundation properties." (line 3)
- watch: "no mathematical or representational necessity forces the value (it is not a property of the bit width, the algorithm, or the IEEE/native primitive ladder)." (line 14)
- watch: "Hot and Cold project to a native bucket up to N=128; Warm and Precise up to N=64; both fall to the wide bucket above." (line 40)
- watch: "Still-open sites include `arvo-graph` (rank, topo, path, waist, components, spanning), `arvo-sparse` (rcm, block, dm), and `arvo-comb`." (line 24)

### mock/research/README.md
- date: none stated
- by: not stated
- holds: An index of the research directory, listing the files imported verbatim from the polka-dots and saalis workspaces and naming material left behind that is not copied here.
- headline: "Source material backing the arvo designs." (line 3)
- watch: "`design_rounds/202603241200_topic.arvo-strategy-markers.md`: origin of the Hot/Warm/Cold/Precise strategy markers. Rank ordering, cross-strategy resolution." (line 10)
- watch: "`rust-nightly-features-for-type-constraints.md`: nightly-only features that arvo leans on (const-generic expressions, ConstParamTy, specialization, etc.). Grounds arvo's nightly-by-default stance." (line 9)

### mock/research/arvo-bitmask-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-bitmask`, listing the round-id source citations migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-bits-contracts-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-bits-contracts`, listing the round-id source citations, design round files and sketch paths migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-bits-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-bits`, listing the round-id source citations migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-comb-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-comb`, listing the round-id source citations migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-deepdive_strategy-bound-trilemma-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for the strategy-bound trilemma deep dive, listing the round and changelist sources, the original research note and three Rust tracking issues.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-graph-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-graph`, listing the round-id source citations migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-hash-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-hash`, listing the round-id source citations migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo`, listing the round-id source citations migrated out of the Tier 1 design template, including the strategy-marker topics.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)
- watch: "202603241200_topic.arvo-strategy-markers.md (strategy definitions)" (line 11)

### mock/research/arvo-mask-contracts-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-mask-contracts`, listing the design round sources migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-numeric-contracts-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-numeric-contracts`, listing two design round sources migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-refit-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-refit`, listing three design round sources migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-sparse-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-sparse`, listing the round-id source citations migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-spectral-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-spectral`, listing the round-id source citations migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-storage-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-storage`, listing design round sources, sketch validation notes for `WideBits<BYTES, A>`, a senior architectural audit verdict and the nightly compiler used for a spike.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-strategy-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-strategy`, listing design round sources, sketch validation notes for `WideBits<BYTES, A>` over `Align` markers, and a senior architectural audit verdict.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-tensor-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-tensor`, listing the round-id source citations migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/arvo-transparent-history.md
- date: none stated (cites the doc-audit round of 2026-05-05)
- by: not stated
- holds: A Tier 2 audit-trail file for `arvo-transparent`, listing two design round sources migrated out of the Tier 1 design template.
- headline: "Round identifiers, deprecated changelist filenames, and internal sketch paths are recorded here for contributors; they no longer appear in the rendered Tier 1 output." (line 6)

### mock/research/audits/129_lint_allow_audit.md
- date: 2026-04-21
- by: not stated
- holds: An audit of every `lint:allow(arvo-types-only)` and `lint:allow(no-bare-numeric)` site in the arvo workspace source (task #129), classifying each as structural, externally blocked or deferred, with collateral findings for other repos.
- headline: "**Verdict:** all 5 unique source-file sites are structural or externally blocked; zero fixable-now replacements." (line 5)
- watch: "round: "even where lints technically allow, prefer arvo primitives" (line 14)
- watch: "`BitPrim` trait (`self.0.get_bit(idx.0 as u32)`). `BitPrim` is an" (line 54)
- watch: "All 15 annotations across 5 sites are genuinely necessary under the" (line 96)

### mock/research/audits/2026_05_02_expert_a_architectural_dogfooding.md
- date: 2026-05-02
- by: Expert A (title: "Expert A, 2026-05-02")
- holds: A dogfooding audit of branch `feat/usize-const-arith` listing 18 findings where arvo fails to consume its own abstractions (hand-rolled parallel types, bare primitive literals, duplicate trait surfaces), each with location, smell, generalisation and priority, and a summary of structural patterns.
- headline: "arvo has built rich abstractions (`Bits<N, S, Sign>`, `MultiContainer<HiT, LoT>`, `Bounded`, `Identity`, `BitPrim`, `BitLogic`) but does not consistently consume them itself." (line 5)
- watch: "Two parallel const-trait surfaces ship for the same answer." (line 37)
- watch: "`BitPrim` declares `const WIDTH: u16`. `HasBitWidth` declares `const WIDTH: USize`. Two trait surfaces for the same axis at different types." (line 101)
- watch: "**Smell:** Two unrelated traits sharing the name `BitPrim`." (line 177)

### mock/research/audits/2026_05_02_expert_b_const_trait_completeness.md
- date: 2026-05-02 (directory-adjacent file name stamp)
- by: Expert B (file name)
- holds: A const-trait completeness audit of arvo on branch `feat/usize-const-arith`, listing 40 findings on traits and impls not yet `const`, six missing const-bridge traits, cycle hazards and missing const-context smoke tests.
- headline: "arvo is now genuinely const-callable along the spine." (line 9)
- watch: "`core::ops::Deref` and `core::convert::AsRef` are NOT const-stable in stdlib, so direct `impl const Deref` is rustc-blocked." (line 97)
- watch: "`pub const trait OverflowPolicy: sealed::Sealed + Copy + Clone + Default + 'static { const DISCRIMINANT: u16; }`" (line 179)
- watch: "`Strategy::NAME` is `#[cfg(debug_assertions)]`. A `const _: &'static str = <Hot as Strategy>::NAME;` works in debug but fails in release" (line 557)
- watch: "1. **`ConstEq`** (and `ConstOrd` as supertrait): `arvo-strategy`." (line 609)

### mock/research/findings_graph_spectral_202605111719.md
- date: 202605111719 (round id in the title and file name)
- by: not stated
- holds: Findings for round 202605111719: two bench bundles (RCM and Fiedler spectral bisection) run through the bench harness at N of 16, 32 and 64, with tables of timings, the `Cap` const-generic workaround, and the in-scope and deferred follow-ups.
- headline: "These bounds clear the budget that hilavitkutin's plan stage uses" (line 118)
- watch: "plan compilation). The choice of W (Bits<64> vs wider) and operator" (line 120)
- watch: "arvo-toolbox-not-policer principle, with these numbers as one" (line 122)

### mock/research/imported-from-polka-dots/design_rounds/202603241200_topic.arvo-strategy-markers.md
- date: 2026-03-24
- by: not stated
- holds: A polka-dots design topic (status IN PROGRESS) defining four strategy markers, `Hot`, `Warm`, `Cold` and `Precise`, for `UFixed<I, F, S>` and `IFixed<I, F, S>`: container choice, same-width and cross-width arithmetic, bitpacking, SIMD, a comparison table, layout derivation, strategy conversion, semantic aliases and open questions.
- headline: "The strategy marker IS the layout decision." (line 204)
- watch: "**Same-width arithmetic:** wrapping. Single instruction per op. May" (line 62)
- watch: "**Same-width arithmetic:** saturating. Overflow clamps to max/min" (line 148)
- watch: "1. **Strategy resolution:** if S1 != S2, the result strategy is the" (line 251)
- watch: "more conservative: Precise > Cold > Warm > Hot." (line 252)
- watch: "Consumers can override the strategy: `UFixed<0, 8, Hot>` is a" (line 298)
- watch: "in debug mode (e.g. panic on chain overflow in debug, wrap in release)?" (line 306)

### mock/research/imported-from-polka-dots/design_rounds/202603241300_topic.arvo-strategy-aliases-and-open-questions.md
- date: 2026-03-24
- by: not stated
- holds: A polka-dots design topic (status IN PROGRESS) that makes the semantic aliases generic over the strategy with a default, and resolves four open questions from the strategy-markers topic: Warm in debug versus release, signed containers, the maximum logical width and u128, and mixed-strategy columns.
- headline: "**Resolution: Warm is the same everywhere.** No debug/release split." (line 57)
- watch: "container, that's a logic error in the consumer's code, and it wraps" (line 63)
- watch: "If you want debug assertions on overflow, use Precise — it saturates" (line 66)
- watch: "Signed saturation clamps to `i_MIN..i_MAX` of the logical range" (line 80)
- watch: "**Resolution: I + F <= 64. No u128 containers.**" (line 87)
- watch: "Therefore: Warm and Precise for I + F > 32 use u64 as the container" (line 98)

### mock/research/imported-from-polka-dots/design_rounds/202603241400_topic.arvo-crate-dependency-revision.md
- date: 2026-03-24
- by: not stated
- holds: A polka-dots design topic (status IN PROGRESS) introducing `arvo-bits` as a bit-level contract crate, revising the arvo crate dependency tree, and stating that `arvo-graph`, `arvo-sparse`, `arvo-spectral` and `arvo-comb` are generic over numeric trait bounds rather than concrete arvo types.
- headline: "Note: arvo-graph, arvo-sparse, arvo-comb do NOT depend on arvo-bits" (line 98)
- watch: "W: Add<Output = W> + TotalOrd + Copy + FromConstant" (line 197)
- watch: "If W is `UFixed<16, 0, Hot>`, rank accumulation wraps on overflow." (line 204)
- watch: "If W is `UFixed<16, 0, Precise>`, it saturates. The algorithm is" (line 205)
- watch: "identical. The consumer chooses the safety level by choosing W." (line 206)
- watch: "Default strategy is Hot for bit-level aliases — these are typically" (line 179)

### mock/research/imported-from-polka-dots/design_rounds/202603241500_changelist.doc.md
- date: 2026-03-24
- by: not stated
- holds: A polka-dots doc changelist (status IN PROGRESS) for the hilavitkutin and arvo crate family, comparing against a deprecated changelist, listing the revised 10-crate structure with `arvo-bits` added, and the per-crate documentation changes.
- headline: "was the first doc CL attempt. It was deprecated because the arvo" (line 15)
- watch: "- Strategy-generic semantic aliases (`Angle<S: Strategy = Warm>`)" (line 25)
- watch: "BIT_WIDTH is logical width, size_of is physical (container) width." (line 157)

### mock/research/imported-from-polka-dots/rust-nightly-features-for-type-constraints.md
- date: 2026-03-13
- by: not stated
- holds: Research into Rust unstable features for compile-time layout enforcement through the trait system, surveying six const-generic and const-trait features plus build-infrastructure features, with risk assessment, an adoption recommendation, and an appendix scoring all 66 Rust 2026 project goals.
- headline: "**`min_generic_const_args`** is the direct and complete solution:" (line 741)
- watch: "The design principle: **traits are the enforcement mechanism**. If a" (line 82)
- watch: "The `~const` bound means \"const if the caller is const, runtime" (line 280)
- watch: "| `const_trait_impl` | `#![feature(const_trait_impl)]` | Const-evaluable trait methods, essential for compile-time schema work |" (line 896)
- watch: "| `generic_const_exprs` | Dead path, superseded, triggers ICEs in combination with features we use |" (line 917)

### mock/research/imported-from-saalis/design_rounds/202603170810/202603170800_topic.primitives-design.md
- date: 2026-03-17
- by: not stated
- holds: A saalis design round topic for `saalis-primitives`, stating that the crate holds only `repr(C)` atoms crossing the ABI boundary: seven enums with explicit discriminants and an ABI version constant, with a list of what does not belong there.
- headline: "Raw `repr(C)` atoms that cross the ABI boundary. Nothing else." (line 15)
- watch: "The crate is `#![no_std]`, zero dependencies." (line 10)

### mock/research/imported-from-saalis/design_rounds/202603170810/202603170810_changelist.doc.lock.md
- date: 2026-03-17
- by: not stated
- holds: A locked saalis doc changelist naming one file to write, the `saalis-primitives` design template with its seven enums, ABI version constant and design rules.
- headline: "Write complete design: 7 repr(C) enums with variant tables," (line 6)

### mock/research/imported-from-saalis/design_rounds/202603170810/202603170815_changelist.src.lock.md
- date: 202603170815 (file name stamp)
- by: not stated
- holds: A locked saalis source changelist naming one file, `saalis-primitives/src/lib.rs`, to hold seven `repr(C)` enums and one constant.
- headline: "7 repr(C) enums + 1 constant. Exactly as documented in DESIGN.md.tmpl." (line 4)

### mock/research/sketches/202605031400_hlist_heterogeneous_container/08_strategy_aware_native_buckets.md
- date: 2026-05-03 (frontmatter `date`)
- by: not stated
- holds: An apply-time finding that sketch 07's Strategy-erased native bucket projection does not match production arvo's Strategy-aware bucket boundaries, with the revert taken, four redesign options, a recommended scope reduction and a discipline lesson.
- headline: "outcome: FAILS WITH design gap — sketch 07's Strategy-erased native bucket impls don't match production arvo's Strategy-aware bucket boundaries." (line 3)
- watch: "- **Hot / Cold**: minimum aligned per bucket. `1..=8 -> u8`," (line 72)
- watch: "- **Warm / Precise**: 2x logical per bucket. `1..=8 -> u16`," (line 75)
- watch: "because the four strategies make different precision/throughput" (line 102)
- watch: "tradeoffs (Hot wraps in min container; Warm has overflow headroom" (line 103)
- watch: "in 2x; Precise saturates in 2x; Cold bitpacks). The bucket boundary" (line 104)
- watch: "`#[diagnostic::on_unimplemented]` note." (line 79)

### mock/research/sketches/202605031400_hlist_heterogeneous_container/README.md
- date: 2026-05-03T14:00Z
- by: not stated
- holds: The index for six sketches validating the container redesign, recording the cons-list padding finding, the design pivot to byte-sequence storage above 128 bits, a per-sketch outcome table and the architecture pillars each one validates.
- headline: "**Status**: ALL SIX SKETCHES COMPLETE — ARCHITECTURE VALIDATED, READY FOR #316 DOC CL" (line 4)
- watch: "**The same issue affects the existing `MultiContainer<u64, u128>`**: verified at `/tmp/mc_existing.rs` runtime — 32 bytes, identical to `MultiContainer<u128, u128>`." (line 26)
- watch: "- **Hot**: SIMD-aligned byte-sequence (`#[repr(C, align(N))]` where N matches the largest SIMD vector targeted, e.g., 32 for AVX-2, 16 for SSE2/NEON)." (line 43)
- watch: "- **Precise**: align-1 byte-exact. Saturating semantics; full precision." (line 46)
- watch: "`MultiContainer<HiT, LoT>` and `MultiContainerHalf` are **deleted** in #316" (line 50)

### mock/research/sketches/202605031548_round_4_const_hash_and_narrowing/README.md
- date: 2026-05-03T15:48Z
- by: not stated
- holds: The index for two Round 4 sketches validating `NarrowFromU64` orphan coherence and cross-crate `ConstHash` dispatch, with three findings (the `[const]` host-effect bound, sealing `Project` rather than demoting `Picker`, and the propagation pattern) and what the doc changelist captures.
- headline: "**Status**: BOTH SKETCHES PASS WITH TWO FINDINGS APPLIED. READY FOR DOC CL." (line 4)
- watch: "pub const trait ConstHash<const N: u16, S: Strategy, Sign: Signedness>: Sized" (line 35)
- watch: "<S as BitsContainerFor<N, Sign>>::T: [const] NarrowFromU64<N, S, Sign>," (line 38)
- watch: "- Bridge family (ConstFrom / ConstTryFrom / ConstDeref / ConstAsRef) follows the same `[const]` host-effect pattern wherever bounds project through const-trait associated types (Finding 3 corollary)." (line 94)

### mock/research/sketches/202605040602_bitfield_macro_routing/README.md
- date: 2026-05-04T06:02Z
- by: not stated
- holds: A sketch asking whether the bitfield macro's hardcoded mask construction can be routed through `BitPrim::mask_low` and stay const-callable on the dispatched container, concluding it works and the hardcoded patterns come out.
- headline: "`WORKS`. The probe compiles with `pub const trait BitPrim` and `impl const" (line 44)
- watch: "`BitPrim::mask_low` saturates at `WIDTH`: passing `n >= WIDTH` returns the" (line 65)
- watch: "all-ones value, passing `n == 0` returns zero." (line 66)
- watch: "`lint:allow(no-bare-numeric)` markers come out." (line 61)

### mock/research/sketches/202605040602_maskops_blanket_shape/README.md
- date: 2026-05-04T06:02Z
- by: not stated
- holds: A sketch resolving the shape of a blanket `MaskOps` impl for the `Mask<W>` chassis, choosing to drop the `const W: Width` parameter, with the probe outcome and what the source changelist applies.
- headline: "`WORKS` (path a). The reshaped trait drops the const-Width parameter; the" (line 49)
- watch: "satisfy arvo's \"tools, not policy\" stance: consumers pick the chassis" (line 35)
- watch: "flavor (`Mask<Bits<64, Hot, Unsigned>>` or `Mask<Bits<256, Hot, Unsigned>>`)" (line 36)
- watch: "bounds on the bit-trait family." (line 51)

### mock/research/sketches/202605111110_debug-smoke/FINDINGS.md
- date: 202605111110 (directory stamp; the file cites round `202605111110`)
- by: not stated
- holds: A finding on the `numeric_debug.rs` smoke test, kept outside `mock/crates/` because restoring it there would reattach the deleted tier, with the corrections about `Warm` and `Additive`, what has to be true before it runs again, and the obligation that outlives it.
- headline: "**Outcome: INCONCLUSIVE, and it cannot be anything else here.** The check does" (line 3)
- watch: "It was written for round `202605111110` against a crate tree that has since been" (line 14)
- watch: "deleted. It imports `arvo::strategy::{Additive, Identity, Warm}`," (line 15)
- watch: "- **It names `Warm` as though the strategy set were settled.** It is not. Op has" (line 29)
- watch: "stated the four inherited names are a prior attempt at the intent rather than" (line 30)
- watch: "the intent, and the number, the names and the decomposition are all open." (line 31)
- watch: "every strategy, without alloc and without std." (line 63)

### mock/research/sketches/202605111719_graph_spectral/02_generic_block_diagonal_body.md
- date: 202605111719 (directory stamp)
- by: not stated
- holds: A design probe (status DESIGN-PROBE-DURING-SRC) proposing to lift `block_diagonal` from a hardcoded `BitMatrix` to generic over `T: SparseAdjacency<N>`, giving the proposed body, why it is a design rather than a compile probe, and the fallback.
- headline: "The body shape (BFS / DFS with array stack + visited mask) is unchanged from the existing impl. The substitution is mechanical." (line 78)
- watch: "The `VisitedMask<N>` here stands in for the existing `Mask<Bits<64, Hot, Unsigned>>` only when `N <= 64`." (line 70)
- watch: "T: SparseAdjacency<N>," (line 16)

### mock/research/sketches/202605111719_graph_spectral/03_linear_operator_sparse_laplacian.md
- date: 202605111719 (directory stamp)
- by: not stated
- holds: A design probe (status DESIGN-PROBE-DURING-SRC) proposing a sealed `LinearOperator<F, N>` trait as the only surface spectral methods consume, with a `SparseLaplacian` impl computing `L * x` from CSR adjacency, an open question on `successors_indexed`, and the fallback.
- headline: "`LinearOperator<F, N>` is a plain trait (not `pub const trait`). The trait-solver complexity is one step less than S1, which already passed." (line 68)
- watch: "F: Add<Output = F> + Sub<Output = F> + Mul<Output = F> + FromConstant + Copy," (line 35)
- watch: "pub trait LinearOperator<F, const N: Cap>: sealed_op::Sealed" (line 14)

### mock/research/sketches/202605111719_graph_spectral/04_wider_return_shape.md
- date: 202605111719 (directory stamp)
- by: not stated
- holds: A design probe (status DESIGN-PROBE-DURING-SRC) proposing to replace the 64-capped `Mask` returns of `dulmage_mendelsohn` and `spectral_bisection` with `(USize, [USize; cap_size(N)])`, with an optional packing helper, an open question and the risk.
- headline: "Lowest-risk of the four sketches; the new shape is already validated by `block_diagonal`." (line 82)
- watch: "`spectral_bisection` returns `(Mask<Bits<64, Hot, Unsigned>>, Mask<Bits<64, Hot, Unsigned>>)`. Both cap at 64 nodes regardless of `N`." (line 7)
- watch: "W: BitPrim," (line 56)

### mock/research/sketches/202605111719_graph_spectral/README.md
- date: 2026-05-11T17:19Z
- by: not stated
- holds: The index for four sketches of round 202605111719, naming the four trait-shaped abstractions the topic commits to, the one live compile probe and three design probes, the decision rule between them and the fallbacks.
- headline: "**Status**: S1 compile-probed (PASSES). S2 through S4 documented for SRC-phase verification." (line 5)
- watch: "1. `SparseAdjacency<N>` sealed `pub const trait` with associated lifetime-generic iterator type for successors / predecessors." (line 12)
- watch: "3. `LinearOperator<F, N>` sealed trait with `SparseLaplacian<T, W>` impl driving Fiedler / power iteration without dense Matrix materialisation." (line 14)
- watch: "- `~/Dev/clause-dev/.claude/rules/arvo-toolbox-not-policer.md` — frames the trait-shaped surface as tools-not-policy." (line 50)

### mock/research/sketches/202605291942_csr-live-counts/findings.md
- date: 2026-05-29
- by: not stated
- holds: A sketch reproducing a miscount in the packed `Csr::with_transpose` when the input carries capacity slack, and validating that bounding the scatter by `live_nnz` removes it.
- headline: "Hypothesis CONFIRMED. The packed transpose corrupts real-node predecessor sets" (line 75)
- watch: "Compiled with `rustc --edition 2024` (no nightly features needed; plain `usize`)." (line 58)

### mock/research/sketches/202605301400_capacity-trait-migration/FINDINGS.md
- date: 2026-05-30
- by: not stated
- holds: A sketch reproducing arvo's cross-cutting trait families with toy stand-ins to test migrating `const N: Cap` to `C: Capacity`, answering four questions and settling the migration as mechanical parameter substitution in one atomic round.
- headline: "**Verdict:** GREEN. The migration of the cross-cutting trait family is mechanical and sound on the pinned nightly. Proceed." (line 6)
- watch: "- **Q4 keep `const` on the trait declaration.** Proven. A `const trait Foo<C: Capacity>` admits a `const` impl when the method body is const-clean" (line 32)
- watch: "The `pub const trait` qualifier stays on `SparseAdjacency` / `BidirectionalSparseAdjacency`; impls remain plain." (line 38)
- watch: "The `BitMatrix<W, const N: Cap>` `W` axis (per-row bit width, `Bits<64>` today, the firing-12 \"64-node column wall\") is a separate widening concern" (line 44)

### mock/research/sketches/202606071600_const-capacity-waist-probe/FINDINGS.md
- date: 202606071600 (directory stamp; the file names the pinned nightly-2026-05-28)
- by: not stated
- holds: A probe showing a `const fn` generic over a `Capacity`-style GAT trait can construct and index `C::Array<T>` in const context and call `[const]` bit-contract methods in the same body, clearing the GATE-2 foundational gate.
- headline: "**Outcome: WORKS.**" (line 5)
- watch: "2. **U1: `[const]` trait method calls inside a const fn body.** A `const trait Row` (mirroring arvo `BitAccess`/`BitSequence`) with `impl const Row for W` is callable in a const fn" (line 19)
- watch: "the pinned nightly uses the `const trait Foo` KEYWORD form, NOT the `#[const_trait]` attribute (which errors \"trait is not const\"). Bounds use `T: [const] Trait`; impls use `impl const Trait`." (line 21)

### mock/research/sketches/202606231130_fixed-point-one-construction/FINDINGS.md
- date: 202606231130 (directory stamp)
- by: not stated
- holds: A sketch hunting a const-evaluable generic construction of `Identity::ONE` as the fixed-point one for `IFixed`/`UFixed`, recording four failed or untaken hypotheses and the one that works, a free const fn doubling through `IArith::i_add`.
- headline: "**H9 (double raw-1 F times via `i_add` in a FREE const fn): WORKS.**" (line 36)
- watch: "H1 (container-T `Shl` bound): FAILS WITH E0391 cycle. Confirms the ROOT CAUSE: any where-bound that" (line 29)
- watch: "references the container projection `<S as BitsContainerFor<{ifixed_bits(I,F)}>>::T` cycles predicate" (line 30)
- watch: "Use the ALWAYS-`const` bound form (`S: const IArith`, `Bits: const Identity`), not `[const]`" (line 46)

### mock/research/sketches/202606231230_precise-logical-bound-saturation/FINDINGS.md
- date: 202606231230 (directory stamp; the sketch names nightly-2026-05-28)
- by: not stated
- holds: A sketch showing Precise can saturate at the logical bound rather than the container bound with a clamp and no widening, giving the working clamp shape, the macro change needed, and five gotchas for the implementer.
- headline: "**Outcome: WORKS.** `sketch_b.rs` and `body_shape.rs` compile and run on" (line 10)
- watch: "The invariant that makes this safe: Precise is always DoubleLogical, so container =" (line 6)
- watch: "3. **Replace `saturating_mul` with `wrapping_mul` + logical clamp for the fixed-point" (line 79)
- watch: "arm.** The current Precise `i_mul_fixed` does `a.saturating_mul(b) >> FRAC`, which" (line 80)
- watch: "saturates at the CONTAINER bound before the shift (the bug)." (line 81)
- watch: "the KNOWN BUG comment in arith.rs (above `impl_u_arith_saturating!`) states this affects EVERY Precise op." (line 88)

### mock/research/sketches/202606231230_widen-above-64-fixed-mul/FINDINGS.md
- date: 202606231230 (directory stamp; the sketch names nightly-2026-05-28)
- by: not stated
- holds: A sketch showing the fixed-point multiply at a 128-bit container can form a 256-bit intermediate via `u128::carrying_mul`, shift and narrow in const context, with the one feature gate it needs, five gotchas including floor rounding, and the candidates rejected.
- headline: "**Outcome: WORKS.** `sketch_a.rs` compiles and runs on `nightly-2026-05-28`" (line 10)
- watch: "The body needs exactly one unstable gate: **`#![feature(const_unsigned_bigint_helpers)]`**" (line 66)
- watch: "1. **Floor rounding, not truncation.** The existing rescale is an arithmetic right" (line 80)
- watch: "shift (floor toward minus infinity), pinned by `i_mul_fixed_negative_floor_rounding`." (line 81)
- watch: "A naive multiply-magnitudes-reapply-sign truncates toward zero and silently" (line 82)
- watch: "The 33..64 Warm/Precise ranges" (line 101)

### mock/research/sketches/202607282100_container-projection-without-gce/FINDINGS.md
- date: 2026-07-28
- by: not stated
- holds: A sketch showing the container projection can carry its bucket selection as typestate rather than computing it in const-generic argument position, removing the need for `generic_const_exprs`, with what was verified, the cost, and what the recorded justification for the gate got wrong.
- headline: "**Outcome:** **WORKS.** Zero feature gates required." (line 4)
- watch: "- `Hot` at 13 bits resolves to `u16`, at 47 bits to `u64`, signed at 7 bits to `i8`." (line 71)
- watch: "- `Warm` at 13 bits resolves to `u32` and signed at 32 bits to `i64`, preserving the 2x-logical rule." (line 72)
- watch: "`arvo-compile-time-last.md` states the substrate is allowed to \"spend trait-solver work on per-N const-trait impls (4 strategies x 64+ widths x 2 sign = hundreds of impls) when the alternative is a runtime container check\"" (line 82)

### mock/research/sketches/202607282230_hlist-arity-dissolution/FINDINGS.md
- date: 2026-07-28
- by: not stated
- holds: A sketch dissolving the hand-capped `Pred`/`Pred2`/`Pred3` arity family, trying seven shapes from blanket bridges through recursive hlist chains to a `Deref` wrapper, recording what failed and why, and arguing Shape F is semantically correct for the existing callers.
- headline: "**Outcome:** **WORKS, two ways.** Zero feature gates. 16 assertions, mutation-checked." (line 4)
- watch: "`arvo/src/predicate.rs` ships `Pred<A>`, `Pred2<A, B>` and `Pred3<A, B, C>`, three trait aliases over" (line 10)
- watch: "`Fn(..) -> Bool` at arities one, two and three." (line 11)
- watch: "**3. `fn_traits` / `unboxed_closures`.** Not attempted. `Fn<Args>` with `Args: Tuple` is the only way" (line 35)
- watch: "to be generic over `Fn` arity, and it fails the vetting gate" (line 36)
- watch: "**Mutation-checked, because passing tests prove nothing until they are shown able to fail.**" (line 112)
- watch: "The pinned toolchain documents 247 unstable language features and **not one of them is variadic generics**." (line 185)

### mock/research/sketches/202607290030_typestate-algorithm-selection/FINDINGS.md
- date: 2026-07-29
- by: not stated
- holds: A sketch verifying from emitted assembly that a property carried in the type selects an algorithm with the discarded one absent, comparing three compile-time encodings, showing `const fn` cannot discriminate on a type, and naming the soundness obligation an asserted property creates.
- headline: "**Outcome:** **WORKS.** No feature gates. Verified from emitted assembly, not from reasoning." (line 4)
- watch: "That is the same shape as `Strategy`, one level up. `Hot` / `Warm` / `Cold` / `Precise` are properties" (line 15)
- watch: "in the type that select storage. `MONOTONE` would be a property in the type that selects an algorithm." (line 16)
- watch: "`MONOTONE = true` is an **unchecked promise**. If a consumer asserts it falsely the library runs a" (line 109)
- watch: "It is closer to an `unsafe` contract than to a tuning knob, and it should be treated as one: a stated contract per property, and ideally a debug-mode assertion cross-checking the selected algorithm against the naive one." (line 113)
- watch: "`arvo-toolbox-not-policer` requires the substrate not to hardcode thresholds or guess the workload;" (line 207)

### mock/research/sketches/202607291400_const-args-under-min-gca/FINDINGS.md
- date: 2026-07-29
- by: not stated
- holds: A sketch establishing that arvo's ergonomic aliases are a live `generic_const_exprs` dependency with no expression under any permitted feature, giving two gate-free escapes, and correcting the earlier gate inventory.
- headline: "**The aliases are a live GCE dependency**, and the facade's gate is" (line 31)
- watch: "arvo's ergonomic aliases are `pub type Uint<const N: u16, S> = UFixed<{ ibits(N) }, { fbits(0) }, S>`:" (line 11)
- watch: "This is Pattern C, the shape `BitsContainerFor` and `Project` already use" (line 69)
- watch: "They are barely used: `Fixed` and `Signed` appear only in two test comments, and `Uint` and `Int` have one use between them outside their own file." (line 96)

### mock/research/sketches/202607291500_capacity-without-supertrait/FINDINGS.md
- date: 2026-07-29
- by: not stated
- holds: A sketch testing op's proposal that `Capacity` and `ConstCapacity` be implemented on the same type with non-const bodies delegating to const ones, finding no supertrait is needed, with the one cost (ambiguous associated type when a consumer binds both) measured.
- headline: "`arvo-capacity` ships both traits as they are. The merge question dissolves: it was asking which cost" (line 66)
- watch: "op's proposal was that neither is necessary: implement both on the same type, in their own contexts," (line 15)
- watch: "`from_fn` takes a closure, closures are not const-callable, and `impl const Trait` must supply const bodies for every method." (line 71)

### mock/research/sketches/202607300500_format-as-exponent-function/FINDINGS.md
- date: 2026-07-30
- by: not stated
- holds: A sketch testing the format concept as an exponent function after Flocq's `generic_format`, finding the proposal's stated gating risk misidentified, expressing all four Flocq formats plus a six-axis composition, and locating the real type-position hazard in the container width.
- headline: "**Outcome:** **WORKS.** Zero feature gates for the plain-trait form, one allowed gate" (line 4)
- watch: "The proposal rests on Flocq's `generic_format β fexp`, where fixed point and floating point differ" (line 12)
- watch: "Floating<24,-149,Gradual>   at -200     -> -149    (FLT, clamped)" (line 35)
- watch: "Floating<24,-149,Flushed>   at -200     -> -126    (FTZ, emin + prec - 1)" (line 36)
- watch: "A six-axis composition `Num<Fmt, Sign, Round, Over, Grow>` resolves to a concrete container" (line 58)
- watch: "`Unbounded` (FLX) is expressible and gives a format IEEE 754 does not standardise" (line 70)

### mock/research/sketches/202607300600_law-markers-derived-over-composition/FINDINGS.md
- date: 2026-07-30
- by: not stated
- holds: A sketch showing algebraic law markers can be derived by blanket impls over a six-axis composition when conditioned on a type-level `(Overflow, Signedness)` pair, and that the forbidden composition is refused with a precise error and no diagnostic attribute.
- headline: "**Outcome:** **WORKS**, both halves. The laws derive, and more importantly the composition the" (line 4)
- watch: "Saturating addition is associative for unsigned operands and not for signed ones, because two-sided" (line 15)
- watch: "clamping is the cause and one-signed operands never walk a clamp back." (line 16)
- watch: "Asserted: wrapping folds and inverts under both signednesses; unsigned saturating folds and does not invert, since nothing undoes a clamp." (line 45)
- watch: "Symmetric saturation is included and also refused for signed operands, confirming by construction that `SAT_SYM` does not restore associativity." (line 76)
- watch: "rounding has no effect on addition's associativity but plainly does on multiplication's." (line 90)

### mock/research/sketches/202608041500_numeral-notation-vehicle/OUTCOME.md
- date: 202608041500 (directory stamp)
- by: not stated (full writeup attributed to `61_amin_the_notation_vehicle.md`)
- holds: The outcome of a probe into whether a declarative macro can read a decimal literal and emit the numeral tower's encoding, finding both macro routes dead, taking the proc-macro fallback, and recording two independent ceilings plus a structural sealing finding.
- headline: "# Outcome: WORKS, with a corrected vehicle" (line 1)
- watch: "neither the bare language nor `min_generic_const_args` admits a generic" (line 12)
- watch: "const in a recursive const-position. Both routes are dead, not merely slow." (line 13)
- watch: "One structural finding beyond what was asked: a macro living in its own" (line 45)
- watch: "crate cannot ever satisfy a trait sealed via a private supertrait it did" (line 46)

### mock/research/sketches/202609021019_the-fraction-decision/README.md
- date: 202609021019 (directory stamp)
- by: not stated
- holds: A sketch testing `Fraction::of`'s stated reason for reading two pairs as whole numbers, finding it false on a large part of its family, confirming the positive-denominator invariant is load-bearing, and giving a rule that gives up nothing.
- headline: "**Outcome: FAILS, in the direction the hypothesis did not anticipate.** The stated reason is false on" (line 7)
- watch: "the `[0, 1)` reading is what lets the rounding modes be stated once rather than once per sign." (line 28)
- watch: "the answer is exact wherever an exact one exists in the width, and where none exists the relative error is at most `1 / MAX`." (line 41)
- watch: "`Exact::is_tie` doubles the stored remainder in `i64` where the neighbouring `round_slot` does the" (line 57)
- watch: "same comparison in `i128`. `233` of the swept remainders leave `i64` when doubled." (line 58)

### mock/research/sketches/202609111010_addition-obligation-and-window/README.md
- date: 202609111010 (directory stamp)
- by: not stated
- holds: A sketch testing four hypotheses the addition round would otherwise assert: where an admission obligation fires in the check-versus-build split, that a `Window<LO, HI>` meets the slot-range obligation, that the exact addition step reproduces the canon's divergence witnesses, and that the carry refusal is reachable.
- headline: "H3 WORKS. `Integer<4>` saturate 952, wrap 0, `UFixed<4, 0>` and `UFixed<4, -3>` saturate 0," (line 47)
- watch: "952 divergent triples for signed saturating addition at four bits (`law::additive_associativity_under_saturation`), zero for wrapping and zero for unsigned saturation, and 4,177,792 at eight bits." (line 27)
- watch: "`Window<-4, 0>`, a signed range, is associative under saturation (0 divergent), and `Window<-1, 7>` is not (70). A rule keyed on signedness gets the first wrong" (line 53)
- watch: "The eight-bit timing in `output.txt` is an ad-hoc spike with no substance, a debug build and one run." (line 63)

### mock/research/sketches/202609190830_the-slot-coordinate-at-sixty-four/README.md
- date: 202609190830 (directory stamp)
- by: not stated
- holds: A sketch testing which of three named repairs lets a slot coordinate reach a 64-bit platform width, finding only the widened `i128` index works and both restatements that keep a signed 64-bit index are refused at 64.
- headline: "WORKS for option Y, FAILS for option Z, in `output.txt`." (line 14)
- watch: "Option Y, in `y_an_i128_index.rs`, carries the index in `i128` and keeps the impl ladder as the bound." (line 24)
- watch: "Two members of a 64-bit range sum to at most `2^65 - 2`, which leaves more than sixty bits of the carrier above every position addition produces" (line 37)

### mock/research/sketches/202609241413_the-half-up-reader-attribution-mutants/README.md
- date: 202609241413 (directory stamp; the round it names)
- by: not stated
- holds: A mutation run breaking the half-up lint's clause reader arm by arm, recording that all 37 mutants are caught, which three walks were added so none is caught by the aggregate arm alone, which tests hold an arm alone, and what the run does not show.
- headline: "`WORKS`: all 37 mutants are caught, 36 of the reader and 1 of the history lint," (line 14)
- watch: "the lint's module doc and `sentences/catalogue.rs` carry the eight sentences it is known to read wrong." (line 151)
- watch: "A mutant being caught says a test depends on the arm, not that the arm reads English correctly" (line 149)

### mock/research/sketches/202611122300_value-over-derived-carrier/FINDINGS.md
- date: 202611122300 (directory stamp; arvo pinned at `d841c5e19325fc60648cc9a4bb48dcbb8b348df4`, toolchain `nightly-2026-05-28`)
- by: not stated
- holds: A five-arm sketch asking how a number type holds its value in the derived carrier and how `add` can be one `const fn` reaching every carrier on the ladder, with arms A, B and D building, C and E refused, the accumulator and term bounds on arm B, an ad hoc assembly reading, and what differed from the brief.
- headline: "The outcome is that arms A, B and D build and pass, arm C is refused in the consumer, and arm E is refused in its own crate." (line 5)
- watch: "Wrap is a shift pair for signed and a mask for unsigned. Saturate is a clamp to the width's range inside the carrier." (line 30)
- watch: "So arm C cannot give a featureless consumer both a `const` item and a function generic over the signature: the feature leaks into the consumer." (line 74)
- watch: "It also agrees for unsigned `Saturate`: members are never negative, so the partial sum only climbs and `min(MAX, sum)` does not depend on order. It fails for signed `Saturate`, in 104352 of the 497664 cases." (line 154)
- watch: "- In a build with overflow checks it panics: `attempt to add with overflow`." (line 169)
- watch: "Arm C waits on `const_trait_impl` reaching consumers. Arm E waits on generic const arguments that are not vetted." (line 202)
- watch: "the `unsafe` in `copy`, whose precondition is that both sides are plain machine integers of equal size" (line 197)

### mock/research/sketches/202611130342_value-add-disassembly/FINDINGS.md
- date: 202611130342 (directory stamp)
- by: not stated
- holds: An ad hoc reading of emitted assembly for the value's `add` at 13 bits in a 16-bit carrier and at full 64-bit width, showing the full-width add aliases the host's, the narrow add is the machine add plus the adaptation, and a first reading that found 24 un-inlined accessor calls.
- headline: "Outcome: WORKS. An ad hoc reading of one build, not a measurement and not a harness artifact." (line 3)
- watch: "`u13_saturating` is an add, one compare against 8191 and a conditional move." (line 24)
- watch: "`i13_saturating` is an add and the two clamps, each a compare and a conditional move. No branch, no" (line 25)
- watch: "call, no overflow test on the carrier: below the carrier width two members cannot leave it, so the" (line 26)
- watch: "`i64_saturating` as an alias of `host_i64_saturating_add`, the overflow-flag select the host's" (line 18)

### mock/research/sketches/202611130501-vt8a_value-mutants/FINDINGS.md
- date: 202611130501 (directory stamp)
- by: not stated (seat tag `vt8a` in the directory name)
- holds: A mutation run over `arvo-numeral`'s four rung mutants, each removing that rung's adaptation onto the declared width, showing each turns the tests running through its rung red and leaves the rest green.
- headline: "Outcome: WORKS. Each of the four mutants turns the tests that run through its rung red, and leaves the" (line 3)
- watch: "every one asserts agreement with `arvo_format::add`, the slot order or the specified panic," (line 8)
- watch: "`from_raw` asserts under both plain markers no longer fire, and the completing twins of `add` and `from_raw` no longer complete." (line 24)
- watch: "These four remove the adaptation and nothing else; a mutant of the dispatch, the bit copy or the comparison is not among them." (line 36)

### mock/research/sketches/202611130513-jy34_an-unnameable-carrier/FINDINGS.md
- date: 202611130513 (directory stamp)
- by: not stated (seat tag `jy34` in the directory name)
- holds: A probe showing `Num<S>` can store the derived machine carrier while no dependent crate can name that carrier's type, by declaring the map through an associated type bound in a sealed supertrait list, with each refused route recorded.
- headline: "Outcome: WORKS, on the toolchain in `outputs/toolchain.txt`, with no feature gate." (line 3)
- watch: "pub trait Sole: DeclaredSignature<Format: Format<Slots: sealed::Carried>> {}" (line 15)
- watch: "`the_carrier_by_projection` is refused with E0220: `S::Repr` with `S: Sole` is not found" (line 30)
- watch: "A derived `Clone` and `Copy` on `Num<S>` bound `S: Copy`, so a generic consumer moving a value twice failed with E0382." (line 43)

### mock/research/sketches/202611130513-jy34_value-mutants/FINDINGS.md
- date: 202611130513 (directory stamp)
- by: not stated (seat tag `jy34` in the directory name)
- holds: A re-run of the four rung mutants against the surface where `from_raw` and `raw` are gone and the carrier cannot be named, showing each mutant turns its rung's tests red and leaves the rest green.
- headline: "Outcome: WORKS. Each of the four mutants turns the tests that run through its rung red, and leaves the" (line 3)
- watch: "The two `add` assertions under the plain markers still fire, because they ask the format's verdict before the rung runs, and `adapted` stays green, because it reaches the carrier through the format's own adaptation and not the rung's." (line 24)
- watch: "a mutant of the dispatch, the bit copy, the slot read behind `Hash` and `Debug`, or the comparison is not among them." (line 40)

### mock/research/sketches/202611130550-m67x_an-alias-that-takes-a-strategy-later/FINDINGS.md
- date: 202611130550 (directory stamp)
- by: not stated (seat tag `m67x` in the directory name)
- holds: A probe on spelling `Uint` so a strategy parameter defaulting to `Warm` can be added later without breaking consumers, by reading the parameter through a `NamesAdaptation` projection, with the case that must fail recorded.
- headline: "Outcome: WORKS, on the toolchain in `outputs/toolchain.txt`, with no feature gate." (line 3)
- watch: "The question is how to spell `Uint`, while the `Warm` cell for integers is unclaimed, so that a" (line 5)
- watch: "strategy parameter defaulting to `Warm` can be added later without breaking any consumer written" (line 6)
- watch: "impl, `NamesAdaptation for Warm`, whose adaptation is `<Warm as Strategy>::Adaptation`." (line 19)
- watch: "`C = Abort` is the debug check of `202611130550-m67x_the-debug-check-axis`, after the adaptation" (line 24)
- watch: "Nor anything about `Warm`'s adaptation itself, which this probe reads and does not decide." (line 46)

### mock/research/sketches/202611130550-m67x_the-debug-check-axis/FINDINGS.md
- date: 202611130550 (directory stamp)
- by: not stated (seat tag `m67x` in the directory name)
- holds: A probe comparing a cargo feature against a type parameter as the vehicle for the axis saying whether a debug build aborts when a result leaves the range, showing the feature carries one consumer's choice into another's types and the parameter does not.
- headline: "Outcome: WORKS for a type parameter with a default; a cargo feature works and carries one consumer's" (line 3)
- watch: "The question is the vehicle for the axis that says whether a debug build aborts when a result leaves" (line 6)
- watch: "the range, chosen by the consumer and separate from the overflow policy." (line 7)
- watch: "- by parameter, the abort is a marker in the type, `U4<C: DebugCheck = Abort>`, with `Quiet` beside `Abort`." (line 11)
- watch: "Cargo unifies the features of one crate across a build, so the choice one consumer made reached a type another consumer declared." (line 25)

### mock/research/sketches/202611130550-m67x_value-mutants/FINDINGS.md
- date: 202611130550 (directory stamp; the file names round 202611130720-wub5)
- by: not stated (seat tag `m67x` in the directory name)
- holds: A third taking of the four rung mutants, on the reworked value that reads its container from the placement's `SoleCarrier` and takes its debug check as a second parameter, showing each mutant reddens its own rung's tests only.
- headline: "Outcome: WORKS. Each of the four mutants turns the tests that run through its rung red, and leaves the" (line 7)
- watch: "the value reads its container from the placement's `SoleCarrier`, the sign from `Sole::SIGNED`, and takes its debug check as a second parameter." (line 8)
- watch: "one test of the debug check binary, `add_completes_under_quiet`, whose signature sits at width 4 in the 8-bit carrier. The `Abort` tests still fire" (line 27)
- watch: "The debug check's `ABORTS` and the admission's whole-range test were each mutated by hand during the round" (line 41)

### mock/research/sketches/base-primitives-gate/202608311729_base-primitives-gate.md
- date: 202608311729 (file name stamp)
- by: not stated
- holds: A sketch asking whether a base primitive design can be written invariant under both branches of `question::the_container_premise`, finding no invariant shape exists, with two deliberate compile refusals as the load-bearing results.
- headline: "**FAILS.** No invariant shape exists. Three files, one runnable and two that must not compile, and" (line 15)
- watch: "Branch A, carrier not in the type, gives one footprint per declared width. Branch B, carrier in the type, gives two footprints at one declared width." (line 21)
- watch: "`ruling::cold_is_for_cold_paths_and_cold_storage` is about the footprint, and on branch A no signature distinguishes the footprint." (line 30)
- watch: "**Blocks** the base primitive design round, which stays in TOPIC." (line 53)

### mock/research/sketches/open-trait-admission/README.md
- date: none stated (cites round `202609010001`)
- by: not stated
- holds: A sketch reproducing an incoherent outside implementor of the open `Slots` trait and showing a defaulted `ADMITTED` const refuses it, with the limitation that the refusal fires at codegen rather than at `cargo check` and what the guarantee therefore is.
- headline: "**The finding is CONFIRMED.**" (line 12)
- watch: "error[E0080]: evaluation panicked: slot range is inverted: MIN exceeds MAX" (line 21)
- watch: "**The refusal fires at codegen and not at `cargo check`.**" (line 51)
- watch: "\"Refused at compile time\" is true of a build and false of a check" (line 69)
- watch: "an inadmissible range cannot reach a produced binary, and it can reach a passing `cargo check`." (line 76)

### mock/research/sketches/phase-over-the-magnitude-range/README.md
- date: none stated
- by: not stated
- holds: Two ad-hoc quick spikes on `has_additive_identity`, which reads no coordinate of the quantum, enumerating the real grid against the shipped function to find three disagreements, and checking the widened const shape compiles and folds to a constant.
- headline: "Two of the three are under-reporting and one is over-reporting, and both" (line 50)
- watch: "`has_additive_identity` reads the phase coordinates and the slot range. It reads" (line 9)
- watch: "no coordinate of the quantum at all, so it answers at magnitude zero while its" (line 10)
- watch: "it is the one that refutes the design's own gloss: a **fractional** phase keeps the identity there, because the quantum has halved by the magnitude that cancels it." (line 57)
- watch: "Neither file measures anything, so nothing here carries a cost, a ratio or a timing" (line 84)

### mock/research/sketches/phase-over-the-magnitude-range/mutation_the_search_is_cut_to_one_magnitude.md
- date: none stated
- by: not stated
- holds: An ad-hoc spike mutating `has_additive_identity` to search one magnitude, recording that eight of the round's new arms fail while none of the three pre-existing arms does, so the defect had been invisible.
- headline: "Eight arms fail, so the suite is measuring the search rather than agreeing with it." (line 47)
- watch: "**None of the three arms this suite already had is among them.**" (line 49)
- watch: "Both come from the same cause, which is that `Biased` is the only shipped point with a phase and it pins three coordinates." (line 61)

### mock/research/sketches/the-adaptation-contract-under-translation/README.md
- date: none stated
- by: not stated
- holds: A spike measuring whether the shipped adaptation map commutes with translation and with reflection, over five steps and thirty-nine verdict lines, and checking the const predicate a consumer would gate on against the map cell by cell.
- headline: "Outcome: WORKS. Both laws are stateable over the shipped map, both regions" (line 3)
- watch: "**The rounding region commutes with translation exactly where what the mode reads" (line 56)
- watch: "besides the residue is not there.** Three of the six shipped modes read nothing" (line 57)
- watch: "besides it and commute on every domain; toward-zero reads the sign at every" (line 58)
- watch: "**The completion region commutes with translation always under wrapping**, which" (line 63)
- watch: "**The first revision of `src/predicate.rs` was unsound and `P1` caught it in six" (line 81)
- watch: "`259_fallin_whether_the_fused_result_composes.md` section 3 argues that under" (line 89)

### mock/research/sketches/the-half-up-lint-mutants/README.md
- date: none stated
- by: not stated
- holds: A mutation run over the `half-up-is-not-a-magnitude-rule` lint's two reader modules as they stood at `b8a7ce4b`, breaking each arm behind an environment switch and recording whether the suite catches it.
- headline: "`WORKS`: every one of the 28 mutants is caught." (line 7)
- watch: "Three mutants survived the first run: `offset`, `raw` and `escape`. Every test" (line 54)
- watch: "That the arms are right about English. It shows each arm is load-bearing for" (line 66)

### mock/research/sketches/the-half-up-lint-reader-mutants/README.md
- date: none stated
- by: not stated
- holds: A mutation run over the rewritten clause reader of the `half-up` lint at `df9d286f` and the arms the anaphora round added, covering the antecedent carry, what counts as another name, the denotation escape, the negator's binding, the list escape and what counts as a reading.
- headline: "`WORKS`: all 21 mutants are caught. One survived the first run and the corpus" (line 13)
- watch: "`nocap` survived. The arm it breaks is the one matching an other-name at the" (line 67)
- watch: "`arvo-format/DESIGN.md.tmpl`, was the only thing holding it, and a test suite" (line 71)
- watch: "The corpus runs to 38 pairs." (line 77)
- watch: "That the arms are right about English. It shows each arm is load-bearing for" (line 86)

### mock/research/sketches/the-half-up-tie-mutation/README.md
- date: none stated
- by: not stated
- holds: One mutation of `arvo-format/src/apply.rs` planting the crate's pre-ruling `half_up` tie rule back into `round_slot`, with the committed test output it produced, since the tree carries the fixed rule and the run is not reproducible from it.
- headline: "`WORKS`, in the sense that the mutation is caught." (line 15)
- watch: "That is the arm the crate carried before the ruling: a tie at a negative" (line 36)
- watch: "position goes down, away from zero, and every other tie goes up." (line 37)
- watch: "--no-fail-fast` from `mock/`, debug profile. Twenty-seven tests fail: 24 in the" (line 42)
- watch: "now reads the sign at a tie while `behaviour_of` says it reads nothing;" (line 58)
- watch: "MATLAB's Round." (line 61)
- watch: "One mutation, one instance. It says the suite notices this rule in this" (line 68)

### mock/research/sketches/the-symmetry-predicate-mutations/README.md
- date: none stated
- by: not stated
- holds: Two committed mutations of `arvo-format/src/symmetry.rs` with their test output, asking whether the predicate's arms run the machinery or read the declaration back, and recording which arms failed in each.
- headline: "`WORKS`, in the sense that the mutation is caught." (line 15)
- watch: "The hypothesis both answer is the same one, and it is the only interesting" (line 8)
- watch: "false, negative_translations: false, ties: true }: the predicate says true and" (line 36)
- watch: "the map says false`, which is the unsound direction. A cell where the predicate" (line 37)
- watch: "`WORKS`, and it is the one that says why the arm it fails was written." (line 47)
- watch: "TowardZero refused a low excursion that no translation in the band can undo" (line 61)
- watch: "file. Restore by writing rather than by moving a backup over it: a backup's mtime" (line 75)

### mock/research/sketches/the-ties-away-alias/README.md
- date: none stated
- by: not stated
- holds: A spike asking where a consumer can write the ties-away-from-zero alias over `arvo-format`'s public surface, sweeping two spellings against an integer oracle and refuting the closed round's limit at `i64::MAX / 2`.
- headline: "`WORKS`, with one corner named. `out/run.txt` is the raw output of" (line 28)
- watch: "The ruling reaches ties away from zero as an alias, `toward_zero(x + sign(x) q/2)`." (line 3)
- watch: "the alias can be written only where the doubled denominator fits, so not past" (line 6)
- watch: "- The read-off spelling matches the oracle at every cell, and at every cell of a" (line 38)
- watch: "- The doubled shift the closed round's arm used is unformable at 548 cells of" (line 48)
- watch: "The design's sentence that the alias is expressible over part of the domain and by" (line 61)
- watch: "That the read-off spelling is what a consumer should write. It shows the alias has" (line 67)

### mock/research/sketches/usize-const-arith-202605021200/FINDINGS.md
- date: 2026-05-02
- by: not stated
- holds: Findings of a nightly sketch establishing that `impl const` lands on every `core::ops::*` trait arvo numerics use, with the op matrix on `USize`, the layered composition pattern, the mixed-arity refusal, per-primitive notes for the source apply and the consumer gates in hilavitkutin.
- headline: "arvo can land `impl const` on every `core::ops::*` trait used by arvo numerics. The sketch validates the full op matrix on USize, the Cap layered-composition shape, the UFixed-shaped const-generic case, the Mask bit-op case, and the load-bearing hilavitkutin Depth case." (line 9)
- watch: "- `feature(const_ops)`. NEW; not currently enabled in arvo. Required to use the standard library's `core::ops::*` traits as const traits at const-context call sites. Without it, every const-context call to `Add::add` etc. fails with `Add is not yet stable as a const trait`." (line 24)
- watch: "Both compile. The first stays inside the typed surface (USize::add is the load-bearing op; Cap's body composes through it). The second reaches to bare `usize` and wraps back, which is exactly the field-access shape this round eliminates." (line 52)
- watch: "The negative test (commented out in the sketch): `const _: USize = USize(1) + 1usize;` does not compile. rustc rejects mixed `USize + bare-usize` because no `Add<usize>` impl exists. Good. Consumers must use `USize::ONE` / `USize::ZERO` / `USize(N)` as the const-time literal form." (line 58)
- watch: "**UFixed<I, F, S>**: existing 4 ops impls (Add/Sub/Mul/Div) at `arvo/src/ufixed.rs:153,167,181,195`. Convert each to `impl const`." (line 68)
- watch: "The grep in the topic addendum surfaced ~50 sites in arvo source that use `.0` field-access on USize / Cap, primarily in `arvo-bitmask/src/ops.rs` (bit-position arithmetic) and `arvo-comb/src/{dp,greedy}.rs` (loop counters)." (line 108)
- watch: "One concern surfaced and resolved: `feature(const_ops)` is currently named that way and was previously called `feature(const_trait_impls_for_std)` or similar." (line 86)

### mock/research/strategy-bound-trilemma.md
- date: 2026-04-28 (promotion stated)
- by: not stated
- holds: A research note from round 202604500000, since promoted to a DEEPDIVE template, formalising why generic-over-width consumer code must restate the `UContainerFor` bound, the three properties that cannot all be held, the three alternative architectures and why each costs one, the unsafe angles that do not help, and the three layers of access.
- headline: "On current rustc (`adt_const_params` + `generic_const_exprs` available, `implied_bounds` not), you can have any two of those three, not all three." (line 29)
- watch: "> **Promoted 2026-04-28**: this analysis is now shipped as" (line 3)
- watch: "1. **Static validity check.** Combinations of `(strategy, bit-width)` that don't make sense fail at compile time. `Uint64<Warm>` produces a `UContainerFor`-bound error pointing the consumer at strategy choice, not a runtime panic or a silent wrap." (line 19)
- watch: "2. **Strategy-driven container width at the type level.** `Bits<8, Hot>` is one byte; `Bits<8, Warm>` is two; `Bits<8, Cold>` is bitpacked to N bits in column storage." (line 21)
- watch: "Costs: every `Bits<8, Hot>` value pays 8 bytes instead of 1. Cold's bit-density at the type level evaporates." (line 43)
- watch: "The pattern across all of these: the bound failure is INTRINSIC to \"the field's type depends on a const-expr derived from generic params\". `unsafe` is not a workaround for this class of problem." (line 69)
- watch: "- **Public consumer APIs use trait-bound abstractions, not bit-width-generic abstractions.**" (line 113)

### mock/research/sketches/value-path/README.md
- date: none stated
- by: not stated
- holds: A sketch asking whether the adaptation's value path answers the same under both settings of overflow-checks, reproducing the wrong answer and the panic before the change and showing the two settings agree afterwards.
- headline: "Identical in both settings. `i64::MAX` wrapping into `[-4, 3]` is `(i64::MAX + 4) mod 8 = 3`, so" (line 37)
- watch: "**Hypothesis.** The adaptation's exact step was performed in the carrier its result has to fit, so a" (line 3)
- watch: "large exact position produced a wrong answer without `overflow-checks` and a runtime panic with it." (line 4)
- watch: "**The correct answer is 0.** The span is 7 and `i64::MAX - (-1)` is 9223372036854775808, whose" (line 21)
- watch: "remainder modulo 7 is 1, so the result is `-1 + 1`. The code returned 5, which is **inside the" (line 22)
- watch: "The two settings must agree. If they ever differ, the exact step has moved back into a carrier that" (line 43)
- watch: "path reachable through `adapt`, at the edges of `i64`, for the wrapping policy. The saturating and" (line 50)

### mock/research/sketches/width-bound/README.md
- date: none stated
- by: not stated
- holds: A sketch asking whether a declared width above the slot index's bound refuses at compile time with a message a reader can act on, with the two failures it produced before the change.
- headline: "**REFUSES, and names why.** `output.txt` carries the diagnostic. It names the bound, the reason the" (line 12)
- watch: "`ruling::never_a_runtime_check_and_one_lowered_path`, rather than producing a raw arithmetic error or" (line 5)
- watch: "`Signed<64>` gave `error[E0080]: attempt to negate i64::MIN`, an arithmetic error about a constant" (line 19)
- watch: "`Signed<63>` **compiled**, panicked in debug, and in release returned a derived width of **zero bits**" (line 21)
- watch: "for a declared 63-bit numeral, because the slot count wrapped negative and the deriving loop never" (line 22)
- watch: "The consumer here instantiates `Signed<63>`. If it ever builds, the bound is not being enforced and" (line 30)

<!-- END -->
