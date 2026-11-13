### mock/design_rounds/202607300800/202607281220_topic.the-ndim-and-shape-design.md
- date: 2026-07-28
- by: not stated (transcript of a design talk; decisions marked "Decision (op)")
- holds: TOPIC-phase transcript of the design talk for `arvo-ndim` and `arvo-shape`, carrying a taxonomy walk, a research addendum, and thirteen recorded decisions D1 to D13. It settles a new crate split (`arvo-capacity`, `arvo-shape`, `arvo-numeric`, `arvo-geom`, facade terminal), shape as a cons-list of capacities extracted to `notko-hlist`, rotors and motors for rotation, and curves as a bench matrix.
- headline: "**Decision (op).** Rank and per-axis extents are carried by a type-level cons-list whose elements are capacities." (line 356)
- watch: "The algebra pass derived a counterexample against shipped code: **signed `Saturating` addition is not associative** under two-sided clamping, while `Wrapping` addition is a full abelian group. Both are `OverflowPolicy` instances sealed at `arvo-strategy/src/axes.rs:38`." (line 150)
- watch: "arvo already parameterises by ZST marker, as `Hot`, `Warm`, `Cold` and `Precise`, so `Monoid<Add>` and `Monoid<Mul>` follow the pattern already present and need no wrapper types." (line 585)
- watch: "ARM's CMSIS-DSP ships saturating multiply-accumulate as a first-class instruction; Ethereum has no float instruction at all; Uniswap V3 picked `Q64.96` so `sqrt(price)` fits exactly." (line 110)
- watch: "`arvo-bitmask/Cargo.toml` lists `arvo-tensor` among its dependencies, because `Capacity`, `ConstCapacity`, `Dim`, `cap` and `cap_size` all live in `arvo-tensor` and `BitMatrix<W, C: Capacity>` binds them." (line 34)
- watch: "Direct3D's UNORM divides by `2^n - 1` rather than `2^n`, so all-ones lands on exactly 1.0, and the same specification separately defines ordinary `i.f` fixed point for subpixel positions." (line 222)
- watch: "Eight crates carry a Cargo dependency on `arvo` anyway: `arvo-bits`, `arvo-tensor`, `arvo-bitmask`, `arvo-hash`, `arvo-sparse`, `arvo-comb`, `arvo-graph`, `arvo-spectral`." (line 51)

### mock/design_rounds/202607300800/202607281547_topic.identity-already-exists-and-what-algebra-still-needs.md
- date: 2026-07-28
- by: not stated (transcript; the one decision is marked "Decision (op)")
- holds: A TOPIC file correcting D13 of the ndim/shape topic: `Identity<Op>` parameterised by an operation marker already ships in `arvo-strategy`, so the proposal restated an existing mechanism in a weaker form. It states the narrow remaining gap as a missing associative `Combine<Op>` sibling and places the algebra structures in `arvo-algebra-contracts` with `arvo-algebra` as the concrete sibling.
- headline: "**arvo has the identity half in full and does not name the other half.**" (line 64)
- watch: "`UFixed<0, F, S>` spans `[0, 1)`, which contains zero and does not contain one: it impls `Identity<Additive>` and no `Identity<Multiplicative>`." (line 46)
- watch: "The mechanism is a sealed witness. `tag_one_representable(int_bits)` maps the const space onto two tags, and `OneRepresentable<TAG>` is implemented only at tag 0." (line 49)
- watch: "exported from `arvo-strategy` as `Additive, Bounded, Identity, Multiplicative, OneRepresentable, SignedIdentity, tag_one_representable`." (line 31)
- watch: "Addition and multiplication arrive through `UArith`, `IArith`, `CrossStrategyOp` and core's operator traits; nothing pairs an associative combine with the `Identity<Op>` that belongs to it." (line 66)
- watch: "`Identity`, `Bounded` and `SignedIdentity` currently sit in `arvo-strategy`, which is the strategy-marker crate, and whether they stay there or move to the algebra crates is open." (line 91)

### mock/design_rounds/202607300800/202607282100_topic.remove-the-forbidden-feature-gates.md
- date: 2026-07-28
- by: not stated (transcript; decisions marked "Decision (op, 2026-07-28)" and "Decision (op)")
- holds: A TOPIC file recording that eleven live `generic_const_exprs` gates survived a soundness sweep on a justification naming a construct that no longer exists. It records four decisions: GCE is forbidden alongside full `specialization`, the container projection migrates to typestate with no feature gate, every remaining forbidden gate is removed, and the rule corpus itself gets a line-by-line audit.
- headline: "**Decision (op, 2026-07-28).** The standing gate, which predates this workspace and was settled during the loimu-era feature vetting, is that a feature is allowed only if it is **not proven unsound or unstable**, **and**, absent a very strong reason, is **itself on the stabilisation or mainlining path**." (line 65)
- watch: "**What the gate actually protects is the container projection**, in `arvo-strategy/src/container.rs`. Sixteen sites across four `BitsContainerFor` impls, all one shape: a const function call on a generic parameter, in const-generic argument position." (line 55)
- watch: "The sketch verifies resolution rather than parsing: `Hot` at 13 bits resolves to `u16`, at 47 bits to `u64`, signed at 7 bits to `i8`; `Warm` at 13 bits to `u32` and signed at 32 bits to `i64`, preserving the 2x-logical rule." (line 99)
- watch: "which names \"per-N const-trait impls (4 strategies x 64+ widths x 2 sign = hundreds of impls)\" among the things the substrate is allowed to spend when the alternative is a runtime container check." (line 105)
- watch: "The `arvo` facade's only live GCE constructs are two static asserts of the form `[(); 1 / is_fractional(F)]:`, at `ufixed.rs:274` and `ifixed.rs:308`." (line 113)
- watch: "op was told at each turn that the forbidden features were fine, and acted on that, while checking only the code and never the feature declarations in the lib roots." (line 129)
- watch: "`arvo-comb/src/lib.rs:16` records it: \"the capacity is a TYPE ... so no `cap_size` expression sits in type position and no `Cap` const generic appears.\"" (line 52)

### mock/design_rounds/202607300800/202607290050_topic.the-restructure-widens.md
- date: 2026-07-29
- by: not stated (records "Decision (op, 2026-07-28)")
- holds: A short TOPIC file carrying D14 alone, moved out of the frozen ndim topic. It withdraws the "unsatisfiable rule" argument for the restructure, keeps D1 and D2 on crate-purpose grounds, and widens the round to cover the four new crates, the fifty-one facade imports, the document disagreements the audit found, and the forbidden-gate migration.
- headline: "**Decision (op, 2026-07-28).** D1 and D2 stand, argued on crate purpose rather than on necessity." (line 14)
- watch: "The \"unsatisfiable rule\" argument recorded in the ndim topic is **incorrect and withdrawn**: `Cap` is declared at `arvo-storage/src/platform.rs:73`, not in the facade, so `arvo-tensor`'s rule was satisfiable as written." (line 14)
- watch: "Fifty of the fifty-one facade imports name a symbol already reachable from a permitted dependency, and none of the seven crates imports `UFixed` or `IFixed`." (line 17)
- watch: "`mock/DESIGN.md.tmpl` contradicts itself about where `Cap` lives (line 43 says `arvo-tensor`, line 86 says the facade, and it is `arvo-storage`)" (line 26)
- watch: "arvo has two predicate families that do not reference each other, and the rehoming question cannot be answered without deciding which of them is the predicate concept." (line 41)

