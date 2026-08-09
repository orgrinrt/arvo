# Op's thirtieth checkpoint: no enumerations, Warm's crossover is wrong, and prior calls go stale

**Date:** 2026-08-07
**Position:** after `137_aaltonen_erasure_without_a_condition.md`, while `138` is in flight.
**Required reading with the standing base.** One call here declares earlier calls stale, so nothing should
build on those until the re-evaluation lands.

Op is present and every call here is his own.

## The erasure gate is met, and how it was met matters

`135b` set the gate: the consumer expresses usage in bits and bytes, the typestate derives the matching
container and numeral representation, it validates, and it erases on lowering, all four at once with no
caveats. File `137` closed it.

The condition the panel had been carrying was never the condition. `132` said the operation body must name a
machine type; measured over six bodies on one payload, that is neither necessary nor sufficient. A body
generic over `L: Limb, const N: usize` with no machine type in its text is two instructions and LLVM folded
it with the native add. A byte array with no carry chain at all is still thirty-one against two. **The real
condition is that the payload be one limb of a register-width type, which is a derived property rather than a
discipline an author observes**, which is what op said it would be at `135b:65-68`.

The seam itself was the broken premise: step B was carried by four files as "rung to machine type", and above
128 bits there is no machine type. The wide payload hole is now closed with a `#[repr(C)]` word cons, the
whole ladder derives gate-free with no width enumerated, and none of the thirty-three-times figure is
inherent: at 256 bits, hand-written eleven instructions against `[u64; W]` eleven, byte-identical.

## No enumerations, and this is the same finding a seventh time

One enumeration remained: the bridge from a written literal to a type, where a consumer wanting a width arvo
did not ship adds one `impl ToNat<Mine> for Idx<7>` line. It was offered as acceptable. **Refused.**

> Hmm. This really looks like just another instance of the spelling out being the problem, all the
> heuristics should be there. If I understand correctly, that is. It should come implicitly from the heavy
> typestate. No enumerations, if we can help it; and I think we have much to explore to actually be able to
> help it.

The panel has now offered op an enumeration three times, in three different places, and he has refused it
three times, each time on the same ground: the information is present in the typestate and what is missing is
the spelling. He has been right on that ground every time it has been tested. **The instruction is to
explore, not to price the enumeration better.**

Note what this rules out along with it. Buying `generic_const_args` plus `-Znext-solver=globally` to remove
the line is not the alternative he is asking for either; that trades an enumeration for a feature and a flag,
and his sentence is about the derivation being implicit rather than about which mechanism pays for it.

## The wide payload becomes a strategy consequence

**Adopted.** Above the native rungs a wide payload is **ragged** for `Cold` and `Precise`, sized to the exact
bits, and **word-rounded** to whole 64-bit limbs for `Hot` and `Warm`. Measured at one numeral: ragged is
fourteen instructions and twenty-five bytes, word-rounded is eleven and thirty-two. Three instructions per
operation against seven bytes per value is exactly the trade the strategy axis exists to carry, so nothing new
is invented and the axis absorbs it.

## Warm's crossover at 65 bits is wrong, and the reason is precise

`137` found that `Warm` crosses into the wide multi-limb rung at **65 bits, not 129**, because Warm takes a
rung of headroom and that headroom is what carries its wrapping semantics. Op:

> This feels unintuitive to me. But if user needs more than 64 bits, that's just a consequence then. But if
> they declare less than 64, and we fit it implicitly into a container that is less efficient, that feels
> wrong too. Surely there's a sensible answer here. The problem is Warm only, and only because it is the
> default. The described behaviour is okay if and when *expected*.
>
> Just using arvo without using the typestate at all, downstream, like just using the algos that then
> implicitly work on bare primitives for downstream callers via the typestate and rust type inference, it
> will be unexpected that the algo runs slower than they want because it widens something that should fit a
> native container.

Three things follow, and the third is the one the panel would have missed.

**A declared width above 64 bits going multi-limb is a consequence, not a defect.** That case is settled.

**A declared width at or below 64 bits landing in a less efficient container is wrong.** The headroom rule
currently produces exactly that, and it produces it on the default strategy, which is where it is least
expected.

**The failure mode is a consumer who never touches the typestate at all.** Someone reaching for arvo's
algorithms, working on bare primitives, with the typestate and Rust's inference filling in the rest, gets a
slower algorithm because something that fits a native container was widened. That consumer has no way to see
why and no reason to look. **The behaviour is acceptable exactly when it is expected**, and it is
unexpected precisely in the case arvo most wants to serve well.

So Warm's headroom rule is reopened. What replaces it has to keep Warm's wrapping semantics and stop widening
a numeral that fits.

## The prior calls that shifted underneath, and are now stale

Two defects were reported: `110:1440`'s claim that the three literature relations are three points of the view
lattice, refuted by the design's own division chapter, which survived because a probe's model made the
invariant true rather than testing it; and `110:1473` calling `AddClosed` shipped when `AddClosed`, `Bias` and
`Adjustment` return zero hits across `mock/crates/`. Op:

> Hmm. It feels like the theory and spec has shifted under my previous calls you mention, which may warrant
> declaring them stale and re-evaluate.

**So the affected prior calls are declared stale rather than patched**, under his own principle
(`108b:11-20`): a ratification is made under the evidence available at the time, and where the grounds have
visibly moved it is re-derived rather than cited. This is the second time in three checkpoints he has retired
one of his own calls on those grounds, after the canonicity withdrawal at `130b:11-30`.

The re-evaluation is owed and is not a patch to two sentences. It has to establish which calls rested on the
refuted lattice claim and on the false status cell, what each of them would say now, and whether the deeper
fix `136` identified applies: that the grade does not determine definedness and the order does not contain
it, so definedness may want to be a third view axis rather than a sentence to correct.

## Standing

Only op's calls are final and they go stale when their evidence moves, which this checkpoint demonstrates
twice. The panel produces canon, not source; `mock/research/` and `mock/benches/` are its ground and
`mock/crates` is out of bounds until the canon is complete and earmarked as arvo's first full canon. Experts
are dispatched one at a time, never in parallel, each reading the ones before it, and each writes its file
incrementally because five dispatches in this panel have died mid-flight and only those that had saved a
partial lost nothing.

The consolidation is promoted to canon whole and supersedes everything before it, so comprehensiveness is a
requirement rather than a virtue, and the erasure gate at `135b` is part of what it is measured against.
