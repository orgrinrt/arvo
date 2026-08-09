# The live option register

**Opened 2026-08-08**, when op extended the explore-do-not-settle mode from one night to roughly the
panel's first hundred files and named the method it runs on. This file is that method's instrument.

**Rebuilt 2026-08-08**, from the member files `02` through `27` directly, after the first pass was
found to have been assembled from `MORNING.md`, a compression every checkpoint that examined it found
defective (rung inflation, a doubled count, a claim spliced across two source commits, a false "exactly
when", 295 numeric tokens against zero line-level anchors while the sources carried 297; see `21`'s
citation audit and `19`'s and `23`'s checkpoints). This rebuild reads only the member files and cites
only them; `MORNING.md` and the prior version of this file were used, where used at all, to locate which
files to open, never as a source for what an option is. The rebuild's own coverage and known gaps are
recorded in `30_willsey_rebuilding_the_option_register.md`.

`SETTLED.md` holds what the predecessor established. `DROPLIST.md` holds what closed and why.
**Neither holds what is open**, and until this file existed the instruction to carry every option
forward had no artifact behind it, which meant options lived in whichever file last mentioned them
and were rediscovered rather than carried.

## What this file is, and what it is not

A register of **live options**: shapes the design could take, each written down in full, each still
admissible. It is a working surface, not a record of decisions, and it has no authority of any kind.
Nothing here is settled, ranked, or preferred, and a member citing an entry as support for anything
has misused it.

**Every option is written out in full, every time.** Not by number, not by reference to the file that
proposed it. An option referenced by number is worthless, and this project has already lost a settled
answer of op's permanently because its options lived only in a tool call.

**An option set is never a boundary.** Op, on the denotation question: "don't even restrict the panel
to these three. Free reign to converge by theory and logic to the best one that serves all other
parts of arvo best." A member finding a shape nobody wrote down adds it here, and that is the most
valuable single act available under this mode.

## How a member uses it

**On arriving:** read the questions your work touches and the options under each. Your file is
evaluated against every live option, not against the one your predecessor happened to favour.

**On finding something:** say which options your finding **fits well**, which it **fits badly**, and
which it **kills**. All three are results. Fitting badly is not the same as killing, and the
difference is whether the option survives the finding at some cost or does not survive it at all.

**On killing one:** move it to `DROPLIST.md` with the diagnostic that closed it and with what would
have to be overturned to reopen it, then strike it here with a pointer. The option space is meant to
shrink from the bottom the whole way through, while nothing is chosen at the top.

**On adding one:** write it out in full, name the file that proposed it, and state what would
distinguish it from its neighbours. An option nobody can tell apart from another option is one
option with two names.

**The selection criterion, when the time comes, is op's**: the shape that serves **all other parts of
arvo best**, not the shape that is best in isolation. So an entry's value is partly a fact about the
other entries, which is exactly why they all have to be visible at once.

## Q1. What does "then validate" require?

**Op's answer is all three**, which makes this a conjunction rather than a choice (`28`, batch one,
Q1). Recorded here because the three parts remain separable, each owes a different instrument, and he
named a challenge route: a member may argue a part is **truly not worth it**, and that phrase is the
bar.

**Admissibility.** The typestate refuses declarations it cannot serve, in both directions: it does
not admit a declaration it cannot honour, and it does not refuse one it could. Instrument owed: a
two-directional sweep. Panel evidence: none (`17` clause C3's own count: fifteen expected-to-fail
probes exist across the panel and not one is about whether a *declaration* is admissible). Over-refusal
is the direction with no natural author, since a consumer who never wrote the shape never files a bug
(`17` clause C3).

**Usage.** The typestate refuses operations that violate the declared invariants, with a diagnostic a
consumer can read. Instrument: a diagnostic battery. Panel evidence: real and substantial, and all
fifteen expected-to-fail probes in the panel are of this kind (`17` clause C3, `12_probes/p11`,
`12_probes/p12`, `15_probes/q10`).

**Self-validation.** The derived container actually holds the declared range, checked at derivation
time rather than assumed. Instrument owed: a range assertion per declaration. Panel evidence:
incidental.

Additional structure from `17`: the criterion's own phrase "no caveats left" makes it a claim about the
absence of a residue, which no instrument can close; the honest form is an enumerated, explicit trusted
base rather than a verdict (`17` section 1, section 6, nine items). And clause four of the same
criterion (erasure) decomposes into three parts with three different standards of proof: layout erasure
holds by construction (`repr(transparent)` is a language guarantee), dispatch erasure holds by
construction conditional on the ban on `dyn`/`TypeId`/unrestricted `specialization` holding, and only
operation erasure has actually been measured, at one program, one arity, one optimisation-level band,
by an oracle later found to have two false-negative regimes (`17` sections 2, 4; see the container
derivation entries below).

## Q2. Which coordinates does a consumer write?

**Total and fraction.** The pair the machinery wants. Integer width is the only coordinate that goes
negative, so keying here makes the whole negative-width corner naturals with no signed ladder.
Repeated squaring drives integer width to minus thirty-one in five steps while total width stays at
one: unbounded in one coordinate system and constant in this one (`06` section 7.2; `15` sections 1.2,
1.3, compiled over the whole 81-shape box with a negative control; `16` sections 10.1, corroborating).

**Integer and fraction.** The familiar pair, kept at the surface, converted at the door at zero cost.
The price is that the numbers typed and the numbers stored differ, so every reflective surface
(diagnostics, rendering, documentation, error text) has to choose which pair it shows. `15` built a
door plus a const-generic "tag" that carries the consumer's own numbers into the diagnostic for free
at runtime, which repairs the diagnostic cost without touching the machinery (`15` sections 4, 4.1).

**Both, with the surface declaring which.** Consumers write integer and fraction; the machinery and
any low-level surface speak total and fraction; each reflective surface states which it shows. Keeps
both audiences at the cost of making the dual vocabulary permanent rather than a door-level
conversion.

**A fourth reading: the numeral is defined once, as a grid cut down to a reach, and the width pair is
what that definition is called in the constant-canonical-exponent case rather than a second
definition.** `24` derives this from the record's own affine value map (`seed/SETTLED_laws.md:274`)
and compiles it gate-free: the fraction width names the grid (the negated canonical exponent), the
total width names the reach measured in the grid's own units, and the integer width is their
difference, which is why it is a view and why it goes negative exactly when the reach lies wholly
below one (`24` sections 2.1, 3.1, 3.2, probes `s1` through `s5`, set equality 121 of 121 in exact
rational arithmetic). Under this reading Q2 is not a fork between two definitions but a question about
what a **consumer-facing projection** of one definition should be, and the width-pair-versus-derived-
view choice this entry is about survives unchanged as the surface question. `24` also names three
sibling framings it did not take and describes why: carry only the width pair and scope floats out
(cheapest, fails equivalence: two teams would build fixed point and float as unrelated types); carry
only the concept and derive widths at use (purest, but a canon that never names what a consumer
actually writes has described the machine and not the tool); carry both as co-equal with a stated
translation (rejected: a translation between two definitions is itself a thing that drifts, which is
the failure this reading exists to prevent); make the reach the primitive and the grid derived,
keying on `(EMIN, EMAX, count)` (rejected: the count of magnitudes in a reach is a sum over binades
under a general canonical exponent, so it is the reach that needs the grid to compute, not the other
way round) (`24` section 5.3).

**Open, connecting to the fourth reading:** the concept-side vocabulary (radix, adjustment, bias,
phase, canonical exponent) and the width-pair vocabulary are not translatable in both directions. From
a width pair to a concept numeral the map is total and injective; from a concept numeral to a width
pair it is partial, presentable at only 4 of 14 representative shapes in `08`'s own classification,
with the misses grouped into six named causes of which three (non-constant canonical exponent, i.e.
floats; a reach count that is not a power of the radix; nonzero phase) are already inside the design
rather than exotic (`24` section 2.2). So a canon carrying the width pair as *the* definition of a
numeral would be silent about the design's own float family, `Ranged` member, and `Bias` axis. Whether
the design admits numerals its coordinates cannot name is stated by `24` as op's own question (`24`
section 9), separate from and prior to which surface pair a consumer writes.

Argued to be **prior to Q8**, because it changes what the shape space is a space of (`15` section 8;
`23` S9/S17 collision, dissolved by the grid-and-reach reading per `24` section 2.3).

## Q3. Is there a mixed-numeral addition?

**None exists.** No operation anywhere in the record adds values from two different numerals. If that
is the design rather than an accident, addition joins the consumer-determined block and the entire
inference surface collapses to multiplication plus the container (`06` section 2.2, its "site 16",
searched across `SETTLED.md`, `seed/SETTLED_laws.md`, `DROPLIST.md`, `02_carried` and `03` and not
found).

**It exists and is inferred.** Mixed-numeral addition is intended, and its result numeral is derived.
Keeps the join in the inference surface and keeps the negative-integer-width corner live at its
residual pairs.

**It exists only through an explicit conversion.** No implicit mixed addition; a consumer converts one
operand into the other's numeral first, and the conversion is where the lossless-conversion predicate
fires. Collapses the inference surface the same way the first option does while keeping the operation
reachable.

