# 24. The seam between two vocabularies

**Date:** 2026-08-08. **Author:** Amin persona. **Status:** a derivation, and it is one
expert. Nothing here settles, per `04`.

`23` finished by naming a gap it declined to fill: two clusters of the panel's work
describe the same object in vocabularies that have never appeared in one sentence, and no
dispatch had been pointed at the seam. This file is that dispatch. It states both
vocabularies from their own sources, gives the map between them, compiles the map, and
then argues which of the two a canon should carry.

The short answer is that they are not two descriptions of one thing. **One of them
describes a grid and the other describes a grid together with a reach**, and the reason
they read as rivals is that the second names both in a pair of numbers while the first
names only the first and holds the second aside on purpose. Once that is said, the map
falls out, it is exact on the objects both cover, and it is heavily partial in one
direction with a nameable miss set.

There is no incompatibility. There is a scope qualifier that goes missing between two
stretches of the panel, and its absence is load-bearing: it is what makes every derivation
in `23`'s Cluster D read as a statement about numerals when it is a statement about the
fixed-point ones.

**And the brief's premise, which is `23`'s, does not survive checking.** The two
vocabularies have appeared in the same sentence, in `06` and `07`, and `06:450-452` states
five of the six restriction conditions this file derives, about itself, as its own largest
hole. The panel's grep could not see that because it tested one direction over a file range
excluding both. So the qualifier is not missing from the panel; it is stated in `06` and
dropped by everything built on it, which is a compression failure with a different remedy
from a gap. Section 0.4.

## Verdict, before the working

**The joining sentence.** A numeral is a **grid** cut down to a **reach**. The grid is the
set of denotable magnitudes: a radix, an adjustment, a bias, a phase, and a canonical
exponent assigning to each binade the power of the radix that is the step inside it. The
reach is which of those magnitudes the numeral covers. The design's fraction width names
the grid, being the negated canonical exponent in the case where that exponent is
constant, at radix two with unit adjustment and zero bias and zero phase. The design's
total width names the reach, measured in steps of that grid. The integer width, being
their difference, names the binade the reach stops at, which is why it is a view rather
than a coordinate and why it goes negative exactly when the reach lies wholly below one.

**The map, and where it fails.** From the width pair to the concept it is total and
injective. From the concept to the width pair it is partial: 4 of 14 representative
numerals inside `08`'s own concept admit a width pair, and the 10 misses group into six
named causes, of which one is the design's own float family (`s2.out`). So the width pair
is a complete coordinate system for the constant-canonical-exponent numerals and names no
others.

**What compiled.** One `no_std` gate-free definition of a numeral as grid-and-reach, with
four projections written against it, in which the general derivation instantiated at the
constant case recovers `2^W` at every row of a twelve-row box, including compile-time
asserts, and the same derivation produces correct magnitude counts for a float, a knee and a
tapered format that have no width pair at all (`s3.out`). Set equality between the two
readings at 121 of 121 in exact rational arithmetic (`s1.out`).

**And one blocker attacked rather than reported.** The knee is not affine, which looked like
the general form's largest cost. Every canonical exponent the design names is `max(K, e + I)`
for two integers, so the design's whole admitted set is a two-integer shape rather than a
list, and the meet closure `08` measured is an algebraic identity on those two integers
holding at 6561 of 6561 pairs (`s5.out`). The enumeration collision `08:392-396` names does
not arise for the shapes the design has. It arises at slope two and at closing under the
join, and section 3.5 shows the join of two knees is worse than the join `08` measured.

**Which vocabulary the canon should carry.** Both, in a stated relation, and the relation
is what makes it one vocabulary rather than two. The canon defines a numeral as a grid and
a reach, states the width pair as the naming of that pair in the constant case, and
derives it rather than positing it. Keeping the width pair is a result and I am not
proposing it be renamed: `15`'s argument for it is sound and this file corroborates its
numbers by another route. What I am proposing is that it stop being introduced as a
definition, because a design with two definitions of a numeral will drift between them,
and it already has.

**One collision to fix before anything is written.** The two vocabularies already share
exactly one word, **phase**, and it means two independent things (`s4.out`). All four
combinations of the two are realisable. A canon carrying both clusters without renaming
one of them is shipping an ambiguity into its most load-bearing sentence.

## 0. Gates

### 0.1 Canon gate

Ran against `RULES.md`, `01` and `04` before starting.

**Aligned.** There is nothing ratified this could conflict with: `01` section 0 makes op's
word terminal only after convergence, and `04:35-37` extends it so that even convergence
does not settle anything tonight. `SETTLED.md:3-14` carries its own warning that every
RATIFIED row was classified under the superseded reading. I have treated every such row as
an ack and said so where I lean on one.

The one thing that would have made this dispatch misaligned is a live consolidation, since
a file that proposes a joining definition would then be editing the canon candidate by
another name. `23` checked this and found `CANON_CANDIDATE.md` predates the panel; I
re-checked rather than taking its word.

```
$ stat -f "%Sm %N" -t "%Y-%m-%d %H:%M" CANON_CANDIDATE.md 23_spj_*.md
   2026-08-07 22:38 CANON_CANDIDATE.md
   2026-08-08 11:43 23_spj_the_sentences_a_canon_could_carry.md
```

There is no live consolidation. This file proposes a sentence and does not promote it.

**And I did not read the commit log before writing the verdict above**, per
`RULES.md:217-225`. The verdict, sections 1 through 3 and the probes were on disk first.
Section 6 records what the log showed afterwards and whether it changed anything.

### 0.2 Test gate

The standing gate is to run the suite before the assigned work. There is no suite in scope
here: this is a research panel writing a canon, `mock/crates` is the dead tree by the
brief's own statement, and the panel's artifacts are markdown and probes rather than a
crate. What I did run is the thing that stands in for it, which is every probe in this
file's own directory, from a clean build directory, with the commands recorded beside each
output. `24_probes/RUN.sh` runs all four and is the reproduction.

I also read `s1`, `s2` and `s4` for the failure this gate exists to catch, which is a test
that cannot fail. Two of them can and one initially did not, which I record in section 3.4
rather than quietly fixing.

### 0.3 The brief's checkable claims

