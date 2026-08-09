# 71. What crosses between two systems, and how many relations that takes

**Author lens:** Orchard. Graded and indexed structure, what an index has to carry for a
composition to be well defined, effects against demands, and the categorical question that
decides whether two features interact coherently before a typing rule is written.
**Position:** fifth file of the number-systems unit, first of its second half, after `65` and
`66` (the two blind cold derivations), `67`, `68`, and the checkpoint `69`. `70` is being
written concurrently on a different question and did not exist when my probes were designed.
**Probes:** `71_probes/`, six instruments, each committed with its output before this file was
written. Pinned `nightly-2026-05-28` (`71_probes/p0_toolchain.txt`), zero feature gates, no
`dyn`, no `TypeId`, no `alloc`.

**The assigned question.** What must two number systems expose for a value to cross between
them, and is that crossing one relation or several.

**The answer in one line, before the argument.** It is several, it is at least four independent
things, and the split that matters is not the one the unit has been drawing: **the meaning of a
crossing is decided by the telescope's first three coordinates on both sides, and its cost by
the last two, and those two depths are exactly the ones the panel has been treating as one
question about "what a conversion preserves".** Everything below is that sentence's evidence,
its consequences, and the one obligation it exposes that neither system can supply on its own.

## 0. Gates, and what I read

**Canon gate: passes, situation two.** `mock/canon/` does not exist, `mock/crates/` is empty by
the declared mutation order, and this panel is writing the first canon. There is nothing to
defend and nothing binds but op's intents. I re-read `INTENTS.md` in full before writing.
Nothing below settles anything.

**Test gate: no suite exists.** `mock/crates/` holds nothing, so there is nothing to run. The
substitute is the probe discipline, and I applied it to my own six instruments: each carries a
stated hypothesis written before the run, each has its output committed beside it, **one carries
a prediction its own output refuted and keeps both the prediction and the corrected form**
(`p5`), and one had a wrong explanatory note attached to a correct measurement, which I found by
checking my own commentary against the numbers and replaced with a computed reason plus two
assertions that crash the probe if the implication fails (`p5`, the FIRES/ABSORBED/COHERENT
block). That last defect is the exact shape `68` section 1 prosecuted in `66`'s hardcoded
conclusion string, and I record that I committed it and caught it rather than pretending I did
not.

