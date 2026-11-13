### mock/design_rounds/202604191900/202603241200_topic.arvo-strategy-markers.md
- date: 2026-03-24
- by: not stated
- holds: Topic resolving what the four strategy markers (Hot, Warm, Cold, Precise) do for `UFixed<I, F, S>` and `IFixed<I, F, S>`: container choice, arithmetic semantics, layout derivation, conversions, semantic aliases. Status says in progress, with four open questions.
- headline: "(default: `Warm`). The strategy determines the backing container" (line 41)
- watch: "## Four strategies" (line 49)
- watch: "**Same-width arithmetic:** wrapping. Single instruction per op. May" (line 62)
- watch: "**Same-width arithmetic:** saturating. Overflow clamps to max/min" (line 148)
- watch: "Bitpacking requires BitPacked-capable storage (a trait contract the" (line 114)
- watch: "Consumers can override the strategy: `UFixed<0, 8, Hot>` is a" (line 298)
- watch: "1. **Warm default on release vs debug:** should Warm behave differently" (line 305)

### mock/design_rounds/202604191900/202603241300_topic.arvo-strategy-aliases-and-open-questions.md
- date: 2026-03-24
- by: not stated
- holds: Topic making semantic aliases generic over the strategy and resolving four open questions from the strategy-markers topic (Warm in debug vs release, IFixed signed containers, maximum width and u128, mixed-strategy columns).
- headline: "Aliases fix the `<I, F>` configuration (that's the semantic meaning)" (line 17)
- watch: "**Resolution: Warm is the same everywhere.** No debug/release split." (line 57)
- watch: "If you want debug assertions on overflow, use Precise — it saturates" (line 66)
- watch: "Signed saturation clamps to `i_MIN..i_MAX` of the logical range" (line 80)
- watch: "**Resolution: I + F <= 64. No u128 containers.**" (line 87)
- watch: "fn process<S: Strategy>(angle: Angle<S>, ratio: Ratio<S>) -> Angle<S> {" (line 41)

### mock/design_rounds/202604191900/202603241400_topic.arvo-crate-dependency-revision.md
- date: 2026-03-24
- by: not stated
- holds: Topic revising the arvo sub-crate dependency tree: introduces `arvo-bits` (BitWidth, BitAccess, BitSequence) beneath `arvo-bitmask`, and states that `arvo-graph`, `arvo-sparse`, `arvo-spectral` and `arvo-comb` are generic over numeric trait bounds rather than concrete arvo types.
- headline: "The mathematical sub-crates (arvo-graph, arvo-sparse, arvo-spectral," (line 187)
- watch: "bitmasks are not numeric types — they're bit-level types with" (line 14)
- watch: "If W is `UFixed<16, 0, Hot>`, rank accumulation wraps on overflow." (line 204)
- watch: "If W is `UFixed<16, 0, Precise>`, it saturates. The algorithm is" (line 205)
- watch: "identical. The consumer chooses the safety level by choosing W." (line 206)
- watch: "W: Add<Output = W> + TotalOrd + Copy + FromConstant" (line 197)
- watch: "These concrete instantiations happen in hilavitkutin, not in the" (line 281)

### mock/design_rounds/202604200100/202604200000_topic.arvo-bits-impl-strategy.md
- date: 2026-04-20
- by: not stated
- holds: Topic choosing, among three options, to implement `arvo-bits` traits (BitWidth, BitAccess, BitSequence) as concrete impls on `UFixed`/`IFixed` via a sealed `BitPrim` trait, instead of extending the `BitPresentation` marker in arvo's L0. Status says resolved.
- headline: "Option 2. Keep the arvo-bits scope tight and avoid reopening the" (line 44)
- watch: "`BitPresentation` as shipped in L0 exposes only `LOGICAL_WIDTH: USize`" (line 18)
- watch: "round. Concrete impls over `UFixed<const I: IBits," (line 45)
- watch: "`BitAccess`, and `BitSequence`. A sealed `BitPrim` trait in" (line 47)

### mock/design_rounds/202604200935/202604200930_topic.arvo-sweep-primitive-vocabulary.md
- date: 2026-04-20
- by: not stated
- holds: Topic listing the arvo-side drift from the workspace primitive-vocabulary discipline (bare `Option`/`Result` to `Maybe`/`Outcome`, `Vec` in tests, raw numeric const-generic accessors kept with an annotation) and the std-boundary `TryFrom` adapter shape. Status says resolved.
- headline: "Scope is the arvo repo only. hilavitkutin + clause consume arvo, but" (line 23)
- watch: "- `arvo_sparse::csr::CsrMatrix::get(&self, row: USize, col: NodeId)" (line 39)
- watch: "- `arvo-bitmask/tests/{mask_scan,mask256}.rs` collect `iter_set_bits()`" (line 52)
- watch: "`.try_into()` routes through" (line 60)
- watch: "Resolution: keep. Annotate each with" (line 98)

### mock/design_rounds/202604201005/202604201000_topic.arvo-identity-fixed-point-first.md
- date: 2026-04-20
- by: not stated
- holds: Doc-only topic restoring arvo's identity framing as a fixed-point arithmetic stack in the principles and design templates, quoting the originals from polka-dots and saalis, and tying the framing to the `arvo-types-only` lint. Status says resolved.
- headline: "> **arvo is a fixed-point arithmetic stack: the exclusive numeric" (line 75)
- watch: "> `UFixed` / `IFixed` / `FastFloat` / `StrictFloat`, never through" (line 78)
- watch: "- Strategy markers stated as type parameters, not runtime config —" (line 97)
- watch: "- Adding new arvo primitives (signed `IByte`/`IWord`/`IDWord`/`IQWord`;" (line 135)

### mock/design_rounds/202604201338/202604201338_topic.arvo-hash-new-crate.md
- date: 2026-04-20
- by: not stated
- holds: Topic adding an opaque `Bits<const N: u8>` to `arvo-bits` and a new `arvo-hash` crate carrying the `ContentHash = Bits<28>` alias plus generic `HashAlgo<N>` and `Hasher<N>` contracts, with eight questions answered by user decisions or provisional choices.
- headline: "**Decision (user, 2026-04-20):** `Bits<const N: u8>` lives in **arvo-bits** (not in arvo-hash); `arvo-hash` defines `pub type ContentHash = arvo_bits::Bits<28>` and depends on arvo-bits." (line 56)
- watch: "**Decision (user, 2026-04-20):** types + **generic algorithm contracts** (no concrete impls)." (line 77)
- watch: "- **A (recommended provisional)** — no Strategy. Hash bits are opaque identities; there is no precision/throughput tradeoff to tune." (line 69)
- watch: "`arvo-bits` offers `BitWidth`, `BitAccess`, `BitSequence` as the standard bit-contract set." (line 104)
- watch: "pub trait HashAlgo<const N: u8> {" (line 83)
- watch: "- Strategy rule: `S: Strategy` required when the type has precision/throughput tradeoff. Hash bits DO NOT — see Q2." (line 49)

### mock/design_rounds/202604270000/202604270000_topic.arvo-bitmask-sweep.md
- date: 2026-04-20
- by: not stated
- holds: Topic recording the 41 bare-primitive violations in `arvo-bitmask`, grouped into classes A to E, and the user decisions on the fix: add a `BitLogic` trait to `arvo-bits`, annotate `core` trait-method signatures and `cap_size` with `lint:allow` tracked by task, rewrite the set-bit iterators over `QWord<Hot>`.
- headline: "**Q1 (user, 2026-04-20):** Option A — extend arvo-bits trait surface with" (line 300)
- watch: "whole-word bitwise ops (`bitor`, `bitand`, `bitnot`, `bitxor`). Classes C+D+E" (line 301)
- watch: "**Q5 (user, 2026-04-20):** Option A — new `BitLogic` trait, plus additionally" (line 352)
- watch: "**Implementing core TRAITS (CONTRACTS) on our" (line 358)
- watch: "**Q6 (user, 2026-04-20):** Option A — per-type concrete impls + separate" (line 395)
- watch: "Core trait-method signatures cannot be changed — the contract fixes `Option`" (line 39)
- watch: "`BitSequence::{trailing_zeros, leading_zeros, count_ones, count_zeros, is_zero}` — whole-word scans returning `USize` / `Bool`." (line 104)

### mock/design_rounds/202604271346/202604271346_topic.const-traits-and-arvo-as-facade.md
- date: 2026-04-27
- by: not stated
- holds: Topic recording a structural redesign: `arvo` becomes a downstream facade over a minimal L0 (`Bits<N, S>`, strategy markers, `Bool`, `USize`), `UFixed`/`IFixed` become compositions over `Bits`, trait contracts split into granular contract crates (`arvo-transparent`, `arvo-strategy`, `arvo-bits-contracts`, `arvo-mask-contracts`, `arvo-narrow-contracts`, `arvo-numeric-contracts`) as `pub const trait`, and `Int<N>`/`Uint<N>` aliases replace per-width aliases. Decisions D-1 to D-12 and resolved questions Q-A to Q-E.
- headline: "The redesign: arvo becomes a downstream facade, not L0. Real L0 shrinks to a minimal set of bare-primitive wrappers." (line 14)
- watch: "| `arvo-transparent` | `Transparent` (const unsafe), `NumericPrimitive` (marker), free `arvo::raw` fn |" (line 58)
- watch: "| `arvo-bits-contracts` | `HasBitWidth` (const), `BitAccess` (const), `BitSequence` (const), `BitLogic` (const), `BitPrim` family (const) |" (line 60)
- watch: "| `arvo-mask-contracts` | `Mask<W>` (const) — abstract trait over Mask64/Mask256/etc. |" (line 61)
- watch: "| `arvo-numeric-contracts` | `Abs` (const), `Recip` (const), `Sqrt` (const), `TotalOrd` (const), `FromConstant` (const), `Predicate` family |" (line 63)
- watch: "Semantic convention: **N = total storage bit count**. `Int<8>` ↔ `i8` semantics (1 sign + 7 integer bits internally). `Uint<32>` ↔ `u32` semantics (32 magnitude bits). This matches std and reads cleanly." (line 139)
- watch: "Cap: `N <= 64` implicitly via `Hot: UContainerFor<N>` (or `Warm`/`Precise` for narrower). Wider widths require Cold (bitpacking) and aren't in scope this round." (line 151)

### mock/design_rounds/202604271346/202604500000_topic.bitwidth-newtype-and-fnv1a.md
- date: 2026-04-26
- by: not stated
- holds: Topic introducing a `BitWidth(u8)` ConstParamTy newtype beside `IBits`/`FBits`, migrating `Bits`, `Hasher`, `HasherExt` and `UContainerFor` const parameters from bare `u8` to it, and landing a concrete `Fnv1a<const N: BitWidth>` hasher with a const-fn `hash_const`.
- headline: "Introduces a `BitWidth` ConstParamTy newtype as the canonical const-generic parameter type for bit-count positions, paralleling existing `IBits` / `FBits` for fixed-point integer / fractional widths." (line 5)
- watch: "Algorithm: canonical FNV-1a-64 (offset basis `0xcbf29ce484222325`, prime `0x100000001b3`). For N < 64, mask the low-N bits of the 64-bit accumulator (matches `hilavitkutin-str::const_fnv1a` semantics for the 28-bit `Str` case)." (line 63)
- watch: "The `UContainerFor` trait (whichever crate defines it; appears to be arvo-bits or arvo) carries the same migration. Bound sites in arvo-bits / arvo-bitmask / arvo-tensor / arvo-graph / arvo-sparse / arvo-comb / arvo-spectral / arvo-hash all update from `Hot: UContainerFor<N>` (with N: u8) to the BitWidth form." (line 40)

### mock/design_rounds/202604271346/202604500000_topic.fixed-point-foundations-and-fnv1a.md
- date: 2026-04-26
- by: not stated
- holds: Topic restating round 202604500000 on the basis that fixed-point types are the foundation: adds L0 `Uint5..Uint64`/`Int8..Int64` aliases over `UFixed`/`IFixed` at `Hot`, gives `UFixed`/`IFixed` `ConstParamTy`, makes `IBits`/`FBits`/`BitWidth` wrap `Uint7`, migrates `Bits`/`Hasher` const parameters, ships `Fnv1a`, and sweeps arvo source for bare primitives. Names the prior bitwidth topic as partially superseded.
- headline: "1. **UFixed is the foundation, not the leaf.**" (line 12)
- watch: "Narrow-bit unsigned aliases (`Uint5`, `Uint6`, `Uint7`) are first-class because the substrate's whole point is exact widths, not byte buckets." (line 39)
- watch: "Algorithm: canonical FNV-1a-64 (offset basis `0xcbf29ce484222325`, prime `0x100000001b3`). For N < 64, mask the low-N bits of the 64-bit accumulator." (line 126)
- watch: "Derive `ConstParamTy` (with `feature(adt_const_params)`) on `UFixed` and `IFixed`." (line 45)
- watch: "`UContainerFor<const N: BitWidth>` migrates the same way." (line 99)

### mock/design_rounds/202604271346/202604500000_topic.foundations-restructure-and-fnv1a.md
- date: 2026-04-26
- by: not stated
- holds: Topic answering an independent design review: moves `Bits<const N: u8>` to arvo L0, makes `IBits`/`FBits`/`Width` wrap `Bits<7, Hot>`, builds `UFixed`/`IFixed` over `Bits` for `ConstParamTy`, makes the `Uint*`/`Int*` aliases generic over strategy with a `Warm` default, reshapes `USize`, bounds `Fnv1a<const N: Width>` at 64, and enumerates the sweep. Names two earlier topics as superseded.
- headline: "This topic addresses every finding by fix-not-drop." (line 8)
- watch: "Strategy is a storage / operational hint; the same numeric domain (`Uint64`) is roughly equivalent across strategies post-deserialisation." (line 87)
- watch: "H5 fix. UFixed-based aliases ship as generic over S with `Warm` default (matching `UFixed::default()`)." (line 70)
- watch: "`Bits<N, S>` aliases (`Bit`, `Nibble`, `Byte`, `Word`, `DWord`, `QWord`) keep `S = Hot` default." (line 89)
- watch: "- arvo-bits L1 stays: trait contracts only (`HasBitWidth` trait, `BitAccess`, `BitSequence`, `BitLogic`) plus blanket impls for arvo's primitive types." (line 17)
- watch: "- arvo-bitmask: `Mask64` / `Mask256` / `BitMatrix<...>` const-generic positions migrate to `Width`." (line 178)
- watch: "3. `Strategy` markers (`Hot` / `Warm` / `Cold` / `Precise`) themselves are unit structs that derive `ConstParamTy + PartialEq + Eq + Copy + Clone + Debug`." (line 64)