### mock/design_rounds/202607300800/202607290100_topic.one-predicate-concept-implemented-twice.md
- date: 2026-07-29
- by: not stated ("The decision is op's and has not been made.")
- holds: A TOPIC file reporting that arvo carries two unrelated predicate families, a runtime `Pred`/`Pred2`/`Pred3` closure alias family in the facade and a type-level `Predicate` marker family in `arvo-numeric-contracts`, neither reading the other. It states the inversion between them, what the missing property costs in `arvo-comb`'s defensive branching, and lists four things the round must decide including where a unified concept lives.
- headline: "Family 1 can be called but can promise nothing. Family 2 can promise but is asked nothing. Neither is worth having alone." (line 99)
- watch: "**Family 2 lives in `arvo-numeric-contracts`.** `Predicate` is declared at `src/lib.rs:97` as `pub const trait Predicate { fn test(self) -> Bool; }`, with five marker subtraits (`IsZero`, `IsPositive`, `IsNonZero`, `IsNonNegative`, `IsZeroOrPositive`)" (line 23)
- watch: "Two callers, both in `arvo-comb`: `greedy.rs:37` and `dp.rs:38`, each taking `feasible: impl Pred2<..>`." (line 20)
- watch: "**A required property, stated as prose in a doc comment, with no way for a caller to say it holds and no way for the library to act on it.**" (line 87)
- watch: "**The constraint is the return type.** `Predicate::test(self) -> Bool`, and `Bool` is declared in `arvo-storage`, which depends on notko." (line 126)
- watch: "An asserted one (`MONOTONE = true`) is an unchecked promise whose falsehood produces a wrong answer rather than a slow one, which is closer to an `unsafe` contract than to a tuning knob." (line 115)
- watch: "the marker still gates which algorithms accept the wrapper" (line 36)

### mock/design_rounds/202607300800/202607290200_topic.the-predicate-decisions.md
- date: 2026-07-29
- by: not stated ("four decisions, all op's, recorded as made")
- holds: A TOPIC file carrying op's four answers (D15 to D18) on the two-predicate-families finding: the typestate predicate wins and the closure family is deleted, derived properties are safe consts while asserted ones are an `unsafe impl`, the concept lives in notko on a truth contract shaped like `Cardinal`, and the Warm/Precise 65-to-128-bit band is decided in the container-projection rewrite rather than patched.
- headline: "**Decision (op, 2026-07-29).** The question was posed as open between four shapes and it is not: the sketches settled it, and Family 1 is dead regardless." (line 12)
- watch: "An **asserted** property is a promise (`MONOTONE = true`) whose falsehood selects a different algorithm and returns a **wrong answer** rather than a slow one." (line 38)
- watch: "**Decision (op, 2026-07-29).** Refusing, as the diagnostic promises, is the honest reading of the two statements. It is not the action. When three artifacts disagree about a whole width band and no test covers it in either direction, the answer is to **revisit the architecture rather than pick a patch**." (line 61)
- watch: "notko declares the **contract for a truth value**, exactly as `Cardinal` is the contract a count type implements per D6, and arvo's `Bool` implements it." (line 47)
- watch: "`arvo/src/predicate.rs` is deleted with its two callers rewritten. `IsZeroOf` and its four siblings survive as the degenerate instance of the same concept" (line 27)
- watch: "`202607282100_topic.remove-the-forbidden-feature-gates.md` migrates `Project<TAG, Sign, BYTES, S>` to typestate, and the bucket table is what changes." (line 65)

### mock/design_rounds/202607300800/202607290500_topic.the-placement-calls.md
- date: 2026-07-29
- by: not stated ("four placements, all op's")
- holds: A TOPIC file writing down placement calls that had been settled in conversation but never recorded: the notko and notko-hlist split corrected, `Enumerator` to notko, the euclidean division family to the algebra crates, and restatements that `Pred` is the typestate predicate and that `arvo-algebra-contracts` and `arvo-algebra` are in the graph. It leaves `bitfield!` and the identity traits unplaced.
- headline: "**Three of these were settled in conversation and never written into a topic file.**" (line 12)
- watch: "`ScalarEuclid`, `EuclidDiv`, `EvenShares` and `EvenSplittable`, today in `arvo/src/traits/euclid.rs`, go to the algebra crates." (line 43)
- watch: "notko-hlist holds the type-level list and everything that folds over it; notko holds the properties and the predicate." (line 26)
- watch: "**`bitfield!`**, which expands to `Bits` accessors rather than to `UFixed`, so `arvo-bits` is at least as plausible as `arvo-numeric`." (line 77)
- watch: "**`Identity`, `Bounded` and `SignedIdentity`**, which sit in `arvo-strategy` today." (line 79)
- watch: "`arvo-algebra-contracts` holds the algebraic structures: `Semigroup<Op>`, `Monoid<Op>`, the ladder above them to whatever depth is wanted, `Combine<Op>` as the associative half that pairs with the existing `Identity<Op>`" (line 68)

### mock/design_rounds/202607300800/202607290600_topic.the-taxonomy-talk.md
- date: 2026-07-29
- by: not stated ("live transcript")
- holds: A TOPIC file opened as a live transcript for the taxonomy talk, after three calls were lost by being held in conversation. It restates the notko / notko-hlist correction and lists an eighteen-item agenda of unplaced symbols, unresolved crates and open design questions. Its "Calls made in this talk" section is empty.
- headline: "A topic file is the transcript of a design talk. Every call lands here in the turn it is made." (line 12)
- watch: "**`bitfield!`**. The only macro in the workspace. Expands to `Bits` accessors and never mentions `UFixed`, so `arvo-bits` is at least as plausible as `arvo-numeric`." (line 29)
- watch: "**`Identity`, `Bounded`, `SignedIdentity`**. In `arvo-strategy` today." (line 31)
- watch: "**The marker family** `IntegerLike`, `FractionLike`, `FloatLike`, `BoolLike`, `BitPresentation`, plus `logical_width_unsigned` and `logical_width_signed`. Unstated in D2." (line 34)
- watch: "**`arvo-refit`**. Has no definitions at all; `no-empty-crate` fires on it." (line 46)
- watch: "**`arvo-hash`**. Appears nowhere in D2's graph." (line 48)
- watch: "**The Warm and Precise band between 65 and 128 bits** (D18). Decided inside the container projection migration, which is what blocks the four-crate split." (line 54)

