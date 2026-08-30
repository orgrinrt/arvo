# 72. Reply: the universal is refuted, its evidence is constructional, and the obligation it leaves is smaller than it reads

**Author lens:** Kiselyov, resumed. This is a reply to `71`'s attack on `67` rather than a new
file on a new question, and it answers from the position `67` took rather than forming a fresh one.
**Probes:** `72_probes/`, two instruments, each committed with its output as soon as it ran
(`52a471c2`, `a8e07af9`). Both are exhaustive at the model widths and neither is a bench.

**The answer to the assigned question first.** I accept `71`'s refutation without reservation. The
universal at `67:252` is false, it is false for exactly the reason `71` gives, and it is the fourth
instance of the failure `67` section 1 catalogues three of, committed in the section that proposed
the shape. The repair is below in the exact words I want it to read.

Then two things `71` did not say, one about its evidence and one about its scope, both of which
make its position stronger rather than weaker.

## 0. Gates, and what I ran rather than read

**Canon gate: passes, situation two.** No canon exists, `mock/canon/` is absent, `mock/crates/` is
empty by the mutation order. Nothing binds but op's intents, which I re-read. Nothing here settles
anything.

**Test gate: no suite exists.** The substitute is the probe discipline, and in this file it is the
work: section 2 is an application of the test gate's own tautology check to another member's
instrument, and section 3 is an application of it to my own, which failed first and is recorded
failing.

**Ran, rather than read about:** `71_probes/p1_five_crossings.py`, reproduced byte-identically
against its committed `p1_output.txt`. All eight of my numbers appear in it (192, 111, 136, 80 at
index 3; 192, 101 at index 2; 108, 1 at index 1), so `71`'s control held and this is not a
disagreement about arithmetic. I read `71_probes/p2_composite_crossings.py` in full at the source
before transcribing its construction, and my own `p2` reproduces its 30-of-256 figure as a control.

**Read end to end:** `71`, `69`, `70`, `68`. **Read at the source:** `71_probes/p1`, `71_probes/p2`,
`65:185-196`, `71:415-430`, `OPTIONS.md:991-994`, `69:66-75`. **Not read:** `71_probes/p3` through
`p8` in full; I take their reported numbers as reported and say so, and section 4's remarks about
`p8` rest on `71`'s prose rather than on its code. **Not re-run:** any instrument of `65`, `66`,
`68` or `70`.

**Nothing here is priced.** No bench ran.

## 1. The refutation, accepted, and the exact repair

`67:252` reads:

> Every crossing is total and preserves values or patterns at 100%. **No crossing preserves
> operations at 100%.**

`67` section 2 states the telescope has five coordinates and `67`'s own rule identifies a crossing
by the smallest index at which two terms disagree. Five coordinates, five classes. I measured three
and quantified over all of them. That is the defect, it is mine, and it is not a matter of framing:
the sentence is drawn from a proper subset of its own author's enumeration, which is precisely the
quantifier failure the same file catalogues at `61` on `56`, at `57b`'s `p7` twice, and at `42`'s
clamp-counting sentence.

I have nothing to say in mitigation and I do not think mitigation is the useful output. What is
useful is that the failure survived a file written specifically to name the failure, which is a
harder fact about the concept than anything the file argued, and `71` is right to spend a section on
it rather than a footnote.

**The repair, minimal, for `67:252`.** Replace `No crossing` with `None of the three`:

> Every crossing is total and preserves values or patterns at 100%. **None of the three preserves
> operations at 100%.**

**The repair I would rather have, because the structure `71` found makes a true universal
available.** The value-level operation of a term is a function of the first three coordinates and
of nothing later. Every crossing moves one coordinate. Therefore:

> **A crossing preserves the value-level operation exactly when it moves no coordinate that
> operation reads, which is to say at the encoding and at the container and nowhere earlier.**

That is a biconditional, it covers all five classes, it is true by construction rather than by
sweep, and it says more than the sentence it replaces. A consolidator should take this one and treat
the minimal repair as the fallback for the register line, which needs to be short.

**And the register line at `67:566`**, which currently reads "There are **three** crossings, not
two", should read:

> There are **five** crossings, one per telescope coordinate, identified by the smallest index at
> which the terms disagree. Each preserves values or patterns totally. The three that move a
> coordinate the value-level operation reads preserve no operation totally; the two that do not,
> preserve it necessarily.

**`67`'s K4 survives untouched** and `71` says so. Its wording is "a crossing **may** have the first
and lack the second", which is existential and is what the five-row table confirms. The universal
was a separate sentence three lines earlier and it is the one that goes.

## 2. The two rows that refute it cannot fail, and that is worth more than the number

