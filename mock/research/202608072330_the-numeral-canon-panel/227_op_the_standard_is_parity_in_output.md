# 227. Op: the standard is parity in output, not in the internals

Unprompted, while seat 226 was deriving `which_operation_set_the_design_ships` and seat 225 had just
returned an answer to it. His words, in full:

> just to note: the standard should be for parity in output, not so much the internals. as long as it
> matches the logic of the spec, but perhaps achieves it slightly different or unconventional way,
> that is fine.

## What it settles

**The standards bound is an obligation about results, not about inventory or mechanism.** A candidate
meets it when what it returns agrees with what the standard documents, for the cases the standard
covers. It does not have to reach that agreement the way the standard's own text describes, and it
does not have to carry the standard's operation list as its own.

Three things follow, and the third is the one that lands on live work:

- **An unconventional route to the same result is fine.** The spec's logic is the thing to match; its
  procedure is one way of expressing that logic and not the only admissible one.
- **The internals are ours.** Container choice, intermediate width, the order operations are composed
  in, and whether a given result is computed directly or assembled from others are all design
  decisions the bound does not reach.
- **Parity does not import an operation set.** That a standard names an operation says the results of
  that operation must be reproducible, not that arvo declares an operation of that name.

## What it corrects, which is a live derivation of tonight

Seat 225 answered `which_operation_set_the_design_ships` by deriving the floor as **the union of the
two named standards' operation sets**, and reasoned from there that because IEEE 754 requires the
fused multiply-add, a design shipping four of the five operations is unreachable for any
standards-adequate core.

**The first half of that does not survive this.** The bound obliges reproducing the fused
multiply-add's *result*, which is the exactly-rounded `a*b + c` with one rounding rather than two, and
that is a statement about output. Whether arvo declares an `fma` operation, computes it from a wider
intermediate, or reaches it some other way, is internals.

**The second half may still hold and now needs its own argument.** The question is no longer whether
the operation is in the set, but whether the result is reachable at all without it, which is a
different claim needing different evidence. It may well be that no composition of the other four
reproduces single-rounded `a*b + c` in general, in which case the conclusion stands on a firmer
footing than it had. That is for the seats to establish rather than for this file to assert.

**The container premise refutation is untouched**, and worth saying so explicitly. It rested on a
container-stated numeral returning `8192` where MATLAB `fi` at thirteen bits under wrap documents `0`.
That is a disagreement in output, which is exactly what this bound is about, so the strongest evidence
under the floor is unaffected.

## How to read this against what he said before

It does not widen or narrow `ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`, which
fixed *which* standards are in scope. This fixes *what being in scope demands*. The two named
standards remain MATLAB `fi`/`fimath` and IEEE 754, with the other three reserved rather than
excluded.

It also sits comfortably with the ratified format spine: arithmetic on a format is an exact operation
in the ambient domain composed with a named total adaptation onto the representable set. That is a
statement about what the result must be, and it says nothing about the route.

## And a minute later, what actually has to ship

> and we can leave each standard a few holes with a FIXME to fill later, to be honest, that's not the
> important thing to ship, it's a fundamentally sound, expressible and mathematically accurate and
> exhaustive framework

**This ranks the two against each other and standards parity loses.** A standard may be covered with
holes in it, each marked, and the design ships anyway. What may not have holes is the framework:
sound, expressible, mathematically accurate, exhaustive.

So the standards are **a check on the framework rather than a gate on shipping**. They are how you
find out whether the framework can express what real conventions need, which is why the two named ones
were named at all, and a gap they expose is a marked hole rather than a blocker.

**The four adjectives are the actual bar** and each is a demand:

- **Sound.** The framework does not admit a construction whose behaviour contradicts what it says.
- **Expressible.** A consumer can state what they mean in it, and a convention that exists can be
  written down in it.
- **Mathematically accurate.** What it computes is what it claims to compute, in the domain it claims.
- **Exhaustive.** It covers its own space rather than the cases somebody happened to reach for.

**A hole in a standard is a `FIXME` and is honest. A hole in the framework is a defect.** The
workspace already has the discipline for the first: a greppable marker at the site, naming what is
incomplete and what unblocks it, so a completion pass finds it rather than reading prose.

## What this does to the two questions the pair is on

**`which_operation_set_the_design_ships` is not gated on standards adequacy at all.** The previous
reading made the standards' operation lists a floor the design had to reach; this makes them a source
of evidence about whether the framework is exhaustive. An operation may be left as a marked hole. What
may not be left is a framework that cannot express it.

**So the shape of the answer moves**: from "which operations must be present" toward "what must the
framework be able to express, and which of those are shipped now against marked later". The second is
a smaller and better-posed question, and it is answerable without the standards being complete.

## Required reading

For every member after this file, and specifically for the pair on
`which_operation_set_the_design_ships`. A derivation that treats the standards bound as constraining
the operation inventory, or as a gate the design must clear before shipping, is reasoning from a
reading he has now corrected twice in one minute.