### mock/design_rounds/202607300800/202607291200_topic.identity-and-bounded-are-algebra.md
- date: 2026-07-29
- by: not stated; D23 is "Decision (op, 2026-07-29)" and D24 is "Decision (agent, on op's explicit delegation, 2026-07-29)"
- holds: A TOPIC file moving `Identity`, `SignedIdentity` and their operation markers out of `arvo-strategy` into `arvo-algebra-contracts`, and arguing that `Bounded` is algebra too because `MIN` and `MAX` are the identities of a bounded lattice. It records `Bounded` as a candidate for dissolving into `Identity<Min>` and `Identity<Max>`, held back by saturating arithmetic's separate reading of the same constants.
- headline: "**So `Bounded` is `Identity<Max>` and `Identity<Min>` wearing one name**, and the argument that produced `Identity<Op>` applies to it verbatim." (line 46)
- watch: "What stays in `arvo-strategy` is the strategy axis itself: `Hot`, `Warm`, `Cold`, `Precise`, `Strategy`, `Resolve` and the container projection." (line 18)
- watch: "`Bounded` has a second, non-algebraic reader: saturating arithmetic clamps to the representational limits of the container, and that is a fact about the representation rather than about a lattice." (line 51)
- watch: "The `OneRepresentable<TAG>` witness and `tag_one_representable` exist solely to remove `Identity<Multiplicative>` from a purely fractional type, so they move with it too." (line 16)
- watch: "Whether the saturating path should name `Identity<Min>` and `Identity<Max>`, or whether a separate representational-limits contract is owed, is a real question and is not settled here." (line 54)
- watch: "D24 is the agent's, made on op's explicit delegation, and is the most suspect thing in this file by the workspace's own provenance rule." (line 63)

### mock/design_rounds/202607300800/202607291210_topic.bitfield-becomes-its-own-crate.md
- date: 2026-07-29
- by: not stated ("one op call"; D25 marked "Decision (op, 2026-07-29)")
- holds: A short TOPIC file placing the `bitfield!` macro in its own crate `arvo-bitfield` rather than in `arvo-bits`, on the grounds that a macro crate may later become a proc-macro crate and a proc-macro crate cannot be an ordinary library. It notes the interaction with the no-alloc framing rule's proc-macro exemption.
- headline: "**Decision (op, 2026-07-29).** Its own crate, rather than following its expansion target into `arvo-bits` or staying beside the rest of what the facade declares." (line 9)
- watch: "As a declarative macro it would sit reasonably in `arvo-bits`, since it expands to `Bits` accessors and never mentions `UFixed`." (line 12)
- watch: "`no-alloc-no-std-framing.md` grants proc-macro crates a configurable exemption, since their source runs in the compiler host and does not ship with the consumer binary, and `mockspace.toml` declares `proc_macro_crates` for exactly that." (line 23)

### mock/design_rounds/202607300800/202607291300_topic.bounded-is-sugar-over-two-identities.md
- date: 2026-07-29
- by: not stated ("one op call"; D26 marked "Decision (op, 2026-07-29)")
- holds: A TOPIC file refining D24: `Bounded` becomes a supertrait-sugar over `Identity<Max>` and `Identity<Min>` rather than a trait carrying its own `MIN` and `MAX`, with the inverted mapping stated at the declaration. It leaves open whether saturating arithmetic's representational limits need a separate contract.
- headline: "**Decision (op, 2026-07-29).** D24 moved `Bounded` to the algebra crates and left \"does it dissolve entirely\" as a follow-up. op closed that: it is a convenience over the two identities, so it should say so in its own declaration rather than duplicate their constants." (line 9)
- watch: "`MIN` is the identity of **max**, because `max(x, MIN) == x`. So `MIN` is `Identity<Max>::IDENTITY`." (line 20)
- watch: "**saturating arithmetic clamps to the representational limits of the container**, and that is a fact about the storage rather than about a lattice." (line 38)
- watch: "If the saturating path reads `Identity<Max>` and `Identity<Min>`, it is naming a lattice to ask a question about a container, and it will read wrongly the first time a type's representable range and its lattice bounds differ." (line 42)
- watch: "`T::MIN` and `T::MAX` stay available as provided methods or associated consts on `Bounded` that read through to the two identities, so no call site changes." (line 32)

### mock/design_rounds/202607300800/202607291600_topic.d3-gate-inventory-is-incomplete.md
- date: 2026-07-29
- by: not stated ("Nothing here is the agent's call", no decision recorded)
- holds: A TOPIC file correcting D3's factual inventory: the facade's `generic_const_exprs` gate is load-bearing for six constructs, not two, because the four ergonomic type aliases in `aliases.rs` carry the same const-fn-in-const-argument shape. It leaves open whether the aliases are migrated or deleted.
- headline: "So the facade's gate is load-bearing for six constructs, not two, and a migration plan that budgets for the asserts alone stops at the first build." (line 34)
- watch: "the shape is refused without a gate and rustc names `generic_const_exprs`; `min_generic_const_args` refuses it and asks for a `const` block; the `const` block then asks for `generic_const_args`. Both successors are outside the allowed set." (line 29)
- watch: "pub type Fixed<const I: u16, const F: u16, S = Warm> = UFixed<{ ibits(I) }, { fbits(F) }, S>;" (line 22)
- watch: "The cure is the one already decided: the same move `Capacity` made, which is why `arvo-tensor` carries no GCE gate today." (line 39)
- watch: "Their whole purpose is letting a consumer write bare `u16` where the underlying types take the `IBits` and `FBits` newtypes." (line 47)

### mock/design_rounds/202607300800/202607291700_topic.storage-decomposes-and-refit-comes-home.md
- date: 2026-07-29
- by: not stated; D27 and D28 marked "Decision (op, 2026-07-29)", with the crate name flagged as the agent's derivation
- holds: A TOPIC file decomposing `arvo-storage` item by item: the comparison bridges to `arvo-numeric-contracts`, `ConstBitEq` to `arvo-bits-contracts`, the platform wrappers to a new `arvo-platform`, `MetaCarrier` to `arvo-strategy`, with the `Bits` remnant renamed `arvo-container`. It deletes `arvo-refit` and brings the refit family home to the container crate, leaving `ConstDefault` unplaced.
- headline: "**Decision (op, 2026-07-29).** `arvo-storage` accumulated three unrelated families under a name that describes only one of them." (line 10)
- watch: "`Narrow`, `Widen`, `Narrowed`, `Widened` and `NarrowFromU64` are strictly container things: masking to the low N bits, zero-extending, sign-extending, moving a value into a carrier of a different width." (line 43)
- watch: "**`arvo-refit` is deleted**, which also removes the `no-empty-crate` finding by removing its cause rather than silencing it." (line 48)
- watch: "That crate declares `TotalOrd`; `ConstPartialEq`, `ConstEq`, `ConstOrd` and `ConstOrdering` are the same question asked const-callably" (line 26)
- watch: "**`MetaCarrier` goes to `arvo-strategy`**, which already hosts `Width`." (line 38)
- watch: "**`ConstDefault` is not obviously anywhere.** It is neither comparison nor bits: it is a construction contract, the const-callable form of `Default`." (line 67)

