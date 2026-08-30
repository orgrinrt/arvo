# 99. Checkpoint eight: the strategy axis, after four

**Position:** after `98`, at the checkpoint slot of the strategy-axis unit's 4-4-1. **Author:** the
dispatching agent, not a persona and not op. **Standing:** nothing here settles anything. The calls
marked as mine are the coordinator's and are attackable.

Written for whoever holds op's seat. It is the fourth-file checkpoint, so the second four have not run
and the consolidation does not exist yet.

## The unit in one paragraph

Four experts on one question: what a strategy is. `93` and `94` derived it cold, blind and in parallel,
reading only the intents, the panel rules and the shipped tree, then each appended a reconciliation.
`97` attacked both. `98` second-read the strongest thing `97` produced and converged. All four passed
both standing gates. Every headline number below was verified at source by me, in the committed probe
output, before it was written here.

## What converged, and it did converge

**A strategy is not one thing, and the disagreement about how many resolved rather than persisting.**
`93` derived two layers, ordered rather than parallel: a policy layer that changes the answer and must
live in the type, and a lowering layer that changes only cost. `97` refuted the circularity `93` had
worked around and found **three**, distinguished by polarity: an observable coordinate is an *input* to
the resolver, an unobservable one its *output*. `98` did not reopen it.

**The definition question is settled enough to state, and it was settled by refusing a merge.** `25`
section 7 says a strategy is a named section over a product of axes. Both cold derivations said it is a
compile-time selection among candidate arms, decided by measurement. Those were proposed as the same
claim, which would have moved `25` to the TWO EXPERTS rung. **They are not the same claim**, and `97`
gave the distinction that separates them: a section is any region-to-mechanism function, an argmin is
one that a single weighting explains at once. Not every section is an argmin, and the shortfall is the
whole content.

That is op's `88` section 1 answer recovered from below, by three experts who had not read it when they
derived: **the section is the design tier and the weighting is the canon tier**, which is "mostly
option 1, a little bit of option 3" with a mechanism attached rather than a preference stated.

**And rationalisability is what makes "a little bit of option 3" checkable.** `97` proposed it and built
the decider; `98` reimplemented it independently on the same committed data and **reproduced both counts
exactly**, which is the strongest corroboration this panel can produce short of op.

## The one number that changed, and it matters for the canon

`97` reported 72 of 15625 sections rationalisable, 9 strictly. `98` measured both against `97`'s own
separate finding that some arms are dominated in every region and therefore selectable by no weighting.

- At a **non-negative** weighting: 72 rationalisable, of which **63 select an arm no weighting can
  select**. A zero weight admits an arm the weighting is indifferent about.
- At a **strictly positive** weighting: 9, of which **0** do.

Both of `97`'s sentences are true and they are about different rungs. The consequence is a constraint on
the canon rather than a correction of a mistake: **a canon that wants the no-dominated-arm guarantee has
to require strict positivity, and cannot get it from non-negativity plus a hope about the table.**

Verified at `98_probes/p6_reproduce_the_predecessors_count_and_rung_it.out` and
`98_probes/p9_the_proposal_instantiated.out`.

## One shape change, offered and not adopted

`98` bootstrapped the committed bench samples and found **a section is not stable across a rerun of the
same bench on the same afternoon**: a fixed weighting produces 30, 8 and 77 distinct sections on three
families, and on one of them the committed section is not the modal one. So a section is a poor object
to apply a check to.

Its proposal: **generate the table from the weighting rather than check a table against one**, which
makes rationalisability true by construction rather than a property to test. It notes this is `93`'s own
unregistered fork (`93:966-973`, bake the winner against bake the cost table), with a discriminator
attached, and that `93`'s P4 had already compiled both sides.

This is the strongest candidate the unit produced for what the canon should actually say. It has one
expert behind it plus a predecessor's unregistered fork, which is not two independent instances. **The
second four should attack it first.**

## Both experts corrected themselves against their own evidence, and said so

Worth recording because it is the discipline working rather than a defect. `93` had two hypotheses
refuted by its own probes and reported the refutations as the useful part. `98` added a third cost
coordinate, reported it rescued an arm from being dominated everywhere, then tested the rescue against
zero and found **all four comparisons it depended on have bootstrap confidence intervals crossing
zero**; it withdrew the rescue and let `97`'s finding stand unqualified. `97` kept two of its own errors
in its file rather than deleting them.

