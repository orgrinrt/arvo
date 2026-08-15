# 144. The weighting half, measured

Two rounds went at the assignment side of the strategy pair. The weighting side has one expert behind it,
`139`, and both attackers said so in their own words. `141:874-876`: "I did not touch the weighting side at
all. `139`'s `p4` weight-cell geometry, the 44.3% mapping difference across targets, and the
Pareto-optimal arm no linear weighting can select are all untested by me." `142:581-584` repeats it and
adds that this is "now two rounds in which nobody has looked at half of the two-component object, and it is
the largest untouched surface in this topic." My dispatch is that surface.

I will state the outcome first, because two of the five results reverse a headline and one of them is mine.

**`139`'s Pareto claim is correct, and I have strengthened it from a grid observation into an exact one.**
Its witness loses by at least one unit at every point of the simplex, not merely at the 2001 points it
sampled. What does not survive is the six-against-five gap in its main table, which is its exact duplicate
arm failing a tie-break and not an instance of the phenomenon at all.

**The 44.3% portability figure is real, reproduces to the grid point, and is a low outlier.** An ordinary
pair of independent cost tables disagrees at a median of 90.2% of the simplex. `139`'s number sits at the
2nd percentile of that distribution, so its two invented targets are unusually *similar* and the reading
drawn from it understates the phenomenon rather than overstating it.

**The portability problem is not what it looks like.** A weighting travels with exactly zero loss, because
it re-resolves on the new target. An arm does not travel at all: freezing one costs a median 128% and a
worst case of 5900% in relative regret. So a fixed weighting selecting a different arm is not a failure of
portability, it is what a portable weighting looks like from the arm's side, and the design's obligation is
to say that the weighting travels and the arm does not.

**The firewall is what makes any of that safe, and nobody had joined the two halves.** With arms that
conform to one policy, a target change moves the selection and moves no answer, over 192,512 checked cases.
With arms that do not, it moves the answer at the arms' entire disagreement rate, 42.14% of inputs, and
nothing in the type says so.

**And I withdrew my own best real-world result.** I surveyed the committed bench corpus for the arm 139's
O-139-C asks for, found exactly one, verified it by a second procedure, and then found the verdict rests on
a 3.8 nanosecond gap against a 79.2 nanosecond interquartile range on the arm it has to beat. It is noise.
The corrected answer is that **no arm set in arvo's committed corpus contains an established
Pareto-optimal arm unreachable by a linear weighting**, and O-139-C closes in the direction `139` named as
the alternative.

Fifteen probe entries. Eight of my predictions were refuted, one went unwitnessed, and one was confirmed and then
withdrawn by me. The two that mattered most are the ones that killed my own findings.

---

## Gates

### Canon gate: passed

Checked against `INTENTS.md` entry by entry.

I1 is demoted to OPEN on op's own word (`INTENTS.md:51-61`), so deriving what a strategy is remains
licensed. I17 (`INTENTS.md:363-383`) forbids arguing for dropping or downgrading the storage-minimising
concern, and nothing here does; section 7's vocabulary bound is about how many *names* a design can
distinguish and takes no position on which concerns exist. I13 (`INTENTS.md:214-235`) is the one RATIFIED
entry and it is what my replacement in section 3 rests on, since a selector reaching an arm is exactly the
mechanism by which a predicated arm gets chosen at all. I14's operating constraints are treated as in force
and `p8` is built inside them: no `dyn`, no `TypeId`, no allocation, const selection, one lowered path.
I15's "never any runtime checks, ever" is what `p8` tests against rather than around.

One thing on the record rather than folded in. My section 3 proposes changing a component of the object the
topic has settled: the selector inside the weighting. That is a claim against the current statement of Q51's
second component, and it is visible here rather than buried, as the dispatch required.

### Test gate: run in full, and it passes

Twelve crates under `mock/benches/variants/` run green by `--manifest-path`, 78 tests. `wide-rung-shared`
takes 133.69s on this host for its 30. `bitpack-write-contend-shared` needs `-- --test-threads=1` and then
gives 15 in 3.03s under `--release`. 78 + 30 + 15 = 123, which confirms the brief's figure.

**That is the fourth independent reproduction of the livelock requirement**, after `139`'s, `141`'s at
46.65s and `143`'s at 2.57s. The mechanism is `139`'s and I did not re-derive it; I confirmed the flag
fixes what I observed. The brief's own warning about counting the token by grep is correct and I did not
need to rediscover it.

**Test quality, on the surface I read.** I read the bodies in `bitpack-carrier-shared`, which no prior
member reports reading, because it is the crate closest to my subject: it is the one whose variants are an
*arm set* in the sense my probes use. It is not decorative. `check_size` decodes the buffer by hand and
compares all four carriers element by element rather than by sum, with a comment saying that a permutation
defect or a compensating pair of errors passes a sum check and fails this one. `all_four_transforms_agree`
cross-checks the dense arms against the packed decoder on the same column.
`validate_output_rejects_a_wrong_sum` feeds an answer off by one and asserts the validator refuses it, with
a doc comment saying that a validation pass which cannot fail is not a validation pass. `total_input_bytes_matches_struct_size`
asserts the declared buffer size against `size_of` at two different `N`, which pins the harness's cast
rather than restating a constant.

I found no tautology, no assertion of a value against itself, no arm compared against itself, and no
sampled law that struck me as choosing what not to find out. Three prior members reached the same verdict
on other crates; I make it four, on a fifth crate.

---

## 1. The brief, tested first

Four factual claims in my dispatch, checked before the assigned work.

**"141 and 142 both say nobody has looked at the weighting side."** True, and I quote both above. Verified
by opening `141:874-876` and `142:581-584`.

**"The suite is 123 across 13 crates."** True, confirmed by running it.

**"That diagnosis has three independent reproductions."** True: `139` at 7.97s, `141` at 46.65s, `143` at
2.57s. Mine is a fourth.

**"The weighting continuum quotients to 5 cells over 7 arms, so a continuous parameter has finite
observable content."** The number reproduces exactly. The *inference* does not need it, and this is a
correction to the brief and to `139:430-431` together, which says "the observable weighting count is
bounded by the arm count, so however continuous the weight space is, the design's vocabulary for it need
never exceed the number of arms."

That is true and it is a fact about functions rather than a fact about weightings. A selector is a map from
weight space into a finite set of arms, so its image is finite and no larger than that set, whatever the
selector is. No sweep can fail to find it and no measurement establishes it. `139`'s `p4` reports it as a
measured result, and the useful content of the measurement is the *number* five, which is a property of the
committed cost tables and not of the design.

**The bound that is not free** is the one nobody stated: for a linear selector the image is not the arm set
and not the Pareto set. It is the set of arms that lie on the lower convex hull, and that is a much smaller
object. Section 7 measures it.

---

## 2. Reproduction, and what matching means here

`RULES.md` says reproduce before refuting. One member in this topic deliberately did **not** match a count,
because its axis set differed from the count it was checking and matching would have shown its instrument
was not independent (`143:194-196`). My case is the opposite and it is worth saying why.

