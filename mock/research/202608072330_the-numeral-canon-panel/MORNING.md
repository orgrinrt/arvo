# The map, for the morning

Accreted through the night of 2026-08-07 as each file lands. **Nothing here is settled**, per `04`.
This is angles opened, routes closed, and questions sharpened, presented for op to think against.

Read this first; it points at the files. Each entry says what changed and what it cost to find out.

## The single biggest shift

**The panel may have been answering a question with no caller.**

`03` noticed in passing that the multiply's result numeral is a **formula over the operands**, not a
lattice join: the product sits at the sum of the widths, the join sits at the pairwise maxima, and
arithmetic calls the first. `06` was dispatched to test that head-on and enumerated **twenty sites**
where the design produces a numeral the consumer did not write.

It replaced the carve it was given. The useful axis is not formula-versus-extremum, because **an
extremum here is always a formula**; coordinatewise maximum is a total function of four declared
numbers. The axis that works is *who determines the answer*:

| | | count |
|---|---|---|
| **D0** | the consumer determines it | 8 sites |
| **D1** | the operands determine it | 8 sites |
| **D2** | a named rule determines it | 3 sites |
| **D3** | nothing determines it | **0 sites** |

**D3 is empty, and `06` argues it cannot be inhabited.** An associated type names exactly one type.
Declaring both honest readings of a cross-family join gives `E0119: conflicting implementations`, so
a design that infers a cross-family target is **picking in one impl**, not computing a least upper
bound, whatever the order says. Its arm 2 compiles, so the type system accepts any cross-family
formula and holds no opinion on which is right.

**What the order is still for.** Two genuine callers, both using it as a **two-place relation**
rather than for its extrema: the lossless-conversion predicate, which fires at every D0 site, and the
accumulator sufficiency check. Four further uses are readers rather than callers: justifying each D1
formula, stating tightness, the value-set quotient that makes "the result numeral" well defined, and
the diagnostic. `06`'s sentence for it: **the order is infrastructure; the lattice is a theorem about
it.**

## Op's own question, and what happened to it

You asked for the consequences of one-family versus several before ruling. `03` delivered them, and
in doing so found that **one of the three readings is not what it appeared to be**.

The promise attached to reading A holds "within one radix, zero bias, and a closed shape space".
The option as put to you carried only the third condition. Drop either of the other two and it breaks
structurally: bias produces 670 of 1326 pairs with disjoint value sets, needing an empty numeral that
the option never names; radix produces 60 structurally unreachable cross-radix joins. And across
kinds, A's admissions still do not close the join, because the exact union of two particular numerals
needs seven values and seven is not a power of the radix, so no uniform shape sits between. `03`'s
verdict: **A appears to be C wearing A's clothes.**

`06` then reframed the question operationally: not whether the numerals form a lattice, but **whether
the admitted shape space is closed under the clamp of every formula's answer**. Measured, it is not,
at 15 of 6561 pairs, and those are exactly the pairs needing negative integer width.

## Routes closed, with what closed them

- **D3, a site where nothing determines the numeral.** Closed by `E0119` on the two honest readings
  of a cross-family join. Coherence, not order theory, is what forbids it.
- **Reading A as stated in `01`.** Closed as stated, because its promise needs three conditions and
  it was offered with one.
- **Changing the order instead.** `03` closed refinement-alone and reach-alone (both lattices that
  say nothing an operation needs) and inclusion-up-to-rounding (fails antisymmetry).
- **The apparent conflict between `03` and the predecessor's `150`.** Not a conflict. They ask
  different questions and both answers are true. See `05`.

## Corrections between files, recorded so neither propagates

- `03`'s option H cites a passage for the multiply being a formula. `06` checked it: the passage
  states the map's shape and **nothing about its content**, and the sum-of-widths arithmetic is
  `03`'s own, unmarked. The conclusion survives on other grounds; the citation does not support it.
