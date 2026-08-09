# Op's thirty-sixth checkpoint: everything varies granularly, and a constant is a function

**Date:** 2026-08-07
**Position:** after `143_orchard_the_profile_axis_and_the_tiers.md`, while `144` is in flight.
**Op calls this settled canon**, in those words, and says it governs future talks. It is not a working
answer to be revisited.

## The ruling

> Function can also be a constant. It's not either or there. And all things change and act granularly, not
> just warm. I call this as intent, settled canon, right now. This small bit in this association now governs
> future talks.

## Part one: the dichotomy was false, and the framing was the dispatching agent's

`142c` framed the finding as cells being functions **rather than** constants, and every dispatch since has
inherited that framing. It is wrong.

**A constant is a function.** A cell that does not vary is a constant function over its domain. So there was
never a choice between the two, and there is no category of design element that is "a constant" as opposed
to "a function".

The consequence is not verbal. It changes which claim carries the burden of proof:

- **Being constant is now the special case**, and it is a claim about a function's behaviour over its
  domain. It has to be established, not assumed.
- **A design sentence stating a fixed value without naming its domain is underspecified by construction**,
  because it has asserted the value without saying over what it holds.
- So for any cell or fact the design states, two things are owed: **what it varies over, and where it is
  constant.** Both halves, always.

## Part two: everything varies granularly, not only `Warm`

This closes a live disagreement, and it closes it past both positions.

`143` argued that build-condition dependence reaches `Warm` alone, on the grounds that `Warm` alone is
defined by imitation of a native primitive. `142c` argued it is a general property of the strategy axis.

**Op's ruling is broader than either.** All things change and act granularly. Not only the build condition,
and not only the strategy axis. Granular variation is the general behaviour of the design's elements, and
`Warm` is simply where the panel noticed it first, because imitation makes the variation visible where
elsewhere it is quiet.

**So `143`'s conclusion on that point rests on a premise that is now overturned.** `144` is the second read
and has been told to say what survives the correction and what does not, rather than adjudicating the
question, which is closed.

## What this does to the panel's accumulated work

**It generalises `142c`'s finding rather than confirming it.** The panel had been treating the preset tables
as fixed grids and was corrected to treat them as varying with the profile. The correction is larger: they
vary, and so does everything else, and the profile is one of the things they vary over rather than the only
one.

**It reframes every "the cell is X" statement in the standing base.** None of them are wrong for being
stated; they are incomplete wherever they do not say over what the value holds. That is a large surface and
it is not a call to go and annotate all of it now. It is the standard the consolidation is written to when
it is written.

**And it is the third instance of the same shape.** A constant standing where a derivation belongs, at
`StoredWidth = doubled`. A constant standing where a function belongs, at the preset cells. And now the
recognition that the distinction itself was misdrawn, because the constant was always a function whose
domain nobody had stated. The standing check adopted at `139b`, that a written artifact standing in for a
derivation is a defect named at the point it appears, is the same rule seen from a fourth side.

## Standing

This is ratified canon by op's own words and does not go stale on the usual terms; it is intent rather than
mechanism, and mechanisms move underneath it. The panel produces canon, not source; `mock/research/` and
`mock/benches/` are its ground and `mock/crates` is out of bounds. Experts are dispatched one at a time,
each reading the ones before it, each writing incrementally, each going down the rabbit hole rather than
reporting blockers.

---

# Clarification, same session: incompleteness is the plan, not a finding

The section above treated the standing base's fixed-value statements as newly revealed to be incomplete, and
framed that as a large surface to be measured against later. Op corrected the framing:

> Yes it is incomplete by a lot. I was under impression we are first tackling the basic shape, perhaps the
> one we reserve for debug assertions time, and we write separate arms for release and such then? It's
> always been incomplete. Nothing changes in standing base.

**Nothing changes in the standing base.** Not now, not at consolidation time as a correction pass. The
incompleteness is not a defect that was just discovered; it is the shape of the work in progress and always
was.

## What the existing tables are

**One arm.** The preset tables and the fixed cells the panel has been arguing over are the **basic shape**,
and op's reading is that this is plausibly the arm reserved for debug-assertions time. The arms for release
and for other conditions are written separately and later.

That is a materially different reading from the one the section above implied, and it is better in three
ways.

**The existing cells are not wrong and not broken.** They are one arm of a function whose other arms are
unwritten. A cell stating a value is a complete statement about its own arm.

**Nothing is owed retroactively.** There is no annotation pass, no sweep, no re-statement of existing
sentences with domains attached. The domain is the arm, and the arm is identified by which table the cell
sits in.

**And the future work is additive rather than corrective.** Writing the release arm does not edit the debug
arm; it adds a table beside it. Under the chain's own rules that matters, because an append invalidates
nothing while a modification invalidates the declared dependents.

## What this does to the ruling above

It does not soften it. Everything still varies granularly, a constant is still a function, and a value still
holds over a domain rather than absolutely. What changes is where the obligation lands: **the domain is
carried by which arm a table belongs to**, rather than by annotating each cell with the conditions it holds
under.

So the question for any cell is not "what is missing from this sentence" but "which arm is this table, and
which arms are not written yet". The first has an answer today. The second is the roadmap.

The panel should therefore stop treating unstated variation as a gap to be filled in place, and start
treating the arm as the unit that gets written, reviewed and eventually promoted.