Four, and the load-bearing one is off in a way that produces a finding rather than a
correction.

**"`23` inventoried thirty candidates, twenty-one carryable."** Holds. `23:905-909` gives
the commands and I re-ran them.

```
$ F=23_spj_the_sentences_a_canon_could_carry.md
$ grep -c '^#### S' $F                -> 30
$ grep -c '^#### S.*BLOCKED' $F       ->  7
$ grep -c '^#### S.*REFUSED' $F       ->  2
```

Thirty, seven, two. Twenty-one carryable follows.

**"Its verdict on whether they cohere was no."** Holds, at `23:938`, and the located break
is at `23:987-998`.

**"The first vocabulary appears in none of the second and third stretches' surface and
derivation files. It returns zero of seven."** **The command is right and the conclusion
drawn from it is wrong, in a way that changes this file's headline.** Section 0.4 is that
finding and it is the most useful thing here. First the narrow check: I ran the whole
matrix rather than one term over one file set.

```
$ for f in 0*.md 1*.md 2*.md; do
    printf "%-58s binade=%s exponent=%s phase=%s radix=%s totwidth=%s fracwidth=%s\n" "$f" \
      "$(grep -ci binade $f)" "$(grep -ci exponent $f)" "$(grep -ci phase $f)" \
      "$(grep -ci radix $f)" "$(grep -ci 'total width' $f)" "$(grep -ci 'fraction width' $f)"
  done
```

Over the eight files `10` through `17`, which are the surface and derivation stretches:
`binade` 0, `radix` 0, `total width` 1 in `13` and 8 in `15`, `fraction width` 3 in `15`.
So the separation is real and the direction of the brief's claim is right.

But `exponent` is **not** zero. It occurs three times, and every occurrence is a different
word:

- `11:194` "exponentiation", in a list of `GHC.TypeNats` operations.
- `15:173` "a comparison of naturals, not of exponentials".
- `14:612` "the `canonical_exponent` naming debt ... stays dropped".

The first two are noise. **The third is not.** A checkpoint recorded the concept's central
coordinate, under its own ratified name, as a naming debt and then dropped it. `08:206-218`
had established that "the design has ratified a name for a concept it does not carry". So
the separation between the two vocabularies is not only that the surface stretch never
picked up the concept vocabulary; it is that the panel's own checkpoint saw the one term
that bridges them and put it on a droplist.

**And `phase` is not zero either**, and this one is the collision. `16` uses it thirteen
times, `08` eight, and they are different things. Section 4.

### 0.4 The bridge already exists, and the instrument that looked for it could not see it

`23:974-978` tested the concept vocabulary against files `11` through `17`, found nothing,
and concluded the clusters do not join. **The grep is correct and the inference from it is
not**, because it tested one direction over a file set chosen to exclude the two files that
already stand on both sides.

`24_probes/s6_vocabulary_overlap.py` runs both directions over both stretches, counting
occurrences rather than lines, with the checkpoints excluded so they cannot manufacture
overlap.

```
$ python3 24_probes/s6_vocabulary_overlap.py
concept side : 6 files  ['02', '03', '06', '07', '08', '18']
surface side : 7 files  ['10', '11', '12', '13', '15', '16', '17']

total width                  11       11   IN BOTH, candidate collision
fraction width                5        3   IN BOTH
integer width                25       14   IN BOTH
phase                        30       13   IN BOTH
```

**The width vocabulary appears eleven times on the concept side**, and almost all of it is
in `06` and `07`. `06:599` writes "Two values at that step is total width 1 with `F = 2`, so
`I = -1`" in the same file that writes, at `06:562`, "the product's maximum lands a whole
binade below the formula's". `07:626-627` sweeps a float family "against the float exponent
span with the fixed-point box held at total width 5".

**So the brief's statement that the two vocabularies have never appeared in the same
sentence is false, and `07:627` is the counterexample.** They have appeared together, in two
files, and the panel's own instrument for detecting it was pointed at a file range that
excluded both.

That is worth more than a correction, because of what those two files say next.

**`06` states the scope qualifier this file was dispatched to derive, about itself, before
this file existed.** `06:450-452`:

> *This appears to hold*, over one family, at one radix, with zero bias, for four sites. I
> did not test the ranged family, nonzero bias, the closed-interval adjustment the record
> names for normalised channels, or any radix but two.

And again at `06:680-686`, called "the largest bound, and it is large": "Every Python
instrument is unsigned fixed-point, at radix 2, with zero bias, in one family."

Compare that to the six restriction conditions section 2.2 derives from the vocabulary:
radix two, canonical exponent constant, adjustment one, bias zero, phase zero, reach a
power-of-radix count. **`06` names five of the six**, missing only phase, and it names them
under different labels reached by a different route: it asked what its instruments covered,
where I asked what the coordinates can express.

Two consequences, and they pull in opposite directions.

**The map in section 2 is better supported than it looked.** It is not one expert deriving a
restriction set from a vocabulary. It is that plus `06`, months of panel-time earlier,
reaching five of the same six conditions from an instrument-coverage audit. Under `RULES.md:116`
that is a second independent instance rather than a second expert, since I read `06` by grep
before finalising, and I am rating it as an instance.

**And the defect is not where the brief and `23` put it.** The qualifier is not missing from
the panel. It is stated twice in `06`, flagged there as that file's largest hole, and then
**dropped by everything downstream of it**. `15`, `16` and `13` all build on `06`'s
territory and none restates the restriction. That is a compression failure rather than a
gap, and the remedy is different: a gap wants a derivation, and a dropped qualifier wants
restoring from the establishing source, which `RULES.md:159` says in as many words.

I have left section 2 as derived rather than rewriting it around `06`, because a derivation
that reaches an existing answer independently is worth having on the record as an
independent derivation. What changes is the verdict's framing and section 6's.

**"`mock/crates` is being nuked and is not evidence."** Holds, and I did not read it. What
I did read from the repository is `seed/SETTLED_laws.md:274` for the affine value map,
because `08` cites it as the establishing source for its own vocabulary and
`RULES.md:159` requires a repair to restore from the establishing source rather than from
the document that carried it.

## 1. The two vocabularies, from their own sources