`139`'s arm tables are literal committed constants in `139_probes/p4_weight_cells.rs`, and every quantity I
am checking is a deterministic function of them. So an independent implementation **must** land on the same
number, and a disagreement would be a defect in one of the two instruments rather than a finding.
Independence here lives in the code, not in the input: `p1` is Python against `139`'s Rust, with its own
argmin, its own dominance test, its own sweep, and exact `Fraction` arithmetic where `139` used `f64`.

| quantity | `139` | mine | verdict |
|---|---|---|---|
| arms, both targets | 7 | 7 | matches |
| Pareto-nondominated, both targets | 6 | 6 | matches |
| distinct winners (cells), both targets | 5 | 5 | matches |
| same weight picks a different arm | 838 of 1891 (44.3%) | 838 of 1891 (44.3%) | matches |
| cell populations, target 2 | 20102 / 3514 / 4153 / 683 / 52149 | identical | matches |
| cell populations, target 1 | 55720 / 4514 / 2269 / 52 / 18046 | 55720 / **4515** / **2270** / 52 / **18044** | differs by 4 of 80601 |

**The one disagreement is the arithmetic, and it is worth a sentence rather than a section.** Four grid
points of 80601 sit on a cell boundary where `f64` rounding and exact rationals land on opposite sides. The
totals agree, the cell count agrees, and no conclusion moves. It is recorded because a later reader
re-running `139`'s probe in exact arithmetic will see different populations and should know why.

---

## 3. The Pareto claim: right, strengthened, and misattributed inside its own file

This is the claim the dispatch says to go at hardest, on the ground that a negative result of the form "no
weighting can select this arm" is exactly the shape a search bug produces for free. It is not a search bug.
It is a theorem, and `139`'s own control was already the right one.

### 3.1 The zero belongs to the arm, and I can say more than the sweep can

`139`'s witness is three arms in two coordinates: endpoints at (0,10) and (10,0) and a compromise at (6,6).
It reports the compromise winning zero of 2001 sweep points, with a control that pulls the same arm inside
the hull and gets 399 wins (`139:400-407`).

A sweep can only report that an arm did not win at the points sampled. So I replaced the sweep with an exact
decision procedure: minimise, over the whole simplex, the largest amount by which the arm's weighted cost
exceeds the best other arm's. That is a small linear program, and it is solved exactly over rationals in
`p1` by enumerating every vertex of the feasible region.

**The answer is +1, exactly.** The compromise arm loses by at least one full unit at every point of the
simplex, not merely at 2001 of them. `139`'s control arm at (4,4) comes out at -1, so it is strictly
selectable. Both directions are decided rather than sampled.

That is a strengthening, not a correction. The underlying fact is the standard one about weighted-sum
scalarisation: it can only ever reach points on the lower convex hull, so an efficient point above the hull
is unreachable regardless of how finely anyone sweeps. `139` discovered it by construction and its
construction is sound.

### 3.2 Two of my own controls fired, and both taught me something

`p1` ran a cross-check that every grid winner must be strictly selectable. **It failed on
`scalar-widened`,** which wins 55720 grid points and is not strictly selectable. Both are correct: the arm
has an exact duplicate in the set, so no weight makes it the unique minimum, and the grid awards it the
tie by index order. Grid-winner and strictly-selectable are different predicates and I had compared across
them. `p1b` repairs it: every grid winner is *weakly* selectable, in both tables.

`p1` also asserted the witness's optimum would be strictly positive and got **zero**. The reason is a
finding rather than a fix. `139`'s witness carries a third coordinate on which every arm scores zero, and
putting all the weight on that coordinate makes every arm score zero and tie. `139` never saw it because
its sweep walks only the edge `(t, 1-t, 0)`. In the witness's true dimension, which is two, the optimum is
+1 as reported above.

**The general form is a design fact and it is new.** A cost coordinate on which every arm carries the same
value makes *every* arm weakly selectable, because all the weight can go there and the argmin is then
decided entirely by the tie-break rule. `p1b` confirms it on a set where three of four arms are strictly
separated before padding and all four are only weakly separated after. That composes with `139:433-436`'s
own point about coordinates from the other side: a coordinate every arm carries a value on is necessary,
and a coordinate the arms do not *differ* on is worse than useless, because it is a direction in weight
space along which the selector says nothing.

### 3.3 The six-against-five gap in the main table is the duplicate, not the phenomenon

`139:400-407` reads the seven-arm tables' strict inequality between the Pareto count and the winner count
as the confirmation of its expectation, and then builds the minimal witness separately.

**The strict inequality in those tables is a different thing entirely.** `p1` decomposes it: the single
Pareto-but-never-winning arm in each table is `scalar-widened [dup]`, `139`'s own C3 control arm, and
deleting it gives Pareto 5 and cells 5 with a gap of zero, on both targets. So the six-against-five is an
exact-duplicate arm losing a tie-break, which is the tie phenomenon of 3.2 and not an unsupported efficient
point.

The gap detector was controlled in both directions before I trusted it: on a set whose every arm is a hull
vertex it reports a gap of zero, and on a set with a genuine unsupported point and no duplicate it reports
the gap. Both fire.

**This does not touch the claim.** The witness carries it and the witness is sound. What it corrects is
that the seven-arm table is not evidence for it, and a later reader taking the 6-versus-5 as a second
instance would be counting a control arm.

### 3.4 How common is it, and is it worth removing

`p2` sweeps random integer cost tables and asks what fraction of Pareto arms no linear weighting can reach.

| coordinates | 4 arms | 6 arms | 8 arms |
|---|---|---|---|
| 2 | 8.0% | 7.9% | 9.5% |
| 3 | 6.6% | 11.2% | 11.7% |
| 4 | 7.7% | 11.5% | 12.7% |

**My prediction of at least 15% at three coordinates and eight arms fell**, at 11.7%. The rate rises with
the arm count at every coordinate count and rises with the coordinate count everywhere except the
four-arm column, so my second prediction fell too, at one cell out of three.

Roughly one Pareto arm in nine is unreachable at the shape `139` proposes. That is not exotic, and it makes
the question of whether to remove the limit a real one rather than a curiosity.

`p9` prices what the limit costs, in the only currency available without a harness: the criterion the
unreachable arm itself optimises. Over 201 unreachable arms in 150 random sets, the gap between that arm
and the best linearly reachable one, at the unreachable arm's own certificate weight, is a **median of
17.6% and a mean of 21.1%**, with a maximum of 72.0%. Zero negative gaps, which is the control, since a
negative would mean the gap formula was inverted.

### 3.5 The replacement: a weighted Chebyshev selector reaches every Pareto arm

An attack that proposes nothing has done half a dispatch, so here is the replacement, with its proof, its
verification, its doability check and its costs.

Take the componentwise minimum of the arm set and subtract one, giving a reference point strictly below
every arm on every coordinate. Score an arm by the largest weighted deviation from that reference rather
than by the weighted sum. Then for any arm `i`, the weight `w_k` proportional to `1 / (c_ik - z_k)` makes
every one of arm `i`'s terms equal, so its score is exactly 1, and any other arm scores above 1 exactly
when it is strictly worse on some coordinate. A non-dominated arm with no exact duplicate is strictly worse
on some coordinate for every rival, so **it is strictly optimal at its own certificate weight**. A
dominated arm is not, because its dominator is no worse anywhere.

