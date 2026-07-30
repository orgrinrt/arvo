# 12: A fresh read of the current shape, from outside the road that led to it

**Reviewer:** Chris Lattner (infrastructure and layering lens: does it compose, will it be adopted,
are the layers cut in the right places).

**What I read:** `11_current_shape_draft.md` (only that file from this directory, per the brief);
the spec topic `mock/design_rounds/202607301200_topic.the-formalization-spec.md`; targeted sections
of `202607301000_topic.inherited-state-from-the-formalization-round.md` (placement calls, the
Warm/Precise 65-to-128 record, the 202607300300 proposal, the 202607300400 taxonomy walk, both
sketch records); `mock/DESIGN.md.tmpl`; the shipped source of `arvo-strategy`, `arvo-storage`,
`arvo/src` (in particular `float.rs`, `cross_strategy.rs`, `width.rs`, the test suites); the
algorithm crates' trait bounds; consumer usage in hilavitkutin and kolli; the workspace rules named
in the brief. I did not read the panel transcripts (files 00 through 10).

**What I verified by running or by reading shipped artifacts**, as opposed to reasoned about:

- The full mock workspace suite: 654 passed, 0 failed, 9 ignored. The one deliberate catalogue-red
  is properly marked (`crates/arvo/tests/fixed_point_div.rs:111`). Test bodies I sampled
  (`strategy_semantics.rs`, `cross_width.rs`) assert concrete values, not tautologies.
- The two live `generic_const_exprs` gates the draft names exist exactly where it says:
  `mock/crates/arvo/src/lib.rs:25` and `mock/crates/arvo-strategy/src/lib.rs:11`. The draft's
  standing-constraints section matches `unstable-features.md` as it stands today. Given this panel
  family's recorded history of inheriting a false GCE premise, I checked this one against source
  first; the draft is telling the truth about it.
- `FastFloat` and `StrictFloat` differ by a compile-time fast-math license and nothing else:
  `mock/crates/arvo/src/float.rs:4` ("enables fast-math semantics (reassociation, ...)"),
  `float.rs:29-41` (the two wrappers over the same `F: Ieee`), `float.rs:217-226` (the
  `arvo_fast_math` cfg switching a default alias between them).
- The shipped cross-strategy surface: `Resolve<S1, S2>::Out` projecting "the more conservative of
  two strategies", and `CrossStrategyOp` implemented only for same-strategy pairs, with the
  toolbox-rule diagnostic (`mock/crates/arvo-strategy/src/cross_strategy.rs:1-50`).
- The algorithm crates bound weights on core-ops-style traits:
  `arvo-graph/src/rank.rs:39` (`W: Add<Output = W> + TotalOrd + Copy + FromConstant`), same shape
  in `path.rs:36`, `spanning.rs:92`, `arvo-comb/src/dp.rs:41`, throughout `arvo-spectral`. No
  algebra ladder exists in source yet (zero hits for `Magma`/`Semigroup`/`Combine` in `crates/`).
- `Width` lives in `arvo-strategy` (`width.rs:33`), and its own doc comment records that it has
  already been relocated once for reachability reasons (`width.rs:8-10`).
- Consumer usage: hilavitkutin has roughly 22 bare `Uint<N>` call sites (defaulting Warm) against
  17 explicit-strategy sites, consistent with the draft's twenty-two-of-thirty-one figure; kolli
  uses arvo numerics at exactly one site (`Uint<16, Precise>`). The draft's "one real downstream
  consumer measured" is hilavitkutin-shaped and its numbers check out.
- The shipped Warm semantics the migration must audit: `crates/arvo/tests/strategy_semantics.rs`
  asserts that `UFixed<8, 0, Warm>` computes `200 + 100 = 300`, held in the u16 container, with no
  wrap and no clamp at the logical range. I return to this below because it is more important than
  the draft treats it as being.

Everything else in this file is reasoning, and is offered as directions to open, not rulings. Where
I hold more than one reading I say so.

## 0. The part that is right, so the rest lands in proportion