### mock/design_rounds/202604271346/202604500000_topic.foundations-restructure-final.md
- date: 2026-04-26
- by: not stated
- holds: Fourth topic of round 202604500000, resolving the open decisions from a second review: `ConstParamTy` narrowed to `Bits`, `IBits`, `FBits`, `Width` and the strategy markers; `USize` keeps `Deref<Target = usize>`; `Fnv1a<N>` bounded by a const-eval guard; `from_raw_uN` masks to N bits; struct-level `Warm` default dropped; `BitWidth` trait renamed `HasBitWidth`; `UContainerFor` gains a diagnostic attribute. Names the third topic as superseded.
- headline: "This topic resolves every open decision concretely; nothing is deferred to \"verify during SRC\" or \"pick one in iteration.\"" (line 6)
- watch: "Critical for ConstParamTy soundness. `Bits<7, Hot>::from_raw_u8(255)` masks to `255 & 0b0111_1111 = 127`." (line 87)
- watch: "Two sources of Warm-default (struct-level + alias-level) creates drift risk. Drop the struct-level default. Aliases carry the strategy default at the consumer surface." (line 107)
- watch: "note = \"Warm caps at I+F<=32; for wider widths, choose Hot or Precise explicitly: `Uint64<Hot>`, `Uint64<Precise>`, etc.\"," (line 133)
- watch: "Mechanical rename of `arvo_bits::BitWidth` → `arvo_bits::HasBitWidth`." (line 124)
- watch: "arvo-bits stays as L1 trait-only crate: `HasBitWidth` (renamed), `BitAccess`, `BitSequence`, `BitLogic` traits + blanket impls on arvo's primitives." (line 169)
- watch: "Per H5: `Uint5<S = Warm>` through `Uint32<S = Warm>` work directly." (line 183)

### mock/design_rounds/202604280000/202604280000_topic.arvo-comb-sweep.md
- date: 2026-04-20
- by: not stated
- holds: Topic recording 46 bare-primitive sites in `arvo-comb` (loop-index `usize`, internal `bool` buffers, public callback `bool` returns) and the user decisions: introduce a new `arvo-tensor` crate hosting `Array`, `Matrix` and `cap_size`, an `Enumerator` trait, `Matrix<Bool, N>` buffers, and `Bool`-returning callbacks.
- headline: "The correct remediation for arvo-comb is to introduce the 1D sibling of" (line 188)
- watch: "**Q1a (user, 2026-04-20):** New crate named `arvo-tensor`." (line 199)
- watch: "**Q3 (user, 2026-04-20):** Change `feasible: impl Fn(...) -> bool` on both" (line 256)
- watch: "**Q4 (user, 2026-04-20):** Defer `arvo::Predicate<X>` trait alias to a" (line 260)
- watch: "`Matrix<W, const N: Cap>` — a `#[repr(transparent)]` wrapper over" (line 182)

### mock/design_rounds/202604280034/202604280034_topic.audit-driven-hygiene-and-Q-D-completion.md
- date: 2026-04-28
- by: not stated
- holds: Round A topic of an audit-driven chain: lint-annotation format fixes, agent-doc sync, doc fixes, `Mask<W>` impls on `Mask64`/`Mask256`, deletion of the `Uint8`..`Int64` std-parallel aliases, merge of `arvo-narrow-contracts` into `arvo-bits-contracts`, and a doc-only IFixed deferral note. States that all decisions are pre-decided and later rounds B to E take the larger items.
- headline: "All decisions in this topic are pre-decided per the 2026-04-28 dual-agent audit (impl audit + perf-expert design critique) and the user's response in the originating session." (line 5)
- watch: "The decision: delete these too. The user's framing: retaining std-parallel sugar undermines the entire point of the substrate, which is to think in exact bits, not in std-primitive buckets." (line 43)
- watch: "- `Uint64<S>` and `Int64<S>` have no `Warm` default because Warm caps at 32-bit logical width." (line 30)
- watch: "does not implement the `Mask<const W: u8>` trait declared in `arvo-mask-contracts`." (line 37)
- watch: "Per workspace rule `arvo-toolbox-not-policer.md`, Cold widen/narrow impls are committed to landing eventually; the BACKLOG-document path is correct." (line 38)
- watch: "- Strategy axes decomposition (Round C)." (line 68)
- watch: "- Cross-strategy resolution warning (Round C)." (line 70)

### mock/design_rounds/202604280100/202604280100_topic.arvo-graph-sweep.md
- date: 2026-04-21
- by: not stated
- holds: Short topic listing six bare-primitive sites (counters and a flag array) in `arvo-graph` and the mechanical resolution of wrapping them as `USize` and `Bool`, with no public signature changes.
- headline: "All sites wrap to USize/Bool. No AskUserQuestion needed — pattern from rounds 1-2 applies directly." (line 45)
- watch: "- `let mut flag: bool = false;` → `let mut flag = Bool::FALSE;` with `.0` extraction." (line 25)
- watch: "The `arvo-tensor::Array<Bool, N>` wrapper is overkill for `is_waist`; staying on raw `[Bool; cap_size(N)]` is the minimum delta." (line 28)

### mock/design_rounds/202604280200/202604280200_topic.arvo-sparse-sweep.md
- date: 2026-04-21
- by: not stated
- holds: Short topic listing five bare-primitive sites in `arvo-sparse` (local counters, two private helpers, one tuple) and the mechanical rewrite to `USize` with no public API change.
- headline: "No new contracts or structural types. No public API change. No `lint:allow` annotations required." (line 25)
- watch: "- Public `CsrMatrix` surface changes — stays as-is." (line 30)

### mock/design_rounds/202604280300/202604280300_topic.arvo-spectral-sweep.md
- date: 2026-04-21
- by: not stated
- holds: Short topic listing four bare-primitive sites in `arvo-spectral` and their mechanical rewrites, with one `lint:allow` pointing at a new task to make `FromConstant` take `USize` instead of `u8`.
- headline: "Mechanical. No public API change to arvo-spectral." (line 36)
- watch: "- fiedler.rs:92 → `lint:allow(no-bare-numeric)` inline with reason pointing at a new task: `arvo::traits::FromConstant::from_constant signature takes u8; wrapping conversion needed until FromConstant is retrofitted to take USize`." (line 28)
- watch: "- `FromConstant` redesign itself (arvo L0 concern, own round)." (line 31)

### mock/design_rounds/202604280400/202604280400_topic.arvo-l0-core-traits.md
- date: 2026-04-21
- by: not stated
- holds: Short topic recording four `core::convert::TryFrom` impls on `IFixed`/`UFixed` (Warm or Precise to Hot) and deciding to annotate them with `lint:allow(no-bare-result)` tracked at #115 rather than remove them.
- headline: "All 4 sites are core trait contract: `core::convert::TryFrom::try_from` returns `Result<Self, Self::Error>`." (line 18)
- watch: "Consumers use `UFixed::<Hot>::try_from(val: UFixed::<Warm>)` to downcast Warm/Precise to Hot with bounded-range narrowing." (line 24)
- watch: "per user's #115 clarification (\"freedom from core is mostly for core concrete types in signatures, not for implementing core-defined contracts\")" (line 24)

### mock/design_rounds/202604280500/202604280500_topic.signed-bits-axis-and-bitwidth-cap-256.md
- date: 2026-04-28
- by: not stated
- holds: Round B topic recording user decisions: a `Sign: Signedness` axis on `Bits` with sealed `Unsigned`/`Signed` markers, widening the meta-newtypes to `Bits<9, Hot>` to reach 256 bits, a strategy-driven `MultiContainer<HiT, LoT>` for widths 65 to 256, extending the `UContainerFor`/`IContainerFor` tables (including Warm into 33 to 256), and reshaping `IFixed` over `Bits<{1+I+F}, S, Signed>`. Leaves three shape choices to the doc phase.
- headline: "All three internal design decisions for this round were resolved by the user during the originating session (2026-04-28); this topic file records them." (line 5)
- watch: "**Decision: option 2.** The user's framing: \"Bits<9, Hot> with honest 0..=511 native range. No offset trickery; 1 wasted const-generic bit costs nothing.\"" (line 84)
- watch: "Per the user's framing: \"what if we want 92bit? 64+32 would be the optimal smallest combo, not 64+64. Needs proper flexibility. Though strategy should hold here too, since two same length will be better for most cases, while smallest footprint bitwise might be better for others.\"" (line 108)
- watch: "- Warm 33..=64: 2x logical, single primitive (`u128` for 33..=64)." (line 156)
- watch: "- Precise 65..=256: same as Warm with saturating arithmetic." (line 163)
- watch: "Per the workspace `arvo-toolbox-not-policer` rule, exposing Warm at wider widths gives consumers the development-friendly default at every width;" (line 165)
- watch: "Lean: extend UArith / IArith." (line 183)

### mock/design_rounds/202604280806/202604280806_topic.cycle-workaround-and-internal-axes.md
- date: 2026-04-28
- by: not stated
- holds: Round C topic with four sub-topics: a `MetaCarrier` parallel carrier to avoid a rustc trait-solver cycle in the 9-bit meta-newtypes (adopted), decomposing `Strategy` into overflow, container-width and storage-layout axis traits (adopted as traits only), and bitfield `S` parameterisation and a cross-strategy warning (both deferred).
- headline: "The parallel-carrier approach is the only path that demonstrably bypasses the trait-solver cycle while preserving layout equivalence and the conceptual framing." (line 115)
- watch: "The `Strategy` axis (Hot / Warm / Cold / Precise) couples three orthogonal concerns:" (line 119)
- watch: "Coupling means a consumer choosing \"I want bitpacked storage\" (Cold) also gets \"wrapping arith with widen-op-narrow\" — they cannot mix bitpacking with saturating arith without writing an entirely new strategy marker." (line 127)
- watch: "Public surface unchanged — `Hot` / `Warm` / `Cold` / `Precise` remain the only strategy markers consumers see." (line 148)
- watch: "When a binary op `Hot * Warm` resolves via `Resolve<Hot, Warm> = Warm`, the resulting expression silently adopts Warm's overflow policy (wrapping at 2x-logical width)." (line 168)
- watch: "Compile-time `#[deprecated]` or `#[diagnostic]` warning at the cross-strategy op site when the resolved strategy differs from both operands' strategy." (line 172)
- watch: "- User directive: do not defer Pass 2; solve in Round C with a workaround that retains the concept." (line 202)

### mock/design_rounds/202604280841/202604280841_topic.layout-soundness-gates.md
- date: 2026-04-28
- by: not stated
- holds: Topic adding compile-time `const _: () = assert!(...)` layout guards in four tiers (the `MetaCarrier` transmute, `Bits` projection cells per strategy, `UFixed`/`IFixed` wrapper sizes, `MultiContainer` layout) in two private `layout_assertions.rs` modules, with an `assert_layout_eq!` macro.
- headline: "The fix: compile-time `const _: () = assert!(...)` guards." (line 16)
- watch: "- Same for Warm at its supported cells (1, 8, 9, 16, 17, 32, 33, 64)." (line 30)
- watch: "- Same for Precise at its supported cells (1, 8, 9, 16, 17, 32, 33, 64)." (line 31)
- watch: "- **Strategy-axis HasAxes correctness check**: cross-checking that the four canonical markers' axis combinations match the doc table at compile time." (line 81)
- watch: "- User directive: \"we should in fact add more sanity checks and gates like this throughout, since the type himmeli is getting somewhat deep, and the moment we work around the rustc soundness checks itself, we really walk a tight line that needs these insurances end to end.\"" (line 90)

### mock/design_rounds/202604281000/202604281000_topic.substrate-completion-batch.md
- date: 2026-04-28
- by: not stated
- holds: Round D topic consolidating seven passes (A to G): soundness extensions, xxhash3 default and `ContentHash` widening, `Narrow` expansion and `arvo-narrow` population, bitfield `S` parameter, Precise at 33..=64 promoted to `u128`, strategy-axis-driven arithmetic dispatch, `Bits<const N: u16>` widening, and a cross-strategy resolution warning.
- headline: "This round consolidates seven sub-passes that all touch the post-bitwidth substrate." (line 6)
- watch: "Per user directive 2026-04-28 (\"we don't want to defer anything we can do now\"), the rounds collapse into one." (line 6)
- watch: "Why it matters: `Hot + Saturating` (the consumer wants Hot's wide container plus saturating overflow) currently has no path." (line 72)
- watch: "Layout assertions captured the inconsistency: Precise at 33..=64 uses u64 (size 8 bytes); Warm at 33..=64 uses u128 (size 16 bytes)." (line 62)
- watch: "The substrate cannot refuse the op (the toolbox-not-policer rule forbids that), but it can warn." (line 90)
- watch: "Per the toolbox-not-policer rule: do not switch the default to Cold." (line 58)
- watch: "The fix: `CrossStrategyOp<S1, S2>` marker trait with `#[diagnostic::on_unimplemented]` on the unresolved projection cells." (line 92)

### mock/design_rounds/202604281400/202604281400_topic.post-d-reviewer-followup.md
- date: 2026-04-28
- by: not stated
- holds: Round E topic listing four reviewer-found cleanups on PR #35: a stale `Bits<const N: u8>` claim in the arvo design doc, em-dashes in layout-assertion comments, an undeclared `arvo_full_layout_audit` cfg, and dead nightly-feature declarations in `arvo-narrow`. Records three decisions, including keeping the audit as a cfg.
- headline: "cleanup case becomes substantial). The only goal is to clear the four" (line 54)
- watch: "Post-Pass-B.4 the Narrow impls relocated to `arvo-bits-contracts` (orphan" (line 44)