Accepting a refutation does not mean accepting the evidential status its author assigned it. `71`'s
index-4 and index-5 VALUE rows are not measurements. They are restatements of the model's
construction, and they cannot fail.

The mechanism, from `71_probes/p1_five_crossings.py` read at the source: `System.add` is
`self.rho(self.dom_add(a, b), self.q)`, which reads neither `self.enc` nor `self.offset`. At index 4
and index 5 the source and target agree on `(dom, q, rho)` and the value map is the identity, so
`measure_value_ops` compares a computation with itself. It is the test gate's own tautology shape,
a result compared to the same computation, arrived at honestly and in a spike rather than in a
suite.

`72_probes/p1` demonstrates it rather than asserting it, by mutation. Give the index-4 target an
encoding that maps every one of the sixteen values onto the single pattern zero, so it is not
injective, not surjective, and not an encoding of anything:

```
index-4 VALUE ops, target encoding twos_complement : 256/256
index-4 VALUE ops, target encoding excess_8        : 256/256
index-4 VALUE ops, target encoding BROKEN_constant : 256/256
```

and at index 5, with the housing offset swept over 0, 1, 2 and 4, the row is 256/256 in every case.
The check does not notice, because it never calls the thing it is nominally about.

**This strengthens `71` and I want that read correctly.** Its X2 says the encoding and the container
"can never change what it computes". A 4-bit sweep would license "does not, at four bits". A row
that cannot fail licenses "never", at any width, in any language, which is what a canon sentence
needs and what the permanence test asks for. So the correction is to the citation and not to the
claim: cite those two rows as **true by construction**, and X2's "never" is earned.

The cost of not making that correction is a number in circulation. "256/256" invites a later reader
to quote a measurement, and this panel's record on quoted numbers is bad enough already.

**And there is a question at index 4 that can fail**, which is what the row should have been. The
value-level operation is not what a consumer gets from a re-encoding; what they get is the operation
the stored bits perform when added without consulting the type. Measured per encoding, per policy,
exhaustively at 4 bits, comparing `load(raw_add(store a, store b))` against the system's own add:

| window | policy | encoding | agreement |
|---|---|---|---|
| signed | wrap | natural, two's complement | 256/256 |
| signed | wrap | biased, excess-8 | 0/256 |
| signed | saturate | natural | 192/256 |
| signed | saturate | biased | 1/256 |
| unsigned | wrap | natural, identity | 256/256 |
| unsigned | wrap | biased, rotate-8 | 0/256 |
| unsigned | saturate | natural | 136/256 |
| unsigned | saturate | biased | 8/256 |

A biased encoding forfeits the raw adder totally, at every operand pair, in both sign domains. That
is `63` section 3.5's constant defect arriving as a number a consumer feels, and it is where index
4's cost actually lands. `71`'s X2 says cost is decided at the last two coordinates; this is that
sentence with a measurement under it rather than a tautology.

**One defect of my own, on the record.** The first version of that table applied the signed
two's-complement decoder to the unsigned window, which is not an encoding of that window at all, and
produced 128/256 and 36/256 rows that meant nothing. Encodings are per-window because an encoding
realises one representable set. The error, the corrected construction and the note are all in the
committed probe.

## 3. Where the endpoints stop determining the crossing, and how much smaller that region is

`71` section 4 is the finding I most wanted to attack and I could not break it. The construction is
right: a step moves exactly one coordinate, so nothing is chosen inside a step, and any
order-dependence in the composite is a fact about composition rather than an artifact. Six pairs,
one diverges, `{Q, rho}`, and the three-coordinate case collapses to two classes keyed on whether
the Q-move precedes the rho-move. My `p2` reproduces the 30-of-256 control exactly.

**The categorical restatement, offered because it is my lens and because it gives the section a
criterion rather than a choice.** The telescope's terms are objects and the steps are morphisms.
A composite of morphisms is determined by its endpoints exactly when the square commutes. `71`
measured six squares and found five commuting and one not. So "which order" is not a question about
crossings in general; it is the question of which squares commute, and the canon can state the
answer as a condition rather than as a policy.

**And the condition is narrower than section 4 states.** `71`'s obligation, as written, is
unbounded: "a canon that says 'the crossing from A to B' without naming an order has said nothing in
that case." `72_probes/p2` bounds it.

- **Widening instead of narrowing.** Narrow wrap into wide saturate, both orders, exhaustive over
  the source's sixteen values: **1 distinct function, 16 of 16 agreeing.** An exact step commutes
  with anything.
- **Narrowing, restricted to source values already inside the target's set.** **1 distinct function,
  16 of 16 agreeing.**
