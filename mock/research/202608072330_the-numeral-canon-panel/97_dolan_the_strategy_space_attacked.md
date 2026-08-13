# 97. The strategy space attacked

**Predecessors:** `93_orchard_the_strategy_axis_derived_cold.md` and
`94_wingo_the_strategy_axis_derived_cold.md`, the unit's cold pair. **Probes:** `97_probes/`, eleven of
them, each committed as it ran.

This is an attacker file in the second half of a 4-4-1, so op's `95` governs its shape: attack is the
floor and the file has to end with something the consolidation can state. Where I break a claim I say
what replaces it, and where a claim survives I take it further and carry it forward.

## 0. The gates

**Canon gate: passed.** Checked against `INTENTS.md` I1 through I17. The assigned question is licensed
by I1, which op demoted to open in his own words: "the strategy set is not closed at exactly four"
(`INTENTS.md:56`). I17 makes the count explicitly beside the point of the intent it carries
(`INTENTS.md:327`). Nothing in the intents forecloses asking what a strategy is, and nothing I was asked
to do conflicts with one. I found no misalignment and nothing to hand back on this gate.

**Test gate: passed, with one finding and a correction to a neighbour's account of it.**

There is no arvo suite. `cargo test --manifest-path mock/Cargo.toml` errors with "the manifest is
virtual, and the workspace has no members", which is the intended state and not a defect. So I gated the
only executable surface my question touches, the bench variant crates, and ran them myself rather than
taking `93`'s and `94`'s word for it, because two unratified files agreeing is shared drift rather than
corroboration. Command and output at `97_probes/test_gate.out`: eleven crates, **96 tests, all green.**

I read bodies rather than names. The strategy-relevant suites are genuinely strong and saying so is a
result. `satfold-shared` decides its associativity law exhaustively over `1 << 24` triples and carries
three deliberately wrong kernels as negative controls, with the reasoning stated in its own doc comment
("The agreement above is worthless if the arms could not have disagreed"). `warm-clamp-shared` checks
against an independent `u128` oracle at every declared key and perturbs a bit inside a chunk that does
not clamp to assert the answer moves. Those are not decorative.

**The one finding, and it is milder than `94`'s account of it.** `94` section 0 calls `bitpack-shared`'s
three tests "a sample", three sizes of one shape. Read against the packing, they are not a sample, they
are a **redundancy**: the column packs 13-bit values, so index `i` sits at bit offset `13i` and every one
of the eight byte phases is exercised within the first eight indices. At `N = 256` the extraction path
is already fully covered on the property those tests assert, and `N = 4096` and `N = 16384` add cache
residency, which matters to the bench and not to the test. So two of the three tests establish nothing
the first does not. That is `the-test-gate.md`'s "redundant tests" rather than its "sampled laws", and it
disqualifies nothing.

**And the gate has a hole nobody's suite can close, which `96` already established.** Fourteen bench
variant crates define a `validate_output` the harness has never invoked, because the harness gated the
per-variant validator on `outputs_may_differ` and exactly one crate declares it. The fix is upstream in
`hiisi-digital/mockspace` PR #18 and arvo's pin has not moved. I did not rest anything on a per-variant
validator, and neither should the consolidation until the pin bumps.

**I applied the same standard to my own probes.** P4's 4661 compile-time assertions, over 256 ordered
pairs, 4096 ordered triples and 16 elements, are shown capable of failing by a committed mutant that
drops one coordinate of the union and does not compile (`97_probes/p4_demand_lattice.out`,
`error[E0080]`). P2 carries two controls that are predicted to fail and do. A check nobody has seen fail
is not a check.

## 1. The answer, before the working

Three claims, in the order they matter.

**One. `25` section 7's sentence and the cold pair's definitions are not the same proposition, and the
gap is measurable.** A section is any assignment of a mechanism to each region. An argmin under a
weighting is a section that some single weighting explains at every region at once. On committed harness
output, **72 of 15625 sections are rationalisable** by a weighting over the two cost axes that family is
about, counting generously so that a tie admits either arm, and the gap is not a fact about that family: it is polynomial against exponential in the number
of regions. So `94` phase two's move of the claim to TWO EXPERTS is moving two different claims under
one name. What replaces it is in section 2, and it is better than either: the section is the design-tier
artifact, the weighting is the canon-tier justification, and **rationalisability is exactly the "little
bit of option 3" op mixed into his answer**, made checkable.

**Two. "Strategy" is not one thing or two. It is three layers, and the ordering the panel keeps
rediscovering is polarity rather than a stratified decision.** `93`'s circularity dissolves rather than
resolving: an observable coordinate is an **input** to the resolver and an unobservable one is its
**output**, so nothing is decided twice and there is nothing to stratify. That reformulation is smaller
than `93`'s and it makes a different prediction, in section 3.

**Three. There is no coordinate on which a join is the right operation, so `Resolve` as a total silent
join is the wrong mechanism, and `93` section 7's recommendation to keep it is refuted by `93`'s own
P8.** On the objective coordinates the join is union and is free (P4 compiles it, all 256 pairs and all
4096 triples). On the observable mechanism coordinates **no order exists**: P3 finds wrapping and
saturating incomparable in three of four swept configurations, at two widths, and finds that the word
"conservative" names three different orders that disagree. On the unobservable coordinates there is
nothing to resolve. Section 4.

**Four. The disagreement `93` and `94` left located is decided by arvo's own committed harness, in
`94`'s favour.** They asked whether any consumer wants two accumulator widths over one stored column, and
both declined to look. The experiment is in `mock/benches/`: `warm-clamp-arity-w13` holds the declared
width, the element count and the transform fixed and sweeps the fold's arity across six points, and the
best arm moves. Section 5.

And a fifth, from the algebra half, which is the one I would most like carried:

**Five. One structural property decides every law verdict this panel has been measuring one at a time,
at every arity, and it is not a new measurement.** If the realisation map respects an operation, the
representable set is a quotient of exact arithmetic and inherits every identity exact arithmetic has.
P2 tests that prediction against exhaustive sweeps across **552 cells** and finds **zero soundness
mismatches and zero conservative mismatches**. P7 then tests it against `35`'s **660 committed law rows**,
generated independently and months of files earlier by somebody answering a different question, and finds
**659 agreements, one conservative row, and zero soundness mismatches**. So the law table is derivable
rather than measurable, and two of the panel's law findings are consequences rather than independent
facts. Section 6.

## 2. Attack one: is `25` section 7 the same proposition the cold pair reached

### 2.1 What each of the three actually says

`25:528-537` proposes: a strategy is a consumer-written name for one coherent policy, assigning one
value on every axis, each assignment a function of the build condition, so "strategies are therefore
named sections over a product of axes rather than values of a single axis".

`93` section 1: "A strategy is a preference: an ordering over candidate implementations of the same
abstract operation, computable at compile time, defined relative to a cost model whose inputs are also
compile-time available."

`94` section 3.2: "A strategy is a **compile-time choice function over a shared set of arms**, whose
value at each region was decided offline by measurement under a stated weighting."

`94` phase two section 1 lists this among nine independent agreements, and section 1's last bullet says
`25` section 7 "already proposes the canon sentence" and that "the second read it was waiting for
arrived twice, independently, on the same night".

**It did not.** `25` names an element of the function space. `93` names an element of the preference
space. `94` names an element of the function space **with a provenance attached** to it. Those are three
statements about two different objects related by a map, and `40` had already drawn that map before any
of them: `resolve : objective × evidence -> mechanism` (`40:23-28`), with the section as
`section(objective)` and the objective as the primitive (`40:209-211`).

