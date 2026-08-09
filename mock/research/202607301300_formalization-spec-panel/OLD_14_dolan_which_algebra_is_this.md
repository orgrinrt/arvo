# 14: Which algebra is this

**Reviewer:** Stephen Dolan (subtyping and biunification lens: distrust an inherited hierarchy on
sight, ask what the smallest structure that is actually correct looks like, and check whether it is
smaller than what shipped).

**What I read.** `11_current_shape_draft.md` in full. `13c_op_the_standard_and_the_mode.md`,
`13_mcsherry_where_the_laws_belong.md`, `13b_op_checkpoint_five.md`, per the brief. Then, because a
question about "which algebra" cannot be answered without knowing what was already decided about the
ladder's shape, the governing design-round text itself:
`mock/design_rounds/202607301200_topic.the-formalization-spec.md` (the crate table and the ladder
paragraph, section 3.7's source) and
`mock/design_rounds/202607300800/202607292300_topic.the-axes-want-research-and-the-ladder-follows-theory.md`
(D46, D47: the standing calls on whether the ladder narrows). Then the prior-art research this design
round already commissioned and, on the evidence below, did not fully carry forward:
`mock/research/202607281616_prior_art/04_algebraic_structure_hierarchies.md` and `99_synthesis.md`.
On source: `arvo-graph/src/{rank,path}.rs`, `arvo-comb/src/dp.rs`, `arvo-sparse/src/rcm.rs`,
`arvo-spectral/src/power.rs`, `arvo-strategy/src/{identity,axes}.rs`, `mock/DESIGN.md.tmpl`. I listed
`ls` on every directory named in the brief and on the two design-round directories above before
reading inside them, per the standing instruction.

**What I compiled, ran, or grepped**, as distinct from what I reasoned about, is in `14_probes/` and
cited by file:line below. Everything else in this file is argument, offered as directions rather than
rulings, and where I hold more than one reading I say so and do not resolve it for the next member.

## 0. A premise check, because the brief's own wording turned out to matter

The brief states the ladder "runs magma, semigroup, monoid, group, abelian group, ring, field." I
could not find that literal chain written down anywhere. `11_current_shape_draft.md:383-389` (the
draft's 3.7) says only that `Combine<Op>` is renamed `Magma<Op>`, "the precise term for a set with a
binary operation and no law claimed, with laws attached as separate markers so a structure is a magma
plus the laws it happens to satisfy," and that the ladder is "declared to the depth the mathematics
goes." The governing spec text at `mock/design_rounds/202607301200_topic.the-formalization-spec.md:306-310`
says the same thing in the same words, and attributes the "vocabulary fixed by mathematics" reasoning to
a decision named D38. D38 exists and is not this: it is
`mock/design_rounds/202607300800/202607291900_topic.the-number-systems-crate.md:12-14` (op, 2026-07-29),
"arvo gets a crate for number-system membership," the decision that created `arvo-num-systems`, with no
mention of the algebra ladder anywhere in it. The reasoning the spec text quotes is D47's, two topic
files later, word for word ("a vocabulary fixed by mathematics cannot be got wrong in a way that later
needs undoing" appears in both). So the spec text's own citation is a misattribution, small on its own
but the kind of thing worth flagging on a document whose central claim is that a settled ladder cannot
be got wrong later: the citation trail for that very claim already drifted once, within the same round,
before this dive started. Neither text enumerates semigroup, monoid, group, abelian group, ring, or
field by name. So the brief's chain is a reasonable *inference* about where "the mathematics goes"
from Magma, and it is the inference anyone trained on Bourbaki-style abstract algebra makes by
reflex, but it is not itself a decision on record, and section 3 argues the reflex is the thing worth
distrusting.

Two standing calls bound this more tightly than the draft text does on its own, and I read them as
governing this dive:

- **D46** (`202607292300_topic...md:8-36`, op, 2026-07-29): a finding that "signed saturating
  addition is not associative while wrapping addition is a full abelian group" was recorded as **a
  research question, not a mechanism to pick**, with three things to establish before any fix ships:
  prior art on numeric dispatch strategy broadly, whether arvo's axis decomposition is wrong, and
  **whether there are more axes than currently modelled**. Nothing in the files I read after that date
  answers the third question directly. Section 4 of this file is an attempt at it.
- **D47** (same file, lines 38-63, op, 2026-07-29): "the algebraic ladder goes as deep as the theory
  does. Not as deep as a named consumer forces, and not truncated to where other ecosystems stopped,"
  explicitly **rejecting** NumHask's collapse of Ring/Field/Distributive/Module to type synonyms and
  `alga`'s succession by the narrower `simba` as decision inputs, on the grounds that those are facts
  about languages without arvo's typestate machinery, not facts about what arvo can afford. The
  obligation attached: "every rung that goes in is sketched and benched to prove that productivity and
  efficiency both hold."

I want to be exact about how what follows relates to D47, because on a fast read it can look like I am
arguing against it. **I am not proposing to narrow the ladder.** Every reading in section 4 widens it:
the mathematics this substrate's shipped consumers actually run on has a branch the current ladder
does not reach at all (idempotent semirings, ordered algebra), and reaching it is depth the draft has
not yet claimed, not a retreat from depth it has. What I am questioning is the *mechanism* by which
each rung earns its place, which is a different axis from how deep the ladder goes, and D47's own
obligation (sketch and bench every rung) is, I will argue, served better by the mechanism in section
4.2 than by declaring named rungs as primary vocabulary each carrying its own proof burden.

## 1. What the shipped consumers actually compute, read from source

Four shapes, distinct enough that treating them as one question ("does `+` fold") is already the
place file 12 and file 13 both found trouble.

**`arvo-graph`'s rank and longest path are max-plus recurrences.** `rank.rs:84`:
`rank[node_i] = if any { w + best } else { w }`, where `best` is the running maximum over already-
computed successor ranks (`rank.rs:65-82`), doc comment stating the recurrence directly at
`rank.rs:5-8`: `upward_rank[v] = weight[v] + max(rank[succ] for succ in successors(v))`. `path.rs:81`
is the identical shape for the single-source longest path, doc at `path.rs:4-8`. Addition is applied
exactly once per node; the reduction that repeats is `max`, over a variable number of predecessors.

**`arvo-comb`'s matrix-chain DP is a min-plus recurrence, the dual.** `dp.rs:100`:
`let candidate = dp.get(lou, ku) + dp.get(k1u, hiu)`, combined by `total_cmp ... Less`
(`dp.rs:101`), i.e. `dp[lo][hi] = min over k of (dp[lo][k] + dp[k+1][hi])`. Nobody in the earlier dive
looked at this crate; file 13 checked `arvo-graph` exhaustively and stated the finding as being about
`upward_rank` and `longest_path` specifically. Probe 1 (`14_probes/01_min_plus_dual.rs`) checks
whether the presets sort the same way for the dual reduction.

**`arvo-spectral`'s power iteration is a genuine sequential fold over `+`, on a float.**
`power.rs:71`: `sq_sum = sq_sum + ns[k] * ns[k]`, a single accumulator walked once, left to right,
inside one call. `FastFloat`/`StrictFloat` addition is non-associative regardless of strategy marker,
because the non-associativity is rounding, not policy (draft 5.1 does not say this outright but the
float wrappers are sealed hardware `f32`/`f64` per draft 5.1's own text, and the prior-art pass already
states plainly that IEEE-754 addition is commutative but not associative because of rounding,
`04_algebraic_structure_hierarchies.md:249-254`).

**`arvo-sparse`'s RCM is pure combinatorics on a permutation, with no arithmetic law in play at all.**
`rcm.rs:36-70` picks a start node by minimum degree, then walks a BFS, appending to a permutation
buffer; the only arithmetic is unsigned index increment by the always-present multiplicative identity
on `USize` (`rcm.rs:56`, `USize as Identity<Multiplicative>`), which is natural-number arithmetic and
carries no interesting law question. I did not look further into `dm.rs` or `block.rs`; if either
performs any genuine matrix arithmetic rather than structural (nonzero-pattern) reasoning, it is worth
a member checking whether it belongs in this picture (Boolean semiring, `⊕ = OR`, `⊗ = AND`, is the
standard algebra for reachability and block structure, and would be a fifth shape in this list; I did
not verify this, it is a direction, not a finding).

Four crates, three distinct algebraic pictures, and file 12's original framing ("a fold, needing
associativity") only actually describes the fourth crate, `arvo-spectral`, and even there the fold
never regroups (section 5 returns to this).

## 2. Probe 1: the min-plus dual sorts exactly like the max-plus case, and it was untested

Compiled and run: `14_probes/01_min_plus_dual.rs`, `rustc -O`, exhaustive over the representable range
`[-4, 3]` (matching file 13's probe 2 for direct comparability):

| arith | `+` associative | `+` distributes over min |
|---|---|---|
| Wrap (`Hot`) | yes | **NO** at `(w=-4, a=-4, b=0)` |
| Saturate (`Warm`/`Cold`) | **NO** at `(-4,-4,1)` | yes |
| SubstituteZero | NO | NO |
| Exact | yes | yes |

Identical shape to file 13's max-plus table (`13_mcsherry...md:168-172`). The preset that folds
(`Hot`, wrap) is exactly the one whose `min`-side distributivity fails; the presets `AddAssoc` would
refuse (`Warm`, `Cold`) are exactly the ones `arvo-comb`'s DP actually depends on. This was not
previously checked and closes the gap file 13 left open (its own section 6 audit is entirely
`arvo-graph`-and-scheduler-shaped; `arvo-comb` never appears in file 13 by name outside a passing
mention that RCM is unrelated combinatorics). The two crates are not independent data points on the
same fact; they are the two canonical instances of one dual pair, `(max, +)` and `(min, +)`, and the
literature has a name for the pair: **tropical semirings**, sometimes "dioids" (Baccelli, Cohen,
Olsder, Quadrat, *Synchronization and Linearity*, the standard reference for max-plus algebra;
Gondran and Minoux, *Graphs, Dioids and Semirings*, 2008, which is specifically about exactly this
graph-algorithm use). The prior-art pass already found the connection and named it, independently and
before this file: `99_synthesis.md:97-99`, "tropical algebra... connects to the shortest-path and
scheduling work `arvo-graph` already does," citing PALMA, a fixed-point integer tropical-algebra
library for embedded ARM with a static-buffer no-alloc mode, as "the only surveyed algorithm family
that arrived already satisfying the constraints." That finding is two days old at the time the ladder
draft was written and does not appear in it: zero hits for `Semiring`, `Dioid`, or `tropical` anywhere
under `mock/crates/` or in `11_current_shape_draft.md`.

## 3. Probe 2: monotone and translation-stable are not the same set, and the reason is a theorem

File 13's probe 01 found, on a signed model, exactly one monotone map among 65536 and exactly one
translation-stable map, and reported both counts without checking whether they name the same map.
Probe 2 (`14_probes/02_monotone_equals_stable.rs`) checks the identity directly, and decodes every map
found so the answer is inspectable rather than asserted. Four models, signed and unsigned, different
widths:

```
A (signed [-2,1]): monotone id=3840 -> [-2,-2,-2,-2,-2,-1,0,1,1,1]   (clamp)
                    stable   id=1252 -> [-2,-1,0,1,-2,-1,0,1,-2,-1]  (wrap, period 4)
B (signed [-1,1]):  monotone -> clamp;  stable -> wrap.  Same shape, smaller.
C (unsigned [0,3]): monotone id=4095 -> clamp (one-sided, only an upper bound exists)
                    stable: FOUR maps, including id=4095 (clamp) and three phase-shifted wraps
D (signed [-3,2]):  monotone -> clamp;  stable -> wrap.  Larger, asymmetric, same shape.
```

**In every signed model, monotone and stable are disjoint singletons naming different maps: clamp and
wrap.** This is not a coincidence of small models, and it is not merely empirical. It is a standard
fact of ordered algebra, provable directly from the definitions: **an orderable group must be
torsion-free.** If `(G, +, ≤)` is a group with a total order compatible with `+` (translation-
invariant: `x ≤ y ⟹ x+z ≤ y+z`), and some nonzero `x` has finite order `n`, then WLOG `x > 0`
(replace `x` with `-x` otherwise), and repeated translation gives `0 < x < 2x < ... < nx = 0`, which is
`0 < 0`, a contradiction. Wraparound addition on a fixed width is addition in the finite cyclic group
`Z/2^N Z` (the prior-art pass already derives this directly, `04_algebraic_structure_hierarchies.md:216-218`:
"`UFixed<I, F, Hot>` under `Wrapping` addition is a genuine abelian group under addition"). Every
nonzero element of a finite group has finite order. So `Hot`'s wraparound arithmetic is a torsion
group, and **no total order can ever be compatible with it**, on any width, signed or unsigned; the
unsigned model above still shows wrap failing monotonicity for the same reason, it is a fact about the
group `Z/nZ`, not about which integers get called negative. This is stronger than what either probe 01
or my probe 2 shows by search: it is a proof that the search could not have come out any other way, on
any width, and it means the fact `!Monotone<Additive> for <wraparound composition>` does not need
per-width exhaustive checking at all; it is a structural corollary of `Sign` and the resolution
constructor, derivable once and applied to every width, at zero const-eval cost, which matters given
`08_fog_the_union_and_what_it_costs.md:449,570`'s finding that exhaustive per-width checking "quadruples
per bit, costs 28 seconds at 8 bits, and is refused by `#[deny(long_running_const_eval)]` at 9."

**Clamp is the dual fact, and it is also a theorem, not a search result.** A monotone (order-
preserving) total map fixing an interval `[MIN, MAX]` pointwise is uniquely the nearest-point
retraction onto that interval, which is exactly clamp: probe 01 already established "the whole
monotone family is the single map `clamp`" (`13_mcsherry...md:236-238`), and this is the general fact
that any order-preserving retraction of a chain onto a convex subchain is the nearest-point map,
because monotonicity forces every point below the interval to the interval's bottom and every point
above to its top, with no freedom left anywhere.

**Why clamp fails to be stable (in the signed case) and wrap fails to be monotone are the same
underlying fact stated from two directions.** Restricting an unbounded totally-ordered abelian group
(`Z` or `Q`, arvo's `Exact` growth policy) to a bounded window admits two structurally different
completions, and only two: **quotient by a subgroup** (mod-n, which preserves the group operation
exactly and destroys the order, because a torsion group cannot carry one), or **retract onto a convex
sublattice** (clamp, which preserves the order exactly and, on a two-sided interval, destroys
associativity, because a large positive value clamped and then translated far negative does not agree
with translating first and clamping once, since the clamp at the intermediate step discards
information the un-clamped sum still carried). `Precise`, refusing instead of recovering, is the third
option: keep both, by not completing at all, which is exactly why it is fallible rather than total.

**The two-sided-versus-one-sided distinction is the whole story for `Sign`, and it dissolves an
apparent asymmetry.** Unsigned saturating addition clamps only from above (there is no representable
value below zero to clamp toward), and clamping from only one side does not have the escape mechanism
that breaks the two-sided case: once a running sum exceeds `MAX` it can never be brought back below
`MAX` by adding another non-negative number, so the "did we saturate" bit, once set, behaves like an
absorbing element and the operation stays associative. Verified directly, `14_probes/03_unsigned_saturate_both.rs`,
`rustc -O`, exhaustive over `[0,7]`: unsigned saturate is associative (`true`, no counterexample) **and**
translation-monotone (`true`, no counterexample); unsigned wrap is associative (`true`) but **not**
translation-monotone (counterexample at `c=0, x=1, y=8`), confirming the torsion-group argument is about
the abstract group `Z/8Z` and not about which integers happen to get called negative. The prior-art pass derived the same fact by hand two
days before the ladder draft, with a worked four-bit counterexample for the signed case
(`04_algebraic_structure_hierarchies.md:219-235`): "Saturating addition clamped at only one end... is
also associative... and this is exactly the tropical-semiring shape NumHask's `Positive`/monus type
relies on... Saturating addition clamped at *both* ends... is where it breaks," with the arithmetic
worked out (`a=7, b=-8, c=-8` under a four-bit signed range disagreeing between groupings). That
derivation is sitting in the repository, cited nowhere in the current draft, and it already contains
the finding file 13 rediscovered by exhaustive search and the finding I am adding here by proof.

**Consequence for the draft's presets.** `Unsigned Warm`/`Cold` are not caught in the tension the rest
of this dive is about at all: they are simultaneously an associative monoid *and* order-compatible,
with no conflict, because they instantiate the well-known "extended naturals with an absorbing top"
construction rather than either a torsion group or a two-sided-clamped interval. The disjointness
McSherry found and the presets-sort-opposite headline is, precisely, a **signed-only** phenomenon.

## 4. What algebra this is: three readings, and they are not mutually exclusive

### Reading one: name the missing branch

The ladder as drawn (magma toward group toward ring toward field) is a single ascending chain in the
sense that each rung strictly implies the one before it. Idempotent semirings do not sit anywhere on
that chain. A dioid requires `⊕` idempotent (`a ⊕ a = a`); an idempotent group is trivial (idempotence
plus invertibility forces every element to equal the identity), so **the moment `⊕` is idempotent, the
chain toward group is closed off, structurally, not by omission.** The mathematics this design needs
to represent (max-plus for `arvo-graph`, min-plus for `arvo-comb`, per the theorem above) sits on a
second branch entirely, built from a **lattice-ordered monoid**: a monoid `(M, +, 0)` carrying a
lattice order `(∨, ∧)` such that `+` is monotone, equivalently distributes over both `∨` and `∧`
(Birkhoff; Fuchs, *Partially Ordered Algebraic Systems*, is the classical reference for the whole
family: ordered semigroup, ordered monoid, ordered group, lattice-ordered group). Adjoining a top
element `⊤` (or bottom `⊥`) to such a structure and taking `⊕ = ∨` (or `∧`), `⊗ = +`, is exactly how a
tropical semiring is built from an ordered group in the standard construction, and it is why the law
`arvo-graph` and `arvo-comb` actually need, "monotone with respect to `TotalOrd`," is not an ad hoc
extra fact bolted onto the ladder: it is **the defining axiom of the branch the ladder is missing**,
stated in its native vocabulary rather than derived after the fact as a workaround for two crates that
"don't fold."

Under this reading, the ladder is not a chain but a **fork at Magma/Semigroup**: one branch toward
cancellative structure (group, abelian group, ring, field, needing inverses, which arvo's `Signed`
numerals have as a representation but which no observed consumer's *law* actually needs, see section
5), the other toward idempotent/ordered structure (semilattice, bounded semilattice, ordered monoid,
lattice-ordered monoid, dioid), with the sign-dependent monotone-versus-associative split of section 3
explaining exactly why arvo's own arithmetic sits differently on the two branches depending on `Sign`
and `Growth`.

### Reading two: do not name a fixed set of rungs at all, derive them

The draft's own Thread C mechanism (section 3.4, "each `Resolution` constructor states its own lemmas
as associated truth-valued members, a type-level fold combines them") already solved a version of this
problem for `AddAssoc`, and solved it precisely because naming a structure by declaring a bespoke impl
per branch hit a coherence ceiling: "as soon as a third true fact needs stating... Rust's coherence
check refuses all three as conflicting" (draft, section 3.4). That is not a `AddAssoc`-specific
accident. It is what happens every time a nominal structure is asked to be true of some instances and
false of others under a shared blanket impl, and it is exactly the shape section 3 of this file just
demonstrated for the whole fact space: `Hot` is associative and not monotone; unsigned `Warm`/`Cold`
is both; signed `Warm`/`Cold` is monotone and not associative; float is commutative and has an
identity and is neither associative nor (near `NaN`) fully monotone; `Precise` is neither, being
partial rather than total. **No single named structure is true of every composition arvo ships**, and
the draft's own habit ("a structure is a magma plus the laws it happens to satisfy," section 3.7) is
already the right instinct; it has just not been carried past `Magma` and `AddAssoc`.

The extension is mechanical and, on the evidence of Thread C, cheap: treat associativity,
commutativity, has-identity, idempotence, distributes-over-`Op2`, and monotone-with-respect-to-`Ord`
each as its own atomic, independently-derived fact (a `[const]` boolean or a marker trait keyed on
`Op` or on `(Op1, Op2)` or on `(Op, Ord)`), computed by the exact fold-of-lemmas mechanism the draft
already built and the fifth pass of Thread C already proved compiles to the identical machine-code
symbol as a hand-written baseline (draft section 4.3, "the checked reference path and a hand-written
baseline... the compiler proved them identical and emitted the same machine-code symbol for both").
Named structures (`Semigroup<Op>`, `Monoid<Op>`, `Semiring<Add, Mul>`, `Dioid<Join, Mul>`, whatever the
consumer-facing vocabulary settles on) become **derived, zero-cost blanket impls over conjunctions of
those atoms**, `impl<T> Monoid<Op> for T where T: Semigroup<Op> + HasIdentity<Op> {}`, rather than
primary declarations each carrying its own sketch-and-bench obligation.

This is not a narrowing of the ladder; every named structure the mathematics has, on either branch,
including ones nobody has written down yet (`OrderedMonoid<Op, Ord>`, `LatticeOrderedGroup<Op, Ord>`)
remains expressible, and remains "declared to the depth the theory goes" in the sense D47 asks for. It
changes *how many independent proofs D47's obligation costs*: today, adding a new named rung means a
new sketch and a new bench for that rung specifically. Under atomic facts, the sketch-and-bench
obligation is paid once per atom, and every future named structure built from already-proven atoms
inherits the proof for free, which is precisely the efficiency the prior-art pass found no evidence
either way for (`04_algebraic_structure_hierarchies.md:300-310`, "this pass found no measurement... of
how much compile time or binary size a `Semiring`/`Ring`/`Field`-depth trait hierarchy actually costs
versus a `Monoid`-depth one"). I did not bench this; per `bench-and-sketch-discipline.md` a compile-
time or binary-size claim belongs in `mock/benches/`, and none exists yet for this question on either
side. What Thread C's fifth pass already benched (draft section 4.3) is the *narrower* claim that one
checked-and-executed definition costs nothing over a hand-written baseline, which is evidence for the
atomic mechanism's cost being low, not proof that the whole hierarchy built on it is.

Coherence is worth being precise about, because it is where the draft's actual, measured ceiling was.
Marker traits with no associated items (which every one of the atomic facts above is) do not collide
under multiple satisfied bounds the way the `Resolution`-conditioned `Combine<Additive>` impls did:
`T: Associative<Op> + Commutative<Op>` being simultaneously true is not an ambiguity, because nothing
downstream has to *choose* which impl fired; it is a conjunction, not competing evidence. The ceiling
the draft hit (three mutually exclusive, none-more-specific impls, refused by coherence) is a symptom
of naming the structure primarily and writing one impl per case; it does not recur once the structure
is a derived conjunction of independently-true atoms.

### Reading three: keep both, and say which does which job

Reading one and reading two are not in tension; they answer different questions the brief asks
separately. "Are the laws properties of an operation, of a pair, or of a structure": on the evidence
above, **associativity, commutativity, has-identity, and idempotence are properties of a single
operation** (parameterised by `Op`, the shape `Identity<Op>` already has, `identity.rs:51-54`);
**distributes-over and monotone-with-respect-to are properties of a pair**, an operation against
either a second operation or an order (`DistributesOver<Op1, Op2>`, `Monotone<Op, Ord>`); **named
structures are properties of a conjunction**, derived rather than primary. "Whether the design's habit
of naming a structure by its laws survives contact with types that satisfy laws only partially": no,
not as a primary nominal commitment, which is exactly why the draft's own Thread C had to stop
declaring `Combine<Additive>` blanket impls and start folding lemmas; it survives, but only in the
derived-from-atoms form, and that form is what should also carry `Monotone` and the ordered-algebra
branch, not a bespoke mechanism invented a second time.

Section 4.1's diagnostic finding (draft, "ten axes render for free... provided every value a consumer
can select is reached through a *named* type rather than a raw structural parameter list") is a real
argument for keeping nominal names around at the consumer-facing layer even once the proof mechanism
is atomic: an error naming `T: Monotone<Additive>` unsatisfied is more useful than one naming a raw
conjunction, exactly the way the draft already prefers named modifier types over exposing every axis.
Nothing here argues against nominal vocabulary as sugar; it argues against nominal vocabulary as the
place the proof burden lives.

## 5. Where ring and field probably do not belong, and where they might

No composition `Number<N, S>` that carries a `Policy` can be an exact ring or field, for a structural
reason rather than an implementation gap. A field requires every nonzero element to have an exact
multiplicative inverse; a finite-width numeral cannot represent one third exactly at any width (the
draft already says this about division, section 5.1: "the exact quotient of two representable values
is generally not itself expressible at any finite width"). A ring requires exact distributivity of `*`
over `+`; any numeral with a `Quantisation` that is not the identity breaks this by construction, the
same way it breaks addition's associativity. **The only numerals for which ring or field talk is exact
are unbounded ones with `Growth::Exact` and no quantisation ever firing**, and arvo does not ship those
as `Number<N, S>` compositions; it ships `arvo-num-systems`'s value-set membership instead (the
untouched crate in the draft's section 1 table, "membership defined through algebraic structure, by
inhabitance not equality"). That crate's `Q` (the dyadic or general rationals) genuinely is a field, as
a mathematical set. No `Number<N, S>` instance can hold all of it.

This suggests ring and field vocabulary belongs primarily as a property of **the number system a
numeral's values are drawn from**, in `arvo-num-systems`, rather than as a property any bounded, policy-
bearing composition could itself claim, which would resolve part of the draft's own open question
(section 5.3: "whether `arvo-num-systems` now depends on this design's format concept or the reverse").
I hold this as a direction, not a finding; I have not read `arvo-num-systems`'s design at all, it is
outside every file the brief named, and the question of what a `Number<N, S>` composition's `*`
actually needs (distributivity over a *bounded* representable set, which is a different and weaker
claim than ring distributivity) is precisely the untested work the draft already flags (section 5.2:
"multiplication... entirely untested... expected to be where the work first gets genuinely hard") and
that I have not touched here either. What multiplication's arrival will need, per section 3's own
logic, is likely an **ordered ring** condition (sign-dependent monotonicity: `x ↦ c·x` is order-
preserving for `c ≥ 0` and order-reversing for `c < 0`, the standard "positive cone" axiom of ordered
rings, Fuchs again), not bare ring distributivity; I flag this as the shape the next multiplication-
focused dive should check against, unverified.

## 6. No default law: the collision with the toolbox rule dissolves the same way twice

McSherry's reading two (section 3 of file 13) and reading three (his section 8's alternative) both
converge, independently of the algebra question, on a fact worth stating plainly because it decides
where any of this actually attaches to a public bound: **none of `arvo-graph`, `arvo-comb`, or
`arvo-spectral` needs a law by default.** `upward_rank` and `longest_path` never regroup (section 1);
`matrix_chain_dp` never regroups either, its recursion structure is pinned by the interval DP, not by
an associative fold; `power_iteration`'s sum is a single fixed left-to-right walk, never chunked. The
law only becomes load-bearing the moment something **regroups**: a chunked parallel fold, an n-way
unrolled accumulator (file 13's bench, `13_mcsherry...md:364-396`, measured this exact regrouping worth
roughly 2x on a single thread inside arvo's own licensed internals), or a future tropical *matrix
product* (all-pairs shortest path via repeated squaring, the standard use of a full dioid, which would
for the first time chain multiple `⊗` applications and would need `⊗` associativity and identity as
well as `⊕`'s properties, a strictly stronger requirement than anything currently shipped needs).

So the practical rule this dive supports, independent of which of the three readings in section 4 the
next round adopts: **no arvo trait bound gates ordinary usage on an algebraic law by default.** A law
is required only by the specific combinator that performs the regrouping (a chunked-reduce, an n-way
accumulator split, a tropical matrix-power routine), which states the fact it needs
(`W: Monotone<Additive> + HasIdentity<Additive>` for a max-plus reduce combinator, `W: Associative<Add>
+ HasIdentity<Add>` for a chunked accumulator), and everything that does not regroup states nothing and
refuses nothing. This is `arvo-toolbox-not-policer.md`'s "no hardcoded limits, anywhere" applied to the
algebra question directly, and it means the collision file 12 raised and file 13 relocated does not
need relocating a second time between arvo's own crates: the fix is that the bound never sat on the
consumer in the first place, it sits on the combinator, wherever that combinator ends up living.

## 7. Cost, under the constraints actually in force

No `generic_const_exprs`, no full `specialization`; `min_generic_const_args`, `min_specialization`,
`adt_const_params`, and the `const_trait_impl` family are available; monomorphisation is the only
dispatch; `#![no_std]`, no `alloc`.

The atomic-fact mechanism in reading two is not a new unstable-feature dependency. It reuses the exact
shape Thread C already verified: a `[const]` generic function checked exhaustively at a small model
width and executed, unmodified, at the real width, with the lemma-fold combining boolean associated
constants. Widening the fact set (adding `Monotone<Op, Ord>`, `DistributesOver<Op1, Op2>`,
`Idempotent<Op>` alongside the existing `Associative`/`Commutative`/`HasIdentity` shape) is more
instances of a mechanism already proven sound and, per section 4.3's own measurement, free at runtime;
it introduces no new feature-gate risk beyond what `AddAssoc` already carries.

Marker-trait conjunctions carry no coherence risk of the kind the draft's `Combine<Additive>` per-
`Resolution` impls hit, because nothing downstream needs to pick a winner among overlapping impls; a
type either satisfies a given atomic fact's blanket condition or it does not, and multiple facts being
simultaneously true is ordinary trait-bound composition, not competing evidence. This is, if anything,
cheaper to typecheck than the current per-`Resolution`-constructor macro table the draft already
navigated around a coherence ceiling to reach.

One naming collision worth a member noticing before it bites: `arvo-strategy/src/axes.rs:88` already
declares `pub struct Min;` as a `ContainerWidth` marker (the minimum-byte-aligned-width `Lowering`
axis), unrelated to an operation marker for `Identity<Min>`/`Combine<Min>`. Anything naming `Min`/`Max`
as operation markers for the semilattice branch (section 4's `Identity<Min>`, gestured at in
`identity.rs:48-50`'s own doc comment as a planned extension, and independently found relevant by
`04_algebraic_structure_hierarchies.md:259-271`'s CRDT/join-semilattice citation) needs a distinct name
or a distinct module, `MinOp`/`MaxOp` or similar; the collision is a naming detail, not a design
question, but it will produce a confusing error the first time someone writes `Identity<Min>` and gets
the wrong `Min`.

## 8. What I would flag for the next member, unresolved

**Whether `Monotone` should unify with the draft's existing, differently-scoped use of the same word.**
Draft section 5.1 already has "the one shipped `Monotone` law implementation only covers the
'nearest, with some tie rule' family of rounding rows," which is monotonicity of a *quantisation
function* with respect to its single argument, a different function from the one this file's
`Monotone<Additive, TotalOrd>` tests (order-preservation of translation by a constant). Both are
instances of "this function is order-preserving," and a single `Monotone<F, Ord>` general enough to
cover a unary resolution function and a binary operation's partial application might be one trait
rather than two concepts sharing a name by accident. I have not checked whether the mechanics actually
unify; it is worth someone trying before two separate `Monotone` traits ship under the same name for
different arities.

**Whether the atomic-fact mechanism actually costs what section 7 argues it costs**, rather than what
I argued it costs. Nothing here is a bench. The prior-art pass already flagged this exact absence
(`04_algebraic_structure_hierarchies.md:300-310`) two days before the current draft, and it is still
absent; per `bench-and-sketch-discipline.md` this belongs in `mock/benches/` as its own piece of work,
comparing a `Semiring`/`Dioid`-depth atomic hierarchy against a `Monoid`-depth one on arvo's actual
combinatorial shape (widths × strategies × signs), not argued from Thread C's narrower, already-
measured claim.

**`arvo-sparse`'s `dm.rs` and `block.rs`**, which I did not read, and which section 1 flags as a
possible fifth shape (Boolean semiring, reachability and block structure) on no more than a guess from
the crate's stated purpose.

**Whether a consumer-facing named vocabulary (`Dioid<Join, Mul>`, `OrderedMonoid<Op, Ord>`) is worth
designing now or is premature relative to `arvo-graph`/`arvo-comb` only needing the single atomic fact
`Monotone<Additive, TotalOrd>` today.** Reading three of section 4 argues both readings compose; it
does not argue for building the full named vocabulary before a second consumer needs it. D47 says
depth is not truncated to what a named consumer forces; whether that licenses building `Dioid` before
`arvo-graph`'s DAG algorithms are the only thing asking for it, or whether the atomic fact alone
already satisfies "as deep as the theory does" for now with the named sugar following once the pattern
repeats, is a judgment call for whoever writes the next changelist, not one I am settling here.

**Multiplication and the ordered-ring / positive-cone shape flagged in section 5.** Entirely untested
by anyone in either dive so far, and per the draft's own section 5.2 "expected to be where the work
first gets genuinely hard." I would want that checked against the same discipline this file applied to
addition, run the exhaustive small-model probe first, before assuming distributivity-over-max-or-min
generalises to `*` the way it did for `+`; sign-dependent order reversal under multiplication is a
genuinely different mechanism from anything addition does, and nothing here establishes it behaves the
same way.