The Numeral half of this design is real design, not inventory. Every one of its five members has a
role in one formula (the affine value map times an exponent form), the literature grounding is the
correct literature, and the identity/policy/lowering cut survived three independent attempts to
break it. Quantisation-as-one-map is genuinely better than the rounding/overflow split every
industrial system ships, and the `ReduceModulo`-at-a-midpoint refusal is the type system doing
exactly what this workspace builds type systems to do. The Thread C fifth-pass shape (one
definition, checked exhaustively at a model width, executed at real width, same symbol) is the
single most valuable mechanism in the whole round. None of what follows retracts any of that.

What follows is what the draft does not ask.

## 1. The headline unification fails on its fourth family, and the failure names a missing axis

Section 3.1 opens: "`UFixed`, `IFixed`, `FastFloat` and `StrictFloat` are names for four
compositions of a single generic type, and what differentiates a composition is where its exponent
lives."

`FastFloat<F>` and `StrictFloat<F>` have the same exponent placement, the same representable set,
the same everything the ten axes can name. What differentiates them, verified in source
(`float.rs:4`, `float.rs:217-226`), is a license: whether the compiler may reassociate, contract,
and collapse signed zeros. That license changes answers while changing no representable value,
which by the design's own sorting test (spec `202607301200`, D54: "if the same values are still
representable and only the arithmetic differs, it is policy") is a `Policy` concern. No `Policy`
axis carries it. So as written, either the headline claim is quietly "two families plus a packaging
pair", or the taxonomy is missing an axis. The draft never confronts this; its 5.1 discussion of
`Stored` gets close (formats, hidden bit, specials) but the Fast/Strict pair is a different gap:
not "the axis is inexpressive of real formats" but "the one thing the shipped product actually
differentiates on has no axis at all."

I would name the missing axis something like arithmetic fidelity: `Exact` (every operation
quantises exactly as specified, no reassociation, no contraction) versus `Relaxed` (the toolchain
may transform under the usual fast-math envelope). Three independent open problems in the draft
converge on this same missing member, which is the strongest signal in my whole read:

1. **Fast/Strict inexpressibility**, above.
2. **The per-operation versus deferred quantisation gap** (draft 5.1). Deferred quantisation is
   precisely a license to not quantise where the spec says an operation occurs; that is a fidelity
   relaxation, not a fifth thing. If quantisation siting is an axis member, the gap closes by
   naming it rather than by picking a winner: the conv-* aliases pin per-operation (SystemC and
   MATLAB semantics require it, see section 8 below), and a relaxed composition may defer.
3. **The FMA collision.** `arvo-always-optimal-internals.md` promises internals free to lower to
   whatever is fastest, explicitly including `llvm.fmuladd`. Contraction quantises once where
   per-operation semantics quantises twice; the answers differ. Without a fidelity axis, the new
   `Policy` machinery and the internals rule contradict each other on every fused operation, and
   the contradiction is silent. With it, `Exact` compositions forbid contraction and `Relaxed`
   ones license it, and the two rules compose instead of colliding.

It also repairs `Deterministic`. The draft banks `Deterministic` as a derived blanket "keyed on the
whole composition" (3.4) while conceding `ConstantTime` cannot be a type-level fact. But
`Deterministic` has the same hole for any composition lowering to hardware floats: a `FastFloat`
composition is not deterministic across targets or toolchains, and nothing in the ten axes
distinguishes it from `StrictFloat`. Either `Deterministic` moves to the measured bin next to
`ConstantTime` (defensible but weak), or the fidelity axis gives it the input it needs: `Exact`
compositions derive `Deterministic`, `Relaxed` ones do not. The second is better and the draft's
5.4 ledger currently cannot express it.

