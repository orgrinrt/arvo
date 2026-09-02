# Checkpoint 40: the shipped source is readable as a prior attempt, not as evidence

**Date:** 2026-08-07
**Records:** op's correction to `145b`, narrowing it to what it was actually about.
**Status:** standing. Supersedes `145b`'s "the shipped source is not read" wherever the two differ.

## What `145b` got wrong

`145b` recorded a real failure. Every brief in this panel had carried the line "`mock/crates` is out
of bounds for writing; reading it is fine", the file before it read the shipped conversion surface and
its tests, and it carried two findings about them to op as deliverables. Op's reaction was to the
findings.

The correction I drew from that was too wide. `145b` said the shipped source is not read during canon
work, and the brief for `146` said it is closed in both directions. Op:

> I mean it is fine to read it for reference of what not to do, and what arvo used to do. But it's
> getting nuked, its designs are getting nuked. The very canon is what we are establishing.

## The corrected line

**Reading is fine. Citing is not.**

The shipped source is legitimate to read for two things, and both are backward-looking:

- **What arvo used to do.** It is a prior attempt at the same intent, by people who were reaching for
  the thing this panel is formalising. Seeing where it got to is worth something.
- **What not to do.** Where it went wrong is a hazard already surveyed, and walking into it again is
  pure loss.

What is prohibited is **treating it as evidence about what is correct**, and **carrying a finding
about its contents as a deliverable**. Those are the two things that happened in the file before, and
they are what the anger was about. A shipped test's assertion is a fact about an implementation that
has been declared dead. It is not a fact about the design and it settles nothing.

So, concretely, for every later dispatch:

- Read it. Say what the prior attempt reached for, if that is useful.
- Do not cite it in support of a claim about what the design is, requires, permits or intends.
- Do not report "the shipped source does X" as a finding. Nobody is defending it.
- Writing to `mock/crates` remains out of bounds, unchanged.

## Why the wide version was wrong on its own terms

The chain rule (`the-canon-design-code-chain.md`) says the canon governs the design and the design
governs the code, and that a tier may only change after everything below it is nuked. The declaration
that the existing designs and code are dead is what licenses this panel to write canon at all.

That declaration removes the shipped source's **authority**. It does not remove its **existence**, and
it does not make it uninformative. A dead implementation is exactly the artifact you can read freely,
because nothing you say about it can be mistaken for a constraint. The wide prohibition treated a
demotion as an erasure, which is the same category error in the opposite direction from the one it was
correcting.

The panel's own rule already carries the right version, at
`panels-argue-the-intent-not-the-wording.md`: existing code is "evidence about why the redesign is
happening" and "a prior attempt at the same intent", read lightly, and not analysed closely. That is
the standing form and `145b` should have restated it rather than replacing it.

## What this does not restore

`145`'s two closing observations stay withdrawn. They were findings about the shipped source offered
as deliverables, which is the part that is still prohibited. The withdrawal was correct for the right
reason even though the rule I wrote around it was too wide.

The `From` question is unaffected. Op's instruction stands: no enumeration, implicit via blankets and
granular bounds where expressing it otherwise fails. The coherence collision with `core`'s reflexive
impl is real, and what follows from it is that a spelling has to be found, not that none exists.
