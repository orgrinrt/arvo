# 42. The law layer: what it is, where it comes from, and how a consumer asks for one

**Date:** 2026-08-08. **Position:** after `41`. **Mode:** explore, do not settle (`00_brief.md`, `04`,
`28`). Nothing here settles anything, and where I say a route is closed I give the diagnostic that
closed it.

## 0. Gates

**Canon gate: passed, and there is nothing to defend.** There is no ratified canon for this panel to
align against. The fixed set is op's own files (`01`, `04`, `28`, `32`, `34`, `36`, `37`, `38`, `39`),
the workspace discipline, and the forbidden-feature list. I checked every claim below against all nine
op files and against `~/Dev/clause-dev/.claude/rules/unstable-features.md`. Every probe I built compiles
on the pinned `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, with no feature gate
of any kind (`grep -c '^#!\[feature' 42_probes/*.rs` returns 0 on all three files, recorded in
`42_probes/RUN.md`). Nothing here proposes work op's files forbid.

**Test gate: there is no suite, and the honest report is a place, not an absence.**

```
$ cd mock && cargo test --workspace
error: manifest path .../arvo/mock contains no package: The manifest is virtual, and the workspace has no members.
$ ls crates/ | wc -l
0
```

`mock/crates` was emptied on 2026-08-08 and stays empty. There is nothing to audit and nothing for the
gate to refuse. The panel's evidence is probes rather than tests, so I applied the gate's spirit to my
own probes instead: each one carries a negative control (the arm that should fail and does), and `p3`
carries a positive result that refuted its own first hypothesis, kept on disk with the refutation rather
than quietly rewritten, per `RULES.md:106-109`.

## 1. The answer, before the working

**The law layer is a vocabulary of derived facts about which algebraic properties an axis assignment
carries, and it is distinct from any mechanism that would act on those facts.** A law is not a value, not
a numeral, and not a strategy. It is a property of an *operation as instantiated by a strategy's
axis values*, stated so that a generic consumer can require it and a compiler can check it.

**Laws come from the axis values, not from the strategy name, and not always from one axis alone.**
`35` established the overflow policy as the primary source (three of four sign-and-policy cells are
exactly reassociable, the fourth fails at 70.1%) and `40` established that the axes divide into
observable ones, where laws live, and unobservable ones, where they cannot. My own probe work below adds
a third layer under that: some laws are properties of a *pair* of axis values acting jointly rather than
of either alone, and at least one of the panel's own measured asymmetries (unsigned saturating addition
associative, signed saturating addition not) has a general algebraic explanation that is not really about
sign at all. It is about whether both of an operation's clamps can be reached in the same computation,
which is a fact about the fold's *trajectory*, not only about the strategy's static axis values.

**A consumer states a requirement by bounding on the axis value's property, not on the strategy name or
the numeral.** `40_probes/p3` already compiled `S::Overflow: AbsorbingTop + MonotoneAdd`, with the
properties implemented on the axis *value* (`Saturate`) rather than declared per preset, so a preset
inherits its properties from the coordinates it assigns and the properties cannot drift from the policy.
My own `42_probes/p2` extends this to the shape that actually matters, a fold with a capacity-derived
accumulator (`35`'s mechanism) whose accumulator type *also* carries the law bound, and shows the two
mechanisms compose in one signature with no interaction hazard: the width-sufficiency check and the
law-satisfaction check refuse independently, for independent reasons, at the same call site.

**A law's failure costs are precise and separable.** `18` showed an absorbing endpoint is sound exactly
while the computation stays at it, not unconditionally. `35`'s `p9` showed min-plus needs an absorbing
top *and* monotonicity, that the two are independently buyable, and that buying one without the other is
measurably not enough (12.6% of shortest paths still wrong). My `p3` sharpens the reachability finding
further: associativity of a clamped operation is not a fact about how many clamps the code has, it is a
fact about how many of them a *specific computation's trajectory* can trigger.

**Laws are stated over the axis value, inherited by the strategy, and consumed by a bound on the
operation.** Not over a bare numeral (a numeral by itself carries no overflow policy to have laws about),
and not over the strategy name (`Warm` denotes nothing about associativity until its overflow coordinate
is fixed, per `40` section 6). This is a design-tier answer about *where the fact lives*; it says nothing
about which mechanism checks it, which is where section 6 draws a line I think the panel should not
cross.

**Should the law layer exist at all, and does it belong to arvo.** Yes on both, and this is not a
default answer, it is one I tried to break. Section 6 makes the case and section 6.4 states, in the
terms the dispatch asked for, the one thing I would refuse to let this panel build: not the vocabulary of
facts, which arvo must own because only arvo knows its own axis values, but a rewriting or extraction
*engine* that decides which facts to exploit. That decision belongs to whoever performs the rewrite, and
building it inside the numeral would be exactly the mistake `arvo-toolbox-not-policer.md` exists to
forbid.

## 2. Method, and what three probes established that nothing else in the panel had

I derived the shape of the law layer from `35`, `40` and `18`, then built three instruments to test
claims none of those three files tested. Two close gaps the panel already knew about; one refutes my own
first hypothesis and finds the real mechanism underneath a measured asymmetry.

**`42_probes/p1`** asks whether commutativity of `+` and `*` is a per-strategy fact worth tracking, since
`18` measured it for interval arithmetic (`18` section 2.2, both at 100%) but nothing measured it for
scalar arithmetic under the overflow policies that gate every other law in the panel. Exhaustive over
`W = 1..=7` unsigned and `2..=7` signed, both policies: **626,224 pairs, zero commute-failures anywhere**
(`42_probes/p1.out`). Commutativity of both operations holds unconditionally, because the pre-clip
quantity (`a+b` or `a*b`) is already symmetric before any policy or width acts on it. It is not a fact
that varies with the strategy, and I would not add it to the law vocabulary as a per-axis marker.

**`42_probes/p2`** composes `35`'s capacity-derived accumulator with `40`'s property-bound mechanism into
one generic fold, and asks whether the two collide. They do not: the width-sufficiency bound and the
law-satisfaction bound refuse independently at the same call site, for different reasons, and `40`'s
observable/unobservable classification, established for a bare binary operation, still separates cleanly
at this composition point. Section 4 is the full account.

**`42_probes/p3`** is the one I would flag first if someone else is picking up from here. I set out to
explain 35's asymmetry (unsigned saturating addition associative at 33 of 33 cells, signed saturating
addition not, at 33 of 33) with the hypothesis that a single active clamp preserves associativity and two
active clamps do not. The first version of the probe refuted that hypothesis directly: a top-only clamp
with an unconstrained floor (operands ranging over both signs) failed at 904 of 3375 triples. I kept that
run in the file with its refutation, per `RULES.md:106-109`, rather than silently rewriting it, and built
the actual discriminating test from what it taught me. Section 5 is the full account and I think it is
the most useful single thing in this file.

I did not read `mock/crates`, which is empty. I read the panel files named in the dispatch end to end,
and `OPTIONS.md` and `DROPLIST.md` in full. Section 9 states the bound honestly.

## 3. What laws are worth naming, and what is a consequence of something else

The dispatch asks which laws are worth naming at all, and which are consequences of others rather than
primitive. Three answers follow from the panel's record plus my own probes, and they sort into three
different kinds of "not primitive": free, derived-from-a-known-structure, and independently necessary.

### 3.1 Free: commutativity of `+` and `*`

`p1`, section 2. Both operations are commutative under both overflow policies and both sign domains,
exhaustively, because the underlying exact operation is already symmetric and neither wrapping nor
saturating cares which operand arrived first. **This is not worth naming as a per-strategy axis fact.**
Adding a `CommutativeAdd` marker trait to the vocabulary that `impl`s for every strategy would be
ceremony: the fact is true of the design as a whole and stating it once, in the canon's prose, costs
nothing and never needs to be checked per strategy.

This matters for the reduction-order question specifically. `35` section 3.6 calls the property a fold
needs "reassociation licence" and measures it as associativity. A reduction that changes both grouping
*and* order (`35`'s `lane2`/`lane4` strided-partial-sum shapes, which combine non-adjacent elements) needs
associativity **and** commutativity together, the combination classical rewriting literature calls AC.
Because commutativity is universal here, the AC question collapses to the associativity question alone:
there is no case in this design where an operation is associative but not commutative, or vice versa, so
"reassociation licence" as `35` and `40` use the phrase already means the full AC property without anyone
having to say AC. I would keep the vocabulary as it stands (associativity, named plainly) rather than
introduce the AC term, because introducing it would suggest a distinction that does not exist in this
design's operations. If a future operation is added that is commutative but not associative, or the
reverse, this collapse stops holding and the vocabulary would need to separate them; nothing here forecloses
that, it only reports that today's operations do not need it.

### 3.2 Derived from a known structure: wrapping's whole bundle is one fact

`35` and `18` measured associativity, the additive inverse, and (implicitly, since it is definitionally a
consequence of associativity plus commutativity plus an inverse plus an identity) the group structure of
wrapping addition as though they were four separate findings. They are one. **Wrapping addition on a
width-`W` numeral is addition in the cyclic group `Z/2^W`.** Associativity, commutativity, the identity
(`0`), and the additive inverse (`2^W - x`) are not four properties somebody checked and found to hold
together by coincidence; they are what "being a group" already implies, and the panel's own numbers say
so without anyone stating the reason: `35` section 3.9 found the inverse at 33 of 33 wrapping cells and
`p2`/`p2b` (`40` section 5.1's citations) found associativity at the same coverage under the same policy.

**This is the clearest instance in the record of a law that should not be named as a primitive fact
because a cheaper, more permanent sentence already implies it.** The canon-shaped statement I would
propose is not "wrapping is associative, commutative, has an identity, and has an inverse" as four line
items; it is "wrapping realises the cyclic group `Z/2^W` under addition", and the four properties are
corollaries a reader derives rather than four facts the canon has to separately assert and separately
verify never to have drifted apart. Nothing forces the design to state it this way, since restating the
four independently is also correct; what favours the group-theoretic sentence is that it costs one clause
instead of four and it is impossible for the four to silently disagree with each other, because they are
not four checks, they are one theorem.

Saturating addition has no comparably tidy single sentence, because as section 5 establishes, its
associativity is not a static fact about the policy at all: it is a fact about whether a given
computation's trajectory reaches both of its absorbing endpoints, and unsigned saturating addition is
associative for a reason that only looks like a policy fact from the outside.

### 3.3 Independently necessary, and not consequences of one another

`AbsorbingTop`, `MonotoneAdd`, closure, identity-existence, and distributivity are not derivable from each
other in this design, and the panel already has the refutations on record.

**Absorption does not imply monotonicity, or the reverse.** `35_probes/p9` built a numeral wrapping in
its interior with a reserved absorbing top, specifically to see whether the two properties could be
bought together cheaply. It absorbs perfectly (0 of 16 failures, matching saturation) and is not
monotone (fails 560 of 2176), and the resulting shortest-path routine is still wrong on 12.6% of in-range
instances. That is a compiled, measured separation: a design that names only `AbsorbingTop` as the
tropical-fold precondition would let `p9`'s hybrid through and would still get the wrong answer at one in
eight.

**Closure (the operation does not widen) is not a law in the algebraic sense; it is the precondition
every other law is stated relative to.** `35_probes/p1` establishes that a fold's accumulator must be
closed or its trip count must be static, as a compiled refusal rather than an argument (four independent
formulations of a widening fold all refused with the identical diagnosis). Every property in this section
is a property of a closed operation on one fixed numeral; none of them is meaningful for an operation
whose output type changes per call, so closure is the gate the whole vocabulary sits behind, not one more
item in it.

**Distributivity and multiplicative associativity are correlated in the data but are not the same fact
logically, and the correlation has a mechanical explanation rather than a deep one.** `35` section 3.7
found both hold almost exclusively at `F == 0` (6-7 of 33 cells, with one shared exception at a
tiny-box row) and both fail broadly otherwise. The reason both share the same near-empty satisfying set
is that both depend on the *same* destructive step, the rescaling right-shift by `F`. A ring's
distributive law and a semigroup's associative law are independent axioms in general algebra; here they
happen to be broken by the same truncation, which is a fact about *this construction* rather than a
theorem that distributivity implies multiplicative associativity or the reverse. I would not merge them
into one vocabulary item, because a different design (say, one that rounds instead of truncates, or that
carries extra guard digits through a chain per `Precise`'s "especially within chains and ops" intent,
`36`) could plausibly separate them again.

## 4. How a consumer states a requirement, composed with the fold layer

`40_probes/p3` demonstrated the mechanism on a bare numeral: a property trait (`AbsorbingTop`,
`MonotoneAdd`) implemented on the axis value (`Saturate`), a `Strategy` trait projecting a preset to its
axis values, and a downstream function bounded on `S::Overflow: AbsorbingTop + MonotoneAdd`. That refuses
correctly at a wrapping instantiation and compiles at a saturating one, with a diagnostic naming the
measurement that justifies it.

Nobody had asked whether that composes with `35`'s fold-accumulator derivation, which is the shape a real
algorithm crate actually needs (`35` section 3.8: arvo-graph's shortest path stands infinity on the top,
relaxes with `+`, and folds over a capacity-bounded set of edges, needing an accumulator wide enough for
the capacity *and* an accumulator whose overflow policy absorbs and is monotone, at once). `42_probes/p2`
builds it.

### 4.1 The composed contract

```rust
pub trait TropicalSumAccum<C, Sa: Strategy> {
    type Acc: CAdd + Copy;
    fn lift(self) -> Self::Acc;
}

impl<We, Se, const K: usize, Sa> TropicalSumAccum<Cap<K>, Sa> for Num<We, Se>
where
    Cap<K>: Log2Ceil,                          // 35's width-sufficiency half
    We: Add<<Cap<K> as Log2Ceil>::Out>,
    Sa: Strategy,
    Sa::Overflow: AbsorbingTop + MonotoneAdd,  // 40's law half
{
    type Acc = Num<<We as Add<<Cap<K> as Log2Ceil>::Out>>::Out, Sa>;
    fn lift(self) -> Self::Acc { Num::new(self.0) }
}
```

Both halves are stated as ordinary `where`-clause bounds, and both are refused independently
(`42_probes/p2.out`):

| arm | what it changes | outcome |
|---|---|---|
| base | element numeral wraps, accumulator strategy saturates and is minimum-headroom | compiles |
| `bad_width` | capacity `7`, which has no `Log2Ceil` row | refused: `Cap<7>: Log2Ceil` not satisfied |
| `bad_law` | accumulator strategy switched to a wrapping one | refused: `Wrap: AbsorbingTop` and `Wrap: MonotoneAdd` both unsatisfied |
| `observable_axis` | the *same generic function*, instantiated at an accumulator strategy differing only on the observable overflow axis from a satisfying one | refuses with the identical diagnostic as `bad_law` |
| (part of base) | the same generic function, instantiated at an accumulator strategy differing only on the **unobservable** headroom axis | compiles, no change |

Nothing in the accumulator-width derivation (`35`'s `Log2Ceil` induction, unchanged from `35_probes/p7`)
had to move to add the law bound, and nothing in the law bound had to know about capacities. The two
mechanisms are orthogonal in the type system for the reason `35` and `40` each independently avoided
const-generic arithmetic in a bound position: both are associated-type and trait-bound shaped rather than
const-expression shaped, so there is no interaction hazard between them, which is a genuine finding rather
than an assumption I am carrying forward from either file. Neither `35` nor `40` tested it; each built
half.

### 4.2 The observable/unobservable split survives at the composition point

`40` section 5 classified axes as observable, where moving them changes the computed answer, and
unobservable, where moving them changes only cost, and established the split for a bare binary operation.
`p2`'s `observable_axis` and default-build arms test whether the split still separates cleanly once the
axis in question belongs to a *derived* type (the fold's accumulator, not the element numeral the
consumer originally wrote). It does: moving the accumulator's headroom (unobservable) leaves the same
generic function compiling; moving its overflow policy (observable) breaks it, at the identical bound and
with the identical diagnostic as the direct law-failure arm. This is worth stating explicitly because it
was not obvious in advance: the accumulator is a *computed* type, assembled from the element numeral and
the capacity by a derivation the caller never sees, and one might have worried that the observable
classification, established for a numeral the consumer writes directly, would not transfer cleanly to a
numeral the derivation produces. It does transfer, because the classification is a property of the axis
value itself (`Saturate` versus `Wrap`), not of who chose it.

### 4.3 Granularity: the panel's own record already refused a blanket law flag

`DROPLIST.md:25-29`, carried from the predecessor panel and part of this panel's fixed record: gating
`arvo-graph`/`arvo-comb`/`arvo-spectral` on a single associativity fact by default was **refused by
measurement**, because it "admit\[s\] the one preset (`Hot`, wrapping) whose recurrences return wrong
answers under these algorithms' own stated specifications, and refuse\[s\] the two (`Warm`/`Cold`,
saturating) that compute correctly, because associativity and the distributivity these algorithms need
are different, complementary laws that invert across the same presets." That is exactly what `35` section
3.6 and 3.7 measure independently in this panel: associativity of `+` and associativity/distributivity
of `*` invert across the same wrap/saturate axis (`+`'s associativity favours saturation over wrapping
for the sign-and-policy cells that matter; `*`'s associativity and distributivity favour `F == 0`
regardless of policy). A single `Sound` marker covering both would be wrong in both directions at once for
some presets. **A consumer states its requirement per property, not per a bundled notion of soundness**,
and `p3` (`40_probes`) already priced what that costs when the properties do not factor into independent
per-axis facts (one `impl` per satisfying combination, since `specialization` and `negative_impls` are
forbidden). That cost is real and it scales with the axis count, per `40` section 9's table; it is not a
reason to bundle, because bundling was already tried and refused on correctness grounds, not on ergonomics
grounds.

## 5. What a law's failure costs, and the mechanism underneath one of the panel's own asymmetries

### 5.1 Partial laws exist, and 18 and 35's p9 both found the same shape independently

`18` section 3.1 measured that a saturating top's absorbing reading is sound "exactly while the
computation stays at it," failing at 936 of 5184 four-step chains the instant an operation (subtraction)
moves off the endpoint into the interior, with zero failures occurring without a clamp. `35_probes/p9`
measured that a reserved absorbing top without monotonicity gets 12.6% of shortest paths wrong. Neither
of these is "the law fails sometimes"; both are precise statements of *when* it fails, expressed as a
condition on the computation rather than as a percentage alone. A canon stating a law as an unconditional
per-strategy fact would misdescribe both.

### 5.2 The mechanism behind 35's associativity asymmetry, found by refuting my own first guess

I wanted to know *why* unsigned saturating addition is associative and signed saturating addition is not,
beyond the measured counts. My first hypothesis was the shape everyone would reach for: one active clamp
preserves associativity (this is the tropical-semiring construction, `min(a+b, TOP)`, which is
associative for the ordinary reason `min` and `+` compose over a totally ordered set), and two active
clamps do not, because the interaction between a floor and a ceiling can differ by grouping order.

`42_probes/p3`'s first version tests exactly that: a single clamp at a ceiling, operands ranging over both
signs (no floor constraint at all). **It refuted the hypothesis: 904 of 3375 triples failed at `top =
3`.** I kept the run rather than rewriting it, because the failure is the more informative half. What it
showed is that "one clamp coded" and "one clamp *reachable*" are different things: with operands allowed
to go arbitrarily negative, the exact intermediate sum can swing far enough below zero and back that the
single ceiling clamp, applied at different points in the association order, produces different results,
even though there is only one `if` in the code.

The corrected hypothesis, tested as the discriminating case in the same file's second half: **associativity
of a clamped operation holds exactly when at most one of its clamps can be triggered by any association
order of the specific fold in question**, not when the code has only one clamp written. Four blocks,
exhaustive:

| block | clamps in the code | is the floor structurally reachable | associativity failures |
|---|---|---|---|
| operands both signs, ceiling clamp only | 1 | yes, unconstrained | **904 / 3375** at `top=3`, growing with width |
| non-negative operands, ceiling clamp only | 1 | no (sum of non-negatives never negative) | **0** at every width tested |
| non-negative operands, explicit floor at 0 *and* ceiling | 2 | no (same reason, floor code present but never taken) | **0** at every width tested |
| non-negative operands, floor above 0 *and* ceiling | 2 | yes, reachable by a small partial sum | **48 / 3375**, **450 / 19683** |

The third row is the one that isolates the mechanism. It has the same two `if`s in the code as the fourth
row and the same zero associativity failures as the second row, because whether the floor is *coded* is
irrelevant; whether it is *reached* by some association order of this fold is what decides the answer.
Unsigned saturating addition is exactly the third row: `SubstituteZero`-style clamping at 0 exists as a
code path and is never taken because the values being folded are already non-negative, so the operation
behaves as a single-ended clamp even though a naive read of the implementation would count two `if`
branches. Signed saturating addition is the fourth row: both endpoints are ordinary, reachable states of
a signed range, so both clamps fire somewhere in a large enough fold and associativity fails generally,
exactly at `35`'s measured 70.1%.

**What this adds to the panel's record.** `35` measured *that* the asymmetry exists and *how much* it
costs (70.1% at n=8, growing with trip count). This adds *why*, as a general algebraic fact about
reachability rather than about sign domain per se, verified on integers with no fixed-point machinery at
all so the finding is not an artifact of arvo's specific widths. It also connects directly to `18`'s
finding in section 3.1, which is the same shape at a different property: an absorbing reading is sound
exactly while the computation's trajectory never leaves the endpoint, and now I have the same
reachability condition governing whether associativity holds. Both are instances of one pattern I would
name if the canon ever wants a general statement: **for a numeral with a bounded representable range, a
law that would hold unconditionally on the unbounded exact values holds on the bounded numeral exactly
on the region of computations whose trajectory does not reach the boundary that would falsify it, and
whether that region is "all computations" or "a strict subset" depends on how many independent boundaries
the operation has and how many of them the specific computation's operand set can reach.**

I would not propose that sentence for the canon as written; it is a mouthful and I built it from two
data points. I would propose the *pattern* be named as a live thing to watch for, because it has now
shown up twice independently (18's absorbing-top soundness, my associativity mechanism), from two
different authors, on two different properties, which under `RULES.md:116-118`'s bar is exactly the
independence that makes an instance count rather than the same author's evidence wearing two hats.

### 5.3 The exchange rate `40` names as unset is the same threshold this section needs

`34` bounds `Hot`'s soundness trade with "provable meaningful gains," left unset. `38` bounds `Warm`'s
mimicry with "consistently just worse," also left unset. `40` section 8 (T3) argues these are one unset
quantity rather than two. A law's failure cost, stated as this section states it (a fraction of instances,
a reachability condition, a percentage of chains), is exactly the number that quantity would be measured
against. I am not proposing a value; I am pointing out that every failure percentage in `35`, `18` and
this file is a candidate input to op's still-unset exchange rate, and a canon that names the rate would
have a ready-made table of numbers to weigh it against, drawn from three independent authors' measurements
rather than one.

## 6. Should the law layer exist at all, and where

The dispatch asks me to challenge this in these words, and I have tried to.

### 6.1 The case for arvo owning the vocabulary of facts

**Only arvo knows its own axis values.** A law is a property of an overflow policy, a sign domain, a
rounding mode: exactly the coordinates `40` shows a strategy assigns. Nothing outside arvo has that
information without re-deriving it, and re-deriving it per consumer crate is exactly the duplication a
shared substrate exists to remove.

**The prior panel already tried relocating it and was refused, on two independent grounds.**
`DROPLIST.md:19-22`: "Relocating the algebraic-law machinery to hilavitkutin, on the theory that
associativity is specifically the contract of parallel reduction: refused by op directly, and
independently undercut by measurement: the regrouping that would have motivated the move already happens
inside arvo's own licensed internals, on a single thread, worth roughly 2x, before any scheduler exists
to relocate to." I am not re-litigating that call; op refused it directly and the panel's own measurement
agreed with him for an independent reason. I cite it because it is exactly the existence-and-locus
question the dispatch asks me to test, and it was already tested, by a different panel, with a ruling on
record. I would flag this as a strong reason not to spend further panel time proposing hilavitkutin as
the law layer's home.

**`arvo-always-optimal-internals.md` needs this vocabulary to be exercised safely, and does not currently
have it.** That rule licenses arvo's own internals to reach for algebraic rewrites (a fused kernel, a
common-subexpression pass, a hand-written microkernel exploiting distributivity) freely, "always
optimal". `35` section 3.7 measured that a distributivity-licensed rewrite changes the answer at up to
70.5% of triples once `F > 0`. So the always-optimal-internals rule, as written, licenses a class of
rewrite whose soundness depends on a fact the rule itself does not name or check. The law layer is not an
optional nicety layered on top of that rule; it is the missing precondition the rule needs to be exercised
without silently trading soundness on every strategy, not only `Hot`. This is, I think, the strongest
single argument that the vocabulary belongs in arvo: a rule already ratified by the workspace discipline
(not by this panel) depends on it existing.

**`I11` names the algorithm crates as arvo's selling point**, and `35` established that their soundness
(a real DAG shortest-path routine, not a synthetic example) depends on exactly these facts. A design that
left the law layer unstated would leave op's own stated selling point silently unsound under half its own
preset table, which the droplist's own carried record ("the presets the design admits today both
return wrong answers under the exact bound they satisfy," `DROPLIST.md:195-198`, from the prior panel,
carried as history rather than as authority) suggests has already happened once.

### 6.2 The case I tried to build against it, and where it actually lands

I looked for a genuine argument that the law layer belongs elsewhere, or should not exist as a distinct
layer at all, rather than assuming the affirmative case above is the whole answer.

**Could the algorithm crates simply assume their preconditions, undocumented, the way the dead tree
apparently did?** That is the status quo `DROPLIST.md:195-198` already found broken, and it is not really
an alternative locus, it is the absence of a law layer, with the cost paid silently at runtime instead of
at compile time. I do not think this survives as a serious option; it is the thing `35` and `18` both
found actively wrong.

**Could the law layer live in hilavitkutin, keyed to the scheduler's own reduction-splitting decision
rather than to the numeral?** This is `DROPLIST.md:19-22` again, and I would add the reason it fails a
second time independent of op's ruling: `40` section 5.4 shows an observable axis "may not be resolved
per arm without the program computing different answers per arm," which means the *fact* of whether
addition is associative under a given strategy has to be fixed and knowable at the point a generic
algorithm-crate function is type-checked, which is arvo's boundary, not hilavitkutin's. hilavitkutin can
certainly *consume* the fact (deciding whether to split a fiber's fold across cores is exactly a place
that would read a law fact rather than assume one), but it cannot be where the fact is established, because
by the time hilavitkutin sees a numeral, the numeral's strategy is already fixed and the question of what
follows from that strategy is a question about arvo's own axis semantics.

**Could the law layer be narrower than the full vocabulary in `OPTIONS.md`'s Q11, stating only what a
fold needs and nothing about, say, distributivity or the total order?** I do not think this survives
either. `18` section 3.8 (via `35`) and section 2 of this file both need the total order for reasons
unrelated to folds (sorting, ranking, DP selection), and `35` section 3.7 needs distributivity for a
reason unrelated to folds (whether internals may fuse a kernel). A law layer scoped only to what a fold
needs would leave those two uncovered while the panel already has evidence both matter.

**So the existence question resolves affirmatively, and the locus question resolves to arvo, on grounds
independent of each other**: a direct op ruling plus an independent measurement (locus), and a ratified
workspace rule that already depends on the vocabulary existing plus a named selling point whose soundness
already broke once without it (existence).

### 6.3 What I would keep as-is

**`25`'s definition of a strategy, refined by `40`.** "A strategy assigns one value on every axis... a
consumer-written name for one coherent policy," with `40`'s addition that the strategy is the *objective*
(a weighting over measurements) and the axis assignment is what the objective *produces*. Nothing in my
own work challenges this; it is the frame every law-bound example in this file sits inside, and it
survives unchanged.

**The property-implemented-on-the-axis-value mechanism from `40_probes/p3`.** I extended it (section 4)
rather than replacing it, and every extension compiled without touching the original arrangement. That is
a real keep in the sense `RULES.md:99-101` asks for: I looked for a reason to change the mechanism and did
not find one.

**`OPTIONS.md` Q9's answer that a generic site needs the trait-relation spelling where the bound must
compose upward and the post-monomorphisation const assertion where the message matters locally (`35`
section 3, Q9).** My `p2` uses the trait-relation form throughout, because the composed fold's bound has
to compose upward through `generic_consumer`, and it does, without needing the const-assertion form at
all. That is a small piece of corroboration for keeping both spellings live rather than picking one.

### 6.4 What I would refuse, in the words the dispatch asked for

**A rewriting or equality-saturation engine living inside arvo, deciding which algebraic rewrites to
apply, is not licensed by anything in this panel's record and I would refuse it if it were proposed.**
This is worth stating explicitly because it is exactly the shape my own background would pull toward, and
the dispatch named that risk directly. My own founding belief, stated plainly: the cost function and the
extraction strategy in an equality-saturation system are where the real engineering judgment lives, and
that judgment is a *domain* decision, not a substrate decision. Deciding *whether* a distributivity-licensed
fusion is worth taking, on a given workload, for a given consumer, is exactly the kind of policy
`arvo-toolbox-not-policer.md` forbids the substrate from making on the consumer's behalf. The FACTS (does
this strategy's axis assignment carry distributivity) belong in arvo, because only arvo knows its own
axis values. The DECISIONS (is this fusion worth taking here) belong to whoever performs the rewrite:
arvo's own internals for a hand-written microkernel dispatch (licensed already by
`arvo-always-optimal-internals.md`, and gated by the same property-trait bound this file demonstrates,
`cfg`-selected or `where`-bounded, no engine required), hilavitkutin's scheduler for a fold-splitting
decision, or vehje's compiler backend for a source-level algebraic simplification of user Clause code.

None of those three consumers needs arvo to contain a rewriting engine. Each needs arvo to expose the
fact, checkable at compile time, in the shape `p3` and my `p2` already demonstrate. **The canon-shaped
sentence I would propose, stated as an intent rather than a mechanism**: *for every axis assignment a
strategy may produce, the design states which algebraic properties of each operation that assignment
carries, in a form a downstream consumer can require and have checked; how any consumer chooses to act on
a carried property, including whether and how to rewrite an expression that could exploit it, is a
decision for that consumer and is never made inside the numeral.* That sentence carries no mechanism, and
it forecloses the one thing I think is worth foreclosing explicitly before someone builds it by accident
under the banner of "the law layer."

## 7. Bearing on the live options

Per `OPTIONS.md`'s own instruction, each gets fits-well, fits-badly, or kills. I cite `OPTIONS.md` by
section and quoted phrase rather than by line, per my brief.

**Q5, is the arithmetic column one axis or two.** *No new bearing beyond `40`'s.* Section 3.2's group-
theoretic observation about wrapping is a fact about the *content* of the overflow axis's cells, not
about how many axes there are, so it does not move this question either way.

**Q9, the crossing at the width surface.** *Corroborates the trait-relation spelling, and section 4.1's
composed contract is a second worked instance beyond `35_probes/p7`.* `35` section 3 found the trait
relation composes upward and the post-monomorphisation const assertion does not. My `p2` builds a second,
independent generic site (the composed fold, not the bare accumulator) entirely in the trait-relation
form and it composes through `generic_consumer` without incident. Small, and it is a second data point on
a question `35` already answered once.

**Q11, what the numeral guarantees to a fold, and what a composition supplies.** *Fits the "both" reading
well, and section 4 is a second, independent instrument compiling it, beyond `35_probes/p7`'s and `40_
probes/p3`'s separate halves.* Neither `35` nor `40` had shown the structure-naming option and the
capacity-relation option composing in one signature; `p2` shows they do, with no interaction hazard, which
narrows "both" from a plausible reading to a demonstrated one at this specific combination (fold plus
tropical property bound). I would not call this settled past that one combination; a different pair of
mechanisms (say, the container derivation's two-output finding from `16`, which I have not read and note
as a gap in section 9) might not compose as cleanly.

**Q12, is the reduction order specified, or is associativity required.** *No new bearing on the options
themselves, but section 5.2 changes what "require associativity" would mean operationally.* If the design
takes the "require associativity" option, section 5.2 says the requirement should not be stated per
strategy as a static table lookup; for signed saturating addition specifically, whether a *given* fold is
associative depends on whether its actual operand set can reach both the floor and the ceiling, which is a
fact about the fold's declared range (a refinement, in `18`'s sense from section 3.3) rather than only
about the strategy. A design requiring associativity for signed saturating folds could, in principle, admit
some of them (those whose declared range structurally cannot reach both ends) rather than refusing all of
them uniformly. I flag this as a possible refinement to the option rather than a new option, because I have
not costed what checking reachability at the type level would require and suspect it is expensive; unpriced,
in the panel's vocabulary.

**Q13, which axes may a build arm move.** *No new bearing; section 4.2's finding that the observable split
survives at a derived (accumulator) type is a small corroboration that the classification is robust enough
to build a rule on, which is what Q13 is asking whether to do.*

**Q14, at what exchange rate does a strategy's preference yield.** *Section 5.3 adds candidate inputs
rather than a value.* Every failure percentage this file, `35` and `18` report (70.1%, 48.9%, 12.6%,
87.5%, 936/5184) is a number of the shape whatever rate op eventually states would be measured against. I
am not proposing a rate.

**Q15, are the axes independently resolvable, and in what order.** *No new bearing; orthogonal to the law
questions this file addresses,* since Q15 is about resolving unobservable mechanism coordinates against
each other (`40`'s headroom-versus-overflow interaction), not about which laws hold.

## 8. What the register should gain

I am not editing `OPTIONS.md`, per my brief. These are for whoever does.

**An addition to Q11's live options.** The "both" option (`35_probes/p7`, structure plus capacity) has a
concrete worked instance beyond `p7`: `42_probes/p2` composes the structure-naming property bound with the
capacity-derived accumulator relation in one signature, with independent refusal for each half. This is
evidence for "both" being buildable at the specific combination tested, not a general proof that every
pairing of a fold-layer mechanism with a law-layer mechanism composes; the register entry should say which
combination was actually tested.

**A new item, offered as a candidate rather than a numbered question: which laws are consequences of
others, and should the canon say so once rather than per-strategy.** Section 3.2's finding that wrapping's
four separately-measured properties (associativity, commutativity, identity, inverse) are one theorem
("this is the group `Z/2^W`") rather than four facts is a genuine simplification available to whoever
writes the canon's law-layer prose. I would not turn this into a numbered question, because I do not think
it is contested; I would flag it as a drafting note for whoever writes the eventual sentence.

**An addition to the register's evidence base for whichever question ends up carrying the reduction-order
material (`35`'s Q12).** Section 5.2's reachability mechanism, verified independent of arvo's fixed-point
representation, on pure integers, with the first hypothesis's refutation kept on the record. It explains
*why* the sign-domain asymmetry `35` measured exists, which is a different kind of evidence from `35`'s
own magnitude measurement and worth citing alongside it rather than instead of it.

**A caution rather than an addition: the register should not acquire a "rewriting engine" or "extraction
strategy" mechanism entry under any of the law-related questions.** Section 6.4 is my argument for why.
Nothing currently in `OPTIONS.md` proposes one, and I am flagging this so that if a future file, reasoning
from the law vocabulary this file and `35`/`40` build, is tempted to propose one, the argument against it
is on record rather than having to be rebuilt from scratch.

## 9. Coverage, bounded honestly

**Read end to end:** `00_brief.md`, `RULES.md`, `INTENTS.md` (all 185 lines), `01`, `04`, `28`, `32`,
`34`, `36`, `37`, `38`, `39`, `35` (in full, all sections), `40` (in full, all sections, two reads because
the tool truncated at line 958), `18` (in full, all sections), `41`, `DROPLIST.md` (in full, both sections
6 and 7), `OPTIONS.md` (Q11 through Q15 in full, plus grepped headings for the rest).

**Read in the region I cite, by opening the lines or files directly:** `07` sections 1 through the
verdict and section 3 (adjunction, the fibre versus the index, the join-and-product-numeral identity),
`08` the verdict and its section headings, `35_probes/p1.rs`, `p2.rs`, `p7.rs` (opened in full, not
summarized, since `p2` reuses `p7`'s shape and I wanted to reuse rather than re-derive), `40_probes/p3.rs`
(opened in full, since `42_probes/p2` extends it directly). Every `file:line` in this document was opened
and its content checked against my claim, not merely resolved.

**Not read:** `02`, `03`, `05`, `06`, `09` through `17`, `19` through `27`, `29` through `31`,
`CANON_CANDIDATE.md`, `MORNING.md`, `PERSONA_CALLS.md`, `SETTLED.md`, `seed/`, `archive/`. Where I refer
to a finding from one of those (`06`'s D0-D3 taxonomy, `16`'s two-output derivation) I rely on `35`'s or
`40`'s account of it and say so in the text each time. **The specific risk:** if `35`'s or `40`'s
paraphrase of `06` or `16` is wrong, my Q11 remark in section 7 inherits it, and I have not independently
verified either paraphrase against the source.

**Not verified:** whether the composed contract in section 4 generalises past the one fold shape I built
it for. `p2` tests a sum-style tropical fold; it does not test a `min` or `max`-style fold, a product
fold, or a fold whose accumulator strategy must itself satisfy a *different* combination of properties
than `AbsorbingTop + MonotoneAdd`. I built one instance and I am reporting it as one instance, per
`RULES.md:116-118`.

**Not measured:** anything about performance, compile time, or trait-solver cost of the composed bound at
larger axis counts. `40_probes/p5` already priced the cost of a non-factoring property (one `impl` per
satisfying combination); my `p2` did not add a new axis, so it does not add new evidence to that pricing,
and whatever it would cost at, say, five or six axes is unpriced, in the panel's vocabulary.

**Probes:** `42_probes/`, committed with sources, raw compiler output and run logs, per `RUN.md`. `p1`
(exhaustive commutativity, 626,224 pairs, zero anchored feature gates). `p2` (composed fold-and-law
bound, four builds, independent refusal on each half, the observable-axis test). `p3` (the reachability
mechanism behind `35`'s associativity asymmetry, built on pure integers with no arvo-specific
representation, first hypothesis refuted and kept on the record, second hypothesis confirmed across four
discriminating blocks). All three on `nightly-2026-05-28`, no feature gates, `grep -c '^#!\[feature'
42_probes/*.rs` returns 0 on every file.

**One instance of evidence is never enough, and I want to be honest about where this file sits on that
bar.** `p1`, `p2` and `p3` share one author and one model, so per `RULES.md:116-118` they are one instance
of evidence wearing three hats, not three. The place in this file with genuinely independent corroboration
is section 5.2's reachability pattern, which now has two independent instances from two different authors
on two different properties (`18`'s absorbing-top soundness, my associativity mechanism), which is two,
not the three the bar prefers. I would want a third, independent instance, ideally on a third property
(distributivity's reachability condition, if it has one, is untested), before I would treat the general
pattern as more than a live thing to watch for.