Still open as of the last file to touch it (`23`, "what is missing": "If the answer is no, a large
part of Cluster B has no caller. One sentence from op collapses it.").

## Q4. What does a datum stand for?

Op explicitly refused to bound this set. **These are what has been written down, not what is
admissible.**

**A point.** A datum is one value. Saturation's absorbing behaviour is then not a denotation at all
but a documented restriction on where a fold is sound. Under this reading saturating is exactly as
unsound as wrapping at the measured rate (`07` section 4.2: 512 of 1024 point-reading failures at n=5,
identical arithmetic to the absorbing reading below).

**An absorbing top.** The top denotes everything above it. **Sufficient but not necessary for
soundness while the computation stays at it**, and the "exactly" qualifier the panel first wrote is
false: `18`'s own committed probe output has an operation set that decreases (multiply by zero) with
absorbing sound at 0 of 512 failures, so "stays at the endpoint" is not a necessary condition (`18`
section 3.1, `18_probes/p2.out` section B; `21` section 1.1 and `19` independently reproduced this from
the same table). Extended to additions and subtractions over a stated domain, 936 of 5184 chains are
unsound under the absorbing reading at four steps; both endpoints absorbing still fails at 840 of 5184
(`18` section 3.1). What is known: the reading holds while the computation stays at the endpoint
(sufficient); what is not known: the exact necessary condition. A candidate exists ("every operation
must map the absorbed set onto a set the numeral denotes exactly", offered by `19` and named in `23` as
S5, blocked) and has not been built or second-read (`23` S5).

**A constructor-level clause.** The denotation clause is a statement about the **constructor** wearing
the grammar of a statement about every datum. The distinction it draws is that a **partitioning** set
denotation is free while an **overlapping** one is not, which is why the design's own quiet cases cost
nothing while intervals cost the order (`18` section 4.1, section 2.1: rounding cells and the
absorbing top are both partitioning and keep the order total at 120 of 120 pairs; intervals overlap and
decidability falls to 42.05% at `U<2,2>`, 35.45% at `U<3,3>`). Two sub-readings of "the clause is in the
right place": (a) the wording is wrong, since it reads as a per-datum property when it is a
per-constructor one, fixable in one sentence with no mechanism; (b) the clause is doing two jobs
(membership, and which denotation the laws quantify over) and should split, at the cost of the boundary
no longer being one sentence (`18` sections 4.1, 4.2).

**A set, admitted generally.** Intervals and set-valued data are first class. This is a canon change
rather than a type addition, because the value-level total order is a precondition of the law layer.
It costs the total order, multiplicative associativity outright (measured: 1818 of some pairs at one
width, 9524 at another, where neither association contains the other), and the additive inverse except
on degenerate data (only 16 of 136 interval data satisfy `A - A == 0`), with distributivity surviving
weakened to containment (100% of failures have the left side contained in the right) (`18` section 2.2,
probes `p3b`, `p3c`). It buys back verified optimisation, rigorous ODE work, exact geometric predicates,
broad-phase culling and static range analysis. **Not free**, contra an earlier reading: the construction
(a pair of numerals with both directed rounding modes reachable per operation) costs nothing to erase
and nothing to derive a container for, but it costs the entire order-and-law layer the design was never
going to supply, which the consumer then has to write themselves (`18` section 1.1).

**A live sub-fork inside every reading above: soundness or bestness.** `07` (section 4.4) shows the
"is this the tightest honest answer" question is a separate, cheaper claim from "is this a sound
answer at all". **Soundness** ("the derived numeral holds every value the operation can produce") is
always true of every formula in the design already, including the un-tightened sum-of-widths product
form, and needs no admissions. **Bestness** ("...and no numeral smaller does") is a strictly stronger
claim, requires the tight product form, and requires admitting both the origin shape and negative
integer width (see Q9-adjacent material below on the tight form). Stating soundness alone is cheap and
correct; stating bestness is a further, priced commitment. `07` recommends this fork explicitly as a
sharper replacement for "should the canon claim tightness" (`06`'s framing), not as a rival denotation
reading; it composes with all four denotation options above.

Note that `Precise` on `inexact` is argued to be this same question one level down: a strategy that
refuses on inexact is the strategy that demands its data keep a point denotation, and the size of that
demand is measured (100% exact for add/sub, falling to 4.60% of in-range divisions admitted at
`U<4,4>` under a point-only strategy) (`18` section 3.4).

> **The strategy set is OPEN, and Q5 and Q6 both read as though it were four.** Op, 2026-08-08 (`39`):
> "the strategy set is not closed at exactly four. These are the ones the last panel settled with, and
> what my amateur ass had written for arvo that we are now redesigning ... so it's entirely open to
> discussion and exploration." The number, the names and the decomposition are all open, and this
> panel has never examined the four **as a decomposition**; they were the ground everything stood on.
>
> That reframes `25`'s finding rather than weakening it. Under a closed set of four, "the four names
> are an exact bijection with a two-by-two of headroom against layout, zero cells spare" reads as a
> tidy coincidence to be explained. Under an open set it is evidence about **what the axes actually
> are**, with the four presets as one sampling of a space rather than its partition.
>
> The counterweight is op's too: a prior design can name the parts well and go wrong in the execution,
> and where a name survives scrutiny it is kept and the keeping is a result. Open is a licence to
> question the decomposition, not an instruction to replace it.

## Q5. Is the arithmetic column one axis or two?

> **Two corrections from `40`.** The axis list should carry the **observable-or-not** classification
> beside each axis, since that is what decides how the axis may be governed and it is not derivable
> from the name. And this entry's corroboration count should read **three, not four**: one of the four
> cited corroborations is about a different column from the claim it supports.


**Two axes.** Overflow policy and intermediate precision are independent, and every strategy needs a
value on both. The evidence is that three presets state an overflow policy and say nothing about
intermediate precision while the fourth (`Cold`, "widen-op-narrow") does the reverse, answering a
different question ("what precision does the intermediate carry") than the one wrapping and saturating
answer ("what happens when a result does not fit") (`25` section 4.2). Widening then narrowing is not
an answer to the question wrapping and saturating answer. Independently corroborated four ways: arvo's
own preset table decomposes into an exact two-by-two of headroom against layout with zero cells to
spare (`25` section 4.1, `25_probes/p2`); four industrial fixed-point systems (SystemC `sc_fixed`, AMD
Vitis `ap_fixed`, Siemens `ac_fixed`, MATLAB `fi`/`fimath`) independently decompose the same way from
outside arvo entirely (the `arvo-strategy-is-a-preset` memory, cited at `25` section 4.1); and a
committed 34-run bench family, `warm-clamp-arity-*`, holds strategy, overflow and layout fixed and
varies the accumulator (intermediate precision) against the container (headroom) with three different
winning arms across 34 runs and no arm winning everywhere, which is the independence the preset table
itself structurally cannot show (`25` section 6.2, `25_probes/p4`).

**One axis.** The four names are values of a single arithmetic-policy axis and the asymmetry is a
wording problem in the preset table rather than a missing axis. Keeps the four-name bijection with
the two-by-two of headroom against layout intact with no new degrees of freedom. Under this reading
widen-op-narrow is a legitimate fourth value alongside wrapping and saturating, on an axis better
named "evaluation policy" than "overflow policy" (`25` section 8, stated as the genuine alternative op
is choosing between, not as refuted).

**A product of several axes.** The arithmetic column is a product of however many policy axes exist,
with the four presets being named sections over it rather than a partition of it. This is the shape
the written strategy definition implies, and it means the preset table is a sample of the space. This
reading is now the one a fuller derivation of what a strategy *is* converges on: a strategy is "a
consumer-written name for one coherent policy" that "assigns one value on every axis," where each
assignment is a function of the build condition (a constant being one case), so named strategies are
**sections over a product of axes** rather than values of a single axis at all (`25` section 0, the
canon sentence proposed; permanence and equivalence both argued to pass). Under this reading the
axis list itself is open past the two contested here: SIMD lane count is argued to be derived (a
function of container width and target vector width) rather than a fifth axis, but this is not
verified (`25` section 4.3, section 9); rounding is a candidate fifth axis absent from arvo entirely
per the cited prior-art memory (`25` section 9).

Prior to Q6, because it decides what a wrap-or-clamp answer would be an answer **about**.

## Q6. Does `Warm` wrap, or clamp?

**Clamp.** Matches the ratified preset table, which gives `Warm` the clamp and gives wrapping to
`Hot` alone. Makes the committed wrapping bench family (`warm-container-*`) measure a different
strategy than the one it names (`20` section 1.4, quoting `warm-clamp-shared`'s own doc). Under
clamping, headroom goes from 2.2x worse at fold arity two to 44x better at arity 256, crossing over
exactly where its own interior-safety predicate (`W + ceil(log2 n) <= width(accumulator)`) says it
should, to the row (`20` section 1.5, `warm-clamp-arity-w13`, 6 arity rows).

**Wrap.** Matches the committed bench family as implemented (`warm-container-*`). Makes the ratified
preset table's `Warm` cell wrong, and op has already declared that cell stale under his restated
intent that Warm behave "like native primitives in regular old rust would" (`20` section 1.4, quoting
`seed/SETTLED_container.md:405-408`). Under wrapping a lazy headroom arm provably cannot win, because
reduction modulo `2^W` factors through reduction modulo `2^C` for any `C >= W` (`20` section 1.3).

**The question dissolves under Q5's two-axis answer**, because then `Warm` states a value on the
overflow axis and a value on the intermediate-precision axis separately, and the two bench families
may be measuring two different axes rather than disagreeing about one (`20` section 1.4, `25` section
6.1: `22`'s and `20`'s two results read as a single-axis refutation under one framing and as two
separate, non-contradictory axis measurements under the two-axis framing).

Standing caveat on the evidence: `20`'s own section 1.5, the clamp-family arity crossover, was written
after reading a commit subject line carrying its own conclusion and is self-flagged as contaminated,
"owed an independent read that has not run" (`20:188-191`, `20:674`; `21` section 2.3 found the map had
dropped this disclosure while carrying the numbers, which reproduce exactly and are not themselves
disputed, only the ordering of when they were derived).

**Directly connects to and is measured by Q7 (the carrier question) and the wide-rung bench (`22`)**:
`22`'s wide-rung bench measures under wrapping only, deliberately, because saturation was what produced
six void cells earlier in the panel (`20` section 2.1); if the wide rung is supposed to clamp, `22`'s
numbers measure the wrong semantics (`22` section 10, section 11).

## Q7. Which carrier is the packing claim about?

**The contention run has landed (`27`) and it moved the answer.** Op's "explore, wait for Fog" (`28`
batch two, Q7) is discharged. The question is not closed, but its shape changed: **the carrier the
claim is about now depends on how many cores are running**, so a claim naming a carrier without naming
a regime is underspecified twice over.

Break-even carrier, one core against four (`27` section 10):

| | one core | four cores |
|---|---|---|
| committed kernels, warm | 6.9 to 7.4 bytes | 2.7 to 3.0 bytes |
| committed kernels, cold | 5.6 to 6.2 bytes | 1.8 to 2.1 bytes |
| best kernel each side, warm | above 8 bytes | about 1.7 bytes |

Against real carriers with the best kernel on each side, past last-level cache (`27` section 10, as
corrected by section 15/10.4): `u64` goes from +5-9% to +67-71%, **`u32` changes sign** from -3-10% to
+48-60%, and `u16`, which packing had never beaten anywhere in the single-core sweep, goes from
-59-111% (`27` section 10's original table) to +9-16% once measured properly past the cache rather
than only 1.1 to 1.3 times past it (`27` section 15, which supersedes section 10's `u16` row
explicitly, section 10.4).

**The mechanism, as a controlled experiment rather than an argument.** The identical `UADALP` change
(a vectorised pairwise-accumulate reduction replacing a scalar one) is worth 23-25% on `d32` at one
core and **+0.9%** at four (`27` section 9, attacking the dense side with the same instrument that
attacked the packed side in `26` section 8, which is the reversal that destroyed `27`'s own first
result; see below). At 33.5M records the best dense kernel in the directory returns **0.99** from four
cores: four times the cores, the time one core took (`27` section 9.2). Under contention a dense
column's cost stops being a property of its kernel and becomes a property of its width.

**It attacked and destroyed its own first result, and that is part of the finding rather than a
footnote.** Attacking only the packed decode gave packing a clean 12-20% win. Attacking the **dense**
side with the identical instrument destroyed that result; the 40% decode win from the isolated attack
is real and kept, but the conclusion it appeared to support was an artifact of an unattacked competitor
(`27` section 9).

Host bound, stated because it limits every number above: one M1, four performance cores, 12 MB L2,
roughly 60 GB/s aggregate. One core already pulls 53-55 of that, so contention arrives at two cores
here and would arrive elsewhere on a different core-to-bandwidth ratio. Reads only, sequential only.
Every warm-against-cold disagreement is a statement about that 12 MB (`27` section 10.2).

**An eight-byte carrier.** The claim is about displacing a machine word, and against one it measures
true past a threshold record count, with the crossing where the eight-byte column stops fitting the
second-level cache and the packed one still does (`26` section 10, single-core baseline; still holds
under `27`'s four-core regime, strengthened).

**A two-byte or four-byte carrier.** Against `u32` the single-core answer is false and the four-core
answer is true (sign change); against `u16` the single-core answer is false and the four-core answer,
measured properly, is also true. A claim stated without naming its carrier and its core count is false
as often as it is true, and the direction of the error depends on both variables (`26` section 10;
`27` sections 10, 15).

**The inequality, with no fixed carrier and no threshold.** Packing pays when the bytes saved per
element divided by available bandwidth exceeds the decode cost added per element (`26` section 10).
Under contention the "available bandwidth" term is per-thread aggregate bandwidth divided among
contending cores, so the inequality's right-hand side is unchanged and its left-hand denominator
shrinks with thread count (`27` section 10, the stated formula: packing pays when `T · w / c` exceeds
aggregate bandwidth). The inequality is argued to be permanent while any particular break-even figure
is a fact about one machine (`26` section 11.1, `27` section 10.3).

**Footprint rather than throughput.** The claim is about resident memory and never was about loop
speed. The multi-fold cut in resident bytes holds whether or not a loop gets faster (`26` section
10.1), and this benefit has not been priced at all by any file in the panel (both `26` and `27` name
this explicitly as uncovered).

**What is unmeasured and would move the answer further, named by `27` itself:** writes (every arm in
both files reads only; an encode surcharge on the write side is unpriced); random or strided access
(the prefetcher-hiding mechanism this whole thread turns on is specific to sequential access; under a
pattern the prefetcher cannot follow, the footprint argument would have a mechanism it currently
lacks); other hosts (a different core-count-to-bandwidth ratio, a different last-level cache size, a
multi-socket or asymmetric-core topology, would all move every crossing named above) (`27` sections
10.2, 13).

### Op answered the regime question on 2026-08-08 (`32`), and it dissolves rather than selects

The question `27` returned was: is the claim about one core, or about the declared parallel workload?
Both are dead as readings, because the canon does not name a regime at all.

> We will run in threads = 1, threads = 2, threads = n where n can be any finite. We don't take stances
> on these. If it gives juice and proves more efficient than the alternatives, we should do that, when
> we can detect we have several cores available. When we don't, we do what is the most efficient thing
> in a single-threaded realm.

The inequality option above is closer and still not it: that option is regime-**free**, and this is
regime-**sensitive**, resolved wherever the core count becomes knowable.

**The option this puts in their place:** the claim is about **whichever regime is detected**, with at
least two arms behind that detection, and the carrier the claim holds against differs per arm. `27`'s
one-core and four-core break-even bands are then both correct, as the two arms rather than as rivals.

Four separable parts, none assumed beyond his words. Adaptation is conditional on **two** proofs,
performance and soundness, and the soundness one is what an optimisation pass will be tempted to trade
against. The core count is **detected**, and whether that detection is build-time, runtime or
consumer-supplied is not stated. The thread count is **unbounded and unranked**, so one core is neither
the default with parallelism as an extension nor the reverse. And "most efficient in each different
situation" is the general form, of which this carrier question is one instance.

**He marks it a ratifiable intent, which is not ratified.** Per his own correction, an opinion given
before the experts converge is an ack. It is not in the canon and may not be cited as settled. Full
text with the options it chose among: `32`.

## Q8. One numeral family, or several?

**Op's instinct is one.** He said so and said explicitly not to act on it, because acting on an
instinct is how the last panel locked itself into a shape that did not fit.

**One family.** Every numeral is a shape in one space, joins are computed within it. The promise
attached to this holds within one radix, zero bias, and a closed shape space, and the option as
originally put carried only the third condition. Dropping bias produces pairs with disjoint value
sets, needing an empty numeral the option never names. Dropping single-radix produces structurally
unreachable cross-radix joins. `03`'s verdict, stated precisely: "reading A appears to be reading C
wearing A's clothes" (`03` section 3.2), because across kinds (fixed point against float) the join is
not delivered by the two named admissions at all: 220 pairs with upper bounds and none least at box 4,
rising to 302 at box 5, with the witness `U<0,1>` against `U<2,0>` having two incomparable minimal
upper bounds and no least one (`03` sections 3.2, 3.2's table).

**Several families.** Numerals are partitioned into families and cross-family relationships are named
rather than computed. The standing analysis is that the one-family reading, once its three conditions
are stated, appears to be this option wearing the other one's clothes.

**The question is not load-bearing where it was thought to be, in a sharper and now-compiled form.**
`06` tested this head-on with a classification of every site in the design where a numeral appears
that the consumer did not spell, into D0 (consumer determines it, 8 sites), D1 (operands determine it
by a total function, 8 sites), D2 (a named rule over a small closed set, 3 sites), and D3 (nothing
determines it). **D3 is empty, and coherence says it must be**: an associated type names exactly one
type, and declaring both honest readings of a cross-family join side by side is refused at `E0119`
(`06` section 1.1, `06_probes/p4_arm3.out`, "conflicting implementations of trait `JoinNum<...>`").
So a design that infers a cross-family target is picking in one impl, not computing a least upper
bound, whatever the order says. Two genuine callers of the order exist, both as a two-place predicate
rather than as an extremum-fetch: the lossless-conversion predicate and the accumulator sufficiency
check (`06` section 5.1). The lattice's meet and join have no caller found anywhere in the record after
two independent looks, though "no caller found" is a weak negative and `06` says so (`06` sections 1.1,
4.1). What the order **is** for: it is called as a relation, and the extrema are quoted at proof time
to justify that every D1 formula's answer is correct (the formula's answer contains every value the
operation can produce), which is invisible in a caller census but is real work (`06` section 5.2).
Restated by `06` itself: the family question stops being "do the numerals form a lattice" and becomes
"is the admitted shape space closed under the clamp of every formula's answer", which is narrower and
testable, and one instance of it (the tight product form) is measured **not** closed, at 15 of 6561
pairs, exactly where negative integer width is needed (`06` section 4.2).

**A route to one family that survives all three conditions.** Nobody has proposed one that survives.
If it exists it would resolve op's instinct and the standing analysis at once.

**D. A tie-break reading, which survives and was not among op's original three.** Stop requiring a
unique least upper bound; return the set of minimal upper bounds (nonempty whenever any upper bound
exists) and name a stated rule that picks one. Totality is bought with a policy instead of with
closure. Not exotic: it is what C's usual arithmetic conversions do, and it is the shape the design
already uses on a neighbouring axis, since the container axis already has incomparable elements at
equal width (two coordinates, bytes and alignment) and the design already answers with a named rule
keyed on the strategy, with no lattice anywhere (`03` section 7.1; `06` section 2.3, `07` section 3.6,
both independently identifying this as F2 and observing the design's existing precedent already
answers exactly this shape of question). The antichain this needs to resolve is measured at width
**2** across three box sizes with the shape count growing (`03` section 7.1, `03_probes/i5.out`).
What has to be named: the rule, and whether the design exposes the antichain to the consumer or only
its choice. Bounded: width 2 is measured over one float family and one fixed-point family at moderate
sizes; adding decimal numerals or several float radices could widen it, untested (`03` section 7.1).

**E. The seam is the step set, a relation computed from the members rather than a declared label,
which survives and was not among op's original three.** Two numerals are in one family exactly when
their admitted step sets are nested, computed rather than declared. Strongest measured support of any
non-original reading: radix 2 with radix 4 behaves as one family with zero structural failures; radix
2 with radix 3 fails as F1 (no bound at all) in 96 pairs; radix 2 with radix 6 fails as F2 in 3 pairs
and F1 in 52, and the three outcomes track step-set nesting exactly (`03` section 7.2, probes `i4.out`
Q17, `i5.out` Q21). Buys: the family relation stops being an unchecked declaration and becomes a fact
the order itself can see; explains the fixed-point-against-float failure with the same mechanism as
the radix-6 case rather than a separate one. What has to be named: whether the relation is transitive
(untested; nesting is not obviously transitive across the shapes measured, so "same family" may have
to stay a relation on pairs rather than a partition); whether it is expressible in the typestate under
the forbidden-feature set (owed a probe, never written) (`03` section 7.2; `06` section 4.3 argues this
reading is the strongest cost escape, since D1 sites otherwise cost quadratically per family added
while a computed relation writes the formula once against the relation).

**H, reframed and largely answered: does either operation have a caller at all?** `03` raised this and
`06` tested it directly (see the "not load-bearing" entry above, into which this has now merged). What
survives as distinctively `03`'s: the meet specifically still has no located caller after two
independent looks (`03` section 1.1, `06` section 4.1), which bears directly on whether the negative-
integer-width admission is "pure cost". It is not pure cost even under this reading, because `06`
separately locates a caller for negative integer width at multiplication's tight product form (`06`
section 7.2), which is not the meet.

**A fork upstream of the whole family question, from `08`: is the canonical exponent a member of the
design, or are its two currently-named values (fixed point, float) the design?** The design has
ratified a name for a general concept, `canonical_exponent`, and carries only two instances of it plus
a third point (gradual underflow) nested under a different name; `08` shows the general concept is a
function over binades and the design's `ExponentForm` axis is a two-point sample of that function space
(`08` sections 1.2, 1.3, 4.1). If the answer is that the two (or three) named values are the design by
fiat, the family question across kinds is settled by that fiat and only the tie-break (D) and the
step-set-seam (E) readings, or refusal (the "several families" reading), remain live for the cross-kind
case, because the join across kinds is then simply not something the design tries to make total (`08`
section 9, Q1). If the answer is the general function, the cross-kind join becomes total (measured: 108
of 108 cross-kind pairs sweep to a join containing both operands once the general concept is admitted,
`08` section headline and probes `i2.out` Q1, Q2), the antichain `03` measured (`U<0,1>` against
`U<2,0>`) dissolves because its true join is a segmented shape strictly inside both of the two minimal
upper bounds `03` found (`08` section headline, `i2.out` Q2b), and posits and tapered formats come in
for free since they classify inside the general concept with a canonical exponent of no currently-named
shape (`08` sections 3.3, headline). The cost of the general reading is a type-level function
(compiled gate-free and shown to erase in `08`'s `p3_segmented_typestate.rs`) that collides in *spirit*
with the ratified "no enumeration, ever" rule and needs `08`'s own defence that it is inductive rather
than enumerated (`08` section 4.1). This entire fork is separately narrowed by `24`'s later finding
that every canonical exponent shape the design currently names (constant, slope one, gradual underflow)
is expressible as `max(K, e + I)` for two integers rather than as a list, so the "general function"
reading's cost is one more integer, not a list, for everything the design has today; the meet closure
falls out as an algebraic identity on those two integers at 6561 of 6561 pairs (`24` section 3.5,
probes `s5`). What is not yet reached by the two-integer form: the join of two "knee" shapes is worse
than the join `08` measured for fixed-point-against-float (four segments rather than the mirror shape),
and posits (slope two) still need a third piece or a different form (`24` section 3.5).

**Closed, and belongs in `DROPLIST.md` rather than here: ordering the numerals by something other than
inclusion.** Refinement alone and reach alone are each total orders per family and each lattices, but
neither is the relation any operation needs, because it ignores range or grid respectively. Inclusion-
up-to-rounding is not an order at all in the direction wanted, because rounding is not injective and
antisymmetry fails (`03` section 7.5).

**Not a fourth answer, offered as a technique rather than a shape: the ambient-and-realisation
decomposition.** Define meet and join in the ambient lattice of finite rational sets (total and free
there for free), define a partial `realise` from a value set to a numeral, make every operation the
composite. Changes what a canon sentence quantifies over (adding a family later adds a theorem about
where realisability holds, rather than invalidating a totality claim), which is a genuinely different
permanence profile from readings A through E, but it does not decide whether the singleton is
realisable, which is op's question in different words (`03` section 7.4, `03:667-669` explicit that
this is not a fourth answer). `07` (section 3.2) and `08` (section 6) both independently arrive at the
same content with substance attached: every derived numeral is the tightest numeral containing the
operation's exact result set, one mechanism rather than several, which makes the ambient-realisation
framing's content available without its vocabulary. `09`'s persona checkpoint recommends dropping the
framing itself as a live option while keeping this content; noted here for completeness since it is
technically still `03`'s own proposal and has never been formally struck.

## Q9. What should the crossing at the width surface be, between a consumer's written number and the
type system's representation of it?

New topic, absent from the register's first pass, discovered in the material behind op's own standing
instruction ("fresh eyes on the container derivation", `01` section 2) as pursued in `10`, `11`, `12`,
`13`, `15`. The container-derivation *mechanism* itself (width to container, given a nat) is
established: it is total, gate-free, and erases (`10`, confirming the closed panel's `137`). What is
open is a separate, upstream question: how does a consumer's written const literal become the
type-level natural the derivation and the law algebra actually operate on. Every route below was
compiled and is a real arm; none is proposed as *the* answer.

**C0. The design as it stands: keyed on const generics directly, with a per-width bridge (`impl
ToNat<Arvo> for Idx<N>`, one row per written width).** Meets the ergonomics bar exactly (`UInt<5>`,
`UFixed<13, 3, Hot>`, identical character counts to every other candidate below). **Closed on
structural grounds, not merely refused on ergonomics grounds as the record previously had it**: no
finite bridge table is closed under the design's own law algebra, at any size, because multiplication
doubles widths and the table can never contain the widths the algebra produces (`11` sections 7.1, 7.2,
compiled: a failure at 48 from two tabled inputs, the row added by hand and the failure reappearing at
96 and 32). This is the same structural objection that already killed the fixed-width carrier, one
level up (`11` section 7.2, citing `seed/SETTLED_container.md:157-169`). The Rust ecosystem's own
`typenum` crate independently reaches the same shape (same carrier struct, same one-method trait, same
one-impl-per-value bridge) at 1148 rows and 4758 generated lines, still capped at 1024 dense, which is
strong evidence this is the shape Rust forces rather than a shape anyone chose (`11` section 3.10), but
`typenum`'s consumer (`generic-array`) crosses the bridge once at entry and stays in type-land, which
arvo's ergonomics bar (a decimal const at the surface, repeatedly) does not allow (`11` section 3.10).

**C1. A raw nat surface, no const bridge at all: a consumer spells the width as a hand-written binary
digit tower directly.** Closed on ergonomics: fails the bar under both op's own narrower reading and
the panel's wider one (`12` section 3, measured at 35 characters against 7 for `C0`/`C4`'s `UInt<5>`).

**C2. A nat surface with a shipped alias layer (`N0`, `N3`, `N5`, ... shipped as names for the
towers).** One character over the bar (`UInt<N5>` against `UInt<5>`) and fails on the panel's added
disqualifier, a type-level magnitude at the alias site (`12` section 3). Also refused on its face: a
shipped `N0..N64` table is the width table `SETTLED.md:110` refuses ("no enumeration, ever, if it can
be helped") (`12` section 6).

**C3. A nat surface with a consumer-side declaration macro minting the names it wants.** Same one-
character-over-the-bar cost as C2, and refused on its face as the macro escape `SETTLED.md:110` also
names (`12` section 6). Worth noting precisely: op's own stated disqualifiers (precision in the
spelling, container types) do not by themselves disqualify a macro; the "no macro call" disqualifier is
the panel's addition to op's sentence, and `12` and `13` both independently declined to lean on it, for
different reasons (`12` section 9; `13` "A macro at the alias site", declined on a mechanism ground
rather than op's wording: the macro would move the type-level crossing to a stage the type system is
not watching, so the validation clause of the erasure gate would be enforced by the macro rather than
the typestate, which is a different design).

**C4, equivalently derived independently as arrangement A. The hybrid: a consumer-facing type alias
carries the const parameter; the numeral itself is keyed on nats; the bridge fires once, at the alias,
and the ceiling never re-fires because the algebra never re-enters the bridge.** `pub type UInt<const
N: u32> = Fixed<NatOf<N>, T0, Warm>;`. Byte-for-byte identical to C0 at every consumer site (`12`
section 3, measured off compiling text). Independently derived twice, by `12` (deriving it as "the bar
never said the width must be a const generic parameter") and by `13` (deriving it, with `12` unread,
by asking "which direction of the crossing is free", per `13`'s stated and probe-timestamp-verified
working order; `14` audited the independence claim and found it holds, with one section flagged
inherited by `13` itself). The table's domain shrinks from unbounded (every width the algebra can
produce) to the finite set of literals a consumer actually writes: three octaves of multiply compile
past a six-to-sixty-five-row table containing none of the produced widths (`12_probes/p03`; `13`
section "Arrangement A", `13_probes/p14`, four multiplies to 256 bits past a 65-row table; `15_probes/
q15`, six-row table, none of the produced widths present). **Still pays the table `SETTLED.md:110`
refuses**, just a bounded one rather than an unbounded one (`12` section 6, table): "the choice is not
between a design with a table and one without... What differs is how much the table has to cover."
**Cost: the diagnostic degrades to an unreadable binary digit tower** at a width mismatch, e.g.
`expected 'Fixed<D1<D0<D1<D1<Term>>>>, ..., ...>', found '...'`, not repairable by `#[diagnostic::
on_unimplemented]` because that attribute does not reach `E0308` (`13` section "Arrangement A's cost";
`12` section 5, its K1). Two independent, composable repairs exist and each costs something: (a) carry
the consumer's own const numbers as a defaulted, otherwise-unused type parameter so `E0308`'s primary
label uses them (`12`'s `p06`; `15`'s "tag", zero runtime cost, one defect: a computed product's tag
does not match a hand-written alias with the same numbers unless deliberately unified, `15` section
4.1, closed by pairing with arrangement D); (b) rebuild the nat ladder in base ten instead of base two
so the (still-present) tower is at least readable and untruncated, costing roughly sixty additional
impls, all tables of digits (`12`'s `p09`, `p10`). **A separate, independently found defect belongs to
this design as it stands and to every candidate below except D: a Rust type alias does not check its
own bounds, so an undeclared width at the alias-*definition* site (the exact site the ergonomics bar
governs) produces no error at all; it surfaces at first use, possibly in another file, naming an
internal type the consumer has never seen** (`12` section 7, two independent instances: `13` section
"What a consumer reads", `15_probes/q15`'s `c2`). One repair is known and it needs an unvetted feature:
`lazy_type_alias` (not on the workspace's vetted-feature list) makes an alias eagerly bound-checked,
closing the silence completely, but costs fifteen library-side bounds across the ladder's own helper
aliases and, more seriously, is a **consumer-side** feature gate that a library cannot itself turn on
for its consumers (`12` section 7, `13` section "What `12` says", which found its own first-pass
verdict of "closed route" wrong and withdrew it after reading `12`).

**B. A named arrangement, distinct from C4/A: the width type visible to a consumer is the literal
itself (an `L<K>` construction), with the structural nat demoted to a hidden `Repr` projection.**
Readable diagnostic recovered with **zero** repair cost (`expected 13, found 12`, with no digit tower
anywhere), because the type printed is the consumer's own number. **The real ceiling is real here**,
and it is what establishes precisely *why* C4/A has none: naming an operation's *output* requires
crossing back from a computed nat to a named literal type, which is the reverse of the one-way crossing
C4/A uses, and the reverse crossing needs a reverse table that must cover every width any operation
*produces* rather than every width a consumer *writes* (`13` section "Arrangement B", the worst message
in the whole set: a digit tower is reported as `Named` unsatisfied, with eight more towers offered as
"help"). This is the sharpest single finding of the width-surface thread: **the ceiling was never a
property of the bridge or of the const surface generically. It is the price of crossing back, and a
table is the only known implementation of either direction of crossing** (`13` section "Where I go
further than `12`", independently corroborated by `14` and `16`). The design rule this yields, offered
as the canon-shaped sentence most likely to survive: **cross once, at literals, in one direction**
(`13`, `14` names this "the sentence most likely to survive into a canon" and `16` reports every
mechanism it built obeying it without aiming at it).

**D. Declare the output width explicitly; check it is wide enough by a free type-level comparison; no
reverse table exists because no output is ever named computationally.** `let out: UFixed<26, 6, Hot> =
a.mul_into(b);` with the trait checking `26 >= I1+I2, 6 >= F1+F2` rather than computing them. **No
ceiling, best diagnostics of any arrangement** (headline error carries the consumer's own numbers
directly, no tower, no repair needed) (`13` section "Arrangement D"; `15_probes/q14` corroborates,
pairing D with the const-generic tag to also carry a name-consistency guarantee). **Real, stated cost:
`let c = a * b;` no longer infers a result type; a product site states its output shape**, which for a
fixed-point library may be the honest shape (somebody has to decide where the point goes in a product)
but is a genuine, un-costed change to what a consumer writes at every arithmetic call site rather than
only at an alias-definition site, which is outside where the ratified bar was written to govern (`13`
sections "Arrangement D", "What is op's"). **Explicitly not worked out by any file: what this does to
tier one, whose entire premise is `T: Add` with no typestate at all**, which `13` names as "the tier
with the most consumers" and "the first thing I would attack next", unattacked (`13` section "Where I
go further"; `14` flags this as the largest single unresolved gap in the whole thread). D and C4/A are
shown to coexist in one crate without coherence trouble (`13_probes/p28`), so a design could compute
outputs where the table covers them and require a declaration past that; whether that composition is
better than either alone is explicitly not decided by anyone.

**Closed and belonging in `DROPLIST.md`: a bare byte-count carrier (`[u8; B]`, B written directly by
the consumer).** No bridge, no table, arbitrary widths with no declaration, and it erases and
vectorises identically to the native form, refuting two of `11`'s own predictions in the process
(`11` section 10.2). Dies not on ergonomics but directly on the ratified acceptance criterion: the
consumer is now naming the container in thin disguise (computing `ceil(bits/8)` themselves), which op
refused explicitly and directly ("Container naming is explicitly wrong. The entire idea of arvo is that
the strategy guides container selection, not the user," quoted at `11:871-873`). Independently
reproduced as C4's sibling failure by `12`'s finding that no candidate escapes the "how many bits are
declared" question cleanly.

**Closed: a macro-generated bridge table reading only the widths syntactically present in a module.**
Not among `10`'s thirteen originally-enumerated routes; attacked directly by `11` and closed
structurally rather than by refusal. It changes *who writes* the table, not that the result is a
table, and it is strictly worse than a hand-written one because it can only see widths that are
syntactically present in source, while the widths a law produces (e.g. `11`'s `b01`'s failing width 48)
appear in no source text at all, being computed by rustc (`11` section 10.1).

**Named, not proposed, and structurally established rather than merely surveyed: the bridge is not
blocked; only its codomain is.** A total, uncapped, enumeration-free const-to-type bridge is compiled
in one blanket impl with no literal anywhere in the crate (`impl<const N: usize, M> ToNat<M> for
Idx<N> { type N = [u8; N]; }`), and it fails only because the codomain overshoots by a factor of eight
(bytes needed is `N`, not `ceil(N/8)`), with the closing division refused in four independent syntactic
positions, all terminally naming `generic_const_args` (forbidden) (`11` section 10.3). This does not
open a route; it replaces "impossible" with a single nameable, permanent reason ("one division by
eight, refused in every position that could compute it"), which is offered as the more honest and more
canon-appropriate sentence.

**Cross-cutting, unresolved, and load-bearing for every candidate above except C4/A once keyed on
`(W, F)`: does any of these arrangements need to represent a negative integer width, and if so can
it?** Every ladder built in this thread (`10`, `11`, `12`, `13` before `15`) encodes widths as binary
or decimal naturals, unable to spell a negative integer width. `06` (Q8/Q9-adjacent material)
established negative integer width has a real caller at multiplication's tight product form (15 of
6561 pairs). `15` dissolved this collision, but only for a design keyed on total-and-fraction width
(see Q2): under that keying the corner is carried by the encodings at zero cost in the mechanism,
compiled over the whole 81-shape box with a negative control (`15` sections 1.1-1.3). Under a design
keyed on integer-and-fraction width, the collision is unresolved for any arrangement in this section,
and `15` reports a signed structural integer was considered and deliberately not built, because it is
trivially constructible but the downstream cost (signed addition, signed comparison, a proof-of-
naturalness gate ahead of the container-rung ladder) is large for a corner the coordinate change
removes entirely (`15` section 6.4). This corner comes back into play if the meet turns out to have a
caller after all (unsettled, see Q8), since the meet is the one place `(W, F)` itself can go negative
(`15` section 1.3).

## The derivation's outputs

**How many outputs does the container derivation have, and what is each one for?** Independently
derived twice, in the corrected-order dispatch shape (`16` derived before reading `15`, contamination
on the *count* only, self-declared): **two**, not one. The **carrier** (called "container" by `15`),
the machine type an operation lowers to, and the **stride** (called "extent" by `16`, corrected to
"stride" after comparing against `15`'s account, since the raw extent turned out to be the declared
width the consumer already wrote rather than a genuine second output; `16` section 10.1). Rung: TWO
EXPERTS on the identity of the two outputs and on what the second is keyed on; ONE EXPERT on the exact
count, by `16`'s own downgrade after finding a commit-subject leak (`16` section 0, `21` section 2.2
confirms this rung split survives audit).

**Why a one-output derivation is a real, silent, structural failure and not a theoretical one.** A
carrier-only derivation of `UFixed<13,0,Cold>` occupies 23.1% more memory than the strategy promises,
because the map from `(width, strategy)` to carrier is not injective: 1024 declarations behind ten
carriers in one swept box, averaging 102.4 declarations per carrier (`16` section 7). **The panel's own
certifying instrument for the erasure gate's fourth clause is structurally blind to this class of
error**: its method compares one operation's emitted code against one native instruction, so it has no
array in it and cannot have one; a carrier-only derivation passes it at full marks, for every strategy,
including `Cold` (`16` section 5, section 7's `p3_blind_suite.rs`, four of four green including a
tautological `size_of` check the probe's own author flags). Pointed at the cases it cannot see, the
instrument does not go quiet: it reports the `Hot` and `Cold` numerals as the *same function*, which
is the assertion a carrier-only derivation would produce (`17` section 0). The check that *would*
catch it (a packed round trip at a nonzero bit-phase) has a second-order blindness: it is data-
dependent, and returns the correct answer whenever the bits it truncated happened to be zero, so a
hand-written test using small counter values (0..64) observes zero of the class where a test filling
the declared width observes 32 of 64 (`16` section 7, `17` sections 3.2, "D1"). And the load type used
to read one element out of a packed run is **neither of the two outputs**: it is a third, derivable
quantity (`floor((W+6)/8)+1` bytes rounded up), and reaching for the carrier as the load type (the
nearest thing to hand) reads too few bits at 28 of 64 widths, wrong exactly when the truncated bits
were nonzero (`16` sections 4, 7).

**What the second output is keyed on, and two negative controls that pinned it there.** Not the width
alone: at `W=24` a first attempt (`8 * ceil(W/8)`) gave three bytes for a `u32` container that is
actually four (`15` section 3.4). Not the rung alone either: `Hot`'s wide-rung arm pads to align 16,
so at `W=200` the byte payload is 25 bytes but the container-and-therefore-stride is 32, which a
rung-only keying misses (`15` section 3.4). **This sub-claim is ONE EXPERT and the register previously
said otherwise.** It cited `16` section 10.2 as independently confirming it; `16:739-742` disclaims
exactly that: "I did not build a wide rung so I cannot confirm or contest it. My `p7` touches the
alignment half of it and not the stride half." Corrected by `44`, which read `16` rather than this
account of it. The stride is keyed on the **strategy-and-rung pair**, on `15`'s evidence alone at the
wide rung; the general keying claim, that stride follows the carrier's size rather than the value's
rounded bytes, **is** carried by both files. Alignment specifically is **not** a third output: it rides on the carrier
(a property of a type, via `align_of`), confirmed by an adversarial construction where two wide
payloads have identical size and identical stride but different alignment (`16` section 10.2,
`16_probes/p7`).

**`Cold` is not a container choice with a field attached; it is a statement about how a run of values
composes, and this is why it has no standalone value form at all.** Independently reached from both
directions: `15` finds a lone `Cold` value has the identical carrier to `Warm` at the same width; `16`
finds a lone packed value has to have a size, so packing cannot be a statement about the standalone
type in the first place (`16` section 10.2, section 12; TWO EXPERTS, both self-report independent
arrival). Consequence for how a canon states the two-output design: "the derivation produces a
container and a stride" invites reading them as two coordinates of one answer; "the derivation answers
a per-value question and a per-aggregate question" states why they are not (`16` section 12).

> **`52` second-read `50` and the refutation holds.** It reran every probe in `50_probes/` and all eight
> reproduced byte for byte with zero feature gates, then hand-traced the closure logic against the
> specific cell that reproduces the unit's `{carrier, stride}` answer, confirming that cell depends on a
> closed strategy set (contradicting op's ruling) and an unbounded const-to-type rule (contradicted by
> compiled refusals). It independently checked the `Precise`-fork algebra and the access-width group
> theory, where the phase set is the subgroup of the integers mod eight generated by the stride's gcd
> with eight.
>
> **The unit's strongest convergence, and its most inflatable claim, disaggregated.** The dispatching
> agent reported this as "three authors, three methods, one destination" and called it the strongest
> thing in the unit. `52` marks its own version as **the weaker link**: "a reading of three sentences in
> three vocabularies", and says a reader who thinks the three are genuinely different ideas should treat
> it as "the part most likely to move". The headline was taken and the hedge dropped, which is the third
> time in this unit a claim was amplified past its author's own qualification.
>
> The honest form, from `53`: the ownership clause exists **in instance form inside `16` itself**
> (`16:280-282`, in the passage that does not use the criterion); `49` derived the **general form cold**,
> before reading any panel file, with its phase-one commit ordering confirmed in history; `50`
> formalised it; `52` second-read it. That is one cold derivation, one in-file precedent, one
> formalisation and one second read, which is a real and unusual base **and is not four independent
> derivations of one principle.**
>
> **Two corrections to `50`, neither load-bearing.** Its "several solutions" framing conflates two
> claims: the criterion has at most **one** solution per fixed parameterisation, and separately leaves
> three background parameters unstated, which is what produces seven answers across sixteen cells. Only
> the second is what its probe shows.
>
> **And the refusal count is contested three ways**, which is itself the finding. `50` said twelve, `52`
> recompiled and said thirteen, and a fourth count by regex over the same `.err` files gives twenty. The
> arithmetic is not in dispute; **the domain is unstated**, which is precisely what `17` identified as
> the failure mode making counts the panel's most fragile claim class. Any canon sentence resting on a
> count of refusals must state what it counts.

> **`51` found the real hole, and it is worth more than the invented one: the existing erasure arm is
> quantified over ONE width.** Swept across 36 widths, the typestate walk **stops matching its
> hand-written twin at W >= 18**.
>
> **And the failure is not extra instructions.** At W=19 the typed arm emits **11** loop instructions
> against the hand-written **34**, and is the **worse** code, because it collapses to one accumulator and
> one element per iteration where the hand-written keeps five and four. Loop-carried dependent work per
> element goes from 2.00 to 6.00. An instrument counting instructions would have scored the typed arm
> the winner.
>
> **The collapse is a conjunction**, isolated by a control that fixes the gather shape and sweeps only
> width: the gather written as a loop over an associated const, **and** W >= 18. Neither alone.
>
> **Two repairs land, and neither changes a character of what a consumer writes.** Moving the gather
> from a loop bound to an associated type with a flat impl per access width erases at **36 of 36**
> widths. A fixed eight-byte load also recovers it, beats the hand-written arm at most widths, and at
> W=47 recovers a split **the hand-written arm cannot**. The first is the shape the workspace rule about
> a refused bound wanting a trait already names.
>
> **Bounds, stated by its author.** One host, one toolchain, **no cycles anywhere**: nothing is timed, no
> bench ran, and what the collapse costs is **unpriced**. Stable at 15 to 16 of 36 widths across three
> optimisation levels and three element counts; one level is degenerate and reported as such. The LLVM
> mechanism is a reading rather than a proof, and three widths recover under nothing.
>
> **It also refuted its own first attribution and kept it**: live access bytes is a function of width, so
> width and access window cannot be varied independently by any experiment inside this design. And its
> first harness produced 36 of 36 green and **could not have failed**, both arms folding to one symbol.

> **`50` found the criterion is not a definition, and the unit's answer is one of seven.**
>
> `16:100-101` puts the set being defined on both sides of its own third clause, making it a **fixpoint
> equation** rather than a definition, and its operator is **non-monotone**: adding a fact makes another
> derivable and removes it, so neither a least nor a greatest fixpoint is available as a tie-break.
> `50_probes/p1` solves it exhaustively across three parameters the sentence never names (site model,
> strategy set, kind regime) and finds **seven distinct criterion-consistent answers, of sizes 0, 2, 3
> and 4**: `{}`, `{access, carrier}`, `{carrier, stride}`, `{access, carrier, compute}`, `{access,
> carrier, stride}`, `{carrier, compute, stride}`, and all four.
>
> **The unit's `{carrier, stride}` is produced by exactly one cell of sixteen**, and that cell reads the
> strategy set as **closed**, contradicting op's ruling that it is not, and the kind boundary as
> **absent**, contradicting twelve compiled refusals in this panel. So the converged answer rests on two
> assumptions the panel has already rejected elsewhere.
>
> **It also corrects `48` and me.** Reading A is not a reading but unsound: its first clause excludes the
> declared width and a site cannot recover it for **389 of 512 declarations**. Reading B does not give
> one, it gives **zero**, or `{access, carrier}` once the kind boundary is honoured, which is the unit's
> arity with a different member. And `48` cited a probe for a collapse that projects a stride an impl
> wrote down rather than recomputing one; `50` built the construction the claim actually needs, and the
> collapse is real.
>
> **The ladder result reverses, and both prior files were computing from an over-estimating closed
> form.** `47` reported zero shared jump points between the native and access ladders. From the closed
> form the access jumps are `[2, 10, 26, 58, 122]`, sharing none with native `[9, 17, 33, 65]`. From the
> **phases a packed run actually reaches**, the access jumps contain all four native ones, so the access
> partition **refines** the native one. A packed run reaches all eight bit phases only when the stride is
> odd, and `16`'s "carrier is the wrong load type at 28 of 64 widths" becomes **16 of 64**.
>
> **The `Precise` fork is not a fork.** Per-step and end-of-chain refusal admit exactly the same chains
> once zero operands are excluded, with refusal admitting 0.000018% of three-multiply chains at F=8
> (`50_probes/p6`). What survives is whether the wide product a fixed-point multiply forms is **carried**
> between operations. Op should not be asked the question as three files have posed it.
>
> **The fact set is not closed under an open strategy set.** One plausible fifth strategy makes the
> derived verdict silently wrong, the repair relocates the fact onto the strategy, and a sixth breaks the
> repair. `49` reached the same shape cold.
>
> **And a magnitude three files called unpriced is now measured**, on the harness with the dense carrier
> as competitor: a runtime-derived access plan costs **3.04x to 3.12x** a compile-time one
> (`mock/benches/bitpack-decoder-shape`, four sizes, committed).

> **A cold derivation (`49`) landed as an independent instance, and corrected itself against the
> panel.** Dispatched under an inverted reading order: it saw only op's intents, the acceptance
> criterion and the workspace rules, derived its answer, committed it, and only then read the panel and
> appended a reconciliation without rewriting phase one.
>
> **It arrived at the same mechanism shape blind**: a trait schema, per-strategy impls, a
> strategy-independent fact schema, and validation falling out of refusal. That is the first agreement
> in this unit that is not a read.
>
> **And it conceded three things to better-established panel evidence**, which is what makes it worth
> having: alignment is recomputable from the carrier rather than a separate fact; its packing flag
> under-specifies what stride captures; and its claim that `Cold`'s storage type diverges from `Warm`'s
> does not survive `15` and `16`'s doubly-independent finding that the standalone carrier is identical
> across strategies at a shared width, with only stride differing.
>
> **One thing it added that the panel had not stated.** Validation can fall out of a plain missing-impl
> refusal (`49_probes/p2`), which is a different and cheaper mechanism than the kind boundary the unit
> spent most of its effort on.
>
> **Its erasure probe is scalar-only, which is exactly the blindness `16` and `17` warn about.** That
> part stands.
>
> **The dispatching agent's generalisation from it was false, and `51` refuted it in one command.** The
> register previously said no probe combined a packed sequence with an assembly-level erasure check, and
> named `16_probes/p3_blind_suite.rs` as the nearest thing. Both halves are wrong. The arm exists:
> `17_probes/t2_aggregate_erasure.rs` builds a packed column, walks it through a generic carrier, walks
> the same bytes through a hand-written twin, and emits assembly for both, with its result on disk and
> reproducing byte for byte. And `p3_blind_suite` says the opposite about itself in its own comments:
> "this is a weak stand-in... There is no array in it and there cannot be."
>
> The supporting census was also wrong: it counted 183 top-level probe files while the corpus holds 350,
> because the grep never descended into subdirectories. **A negative claim about evidence is a claim
> about a place and is checkable in one command**, and this is the sixth instance in this panel of
> something being called missing while the repository held it.

> **File four (`47`) dissolved the one-versus-two fork, and corrected this entry's filing.**
>
> **One richer output suffices if and only if it is a type.** Both spellings were built. Type-valued
> (`47_probes/p1`) compiles gate-free and repairs `16:126-141`'s collapse: the eight `Cold` widths 9
> through 16 that share one carrier get eight distinct single outputs, with a negative control refusing
> three false type equalities. Value-valued (`47_probes/p2`) is **compiled-refused six times across
> three syntactic positions, each naming the forbidden `generic_const_exprs`**.
>
> **So the wall was never information loss. It was the kind boundary**, and one-versus-two was never a
> fork: once the single output is a type with named projections, it **is** the pair wearing one name.
> What is forced is the count of facts that must be available **as types**, which is not the count of
> facts.
>
> **The proposed permanent sentence, arity-free and surviving a third projection:** *the derivation's
> result must make available, as types, every fact a lowering site cannot recompute from a const.*
>
> **Correction to this entry.** The wide-rung alignment collision was filed here under sufficiency. It
> is about **reducibility**: `45_probes/p1_wide_rung_collision.rs:1` states its own subject as whether
> the width-and-stride pair determines the carrier. With a type-valued carrier the pair does separate
> the two strategies at `45`'s own W=256 witness. That filing error is the dispatching agent's.
>
> **And under one reading, two is already insufficient**, which nobody had stated. `Warm` and `Precise`
> at the same width share carrier and stride and differ only in compute (`47_probes/p5`, with the
> equality included as a must-not-refuse control that produces no error). So a compute carrier would not
> be an addition to a sufficient pair; it would be the **repair of an insufficient one**.
>
> **What `16` got right, kept.** The access width is not a third output; it is a function of the width.
> What it costs is a second rung partition, which `16` did not name: the native and access partitions of
> widths 1 to 128 share **zero** jump points (`47_probes/p6`), so one ladder cannot key both, and the
> ladder is precisely the part the design has already refused to enumerate.
>
> **Independence declared honestly:** `47` read `45` and `46` before deriving, so its support for the
> converged claim is a third **read**, not a third independent instance.

> **Converged after an exchange, `44` to `45` to `46` to `45` again, and narrower than `45` first
> stated.**
>
> **Rung, corrected by `48` after the dispatching agent inflated it.** An earlier version of this entry
> called it the panel's first TWO EXPERTS result and said `46` re-derived it independently. `46` says
> the opposite about itself: "my confirmation is a third read, not a third independent instance, and I
> say so rather than claiming a rung I have not earned." The honest rung: **ONE EXPERT on the count**,
> since `16:17-33` downgrades its own agreement there because a commit subject leaked the number before
> it derived anything; **TWO EXPERTS on the content**, from `15` and `16` deriving before either read
> `45`; plus three reads. `44:380-383` writes "two independent derivation routes" while reporting
> `16`'s self-downgrade at `44:72-79`, and that internal inconsistency is where the inflation came from.
>
> **Forced unconditionally, by `Cold` alone.** An injectivity failure where eight distinct `Cold` widths
> collapse onto one native carrier (`16:126-141`), needing only `Cold`'s ratified packing intent plus
> the fact that Rust has no arbitrary-bit-width native type.
>
> **Conditional, and `45` conceded this.** Its second forcing, the wide-rung alignment collision, took
> its carrier types from the dead tree and called that settled architecture. `15:418-429` says the
> opposite about itself, under a heading reading "What the strategy semantics are is NOT settled here,
> deliberately", calling the alignment rule an assumption "safe to leave open". On being resumed, `45`
> built `45_probes/p7`, which compiles the collision at byte counts and alignments sharing nothing with
> the dead tree's types, and states what it does not establish: **the mechanism is general and
> unconditional; whether arvo's strategies ever pick two different alignments is an unratified design
> choice.** So this is a real second forcing waiting on an axis nobody has settled, not a second fact
> standing beside `Cold`'s.
>
> **And a vacuous check was found and replaced.** `45_probes/p4`'s "widening recovers" arm compared a
> value to the identical expression, so it could not fail. `46` caught it; `45` conceded and built
> `45_probes/p6`, which models a genuinely finite intermediate rounded twice. At F=6, of 73,461
> disagreement witnesses, zero headroom disagrees with all and full doubling recovers all, with minimum
> headroom distributing `{1: 57679, 2: 11971, 3: 2788, 4: 695, 5: 243, 6: 85}`. **Most witnesses need
> one bit; a small growing tail needs the full doubling**, which is why the widening cannot be trimmed
> to the common case. The tautological check could not have produced that.

> **Corrected by `45`: this is not blocked on `Precise`, and the two-output shape is already forced.**
> `45_probes/p1` compiles a witness at W=256 where `Warm` takes a carrier of size 32 align 1 and `Hot`
> takes one of size 32 align 16: same width, same stride, same byte count, **different carrier type**,
> at 40 of 640 wide-rung widths, with zero dependence on `Precise` or on sign, and a negative control at
> W=240 where the byte counts differ and no collision occurs. So (width, stride) does not determine the
> carrier on grounds unrelated to `Precise`.
>
> `45_probes/p2` reproduces `16`'s own carrier representation on that identical domain and gets **0
> against 40**, because it is a bare bit count and cannot express the distinguishing fact. That is the
> same instrument blindness `16` diagnosed for a different check in its own section 5, now found in its
> own instrument for this claim.
>
> **What genuinely turns on `Precise` is narrower**: whether a *third* output is needed, a compute
> carrier distinct from the storage carrier. `45_probes/p5` shows that is mechanically expressible
> gate-free under either reading, so even it is blocked on op's intent rather than on the type system.
>
> **And one reading is settled by information content.** If `Precise` means matching the exact
> once-truncated chain answer for every input, widening the intermediate is **forced by pigeonhole**,
> proved exhaustively at F=3..6 by two independently coded instruments cross-checking to identical
> counts (`45_probes/p3`, `p4`). That generalises `35`'s empirical fold result into a proof.
>
> **The honest label is "forced by semantics", not "forced by arithmetic".** `45` argues `16`'s
> type-system-only branch exhibits one refused syntactic form rather than showing reduction is
> impossible, and that `16`'s own trait mechanism already reduces it.

**Whether the two-output shape is forced by arithmetic or only by the type system, previously framed as
blocked on the `Precise` strategy's undecided semantics.** If `Precise` does not widen compute past storage, zero of
251 swept extents map to more than one carrier in the box measured, and the pair's irreducibility rests
only on the const-to-type argument (a type cannot be recovered from a const without re-entering the
forbidden-feature wall, `16_probes/p5b`). If `Precise` *does* widen compute past storage, 64 of 251
extents map to two distinct carriers, and the pair is irreducible as a matter of arithmetic, full stop
(`16` sections 10.2, 12). Nobody has built `Precise` as anything but the default strategy under a
different name in any probe in this panel (`15` section 8, `16` section 12), so this is genuinely
undetermined rather than merely unmeasured.

**A separate, standing disagreement never addressed by either file directly: is the strategy an
upstream selector the ladder never sees, or a key of the ladder itself?** `10` (predates `15`, not
cited by it on this point): "the ladder does not know what a strategy is; it maps a width to a
container. Where the strategy puts the crossover is an input to it" (`10` section 3.4), which reads as
strategy-upstream-of-the-ladder. `15` builds a three-input map keyed directly on `(strategy, width,
sign)` (`15` section 3.2), which is strategy-as-a-key. Both compile. The observable difference is
whether the crossover is a property of the ladder or a property of the caller, which bears on what a
diagnostic can say (`23` names this S21, blocked, and states plainly that neither file addresses the
other).

## Q10. Is the inclusion order's own predicate amended to identify shapes denoting the same value set?

**Restored 2026-08-08 after `31`'s coverage check found it absent.** `02:49-51` names exactly two things
as "genuinely undetermined, and is op's", and only the first (whether `Precision` counts the sign digit)
reached the register. This is the second. It is recorded here from `02` and `03` directly rather than
from any summary.

The question: the order's four-condition predicate reads a numeral's **declared** grid and phase. On a
numeral carrying fewer than two values it reads something the order cannot see, because a singleton lies
on every grid and in every phase, so its declared step is not recoverable from its value set. The four
conditions are therefore sufficient for inclusion always, and necessary only where the source carries at
least two values (`03` section 6, checking `02_carried` section 1.6).

**`03` checked it with three independent instruments and it holds.** Python over 1936 ordered pairs at
radices 2 and 3 with the integer width from minus three to three, so singletons at fine declared grids are
present: 188 disagreements between the predicate and true set inclusion, **188 attributed to a source
carrying fewer than two values, 0 unexplained**. Rust over 484 ordered pairs with a different containment
algorithm: 28 disagreements, 28 same cause, 0 unexplained. Witness in both: a numeral denoting only zero at
a declared step of one eighth is genuinely included in one at a step of one quarter, and the predicate says
no because one quarter does not divide one eighth.

**And the instrument that got it wrong is kept**, which is the part worth carrying. `03`'s first Python
instrument reported zero disagreements over 1024 pairs, which would have refuted the claim. Its shape list
held exactly one numeral carrying fewer than two values, and that one had the coarsest declared step in the
box, so the predicate was never offered the case that breaks it. That is a setup that helps, left unfixed
and named in its own header.

**Why it is not a corner.** `03` reports the amendment **cross-cuts every reading of the family question**,
so it is a precondition for that question rather than a detail inside one of its answers. It also connects
to the predecessor panel's own open question rather than being separate.

**The live options**, which are `03`'s two named candidates plus the do-nothing:

- **Amend the predicate to identify shapes that denote the same value set**, so inclusion is decided on
  denotation rather than on declaration. Removes all 188 disagreements by construction, and changes what
  "the same numeral" means everywhere the order is used as a two-place relation.
- **Restrict the predicate's necessity claim** to sources carrying at least two values, and state the
  singleton case as a documented exception rather than repairing the predicate. Cheaper, and leaves a
  predicate whose necessity half is conditional.
- **Leave it**, on the argument that a numeral carrying fewer than two values is not a case any consumer
  reaches. Nobody has tested that argument, and `03` did not offer it; it is listed so the option space is
  not silently three-sided.

`03` asked explicitly for a second read on this and no second read has run.

## Q11. What does the numeral guarantee to a fold, and what does a composition supply?

**Added from `35`, which calls it the most valuable single item it found.** Q1 through Q10 are without
exception about a single value or a single binary operation. **None is about arity-n or about laws
under reassociation**, and the algorithm crates are the thing op names as the selling point. That is a
gap in the register rather than a wrong ordering.

The finding underneath it: a fold's accumulator is loop-carried, so it has one type, and a widening
binary operation gives it another. Four formulations refused, with four negative controls locating the
boundary at the **runtime trip count** rather than at the widening, which composes fine in expressions
and over static-length lists (`35` section 3, `35_probes/p1`).

- **The numeral carries nothing extra; a fold is the consumer's problem.** Cheapest. Costs the
  algorithm crates the ability to state their preconditions, so each re-derives sufficiency by hand and
  the wrong-answer classes are undetectable at compile time.
- **The numeral names its algebraic structure**, so a contract keys on "an ordered monoid with an
  absorbing top" rather than on a numeral. Makes a numeral with no representable one fail to typecheck
  as a product fold's carrier rather than annihilate at runtime. Costs a vocabulary of structures the
  canon must name and keep.
- **The numeral names its accumulator relation**, keyed on a capacity. Since capacity is a type, the
  accumulator is derivable as the width plus the log of the capacity, compiled gate-free (`35_probes/p7`,
  `p8`). Costs a second input to a derivation that currently takes only numerals, which `06`'s
  D0/D1/D2/D3 taxonomy has no cell for.
- **Both**, which is what `35_probes/p7` compiles: the structure names what a fold may do, the capacity
  how wide the result is. `42_probes/p2` composes the two in one signature with **independent refusal
  for each half**, one for width insufficiency and one for law failure, and `40`'s observable/
  unobservable axis split survives at that composition point. That is evidence for the specific
  combination tested, not a general proof that every fold-layer mechanism pairs with every law-layer
  one.
- **The composition supplies everything and the numeral stays a value type.** The mirror image. Puts the
  accumulator relation in tensor-shaped code rather than in the numeral, at the cost of every
  composition re-deriving it.

## Q12. Is the reduction order specified, or is associativity required?

**Added from `35`.** A genuinely different shape from anything else in the register, and it decides
whether the layer above may split, thread or algebraically rewrite a reduction at all.

Measured exhaustively at n=8 over 16.7M vectors, the fraction whose fold answer depends on the split
(`35_probes/p3`): unsigned wrapping **0**, unsigned saturating **0**, signed wrapping **0**, signed
saturating **70.1%**, and f32 55.4% on a sample. **Three of four sign-and-policy combinations are
exactly reassociable, which float never is, and nobody in the panel had said so.**

- **Require associativity.** A fold may be split only where the operation is associative. Signed
  saturating folds then run in one lane, or run under the strategy that permits a soundness trade.
- **Specify the reduction shape**, as a fixed tree over the index range, independent of detected lane
  and core count. The answer becomes a function of the input and the numeral alone, deterministic at
  any thread count, which unblocks the adaptation intent for every strategy. Costs the sentence that a
  fold is a left fold, and costs a single-core implementation a tree it does not need.
- **Make the reduction shape part of the strategy**, so the performance-first strategy splits freely and
  takes what it gets while the others take the specified shape. Fits the per-strategy framing exactly,
  and costs an axis.
- **Say nothing**, and let the answer depend on the core count. Recorded so the space is not silently
  three-sided. Under the soundness condition this is a sacrifice for every strategy except the one
  whose purpose is to make it.

**The mechanism behind the sign asymmetry, from `42`, and it is not sign.** `35` measured that signed
saturating folds diverge at 70.1% while unsigned saturating diverge at 0, which reads as a fact about
sign or about clamp count. It is neither. `42_probes/p3` refutes "one clamp associates, two do not" and
establishes the real condition: **associativity survives exactly when the fold's actual trajectory
cannot reach both clamped endpoints.**
**CONTESTED, and the quoted form is refuted by `55b_probes/p5`.** Exhaustive over signed-clamp triples:
952 divergences, **448 ceiling-only, 504 floor-only, and ZERO involving both bounds**, zero without a
clamp, zero with same-sign operands. Every divergence is a **one-bound** clamp-then-pullback event, with
witness (7, 7, -1) diverging on the ceiling alone at 6 against 7. If divergence never involves both
bounds, "cannot reach both" cannot be the condition that prevents it.
Two bounds on that refutation, stated by its author: it targets **the form quoted in this register**,
since `55b` did not open `42` itself, and a second read of `42` against `p5` is **owed**. The dispatching
agent wrote the quoted form into this register from a summary, which is how it came to be stated more
strongly than its source may support. Original wording follows.
 Verified on plain integers, independent of any fixed-point
representation: two clamps with an unreachable floor give 0 failures over 2,197 to 15,625 triples,
while a *reachable* floor gives 48 and 450. Unsigned saturating addition never reaches its floor, which
is why it associates; the sign domain is a proxy for reachability rather than the cause. The refuted
first hypothesis is kept on the record.

> **Caution on the mechanism paragraph above, from `55b` (`55_probes/p5`), ONE EXPERT.** Against
> the condition as quoted here and at `56:203-205` ("at most one of its clamps can be triggered by
> any association order"), an exhaustive sweep of signed-clamp triples over Q = [-8, 7] finds 952
> divergent triples of which 448 are ceiling-only and 504 floor-only, with **zero** involving both
> bounds and zero without a clamp event: every divergence in the box is a one-bound event, so the
> quoted two-bound form is wrong about all of them (witness: (7, 7, -1), ceiling only, associations
> 6 against 7). The counts support a pullback mechanism instead: divergence needs a clamp event
> followed by an operand moving the partial sum back toward the interior, and zero divergent
> triples have all-same-sign operands, which is why the sign split in the table above tracks the
> divergence. `55b` has not opened `42`; the refutation targets the quoted form, `42`'s own finer
> per-trajectory statement may already carry the pullback reading, and a second read of `42`
> against `55_probes/p5` is owed before this paragraph is rewritten.

**A drafting note rather than an option.** Wrapping's four separately-measured properties
(associativity, commutativity, identity, inverse) are **one theorem**, that the representation realises
the cyclic group of its width, rather than four independent facts. Whoever writes the law-layer prose
can state it once.

**And commutativity is free.** Exhaustive over 626,224 pairs across both overflow policies and both
sign domains, for addition and multiplication: **zero failures** (`42_probes/p1`). It does not vary
with the strategy, so it costs nothing to state once rather than track per axis value.

**A caution recorded so it does not have to be rebuilt.** No entry here should acquire a rewriting or
equality-saturation *engine*, deciding which algebraic rewrites to apply. `42` argues that is a domain
and extraction-strategy decision the substrate may not make for a consumer, per the toolbox-not-policer
rule, and it refused to propose one despite that being its own home ground. Nothing currently proposes
it; the argument is on record in case something later does.

**A candidate reframing rather than an option, spanning Q5 and Q6.** State per strategy **which
properties the arithmetic has** (does the top absorb, is addition monotone, associative, invertible,
does it distribute) rather than which policy it takes. The argument is that the policy names one thing
while the consumer needs two, and a hybrid buying one of the two is measurably not enough.

## Q13. Which axes may a build arm move?

**Added from `40`, which ranks it first among its own findings.** The axes split into **observable**,
where moving them changes the answer a program computes (overflow policy, intermediate precision,
rounding, reduction shape, sign), and **unobservable**, where moving them changes only cost (headroom,
layout, container, lane count). That split is not derivable from an axis's name and decides how the
axis may be governed.

- **Any axis.** An arm resolves everything, including overflow policy and intermediate precision. The
  design has then adopted Rust's debug-and-release semantic split for every numeral, which is coherent
  and is what the imitation intent points at. Costs: the same source computes different answers under
  different arms, and a downstream bound holding in one arm fails in another, which `40_probes/p3`
  compiles.
- **Unobservable axes only.** Arms resolve headroom, layout, container and lane count freely; every
  observable axis is fixed across arms. Buys a program whose answers do not depend on how it was built.
  Costs the imitation intent on the one axis where Rust's own behaviour is arm-dependent.
- **Unobservable always, observable for the performance-first strategy only.** The per-strategy form.
  It may move an observable axis per arm against a provable meaningful gain, the rest may not.
  Inherits the unset "meaningful" threshold.
- **No classification; state per axis whether it is arm-resolvable.** Cheapest to state, and it does not
  say why, so a new axis has no rule to classify it by.

## Q14. At what exchange rate does a strategy's preference yield?

**Added from `40`.** It is op's own question and it is **unset in two of his sentences at once**: the
"provable meaningful gains" that bound a soundness trade, and "if mimicking is consistently just worse
choice". `40` argues these are one hole rather than two, and that naming the rate once answers both.

The options are a shape rather than a number: **a stated rate per objective**; **a lexicographic
ordering with no rate at all**, which is what every objective except the performance-first one already
has; **a rate supplied by the consumer**; or **silence**, with the consequence that "meaningful" is
decided case by case by whoever writes the arm.

## Q15. Are the axes independently resolvable, and in what order?

**Added from `40`.** Distinct from Q5, which asks whether they are independently **stateable**. This
asks whether they can be **settled** one at a time.

Measured on two matched committed families nobody had cited: the set of containers in contention
**differs between wrapping and saturating at 5 of 6 widths** (`40_probes/p7`). So the axes are
independently stateable and not independently resolvable.

- **Independently resolvable.** A strategy's assignment is a product of per-axis argmins. Cheapest by
  far, and contradicted at a majority of widths on both matched pairs.
- **Resolvable in a stated order**, earlier coordinates fixed before later ones are measured. Coherent
  with the observable split if observable axes come first. Costs the ordering, which the canon must
  then justify rather than assert.
- **Jointly resolvable only.** Resolution ranges over the product. Most faithful to the measurement and
  most expensive, since the matrix is the product rather than the sum.
- **Per axis with interactions named as exceptions.** The pragmatic form. Costs a list that grows by
  discovery with no rule behind it.

## Q16. Which sense does "composition" carry, and does the canon need both words?

**Added from `43`, which found the word overloaded and one sense sitting in this panel's founding
sentence.** This is `24`'s phase collision one word further in, and it is op's to settle because both
senses carry his word on different objects.

**Sense one, a numeral kind bound to a strategy.** This is what `00_brief.md`'s opening sentence means
by "the primitives become named compositions over one format concept", and it is what the inherited
record ratified: compositions are public and bindable by anyone, with semantic names and strategy
presets the default documented path rather than the only one.

**Sense two, an aggregate over numerals.** This is op at `32`: "contracts for things that compose to
bigger units than just numerals alone". Vectors, matrices, sparse structures, graph weights.

The dispatching agent briefed `43` on sense two while the panel's founding sentence uses sense one, and
did not notice. The options are to keep one word and pick a sense, to keep both with distinct names, or
to find that they are the same concept at two scales, which nobody has argued.

**What `43` established about sense two, independent of the naming.** An aggregate is a **binding-time
distinction rather than a container**: its static part carries what a numeral's type cannot, because
those are facts about a *run*. The defining boundary is **capacity static, length dynamic**, and `35`'s
fold refusal is that boundary crossed wrongly. `16`, `35` and `08` each found one instance of this and
none had a word for it.

Three compiled results attached to it:

- **No derivation reads the grid** (`43_probes/p2`): carrier and accumulator are literally the same
  types across grids differing in adjustment, bias, phase and canonical exponent, with three negative
  controls including one proving the equality mechanism is not vacuous.
- **The operations disagree with the derivations, exactly** (`43_probes/s5`, exact rationals): a
  same-grid add reads neither adjustment nor exponent; a multiply reads the exponent; the **bias** is in
  neither set, since a fold's effective origin is the bias times a dynamic count. The product's derived
  step matches at 9 of 9 zero-bias grids against 8 of 18 nonzero-bias.
- **Nesting must be flattened** (`43_probes/s3`, `p4`): per-level derivation is one bit wide on 1201 of
  4096 shapes and never tighter; the flattened form compiles gate-free.

**And one that bears on the erasure gate.** The invariant does not survive lowering (`43_probes/p7`): a
run whose length is bounded by its capacity, enforced by its **only** constructor, still emits two
bounds-check failure paths and the largest body of any arm. One clamp at the loop header takes it to
zero. That is the typestate holding at the type level and not reaching the backend, which is the
microkernelling shape the workspace already names.

> **MEASURED by `61`, and the conjecture resolves with a domain attached.** Absorption and the
> coherence law **are the same law exactly where every operand lies inside the representable set, and
> differ outside it.** Over `57`'s own sweep plus a deliberately widened one: **zero** disagreements
> whenever the operand box is a subset of the representable set, in both sweeps and both operations, and
> 206 disagreements where it is not.
>
> **So `57`'s one-paragraph identification is correct on the domain that matters and false as an
> unrestricted statement.** Every operand of a real fold is already format-typed, so it is inside the set
> by construction; the identification holds there and may not be stated without that restriction.
>
> **A correction, and then its narrowing, both recorded.** The dispatching agent reported that absorption
> mispredicts 150 times off-domain, against 356 and 587 for the two coherence readings. `57b` narrowed
> that correctly: the 150 is the **multiplication** row alone, **addition reads zero on both sweeps**,
> and its own probe reproduces the 150 exactly from the source parameters with **residue zero**, showing
> they are the single collapsed class already isolated elsewhere. So it is a fully accounted structural
> case rather than an unexplained failure mode, and the agent's correction was itself too broad.
>
> **And the wrap ring does not survive nonzero fraction either.** `61`'s second probe extended the
> machinery, which already existed but had only ever been driven at zero fraction. Wrapping induces a
> ring **only at zero fraction**; at every nonzero configuration measured, nine of nine, it fails
> multiplicative associativity and distributivity through the **same shared coarsening code** that broke
> saturation's semiring, and it does not even reach semiring status. Its additive half survives as a full
> **abelian group at every fraction width**, and structurally rather than by sweep, because its addition
> never reads the scale.
>
> **Both results are independent of Q3**: neither probe performs a mixed-numeral operation, stated
> explicitly rather than assumed.
>
> **So the picture across arvo's real domain is now uniform.** At nonzero fraction, multiplication is
> broken for **both** wrapping and saturation by one mechanism, coarsening, while addition survives for
> both. The multiplicative failure is a property of fixed-point multiplication rather than of any
> overflow policy.

> **CAUTION on the absorption result, from `59`, and it lands on the dispatching agent's own entry.**
> The identification of absorption with the coherence law is **argued in one paragraph and never
> measured**, and three files now rest on it. The two instruments are not the same: `56`'s reduces
> **both** arguments over the ambient window, while `57`'s reduces **only the left** and draws its second
> operand from a box that is not inside the representable set. `57`'s bridging step assumes the second
> operand is already representable, which its own sweep does not enforce. **Measuring absorption against
> coherence is the cheapest outstanding item in this unit by a distance**, and until it is done the
> identification is a conjecture with three dependents.
>
> **And the "4248 configurations" figure is mostly not numerals.** The format-shaped count anywhere in
> unit two is **100**. State what a count counts.
>
> **`57`'s refutation of `42` is real and was overstated twice.** `42` published the refuting row itself
> and its own probe names the surviving hypothesis, so `57` correctly calls the defect a vocabulary slip
> and then leads with a much larger headline. **The dispatching agent's brief compressed it further**,
> dropping both the slip framing and the fact that `42` refuted itself. That is the fourth time in this
> panel the agent has amplified a claim past its author's own framing.
>
> **The fraction result reaches further than `58` took it.** Of the unit's probe sources, most carry no
> scale token at all. In particular `35_probes/p3`, which carries the reduction-order divergence table,
> is same-scale additive, so **that table is fraction-independent rather than awaiting a fractional
> check**.
>
> **But it rests on a premise no file in the unit read.** Q3, whether a mixed-numeral addition exists,
> is open, and under its second option addition aligns scales, alignment is a shift, and a shift is
> exactly the coarsening that kills multiplication. **The unit's one unconditional result is conditional
> on an open register question**, and none of the five files cites Q3.
>
> **Against arvo's real domain**, the cells stand as: unsigned addition good at every fraction width;
> unsigned multiplication at nonzero fraction dead; signed addition 70.1 percent divergent; and **signed
> multiplication at nonzero fraction entirely unmeasured**, which is the cell the default strategy would
> occupy.

> **`60` reconciled `58`'s impossibility with `55`'s model, deriving it cold.** The per-operation model
> **extends to chains if and only if adaptation is unfused from the operations.** `58` showed a chain of
> eager fixed-point multiplies cannot be written in the per-operation model because the rescale is baked
> into every pairwise step; `55`'s model is the standard one. Both hold, and the fusion is what separates
> them.
>
> Its derived object: a chain is **a composition of exact operations together with a schedule of
> adaptation points**, and the schedule is part of the function's meaning rather than an implementation
> choice. Exactness then grades: composite correct rounding, stepwise, bounded drift, and the structural
> exactness fixed point already possesses. Measured: saturating folds are order-dependent while wrapping
> and widen-then-adapt are invariant.
>
> **A format concept whose operations close over the format cannot state op's chain intent at all.** What
> it must carry is small: the exact-result width algebra, adaptation as a first-class named map, and the
> exactness predicate. The general spelling of the width algebra is refused without a forbidden feature,
> with the compiler error committed; the trait-contract spelling compiles on the pin.

> **`62` measured the empty signed cell, and found the panel's first design-actionable lever.**
>
> **The signed cell is broken where the unsigned story was a theorem.** Signed two's-complement
> saturating multiplication fails associativity **at zero fraction**, at 28, 160, 780 and 3516 triples
> across widths 3 to 6. Hand witness at width 4: `(7*7)*-1 = -7` against `7*(7*-1) = -8`.
>
> **The mechanism is the range's asymmetry under negation**, not the clamp as such: the negation of the
> ceiling is interior while the negation of anything past it falls beyond the floor. And the lever
> follows directly: **a symmetric clamp restores exact multiplicative associativity at every width, zero
> failures**, where the asymmetric range gives 98 at width 5. **One code point is the whole difference.**
>
> That is the first thing this panel has produced that a consumer would feel, and it is a live option
> rather than a recommendation: giving up one representable value buys back an algebraic law.
>
> **At nonzero fraction everything multiplicative dies anyway**, both policies, both rescale spellings,
> 54 of 54 rows. But the coarsening attribution transfers only as **sufficiency**: coarsening alone
> breaks signed associativity with no reduction present, while the unsigned converse that the clamp is
> clean does **not** hold. The signed cell has **two independent mechanisms**, and at the widest fraction
> the two policies fail on the **identical set** of 380 triples.
>
> **The additive half is policy-split in the signed domain**, unlike unsigned where both policies keep
> it: wrapping stays a full group at every fraction width, saturation is dead with counts exactly
> fraction-invariant.
>
> **And the rescale is two different maps on signed data.** Truncation and arithmetic shift move every
> count, zero none, and split the accumulator grade: the shift gives an exactly-F saving at fold lengths
> 3, 4 and 5, while truncation gives an irregular 4, 3, 5, against the unsigned 3, 3, 4. That is direct
> evidence for a rounding axis the register does not currently carry.
>
> Its own probe discipline fired: its first mutant was not a mutant, the run printed FAILS, and the bad
> run is kept on disk beside the corrected one.

> **`57b` withdrew its own shared theorem and replaced it with two hypotheses that predict every cell
> the unit measured.** This is unit two's synthesis and its strongest canon material.
>
> **H1: the ambient operation is associative. H2: the reduction's kernel is a congruence.**
>
> Evaluated mechanically over the signed cube at two scales: **24 cells, zero sufficiency violations,
> zero cells associative without both hypotheses, zero residue.** Every cell predicted in both
> directions, with the instrument validated by observing each hypothesis both true and false, so it
> cannot be predicting by accident.
>
> **And the two fail differently, which is the design content.** H2 fails by **range geometry and is
> fixable**, which is why a symmetric range restores signed multiplication at zero fraction. H1 fails by
> **the rescale and is not fixable**, because there is no associativity left to inherit. It therefore
> predicts, without measuring the cell, that a symmetric clamp **buys nothing at nonzero fraction**.
>
> **It collapses several of the unit's separate findings into one frame.** Addition's H1 always holds, so
> additive verdicts are fraction-invariant. Wrapping's H2 always holds, so its verdicts turn on H1 alone.
> Nothing multiplicative survives nonzero fraction anywhere, because H1 fails there.
>
> **And it conceded the `42` framing in full**, keeping only the narrow propagation point that three
> later artifacts reasoned from the slip. Its own probe failed twice first, by **the same
> over-quantification** one file had just caught in another, made after reading that correction. Both
> failed runs are on disk with their hypotheses attached.

## Q17. Where does the fraction boundary fall, and which results are integer-only?

**Added from `58`, unit two, and it splits the unit's strongest candidate in two.** Arvo is a
fixed-point substrate; almost everything unit two measured was at zero fraction width, which is
integers.

**The additive half needs no boundary at all, and is stronger than it was claimed to be.** The addition
used in `57_probes/p3` and `55_probes/p4` never reads the scale parameter: addition never rescales, at
any fraction width. So the absorption result for addition is **unconditional**, rather than merely swept
clean at three widths as its author reported.

**The multiplicative half was tested entirely at zero fraction, and nobody noticed.**
`57_probes/p2`'s multiply is bare `a * b` with no rescale, and `55_probes/p4` has the identical gap.
Verified directly: the same file's `p3` does use `(a * b) / scale`, so this is specific to the probes
carrying the theorem rather than a habit.

**And `58` argues the gap is not fixable in that form.** No fixed-width eager multiply at nonzero
fraction can supply the exactly-associative ambient operation the sufficiency proof needs, because
rescaling is baked into every pairwise step rather than being optional. Hand-checked witness at F=2 with
operands 3, 5, 7. It also finds `57`'s "clamp only" ablation arm is zero-fraction relabelled rather than
a genuine ablation, since its branch never reads the scale, and that no such ablation is definable
because coarsening **is** what nonzero-fraction multiplication does.

**A measurement nobody had taken, and `60` then corrected it.** `58` reported the multiplicative fold's
accumulator saving exactly one rescale's width below full precision, constant in fold length. `60`'s
probe D shows that is **adaptation fusion, and therefore rounding-conditional and not constant**: under
truncation the saving is 3 bits at fold length 3 and 4, then **4 bits at 5**; under round-to-nearest-even
it is **zero** at 3 and 4, and 3 at 5. So the equality form breaks at n=5 and vanishes entirely under a
different rounding mode, answering the open item `58` itself flagged.

What survives is the shape rather than the constant: the multiplicative fold needs **linear growth with
no closed form**, against addition's logarithmic width-plus-log-of-capacity. That
directly undercuts Q11's accumulator-derivation option as an **additive-only** mechanism, and bears on
op's stated intent that the accuracy strategy holds within chains.

**And rounding mode is not the lever.** Round-to-nearest against truncation changes violation
**magnitude** and never **existence**, which answers a question `57` had flagged as open.

`58`'s first run carried a real bug that turned out to reveal a second independent additive mechanism,
kept on disk beside the corrected run.

## Questions with live options that op has not been asked

Kept separate because the eight (now nine, with Q9 above) carry his direction where he has spoken and
these do not.

**Does the canon carry a numeric threshold at all**, or only the inequality that generates one? The
reading on offer is that an inequality is permanent while a figure is a fact about a machine
(explicit in `26` section 11.1 and `27` section 10.3), and it is marked as a reading rather than a
result by both files that state it.

**Is the derived numeral required to be the tightest honest answer?** The sum-of-widths product form
is not tight: it wastes exactly one bit on a characterised minority of pairs (476 in one box, exactly
reconciled against an earlier count of 461 by `15` as two different, both-correct conventions:
461 = the 160 zero-only pairs plus the 301 narrow-operand-total-width-1 pairs; 476 = that plus the 15
negative-integer-width pairs the clamp otherwise hides, `15` section 1.5), where an operand denotes
only zero, or the narrower operand's total width is one. A tight form exists and its predicate reduces
to a one-line condition, compiled with a negative control (`06` section 7.1, section 7.3). So a canon
sentence claiming tightness would be false as the design stands, and (per the soundness/bestness fork
under Q4) the options are to state only soundness (cheap, always true, no admissions needed), to derive
the tight form and state bestness (requires admitting the origin and negative integer width, per Q8's
tie between the two closure conditions and this tight form's own residual, which `07` shows are
disjoint regions of one formula's codomain rather than two independent repairs: needed by 1 input and
5,487 inputs respectively, with **0** inputs needing both, `07` section 3.1), or to say nothing about
tightness at all.

**How many outputs does the container derivation have?** See the dedicated section above; this line is
kept as a pointer since it was the entry point into that material.

**What a strategy *is*, as a definition rather than a table.** Proposed, not yet second-read: "a
consumer-written name for one coherent policy over how a numeral is represented and how its arithmetic
behaves... a strategy assigns one value on every axis, and each assignment is a function of the build
condition, a constant assignment being one case of that. Strategies are therefore named sections over
a product of axes rather than values of a single axis" (`25` section 7, the canon sentence proposed).
Grounded in op's own ratified intent that "everything varies granularly, and a constant is a function"
(`143b:10-12`, quoted and verified at `25` section 3.3) and in his ruling that arvo's strategy axis and
notko's profile pipeline are "not one mechanism" but share "synergy, nothing more" (`144b`, quoted at
`25` section 5.1, voiding an adjacent unratified claim in `142c` that would have identified them). The
definition is offered as ONE EXPERT and explicitly wants a second, order-inverted read on the specific
claim that it is sections-over-a-product rather than values-of-an-axis (`25` section 9). This is the
answer proposed to (and does not resolve, since it holds under either answer to) Q5's one-axis-or-
several fork; it is a separate finding from Q5 in that it is a *definition* of the concept rather than
a count of its axes.

**What the necessary condition for a sound absorbing-top denotation is.** See Q4. A sufficient
condition is known (stays at the endpoint); the necessary one is not, is named as the "single thing" one
member "would most want checked by someone else" (`18:403-404`), and has been checked twice by
independent readers of the same probe table without being resolved, only refuted in its over-strong
"exactly" form (`19`, `21` section 1.1). This blocks both a canon sentence about saturation and the
`Precise`-on-`inexact` question (`23` S5, "the cheapest unblocking item in the inventory").

**Whether `Precision` counts the sign digit.** The record answers both ways on adjacent rows of one
family table (`IFixed<I,F>` counts it, `FastFloat<P,...>` does not), and no checkpoint rules on it
(`02` section 1.4). It decides: what the sign domain moves (both endpoints, or the floor alone);
whether the three sign domains at equal precision form a chain or leave `NonNegative` incomparable;
whether `Symmetric` at precision one denotes exactly the zero set (bearing on whether a zero-width
numeral already exists under one reading without needing to be separately admitted, `02` section 1.5);
and whether two of the three sign domains collapse at an odd radix. All four consequences computed
under both readings so nothing in `02`'s own derivation depends on the answer, but the answer itself is
not decided by mathematics.

**Does the design want narrowing to compose?** Two narrowings equal one narrowing exactly when the
rounding mode's direction switches only at points the coarser numeral's own grid holds; round to
nearest never satisfies this, the directed modes and round-toward-zero always do (`07` section 2.4,
tested decisively by moving a pivot on and off the coarser grid: 0 failures on-grid, 7 off-grid, across
several trials). If composable narrowing is wanted, this is a constraint on which rounding modes a
numeral may carry that nothing in the record currently states. If not, the canon owes the sentence that
narrowing twice is not narrowing once.

**Is the cross-kind join closed, or merely priced?** `03` establishes no admission from the original
three repairs it. `07` measures that closing the shape space under intersection anyway (the standard
"no best abstraction" response from abstract interpretation) costs a third family of segmented numerals,
sized at roughly 16 to 34 percent more shapes than the two named families contain, with every added
shape new and unnamed by either (`07` section 4.3). `08` sharpens this: the closure buys the "glue"
shapes but specifically does **not** reach tapered (posit-shaped) formats, because intersection takes
the pointwise maximum of canonical-exponent slopes and every posit measured has slope two somewhere
while every measured intersection tops out at slope one (`08` section 3.3). Nobody is proposing the
Moore closure as the design (`03`'s reading D and E are both cheaper and reading D has a design
precedent, per Q8), and it is explicitly marked dropped by `09`'s and `14`'s persona checkpoints as "the
direction most likely to eat a week and produce nothing", though this is persona judgement carrying no
authority (`09` section 6, `14` section 9).

**The seam sentence between the two vocabularies of "numeral" the panel used.** See Q2's fourth
reading, drawn out in full there. Restated here as its own open item because it is prior to, and cuts
across, several of the entries above: it is what makes S9 (Q8's tightest-answer material) and S17 (this
register's Q2 "total and fraction" entry) describe the same object rather than appearing to
contradict, and it is what makes the container-derivation's two-output finding scoped correctly to the
constant-canonical-exponent case rather than misread as a general claim about every numeral including
floats (`24` sections 2.3, 6).

**Should the panel's own "phase" collide across the two vocabularies it uses?** Mechanical, cheap, and
found by measurement rather than by intuition: the concept-side "phase" (a property of a grid, one
number per numeral, inside `08`'s own membership predicate) and the storage-side "phase" (`16`'s bit
offset within a byte for one element of a packed run, cycling through residues) are independent, all
four combinations occur, and both readings are load-bearing in their own files (`24` section 4,
`24_probes/s4`). `24` proposes, as a suggestion rather than a call, keeping "phase" for the value-space
sense (the one with literature behind it and inside the predicate) and renaming the storage-space one
to "bit offset" or "alignment residue", noting `16`'s own prose already half-reaches for "bit-phase" as
a compound without naming the collision (`24` section 4).

**Where does wrapping live, restated after `56`'s attack and `55b`'s reply: one reduction slot,
two law roles, and the domain filing withdrawn.** Proposed by `55` (`55_probes/p2`), attacked by
`56` (`56_probes/q1`, re-run by `55b`), amended by `55b` (`55_probes/p4`). The original entry's
three options are now two, and the mechanism is measured rather than argued.

The measured structure (exhaustive at 4 bits, both instruments): a total retraction onto Q can
hold two independent law families, the adaptation laws (monotone, distance-minimising; what lets
order and error transport, which accuracy claims consume) and the coherence law (rho(a op b) =
rho(rho(a) op rho(b)); what makes chains exact). All four cells are inhabited: signed saturation
holds adaptation and fails coherence (476 chain-divergent triples); wrap holds coherence for
addition and multiplication and fails both adaptation laws; unsigned add-only saturation over a
nonnegative window holds both; the opposite-bound mutant holds neither (`56_probes/q1_output.txt`).
Coherence is definitionally a homomorphism onto the induced operation a # b = rho(a op b), so each
coherent policy computes exactly in an induced algebra, and the algebras grade (`55_probes/
p4_output.txt`): wrap induces the ring Z/16; unsigned saturation induces a commutative semiring
(distributivity at zero failures, no additive inverses); signed saturation induces a unital
commutative magma that is not a semigroup (952 associativity failures on Q itself). The licensed
rewrites follow from the grade: a group licenses reassociation and cancellation, a monoid or
semiring reassociation only, a magma neither. ONE EXPERT on the grading; the semiring row is the
part most wanting a second read, and width transfer past 4 bits is argued, not measured.

The live options:

- **One slot, members classified along the two law roles.** Every generic sentence over the slot
  quantifies over the family that carries it; no member is an exception to anything; the induced-
  algebra grade is the content the law layer states per policy. This is the shape both `56` and
  `55b` now defend, from opposite starting points.
- **Wrap stays an axis value with a permanent exception list.** The original option one. Its cost
  was undercounted in the first version of this entry: chain-exactness sentences carry a
  saturation exception in the mirror direction, so the exception list runs both ways.

Folded, on `56`'s observational-equivalence argument (accepted by `55b`): the previous options two
(wrap as a change of ambient domain) and three (wrap as a named composite) are one semantics with
two drafts. Once a numeral's Q is declared, the section out of Z/2^N is fixed by the declaration,
as `u8` against `i8`; no probe, bench or consumer program distinguishes them; the choice is
drafting economy and must not be dispatched as an empirical question. The conversion-out cost the
first version of this entry carried is withdrawn on the same argument. What replaces it, owed
under every filing equally and therefore not a distinguisher: a wrapped numeral has no
arithmetic-compatible order (a finite cyclic group admits none), so the canon owes one sentence
saying which order comparison means there.

## Unit three: the number-system topic

Added by the coordinator from `65` and `66`, the unit's two blind cold derivations, each sourced. **The
register was rebuilt from files `02` through `27` and had not been extended since**, so unit two's
options live only in `63` and unit three's lived nowhere until this section. That is the exact failure
`RULES.md` records twice: an unresolved option has no result attached, so nothing grips it and it falls
out. These are written out before the unit's consolidation rather than after it.

### Q18. Does a system's selected adaptation belong to its identity or to its realisation

Three positions, not two, and whether the disagreement is substantive or nominal is itself disputed.
`65` folds the adaptation into system identity. `63`'s C2 puts it in realisation. `66` argues the format
unit's two-layer split (format, then the strategy-selected induced algebra) dissolves the question
rather than answering it. **What would distinguish them:** whether two strategies over one format are
one system with two realisations or two systems, and whether anything downstream reads the difference.
`65` and `66` both note that arvo's types carry the strategy either way, so type identity agrees under
all three and the residue may be naming.

### Q19. Are the proposed level hierarchies the same cut or different ones

`65` proposes three levels (system, representation, format) with a change-test attached. `66` proposes
five (number, system, representation scheme, format, container). **What would distinguish them:** apply
each one's own change-test to the other's levels and see which collapse. If they collapse cleanly, one
cut is a refinement of the other and the question is granularity. If they do not, they disagree about
what a level is.

### Q20. Is the inventory of number systems open or closed

Both files reach this and neither closes it (`65` sections 6 and 7, `66`'s own section on it). A closed
set is checkable and a canon can enumerate it. An open ring anticipates systems nobody has named, at
the cost of the canon being unable to say what it covers. **What would distinguish them:** whether the
concept has a membership test that does not enumerate.

### Q21. Is "number system" broad enough to include things that are not about magnitude

`65` takes the broad reading explicitly, naming the two-element Boolean algebra and the vector space
GF(2)^n as number systems "not about magnitude at all" (`65` section 1). `66` carries it open and
records that `63` takes no position either way, while `63`'s own worked example of one format hosting
several systems is consistent with the broad reading without asserting it. **One cold derivation leaning
broad is one instance, and one instance decides nothing.**

### Q22. Are set-valued carriers admitted or scoped out

Intervals, and any future error-tracking pair, make the carrier a set of sets (`65` carried-open 1).
Admitting them generalises the concept cleanly. Scoping them out keeps it smaller. **What would
distinguish them:** whether certified accuracy should be expressible inside the system concept or beside
it.

### Q23. Is the role set closed

Storage, compute, interchange, and possibly chain-extent as a fourth (`65` carried-open 2). Closed is
checkable; open anticipates roles nobody has named. `65` leans closed-with-amendment and holds it open.
This one matters because op's plural in "numeral representations" is what the role set explains.

### Q24. Does the canon speak about cost at all

An axis in the canon with the numbers on the harness, against the canon staying silent on cost entirely
(`65` carried-open 3). `65` proposes the first and calls the second defensible and smaller.

### Q25. How is the law inventory named

Marker contracts per law, a structured declaration the derivation reads, or both (`65` carried-open 4).
Expressibility of the simplest form is established by `65`'s probe. Which form the canon should
**describe**, without spelling an implementation, is the open part.

### Q26. What kind of thing is a platform-width type

`63`'s C2 states that a value set depending on other data "has no Q and is not a format but storage",
which read literally places `USize` and `Cap` outside format in the panel's own narrow sense. `66` reads
this as a third option beyond its own carried fork: not a degenerate instance of the shape family and
not an orthogonal axis, but a different kind of thing the format concept need not account for.
**Weight it accordingly:** "storage" is named once, in passing, in a file about a different topic.

### Q27. Is interoperation conversion, resolution, or neither

`66` finds nothing in `63`, `64` or `65` addressing it directly. The nearest material is `63`'s Q3 on
mixed-numeral addition, which is open, explicitly op's, and load-bearing for the format unit's strongest
unconditional result. Which strategy's laws govern a cross-strategy operation is adjacent to that and
not the same question.

### Q28. Should the canon speak of arbitrary-width demands at all

`65` deliberately did not probe the general width-to-container projection and rests its pipeline claim
on width-as-marker (`65` carried-open 5). Whether the canon should speak of arbitrary const-width
demands or leave width families to design interacts with the forbidden-features boundary and needs its
own evidence.

### Q29. What does the admission contract ask a candidate to expose

Added by `73`, which attacked the standing answer and found its sufficient direction refuted.
**(1) Prefix 3 with the reduction's two law verdicts**, which is `71` X3 (`71:675-679`) and which
`72:227` accepts. Cost: refuted as a sufficient condition twice over. Every system has a second
declaration of itself in which it names its own computed algebra as its ambient domain; that term
satisfies the list verbatim, computes the identical function, and reports both law families clean
(`73_probes/p1`), and a mutation set including a reduction that adapts every value to zero cannot
make the collapsed verdict fail in four of four attempts (`73_probes/p1c`). Separately, that same
zero reduction passes both verdicts under its **honest** declaration while computing nothing. The
necessary direction of X3 is untouched and survives.
**(2) The same, plus the ambient domain's own law inventory, plus a third verdict, the
retraction.** Cost: one more item, one more verdict, and the ambient inventory is a law check that
carries the transfer proviso for the operations with no closed form. Buys: the collapse becomes
harmless rather than forbidden, so the closed-operation systems keep their only honest declaration;
the useful facts are conjunctions, measured as `induced associates iff (ambient associates and the
reduction is coherent)` in 16 of 16 cells with neither conjunct alone predicting (`73_probes/p1d`);
and the conjunction is expressible as a blanket implication, with the weaker design compiling in
the same file so the difference is a compile outcome rather than an argument (`73_probes/p3`).
**(3) Admission relative to a consumer-supplied ambient domain**, the system exposing only its
representable set and its reduction. Buys: the collapse is unstateable rather than caught. Cost: a
candidate does not determine its own identity until a frame is chosen. It does **not** contradict
`63` C2, which says a format is identified by (D, Q) and which (3) keeps; what (3) denies is that a
candidate supplies every component of its own identity.
**What would distinguish (2) from (3):** whether any canon sentence must be true of a system with
no ambient domain named. The ingest door was proposed as that case and is not one: `68:335-337`
makes the ingest predicate a function of (type parameters, bits), which is a coordinate-2 and
coordinate-4 question and needs no ambient domain.

### Q30. Is admission a predicate or a location

Added by `73`. **A predicate**, returning member or not. Cost: it discards the coordinate, which is
the part a consumer needs, and makes every mislocated member read as a non-member, which is most of
the disputed cases (a Gray code, two's complement and a stride are coordinate-4 and coordinate-5
choices rather than rejections). **A location**, returning which coordinate the candidate fixes and
whose it is under `70` L3's ownership key. Cost: an output type rather than a truth value, and it
presumes a coordinate count that `70` L2 argues the canon should not commit to. **A location for
membership and a predicate for hosting**, which is the shape Q31's split falls into. Cost: two
procedures where a reader expects one. **What would distinguish them:** whether any canon sentence
needs to quantify over non-members.

### Q31. Does the canon use one word or two for "is a number system" and "can arvo carry it"

Added by `73`, which argues the two questions have different answers, different decidability and an
inhabited two-by-two. **One word**, folding `68` section 5's residue clauses into the concept.
Cost: the concept then excludes unbounded exact rationals as a matter of mathematics, which is
false, and every hosting clause becomes conditional on the unratified `no_std` and no-`dyn`
constraints inside a sentence about arithmetic. **Two words.** Cost: two admission passages and a
reader told which is which at every use. **Two words with the second scoped to a target**,
composing with `70` L4's target-indexed families. Cost: the same, plus a quantifier over
compilations. **What would distinguish them:** whether the canon ever says something true of a
system arvo cannot host, and it already does, since the bounded windows it admits are defined as
bounded windows **of** systems it cannot host.

### Q21 amendment, from `73`

The entry's stated discriminator is measured not to cut. Every total order on the carrier was
enumerated at widths 2 and 3, and wrapping addition (kernel K1, from I3 per `65:258-259`) and xor
over GF(2)^n admit **zero** compatible total orders each, for the same structural reason, while
saturating addition and min admit some; so an order-or-magnitude reading groups a kernel item with
the candidate the narrow reading exists to exclude, and splits wrapping from saturating inside one
kernel item (`73_probes/p2`). What remains of Q21 is `67:440-446`'s question, whether the ambient
operation family is fixed at (+, x) or is a parameter, and the two entries should be read as one.
**The instance count is unchanged: still one leaning broad, still `65`'s.**

## Standing

Nothing in this file is evidence, and nothing in it is a decision. It is the working set.

An entry leaving for `DROPLIST.md` needs a diagnostic and a statement of what would reopen it. An
entry arriving needs to be written out in full and distinguished from its neighbours. An entry
sitting here needs nothing, and may sit for the whole hundred files, which is the intended behaviour
rather than a stall.

**On this rebuild specifically:** this file was rebuilt from files `02` through `27`. Files `28` and
`29` are op's own answers and a dispatcher's note respectively and contain no new options of their own
to carry (`28` restates and re-scopes the eight questions above; `29` is process commentary). Whatever
member file comes after this one should extend this register in place, per the method it documents,
rather than reading a compression of it.