### 1.1 Vocabulary A, the concept

`08:175-186` starts from the record's affine value map at `seed/SETTLED_laws.md:274`: the
value of a stored integer `k` is `A * r^ε * k + B`, with the note that `A` and `B` cannot
be folded together because one changes the spacing and the other moves the origin. As a
set,

$$V(N) \;=\; \bigl\{\, A \cdot r^{\,\varepsilon} \cdot k + B \;:\; k \in K \,\bigr\}$$

`08:190-192` then does the move that makes it a concept rather than a formula. Group the
values by radix-binade. Inside binade `e` the step is `A \cdot r^{f(e)}` for an integer
`f(e)`, and `f` is Flocq's canonical exponent. The family is the shape of `f`: constant is
fixed point, `f(e) = e - p + 1` is float, constant below a knee and slope one above is
gradual underflow, slope two or more somewhere is a taper.

So an A-numeral, written out with nothing left implicit, is

$$\bigl(\, r,\; A,\; B,\; \phi,\; f,\; K \,\bigr)$$

a radix, an adjustment, a bias, a phase, a canonical exponent function, and an index set.

**The index set is the part `08` deliberately does not develop, and it is the whole of the
seam.** `08:583-584` says so in its own limits section: the boundary sentence "says nothing
about the endpoints, which is the other half of the inclusion test and which I held aside
throughout". The membership predicate at `08:222-224` quantifies over binades and asks
about the step and the phase. It never asks how far the value set reaches.

That is a correct thing to do when the question is membership. A representation is a
numeral or is not, and where its endpoints sit does not bear on that. It becomes a problem
only when a second vocabulary shows up whose first coordinate is exactly the thing held
aside.

### 1.2 Vocabulary B, the widths

`15:105-124` keys a numeral on `(W, F)`, total width and fraction width, with `I = W - F`
a derived view "computed for display and never stored". The argument is measured rather
than asserted: over 6561 product pairs, `I < 0` at 15 and `W < 0`, `F < 0` at zero, so the
pair `(W, F)` carries the whole negative-width region in naturals with no signed ladder.
`15:117-125` pushes it: repeated squaring of `U<0,1>` drives `I` to `-31` in five steps
while `W` stays at 1.

There is no radix in this vocabulary. No adjustment, no bias, no phase, and no canonical
exponent. There are two naturals.

### 1.3 The asymmetry, stated before the map

Each vocabulary has something the other does not, and they are not the same kind of thing.

A has the grid in full generality: the exponent form is a function, the radix is a
parameter, the adjustment and bias and phase are free. It does not have the reach.

B has the reach, and it has the grid collapsed to a single natural. It does not have the
exponent form as anything variable.

**So this was never a translation problem.** Two vocabularies that describe the same object
have a translation between them, and a missing translation is a defect. These describe
overlapping parts of one object, and what is missing is the sentence saying which part each
one is. That sentence is cheaper than a translation and it is what a canon is for.

## 2. The map

### 2.1 From a width pair to a concept numeral

Take `r = 2`, `A = 1`, `B = 0`, `φ = 0`, `f ≡ -F`, and `K = [0, 2^W)`. Then

$$V \;=\; \bigl\{\, 2^{-F} k \;:\; k \in [0, 2^{W}) \,\bigr\}$$

and the three coordinates line up:

**`F` is the negated canonical exponent.** The step is `2^{-F}` in every binade, which is
what a constant `f` means. This is the grid.

**`W` is the reach measured in steps.** The magnitudes run over `[0, 2^{W-F})`, and that
interval holds `2^{W-F} / 2^{-F} = 2^W` steps. This is the reach, and it is expressed in
the grid's own units rather than in absolute terms, which is why it is a natural and stays
one.

**`I = W - F` is the exclusive top binade.** The reach stops below `2^{W-F}`, so `I` names
which binade it stops at. Negative `I` means the reach lies wholly below one.

That last line is the explanation `15` measured and did not give. `15:127` observes that
under repeated squaring "the total width does not move. One bit holds every one of those
values, because two values need one bit however far below the point they sit." Under the
grid-and-reach reading that is not a coincidence to be observed, it is what `W` counts:
squaring `{0, 1/2}` gives `{0, 1/4}`, the step halved and the reach still holds two steps,
so `F` moved and `W` did not. **`W` counts steps, `F` names the step, `I` says where the
reach landed.** Two of those are storage and one is a view, and the reason the view is the
one that goes negative is that position is the only coordinate of the three that is not
measured in the grid's own units.

### 2.2 Is it total, injective

**From B to A: total and injective.** Every `(W, F)` of naturals maps to an A-numeral, and
distinct pairs give distinct value sets, because `F` is recoverable as the negated step
exponent and `W` as the log of the cardinality.

**From A to B: partial, and the misses are nameable.** A concept numeral presents as a
width pair only when all six of these hold: the radix is two, the canonical exponent is
constant, the adjustment is one, the bias is zero, the phase is zero, and the reach is a
power-of-two count anchored at zero.

`s2` runs the presentability test, which is decidable by construction, over fourteen
representations built from `08`'s own classification table. Every one of them is INSIDE
`08`'s concept.

```
$ python3 24_probes/s2_which_numerals_the_wf_keying_misses.py
presentable: 4/14    not presentable: 10/14
```

The misses, grouped by which A coordinate the pair does not carry:

| miss cause | representations | `08` reference |
|---|---|---|
| canonical exponent not constant | float, float with subnormals, posit-shaped | `08:308`, `08:309`, `08:311` |
| reach count not a power of the radix | Ranged at 12 and at 100 values | `08:307` |
| phase not zero | half-unit-biased | `08:306` |
| radix not two | decimal | `08:310` |
| bias not zero | bias one half | `seed/SETTLED_laws.md:274` |
| adjustment not one | adjustment three | same |

**The first row is the one that matters, and it is not exotic.** The design has floats.
`08:196-198` gives `ExponentForm = Ranged` as the design's own float member, and the
workspace's own crate identity carries `FastFloat` and `StrictFloat`. So the design
contains objects that the width pair cannot name, and every sentence keyed on the width
pair is silent about them.

