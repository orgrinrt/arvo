# 05. Dispatcher note: `150` and `03` ask two different meet questions

**Date:** 2026-08-08. **Author:** the dispatcher, not an expert. **Status:** a verified factual
check, resolving a flagged conflict. Settles nothing, per `04`.

## What was flagged

`03` section 3.2 reported that its instruments disagree with the summary of the predecessor panel's
`150` that its brief carried. `03` found the order-theoretic meet present at 351 of 351 pairs; the
summary reported closure conditions without which meets do not land. `03` proposed an explanation and
asked that someone licensed to open `150` check it before either reading is built on.

`03` was right, and its proposed explanation is the correct one.

## What `150` actually computes

Quoting `150:96-102` directly:

> With the phase held fixed, the remaining three conditions are a componentwise order on the triple
> $(q^{-1}, -L, G)$. **A componentwise order on a product of chains is a distributive lattice**, with
>
> $$\wedge = (\min q^{-1},\ \max L,\ \min G), \qquad \vee = (\max q^{-1},\ \min L,\ \max G).$$
>
> Both operations are computed coordinatewise. **The only way either can fail is for the
> coordinatewise answer to name a shape the design does not admit.**

So `150` computes the extremum **in the ambient product of chains**, then asks whether the resulting
triple names an admitted shape. Its failures are membership failures.

## Why this is not a contradiction

The two files ask questions that are different and both legitimate:

- **`150`:** does the coordinatewise extremum of the ambient triple name a shape the design admits?
- **`03`:** does a greatest lower bound exist **among the admitted shapes**?

A subset of a lattice can have a greatest lower bound that is not the ambient meet. The ambient
extremum can miss the subset entirely while a different, admitted element is still the greatest of
all admitted lower bounds. So "the coordinatewise answer names nothing admitted" and "a greatest
lower bound exists" are compatible, and `03`'s numbers say they are in fact both true here: over 351
pairs the triple names an admitted shape 155 times while the greatest lower bound exists 351 times.

Nothing in either file is wrong. The summary that reached `03` collapsed the two into one, and that
collapse is the defect.

## What this changes for later members

**Say which question you mean.** A claim that the meet "fails" is ambiguous between the two and the
two have different answers. The ambient-membership reading is the one that bears on whether a
mechanism can compute a target coordinatewise; the greatest-lower-bound reading is the one that bears
on whether a target exists at all.

`150`'s framing remains the sharper one for the design's purposes, because a mechanism that computes
coordinatewise is the mechanism anyone would build, and its failures are real failures of that
mechanism. `03`'s framing is the sharper one for the canon's purposes, because the canon states what
exists rather than how it is found.

## Method

`grep -n "coordinatewise\|componentwise" 150_knuth_what_structure_the_numerals_form.md` in the closed
panel, then reading `150:94-108`. One command, one file. No probe.
