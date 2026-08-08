# Op: the strategies weigh measurements differently, and none of it is clear cut

**Date:** 2026-08-08. **Position:** immediately after `37`, correcting it.
**Required reading, and `37` may not be read without it.**

## His words, verbatim

> I should add that Warm does not merely imitate, its intent is to be intuitive best choice for most
> every use case, and the intuitive part demands it mimics, but it does not make it absolutely
> required, if mimicking is consistently just worse choice.
>
> Cold does not *have to* drop efficiency wins elsewhere. It can use the same paths Hot uses, not
> because it needs to by intent, but nothing in its intent would fight it. But if the path fights the
> intent, then it's not for Cold. And as such, it's not clear cut. You should not write these as clear
> cut and settled. The *intent* is clear I think, but nothing about them is absolute otherwise. All of
> them should be decided by measurement, just measuring different things, and, this is I think the
> mental unlock: They weigh different measurements differently. For the most part, they probably agree,
> because in general, the best answer fits all, because it fights none of their intent. But perhaps my
> instinct is wrong there, and all truly differ for the most part. Because that is possible, we don't
> ratify these as absolutes, rather, *intent* as stated by me here and prior

## The correction, stated against what `37` actually said

`37` sorted the carrier question into four answers of different **kinds**: `Hot` decided by
measurement, `Cold` decided by intent and "possibly not by measurement at all", `Precise` by neither,
`Warm` by lookup. It went further and wrote that for `Cold` "a measurement could not overturn this".

**That is the clear-cut framing op is rejecting, and it is wrong on its own terms.**

**All four are decided by measurement.** They differ in *what they measure* and, the load-bearing part,
**in how they weigh what they measure**. A strategy is not a rule that short-circuits evidence; it is a
weighting over evidence.

So the correction to `37`'s list is not a detail. The list had `Cold` deciding without measurement,
which would make its answer unfalsifiable, and unfalsifiable is the opposite of what this design is
for.

## What each clarification changes

**`Warm` is not defined by imitation.** Its intent is to be the **intuitive best choice for most every
use case**. Imitation is downstream of that: intuitiveness demands mimicry, because what a Rust
programmer finds intuitive is what plain Rust does. But **mimicry is not absolutely required, and
loses if it is consistently just the worse choice.**

This sharpens the ratified prior claim rather than contradicting it. `seed/SETTLED_strategy.md`
section 3 records `Warm` as "defined by imitation of a native Rust primitive, as a standing intent
that outranks its mechanism". The objective underneath the imitation is what is new here, and it means
`Warm` has an escape from imitation that the earlier statement did not describe.

**`Cold` is not obliged to be slow.** It may use the same paths `Hot` uses, not because its intent
demands them but because **nothing in its intent fights them**. The rule is stated as a test rather
than a preference: **if the path fights the intent, it is not for `Cold`.** So the leeway recorded in
`36` is a permission that goes unused whenever a fast path costs nothing in residency, and `37`'s
reading of `Cold` as licensed-to-be-slow overstated it into a disposition.

## The mental unlock, and why it is the useful part

> They weigh different measurements differently.

That is a compact model of the whole axis and it explains the shape of every open question here. The
strategies are not four different rules. They are four different **weightings over the same evidence**.

Two consequences follow, and op states both:

**They probably agree most of the time**, "because in general, the best answer fits all, because it
fights none of their intent". A single arrangement that is fast, small and accurate is not in tension
with any weighting, and there is no reason to expect disagreement where no trade exists.

**And that instinct may be wrong**, in which case they genuinely differ for the most part. He names
this possibility explicitly rather than assuming the convenient case.

**This is a testable claim about the design, and nobody has tested it.** How often do the four
weightings pick the same arrangement? The panel has measurements from `20`, `22`, `26` and `27` that
were taken for one weighting, and the question of whether the answers coincide has never been posed.
It is a real piece of work and it is now well defined.

## What follows for the evidence already in hand

**The footprint gap is real after all.** `37` concluded that `Cold`'s unpriced footprint might not be a
defect, because `Cold` would decide by intent. Under the corrected model `Cold` decides by
measurement, weighting residency heavily, so **the footprint measurement that `26` and `27` both name
as never taken is genuinely owed**. That is a straight reversal of `37`'s last section and it is the
practical consequence of the correction.

**The throughput measurements keep their value and gain a boundary.** They remain the strongest
evidence in the panel. They are heavily weighted by `Hot` and lightly by `Cold`, rather than being
about `Hot` alone. A `Cold` decision may still consult them, and will not be governed by them.

## Standing, and it is now precise

Op draws the line explicitly:

> Because that is possible, we don't ratify these as absolutes, rather, *intent* as stated by me here
> and prior

**The intents are what may be ratified.** The per-strategy conclusions, the four-way sorting of the
carrier question, and any claim that a strategy decides without evidence are **not** ratifiable and
were never his.

Two of the four intent statements sit on the prior panel's record as **RATIFIED** and are quoted with
provenance in `37`. Everything added across `34`, `36`, `37` and this file is direction of high
confidence, marked canon-bound by him, and **not settled**, because under his own correction an
opinion given before the experts converge is an ack rather than a ruling.

And the instruction that governs how any of this is written down: **do not write them as clear cut and
settled.** The intent is clear. Nothing else about them is absolute.