**The second row is the design's too.** `08:307` classifies a ranged grid whose count is
not a power of the radix as "expressed by `Ranged`, not by the anchored family".

**The third row is the design's own bias axis.** `08:306` calls half-unit-biased "the only
case the `Bias` axis earns", so the axis exists for exactly the case the width pair cannot
carry.

Three of the six miss causes are inside the design, not outside it. That is the concrete
content of the missing scope qualifier.

### 2.3 What this does to `23`'s open questions

`23:993` asked, as the consequence of the unwritten seam, "whether the two-output
derivation applies to a float". **It does not, as stated.** S18 is keyed on `(W, F,
strategy, sign)` and a float has no `F`. That is not a defect in S18; it is a missing
clause, and the clause is one line: the derivation is stated for the constant case.

`23:382-395` recorded the S9-against-S17 collision as unresolvable because "they are in
different coordinate systems". The grid-and-reach reading dissolves it rather than picking
a side, and this is worth stating precisely because `23` explicitly declined to. S9 says
the tightest-numeral computation admits shapes whose integer width is negative. S17 says
the integer width is a derived view. Under grid-and-reach both are true of the same object
and neither is about storage: the tightest computation produces a grid and a reach, the
reach may land wholly below one, and the integer width is the name of where it landed.
**S9 is a statement about reaches and S17 is a statement about coordinates**, and they were
only in tension while "integer width" was read as a thing a numeral has rather than a thing
a numeral's position is called.

I want to be careful about how much that is worth. It does not answer op's question at
`MORNING.md:20` about what a consumer writes, which is a surface question and stays open.
It removes one of the two reasons `23` gave for S9 being unwriteable, and the other reason,
that the vocabulary is unsettled, is what this file is about.

## 3. What compiled

### 3.1 s1: the two readings denote the same sets

`24_probes/s1_grid_reach_agreement.py`, exact `Fraction` arithmetic throughout, no floats.
Two arms. The first builds the value set twice for every `(W, F)` in a box, once from B's
reading and once from A's reading with the six restrictions applied, and asks whether the
sets are equal. The second runs `08`'s membership predicate over a pool containing both
sets that must be accepted and sets that must be rejected, which is what gives the arm
teeth; section 3.4 says why it is written that way and what it cost me.

```
$ python3 24_probes/s1_grid_reach_agreement.py
box: W in [0,10], F in [0,10]  ->  121 numerals
Q1 set equality      : 121/121

Q2 08's predicate over a discriminating pool (accepts AND rejects):
    ok  fixed W=4 F=2                        got accept  want accept
    ok  fixed W=8 F=0                        got accept  want accept
    ok  fixed W=1 F=4                        got accept  want accept
    ok  fixed W=0 F=0                        got accept  want accept
    ok  fixed W=6 F=6                        got accept  want accept
    ok  HUB W=4 F=2 (phase half a step)      got reject  want reject  binade -3: 1/8 is not on the step lattice
    ok  float p=4 e=-3..3                    got reject  want reject  binade -2: step 1/32, expected 1/64
    ok  decimal 2 digits F10=1               got reject  want reject  step 1/10 is not a power of radix 2
    ok  ragged progression                   got reject  want reject  binade 0: 2 distinct steps [1/4, 1/2]
    ok  fixed W=4 F=2, wrong step asserted   got reject  want reject  binade -1: step 1/4, expected 1/8
    -> 10/10 verdicts correct
```

Five rejects, and each fires for a **different** clause of the predicate: the phase
lattice, the constancy of the step, the step being a power of the radix, the progression
itself, and the step's value. That is what establishes the predicate has more than one
tooth, rather than one clause doing all the work.

And the `I` reading, printed on the cases that motivate it:

```
  W=1   F=8   I=-7   reach=[0,2^-7) = [0,1/128)   max value 1/256
  W=1   F=32  I=-31  reach=[0,2^-31) = [0,1/2147483648)   max value 1/4294967296
```

`I = -31` at `W=1, F=32` is `15:124`'s fifth repeated-squaring step, reproduced from a
derivation that never uses `15`'s formula. That is a second instance of evidence for `15`'s
table and not a second expert, since I read `15` first.

### 3.2 s3: one definition, four projections, gate-free

The prose above is a hypothesis until something builds. `24_probes/s3_one_definition_two_projections.rs`
declares a numeral as a grid and a reach and nothing else, writes the derivation once
against that declaration, and then supplies four projections: the width pair, a float, a
slope-two taper, and the knee that section 3.5 arrives at.

Feature gates: none. Const generics with associated consts only, no arithmetic in any
bound, so neither `generic_const_exprs` nor `generic_const_args` is reachable. No `dyn`, no
`TypeId`, no `alloc`, `#![no_std]`, `#![forbid(unsafe_code)]`.

```
$ rustc --version
rustc 1.98.0-nightly (57d06900f 2026-05-27)
$ rustc --edition 2021 --crate-type lib --crate-name s3 \
        s3_one_definition_two_projections.rs --out-dir build
$ rustc --edition 2021 s3_run.rs --extern s3=build/libs3.rlib -L build --out-dir build
$ ./build/s3_run
```

The library builds clean on its own, which is the constraint check; the driver is
std-linked scaffolding so the numbers print.

**Check one, the load-bearing one.** The general derivation, which walks the reach's
binades and asks the grid for the step in each, instantiated at the constant case, must
produce `2^W`. Over twelve rows spanning `W` from 0 to 12 and `F` from 0 to 32, including
`W=1, F=32`:

```
width box (12 rows, W in 0..12, F in 0..32): PASS
compile-time asserts: W=4 F=2 -> 16, W=4 F=0 -> 16, W=1 F=32 -> 2
```

The compile-time asserts are a separate `const _: () = { assert!(..) }` block, so three of
those rows are established at type check rather than at run time.

**Check two, the same derivation on families with no width pair.**

```
float p=4, binades -3..3 : 57 magnitudes (7 binades * 8 + zero = 57)
float p=3, binades -2..2 : 21 magnitudes (5 binades * 4 + zero = 21)
tapered slope-two p=4    : 128 magnitudes
```

**Check three, the discriminating fact.**