Alternative reading, to hold honestly: op ratified D50 ("the crate split survives as packaging, not
as a mathematical claim"), and one could read Fast/Strict as pure packaging too, outside the
formalization's scope, with 3.1's sentence just overclaiming. If so, the fix is one sentence in the
spec (the unification covers the value sets and the quantisation, not the optimization license) and
an explicit note that `Deterministic` cannot be derived for float compositions. But the
three-arrows convergence above makes me think the axis is the truth and the packaging reading is
the patch.

## 2. Mixed-operand arithmetic is absent from the taxonomy entirely

The shipped surface has a whole mechanism for it: `Resolve<S1, S2>` picking "the more conservative"
strategy, `CrossStrategyOp` as the warn-not-refuse diagnostic
(`arvo-strategy/src/cross_strategy.rs:3-7`, backed by `arvo-toolbox-not-policer.md`'s worked
example, and by 18 shipped cross-width tests). The spec that replaces the strategy bundle says
nothing about what happens when `Number<N1, S1>` meets `Number<N2, S2>` at an operator. Neither
does the draft: its section 5 catalogue of the open and the broken never mentions mixed operands.

Under ten axes this is not a detail. "More conservative" was hand-waved even at four presets; at
ten axes it needs a defined join per axis, and several axes have no natural order (what is the join
of `ToEven` and `TowardZero`? of two different `Bias` origins?). Two coherent shapes exist and the
spec must pick one:

- **Define the join** where one exists (widths, growth, stored width have natural lattices) and
  refuse-with-diagnostic where none does, keeping a `Resolve`-shaped mechanism but derived per axis
  rather than asserted per preset pair.
- **Abolish implicit mixing**: mixed-operand ops always go through an explicit requantisation at
  the call site (`.requantise::<Target>()` or the modifier vocabulary from Thread A), and the
  substrate ships the conversion, not the resolution. This is cleaner, more honest about the fact
  that any resolution is a quantisation someone chose, and consistent with the toolbox rule's
  preference for visible casts over silent adoption; its cost is call-site noise in mixed-width
  arithmetic, which the current consumers do very little of (verified: hilavitkutin's sites are
  overwhelmingly single-composition).

Related and larger: **conversion is quantisation, and the spec does not connect them.** Converting
`Number<N1, S1>` to `Number<N2, S2>` is exactly the map section 3.3 defines: take an exact value,
land it on a representable set, resolve the five situations. The refit family (`Narrow<T>` /
`Widen<T>` in `arvo-bits-contracts`, re-exported through `arvo-refit`) is today an independent
hand-written surface in a crate this round barely touches. One definition should serve arithmetic
(widen, operate exactly, quantise), conversion (quantise into the target numeral), storage
narrowing, and cross-composition resolution. That is the same one-definition-monomorphised-twice
insight Thread C's fifth pass already had, applied one level up, and it would let the Thread C
checking machinery certify the conversion surface for free instead of leaving refit outside the
perimeter. As it stands, the 5.4 ledger's trusted bin quietly contains the entire refit family and
nobody has noticed, because refit is not in the taxonomy.

## 3. `Growth` and `Widening` are not independent, which breaks the axis product where nobody looked

Read the preset table (draft 3.5) as data about the axis space:

| | growth | stored width | widening |
|---|---|---|---|
| `Hot` | narrowed to operand | minimum | none |
| `Cold` | exact | minimum | per operation |
| `Warm` | exact | doubled | in container |
| `Precise` | exact | doubled | per operation |

`Growth = Exact` with `Widening = None` is unimplementable: the exact intermediate has to live
somewhere. So the ten-axis product contains points that cannot be lowered, and nothing anywhere in
the draft names the compatibility predicate. This matters in two directions.

First, three of the four presets sit exactly on the line "widening is derivable from (growth,
stored width)": narrowed-to-operand needs none; exact plus minimum needs per-operation; exact plus
doubled can use the container. `Precise` deviates (doubled, yet per-operation). If that deviation
has a reason (perhaps: exact growth of products outruns even a doubled container, so widen anyway),
it should be stated, because it is the only evidence that `Widening` is a real degree of freedom
rather than a derived quantity. If it has no reason, the axis count is nine, and one of the three
`Lowering` members dissolves into a computed projection. Either outcome is better than the current
state, where an axis that might be derivable is offered to consumers as a free choice whose invalid
region they discover by compile error with no diagnostic story.

Second, and structurally more interesting: fixing this requires a cross-contract constraint, a
where-clause of the shape "this `Lowering` is sufficient for this `Policy`". The 5.2 phantom-type
work goes to great lengths to make one direction of cross-contract reading unwritable (a law must
not read `Lowering`). This finding shows the other direction is mandatory (a lowering must be
constrained by policy). The real invariant is therefore an asymmetry, sharper than anything the
draft states: **`Policy` may constrain `Lowering` (sufficiency), `Lowering` may never inform
`Policy` or the laws (independence).** I would put that sentence in the spec verbatim, because it
is the actual design rule the fused-versus-split debate, the phantom-type closure, and this
compatibility predicate are all instances of, and stating the general rule once beats rediscovering
its instances at each mechanism.