### mock/design_rounds/202604281600/202604281600_topic.strategy-trilemma-deepdive.md
- date: 2026-04-28
- by: not stated
- holds: Doc-only topic promoting the research note `strategy-bound-trilemma.md` to a shipped `DEEPDIVE_strategy-bound-trilemma.md.tmpl` for arvo, with a pointer left on the research file and a forward-pointer to `CrossStrategyOp<S1, S2>`. Records three decisions.
- headline: "The trilemma has earned promotion: it explains a structural" (line 24)
- watch: "whose entire design rationale rests on the trilemma's \"trait-first, alias-first," (line 16)
- watch: "Pass G shipped `CrossStrategyOp<S1, S2>` with `#[diagnostic::on_unimplemented]`," (line 15)
- watch: "generics and the where-clause in `ufixed_bits(I, F)` form that is current" (line 91)

### mock/design_rounds/202604290000/202604290000_topic.strategy-name-debug-gate.md
- date: 2026-04-21
- by: not stated
- holds: Short topic deciding to gate the `Strategy::NAME` string constants behind `#[cfg(debug_assertions)]` on the trait item and its four impls (Hot, Warm, Cold, Precise), so release builds drop the strings, with `RANK: u8` staying as the release-path strategy identity.
- headline: "Wrap `NAME` in `#[cfg(debug_assertions)]` on the trait item and all four impls." (line 27)
- watch: "pre-hilavitkutin-str crates must gate every `const NAME: &str` / `static NAME: &str` behind `#[cfg(debug_assertions)]`." (line 19)
- watch: "Only `tests/cross_width.rs` uses `::NAME` (lines 20, 25, 30, 35, 40 — all test assertions). No release-path consumer; `RANK: u8` carries strategy identity for cross-width resolution." (line 23)
- watch: "Tests compile under `cfg(test)` which implies `debug_assertions`, so assertions still resolve." (line 27)

### mock/design_rounds/202604300000/202604300000_topic.arvo-bits-fix-and-bitfield.md
- date: 2026-04-21
- by: not stated
- holds: Topic fixing `Bits<N>` in `arvo-bits`, which always stored a `u64`, by dispatching its container through a new `BitContainerFor<N>` trait (smallest native primitive), and adding a declarative `bitfield!` macro for named sub-ranges. Records decisions D1 and D2 with compile-time checks.
- headline: "The decision is not \"rename the type to match reality\" — the" (line 44)
- watch: "**The originals have NO `Bits<N>` struct.**" (line 27)
- watch: "Introduce `BitContainerFor<const N: u8>` in arvo-bits mirroring" (line 68)
- watch: "container dispatch via `UContainerFor<S, BITS>`" (line 23)
- watch: "Fix `Bits<N>` in place. Do not rename." (line 116)
- watch: "- Separate unsigned/signed bitfields. Sub-ranges are unsigned bit" (line 217)

### mock/design_rounds/202604300826/202604300826_topic.refit-narrow-widen-completion.md
- date: 2026-04-30
- by: not stated
- holds: Topic completing the bit-width refit trait family: same-container `Narrow` impls, a new `Widen<T>` trait without a const-N parameter, direct primitive-to-`Bits` impls, `Bits::from_narrowed`/`from_widened` constructors, and renaming `arvo-narrow` to `arvo-refit`. Records decisions A to E and out-of-scope items.
- headline: "The Narrow side of the bit-width refit family shipped during the audit" (line 20)
- watch: "`arvo-strategy::UWidenFrom` / `IWidenFrom` are *strategy* widening" (line 34)
- watch: "- **Lossy widen variants.** All Widen impls preserve the source value" (line 248)
- watch: "- **Cross-strategy Narrow / Widen** (Hot → Warm logical widening with" (line 243)
- watch: "Hilavitkutin's `Bits<32, Hot>` → `Bits<28, Hot>` (both u32-backed)" (line 27)
- watch: "pub use arvo_bits_contracts::{Narrow, Narrowed, Widen, Widened};" (line 212)

### mock/design_rounds/202604301539/202604301539_topic.refit-polish-reviewer-findings.md
- date: 2026-04-30
- by: not stated
- holds: Topic answering reviewer findings on PR #40: retracts a doc claim that signed sub-carrier-width `Widen` sign-extends at bit M-1 and documents the contract as bit-pattern-preserving for both `Narrow` and `Widen`, plus three small doc and whitespace fixes.
- headline: "The right fix is to retract the sign-extension claim in the docs" (line 37)
- watch: "> Cross-Bits refit for signed sub-carrier-width `Bits<M, S, Signed>`" (line 65)
- watch: "The existing signed `Narrow` macro produces a destination carrier" (line 49)
- watch: "- **The `arvo-narrow → arvo-refit` rename.** Captured by the" (line 98)

### mock/design_rounds/202604301700/202604301700_topic.arvo-narrow-rename-to-arvo-refit.md
- date: 2026-04-30
- by: not stated
- holds: Topic renaming the `arvo-narrow` crate to `arvo-refit` (directory, `Cargo.toml`, workspace members, `mockspace.toml` forbidden-import entries, doc templates), keeping archived round references and leaving trait and method names unchanged.
- headline: "The crate name `arvo-narrow` undersells the surface and lies about" (line 20)
- watch: "(202604300826) selected this name explicitly: \"arvo-narrow gets" (line 23)
- watch: "renamed to something that contains both widen and narrow," (line 24)
- watch: "`arvo-narrow-contracts` (the L0.5 trait crate that merged into" (line 93)
- watch: "The arvo facade re-export already routes through `arvo-bits`" (line 107)

### mock/design_rounds/202604310000/202604310000_topic.bits-from-impls-and-bitfield-masks.md
- date: 2026-04-21
- by: not stated
- holds: Topic adding two `From` impls on `Bits<N, S>` (exact container and `u64` for `Hot`) and per-field `MASK` constants on `bitfield!`-generated structs, with the cascade that removes duplicated mask constants from `hilavitkutin-str`'s `Str`. Records D1 and D2.
- headline: "Ship both From impls." (line 63)
- watch: "`From::from` is a trait method; calling it inside `const fn` requires" (line 50)
- watch: "Generate only `MASK`. `SHIFT` and `WIDTH` constants are not currently" (line 95)
- watch: "- `ConstFrom` sealed trait — over-engineering for the ergonomic gain." (line 135)

### mock/design_rounds/202604450000/202604450000_topic.writing-style-sweep.md
- date: 2026-04-21
- by: not stated
- holds: Topic listing ten writing-style warnings (em-dash density, exclamations, a flat opening list) across arvo templates and doc comments, with a prose-only edit plan and validation by the lint.
- headline: "Prose-only edits. No structural / meaning changes to any document." (line 23)

### mock/design_rounds/202604460000/202604460000_topic.from-constant-usize.md
- date: 2026-04-21
- by: not stated
- holds: Topic changing `arvo::traits::FromConstant::from_constant` to take `USize` instead of `u8`, listing consumer sites in `arvo`, `arvo-graph`, `arvo-comb` and `arvo-spectral`, and dropping the tracked `lint:allow` in `fiedler.rs`.
- headline: "USize (wrap around host usize) is the correct semantic type: count /" (line 23)
- watch: "- Trait signature: `fn from_constant(n: USize) -> Self`." (line 43)
- watch: "expected to fit the concrete type's range; out-of-range truncates" (line 60)

### mock/design_rounds/202604470000/202604470000_topic.predicate-trait-family.md
- date: 2026-04-21
- by: not stated
- holds: Topic shipping a three-arity `Pred`/`Pred2`/`Pred3` trait family in `arvo` with blanket `Fn` bridge impls, to name the `Fn(...) -> Bool` bound used by two `arvo-comb` callback sites.
- headline: "Ship a three-arity predicate trait family in `arvo`:" (line 25)
- watch: "- No `dyn` support; the family is monomorphised just like everything" (line 72)
- watch: "Two consumer sites in arvo-comb take predicate-shaped callbacks:" (line 7)

### mock/design_rounds/202604480000/202604480000_topic.backlog-type-surface-reminder.md
- date: 2026-04-21
- by: not stated
- holds: Topic adding a compact type-surface reminder block to each arvo `BACKLOG.md.tmpl` (use `USize`/`Bool`/`UFixed` and `Maybe`/`Outcome` instead of bare primitives, `Option`, `Result`), with a parallel round planned in hilavitkutin.
- headline: "Prepend a single compact reminder block to every BACKLOG.md.tmpl:" (line 17)
- watch: "> **Type-surface reminder**: new entries use `arvo::USize` /" (line 20)

### mock/design_rounds/202604490000/202604490000_topic.arvo-docs-refresh.md
- date: 2026-04-21
- by: not stated
- holds: Doc-only topic enumerating six drift items between per-crate `DESIGN.md.tmpl`/`BACKLOG.md.tmpl` templates and shipped source (`Pred` family, `FromConstant` taking `USize`, `bitfield!` grammar, `cap_size` and `Matrix` homes in `arvo-tensor`, `BitSequence` on `Bits`).
- headline: "Recent impl rounds have shipped several changes that the per-crate" (line 7)
- watch: "- #123 — `FromConstant::from_constant` input changed from `u8` to" (line 13)
- watch: "- arvo-bits `BitSequence` impl on the `Bits` type is shipped; the" (line 21)

### mock/design_rounds/202604500000/202604500000_topic.bench-harness-adoption.md
- date: 2026-04-28
- by: not stated
- holds: Topic adopting the mockspace v2 bench harness in arvo with a `ContentHash` bench comparing `Fnv1a` and `XxHash3` at input sizes 64, 256, 1024 and 4096 bytes: workspace members, variant cdylibs, a const-generic input-size routine, `outputs_may_differ = true`, and the mockspace dependency bump from `main` to `dev`.
- headline: "Adopt the mockspace v2 bench harness in arvo." (line 3)
- watch: "Both implement `Hasher<N>` (file `algo.rs`) where `N` is the *output*" (line 40)
- watch: "`pub type ContentHash = Bits<64, Hot>` per arvo-hash's aliases." (line 136)

### mock/design_rounds/202605021200/202605021200_topic.expand-to-all-numerics.md
- date: 2026-05-02
- by: not stated
- holds: Addendum topic widening round 202605021200 from `USize` and `Cap` to every arvo numeric primitive: adds const constants and `impl const` operator impls, converts existing non-const `Add`/`Sub`/`Mul`/`Div` impls on `UFixed`, `IFixed`, floats, `Bits`, `Mask64`, `Mask256`, `BitMatrix`, and sweeps `.0` field accesses. Names the original topic as frozen and superseded in scope.
- headline: "The round now covers every arvo numeric primitive that has (or should have) `core::ops::*` impls." (line 5)
- watch: "**Driver:** User direction after the original topic landed: \"not only USize or Cap, all of the arvo prims that share those impls, which is, all the numerics.\"" (line 7)
- watch: "**`Bits<N, S, Sign>` (arvo-storage)**: bit-storage primitive." (line 40)
- watch: "5. Mixed-arity ops are absent: `USize + 1u32` does not compile, only `USize + USize::ONE` or `USize + USize(1)`." (line 77)
- watch: "**`UFixed<I, F, S>` (arvo)**: lines 153, 167, 181, 195 of `mock/crates/arvo/src/ufixed.rs` already carry `impl<...> Add/Sub/Mul/Div for UFixed<...>`." (line 34)

### mock/design_rounds/202605021200/202605021200_topic.usize-const-arith.md
- date: 2026-05-02
- by: not stated
- holds: Topic proposing inherent constants and `impl const` arithmetic on `USize` and `Cap` in `arvo-storage` so consumers stop reaching for `.0`, with open decisions on overflow semantics, mixed-type ops and a lint forbidding `.0` access on arvo primitives. Recommends matching primitive `usize` overflow.
- headline: "`arvo-storage::USize` is `pub struct USize(pub usize)` with `Copy`, `Eq`, `Deref<Target = usize>`, and `ConstParamTy`." (line 9)
- watch: "**Overflow semantics**: USize is platform-pointer-width; overflow at `usize::MAX` is the underlying primitive's contract." (line 47)
- watch: "**Recommendation**: option (a), match primitive. This is \"USize is the typed surface for `usize`; arithmetic semantics are `usize`'s.\" Document explicitly." (line 47)
- watch: "(c) saturating." (line 47)
- watch: "1. **Saturating-vs-wrap-vs-checked semantics** on USize arithmetic." (line 92)
- watch: "`UFixed`, `IFixed`, `Bits` carry their own arithmetic via L0.5 contracts (`arvo-numeric-contracts`) and are not affected by this round." (line 21)

### mock/design_rounds/202605021400/202605021400_topic.const-trait-machinery-lift.md
- date: 2026-05-02
- by: not stated
- holds: Topic folding the deferred Layer B of round 202605021200 back into one PR: lifts `UArith`/`IArith`, `BitPrim` family and `Bits` blanket impls to const traits, adds `UPrimZero`/`UPrimOne`/`IPrimNegOne` const traits at the `arvo-strategy` layer, and converts float and `Mask64`/`Mask256` ops to `impl const`. Lists five blockers with resolutions and a nine-commit order.
- headline: "The user's intent on PR #42 is a complete refactor: ship the entire const-trait surface in one PR, not two." (line 13)
- watch: "Bodies call `wrapping_add`, `saturating_add`, `wrapping_div`, etc. on bare primitives." (line 21)
- watch: "`f32 + f32` and `f64 + f64` are const-stable on nightly with `feature(const_ops)` (already enabled in arvo)." (line 35)
- watch: "- Cross-strategy const ops (Resolve already handles strategy mixing; const surface remains future work)." (line 60)
- watch: "Single synthetic blocker: arith.rs uses `<<Self as UContainerFor<$bits>>::T as Default>::default()` in the div-by-zero guard." (line 21)