```
constant grid F=2: f(-2) = -2, f(5) = -2  -> one step exponent, so F names it
slope-one grid p=4: f(-3) = -6, f(3) = 0  -> 7 distinct, so no F exists
```

That is the whole of why a float has no fraction width, stated as a compiled measurement
rather than as an argument: the fraction width is the step exponent, and a slope-one grid
does not have one step exponent.

**What s3 establishes and what it does not.** It establishes that the grid-and-reach
definition is expressible under the design's constraints, that a single derivation written
against it specialises correctly to the design's own quantity, and that the same derivation
reaches families the design's coordinates cannot name. It does **not** establish that this
is the right shape for an implementation, and its names and arities are scaffolding. It
also does not cover the knee, which is stated in the file's own doc comment: the grid
carries `f` as an affine slope and intercept, which covers constant, slope one and slope
two, and gradual underflow is not affine. Section 5.2.

### 3.3 s4: the word both vocabularies already share

`24_probes/s4_two_phases.py` exhibits all four combinations of the value-space phase and
the storage-space phase, which establishes independence rather than asserting it.

```
value phase zero      + storage phase single  : PRESENT  W=8  F=0 anchored
value phase zero      + storage phase cycling : PRESENT  W=13 F=0 anchored
value phase non-zero  + storage phase single  : PRESENT  W=8  F=0 HUB
value phase non-zero  + storage phase cycling : PRESENT  W=13 F=0 HUB
```

Section 4 says what to do about it.

### 3.4 I wrote a probe that could not fail, and repairing it found a defect

`s1`'s second arm, as first written, ran `08`'s predicate only over sets the same function
had just generated as multiples of `2^{-F}`, and then checked that the differences were
`2^{-F}`. It reported 121 of 121 and it was asking whether arithmetic works. A check that
cannot fail is the specific thing the standing test gate names, and I had written one and
was about to report its number.

The repair was to give the arm a pool containing rejects, and the rejects then found a
defect in the predicate rather than confirming it. **Decimal at two digits was accepted at
radix two, and should not have been.** The predicate checked that the step was constant
across a binade and never checked the further requirement in `08:556-558` that the step be
"that radix to some power". The decimal set is a perfectly good arithmetic progression at
step one tenth in every binary binade, so it walked straight through.

Adding the missing clause takes the pool to 10 of 10, and the interesting part is which
representation caught it. `08:230-233` uses decimal for exactly this: "the same decimal
value set is inside at radix ten and outside at radix two", and calls the radix "a
parameter of the question". So the clause I had dropped is the clause that makes `08`'s
own radix-dependence result true, and the instrument only reproduced that result after the
repair.

Two things follow that are worth more than the fix. **A probe over only the objects it
generates is not a check**, and it will report a perfect score, which is the worst
possible failure mode because the number looks like evidence. And **the rejects in a pool
are where the instrument gets tested**, not the accepts: every accept in this arm passed
both before and after the repair, and the whole finding lives in one reject.

I am reporting this rather than quietly shipping the repaired version because the panel's
gate exists for it, and because the same question should be asked of `s2`, which I checked:
`s2`'s presentability test requires the step to have numerator one and a power-of-two
denominator, so it carried the clause `s1` was missing and rejected decimal correctly from
the start. That is luck rather than discipline, and I am calling it luck.

### 3.5 s5: the knee needs two integers, not a list

Section 5.2 below first listed, as the general form's largest cost, that `s3`'s grid
carried the canonical exponent as a single affine piece, which does not reach gradual
underflow. `08:392-396` records that going to the full function space collides with a
refusal of enumeration ratified four times, so this looked like the place the general form
gets expensive.

**It is not, for the shapes the design has.** Every canonical exponent the design names is

$$f(e) \;=\; \max\bigl(K,\; e + I\bigr)$$

for two integers. Constant is the case where the sloped piece never wins over the reach.
Slope one is the case where the floor never wins. The knee is the case where both win
somewhere. That is one more integer than `s3` had, not a list.

```
$ python3 24_probes/s5_knee_without_enumeration.py
Q1: the design's named shapes in the two-integer form   -> 6/6
```

Six shapes, two fixed point, two float, two gradual underflow, all expressible.

**And the closure result falls out of the form rather than being surveyed**, which is the
part worth more than the expressibility.

```
Q2/Q3: closure over a pool of 81 grids, 6561 ordered pairs
   meet (pointwise max) stays in the family : 6561/6561
   the closed form max(max(K1,K2), e+max(I1,I2)) holds : True
   join (pointwise min) stays in the family : 3969/6561
   join leaves the family                   : 2592/6561
```

The meet of two grids is the pointwise maximum of their canonical exponents (`08:625-628`),
and the pointwise maximum of two max-of-affine functions is a max-of-affine function, by
the identity above, which the probe checks at every one of 6561 pairs rather than
asserting. The join is the pointwise minimum and there is no corresponding identity, so it
leaves the family at 2592 of 6561.

**`08` measured that asymmetry and this says why it holds.** `08:603-607` reports the
design's three shapes meet-closed and not join-closed, with the meet closure adding nothing
and the join closure at least doubling. Under the two-integer form that is not a property of
the three shapes, it is a property of the shape of `f`, and it would hold for any design
whose canonical exponents are maxima of affine pieces.

The probe also reproduces `08`'s join result independently. Joining a pure fixed shape with
a pure float gives slope runs `[(1, 55), (0, 25)]`, slope one at the bottom and slope zero
above, which is exactly `08:436-440`'s mirror shape, reached from a construction that never
uses `08`'s pool.

**One thing s5 found that `08` did not have.** The join of two *knees* is worse than the
join `08` measured. Slope runs `[(0, 38), (1, 2), (0, 2), (1, 38)]`, four segments, so it is
neither in the family nor the mirror shape. `08` asked its join question across kinds, so
the within-knee case did not arise. The consequence is that closing under the join does not
cost one more named shape as `08:442` suggests it might; the segment count grows with each
join, and that is where the function-space question genuinely starts.