- `03` locates negative integer width only at the meet and infers that reading A's admissions are
  pure cost if the meet has no caller. `06` found **negative width does have a caller** elsewhere,
  at 15 residual pairs, so the inference does not follow. Honest size: a shrinking corner, 7 of 625,
  11 of 2401, 15 of 6561.

## A finding nobody asked for, and it would have made a canon sentence false

**The sum-of-widths product form is not tight.** Tight at 6100 of 6561, wasting exactly one bit on
461, with the wasteful region characterised exactly: 160 where an operand denotes only zero, 301
where the narrower operand's total width is one. The mechanism is that a numeral's reach is just
below its ceiling, and the formula prices each operand at the ceiling.

So **a canon sentence claiming the derived numeral is the tightest honest answer would be false.**
`06` derived the tight form, found its predicate reduces to a one-line condition, and built it
gate-free with a negative control that fails at `E0080`, so the assertions are load-bearing rather
than decorative.

## The two cheapest questions waiting for you

Both are one sentence, and each collapses a large part of the map.

**Does the design have a mixed-numeral addition at all?** `06` found no operation anywhere in the
record that adds values from two different numerals. If it does not, addition joins the
consumer-determined block and **the entire inference surface is multiplication plus the container**.

**What does the top of a saturating numeral denote?** `07` found that saturation's soundness is an
**unstated concretisation choice**, and that the two readings are not close. Read the top as a point,
and saturating is exactly as unsound as wrapping, 512 of 1024 at one size. Read it as absorbing
everything above, and it is sound at zero failures for every size tried while wrapping reaches 55,085
of 65,536. **Identical arithmetic, opposite verdicts.** The design's own algorithm crates already
behave as though this were answered.

## The frame that turned out to explain the other two files

`07` asked whether the exact-to-representable pair is an adjunction, a framing the predecessor's 320
files never once used. Verified: zero hits across the whole tree for the vocabulary, one near-miss
citing it as external prior art. The ground was new.

**There are two adjunctions, not one, and the panel has been holding them as a single object.** One
at the fibre (rounding into a fixed numeral, against the embedding) and one at the index (which
numeral, given a set of exact values). The consequence that matters:

**`06`'s D0/D1 split is the fibre/index line falling out of the mathematics**, rather than a taxonomy
imposed on the sites. Two experts arrived at the same boundary from opposite directions, one by
enumerating twenty sites and one by setting up the adjunction, neither having derived it from the
other.

The index level holds, and its condition is meet-preservation, which the record already carries. The
fibre level holds against the embedding **only for round-toward-positive-infinity, and only in
range**: zero failures over 34,976 pairs, 184 once out-of-range values are admitted.

**The distinction `06` needed corrected simply dissolves.** The join and the product numeral are the
same function at different arguments, so formula-versus-lattice was never a real separation.

**And `06`'s tight product form now has independent corroboration.** Computing the least containing
numeral from the value set directly agrees with it at 400 of 400 operand pairs, by a derivation that
never uses `06`'s inequality. That is two instances arrived at differently, which is the bar.

### What the frame predicts that the panel did not have

- **The fold's sufficiency check is a single diagonal**, and the range half needs no bound at all.
  Zero unsound sequences on and above it, nonzero strictly below. It compiles as an ordinary bound,
  gate-free, **refuses at type-check rather than at monomorphisation**, and erases to the same ten
  instructions as the unguarded fold, with both guarded call sites folded onto one symbol.
- **A refined composition law**, arrived at after the expert's own first prediction over-fired: a
  rounding mode composes across nested grids exactly when its direction switches only at points of
  the coarser grid. Four pivots on the grid, zero failures; four off it, seven each.
- **The cross-kind case is priced rather than closed**: completing it settles at a 16 to 34 percent
  enlargement, every added shape a segmented grid that neither family names.

### The cost, and the recommendation