So the question that decides whether they are the same proposition is whether `section` is injective,
and whether it is surjective onto the sections.

### 2.2 It is not surjective, and the shortfall is the whole content of the disagreement

Not every function from region to mechanism is an argmin of a weighting. The ones that are, are the
**rationalisable** sections: some single weighting explains every choice at once. This is the revealed
preference question in its ordinary form, and for a linear weighting it is a linear feasibility problem
in the weight vector, so it is decidable rather than a matter of taste.

`97_probes/p1_rationalisable_sections.py` decides it on committed harness output. Six record counts from
`mock/benches/bitpack-carrier-width_n*.csv`, five arms, cost in two dimensions both of which come from
the repository: median `algo_ns` divided by the record count, and bits per element, which is a static
fact about the arm rather than a measurement. I ran no benchmark; I read one.

```
regions            : 6
arms per region    : 5
SECTIONS, all      : 5^6 = 15625
tie directions     : 29
SECTIONS, rationalisable by a non-negative weighting : 9
ratio              : 9 / 15625 = 0.057600%
```

**Two of the five arms are selected by no weighting anywhere**, because they are Pareto-dominated on
both axes in every region: `bitpack-carrier-d64` and `bitpack-carrier-packed`. Under `25`'s definition
those are perfectly nameable mechanisms a section may assign. Under the cold pair's they are
unreachable. That is a second, structurally different way the definitions come apart, and it needs no
direction sweep, so it does not inherit the thin-band fragility of the count above.

**Nine is the strict count and it is not the number to quote.** P1 asks which sections arise as *the*
argmin under some direction, breaking ties deterministically. The honest question is which are *an*
argmin, because where two arms cost the same under a weighting the design genuinely may take either.
`97_probes/p9_the_decider.py` answers that one exactly, and gets **72**.

The two reconcile completely, which is worth more than either number alone. `packed` and `packed-simd`
carry the same 13 bits, so a weighting that ignores time cannot tell them apart, and every one of the
`2^6 = 64` mixtures of the two across six regions is weakly admissible. Nine strict sections, one of
which is already in that 64. The union is 72, and the probe checks that the union **is** the decider's
set rather than merely matching its size:

```
  mixtures of those across 6 regions       : 2^6 = 64
  strict sections, recomputed here         : 9
  in both sets                             : 1
  union                                    : 72
  decider's count                          : 72
  the union IS the decider's set           : True
```

**So the figure to carry is 72 of 15625, which is 0.461%.** It is the conservative one and it is still
three orders below the section count.

And the count is robust. Several of the nine strict bands are razor thin, which is why P1 carries a
sensitivity pass:

```
  tolerance   0.0%  rationalisable sections:      9 of 15625  (0.0576%)
  tolerance   1.0%  rationalisable sections:    134 of 15625  (0.8576%)
  tolerance   5.0%  rationalisable sections:    744 of 15625  (4.7616%)
  tolerance  10.0%  rationalisable sections:    760 of 15625  (4.8640%)
```

At a ten percent tolerance, far past any noise floor in that family, the admissible sections are still
under five percent of the sections.

### 2.3 And the gap is structural, not a fact about that bench family

One instance decides nothing, so `97_probes/p1b_the_gap_is_structural.py` establishes the same gap
without using the data. As the weighting varies, the argmin in one region changes only where two arms
tie, and each tie is a hyperplane through the origin in weight space. The achievable sections are the
cells of an arrangement of at most `|E| * C(|A|, 2)` hyperplanes in `k` dimensions, which is at most
`sum(C(H, i) for i < k)` cells: **polynomial in the number of regions, of degree `k - 1`, against
`|A|^|E|` sections.**

```
 regions   arms       sections      bound   max seen  mean seen
       6      5          15625         61         12       6.18
       8      4          65536         49         12       7.13
      12      6     2176782336        181         18      12.08
```

Four hundred random cost tables per row, exact enumeration, no table exceeded the bound. So the
committed data is one instance of a gap the shape of the question forces, and adding regions widens it
without bound.

### 2.4 What replaces the collapsed agreement, and it is better than any of the three

**The rationalisability constraint is what op's `88` answer is made of, and nobody has named it.**

Op was choosing among a preset naming a point in a space of axes (option 1), an irreducible identity
(option 2), and nothing but a weighting (option 3). He answered "Mostly option 1, but a little bit of
option 3 with it. Hard to put into words, hopefully you get my meaning here" (`88:20-21`), and `88`
records that a later expert finding the two readings pull apart has found something real.

Here is where I think they pull apart and, more usefully, where they join.

**The design tier writes points.** `94` section 3.2 is right that a const function cannot measure
anything, so the objective is evaluated offline by a person and what reaches the compiler is a table.
That table is a section. Option 1, and it is most of it.

**The canon tier writes the objective.** `40` section 3.2's permanence argument: a canon stating the
table is wrong the next time somebody measures, and a canon stating the objective is not. Option 3, and
it is the smaller part.

**The little bit of option 3 that survives into the design tier is one checkable constraint: the table
must be rationalisable.** That is not decoration and it is not a restatement. It is the difference
between a strategy's name being predictive at regions nobody measured and it being a lookup key. If one
weighting explains the whole table, then a consumer reading "this one weighs time heavily" can predict
what it will do at a region nobody has benched, and a later expert adding a region has a rule for filling
it in. If no weighting explains the table, the name carries no information beyond the rows already
written, and every new region is a fresh decision with nothing to derive it from.

**Nothing currently checks this, and nothing in the panel has named it.** `94`'s W1 bakes the winner per
region as an associated const, which is what a table of measured results naturally becomes, and a
hand-written table of that shape is free to be irrational in the technical sense.

**And the check is a decision procedure rather than a nice idea**, which is what
`97_probes/p9_the_decider.py` establishes. The question is homogeneous linear feasibility: is there a
non-negative non-zero `w` with `w . cost(chosen, e) <= w . cost(a, e)` for every region and every arm.
The feasible set sits inside the non-negative orthant so it is pointed, so it is non-trivial exactly when
it has an extreme ray, and every extreme ray lies on `k - 1` independent tight constraints. The decider
enumerates those in exact rational arithmetic, with no sampling, no tolerance and no floating point
comparison, at any number of cost dimensions rather than only at two. It answers four constructed
three-dimensional cases correctly, including one whose expected answer was mine and wrong.

It runs **where a table is written**, once, offline. Not at compile time and not at run time. That is the
whole cost of keeping it, which is the bound `small-wins-compound-into-the-program.md` puts on taking a
win.

**And it is cheap to keep**, which is the bound `small-wins-compound-into-the-program.md` puts on taking
a win. The check runs where the table is written, not where it is used, so it costs no instruction and
no monomorphisation.

### 2.5 What this does to the rung, stated plainly

`94` phase two records the sections-over-a-product claim as reaching TWO EXPERTS on the night, with `25`
as a third and earlier instance. **That reading does not survive.** `25` proposed a section. `93` and
`94` derived a preference and a choice-function-with-provenance respectively. The three agree that the
object is not a value of one axis, which is a real agreement and is the part that should carry the rung.
They do not agree on what the object is, and the difference is 0.058 percent of the sections on the one
family where it has been counted.

**F-A. `25` section 7's "named sections over a product of axes" and the cold pair's argmin definitions
are different propositions, and the set of sections a weighting can produce is a vanishing fraction of
the sections: 72 of 15625 counting ties generously, 9 counting strictly.**
`holds for: regions = 6, arms = 5, cost dimensions = 2, cost source = committed
bitpack-carrier-width_n* medians and declared bits per element, tolerance in {0, 1, 2, 5, 10}%,
threads = 1, target features any`
Evidence: `97_probes/p1_rationalisable_sections.py` by exact direction sweep and
`97_probes/p9_the_decider.py` by exact cone feasibility, two methods sharing only their input, reconciled
as a set identity rather than as a matching count.

