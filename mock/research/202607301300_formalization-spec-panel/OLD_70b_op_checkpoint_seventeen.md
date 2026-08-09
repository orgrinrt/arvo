# Op's seventeenth checkpoint: the presets ratified, and the rhythm turns to exploring

**Date:** 2026-08-04
**Position:** after `70_wronski_the_presets_re_derived.md`. **Required reading** with the consolidation.

## Both preset tables are ratified

**Fixed-point**, D71's construction with the two dead rows dropped: `Hot` truncates toward negative
infinity, reduce-modulo out of range, minimum width, dense. `Cold` and `Warm` round nearest ties-to-even
and clamp, differing in stored width and layout. `Precise` refuses out of range at doubled width. Every
pair still differs in at least two cells. The lowering door is **inert** for fixed-point, since native
ALU operations and the quantiser composition fold to identical codegen.

**Float**, newly derived: the same in-range directions, but `Warm` and `Cold` reach the far point where
`Specials` supports it rather than clamping, `Warm`'s stored width drops to minimum, and the doors split
`Hot` and `Warm` on hardware against `Cold` and `Precise` on software.

The `Warm` stored-width divergence is the sharp cell and it is derived rather than assumed: a real FPU
delivers correctly-rounded results with no visible extra storage, while arvo's fixed-point path has no
hardware behind it, so the same intent produces different widths on the two kinds of number.

Both tables survive the mechanical test: delete every shipped-source citation and the argument stands.

## The one open cell gets op's instinct, and a stress test before it locks

The cell: what `Warm` and `Cold` do out of range on a float numeral whose `Specials` carries no infinity.
Nothing to saturate toward, clamping contradicts the plain-float intuition `Warm` exists to preserve, and
refusing would make it `Precise`.

Op's answer:

> Option 2 as my instinct, stress tested and evaluated by an expert still, before locking

So the working answer is **clamp to the largest finite representable magnitude**, on the reading that
without an infinity to reach, the far point *is* the largest representable magnitude and the intuition
degrades gracefully rather than failing. It is not locked. A dispatch stress-tests it against the
alternatives, including the well-formedness reading that refuses such a numeral under those presets at
declaration, and the cell locks on that expert's evaluation rather than on the instinct alone.

## The grounding split is adopted, both halves

`tree-fact` and `tree-meaning` become separate provenance grounds. The first is licensed and covers what
currently exists and how it currently behaves; the second, shipped prose read as design justification, is
forbidden. Every consolidation runs the one-sentence test with its table diff: **does a row's
justification survive deleting its shipped-source citation?** If not, it was never grounded in the
design.

The existing machinery checked that citations resolve rather than what kind of evidence they were, which
is exactly how a well-cited wrong claim passed every gate and propagated through four documents before
op caught it on his return.

## The rhythm, and where it points next

Op's stated shape for the remaining work: settle the current focus, then explore for more, then close
the findings, then explore again, alternating until a full spec emerges that is proven, valid, and
importantly **efficient and ergonomic**, in his words invisible for the most part to downstream
consumers while doing real work underneath and lowering transparently to optimal instructions.

This closing stretch is done: the presets, the source-justification sweep, and the persona backlog are
all settled. **The exploring stretch begins**, and op's steer for it is deliberately unpointed:

> Whatever the panel has never looked at.

No aim toward either half of the stated end state. Dispatches go at genuinely unexamined ground, and the
first of them establishes what that ground is rather than the dispatcher choosing it from his own blind
spots.

## Standing

Only op's calls are final. The panel produces canon, not source; `mock/research/` and `mock/benches/` are
its ground and `mock/crates` is out of bounds until the canon is complete and earmarked as arvo's first
full canon. The intent outranks every instruction, is vague on purpose, and is inferred rather than read
literally.