**Read end to end:** `INTENTS.md`, `RULES.md`, `69`, `67`, `68`, `65` (both phases), `66` (both
phases), `63` sections 3, 5, 6, 7, 8 and 9, `OPTIONS.md` Q3, Q4, Q9, Q10, Q11, Q12 and Q18
through Q28 and the standing section, `DROPLIST.md` section 6.
**Opened at the source to check a specific claim:** `67:236-274` and `67:564-570` (section 1's
attack), `67:634-640` (K4), `67:276` and `67:330-331` (section 6's extension), `66:310-345`
(section 5), `65:185-196` and `65:533-536` (section 6), `68:347-362` (section 6),
`63:205-243` and `63:616-624` and `63:692-706` (throughout), `DROPLIST.md:106-108`,
`00_brief.md:130-163`.
**Not read:** `01` through `62` except as reached through `63`, `65`, `66`, `67`, `68`,
`OPTIONS.md` and `DROPLIST.md`; `seed/`; `archive/`; `70`, which did not exist. Every statement
here about `03`, `16`, `35`, `55`, `56`, `57`, `60` is sourced to one of those and inherits
their errors if any. **I did not re-run any other member's instrument.** `68` re-ran `65`'s and
`66`'s thoroughly and I rely on that report rather than duplicating it; my `p1` independently
reproduces all eight of `67`'s crossing numbers, which is the one place I did check another
member's arithmetic, and it was to make my own new rows trustworthy in the same instrument
rather than to audit `67`.

**I built no bench and nothing here is priced.** Every remark below that sounds like cost is a
statement about which level of the telescope a crossing does work at, never about how much, and
where the word "cost" appears it means "does work a machine performs" and not a magnitude.

**One thing to report outside the question, per the standing instruction.** `69` records two
questions as op's and in flight, and both bear on this file. I carry both branches at section 9
rather than assuming either, and I say where my answer would move.

## 1. `67`'s crossing result is a quantifier failure, at the index it did not measure

This has to come first, because `67` section 4 is the nearest existing material to my question
and its headline is false as stated.

**The sentence.** `67:252`: "Every crossing is total and preserves values or patterns at 100%.
**No crossing preserves operations at 100%.**" And `67:564-566` carries it into the register:
"There are **three** crossings, not two, identified by the smallest telescope index at which the
terms disagree, and each preserves values or patterns totally while none preserves operations."

**The telescope `67` itself states has five coordinates**, at `67:143`:

    Format := (D : Ambient) x (Q : Reach(D)) x (rho : Reduce(D, Q)) x (E : Encode(Q)) x (C : Hold(E))

and `67`'s own rule is that a crossing is identified by the smallest index at which two terms
disagree. Five coordinates, five crossing classes. `67` measured three of them, at indices 1, 2
and 3, and quantified over all crossings.

**The two it did not measure are exactly the two where the universal fails.** `71_probes/p1`
measures all five in one instrument, and it measures two operation levels and never conflates
them: the **value** level, the system's own operation `adapt(exact(a, b))` on carrier values,
and the **pattern** level, the container's own wrapping operation on the stored bits, which is
what a consumer gets by operating on the bits without consulting the type. That is `63` section
3.5's raw-adder property, and it is a different question from the first.

| telescope index | value map | pattern map | VALUE ops | PATTERN ops |
|---|---|---|---|---|
| 1, ambient domain | not typed | identity | 108/256 against xor, 1/256 against min | not typed |
| 2, representable set | inclusion | sign-extend | 192/256 add, 101/256 mul | not measured |
| 3, adaptation | identity | identity | 192/256 add, 111/256 mul | **256/256** |
| 4, encoding | identity | bijection, 0 of 16 fixed | **256/256** | **0/256** |
| 5, container | identity | shift | **256/256** | **256/256** |

Two things about this table, and the second is the one worth having.

**First, the control held.** All eight of `67`'s numbers reproduce exactly in an instrument
written from its prose rather than its code: 192 and 111 signed, 136 and 80 unsigned at index 3;
192 and 101 at index 2; 108 and 1 at index 1 (`71_probes/p1_output.txt`). So this is not a
disagreement about arithmetic. `67`'s measurements are right and its universal is drawn from a
proper subset of its own enumeration.

**Second, the structure the missing rows reveal is a mirror pair, and it is the answer to the
assigned question's second half.** Index 4 preserves the value-level operation at 256 of 256 and
destroys the pattern-level operation at 0 of 256. Index 1 preserves the pattern map totally and
has no value-level operation to preserve, because the two carriers are different sets. **The
value relation and the pattern relation are two independent relations**, and each telescope
coordinate decides which of the two is the identity. A vocabulary with one arrow for "a
conversion" is naming one of them and is silent about which.

And index 3, the crossing `67` correctly identifies as the one that looks free, is now sharp
rather than suggestive: it preserves the pattern map at 16 of 16 **and the untyped pattern-level
operation at 256 of 256**, and destroys only the value-level operation. So it is free at every
level a machine can see, and not free at the level that decides the answer. That is a better
statement of `67`'s K4 than K4 makes, and it is measured.

**Why I am spending a section on this rather than a footnote.** `67` section 1 catalogues three
recorded instances of a panel sentence acquiring the wrong quantifier (`61` on `56`'s coherence
law, `57b`'s `p7` twice, `42`'s clamp-counting sentence) and concludes, correctly, that "a
concept whose sentences keep acquiring the wrong quantifier does not have a quantifier problem.
It has a shape problem" (`67:123-124`). The fourth instance is in that same file, in the section
that proposes the shape. That is not a criticism of its author; it is evidence for its own
thesis, and it is the strongest evidence the thesis has, because the failure survived a file
written specifically to name it.

**What survives, and I say so because keeping something is a result.** `67`'s K4 as written
survives in full: it says a crossing "**may** have the first and lack the second", which the
table above confirms and does not contradict. What has to go is the universal at `67:252` and
the register line at `67:566` that carries it. The repair is two words and no evidence moves.

## 2. A crossing is not one relation, and it is not three either

Four independences, each measured, and together they are the answer to "one relation or
several".

**The value relation and the pattern relation are independent** (`p1`, section 1's table). Index
4 has a value identity and a pattern bijection; index 1 has a pattern identity and no value
relation at all. Neither determines the other.

**The two things a crossing can preserve are independent, and they are the two law families the
concept already carries** (`71_probes/p3`). For a narrowing crossing:

- the crossing preserves ORDER exactly when the target's selected reduction is MONOTONE, which
  is `63` C4's adaptation-law family, the one facing the source;
- the crossing preserves the OPERATION exactly when the target's selected reduction is COHERENT,
  which is `63` C4's coherence law, the one facing the target.

Twelve cells, two windows by three reductions by two operations, and the pairing holds in every
one. All four combinations of (monotone, coherent) are inhabited, so neither property implies
the other: nonneg saturation has both, signed saturation has monotone without coherent, wrap has
coherent without monotone, and `56`'s opposite-bound mutant has neither
(`71_probes/p3_output.txt`, regime 1).

The operation half of that pairing is **definitional** once the source's own reduction is held
out of the schedule, and the probe says so in its own header rather than presenting arithmetic
as a discovery. What the definitional identity buys is not a new law; it is a **placement**: the
crossing's commuting square and `63`'s coherence law are the same statement, and the panel has
been carrying coherence as a fact about chains.

**A crossing that composes with an operation is an adaptation point, and the count of adaptation
points is part of the crossing's meaning** (`p3`, regime 2). Once the source's own reduction has
fired, a coherent target reduction is no longer sufficient: signed source saturation into
coherent target wrapping diverges on 990 of 1024 operand pairs for addition and 3155 of 3507 for
multiplication. The hand witness is `a = b = 31` in a `[-32, 31]` source crossing into `[-8, 7]`
under wrap: crossing the sum gives -1, summing the crossings gives -2. Two adaptations sit in
the schedule where the model allows one. That is `63` C1's unfused condition (`63:616-624`)
arriving at the crossing layer, and it is double rounding under its own name.

**A crossing between systems differing at more than one coordinate is not determined by its
endpoints** (`71_probes/p2`). Section 4 is that finding, and it is the one that changes what a
canon sentence may say.

So: at least four relations, or rather one relation on values, one on patterns, one verdict per
law family, and one choice that belongs to neither system. Not one arrow.

## 3. What must be exposed: meaning at prefix three, cost at prefix five

Assembling section 1 and section 2 into the answer the question asks for.

**For the meaning of a crossing, each side must expose its first three telescope coordinates and
nothing else.**

1. **The ambient domain.** Needed before anything, because it decides whether a value-level
   crossing is typed at all. Where the two ambient domains differ, there is no value map: `p1`'s
   index-1 row has no value relation, only a pattern relation, and the operations agree on 108
   of 256 pairs against xor and 1 of 256 against min. A vocabulary that calls that "a
   conversion" is naming something that converts nothing.
2. **The representable set, as a set.** Needed to decide whether the crossing must adapt at all,
   which is the difference between a total injection and a lossy map.
3. **The selected reduction, with its two law verdicts.** Needed to decide what the crossing
   preserves, per section 2's pairing.

**For the cost of a crossing, each side must expose its last two coordinates, and they cannot
change its meaning.** `p1`'s index-4 and index-5 rows preserve the value-level operation at 256
of 256, which is the mechanical statement that a re-encoding and a re-housing cannot change what
a crossing computes. What they change is whether the crossing is work: index 5 preserves the
pattern-level operation at 256 of 256 and index 4 destroys it at 0 of 256, which is `63` section
3.5's constant defect measured from the crossing side.

**So meaning and cost are keyed at different depths of the same chain, and the canon can say so
in one sentence.** This is what I would most want the consolidation to take, because it does two
things at once: it tells a consumer which coordinates they must agree on for a crossing to mean
anything, and it tells an implementer which coordinates decide whether the crossing is free.

It also gives the "what must be exposed" question a bounded answer instead of an open list. The
exposure is prefix 3, on both sides. Nothing at indices 4 and 5 is owed for the crossing to be
well defined, and nothing outside the telescope is owed at all.

**And one dependency nobody has stated.** Deciding clause 2, whether the source's set is
contained in the target's, is exactly the inclusion predicate of `OPTIONS.md` Q10
(`OPTIONS.md:1010-1053`), which is recorded there as genuinely undetermined and op's, and which
`03` checked with three instruments and found sufficient always and necessary only where the
source carries at least two values, with 188 disagreements at radices 2 and 3 all attributed to
that cause. **Q10 is therefore not a corner of an order. It is a precondition of the conversion
contract**, because a crossing that is lossless in fact and refused by the predicate is a
lossless crossing the design cannot admit. I did not read `03` and I did not build an instrument
for its predicate, so this is a dependency stated and not measured, and I decline to reproduce a
predicate I have not opened. `OPTIONS.md:1053` records that `03` asked for a second read and
none has run; this is a second reason to run it.

## 4. Whose reduction governs the loss, and why neither system can say

The sharpest finding, and the one that constrains how the canon may write any sentence about
crossings.

`71_probes/p2` asks whether a crossing between systems differing at several coordinates depends
on the order the coordinates are crossed in. The construction is what makes the answer mean
something: **a step moves exactly one coordinate, so the two terms either side of a step agree
on every other coordinate, and where a step needs a reduction there is exactly one in scope.**
Every single-coordinate crossing is canonical and nothing is chosen inside a step. Any
order-dependence in the composite is therefore a fact about composition rather than an artifact
of an arbitrary choice.

Six unordered pairs over the movable coordinates {Q, rho, E, C}, both orders each. **Exactly one
pair diverges** (`71_probes/p2_output.txt`):

| pair | distinct functions | source values agreeing |
|---|---|---|
| {Q, rho} | 2 | 30/256 |
| {Q, E} | 1 | 256/256 |
| {Q, C} | 1 | 256/256 |
| {rho, E} | 1 | 256/256 |
| {rho, C} | 1 | 256/256 |
| {E, C} | 1 | 256/256 |

And the three-coordinate case {Q, rho, E} has six orders that collapse to exactly **2** distinct
functions, and the probe checks the grouping mechanically: the classes are exactly "the Q-move
precedes the rho-move" and "it does not". The other coordinate is irrelevant to the split.

**The diagnosis.** A Q-move is the only lossy step, and rho is the only coordinate that changes
what the loss does. Take the two orders apart:

- Narrow first, then restrategise: the narrowing happens under the **source's** reduction.
- Restrategise first, then narrow: the narrowing happens under the **target's**.

Those are two different functions, they agree on 30 of 256 source values, and **0 of the
divergent witnesses have a source value already inside the target's set**, which is the probe's
own check that the divergence is entirely about the loss and not about the transport.

**So the canon may not write "the crossing from A to B".** The phrase is well formed only when A
and B differ at one coordinate. Where they differ at more, an order has to be named, and the
canon has exactly three positions available, none of which is forced by anything measured here:

- **the source's reduction governs the loss**, which reads as "the value is finished being a
  value of A before it becomes one of B";
- **the target's governs**, which reads as "the crossing is the target's business";
- **the crossing site names a third**, which is the most expressive and the only one that makes
  the choice visible where it is made.

I decline to pick among them and I say why: this is a choice about what a consumer's written
crossing means, and `RULES.md` puts naming calls of that kind with op. What the panel owes him
is the fact that the choice exists and is observable at 226 of 256 source values, which is what
this section is.

**One thing that is settled, and it is a bound on every option.** The two orders are not equally
principled but they are equally **typed**: `71_probes/p4` builds both composites and both are
well formed, because each is a chain of steps that exist. The typestate cannot break the tie.
This is worth stating plainly because the workspace's standing reflex is to push an invariant
into the types until the wrong program is unwritable, and here that reflex does not reach: both
programs are right programs, and only a canon sentence says which one the notation means.

## 5. Conversion and resolution are one obligation at two arities

`66:310-345` separates **conversion** (a value moves between systems) from **resolution** (which
system's laws govern an operation whose operands disagree), files them as "two separable
questions", and says conflating them is the likelier mistake. `OPTIONS.md` Q27 carries the split
and `67:564-570` answers it by classifying crossings, which is the conversion half.

I think the separation is real at the surface and not real underneath, and `71_probes/p5`
measures the reason. Take one mixed expression, `a + b` with `a` in system A and `b` in system B
and the result declared in system C, and write out the three schedules a design might mean by
it:

    UNFUSED   rho_C(a + b)                    one adaptation.   `63` C1's model.
    CONVERT   rho_C(rho_C(a) + rho_C(b))      three.            conversion, then operate.
    VIA_A     rho_C(rho_A(a + b))             two.              operate in the wider, then cross.

**My prediction was that the three always differ, and it was refuted.** In the cell where both
reductions are wrapping, all three coincide at 2048 of 2048 operand pairs. I keep the prediction
and the refutation, because the corrected form is the better result:

| rho_A | rho_C | UNFUSED = CONVERT | UNFUSED = VIA_A | all three |
|---|---|---|---|---|
| wrap | wrap | 2048/2048 | 2048/2048 | 2048/2048 |
| wrap | saturate | 1200/2048 | 1792/2048 | 944/2048 |
| saturate | wrap | 2048/2048 | 1793/2048 | 1793/2048 |
| saturate | saturate | 1200/2048 | 2048/2048 | 1200/2048 |

The probe computes three properties per cell independently of those counts, and **asserts** the
two implications rather than inviting the reader to eyeball them, so a failure would crash it:

- **CONVERT coincides with UNFUSED exactly when the target's reduction is coherent.**
- **VIA_A coincides with UNFUSED exactly when the source's reduction is absorbed by the
  target's**, meaning `rho_C(rho_A(x)) = rho_C(x)` on every reachable value.

That second one corrects an explanation I had written into the probe and which was wrong:
saturation into a nested window shows no double rounding not because the source's reduction
never fires (it fires) but because clamping to `[-32, 31]` and then to `[-8, 7]` is clamping to
`[-8, 7]`. Nesting, not absence.

And one structural fact worth carrying: over all four cells and 8192 operand pairs, the number
of pairs at which all three routes give three different answers is **0**. The routes differ
pairwise and never three ways at once.

**The reading.** A conversion function does not by itself fix what a mixed operation means, and
the table says exactly when it does. What fixes it is how many adaptation points the schedule
contains and where they sit, which is `63` C9's "the schedule is part of the function's meaning"
(`63:692-706`) arriving at the place two systems meet rather than along a chain. So conversion
and resolution are **one obligation asked at two arities**: a conversion is the unary case with
the target declared, a resolution the n-ary case with the target derived, and both owe the same
three things, a common ambient domain, a target representable set, and which reduction governs
the loss. Only the third is a choice, it is section 4's choice, and neither operand's own system
is privileged to make it.

What `66` got right and I would keep: resolution "needs a rule, not just a conversion function"
(`66:329-331`). What the measurement adds is which rule. It is not a precedence order over
strategies, which is what the prior design reached for and `66` correctly marks as evidence
rather than adoption. It is a **schedule**, and the vocabulary for schedules already exists in
this panel at `63` C9.

## 6. Roles: three of the four are realisation variants and the fourth is a schedule

Taking Q23, which the dispatch names as adjacent and which `69` records as mattering because
op's plural in "numeral representations" is what the role set exists to explain.

`65:185-196` proposes storage, compute and interchange as the roles a demand derives
representations for, and `65:533-536` carries "possibly chain-extent as a fourth" open, leaning
closed-with-amendment. `68` section 6 attacks the bottom of both hierarchies on a different
ground. My attack is on the role set's **homogeneity** and it is measured rather than argued.

**Storage, compute and interchange differ from each other at telescope indices 4 and 5 and
nowhere else, on `65`'s own account of them.** `65:190-192` ties each to a strategy rather than
the reverse, so one demand carries one strategy and derives several role representations from
it, which fixes the ambient domain, the representable set and the selected reduction across the
roles and leaves only the encoding and the container free. **The discriminator, and it is
checkable:** if a design ever lets the storage role and the compute role select different
reductions, a role change becomes an index-3 crossing as well, and `p1`'s index-3 row says such
a change does not preserve the value-level operation. So "may a role change the selected
reduction" is the single question that decides whether roles are realisation variants at all,
and nobody has posed it. A packed storage form and a native compute form of one value have the same
ambient domain, the same representable set and the same selected reduction; what differs is
which pattern names which value and which physical bits hold it. And `p1`'s index-4 and index-5
rows say that such a difference preserves the value-level operation at 256 of 256. **So a role
change of that kind cannot change what anything computes.** It is a realisation variant, and the
plural in op's criterion is, for these three, the codomain of the crossing at indices 4 and 5.

**Chain extent is not that.** `65:240-250` derives it from I7: a chain under an accuracy strategy
may run in a wider or redundant intermediate and adapt once at the boundary rather than at every
step. That changes the number of adaptation points, and `p3` regime 2 and `p5` both measure what
changing the number of adaptation points does: 990 of 1024 divergent operand pairs in one cell,
three distinct functions for one written expression in another. **A chain-extent "role" changes
the function. The other three cannot.**

So the four proposed roles are two different kinds of thing wearing one word, and the difference
is exactly measurable: a realisation role preserves the value-level operation totally, a
schedule role does not. That is the same shape of finding `68` section 6 makes about the
hierarchies' bottom tier and `67` section 6 makes about shared parameters, arriving from a third
direction, and the three compose rather than competing.

**The repair, offered rather than settled.** Keep storage, compute and interchange as
realisation variants of one identity, where the question "is the set closed" is answerable
because it asks something bounded: are the reasons to want a different (E, C) for one
(D, Q, rho) enumerable. Take chain extent out of the role vocabulary and leave it where `63` C9
already puts it, in the schedule, where its own machinery is. Q23 then becomes a real question
with a criterion instead of a list with an odd member.

**And this bears on interchange specifically, in a way that supports `68`.** `68:283-296`
establishes that stored bits are not self-describing, so interchange validity is conventional
and the system identification travels out of band. `p1`'s index-1 row is that claim measured
from the crossing side: sixteen patterns, three ambient domains, every pattern carried by all
three, and the operations agreeing on 108 of 256 and 1 of 256 pairs. The bits witness nothing.
I read `68` before building `p1`, so this is a support with its own instrument rather than an
independent arrival, and I mark it as such.

## 7. What the crossing question contributes to Q20 and Q21

Both are named in the dispatch as adjacent and mine to take if the material leads there. It
does, for one of them more than the other.

**Q20, is the inventory open or closed, and what a membership test that does not enumerate would
look like.** `OPTIONS.md:1604-1609` states the question in exactly those terms. The material
above supplies a candidate test and it is not a new mechanism: **a system is a member when it
can expose prefix 3, that is, when it names an ambient domain, a representable set that is a
constant of the type, and a reduction onto that set whose two law verdicts are decidable.**
That is the admission contract `65` section 7 and `63` section 7 both already reach, arriving
from a third direction, and what the crossing question adds is a reason it is the right list
rather than a convenient one: **it is exactly what one system must show another for a value to
move between them**, and under I11 ("our main selling point are the algo crates ... as well as
the contracts for things that compose to bigger units than just numerals alone") a system that
can cross to nothing composes with nothing. The admission test and the crossing contract are the
same list, and that is a reason to believe the list is complete rather than merely sufficient.

**Q21, is "number system" broad enough to include things that are not about magnitude.**
`OPTIONS.md:1611-1617` records `65` leaning broad, `66` carrying it open, `63` taking no
position, and adds "**One cold derivation leaning broad is one instance, and one instance
decides nothing.**"

I read `65` before forming a view, so I am not a second independent instance and I will not be
counted as one. What I have is a different **argument** for the same conclusion, and the
argument is the contribution rather than the vote.

Under a narrow reading, GF(2)^n and the Boolean lattice sit outside the concept. Then the
index-1 crossing, which `p1` measures and which arvo's consumers perform whenever a mask and a
numeral share a container, is a crossing between something inside the concept and something
outside it, and the concept has no way to type its contract. The narrow reading does not merely
leave those structures unnamed; it makes a real and measurable consumer act unstateable.
`p1`'s index-1 row is that act: three ambient domains over one carrier, agreeing on every
pattern and on 108 of 256 and 1 of 256 operand pairs respectively.

So the broad reading is forced by the crossing question rather than chosen for tidiness, and a
narrow reading needs a companion concept plus a crossing vocabulary spanning both, which is
strictly more machinery for the same coverage. **Still one instance on the conclusion. A second
cold derivation is what Q21 needs and this is not it.**

## 8. Doability, and the one thing the typestate cannot do

A canon must be able to say which things are doable. `71_probes/p4_crossing_contract.rs`
compiles clean on the pin, zero warnings, zero feature gates, no `dyn`, no `TypeId`, no `alloc`
(`71_probes/p4_positive.txt`, exit 0).

**What it establishes.** The telescope's coordinates are separate types; a system is a completed
term; a step is a crossing that moves exactly one coordinate, stated as a where-clause holding
every other coordinate fixed, which is the type-level form of section 4's "nothing is chosen
inside a step". A crossing **declares what it preserves**, and the declaration carries the
matching law obligation: an order-preserving step requires the target's selected reduction to be
`Monotone`, an operation-preserving step requires it to be `Coherent`. The law rows are `p3`'s
measured verdicts transcribed, and the probe header says so, so nobody cites the impls as facts.

Three negatives, each generated by a committed script from the positive file and each with its
transcript beside it (`p4_negatives.sh`, `p4_n1.rs` through `p4_n3.rs` and their `.stderr`):

- **N1**, narrowing into wrapping declared order-preserving: refused, "doesn't satisfy
  `At<WrapK, U4>: Monotone`".
- **N2**, narrowing into signed saturation declared operation-preserving: refused, "doesn't
  satisfy `At<SatK, S4>: Coherent`".
- **N3**, a direct crossing between systems differing at two coordinates: refused, and the
  diagnostic names which coordinate is at fault, "`<Sys<RingZ, U4, SatK, Twos, Plain> as
  System>::Red = WrapK` was not satisfied".

**My prediction said E0277 and the diagnostic is E0599** with the unsatisfied bound named in the
note. The substance held and the code did not, and I record the miss rather than restating the
prediction to match.

N3 is the one worth having: **the ill-formed phrase "the crossing from A to B" has no type**
when A and B differ at more than one coordinate, so the design can make it unwritable rather
than detected. That is the workspace's own thesis working.

**And the thing it cannot do, which is why section 4 goes to op.** Both routes through the
two-coordinate crossing are built from steps that exist, and the positive file constructs all
four of their steps and compiles. So both composites are well typed, and `p2` measured them as
different functions agreeing on 30 of 256 values. **The typestate does not settle which one the
notation means.** A canon sentence must.

Erasure: six `const` assertions that every step witness and both route types are zero-sized,
discharged at compile time. Stated with its bound, because the distinction matters: **the
contract erases, the map does not necessarily.** `p1`'s index-4 row is 0 of 256 at the pattern
level, so a re-encoding step is real work at runtime while its witness is a zero-sized type.
Those are two claims and the assertions are only the first. Nothing here is priced.

## 9. Where this depends on the two questions in flight, and both branches

`69` records two questions as op's and in flight. I do not answer them and I do not assume
either answer.

**Q-A, which verb "validate" is.** My section 3 says the meaning of a crossing is decided at
prefix 3. Under the **compile-time** reading, that decision is made once per crossing site at
monomorphisation, and `p4` is what it looks like: the obligation is a trait bound, the witness
is zero-sized, and nothing runs. Under the **runtime ingest** reading, `68` section 4's door is
mandatory wherever bits arrive without their construction history, and a crossing whose source
term is unknown is precisely such a place: an interchange crossing is an index-1 or index-2
crossing whose source coordinates were never witnessed, so it needs the door and the compile-time
obligation has nothing to attach to. **Both readings leave section 3's list unchanged**, which is
the useful part: prefix 3 is what must be exposed either way, and the two readings differ only in
where the check runs. Section 4's choice about whose reduction governs is likewise unaffected.
So nothing here waits on Q-A, and I say so rather than hedging.

**Q-B, whether the long-standing constraints are op's intents.** My erasure paragraph in section
8 rests on monomorphisation being the dispatch, which rests on no `dyn` and no `TypeId`, which
`69` and `67:48-59` both report appears nowhere in `INTENTS.md`. If that constraint is inherited
rather than intended, then section 8's zero-sized-witness claim is a fact about a design choice
rather than about the concept. **Everything else in this file is independent of it**: sections 1
through 7 are arithmetic and would hold under any dispatch discipline in any language, which is
the permanence test doing its job. I state the exposure rather than implying my whole file rests
on unratified ground, because it does not.

## 10. Fits against the register

**Kills nothing.** No live option anywhere is closed by this file. Written out in full so a
consolidator can lift them, per the register's own convention and because two prior
consolidations each lost a live option.

**Q27 (is interoperation conversion, resolution, or neither) gains its answer's shape, and the
shape is different from the one `67` gave it.** Five crossing classes rather than three, one per
telescope coordinate, measured in `p1`. Two independent relations inside each crossing, on
values and on patterns, with each coordinate deciding which is the identity. Two independent
preservation properties, which are `63` C4's two law families one each, measured in `p3`. And
conversion and resolution as one obligation at two arities rather than two questions, measured
in `p5`. The entry's current line, that none preserves operations, should be replaced by the
table in section 1.

**Q23 (is the role set closed) gains a criterion and an attack.** Three of the four proposed
roles differ at telescope indices 4 and 5 only and therefore preserve the value-level operation
at 256 of 256; the fourth changes the number of adaptation points and changes the function. The
question "is the role set closed" is answerable for the first kind and malformed for the mixed
set. Section 6.

**Q20 (open or closed inventory) gains a membership test that does not enumerate**, and a reason
to believe it is the right one: it is prefix 3, and prefix 3 is exactly what one system must show
another for a value to cross. Section 7.

**Q21 (broad or narrow) gains an argument, not an instance.** A narrow concept cannot type the
index-1 crossing, which is a real consumer act between a mask and a numeral over one container.
Still ONE EXPERT on the conclusion, and it is still `65`'s. Section 7.

**Q10 (the inclusion predicate) gains a consumer.** It is a precondition of the conversion
contract rather than a corner of the order, because a crossing's losslessness is decided by it.
Its requested second read is still unrun. Section 3.

**Q3 (mixed-numeral addition) gains the measurement it was missing.** `OPTIONS.md:143-161`
carries three options: no mixed addition, mixed addition with an inferred result, and mixed
addition through an explicit conversion. `p5` prices the difference between the second and third
in function terms rather than in mechanism terms: the explicit-conversion option is the CONVERT
route, the inferred option is the UNFUSED route, and they coincide exactly when the result
system's reduction is coherent and otherwise differ on 848 of 2048 operand pairs in the measured
cell. **So the third option is not a cheaper spelling of the second. It is a different
function**, and `63:778-784` already flags Q3 as load-bearing for the format unit's strongest
unconditional result. This is the sharpening the entry asks for, and the question stays op's.

**A new option, written out in full: whose reduction governs a lossy crossing.** Three readings,
all coherent, none forced by anything measured. **The source's**: a value finishes being a value
of A before it becomes one of B, so a narrowing crossing uses the source's selected reduction.
Cost: the target's declared policy does not govern values entering it, which reads badly at an
ingest boundary. **The target's**: the crossing is the target's business. Cost: a value can be
adapted by a policy its own system never selected, and under the composite reading of section 4
this is the route that makes a wide wrapping value saturate. **A third named at the crossing
site**: the crossing declares its reduction the way `p4` has it declare what it preserves. Cost:
one more thing at every crossing site, and a consumer who writes nothing gets no default. **What
would distinguish them:** whether any consumer writes a crossing whose correct answer differs
from both endpoints' own selections. Nobody has looked, and the observable difference between
the first two is 226 of 256 source values (`p2`).

**A new option, written out in full: does the canon name crossing classes at all.** **Name all
five**, one per telescope coordinate, so a consumer's crossing states which coordinate it moves
and the law obligation attaches to the class. Cost: five names and a rule that a composite names
its intermediate, which `p4` shows is expressible and which makes the two-coordinate phrase
unwritable. **Name none, and give one crossing relation with a declared preservation set.** Cost:
the classes still exist and are recovered by reading which coordinates agree, so the vocabulary
saves five names and spends them again in every sentence that needs to say which crossing it
means. **Name two, values and patterns**, and leave the coordinates to the design. Cost: cheapest,
and it cannot state section 4's order problem, because that problem is about coordinates rather
than about what is preserved. **What would distinguish them:** whether any canon sentence needs
to quantify over one crossing class and not another, and section 2's independences say at least
two do.

## 11. Candidate canon sentences

Each offered to the consolidation, not as a settlement. Each tested against permanence (still
true and useful after a from-scratch rewrite in another language in another decade) and
equivalence (three independent implementations behave the same). Rungs stated honestly.

**X1, what a crossing is.** *A value crosses between two numeral systems along the coordinates
at which they differ. A crossing is not one relation: it carries a relation on values, a
relation on patterns, and a verdict for each of the two law families, and none of the four
determines another.* Permanence: passes, no mechanism named. Equivalence: passes, since three
implementations disagreeing on the independences would disagree on which crossings are
admissible. Rests on: sections 1 and 2, `p1` and `p3`. ONE EXPERT.

**X2, the two depths.** *What a crossing means is decided by the ambient domain, the
representable set and the selected reduction on both sides. What a crossing costs is decided by
the encoding and the container on both sides, and those two can never change what it computes.*
Permanence: passes. Equivalence: passes. Rests on: section 3, and specifically `p1`'s index-4
and index-5 rows preserving the value-level operation totally while the pattern-level operation
goes from total to nil. ONE EXPERT, and it is the sentence I would most want attacked, because
every other sentence here is scoped by it.

**X3, what a system exposes.** *A system exposes, for the purpose of crossing, exactly its
ambient domain, its representable set, and its selected reduction with that reduction's two law
verdicts. It owes nothing else, and a system that cannot exhibit those three cannot be crossed
into and composes with nothing.* Permanence: passes. Equivalence: passes. Rests on: sections 3
and 7, and I11 for the composition clause. ONE EXPERT, and it coincides with the admission
contract `65` section 7 and `63` section 7 reach by other routes, which is worth saying and is
not corroboration, since I read both.

**X4, the crossing's two preservations are the concept's two law families.** *A crossing
preserves order exactly when the reduction it adapts through is monotone, and preserves
operations exactly when that reduction is coherent. These are the adaptation laws and the
coherence law, and the crossing is their third consumer beside order transport and reassociating
folds.* Permanence: passes. Equivalence: passes; the pairing is measured over twelve cells with
all four combinations inhabited. Rests on: `p3` regime 1, `63` C4 for the families, `67` section
5 for the first two consumers. The operation half is definitional once the schedule is fixed and
the probe says so.

**X5, the endpoints do not determine the crossing.** *Where two systems differ at both their
representable set and their selected reduction, the crossing is not determined by the pair: it
depends on whether the set moves before or after the reduction, and the two answers are the
source's policy governing the loss and the target's. A canon that says "the crossing from A to
B" without naming an order has said nothing in that case.* Permanence: passes. Equivalence:
passes, and it is the sentence with the sharpest equivalence consequence, since two
implementations differing here compute different answers on 226 of 256 values. Rests on:
section 4, `p2`. ONE EXPERT.

**X6, conversion and resolution.** *Moving one value into a declared system and combining
several values from disagreeing systems are one obligation at two arities. Both name a common
ambient domain, a target representable set, and which reduction governs the loss, and both are
schedules: the number of adaptation points and their positions are part of what the expression
means. The schedule stops being observable exactly where the reductions involved are coherent
and absorb one another.* Permanence: passes. Equivalence: passes. Rests on: section 5, `p5`,
`63` C1 and C9. ONE EXPERT.

**Deliberately not offered as sentences:** any ruling on whose reduction governs a lossy
crossing, because section 4 establishes that the choice exists and `RULES.md` puts naming calls
with op; any name for a crossing class, because `67` declined to coin one for a good reason and
five bad names are worse than one; any magnitude, because nothing here is a bench and nothing is
priced; and any statement about whether the role set is closed, because section 6 shows the
question is currently asked over a set with two kinds of member in it.

## 12. What I could not settle

**Whether the index-2 crossing has a pattern-level story.** `p1` leaves the pattern-level
operation at index 2 unmeasured, because a widening's pattern map changes width and the
container's own operation is not the same function on both sides, so the comparison needs a
convention I did not want to invent inside a probe. It is the one blank cell in section 1's
table and I state it rather than filling it plausibly.

**Whether a crossing can be lossy in the pattern relation while total in the value relation.**
Every crossing I measured is total on values or has no value relation. A redundant encoding
would be the case to look at, and `63:258-262` records redundant encodings as wholly unexamined
with `59`'s untested hypothesis attached. I did not build it, and it is the single cheapest
thing the unit's remaining files could add to this question.

**Whose reduction governs, section 4's choice.** Not mine, and I decline to lean.

**The Q10 dependency, measured.** Section 3 states it and does not instrument it. I did not read
`03` and I will not reproduce a predicate I have not opened.

**Transfer past the model width.** Everything exhaustive here is at 4, 5 or 6 bits. `68` section
2.4 established inside this panel that the ceiling is forced by the toolchain rather than chosen
(the 9-bit exhaustive const check refused by `deny(long_running_const_eval)`) and that uniformity
of construction does not by itself carry the transfer, with the droplist's compiled
counterexample of a property true at eight bits and false at nine. My results inherit that
proviso in full, and the two that would worry me most under it are `p2`'s divergence count and
`p5`'s cell table, both of which are counts rather than existence claims. The existence claims
(that the pairs diverge at all, that three routes are three functions in three of four cells) are
witnessed by named operand pairs and do not depend on the width.

**Whether "crossing" is even the right word**, given that `OPTIONS.md` Q9 already uses it for
something else entirely: the crossing at the width surface, between a consumer's written const
and the type-level natural, whose surviving design rule is "cross once, at literals, in one
direction" (`OPTIONS.md:638-640`). Two crossings in one vocabulary, both load-bearing, neither
aware of the other. That is the same collision `24` found for "phase" and the register still
carries as open, and I report it rather than renaming anything, because renaming is cheap to
propose and expensive to get wrong.

## 13. Coverage, bounded honestly

**Built and committed:** five instruments in `71_probes/`, each with its output or refusal
transcript, all committed before this file was written. `p1` (five crossing classes, exhaustive
at 4 bits, reproducing all eight of `67`'s numbers), `p2` (six coordinate pairs plus the
three-coordinate case, exhaustive over 256 source values), `p3` (twelve law cells in regime one and
sixteen in regime two, each exhaustive over its source window's 4096 ordered operand pairs,
partitioned between the regimes by whether the source's reduction fires), `p4` (the typestate
contract, one positive compile and three generated negatives with transcripts), `p5` (three
routes over 2048 operand pairs in four cells, with two implications asserted rather than
eyeballed), `p6` (the citation checker below).

**Citations checked mechanically:** `71_probes/p6_check_my_own_citations.py` opens every
`file:line` this document cites and tests that the target contains the text the claim depends
on rather than merely resolving. Thirty-seven citations, zero failures
(`71_probes/p6_output.txt`). The instrument found three failures on its first run, all three of
them defects in the checker rather than in the citations: it compared raw lines, and prose
wraps. The fix is in the committed source and the three targets were confirmed by hand before
it was made.

**Verified at the source rather than remembered:** every `file:line` in this document was opened
before it was cited. The eight numbers I attribute to `67` were reproduced rather than copied.

**Not done:** no bench, so nothing is priced and every cost-flavoured word here means "does work"
rather than "costs this much". No instrument for Q10's predicate. No re-run of any other member's
probe. No attack on `63`'s H1/H2 frame, which `63:665-673` still names as the thing it most wants
attacked and which is out of this unit's scope. No pattern-level measurement at index 2. Nothing
at nonzero fraction width, which matters more than it may look: `63` C7 (`63:676-681`) records
that no multiplicative structure survives a nonzero fraction width for any policy, so every
multiplicative row above is at F = 0 and says nothing about F > 0.

**Nothing here settles anything.** The mode is explore. Sections 1, 4 and 5 are what I would most
want the unit's remaining files to attack, and section 1's repair is two words in `67` and one
line in the register.
