# Op: the spirit outranks all of it, and keep the shape where you can

**Date:** 2026-07-31
**Position in the panel:** written immediately after `16c`, completing the posture correction.
**Required reading** for every member from here, together with `16b`, `16c` and `13c`.

`16b` and `16c` told members to stop auditing the code and to design the downstream contract. Taken
alone they over-correct, and this file is the counterweight the lead designer added.

## Keep the current shape where keeping it costs nothing

Op, verbatim:

> If we can keep the current shape, we should. The best design is one that sacrifices none of the real
> design improvements and work, with least amount of work to write on the current codebase.

So "assume everything is rewritten" is a licence to stop defending the existing code. It is not an
instruction to discard it. Rewrite cost is a real cost and it is the tiebreaker between designs that
are otherwise equal against the intent. A proposal that throws away a working shape for a marginally
better one, or for symmetry, is worse than one that reaches the same place by a smaller move.

Both failures are available and this review has now been warned about each. Auditing what is already
condemned, which `16b` addressed. And discarding shape that still serves, which this addresses.

## The spirit outranks everything, including everything in these checkpoints

> And also, none of this should *override* the real intent of the design. Which the existing code also
> attempted to solve. It wasn't good enough at that, but the intent, and the reshaped, reworded, one
> from the talks if any lead designer calls change it, still trumps all else. The spirit.

Two things follow, and the second is the one a member is most likely to miss.

**The intent is supreme.** Nothing in `16b`, `16c` or this file overrides it. Where a posture
instruction and the design's actual intent point different ways, the intent wins and the instruction
was badly worded.

**The existing code was an attempt at that same intent.** It was not good enough, which is why the
rework exists. But it was reaching for the same thing, which is precisely why its shape is worth
keeping where it still serves, and why treating it as merely wrong is as much an error as treating it
as authoritative. The version of the intent that governs is the one as reshaped and reworded through
the talks, wherever the lead designer's calls have moved it.

## The intent is vague, and that is a property rather than a defect

> It can be vague and as such can't be taken literally, just inferred and evaluated against. It's
> subjective work, which means the experts can't choose one angle to it, which the rules and skill
> already formalise as a clear rule.

The intent cannot be read literally. It is inferred, and a design is evaluated against it rather than
checked off from it. That makes this subjective work in the exact sense that two competent readers can
infer differently from the same intent and both be reasoning honestly.

Which is why a member may not resolve to a single angle. That rule already exists and applies
throughout this review; the vagueness of the intent is the *reason* for it rather than an awkward
exception to it. Carry the readings, say what distinguishes them, and leave the choice where it
belongs.

A member who finds themselves certain about what the intent requires should treat that certainty as a
signal to look for the reading they have discarded.
