# Op's eighteenth checkpoint: the far point ratified, one carrier, and back to closing

**Date:** 2026-08-04
**Position:** after `74_lattner_the_taxonomy_rechecked.md`. **Required reading** with the consolidation.

## The far point is ratified as one rule

**The far point is the supremum of a numeral's ordered representable values.** Op's instinct on the
no-infinity cell survived its stress test and generalised past its own framing: the infinity case, the
ratified fixed-point clamp case and the no-infinity case are three instances of one rule rather than
three cells to fill separately. NaN needs no exclusion clause, because the supremum is order-theoretic
and NaN is not in the order. Compiled as a total const-callable projection across all four `Specials`
members, no feature gates.

The cost is ratified with it rather than smoothed away: a come-back sum saturates to zero against a true
value at full scale, and the mitigation is the design's own witness-carrier decision, a far-point kind
published through the grade with silence dominating the join, plus a consumer opt-in bound that refuses
at the call site.

The alternative (refuse such a numeral under those presets at declaration) is declined with teeth: it
would forbid `Cold` paired with `E4M3`, which is exactly that format's silicon deployment profile, and it
crosses the warn-never-police line. A genuine third option nobody had named, overflow-to-NaN, exists in
shipping silicon, was given its hearing, and is declined for the preset table, with the note that if ever
wanted it is an environment fact rather than a resolution constant.

## One bottom carrier, and `Capacity` keeps its name

**Adopted.** The design currently holds two type-level natural encodings of one concept: capacity as a
type, from op's own migration, and the tower's sealed value-unique naturals, from the encoding work
ratified at `44b`. Nobody decided they should differ; they differ only in when they arrived. They unify
into one sealed bottom carrier crate, compiled feasible across four crates with no gates under `no_std`,
with the seal surviving the crate split (the attack crate is still refused by rustc's own sealed-trait
diagnostic).

Op's condition, verbatim:

> Unify. But I would still keep the semantic alias of a Capacity, even if it becomes non-bespoke in
> mechanism. However, everything following the same typestate explicitness and clever design to get all
> of our contracts expressed without forbidden features is a net win in my books.

So the mechanism unifies and **the vocabulary does not**: `Capacity` stays a named semantic alias over
the shared carrier, in the same pattern the design already uses elsewhere. A domain keeps its own word
for the thing even when the thing beneath is shared.

One consequence rides with it: the pending compile-cost bench was scoped to price facade machinery, and
after the unification it prices shared machinery every consumer pays for. Its exit condition is restated
before it runs.

## `Layout::Bitpacked` goes to a compute-side expert

The axis has been carrying two meanings across the whole corpus, unnoticed until the byte image was
examined: byte-aligned slots, where every value has an independent byte image, against zero inter-value
padding, where a single value's byte image is not a well-formed request because shared bytes occur at
every field width that is not a multiple of eight.

Op:

> This requires someone more versed in the compute side theory to confirm, but to me, the cost and
> complexity should amortize to cold alone, so option 3 seems right to me. However the description on
> option 1 is a bit ambiguous so it might already achieve this implicitly from unnamed context.

So the working reading is **two instances rather than one**, on the ground that the cost and complexity
should be confined to `Cold` rather than spread, and the dispatch that settles it is a compute-side one
rather than a type-side one. It also resolves the ambiguity op flagged: whether the two readings are
genuinely distinct storage strategies or whether one already implies the other under context the
descriptions left unnamed.

## Back to closing

Three exploring dispatches landed and each found real unexamined ground: the external images of a value
(text, bytes, digest), and the eleven-row taxonomy the review recorded as its own blind spot in its first
consolidation and never returned to in sixty-one files. Op's rhythm turns back:

> Close now.

So the next stretch closes what the exploring opened, and consolidation eight follows it.

## Standing

Only op's calls are final. The panel produces canon, not source; `mock/research/` and `mock/benches/` are
its ground and `mock/crates` is out of bounds until the canon is complete and earmarked as arvo's first
full canon. The intent outranks every instruction, is vague on purpose, and is inferred rather than read
literally.