That is a construction rather than a search, so the reachability of each arm is decided rather than swept.
`p2` verifies it exactly on every arm of every random set it drew: **3667 Pareto arms checked, 3667
reached, zero missed**, with zero dominated arms receiving a certificate and zero linearly selectable arms
lost.

The illustration that makes it concrete is `p3`'s collinear set. Five cost points on one straight tradeoff
frontier: a linear selector can name exactly the two extremes and nothing between them, and a Chebyshev
selector names all five.

**It costs nothing at compile time**, which is the doability obligation and is why `p8` exists. A const
`max` over integer products is nothing a const evaluator struggles with, and the emitted code on
`aarch64-apple-darwin` at `opt-level=3` is:

| entry point | body | arm_a | arm_b | arm_c | conditional branches |
|---|---|---|---|---|---|
| linear, w=(1,3) | 2 lines | 0 | 1 | 0 | **0** |
| linear, w=(1,1) | 2 lines | 1 | 0 | 0 | **0** |
| Chebyshev, w=(1,1) | 2 lines | 0 | 0 | 1 | **0** |
| runtime-selected control | 12 lines | 1 | 1 | 1 | **3** |

Each const-selected entry is one unconditional branch to one arm. The control keeps all three arms and its
branches, which is what makes the absence above a result rather than a limitation of the scan. `139`'s own
`p5` scan first reported zero arms everywhere because it was looking at `extern "C"` thunks one hop above
the monomorphised symbol; `p8` removes that hazard at the source by making the dispatch `inline(always)`,
so each entry point's own body *is* the dispatch.

On the same cost table the two selectors disagree at 624 of 1600 integer weights, and the compromise arm is
picked by Chebyshev and never by linear, which is the point of the replacement rather than a side effect.

### 3.6 What the replacement costs, measured against itself

`p9` attacks it, because an offer with only its advantages measured is half a dispatch.

**Plain Chebyshev can select a dominated arm.** This is the known defect: ties at the maximum are not
broken by the remaining coordinates. Measured, it picks the dominated arm at 11 of 41 weights on a
two-arm witness. The augmented form, adding a small multiple of the summed deviation, picks it at zero.
**So the design ships the augmented form or it ships a selector that can pick a strictly worse arm.**

**I expected it to travel worse, because it carries a second target-dependent object in the reference
point. It does not.** Over 120 independent target pairs the linear selector switches arms at 86.9% of
weights and the Chebyshev selector at 86.9%, indistinguishable at one decimal. **And my second prediction
here fell outright**: declaring the reference point in the design rather than computing it per target does
not reduce the switch rate, it moves it to 87.1%. That closes a route rather than opening one.

**I did not witness one thing I predicted.** A linear selector's win regions are intersections of
half-spaces and therefore convex, so each arm wins on one connected interval of the weight edge. A
Chebyshev region need not be, and I searched 397 random two-coordinate sets at a grid resolution of 1/2000
without finding a split region. **H3 is unestablished and I do not use it.** The linear side of the same
walk reported one run per arm in every set, which is the control that says the walk could have seen a split.

### 3.7 What this does to the two-component object

The dispatch asks for this directly, and the answer is narrower than it first looks.

**The object survives. The count of components is not what is wrong.** Fifteen probe entries produced no candidate
for a third component and no reason to merge the two, and section 5 is an argument for the split rather
than against it: the whole reason a target change is harmless is that one component fixes the answer and
the other only picks among ways of computing it.

**What does not cover its own space is the selector inside component two.** As the topic states it,
component two is a weighting over cost coordinates, and a weighting is read as a linear objective. Under
that reading the component cannot name every arm the arm set contains: it names the vertices of the lower
convex hull and nothing else, and `p3`'s collinear set makes the shape of the gap visible by naming two of
five points on one straight frontier.

**So the correction is one word inside component two, not a change to the pair.** Either the component is
defined with a selector that reaches every non-dominated arm, which is O-144-A and is measured to cost
nothing at compile time and nothing in portability; or the component keeps the linear selector and the
design records, as part of what a weighting *is*, that it names hull vertices rather than arms. Both are
honest. What is not honest is leaving "a weighting over cost coordinates" to be read as "any arm can be
asked for", because that is false at roughly one non-dominated arm in nine.

**And the choice between the two is not urgent, on the evidence.** `p10c` finds no established instance of
the limit anywhere in arvo's committed corpus, so the linear selector is sufficient for every arm set the
library has today. The design should say which of the two it is doing rather than resolve it under
pressure, and the moment an unsupported arm appears in a real arm set the answer changes.

---

## 4. Portability: the number is a low outlier and the phenomenon is misnamed

### 4.1 44.3% carries no information about weightings

`139:396` reports that the same weight vector selects a different arm at 838 of 1891 grid points across its
two cost tables, and reads it as the observable weighting structure being target-dependent in the way that
matters.

`p4` asks what an ordinary pair of tables gives. Over 200 random independent pairs at three coordinates and
seven arms: minimum 33.8%, **median 90.2%, mean 84.4%**, maximum 100.0%.

**`139`'s 44.3% sits at the 2nd percentile of that distribution.** Its two invented targets are unusually
similar, not unusually different. The conclusion it draws is strengthened rather than weakened by this, and
the *number* is a fact about how much two cost tables differ. Nothing in the design should be gated on its
size, and a later reader who cites 44.3% as a magnitude is citing a property of two invented tables.

### 4.2 What travels is the weighting, and the arm never travels

This is the part I think the topic has had backwards, and it is one measurement away from being obvious.

A weighting carried to a new target re-resolves there, and by construction it lands on that target's
optimum for that weighting. `p4` confirms it as a code check rather than as a finding: zero non-optimal
cases in the whole sweep, which is definitional and would have indicated a defect if it were anything else.

An arm carried to a new target is frozen. `p4` measures the relative regret of that over 20,874 cases where
the frozen arm is not the new optimum: **mean 200.0%, median 128.2%, worst 5900.0%.** On identical targets
both are exactly zero, which is the control.

**So a fixed weighting selecting a different arm on a different target is not a portability failure. It is
the weighting working.** The design owes one sentence and it is not the one the 44.3% invites:

> A weighting travels. An arm does not. A predicate that names an arm is bound to the target it was
> measured on; a predicate that names a weighting is not.

That has a direct consequence for this panel's own notation. Under `RULES.md:486-541` a finding lists the
region it holds in, and a finding whose region names a specific arm has silently acquired a target
dimension it did not list.

### 4.3 The one thing that does make a selection travel, and exactly how far

This is the composition with `140`'s shared-baseline obligation, and it is the reason the two untouched
claims belong in one file.

Normalising every arm's cost by one named baseline arm's cost, per target, is a per-coordinate rescaling of
the cost table. So it should remove exactly the part of a target difference that *is* a per-coordinate
rescaling, and nothing else. Both halves are measured in `p4`:

- On target pairs differing by a pure per-coordinate rescaling, the un-normalised switch rate is a mean of
  18.0% and a maximum of 49.2%. **Normalised, it is exactly 0.0% at every pair.**