### mock/design_rounds/202607300800/202607291710_topic.arvo-float-and-nan-as-typestate.md
- date: 2026-07-29
- by: not stated; D29 is "Decision (op, 2026-07-29)" and D30 is "Observation (op, 2026-07-29)"
- holds: A TOPIC file creating `arvo-float` to hold the float wrappers, the free float helpers and the float-specific carve-outs, on the ground that fixed point and IEEE floats are not the same kind of thing. D30 records op's reframing that NaN-absence is a property a type could carry, so the `ConstSign` carve-out is a workaround for a property never given a type.
- headline: "**So the carve-out is not a necessity, it is a workaround for a property that was never given a type.**" (line 40)
- watch: "An IEEE float is a hardware representation with its own rounding, its own infinities and its own NaN." (line 15)
- watch: "floats cannot implement `ConstOrd` at all, because NaN breaks reflexivity, and `ConstSign` exists precisely to give the ord-requiring predicates a path that does not go through `ConstOrd`." (line 17)
- watch: "a float known to be non-NaN satisfies reflexivity and can implement the ordering contract like anything else, and a float that might be NaN **fails to compile** at the point it is asked for an ordering" (line 34)
- watch: "The refusal becomes a trait-resolution error at the use site, which is the same mechanism `OneRepresentable` already uses to remove `Identity<Multiplicative>` from a purely fractional type." (line 36)
- watch: "It takes `FastFloat`, `StrictFloat`, the `Float` alias, `Ieee`, the free helpers `abs_f32`, `abs_f64`, `sqrt_f32`, `sqrt_f64`" (line 9)

### mock/design_rounds/202607300800/202607291720_topic.the-aliases-are-kept.md
- date: 2026-07-29
- by: not stated ("one op call"; D31 marked "Decision (op, 2026-07-29)")
- holds: A TOPIC file keeping the four ergonomic aliases `Fixed`, `Signed`, `Uint` and `Int`, overturning a proposal to delete them on use-count grounds, rehoming them to `arvo-numeric` and migrating them off `generic_const_exprs` under D2's cure. The constraint recorded is that `Uint<13>` must still read as `Uint<13>`.
- headline: "**Decision (op, 2026-07-29).** `Fixed`, `Signed`, `Uint` and `Int` are kept, rehomed to `arvo-numeric` as D2 already plans for the types they alias, and migrated off `generic_const_exprs` under the cure D2 settled." (line 9)
- watch: "**And the naming axis they express is one the stack will want as a bound.** `Signed` and `Uint` name the signedness axis; `Fixed` names the fixed-point form." (line 18)
- watch: "The computation is a **wrapping**, not arithmetic: `ibits` lifts a bare `u16` into the `IBits` newtype." (line 32)
- watch: "The cure is D2's, the same one `Capacity` already applied: nothing computes in type position." (line 35)

### mock/design_rounds/202607300800/202607291730_topic.the-markers-and-the-width-const-fns.md
- date: 2026-07-29
- by: not stated; D32 is "Decision (op, 2026-07-29)" and D33 is "Derived (agent), on op's instruction"
- holds: A TOPIC file splitting the marker family by subject matter (the number-kind markers to `arvo-numeric-contracts`, `BitPresentation` to `arvo-bits-contracts`) and placing the width const fns, with `width_le_64` alone going to `arvo-strategy`. It then gathers the round's remaining open calls into one section: `BitMatrix`'s axes, the PGA fork, the UNORM encoding, `Natural`, `ConstDefault` and D18's band value.
- headline: "**Decision (op, 2026-07-29).** `IntegerLike`, `FractionLike`, `FloatLike` and `BoolLike` go to **`arvo-numeric-contracts`**." (line 10)
- watch: "`width_le_64` is the one that goes elsewhere, and the reason is that it asks a question about *container dispatch* rather than about a fixed-point value: it is the strategy crate's business which widths fit which carrier." (line 33)
- watch: "the bivector.net convention against Eric Lengyel's rigid geometric algebra, differing on dimensional assignment, on what wedge and antiwedge mean, and on normalisation." (line 63)
- watch: "kirjo's colour pass found Direct3D divides by `2^n - 1` so all-ones lands exactly on 1.0, while the same specification separately defines ordinary `i.f` fixed point for subpixel positions." (line 69)
- watch: "**D18's band value.** What `Warm` and `Precise` resolve to between 65 and 128 bits, decided inside the container-projection migration rather than before it. This is the one that blocks the changelist." (line 85)
- watch: "`rows: C::Array<Mask<W>>` makes W the per-row bit width and C the row count, so a 64-bit row with 100 rows is 64 by 100 while the names suggest square." (line 53)

### mock/design_rounds/202607300800/202607291800_topic.the-last-four-calls.md
- date: 2026-07-29
- by: not stated; D34 to D37 marked "Decision (op, 2026-07-29)", with D34's consequence marked "Derived (agent)"
- holds: A TOPIC file closing the round's last four open calls: the Warm and Precise band above 64 bits resolved by making 2x-logical hold at every width, `BitMatrix` implementing the n-dimensional contracts rather than being renamed, UNORM showing arvo's identity does not name its encoding, and `ConstDefault` going to notko.
- headline: "The resolution the intent forces is that **Warm and Precise carry 2x-logical at every width**, so their wide bucket is sized from twice the logical width rather than from the logical width." (line 40)
- watch: "A strategy marker is the mapping of an intended mathematical concept onto the arithmetic a machine actually performs, so the answer comes from what a consumer, a mathematician among them, is entitled to expect from the marker they named." (line 15)
- watch: "The container **shrinks by 56 bits** as the logical width grows by one, and every width past 64 carries a marker whose meaning no longer holds. Nothing warns; the type still reads `Warm`." (line 34)
- watch: "Warm and Precise pass a second const fn (`(2 * N).div_ceil(8)`) where they today pass `bytes_for_u16(N)`; Hot and Cold are untouched, since byte-exact is what their markers promise." (line 44)
- watch: "it retires the `on_unimplemented` note's advice to \"choose Hot or Cold explicitly\", which was never reachable." (line 50)
- watch: "Divide by `2^F - 1` and all-ones lands exactly on 1.0, which is the Direct3D UNORM encoding, and the identity exists." (line 82)

### mock/design_rounds/202607300800/202607291900_topic.the-number-systems-crate.md
- date: 2026-07-29
- by: not stated; D38 and D39 are "Decision (op, 2026-07-29)", the literature grounding and the inhabits table marked as the agent's
- holds: A TOPIC file creating `arvo-num-systems` to carry number-system membership as generic bounds (ℕ through surreals and p-adics), defined through `arvo-algebra-contracts` rather than as opaque markers. It fixes the predicate as "inhabits" rather than "equals" and derives that arvo's fixed-point types inhabit the dyadic rationals ℤ[1/2], so no arvo type is `Real`.
- headline: "**So no arvo type is `Real`, and none is `Rational` in full.**" (line 105)
- watch: "Contracts without proofs would be marker traits a consumer could implement wrongly. The interesting part is the const assertions and typestate machinery that **statically prove membership holds**, so an impl exists only for types that genuinely satisfy it." (line 76)
- watch: "A `UFixed<I, F, S>` is an integer over 2^F, so every value it holds is a fraction with a power-of-two denominator. Those are the **dyadic rationals**, the ring **ℤ[1/2]**, a proper subset of ℚ." (line 91)
- watch: "`FastFloat` / `StrictFloat` | ℤ[1/2] plus specials | infinities and NaN are outside any of these sets" (line 103)
- watch: "The computing term for the same structure is the **numerical tower**, from Scheme and Common Lisp [...] ordering Number ⊃ Complex ⊃ Real ⊃ Rational ⊃ Integer." (line 33)
- watch: "The same marker that lets a non-NaN float implement the ordering contract is what lets it claim dyadic membership." (line 112)