**What `08` already had, and what this adds.** `08:545-546` frames the affordability
question as "a type-level list ... compared with two integers", so the two-integer baseline
is `08`'s own and I am not claiming it. What is new here is three things `08` does not have:
that the **knee specifically** fits the two-integer form, where `08:198` and `08:204` carry
gradual underflow as a separate nested axis and describe the design as "the function space
collapsed to two points, with a third point added separately under a different name"; that
the form is closed under the meet as an **identity** on those two integers rather than as a
survey result; and that the join of two knees is worse than `08:442`'s "one more named
point, symmetric to `Underflow`". The first of those removes an axis the design currently
carries separately, which is a simplification rather than a generalisation.

**Compiled too.** `s3`'s grid now carries a `FLOOR` and its `canonical_exponent` is the max
of the affine piece and the floor. Still no feature gates, still `no_std`, and the knee
projection goes through the same magnitude walk as everything else:

```
check 4: the knee, carried by the same definition with one more integer
         KneeGrid<p=4, knee at binade -6>: f(-9)=-9 f(-7)=-9 f(-6)=-9 f(-4)=-7 f(0)=-3
         constant below the knee: true   sloped above: true
         magnitude count through the same walk: 87
```

Constant at `-9` through the taper, then rising one per binade above the knee. No new
machinery was written for it: one associated const, and the derivation was untouched.

**What this does not establish.** It says nothing about slope two, which `s5` did not put in
the family and which needs a third piece or a different form; posits stay outside on this
reading as they do on `08`'s. It says nothing about compile time, which `08:545-550` names
as the measurement that would decide affordability and which is still **unpriced**. And the
`FLOOR` spelling in `s3` is scaffolding: a real design might carry the knee as a binade
rather than as a floor on `f`, and the two differ by an offset I chose to make the arithmetic
short.

### 3.6 s6, and a probe that reported zero because it was broken

`24_probes/s6_vocabulary_overlap.py` produced section 0.4's finding and section 4's
measurement. It is described where its results are used rather than here.

What belongs here is how it started. The first version was a shell script, and it reported
**zero** for every term in both vocabularies, including `phase`, which I had already
measured at 30 and 13 with a different command minutes earlier. The cause was an `IFS`
change that broke glob expansion, so every `grep` ran against no files at all.

I caught it only because I had a known answer to check against. A zero from a broken command
is indistinguishable from a zero from a working one, and this one would have supported the
brief's premise rather than contradicting it, so the failure mode was to confirm what I had
been told and stop. `RULES.md:124-126` requires every number to come with its command, and
this is the case that shows why the command alone is not enough: the command was there, it
ran, it exited zero, and it measured nothing.

The rewrite is in Python and asserts its file counts before counting anything, which is the
cheap guard against the same class.

## 4. The phase collision, which has to be settled before a canon is written

The two vocabularies overlap in ten words and collide in one, and it is the worst possible
word for them to collide in, because it appears inside `08`'s membership predicate.

`s6` measures the overlap rather than leaving it at what I noticed. Ten terms appear on both
sides: `phase`, `encoding`, `denot`, `total width`, `fraction width`, `integer width`,
`container`, `rung`, `extent`, `width pair`. I read every one. Nine are shared innocently,
either because they mean the same thing on both sides or because one side's use is not
load-bearing. **One is a collision**, and the useful shape of that answer is that a wide
overlap contains a single hazard, so the fix is small and locating it needed a measurement
rather than a hunch.

**Value-space phase.** `08:222-224`: "are the denotable magnitudes an arithmetic progression
whose step is one adjustment times a power of `r`, **all at one phase**". `08:306` uses it
for half-unit-biased, "progression, phase half a step". This is a property of the grid,
fixed by the numeral's type, and it is one number per numeral.

**Storage-space phase.** `16:178-179`: "Its phase within a byte is `13k mod 8`, and since
thirteen and eight are coprime the phase cycles through all eight residues." This is a
property of an element's index within a packed run. `16:186` gives the load width as a
function of `W` alone precisely because the phase is unknown, and `16:399` verifies a closed
form "over all eight bit-phases for every width 1 to 1024".

They are independent, all four combinations occur, and `16`'s use of the word is the one
that does real work in the derivation, since the whole load-type finding at S20 turns on
it.

**This is not a matter of taste and it is not cosmetic.** A canon sentence of the form "the
numeral's phase determines the load type" would be false under one reading and true under
the other, and a reader has no way to tell which was meant. `23:1057` already records that
the panel has compressed a ratified sentence wrongly once and it cost an expert an hour.
This is the same hazard with a shorter fuse, because both readings are in the panel already
and both are correct in their own file.

**What I would do, and it is a suggestion rather than a call.** Keep **phase** for the
value-space notion, because it is the one with literature behind it and the one inside the
membership predicate, and rename the storage-space one to **bit offset** or **alignment
residue**. `16`'s own prose already reaches for "bit-phase" as a compound at `16:373`,
`16:399` and `16:668`, which is the file half-noticing the collision without naming it.

The cost of the rename is small and falls on one file's vocabulary. The cost of not doing
it falls on every future reader of the canon.

## 5. Which vocabulary the canon should carry

`23` framed the choice as one vocabulary, the other, or a third that subsumes them. I think
that framing is right and the answer is the third, with a qualification that stops it being
a rewrite.

### 5.1 The argument

**One definition, projected, rather than two definitions maintained.** The failure a canon
with two definitions of a numeral produces is not that a reader is confused on first
reading. It is that the two definitions drift, each in the direction of the work being done
against it, and nobody notices because each is locally correct. That has already happened
here, in one night, within a single panel: `08` developed the grid and held the reach
aside, `15` developed the reach and collapsed the grid to a natural, both were right, and
`14:612` dropped the term that would have connected them.

So the canon defines a numeral once, as a grid and a reach, and states the width pair as
what that definition is called when the canonical exponent is constant. The width pair is
then **derived** and cannot drift from the definition, because there is nothing to drift
from it to.

**Keeping the width pair is a result and I want to be explicit that I am keeping it.**
`15`'s case for `(W, F)` over `(I, F)` is measured, it survives my derivation, and my
derivation explains it: `W` counts the reach in the grid's own units, which is exactly why
it stays natural where `I` does not. Nothing here proposes renaming it, and a proposal to
rename it on the strength of this file would be the vocabulary churn `RULES.md:99-101`
warns about. What changes is its status, from definition to projection.

