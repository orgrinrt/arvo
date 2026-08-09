# Op on the standard, and how this review runs from here

**Date:** 2026-07-31
**Position in the panel:** written immediately after `13b`. **Required reading** with the numbered
files and every earlier checkpoint.

## The standard everything is measured against

Op was offered three options on where the algebraic laws should sit and declined to pick, on the
grounds that it was never a question for op:

> This isn't my call. Already the instruction is clear: Optimal, ideal, representative of the math,
> and also, the principle that arvo has to be able to represent mathlab, ieee standard 754, systemc
> etc etc, which means the abstractions are what truly matter, the typestate

So the standard is fixed and it is not a preference to be consulted per question. Every design
question in this review is answered by finding the answer that is:

1. **Optimal and ideal**, not adequate, not conventional, not the smallest change from what ships.
2. **Representative of the mathematics.** The structure the design names should be the structure the
   mathematics has, not one adjacent to it that happens to be easier to encode.
3. **Capable of representing the established systems.** MATLAB, IEEE 754, SystemC and the rest are not
   inspirations to borrow from; they are a *test*. An abstraction that cannot express one of them is
   an abstraction that is not general enough, and that is a defect rather than a scope boundary.

And the consequence op draws from the three together: **the abstractions are what truly matter, the
typestate.** Not the packaging, not the crate graph, not which preset carries which value. Those
follow. A member facing a choice between a cleaner abstraction and a cheaper arrangement resolves it
toward the abstraction and reports the cost rather than trading the abstraction away.

A member should therefore not ask what op prefers on a question of this kind. It has been answered in
advance. Find the optimal answer against the standard above and show the work.

## How the review runs from here

Op's instruction, verbatim:

> Don't poll this. I will literally say when we are done. This current one should be another deep dive
> like the prior ten, into this specifically. It may take another ten. Then after that, we again
> consolidate and start a new fresh eyes based on that, do another 10 or so experts focusing on
> another area, and we do this until our very design is both concrete, valid and critically, ideal,
> optimal, the dream achieved, nothing less will we stop for.

So the shape is a repeating cycle, and this review is one turn of it:

- **A deep dive.** Roughly ten members, sequential and cumulative, going all the way into one area.
- **A consolidation.** The area's result compacted into a standalone statement of the shape.
- **A fresh read.** A member who is given only the consolidation, with the transcripts withheld, so
  the next area is chosen by someone not carrying the last one's assumptions.
- **The next deep dive**, on whatever that read exposes.

Repeating until the design is concrete, valid, and ideal. Nothing less is a stopping condition, and no
member should treat running long as a reason to converge early.

**The current deep dive is the algebraic laws**: what they are, which structures the mathematics
actually calls for here, and where they live inside arvo. Not whether to have them, and not whether
they move to another repository; `13b` settled that they stay in arvo, in a place still to be
designed.

## What the first file of this dive inherits

`13_mcsherry_where_the_laws_belong.md` established four things that any answer has to survive, each
verified by running code rather than reading it:

- `arvo-graph`'s recurrences are max-plus, not folds. Addition is applied once per node with the
  grouping pinned by the graph, so associativity is not what makes their answers correct.
- Exhaustively over 64 graphs and 625 weight vectors: wrapping is associative but **fails**
  distributivity over maximum, while saturating is non-associative and **satisfies** it. A gate on
  associativity admits the preset that breaks these crates and refuses the two that work.
- `arvo-spectral/src/power.rs:71` is arvo's one genuine fold over addition, over a float type, which
  an associativity gate refuses at every strategy.
- Grouping rather than order is the axis: contiguous chunking preserves element order exactly and
  still changes the grouping, so a documented order does not substitute for a law.

Together those say the ladder the spec proposes is not the algebra this substrate needs, and that
semirings and distributivity are at least as load-bearing as the group-and-ring line the spec drew.
Whether the right answer is a different ladder, a different shape entirely, or laws that describe
rather than gate, is what this dive is for.

## Not our concern

Op, on the hilavitkutin workspace breakage and the `ConvergenceBuffer` defect recorded in `13b`:

> It's not our concern. We work arvo. The mockspace has been getting updated daily the past days, and
> had a big rework lately, so most of the repos haven't yet migrated. Not our concern and it has been
> working just fine until yesterday. That means nothing to us.

So the manifest failure is migration lag from an in-flight tool rework, not decay, and neither finding
is this review's to act on. They stay recorded in `13b` for whoever works hilavitkutin next. No member
should spend a dispatch on them, and no member should treat the dead suite as evidence about the
engine's quality.