### mock/design_rounds/202607300800/202607291910_topic.the-box-and-the-rotation-bench.md
- date: 2026-07-29
- by: not stated; D40 and D41 are op's, the general type name and the PGA correction marked as the agent's
- holds: A TOPIC file naming the n-dimensional box `Orthotope<N>` with `Rect` as the rank-2 alias, and instructing that the rotation representation be settled by a broad bench after its requirements are named. An appended same-day correction retracts the agent's claim that the PGA fork was drift, quoting Lengyel's own disagreement on all three axes.
- headline: "**Decision (op, 2026-07-29).** Bench it, in the sense D11 established, but not before naming what is actually being benched for." (line 48)
- watch: "The section above claimed the round's \"live unresolved split\" framing was unsupported drift from an unratified research memo. That claim is wrong, and the memo was right. Retracted in full." (line 114)
- watch: "The matrix wants the actual competing representations of rotation and rigid motion, which include at least rotation matrices, quaternions, axis-angle, Rodrigues parameters, the Cayley transform, dual quaternions for rigid motion, Lie-algebra exponential maps on so(3) and se(3), and the geometric-algebra rotors under both conventions." (line 82)
- watch: "fixed-point representability of each form, which operations must be cheap (compose, apply to a point, invert, interpolate, renormalise), how each degrades under repeated composition at finite precision, storage width" (line 87)
- watch: "Unratified research is presumed wrong where it conflicts with the canon. This memo conflicted with nothing in the canon: it reported a fact about external literature, checkable against that literature." (line 135)
- watch: "**`KCell` collides twice, and both times toward the wrong shape.** In algebraic topology an n-cell is \"homeomorphic to the open unit ball B^k\"" (line 21)

### mock/design_rounds/202607300800/202607291920_topic.hashing-distributes-two-ways.md
- date: 2026-07-29
- by: not stated; D42 is "Decision (op, 2026-07-29)", the crate name and sequencing marked as the agent's under delegation
- holds: A TOPIC file answering what becomes of `arvo-hash`: by machinery its finite-field and modular arithmetic contracts go to `arvo-algebra-contracts`, and by purpose hash, PRNG and noise become one family in a renamed `arvo-pseudorand`. The contracts split is deferred until there are three traits to hold.
- headline: "**Decision (op, 2026-07-29).** Distribute along both, and distribute correctly according to what the mathematical field itself establishes rather than according to what is convenient." (line 19)
- watch: "FNV-1a | XOR, then multiply by a prime where \"the multiply returns the lower n bits of the product\" | the ring ℤ/2ⁿℤ" (line 27)
- watch: "`arvo-rand` is the obvious shortening and is actively wrong. arvo is `no_std` with no entropy source, so every one of these functions is deterministic and reproducible" (line 54)
- watch: "A non-cryptographic hash like FNV-1a is a mixing function with good avalanche behaviour, not a proven pseudorandom function." (line 63)
- watch: "When the split lands, `Hash<N, S>` moves to the contracts side with `ConstHash`. Under `arvo-bridge-home-rule.md` the trait lives where its return type is reachable" (line 77)
- watch: "`Ring`, `Field` and `FiniteField` join the ladder there alongside `Semigroup` and `Monoid`; GF(2ⁿ) concrete arithmetic goes to `arvo-algebra` as its sibling." (line 33)

### mock/design_rounds/202607300800/202607292100_topic.rank-spans-both-domains.md
- date: 2026-07-29
- by: not stated; D43 and D45 are "Decision (op, 2026-07-29)", D44 a recording, the consequence table the agent's
- holds: A TOPIC file settling that rank and extent are declared once in `arvo-shape` and implemented by both the value and bit domains, with op correcting the word-packing framing of the bit projection. It records per-axis extents as already decided, and gives saturating arithmetic its own representational-limits contract in the container crate instead of borrowing the lattice.
- headline: "**So the bit domain is not a projection needing an exemption.** A bit shape is a shape whose element is one bit and whose extents are exact." (line 31)
- watch: "A consumer writes `Uint<7>` and means seven bits, rather than being pushed to 8 or 16 because that is the ladder every other framework offers." (line 19)
- watch: "Saturating arithmetic clamps to what the container can physically hold. That is a fact about storage. `Identity<Min>` and `Identity<Max>` are facts about a lattice." (line 77)
- watch: "`BitsContainerFor<N, Sign>` projects strategy, width and sign onto a native primitive or a wide byte sequence, every one of them `repr(transparent)`" (line 25)
- watch: "the contract is per-element at the boundary, and the implementation does whole-container AND, OR, XOR and popcount underneath." (line 50)
- watch: "**The mask contract is not absorbed.** `arvo-mask-contracts` keeps set, clear, test and count." (line 54)

### mock/design_rounds/202607300800/202607292200_topic.required-reading-lengyel-on-pga.md
- date: 2026-07-29
- by: not stated (quotes Eric Lengyel as the external source)
- holds: A TOPIC file pointing at Lengyel's "Projective Geometric Algebra Done Right" as required reading before D41's bench matrix is designed, quoting his three substantive disagreements with the dual construction model and the companion sites' grade/antigrade framing. It settles nothing about which convention arvo takes.
- headline: "Both errors came from working off summaries. The article is the primary source that settles it." (line 25)
- watch: "Their model requires that \"points be represented by trivectors and planes be represented by vectors. This is backwards.\"" (line 33)
- watch: "\"Their model requires that ∧ corresponds to a dimension-decreasing meet operation and ∨ corresponds to a dimension-increasing join operation, but in the Grassmann algebra used by the rest of the world, the meanings are the other way around.\"" (line 37)
- watch: "RGA \"completely subsumes conventional models that include homogeneous coordinates, Plücker coordinates, quaternions, and screw theory\"." (line 59)
- watch: "the disagreement is substantive, so the choice has consequences for which operations are cheap and how normalisation behaves under repeated composition" (line 67)

