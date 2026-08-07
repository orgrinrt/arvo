# Op's thirty-second checkpoint: the answer still evades, and conceding is now a valid answer

**Date:** 2026-08-07
**Position:** after `139_ovadia_the_derivations_that_stop_short.md`.
**Required reading with the standing base.** Two standing instructions here bind every future dispatch and
have been encoded outside the panel; see the last section.

Op is present and every call here is his own.

## The findings hold. The solutions do not.

`139` established real things and then proposed answers op has refused. Both halves matter and they are
separable, which is the shape of this checkpoint.

**Warm.** The measurement stands: the current rule is `rung(rung_bits(W)+1)`, a rounding applied to a
rounding, and it widens 64 of the 64 widths at or below 64 bits. Warm's `u128` form at 64 bits is a rolled
scalar loop against thirty-two `add.2d`, roughly 1600 instructions against 81 over sixty-four elements. Op:

> Okay, so 1600 instructions for a simple loop with something that should be native literals llvm can
> optimise as they wish, is unacceptable. So the conclusion and the findings hold, but the solutions
> proposed don't. So we find a better answer.

So the headroom rule is condemned and neither offered replacement is taken. The deeper finding underneath
it, that below the rung a mask is required regardless so headroom and mask are two mechanisms for one job,
survives as a finding and is the most promising thread.

**The bridge.** `139` found the wall is arithmetic rather than const-to-type, which is new and useful:
`type const` associated consts do reach const-argument position under the allowed feature, and the
diagnostic names the forbidden feature by name. Its proposal was a function-like proc macro, `UFixed!(13, 3,
Warm)`, twenty lines, no table, no cap, no feature, no flag, with the erasure gate holding straight through
it. Op:

> We'll gladly take all the proc macro crates we need and other optimisations alike. However, using a macro
> invokation in place of a type is not what we want, and I've already ruled on this. The answer still evades
> us, and finding it is the job, *not* settling for a solution we've already ruled out.

Note the split precisely, because it is easy to collapse: **proc-macro crates are welcome**, and arvo may
take as many as it needs. **A macro invocation standing where a type should be written is refused**, and was
refused before. The mechanism is fine; the surface is not.

**The stored width.** The layout slack finding stands and needed nothing new in the typestate. But
`110:3248`'s `W_F <= W_S` is backwards at decimal32 and `139` could not tell which width `W_F` denotes. Op:

> The `W_F <= W_S` is ambiguous enough that if any of the described findings or solution is built on it, it
> needs a new look from someone else that might understand it better.

So anything resting on that bound is held until someone who can read it correctly has looked.

## Conceding is a valid answer, and it is now a standing instruction

This is the most consequential thing in this checkpoint and it is about how the panel works rather than
about arvo. Op:

> The experts have to concede if they can't find the answer, and "I can't find it, I'm not good enough,
> someone else has to try in place of me, as I've exhausted my brain and found no answer" is a valid answer
> from these expert calls. In which case, it's honest, and doesn't propose a solution as a vacuous
> ceremonial habit. The solution they deliver can also be "Someone better or more versed in this specific
> thing needs to look at it". Or "This problem needs fresh eyes to contribute, I wasn't able to crack it
> alone".
>
> We need to encode that into all of the agent persona files, as well as any related rules we have for how
> these flows work, as well as our own context and brief docs for this specific panel.

The failure being corrected: a dispatch that ends without an answer feels incomplete, so the expert reaches
for something to fill the answer slot. What lands there is a refused option restated, a compromise nobody
asked for, or a mechanism that satisfies the words of the question and misses what it was for. **That is
worse than a concession**, because it looks like progress, it enters the record as a candidate, and the next
reader spends their time refuting it rather than solving the problem.

A concession is a report rather than a shrug: what was tried, why each attempt failed with the diagnostic or
measurement that closed it, where the wall is as precisely as it can be stated, what kind of help would move
it, and any least-bad residue marked as a residue rather than as a proposal. The enumeration of dead routes
is usually the most valuable part.

## An expert's code is a spike and never canon

Op's clarification, which governs how every probe in this panel is read from here:

> Any written code by prior experts are not canon and shouldn't be treated as such. They should build
> towards the design, the spec, and any code they write, is purely just spikes, sketches, not worthy of
> referencing as other than one instance of proof for or against something. None of it becomes canonical and
> none of the decisions made in those spikes or sketches are intended to be taken literally. Derive the
> intent of the check (they all check something, none of it aims to write actual shipping code, so all of
> them take shortcuts everywhere to test the one thing they want to test, and even at that, should be
> presumed flawed, shortcuts taken, invalid, etc.)

So: a probe is one instance of proof for or against one thing, and that is its entire evidential weight. Its
incidental decisions, names, arities, field orders, trait shapes, which case it instantiated, are
scaffolding rather than design. It is presumed flawed by construction. **Cite a probe for what it proved,
never for how it was written.**

This is not hypothetical in this panel. A probe set a definedness flag from a refusal flag, which made a
lattice identity true by construction rather than testing it, and the resulting false claim stood in the
design for dozens of files. Separately, `130`'s section 10 cites five probe files by name, none of which
exists anywhere in the repository.

## The pattern becomes a standing check in the canon

**Adopted.** Each of the three short derivations compensated for a missing derivation with a written
artifact that then read as a design element: a table, a declared member, a blunt rule. The canon states the
check: **a written artifact standing in for a derivation is a defect, to be named at the point it appears**,
so a later reader can tell one from a real design element.

Note that this is the same failure as the two instructions above, one level up. A ceremonial proposal is a
written artifact standing in for a solution nobody found. A probe's incidental spelling is a written
artifact standing in for a decision nobody made. All three read as progress and all three cost the next
reader more than the gap they concealed.

## What was encoded outside the panel

Per op's instruction, this is not a panel note. It is a workspace rule:

- **`.claude/rules/conceding-is-an-answer-and-expert-code-is-a-spike.md`**, new, both halves stated in full
  with op's words quoted.
- **All 65 persona files** in `.claude/agents/` carry the block inline, so a badly framed brief cannot
  disarm it.
- **`expert-dispatch-defends-the-canon.md`** and **`panels-argue-the-intent-not-the-wording.md`**
  cross-reference it, since the first governs what a dispatch must carry and the second bounds the
  constructive-deliverable obligation: owed where an answer exists, never manufactured where one does not.

## Standing

Only op's calls are final and they go stale when their evidence moves. The panel produces canon, not source;
`mock/research/` and `mock/benches/` are its ground and `mock/crates` is out of bounds until the canon is
complete and earmarked as arvo's first full canon. Experts are dispatched one at a time, never in parallel,
each reading the ones before it, and each writes its file incrementally.

The consolidation is promoted to canon whole and supersedes everything before it, so comprehensiveness is a
requirement rather than a virtue, and the erasure gate at `135b` is part of what it is measured against.
Both open questions, the bridge and Warm's rule, come before the consolidation on op's call at `138b`.