- On arbitrary target pairs it moves from 85.8% to 83.0%, a change of 2.8 points.

The control that had to fire did: the rescale moves the selection before normalising, so there was
something to remove.

`p9` shows the same holds for the Chebyshev selector, whose reference point is also rescaled away: 23.8%
before, exactly 0.0% after.

> A weighting travels across a target change exactly to the extent that the change is a per-coordinate
> rescaling of the cost table, and one shared baseline arm normalised per target is what makes that part
> travel. The rest of a target change reorders arms genuinely and no normalisation recovers it.

### 4.4 On real arms, the reordering is established in six families

`p10c` applies the same question to the committed bench corpus, where the "targets" are input sizes and a
weighting over them is a consumer saying which working-set sizes they care about. Of 24 families whose
smallest-size winner differs from their largest-size winner, **6 survive a gate requiring both deciding
gaps to exceed the harness's own scatter**, several by large margins: `warm-clamp-arity-l2` at 97464 ns
against a 1601 ns threshold at the small end and 26284 ns against 4217 ns at the large.

**My prediction that two thirds would survive fell**, at 6 of 24. Six is enough to establish the
phenomenon on committed measurements rather than on invented tables, and the eighteen that fail are a
reminder that most apparent reorderings in this corpus are inside the noise.

---

## 5. What the firewall is actually buying, which nobody had measured

`139` proposed the observability firewall and gave one reason: without it "two builds of one program
produce different results with no predicate anywhere naming the difference". `141` endorsed the
proposition and attacked only the repair; `142` conceded the repair and kept the proposition. So it stands
at two experts with its consequence for the weighting side unmeasured, because neither attacker touched the
weighting side.

`p6` joins it to section 4 on real fixed-point arms rather than on cost vectors, using the two arm pairs
this topic already established.

**Conforming arms**, two routes to one saturating fixed-point multiply differing in how the product is
formed: **0 disagreements** over the whole input space at `F` in {0, 3}. That is a third instrument on
`139`'s `p2` part A result.

**Non-conforming arms**, the fused and stepwise multiply-add: **42.14% of triples at `F = 0` and 31.96% at
`F = 3`**, which reproduces `139`'s signed saturating row and `141`'s digit for digit on a third model.

Then the cross-target question, with cost tables making different arms win on different targets:

| | selection differs | answer moves |
|---|---|---|
| conforming arms, F=0 | 47 of 325 weights | **0** of 192,512 cases |
| conforming arms, F=3 | 47 of 325 weights | **0** of 192,512 cases |
| non-conforming, F=0 | 47 of 325 weights | 5,192,372 cases, **42.14% of inputs per weight** |
| non-conforming, F=3 | 47 of 325 weights | 3,938,224 cases, **31.96% of inputs per weight** |

At weights where the selection agrees, zero in every cell. On identical targets, zero. Both controls fire.

**So a selection change exposes the arms' whole disagreement, not a fraction of it**, and the composed
statement is one the canon can carry:

> With the firewall, a target change moves which arm runs and moves no answer. Without it, a target change
> moves answers at the arms' full disagreement rate, and no predicate names the change, because the cost
> table is not in the type.

That upgrades `139`'s own objection from a hypothetical to a quantity, and it says why the firewall is not
merely a rule about what a cost model may do. **It is the precondition that makes a weighting a portable
object at all.**

---

## 6. The shared-baseline obligation, given the second read it has been owed for three files

`140:620-624` states it: "every strategy's cost claim is stated against the same named arm." `141:776-779`
declines to be the second read and says so. `143:418-420` calls it "the oldest unexamined thing in my
file" and points the next dispatch at it. `142` did not touch the weighting side. So it has stood at one
expert across three files, and it is squarely on my surface, because a baseline is a transformation of a
cost table and a cost table is what a weighting reads.

**The obligation is right and it is under-specified, and the gap is the dimension.**

`140`'s evidence sweeps triples of **scalar** costs: a shared baseline reorders nothing in 840 comparisons
and per-arm baselines reorder 56.3% of 35724. `p5` reproduces both exactly, including the 20106. The scalar
half is a theorem rather than a measurement: `base / cost` is strictly decreasing in cost for a fixed
positive base, so ranking by the reported figure is ranking by cost, and nothing could have come out
otherwise.

But a strategy's second component is a weighting over cost **coordinates**, so a cost is a vector and "the
ranking by absolute cost" is not defined until a weighting fixes it. That leaves two inequivalent readings
of what one shared baseline does, and `140`'s probe cannot distinguish them because it has one coordinate:

**Per coordinate, before the weighting.** Divide coordinate `k` by the baseline's value on coordinate `k`,
then weight. Weighting the normalised cost by `w_k` is weighting the raw cost by `w_k / b_k`, so this is a
**change of weighting** wearing a normalisation's clothes.

**Once, on the weighted scalar.** Weight first, then divide the single resulting number by the baseline's
weighted cost. That divides every arm's figure by one positive constant, which is the scalar case again.

`p5` measures both on 34440 cases of an arm set and a weight:

- per coordinate, before: **changes which arm a fixed weighting picks at 24.6%**
- once, on the scalar: **changes it at 0.0%**

The two arms control each other, which is `140`'s own mutual-control shape: if both came out zero the
comparator would not be comparing. A witness, printed by the probe: baseline arm `(44, 24, 17)`, weight
`(1/40, 1/10, 7/8)`, raw cost picks `(15, 5, 8)`, baseline-normalised cost picks `(57, 41, 4)`. Same arms,
same weighting, one named shared baseline, two answers.

The case that had to fail did: when the baseline arm costs the same on every coordinate, the per-coordinate
form moves nothing in 10380 cases, because a uniform rescale is a scalar rescale. And `140`'s hazard
survives the dimension change: per-arm baselines report a winner that is not the winner at 33.5% under
vector cost.

**The repair, addressed to `140` and `143`.** One named baseline arm is necessary and it is not sufficient.
Where the division happens decides whether it is a normalisation or a reweighting, and the obligation has
to say so:

> Every strategy's cost claim is stated against the same named arm, and the comparison is made after the
> weighting, on the weighted scalar. A per-coordinate normalisation applied before the weighting is a
> change of weighting, and the arm a consumer asked for is not the arm they get.

And section 4.3 is the other face of the same coin: the per-coordinate form is exactly the transformation
that makes a weighting travel across a rescaled target. It is not wrong, it is a different operation with a
different purpose, and using one where the other is meant is the failure.

---

## 7. How many named weightings can ever be told apart