### mock/design_rounds/202607300800/202607292300_topic.the-axes-want-research-and-the-ladder-follows-theory.md
- date: 2026-07-29
- by: not stated ("All four are op's"; D46 to D49 marked "Decision (op, 2026-07-29)" and "Correction (op, 2026-07-29)")
- holds: A TOPIC file carrying four op calls: the conditional-law finding becomes a research task about whether arvo's strategy axes are split correctly, the algebraic ladder follows the mathematics rather than other languages' retreats with each rung sketched and benched, width stays a public const parameter lowering to typestate internally, and determinism is reframed as what the design already guarantees rather than an unnamed property.
- headline: "**What is in doubt is whether the axes are split correctly.**" (line 18)
- watch: "The finding that signed saturating addition is not associative while wrapping addition is a full abelian group may be reporting something about arvo's axis decomposition rather than only about saturation." (line 18)
- watch: "**Whether there are more axes.** The two `OverflowPolicy` instances and the four strategy markers may be an incomplete covering of what a consumer actually chooses between." (line 32)
- watch: "**Decision (op, 2026-07-29).** `Bits<13, Hot>` keeps reading as `Bits<13, Hot>`. The `Wid<N>` typestate the container-projection sketch introduced is an internal representation" (line 67)
- watch: "**Why fixed point is reached for here is accuracy together with demonstrable, proven determinism, which floats can never provide.**" (line 84)
- watch: "The behaviour is determined by the exact shape and type a consumer chooses, so determinism holds for everyone who chooses the same things." (line 88)

### mock/design_rounds/202607300800/202607300100_topic.warm-and-precise-above-64-bits.md
- date: 2026-07-30
- by: not stated ("described, not resolved")
- holds: A TOPIC file describing a conflict between the container projection, the `on_unimplemented` note and the design document over what `Warm` and `Precise` do between 65 and 128 bits. It lists three shapes the resolution could take, records that the one test naming the boundary had been asserting a name-resolution typo, and stops at description.
- headline: "Design is the oracle, so the projection is what drifted." (line 41)
- watch: "Warm / Precise cover 1..=64 via 2x-logical primitives (u16/u32/u64/u128); for 65..=128 they have no native bucket and must use Hot or Cold. Above 128 bits all four strategies dispatch through `WideBits<bytes_for(N), A>`." (line 32)
- watch: "Confirmed by compiling it rather than by reading it. `<Warm as BitsContainerFor<65, Unsigned>>::T` is `WideBits<9>`." (line 19)
- watch: "the answer may be a wide bucket with different alignment, or a warning rather than a refusal, per `arvo-toolbox-not-policer.md`: the substrate surfaces the consequence and the consumer decides." (line 58)
- watch: "The one test that named this boundary was `warm_bound_negative`, a compile-fail fixture asserting that `UFixed<33, 32, Warm>` does not resolve." (line 63)
- watch: "A compile-fail test proves only that the code fails to compile, and says nothing about why unless the snapshot is read." (line 73)

### mock/design_rounds/202607300800/202607300300_topic.the-formalization-proposal.md
- date: 2026-07-30
- by: "the agent's full proposal, put for op's agreement point by point"
- holds: A TOPIC file proposing that arvo's four numeric families become named compositions over one formalization, modelled on Flocq's `generic_format`, in five layers: number systems, algebra, format, landing policies (rounding, overflow, growth), composition with laws derived rather than asserted, and semantic names. It states three risks, including that it contradicts D29's reason for `arvo-float`.
- headline: "stop shipping four numeric families with a preset marker, and ship one formalization whose instances carry semantic names." (line 10)
- watch: "`Rounding` | the result is too precise | `Trunc`, `Floor`, `Ceil`, `NearestEven`, `NearestAway`, `Convergent`, `Stochastic`" (line 57)
- watch: "`Overflow` | the result is too large | `Wrap`, `Saturate`, `SaturateSym`, `SaturateZero`, `WrapSignMagnitude`, `Trap`" (line 58)
- watch: "`Rounding` is the axis arvo does not currently have at all. `Overflow` is the axis it has with two of the field's five values. `Growth` is the axis currently buried inside the strategy markers" (line 61)
- watch: "`Hot`, `Warm`, `Cold` and `Precise` are documented as **presets** over (growth, overflow, container) rather than as an axis, which is what the research showed they already are." (line 90)
- watch: "`Saturate`, signed, two-sided | monotone commutative unital magma on a bounded lattice | commutative, monotone, **not associative**" (line 74)
- watch: "SystemC hands a consumer `SC_SAT` and says nothing about associativity. This refuses the fold instead." (line 99)

### mock/design_rounds/202607300800/202607300400_topic.the-formats-named-and-the-taxonomy-revisited.md
- date: 2026-07-30
- by: not stated; D50 to D52 marked "Decision (op, 2026-07-30)", the underflow decomposition marked as the agent's proposal
- holds: A TOPIC file recording op's three calls on the formalization proposal (the format concept unifies while `arvo-float` survives as packaging, law markers are derived by blanket impls, compositions are public with semantic names as the default), naming the four Flocq formats from source, proposing an underflow axis, and listing every earlier call the proposal reopens.
- headline: "**Decision (op, 2026-07-30).** One `Format` concept parameterised by an exponent function, so the mathematics is stated once." (line 12)
- watch: "| rounding | `Trunc`, `Floor`, `Ceil`, `NearestEven`, `NearestAway`, `Convergent`, `Stochastic` | **absent** |" (line 90)
- watch: "| overflow | `Wrap`, `Saturate`, `SaturateSym`, `SaturateZero`, `WrapSignMagnitude`, `Trap` | exists with 2 of 6 |" (line 91)
- watch: "`Hot`, `Warm`, `Cold` and `Precise` become documented presets over (growth, overflow, container), which is what the research showed they already are." (line 95)
- watch: "`Flushed` is what x86 and ARM actually do with the FTZ and DAZ flags set, which games and DSP routinely enable, so a consumer can state it in the type rather than in a comment." (line 77)
- watch: "`FLT` | `max(e - prec, emin)` | \"floating-point format with gradual underflow\". IEEE 754's model: full precision to `emin`, then subnormals degrading" (line 51)
- watch: "`arvo-toolbox-not-policer.md` decides it independently: sealing the composition would be arvo deciding which combinations a consumer may want." (line 38)

### mock/design_rounds/202607300800/202607300700_topic.consolidated-round-state.md
- date: 2026-07-29 (stated "Date: 2026-07-29", branch `feat/arvo-shape-topic`; summarises files dated through 2026-07-30)
- by: not stated
- holds: A 2285-line consolidation of every topic file open in the restructure design round, in chronological order by filename timestamp, so a later reader gets the round's full state from one file. It restates each topic's findings and decisions D1 to D52 with provenance marked, and ends with the two sketches (format as an exponent function, law markers derived over the composition), both WORKS.
- headline: "This document consolidates every topic file currently open in this design round, in chronological order by filename timestamp, so a later reader gets the round's full current state from this file alone." (line 6)
- watch: "arvo has 2 values (its `OverflowPolicy`) against SystemC/Vitis's 5 (`AP_SAT`, `AP_SAT_ZERO`, `AP_SAT_SYM`, `AP_WRAP`, `AP_WRAP_SM`); three of five have no arvo equivalent." (line 1946)
- watch: "rounding/quantization is absent from arvo entirely, where SystemC has 7 modes (`SC_RND`, `SC_RND_ZERO`, `SC_RND_MIN_INF`, `SC_RND_INF`, `SC_RND_CONV`, `SC_TRN`, `SC_TRN_ZERO`) and MATLAB has 6 (Nearest, Ceiling, Convergent, Zero, Floor, Round); this means every arvo narrowing operation has an implicit, unnamed, unselectable rounding behaviour." (line 1942)
- watch: "quoting the SMT theory (\"addition is commutative under both methods, but only addition using the wrap-around method is associative\")" (line 1963)
- watch: "symmetric saturation (`SC_SAT_SYM`/`AP_SAT_SYM`) does NOT restore associativity, worked example `(127+1)+(-1) = 126` versus `127+(1-1) = 127`" (line 1968)
- watch: "`02_refusal.rs`, **fails to compile, which is the result.** Asking to fold a signed saturating type produces `error[E0277]: the trait bound `(Saturate, Signed): AddAssoc` is not satisfied`" (line 2264)
- watch: "`Strategy` itself is characterised as a preset bundling container-width policy, overflow policy and intermediate-precision policy rather than an axis" (line 1948)

