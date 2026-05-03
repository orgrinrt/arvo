**Date:** 2026-05-03T14:30Z
**Phase:** TOPIC
**Scope:** Corrective topic for round #316 foundational redesign. Folds in findings from senior architectural audit dispatched 2026-05-03T14:00Z.
**Source topics:** Original topic `202605031400_topic_foundational_redesign.md` (frozen at commit 9c5c672); senior audit verdict PROCEED-WITH-CHANGES.

# Corrective topic: audit-driven refinements

The original topic file recorded seven structural decisions for #316. A senior architectural audit poked holes in five of those, surfaced one critical semantics bug that would have caused silent consumer breakage, and identified six pre-doc-CL blockers. The audit verdict is **PROCEED-WITH-CHANGES**: the architectural direction holds (byte-sequence above 128, native-below, MultiContainer deletion, single-impl projection), but specific premises are oversold and decisions need refinement.

This corrective topic does not deprecate the original. It overrides where the original is wrong and extends where the original was incomplete. The original stays committed as audit trail; both topics feed into the doc CL.

## Audit verdict + summary

**Verdict**: PROCEED-WITH-CHANGES. Direction sound; specific decisions need fixing.

The audit found:
- 1 CRITICAL semantics bug (silent endianness flip in leading/trailing_zeros across the MultiContainer→WideBits transition).
- 2 CRITICAL framing errors (sketch 01 conclusion overstated; 128-bit boundary asserted but not defended).
- 4 HIGH issues (Hot alignment tiers cargo-culted; single-impl projection sketch incomplete; consumer-impact assertion not measured; Width relocation chicken-and-egg).
- 4 MEDIUM issues (cap framing sloppy, AVX-512 hybrid cores, doc CL per-template enumeration depth, const_mut_refs MSRV).
- 3 LOW issues (target list hedging, uninit pad bytes, read_unaligned hardening).
- 6 OBSERVED-CORRECT items (audit-validated wins worth recording).

## Pre-doc-CL blockers (audit-flagged, address in this corrective topic)

### Correction 1 — Lock byte-ordering convention for leading/trailing_zeros (audit C1)

**Decision: byte 0 is least-significant, mirror MultiContainer's lo-first convention.**

Concretely:
- Bit index 0 is the LSB of byte 0.
- `count_ones`: order-independent (commutative).
- `trailing_zeros`: walks from byte 0 LSB upward — byte 0's `trailing_zeros()` first; if all zero, advance to byte 1; etc.
- `leading_zeros`: walks from highest-index byte's MSB downward — `bytes[BYTES-1].leading_zeros()` first; if all zero, advance to `bytes[BYTES-2]`; etc.

Sketch 02 had the opposite convention (byte 0 most-significant, "for visual coherence with hex literals") and explicitly noted "production may differ". This corrective fixes the sketch convention to match MultiContainer's existing `lo`-first semantics, which preserves observable behaviour for every Bits<N> consumer that uses these methods.

**Why this is critical**: every existing consumer of `Bits<N, S, Sign>::leading_zeros` / `trailing_zeros` whose container shape currently lowers to MultiContainer expects bit 0 == LSB and the cascade to walk lo-first. If WideBits flipped this silently, the redesign would produce different numerical results for the same logical values — a broken-build-or-broken-test silent behaviour change. Per the topic-claim discipline, the corrective topic locks this convention so the doc CL records it explicitly and the src CL implements it correctly.

Documentation contract on the new `WideBits` BitPrim impl: "bit index 0 is the LSB of byte 0; `trailing_zeros` walks from byte 0 upward; `leading_zeros` walks from the highest-index byte downward; both functions match the semantics of `u128::trailing_zeros` / `u128::leading_zeros` viewed as bit-significance order, with byte 0 being the low 8 bits of the logical value."

### Correction 2 — Reframe sketch 01's invalidation finding accurately (audit C2)

**Decision: sketch 01 invalidated the heterogeneous-Cons claim *under repr(C)*, not heterogeneous storage in general.**

The original topic file recorded the finding as "the 'optimal-fit' heterogeneous-Cons was fiction" and the sketch README escalated to "the supposed benefit doesn't exist". Audit found this overstates: `repr(packed)` would have given exact-bit composition. The sketch dismissed `repr(packed)` in one parenthetical without testing it.

The corrected finding for the doc CL:
> Heterogeneous storage with natural alignment cannot beat homogeneous byte storage in physical size. We choose byte storage to avoid `repr(packed)`'s unaligned-access tradeoffs and the per-pair impl combinatorics, not because heterogeneous storage is impossible. The pivot is sound; the negative finding is narrower than the original framing implied.

The sketch README is updated separately (not edited; sketch is committed history) — the corrective topic carries the corrected framing forward into the doc CL.

