# Op's thirty-third checkpoint: Warm behaves like Rust, and his intent keeps not sticking

**Date:** 2026-08-07
**Position:** after `140_fog_warm_without_widening.md`.
**Required reading with the standing base.** The Warm statement below is a standing intent, not a
mechanism, and it governs every later container and strategy decision.

Op is present and every call here is his own.

## Warm behaves like a native Rust primitive, and the intent outranks the mechanism

`140` closed by flagging that its whole dispatch, and `139`'s before it, rested on an untested assertion:
that `Warm`'s semantics are wrapping at the declared width `W`. Four files had carried it from one line.
Op's answer moved the question out of the mechanism entirely.

> WE are the designers of the strategies and how we define them. WE are the architects. Which means, we can
> arbitrarily choose whatever. My standing call is "It should behave like native primitives in regular old
> rust would". Warm is the name for the default case that is not optimised for cold paths and cold storage,
> nor does it emphasise precision, but it also doesn't need to excessively shed any inefficiencies at the
> cost of accuracy and stability like Hot. It's the intuitive default way things behave, as they do behave
> with regular old primitives.

And the principle underneath it, which is the more important half:

> The intent, here, is what matters. The mechanisms and theory may live freely and shift under and around
> it, the intent is what remains and matters.

So `Warm` is defined by **what a consumer expects**, not by a rule someone derived. It is the intuitive
default: not tuned for cold paths or cold storage, not emphasising precision, and not shedding
inefficiencies at the cost of accuracy and stability the way `Hot` does. A consumer who never reads a word
of documentation should find it behaves the way plain Rust primitives behave.

That is a constraint on the container rule rather than a consequence of it. Any storage or overflow
mechanism is free to change as long as the experienced behaviour stays the one a Rust programmer already
has in their hands.

## Op has now told the panel twice in two days that something is already written and does not stick

This is a finding about the canon rather than about arvo, and it is the second instance inside two
checkpoints, so it is a pattern rather than an incident.

On the aliases, at `138b`: "We've already established this, I believe it is written down too, somehow it
just doesn't stick. So find a way to make it stick."

On Warm, here: "That's already in there, as a ruling, at least should be, since I've written these very
words at least once before... AND this, too, is already written and established, this one I've written down
and spelled out several times already, not even just once."

**Both are intent statements, and both are the kind of thing the panel keeps re-deriving from mechanism
instead of reading.** The design's mechanisms are recorded well and its intents are recorded somewhere a
later member does not look. That is a presentation defect in the canon and it is now the second time op has
had to restate something rather than cite it.

The instruction from `138b` therefore generalises: the canon needs a form that makes an intent statement
stick, not another statement of it. Whoever writes that section owes an account of why a reader keeps
forming the wrong model from the current presentation. **Warm's definition and the alias statement are the
two known instances and there are probably more**, since nobody has looked for the class.

## The headroom rule: structural constraint noted, calls held, fresh eyes owed

`140` proposed deleting headroom for every strategy, with the container the minimum aligned native for `W`
and the width discipline moved onto the operation's result. It then **conceded its own pricing half**,
which is the new evidence rule working on the first dispatch after it landed: it read the rule mid-flight,
applied it to its own instruction counts, and reported the magnitude as unpriced rather than defending the
numbers.

Op's disposition, and the three parts are not exclusive:

> Note the structural constraint, hold calls on it until there are actual benches, and until someone is
> confident enough on their take to do the benches, it's not an answer anyone asked for. Which is entirely
> valid as we've now established, but requires fresh eyes on the case before it should ever come to me.

So: **the structural findings are recorded, no call is made, and this does not return to op until someone
has run harness benches.** A proposal that cannot price itself is a legitimate concession under the rules
adopted this session, and it is explicitly not an answer to the question asked.

The structural findings that survive without magnitude, and that a bench would confirm or refute:

- Headroom is redundant where it works, because below the rung a projection onto `W` is required regardless.
- The projection is **not per-operation**. `x mod 2^W` is a ring homomorphism for the operations concerned,
  and eagerly-projected and lazily-projected forms **fold to the same symbol**, so no design mechanism is
  needed to defer it.
- The headroom container forfeits the vector form entirely across a band of widths.
- `Precise` wants the deletion more strongly than `Warm`, because at an exactly-filled width the machine has
  a single saturating-add instruction and nothing built over a wider container can be it.

Four harness benches are named as owed, with real competitor arms rather than the current rule against one
proposal.

## The fourfold error, corrected, ruling intact

`139` reported Warm's wide form at roughly 1600 instructions against 81 and op ruled on it. The figure is
wrong by a factor of four: the loop is unrolled four elements per iteration, so the trip count over
sixty-four elements is sixteen, and the real figure is **339**. Every rolled figure in `139` carries the
same factor.

**Op's ruling stands**, because he ruled on structural ground: a loop over values fitting a native register
must not lose its vector form. Corrections are marked in place at `139` and `139b`. Under the evidence rule
neither figure prices anything anyway.

## What became standing procedure, encoded outside the panel

Op asked not to be consulted case by case on these again.

**Recovering a claim that rests on an uncommitted spike.** If the spike exists, recover it into the probe
directory beside the citing file and commit; that is the whole remedy, with no audit pass and no rewrite. If
it cannot be recovered, write an explicit addendum as a new file naming what is void and what depended on
it. If recovery works and the wording is already accurate, stop. **Do not police whether someone else's
probe was a good probe**: every expert validates what they themselves depend on, which is where a worthless
spike gets caught, and a true thing survives being re-checked.

**One instance of evidence is never enough.** Three or more independent instances is the preference, and
independence matters as much as the count, since three probes sharing one model or one author's assumption
are one instance wearing three hats. This composes with the two-expert agreement rule: two experts agreeing
is about provenance, three instances is about the claim.

Both are in `.claude/rules/evidence-lives-in-the-repo-or-it-never-happened.md` and inline in all 65 persona
files.

## The recovery this session

Fourteen probe directories, 361 files, moved from scratch into the repo as `119_probes` through
`140_probes`. That includes `130_probes`, which file `138` had reported as **absent** when it found `130`
citing five probes that existed nowhere. They existed, in scratch. One defect produced both a false absence
and an unverifiable claim, which is the rule's case made twice in a single instance.

## Standing

Only op's calls are final and they go stale when their evidence moves. The panel produces canon, not source;
`mock/research/` and `mock/benches/` are its ground and `mock/crates` is out of bounds until the canon is
complete and earmarked as arvo's first full canon. Experts are dispatched one at a time, never in parallel,
each reading the ones before it, and each writes its file incrementally.

The consolidation is promoted to canon whole and supersedes everything before it, so comprehensiveness is a
requirement rather than a virtue. The erasure gate at `135b` and the Warm intent above are both part of what
it is measured against.
