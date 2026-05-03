**Date:** 2026-05-03T14:00Z
**Phase:** TOPIC
**Scope:** arvo substrate foundational redesign — byte-sequence container above 128 bits, strategy-aware alignment, single-impl projection, MultiContainer deletion. Tracks task #316.
**Source topics:** Senior audit 2026-05-03; Sketch 01-06 in `mock/research/sketches/202605031400_hlist_heterogeneous_container/`; user's pre-compact architectural locks; this session's three structural decisions.

# Topic: foundational arvo container redesign

This round is the foundational restructure that sketches 01-06 unblocked. It deletes the binary `MultiContainer<HiT, LoT>` family, introduces byte-sequence storage above 128 logical bits, decouples the strategy axis from per-N projection enumeration, and unifies the three container projection traits into one.

The work is bounded by the architectural pillars locked in the resume memory and the structural decisions taken at the start of this topic.

## Background — what changed since Round 2

Round 2 (closed 2026-05-03 at HEAD c1e4ba9) lifted `Bits<const N: Width>` and added typed arithmetic helpers for LOGICAL_WIDTH. The senior audit dispatched after Round 2 close found:

- **CRITICAL S1**: `Signedness` lift to `pub const trait` claimed in Round 1 SRC CL line 92 but never landed. Source still has plain `pub trait`.
- **CRITICAL S4**: `MultiContainerHalf` lift to `pub const trait` similarly claimed but not landed.
- **HIGH S6**: Round 1 SRC CL line 213 stated "five cycle-avoidance probes have recorded outcomes" but the outcomes lived only in commit messages, never inline in the CL.
- **HIGH S3**: `Bits<256>` documented as the cap but the container projection table tops at 255. Off-by-one.
- **CRITICAL R1**: Round 3 (#313) Bits-with-Width lift has chicken-and-egg with Width relocation.
- Architectural-level critique: `MultiContainer<HiT, LoT>` was the lazy fallback choice from a prior round; container projection above 128 bits should be re-architected.

Six sketches in `mock/research/sketches/202605031400_hlist_heterogeneous_container/` validated the redesign:

| Sketch | Status | Validates |
|---|---|---|
| 01 hlist_basic | invalidates heterogeneous claim | repr(C) HList pads to alignment-of-largest-prim; "optimal-fit" heterogeneous-Cons was fiction; same applies to existing `MultiContainer<HiT, LoT>` |
| 02 widebits_basic | WORKS | `WideBits<const BYTES>` with `[u8; BYTES]` under repr(C) is align-1 across all tested widths; per-byte BitPrim ops correct |
| 03 aligned_widebits | WORKS | `Aligned16/32/64<const BYTES>` discrete tiers via `repr(C, align(N))` literal; const-generic align unexpressible in Rust |
| 04 simd_count_ones | WORKS | cfg(target_arch) gating with x86_64 + aarch64 intrinsics compiles; identical results across scalar / chunked / intrinsic-load paths |
| 05 single_impl_projection | WORKS | feature(generic_const_exprs) accepts the `where [(); bytes_for(N)]:` + const-fn associated type pattern; one impl per strategy replaces the per-N table |
| 06 bits_end_to_end | WORKS | `Bits<W, S, Sign>` over the projection across N ∈ {7, 13, 64, 128, 200, 256, 4096} × all 4 strategies |

The architecture pillars locked in the resume memory (`project_resume_post_compact_2026_05_03_arvo_redesign_pivot.md`) drive the doc CL: 128-bit native boundary, strategy-aware alignment, single-impl projection, no legacy shims, cfg-gated platform paths, sketch-then-commit, claim-vs-source contract.

## Decisions locked in this topic

### Decision 1 — Hot strategy alignment tier

**Single `Hot` type, alignment cfg-driven by target_feature.**

Hot exposes one consumer-visible strategy marker. Internal alignment is picked at compile time per cfg(target_feature):
- SSE2 / NEON baseline → align(16).
- AVX-2 / SVE2 → align(32).
- AVX-512 → align(64).

Consumer writes `Hot` and receives the best alignment for the build target. This matches the workspace's `arvo-always-optimal-internals` rule: public surface stays consumer-facing, internals do whatever's optimal. Per `arvo-toolbox-not-policer`, we are not forcing the consumer to pick a tier they don't have visibility into; the build target IS that visibility.

Rejected alternatives:
- Discrete `Hot16` / `Hot32` / `Hot64` markers — forces consumer to know target_feature; doesn't fall through gracefully on weaker targets.
- Hot always Aligned16 baseline — leaves AVX-2 + AVX-512 alignment optimization on the floor without justification.

### Decision 2 — N ≤ 128 vs N > 128 specialisation

**N ≤ 128 specialises to native primitives (u8/u16/u32/u64/u128); N > 128 uses `WideBits<bytes_for(N)>`.**

Five native buckets below 128:
- 1 ≤ N ≤ 8 → u8
- 9 ≤ N ≤ 16 → u16
- 17 ≤ N ≤ 32 → u32
- 33 ≤ N ≤ 64 → u64
- 65 ≤ N ≤ 128 → u128

Above 128: `WideBits<{bytes_for(N)}>` for Warm/Cold/Precise (align-1 byte-exact); the Hot variant uses a cfg-driven alignment wrapper around the same byte sequence.

Preserves the substrate's existing native-arithmetic codegen quality below 128. WideBits above 128 is the only zone where byte-level composition is the routine path; below 128 the substrate keeps speaking native.

Rejected alternatives:
- Uniform WideBits<bytes_for(N)> for ALL N — simpler shape but gives up native-ops codegen below 128 for no benefit; the substrate's primary downstream consumers already exercise N ≤ 64 heavily and pay the native-arithmetic price every cycle.
- Hybrid via repr(transparent) specialisation hooks — fragile; depends on layout assumptions sketches 02-03 didn't prove.

### Decision 3 — Container projection trait unification

**Delete `UContainerFor<N>` and `IContainerFor<N>` entirely. `BitsContainerFor<const N: Width, Sign: Signedness>` is the single projection trait.**

Per `no-legacy-shims-pre-1.0`, no deprecation wrappers, no re-exports. Delete the old shapes; rewrite call sites.

UFixed where-clauses migrate from `S: UContainerFor<N>` to `S: BitsContainerFor<N, Unsigned>`; IFixed similarly to `S: BitsContainerFor<N, Signed>`. The user-facing `UFixed<I, F, S>` and `IFixed<I, F, S>` types stay unchanged — only their where-clauses move.

Rejected alternatives:
- Keep U/IContainerFor as type aliases re-exporting the unified shape — exactly the deprecation pattern banned by no-legacy-shims-pre-1.0.
- Keep three traits coexisting — triple trait-solver work for no benefit; doesn't match what sketches 05-06 validated.

### Decision 4 — Width newtype in const-generic position

**`const N: Width` in every projection trait + Bits + UFixed/IFixed where-clauses.**

Round 2 (#312) already lifted `Bits<const N: Width>`. The projection traits still use `const N: u16` (verified at `mock/crates/arvo-strategy/src/container.rs:26,46,71`). This round lifts the projection traits to `const N: Width` to match. Width is a Round 2 newtype around u16; cap stays at 65535 logical bits (8KB per `Bits<N>` value).

This is the corollary the audit's R1 finding flagged ("Round 3 chicken-and-egg with Width relocation"). With #316 doing the lift, Round 3's mechanical work proceeds without relocation friction.

### Decision 5 — Single-impl projection via feature(generic_const_exprs)

**Per-strategy impl blocks (4 total: Hot, Warm, Cold, Precise), each computing the associated type via const fn over N.**

The const fn cascade:
1. `bytes_for(n: Width) -> usize` — round logical bits up to bytes.
2. The impl's `type T` is computed as either a native primitive (N ≤ 128) or `WideBits<{bytes_for(N)}>` (N > 128) for Warm/Cold/Precise.
3. Hot wraps the same in a cfg-driven aligned type for the SIMD path.

Sketch 05 validated the pattern: `where [(); bytes_for(N)]:` is the witness that satisfies the trait solver. Per-N projection enumeration goes away (256 entries → 4 impls).

The "if N ≤ 128 then native else WideBits" step needs an internal mechanism. Two viable patterns from sketch validation:

- **Pattern A**: helper trait `NativeOrWide<N>` with two non-overlapping impls (one for N ≤ 128 with five sub-impls per native bucket; one for N > 128 with the WideBits projection). Each impl bottoms out at the right concrete type. The strategy projection composes through this helper.
- **Pattern B**: a per-bucket sub-projection trait (5 native buckets + 1 WideBits bucket = 6 impls of a sub-trait), composed into the strategy projection.

Both express the same logic. Pattern A is fewer impls; Pattern B is more reflection-friendly for downstream traits that want to query "is this N native or wide?". Doc CL captures the choice; SRC CL drives toward whichever rustc accepts cleanly when implementation begins.

### Decision 6 — MultiContainer + MultiContainerHalf deletion (no shims)

**Both types and the `arvo-bits-contracts/src/multi_container.rs` BitPrim impl are deleted entirely.** No deprecation alias, no re-export. Per `no-legacy-shims-pre-1.0`. Every call site migrates to the new projection.

Affected files:
- `mock/crates/arvo-strategy/src/multi_container.rs` (112 LoC) — deleted.
- `mock/crates/arvo-bits-contracts/src/multi_container.rs` (entire file) — deleted.
- `mock/crates/arvo-strategy/src/lib.rs` — `pub mod multi_container` line removed; re-exports deleted.
- `mock/crates/arvo-bits-contracts/src/lib.rs` — `pub mod multi_container` line removed.
- `mock/crates/arvo-storage/src/lib.rs` — re-export removed if present.
- `mock/crates/arvo-strategy/src/arith.rs` (12 references) — every `MultiContainer<HiT, LoT>` impl block deleted; arithmetic on Bits above 128 bits goes through the `WideBits` BitPrim impls instead.
- `mock/crates/arvo-strategy/src/container.rs` (19 references) — per-N MultiContainer entries in U/IContainerFor tables deleted (the tables themselves are also deleted per Decision 3).
- `mock/crates/arvo-storage/src/bits.rs` (4 references) — type-inference helpers updated.
- `mock/crates/arvo-storage/src/layout_assertions.rs` (39 references) — assertions purged; replaced with new WideBits + AlignedWideBits layout assertions.

### Decision 7 — Doc updates across affected templates

**13 doc templates touch substrate names that this round removes or reshapes.** Each one needs a doc CL entry:

- `mock/DESIGN.md.tmpl`
- `mock/agent/MAIN.md.tmpl`
- `mock/crates/arvo/DESIGN.md.tmpl`
- `mock/crates/arvo/BACKLOG.md.tmpl`
- `mock/crates/arvo/DEEPDIVE_strategy-bound-trilemma.md.tmpl`
- `mock/crates/arvo-strategy/README.md.tmpl`
- `mock/crates/arvo-strategy/DESIGN.md.tmpl`
- `mock/crates/arvo-strategy/BACKLOG.md.tmpl`
- `mock/crates/arvo-storage/DESIGN.md.tmpl`
- `mock/crates/arvo-storage/BACKLOG.md.tmpl`
- `mock/crates/arvo-bits-contracts/DESIGN.md.tmpl`
- `mock/crates/arvo-bits-contracts/BACKLOG.md.tmpl`
- `mock/crates/arvo-hash/DESIGN.md.tmpl`

Doc CL drives the substantive rewrites; src CL is mechanical after.

## Sub-topic enumeration for the doc CL

The doc CL captures these sub-topics in per-crate, per-file detail:

- **T1**: Introduce `WideBits<const BYTES: usize>` in `arvo-storage` (new file `widebits.rs`). Const-generic over byte count; align-1 baseline.
- **T2**: Introduce strategy-aware aligned wrapper for Hot in `arvo-storage` (new file `aligned_widebits.rs` or co-located). cfg(target_feature) drives the literal alignment value.
- **T3**: Replace `UContainerFor<N>` + `IContainerFor<N>` + per-N `BitsContainerFor<N, Sign>` with one `BitsContainerFor<const N: Width, Sign: Signedness>` trait whose impls are 4 (one per strategy). Each impl computes `type T` via the native-or-WideBits cascade. Single-impl projection via feature(generic_const_exprs).
- **T4**: Delete `MultiContainer<HiT, LoT>`, `MultiContainerHalf`, and the BitPrim impl on MultiContainer. Update all call sites to the new projection. No deprecation shim.
- **T5**: Update `Bits<const N: Width, S, Sign>` to use the new projection (mechanical; the `<S as BitsContainerFor<N, Sign>>::T` lookup keeps the same shape).
- **T6**: Migrate UFixed/IFixed where-clauses to `BitsContainerFor<N, Unsigned>` / `BitsContainerFor<N, Signed>`. Public surface unchanged.
- **T7**: Purge `layout_assertions.rs` of MultiContainer assertions. Add new WideBits + AlignedWideBits + per-strategy projection layout assertions.
- **T8**: Update 13 doc templates. Substantive rewrites of strategy / storage DESIGN.md.tmpl + BACKLOG.md.tmpl entries that reference MultiContainer. The `DEEPDIVE_strategy-bound-trilemma.md.tmpl` keeps its trilemma framing but updates the worked example to use the new projection.
- **T9**: Sketch references in the doc CL — the doc CL cites sketches 02-06 by filename for each architectural claim that the sketches validate. Per `cl-claim-sketch-discipline.md`.
- **T10**: SRC CL `## CHANGE: <symbol> FROM ... TO ...` blocks for every substrate-name change. Per the structured grammar from `cl-claim-sketch-discipline.md`. Verified at lock time by discipline (until mockspace #318 ships).

## Out of scope for this round

- Bench-driven SIMD platform expansion beyond the SSE2 + NEON baseline. → #320.
- Asm microkernels for hot ops. → #321.
- HotTruncate strategy variant for wide widths (lossy compute opt-in). → #322.
- Mockspace cl-claim-vs-source-mismatch lint. → #318.
- Round 3 work (Mask + BitMatrix + mask-contracts + BitPrim::WIDTH typed). → #313, post-#316.

## Lock criteria

The topic file is frozen at commit. Before the doc CL opens:
- All structural decisions recorded above are final and reflect what the user approved.
- Sub-topic enumeration is complete enough that the doc CL has a per-crate, per-file scope to fill in.
- Out-of-scope follow-ups are linked to existing tasks (#318, #320, #321, #322, #313).

The doc CL captures per-crate, per-file mechanical scope. The src CL applies after doc CL locks. Lock + close per the standard mockspace flow.

## References

- Senior audit 2026-05-03: dispatched after Round 2 close; findings list above.
- Sketches: `mock/research/sketches/202605031400_hlist_heterogeneous_container/01-06`.
- Workspace rules: `~/Dev/clause-dev/.claude/rules/no-legacy-shims-pre-1.0.md`, `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`, `~/Dev/clause-dev/.claude/rules/arvo-toolbox-not-policer.md`, `~/Dev/clause-dev/.claude/rules/arvo-always-optimal-internals.md`, `~/Dev/clause-dev/.claude/rules/arvo-compile-time-last.md`.
- Tasks: #316 (this round), #317 (sketches, done), #318 (mockspace lint), #319 (workspace rule, done), #320 (SIMD expansion), #321 (asm microkernels), #322 (HotTruncate), #313 (Round 3, deferred).