`139:430-431`'s bound is the arm count. The real bound is much smaller and it is a design number, because
the coordinate set is a design choice (`139`'s O-139-E) and the arm count is not.

`p3` and `p3b` measure the size of the linear selector's image over random arm sets. The mean number of
arms a linear weighting can ever pick uniquely:

| coordinates | 8 arms | 16 arms | 24 arms |
|---|---|---|---|
| 1 | 1.00 | 1.00 | 1.00 |
| 2 | 2.45 | 2.60 | 2.70 |
| 3 | 3.55 | 4.70 | 5.75 |
| 4 | 4.45 | 6.90 | 8.65 |
| 5 | 5.70 | 9.05 | 12.50 |

**My saturation prediction fell.** I expected the image to be nearly flat in the arm count, and above three
coordinates the arm count moves it more than a coordinate does. What survives is the level rather than the
growth, and the level is the useful part: **at three cost coordinates, which is what `139` proposes, a
linear selector can distinguish four to six arms however many exist.**

So a design that ships three cost coordinates and many more than a handful of named weightings has named
points nothing can tell apart. The Chebyshev image is the non-dominated count exactly, by the certificate
of 3.5, so it is wider at every shape measured and widest where the frontier is flat.

The controls are what make the table worth reading. At one coordinate the image is exactly 1 at every arm
count, which is the case that must fail and does not. No dominated arm is ever strictly selectable. And the
separation LP the table is computed with was checked against the slow full enumeration on 770 arms with
**406 strictly negative, 63 exactly zero and 301 strictly positive optima and zero mismatches**, so the
agreement covers the boundary rather than only the easy side.

**Two dead solvers are recorded in `exact_lp.py` rather than deleted.** Full vertex enumeration over every
constraint did not finish in ten minutes at five coordinates and 24 arms. Wrapping it in an active set that
never dropped a constraint did not finish either, because the working set grew until the inner enumeration
was the outer one. What runs is the dual characterisation, decided by an exact phase-one simplex.

---

## 8. `143`'s second argument of the count

`143:296-303` accepts `141`'s joint statement and adds one thing: the count `|A / ~_O|` has two arguments
that move, because the assignment set `A` "is parameterised by where the denotation and realisation levels
are cut". It prices it from `140_probes/p6_out.txt`, where the full product gives 24 classes and cutting
the intermediate axis gives 12 or 14, and concludes that a shape-to-count table "is not currently writable".

The arithmetic is right. What I attack is that the cut is free, and the attack has a measured half and an
argued half.

**Measured.** `p7` sweeps six shapes by four observation sets by four axes and finds a biconditional with
no exceptions: **cutting an axis out of `A` changes the count if and only if that axis is answer-visible
under `O`.** Where the axis is invisible the cut is a no-op, so there is nothing to parameterise. Where it
is visible the cut changes the count for exactly the same reason enlarging `O` does.

And the same cut is a no-op under one observation set and not under another. At unsigned `F = 1` the
intermediate axis is invisible under `{add, sub, mul}` and the cut moves nothing, while under all five
operations it moves the count from 7 to 5. **So the level of the denotation line is not a free parameter;
it is read off the observation set.**

The controls are three. A dead axis whose two positions are literally the same function never moves a count
at any shape or observation set. The intermediate axis comes out visible in 12 cells and invisible in 12,
so the biconditional is tested in both directions rather than one. And the partitioner separates wrap from
saturate and merges the dead axis's positions.

**Argued, and marked as argument.** An answer-visible axis has positions denoting different answers. Under
Q51's repair, component one fixes the denoted answer and component two ranges over realisations of it, so
an answer-visible axis is component one by that criterion, and cutting it out of `A` puts two denotations
under one type. That is what the firewall forbids and what `141` and `142` both endorsed.

**So the second argument is either a no-op or forbidden, and there is no live case in between.** The count
has one argument that moves, the observation set, plus the axis set the design ships. A shape-to-count
table is writable once the axis set is named, and does not additionally wait on a denotation line.

The last step rests on a proposition at two experts rather than on a measurement, and I have marked it
where it appears. `143`'s numbers stand; it is the inference from them that I am contesting.

---

## 9. Replacements owed, addressed to whoever I refuted

**To `139`, on the selector.** Ship an augmented weighted Chebyshev selector rather than a linear one. It
reaches every Pareto arm by construction, verified on 3667 of 3667; it const-evaluates and lowers to one
unconditional branch, verified in emitted assembly with a runtime-selected control; it costs the same in
portability, at 86.9% against 86.9%; and the plain form's ability to pick a dominated arm is removed by the
augmentation, at 11 of 41 weights down to 0. What it buys is a median 17.6% in the consumer's own
criterion, in the roughly one case in nine where the wanted arm is unsupported.

**To `139`, on the same thing, if the above is too much mechanism.** Keep the linear selector and record
the limit as a named predicate rather than as a footnote: a linear weighting reaches an arm exactly when
that arm is a vertex of the lower convex hull of the arm set, and a design that wants an interior
compromise arm has to name it directly rather than reach it by weight. This is cheaper and it forecloses
nothing, because `p10c` finds no established instance of the limit in arvo's current corpus.

**To `139`, on the seven-arm table.** Drop the six-against-five reading. The gap there is the C3 control arm
losing a tie-break. The witness carries the claim on its own and does not need it.

**To `139`, on the quotient.** State the finite-observable-content property as the fact about functions it
is, and keep the number five as what the measurement produced about those tables. The measurement is worth
having; it is not what establishes finiteness.

**To `139`, on cost coordinates.** Add the second half of the coordinate criterion: a coordinate exists only
if every arm carries a value on it, **and** only if the arms differ on it. A coordinate they do not differ
on makes every arm weakly selectable and hands the decision to the tie-break rule.

**To `140` and `143`, on the shared baseline.** The obligation is right and needs one clause: the
comparison is made after the weighting, on the weighted scalar. Per-coordinate normalisation before the
weighting changes which arm a fixed weighting picks at 24.6%.

**To `143`, on the second argument.** Replace "the count is a function of two arguments and the second
moves" with "the count is a function of the observation set and the axis set, and removing an axis is a
no-op wherever the axis is answer-invisible". The shape-to-count table is writable; what it waits on is the
axis set being named, which is a design act rather than an open question.

**To the topic, on what the canon owes about targets.** Two sentences, both measured: a weighting travels
and an arm does not; and the firewall is what makes the first sentence safe.

---

## 10. Findings, with predicates

Per I13 and `RULES.md:486-541`. An absent dimension claims nothing anywhere that dimension is present.
Every probe here runs on one thread, so none of these findings holds anywhere threads exist.

**F144-1. `139`'s weight-cell geometry reproduces exactly on an independent instrument**, at 7 arms, 6
non-dominated, 5 cells on both targets, and 838 of 1891 grid points switching, with four grid points of
80601 differing on target 1 between `f64` and exact rational arithmetic.

```
holds for: the two committed cost tables in 139_probes/p4_weight_cells.rs,
           cost coordinates = 3, arms = 7, selector = linear,
           weight grid resolution in {1/400, 1/60} on the 2-simplex,
           arithmetic in {IEEE binary64, exact rational},
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-2. `139`'s compromise arm loses by at least one unit at every point of the simplex**, decided
exactly rather than swept, and its control arm inside the hull is strictly selectable at -1.

```
holds for: arms = 3 at costs (0,10), (10,0), (6,6) and the control (4,4),
           cost coordinates = 2, selector = linear,
           decision procedure = exact vertex enumeration over rationals,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-3. The six-against-five gap in `139`'s seven-arm tables is its exact-duplicate control arm.**
Deleting it gives 5 non-dominated and 5 cells with a gap of zero on both targets.

```
holds for: the two committed cost tables in 139_probes/p4_weight_cells.rs,
           cost coordinates = 3, arms in {6, 7}, selector = linear,
           tie-breaking = lowest index, weight grid resolution = 1/400,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-4. A cost coordinate on which every arm carries the same value makes every arm weakly selectable.**

```
holds for: arms = 4, cost coordinates in {2, 3} with the third constant across arms,
           selector = linear, decision procedure = exact vertex enumeration,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-5. Between 6.6% and 12.7% of non-dominated arms are unreachable by any linear weighting**, rising
with the arm count at every coordinate count.

```
holds for: cost tables drawn uniformly from integers 1..20,
           arms in {4, 6, 8}, cost coordinates in {2, 3, 4},
           120 tables per cell, no exact duplicate arms,
           selector = linear, decision procedure = exact vertex enumeration,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-6. A weighted Chebyshev selector with a utopia reference reaches every non-dominated arm**, at 3667
of 3667, with zero dominated arms certified and zero linearly selectable arms lost.

```
holds for: cost tables drawn uniformly from integers 1..20,
           arms in {4, 6, 8}, cost coordinates in {2, 3, 4},
           reference point = componentwise minimum minus one,
           certificate weight proportional to the reciprocal deviation,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-7. A const Chebyshev selector lowers to one unconditional branch to one arm**, with a
runtime-selected control retaining all three arms and three conditional branches.

```
holds for: selector in {linear, weighted Chebyshev}, arms = 3, cost coordinates = 2,
           integer weights and integer reference point, selection by associated const,
           dispatch inline(always), arms inline(never),
           opt-level = 3, no feature gates beyond the stable language,
           threads = 1,
           target = aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f
```

**F144-8. Plain Chebyshev selects a dominated arm and the augmented form does not**, at 11 of 41 weights
against 0 of 41.

```
holds for: arms = 2 with one dominating the other, cost coordinates = 2,
           augmentation coefficient = 1/1000, weight grid resolution = 1/40,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-9. The linear limit costs a median 17.6% in the criterion the unreachable arm optimises.**

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms in {5..9}, cost coordinates = 3,
           150 tables containing at least one unreachable non-dominated arm,
           gap measured at the unreachable arm's own certificate weight,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-10. Across independent cost tables a fixed weighting selects a different arm at a median of 90.2%
of the simplex**, and `139`'s 44.3% sits at the 2nd percentile of that distribution.

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms = 7, cost coordinates = 3, 200 independent target pairs,
           selector = linear, weight grid resolution = 1/40,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-11. Carrying an arm across a target costs a median 128.2% relative regret and carrying a weighting
costs exactly zero.**

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms = 7, cost coordinates = 3, 200 independent target pairs,
           selector = linear, weight grid resolution = 1/40 sampled every seventh point,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-12. Normalising per target by one shared baseline arm drives the cross-target switch rate to exactly
zero on pure per-coordinate rescalings and moves it 2.8 points on arbitrary target pairs**, for the linear
selector, and to exactly zero from 23.8% for the Chebyshev selector.

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms = 7, cost coordinates = 3,
           rescale factors drawn uniformly from integers 1..8,
           100 pairs per arm, selector in {linear, weighted Chebyshev},
           weight grid resolution in {1/40, 1/30},
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-13. A per-coordinate shared-baseline normalisation applied before the weighting changes which arm a
fixed weighting selects at 24.6%; applied once to the weighted scalar it changes it at 0.0%.** The
per-coordinate form moves nothing when the baseline arm's cost is uniform across coordinates.

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms = 7, cost coordinates = 3, baseline arm drawn uniformly,
           34440 (arm set, weight) cases, selector = linear,
           weight grid resolution = 1/40 sampled every third point,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-14. Under per-arm baselines with vector cost, the reported winner is not the winner at 33.5%.**

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms = 5, cost coordinates = 3, each arm's baseline drawn independently,
           20760 cases, selector = linear, weight grid resolution = 1/40 sampled every fifth point,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-15. A target-dependent selection is answer-invisible with conforming arms and exposes the arms' full
disagreement rate with non-conforming ones**, at 0 of 192,512 cases against 42.14% of inputs per weight at
`F = 0` and 31.96% at `F = 3`.

```
holds for: numeral fixed-point signed, W = 6, F in {0, 3},
           overflow = saturating, rounding = truncate toward zero,
           conforming pair = two routes to one multiply,
           non-conforming pair = fused and stepwise multiply-add,
           operations {multiply, multiply-add}, arity in {2, 3}, chain length in {1, 2},
           container width = declared width,
           cost coordinates = 3 with two committed synthetic tables,
           selector = linear, weight grid resolution = 1/24,
           inputs exhaustive over the declared range,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-16. Cutting an axis out of the assignment set changes the class count if and only if that axis is
answer-visible under the observation set.** Biconditional with no exceptions over 96 cells.

```
holds for: numeral fixed-point, W = 4, F in {0, 1, 2},
           signedness in {unsigned, signed},
           assignments = rounding {toward zero, floor}
             x overflow {wrap, saturate both, saturate high only}
             x intermediate {stepwise, exact}
             x a dead axis with two identical positions,
           observation sets = {add, sub, mul}, {madd}, {mul, madd},
             {add, sub, mul, madd, msub},
           inputs exhaustive over the declared range,
           container width = declared width,
           overflow limit read at the declared width,
           accumulator width = unbounded,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-17. A linear selector can ever pick uniquely between 1.00 and 12.50 arms**, governed strongly by the
coordinate count, and at three coordinates it is 3.55 to 5.75 arms whatever the arm count.

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms in {8, 16, 24}, cost coordinates in {1, 2, 3, 4, 5},
           20 tables per cell, no exact duplicate arms,
           selector = linear, decision procedure = exact phase-one separation,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F144-18. No arm in arvo's committed bench corpus is established as non-dominated and linearly
unreachable.** Zero of 35 families, once every deciding gap is required to exceed the harness's own
within-arm interquartile range.

```
holds for: the committed CSVs under mock/benches/ as of this file,
           35 bench families with at least 3 size points and at least 4 variants,
           cost vector = median algo_ns per size point,
           cost coordinates = the family's size points, 3 to 12,
           arms = the family's variants, 4 to 9,
           selector = linear,
           gate = every deciding gap above the larger of the two arms' half-IQR,
             and unreachability retested at the optimistic band edge,
           threads as the harness ran them,
           target features = host (aarch64-apple-darwin)
```

**F144-19. The winner moves across the size axis in 6 of 24 committed bench families**, with both deciding
gaps above the harness's own scatter.

```
holds for: the committed CSVs under mock/benches/ as of this file,
           24 families whose smallest-size and largest-size winners differ on the medians,
           cost vector = median algo_ns per size point,
           gate = both deciding gaps above the larger of the two arms' half-IQR,
           threads as the harness ran them,
           target features = host (aarch64-apple-darwin)
```

---

## 11. What I carry forward unchanged, with a count

**Eleven positions kept, from four members.** Marked where I established the agreement independently,
because that is the only thing that earns the two-expert rung.

From `139`:

1. **The two-component pair.** Fourteen probes, none of which produced a candidate for a third component or
   a reason to merge the two. Kept.
2. **A Pareto-optimal arm can be unreachable by any linear weighting.** *Independently established and
   strengthened*, `p1`/`p1b`, by exact decision rather than by sweep. Second instance, and the strengthening
   is mine.
3. **The weighting continuum has finite observable content.** Kept with the correction in section 1 that it
   needs no measurement.
4. **The 44.3% mapping difference.** *Independently reproduced to the grid point*, `p1`. Second instance.
5. **The observability firewall.** I built nothing against it and section 5 is a measurement of what it
   buys rather than a test of whether it holds. Carried as `139`'s, at two experts on `141`'s and `142`'s
   agreement.
6. **The livelock diagnosis of `bitpack-write-contend-shared`.** *Independently reproduced*, fourth
   instance. The mechanism is `139`'s.
7. **Packing is answer-invisible**, at the column, per `141`'s narrowing. Not tested by me; carried.

From `140`:

8. **The shared-baseline obligation.** *Independently reproduced on its own scalar ground*, `p5`, including
   the 20106 of 35724. Second instance for the scalar half, and section 6 adds the dimension clause it was
   missing.

From `141`:

9. **The fused and stepwise multiply-add differ at 42.14% at signed saturating `F = 0`.** *Independently
   measured*, `p6`, on a third model. Fourth instance after `139`, `141` and `142`.
10. **The count is a quotient with the observation set as its parameter.** *Independently supported*, `p7`,
    whose biconditional is what makes the parameterisation exact.

From `142`:

11. **The intermediate axis positions and the fusion arms are the same two functions.** Not retested; `p6`
    uses them as one pair on that basis and I carry the identity as `142`'s and `141`'s.

**The dimensions each convergence actually varied**, because `143:169-184` establishes that two agreeing
instruments agree about the intersection of their dimensions and not the union, and a count of instances
that does not say what they share is the shape of a vacuous agreement.

- **Item 2, the unreachable Pareto arm.** `139` varied the weight grid at 1/2000 over one edge of the
  simplex, on one arm set, in `f64`. I varied the decision procedure, not the arm set: exact rational
  vertex enumeration over the whole simplex, on the same three arms. **We share the arm set and differ in
  the procedure**, so what is corroborated is the verdict about those arms and not its generality. The
  generality is `p2`'s separate sweep over 1080 random arm sets, which shares nothing with `139`.
- **Item 4, the 44.3%.** Same input by construction, different implementation and different arithmetic.
  **We share everything except the code**, which is why section 2 says matching was mandatory here rather
  than suspicious.
- **Item 8, the shared baseline.** `140` swept scalar costs in {1..6} against baselines in {1,2,3,4,6,8,12}
  at three arms. I swept the identical space and reproduced its counts, then moved to a space it does not
  reach: vector costs at three coordinates with a weighting. **We share the scalar region entirely and
  intersect nowhere else**, which is exactly why the vector result is an addition rather than a
  contradiction.
- **Item 9, the 42.14%.** `139` at W=6 over all F, both signednesses, both overflow positions;
  `141` on an independent model with two truncation spellings; `142` over six rounding modes; mine at W=6,
  F in {0,3}, signed, saturating, truncate toward zero. **The intersection of all four is signed
  saturating truncate-toward-zero at W=6, F=0**, and that single cell is where the four-instance claim is
  actually four instances. At F=3 it is three, and outside signed saturating it is fewer still.
- **Item 6, the livelock.** Four reproductions on the same host and the same crate, varying only the build
  profile and the load. **The intersection is nearly the union**, so the four instances are close to one
  instance repeated, and I count them as evidence that the flag works rather than as evidence about the
  mechanism, which remains `139`'s alone.

**Zero positions carried from any panel file other than `139`, `140`, `141`, `142`, `143` and `OPTIONS.md`
Q51**, which with `INTENTS.md` and `RULES.md` is my whole reading.

---

## 12. Options, each with what would close it

**O-144-A. The selector is augmented weighted Chebyshev rather than linear.**
It reaches every non-dominated arm, costs the same at compile time and in portability, and removes a limit
worth a median 17.6% where it bites. Against it: it needs a reference point, which is a second object the
design has to say where it comes from, and `p9` shows declaring it does not help portability.
*Closes on*: whether any arm set arvo ships ever contains an established unsupported efficient arm. `p10c`
says none does today across 35 families. So the honest default is linear, and this option reopens the
moment one appears.

**O-144-B. The canon states that a weighting travels and an arm does not.**
Two measured sentences, and the second is what makes a predicate naming an arm target-bound.
*Closes on*: whether any predicate in the corpus names an arm rather than a weighting. That is a grep over
the panel's findings and it is cheap.

**O-144-C. The shared baseline is applied after the weighting, on the weighted scalar.**
Section 6. The alternative, per coordinate before the weighting, is a different and also useful operation:
it is what makes a weighting travel across a rescaled target.
*Closes on*: whether the design wants one baseline for reporting or one for normalising. They are different
objects and the measurement says they cannot be the same one.

**O-144-D. A cost coordinate must be one the arms differ on, not merely one they all carry a value on.**
*Closes on*: whether any candidate coordinate in the design is constant across the arm set. If one is, it
contributes nothing and hands its region of weight space to the tie-break rule.

**O-144-E. The number of named weightings is bounded by the coordinate count rather than by taste.**
At three cost coordinates a linear selector distinguishes four to six arms whatever the arm count.
*Closes on*: how many cost coordinates the design ships. Until that is fixed the bound cannot be stated,
and once it is fixed the bound is a table lookup.

---

## 13. Coverage, bounds, and my predictions that fell

**My predictions that fell, which is the part worth reading.** Seven, of which four changed a conclusion
rather than a detail.

| # | prediction | verdict |
|---|---|---|
| A3 | the 6-versus-5 gap is the duplicate | confirmed, and it makes `139`'s reading of its own table wrong |
| A5 | the witness's optimum is strictly positive in three coordinates | **refuted**, it is zero, and the reason is a dead coordinate, which became F144-4 |
| C6 | every grid winner is strictly selectable | **refuted by my own control**, and the repair is that the right predicate is weak selectability |
| E1 | at least 15% of Pareto arms unreachable at d=3, n=8 | **refuted**, 11.7% |
| E2 | the rate rises with the coordinate count | **refuted at one cell of three**, the four-arm column |
| H3 | a Chebyshev win region splits on the weight edge | **not witnessed** in 397 sets, unestablished, not used |
| J2 | the linear image saturates in the arm count | **refuted**, above three coordinates the arm count moves it more |
| X1 | Chebyshev travels worse than linear | confirmed only at equality, 86.9% against 86.9% |
| X2 | declaring the reference point improves portability | **refuted**, 87.1% against 86.9%, route closed |
| CC3 | the real witness survives dropping any single coordinate | **refuted**, it depends entirely on the largest size point |
| EE3 | two thirds of reordering families survive a pairwise gate | **refuted**, 6 of 24 |
| AA1 | a real unreachable Pareto arm exists in the corpus | **confirmed and then withdrawn by me**, see below |

**The withdrawal, because it is the most important thing in this file about my own work.** `p10` surveyed
35 committed bench families, excluded 21 on two controls, and found exactly one arm that is non-dominated
and linearly unreachable. `p10b` verified it by a second decision procedure, confirmed both procedures agree
on every arm of that table, and confirmed a deliberately dominated arm is rejected. Then its last control
found that dropping the largest size point makes the arm dominated, so the whole verdict rests on one
coordinate. At that coordinate the deciding gap is **3.8 nanoseconds against a 79.2 nanosecond
interquartile range** on the arm it must beat, a ratio of 0.05.

**`p10`'s headline is wrong and its output stays committed with the wrong headline in it**, because the
sequence is the finding and deleting it would leave a clean file where an instructive failure happened.

**The mechanism is worth more than the incident.** `p10`'s noise control compared arm-to-arm spread against
within-arm spread **per family**, and `warm-clamp-arity-w32` passed it at 9.2. That gate is real and it is
at the wrong granularity: a family can have arms separated by microseconds while the single pairwise
comparison that decides a verdict is separated by nanoseconds. **Dominance and selectability are pairwise
questions, so the gate has to be pairwise.** `p10c` builds the pairwise gate, controls it in both
directions, and finds zero established instances in the whole corpus.

**What I did not do.**

- **I did not attack the assignment side.** Sections on the class count touch it only where `143`'s F4
  reaches into my surface. `141` and `142`'s accumulator results, the rounding partition and the fusion
  arms are all carried as theirs.
- **I did not price anything on the bench harness.** No claim here is a bench result and none is called
  one. `p10` and `p10c` **read** committed harness output and do arithmetic on it; they take no
  measurement. Whether the Chebyshev selector's extra const evaluation costs measurable compile time is
  **unpriced**, and `mock/benches/` is where that would be answered.
- **My synthetic cost tables are synthetic**, and everything resting on their geometry establishes a shape
  rather than a magnitude. The magnitudes that are real are the ones from committed CSVs in `p10` through
  `p10c`, and the arithmetic ones in `p5` and `p7`.
- **Everything is `threads = 1`** except where a predicate says the harness ran it otherwise, so under the
  panel's notation these findings do not hold anywhere threads exist.
- **Every fixed-point measurement is at model widths**, `W` in {4, 6}. No transfer argument to 64 bits and
  I am not offering one.
- **Container width equals declared width** in every fixed-point instrument, the same narrowing `139`
  reported against itself and `141` and `142` inherited.
- **I did not read `40`, `93`, `102`, `106`, `107` or `108`.** Where `143` and `141` cite Q51's account of
  `108` section 7, I am relying on that account. Section 8's argued half moves if Q51's compression of it
  is wrong; nothing else in this file does.

**My citations were checked by opening them.** `p11` parses every `file:line` span out of this file
rather than checking a list I remembered, requires each to carry a declared substring, and reports a
citation with no declaration as a failure. Twenty distinct spans, zero failures after two repairs, with
three controls that fire: a phrase absent from the cited file, a phrase present in the file but outside the
cited span, and an undeclared citation.

**Both repairs were mine and one of them was real.** The first was the checker's fault: a naive substring
match reported a mismatch on a quotation that is wrapped across two source lines, so whitespace is now
normalised. The second was the file's: I cited `139:403-406` for a sentence that sits on line 407, so the
span stopped one line short of the thing it was cited for. A resolution check passes that silently. It is
corrected to `139:400-407` above.

**Where I would want a second pair of eyes first.** Section 6's repair to the shared-baseline obligation.
It is the only place where I am changing a stated obligation rather than measuring around one, it rests on
one instrument, and the specific thing to check is whether the two normalisation orders are the two a
design would actually consider or whether I have built a fork that does not arise. If a second reader
finds a third order, or finds that the reporting baseline and the normalising baseline are the same object
in some formulation I have not seen, section 6's repair moves.

**And the thing I most want handed on rather than defended.** `p10c`'s lesson about gate granularity. A
noise check at the wrong granularity passes while the comparison it was supposed to protect fails by a
factor of twenty, and nothing about the passing number looks wrong. This panel checks controls carefully
and I have not seen anyone check what a control is *quantified over*. I am one expert on that and it is a
claim about method rather than about arvo.

---

## Appendix: the probes

Fifteen entries over seventeen source files, counted by
`ls 144_probes/*.py 144_probes/*.rs 144_probes/*.sh | wc -l`, each committed with its output before or
alongside this file, and each proving one thing once.

1. `p1_reproduce_and_decompose.py`: `139`'s weight-cell numbers reproduced exactly on an independent
   instrument, the six-against-five gap decomposed to its duplicate arm, and two of my own controls firing.
2. `p1b_strict_versus_weak_and_the_dead_coordinate.py`: the repairs, the witness decided exactly at +1 in
   its true dimension, and the dead-coordinate finding.
3. `p2_is_the_linear_limit_typical_and_what_lifts_it.py`: the unreachable-arm rate over random tables, and
   the Chebyshev certificate verified in both directions on 3667 arms.
4. `p3_how_many_weightings_can_ever_be_told_apart.py`: the selector's image against the arm count, and the
   collinear case where linear names two of five points.
5. `p3b_the_vocabulary_is_governed_by_the_coordinate_count.py`: the image against both arguments, with the
   separation LP checked against full enumeration across 770 arms including 63 exact zeros.
6. `exact_lp.py`: the shared decision procedures, with two dead solvers recorded rather than deleted and a
   self-test that fails unless the boundary is exercised.
7. `p4_what_actually_travels_across_targets.py`: 44.3% located at the 2nd percentile, the arm-versus-
   weighting regret, and the rescale conditional.
8. `p5_the_shared_baseline_at_the_dimension_it_actually_has.py`: `140`'s scalar result reproduced and the
   vector case measured, with the mutual control and the uniform-baseline case that must fail.
9. `p6_the_firewall_is_what_makes_selection_portable.py`: the composition, on real fixed-point arms, with
   the conforming and non-conforming pairs and both controls.
10. `p7_the_second_argument_of_the_count.py`: the visibility biconditional over 96 cells, with a dead axis
    as the control and both directions exercised.
11. `p8_a_nonlinear_selector_is_const_and_lowers_to_one_branch.rs` and `p8_scan.sh`: the doability check,
    with the emitted assembly committed and a runtime-selected control that keeps its arms.
12. `p9_what_the_replacement_costs.py`: the replacement attacked, one prediction of mine closed as a dead
    route, and the limit priced at a median 17.6%.
13. `p10_the_linear_limit_on_real_arms.py`: the committed corpus surveyed, with the headline that turned
    out to be wrong, kept.
14. `p10b_verifying_the_one_real_witness.py` and
    `p10c_the_witness_was_noise_and_the_gate_was_at_the_wrong_granularity.py`: the verification that found
    the defect, and the pairwise gate that should have been there from the start.
15. `p11_check_my_own_citations.py`: every citation in this file opened and its content tested, with the
    parse driving the check so an undeclared citation is itself a failure.

Three of them exist only because an earlier one refuted me, and the last two exist only because the tenth
was wrong.
