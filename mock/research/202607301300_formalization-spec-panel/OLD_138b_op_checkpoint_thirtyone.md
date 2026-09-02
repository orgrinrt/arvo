# Op's thirty-first checkpoint: the aliases are aliases, and three derivations stop short

**Date:** 2026-08-07
**Position:** after `138_knuth_the_families_and_their_laws.md`, while `139` is in flight.
**Required reading with the standing base.**

Op is present and every call here is his own.

## The aliases are aliases, and this is the thing that must stop slipping

`138` established that the design has no four families. `UFixed` and `IFixed` differ only in `Domain`,
`FastFloat` differs from `UFixed` only in `ExponentForm`, `Decimal` differs from `FastFloat` only in
`Radix`. They are points in a product of four axes, two of the supposed four are fixed point, and there is a
fifth numeral, the decimal `Implicit` one, that the family framing has no slot for and that op's conformance
claim is actually about.

Op adopted the axis presentation, and then named the cause of the confusion, which is the part worth
carrying:

> I think what confused the panel (they already reached this very same thing and realization before...), is
> that I mandated we keep the aliases, but they remain convenience aliases to the underlying one
> representation they all are. So let's explicitly mention that a lot of the existing "families" and such
> are kept usually only as aliases to the real theory and numeral representation. We've already established
> this, I believe it is written down too, somehow it just doesn't stick. So find a way to make it stick.

Three things follow.

**There is one representation.** The named types are convenience aliases over it. The aliases were mandated
by op and they stay; what is wrong is treating them as the subject rather than as spellings.

**The panel has reached this before and lost it.** That is the actual defect. A fact that has been
established, written down, and then re-derived from scratch by a later member is not a knowledge gap, it is
a presentation failure in the document that was supposed to carry it. The instruction is not to state it
again. It is to **find a form that makes it stick**, which is a question about how the canon presents the
axes and the aliases, not about the design itself.

**So the canon states the product of axes as the subject**, and names the aliases as aliases at the point
where each is introduced, rather than opening with a family list that a reader will take for the taxonomy.
Whoever writes that section owes an explanation of why a reader keeps forming the wrong model from the old
presentation, because that is the thing being fixed.

Separately, and it bears on how much of `130`'s section 10 can be trusted: **it cites five probe files by
name and none of them exists anywhere in the repository.** There is no `130_probes/`. Its claim of
"compiled, gate-free" could not be checked, only redone, which `138` did.

## The stored width derives, and the overshoot is not a limitation to accept

`138` established the stored width is derivable exactly as `sign + ceil(log2(R^(P-h) * span))`, checked
against every binary and decimal interchange format IEEE 754-2019 names plus bfloat16, E4M3 and E5M2,
eleven of eleven with zero failures. The per-field derivation everyone reaches for overshoots at decimal32
and decimal64 while being exact at decimal128 and at every binary format, so a derivation checked at binary
alone reports itself total and is wrong.

Op ruled: derive it, and **do not accept the overshoot as a limitation**.

> I say the intent is correct, but I doubt it's a fundamental problem we can't solve. There's simply
> something we need to make the derivation accurate and not overshoot, so all we need is name that, thread
> it into the typestate so it reaches us there, and use that to make it accurate and ideal.

That two formats and only two overshoot is a clue rather than an irregularity: something distinguishes
decimal32 and decimal64 from decimal128 and from binary, and naming it is most of the fix.

## The conformance claim, and a correction to how it was put to op

`138` checked the claim at `110:2379-2381` against the standard. "Delivers IEEE's values" is correct, and
the theorem supporting it is absent from the document: no operation's delivered value depends on which
cohort member an operand is, over 1,280,000 comparisons with zero divergences, with `quantize` and
`roundToIntegralExact` as the exact complement the design had already carved out. "Strictly stronger than
the standard" reaches the right conclusion for the wrong reason and holds only for multiplication, since
addition does not widen. The clause about this being unavailable to a language with only runtime decimals is
false.

**Op's response was to correct the question rather than answer it**, and he was right to.

> This doesn't seem obvious to me. What you present does not imply the answers as options you gave; it
> sounds fine and intended, no? So it's only semantics? In prose? Fix that to be accurate. If there's a real
> typestate expression here we want to extract though, experts want to take a peek at it.

The dispatching agent had put a prose-accuracy repair to him as a three-way design decision. It is not one.
**The prose gets fixed to be accurate**, without a ruling. What does need an expert is whether the cohort
theorem and the multiplication-only qualification are merely things to say, or whether either is a property
the typestate should carry and check.

Op also stated why the panel exists, and it belongs in the record because it governs how questions should
reach him:

> As said, maths aren't my domain, I'm a software engineer and architect, which is also the reason this
> panel exists in the first place. I need their insight and knowhow, it's not for fun. I give my calls based
> on what I'm given, explained, and sometimes I simply can't see the framing like now, and requires people
> with understanding for the nuance to talk it through and reveal a better way to spell it out to me, or to
> the solver, or just rethinking and spitball in general.

So a question put to op must be a question he is positioned to answer. A mathematical fact dressed as a
preference wastes the one thing the panel is for.

## Three derivations that stop short, dispatched as one problem

The width bridge, the stored-width overshoot, and Warm's headroom rule are three places where a derivation
stops one step short of what the typestate already holds, and the design compensates by enumerating a table,
declaring a member, or applying a blunt rule. **Op has said a version of the same sentence about all three**,
and nobody had treated them as one question. `139` is dispatched on exactly that, with the hypothesis stated
as a hypothesis to test rather than a premise.

His instinct on this specific point has now been tested six times in this panel and has held six times, each
time because the panel stopped at the first well-supported answer.

## Sequencing

**Both open questions before the consolidation**, on op's call. The container story is the most-revised part
of the design and a consolidation written before the enumeration and the Warm rule settle would land stale on
its most load-bearing section.

## Standing

Only op's calls are final and they go stale when their evidence moves. The panel produces canon, not source;
`mock/research/` and `mock/benches/` are its ground and `mock/crates` is out of bounds until the canon is
complete and earmarked as arvo's first full canon. Experts are dispatched one at a time, never in parallel,
each reading the ones before it, and each writes its file incrementally.

The consolidation is promoted to canon whole and supersedes everything before it, so comprehensiveness is a
requirement rather than a virtue, and the erasure gate at `135b` is part of what it is measured against.