**And the general form has to be stated in the canon rather than left implicit**, because
of the permanence test. "A numeral is a total width and a fraction width" fails permanence
the moment the design's own float family is considered, and the design has one today. A
canon sentence that is false about `FastFloat` is not a canon sentence.

### 5.2 What it costs, honestly

**The knee cost turned out not to be a cost, and section 3.5 is the attack that
dissolved it.** I listed it here first, then went at it, and the result is that every
canonical exponent the design names is `max(K, e + I)` for two integers, compiled gate-free
into the same definition, with the meet closure falling out as an identity. So the general
form reaches the design's whole admitted set for the price of one associated const over the
constant-only form, and `08:392-396`'s enumeration collision does not arise here. It arises
one step further out, at slope two and at closing under the join, and section 3.5 says where.

What survives as a real limit is narrower: the canon can define a numeral as a grid and a
reach without deciding **which grids the design admits**, and it should, because the
admitted set is a separate question with its own open edges. `08:580-584` already draws that
line for its own sentence. The definition is the shape; the admitted set is a statement
about which shapes, and today that is the two-integer family, with posits outside it.

**The second cost is that the canon then owes a sentence about the reach**, which `08`
held aside on purpose and nobody has picked up. `s3` carries `EMIN`, `EMAX` and a separate
`HAS_ZERO`, and the separation of the zero flag from the binade bounds is a scaffolding
decision of mine rather than a finding. Whether zero is a reach question or a denotation
question is genuinely open and I did not settle it.

**The third cost is a rewrite cost I cannot price.** Whether stating the general form
changes anything in the derivation work is a question about artifacts I did not read, and
under `RULES.md:119-122` nothing has measured it. **Unpriced**, and I am using the word
rather than reaching for an impression.

### 5.3 The alternatives I did not take, described so the next expert starts from them

**Carry only the width pair and scope it.** The canon says a numeral is `(W, F)` and adds
that floats are a separate object. Cheapest, and it is what the design does today by
accident. It fails the equivalence test: two teams reading it would build fixed point and
floats as unrelated types, which is the opposite of the founding premise `08:586-588`
confirms, and it wastes `08`'s whole result.

**Carry only the concept and derive the widths at use.** The canon says a numeral is
`(r, A, B, φ, f, K)` and never mentions widths. Purest, and it is what `07`'s frame would
suggest. I did not take it because the width pair is what a consumer touches and a canon
that does not name the thing a consumer writes has described the machine and not the tool,
which is `23:1059`'s own complaint about the inventory.

**Carry both as co-equal with a stated translation.** A translation table in the canon
between the two coordinate systems. I rejected this on the deduplication ground in 5.1: a
translation between two definitions is a thing that has to be maintained, and the drift it
is supposed to prevent is exactly the drift that produces a stale translation.

**Make the reach the primitive and the grid derived**, keying on `(EMIN, EMAX, count)`.
Symmetric to the chosen shape and I spent some time on it. It is worse for one concrete
reason: the count of magnitudes in a reach is a sum over binades under a general `f`, so
the count is derived from the grid rather than the other way round, and the coordinate that
would be primitive is the one that needs the other to compute. Keying on the grid and
measuring the reach in its units is the direction where both are cheap.

## 6. Is this an incompatibility

The brief said to stop and report if joining the two exposes a genuine incompatibility
rather than a translation, because that would mean two parts of the panel have been
describing different objects.

**They have not.** The objects agree exactly where both are defined, at 121 of 121 on set
equality in exact arithmetic, and `08`'s own membership predicate accepts every width pair
at 121 of 121. There is no case in the box where the two readings disagree about a value.

**But the brief's phrasing is one word off in a way worth stating.** They are not
describing different objects and they are not describing the same object either. They are
describing **different amounts of the same object**, and the panel has been reading a
partial description as a total one. That is not an incompatibility and it is not a
translation. It is a missing scope qualifier, and the reason it has been invisible is that
a partial description of an object reads exactly like a total one when nothing outside its
domain has come up.

The cost so far is bounded and I want to say what it is rather than leaving it as a shape.
Everything in `23`'s Cluster D is stated without the qualifier: S17, S18, S19, S20, S21 and
S22 all read as statements about numerals and all are statements about the fixed-point
ones. None is wrong. Each would be wrong if a reader applied it to a float, and a canon is
exactly the document a reader applies to everything.

**And section 0.4 changes what kind of failure that is.** `06` wrote the qualifier down, at
`06:450-452` and again at `06:680-686`, and called it its own largest hole. Everything built
on `06` dropped it. So this is not the panel failing to notice a restriction; it is a
restriction stated once at the establishing source and lost in every compression after it,
which is the failure `a-compression-is-checked-by-someone-else` describes and which this
panel has already paid for four times by `21`'s count.

The practical difference is in the remedy. A gap wants somebody dispatched to derive the
answer, which is what this dispatch was. A dropped qualifier wants restoring from the
establishing source, per `RULES.md:159`, which is cheaper and which nobody has been asked to
do. **I would put the restoration ahead of anything else in section 7**, and I have not
reordered section 7 because the restoration is not mine to perform: it touches five files I
read only by grep.

### 6.1 The commit log, read after the verdict was on disk

`RULES.md:217-225` forbids reading it first and does not forbid reading it at all. I read
it after sections 0 through 5 were written, to check whether anything in it contradicts
what I derived.

```
$ git log --oneline -30 -- mock/research/202608072330_the-numeral-canon-panel/
```

Thirty subjects. **One names the seam**, `10311e2` "research: record the canon inventory,
the unjoined seam, and this file as a contamination source", which is `23`'s own commit
and names the gap rather than any answer to it. **None mentions grid, reach, binade,
canonical exponent, total width or fraction width**, checked with a grep over the subject
list rather than by reading. So the log carries the existence of the gap, which the brief
already told me, and carries nothing about how it closes.

Nothing above changed. What the log did add is a small confirmation of the rule that
forbade reading it: `2e6d72c` is titled "close the commit-log leak", which is the incident
`RULES.md:217-225` was written from, and it is two files after the leak happened. The
subjects here are informative enough that reading them first would have told me a
predecessor had already located the seam and, from `d84b859` and `b3ecaf9`, that its counts
had been corrected twice. Neither would have changed the derivation, and both would have
made the derivation feel less like mine.