- **The complement.** 240 out-of-range values, 2 distinct functions, 14 agreeing.
- **Both directions of the reduction pair**, so the finding is not about saturation specifically:
  wrap into saturate and saturate into wrap both give 2 functions at 30 of 256, and the two
  same-reduction cases are single-coordinate crossings with no order to name.

`71` states the half of this it measured, that none of its divergent witnesses is a source value
inside the target's set. The measurements above are the converse direction, and the containment
holds: **every divergent value is lossy.**

**My own biconditional was refuted by my own probe and I keep it with the correction.** I predicted
the divergent set would *equal* the lossy set. It does not: 226 values diverge, 240 are lossy, and
14 lossy values agree anyway. The closed form for the exceptions, derived after seeing them and then
checked exactly in the probe, is that off-range the two orders agree precisely when the wrapped
representative lands on the bound the clamp would have picked, which is `v` congruent to `hi` above
the window and to `lo` below it. The predictor reproduces all fourteen and nothing else.

**What that changes for what op is asked.** The obligation is not "name an order for every
multi-coordinate crossing". It is **"name an order for every lossy one"**, and a crossing that loses
nothing is endpoint-determined and needs no canon sentence at all. That matters because the widening
composites are the common case in the material this panel already carries: an accumulator entry, a
promotion into a wider intermediate, and `60`'s window mechanism are all Q-growing. `71`'s three
positions for op remain exactly as it states them, with the question they answer scoped to the lossy
case.

## 4. Judgement on the structure `71` built on the telescope

I was asked for this specifically, so it is stated as judgement with the reasoning attached rather
than as a list of agreements.

**X2, meaning at prefix 3 and cost at prefix 5: correct, and it is the best sentence in the file.**
It is not an over-reading. Two independent supports it does not cite, both from `67`'s own `p3`,
which was built for a different question and therefore did not select for this: a packed run's
stride is a shared parameter at the housing coordinate and leaves the representable set constant at
one distinct set over four strides; block floating point's shared exponent moves the representable
set, eight distinct sets over eight exponents, and is therefore an index-2 difference rather than an
index-5 one. Both are cases where a reader might expect the last coordinate to change meaning, and
in both the meaning moves only where the first three do. X2 survives the two hardest cases I have.

**Section 4, the endpoints: correct, important, and now scoped.** Section 3 above.

**Section 2, the pattern relation need not be a function: accepted, and it amends my own K1.** I
wrote the telescope as a sequence of choices, which reads as a sequence of functions. `71`'s `p8`
shows the fourth component ranges over relations, since a redundant encoding gives 81 strings onto
31 values with only 2 of 16 values having a unique image. So K1's wording should say the components
range over **sets of relations**, with the function case the ordinary one. That is a real correction
to my sentence and I take it. I did not open `p8` and this rests on `71`'s prose.

**Section 5, conversion and resolution as one obligation at two arities: accepted, and the telescope
says why.** An n-ary operation whose operands sit at different terms is n crossings into a common
term plus one operation performed there. So both cases are "name a target term and a path to it",
the unary case with the target declared and the n-ary case with it derived, and section 4's choice
appears once per operand rather than once. That is `71`'s sentence with the mechanism visible.

**Section 7, the Q20 membership test: accepted.** A system is a member when it can exhibit prefix 3.
The argument that this is the right list rather than a convenient one, because it is exactly what
one system must show another for a value to cross, is the strongest thing in that section and it is
not something I had.

**Section 6, the roles: this is where I think `71` over-reads, and it is the one place.** The claim
at `71:415-417` is that storage, compute and interchange "differ from each other at telescope
indices 4 and 5 and nowhere else, on `65`'s own account of them". Two things are wrong with it.

First, the register carries a live reading under which the compute role differs at index 2.
`OPTIONS.md:991-994` records as genuinely undetermined whether `Precise` widens compute past
storage, with 64 of 251 extents mapping to two distinct carriers if it does. A compute
representation holding values the storage set does not contain is a different representable set,
which is an index-2 difference, not a realisation variant. And `65`'s own text for the compute role
at `65:188-189` says "a native-width two's complement **or a redundant intermediate**", and a
redundant intermediate is precisely a form that holds what the format cannot.

Second, `65` does not file chain extent as a fourth role separate from compute. At `65:191-192` it
says Precise's intent is about "the compute role *across a chain*, which is a role with an extent
longer than one operation". So `71`'s clean two-kind split, three realisation variants against one
schedule, runs through the middle of one of `65`'s three roles rather than between them.

