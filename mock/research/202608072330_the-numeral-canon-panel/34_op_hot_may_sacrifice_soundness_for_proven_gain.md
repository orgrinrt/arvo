# Op: the soundness condition is not uniform, and Hot is the exception by design

**Date:** 2026-08-08. **Position:** immediately after `32`, correcting it before it was built on.
**Required reading, and `32` may not be read without it.**

`32` recorded op's intent that arvo adapts to the cores it finds, "wherever it is proven to improve
performance without sacrificing the soundness". Read alone, that makes soundness a uniform side
condition over every strategy. It is not.

## His correction, verbatim

> Just throwing in quickly, before it gets misread: On my previous message, I said most efficient and
> using the cores without sacrificing soundness; That is true in general. However, the strategies we
> have (Hot, Cold, etc) behave differently and have their own purpose, intent, that shapes what the
> answer is. So "without sacrificing soundness" is property of all of them except Hot. Hot *can*
> sacrifice soundness, that is its explicit purpose, but it should not lose it for nothing, instead,
> provable meaningful gains. I think this has to be added in there

## What changes

**The soundness condition is per strategy, not global.** For every strategy except `Hot` it is a hard
side condition: an adaptation that trades soundness away is refused however fast it is. For `Hot` it
is not a side condition at all.

**`Hot` may sacrifice soundness, and that is its explicit purpose rather than a tolerated defect.**
This is the sentence most at risk of being softened by a later reader into "Hot is permitted some
imprecision". It is stronger than that: giving up soundness is what `Hot` is **for**.

**`Hot`'s condition is a price rather than a prohibition.** It "should not lose it for nothing,
instead, provable meaningful gains". Three things sit in that clause and they are separable:

- The gain must be **provable**, which on this workspace's evidence rules means the bench harness with
  real competitor arms, not an argument and not an ad-hoc spike.
- The gain must be **meaningful**, which is a threshold nobody has set and which is not the same word
  as provable. A real but negligible gain does not buy a soundness loss.
- Losing soundness **for nothing** is the failure named, so the defect is an unpriced trade rather
  than the trade itself.

**And the general principle above the instance:** each strategy has "their own purpose, intent, that
shapes what the answer is". So a question of the form "what does arvo do here" may have as many
answers as there are strategies, and a single answer to such a question should be suspected of having
collapsed the axis.

## Where this lands in the panel's open questions

Not settling any of them, and each of these is a connection to test rather than a conclusion.

**It bears directly on what a strategy is.** `25` defined a strategy as a named section over a product
of axes, never derived, the one input the substrate cannot compute. This adds that the sections differ
in **what they are permitted to trade**, which is a property `25`'s definition does not currently carry
and may need to.

**It bears on the arithmetic-column question.** If `Hot` alone may trade soundness, then whatever axis
carries that permission is an axis the other three take a different value on, which is material to
whether the arithmetic column is one axis or several.

**It bears on the wrap-or-clamp question.** The ratified preset table gives wrapping to `Hot` alone,
and wrapping is precisely a soundness trade. That is at minimum consistent with this intent and may be
an instance of it.

**It bears on the packing carrier.** `32`'s adaptation rule now has two forms: for `Hot`, an
adaptation may take a soundness loss against a proven meaningful gain; for the rest, it may not, and
the fastest sound arm wins.

## Standing

`32` is marked by op as a ratifiable intent. This file is part of that same intent rather than a
separate one, and it carries the same standing: **direction of unusually high confidence, marked as
canon-bound, not ratified, and not citable as settled.** Per his own correction, an opinion given
before the experts converge is an ack, and ratification is the last step.

The threshold hidden in "meaningful" is unset and is a real gap. Nobody should invent a number for it.