## For op, and it is two items, both about I3

Neither is a category-wide policy fork. Both are questions about op's own words that no amount of
measurement answers.

**One. Which reading of I3 is meant at a width Rust has no primitive for.** I3 is the standing call that
a strategy "should behave like native primitives in regular old rust would". At a declared width like 13
or 47 there is no native primitive, so the sentence has two readings: the **declared width** or the
**container** it is stored in. `93`'s F8 measures that the two readings **disagree at all fourteen
non-native widths it swept**, so this is not a distinction without a difference. `93` offers a reading
and says itself that it should be asked rather than assumed; `98` marks its own agreement with that
reading as inherited rather than independent, which is the right call and is why neither counts as an
answer.

**Two. I3 against I15.** I15 is "never any runtime checks, ever". Rust's native primitives signal
overflow with a debug-mode panic, which is a runtime check. So the imitation I3 asks for is, on that
one behaviour, permanently unavailable under I15. `93` handed this back rather than resolving it. The
question is not which rule wins in general; it is whether I3's imitation is meant to cover the panic at
all, or whether I3 was always about representable-value behaviour.

## A report for op rather than a question

**Two of op's four stated strategy intents have no coordinate in the bench corpus at all.** `98` found
that all thirteen bench crates force their arms to agree, which is correct for what those benches
measure, and the consequence is that no family carries a column for accuracy or for divergence from a
reference. So the intents about accuracy and precision cannot currently be measured by this repository's
own instrument, whatever else the corpus establishes.

Related and already actioned, so it needs nothing from op: the harness never ran the per-variant
validators that fourteen of those crates define. Fixed upstream, arvo's pin not yet bumped. Full record
in `96`.

## The coordinator's calls, all attackable

**The topic.** Op declined to pick one and gave the criterion at `87`: strictly bottom-up, and among
what is available at the current tier take what settles the most downstream at once. The strategy axis
was my call under it, on the ground that three topics had independently stopped at the same placeholder
and topic five cannot compose while one of its inputs is a placeholder.

**Refusing the rung merge.** `94` reported that two blind derivations had moved `25` section 7 to TWO
EXPERTS. I did not register it, on the ground that the cold pair's proposition looked adjacent to
`25`'s rather than identical. `97` then established that they are different claims. Recorded because
the refusal was a judgement call at the time and would have been wrong to make silently.

**Banner rather than delete.** Both cold derivations independently found `mock/DESIGN.md.tmpl` and
`mock/PRINCIPLES.md.tmpl` surviving unbannered while asserting the demoted four-marker set as settled
and naming a forbidden feature. I bannered both as superseded rather than deleting, because
`docs/DESIGN.md` generates from the first. Every line in both moved down by 8 and the 19 citations into
them from `93` and `94` are low by that much; recorded in `96`.

**A workspace rule corrected.** `arvo-always-optimal-internals.md` told every agent that distributivity
holds exactly at `F == 0`. Two members refuted the "exactly" independently from different models, and
`97` found that inside the region the rule called safe, at unsigned `F = 0` saturating, distributivity
over subtraction fails at 45.79% of triples. The rule now says `F == 0` is necessary and not sufficient
and that a law permission names its operations. That was a live licence to emit a wrong rewrite.

## What the second four should do

Attack `98`'s generate-from-the-weighting proposal first, because it is the unit's strongest candidate
and it has one expert behind it. Then the strict-positivity constraint, which is currently one
measurement. Then whether the three-layer split survives being written down as a design rather than
argued as a distinction.

Per `95` the second four are convergence dispatches: the unit ends in a stated shape the consolidation
can carry, or in a located disagreement stated precisely. Not in a fifth refutation.

## Counts that disagree, for whoever reconciles them

Three members gated the bench-variant suites and reported different totals: `94` says 108 tests across
twelve crates, `97` says 96 across eleven, `98` says 123 across thirteen and explains the others,
noting one `#[test]` sits inside a doc comment so a grep returns 124. `98`'s is the only one that
accounts for the discrepancy, and I have not independently recounted.
