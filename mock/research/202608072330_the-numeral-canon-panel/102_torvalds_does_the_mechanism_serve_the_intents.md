# 102. Does the mechanism serve the intents

**Position:** seventh file of the strategy-axis unit, second of its convergence half.
**Author:** the `torvalds` persona. **Predecessor:** `101_wronski_the_cost_coordinates.md`.
**Probes:** `102_probes/`, six of them, all committed as they ran.

**Standing:** nothing here settles anything. Op decides. Where I say a thing is wrong I mean it, and
where I say a thing survives I mean that too, and both are suggestions.

## 0. The gates

Both run before the assigned work. Neither can be waived by a brief and this one did not try.

### 0.1 The canon gate: passed

Checked against `INTENTS.md`, `RULES.md`, and op's own files `83`, `85`, `87`, `88` and `95`.

The assigned question is "does the converged mechanism serve what op says each strategy is for", which
is a question measured against op's stated intents rather than one that presumes an answer. That is the
right shape and it is the question `101` forced by counting the coordinates. Nothing in the unit's
converged position conflicts with a stated intent in a way that would make the work forbidden rather
than wrong, so there is nothing to refuse and the gate passes.

**One thing I want on the record about the gate rather than about the work.** The brief handed me the
unit's converged position as "claims you may test rather than premises", which is correct handling and
is what let me find what I found. I took it literally: sections 2 and 3 below break two things the unit
believes, and section 4 puts a predicate on a third. None of that was reachable from a brief that
presented the position as settled context.

### 0.2 The test gate: passed, with two observations and no refusal

`mock/crates/` is empty by design, so the suite that exists is the bench-variant crates'. They are not
Cargo workspace members; `cargo test --workspace` in `mock/benches` runs the root binary and reports
**0 tests**, which is how three members reached three different totals. Run per crate:

```
cd mock/benches/variants && for d in *-shared; do (cd "$d" && cargo test -- --test-threads=1); done
```

**123 tests across 13 crates, all passing.** That reproduces `98`'s count exactly and is the fourth
independent recount; `99` records that it had not itself recounted. I did, and `98` is right.

I read the bodies in the surface I touch, which is the cross-arm agreement assertions, because my
question turns on what those assertions establish. **They are real tests.** `wide-rung-shared`'s
`every_arm_matches_the_independent_oracle_at_every_declared_key` compares every arm against a
computation in a different radix sharing no code with any of them, and its own doc comment states why
mutual agreement would not have been enough: "Agreement between the arms establishes only that they
agree" (`mock/benches/variants/wide-rung-shared/src/tests.rs`, the doc comment above that test).
`warm-container-shared`'s `all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key` does
the same and additionally asserts the input buffer's alignment before trusting any number from the
family. Nothing here is tautological, nothing is a smoke test, and the oracles are independent rather
than restatements of the arm under test.