### Correction 3 — Defend the 128-bit boundary or move it (audit C3)

**Decision: keep 128, with explicit framing as "rustc primitive boundary" — not "fully native instruction" boundary.**

The topic asserted N ≤ 128 maps to native primitives because that's "where native ops are best." Audit pushed back: u128 lowers to multi-instruction sequences on every relevant ISA (two `addcarry`-chain ops on x86-64 for add, multiple register-pair ops on aarch64). The actual single-instruction-add boundary is 64 on x86-64, 64 on aarch64.

The defended framing:
> The boundary at 128 captures rustc's native primitive ladder: u8/u16/u32/u64/u128 each have stable Rust support with stdlib ops that lower as well as the toolchain currently does for that width. Below 128 we stay in stdlib ops; rustc's u128 lowering improves over time and is the substrate's lower-bound expectation for "wide native". Above 128 we accept that no native primitive exists and route to byte-sequence storage with custom ops. If rustc gains a native u256 primitive (proposals exist), the boundary moves; the substrate is a single const fn change away from absorbing that. The 128 number is not a hardware-instruction claim; it is a rustc-primitive-ladder claim.

This framing also addresses audit M4 implicitly: rustc 1.83+ stabilises `const_mut_refs`, our nightly target is well past that, no MSRV concern.

Rejected: option (a) "drop to 64 boundary" — would force WideBits<16> chunked-u64 ops for 65-128, equivalent for bitwise/add/sub but worse for mul (no native u128 mul lowering for N=65..=128). The substrate keeps the u128 win.

### Correction 4 — Downscope Hot to align(16) baseline only (audit H1)

**Decision: `Hot` strategy uses align(16) baseline only. AVX-2 align(32) and AVX-512 align(64) are deferred to #320 with bench-driven evidence.**

Original topic decision 1 had Hot pick alignment cfg-driven per target_feature: SSE2/NEON → 16, AVX-2 → 32, AVX-512 → 64. Audit's H1 dismantled this:
- The aligned-vs-unaligned perf delta on Sandy Bridge+ is essentially zero outside cache-line crossings.
- Sketch 04 uses `_mm_loadu_si128` (unaligned), not `_mm_load_si128` (aligned). The aligned-load instruction variant requires alignment but the substrate doesn't use it.
- AVX-512 align(64) for `WideBits<17>` is 64 bytes for 17 logical bytes — 4× storage overhead, silently eaten by the consumer.
- The cfg-driven internal alignment with no consumer visibility is *exactly* the "policy hardcoded on consumer's behalf" pattern that `arvo-toolbox-not-policer.md` prohibits.

Downscoped Hot:
- `Hot` strategy = `AlignedWideBits16<bytes_for(N)>` for N > 128. Always align(16). This covers SSE2 + NEON (the actual current floor of supported targets).
- `Hot` for N ≤ 128: the native primitive picks its own alignment (u128 is align(16) on most x86-64 targets, align(8) on others). No additional wrapper.
- AVX-2 / AVX-512 alignment is a **separate axis** added in #320 if bench evidence justifies the storage overhead. Likely shape: opt-in `HotAvx2` / `HotAvx512` strategy markers, **consumer-visible**, with documented storage cost tradeoff. Toolbox-not-policer compliant.

This collapses the Hot story from "complex cfg-driven internal magic" to "Hot is align(16) baseline; wider alignment is a separate consumer-visible decision later." Cleaner, smaller surface, matches the workspace rules.

### Correction 5 — Sketch 07 written; Pattern C (const-tag dispatch) is the chosen mechanism (audit H2)

**Sketch 07 status: COMMITTED, WORKS via Pattern C.**

Sketch path: `mock/research/sketches/202605031400_hlist_heterogeneous_container/07_native_or_wide_projection.rs`. Compiled clean on rustc 1.96.0-nightly. 18 const-time assertions verify the projection across the full N range × all Sign × all Strategy combinations.

**Three patterns considered, only Pattern C works**:

- **Pattern A (helper marker — audit's suggested form)**: two impls of `BitsContainerFor<N, Sign>` on different marker types (`Marker` for native, `WideMarker` for wide), each with disjoint const-bool where clauses. **Compiles**, but the user-facing `Bits<N, S, Sign>` requires a single `<S as BitsContainerFor<N, Sign>>::T` lookup; the consumer cannot pick which marker to use. Doesn't compose into the substrate. Rejected.

- **Pattern B (direct overlap on Strategy)**: two impls of `BitsContainerFor<N, Sign> for Hot` with disjoint where-clauses (`(N <= 128) as usize - 1` vs `(N > 128) as usize - 1`). **Rustc rejects with E0119 (conflicting implementations)** even though the where-clauses are mutually exclusive. The trait solver does not reason about const-bool disjointness for overlap detection. Confirmed dead end (verified at `/tmp/sketch_07b_direct_overlap_test.rs`).

- **Pattern C (const-tag dispatch — chosen)**: single per-Strategy impl on the Strategy type itself. Internal const fn `tag(N) -> usize` returns 0..=5 (5 native buckets + 1 wide). A helper trait `Project<TAG, Sign, BYTES, S>` has per-(TAG, Sign, S) impls that are distinct because TAG is a const-generic value distinct per impl. No overlap; no E0119. Native buckets are Strategy-erased (10 impls: 5 buckets × 2 Sign); wide bucket is Strategy-dependent (8 impls: 1 bucket × 4 Strategy × 2 Sign). Plus 4 BitsContainerFor impls (one per Strategy). Total 22 impls replacing 2048+ per-N×Sign×Strategy entries — ~99% reduction.

The audit's H2 finding was correct in flagging this as a sketchable claim and correct that the if-then-else projection wouldn't work as the original topic vaguely described it. The audit's specific "Pattern A" suggestion was the intuition; the actual mechanism that compiles cleanly is Pattern C. Sketch 07 records both for the audit trail.

**Doc CL adopts Pattern C.** The src CL writes:
- `arvo-strategy/src/projection.rs` (new file): `tag()` const fn, `Project<TAG, Sign, BYTES, S>` helper trait, all 18 helper-trait impls, 4 user-facing `BitsContainerFor` impls.
- The user-facing `Bits<const N: Width, S: Strategy, Sign: Signedness>` resolves its container via `<S as BitsContainerFor<N, Sign>>::T` exactly as before — only the projection's internal mechanism changes.

### Correction 6 — Width relocates to arvo-strategy (audit H4)

**Decision: Width newtype relocates from arvo-storage to arvo-strategy as part of #316.**

Audit H4 caught a layering inversion: the projection traits `BitsContainerFor<const N: ?, Sign>` live in arvo-strategy. If `?` is `Width`, then arvo-strategy needs to reach `Width`, but `Width` currently lives in arvo-storage which depends on arvo-strategy (lint-forbidden-arvo-strategy prohibits arvo_storage::* in arvo-strategy).

Per `arvo-bridge-home-rule.md`: a substrate type lives in the lowest crate where its return type / position-as-const-generic-param is reachable. Width's position-as-const-generic-param of the projection traits forces it to be reachable from arvo-strategy. Therefore Width relocates to arvo-strategy (or to a layer at-or-below arvo-strategy, but no lower layer needs it).

Concrete moves:
- `Width` newtype declaration moves from `arvo-storage/src/meta_bits.rs` to `arvo-strategy/src/width.rs` (new file).
- arvo-storage re-exports it via `pub use arvo_strategy::Width;` for source-compatibility within arvo's facade.
- The lint pack's `lint-forbidden-arvo-strategy` rule is unchanged (arvo-strategy still doesn't depend on arvo-storage).
- Round 2's `Bits<const N: Width>` lift continues to work — the Bits declaration in arvo-storage now reaches Width via its own crate's re-export from arvo-strategy.
- Round 2's `MetaCarrier` companion stays in arvo-storage (it carries Width but doesn't define it; companion stays where ConstParamTy_-shaped types belong).

Rejected: keep projection traits on `const N: u16`. Wastes the typed-position lift Round 2 already did; would mean the projection layer downcasts Width to u16, which is a step backward.

## Medium-priority refinements (folded in)

### M1 — Cap framing tightened

Original topic claimed Width=u32 lift "no architectural impact on the WideBits shape." Audit M1 corrected: Width=u32 implies `bytes_for(N)` up to ~537M bytes, which exceeds usize on 32-bit targets — that would be a different substrate (heap-resident, pageable). The 65535-bit cap is fine for current consumers (post-quantum lattice crypto, RSA up to 16384, ECC field elements all fit). Doc CL records: "65535-bit cap is sound for in-scope workloads; lifting Width is not a same-substrate change."

### M2 — AVX-512 heterogeneous-core targets

The cfg model is fine because it's compile-time-per-build. Doc CL clarifies: "cfg(target_feature = 'avx512f') means the build assumes AVX-512 is uniformly available on the target. Runtime feature detection (CPUID-gated dispatch) is a separate axis covered by #320." Intel hybrid P+E cores disable AVX-512 globally; Zen 4 enables it per-core. The substrate doesn't try to handle hybrid-runtime-state; that's the OS's job.

### M3 — Doc CL per-template enumeration depth