**F-A2. Rationalisability is decidable exactly, at any number of cost dimensions, by enumerating the
extreme rays of a pointed cone in rational arithmetic.**
`holds for: cost dimensions in {2, 3}, arms in {3, 5}, regions in {2, 6}, arithmetic exact rational`
Evidence: `97_probes/p9_the_decider.py`, agreeing with the independent sweep on the committed data and
answering four constructed cases correctly.

**F-B. The gap is polynomial against exponential in the number of regions, so it is a property of what
a weighting is rather than of any dataset.**
`holds for: regions in 2..20, arms in {4, 5, 6}, cost dimensions in {2, 3}, cost tables arbitrary`
Evidence: `97_probes/p1b_the_gap_is_structural.py`, a counting bound plus exact enumeration over 2000
random cost tables with zero violations.

## 3. Attack two: is "strategy" one thing or two

`93` section 3 splits into a policy layer that changes the answer and a lowering layer that does not, and
argues the two are **ordered**, with the order forced: "the policy layer must be decided first, because
it *defines the space* the lowering layer optimises within". `94` phase two withdrew its own three-way
split and adopted this.

Two things about that. The split is right and I keep it. **The forcing argument is not, and what is
actually going on is smaller.**

### 3.1 The circularity `93` resolves does not exist

`93` states it as an apparent circularity: the preference ranks arms by cost, the arm set depends on
which rewrites are legal, legality depends on the laws, the laws depend on the policy the preference was
supposed to choose. It then resolves it by stratification.

The premise is wrong at its last step. **The preference was never supposed to choose the policy.** A
policy coordinate is observable, meaning moving it changes the value the program computes (`40:398`),
and `40` section 5.4 is the compile that shows what happens if a build arm moves one: the same consumer
source computes different answers under two arms. A coordinate the consumer must be told about is a
coordinate the consumer supplies. So it is an input to the resolver, not an output of it.

Written as a type rather than as a phase ordering:

```
Arms    :  Policy -> Set
resolve :  (p : Policy) -> Evidence -> Arms(p)
```

That is a dependent function, and its dependency structure is the entire content of `93`'s "ordering".
Nothing is decided first because nothing is decided twice. There is no circularity to break and no
stratification to force.

### 3.2 The distinction that actually does the work is polarity

An observable coordinate appears in **input** position: the consumer writes it, and every consumer of a
value must agree about it, because they cannot recover it from the bits. An unobservable coordinate
appears in **output** position: the resolver produces it, it may differ per call site, per build and per
target, and no consumer can observe that it did.

That single distinction reproduces every consequence `93` and `94` drew, and it drops the argument that
carried them:

- `93` section 5's "the type parameter must name the preference, never the resolution", and its
  observation that a marker meaning "aligned and vectorised" is a lie in the type on a target with no
  vector unit. That is the output half.
- `94`'s W9, policy on the value and plan at the site, with one value type folding three ways at three
  sites and zero casts. That is both halves at once.
- `93` section 7's "the lowering layer needs no resolution at all, because it is unobservable and each
  operation can pick its own optimum". That is the output half again, and it is right.
- `40` section 6.4's finding that an unobservable axis's resolution depends on an observable axis's
  value, five of six widths at two percent tolerance. Under polarity that is not a surprise needing an
  ordering rule: it is the resolver reading its inputs.

**And it makes a prediction the stratification reading does not.** Under stratification, a policy
coordinate is decided first, which leaves open that it might be decided by measurement as long as it is
decided early. Under polarity, a policy coordinate can never be decided by measurement at all, in any
phase, because doing so makes the program's answer a function of a benchmark. That is `40` section 5.4's
arm swap stated as a structural impossibility rather than as a caution.

### 3.3 So the answer to "one thing or two" is three, and they are not the same kind of thing

**Objectives.** What the consumer wants weighed. Consumer-supplied, and the mixing question has an
answer: union. Section 4.1.

**Observable mechanism coordinates.** Overflow policy, rounding, and whatever else changes the computed
value. Consumer-supplied, because nobody else can supply them. The mixing question has no join.
Section 4.2.

**Unobservable mechanism coordinates.** Headroom, layout, lane count, instruction selection.
Resolver-produced. There is no mixing question at all. Section 4.4.

`94` phase one had three components, `93` had two layers, `94` phase two collapsed to `93`'s two. The
right count is three, and the reason the panel kept getting two is that the first and second are both
consumer-supplied and therefore look like one thing from the carrier's point of view, while they behave
completely differently when two of them meet.

**And I will name the shape of the error, because it is the third instance in this panel.** Asking "what
relates two strategies" is asking one question of three layers with three different answers, which is
`never-ask-which-single-rule-governs.md` arriving from inside the panel rather than from a dispatch to
op. `88` section 4 records op rejecting this shape for the third time and saying so in those terms.

**F-C. The observable-or-not classification is the polarity of the coordinate, and it determines both
who supplies it and whether a mixing question exists for it.**
This is a derivation from `40:398`'s definition and `40` section 5.4's compile, not a measurement, and
it is labelled as one. It carries no predicate because it is not a finding about arvo's arithmetic.

## 4. Attack three: the order structure, and what relates two strategies

### 4.1 On the objectives, the join is union, and `93`'s own probe said so before `93`'s prose priced it

`93`'s P1b Part B reports that four markers carrying one demand each leave 12 of 16 ordered pairs
unresolvable, and that the smallest set closed under the members' own resolution has 15 elements.
`93` section 4 lists carrying that closure as response (a) and prices it: "the space is larger than
four, so every 'what does this strategy do' question needs an answer parameterised by axis rather than
looked up per marker" (`93:384-386`). Section 8 carries the cost framing forward, and section 17 repeats
it in the option list.

**Fifteen is `2^4 - 1`.** The closure of `d` one-demand generators under union is the free join
semilattice on `d` generators, whose carrier is the non-empty subsets. It is not a set anybody has to
hold. Four generators are named and every other element is a formal join of them, exactly the way a
formal union type is not a new named type.

`97_probes/p4b_the_closure_is_free.py` reproduces both of `93`'s numbers from an independently written
enumeration and identifies the object:

```
d = 4 generators: speed, residency, accuracy, familiarity
  ordered pairs of generators                : 16
  pairs whose union is NOT itself a generator: 12
  closure under union                        : 15 elements
  2^d - 1                                    : 15
  identical                                  : True
  ordered pairs unresolvable INSIDE the closure: 0 of 225
  names a design must write down             : 4 (the generators)
```

**And `93`'s own probe output already said this**, in its closing lines: "The powerset is closed by
construction, which is the point: closure is not something a marker set can be given, it is what a
LATTICE is. A named preset is then a point in it and naming more costs nothing structural." The finding
was in hand and the file that produced it priced it as a cost anyway. That is a compression loss inside
one file between its probe and its prose, and it is worth naming because the option register inherited
the prose rather than the probe.

**It compiles on the pin and the join is total and lawful.**
`97_probes/p4_demand_lattice.rs`, `#![no_std]`, **zero feature gates**, no `dyn`, no `TypeId`, no
`generic_const_exprs`. The join is one blanket impl carrying an associated type, which is
`a-refused-bound-wants-a-trait-not-a-feature.md` applied before the wall rather than after. Asserted at
compile time:

- **all 256 ordered pairs**: the join's demand set is the union of the two, and the join commutes;
- **all 4096 ordered triples**: the join associates;
- **all 16 elements**: the join is idempotent, and the empty demand set is its identity.

An earlier revision of that file asserted associativity over a sample of five triples with a written
argument that the pair table implied the rest. The argument is sound and the sample is still exactly
what `the-test-gate.md` names, so it was replaced with the whole table. The mutant confirms the
assertions can fail: dropping one coordinate of the union gives `error[E0080]: evaluation panicked: the
join of two demand sets is not their union`.

**And a computed demand costs nothing at the point of use, which is the half `93`'s and `94`'s erasure
probes do not cover.** Every one of them selects on a demand or a region written literally at the site.
The whole of this section proposes that the demand set a mixed expression carries is the **output** of a
join rather than a name anybody wrote, so if reading a computed type costs anything the solver cannot
fold, the generated lattice has a price at every mixed site.

`97_probes/p8_does_a_computed_demand_erase.rs` compares five entry points at `-O` on
aarch64-apple-darwin, with the both-demands point given its own distinct arm so a join that collapsed
onto a generator would land on a different symbol:

```
entry_joined vs entry_handwritten    identical body   computed demand against a direct call
entry_joined vs entry_speed          different        must not collapse onto its first operand
entry_joined vs entry_residency      different        nor onto its second
entry_joined vs entry_different      different        nor onto an unrelated arm

_entry_joined:
	b	...arm_both
```

One instruction. The computed demand reaches its arm exactly as a hand-written call does, and differs
from all three negative controls. A compile-time assertion pins that the join really lands on the
both-demands point, and a mutant that makes the join drop a coordinate fails that assertion with
`error[E0080]` rather than silently making the comparison vacuous.

**Two things this buys that the flat set cannot.**

*Silence is a first-class element.* `40`'s p2 finds that every one of the four named strategies is silent
on exactly one axis, so the number of points a consumer can request by writing a name is **0 of 16**
(`40:486-492`). Under a total-assignment product that is sixteen cells to fill. Under a demand set,
absence of a demand is the statement "the consumer asked nothing here and the resolver is free", which
is `40` section 6.3's reading made structural instead of interpretive.

*The escalation pathology disappears.* `93`'s F4 reports that all four admissible tables on the flat set
"make Precise the top and escalate every mixed expression to it", and calls the cost "Nobody asked and
everybody pays" (`93:354-357`). Under the demand lattice, the join of a speed demand and a residency
demand is the element demanding both, and P4 asserts at compile time that it is not the accuracy demand
and has lost neither operand's. Nobody asks and nobody pays.

**F-L. A selector reading a demand set computed by the type-level join emits one tail branch, identical
to a hand-written call to the arm it resolves to, and distinct from the arms either operand resolves to.**
`holds for: coordinates = 2, arms = 4, target = aarch64-apple-darwin, rustc 1.98.0-nightly (57d06900f),
edition 2024, opt-level 3, panic = abort, feature gates = 0, no_std, threads any (a compile-time
artifact), target features baseline`
Evidence: `97_probes/p8_does_a_computed_demand_erase.rs` with `97_probes/p8_mutant_join_collapses.rs` as
the negative control. This is a shape, not a price: the compile-time cost of the lattice is unpriced and
is said to be unpriced.

**F-D. The closure `93` prices as a cost is the free join semilattice on its own generators, so it is
generated rather than enumerated: `d` names, `2^d - 1` elements, zero unresolvable pairs, and a total
lawful join in one impl.**
`holds for: d in {3, 4, 5} for the counting, d = 4 for the compile, rustc 1.98.0-nightly (57d06900f),
edition 2024, feature gates = 0, no_std, threads any (a compile-time artifact), target features any`
Evidence: `97_probes/p4b_the_closure_is_free.py`, `97_probes/p4_demand_lattice.rs` with
`97_probes/p4_mutant_dropped_demand.rs` as the negative control.

### 4.2 On the observable coordinates there is no join, and "conservative" names three orders

`mock/DESIGN.md.tmpl:43` names `Resolve<S1, S2>` and `arvo-toolbox-not-policer.md:76` gives its intended
behaviour: "Cross-strategy binary op where overflow policies disagree (`Hot wrapping + Precise saturating
→ Precise`)". `93` section 7 keeps it, "provided it is a componentwise join over the **policy layer
only**" (`93:632-633`).

A join needs an order. I went looking for it and there are three candidates, all reasonable, and they
disagree. `97_probes/p3_does_a_conservatism_order_exist.py` computes all three exhaustively over the
whole representable domain, at `W = 5` and `W = 6`, for signed and unsigned, at `F = 0` and `F = 1`,
over an inventory that deliberately mixes an algebraic family with an order family so a ladder cannot be
an artifact of picking one family.

**By honoured laws, wrapping and saturating are incomparable in three of the four configurations**, at
both widths. Unsigned at `F = 0`:

```
    wrap       vs saturate   : INCOMPARABLE
        only in wrap     : mul_over_sub
        only in saturate : add_monotone, mul_monotone_nonneg, top_absorbing
```

**By how often the answer is wrong, they are exactly tied**, in every configuration: both are wrong on
precisely the pairs whose exact result is not representable, which is the same set.

**By how far wrong the worst case is, saturating is above wrapping**, in every configuration.

So "resolve toward the more conservative side" has no referent until somebody says which of the three is
meant, and two of the three do not give an order at all.

**And there is a fourth result in that probe that kills a reading of Q41 outright.** Saturating and
exact-in-a-wider-rung are **incomparable** at `F = 0`, both signednesses, both widths:

```
    saturate   vs exact      : INCOMPARABLE
        only in saturate : top_absorbing
        only in exact    : add_assoc, distrib, mul_assoc, mul_over_sub