### mock/design_rounds/202605021600/202605021600_topic.const-trait-bridges-and-mask-traits.md
- date: 2026-05-02
- by: not stated
- holds: Topic resolving four design smells left in PR #42: replaces hardcoded `u64::MAX` mask constants with const traits (`Bounded`, `Identity` in `arvo-strategy`, `Filled` in `arvo-mask-contracts`), adds `is_zero` to `BitPrim`/`IBitPrim`, makes the `Bits` blanket impls `impl const`, adds a `UBitContainer`-bridged inherent `ZERO`, and removes inherent `EMPTY`/`FULL` from `Mask64`/`Mask256`.
- headline: "This round is a hard requirement for PR #42 to merge." (line 109)
- watch: "The right shape is a const trait surface (`Bounded` for MIN/MAX, `Identity` for ZERO/ONE, or a mask-specific `MaskBounds` trait) with a blanket impl." (line 14)
- watch: "3. **The \"synthetic blocker\" framing was a cop-out.**" (line 16)
- watch: "This is the bridge pattern the reviewer named: \"introduce our own Eq, Ord etc. traits\" via a substrate-level surface that const-impls on the bare primitive." (line 18)
- watch: "- Const `core::cmp::PartialOrd` / `Ord` on USize and Cap — usize::partial_cmp is not stable as const; resolution belongs in a separate cmp-substrate round." (line 114)
- watch: "- Cross-strategy mask resolution." (line 115)

### mock/design_rounds/202605021800/202605021800_topic_expanded_full_p0_const_trait_dogfood.md
- date: 2026-05-02
- by: not stated
- holds: Round 1 expanded-scope topic folding every P0 finding from two audits (architectural dogfooding, const-trait completeness) into one round: const lifts of marker traits, `ConstEq`/`ConstOrd`/`ConstDefault` bridges, `MultiContainer` `BitPrim`/`Bounded`/`Identity` impls, `IBitPrim` parity, `Bits<N, S, Sign>` blanket generalisation, `impl const` for numeric-contracts and strategy bridges, float `Identity`. Lists deferrals to Rounds 2 to 5 and a closure criterion.
- headline: "This topic expands Round 1 to cover every P0-marked finding from both audits except the Bits&lt;const N: Width&gt; lift (which has its own coherent scope and biggest blast radius — earns Round 2)." (line 12)
- watch: "User redirect 2026-05-02: fold all P0 substrate work originally scoped across multiple rounds into Round 1 (\"no deferrals or kicking the bucket; all P0 in first steps; prefer early to late\")." (line 6)
- watch: "User feedback: \"all P0 in the first steps like they should... no deferrals or kicking the bucket. Meaning the Multicontainer BitPrim here, but others in similar grey area, prefer early to late.\"" (line 10)
- watch: "- **Decision 2** (substrate bridges): `ConstEq`, `ConstOrd`, `ConstDefault` in `arvo-strategy`, with per-primitive-wrapper coverage." (line 19)
- watch: "### Decision 6: IBitPrim gains parity with BitPrim" (line 45)
- watch: "### Decision 7: BitAccess / BitSequence / BitLogic blanket impls generalise to `Bits<N, S, Sign>`" (line 59)
- watch: "### Decision 10: UWidenFrom / UNarrowFrom / IWidenFrom / INarrowFrom → const trait" (line 94)

### mock/design_rounds/202605021800/202605021800_topic_substrate_const_trait_foundation.md
- date: 2026-05-02
- by: not stated
- holds: Narrower Round 1 topic: lifts sealed marker traits (`Signedness`, `BitsContainerFor`, axis traits, `CrossStrategyOp`, `Ieee`, marker traits, `AsBool`) to `pub const trait`, defines `ConstEq`, `ConstOrd`, `ConstDefault` bridges in `arvo-strategy`, collapses `UPrimConst`/`IPrimConst` into `Bounded` plus `Identity` plus a new `SignedIdentity`, and gives `USize`/`Cap` `Bounded` and `Identity`. Defers `MultiContainer`, `IBitPrim` parity and others to Rounds 2 to 6.
- headline: "This round covers ONLY foundational const-trait lifts and substrate-bridge introduction." (line 14)
- watch: "Stdlib `core::cmp::PartialEq` and `core::default::Default` are not const-stable." (line 34)
- watch: "Two parallel const-trait surfaces ship for the same axis: `<u8 as Bounded>::MAX` and `<u8 as UPrimConst>::MAX` are the same value reached two different ways." (line 69)
- watch: "- MultiContainer BitPrim impl (Round 2 / #311)" (line 94)
- watch: "- IBitPrim parity with BitPrim (Round 2 / #311)" (line 95)

### mock/design_rounds/202605030400/202605030400_topic_round_2_typed_width_and_ieee_relocation.md
- date: 2026-05-03T04:00Z
- by: not stated
- holds: Round 2 topic: relocates the `Ieee` const trait into `arvo-strategy` with `Ieee::WIDTH` typed as `Width`, replaces bare `as usize` casts in the `LOGICAL_WIDTH` bodies with typed `Width` arithmetic, and moves `Width`/`IBits`/`FBits`/`MetaCarrier` into `arvo-strategy`. States that the full `Bits<const N: u16>` to `Width` lift is left for Round 3.
- headline: "The round is mechanical and follows from prior decisions." (line 16)
- watch: "- Does not change the public API of `Bits<const N, S, Sign>`." (line 139)
- watch: "**Ieee lives in arvo-strategy.** Co-location with Identity / Bounded" (line 156)
- watch: "- Does not touch `arvo-mask-contracts::Mask<const W: u16>`." (line 146)
- watch: "The bridge-home rule (workspace .claude/rules/arvo-bridge-home-rule.md)" (line 184)

### mock/design_rounds/202605030400/202605030500_topic_round_2_scope_correction.md
- date: 2026-05-03T05:00Z
- by: not stated
- holds: Scope-correction topic dropping Deliverable C (relocating `Width`/`IBits`/`FBits`/`MetaCarrier` to `arvo-strategy`) from Round 2, keeping the `Ieee` relocation and the `LOGICAL_WIDTH` typed-arithmetic cleanup, with `Ieee::WIDTH` staying `u16` for the round. Lists which locked decisions of the original topic survive.
- headline: "**Deliverable C is removed from Round 2.**" (line 17)
- watch: "- `Ieee::WIDTH: u16` stays bare-primitive-typed for this round." (line 59)
- watch: "The `FromU8Ieee` companion trait travels with `Ieee` (same logical" (line 66)
- watch: "3. **Bits<const N: u16> stays for Round 2.** Decision 7 of the original" (line 95)
- watch: "The seal trait (`sealed::Sealed`) moves alongside the trait declaration." (line 62)

### mock/design_rounds/202605031400/202605031400_topic_foundational_redesign.md
- date: 2026-05-03T14:00Z
- by: not stated
- holds: Topic for task #316 recording a foundational container redesign: delete `MultiContainer`/`MultiContainerHalf`, introduce `WideBits<const BYTES>` above 128 logical bits, keep native primitives at N <= 128, a single cfg-aligned `Hot` marker, one `BitsContainerFor<const N: Width, Sign>` projection replacing `UContainerFor`/`IContainerFor`, and four per-strategy impls via `generic_const_exprs`. Lists sketches 01 to 06, thirteen doc templates affected and out-of-scope follow-ups.
- headline: "This round is the foundational restructure that sketches 01-06 unblocked." (line 8)
- watch: "**Single `Hot` type, alignment cfg-driven by target_feature.**" (line 40)
- watch: "Consumer writes `Hot` and receives the best alignment for the build target." (line 47)
- watch: "**N ≤ 128 specialises to native primitives (u8/u16/u32/u64/u128); N > 128 uses `WideBits<bytes_for(N)>`.**" (line 55)
- watch: "**Delete `UContainerFor<N>` and `IContainerFor<N>` entirely. `BitsContainerFor<const N: Width, Sign: Signedness>` is the single projection trait.**" (line 74)
- watch: "Per `no-legacy-shims-pre-1.0`, no deprecation wrappers, no re-exports." (line 76)
- watch: "- HotTruncate strategy variant for wide widths (lossy compute opt-in). → #322." (line 164)

### mock/design_rounds/202605031400/202605031430_topic_audit_corrections.md
- date: 2026-05-03T14:30Z
- by: not stated
- holds: Corrective topic for round #316 folding in a senior audit (verdict proceed-with-changes): locks byte 0 as least-significant for `WideBits` `leading_zeros`/`trailing_zeros`, narrows the sketch 01 finding, defends the 128-bit boundary as a rustc primitive-ladder boundary, downscopes `Hot` to align(16), adopts const-tag dispatch (Pattern C, sketch 07) for the native-or-wide projection, and relocates `Width` to `arvo-strategy`. Records zero consumer references to `MultiContainer`.
- headline: "The audit verdict is **PROCEED-WITH-CHANGES**: the architectural direction holds (byte-sequence above 128, native-below, MultiContainer deletion, single-impl projection), but specific premises are oversold and decisions need refinement." (line 8)
- watch: "**Decision: byte 0 is least-significant, mirror MultiContainer's lo-first convention.**" (line 28)
- watch: "**Decision: keep 128, with explicit framing as \"rustc primitive boundary\" — not \"fully native instruction\" boundary.**" (line 55)
- watch: "> The boundary at 128 captures rustc's native primitive ladder: u8/u16/u32/u64/u128 each have stable Rust support with stdlib ops that lower as well as the toolchain currently does for that width." (line 60)
- watch: "**Decision: `Hot` strategy uses align(16) baseline only. AVX-2 align(32) and AVX-512 align(64) are deferred to #320 with bench-driven evidence.**" (line 68)
- watch: "- The cfg-driven internal alignment with no consumer visibility is *exactly* the \"policy hardcoded on consumer's behalf\" pattern that `arvo-toolbox-not-policer.md` prohibits." (line 74)
- watch: "Likely shape: opt-in `HotAvx2` / `HotAvx512` strategy markers, **consumer-visible**, with documented storage cost tradeoff." (line 79)

### mock/design_rounds/202605031530/202605031530_topic_post_316_drift_cleanup.md
- date: 2026-05-03
- by: not stated
- holds: Mechanical topic cleaning prose drift left after round #316 deleted `UContainerFor`, `IContainerFor`, `MultiContainer`, `MultiContainerHalf` and `AlignedWideBits16`: DESIGN template edits, doc-comment rewrites in several `.rs` files, three em-dashes, and a note on the lint coverage gap.
- headline: "Mechanical cleanup. No design decisions. No new types or traits." (line 25)
- watch: "Also fix the false claim that `Warm` is \"not implemented for the `33..=64` bit range\" (it's `65..=128` that has no native Warm bucket; Warm covers `1..=64`)." (line 34)
- watch: "describe the actual `WideBits<BYTES, A: Align = A1>` parametric shape" (line 29)

### mock/design_rounds/202605031548/202605031548_topic_round_4_const_hash_and_narrowing.md
- date: 2026-05-03
- by: not stated
- holds: Round 4 topic expanding #314: a `ConstHash<N, S, Sign>` trait replacing per-N `hash_const` inherents on `Fnv1a`/`XxHash3`, a `NarrowFromU64<N, S, Sign>` const trait in `arvo-bits-contracts`, bounded-generic `Hasher<N>` impls replacing 128 macro-pasted impls, macro deduplication and sealing of `Project`, deletion of `HasherExt`, a `ConstFrom`/`ConstTryFrom`/`ConstDeref`/`ConstAsRef` bridge family, an algorithm-crate `USize` sweep, and `mask_low_bits`. Records decisions A to D including required sketches.
- headline: "**Single consolidated round on `feat/const-hash-and-narrowing`.** All 9 sub-passes land under one timestamped round." (line 97)
- watch: "1. `hash_const` per-N inherent methods on `Fnv1a<N>` and `XxHash3<N>` were appended as the bench-shim workaround for the cross-crate `.hash()` failure. The user flagged this as discipline violation: trait identity must be focused, not bolted onto every per-N implementor as inherents." (line 12)
- watch: "**`NarrowFromU64<const N: u16, S: Strategy, Sign: Signedness>` lives in `arvo-bits-contracts`.** Returns the dispatched container `<S as BitsContainerFor<N, Sign>>::T`." (line 103)
- watch: "**`ConstHash<N, S, Sign>` mirrors NarrowFromU64.** Returns `Bits<N, S, Sign>`." (line 113)
- watch: "A consumer hashing into a Cold container (compact storage of pass fingerprints in clause-codegen) or signed bits (specialised use case) gets the typed result without an extra cast at the call site." (line 117)
- watch: "4. `Project` is unsealed and `Picker` is `pub`." (line 18)

### mock/design_rounds/202605031658/202605031658_topic_round_4_pr_review_fixes.md
- date: 2026-05-03
- by: not stated
- holds: Thin follow-up topic fixing senior review findings on PR #43: broken tests using the deleted `HasherExt` and per-N `hash_const`, a `debug_assert!` for `XxHash3` streaming truncation, an em-dash, stale `Width` doc references, and one canonical impl each for the `ConstFrom`, `ConstAsRef` and `ConstDeref` bridges (`ConstTryFrom` deferred).
- headline: "**Single thin follow-up round on `feat/const-hash-and-narrowing`**, mirroring the round 202605031530 drift-cleanup pattern that ran post-#316." (line 33)
- watch: "**Add `debug_assert!(self.pos + bytes.len() <= self.buffer.len(), ...)` to `update()`.** This is the correct shape for a contract violation: visible in debug, no perf cost in release." (line 37)
- watch: "2. **`XxHash3` streaming silent truncation**: `update()` drops bytes beyond the 256-byte buffer with no diagnostic." (line 17)
- watch: "5. **Bridge family ships with zero impls**: `ConstFrom`, `ConstTryFrom`, `ConstDeref`, `ConstAsRef` declared in round 4 with no implementor anywhere in the workspace." (line 23)
- watch: "- `ConstFrom<u16> for Width` (arvo-strategy/src/width.rs)" (line 45)