The original topic listed 13 doc templates by filename. Audit M3 raised: per-template, what backticked names are removed (`MultiContainer<HiT, LoT>`, `MultiContainerHalf`, `UContainerFor`, `IContainerFor`), what's reshaped (`BitsContainerFor`), what's added (`WideBits`, `AlignedWideBits16`). Doc CL must enumerate per-template, not just listing filenames. Lint-time grep for stale `MultiContainer` references is the lock criterion.

## Low-priority refinements (folded in)

### L1 — Target list hedging

README and doc CL phrase target support as "modern application-class targets (x86-64 ≥ Sandy Bridge 2011, aarch64, ARMv7-A, RISC-V cores with `Zicclsm`, WASM)". Embedded MCU targets are not in scope; the substrate's no-std discipline doesn't imply MCU support.

### L2 — Uninitialised pad bytes UB note

`AlignedWideBits16<BYTES>::from_bytes(bytes: [u8; BYTES])` stores the array but the trailing pad bytes (size_of::<Self>() - BYTES) are not zero-initialised. Reading the full struct as bytes via transmute or raw-pointer cast is UB. Doc CL records this on the type's safety contract; only `as_bytes()` (which returns `&[u8; BYTES]`, not the full struct) is safe.

### L3 — read_unaligned over chunks_exact + try_into

Sketch 04's `count_ones_chunked_u64` uses `chunks_exact` + `try_into().unwrap()`. Production WideBits ops should use `core::ptr::read_unaligned` directly to avoid the panic-branch. Tracker only — #320 territory.

## Audit-validated wins to record in doc CL

The audit's OBSERVED-CORRECT findings should be cited explicitly in the doc CL so the audit trail records what was validated, not just what was changed:

- **O1**: MultiContainer "u64+u128 saves bytes" was always 32 bytes (sketch 01 + audit verification). Documented inline in doc CL per `cl-claim-sketch-discipline.md`.
- **O2**: Single-impl-per-strategy via const-fn associated type is sound for the uniform case. Sketch 05 validated; sketch 07 extends to the native-vs-wide branching.
- **O3**: Cold's column-store layout is above the per-Bits container. The previous design that conflated this is corrected.
- **O4**: WideBits<BYTES> align-1 uniform across widths is verified (sketch 02).
- **O5**: cfg(target_arch) intrinsic surface compiles cleanly on x86_64 + aarch64 (sketch 04).
- **O6**: no-legacy-shims-pre-1.0 application is correct; MultiContainer deletion without alias is right.

## H3 measured: zero consumer references

Audit H3 required measuring MultiContainer references in workspace consumers (hilavitkutin, viola, vehje, notko, viola-grammar-ts, viola-script-lints, mockspace, mockspace-hilavitkutin-stack-lints).

Result (measured 2026-05-03T14:30Z): **zero** references in any consumer repo. Deletion is safe; no migration appendix needed.

## Updated lock criteria for #316 doc CL

Adding to the original topic's lock criteria:

- All six audit-flagged blockers (C1, C2, C3, H1, H2, H4) addressed in this corrective topic.
- Sketch 07 written, committed, and demonstrating native-or-wide projection compiling.
- Width relocation path documented per-file (arvo-strategy/src/width.rs new, arvo-storage re-export, arith.rs internal mig, etc.).
- Hot strategy downscoped to align(16) baseline only.
- Byte-ordering convention locked.
- 128-bit boundary defended explicitly as rustc-primitive-ladder.
- Consumer-impact measurement (zero references) recorded.
- OBSERVED-CORRECT items cited inline in doc CL where they validate architectural claims.

## What does NOT change from the original topic

These decisions stand unchanged:

- D2 (native-below-128, WideBits-above-128) — core architecture preserved, framing of the boundary refined per C3.
- D3 (delete U/IContainerFor, unified BitsContainerFor only) — projection trait unification stands.
- D6 (MultiContainer + MultiContainerHalf full deletion, no shims) — direction unchanged; H3 confirms safety.
- D7 (13 doc templates need updates) — list stands; per-template enumeration added per M3.

## References

- Original topic: `mock/design_rounds/202605031400_topic_foundational_redesign.md` (frozen at commit 9c5c672).
- Audit verdict + findings: senior architectural review dispatched 2026-05-03T14:00Z (returned 14:55Z).
- Workspace rules referenced: `arvo-toolbox-not-policer.md`, `arvo-always-optimal-internals.md`, `arvo-bridge-home-rule.md`, `cl-claim-sketch-discipline.md`, `no-legacy-shims-pre-1.0.md`.
- Tasks: #316 (in-progress), #317 (sketches; sketch 07 requirement added per H2), #318 (mockspace lint), #320 (SIMD platform expansion incl Hot AVX-2/AVX-512 deferred per H1), #321 (asm microkernels), #322 (HotTruncate), #313 (Round 3 deferred behind #316).