## 4. Delivery is a fourth contract, and the draft is one step from saying so

The `Lowering` charter is "Changes no answer" (spec line 54) and the draft's sorting test asks
about representable values, answers, and cost. Thread B's sharpest reframe (draft 4.2) then files
refusal delivery (checked sum versus absorbing bottom versus sticky flag) under `Lowering`. But a
delivery choice changes the return type of `+` at every call site. That is not a cost; it is
observable in the type system, it changes what consumer code type-checks, and it is precisely the
"third sort the test does not ask" the draft twice gestures at (3.1, 4.2) without resolving.

The taxonomy is missing its fourth question: not "which values", not "which answers", not "what
cost", but **"what shape does the result present to the consumer"**. Call it Observation or
Delivery. Once named, several homeless things move in together and stop being anomalies:

- Refusal delivery (Thread B's whole subject), including the sticky-flag and bottom carriers and
  the `settle()`/`observe()` perimeter the 5.4 ledger already treats as a trust boundary.
- `Fallibility<T>`, currently an associated member of `Quantisation` (spec line 156), i.e. of
  `Policy`. By the design's own sorting it does not belong there: whether `Refuse` fires is policy;
  how a firing travels is not. The draft flags "should it be computed" (4.2) but not "is it on the
  wrong contract", and the second question comes first.
- `ConstantTime`, which the draft already concedes is "keyed on data that does not decide it"
  because delivery decides it (4.2). With delivery as a contract, `ConstantTime` keys on
  (delivery, target), which is at least the right shape even though it stays a measured promise.
- The stochastic-rounding exclusion (5.1): the honest reason is that resolution constructors are
  pure ZSTs; a dithered quantiser needs state threading, and state threading is a carrier
  question. Recording the exclusion against the Delivery contract keeps the door findable for the
  audio/ML consumer who will eventually knock.
- Growth's two-refusal-sites problem (5.2's unaddressed carrier-join extension) is naturally a
  Delivery composition question, not a Policy one.

The alternative reading: keep three contracts and widen `Lowering`'s charter to "changes no value
and no answer, may change the call-site type". That is workable but it costs the laws: every law
proven under Kleene equality must then be quantified over deliveries (does translation stability
survive the bottom carrier? the sticky flag?), and the phantom-type closure that makes laws unable
to read `Lowering` would make them unable to read the very member that decides whether "both sides
refuse" is even observable. I think that tension is the design telling you delivery is not
lowering. Either way, this is the most consequential unresolved sort in the document and it
deserves to be resolved as a named question, not absorbed into Thread B's next iteration.

## 5. The law machinery's first real clients are the algorithm crates, and nobody has asked what they will bound on

This is, for my lens, the largest untouched surface in the whole round. The draft derives
`AddAssoc` with real care and states the consequence ("Only `Hot` folds for signed values", 3.5).
It never traces that consequence into the four crates that actually fold.

Verified: `arvo-graph::rank` folds weights bounded on `Add + TotalOrd + Copy + FromConstant`
(`rank.rs:39`), and the same bound shape runs through `path`, `spanning`, `dp`, and all of
`arvo-spectral`. Three collisions follow, none mentioned in the spec, the draft, or the crate
table:

**(a) If the algorithm crates adopt the algebra ladder, signed non-Hot compositions stop
compiling in them.** A signed `Warm` edge weight in `upward_rank` would be refused, because
clamping is not translation-stable. That is the substrate refusing a combination a consumer has a
real reason to want; consumers fold saturating values on purpose and accept order dependence (any
mixing accumulator does). `arvo-toolbox-not-policer.md` says warn, never refuse; the design's
proudest mechanism says refuse. This is a genuine intent-level fork and it needs op, but I will
state the resolution I believe is right, because it dissolves the collision instead of picking a
side: **a sequential fold with a fixed traversal order is a well-defined function without
associativity.** Associativity is required only when the reduction order is unspecified, which in
this stack means parallel reduction, which is hilavitkutin's layer, not arvo's. So: arvo's
algorithm crates bound on the weakest structure that makes each algorithm well-defined under its
documented iteration order (magma plus documented order), and offer lawful-bound variants; the
`AddAssoc`-gated surface becomes the contract of *parallel* reduction one layer up. That gives the
law machinery its first genuinely paying customer (hilavitkutin's trunk-parallel folds, where a
grouping-dependent answer really is a bug) instead of spending its capital refusing sequential
folds that were never wrong.