```

The absorbing top is a law that only a lossy policy has. So the accuracy-first policy is **not** the top
of the law order, and `76`'s conjecture as recorded at `OPTIONS.md` Q41, "Precise at the top and Hot's
honored set a subset of it", cannot hold in the law order however the inventory is chosen, because
exactness destroys a law saturation provides. `40` section 6.2 already named the consumer who needs the
one exactness destroys: a min-plus relaxation stands infinity on the top and needs it to absorb.

### 4.3 Which corrects `93`'s F11, precisely, and it is the second read `93` asked for

`93`'s F11 says Q41's three options are answers on different regions with signedness as the separating
predicate, and its prose reports that for unsigned "the sets nest, saturating honours everything wrapping
does plus monotonicity and top absorption, and Q41's option (a) holds with saturating on top. **That is
`76`'s conjectured direction, established rather than conjectured**" (`93:1042-1044`).

`93`'s P8 inventory is eleven laws and **contains no subtraction at all**: add-comm, add-assoc,
add-ident, mul-comm, mul-assoc, mul-ident, mul-zero, distrib, retract-add, monotone-add, absorb-top.
Adding one law that involves subtraction, distributivity of multiplication over subtraction, breaks the
unsigned nesting at `F = 0`, because wrapping honours it and saturating does not.

**F11 itself is correct and I am not contradicting it.** Its predicate lists `operations {add, mul}` and
`law inventory as listed`, so under I13's notation it claims nothing about subtraction. What does not
survive is the sentence drawn from it, that the unsigned case establishes `76`'s direction. That
conclusion is inventory-dependent, and `93` said so itself: "The one number I would most like checked by
someone else is P8's law inventory. Eleven laws is a choice, and a different eleven could nest
differently" (`93:1178-1180`). It does.

**F-E. The overflow axis carries no conservatism order that is stable across readings. By honoured laws
wrapping and saturating are incomparable in three of four configurations; by frequency of a wrong answer
they are tied in all four; by worst-case magnitude saturating is above wrapping in all four. Saturating
and exact are incomparable at `F = 0`.**
`holds for: W in {5, 6}, F in {0, 1}, signedness in {unsigned, signed}, overflow in {wrap, saturate,
exact}, operations {add, sub, mul}, laws as enumerated in the probe, arity 3 for the algebraic family and
arity 2 for the order family, values exhaustive over the representable domain, threads = 1, target
features any`
Evidence: `97_probes/p3_does_a_conservatism_order_exist.py`.

### 4.4 So what does relate two strategies, stated constructively

Three answers on three regions, which is the shape I13 asks for rather than one policy over a category.

**On the objective coordinates: join, and it is free.** Union of demands. Total, commutative,
associative, idempotent, one impl, silence as the identity, and the closure generated rather than named
(P4, P4b).

**On the observable mechanism coordinates: no join exists, so the operation reports.** Not "refuses for
safety", which `arvo-toolbox-not-policer.md` names as an incorrect shape and which `94` section 6
correctly declined to propose. A report that two operands demand different computed answers and the site
must say which, which is the diagnostic that rule asks for. `93` and `94` converged on the refusal being
two arms rather than one policy; F-E supplies the predicate that separates them, and it is **not** "where
the demands happen to have a join" but the far simpler `observable(coordinate)`, which is static.

**On the unobservable mechanism coordinates: nothing.** Each operation resolves its own, per site, per
build, per target. There is no mixing question, and a design that has one has put an unobservable
coordinate in an input position.

**This is smaller than the alternatives on the table and it subsumes them.** `93`'s four responses in its
section 4 are (a) carry the closure, (b) refuse and require a declaration, (c) the demands are on
different roles, (d) implicitly, keep the flat join. Under the three-layer split, (a) is the objective
layer's answer, (b) is the observable layer's answer, (c) is true and is the observation that the four
names mix layers, and (d) is refuted by F-E. They are not four competing designs. They are the correct
answers to three different questions plus one wrong one, which is why `93` could not choose between them
and reported leaning toward (a) and (c) being the same answer seen from two sides. They are, and the side
they are seen from is the layer.

## 5. Attack four: the disagreement the cold pair left located

`94` phase two section 6 carries three surviving disagreements, and the first is with `93`: whether
intermediate precision, the width an accumulator carries through a fold, sits on the value or at the
site. `93` section 5 says the value. `94` section 3 says the site. Both name the same discriminator,
whether any consumer wants two accumulator widths over one stored column, and **neither ran it**. `93`
declined because both named consumers are pinned to a tier that has been declared dead. `94` thought the
question survives the tier being dead and did not run it either.

**The discriminator does not need a consumer, and arvo's own harness has been carrying it.** `25` section
6.2 found the family and used it for a different question, and reported that no file in the panel cites
it.

`warm-clamp-arity-w13` is six committed runs that hold the declared width at 13 bits, the element count
at 8192 and the transform at the chunked clamping fold, and sweep the fold's arity through 2, 4, 8, 16,
64 and 256. That is decoded from the crate's own key encoding rather than from the title
(`variants/warm-clamp-shared/src/lib.rs:83`), and `97_probes/p6_does_the_accumulator_follow_the_fold.py`
asserts the control is held before reporting anything.

**The stored column is the same shape at every one of the six points. Only the fold moves.** So if the
best arm moves with it, the accumulator's right width is a fact about the fold and is not recoverable
from the value.

```
   arity      acc64     accfit  accfit-dyn       head   min-lanes    minimum
       2      854.4      285.2      4350.2      539.0       320.0      276.9
       4      734.4      294.6      1937.7      565.4       410.6      314.6
       8      612.3      547.3      1171.2      540.0      1127.3      542.5
      16      566.5      293.4       762.5      332.9      1099.8     1091.9
      64      542.3      217.5       286.4      255.6       622.0     7081.5
     256      535.2      202.1       208.3      231.1       454.4     9781.0
```

Contending sets, at three tolerances rather than a strict argmin, because at arity 8 three arms sit
within two percent of each other and a strict argmin there measures the noise:

```
tolerance 2%
  arity    2 : minimum
  arity    4 : accfit
  arity    8 : accfit, head, minimum
  arity   16 : accfit
  arity   64 : accfit
  arity  256 : accfit
  distinct contending sets across the six arities: 3
```

Three distinct sets at zero and two percent, four at five percent. **At arity 2 the best arm gives the
fold no widened accumulator at all and uses the minimum container; from arity 4 upward the best arm sizes
the accumulator from the arity by the design's own interior-safety rule.** One stored column, two
different right answers, decided by something the column does not know.

**So `94` is right and `93` is wrong on this one, and the evidence was in the repository the whole time.**
A design carrying intermediate precision on the value cannot fold one column at two arities without a
cast that changes no value, which is exactly the cost `94` section 7 named and could not price.

**Which comparison answers this, and which does not, because the obvious one does not.** The head-to-head
between the arity-derived accumulator and a fixed 64-bit one has **zero sign changes**: the arity-derived
arm wins at all six arities, by 1.12x to 3.00x. That is a real finding about those two arms and it says
nothing about the carrier, because a rule that always wins could live anywhere. What answers the carrier
question is the whole field moving, and I record the distinction because my own first pass keyed the
verdict on the head-to-head and got the wrong answer out of the right data.

### 5.1 And a second result in the same table, which prices something `94` said was unpriced

`94` section 10 states plainly that "everything about cost in this file is unpriced" and that its
emitted-code observations are shapes rather than measurements. Its W2 is the const-availability finding:
a selection on a const region fact erases to a direct branch, and the identical selection on a runtime
fact emits a compare and a conditional.

The same six runs price it, because `accfit` and `accfit-dyn` are the same accumulator rule with the
arity known at compile time against the arity passed as a runtime value, with everything else held:

```
   arity      arity const    arity runtime        ratio
       2            285.2           4350.2       15.25x
       4            294.6           1937.7        6.58x
       8            547.3           1171.2        2.14x
      16            293.4            762.5        2.60x
      64            217.5            286.4        1.32x
     256            202.1            208.3        1.03x
