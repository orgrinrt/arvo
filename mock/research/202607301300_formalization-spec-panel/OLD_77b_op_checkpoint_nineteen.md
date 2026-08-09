# Op's nineteenth checkpoint: the pricing pillar named, and capacity reopened as a question

**Date:** 2026-08-04
**Position:** after `77_ringer_the_pillars_recovered.md`, immediately before consolidation eight.

## The pricing pillar becomes the fourth design rule

Op reasserted it, and the wording is his:

> Compile time is nothing. That can be literal minutes for all we care (although we'd want to optimize
> it too, but only secondarily), the important measurement is the actual runtime and lowered code. So
> benching compile times is just noise if it wins us real perf and efficiency on runtime. This should be
> stated clearly and absolutely: We *want* long compile times, if it resolves to snappy optimal runtime
> with the extra soundness, safety and numeric machinery amortized fully at compile.

And the sharper claim, which does not follow from the general workspace rule alone and therefore gets its
own spec sentence:

> it's always amortize runtime cost in compile, const time, absolutely always, no matter the strategy
> (strategy just means we still do full amortization on type level or const at build time, but the
> runtime is different than the other strategies, so that causes runtime cost, NEVER do any strategy
> defer the cost to runtime that it can avoid!)

**A strategy marker changes what happens at runtime. It never changes how much is amortised at compile
or const time.** All four presets verify to the same depth; they differ in what they then emit.

**Adopted as the fourth design rule**, beside the spine rule, carrier-at-birth and layer-keying, in the
consolidation's rules section, which is the part of the base document every expert demonstrably reads and
acts on.

### What the audit found, stated because op was right to check

The pillar has **not** been violated in substance. All 24 citations of `arvo-compile-time-last.md` across
the corpus argue in the licensing direction rather than the restricting one, and the one place compile
cost touched a design fork got the ordering explicitly right, closing on the guarantee before consulting
cost at all.

What decayed is wording. The rule's own "common misreading" clause, the part that names and forbids
exactly op's fear, is quoted nowhere in 77 files. Meanwhile the consolidation's vocabulary for that fork
("gated on the bench", "becomes the gate", eleven occurrences) never states in the same breath that the
gate only separates routes already tied on correctness. A reader of the base document alone would
reasonably conclude compile time was deciding the design. And the pillar was never named among the
review's own design rules despite being cited more often than any of the three before they were named.

Every other pillar checked out intact. The one that had decayed before, design-the-shape-not-the-code,
was caught and repaired by the review itself at file 69.

## `Layout::Bitpacked` has one meaning, with one follow-up

**Ratified: one meaning, zero inter-value padding.** Op had leaned toward two instances so the cost would
confine to `Cold`; the compute-side dispatch replaced that rather than confirming it, and op's own second
sentence turned out to be the right one. The byte-aligned reading is not a second bitpacked instance: it
is what `Layout::Dense` already does at a narrow stored width, which the ratified table assigns to `Hot`.
The confinement op wanted is achieved by the axes already in hand, and `Cold`'s own intent forecloses
byte-rounding from bitpacked's definition.

**Follow-up op asked for:** the measured price, 4.6x to 5.5x sequential and about 2.2x random against
dense native, gets a second look to establish whether that multiple is inherent to bitpacking or an
artifact of the access pattern measured.

## The facade fork closes on soundness

**Ratified.** All three expressions of the guarantee under the alternative route die to compiler
diagnostics, and one of them passes `cargo check` silently while failing at link, which would have
re-opened the identity defect the review spent a stretch on. The fork closed on correctness before cost
was consulted, which is the pricing pillar working rather than being overridden.

## Capacity: the unification is reopened, as a better question than the one that was asked

The dispatcher's framing of this was wrong twice and op caught it. Stated correctly:

`Capacity` as it exists **is** the antidote to the forbidden feature, exactly as op built it, and nothing
found here changes that. What file 76 established is about the proposed **unification**, and the two
type-level naturals differ in kind rather than spelling. `Capacity`'s is a type whose parameter is an
array-length const, per op's own migration ("the capacity is a TYPE ... so no `cap_size` expression sits
in type position"). The tower's is inductive and value-unique, `Nat ::= Z | Pz<P>`, `Pos ::= H | O<P> |
I<P>`: a number *is* a recursive type. Unifying them means replacing every capacity with a recursive type
and then asking that recursive type for an array length, which puts a const operation back in type
position and rustc names the forbidden feature in its help text. The sanctioned successor cannot express
`2 * P::VAL`, the entire content of a binary inductive natural. So the unification as spelled would have
undone op's own antidote, and the feasibility probe that cleared it never saw this because it declared
the capacity trait as a bare const with no associated array type.

Op then reframed the whole thing, and his reframing is better than the narrowing that was on offer:

> Capacity simply denotes a fixed length. It's not a numeral itself. But it contains a numeral that
> expresses this length. And it's used in collections and sets. Which means it is also the same as
> infinity on infinite number sets, and the lastmost number in finite sets. Which means conceptually
> "Capacity" already must exist. We just haven't wired it up to concrete collection usage. Does it map
> directly, or does Capacity become an alias to whatever expresses that length in theory side for us?

**This is a dispatch, not a call, and it goes first in the coming four.** The lead worth naming for it:
the far-point rule ratified at `74b` says the far point is the supremum of a numeral's ordered
representable values, with the infinity case and the largest-finite case as two instances of one
statement. Op's reading of capacity, the lastmost element of a finite set and infinity on an infinite
one, is that same statement applied to a collection's index set rather than to a numeral's value set. If
that holds, capacity is not a second concept needing a carrier; it is the supremum notion the design
already has, wired to a different set. Whether it maps directly or becomes an alias over the theory-side
expression is what the dispatch derives, with the two independent reads any design call requires.

## Order of work

Op: consolidate, then four on the remaining open gaps before widening to exploration again, keeping the
four-checkpoint-four-checkpoint-consolidate rhythm. The open list, not the interesting list, is the work
queue for the coming stretch.

## Standing

Only op's calls are final. The panel produces canon, not source; `mock/research/` and `mock/benches/` are
its ground and `mock/crates` is out of bounds until the canon is complete and earmarked as arvo's first
full canon. The intent outranks every instruction, is vague on purpose, and is inferred rather than read
literally.
