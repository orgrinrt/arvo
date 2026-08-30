# Op's checkpoint after the first four, and the directions it sets

**Date:** 2026-07-30
**Position in the panel:** written between `04_torvalds_does_it_earn_its_keep.md` and
`05_leijen_fallibility_without_poisoning.md`. Panellist 05 received this content inside its dispatch
prompt; every member after 05 reads it here, as a panel file, alongside the others.

**Required reading for every subsequent member**, together with the numbered expert files.

This file exists because op is a member of this panel, not its audience. It was initially omitted,
with op's steer folded into a dispatch prompt instead, which is the mistake the workspace's own panel
rule names as a *good pattern* being skipped: the vehje certified-generation panel carries op's
mid-flight response as its third file, and every later expert there inherited it directly rather than
through an intermediary's paraphrase. Recorded so the omission is visible rather than tidied away.

## Standing instruction on how this panel runs

The panel keeps running. **Op will say when it is ready for synthesis**; no member should treat itself
as the last, and no member should write a synthesis unless explicitly dispatched to. Op takes a
checkpoint like this one after every two experts.

## What op said, in op's words

On the diagnostics finding and the newtype-faces alternative that panellist 04 opened:

> Option 1 but not just price, iterate on; there might be ergonomics to be won when taking further
> and specializing, instead of stopping at this solution

On `Precise` being fallible and the exile from the algorithm crates that 04 and 02 established:

> Option 2 feels most at home here, and perhaps objectively is the "ideal" choice we want anyway. But
> again, I feel there are potential yet untapped

On panellist 03's proposal of compile-time const checks against an oracle, to make leaf facts
falsifiable rather than typed by hand:

> Option 3 but again, iterate on it with the experts further, perhaps a better shape emerges and
> something that might make it fit typestate better

On the panel's continuation:

> No we will keep running this panel, I'll tell you when we are ready for synthesis. Let's get
> similar checkpoint with me every 2 experts, too

## What that sets for the members after this

Three threads are explicitly **open for iteration, not settled**, and op's answer in each case was to
push further rather than to pick from what was offered. A member that treats any of the three as
decided has misread this file.

**Thread A, the consumer-facing surface.** Panellist 04 verified that rustc expands type aliases in
diagnostics, so the spec's alias story keeps the input spelling and destroys the error spelling, and
it proposed concrete newtype faces over an internal composition as a structural fix. Op's direction is
that stopping at newtype faces is stopping too early: the question is what ergonomics are available if
the surface is taken further and specialised, not merely whether faces beat aliases.

**Thread B, fallible arithmetic.** Op keeps `Precise` refusing and prefers widening the algorithm
crates to accept it, while stating there is untapped potential in the shape. So the live question is
what the best possible form of fallible arithmetic in a no-std, no-alloc, monomorphisation-only
substrate is, and what it *unlocks*, rather than what the current form costs.

**Thread C, leaf truth.** Panellist 03 established that the type machinery delivers totality and
coherence but never the truth of a leaf fact, and proposed solver-free const checks. Op's direction is
to iterate on the shape rather than adopt the sketch, and specifically to look for a form that fits
the typestate discipline better than a check bolted alongside it.

## One standing note on authority, from op

Restating what the shared brief already says, because it bears on all three threads and on every
finding the panel produces:

Only op's calls are final, and even those go stale the moment something better surfaces or new
material appears. The spec's calls are one day old. Any member is free to argue against any of them,
including the ones this file has just reaffirmed, provided the argument is made rather than asserted.