**Neither point breaks section 6's contribution**, which is the criterion: whether a role may change
a coordinate the value-level operation reads is what decides whether roles are realisation variants.
What changes is the criterion's shape. `71` poses it as one question, "may a role change the
selected reduction". It is three: **may a role change the ambient domain, the representable set, or
the selected reduction.** The register already carries a live answer of "possibly" to the middle one,
which means the role set as currently proposed is mixed on the evidence the panel already holds
rather than only on a question nobody has posed.

## 5. What this does to `67`'s own candidates

Stated so a consolidator can take the corrections without re-reading `67`.

**K4 survives as written.** `71` says so and I agree; its "may" is existential.

**K2 gains a fourth instance, and the instance is mine.** K2 says every canon sentence names the
prefix it quantifies over, and rests on three recorded quantifier failures. It now rests on four, the
fourth being `67:252` itself. A claim whose supporting evidence includes its own author's violation
of it is in an unusual position, and the honest reading is that this strengthens K2 rather than
weakening its author: the failure is not a lapse of attention, since attention was exactly what was
being paid.

**K1 is amended.** The telescope's components range over sets of relations rather than sets of
functions, per `71`'s `p8`. The dependency structure is unchanged.

**A new candidate, from section 3, offered as K7.** *A composite crossing that loses nothing is
determined by its endpoints. Where it loses, it is determined by its endpoints together with an
order, and the two orders are the source's policy governing the loss and the target's. So an order
must be named exactly where a crossing is lossy, and nowhere else.* Permanence: passes. Equivalence:
passes, and sharply, since two implementations differing here compute different answers on 226 of
256 values in the measured case. Rests on `71`'s `p2` and `72_probes/p2`. ONE EXPERT on the scoping
half; the underdetermination half is `71`'s.

## 6. Register

**Kills nothing.**

**Q27's line, as `71` says, should be replaced.** My preferred replacement text is in section 1. I
would add one clause `71` does not: the table's two 256-of-256 value rows are constructional rather
than measured, so a reader must not cite them as a width-bounded result.

**A new option, written out in full: when must a crossing name an order?** **Always, for every
crossing whose terms differ at more than one coordinate.** Cost: a consumer writing a widening
promotion, which loses nothing and is endpoint-determined, still writes an order they cannot get
wrong, which is ceremony with no content. **Only where the crossing is lossy.** Cost: whether a
crossing is lossy is a property of the operand as well as the terms, so the rule is stated on terms
(does the target's representable set contain the source's) and is conservative on the operand.
Buys: the widening composites the panel already relies on need no sentence. **Never, with the
canon fixing one order globally.** Cost: the fixed choice is wrong for one of the two readings at
every lossy site, and section 4's three positions become one with no escape. **What would
distinguish them:** whether any consumer writes a lossy crossing at all without an explicit
narrowing already in view, and whether the containment measured here survives past the model width.

## 7. The two questions in flight

Neither is answered here and both branches are carried.

**Which verb "validate" is.** Nothing in this file moves under either reading. Sections 1 through 3
are arithmetic about which coordinates an operation reads, and the answer is the same whether
membership is checked at compile time per type or at runtime per datum. The one place it touches:
if the runtime ingest door exists, then an ingest is a crossing whose source term was never
witnessed, and section 3's scoping does not apply to it, because losslessness cannot be decided from
a term nobody has. So the ingest case needs the door under both readings and needs it for a reason
section 3 supplies.

**Whether the long-standing constraints are op's intents.** Nothing in this file rests on them. Both
probes are arithmetic over integers and would hold in any language under any dispatch discipline.
`67`'s own erasure claims do rest on them and `67:48-59` said so; that exposure is unchanged and
unaddressed by this reply.

## 8. What I could not settle

**Why 226 and not 240.** The closed form for the fourteen exceptions is checked exactly and I did
not derive it before seeing them, which is a weaker epistemic position than a prediction that held,
and I mark it as such.

**Whether the containment survives past the model width.** `p2` is one pair of widths. The
containment direction is an argument as well as a measurement (an exact step commutes), so I expect
it to hold generally, and I have not proved it and did not build the width sweep.

**`71`'s `p8` at the source.** I accepted the section-and-retraction finding and the K1 amendment on
`71`'s prose without opening the probe. That is the one dependency in this file I would most want a
later reader to check rather than inherit.

**Whether "crossing" is the right word at all**, given `OPTIONS.md` Q9 already uses it for the width
surface. `71` raises it, I have no better candidate, and I decline to coin one for the same reason
`67` declined to name the index-3 crossing.

**Nothing here settles anything.** The mode is explore. Section 2's evidential correction and
section 4's objection to the role split are what I would most want the unit's remaining files to
attack.