## 7. Where I would go next, in order

**Restore `06`'s scope qualifier into everything downstream of it.** Section 0.4. `06`
states it twice and `15`, `16` and `13` all build on `06`'s territory without it. This is
restoration from an establishing source rather than new work, it is the cheapest correct
thing available, and until it is done every Cluster D sentence reads wider than it is. I did
not do it because it touches five files I read only by grep, and a restoration performed
from a grep is how a qualifier gets restored in the wrong words.

**Settle the phase collision.** Nearly as cheap, entirely mechanical, and it blocks nothing
until someone writes a canon sentence with the word in it, at which point it blocks
everything. `s6` measures the whole overlap and finds ten shared terms of which this is the
only genuine collision, so the fix is bounded: one decision and one file's vocabulary.

**Second-read this file's map**, with the order inverted per `RULES.md:208-212`, because
everything here is one expert. The specific claim I would most want derived independently
is `W` counting the reach in steps, since it is the claim the joining sentence turns on and
it is the one that reproduces `15`'s table from a different direction. If a second expert
reaches it without reading this file, the joint moves to two experts and `23`'s missing
spine has a rung under it.

**Take the reach question `08` held aside.** What the index set is, whether zero is a reach
property or a denotation property, and whether the reach has to be an interval at all.
`08:583` names it as the other half of the inclusion test and nobody has picked it up. It is
now also the half that the width vocabulary is entirely made of, which makes it more
load-bearing than it looked when it was set down.

**Ask whether the design admits a non-affine grid**, which is `08:390-419`'s question and
is upstream of how much generality the canon's definition needs. I did not attack it
because it is `08`'s open question rather than mine and re-deriving it here would be a
second answer to a question that already has one waiting for a second read.

## 8. What I did not cover

**Read in full:** `RULES.md`, `01`, `04`, `23`, `08` sections 1, 2, 4 and 5, `15` sections
1 and 2.

**Read at the passages bearing on the map:** `16` sections on the bit phase and the load
type, `13` at its total-width occurrence, `11` and `14` at their `exponent` occurrences.

**Read only by grep, then at the passages the greps turned up:** `06` (its scope-qualifier
sections and its width-vocabulary uses), `07` (its float sweep). Both turned out to matter
more than the brief suggested, which is section 0.4, and I want to be clear that I found
them by measuring the overlap rather than by reading them, so what I know about those two
files is what `s6` pointed me at.

**Read only by grep:** `02`, `03`, `10`, `12`, `17`, `18`, `19`, `20`, `21`, `22`,
`SETTLED.md`, `seed/`.

**One citation here is second-hand and I am marking it as such.** `MORNING.md:20`, for op's
question about what a consumer writes, is taken from `23:1033` and `15:697-707`. I did not
open `MORNING.md`, because the brief excludes it as a contamination source, so I cannot
confirm that line number and I am not claiming to. Everything else in this file resolves
against a file I opened: `24_probes/citations.out` records 65 citations, 51 unique, zero
out of range, and I content-checked the ten I had taken from another file's account rather
than read, of which zero were wrong.

**Never opened:** `CANON_CANDIDATE.md`, `DROPLIST.md`, `PERSONA_CALLS.md`, `MORNING.md`
(excluded by the brief), every probe directory except my own, and all of `mock/crates`.

**What that costs, specifically.** I did not verify `08`'s classification table against
`08_probes/i1b.out`; my `s2` rebuilds representative value sets from `08`'s prose
descriptions rather than reading its probe outputs, so if `08`'s table misclassifies a row,
`s2` inherits it. The rows I would check first are decimal and Ranged, because both are
cases where `08`'s own text notes the answer depends on which radix the question is asked
at.

I did not check whether the grid-and-reach framing contradicts anything in `seed/` beyond
the one line at `SETTLED_laws.md:274` that `08` cites. `23:1179-1182` flags that three of
the night's four compression defects were found by reading `seed/` against a summary, and
this file is a summary of two other files, so the same risk applies to it and I cannot
bound it.

I did not price anything. No bench harness run bears on any sentence here and none is
claimed to.

I did not attempt the surface question. Whether a consumer writes `(I, F)` or `(W, F)` is
op's at `MORNING.md:20` per `15:697-707`, this file is about the machinery's vocabulary,
and nothing above bears on the surface's choice. If anything, the grid-and-reach reading
makes the surface question sharper rather than answering it, because it says the two
coordinates a consumer writes are naming a grid and a reach and the consumer may reasonably
want to write either the reach's size or the reach's position.

**And the evidence here is largely one instance wearing several hats.** `s1` through `s6`
are six probes by one author on one model in one sitting, and `RULES.md:116` sets the bar at
three independent instances. They are independent in *instrument*: an exact-rational
enumeration, a decidable presentability test, a compiled type-level construction, a residue
argument, an exhaustive closure check and a corpus count. They are not independent in
*derivation*, since all six were built from the same reading of the same two files, so a
mistake in that reading is in all of them.

**The one genuinely independent instance is `06`.** Section 0.4: it reaches five of the six
restriction conditions from an instrument-coverage audit rather than from the vocabulary,
and it did so before this file existed and without any of it in view. That is the second
instance, and the honest count for the restriction set is two instances and one expert.
Everything else here is one of each.

## 9. What is op's

One thing, and it is not the seam sentence.

**Whether the design admits numerals its coordinates cannot name.** The design contains a
float family (`08:196-198`), a `Ranged` member whose count need not be a power of the radix
(`08:307`), and a `Bias` axis that exists for the one case with a non-zero phase
(`08:306`). All three are outside the width pair. A canon can say the numeral concept is
general and the width pair names the constant case, or it can say the design's numerals are
the width pair and the other three are separate objects. Both are coherent and they produce
different designs.

That is a question about what the design is for rather than about what is true, so I am not
proposing an answer. What this file adds to it is that the question is now measured: the
misses are 10 of 14 with six named causes, and three of the causes are already inside the
design.

Everything else here wants a second expert, not op. Per `01` section 0 and `04`, there is
nothing converged to bring him.