**(b) `TotalOrd` is the comparison Thread B showed silently discards an absorbing bottom** (the
pre-2019 IEEE minNum defect, draft 4.2). If bottom delivery ships in any form,
`arvo-numeric-contracts` must grow a propagating comparison contract, and every `TotalOrd` bound in
the algorithm crates becomes a decision point. That crate appears nowhere in the new crate table
and nowhere in the spec.

**(c) Two parallel operation vocabularies are about to coexist**: the core-ops-shaped
`arvo-numeric-contracts` surface (`Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`) and the new
`arvo-algebra-contracts` ladder (`Magma<Op>`, laws as markers). Without a stated relationship (I
would state: the ops traits are the syntax, the algebra markers are laws *about* those ops, and
nothing may declare a law for an op it does not implement) the ecosystem fragments along the
copies, which is the exact failure `vocabulary.md` and the one-definition discipline exist to
prevent. The crate table (3.7) needs a row for `arvo-numeric-contracts` stating its fate, and the
spec needs a sentence on what an algorithm crate's bounds become. Today both are silent, and
silence here will be filled crate by crate, differently.

## 6. The crate table splits along contract names, not along seams, and its strongest justification was disproven by the round's own work

The six-crate table (3.7) reads as one-crate-per-trait. The strongest architectural argument for
it, that a crate boundary makes laws provably unable to read `Lowering`, was tested and **failed**:
the draft's own 5.2 records that the crate owning the real `Number` type can always write the
conditioned law, and that the phantom `LogicalNumber` closure works "independent of whether any
crate split exists at all". So the split is packaging. Packaging is fine, but then it must answer
packaging questions, and it currently answers none of them:

- **The vocabulary has no home.** `Width` sits in `arvo-strategy` today (`width.rs:33`), placed
  there by a previous relocation done precisely for reachability (`width.rs:8-10`). Under the new
  table, `arvo-numeral` needs it (`LogicalWidth`), `arvo-lowering` needs it (`StoredWidth`),
  `arvo-policy` needs it (`IntermediateWidth` inside `Narrowed`), and `Exponent` has the same
  triple citizenship. The table has no row for them. Whichever contract crate hosts them becomes a
  dependency of the other two, quietly reintroducing the edges the split was meant to sever; a
  seventh vocabulary crate fixes that at the cost of another crate. This exact class of problem
  has already forced one relocation in this repo's history; deciding it after the split ships
  would force another.
- **The container projection has no home.** `arvo-strategy` "holds only the four presets and
  nothing else" evicts `BitsContainerFor`/`Project`, the most load-bearing mechanism in the
  shipped crate. Presumably it lands in `arvo-container`, the one row the draft itself flags as
  reviewed by nobody "despite being exactly what the new `Lowering` contract governs from above"
  (section 1 table). That admission should be read as a priority, not a footnote: `arvo-container`
  is where `Lowering`'s charter gets implemented, and it is the least-examined crate in the round.
- **`Bits<N, S, Sign>`'s `S` parameter is the split's real test, and nobody has stated it.** Under
  the three contracts, a storage type has no business being parameterised by anything that can name
  `Policy` members; `Bits` should re-bound `S` from the fused strategy to `Lowering` alone. That
  is the same leak 5.2 fights, one level down, and it is cheap to state now and expensive to
  retrofit after more consumers write `Bits<13, Hot>` against the fused bound. The draft's 3.8
  says `Bits<13, Hot>` "still reads as itself"; true, and the interesting question is what `Hot`
  *is* to a `Bits` afterward, which is exactly the sentence the spec is missing.