### mock/design_rounds/202608082157/202607301000_topic.inherited-state-from-the-formalization-round.md
- date: 2026-07-29 (stated "Date: 2026-07-29", branch `feat/arvo-shape-topic`; filename stamps 202607301000)
- by: not stated
- holds: Byte-identical to `mock/design_rounds/202607300800/202607300700_topic.consolidated-round-state.md`: the 2285-line consolidation of every topic file in the restructure round, in filename-timestamp order, restating each topic's findings and decisions D1 to D52 with provenance marked, ending with the two sketches that came back WORKS.
- headline: "This document consolidates every topic file currently open in this design round, in chronological order by filename timestamp, so a later reader gets the round's full current state from this file alone." (line 6)
- watch: "rounding/quantization is absent from arvo entirely, where SystemC has 7 modes (`SC_RND`, `SC_RND_ZERO`, `SC_RND_MIN_INF`, `SC_RND_INF`, `SC_RND_CONV`, `SC_TRN`, `SC_TRN_ZERO`) and MATLAB has 6 (Nearest, Ceiling, Convergent, Zero, Floor, Round)" (line 1942)
- watch: "arvo has 2 values (its `OverflowPolicy`) against SystemC/Vitis's 5 (`AP_SAT`, `AP_SAT_ZERO`, `AP_SAT_SYM`, `AP_WRAP`, `AP_WRAP_SM`); three of five have no arvo equivalent." (line 1946)
- watch: "symmetric saturation (`SC_SAT_SYM`/`AP_SAT_SYM`) does NOT restore associativity, worked example `(127+1)+(-1) = 126` versus `127+(1-1) = 127`" (line 1968)
- watch: "`02_refusal.rs`, **fails to compile, which is the result.** Asking to fold a signed saturating type produces `error[E0277]: the trait bound `(Saturate, Signed): AddAssoc` is not satisfied`" (line 2264)
- watch: "arvo already parameterises by ZST marker (Hot, Warm, Cold, Precise), so Monoid<Add> and Monoid<Mul> follow the existing pattern with no wrapper types needed." (line 658)

