# Op: what each strategy is for, and Warm is not among them

**Date:** 2026-08-08. **Position:** after `34`, extending it from `Hot` alone to the axis.
**Required reading.** Op says this "can go into the canon / settled list", and in the same breath says
it does not have to be and is open for discussion and exploration. Both halves are recorded.

## His words, verbatim

> I should still clarify one thing more: Strategies are all different, and the intent behind Hot is
> performance, efficiency, even at the cost of accuracy or soundness. Cold is optimised for cold paths
> and cold storage, which means, it aggressively minimises and bitpacks, *but* because it optimises
> for cold paths, it has more leeway to do things non-efficient. Precise on other hand is the one that
> sacrifices as much performance and efficiency as makes sense, to be the most precise possible answer,
> throwing out all cold or hot axis optimisations to be *accurate* and *precise*, especially within
> chains and ops, not only alone. I think this is meaningful. In fact, the strategy and the above
> descriptions I gave on the ones we have kind of settled in general (though don't have to be; Open for
> discussion and exploration!), all this can go into the canon / settled list.

## What this fills

`23` found that eight sentences mention the strategy axis, three name it inside their canon sentence,
and **none defines it**. `25` checked that and found it held more widely: fifteen mentions, zero
definitions, across two panels. `25` then wrote a definition of what a strategy **is** structurally, a
named section over a product of axes, never derived, the one input the substrate cannot compute.

**What `25` could not supply is what each section is for.** That is what this is. The structural
definition and the per-strategy intent are different things and both are needed: one says what kind of
object a strategy is, the other says what each of the named ones means.

## The three intents, kept apart

**`Hot`: performance and efficiency, at the cost of accuracy or soundness.** This is `34` restated and
widened. Trading soundness away is what `Hot` is for, bounded by `34`'s condition that it not be lost
for nothing but against a provable meaningful gain.

**`Cold`: cold paths and cold storage.** Two clauses that pull in opposite directions and both are
his. It **aggressively minimises and bitpacks**, which is the storage half. And **because** the path
is cold, it **has more leeway to do things non-efficient**, which is the time half. So `Cold` is not
"slow but small" as a tolerated tradeoff; the slowness is licensed by the coldness of the path rather
than merely accepted.

**`Precise`: accuracy, at as much cost in performance and efficiency as makes sense.** It throws out
**both** the hot and the cold axis optimisations, which is stronger than being the opposite end of
`Hot`. And the clause that carries the most weight: **"especially within chains and ops, not only
alone"**. `Precise` is a claim about composed computation rather than about a single operation, so
whatever it means has to hold across a chain and not merely per step.

## Warm is absent, and I am not filling it in

The named strategies are four. He described three. **`Warm` has no intent recorded here**, and
inventing one would be exactly the failure this panel exists to avoid, so it stays a gap.

This matters more than a missing entry usually would, because `Warm` is the default, and because the
panel already holds an open question about what `Warm` does on overflow (`OPTIONS.md`, Q6) where two
committed bench families implement opposite readings and disagree in direction. A `Warm` intent would
bear on that question directly. Its absence is now a named hole rather than an oversight.

## Connections to live questions, offered as connections rather than answers

**`Cold`'s leeway may make throughput the wrong axis for it.** `26` and `27` measured packing against
dense carriers by **throughput**, and concluded the trade depends on the carrier displaced and on the
core count. If `Cold` exists for cold paths, where by construction the leeway to be non-efficient is
granted, then a throughput comparison is measuring the axis `Cold` is least concerned with. Both files
noted independently that they never priced the **footprint** benefit at all, which is the axis `Cold`
is named for. That is a gap in the evidence rather than in the intent, and it is now pointed at.

**`Precise` within chains bears on what a datum denotes.** `18` established that an absorbing endpoint
is sound exactly while the computation stays at it, with 936 of 5184 chains unsound once subtraction
enters, and named `Precise` on `inexact` as the same question one level down. "Precise within chains
and ops, not only alone" is a statement about exactly that domain.

**The intents are per strategy, so a single answer to a strategy-spanning question is suspect.** `34`
already made this point about soundness. Extended here: if each strategy has its own purpose, a
question of the form "what does arvo do" may have four answers, and one answer should be checked for
having collapsed the axis.

**It bears on whether the arithmetic column is one axis or two** (Q5). Three intents that differ in
what they trade suggest the trade itself is an axis, which is a different claim from the four names
being values of one arithmetic policy.

## Standing, and the two halves of it

He says this "can go into the canon / settled list" and that the descriptions are "kind of settled in
general". He also says, in the same sentence, "**though don't have to be; Open for discussion and
exploration!**"

So it is **direction marked as canon-bound and explicitly left open**. Per his own correction, an
opinion given before the experts converge is an ack meaning the direction checks out, and ratification
is the last step, reached only when a converged thing is brought to him. **Nothing here is ratified,
nothing may be cited as settled, and the invitation to discuss and explore is part of the instruction
rather than politeness.**

The register is not edited from this file, because a member is reading it. The edit follows when that
dispatch lands.
