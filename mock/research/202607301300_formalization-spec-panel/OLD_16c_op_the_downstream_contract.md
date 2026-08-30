# Op: the spec is the subject, and the downstream contract is part of the design

**Date:** 2026-07-31
**Position in the panel:** written immediately after `16b`, extending it. **Required reading** for
every member from here.

`16b` said stop auditing the code. This sharpens what to do instead, and adds an obligation no brief
in this review has carried.

## The spec is what gets critiqued

Op, verbatim:

> Make them critique and analyse the spec we are writing. Current code can exist as extra info about
> why we are redesigning; it is wrong, broken, insufficient, we know this, this is why the panel
> exists.

So the subject is the spec. The existing code has exactly one remaining use, which is evidence about
*why* the redesign is happening, and it should be read lightly and for that.

The reasoning is worth stating because it disposes of a whole category of contribution:

> the premise is, anything that exists nowadays, can be overwritten and shouldn't be analysed too
> closely, because the very reason we are reworking things already builds in the implication that the
> existing shit is fucked

Nobody convenes a panel to redesign something that works. The current state being broken is the
premise of the exercise, not a finding within it. Demonstrating it again, however rigorously, spends a
dispatch on something already agreed.

## The obligation nobody has been given: design the downstream contract

This is new, and it applies to every member from here.

The design deliberately stops short in places, because the mechanism belongs to a build layer, a code
generator, or another repository. Custom compiler flags and passes are the clearest case: arvo
expresses intent in the typestate and a build layer discovers that intent and lowers against it.

Every member now owes that boundary a **design**, not an observation. For the part of the design they
touch:

- **How it works for a downstream target doing the lowering.** Concretely. What that target reads out
  of the types, what it can determine from what it reads, and what it does with it.
- **What arvo needs back from that target** for the intent to be realised, where arvo cannot express
  the thing concretely on its own side.

Op's words on the standard for this:

> It has to be documented and designed for, no handwaved, but we should acknowledge this and answer to
> it

So the requirement is stated, designed for and written down. It is never left implicit and never
deferred to whoever notices it later.

## And what is not wanted

> but not fault arvo or the design for being unable to express a thing it fundamentally can't, unless
> we write our own build harness on arvo end that is a pita to maintain and keep compatible with
> others like hilavitkutin build, and also, extremely inconvenient for downstream users to adopt

A substrate that would have to grow its own build harness to close a gap is being asked to take on
something painful to maintain, hard to keep compatible with the build layers that already exist, and
inconvenient for every downstream consumer to adopt. Naming that gap as a defect of the design is not
analysis, and it has now happened more than once in this review.

## What is very much wanted

> However, answers to these problems that I haven't thought of, are 100% welcome and should be
> considered, instead of stating existing faults and limitations etc

A way to close one of these boundaries that nobody has thought of is worth more than any number of
observations that the boundary exists. If a member sees one, that is the contribution, and it should
be developed rather than mentioned. The boundary being hard is exactly why an unexpected answer to it
is valuable.

The standard from `13c` is unchanged and still decides: optimal and ideal rather than adequate;
representative of the mathematics; capable of representing MATLAB, IEEE 754 and SystemC as a test. The
abstractions and the typestate are what matter.
