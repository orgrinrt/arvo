# Op: be solution oriented, not only adversarial

**Date:** 2026-08-01
**Position in the panel:** written during file 25's run, which received it mid-flight.
**Required reading** for every member from here, with `13c`, `16b`, `16c` and `16d`.

## The correction

Op:

> I think we'll want to get these people a bit more solution oriented too, not just literally attacking
> and finding holes alone.

This is a correction to how members have been briefed, not to how they worked. The dispatching prompts
slid, over several members, into "test this claim, find its hole", and the members correctly did what
they were asked. The drift was mine.

**Every member owes a constructive deliverable, not only findings.** Adversarial verification is
genuinely valuable in this review and has overturned findings repeatedly, including several a member
found in their own work by compiling it. But it is the floor, not the work.

Concretely, from here:

- Where something fails, say **what should replace it**, in enough detail that someone could build
  from it.
- Where a proposal holds, **take it further** than its author did rather than stopping at
  confirmation.
- Where a gap is found that nobody has named, **propose the shape that fills it**.
- A proposal offered with stated low confidence beats a finding offered alone. Say which it is and
  move on.

## The evidence is this panel's own record

The files that moved the design furthest all built something rather than only reporting:

- the relocation of rounding from multiplication to narrowing, which made the multiplicative half
  tractable at all
- a working verifier at 68 lines, catching a defect an earlier member had concluded could not be
  caught inside arvo
- three concrete mechanisms for crossing the abstraction boundary between a composition and the
  generic code that consumes it
- a graded reading that deleted a mechanism the design believed it needed, along with a cross-crate
  dependency it did not know it had

The least valuable contributions, by contrast, have been correct catalogues of what is wrong with code
nobody proposes keeping, which `16b` already addressed for a different reason.

## What does not change

The standing posture is unchanged and this does not soften it. Carry more than one reading of anything
substantive, do not resolve to a single thesis, suggest rather than rule, and be honest where your
field's answers do not transfer. Several members have delivered exactly that negative verdict about
their own subject and it was valuable each time; a negative verdict *with* a proposal for what to do
instead is more valuable still.

Only op's calls are final, and even those go stale.