### mock/design_rounds/202605031748/202605031748_topic_round_3_mask_bits_generic.md
- date: 2026-05-03
- by: not stated
- holds: Round 3 topic collapsing `Mask256`, `BitMatrix64`/`BitMatrix256` and the `Mask64`/`Mask256` aliases onto the generic `Mask<W>` and `BitMatrix<W, N>` chassis, renaming the contracts trait to `MaskOps<const W: Width>`, adding `BitPrim::mask_low`, lifting `Ieee` to an `Identity + Bounded` supertrait with float `Identity` impls, and routing `Mask::empty` through `Identity::ZERO`. Records decisions A to I.
- headline: "**Core + dogfood sweep.** Findings F1, F7, F12 are the core; F9, F10, F11 land as dogfood callouts in the same round." (line 54)
- watch: "**Delete `Mask64` and `Mask256` aliases.** Per `no-legacy-shims-pre-1.0`, names that exist only as parallel handles for the canonical shape do not survive into shipping APIs." (line 64)
- watch: "The audit recommended keeping `pub type Mask64 = Mask<...>` aliases. The user's call here overrides the audit on a discretionary point; the audit's structural recommendation (the collapse) stands." (line 68)
- watch: "**`arvo_mask_contracts::Mask<const W: u16>` renames to `MaskOps<const W: Width>`.**" (line 72)
- watch: "**Add `fn mask_low(n: USize) -> Self` to `BitPrim` and `IBitPrim` with default impl bodies.**" (line 90)
- watch: "**Type-generic over W.** The existing `Mask<W: BitSequence + BitAccess + Copy + Default>` chassis stays." (line 58)
- watch: "2. Lift `Ieee` to `pub const trait Ieee: Identity + Bounded { ... }`." (line 115)

### mock/design_rounds/202605040438/202605040438_topic_round_3_doc_drift_cleanup.md
- date: 2026-05-04
- by: not stated
- holds: Tiny topic correcting two doc references to the deleted `Mask64`/`Mask256`/`BitMatrix64`/`BitMatrix256` types (in the `arvo-mask-contracts` README template and `arvo-bitmask/src/node.rs`) after PR #44, carrying no design decisions.
- headline: "This round carries **no design decisions**." (line 18)
- watch: "describes the trait as an \"abstract surface over `Mask64`, `Mask256`, `BitMatrix` and other concrete masks from `arvo-bitmask`\"." (line 10)
- watch: "- `MaskOps` concrete trait impls and `bitfield!` macro routing through `BitPrim::mask_low`. Deferred to Round 5 (#315) per round 202605031748 src CL final notes." (line 30)

### mock/design_rounds/202605040602/202605040602_topic_round_5_consttry_and_const_feature_gate.md
- date: 2026-05-04
- by: not stated
- holds: Round 5 topic 1: places `ConstTry`/`ConstFromResidual` const traits in notko (with a `Bool` impl in `arvo-storage`), mirroring core's two-trait `Try` shape, and gates notko's const-trait surface behind a default-on `const` cargo feature using dual impl blocks, leaving arvo ungated.
- headline: "This topic captures two interlocked decisions: where the substrate's const-callable Try / FromResidual bridge lives, and how the const-trait machinery in notko gets gated behind an opt-out cargo feature." (line 8)
- watch: "arvo unconditionally requires const traits (substrate identity per `arvo-always-optimal-internals.md`); the gate would be vacuous noise." (line 78)
- watch: "As of nightly 2026-05, `core::ops::Try` is not `pub const trait` and there is no `const_try` feature gate in flight." (line 14)
- watch: "Add cargo feature `const` to notko. Default features include `const`." (line 44)

### mock/design_rounds/202605040602/202605040602_topic_round_5_mechanical_cascades_and_smoke_tests.md
- date: 2026-05-04
- by: not stated
- holds: Round 5 topic 3: adds `ConstEq`/`ConstDefault` impls to `bitfield!` output, a blanket `MaskOps` impl on `Mask<W>`, routes the `bitfield!` slot masks through `BitPrim::mask_low`, and lists const-context smoke tests across several arvo crates plus notko.
- headline: "This topic covers the mechanical residue of Round 5: applications of bridges that already shipped (F38), the chassis MaskOps cascade deferred from Round 3, the bitfield macro routing through `BitPrim::mask_low`, and const-context smoke test coverage spanning the audit-specified gaps plus this round's new surface." (line 8)
- watch: "A single blanket impl covers every concrete `Mask<W>` instantiation. No per-W concrete impls (no `impl const MaskOps<{Width::W64}> for Mask<Bits<64, Hot, Unsigned>>` etc.) ship in Round 5." (line 44)
- watch: "`min_specialization` is NOT enabled for this round." (line 46)
- watch: "- F26: `Mask<Bits<W, Hot, Unsigned>>` const-context smoke. Replaces F26's literal Mask64 reference. Crates `arvo-bitmask/tests/mask_const_arith.rs`." (line 62)
- watch: "- F30: BitPrim / IBitPrim const-context. Crate `arvo-bits-contracts/tests/bitprim_const_access.rs`." (line 66)
- watch: "- F35: MetaCarrier::as_bits const-context. Crate `arvo-storage/tests/metacarrier_const_arith.rs`." (line 67)

### mock/design_rounds/202605040602/202605040602_topic_round_5_niche_maybe_family_and_binpack.md
- date: 2026-05-04
- by: not stated
- holds: Round 5 topic 2: adds a transparent niche-Maybe wrapper `Slot<T: NonZeroable>` to notko and a USize-shaped `NUSize` (with a +1/-1 shift) to arvo-storage, uses `Array<NUSize, N>` as the arvo-comb binpack return to remove the F14 sentinel ambiguity, and defers the NU8 to NU64 family and `NBits<N>`.
- headline: "This topic covers the substrate's transparent niche-Maybe primitives, plus the arvo-comb binpack return-type rework that motivated the family." (line 8)
- watch: "Round 5 ships only `NUSize`. The N-prefix family naming (`NUSize`, `NU8`, `NU16`, `NU32`, `NU64`) is established as a convention so future rounds can extend without bikeshedding, but the broader family adds source CL volume without unblocking Round 5 work." (line 71)
- watch: "Consumers iterate the array and call `slot.as_maybe()` or `slot.is_some()` to distinguish placed-vs-not." (line 79)
- watch: "`NUSize` lives next to `USize` in arvo-storage." (line 50)
- watch: "Expert A's audit Finding 14 named that `arvo-comb/src/binpack.rs` uses `USize(0)` as both \"first valid bin\" and \"didn't fit\" sentinel." (line 12)

### mock/design_rounds/202605041051/202605041051_topic_round_6_audit_smoke_tests.md
- date: 2026-05-04
- by: not stated
- holds: Round 6 topic 1: lists seven const-context smoke test files (audit findings F26 to F30, F35, F36) across arvo-bitmask, arvo-bits, arvo, arvo-bits-contracts, arvo-storage and arvo-strategy, each exercising existing const-callable surface in `const _: () = { ... };` blocks, with no source or API change.
- headline: "The tests are pure additions." (line 10)
- watch: "Exercises `Mask<Bits<W, Hot, Unsigned>>` for representative widths (8, 32, 64, 256) inside `const _: () = { ... };` blocks." (line 22)
- watch: "Both files use representative width tuples covering Hot / Warm / Cold strategies." (line 34)
- watch: "The semantic is \"math-imprecise LLVM intrinsics route through fast where allowed, strict otherwise\"; the test confirms both paths are const-callable." (line 40)
- watch: "Exercises the `BitPrim` / `IBitPrim` const trait surfaces directly on the bare-primitive container types (u8, u16, u32, u64, u128 for unsigned; signed counterparts for IBitPrim)." (line 46)
- watch: "Exercises `Resolve<Other>::Out` const-context access across the four strategy markers (Hot, Warm, Cold, Precise) and pairwise resolution outcomes." (line 58)

### mock/design_rounds/202605041128/202605041128_topic_ufixed_ifixed_bounded.md
- date: 2026-05-04
- by: not stated
- holds: Round 7 topic 1: adds const `Bounded` blanket impls to `UFixed` and `IFixed` that forward MIN/MAX from the inner `Bits`, and extends the Round 6 const tests with MIN/MAX assertions.
- headline: "This topic covers a single substrate-completeness gap surfaced by the Round 6 senior review on PR #48." (line 8)
- watch: "The substrate stays toolbox-not-policer: consumers that previously had to reach for `Bits` to project MIN/MAX through transparent unwrap can now project through the typed UFixed / IFixed surface directly." (line 10)
- watch: "Add `impl<const I: IBits, const F: FBits, S: Strategy> const Bounded for UFixed<I, F, S>` with the same single-predicate cycle-avoidance shape Identity uses (see ufixed.rs:60-67)." (line 24)
- watch: "- `mock/crates/arvo-storage/src/bits.rs:101` (Bits Bounded impl that gets forwarded)." (line 60)

### mock/design_rounds/202605041144/202605041144_topic_ufixed_ifixed_const_family_doc.md
- date: 2026-05-04
- by: not stated
- holds: Round 8 topic 1: a doc-only addition to `mock/crates/arvo/DESIGN.md.tmpl` of a subsection enumerating the const-callable trait family on `UFixed`/`IFixed` and the predicate-bundling shape, plus a BACKLOG entry about logical-width bounds.
- headline: "This topic adds a dedicated subsection to `mock/crates/arvo/DESIGN.md.tmpl` enumerating the canonical const-callable trait family that UFixed and IFixed impl via the predicate-bundling pattern." (line 8)
- watch: "Bounded MIN / MAX semantics inherited as container bounds (not logical bit-width bounds); whether logical-width bounds should ship as a separate trait is BACKLOG, not Round 8 scope." (line 28)
- watch: "today `<U16Warm as Bounded>::MAX` returns u32::MAX (the container bound) because the Warm 16-bit container is u32." (line 32)
- watch: "Identity (ZERO / ONE), Bounded (MIN / MAX), ConstPartialEq + ConstEq (value equality at const time), ConstBitEq (bit-pattern equality), ConstOrd (total ordering at const time), ConstDefault (typed default)." (line 24)

### mock/design_rounds/202605051400/202605051400_topic_doc_audit_2026_05_05.md
- date: 2026-05-05
- by: not stated
- holds: Doc-only round consolidating two 2026-05-05 documentation audits of arvo into one doc CL covering doc-vs-source drift, Tier 1 leakage (round ids, `## Sources` blocks), polka-dots references in rendered output, and spelling drift; lists eight decisions and names a later src CL for rustdoc and `Cargo.toml` metadata.
- headline: "This round consolidates the workspace-wide documentation audit findings for arvo into a single doc-only round." (line 8)
- watch: "The `#316` heterogeneous-N-ary-container round renamed the runtime container from `MultiContainer` to `WideBits` and changed `const N: Width` back to `const N: u16`, but the rename did not propagate into 9 `DESIGN.md.tmpl` files that still narrate the old shape." (line 10)
- watch: "Wording: \"Warm projects to native primitives up to 128 bits; widths 129..=256 require Hot or Precise\" replaces the contradictory passages." (line 37)
- watch: "Where the prose currently reads \"Per round 202604271346, every trait whose methods are sensibly callable in const context is `pub const trait`,\" it becomes \"Every trait whose methods are sensibly callable in const context is `pub const trait`.\"" (line 31)
- watch: "The `arvo` README plus body still advertises the deleted `Uint8`/`Int8`-family std-parallel aliases." (line 10)

### mock/design_rounds/202605081600/202605081600_topic_bitmask_widebits_drift.md
- date: 2026-05-08
- by: not stated
- holds: A one-paragraph correction of the arvo-bitmask DESIGN body, replacing the stale `MultiContainer` backing claim for `Mask<Bits<256, Hot, Unsigned>>` with the `AlignedWideBits16<32>` / `WideBits<32>` projections.
- headline: "This round corrects the single paragraph to the post-#316 reality." (line 14)
- watch: "`Mask<Bits<256, Hot, Unsigned>>` projects through" (line 20)
- watch: "`BitsContainerFor<256, Unsigned>` to `AlignedWideBits16<32>` (32 bytes," (line 21)
- watch: "16-byte alignment for the Hot SIMD baseline). The Warm/Cold/Precise" (line 22)

### mock/design_rounds/202605111110/202605111110_topic_debug_on_numerics.md
- date: 2026-05-11
- by: not stated
- holds: Decides to ship plain non-const `core::fmt::Debug` impls on arvo-storage `USize`, `Bool`, `Cap`, `Bits` and arvo `UFixed`, `IFixed`, `FastFloat`, `StrictFloat`, via projection to the underlying primitive; a const `ConstDebug` trait is named as out of scope.
- headline: "Consumer code that derives `Debug` on structs holding arvo primitives cannot compile, forcing localised `transmute_copy`-style workarounds at consumer call sites." (line 10)
- watch: "`ConstDebug` as a `pub const trait` is the principled long-term shape (write-into-fixed-buffer pattern: `const fn const_debug(&self, buf: &mut [u8; DEBUG_MAX_LEN]) -> USize;` with blanket `impl<T: ConstDebug> core::fmt::Debug for T`)." (line 31)
- watch: "- `arvo_storage::Bits<const N: u16, S: Strategy, Sign: Signedness>` (where `Self::Inner: Debug`)" (line 21)
- watch: "The hilavitkutin runtime megaround surfaced this gap when its engine id types (FiberId / PhaseId / TrunkId / UnitId) needed `Debug` for their containing structs to derive cleanly." (line 10)