**Observation one, and it is a real hole.** `bitpack-shared` has **no cross-arm agreement assertion of
either kind**, mutual or oracle-backed. `102_probes/p1_the_corpus_compares_cost_at_a_fixed_answer.out`
section B counts them and it is the only crate at zero. Its answer-equivalence therefore rests entirely
on the harness's cross-variant byte comparison, which is the mechanism `96` reports was silently
disabled for a neighbouring check until it was fixed upstream. The pinned harness has the fix
(`bench-harness/src/validation.rs`, `validation_plan`, whose own comment names the defect: "Gating this
on `outputs_may_differ` instead is what silently disabled every validator written by a routine that also
wanted byte-identical output"). So the hole is currently covered. It was not always, and a family whose
only correctness check lives outside its own crate is one upstream regression away from measuring
nothing. That is a catalogue entry rather than a refusal.

**Observation two.** I confirmed `101`'s test-gate finding from the source side. `bitpack-wide-d16` and
`bitpack-wide-d16-control` differ in their sources only by a doc comment and by the exported symbol and
function names, which is consistent with `101`'s report that the emitted code differs at three
constant-pool load offsets: a longer symbol name moves the pool. And the two headers are not equally
careful. `bitpack-carrier-d16-control`'s says "The byte-identity is not assumed:
`26_probes/control_identity.sh` extracts both function bodies from the two built dylibs and diffs them."
`bitpack-wide-d16-control`'s says "byte-identical to `bitpack-wide-d16`" and names no check. `101` is
right that the property has to be checked per family rather than inferred from the naming convention.

## 1. The answer, before the working

Seven claims. The first two are the assigned questions and the rest are what answering them turned up.

**One. `97` attack one is right, I concede my own prior file, and the reason is larger than the count
`97` measured.** `25` section 7 and the cold pair's argmin definition are not two strengths of one
proposition. They describe **two different layers with opposite polarity**, and no merge of them was
ever available at any count. `102_probes/p2` runs `97`'s own polarity test on `25`'s own axis list, one
axis at a time: **three of `25`'s four axes change the value the program computes and one does not.** An
argmin over cost ranges over output position; three quarters of `25`'s object sits in input position.
Section 2.

**Two. The converged mechanism serves op's intents on a region that does not contain them.** Every
committed family's arm set is **answer-equivalent by construction**: twelve of thirteen shared crates
take the harness's byte-exact cross-variant default and the thirteenth replaces it with a per-arm oracle,
which is stronger. So every number in the corpus compares **cost at a fixed answer**. Op's intents do not
live there: I5 trades accuracy for speed, I7 buys accuracy with speed, I3 asks for a particular answer,
and I9 says outright that a strategy "change[s] what the 'correct' answer is". Each ranges over arms that
**disagree**. Section 3, probe p1.

**Three. `101`'s accuracy crossing is not a fork between two arms. It is the two ends of a family with
`k + 2` members, and the interior is on the Pareto front.** Reproduced on a chain built independently of
`101`'s, and `101`'s finding carries a predicate its own statement does not: **the chain must be
non-contracting**, because a contraction damps error rather than accumulating it and there is nothing to
weigh inside one. At `k = 64` the front holds 65 arms and a sweep of exchange rates selects six distinct
ones. Section 3.4, probe p3, which took two versions and whose first version's failure is the finding.

**Four. Chain depth is const-available with no forbidden feature, and the resulting arm reaches one
lowered path.** The refused spelling is `Fx<{D + 1}>`, arithmetic in a type argument. The accepted one
carries depth as a **type** with the number as an associated const, so the increment is ordinary const
evaluation. A four-step chain with the policy switching at depth 2 emits **zero conditional branches** and
truncates at its first step and rounds at its last three, in one function. Section 3.4, probe p4.

**Five. The unit's coordinates are two kinds and nothing has said so: MEASURED and COMPUTED.** Time is
measured and has a noise floor the corpus reports. Bits per element is declared and exact. So is error
against a rational reference. And the distinction is not bookkeeping: **a weighting may include a
measured coordinate only where every arm it ranges over computes the same answer.** Where the arms
disagree, a bench rerun changes the value the shipped program produces. Section 4, probe p5.

**Six. That puts a predicate on `100` section 6.1's Arm C that its statement does not carry.** The band
accepts a committed entry which is not the argmin. Over answer-equivalent arms that costs 0.045% to 3.83%
of speed, which is what `100` priced. Over answer-differing arms it accepts **a different answer than the
strategy names**, which is not a cost to trade. Section 4.2.

**Seven, and it is the constructive half.** `97` section 3.2 states that a policy coordinate "can never be
decided by measurement at all, in any phase, because doing so makes the program's answer a function of a
benchmark". Op's I5 says the opposite: soundness may be sacrificed for "provable meaningful gains", which
is a measurement deciding an observable. **They are both right and the resolution is that "measurement"
names two things.** An exact error computation against a reference is not a benchmark: it is
reproducible to the bit, has no noise floor, and does not move on a rerun. So op's intents need a
**computed** coordinate, which lands on the same side as bits per element rather than on the timing side,
and `97`'s impossibility bites the timing side only. The mechanism serves I5 and I7 once that is said out
loud. Section 4.1.

---

## 2. Question two: `25` section 7 against `97` attack one

I wrote `25`. Section 7 of it proposed that a strategy is "a consumer-written name for one coherent
policy", assigning one value on each of four independent axes, so that "strategies are therefore named
sections over a product of axes rather than values of a single axis". `94` phase two proposed that this
and the cold pair's definitions were one claim reaching TWO EXPERTS. The coordinator declined to register
it. `97` attack one then established they are different claims and measured the gap at 72 of 15625
sections rationalisable by a weighting, 9 strictly.

### 2.1 I concede, and I did not need `97`'s count to do it

**`97` is right and `94` phase two was wrong to merge them.** I say that first and plainly because it is
my own file being refuted and the concession should not be buried under what I want to add.

But I do not concede on `97`'s ground, and that matters, because the ground decides what replaces the
merged claim. `97`'s argument is that the section space is much larger than the rationalisable subset, so
`25`'s definition is strictly weaker and the shortfall is the content. That is true and it is the smaller
of the two differences.

**The larger one is that the two definitions are not about the same objects.** They differ in domain and
in codomain both, and `97` gestures at this ("`25` names an element of the function space, `93` names an
element of the preference space") without following it down.

- **Domain.** `25` indexes by the **build condition**: "each assignment is a function of the build
  condition, a constant assignment being one case of that". `94` indexes by the **region**, which in this
  corpus is element count, width, arity, thread count and mode. A build condition is a `cfg`. A region is
  a property of the consumer's workload. These are disjoint sets of things and nothing has noticed.
- **Codomain.** `25` assigns values on headroom, packing, overflow and intermediate precision. `94`
  assigns an arm. An arm is a mechanism, so the codomains overlap, but only if every one of `25`'s axes
  is a mechanism choice. Section 2.2 measures that it is not.

### 2.2 Three of `25`'s four axes change the answer, and that is the actual refutation

`97` section 3.2 supplies the test and nobody ran it on `25`'s list. An **observable** coordinate is one
whose movement changes the value the program computes, so the consumer must be told and it sits in input
position. An **unobservable** one changes cost only.

`102_probes/p2_which_of_25s_axes_change_the_answer.rs` runs it, one axis at a time, the other three held
fixed, at a declared width of 13 which Rust has no primitive for.

```
AXIS                            ring     nonring   VERDICT
1 headroom                    0/640     500/640    OBSERVABLE only past a non-ring step
2 packing                     0/640       0/640    UNOBSERVABLE everywhere swept
3 overflow policy           511/640     511/640    OBSERVABLE in both regimes
4 intermediate precision      0/640     570/640    OBSERVABLE only past a non-ring step
```

**So `25` section 7's object is three parts policy and one part lowering, and the cold pair's object is
entirely lowering.** They have opposite polarity. `25`'s three observable axes are inputs a consumer
supplies; an argmin over cost produces outputs. There was never a merge available, at any
rationalisability count, and `97`'s 0.058 percent is a fact about how much of one space the other reaches
rather than about whether they are the same space.

### 2.3 The ring boundary, which is a predicate and not a verdict

**Version one of that probe was setup that helps, committed by me, and the fix is worth more than the
run it broke.** It swept only additive chains ending in a mask and reported headroom and intermediate
precision as identical at 0 of 640. Both outputs are kept at
`102_probes/p2_first_version_setup_that_helped.out`.

The zeros are not noise. **Reduction mod `2^W` is a ring homomorphism**, so any composition of `+`, `-`
and `*` gives the same low `W` bits whatever width it was computed at. A construction made only of those
cannot show a headroom difference and cannot show an intermediate-width difference. My sweep proved the
law it was standing on rather than the axis it was aiming at, which is exactly `the-test-gate.md`'s
"setup that helps" and I would have shipped it.

The fixed version reports the boundary instead of a verdict, and the boundary is the useful object:

> Headroom and intermediate precision are **invisible across any composition of `+`, `-` and `*`, and
> become visible at the first step that is not a ring operation**: a shift, a division, a saturation, a
> comparison.

Two things follow that the unit does not carry.

**A predicate for free, on the shape I13 asks for.** In a pure ring region, headroom and intermediate
precision are unobservable, which means they are the resolver's to choose and a strategy need not fix
them. That is a region where an arm may widen an accumulator or narrow it without telling anybody, and
it is a large region: it is every fixed-point add, subtract and multiply chain with no rounding step in
it.

**And evidence toward `25` section 8's own open question, from a side nobody has looked at.** `25`
asked op whether the arithmetic column of the preset table is one axis or two. p2 says headroom and
intermediate precision have the **same** observability predicate. They are distinguishable as mechanisms
and they are not distinguishable by what a consumer can observe. That does not answer the question, and
it says something about what the answer costs: if they are two axes, they are two axes a consumer cannot
tell apart except past a non-ring step.

### 2.4 What replaces the merge, and it is op's `88` answer with a shape

Op was asked whether a strategy is a preset naming a point in a space of axes (option 1), an irreducible
identity (option 2), or nothing but a weighting (option 3). He answered "Mostly option 1, but a little
bit of option 3 with it. Hard to put into words, hopefully you get my meaning here", and `88` records
that a later expert finding the two readings pull apart has found something real.

`97` read the mix as: the design tier writes points, the canon tier writes the objective, and the little
bit of option 3 surviving into the design tier is the rationalisability constraint. That is a good
reading and I think it is one layer too clever.

**The simpler reading, which p2 supports: a strategy is a pair, and op's two options are its two
components.**

> A strategy is **an assignment on the observable policy axes**, which the consumer supplies and which
> fixes what the program computes, **together with a weighting over cost coordinates**, which selects
> among the arms that produce that answer.

Option 1 is the first component: a point in a space of axes, which is exactly what `25` section 7
described, restricted to its observable three. Option 3 is the second: a weighting, which is exactly what
`93` and `94` derived. "Mostly 1 with a bit of 3" is then the honest proportion rather than a hedge,
because the first component is what a consumer must be told and the second is what the compiler resolves
without telling anyone.

**And it is not a compromise between the three definitions. It is a claim that they were describing
different components of one object**, which is why every attempt to rank them against each other has
produced a distinction rather than a winner.

### 2.5 Does the pair reproduce what the unit established

A replacement is worth nothing if the unit's results do not survive it. They do, and several of them
sharpen.

- **`97`'s three layers** are the pair with its second component's output named. Objectives are the
  weighting; observable mechanism coordinates are the policy assignment; unobservable ones are what the
  weighting resolves.

  **This is not the TWO EXPERTS rung and I nearly claimed it as one.** I read `97` section 3.2's
  definition of polarity as required reading before I built anything, and p2 applies *`97`'s test* to
  `25`'s axis list. That is inherited, and `RULES.md` is explicit that inherited agreement is not found
  agreement. What is mine is narrower and I am stating it at its real size: `97` established the
  distinction and did not run it on `25`'s axes, and running it there is what shows the two definitions
  cannot merge. A second reader who derives polarity without reading `97` would earn the rung. I did
  not.
- **`98`'s exchange-rate-not-priority finding** applies to the weighting component and is untouched.
- **`98` section 4.2's threshold result**, that "minimise A subject to B at most t" has no weighting,
  stops being a cost of the definition. A threshold on an answer is a policy assignment, not a weight. So
  the canon does not have to say "no strategy is a hard bound on a measurement"; it says a hard bound
  lives in the first component.
- **`100`'s const assertion** applies to the weighting component and is untouched, except for the
  predicate section 4.2 puts on its Arm C.
- **`101`'s coordinate ceiling** applies to the weighting component, and the pair explains why the ceiling
  looked fatal. `101` counts 1, then 9, then 42 distinguishable strategies as coordinates are added, and
  concludes that an intent with no coordinate is inexpressible. Under the pair, **two of op's four intents
  were never asking for a coordinate.** Section 3 works that through.

---

## 3. Question one: does the mechanism serve the intents, one at a time

The converged mechanism, as `99` and Q43 state it: ship the weighting, the cost table and the generated
winner table, and assert at const time that the third is the argmin of the first over the second.

### 3.1 The region the mechanism is defined on, established rather than assumed

`101` established that the corpus measures effectively one coordinate. The prior question is what kind of
arm set it measures anything over, and `102_probes/p1` answers it three independent ways: the harness's
consent switch per crate, the crates' own agreement assertions and what each compares against, and the
`score` column across every committed row.

```
crates declaring outputs_may_differ = true : 1 of 13  ['satfold-shared']

crate                             mutual-only  oracle-backed
TOTAL                                      48             18

committed CSV files            : 254
data rows read                 : 104080
rows with a non-empty `score`  : 0
```

Twelve crates take the harness default, which is byte-exact cross-variant comparison. The thirteenth,
`satfold-shared`, turns it off and replaces it with a per-arm oracle, and its own doc comment says why
that is stronger rather than weaker: "every arm is compared against an independent reference computed
from the same input, so agreement between arms follows, and an error that moved every arm the same way is
caught here where a cross-variant comparison would pass it."

The file counts and row count reproduce `101` exactly, from a different reader.

> **Every arm set in the committed corpus is answer-equivalent. Every number in it compares cost at a
> fixed answer.**

That is correct bench design and I am not criticising it. `98` already said so and
`warm-container-shared`'s own doc comment says it: without the agreement requirement the fast arm is fast
because it is doing less. What follows is not a defect in the corpus. It is the **domain of definition**
of the mechanism the unit built on top of it.

**And one thing nobody in the unit has named.** The harness has a **third** cross-variant regime, between
byte-exact and none. `Routine::max_relative_error` returns `Option<f64>` and when it is `Some(eps)` the
plan becomes `CrossVariant::Approx(eps)`, comparing element-wise within a relative error rather than
byte-exactly (`bench-core/src/lib.rs`, `max_relative_error`; `bench-harness/src/validation.rs`,
`validation_plan`). **Zero arvo variants set it**, checked by grep across all 94. So the instrument
already models arms that disagree by a bounded amount, which is the shape op's intents actually have, and
arvo has never used it. `101` enumerated six reachable dead columns and did not name this, because it is
not a column: it is a validation regime, which is a different kind of reachable thing.

### 3.2 I5, the speed-first intent

> "the intent behind Hot is performance, efficiency, even at the cost of accuracy or soundness"
>
> "Hot *can* sacrifice soundness, that is its explicit purpose, but it should not lose it for nothing,
> instead, provable meaningful gains." (`INTENTS.md`, `## I5`)

`101` reads this as one intent with a measured half and an unmeasured half, and notes sharply that the
speed intent "which looks fully served, contains an accuracy term in its own bound". That is right and it
is not the whole shape.

**Under the pair, I5 splits cleanly and neither half wants an accuracy coordinate.**

The sacrifice is a **policy assignment**: wrapping rather than saturating, a narrow intermediate rather
than a wide one, an unsound reassociation rather than a sound one. That is component one, it is
observable, and the consumer is told by choosing the strategy.

"Should not lose it for nothing, instead provable meaningful gains" is a **bar on the designer choosing
that assignment**. It is not a term in a runtime weighting and it is not a coordinate. It says: do not put
a lossy assignment in this strategy unless you can show what it buys. That is a discipline on the design
tier, checkable by a human reading a bench, and it is the same shape as `100`'s const assertion in that
it runs where the artifact is written rather than where it is used.

**So I5 is expressible today**, and what it needs from the corpus is not an accuracy column: it is a bench
comparing a sound arm against an unsound one, with the difference between them reported. `98` found
exactly one such arm, `bitpack-write-unsound`, whose corruption rate under real concurrency is measured in
a test as a rate over trials rather than as a column. That is I5's own evidence sitting one plumbing step
away from being usable.

### 3.3 I6 and I17, the storage-first intent

> "it should remain small for memory or disk storage, because it's just sitting basically" (`## I6`)
>
> "Cold does not *have to* drop efficiency wins elsewhere. It can use the same paths Hot uses ... But if
> the path fights the intent, then it's not for Cold." (`## I6`, the bound on the leeway)

The coordinate exists, is declared rather than measured, and is exact. `101` section 2.3 establishes that
and reads it correctly: bytes per element of a declared layout is a static fact and measuring it would be
worse than declaring it.

**I6 is expressible today and it is the only one of the four that is.** A weighting with a large weight on
the storage coordinate and a non-zero one on time is exactly the intent's shape, and the last clause is
what a non-zero rather than lexicographic weight buys: Cold takes a speed win that costs no size, and
declines one that costs size, which is what "if the path fights the intent, then it's not for Cold" says.

`98` section 4.1 already established that op's four intents each name a primary concern and then refuse
to make it absolute, and measured what the refusal buys: 4 available behaviours under a priority order
against 58 under a weighting on its three-coordinate table. I6's last clause is the clearest instance of
that refusal in op's own words, and I am recording it as a second reading of `98` section 4.1 reached from
the intent's text rather than from `98`'s probe.

### 3.4 I7, the accuracy-first intent, which is where the work was

> "the most precise possible answer, throwing out all cold or hot axis optimisations to be *accurate* and
> *precise*, especially within chains and ops, not only alone." (`## I7`)

`101` section 6.1 is the finding that made my dispatch necessary: accuracy over a chain is not a per-arm
scalar, because the per-operation and chain rankings **cross**. Its constructive answer is that chain
length is a region dimension rather than a coordinate, which is the shape the corpus already uses for
thread count.

I attacked that answer from both ends: is the crossing what it looks like, and does the region indexing
it proposes actually reach a const predicate.

#### The crossing is the two ends of an arm family

`102_probes/p3_the_crossing_is_an_arm_family.py` builds the phenomenon from its description rather than
from `101`'s probe, so that reproducing it is evidence rather than a copy. Reference is
`fractions.Fraction`, so the error reported is the real one.

**Version one found no crossing at all**, and that failure is a finding rather than a miss. It used a
contracting chain, `a <- a*3/4 + x`. A contraction damps old error geometrically, so neither the biased
arm's linear accumulation nor the unbiased arm's random walk gets anywhere and both reach a steady state.

> **`101`'s crossing carries a predicate its own statement does not: the chain must be non-contracting.**
> Inside a contraction there is nothing for an accuracy-weighing strategy to decide, because error is
> damped rather than accumulated.

Version one's second failure was mine and more ordinary: it scored arms on accuracy alone, so
round-to-nearest on the finest grid dominated at every depth and the "best" switch depth collapsed to
zero. **An accuracy-only model has no fork in it.** The object appears only when the rounding carries a
cost. Both versions are committed.

Version two, non-contracting, two coordinates, error against exact rationals and a declared count of
round-to-nearest steps:

```
    k      A err      B err  leader
    1    0.24999    0.25003  A
    2    0.49845    0.33581  B
   16    4.00521    0.89106  B
   64   16.03592    1.79268  B

CROSSING at k in [2]
```

The crossing reproduces, on a chain built independently, with A's error growing linearly and B's
sublinearly, which is the mechanism `101` names. And then the arm family:

```
  k = 16                              k = 64
    Pareto front size: 17 of 18         Pareto front size: 65 of 66
    distinct arms selected across       distinct arms selected across
    40 exchange rates: 5                40 exchange rates: 6
```

**The two arms `101` compares are the two ends of a family with `k + 2` members, and the interior is on
the Pareto front.** Reading the crossing as "pick A below it and B above" takes two points from a front
that has sixty-five. And which interior point a strategy takes is decided by a weighting, which is the
converged mechanism working rather than failing: **this is the first instance in the unit of a strategy
weighing something other than time and bytes**, and it behaves the way `98` section 4 says a weighting
should.

#### And the depth reaches a const predicate, with no forbidden feature

All of that rests on the depth being available where the arm is chosen. The natural spelling increments a
const generic, `Fx<{D + 1}>`, which is arithmetic in a type argument and needs `generic_const_exprs`,
which is forbidden. Nobody in the unit had checked whether there is another spelling.

There is, and it is the workspace's standing reflex landing on a case nobody had pointed it at: a refused
bound wants a trait. `102_probes/p4_is_chain_depth_const_available.rs` carries depth as a **type** with the
number as an associated const, so `D::VALUE + 1` is arithmetic in a value position and ordinary const
evaluation handles it.

**It compiles, `#![no_std]`, with zero `#![feature(...)]` gates, zero `dyn`, zero `TypeId`, on the pin.**
And the const predicate reaches one lowered path, which is what I15 asks for. The emitted assembly for a
four-step chain switching policy at depth 2:

```
_chain_switch_at_2:
	mul	x8, x1, x0
	add	x8, x2, x8, asr #8
	mov	x9, #128
	madd	x8, x8, x1, x9
	add	x8, x2, x8, asr #8
	madd	x8, x8, x1, x9
	add	x8, x2, x8, asr #8
	madd	x8, x8, x1, x9
	add	x0, x2, x8, asr #8
	ret
```

Zero conditional branches. The first step emits `mul` with no rounding constant and the last three emit
`madd` against `#128`: truncate at depth 1, round at depths 2, 3 and 4, exactly what the type says, with
nothing left to decide at run time. The two control instantiations, switch past the end and switch at
zero, emit all-`mul` and all-`madd` respectively.

A wrong depth is a **build** failure rather than a test failure, checked rather than asserted:

```
error[E0080]: evaluation panicked: assertion failed: <D2 as Depth>::VALUE == 5
```

#### And then p4 cut against p3, which is why both are here

This is an ad-hoc quick spike with no substance for any how-much question. It took no timing and prices
nothing.

With that said, the three emitted sequences are 9, 10 and 10 instructions. **Rounding to nearest costs one
hoisted `mov` for the whole chain on this target, because aarch64 fuses the add-half into `madd`.** p3's
second coordinate is a count of round-to-nearest steps, and on this target that count does not correspond
to a per-step cost at all.

So p3's arm family is real as an accuracy structure and **its trade is a fact about the target rather than
about the arithmetic**. On a target with a fused multiply-add the interior of that Pareto front may
collapse to its round-everywhere end. That is unpriced, it needs the harness rather than a spike, and it
is the first thing I would attack next.

It is also `25` section 7's own clause arriving with evidence from an unexpected direction: an assignment
on an axis is a function of the build condition, and here is an axis whose **existence as a trade** is a
function of the build condition.

**So: I7 is expressible, and the mechanism serves it.** What it needs is a computed accuracy coordinate,
chain depth in the region, and a non-contracting chain for there to be anything to decide. All three are
reachable, two of them are demonstrated above, and the third is `101`'s.

### 3.5 I3 and I4, the imitate-the-native-primitive intent

> "It should behave like native primitives in regular old rust would" (`## I3`)
>
> mimicry "does not make it absolutely required, if mimicking is consistently just worse choice" (`## I4`)

`98` section 4.3 measured the shipped headroom rule Pareto-dominated on (time, bytes) in 18 of 22
committed `warm-container-*` runs, with three of the four survivors' confidence intervals crossing zero.
So no weighting over (time, bytes) selects it, and whatever the extra byte buys is not on that list.

**Under the pair this stops being a missing coordinate and becomes a policy assignment.** "Behaves like a
native primitive" is a statement about **what answer the program produces**, not about how much of
something it costs. It fixes the observable axes: this overflow policy, this intermediate width, this
rounding. It is component one.

And that reading is what makes I4 coherent. I4 says mimicry is dropped "if mimicking is consistently just
worse choice", which under a coordinate reading is a finite weight on a divergence axis and under the pair
reading is simply: **the designer may choose a different policy assignment when the imitating one is
consistently worse.** The second is a decision made once, at the design tier, by a person, which is what
op's sentence actually describes. The first requires a divergence number per arm per region that nobody
knows how to define, because "how far is wrapping from saturating" has no units.

**So I3 and I4 are expressible today**, and the reason they looked inexpressible is that the unit was
looking for them in the wrong component. `93` reached the neighbourhood of this and stepped past it: it
named "divergence from a reference semantics" as a fourth axis and observed that "an imitation constraint
is a different kind of specification from a weighting" before resolving it as a weighting anyway
(`93` section 1, quoted at `98` section 4.3). **`93`'s observation was right and its resolution was
wrong**, and I am saying so as a second reader of the same sentence.

What remains genuinely open about I3 is `99`'s two questions, both op's: which reading is meant at a width
Rust has no primitive for, and whether the imitation was ever meant to cover the debug-mode overflow
panic that I15 forbids. Neither is a measurement question and neither is mine.

### 3.6 I8 and I9, which are the two that decide the whole thing

> "All of them should be decided by measurement, just measuring different things, and, this is I think the
> mental unlock: They weigh different measurements differently." (`## I8`, and the intent is that first
> sentence, per op at `88` section 2)

> "The strategies aren't orthogonal to the threaded question you had, or its answer, strategies are the
> variables that change what the 'correct' answer is for what we choose as the path." (`## I9`)

**I9 is the one the unit has not reckoned with, and it is the reason my question has an answer at all.**

A strategy "changes what the correct answer is". That is not a statement about cost. It is a statement
that different strategies compute **different values** from the same source, and it is exactly what p2
measures for three of `25`'s four axes. The converged mechanism ranks arms by cost at a fixed answer, so
**on its own it cannot express I9 at all**: an argmin over answer-equivalent arms is definitionally
answer-preserving.

That is not a refutation of the mechanism. It is a statement of what the mechanism is for, and the pair is
the shape that carries both: **component one is I9, component two is I8.**

And I8 then reads exactly as op wrote it, with nothing left over. All strategies are decided by
measurement; they measure different things; they weigh different measurements differently. Under the pair,
"measuring different things" is the coordinate set differing per strategy, which `101`'s ceiling makes
concrete, and "weigh differently" is the weighting. Component one is not decided by measurement, and I8
does not say it is: it says the strategies are, which is a statement about how a strategy is arrived at
rather than about every part of what one is.

### 3.7 The map, which is different from `98`'s and `101`'s

`98` and `101` both report two of op's four intents as having no coordinate, and `101` sharpens that to
inexpressible. Under the pair the map is:

| intent | component | status |
|---|---|---|
| I5, speed first | policy assignment plus a design-tier bar | expressible; the bar wants a sound-against-unsound bench that does not exist |
| I6 and I17, storage first | weighting, over a declared exact coordinate | expressible today, and the only one that is fully served now |
| I7, accuracy first | weighting, over a computed coordinate, with depth in the region | expressible; the coordinate is reachable and demonstrated in p3, the region indexing in p4 |
| I3 and I4, imitation | policy assignment | expressible today; what remains open is op's two questions at `99` |

**So `101`'s inexpressibility result is right about the mechanism it tested and wrong about the intents.**
No weighting over the corpus's coordinates distinguishes on accuracy, which is exactly what `101` proves
and I reproduce nothing against. What does not follow is that the intent is inexpressible, because two of
the four were never weighting-shaped. `101`'s own sentence is careful enough to survive this: "A strategy
whose intent names a quantity with no coordinate is not unmeasured. It is inexpressible." True. I3 and I5
do not name a quantity.

---

## 4. The split the unit needs and does not have: measured against computed

Section 3 says the mechanism serves the intents once the pair is in place. This section is the constraint
that comes with that, and it is the one I would most want carried into the consolidation.

### 4.1 `97`'s impossibility against op's I5, and why both are right

`97` section 3.2, stating what polarity buys over stratification:

> "Under polarity, a policy coordinate can never be decided by measurement at all, in any phase, because
> doing so makes the program's answer a function of a benchmark."

Op's I5 says a lossy assignment is admissible for "provable meaningful gains", which is a measurement
deciding an observable. Read flatly, one of these has to go.

**Neither does, because "measurement" names two things and the unit has been using one word for both.**

`102_probes/p5` separates them on the corpus's own data. Part one bootstraps the committed carrier samples
and counts how often the pure-time argmin moves:

```
regions whose pure-time argmin is NOT unique across 2000 resamples: 5 of 6
```

That is `100`'s mechanism reproduced from a third implementation, and over these arms it is harmless: they
all compute the same value, so a flip buys or loses a little time. `100` prices the whole thing at 0.045%
to 3.83%.

Part two is a counterfactual and is labelled one everywhere it appears, because **no committed family has
arms that disagree**, which is the finding rather than a gap in the probe. It takes p3's real, exactly
computed error table at `k = 16` and attaches a time coordinate whose noise is the corpus's own measured
floor, 0.273%, from `100` section 7.1:

```
    exchange rate   distinct arms   distinct errors   error spread
            0.020               2                 1        0.00000
            0.050               3                 2        0.04651
            0.100               4                 3        0.15976
            0.500               9                 8        1.26732
            1.000              15                15        3.35597
            2.000               7                 7        1.50195
```

The last column is declared ulp. Where it is non-zero, **a rerun of the bench changes the numeric answer
the shipped program produces**, at the same source, the same weighting and the same inputs. Six of the
seven rates swept. The seventh is not a reprieve: at that rate the two lowest-error arms tie because their
errors are equal to five figures.

Part three ranks the same table by computed coordinates only, error against a rational reference and the
declared step count. **One arm at every rate, zero spread, by construction rather than by luck.**

So the resolution:

> **A coordinate is MEASURED or COMPUTED, and the distinction is load-bearing rather than bookkeeping.**
> Time is measured and carries the corpus's noise floor. Bits per element is declared and exact. Error
> against an exact reference is computed and exact.
>
> **A weighting may include a measured coordinate only where every arm it ranges over computes the same
> answer.** Where the arms disagree, every coordinate in the weighting must be computed or declared,
> because otherwise the program's output is a function of a benchmark's noise.

`97` is right about benchmarks and I5 is right about measurements, and the sentence that reconciles them is
that an exact error computation is not a benchmark. It has no noise floor, it does not move on a rerun,
and it is reproducible to the bit. `101` noticed the first half of this, that the coordinate set is mixed
and a tolerance band should not be applied uniformly across it, and read it as a caution. It is more than
a caution.

**And the way out is a licence rather than a restriction.** The coordinate op's accuracy intent needs is a
computed one by nature: p3 computes it, and the harness's own `score_output` hook is shaped for exactly
that, returning a number a routine computes about its own output rather than a timing
(`bench-core/src/lib.rs`, `score_output` and `score_dimensions`). So the coordinate that looked like the
hardest missing thing lands on the easy side of the split.

### 4.2 The predicate `100`'s Arm C does not carry

`100` section 6.1 offers four arms with disjoint predicates, which is the form I13 asks for and is good
work. Arm C:

> "**Arm C, the band rather than the equality.** State the differential with a tolerance. `holds where the
> region's competing arms are separated by less than the coordinate's resolution but more than zero`"

**That predicate is incomplete and the missing dimension is the one section 4.1 names.** A band accepts a
committed winner-table entry which is not the argmin of the stated weighting. Over answer-equivalent arms
that is a small speed cost and `100` prices it: 0.045% to 3.83%. Over answer-differing arms it accepts **a
different answer than the strategy names**, and there is no percentage that describes that, because it is
not a cost on any coordinate the weighting reads.

The corrected predicate, offered:

> Arm C `holds where the region's competing arms are separated by less than the coordinate's resolution,
> more than zero, **and compute the same value**`.

`100` could not have found this, because every family it worked on satisfies the added clause silently and
p1's census is what makes the clause visible at all. I am not recording this as a defect in `100`. It is
a dimension that had no reason to appear until somebody asked what the corpus's arm sets are.

**And the same clause bounds Arm A.** `100`'s Arm A is generate-and-assert with equality, whose predicate
requires every weighed coordinate to have a measurable value and the competing arms to be separated by
more than the harness's resolution. Over answer-differing arms ranked by computed coordinates, the
separation condition is vacuous, because a computed coordinate has no resolution to be separated by. That
is a widening of Arm A rather than a narrowing, and it is the good news in this section: **the equality
assertion is exactly right where the band is inadmissible.**

---

## 5. What I keep, and why keeping it is a result

`RULES.md` says keeping something with your own reasoning behind it is a contribution. Four things, and I
went looking for reasons to break each.

**The converged mechanism, whole.** Ship the weighting, the cost table and the generated winner table, and
assert at const time that the third is the argmin of the first over the second. I attacked it from the
side nobody had, which is what it ranges over rather than how it is checked, and it survives with a
predicate attached rather than a hole in it. `100`'s composition is the right object and section 4 narrows
one of its four arms and widens another.

**`97`'s polarity distinction, which I kept after trying to break it.** Observable in input position,
unobservable in output position. I did **not** derive it independently: I read `97` section 3.2 before
building anything, and section 2.5 corrects an earlier version of this paragraph that claimed otherwise.
What I did was point it at a list `97` never tested, `25`'s own four axes, expecting it to blur, and it
did not: it separated them three to one and produced the ring boundary as a bonus. A distinction that
stays sharp on a list its author did not choose is worth keeping, and that is a weaker claim than the
rung and a real one.
**`101`'s region-against-coordinate line.** "A quantity belongs in the region when a strategy's answer may
differ across it, and in the cost vector when a strategy's answer is scored on it." p3 and p4 are that
sentence built and compiled, and it held under both. It is the most portable thing `101` produced and I
would put it in the consolidation close to verbatim.

**`98`'s exchange-rate reading of op's four intents.** Every one names a primary concern and refuses to
make it absolute. I read the four intents' text independently while working through section 3 and reached
the same thing on I6's last clause in particular, which is the clearest instance and which `98` cites
without singling out.

---

## 6. A converged statement, offered

`95` asks a unit to end in agreement with at least something. This is what I believe `25`, `93`, `94`,
`97`, `98`, `100`, `101` and this file jointly support, once the pair is in place. It is a suggestion and
op decides.

> A **strategy** is a pair. Its first component is an assignment of values on the axes a consumer can
> observe: the ones where moving the assignment changes the value the program computes. Its second is a
> weighting over cost coordinates, which selects among the arms that produce that value.
>
> The first component is supplied and never derived, because a coordinate a consumer cannot recover from
> the bits is one every consumer of a value must agree about. The second is resolved and never supplied,
> because nothing a consumer can observe depends on which arm was chosen.
>
> A cost coordinate is **measured** or **computed**. A measured one has a resolution; a computed one is
> exact. A weighting may read a measured coordinate only where every arm at the region computes the same
> value. Where they do not, every coordinate it reads is computed, because otherwise the program's output
> is a function of a benchmark's noise.
>
> A quantity over which a strategy's answer may differ belongs to the region. A quantity on which a
> strategy's answer is scored belongs to the cost vector. Chain depth, element count, width, arity and
> thread count are the first kind. Time, footprint and error against a reference are the second.
>
> The shipped artifacts are the weighting, the cost table and the winner table derived from them, with a
> const assertion that the third is the argmin of the first over the second. The assertion is an equality
> where the coordinates are computed and where measured arms are separated by more than the instrument's
> resolution, and it is a band only where the arms it ranges over compute the same value.

**Permanence.** Every sentence survives a rewrite. None names a container, a width, a marker, a type
parameter or a table cell, and none names a count of strategies.

**Equivalence.** Three teams implementing this produce things that behave the same on what matters: a
consumer supplies the answer-fixing part, the compiler resolves the rest, the resolution is derived from a
stated weighting over a committed table and checked at build time, and nothing that changes an answer is
decided by a timing. They differ on how many strategies ship and what they are called, which is the arm
rather than the concept.

**Where it is weaker than I would like, stated rather than hidden.** The pair's first component is a set
of axes and I have measured four of them, which are `25`'s four, at one width, on one arithmetic. Whether
that list is complete is exactly `25` section 8's open question and I have not closed it. The pair does
not depend on the list being right; its application does.

---

## 7. Located disagreement, carried as that

**With `101`, on inexpressibility.** `101` concludes that two of op's four stated intents are
inexpressible because no weighting over the existing coordinates distinguishes on them. I agree with every
measurement and disagree with the scope of the conclusion: two of the four are not weighting-shaped, so a
coordinate ceiling does not bound them. This is a disagreement about what an intent asks for rather than
about any number, and it is decidable by op in one sentence, which is why section 10 puts it to him rather
than resolving it.

**With `97`, on how `25` and the cold pair differ.** We agree they are different propositions. `97` locates
the difference in the size of the rationalisable subset; I locate it in polarity and in the domain of the
indexing, and I think the counting result is a consequence rather than the cause. If `97` is right and I am
wrong, the pair is unnecessary and rationalisability is the whole story. p2 is what decides between us and
it points my way, but p2 tests four axes at one width and a wider sweep could move it.

**Unresolved, and I could not close it.** Whether the harness's `Approx(eps)` regime is the right home for
answer-differing arms, or whether it is a validation tolerance that has nothing to do with a cost
coordinate. It models a bounded disagreement, which is the shape op's intents have and which `98` section
4.2 shows a weighting cannot express. But a validation tolerance is a gate on whether a run is trustworthy,
not a number an arm is scored on, and I do not know whether those should be the same object. It wants
somebody who has actually run the harness's quality path, which is nobody yet.

---

## 8. Shapes found and not taken, with what closed each

The next expert attacking from a different angle starts from this list rather than from nothing.

**Accuracy as a per-arm scalar with the region unchanged.** Closed by `101` section 6.1 and re-closed by
p3: the rankings cross, so one number per arm names an arm that is wrong past the crossing.

**A divergence coordinate for I3, measuring distance from native semantics.** Closed on definition rather
than on measurement. "How far is wrapping from saturating" has no units, and the two behaviours are not
points on a line. Section 3.5 moves I3 to the policy component instead, which needs no such number.

**Incrementing a const generic to carry chain depth.** Closed by the forbidden-feature list:
`Fx<{D + 1}>` needs `generic_const_exprs`. p4 takes the trait route instead and it costs nothing.

**Carrying chain depth as a runtime counter.** Closed by I15 before it was built: a runtime counter feeding
an arm selection is a runtime check, and `85` section 1 is explicit that the runtime column does not exist.

**Making the switch depth a function of the total chain length rather than the depth so far.** Not closed,
and I could not build it. The total length is not available at the point where an operation lowers, only
the depth reached. p4's arm uses depth-so-far, which caps the accumulated bias rather than optimising
against a known endpoint, and p3 suggests that is enough to reach the Pareto front. Whether a
total-length-indexed arm beats a depth-indexed one is unmeasured.

**Reading `score_output` as the accuracy coordinate directly.** Not closed and not taken. `101`'s p7
compiles a routine that fills it, which is further than I got, and the open part is that the harness's
quality path (`bench-harness/src/quality.rs`, per `101` section 6.2) has never been run by anything in
this repository, so what it produces is unknown rather than known-inadequate.

**Using `max_relative_error` to admit answer-differing arms into a family.** Found in the harness source,
zero arvo variants use it, and I did not build against it. It is the cheapest visible route to a bench
family whose arms disagree, which is what I5's bar and I7's coordinate both need. Section 7 says why I
could not settle whether it belongs.

**Testing whether the ring boundary from p2 gives a usable arm.** Not taken. p2 establishes that headroom
and intermediate precision are unobservable inside a pure `+ - *` region, which means a resolver may
widen or narrow freely there without telling anybody. That is a licence-shaped finding and somebody
should price what it buys. I ran out of question.

---

## 9. Findings, each with its predicate

Per I13 and `RULES.md`. Listed with `any` means established across it; listed with a fixed value means
established there and only there; absent means it does not hold anywhere that dimension is present.

**F-102-1. `25` section 7's four axes are not one kind of thing: overflow policy is observable
unconditionally, packing is unobservable, and headroom and intermediate precision are observable only past
a step that is not a ring operation.**
`holds for: W = 13, operations in {+, -, *, >>}, column lengths in {1, 2, 3, 4, 8, 16, 32, 64, 128, 1024},
seeds 1..64, unsigned, threads = 1, host = aarch64-apple-darwin, rustc nightly-2026-05-28`
Evidence: `102_probes/p2_which_of_25s_axes_change_the_answer.rs` and its `.out`, with the setup-that-helps
first version and its output committed beside them.

**F-102-2. Every arm set in the committed corpus is answer-equivalent, and no committed row carries a
per-arm quality number.**
`holds for: all 13 shared variant crates, all 254 committed CSV files, all 104080 committed data rows,
mockspace pin bce17f6c`
Evidence: `102_probes/p1_the_corpus_compares_cost_at_a_fixed_answer.py` and `.out`, three independent
readings; the harness gating quoted at `102_probes/p1_harness_gating.out`.

**F-102-3. The harness carries a third cross-variant regime, a relative-error tolerance, and zero arvo
variants use it.**
`holds for: mockspace pin bce17f6c, all 94 variant crates`
Evidence: `bench-core/src/lib.rs` `max_relative_error`, `bench-harness/src/validation.rs`
`validation_plan`, both quoted at `102_probes/p1_harness_gating.out`; the count by grep across `variants/`.

**F-102-4. The accuracy crossing requires a non-contracting chain, and inside a contraction neither arm's
error accumulates.**
`holds for: declared grid 2^-8, fine grid 2^-9 and 2^-12, gain in {1, 3/4}, chain lengths 1..128,
512 seeds, reference exact rational`
Evidence: `102_probes/p3_the_crossing_is_an_arm_family.py` against
`102_probes/p3_first_version_contractive_chain_no_crossing.py`, both committed with their outputs.

**F-102-5. The two arms the crossing compares are the ends of a family with `k + 2` members whose interior
is on the Pareto front, and a weighting selects interior members.**
`holds for: declared grid 2^-8, fine grid 2^-9, gain = 1, k in {4, 16, 64}, 512 seeds, coordinates
(exact error, declared rne-step count), 40 exchange rates spanning 2^-20 to 2^20`
Evidence: `102_probes/p3_the_crossing_is_an_arm_family.out`.

**F-102-6. A chain's depth is const-available with no forbidden feature, and a depth-indexed rounding
policy lowers to one path with zero conditional branches.**
`holds for: rustc nightly-2026-05-28, target aarch64-apple-darwin, opt-level 3, `#![no_std]`, chain length
4, switch depths in {0, 2, 99}, FRAC = 8`
Evidence: `102_probes/p4_is_chain_depth_const_available.rs`, `p4_emitted.s`,
`p4_is_chain_depth_const_available.out`, with the negative control's `E0080` recorded.

**F-102-7. On aarch64 the round-to-nearest sequence costs one hoisted instruction for a whole chain rather
than one per step, because the add-half fuses into `madd`.**
`holds for: target aarch64-apple-darwin, opt-level 3, rustc nightly-2026-05-28, chain length 4, FRAC = 8,
i64 accumulator`
Evidence: `102_probes/p4_emitted.s`. **This is an ad-hoc quick spike with no substance for any how-much
question.** It establishes a structural fact about three emitted sequences and prices nothing.

**F-102-8. A weighting that includes a measured coordinate and ranges over arms that disagree makes the
program's output a function of the benchmark's noise; the same weighting over computed coordinates is
stable by construction.**
`holds for: noise sigma 0.00273 relative (the corpus's own measured floor), 18 arms from p3's k = 16
family with exactly computed errors, 2000 resamples, 7 exchange rates from 0.02 to 2.0`
Evidence: `102_probes/p5_a_measured_coordinate_over_answer_differing_arms.py` and `.out`. **Part two of
that probe is a counterfactual**: its error column is real and exactly computed, its time column is
synthetic, because no committed family has arms that disagree.

**F-102-9. The pure-time argmin on the committed carrier family is not unique across resampling in 5 of 6
regions.**
`holds for: bitpack-carrier-width_n* as committed, 6 arms, 80 samples per arm per region, median
estimator, 2000 bootstrap resamples, threads = 1`
Evidence: `102_probes/p5_a_measured_coordinate_over_answer_differing_arms.out` part 1. This is a third
independent instance of `100`'s mechanism.

**F-102-10. The bench-variant suite is 123 tests across 13 crates, all passing, and `bitpack-shared` has
no cross-arm agreement assertion of either kind.**
`holds for: the tree at commit 7ac27921, rustc nightly-2026-05-28, run per crate with --test-threads=1`
Evidence: the per-crate run recorded in section 0.2;
`102_probes/p1_the_corpus_compares_cost_at_a_fixed_answer.out` section B.

---

## 9b. What this does to the register's live options

Written here rather than into `OPTIONS.md`, because the register is the coordinator's and members of this
panel have already had citations broken by a file growing underneath them while they read it.

**Q43, the composition.** Survives whole, with one arm's predicate corrected. Section 4.2 adds `and
compute the same value` to Arm C's predicate and observes that the same clause *widens* Arm A: over
answer-differing arms ranked by computed coordinates, Arm A's separation condition is vacuous because a
computed coordinate has no resolution to be separated by. So the equality assertion is exactly right
where the band is inadmissible, which is a better story for Q43 than it had.

**Q44, strict positivity against non-negativity.** Untouched by anything here. My work is about what the
weighting ranges over rather than about the sign of its entries. `101`'s fourth option, requiring a
unique argmin, is unaffected either way.

**Q45, arms no weighting can ever select.** Weakly affected, in favour of its option (b). `101` reports
(b) as measured and currently unsupported, because the third coordinate `98` tried did not survive its
own significance test. Section 3.4 exhibits a coordinate that would un-dominate arms, error against an
exact reference, and it is exact rather than measured so it has no confidence interval to cross zero.
That does not resurrect the specific arms `97` found dominated; it says the option's mechanism is real.

**Q47, the two I3 questions.** Untouched and still op's. Section 3.5 moves I3 into the policy component,
which changes nothing about either question: which reading is meant at a non-native width, and whether
the imitation covers the panic I15 forbids, are both about what op meant rather than about where the
intent sits in the mechanism.

**Q48, the coordinates.** Reframed rather than answered. Its three options (add the coordinates, use the
region instead, declare the ceiling) are all still live and section 3.7 changes which intents they have
to cover: two of op's four are not weighting-shaped, so the ceiling does not bound them. And section 4
adds a dimension the entry does not have: the coordinates split into measured and computed before they
split into present and absent, and the split decides where a weighting may be used at all.

**One option nobody has entered**, offered as a candidate rather than a proposal, and it is section 2.4:
**a strategy is a pair**, an assignment on the observable policy axes together with a weighting over cost
coordinates. Its rivals are the three definitions the unit has been ranking against each other, and what
distinguishes it from all three is that it claims they were describing different components rather than
competing. What would kill it: a fourth axis of `25`'s kind that is neither observable nor a cost
coordinate, or op saying at section 10 that he meant one object.

**And one option `101` opened that I could not close**, so it stays open with a note. The harness's
`max_relative_error` regime admits arms that disagree by a bounded amount, which is the shape op's
intents have and which `98` section 4.2 proves a weighting cannot express. Zero arvo variants use it. I
do not know whether it is a cost coordinate or a validation gate and section 7 says why.

## 10. For op, and it is one thing

Not a category-wide policy fork, and I have checked it against the shape he has now rejected three times.
It is a question about his own words that no measurement answers.

**Is "the strategy changes what the correct answer is" (I9) a statement about the strategy as a whole, or
about one component of it?**

The unit has been building a mechanism that ranks interchangeable implementations by cost. I9 says a
strategy changes the answer, which that mechanism definitionally cannot do. Section 2.4 proposes that a
strategy is a pair and that I9 is its first component, which makes both true at once and makes `88`
section 1's "mostly option 1, a little bit of option 3" a decomposition rather than a proportion.

That is my reading and it is the load-bearing thing in this file, so it should be checked rather than
inherited. What I am asking is not which design to take: it is whether the pair is what he meant, or
whether he meant one object that does both and the unit has been mis-drawing the line.

**And the counterweight, so this is not a leading question.** If a strategy is one object rather than a
pair, then everything in section 3 changes: `101`'s inexpressibility result stands at full strength, two
of the four intents genuinely have no expression, and the coordinate work becomes the unit's critical
path rather than a component of it. Both readings are coherent designs and I can build either.

**What I am not asking.** Which reading of I3 is meant at a non-native width, and whether I3's imitation
covers the overflow panic. Both are already at `99` and Q47 and both are still his; nothing here bears on
either.

---

## 11. What I did not do, and could not settle

**Read in full:** `INTENTS.md`, `RULES.md`, `99`, op's `83`, `85`, `87`, `88`, `95`, `OPTIONS.md` entries
Q43, Q44, Q44-addendum, Q45, Q46, Q47, Q41-addendum, Q48, `97` sections 1 to 3.3, `98` sections 4 to 6,
`100` sections 1, 6, 6.1, `101` sections 1 to 4.3 and 6 to 8, `25` sections 0, 7, 8, 9.

**Read in part, and named because a claim of mine leans on it:** `100` sections 7 and 8 through `101`'s
account and Q43's, not the source. My section 4.2's correction to Arm C's predicate rests on `100`'s
section 6.1 text, which I did read, and on its band cost figures of 0.045% to 3.83%, which I took from
`100`'s section 1 rather than from section 4 where they are derived. If those figures are wrong my
argument's shape is unchanged and its magnitudes are not mine.

**Not read:** `93`, `94` and `96` in full; every panel file before `25` except through the register. `93`'s
section 1 and its F8 I know only through `97` and `98`'s quotations of them, and section 3.5 leans on one
such quotation, which I have flagged inline.

**Not verified:** `100`'s 93.8% false-alarm figure, `98`'s 1200-of-1200 and 4-against-58 counts, `101`'s
p7 fidelity-coordinate compile. Each is cited above for what it proved and none of my probes depends on
one.

**Could not settle:** whether `max_relative_error` is a cost coordinate or a validation gate (section 7);
whether p3's rne-step coordinate corresponds to a real cost on any target, which needs the harness and
which p4 gives reason to doubt on aarch64 specifically; whether the ring-boundary licence from p2 buys
anything measurable.

**Ran no bench.** Every number above is either read from committed harness output, computed exactly, or
read off emitted assembly and labelled as an ad-hoc spike with no substance. Where I say something is
unpriced I mean the harness has not run on it.

---

## 12. Coverage of the citations

I did not check my citations by reading them. `102_probes/p6_check_my_own_citations.py` opens every one
and tests its **content**, which is the practice `RULES.md` records from `25_probes/` as the cheapest
correctness tool the panel has. It checks 32 headings by name, 27 quotations verbatim against the file
they are attributed to, and 5 items in the bench tree.

```
checked: 32 headings, 27 quotations, 5 source items
failures: 0
```

**That zero is after three rounds, and the two intermediate rounds are the reason to trust it.**

The first run reported **11 failures**. Nine were my checker rather than my citations: a quotation
wrapped across lines in the source, or carried inside a blockquote, is still verbatim, and a raw
substring search reports the line break as a miscitation. Normalising whitespace and blockquote markers
on both sides took it to two. The tenth was the same class one level down, a doc comment whose `///`
prefixes sat inside the phrase.

**The eleventh was real and it was mine.** I had quoted I6 as "It should remain small for memory or disk
storage". Op wrote "it should remain small", lowercase, mid-sentence, and I had silently promoted a
clause to a sentence by capitalising it. That is small and it is exactly the class this panel has been
bitten by twice: a quotation that reads correctly and is not what he said. The file is corrected and the
correction is here rather than silent.

**The lesson I would pass on is about the false alarms rather than the hit.** Nine of eleven were the
instrument, and an instrument that cries wolf nine times is one somebody stops running. Normalising
before comparing is what made the tenth and eleventh visible at all, and I would not have looked past a
report of eleven failures I already believed were formatting.

Cited by heading rather than line throughout, per the brief, because two members of this panel have
already had citations broken by a predecessor's file growing underneath them. The exceptions are the four
source files in the pinned mockspace checkout, which I cite by file and item name rather than by line for
the same reason: the pin is fixed but the item names are stabler than the offsets.

**What p6 covers:** all eleven quotations from `INTENTS.md`; op's `88` sections 1 and 4 and `85`
sections 1 and 2; `97` sections 2.1 and 3.2 and its 72-of-15625 figure; `98` sections 4.1, 4.2, 4.3 and
5 and its 18-of-22 figure; `100` section 6.1 including Arm C's predicate verbatim, and its 0.045%,
3.83% and 0.273% figures; `101` sections 2.3, 4.2, 6.1, 6.2 and 7 including its inexpressibility
sentence and its region-against-coordinate sentence verbatim; `25` sections 7 and 8; Q43, Q44, Q47 and
Q48; and five items in the bench tree. The four harness and bench-core items are quoted verbatim into
`102_probes/p1_harness_gating.out` so a later reader does not have to find the pinned checkout.

**One correction I made to my own probe output rather than to a predecessor.** p5's conclusion originally
read "at every exchange rate swept", which is false: one of seven rates has zero spread. The probe now says
six of seven and explains the seventh. The wrong sentence existed for one run and is not in the committed
output.

**One thing in my own probe output I corrected in place and am naming here.** p4's first constraint count
reported one `#![feature(` and one `dyn`, both of which were matching **my own doc comment saying the file
has neither**. The recount over non-comment lines only is appended to the same output file, with the
reason. A grep that matches the sentence claiming a thing is absent is the cheapest possible way to report
a false positive and I nearly shipped it.
