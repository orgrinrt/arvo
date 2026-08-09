# Op's sixteenth checkpoint: back in the loop, and a regression corrected

**Date:** 2026-08-04
**Position:** after `68_consolidation_seven.md`. **Required reading** with the consolidation.

Op is back. The overnight persona-checkpoint override (`48b`) ends here; every checkpoint from now is
op's own. The five persona checkpoints (`48b`, `53b`, `57b`, `62b`, `67b`) stay in the record as
persona-decided, and op is walking their calls individually rather than confirming them as a block.

## The scope correction, stated first

Op:

> I noticed you were talking about writing implementations and such in the mockspace; we are strictly
> designing here. The panel works in research subdir intentionally; we want to formalise, fully define,
> the ideal shape to set as the new canon. This is our job. Not implementing anything. We will settle
> the canon in full, and then start a design round about the new settled taxonomy, creating it and its
> docs, then implementing in source it as stubs. Then we'll start doing design rounds where we go
> through the settled canon piece by piece to implement it into the stubs. But we will not go there
> until the full design is settled, the spec is complete and answers all, and we can earmark it as the
> first full canon in arvo, to guide all future work.

So `67b`'s authorisation of the `arvo-strategy` migration round is **withdrawn**. It was never executed
and it should not have been issued: the panel produces canon, not source changes.

The boundary, as op stated it separately: **benches and sketches are fine and in fact encouraged**,
because the design cannot be settled without measured results. Only `mock/crates` is forbidden. The two
files that landed work in `mock/benches/` therefore stand, including the harness repair.

## The four calls confirmed

- **`Int` is dropped** from the ratified table. Three independent reads, empty grounding set.
- **Exponent bounds are types**, both the ranged numeral's pair and the fixed-exponent numeral's single
  one. Two independent compiles; the const route closed under every permitted feature.
- **`Radix` is sealed**, so radix 0 and radix 1 become unspellable.
- **`Specials` is a product**, not a chain, **pending the primary-source check** on the `E4M3` exponent
  figure against the specification rather than vendor documentation.
- **The layer-keying rule** is confirmed as a design rule, beside the spine rule and carrier-at-birth.
- **The transfer-ground scheme** is confirmed.
- **The `TotalOrd` split** is confirmed.

## The regression, and what it voids

The dispatcher justified file 59's preset-forcing argument to op by quoting shipped doc comments. Op:

> We, as said before, shouldn't reference or compare existing code or its comments, they are by
> definition deprecated and wrong on the new design. And also, we are fully free to restructure the
> strategies and their meanings.

Correct, and the defect is in file 59 itself rather than only in the summary of it: `59`'s table
justifies three of its four rows as "forced by their own shipped doc comments". That is a reading of
code this round replaces, not a derivation from the design, and no reader caught it across three
subsequent files.

**Two rows revert to open.** `Hot` and `Precise` survive re-derivation from the design's own statement
of intent (`202607301100_topic.the-formalization-talk.md:1659-1661`): "as fast as possible" is the
hardware door, and "most precise at the price of both storage and compute" is the software quantiser.
`Warm` and `Cold` were never established.

## Op's statements of the two open presets

**Warm**, verbatim:

> For warm, I think we should assume that it'll work the same as writing regular old floats would work.
> If that takes the hardware door, then that's truly the intent behind it. The intuition is that it
> works and behaves as f32 and f64 etc in rust today without any framework on top of it. For hot and
> precise and cold, we explicitly lose that intuition for their intended behavior instead.

**Cold**, verbatim:

> It should be something between warm and precise. Cold also tells us it's seldom computed or used,
> it's on a cold path. It can take more cost than warm, but shouldn't just be precise in disguise.
> That's the intent. As to what it concretely means under current design, I am not sure, since I was
> away for the night which did most of the design converging.

So `Cold` carries two meanings, not one: **cold storage and cold path.** Seldom computed, so it may pay
more than `Warm`, and it must remain distinguishable from `Precise` on at least two cells per D71's own
construction.

## The larger thing this exposes: D71 has been overtaken

D71 (op, 2026-07-30, in the talk topic) redefines the four presets across six axes with every pair
differing in at least two cells. It is op's ratified canon for the presets and the panel has not
re-derived it once, while ratified panel calls have moved the ground under two of its rows: `Widening`
was removed as an axis entirely, and `Growth` left the law key.

And D71 was derived for fixed-point numerals. The float case has never been given a reading of any
preset, and op's `Warm` statement collides with D71's own `Warm` row: a plain Rust `f32` saturates to
infinity out of range, which is neither `clamp` nor `reduce modulo` nor `refuse`.

## The two dispatches this sets

1. **The shipped-source-justification sweep.** Op confirmed it. One dispatch audits every panel file for
   conclusions justified from shipped source or its comments rather than from the design record, and
   reports which survive re-derivation and which do not. The review has caught this defect class four
   times in other forms and has never checked itself for it systematically.
2. **The preset re-derivation.** D71 restated against the current axis set, plus a float reading of each
   preset, with op's two statements above as the intent that governs.

## Standing

Convergence and the novelty posture hold. The intent outranks every instruction, is vague on purpose,
and is inferred rather than read literally. Only op's calls are final. The panel produces canon; source
work waits until the canon is complete and earmarked as arvo's first full canon.