```

**Knowing the arity at compile time is worth 15x at arity 2 and 1.03x at arity 256**, and the shape is the
one I13 asks for: the win is not uniform, it is concentrated in a region, and the region is nameable. A
short fold pays almost all of its cost in the per-fold decision the const arm erases; a long fold
amortises it. So this is an arm with a predicate rather than a rule about runtime facts in general, and
the predicate is on the arity.

**F-I. At a fixed declared width, element count and transform, the best arm changes with the fold's
arity: the minimum container at arity 2 and the arity-derived accumulator from arity 4 upward.**
`holds for: W = 13, elements = 8192, transform = chunked clamping fold, fold arity in {2, 4, 8, 16, 64,
256}, signedness = unsigned, arms = the six of warm-clamp-arity-w13, tolerance in {0, 2, 5}%, host =
Apple M1, rustc 1.98.0-nightly (57d06900f), harness = mockspace-bench-harness, threads = 1`
Evidence: `97_probes/p6_does_the_accumulator_follow_the_fold.py`, reading committed harness output. The
per-variant validator did not run for this family, per `96`, so the arms are not cross-checked for
computing the same answer by the harness; `warm-clamp-shared`'s own tests do check that against a `u128`
oracle at every declared key.

**F-J. Deriving an accumulator from a compile-time-known fold arity is 15.25x faster than deriving it
from the same arity supplied at runtime at arity 2, falling monotonically in the arity to 1.03x at 256.**
`holds for: the same predicate as F-I`
Evidence: the same probe. This is the priced form of `94`'s W2, which `94` reports as a shape only.

## 6. Attack five: the algebra boundary, and one rule replacing a table

### 6.1 The criterion

`93` measures which laws hold per policy, per `F`, per signedness, per arity. `94`'s probe C measures
retraction and associativity and reports them as independent permissions. `40`'s observable table cites
`35` for six of its eight rows. Every one of these is a law verdict obtained by sweeping.

They are all consequences of one property, and it is not new mathematics.

Let `pi` be the realisation map taking an exact value to a representable one under the declared rounding
and boundary policy. If `pi` respects an operation, meaning that realising an intermediate result changes
nothing, then the representable set with the induced operations is a **quotient** of exact arithmetic. A
quotient inherits every identity its source satisfies. So:

> a law holds in the representable set **iff** it is an identity of exact arithmetic **and** `pi`
> respects every ordered nesting of operations the law contains.

`97_probes/p2_congruence_predicts_the_laws.py` states that rule before running and tests it against
exhaustive sweeps. Twenty-three configurations across signedness, policy, fraction width and rounding
mode, at `W = 4`, `5` and `6`, exhaustive over every pair and every triple, all integer arithmetic:

```
cells                              : 552
SOUNDNESS mismatches (refute)      : 0
conservative mismatches (safe)     : 0
```

Not one law the criterion predicted to hold, fails. Not one it predicted to fail, holds. Two directions
are counted separately on purpose: a soundness mismatch would refute the criterion, and a conservative
one would mean it is safe but pessimistic. Neither occurs.

**What that buys is not a smaller sweep, it is a finite one.** The retraction table has `|ops|^2` entries.
The space of identities is infinite, over every arity. Deciding the finite table decides the infinite
family. `OPTIONS.md`'s own entry on how a chain-level verdict might be lifted lists "(b) A structural
argument about the representation" and says "no file in this panel has asked it of any law it measured".
This is that route in general form, and it is decidable by inspection of a table nobody has to grow.

### 6.2 It refutes `94`'s W4, with a closed form that reproduces `94`'s own number

`94`'s W4: "Retraction and associativity are independent permissions with different regions. A strategy's
licence component is a vector of permissions, not a bit" (`94:420-421`). The evidence is one cell:
wrapping subtraction retracts and does not associate.

Subtraction is not associative **in the integers**. `(a - b) - c` and `a - (b - c)` differ by exactly
`2c`. So under wrapping they agree exactly when `2c` vanishes modulo `2^W`, which is two values out of
`2^W`, and the failure rate is `1 - 2^(1-W)` with no free parameter.
`97_probes/p2b_the_subtraction_control_is_arithmetic.py`:

```
  W       predicted        measured  verdict
  5       93.75000%       93.75000%  exact
  6       96.87500%       96.87500%  exact
  8       99.21875%       99.21875%  exact
```

`94` section 4.2 reports 99.22% at `W = 8`. The closed form gives 99.21875%, and `W = 8` is a width no
probe of mine swept, so that agreement is a prediction rather than a fit.

**So the cell carries no information about the overflow policy.** Wrapping contributes only the two
coincidences where `2c` vanishes. A design taking that cell as evidence for a permission vector would be
carrying a bit for a law that is false before any policy is applied.

**What survives of W4, and it is not nothing.** The retraction verdicts genuinely differ per operation
pair: P2's own tables show `pi` respecting `add>add` and breaking `add>sub` in the same configuration.
So a design does need a verdict per pair rather than one flag. What does not survive is the claim that
retraction and identity-preservation are independent permissions to be carried separately: **retraction
is the generator and the identities are its consequences**, so there is one thing to carry and the rest
is derived.

### 6.3 A hazard in a live workspace rule, which is what the criterion found first

`arvo-always-optimal-internals.md:55-56` carries the panel's result as: "**multiplicative associativity
and distributivity hold exactly at `F == 0` and fail everywhere else.**"

`93`'s P7 already narrowed the converse: `F = 0` is necessary and not sufficient, because signed
two-sided saturation fails there. My P2 reproduces that independently at `W in {4,5,6}` (signed,
saturating, `F = 0`: distributivity fails at 45.56% of triples, additive associativity at 24.60%), which
is a second instance of `93`'s finding from a different model.

**What is new is a hazard inside the region where the sentence is fully right.** At unsigned, `F = 0`,
saturating, distributivity over **addition** holds, and distributivity over **subtraction** fails at
45.79% of triples at `W = 6`. A reader taking "distributivity holds at `F == 0`" as a licence to factor
`a*b - a*c` into `a*(b - c)` is wrong on nearly half the inputs, at the exact predicate the rule names as
safe.

This is not an argument for weakening the rule. It is an argument that a law permission has to name the
**operations** it covers and not just the fraction width, which is what F-F below does and what the
criterion makes cheap: `pi` respects `mul>add` and breaks `mul>sub` in that configuration, and the two
verdicts sit side by side in the same table.

**F-F. A law holds in the representable set exactly when it is an identity of exact arithmetic and the
realisation map respects every ordered nesting of operations it contains. This predicts every law verdict
in the swept region with no soundness mismatch and no conservative mismatch.**
`holds for: W in {4, 5, 6}, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow in {wrap,
saturate}, rounding in {truncate, nearest}, operations {add, sub, mul}, operand window in {full, declared
non-negative}, laws {add_comm, mul_comm, add_assoc, mul_assoc, distrib, mul_over_sub, sub_assoc,
sub_comm}, arity 2 and 3, values exhaustive over the representable domain, threads = 1, target features
any`
Evidence: `97_probes/p2_congruence_predicts_the_laws.py`, 552 cells, both mismatch directions counted.

**F-G. At `F = 0`, unsigned, saturating, distributivity over addition holds and distributivity over
subtraction fails.**
`holds for: W in {4, 5, 6}, F = 0, signedness = unsigned, overflow = saturate, operations {add, sub,
mul}, arity 3, values exhaustive, threads = 1, target features any`
Evidence: same probe. At `W = 6` the failure rate is 45.79% of 262144 triples.

**F-H. Restricting a signed saturating type to a declared non-negative operand window recovers additive
associativity, multiplicative associativity and distributivity over addition, all of which two-sided
saturation loses.**
`holds for: W in {4, 5, 6}, F = 0, signedness = signed, overflow = saturate, operand window = declared
non-negative, operations {add, mul}, arity 3, values exhaustive, threads = 1, target features any`
Evidence: same probe. This was a prediction of the criterion made before running, on the ground that a
one-sided clamp is a congruence and a two-sided one is not, and it independently retrodicts `82`'s
declared-window result, which I have not read.

### 6.4 The diff against `35`, which `93` named as work it was leaving undone

`93` phase two says plainly: "The largest thing I did not do: I did not read `35_mcsherry`, which `40`'s
observable-axis table cites for six of its eight rows and which is the panel's existing measurement of
exactly the laws my P2b, P2c, P7 and P8 measure... Somebody should diff them, and that is a real piece of
work I am leaving undone."

`97_probes/p7_diff_the_criterion_against_35.py` is that diff, and it is a better test than another sweep
of mine would have been. `35`'s model was written independently, long before this unit, by somebody
answering a different question, and its probes emit machine-readable CSV: `35_probes/p2.out` unsigned and
`35_probes/p2b.out` signed, both over `w` in 2 to 7 and `f` in 0 to `w`, both policies, five laws.

The criterion needed one extension to reach `35`'s battery, because `35` measures an order law and P2 did
not. It is the same move rather than a second rule: a quotient by a congruence inherits identities, and a
**monotone** quotient inherits order facts. So an identity holds iff the map respects the nestings, and an
order law holds iff the map is monotone. Wrapping is not monotone and saturating is, which is what
`35`'s monotonicity column says at every row.

```
law                signedness    agree   conservative    SOUNDNESS
add_assoc          signed           66              0            0
add_assoc          unsigned         66              0            0
additive_inverse   signed           66              0            0
additive_inverse   unsigned         66              0            0
distributivity     signed           66              0            0
distributivity     unsigned         66              0            0
monotonicity_add   signed           66              0            0
monotonicity_add   unsigned         66              0            0
mul_assoc          signed           66              0            0
mul_assoc          unsigned         65              1            0