My suggestion, offered against the table rather than as a ruling: the cut that has earned its keep
in this repo is contracts-versus-implementations (the existing L0.5 pattern, stated in
`DESIGN.md.tmpl`'s own "optimises transitive dependency size" rationale). One contracts crate
holding all three contracts plus the shared vocabulary delivers every benefit the six-way split
delivers (the compile-time measurements in 3.9 say the difference is negligible, which cuts
against fine splitting, not for it), with zero homeless-vocabulary problems and three fewer
boundaries for the next round to re-litigate. Split further only when something concrete pays for
it: a real consumer wanting policy-without-numeral, or a measured build-parallelism win. Crate
boundaries are the hardest thing in a Rust ecosystem to walk back, because Cargo makes them public
identity; six speculative ones is five more bets than the design needs to place this week.

## 7. Sequencing: the verification spine should gate the crate moves, and the current order is backwards

The round has already banked a dozen relocations (capacity, shape, platform, container rename,
bitfield, float packaging, plus this table) while the Thread C spine, the thing every law, preset,
convention and membership fact is a client of, "has not been tried against arvo's real storage
representation (`Bits<N, S>`-shaped values), only against a small integer model" (draft 4.3), and
sits at pass five of a sequence where each of the previous passes was broken by whoever compiled it
next.

I want to sharpen why the `Bits` trial is not a formality. The 5.4 transfer argument ("a fact
checked exhaustively at three or four bits holds at every width") is defended by the specialization
and `TypeId` bans: the checked function cannot ask which width it runs at. But the shipped carrier
*changes code path by width bucket*: native primitives to 64 or 128, then `WideBits<BYTES>` limb
arithmetic above (`arvo-strategy` container table; `DESIGN.md.tmpl` L0 row). A model-width check
runs the native path. A width-100 composition runs the limb path. Same source text, structurally
different execution, and the carrier-agreement obligation (4.3's "second, smaller obligation") is
checked at the model width only, so a limb-arithmetic defect in the wide carrier is invisible to
the entire checking apparatus **by construction**, not by omission. The 5.4 ledger does list the
per-width primitive ops as trusted, but it understates the situation: they are not merely
untouched by the new machinery, they are outside the transfer argument's assumptions whenever the
code path is bucket-dependent. The wide bucket is also exactly where the design/source disagreement
already recorded in this round lives (the Warm/Precise 65-to-128 band, where projection, diagnostic
and design document mutually disagreed for months because no test pinned any of them,
`202607301000` section `202607300100`).

Concrete suggestion: make "the Thread C spine compiles and its obligations pass against
`Bits`-backed `Number` at one native width and one `WideBits` width" the gate for every crate
relocation in the table. If the spine holds there, the packaging is safe to land in any order. If
it does not, the packaging would have calcified around a broken spine, and this workspace has
already paid for that mistake once at ecosystem scale elsewhere. The deep unblocking piece first;
the boxes after.

## 8. The banked and the open are entangled, and one banked item is unimplementable as stated

The draft's structure implies section 3 is safe to build and section 5 is future work. Two places
where that ordering is inverted:

**The preset redefinition (3.5, banked) cannot be stated until quantisation siting (5.1, open) is
decided.** Verified against the suite: `strategy_semantics.rs` pins that today's `Warm` at logical
width 8 computes `200 + 100 = 300`, held in the container, with no per-operation enforcement of the
logical range at all. So `Warm`'s migration is not "wrap becomes clamp"; it is "no per-operation
range semantics becomes some range semantics", and *which* results change depends entirely on
whether quantisation fires per operation (300 becomes 255 immediately) or deferred (300 survives
until a store). The audit obligation 3.5 carries (flip the pinned assertions test by test) cannot
even enumerate its cases until 5.1's siting question closes. A banked item that depends on an open
item is open.

**The conv-* adequacy test (3.6, banked) quietly decides the siting question.** SystemC's
`sc_fixed` and MATLAB's fi both quantise per operation; if `conv-systemc` aliases must compute
SystemC's numbers (and the draft's own stronger vendor-vector test, 5.1, says they must), then
per-operation siting is forced for those aliases. Presets are then free to differ only if siting is
itself expressible, which is finding 1's fidelity axis arriving from a third direction. I would
let the conventions decide it: they are the falsifiability instrument this design already believes
in, and they have an answer where the abstract argument has a stalemate.

## 9. A canonical-form story is missing, and three recorded defects are the same defect

Three separate open items are one class: `FullRange<1>` is value-equal to a power of the radix but
type-distinct from `Unit` (5.1); `Offset<0>` versus `Zero` has the same shape waiting to happen
(3.2 lists both); and Thread A's modifier spellings do not canonicalise
(`LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>` versus the other order, 4.1). Every derivation in
3.4 that conditions on a constructor's *name* rather than the value it computes carries this class
of gap, and every diagnostic that renders a spelling rather than a normal form carries the other
half.

The standard cure is the one compilers use: a canonicalisation pass at the composition boundary.
Constructors normalise before any derivation reads them (`FullRange<1>` rewrites to `Unit` with an
exponent shift, `Offset<0>` to `Zero`, modifier chains sort into a fixed axis order), derivations
and law keys only ever see normal forms, and textual snapshots become stable. One mechanism closes
the membership gap, the future bias gap, and the modifier-ordering non-determinism at once, instead
of three patches. In this type-level setting the canonicaliser is a family of type functions
(associated-type projections from raw composition to normal form), which is more machinery, but it
is machinery with one definition that every derivation then reuses, which is the trade this
workspace habitually takes and should take here.

A free theorem worth adding to the derived set while in this territory: **raw-integer comparison
agrees with value comparison exactly when the affine map is monotone** (positive adjustment; any
fixed bias). Nothing states it, everything depends on it: `Cold`'s cheap bitpacked compare, every
`TotalOrd` impl on a fixed composition, the sort in every algorithm crate. It is derivable from
the numeral's own members, it is the kind of fact this framework exists to compute, and stating it
would make the first negative-adjustment or exotic-bias constructor fail loudly at the comparison
impl instead of sorting wrong at runtime.

## 10. Direct answers to the wide questions

**Is this restructuring the right thing to be doing at all?** The formalization, yes,
unambiguously: it is the rare case where the mathematical structure and the engineering win
coincide, and it survived hostile compilation. The simultaneous re-packaging, not yet: the crate
table is speculative packaging whose best argument the round itself disproved (finding 6), and it
should trail the verified spine (finding 7), not lead it. The two halves of the round have
different risk profiles and should not share a fate.

**Is ten axes across three contracts a design or an inventory?** The `Numeral` five are a design:
each member is a coordinate of one formula, and removing any breaks expressibility somewhere
concrete. The `Policy`/`Lowering` five are still part inventory: their product space contains
unimplementable points nobody has fenced (finding 3), their charter cannot hold delivery (finding
4), one axis may be derivable (Widening), and the one shipped differentiator they cannot express is
the one the product actually uses (finding 1). The test of a design over an inventory is that the
product of the axes is exactly the meaningful space. Identity passes that test today; the other two
contracts do not, yet, and the fixes are all nameable.

**What is missing from the taxonomy entirely?** In descending order of cost-if-ignored: mixed-
operand composition and its join (finding 2); conversion-as-quantisation, which currently leaves
the whole refit surface outside the verification perimeter (finding 2); arithmetic fidelity, with
three independent arrows pointing at it (finding 1); the delivery/observation contract (finding 4);
canonical forms (finding 9); the algorithm-crate bound story and the sequential-versus-parallel
lawfulness split (finding 5); the monotone-encoding comparison theorem (finding 9).

**Are the parts nobody examined the parts that matter?** Two of them, yes. `arvo-container` is
where `Lowering` actually becomes real, and it carries a "reviewed: No" in the draft's own table;
it should be the next review target before any doc changelist. And the algorithm crates are where
the law machinery either earns adoption or teaches consumers to route around it; a substrate whose
proudest guarantee first manifests downstream as "your graph rank stopped compiling" will get the
guarantee disabled at twenty call sites with lint escapes, which is how technically superior
systems fail at engineering. The sequential/parallel split in finding 5 is, I believe, the shape
that makes the guarantee land where it is wanted and stay out of the way where it is not, which is
the whole art of shipping infrastructure people keep.

## What I did not do

I ran no new compile probes; every verification above is against the shipped tree and the existing
suite, so `12_probes/` is empty. The findings that most deserve probes next, if the review
continues in the compiling register it has used so far: the `Bits`-backed Thread C spine at a
`WideBits` width (finding 7, the one I predict breaks first), a two-line demonstration that
`Growth = Exact, Widening = None` has no implementable carrier (finding 3), and a mixed-operand
`Number` addition under any proposed join (finding 2, where I expect the absence of a design to
become undeniable within a page of code).