### mock/design_rounds/202605111719/202605111719_topic.algorithm-genericity-correction.md
- date: 2026-05-11
- by: not stated (the header names "user pushback" as the origin of the correction)
- holds: A sister topic that corrects the framing of the prior graph-spectral topic: the arvo-graph, arvo-sparse and arvo-spectral algorithms become generic over `W: BitPrim` (and drop the baked-in `Hot` and `Unsigned` literals) instead of monomorphising to `Bits<64, Hot, Unsigned>`, with a default-W alias, a generic `ContentHash<const N: u16 = 64>`, and a stated list of what is left untouched.
- headline: "That framing accepts the bad-design smell as a substrate property." (line 12)
- watch: "The algorithms in arvo-graph, arvo-sparse, and arvo-spectral arbitrarily monomorphise to `BitMatrix<Bits<64, Hot, Unsigned>, N>` and `Mask<Bits<64, Hot, Unsigned>>` in their public signatures." (line 14)
- watch: "`Hot` is the consumer-facing strategy default; baking it into the algorithm signature forecloses the consumer choice that arvo-toolbox-not-policer says belongs to the consumer." (line 16)
- watch: "The `N <= 64` runtime assertions in `arvo-spectral::fiedler_vector` and `arvo-spectral::partition` go away." (line 43)
- watch: "Resolution: drop `Hot` from every adjacency / mask signature; let the consumer's W carry the strategy." (line 77)
- watch: "`arvo-hash::ContentHash = Bits<64, Hot>` is the same smell at a different layer." (line 81)
- watch: "- The `BitPrim`-allowlist set of containers. Theory-forced by the LLVM lowering target." (line 99)

### mock/design_rounds/202605111719/202605111719_topic.graph-spectral-for-hilavitkutin.md
- date: 2026-05-11
- by: not stated
- holds: Round 202605111719 topic: extends arvo-sparse and arvo-spectral so hilavitkutin's plan-stage stubs (`rcm_reorder`, `block_diagonalise`, `spectral_partition`) can call real algorithms, via a sealed `SparseAdjacency` trait, CSR impls, wider array-shaped returns, a `LinearOperator` trait with a sparse Laplacian, problem-shaped traits with named-algorithm defaults, and two bench bundles.
- headline: "This round closes the cross-repo gap by extending arvo-sparse + arvo-spectral so hilavitkutin can call real algorithms with its own data shape." (line 9)
- watch: "1. **Capacity cap at 64 nodes.** Every arvo-sparse algorithm takes `BitMatrix<Bits<64, Hot, Unsigned>, N>` and returns `Mask<Bits<64, Hot, Unsigned>>`. The canonical 64-bit container caps usable N at 64. hilavitkutin's `MAX_UNITS` runs to thousands per realistic schedule." (line 17)
- watch: "This is a breaking change to two function signatures. arvo is pre-1.0, so per workspace `no-legacy-shims-pre-1.0` we drop the old mask-returning shape outright; no deprecation alias." (line 41)
- watch: "This adds one trait layer per algorithm-family. It is not policer-shaped: the trait is open, the default is documented, no consumer choice is foreclosed. Aligns with `arvo-toolbox-not-policer`." (line 82)
- watch: "Decision: trait-shaped. Generic over `T: SparseAdjacency<N>` with `N: Cap`. Both `BitMatrix<Bits<64, Hot, Unsigned>, N>` and a new `Csr<...>` impl land in this round." (line 31)
- watch: "Per the user's framing nudge: ship problem-shaped trait or free-fn names with the named algorithm as the canonical first impl." (line 69)
- watch: "- Wider adjacency containers beyond 64-node BitMatrix. The existing `BitMatrix<W, N>` chassis is generic over `W`; instantiating with `Bits<256, Hot, Unsigned>` extends to 256 nodes without algorithm change." (line 120)

### mock/design_rounds/202605111719/202605111741_topic.arvo-arbitrary-limitations-audit.md
- date: 2026-05-11
- by: not stated
- holds: A catalog-only audit round listing, in five categories, signatures, aliases, runtime gates and assertions across arvo-graph, arvo-sparse, arvo-spectral, arvo-hash and arvo-strategy that name `Bits<64, Hot, Unsigned>` or an `N <= 64` bound, with a table of follow-up rounds and a provisional lint shape; it makes no source change.
- headline: "The principle this round formalises: **the substrate ships generic surfaces and named defaults, never hardcoded literals masquerading as substrate properties.**" (line 12)
- watch: "Consumers choose width, strategy, and sign per workload, exactly as `arvo-toolbox-not-policer` says." (line 12)
- watch: "`arvo-hash::ContentHash = Bits<64, Hot>` is one specific instance." (line 57)
- watch: "Convert every `assert!(n <= some_bound)` in arvo to a `where { const_assert!(N <= some_bound) }` or `where [(); some_const_expr]:` form." (line 65)
- watch: "The principle: an algorithm body that uses `BitAccess` / `BitLogic` / `BitSequence` trait methods is strategy-agnostic and should not name `Hot` literally; an algorithm body that calls `core::arch::*` intrinsics is theory-forced to whatever `Hot` lowers to on the target and should document the dependency." (line 74)
- watch: "- `arvo-spectral/src/fiedler.rs:72` — `assert!(n <= 64, ...)`. Disappears with W-generic returns." (line 68)
- watch: "A `grep -rn \"Bits<64, Hot, Unsigned>\" mock/crates/*/src/*.rs` returns 36 hits across signatures." (line 33)