### mock/design_rounds/202608082157/202607301100_topic.the-formalization-talk.md
- date: 2026-07-30
- by: not stated ("live transcript"; decisions D53 to D75 marked "Decision (op, 2026-07-30)", several sections marked as the agent's derivation)
- holds: The live transcript of the formalization round, 1847 lines, formalising how arvo works from source and then settling a new spine: one `Number<N: Numeral, S>` type with the axes split into identity (`Numeral`), policy (`Policy`) and lowering (`Lowering`) contracts, ten axes ratified, rounding and overflow unified into one `Quantisation` map over five situations, the four presets redefined from their intent, and one crate per contract. Many per-axis rows (A1, A2, B1, C2 to C4, D1 to D3, E1, E4, E5) are left open.
- headline: "**D69. The axis set is ratified at ten axes across three contracts, with `LogicalWidth` on `Numeral`.**" (line 1621)
- watch: "**D64. The policy axis is `Quantisation`, one axis over five situations.** Decision (op, 2026-07-30). Rounding and overflow are not two axes; they are the in-range and out-of-range halves of one map from an exact value onto the representable set" (line 1387)
- watch: "So one type today rounds toward negative infinity when multiplied and toward zero when divided. Both are defensible choices; having both, unnamed, in one type is not one." (line 1050)
- watch: "IEEE 754 defines five rounding-direction attributes: `roundTiesToEven`, `roundTiesToAway`, `roundTowardPositive`, `roundTowardNegative`, `roundTowardZero`. SystemC defines seven quantization modes: `SC_RND`, `SC_RND_ZERO`, `SC_RND_MIN_INF`, `SC_RND_INF`, `SC_RND_CONV`, `SC_TRN`, `SC_TRN_ZERO`." (line 1054)
- watch: "**D54. The axes split in two, on whether they say what the number is or how it is computed.** Decision (op, 2026-07-30). Op's statement of the principle: what the number *is* does not change through strategies." (line 334)
- watch: "So under D51 the fast float has an `AddAssoc` impl and the strict float does not, and the fold that silently reassociates is refused for `StrictFloat` by the same mechanism that refuses it for signed saturating fixed point." (line 304)
- watch: "**D67. arvo writes the abstraction; every established convention ships as an optional feature defining that convention's vocabulary as aliases over it.** Decision (op, 2026-07-30). [...] `conv-ieee754`, `conv-systemc`, `conv-matlab`, `conv-amd-vitis`, `conv-flocq`" (line 1459)

### mock/design_rounds/202608082157/202607301200_topic.the-formalization-spec.md
- date: 2026-07-30
- by: not stated ("a first draft by two people"); provenance section marks four items as the agent's derivations
- holds: A single-document statement of the shape the formalization talk arrived at, derived from the round's two prior files and deciding nothing: three contracts (`Numeral`, `Policy`, `Lowering`), ten axes, what is derived rather than declared, the four presets redefined, the `conv-*` alias sets, the crate split, and six sketch obligations that gate the source changelist. It is the artifact the expert panel receives.
- headline: "arvo's four numeric families become one type." (line 23)
- watch: "| in-range | truncate | nearest, ties to even | nearest, ties to even | nearest, ties to even |" (line 252)
- watch: "| out-of-range | reduce modulo | clamp | clamp | refuse |" (line 253)
- watch: "A recovery is *faithful* when it leaves the operation the operation of an algebraic structure on the representable set; modular reduction is, since wrapping addition is the group operation of `ℤ/2ⁿℤ`, and clamping, substituting and refusing are not." (line 204)
- watch: "Its instances are the three Flocq distinguishes: `Unbounded` (no floor, no subnormals, FLX), `Gradual` (subnormals, IEEE 754's choice, FLT) and `Flushed` (abrupt, flush to zero, FTZ)." (line 99)
- watch: "What does change for an existing consumer is behaviour, not spelling: the presets are redefined, so `Hot` stops truncating toward zero on divide and `Warm`, `Cold` and `Precise` stop truncating at all." (line 320)
- watch: "**if a convention's mode cannot be written as an alias, the abstraction is not general enough.**" (line 280)

### mock/design_rounds/202608082157/202608082156_topic.nuking-the-tree-for-the-canon.md
- date: 2026-08-08
- by: not stated; opens with an op quote dated 2026-08-08
- holds: A short TOPIC file recording op's instruction to delete every crate under `mock/crates` and empty the workspace `Cargo.toml`, framed as the mutation order rather than housekeeping. It names what goes (the crates and `mock/Cargo.lock`) and what stays (benches, research, design rounds, agent, lints, root templates), and records a mockspace panic when `mock/crates` does not exist.
- headline: "Let's nuke the entire mock/crates and make the Cargo.toml in mock/ be an empty workspace. This is what we should've done initially when we started working on the canon. Let's now do it for real" (line 7)
- watch: "mockspace panics rather than erroring when `mock/crates` does not exist: `can't read crates dir: NotFound` at `src/parse.rs:12`." (line 45)
- watch: "an agent that helpfully consults the current designs or the shipped source during canon work is not being diligent. It is reattaching the tier that had to be detached for the work to be permitted." (line 18)
- watch: "**Stays:** `mock/benches`, because its artifacts are committed evidence and several of the panel's doability claims cite them." (line 29)

### mock/design_rounds/202608082200/202608082200_topic.the-per-crate-designs-go-with-their-code.md
- date: 2026-08-08
- by: not stated
- holds: A short TOPIC file for round two of the tree nuke, deleting the fifty-two per-crate `*.md.tmpl` documents that round one could not take because design and source are gated to different phases. It keeps the three workspace-level templates and notes that git history and `origin/dev` hold everything removed.
- headline: "A per-crate design describing deleted code is exactly that. It is more dangerous than the code was, because it reads as intent." (line 18)
- watch: "\"Surviving code says the old design is still true; surviving design says the old canon is still true. Both then get read, both mislead, and worse, both get **defended**, because they are concrete and detailed and look authoritative next to an abstract statement.\"" (line 15)
- watch: "Fifty-two documents remain describing sixteen crates that no longer exist." (line 9)

### mock/design_rounds/202608090856/202608090855_topic.bench-workspace-exclude-regression.md
- date: 2026-08-09
- by: "dispatched expert (thread-axis bench, numeral canon panel file 78)"
- holds: A one-round infrastructure fix: the crate-deletion commit emptied `mock/Cargo.toml`'s member list and dropped the bench entries with it, so every crate under `mock/benches/` refuses to build. The fix is adding `exclude = ["benches"]` to the workspace table, and the file records that the harness had been broken since 2026-08-08 21:59:31.
- headline: "Add `exclude = [\"benches\"]` to `mock/Cargo.toml`'s `[workspace]` table." (line 44)
- watch: "error: current package believes it's in a workspace when it's not:" (line 27)
- watch: "it broke every bench in the directory, including the ones the panel had already run and cited (files `26` and `27`), from 2026-08-08 21:59:31 onward." (line 35)
- watch: "bench crates are not governed by the canon/design/code chain the empty member list exists to protect, and the deletion commit's own reasoning already says so" (line 48)

### mock/design_rounds/202608311902/202607270711_topic.the-lints-can-see-the-whole-crate-now.md
- date: 2026-07-27
- by: not stated
- holds: A topic filed deliberately without doing the work: mockspace's Rust lints used to read only each crate's `src/lib.rs`, and once a dispatcher runs every per-file lint over every source file the tree reports 128 findings. The file sorts them into layer violations, definition-site float findings, and a handful of others, and records why the triage was left to whoever opens arvo next.
- headline: "**128 findings**, from a run of the patched engine against this tree on 2026-07-27." (line 27)
- watch: "**Layer violations, and these are real.** Fifty-two `forbidden-imports` findings across seven crates," (line 49)
- watch: "`arvo-tensor/src/array.rs` opens with `use arvo::USize;` while rule 10 says arvo-tensor [...] depends on arvo-storage, arvo-bits-contracts and arvo-numeric-contracts and nothing else." (line 51)
- watch: "**Definition-site floats, and most of these are the lint being wrong.** Seventy-two `no-float` [...] findings: 53 in arvo, 15 in arvo-strategy, 4 in arvo-storage." (line 58)
- watch: "`impl Sealed for f32 {}` and `impl_signed_identity!(-1.0, f32, f64)`, in the crate whose entire job is [...] to define the IEEE float strategy." (line 61)
- watch: "**A handful of others.** `arvo-refit` has no definitions at all and `no-empty-crate` says so." (line 72)
- watch: "`no-bare-result` currently says `use Result<T, LoimuError>`. There is no `LoimuError` in this [...] workspace; the name belongs to a downstream consumer and leaked into the lint's text." (line 79)

### mock/design_rounds/202608311902/202607311100_topic.lint-pack-rename.md
- date: 2026-07-31
- by: not stated
- holds: A config-only topic: the shared lint pack renamed its crate to `mockspace-extra-lints` on the split lint-trait ABI, so the `[lint-crates]` entry in the root `mockspace.toml` is repointed. It also records the per-instance `--no-verify` blessing for the migration commit, since the repaired commit gate now sees arvo's pre-existing forbidden-imports debt.
- headline: "the rename is the whole change here: the `[lint-crates]` entry now names" (line 15)
- watch: "The engine's [...] builtin set simultaneously dropped `no-bare-vec`, `no-box` and `no-float` as [...] standalone registrations (`no_alloc` subsumes the first two, `no_bare_numeric` [...] the third)" (line 11)
- watch: "arvo's pre-existing debt (52 forbidden-imports errors, [...] arvo-tensor importing `arvo::{Cap, USize}` against the L2 layering rule)" (line 25)
- watch: "Op blessed a per-instance `--no-verify` for this migration" (line 28)

### mock/design_rounds/202608311902/202608311729_topic.the-base-primitive-design-is-blocked-on-the-container-premise.md
- date: 2026-08-31
- by: not stated
- holds: A topic recording that a round opened on the base numeric primitives was refused at the canon gate, because the container premise is open, stands at one expert, and no wording of the design's first sentence is true on both of its branches. It carries committed probes whose two compile refusals show the branches differ in arity, in what a consumer can express at one declared width, and in what a homogeneous container can hold, and it names the three questions that would unblock the round along with the inherited topics it does not shed.
- headline: "**Status:** refused at the canon gate, filed so the next round has to deal with it" (line 7)
- watch: "`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` fixes the rounding [...] vocabulary at six names. `ruling::the_overflow_panic_is_permitted_and_bounded` fixes the panic to [...] debug builds and puts a declared mode in its place on release." (line 24)
- watch: "whether the strategy is an upstream selector the ladder never sees or a key of the ladder itself." (line 79)
- watch: "Whatever makes sense next and is natural and gives most answers that settle downstream questions at [...] the same time and enable/unblock further design and exploration work." (line 146)
- watch: "`does_warm_wrap_or_clamp` is answerable now from three ratified rows without waiting on anything," (line 163)
- watch: "`where_wrapping_lives` is answered by the ratified factoring, since wrapping and saturation are both [...] total maps onto the representable set and therefore both members of the one slot." (line 166)
- watch: "`dimension::overflow_policy` lists [...] `panic` among the axis values, and under the ratified factoring an adaptation is total while panic [...] diverges" (line 173)

<!-- END -->