**The vocabulary, and nothing else.** Section 5.1 of `07` restates every result without it. Its own
recommendation is to keep the frame in the audit trail and out of the canon, with one qualification:
the soundness-versus-bestness pair is hard to state crisply without it. It flags that risk against
the record's own precedent, the finest-view mechanism whose literature relation was refuted and never
repaired.

Neighbouring framings were mapped rather than one being exhausted. Closure operator: same content,
reads better for a canon. Monad and comonad: **degenerate**, because posets make every coherence
automatic, and saying so forecloses a direction rather than opening one. Congruence-times-interval
reduced product: flagged as the most promising lead and explicitly marked unverified against the
literature's own definitions.

## The founding premise, tested from outside, and what it turned out to hide

`08` took the panel's own founding sentence, that the primitives are named compositions over **one
format concept**, and asked what that concept excludes. Twenty-one representations classified
mechanically against one test.

**The premise survives.** Eighteen of the twenty-one are inside. The three axes that carve the space
are the value set, the encoding, and the locus, and they are independent.

**The boundary, in one sentence a canon could carry:**

> A representation is a numeral when a datum denotes one rational, when the denotable magnitudes in
> each binade of some admitted radix form one arithmetic progression at one phase whose step is that
> radix to some power, and when the set is fixed by the type alone.

Everything outside fails exactly one of those three clauses, and **which clause it fails names the
layer it belongs to instead**, which is a better outcome than a list. Logarithmic and rational-slash
formats and complex radices fail the first; unevaluated sums fail the second; block floating point
and frame-of-reference column encodings fail the locus clause; intervals and stochastic streams fail
the denotation clause.

### The result nobody predicted, and it resolves `03`'s cross-kind failure by another route

**The design's named shapes are meet-closed and not join-closed.**

Gradual underflow **is** the meet of a fixed-point format and a float. Measured twice, once by
enumeration and once by the trait solver in a diagnostic. The join is the mirror shape, and **it has
no name**, which is the whole of `03`'s cross-kind join failure.

And `03`'s own witness pair does have a join in the general class. It is `{0, 1/2, 1, 2, 3}`, sitting
strictly inside both of the minimal upper bounds `03` named. So the antichain `03` measured is an
artifact of the **named** shapes rather than of the concept.

This also bounds `07`'s completion result: the Moore closure buys the glue shapes and **not** the
tapers, because intersection takes the pointwise maximum while posits need a steeper slope.

### Two further things the survey turned up

**The panel has been using a two-point sample of its own concept.** The exponent form has two
instances where the concept has a function space, and a ratified naming call renamed the literature's
term to one that appears in no axis table.

**Posits are inside the concept and outside every named shape.** All three configurations classify
inside, with a canonical exponent of no named form. Separately, five representations differ from a
plain fixed-point numeral in **no value-set respect at all**: residue systems, thermometer,
carry-save, negabinary, and mixed radix. Whether those are worth distinguishing is an encoding
question, not a value question.

**The probe three predecessors said was owed is now written**: a segmented format expressed in the
typestate, gate-free, refusing at type check, erasing to one symbol whose body is a bare return.

`08` also flags, unprompted, that most of its agreement with `03`, `06` and `07` is **inherited
rather than independent**, which is the distinction this panel's provenance ladder turns on.

## Method notes worth your attention

Both experts kept a broken instrument rather than deleting it. `03`'s first reported zero
disagreements through setup that helped; `06` killed two for cost and kept them as controls. Every
number above was produced by a command recorded in the probe directories.

**Everything is unpriced.** No bench harness run bears on any of it, and no file claims otherwise.

## What is still open, and what is running

Open and named: whether mixed-numeral addition exists; whether the shape space should be closed
under formula clamps; the six items in `06` section 10; the family question itself, now reframed.

Running: `07`, on whether the exact-to-representable pair is an adjunction, a framing the panel has
never used, and what it buys or costs if it is.