rows diffed                        : 660
agree                              : 659
conservative (safe, predicted fail): 1
SOUNDNESS mismatches (refute)      : 0
```

The single conservative row is unsigned, `w = 2`, `f = 1`, saturating, multiplicative associativity: the
criterion predicts it fails and `35` measures zero failures in 64 triples. A four-element domain is small
enough for an identity to survive a map that does not respect it by coincidence, and a criterion that is
sound without being complete is exactly what is wanted for gating an arm, because it never licenses a
rewrite that is wrong.

**And the two model mismatches I hit on the way are worth more than the agreement, because both were mine
and both were diagnosable.**

The first revision put the truncating shift inside the *exact* multiply rather than inside the realisation
map, which left the map with nothing to round and produced **twelve soundness mismatches, every one of
them `mul_assoc` at `f == w`**. At `f == w` the product never leaves the range, so the boundary policy
never fires and only the rounding can break anything, and a model that has already rounded before the map
runs cannot see it. That is the criterion being tested against a model that had smuggled the quantisation
into the wrong side, and the fix was to import P2's construction rather than re-derive it, so the two
files cannot drift.

The second revision still carried **four soundness mismatches, all signed `mul_assoc`**, and the cause is
named in `35`'s own source: `35_probes/p2b_laws_signed.rs:72-74` says its signed multiply uses "Arithmetic
shift right, which floors rather than truncating toward zero. Named because it is a rounding choice and a
different one would move the multiplicative counts." My prediction was computed with truncation toward
zero. Rounding the same way `35` rounds took the four to zero.

Both are the same lesson and it is the one worth carrying: **a disagreement between two models is a
question about the models before it is a result about the thing.** The four mismatches would have read as
a refutation of the criterion, in a file with a sound argument and an exhaustive sweep behind them, and
the refutation would have been of a rounding choice.

**F-K. Over `35`'s whole committed box, 660 rows generated independently of this unit, the criterion
predicts no law to hold which fails.**
`holds for: w in 2..7, f in 0..w, signedness in {unsigned, signed}, overflow in {wrap, saturate},
rounding = arithmetic shift right, laws {add_assoc, mul_assoc, distributivity, additive_inverse,
monotonicity_add}, arity 2 and 3, values exhaustive over the representable domain, threads = 1, target
features any`
Evidence: `97_probes/p7_diff_the_criterion_against_35.py` against `35_probes/p2.out` and
`35_probes/p2b.out`. One row of 660 is conservative and none is unsound.

## 7. What I keep, and why keeping it is the result

**`25` section 7's sentence, at the design tier, with the objective named above it.** It is right about
what a resolved strategy produces and it survives every attack in this file. What section 2.4 adds is
where it sits and what constrains it, not a replacement. Rewrite cost is real and this sentence has been
paid for twice.

**`40`'s two-space split and its observable classification.** Section 3 is that classification carried one
step further into polarity. `40` got there first and got the harder half.

**`93`'s policy-and-lowering distinction.** Correct and load-bearing. Only its forcing argument goes.

**`93`'s preference-not-resolution discipline**, that the carried parameter must name what was asked for
rather than what was chosen. Section 3.2 is that discipline restated as a variance rule, which is why it
holds rather than being good taste.

**`94`'s W9 measurement**, that policy on the value with the plan at the site reaches one lowered path
with no conditional and no cast. Nothing here touches it and it is the only priced thing either cold
derivation produced about carriers.

**`arvo-toolbox-not-policer.md`'s diagnostic-not-directive posture.** Everything in section 4.2 that looks
like a refusal is a report of a conflict that exists, which is what that rule asks for and not what it
forbids.

**And the four names.** Nothing in this file argues for renaming anything. Under the demand reading each
is a generator, which is additive: every existing spelling keeps working and a fifth costs one symbol.
`94` section 8 reached the same place and I agree with it.

## 8. A converged statement, offered

Op's `95` asks a unit to end in agreement with at least something. This is what I believe `93`, `94`,
`40`, `25` and this file jointly support, written so the consolidation can take it, argue with it, or
leave it. It is a suggestion. Nothing here is settled and op decides.

A strategy is a **consumer-supplied statement of what matters**, over measurements rather than over
implementations. It names no implementation and owns none.

It has **three parts, distinguished by polarity rather than by subject matter**. What the consumer wants
weighed. What the consumer must declare because it changes the computed answer and no later consumer can
recover it. And what the resolver produces because it changes only cost.

The first two travel with the thing they qualify and the third does not, and that is a consequence of the
polarity rather than a separate rule: an input cannot be supplied by the party that receives it.

**Which thing an input qualifies is a separate question from its polarity, and it is answered by what
knows the answer.** A boundary semantics is a fact about a value and travels with it. An intermediate
width is a fact about a reduction and travels with the reduction, because the same stored column is folded
at different arities and the right width is different at each. That is measured rather than argued.

The **first part joins**, by union, freely, with silence as the identity and the closure generated from
the named demands rather than enumerated. The **second part does not join**: no order over it survives
the three readings of "conservative", so where two operands disagree the operation reports a conflict
that is real. The **third part has no mixing question**, because nothing observable distinguishes two
resolutions of it.

What reaches the compiler is a **table**, evaluated offline by a person, and the constraint the canon
puts on that table is that it be **explicable by one statement of what matters**: some single weighting
must account for every row at once. That constraint is what makes a name predictive at a region nobody
has measured, and it is checkable offline at no compile-time cost.

Which **laws** a configuration honours is **computed, not chosen, and not measured law by law**. An
identity of exact arithmetic holds exactly where the realisation map respects the nestings that identity
contains, and an order law holds exactly where that map is monotone. Both are finite tables, and between
them they settle an infinite family of laws at every arity.

A preference weighing a **chain** rather than an operation is a different shape and is served by not
quantising in the interior, at a width cost linear in the chain length. `93`'s F7 and `94`'s W7 agree on
this from two parameter settings and I add nothing to it.

## 9. Located disagreement, carried forward as that

**With `93`, on the forcing argument.** `93` section 3 says the stratification is forced and derivable.
I say there is nothing to stratify and the structure is a dependent function. We agree on every
consequence and disagree on why. What would distinguish us: whether a build arm may resolve an observable
coordinate. Under `93`'s reading that is permitted if done first; under mine it is impossible in any
phase. `40` section 5.4 already compiled the case and calls the canon-shaped question "which axes is an
arm permitted to move", so the discriminator exists and is one op answer away, and it is a question about
intent rather than a measurement.

**With `93`, on keeping `Resolve` as a componentwise join over the policy layer.** F-E says the order it
would join over does not exist in three of four configurations. `93` has not answered that because
`93`'s own P8 produced the evidence and its section 7 was written before P8 ran. This one I think is
settled against `93` rather than open, and I would want `93` resumed on it rather than a fresh opinion.

**With `94`, on W4.** Section 6.2 refutes the cell it rests on with a closed form. What survives is
narrower and is stated there. I do not think this is open either, but `94` should answer rather than me
recording a win.

**Between `93` and `94`, on intermediate precision, I am not carrying this forward as a disagreement,
because section 5 decides it.** The committed harness shows one stored column wanting two different
accumulators at two fold arities, so the accumulator is not the value's property and `94` has it right.
Two things keep it from being a closed question rather than an answered one. It is one family, one
declared width, one element count and one transform, so the predicate on F-I is narrow and the honest
statement is that intermediate precision is the fold's in that region. And `93` should answer rather than
have me record a win, because a resumed member replies with the reasoning that produced its claim while a
fresh one produces a new opinion.

Under section 3's polarity reading the answer also has a shape rather than being a bare result: `40`'s
table classes intermediate precision as **observable** on `35`'s measurement, so it is an input, and the
question was never where it is stored but which party declares it. A fold declares its own arity. A
column does not know how it will be folded.

## 10. Reported outside my question

**The two root templates now carry a superseded banner.** `93` section 10 and `94` section 1 both
reported `mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl` as live, unbannered, and asserting as
settled the thing I1 demoted, and both reported `PRINCIPLES` naming a forbidden feature. Both documents
now open with "**Superseded. This document describes a design that is being replaced, and it is not
current.**" and name both problems explicitly. Recording that it was actioned so the consolidation does
not carry it as open, and noting that every citation into those files in `93` and `94` is now low by
eight lines.

**Fourteen bench validators have still never run.** `96` establishes it, the fix is upstream in mockspace
PR #18, and arvo's pin has not moved. Any pricing claim in this unit that would have been cross-checked
by a per-variant validator is cross-checked by nothing. I did not rest on one.

**Two arms in a committed bench family are Pareto-dominated in every region.** `bitpack-carrier-d64` and
`bitpack-carrier-packed` are beaten on both time and bits by another arm at all six record counts
(`97_probes/p1_rationalisable_sections.out`, dominance pass). That is not a defect in the bench, which is
about the carrier sweep rather than about strategy selection, and it does mean two of its five arms
cannot be selected by any weighting-defined strategy. Worth knowing before anybody cites that family as
the arm set a strategy chooses among.

**`40` section 6.5's flagged arm is untouched by anything here.** `precise-container-width-l1`'s `kernel`
arm returns a flat 63 to 68 nanoseconds at every declared width against a field two orders of magnitude
above it. I used a different family and did not depend on it.

## 11. What I did not do, and what I could not settle

**I ran no benchmark.** Every timing figure in this file is read from committed harness output somebody
else produced. My own probes are exhaustive arithmetic sweeps, compile-time assertions, and one
enumeration over committed medians. Where nothing has been measured I have said unpriced rather than
reaching for a number.

**I did not price the demand lattice, and P8 narrows rather than closes that.** P4 establishes that it
compiles with no feature gate and that the join is total and lawful, and P8 establishes that a computed
demand reaches its arm in one instruction. Neither says what the lattice costs in **compile time** or in
monomorphisation breadth, which are how-much questions and belong on the harness. `94`'s W3 prices the
closely related unbundling at eight bytes and one symbol for a fifth point and I did not reproduce it.
So the emitted-code side of section 4.1 is established and the build-time side is unpriced, and that word
is used deliberately.

**Everything in P2 and P3 is a model width.** `W in {4, 5, 6}`, because arity-3 exhaustive sweeps are
`2^(3W)` and the wall arrives within a couple of bits. The criterion in F-F is a structural argument that
would extend, and I claim it only where I swept it, per I13. The one place I state a closed form (P2b) I
checked it at a width I did not otherwise use.

**On the bench corpus specifically.** I read `bitpack-carrier-width_n*` and `warm-clamp-arity-w13_n*`
from their committed CSVs and meta, and I decoded the second family's keys from its own variant crate
rather than from its title. I did not read any other family, and I did not read the findings prose of
either, only the raw samples, because a findings file is the harness's own compression and I wanted the
medians from the rows.

**I did not read most of the panel.** In full: `INTENTS.md`, `RULES.md`, `93` and `94` including both
phases, `83`, `85`, `87`, `88`, `95`, `96`, `25` sections 0, 4, 5, 7, 8 and 9, `40` sections 0, 1, 3, 5
and 6, `OPTIONS.md` Q5, Q41 and the unasked-questions section, `DROPLIST.md` by grep. Of the probe
directories I opened `93_probes/p1b_demands_and_closure.out` and
`93_probes/p8_q41_do_the_honoured_law_sets_nest.out` in full, because section 4.1 and section 4.3 rest on
what those two actually contain rather than on their files' accounts of them. I did not open `40_probes/`
at all, so my three citations into `40` sections 5.4, 6.2 and 6.4 are through `40`'s own account of its
probes, and if that account is wrong those three paragraphs move. Of `35` I read its two law probes'
sources and their committed CSVs in full, because section 6.4 is a diff against them, and I did not read
`35`'s prose beyond its section index.

**I could not settle whether the rationalisability constraint is one the design wants.** I have shown it
is the checkable content of op's "little bit of option 3", that it is decidable, and that it is what makes
a name predictive at an unmeasured region. Whether the design would rather have a free table and give up
that prediction is a question about intent, and section 2.4 is an argument rather than a measurement. One
probe is one instance and this is one probe.

**I did not attempt the axis set.** `93` section 2 lists six axes and believes the sixth, reproducibility
across targets and builds, is genuinely missing. I have no evidence either way and did not look. P4's
four demands are scaffolding chosen to reach the check; their names, their count and the byte mask are not
proposals.

**And I did not settle what happens to an observable coordinate whose value is genuinely not
const-available.** `94` section 3.3's distinction between a correctness predicate that must be const and
a profitability predicate that merely wants to be is the sharpest thing either cold derivation offered on
that, op's `83` explicitly left it open, and nothing here touches it.

## 12. Coverage of the citations, and what checking them found

Every `file:line` in this document is opened and its content tested by
`97_probes/p5_verify_my_citations.py`, which is `25` section 9's instrument applied rather than admired:
a citation landing two lines from its content still resolves, and only reading the target and testing for
an expected word catches it. Nineteen citations, and the current state is committed:

```
citations checked: 19   ok: 19   failed: 0
```

**It was not seventeen of seventeen on the first run, and the failure was mine.** The file cited
`arvo-always-optimal-internals.md:54-55` for the sentence "multiplicative associativity and
distributivity hold exactly at `F == 0` and fail everywhere else". The sentence is at lines 55 and 56.
Line 54 is blank, and `sed -n '54,58p'` prints a blank first line that reads as absent, which is exactly
how the off-by-one got in. The citation still resolved, it still looked right in the terminal, and only
the content test caught it. Corrected against the file rather than from memory.

Separately, five of the thirteen citations I checked before writing anything resolved to a heading or a
paragraph boundary one or two lines from the content, and were corrected then. So the instrument fired
six times on one file.

That is the sixth and seventh recorded instance of this failure class across two panels, and the number
is reported rather than quietly fixed, because `RULES.md` records that five instances went by before
anybody counted.

The panel-file citations that matter most to my conclusions are `93:384-386`, `93:354-357`,
`93:632-633`, `93:1042-1044`, `93:1178-1180`, `94:420-421` and `88:20-21`, and each was read in its
surrounding paragraph rather than as a line.