### mock/design_rounds/202605112039/202605112039_topic.sparse-shame-removal.md
- date: 2026-05-11
- by: not stated
- holds: A doc-only follow-up round that removes the `## CsrBidirectional` entry from `mock/crates/arvo-sparse/SHAME.md.tmpl` after the type landed in round 202605111719 (PR #63); no source change.
- headline: "Remove the `## CsrBidirectional` heading and its body paragraph from" (line 24)
- watch: "Round 202605111719 introduced `CsrBidirectional<ROWS, NNZ, W>` along" (line 8)
- watch: "with the rest of the `SparseAdjacency` / `BidirectionalSparseAdjacency`" (line 9)

### mock/design_rounds/202605112200/202605112200_topic.cap-usize-const-from-bridges.md
- date: 2026-05-11
- by: not stated
- holds: Round 202605112200 topic: adds `impl const From<Cap> for USize` and `impl const From<USize> for Cap` in arvo-storage (and lifts the `Bool` and `bool` `From` impls to const), keeps `usize` edges, the `.0` lint and the `pub` inner fields out of scope.
- headline: "The idiomatic Rust path is `From<Cap> for USize` so `let n: USize = N.into()` works in returns, expression positions, and trait-bound calls." (line 14)
- watch: "Decision: `impl const From<Cap> for USize` and `impl const From<USize> for Cap`. Const-callable at call sites including the new `.into()` form. Per `arvo-always-optimal-internals` and `arvo-compile-time-last`, runtime cost is zero (repr-transparent layout, monomorphisation collapses to a copy)." (line 47)
- watch: "Decision: just `Cap ↔ USize` this round." (line 37)
- watch: "Side effect: `From<bool> for Bool` and `From<Bool> for bool` at the same file should also lift to const for consistency." (line 49)
- watch: "- `arvo-tensor/src/cap.rs:17` — `cap_size` const fn (canonical Cap → usize projection)." (line 95)
- watch: "- Workspace rule `arvo-toolbox-not-policer.md` — substrate ships sharp tools, consumer chooses; this round adds one such tool (the `.into()` ergonomic path)." (line 97)

### mock/design_rounds/202605112230/202605112230_topic.from-bridges-review-followup.md
- date: 2026-05-11
- by: not stated (the header names a pr-reviewer-senior pass on PR #62 as its source)
- holds: A corrective round after the review of PR #62: rewrites a stale `From<bool> for Bool` paragraph in the arvo-storage `DESIGN.md.tmpl` and routes the `from_bridges.rs` test unwraps through `Transparent::raw` instead of `.0`; no new types or impls.
- headline: "This round addresses both." (line 14)
- watch: "`mock/crates/arvo-storage/DESIGN.md.tmpl` lines 330-332 still say \"Stdlib `From<bool> for Bool` and `From<Bool> for bool` impls remain non-const for boundary coverage. The parallel `ConstFrom` impls land in Round 4 (#314)\"." (line 10)
- watch: "- Route test-file unwraps through `<T as Transparent>::raw(...)` for arvo newtypes; the const-context assertions exercise the From trait method, not field access." (line 19)

### mock/design_rounds/202605250730/202605250730_topic.arvo-debug-ufixed-ifixed.md
- date: 2026-05-25
- by: not stated
- holds: Records that `Bits`, `USize`, `Cap`, `Bool`, `FastFloat` and `StrictFloat` already derive `Debug`, and proposes manual `core::fmt::Debug` impls on `UFixed` and `IFixed` delegating to the inner `Bits`, with one smoke test per type (task #428).
- headline: "Debug needs the same manual treatment." (line 19)
- watch: "- `Bits` at `arvo-storage/src/bits.rs`: `#[derive(... Debug ...)]` already shipped." (line 11)
- watch: "- `FastFloat`, `StrictFloat` at `arvo/src/float.rs`: `#[derive(... Debug)]` already shipped." (line 13)
- watch: "delegating to the inner `Bits` value's Debug impl with a wrapper that names the type: `f.debug_tuple(\"UFixed\").field(&self.0).finish()`." (line 25)

### mock/design_rounds/202605290259/202605290259_topic.tensor-cap-ctor.md
- date: 2026-05-29
- by: not stated
- holds: Proposes adding `pub const fn cap(n: usize) -> Cap` to `arvo-tensor/src/cap.rs` as the inverse of `cap_size`, so a `Cap` can be formed in const-generic position from a `usize`, for the hilavitkutin Phase C adapter.
- headline: "Add `pub const fn cap(n: usize) -> Cap { Cap(USize(n)) }` to `arvo-tensor/src/cap.rs`," (line 26)
- watch: "`arvo-tensor` ships `cap_size(Cap) -> usize`, the canonical projection used in" (line 8)
- watch: "justification as `cap_size`'s `usize` return), carried under a `lint:allow` with" (line 29)

### mock/design_rounds/202605291419/202605291419_topic.bool-try-residual-bound.md
- date: 2026-05-29
- by: not stated
- holds: Adapts `Bool`'s `core::ops::Try` impl in arvo-storage to core's new `Residual<Self::Output>` bound by replacing `Infallible` with an owned uninhabited `BoolResidual`, bundled with the nightly-2026-05-28 toolchain pin and notko's `try_trait_v2` feature.
- headline: "Adapt `Bool`'s core `Try` to an owned residual; keep `?` ergonomics." (line 12)
- watch: "`Bool`'s `Try` is infallible: `branch` always returns `Continue`, the residual is the uninhabited case." (line 8)
- watch: "- Add `pub enum BoolResidual {}` (uninhabited) to `arvo-storage` platform module, re-exported at the crate root alongside `Bool` (the public `Try` impl exposes it, so it must be `pub` and reachable; a private residual on a public `Try` impl is E0446)." (line 14)
- watch: "- `Bool`'s `notko::ConstTry` / `ConstFromResidual` impls are UNCHANGED: notko's `ConstTry` trait carries the unbounded `type Residual;`, so `Residual = Infallible` stays valid on the const path." (line 18)

### mock/design_rounds/202605291438/202605291438_topic.bool-residual-doc-and-notko-feature.md
- date: 2026-05-29
- by: not stated
- holds: Follow-on to round 202605291419: fixes the arvo-storage `DESIGN.md.tmpl` line that claims `Bool` implements `Try` with `Residual = Infallible`, and enables notko's `try_trait_v2` feature in `mock/Cargo.toml`.
- headline: "Now that arvo compiles on `nightly-2026-05-28`, the two deferred pieces land here." (line 6)
- watch: "`mock/Cargo.toml` notko dep `features = [\"const\"]` becomes `[\"const\", \"try_trait_v2\"]`, so notko's `?`-on-`Maybe`/`Outcome`/`Just` impls compile in arvo's build." (line 11)
- watch: "- No source-type changes beyond round 202605291419; this round is doc + build-config only." (line 16)

### mock/design_rounds/202605291759/202605291759_topic.scope-gce-gates.md
- date: 2026-05-29
- by: not stated
- holds: Records a remove-and-check test of `#![feature(generic_const_exprs)]` across twelve arvo crate roots: four are removed as dead, eight keep the gate with a justification comment, and the `cap_size` definition in `arvo-tensor` gets a soundness note (task #626).
- headline: "Four crates compile clean without the gate, including doctests: `arvo-bits`," (line 34)
- watch: "`arvo-bits-contracts`, `arvo-hash`, `arvo-storage`. Their gates are dead and get" (line 35)
- watch: "`arvo-sparse`, `arvo-strategy`, `arvo-tensor`. These keep the gate and gain a" (line 43)
- watch: "`arvo-bits-contracts` DESIGN doc already states its predicate-per-" (line 36)
- watch: "`arvo`, `arvo-bitmask`, `arvo-comb`, `arvo-graph`, `arvo-spectral`," (line 42)
- watch: "that call the single `cap_size` const fn in `arvo-tensor/src/cap.rs`. `arvo`" (line 47)

### mock/design_rounds/202605291942/202605291942_topic.csr-live-counts.md
- date: 2026-05-29
- by: not stated
- holds: Proposes giving arvo-sparse `Csr` and `CsrBidirectional` runtime `live_rows` and `live_nnz` fields so a capacity-with-slack adjacency (hilavitkutin's `DependencyGraph`) drives the `rcm`, `block` and `dm` algorithms without the slack tail being read as edges; the packed case stays the default (task #634).
- headline: "The fix belongs in arvo, not in the consumer." (line 13)
- watch: "This round parameterises the existing `Csr` / `CsrBidirectional` over the live counts rather than introducing a separate type." (line 15)
- watch: "`Csr` and `CsrBidirectional` gain two runtime fields, `live_rows: USize` and `live_nnz: USize`." (line 26)
- watch: "arvo-sparse's own DESIGN states the intent: \"any consumer that ships its own adjacency representation implements the trait once and reuses every algorithm.\"" (line 47)

### mock/design_rounds/202605300223/202605300223_topic.spectral-live-node-count.md
- date: 2026-05-30
- by: not stated
- holds: Follow-on to the loose-CSR round: makes arvo-spectral's `power_iteration`, `fiedler_vector`, `k_way_partition` and `spectral_bisection` iterate over a live node count obtained from a new `live_dim()` default method on `LinearOperator`, overridden by `SparseLaplacian`, instead of `cap_size(N)`.
- headline: "The fix puts the knowledge where the data lives." (line 21)
- watch: "`LinearOperator<F, N>` gains a default method `fn live_dim(&self) -> USize { cap_size(N) }`." (line 23)
- watch: "arvo-spectral's iterative algorithms (`power_iteration`, `fiedler_vector`, `k_way_partition`, `spectral_bisection`) operate over the full const-generic dimension `cap_size(N)`." (line 8)
- watch: "Encoding the live count on the operator follows the pattern arvo uses elsewhere, the type that holds the data exposes the derived property." (line 25)

### mock/design_rounds/202605301321/202605301321_topic.capacity-as-type-foundation.md
- date: 2026-05-30
- by: not stated (the header cites a design talk decision T13 "locked with op")
- holds: Round topic for arvo-tensor shipping a non-generic `Capacity` trait (with a GAT `Array<T>`) and a `Dim<const DIM: usize>` marker, so a fixed capacity is a type and no const expression sits in type position; existing containers and the engine are left for later rounds (tasks #650, #651, #652).
- headline: "A capacity becomes a TYPE." (line 15)
- watch: "The decision (T13, locked with op, reviewed by a code-architect) is to stop putting a const generic in capacity position at all." (line 15)
- watch: "This is the same escape arvo already uses for bit widths: `BitsContainerFor` dispatches `(strategy, width, sign)` to a container TYPE rather than computing a length." (line 15)
- watch: "The implementing TYPE is generic over `const DIM: usize`; the `Capacity` TRAIT stays non-generic, so consumers bind `C: Capacity` with no const parameter and keep the type-dispatch property." (line 50)
- watch: "The existing `Array<T, const N: Cap>`, `Matrix<W, N>`, `Csr`, and `BitMatrix` are untouched." (line 64)
- watch: "arvo-tensor. The trait's return types are `arvo::Cap` (defined in arvo-storage, L0) plus plain arrays (no dependency), so the bridge-home rule would permit a home as low as arvo-storage." (line 68)

### mock/design_rounds/202605301415/202605301415_topic.capacity-container-migration.md
- date: 2026-05-30
- by: not stated
- holds: Round topic (task #651) migrating the fixed-capacity containers and algorithms in arvo-tensor, arvo-bitmask, arvo-sparse, arvo-graph, arvo-comb and arvo-spectral from `const N: Cap` to `C: Capacity`, as one atomic round, with `Capacity::from_fn` added to the foundation and a likely drop of the `generic_const_exprs` gate per crate.
- headline: "This round uses the foundation." (line 11)
- watch: "Two cross-cutting traits couple the containers and their algorithm crates into one connected component." (line 17)
- watch: "The no-legacy-shims rule forbids a transitional parallel definition (an old `const N: Cap` shape coexisting with the new `C: Capacity` shape), so the migration cannot be sliced into independently-green rounds." (line 19)
- watch: "The `pub const trait` qualifier on `SparseAdjacency` and `BidirectionalSparseAdjacency` survives the parameter-kind change, because their impls are plain (non-const) impls carrying ordinary runtime bodies; the `const` declaration is orthogonal to the parameter kind and stays as-is." (line 37)
- watch: "Each algorithm crate that has no other GCE use can drop its `#![feature(generic_const_exprs)]` gate." (line 41)
- watch: "The `BitMatrix<W, const N: Cap>` per-row bit width `W` (`Bits<64>` today, the column cap that bounds adjacency at 64 nodes) is a separate widening concern." (line 45)

### mock/design_rounds/202605301615/202605301615_topic.bitmask-rustdoc-capacity-fixup.md
- date: 2026-05-30
- by: not stated
- holds: A comment-only fixup round for three arvo-bitmask rustdoc surfaces (`lib.rs`, `dirty.rs`, `node.rs`) that still describe `BitMatrix<W, const N: Cap>` after the capacity migration changed it to `BitMatrix<W, C: Capacity>`.
- headline: "This round corrects the three comments to the migrated `BitMatrix<W, C>` / `BitMatrix<W, C: Capacity>` spelling." (line 13)
- watch: "The capacity-container migration (round 202605301415, PR #74) changed `BitMatrix<W, const N: Cap>` to `BitMatrix<W, C: Capacity>`." (line 7)

### mock/design_rounds/202606051246/202606051246_topic.graph-algos-generic-width.md
- date: 2026-06-05
- by: not stated
- holds: Proposes parameterising the arvo-graph DAG algorithms (`waist_detect`, `components`, `topo_sort`, `upward_rank`, `longest_path`, spanning tree) over a row-word type parameter instead of the hardcoded `BitMatrix<Bits<64, Hot, Unsigned>, C>`, with bodies unchanged and a new wide-word test.
- headline: "This is the toolbox-not-policer shape: the library stops dictating a 64-node ceiling it has no reason to impose, and the consumer chooses the width its workload needs." (line 13)
- watch: "currently hardcode `BitMatrix<Bits<64, Hot, Unsigned>, C>`. Generalize them over the row-bearing word." (line 4)
- watch: "The cap is only in the algorithm signatures." (line 9)
- watch: "bounded `BitSequence + BitAccess + BitLogic + Copy + Default + Identity`, takes `BitMatrix<that, C>`, and returns `Mask<that>` where it returned a mask." (line 11)
- watch: "The `Mask<W>` chassis works uniformly across native-primitive backings (`Bits<8..=128>`) and the wide bucket (`Bits<129..=256>`) through the `BitPrim` impls on `WideBits`, per the `Mask` module doc." (line 7)

### mock/design_rounds/202606051310/202606051310_topic.graph-generic-width-rank-path-coverage.md
- date: 2026-06-05
- by: not stated
- holds: A test-only coverage extension on the branch of round 202606051246: wide-word assertions for `upward_rank`, `downward_rank`, `longest_path` and `spanning_tree` on a 129-node bowtie at `Bits<256, Hot, Unsigned>` / `Dim<129>`.
- headline: "This round closes that gap: it adds wide-word assertions for the four remaining algorithms on the 129-node bowtie at `Bits<256, Hot, Unsigned>` / `Dim<129>`, the same wide-bucket fixture the existing wide-word tests use." (line 7)
- watch: "Add tests to `tests/generic_width.rs` exercising `upward_rank`, `downward_rank`, `longest_path`, and `spanning_tree` at `Bits<256, Hot, Unsigned>` over the 129-node bowtie with unit weights." (line 11)

### mock/design_rounds/202606061200/202606061200_topic.bitmask-contract-trait-reexport.md
- date: 2026-06-06
- by: not stated
- holds: Proposes that arvo-bitmask re-export `BitSequence`, `BitAccess` and `BitLogic` from arvo-bits-contracts, so a consumer can name the row-word bound of `BitMatrix<W, C>` through L2; no behaviour change.
- headline: "arvo-bitmask re-exports `BitSequence`, `BitAccess`, and `BitLogic` from" (line 29)
- watch: "`BitSequence + BitAccess + BitLogic + Copy + Default`, where the three" (line 10)
- watch: "No behaviour change, no new code, no new trait. arvo-bitmask already imports" (line 35)
- watch: "The layering intent is that L3+ consumers reach the bit contracts through the" (line 41)

### mock/design_rounds/202606070756/202606070756_topic.const-capacity-surface.md
- date: 2026-06-07
- by: not stated
- holds: Proposes a sibling `ConstCapacity` const trait with a `Dim<N>` impl in arvo-tensor (const `filled`, `get`, `set`), additive to the existing `Capacity`, so a const fn generic over `C: [const] ConstCapacity` can build and index scratch arrays at compile time.
- headline: "A const-callable capacity surface in arvo-tensor: a new `ConstCapacity` trait," (line 8)
- watch: "The existing `Capacity` cannot become a `const trait`. Its `from_fn` method" (line 22)
- watch: "The `const_trait_impl` model requires that `impl const Trait` provide a" (line 25)
- watch: "Requires `#![feature(const_trait_impl)]` in arvo-tensor's lib.rs (WATCH-tier" (line 78)
- watch: "impl<const N: usize> const ConstCapacity for Dim<N> {" (line 59)

### mock/design_rounds/202606070807/202606070807_topic.const-waist-detect.md
- date: 2026-06-07
- by: not stated
- holds: Proposes `waist_detect_const` in arvo-graph, a const-fn analog of `waist_detect` built over the `ConstCapacity` and const `BitAccess` contracts, returning a `C::Array<Bool>` flag per topo position, with the runtime version left untouched.
- headline: "A const-callable waist detector in arvo-graph: `waist_detect_const`, the const" (line 8)
- watch: "`waist_detect` is left untouched (op directive: an additional const version, not" (line 12)
- watch: "over the `ConstCapacity` contract and the const `BitAccess` bit contract, so a" (line 10)
- watch: "ALLOWED; same gate R1a added to arvo-tensor. No `generic_const_exprs` over the" (line 59)

### mock/design_rounds/202606070816/202606070816_topic.const-waist-test-comment.md
- date: 2026-06-07
- by: not stated
- holds: A test-only round adding one clarifying comment in `tests/waist_const.rs` naming the fixture-scoped `j < 8` bound in `mk_row`, after a local review finding on PR #77.
- headline: "A one-line clarifying comment in `tests/waist_const.rs`." (line 8)

### mock/design_rounds/202606190038/202606190038_topic.arvo-const-cap-slice-accessor.md
- date: 2026-06-19
- by: not stated
- holds: Adds const `slice` and `slice_mut` methods to arvo-tensor's `ConstCapacity`, so a const fn can view the backing array as `&[T]` / `&mut [T]`; folded into the trait rather than a sibling, with the runtime `Capacity` side unchanged.
- headline: "Add two const methods to `ConstCapacity`:" (line 19)
- watch: "context. `Capacity`'s `AsRef` / `AsMut` are not const-callable. A const fn" (line 11)
- watch: "trait. Pre-1.0 churn permits adding the methods with no default." (line 30)
- watch: "The runtime `Capacity` side keeps `AsRef`/`AsMut` verbatim. No `Capacity`" (line 38)

### mock/design_rounds/202606231001/202606231001_topic.fixed-point-multiply-widening.md
- date: 2026-06-23
- by: not stated (a decision is attributed to op, 2026-06-23)
- holds: Proposes making `Mul` on `IFixed`/`UFixed` rescale for `F > 0` through a widen-multiply-shift-narrow `i_mul_fixed` / `u_mul_fixed` in arvo-strategy, changing `Identity::ONE` to the fixed-point one (raw `1 << F`) at the same time, native-widen now and WideBits-widen deferred.
- headline: "`Mul` for `IFixed`/`UFixed` becomes the genuine fixed-point multiply for `F > 0` (`F = 0` stays integer" (line 29)
- watch: "`UArith::u_mul`, which are the container's `wrapping_mul` (Hot/Warm/Cold) or `saturating_mul` (Precise): a" (line 10)
- watch: "## Decision (op, 2026-06-23): make bare `*` rescale for `F > 0`, with overflow-safe widening" (line 27)
- watch: "The multiply must be OVERFLOW-SAFE, not a non-widening `(a *raw b) >> F` (which overflows the container" (line 33)
- watch: "Cold widening table in a later round\"); op's call is that the first real need surfaces that impl now." (line 37)
- watch: "(`arvo-bits-contracts` `widen.rs` / `cross_domain.rs`). The strategy's overflow policy applies to the" (line 46)
- watch: "WIDE multiply (Precise saturates the wide product, Hot/Warm/Cold wrap the wide product), then the shift" (line 47)

### mock/design_rounds/202606231218/202606231218_topic.fixed-point-mul-review-fixes.md
- date: 2026-06-23 (the round id stamps it; the file states no date)
- by: not stated
- holds: A test-only round for `arvo/tests/fixed_point_mul.rs` after the PR #79 review: replaces a Precise saturation test that used an out-of-range operand, adds signed negative-operand guards, and adds `#[ignore]`d catalogue reds tracked by task #3.
- headline: "A senior PR review found a test-quality defect and coverage gaps." (line 10)
- watch: "- Replace `i_mul_fixed_precise_saturates_on_overflow`: it passed raw 200 to an 8-bit-logical Precise type" (line 16)
- watch: "- Add catalogue reds (`#[ignore = \"catalogue: ...; tracked #3\"]`) for the genuine gaps: Precise" (line 21)
- watch: "logical-range saturation on true overflow, the deferred WideBits widen for >64-bit-logical widths, and" (line 22)

### mock/design_rounds/202606231229/202606231229_topic.precise-saturates-at-container-not-logical-bound.md
- date: 2026-06-23
- by: not stated
- holds: An open standing topic documenting that every `Precise` arithmetic operation in arvo-strategy saturates at the container bound instead of the logical bound, with the reasons the logical bound is intended, a worked `i_mul_fixed` example, and a proposed fix that clamps to the logical range (task #3).
- headline: "Every `Precise` arithmetic operation saturates at the **container** bound, not the **logical** bound." (line 14)
- watch: "`Precise` silently fails to saturate exactly where saturation is the whole point of choosing `Precise`." (line 26)
- watch: "`arvo-strategy/src/arith.rs`, `impl_u_arith_saturating` and `impl_i_arith_saturating` implement every op" (line 19)
- watch: "4. It is what `Precise` users expect. Choosing the saturating strategy means clamp at the maximum the type" (line 42)
- watch: "DSP / audio / graphics saturation semantic." (line 44)
- watch: "Out-of-range container inputs (values that already exceed the logical range before the op) are garbage-in per" (line 87)

### mock/design_rounds/202606231430/202606231430_topic.fixed-point-divide.md
- date: 2026-06-23
- by: not stated
- holds: Proposes `i_div_fixed` / `u_div_fixed` in arvo-strategy so `IFixed::div` / `UFixed::div` rescale for `F > 0`, with per-strategy widening, truncate-toward-zero rounding, and a catalogue red for the 128-bit container case that needs 256/128 long division.
- headline: "`IFixed::div` / `UFixed::div` call `i_div` / `u_div`, which are plain integer `wrapping_div`." (line 9)
- watch: "Integer `/` truncates toward zero. The multiply floors toward minus infinity (an artifact of its arithmetic" (line 42)
- watch: "fewer remainder/correction step per arm). This leaves a documented asymmetry: `*` floors, `/` truncates." (line 44)
- watch: "Mirror the existing convention: wrapping strategies (Hot / Cold / Warm) return the numerator `a`; Precise" (line 37)
- watch: "numerator by a 128-bit divisor needs 256/128 long division. There is no `carrying_div` intrinsic (unlike" (line 30)
- watch: "- **Precise 1..=64 (DoubleLogical 2N):** same in-container shift+divide, then clamp the quotient to the" (line 27)

### mock/design_rounds/202606232230/202606232230_topic.euclid-and-even-split.md
- date: 2026-06-23
- by: not stated (the header cites an "op design-talk" as a source)
- holds: Proposes three small outer traits in arvo (`ScalarEuclid`, `EuclidDiv`, `EvenSplittable`) blanket-impl'd over `IFixed` / `UFixed`, bounded on a new inner per-width contract in arvo-strategy, so a consumer can split a fixed-point quantity into conserving shares without unwrapping the raw container.
- headline: "The arithmetic belongs in arvo." (line 13)
- watch: "## Design (op-confirmed): small separate traits + blanket impls over an inner per-width contract" (line 22)
- watch: "Inner contract (arvo-strategy, alongside `IArith`/`UArith`, macro-impl'd per strategy x width where the" (line 31)
- watch: "- `EuclidDiv`: `div_euclid(self, rhs: Self) -> Self`, `rem_euclid(self, rhs: Self) -> Self`. std-parity" (line 41)
- watch: "Both euclid variants ship so the consumer chooses (ULP-accurate vs whole-unit); rustdoc on each names the" (line 48)
- watch: "Pure arithmetic; deterministic. `#![no_std]`, no alloc: the split returns a lazy iterator (a small struct" (line 60)

### mock/design_rounds/202606232300/202606232300_topic.euclid-overflow-fixes.md
- date: 2026-06-23
- by: not stated (the header cites pr-reviewer-senior on PR #81)
- holds: Post-review fixes for two debug-panic overflows in the euclid/split round (`even_split_parts` forming `base + 1` eagerly, and `div_euclid_whole` shifting an out-of-range quotient), with a note on `split_evenly(USize(0))` and two catalogue tests.
- headline: "The euclid/split round shipped two overflow paths that panic in debug on legal inputs, missed because the" (line 10)
- watch: "1. `even_split_parts` forms `base + 1` eagerly even when the remainder is zero and `base_plus` is never" (line 14)
- watch: "2. `div_euclid_whole`'s `a.div_euclid(b) << FRAC` overflows when the `Self` divisor `b` is sub-unit" (line 19)
- watch: "`EuclidDiv` (a `Self` divisor can be a fraction). Fix: saturate the shift (`saturating_mul` by" (line 23)
- watch: "Width-8 (`IFixed<7,0,Hot>` / `UFixed<8,0,Hot>`) `split_evenly(USize(1))` on the container MAX conserves and" (line 33)

### mock/design_rounds/202607270015/202607262200_topic.release-blockers.md
- date: 2026-07-26
- by: not stated
- holds: A round covering the defects found reviewing the `dev` to `main` release: the floating toolchain pin, `Identity::ONE` being zero or off-by-one for purely fractional `UFixed`, a signed `i_mul_fixed` panic at the container minimum, a withdrawn bit-matrix report with a remaining axis-naming question, and honesty and hygiene items.
- headline: "A review of the `dev` to `main` release PR found three defects that are wrong in shipped code rather than" (line 7)
- watch: "`mock/rust-toolchain.toml` reads `channel = \"nightly\"`. Every command run inside `mock/`, which is the" (line 14)
- watch: "`UFixed<0, 8, Hot>::ONE` has raw `0`. `UFixed<0, 8, Precise>::ONE` has raw `255`. Neither is one, and the" (line 36)
- watch: "`<Hot as IArith<128>>::i_mul_fixed::<0>(i128::MIN, 1)` panics with \"attempt to negate with overflow\"." (line 89)
- watch: "Two reasons this is a blocker rather than an edge case. `Hot` and `Cold` are the strategies that promise" (line 95)
- watch: "not. `BitsBitPrim` declares \"leaves self unchanged for `idx >= WIDTH`\" and the implementation holds it:" (line 121)
- watch: "**Raw values escape the declared logical range** on `Hot` and `Cold`, because wrapping happens at the" (line 157)

### mock/design_rounds/202607270015/202607262330_topic.identity-per-operation.md
- date: 2026-07-26
- by: not stated
- holds: Records the move from `Identity::ONE`/`ZERO` to an `Identity<Op>` trait keyed on an operation (`Additive`, `Multiplicative`), the rejected earlier fixes, a sealed tag projection carrying representability, the `wrapping_neg` multiply fix, the toolchain pin fix, the withdrawn bit-matrix report, why the gates missed it, and the test and scope plan.
- headline: "An identity is always an identity **with respect to an operation**." (line 51)
- watch: "**401 tests passed throughout.** Every one of them named a width where the constant happened to come out" (line 34)
- watch: "`UFixed<0, F, S>` implements `Identity<Additive>` and no `Identity<Multiplicative>`. The absence is the" (line 63)
- watch: "`UArith<N>` and `IArith<N>` do not help either: they are implemented on **strategies**, keyed on container" (line 85)
- watch: "final doubling in `ufixed_fixed_one` carries off the top of the container: the wrapping strategies wrap it" (line 28)
- watch: "casts to `i128::MIN`, and negating that overflows. `Hot` and `Cold` are the strategies that promise not to" (line 126)
- watch: "Fixed with `wrapping_neg`, which is identical for every other input and yields the minimum itself there," (line 129)

### mock/design_rounds/202607280130/202607280100_topic.review-findings-on-the-identity-round.md
- date: 2026-07-28
- by: not stated (the header names an expert review of PR #2 on `orgrinrt/arvo`)
- holds: Records what a review of the closed identity round found: `SignedIdentity` implemented only on bare primitives despite four artifacts claiming otherwise, a wrong-sign `Precise::i_div_fixed` at the container minimum, sampling gaps in `never_panic_contract.rs`, `identity_laws.rs` and the compile-fail pin, two corrections to the locked changelist record, and a counter-increment conflation left unacted.
- headline: "`SignedIdentity` is implemented on bare primitives and on nothing else." (line 13)
- watch: "`Precise::i_div_fixed` returns the wrong sign at the container minimum." (line 40)
- watch: "The round replaced a panicking `/` with `wrapping_div` at every site uniformly, which" (line 45)
- watch: "The mistake was making `Precise` consistent with `Hot` rather than with itself. A saturating strategy has one" (line 49)
- watch: "`never_panic_contract.rs` exercises `Hot` and `Cold` at 128 bits only. That width routes exclusively through" (line 57)
- watch: "`identity_laws.rs` stops at 64 logical bits, described in a comment as keeping the matrix \"square\" across" (line 62)
- watch: "Counter increments across `arvo-comb`, `arvo-sparse` and `arvo-graph` read" (line 87)

### mock/design_rounds/202607290130/202607290100_topic.the-round-repeated-its-own-defect.md
- date: 2026-07-29
- by: not stated (the header names a second independent review of PR #2)
- holds: Records that the correction round shipped a locked changelist with three false claims against its source, a `Precise::i_div` / `i_div_fixed` zero-divisor sign defect, a wrongly documented div-by-zero contract in `arith.rs`, remaining sampling gaps in its own tests, a vacuous `warm_bound_negative.rs`, and em-dashes carried into new files by the `traits.rs` split.
- headline: "That is the same failure three times on one branch, and the third instance is the worst, because it was made" (line 12)
- watch: "`Precise::i_div` and `Precise::i_div_fixed` return the logical maximum on a zero divisor whatever the sign" (line 48)
- watch: "This is the previous round's own finding restated: a saturating strategy has one answer at an extreme and it" (line 52)
- watch: "`arith.rs` tells consumers \"Div-by-zero: wrapping strategies use `wrapping_div` (panic convention); Precise" (line 59)
- watch: "No probe in the law matrix is negative, so `x * ONE == x` is asserted on a negative operand nowhere, and the" (line 74)
- watch: "`warm_bound_negative.rs` is vacuous. It is meant to prove that `UFixed<I, F, Warm>` fails to compile past the" (line 84)

### mock/design_rounds/202607300800/202607270711_topic.the-lints-can-see-the-whole-crate-now.md
- date: 2026-07-27
- by: not stated
- holds: An owed-work topic recording that mockspace's lints now read every module file and report 128 findings across the arvo crates, sorted into layer violations, definition-site float findings and a few others, with a note not to follow the `no-bare-result` message literally and the reasons the work was filed rather than done.
- headline: "This topic exists to be in the way." (line 8)
- watch: "**128 findings**, from a run of the patched engine against this tree on 2026-07-27. By crate:" (line 27)
- watch: "forbids it. `arvo-tensor/src/array.rs` opens with `use arvo::USize;` while rule 10 says arvo-tensor" (line 51)
- watch: "`impl Sealed for f32 {}` and `impl_signed_identity!(-1.0, f32, f64)`, in the crate whose entire job is" (line 60)
- watch: "to define the IEEE float strategy. Naming `f32` there is the definition-site exception the workspace" (line 61)
- watch: "`arvo-refit` has no definitions at all and `no-empty-crate` says so." (line 72)
- watch: "`no-bare-result` currently says `use Result<T, LoimuError>`. There is no `LoimuError` in this" (line 79)

### mock/design_rounds/202607300800/202607281005_topic.arvo-shape-two-dimensional-primitives.md
- date: 2026-07-28
- by: not stated
- holds: Names an `arvo-shape` crate for two-dimensional primitives (points, extents, rectangles, affine transforms, curve primitives) declared over arvo's ops abstractions, lists three consumers, and puts layout in a separate `lato` crate; named but not scheduled.
- headline: "arvo has no two-dimensional geometry at all." (line 11)
- watch: "Because arvo is **already free of hardcoded scalars**, and that is the property the whole thing turns" (line 24)
- watch: "on. arvo works on integers as well as fixed point and carries its own float abstractions, and an" (line 25)
- watch: "integer is fixed point with `F = 0`. A shape declared over arvo's ops abstractions is therefore" (line 26)

### mock/design_rounds/202607300800/202607281052_topic.shapes-are-n-dimensional.md
- date: 2026-07-28
- by: not stated
- holds: Corrects the preceding two-dimensional shape topic: shapes are N-dimensional with rank as a parameter, and the rank-limited `arvo-tensor` types `Array` and `Matrix` are to port onto the general form; naming is left open and nothing is scheduled.
- headline: "Rank is therefore a parameter, in the same way width already is across arvo." (line 26)
- watch: "`arvo-tensor` is the rank-limited piece, and it says so plainly: \"fixed-shape tensor wrappers with" (line 31)
- watch: "exists, `arvo-tensor` ports onto it, and `Array` and `Matrix` become the rank-1 and rank-2" (line 36)
- watch: "`arvo-comb`, `arvo-spectral` and `arvo-graph` all build on const-generic structures over arvo" (line 39)

### mock/design_rounds/202607300800/202607281126_topic.what-arvo-ships-toward-n-dimensions.md
- date: 2026-07-28
- by: not stated
- holds: Internal research on what arvo already ships toward N dimensions: the `Capacity` / `Dim<N>` foundation and the `ConstCapacity` sibling, the rank-limited `arvo-tensor` and `arvo-bitmask` pairs, the square `Matrix` limitation, the consumers that would be re-read, what `arvo-tensor` excludes, and six open questions; no decisions.
- headline: "**So arvo can already express rank N.**" (line 45)
- watch: "> use site, but **the round-trip `cap_size(cap(N))` overflows the well-formedness evaluator and ICEs" (line 59)
- watch: "**This is the hard constraint on any N-dimensional design.** Whatever expresses rank must not" (line 65)
- watch: "Both already bind `C: Capacity` rather than a `Cap` const generic, so both already sit on the" (line 98)
- watch: "`Matrix<W, C>` is **square**: one capacity parameter serves both axes. A rank-2 shape with different" (line 108)
- watch: "Note that `arvo-spectral` re-exports `Matrix` from `arvo-tensor`, and `arvo-bitmask` and" (line 128)
- watch: "> - **ML-style broadcasting, reshaping, transpose chains, not aligned with arvo's role.** Consumers" (line 141)

### mock/design_rounds/202607300800/202607281127_topic.arvo-ndim-and-shape.md
- date: 2026-07-28
- by: not stated (it records op's proposed split verbatim)
- holds: Works through op's proposed `arvo-ndim` and `arvo-shape` split: what each crate would contain, loimu's prior alias table, what has to port across the arvo crates, the const-expression constraint, and seven open questions for a design talk; no decisions beyond op's steer.
- headline: "Everything below works through that steer." (line 20)
- watch: "> perhaps add an `arvo-ndim` crate along with shape, and reserve shape for things using the" (line 12)
- watch: "> `arvo-ndim` to express spatials (`Point<N: Dim>`, `Rect<X: Into<Point<2>>, Y: Into<Point<2>>>` or" (line 13)
- watch: "> They should work for theoretical math as well as real spatial concrete ariths." (line 17)
- watch: "`arvo-tensor` explicitly refuses as \"not aligned with arvo's role\". `arvo-ndim` naming rank makes" (line 66)
- watch: "is the case where arvo's own `Identity` could not represent one, since the raw encoding of one is" (line 86)
- watch: "The consolidation precedent is established and points one way: `cap_size` was duplicated in" (line 121)

<!-- END -->
