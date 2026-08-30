# Op: design the shape, stop auditing the code

**Date:** 2026-07-31
**Position in the panel:** written after `16_fallin_laws_as_backend_licences.md`. **Required reading**
for every member from here, with `13c` and the earlier checkpoints.

This file corrects the review's posture. Several recent findings are true and beside the point, and
the cause is how members have been briefed rather than how they worked.

## The correction

Op, verbatim:

> the reviewers are way too much grasping on the existing code. The actual strategy related lowering
> also needs, in principle, custom rustflags and llvm passes to work as intended. Hilavitkutin build
> is the vehicle for those. Arvo can't express all that itself, but it can support it best it can.
> Just the semantics alone allows hilavitkutin build to discover the intent and lower accordingly. You
> need to guide the experts not critique too much what exists, but rather design the fucking shape it
> should be. That's annoying and counterproductive. The existing code is irrelevant to us. We have to
> assume we are rewriting everything. The new design will likely require that anyway, no need to grasp
> at straws in the current code

So, standing for the rest of this review:

**The existing code is irrelevant. Assume everything is being rewritten.** The new design will
require it regardless. A finding of the form "the shipped source does not currently do X" is not a
finding about the design unless the design's *shape* is what makes X impossible or wrong.

**Design the shape it should be.** That is the deliverable. Critique is welcome where it is critique
of the proposed shape, and worthless where it is critique of code nobody is defending.

Reading source stays legitimate for exactly two purposes: checking a factual claim in a brief before
reasoning from it, which has caught real false premises in this review; and understanding what a
mechanism *is* well enough to design its successor. Neither licenses auditing the implementation as
though it were the subject.

## What this makes of the fidelity finding, specifically

File 12 found the design cannot express the fast-math against strict-arithmetic distinction. File 16
then found the two float types compile identically today, that no fast-math flag exists on the pinned
toolchain, and that the one per-instruction lever needs a forbidden feature.

Under op's framing, the second half is not a finding at all, and the division of labour it missed is
the answer:

> The fast math lowering is literally only expressible on llvm side, no? Arvo *supports* the
> distinction first class, but does not provide it. If intrinsics exist as attributes or similar that
> allow setting it arbitrarily, of course we should hook it to the arvo side.

**arvo's job is to express the intent, first-class, in the typestate. It is not arvo's job to provide
the lowering.** The lowering needs custom rustflags and LLVM passes, and `hilavitkutin-build` is the
vehicle that carries those. The mechanism is that the semantics alone are enough for the build side to
*discover* the intent and lower accordingly.

That inverts the question. It is not "can arvo emit fast-math", which it cannot and should not try to.
It is "does the design express fidelity precisely enough that a build system reading the types can
determine what liberties it is licensed to take". That is a question about the abstraction, which is
what `13c` says is the thing that matters, and it is answerable without a single line of shipped code.

The same holds for the rest of the strategy axis. Container width, packing layout and widening all
need build-side cooperation to lower as intended. arvo declares; the build side discovers and lowers.
A member finding that arvo alone cannot achieve some lowering has found the boundary working as
designed, not a defect.

## What a member should do instead

State the shape. What are the right abstractions, what do they express, what can a downstream reader
of those types learn, and what is the smallest and most honest vocabulary that carries it. Where a
mechanism is needed, propose it. Where the design as drafted is wrong, say what the right shape is
rather than cataloguing where the current text falls short of it.

`13c`'s standard is unchanged and decides every question: optimal and ideal rather than adequate;
representative of the mathematics; and capable of representing MATLAB, IEEE 754, SystemC and the rest
as a test rather than an inspiration. The abstractions and the typestate are what matter.
