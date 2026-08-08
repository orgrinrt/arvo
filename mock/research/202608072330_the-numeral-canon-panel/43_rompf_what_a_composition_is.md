# 43. What a composition is, and what the canon owes it

**Date:** 2026-08-08. **Author:** the `rompf` persona. **Predecessor:** `42_willsey_the_law_layer.md`.
**Probes:** `43_probes/`. **Mode:** explore, do not settle (`00_brief.md`, `04`, `28`).

**Status: COMPLETE.** Written to disk on arrival and extended in place, per `RULES.md:207-208`.

## 0. The answer, before the working

**The word is overloaded, and one of its two senses sits in the panel's founding sentence.** In the
predecessor panel's vocabulary a **composition** is a numeral kind bound to a strategy, `Number<N, S>`,
and op ratified a call about exactly that object: "Compositions are public and bindable by anyone;
semantic names and strategy presets are the default documented path, not the only path"
(`archive/CANON_CANDIDATE.md:3386-3387`). In op's `32` a **composition** is an aggregate: "the contracts
for things that compose to bigger units than just numerals alone" (`32:73`). `00_brief.md:8` says "the
primitives become named compositions over one format concept", which is the first sense, and my dispatch
reads it as the second. Section 3 measures the collision. It is `24`'s phase collision again, one word
further in, and this time the word is in the sentence the panel was founded on.

**In the aggregate sense, a composition is a binding-time distinction rather than a container.** Its
static part supplies the quantities a numeral's own type cannot carry, because they are facts about a
**run** rather than about a **value**: a stride, a capacity, a traversal order, a shared scale. Its
dynamic part is the run. Three files have each found one instance of this and none named the shape.
`16`, through `OPTIONS.md`'s account, finds the container derivation has a per-value output and a
per-aggregate one, and that `Cold` "has no standalone value form at all". `35:41-43` finds the fold
accumulator is "a function of the element width and the **capacity**, which is a quantity that lives in
the composition rather than in the numeral". `08:287-290` finds that "no per-datum type can express a
constraint that holds between data". Those are one sentence written three times.

**The defining boundary is capacity against count.** A composition's capacity is static and its length is
dynamic, with `len <= capacity` as its own invariant. `35_probes/p1` is that boundary crossed wrongly:
four formulations of a widening fold refused, with the wall located, in `35:97`'s words, at "the runtime
trip count and nothing else". The escape `35` found, that a capacity is a type, is not a trick. It is the
observation that the **bound** was known at compile time all along and had been classified as runtime
because the **count** is. Naming that as the composition's defining boundary is what stops the same
misclassification happening again on the next quantity.

**No derivation in the panel reads a numeral's grid, and that is a binding-time fact with a
consequence.** `43_probes/p2` compiles the check: the carrier and the fold accumulator are literally the
same types across numerals differing in adjustment, bias, phase and canonical exponent, and the law bound
is satisfied or refused by the strategy alone at every grid. Three negative controls confirm the checks
are not vacuous. So **the grid coordinates are the ones a composition may hold at run time**, shared over
a run, with every derivation unchanged. That is what frame-of-reference, delta and shared-scale column
encodings are, and they are the four representations `08:282-290` puts outside the numeral on its locus
clause while saying at `08:292-294` that the class is "what that workload actually uses".

**But the operations do not all agree with the derivations, and the split is exact.** `43_probes/s5`
measures it in exact rational arithmetic. A same-grid **addition** reads neither the adjustment nor the
canonical exponent, so those two are free to be dynamic and a shared-scale sum fold costs nothing. A
**multiplication** reads the canonical exponent, which is the rescale, so a dynamic exponent costs one
runtime shift per multiply. And the **bias** is in neither set: a raw addition does read it, a sum's
effective origin is `n·B` with `n` the dynamic trip count, and at nonzero bias the product's derived grid
step stops being a function of the operands' coordinates at all, matching the predicted `A²r^{2e}` at 8
of 18 grids against 9 of 9 at zero bias.

**Compositions nest, and the nest must be flattened before the derivation runs.** `43_probes/s3` measures
exhaustively that composing the per-level accumulator derivations is sound and not tight: one bit wide on
1201 of 4096 two-level shapes and up to two bits at three levels, with zero shapes where flattening is
worse and a negative control at 6502 overflows one bit narrower. `43_probes/p4` compiles the flattened
form gate-free, asserts the one-bit gap at `s3`'s witness at compile time, and shows the flattened
capacity is a type the derivation consumes.

**A numeral is not a degenerate composition, and the degeneracy that does hold is worth having.** The
accumulator derivation degenerates correctly at capacity one, checked at compile time in `p4`, and
nesting a capacity-one level is the identity over every shape `s3` checked. What does not degenerate is
everything else: a composition has a length, an index and a traversal order, and a numeral has none of
them and no place to put them. So the canon has **two** concepts and owes the relation between them,
and the relation is cheap, because it is one staging discipline applied to two different static
descriptions.

**Whether compositions belong in arvo is two questions with different answers.** The **contract** must be
arvo's, for the same reason `42:33-38` gives for the law layer: it is the second input to derivations arvo
already owns, and a consumer re-deriving it is `35`'s wrong-answer classes going undetected at compile
time. Whether arvo also ships concrete **containers** is separate, I did not settle it, and section 8.2
states what would decide it.

## 1. The gates

**Canon gate: passed, and there is nothing to defend.** There is no ratified canon. The fixed set is op's
own files (`01`, `04`, `28`, `32`, `34`, `36`, `37`, `38`, `39`), the workspace discipline, and the
forbidden-feature list, and I checked my work against all of them plus
`~/Dev/clause-dev/.claude/rules/unstable-features.md`. Every Rust probe compiles on the pinned
`nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, with no feature gate of any kind:
`grep -c '#!\[feature' 43_probes/*.rs` returns 0 on both files, recorded in `43_probes/RUN.md` alongside
the toolchain banner.

**One ambiguity, handed back rather than resolved, per the gate's ambiguity clause.** Section 3 finds the
panel's founding sentence uses "composition" in a sense my dispatch does not. I have not resolved which
sense the canon should carry, because that is a question about what the design is rather than about what
is true, and both senses have op's word attached to different objects. I answer the dispatch's question
under the aggregate sense, mark every place the other sense would answer differently, and say in section
10 what the register should gain.

**Test gate: run, and the honest report is a place rather than an absence.**

```
$ cd mock && cargo test --workspace
error: manifest path `.../arvo/mock` contains no package: The manifest is virtual,
       and the workspace has no members.
$ ls crates | wc -l
       0
```

`mock/crates` was emptied on 2026-08-08 and stays empty, so the suite is empty by construction, there is
nothing to audit and nothing for the gate to refuse. I applied the gate's spirit to my own probes
instead. Every one carries a negative control, and `p2` carries three, because the checks it performs
are type-equality assertions and a type-equality assertion whose mechanism is vacuous reports a perfect
score. `--cfg sameneg` is the arm that proves `SameAs` refuses two genuinely different types; without it
every equality check in `p2` would be measuring nothing, and that failure mode has already happened once
in this panel (`24` section 3.4, a probe that "could not fail" and reported 121 of 121).

And one hypothesis of mine was refuted by my own probe and is kept on the record rather than rewritten:
`s5`'s first version asked whether a product's **result set** is a uniform grid, got 0 of 27 at every
bias, and I nearly reported that as a bias finding. It is a fact about squares. The corrected question is
in `s5` Q5 and its answer is the one worth having.

## 2. Method, and what I refused to count

I derived the shape from the binding-time structure of the panel's own derivations, then built
instruments. Four probes, independent in kind rather than in wording: a corpus census over the panel's
own prose (`s1`), exhaustive integer arithmetic (`s3`), exact rational arithmetic (`s5`), and the
compiler (`p2`, `p4`).

**I did not read `mock/crates`**, which is empty, and I did not go to git history for it. I read
`mock/benches/` not at all, because nothing in this file is a magnitude and `RULES.md:224-228` names that
directory as the thing that prices, which is not what I am doing. Where a magnitude would decide
something I say it is **unpriced**.

**I read the archived consolidation, and only to establish a vocabulary fact.** `archive/README.md` and
`RULES.md` mark `archive/CANON_CANDIDATE.md` as greppable and never citable as authority, and its own
header says it is "never a citation". I cite it once, at `3386-3387`, for what the word "composition"
**meant** in the record this panel inherited its founding sentence from. That is a fact about a document,
not a claim about what is correct, and it is the only kind of claim an archived document can support.
Nothing else in this file rests on it.

**One thing I refused to count as corroboration.** `35` section 3.8 and `16`'s two-output finding both
agree with my section 3, and I read both before deriving. Under `RULES.md:116-118` that is inherited
agreement. What is independent is the measurement: `p2` and `s5` ask a question neither file asks, about
which coordinates are read rather than which outputs exist.

## 3. The word is overloaded, and the collision is in the founding sentence

`43_probes/s1` extracts every occurrence of `composit*` / `compose*` in the panel directory, in its
instruments, in the archived consolidation and in the seed sweeps, with three lines of context, and
classifies each by which sense the context forces. Every occurrence is printed so a reader can disagree
with a classification rather than trusting a count.

```
GRAND TOTAL
  AGGREGATE  47
  BIND       37
  FUNC       50
  UNCLEAR    105
```

**The counts are the weakest part of this section and I am not leaning on them.** The classifier is a
keyword heuristic and 105 of 239 occurrences defeat it. What the counts establish is only that all three
senses occur in quantity, across two panels, with no file disambiguating.

**The instances are the finding.** Three, each verified by opening the line.

**The BIND sense, with op's word attached.** `archive/CANON_CANDIDATE.md:3386-3387` records D52, op,
2026-07-30: "Compositions are public and bindable by anyone; semantic names and strategy presets are the
default documented path, not the only path", citing `arvo-toolbox-not-policer.md` as independently
deciding it "because sealing the composition would be arvo choosing which combinations a consumer may
want". The object is `Number<N: Numeral, S>` (`archive/CANON_CANDIDATE.md:802-805`). A composition is a
numeral kind bound to a strategy, and a preset is a name for one.

**The AGGREGATE sense, with op's word attached.** `32:73`, verbatim: "the contracts for things that
compose to bigger units than just numerals alone". `32:82-83` reads it as "Composition to units bigger
than a numeral is a first-class concern, named in the same breath as the algorithm crates."

**And the founding sentence, which is neither disambiguated nor disambiguable from its context.**
`00_brief.md:8`: "the primitives become named compositions over one format concept". Read under BIND it
says: `UFixed`, `IFixed`, `FastFloat` are named bindings of a numeral kind to a strategy, over one
concept of format. That reading is coherent, it is what the inherited vocabulary means, and it is almost
certainly what the sentence was written to say. Read under AGGREGATE it says something about vectors and
matrices, which is what my dispatch reads it as. `s1`'s classifier, given three lines of context around
that exact sentence, classifies it FUNC, which is wrong under both readings and is the cleanest
demonstration available that the sentence does not carry its own sense.

**Why this is not cosmetic, in `24`'s own terms.** `24` section 4 found the two vocabularies share ten
words and collide in one, `phase`, and said: "A canon sentence of the form 'the numeral's phase
determines the load type' would be false under one reading and true under the other, and a reader has no
way to tell which was meant." The same sentence with `composition` for `phase` is worse in two respects.
The word is in the founding statement rather than inside a predicate, so it scopes the whole document.
And both senses have an op statement behind them, so neither can be renamed by an agent's preference.

**What I would do, offered as a suggestion rather than a call, and not the rename it looks like.** The
two senses are not rivals; they are a **binding** and an **aggregation**, and the second is what op's
`32` names as a first-class concern in the same breath as the algorithm crates. Keeping `composition`
for the aggregate and finding another word for the binding costs a rename of a term whose home document
is archived. Keeping it for the binding and finding another word for the aggregate costs a rename of a
term op used in a live file two days ago. That asymmetry points one way and I am not going to pretend it
decides anything, because the founding sentence uses it the other way and a panel that renames its own
founding sentence's noun should do it deliberately rather than as a side effect of one file.

**The rest of this file is about the AGGREGATE sense**, which is what my dispatch asked and what op's
`32` names. Where a finding would read differently under BIND I say so.

## 4. What a composition is

### 4.1 Three instances of one shape, none of which named it

The panel already holds three findings about compositions. Read together they are one sentence.

**`16`'s second output.** Reaching me through `OPTIONS.md`'s account rather than the file, which I did
not read: the container derivation has **two** outputs, a carrier and a stride, and the framing
`OPTIONS.md` quotes is that "the derivation answers a per-value question and a per-aggregate question".
`OPTIONS.md`'s same section records `Cold` as "not a container choice with a field attached; it is a
statement about how a run of values composes, and this is why it has no standalone value form at all",
reached independently from both directions by `15` and `16`.

**`35`'s capacity.** `35:41-43`: the accumulator's width "is a function of the element width and the
**capacity**, which is a quantity that lives in the composition rather than in the numeral". `35` section
3.8 lists what a composition needs that a value does not, and its own count of aggregate-keyed outputs is
"at least three: carrier, stride, and the accumulator relation", with the explicit note that it is not
proposing they are one mechanism.

**`08`'s locus clause.** `08:287-288`: "**no per-datum type can express a constraint that holds between
data**", stated as the structural reason block floating point, microscaling, delta and
frame-of-reference encodings are outside the numeral.

**The sentence under all three.** A numeral's type describes a **value**. There are quantities a
computation needs that are not properties of any value: how far apart two of them sit in memory, how many
of them there will be, which of them is first, what scale they share. Every one of those is a property of
a **run**. A composition is the thing whose static part carries them.

That is a binding-time statement rather than a data-structure one, and stating it that way is what makes
the three findings one finding rather than three coincidences. `16` found a quantity that is not
per-value and had to put it somewhere. `35` found a quantity that is not per-value and had to put it
somewhere. `08` found a **class** of quantities that are not per-value and named the layer rather than
the mechanism. None of them had a word for the thing they were putting it in.

### 4.2 The defining boundary is capacity against count

A composition's static part carries a **capacity**; its dynamic part carries a **length**; and
`len <= capacity` is the composition's own invariant, the one thing it guarantees that neither the
numeral below it nor the algorithm above it can.

This is the boundary `35_probes/p1` found the hard way. Four independent formulations of a widening fold
are refused, and `35:96-97` locates the wall precisely: "it composes perfectly well when the arity is a
compile-time fact... **The boundary is the runtime trip count and nothing else.**" Arms A and C compile
because the arity is static; arms B1 through B4 refuse because it is not.

**The escape was not a trick, and calling it one is how the lesson gets lost.** `35`'s answer is that a
capacity is a type, and the reason that works is that the quantity the derivation needs was never the
count. It is the **bound**. A sum of at most `C` values each below `2^W` is below `2^{W + ceil(log2 C)}`
whatever the actual trip count turns out to be. The bound was a compile-time fact the whole time, and it
had been classified as a runtime one because the count is a runtime one and nobody had separated them.

That is the failure mode worth naming in a canon, because it will happen again on the next quantity. A
composition's job is to hold the static half of a pair whose dynamic half is obvious, and the pairs are
easy to conflate: capacity against length, stride against offset, shape against index, shared scale
against per-element residual. In every pair the static half is what a derivation needs and the dynamic
half is what a loop needs, and reading the pair as one quantity puts the whole pair on the dynamic side.

### 4.3 Where a composition's shape is decided

The dispatch asks this directly and it is the part I am actually here for. Six places, and the
interesting content is which quantities sit at which and which pairs straddle two.

**At the canon.** Which kinds of shape exist at all: whether a composition is one-dimensional or has a
rank, whether its capacity is a single number or a tuple, whether a sparse structure's capacity is its
dense extent or its nonzero bound. None of these is decided anywhere in the panel and none of them is a
mechanism question.

**At the type, when a composition is written.** The element numeral, the capacity, the layout, the rank.
This is where `35`'s capacity and `16`'s stride live. `p4` compiles a nest at this stage and derives its
flattened total capacity here.

**At a build arm.** `40:47-53` divides the axes into observable, where moving them changes the
answer, and unobservable, where they change only cost. `40`'s Q13 asks which an arm may move.
**A composition's own shape splits the same way, and nobody has classified it.** Layout and stride are
unobservable: `Cold` versus `Warm` changes bytes and not values. Capacity is **observable**, because it
is an input to the accumulator width and a narrower accumulator changes the answer, which my own
`43_probes/s3` negative control measures at 6502 overflows over 8192 triples one bit narrower. So the
observable/unobservable cut applies to composition-level
coordinates as well as numeral-level ones, and it lands differently: the composition's layout coordinate
is free for an arm and its capacity coordinate is not.

**At the expression, where a consumer writes a fold.** The accumulator type is derived here, from facts
fixed two stages earlier. Nothing new is decided; what happens is that the derivation runs.

**At construction, when a composition is populated.** The shared runtime coordinates, if the design
admits any: a block exponent, a reference value, a dictionary. Section 5 is about which coordinates may
be here.

**At run time.** The length, the index, the actual trip count. This is the set a type cannot reach, and
`35_probes/p1` is what happens when a derivation tries.

**The one thing this table makes visible that the panel's questions do not.** Four of the six stages are
compile-time and two are run-time, and every failure the panel has recorded about aggregates is a
quantity assigned to the wrong side of that line rather than to the wrong stage within a side. `p1`'s
refusal is a static bound read as dynamic. `16`'s carrier-only derivation, per `OPTIONS.md`'s account,
occupying 23.1% more memory than the strategy promises, is a per-aggregate quantity read as per-value.
Both are one-step-across-the-line errors, and neither is a subtle one once the line is drawn.

## 5. What "over one format concept" means for a composition

### 5.1 `08`'s own boundary excludes compositions, deliberately, by its first clause

`08:556-560` offers the boundary sentence: "**A representation is a numeral when a datum denotes one
rational**, when the denotable magnitudes in each binade of some admitted radix form one arithmetic
progression at one phase whose step is that radix to some power, **and when the set is fixed by the type
alone.**"

A composition's datum denotes an indexed family, not one rational, so it fails clause one. `08` says so
about the case it examined: intervals are "outside, and buildable above" (`08:330`), and `08` section
4.5 develops that as a consumer-side composition needing nothing from the numeral that the design does
not already have.

**So "over" in the founding sentence cannot mean "a member of".** Under the aggregate reading, a
composition is not inside the format concept and was never going to be. What "over" can mean is
**parameterised by**: the composition's element is a numeral, the numeral is described by the format
concept, and the composition inherits nothing except its element's description.

That is the cheap answer and it is right as far as it goes. The rest of this section is where it stops
being right.

### 5.2 No derivation reads the grid, and that is compiled

`43_probes/p2` asks which of a numeral's static coordinates each of the panel's derivations actually
consults. The method is type equality: write each derivation keyed only on what it claims to need, then
assert that the derived type is literally the same across two numerals differing only in the grid.

Three grids, all inside `08`'s concept. `GridA` is plain binary fixed point at unit adjustment, zero
bias, zero phase, constant canonical exponent. `GridB` differs in **adjustment**, **bias** and **phase**,
and is the half-unit-biased shape `08:306` calls "the only case the `Bias` axis earns". `GridC` differs in
the **canonical exponent**.

| derivation | keyed on | same type across GridA / GridB? | across GridA / GridC? |
|---|---|---|---|
| carrier (`15`, `16`) | reach, strategy | yes, asserted | yes, asserted |
| fold accumulator reach (`35`) | reach, capacity | yes, asserted | yes, asserted |
| law bound (`40`, `42`) | strategy's overflow axis | satisfied at all three grids | satisfied at all three grids |
| value map | every grid coordinate | **no**, and the negative control proves it |  |

The accumulator arithmetic is checked rather than trusted: `13 + ceil(log2 8) = 16`, `13 + ceil(log2 1) =
13`, `8 + ceil(log2 32) = 13`, all as compile-time assertions.

**Three negative controls, because the mechanism could report a perfect score while measuring nothing.**
`--cfg negcontrol` asserts the value map ignores the grid and fails at `E0080` with the assertion text.
`--cfg lawneg` instantiates the law site at a wrapping strategy and fails at `E0277`, "the trait bound
`Wrap: AbsorbingTop` is not satisfied". `--cfg sameneg` asserts two genuinely different derived reaches
are the same type and fails at `E0277` on `SameAs`. Without the third, every equality check above is
consistent with `SameAs` being trivially satisfiable.

**The consequence.** A coordinate no derivation reads is a coordinate that does not have to be static for
the derivations to run. So the grid coordinates are candidates for a composition to hold at run time,
shared over a run. `p2`'s `Run` type does exactly that, carrying a shared bias as an ordinary field, and
asserts that the derived accumulator reach is unchanged.

### 5.3 The operations disagree with the derivations, and `s5` says exactly where

A derivation running is not the same as a program running. `43_probes/s5` asks the second question in
exact rational arithmetic over the record's own affine value map, `v(k) = A·r^e·k + B`
(`seed/SETTLED_laws.md:274`, quoted at `24:262`).

**Which coordinates a raw same-grid operation reads:**

| operation | adjustment | bias | canonical exponent |
|---|---|---|---|
| add | not read | **read** | not read |
| multiply | **read** | **read** | **read** |

The adjustment and canonical exponent rows for addition are the useful ones and they are the ones I did
not expect to be so clean. Two values on one grid add by adding their stored integers, whatever the
adjustment and whatever the exponent, because both scale out. **So a composition may hold the adjustment
and the canonical exponent at run time and pay nothing at all for a same-grid sum fold.** That is the
arithmetic reason block floating point and microscaling formats work for accumulation, derived here
rather than assumed.

Multiplication reads the exponent, which is the rescale, so a dynamic exponent costs one runtime shift
per multiply rather than a compile-time constant. Real, and bounded, and it is why hardware block formats
do exponent arithmetic once per block.

**And the bias is in neither set, which is the sharper half.** Three measurements:

- A raw addition **does** read the bias. Solving `A·r^e·k + B = v(k₁) + v(k₂)` gives
  `k = k₁ + k₂ + B/(A·r^e)`, so the stored-integer add needs a correction term. Over the box, the raw
  result is an integer index in only 833 of 1323 (grid, pair) cases, and the misses are the biased grids.
- A sum's effective origin is **`n·B` with `n` the dynamic trip count**, not the static capacity.
  Measured at `n = 1, 2, 3, 5, 8`, the effective bias is `B, 2B, 3B, 5B, 8B`. The adjustment and the
  canonical exponent survive a zero-bias sum unchanged over every `(A, e, n)` checked.
- At nonzero bias, the product's derived grid stops being a function of the operands' coordinates. The
  smallest grid containing the product set has step `A²r^{2e}` at **9 of 9** zero-bias grids and at only
  **8 of 18** nonzero-bias grids. The cross term `A·B·(k₁+k₂)` puts `A·gcd(A,B)`-shaped quantities into
  the difference set, so the derived step depends on an arithmetic relation between two coordinates
  rather than on either.

**What that means for a composition.** The coordinates a composition may share **dynamically** and the
coordinates that survive **arity-n** are different sets, and the bias is in the first and not the second.
A composition holding a shared bias is a frame-of-reference column, which is exactly the encoding `08:292`
says the workload uses, and the operation it supports is **difference and comparison**, where the bias
cancels, rather than summation, where it accumulates at a rate the type system cannot see.

I did not expect that and I would not have predicted the sign of it. My prior was that the bias would be
the freest coordinate, because it is the one no derivation reads. It is the freest for the derivations
and the most expensive for the operations, and the two questions had to be asked separately to see it.

### 5.4 So the locus clause is a delegation rather than an exclusion

`08` section 2.3 puts block floating point, microscaling, delta and frame-of-reference encodings outside
the numeral, on the ground that "no per-datum type can express a constraint that holds between data"
(`08:287-288`), and names the layer it hands them to as **storage**.

Sections 5.2 and 5.3 say what that layer would have to be, and it is the composition. The constraint
"every element of this run shares one scale" is a static fact about a run, which is exactly what section
4.1 says a composition's static part is for, and the coordinate it shares is one no derivation reads. So
`08`'s exclusion is correct and its consequence is not that the class is out of scope. It is that the
class **is** the composition layer, described from the numeral's side.

**Two things follow that I would put in front of whoever writes the boundary sentence.**

The boundary's third clause, "fixed by the type alone", is doing two jobs. It excludes a value set that
depends on other data, which is right for a numeral. It also reads, to a later reader, as excluding the
class from the design, which it does not: `08` itself says the layer is storage rather than nowhere.
Splitting those in the sentence costs one clause.

And the design owes a statement about **which** coordinates a composition may share, because sections 5.2
and 5.3 say the answer is not "all of them". A shared adjustment or canonical exponent is free for
addition and cheap for multiplication. A shared bias is free for the derivations, breaks the raw add, and
does not survive a fold as a static fact. If the canon says nothing, a consumer will build the third one
and discover the third fact at run time.

## 6. Does a composition compose further

### 6.1 Capacity composes multiplicatively, and composing the derivations is not the same thing

`43_probes/s3` measures it exhaustively on integers. Two derivations are available for a nest of
compositions, a matrix of rows or a CSR of blocks:

```
NESTED   derive per level and compose:  acc(acc(W, N), M) = W + lg N + lg M
FLAT     flatten the capacity, derive once: acc(W, M·N) = W + lg (M·N)
```

Over `m, n` in `[1, 64]`, 4096 pairs:

```
FLAT strictly wider than NESTED : 0
equal                           : 2895
NESTED strictly wider than FLAT : 1201  (29.3%)
slack histogram (extra bits)    : {1: 1201}
```

At three levels, over 13824 triples, the slack histogram is `{0: 4377, 1: 8471, 2: 976}`, so it
accumulates rather than cancelling.

**Sufficiency, checked directly rather than by the inequality.** Over 8192 `(w, m, n)` triples with every
element at its maximum, the flat accumulator overflows on **0** final totals and **0** per-row
intermediates, so flattening does not cost the nested traversal the intermediates it materialises. The
negative control, one bit narrower, overflows on 6502 of them.

**So composing the per-level derivations is sound and not tight**, and the tight answer is to flatten the
capacity first. That is a statement about **when** the composition's shape is resolved, which is the
question I was sent for: the whole nest's shape must be flattened at the type, before the derivation
runs, rather than the derivation being applied level by level as the type is built up.

### 6.2 The flattening is expressible, gate-free

`s3` is arithmetic and says nothing about whether a type system can perform it. `43_probes/p4` asks that.

Type-level multiplication of capacities is inductive and needs no const arithmetic in any bound:
`_TYPE_LEVEL_PRODUCT_IS_EXPRESSIBLE` asserts `3·5 = 15`, `4·4 = 16`, `1·7 = 7`, `0·8 = 0` at compile
time. `ceil(log2 ·)` is the three-impl induction over a positive binary representation that `35_probes/p8`
established, reproduced here because `p4` needs it and checked against the arithmetic at eight values.

The flattened derivation then runs over the nest's total capacity, and its results are asserted:

```
Flat3x5 = Run<Run<Num<13>, 3>, 5>    flat reach 17   nested reach 18
Flat4x4 = Run<Run<Num<13>, 4>, 4>    flat reach 17   nested reach 17
Single8 = Run<Num<13>, 8>            flat reach 16
Num<13>                              flat reach 13
```

`--cfg tightneg` asserts flat and nested agree at the `3×5` witness and fails at `E0080`, so the two
derivations are genuinely being computed rather than read off one number. `--cfg missingcap` removes the
flattening for one nest and fails at `E0277` on `TotalCap`, so the flattening is load-bearing rather than
decorative.

**What `p4` does not establish, stated plainly.** It carries a general type-level product for the Peano
encoding and states the product per nest for the positive-binary encoding that `Lg` consumes, because
`s3` had already checked the arithmetic exhaustively and what `p4` had to show is that the flattened
capacity is a **type the derivation consumes**. A design would carry the general positive-binary product.
Whether it is affordable at capacities in the thousands is **unpriced**, and it is the same open question
`35` section 8 leaves about `ceil(log2)` at the type level.

An earlier draft of `p4` carried a half-written positive-binary addition whose `Twice` row was simply
wrong and which nothing used. It is deleted rather than left in place, and the deletion is noted in the
probe, because an unexercised wrong impl in a probe is what the panel's spike rule warns about and what a
later reader would cite.

### 6.3 The structure is not closed, and that is the right shape rather than a gap

A composition's element may be a numeral or another composition, so compositions nest. But the **fold**
takes a composition to a numeral, not to a composition. So the two concepts are not a closed algebra with
one carrier; they are a static shape that nests, plus an eliminator that leaves the shape entirely.

That is worth stating because the alternative reading is available and is wrong. If a composition were
numeral-like in its own right, one would expect the numeral's operations to lift to it, and some of them
do: elementwise addition of two runs is a run. But the operation the algorithm crates actually perform is
the fold, and `35` section 3 is an entire file about the fold's requirements. A vocabulary that made
compositions numeral-like would make the fold the odd operation out, when the fold is the reason the
layer exists.

**And there is a second eliminator nobody has named.** Indexing takes a composition and a runtime index
to an element. It is the operation where the dynamic half of every pair in section 4.2 is consumed, and
it is the operation where a composition's `len <= capacity` invariant is what makes a bound check
removable. `16`'s load-type finding, through `OPTIONS.md`'s account, is about exactly this operation: the
load type used to read one element out of a packed run is "neither of the two outputs" but a third
derivable quantity, and reaching for the carrier reads too few bits at 28 of 64 widths. I did not
investigate indexing and it is the largest thing I left, named in section 8.

## 7. Is the numeral a degenerate composition

**Partly, and the part that degenerates is worth keeping while the part that does not is why there are two
concepts.**

**What degenerates.** `p4` asserts at compile time that the flattened accumulator reach of a bare numeral
is its own reach, because `lg 1 = 0`. `s3` confirms that nesting a capacity-one level is the identity over
every `n` in `[1, 4096]`. So the derivation is total over capacities including one, and a numeral is a
consistent boundary case of the accumulator relation rather than a special case needing its own rule.
That is a real simplification and I would keep it: a canon stating the accumulator relation does not need
a separate sentence for the arity-one case.

**What does not degenerate, and it is most of it.** A composition has a length, an index and a traversal
order. A numeral has none of them, and more to the point has nowhere to put them: there is no index into
a value, and `35_probes/p1`'s whole result is that a value-level derivation cannot see a trip count. A
composition also has a layout coordinate, and `OPTIONS.md`'s account of `16` says a lone packed value has
the identical carrier to an unpacked one, which is the same observation from the storage side: packing is
not a property a single value can have.

**So the canon has two concepts and owes the relation.** The relation is not "a numeral is a composition
of capacity one"; that reads as an identity and it is false in four coordinates. It is that both are
**(static description, dynamic payload)** pairs under one staging discipline, differing in what the static
description determines. A numeral's static part determines the value set, the carrier and the operations'
semantics. A composition's static part determines the count bound, the stride, the traversal and any
shared coordinate.

Written that way the two concepts share a shape and not a definition, which is what makes the canon owe
one sentence rather than a translation table. `24:734-747` makes the same argument about the width
pair against the grid-and-reach reading, for the same reason: "a translation between two definitions is a
thing that has to be maintained, and the drift it is supposed to prevent is exactly the drift that
produces a stale translation." Whether the numeral-to-composition relation is a case of that pattern or
merely resembles it, I did not establish.

## 8. Should compositions be in arvo at all

The dispatch asks me to challenge this in these words and I have tried to. The answer splits and the two
halves have different strengths of evidence.

### 8.1 The contract belongs to arvo, and the argument is `42`'s with a different subject

**The composition contract is the second input to derivations arvo already owns.** `35`'s accumulator
relation takes a numeral **and a capacity**, and `06`'s D0/D1/D2/D3 taxonomy has no cell for it, because
that taxonomy classifies a site by which **operands** determine the numeral and the capacity is not an
operand. Something has to supply the second input, and it cannot be the consumer without the consumer
re-deriving `35`'s sufficiency argument by hand. `35` section 3.3, 3.4 and 3.10 are three worked
instances of what happens when a precondition of that kind is assumed rather than stated, one of which is
a documented downstream invariant that does not hold at 87.5% of inputs.

**Only arvo knows what the second input has to satisfy.** This is `42:370-376`'s argument with the
subject changed: "Only arvo knows its own axis values." Only arvo knows that the accumulator relation is
`W + ceil(log2 C)` rather than something else, because only arvo knows what `W` means. A composition
contract stated outside arvo would be stating a relation over arvo's own coordinates from outside, which
is the shape `42` argues cannot work for the law layer either.

**And `I11` names it.** `32:73`, op: "the contracts for things that compose to bigger units than just
numerals alone", in the same sentence as the algorithm crates and immediately before "But we need this
base to work, to build the bigger things". Whatever else is open, the **contracts** are named as arvo's
by op's own word two days ago. That is direction rather than a lock, per `RULES.md:20-27`, and it is
direction pointing at exactly this question.

### 8.2 Whether arvo ships containers is a different question and I did not settle it

The contract and the container are separable, and the panel has been treating them as one because the
dead tree shipped both. A design could state the composition contract in arvo, and let hilavitkutin,
vehje and each downstream consumer bring their own concrete storage satisfying it. The workspace's own
`use-the-stack-not-reinvent.md` argues the opposite for consumer crates, and the `no-alloc` framing's
"take the trait; storage is the consumer's problem" argues for it.

**What I could not do is decide it, because the deciding evidence is a fact about consumers and I have
none.** The question is whether two consumers' concrete containers would differ in ways a shared one
could not serve, and answering it needs a survey of what hilavitkutin and vehje actually hold. `35`
section 9 names the same gap about its own requirements: "I did not read vehje at all... this remains the
largest gap." I inherit it and I did not close it.

**What I can say is what would decide it.** If every downstream aggregate is a fixed-capacity run of one
element numeral, one container serves and shipping it is cheap. If the aggregates differ in rank, in
sparsity structure, or in who owns the memory, then the contract is what is shared and the containers are
not, and shipping one is arvo choosing a consumer's storage, which
`arvo-toolbox-not-policer.md` forbids in as many words. That is a checkable question and it is a dispatch
rather than an argument.

### 8.3 What I would refuse

**A composition contract that carries a traversal *engine*.** The same refusal `42:460-484` makes for
the law layer, and for the same reason. Whether a fold is split across lanes, tiled, blocked or fused is
a decision belonging to whoever performs it: arvo's own internals under
`arvo-always-optimal-internals.md`, hilavitkutin's scheduler, or a consumer's own loop. What arvo owes is
the **fact** that the reduction is reassociable, which `35_probes/p3` measures and `42:299-334`
explains, plus the **fact** that the capacity bounds the trip count. Deciding what to do with those facts
is the consumer's, and a composition contract that schedules is the toolbox rule broken in a new place.

I flag this because it is the shape my own background pulls toward, exactly as `42` flagged the rewriting
engine, and because a "composition layer" is a more natural home for a scheduler than a "law layer" is.
Nothing currently proposes one. The argument is on record so it does not have to be rebuilt.

## 9. Bearing on the live options

Per `OPTIONS.md`'s own instruction, each gets fits-well, fits-badly, or kills. I cite `OPTIONS.md` by
section and by a phrase verified with `grep -F`, never by line, per my brief.

**Q11, what the numeral guarantees to a fold and what a composition supplies.** *Fits the "both" reading
well, and adds a third thing to the list of what a composition supplies.* Q11's options are about the
**accumulator**: whether the numeral names its algebraic structure, its accumulator relation, both, or
neither. Sections 4 and 5 say a composition supplies more than a capacity: it also supplies the stride
(`16`), the traversal order (Q12's subject), and any shared grid coordinate (section 5.3). The entry's
last option, "The composition supplies everything and the numeral stays a value type", is the one my
section 5.2 bears on hardest and it survives better than the entry's cost line suggests: the reason it
"costs every composition re-deriving it" is that the derivation lives nowhere shared, and section 8.1
argues the contract can be shared without the container being.

**Q12, is the reduction order specified or is associativity required.** *Fits well and gains a distinction
the entry does not carry.* Q12's options are about the **shape** of a reduction. Section 4.3 says a
composition's traversal order is decided at the type, at a build arm, or at run time, and `40`'s
observable/unobservable cut applies: the reduction shape is observable per `40:409`, so it is on
the side an arm may not move for any strategy but the performance-first one. The addition is that
**capacity is also observable and layout is not**, so a composition's own coordinates split across the
same cut, which nothing in Q12 or Q13 currently says.

**Q13, which axes may a build arm move.** *Fits well, and the axis list is incomplete in a way the entry
would want to know.* Q13's four options are all about numeral-level axes. Section 4.3 finds
composition-level coordinates split the same way, with layout unobservable and capacity observable, and
`40`'s own framing that "the split is not derivable from an axis's name" applies with more force here:
nothing about the word "capacity" says it changes the answer, and it does, through the accumulator width.

**Q2, which coordinates a consumer writes.** *A new consideration, and it fits the fourth reading best.*
`24`'s grid-and-reach reading separates the grid from the reach, and section 5.2 measures that no
derivation reads the grid while all of them read the reach. That is an independent reason to keep the two
separate in the vocabulary: the split `24` derived from expressibility turns out to coincide with the
split between what a derivation consults and what it does not. I read `24` before deriving, so this is a
second instance rather than a second expert.

**Q4, what a datum stands for.** *Bears on the "a set, admitted generally" reading, and it does not
kill it.* `08:330` already routes intervals to a composition above the numeral, and section 4.1 says what
that layer is. The register's cost line for the set reading is that it "costs the total order,
multiplicative associativity outright... and the additive inverse", which is a cost at the **numeral**
level. Under the composition reading the cost lands where `08` puts it, on a pair of numerals built above,
and `35`'s objection that "the algorithm layer does not accept this type" is a statement about which
contracts the composition satisfies rather than about the numeral. That does not make the option cheap.
It moves the cost to a layer where it can be stated.

**Q9, the crossing at the width surface.** *No new bearing found, and I looked.* My probes are keyed on
type-level reaches throughout and none of them crosses from a const literal, so `13`'s "cross once, at
literals, in one direction" is neither tested nor contradicted here. `p2` and `p4` both obey it without my
aiming at it, which is what `35` and `16` also reported, and which under `RULES.md:116-118` is a third
instance of the same weak kind rather than corroboration.

**Q1, Q3, Q5, Q6, Q7, Q8, Q10, Q14, Q15.** *No bearing found, and I checked each.* One observation on Q8
rather than a finding: `35` section 5's result that the algorithm layer does not care how many numeral
families there are survives my derivation for a sharper reason. Every composition-level quantity in this
file (capacity, stride, traversal, shared coordinate) is stated over a numeral's reach and its axis
values, and none of them mentions a family. So the family question is invisible from the composition
layer as well as from the algorithm layer, which is two layers rather than one.

## 10. What the register should gain

I am not editing `OPTIONS.md` or `INTENTS.md`, per my brief. These are for whoever does.

**A new question, and the one I would rank first. Q16: what is a composition, and does the canon carry one
concept or two?** Its live options, in full:

- **A composition is a container of numerals, and the canon describes it as a data structure.** Cheapest
  and it is what the dead tree shipped. Costs the ability to say why `Cold` has no standalone value form
  and why a fold's accumulator needs a capacity, because both are facts about a run that a container
  vocabulary has no slot for.
- **A composition is the static description of a run, paired with the run.** The reading this file
  derives. Buys one sentence covering `16`'s stride, `35`'s capacity, `08`'s locus clause and any shared
  coordinate. Costs the canon a vocabulary for the static-against-dynamic split, which it does not
  currently have, and which is the same vocabulary `40`'s observable/unobservable cut needs.
- **A composition is a numeral-like object in its own right, with the numeral as its capacity-one case.**
  Buys one concept where the reading above has two. Costs the four coordinates section 7 says do not
  degenerate (length, index, traversal, layout), and makes the fold, which is the operation the algorithm
  crates actually perform, the one operation that leaves the carrier.
- **A composition is the consumer's, and arvo carries only the contract it must satisfy.** Section 8.2's
  fork. Buys the toolbox rule outright and costs a shared container that consumers may want. Undecided
  here, and section 8.2 says what would decide it.

**A second new question. Q17: which coordinates may a composition hold at run time, shared over its
run?** Section 5.3 is the argument and the answer is not "all of them":

- **None.** Every coordinate is static; frame-of-reference, delta and shared-scale encodings are outside
  the design, which is where `08`'s locus clause leaves them today. Costs the class `08:292` says the
  workload uses.
- **Any coordinate no derivation reads**, i.e. the whole grid, per `p2`. Cheapest to state and it is
  wrong for the bias, per `s5`: a raw addition reads it, a fold's effective origin is `n·B` with `n`
  dynamic, and a product's derived grid stops being a function of the operands' coordinates.
- **The adjustment and the canonical exponent only.** What `s5` supports: a same-grid add reads neither, a
  multiply reads the exponent and pays one runtime shift, and both survive a fold unchanged. This is
  block floating point and microscaling, and it is the option with measurement behind it.
- **Any coordinate, with the operations it costs stated per coordinate.** The most permissive, and the
  one that costs the canon a table rather than a rule.

**An addition to Q11's live options.** The register's list of what a composition supplies is the capacity.
Sections 4.1 and 5.3 add the stride, the traversal order and the shared coordinate, and section 6.1 adds
that when compositions nest the capacity supplied is the **flattened product** rather than the per-level
one.

**An addition to Q13's entry.** Its four options range over numeral-level axes. Composition-level
coordinates split across the same observable/unobservable cut, with layout unobservable and capacity
observable, and the entry's own observation that "the split is not derivable from an axis's name" applies
to them.

**A drafting note rather than an option, for whoever writes the boundary sentence.** `08:556-560`'s third
clause, "fixed by the type alone", excludes the shared-scale class from the **numeral** and reads as
excluding it from the **design**. `08:287-290` already says the layer is storage rather than nowhere.
Splitting the two costs one clause and saves a reader concluding the design forgot.

**One droplist entry, with its diagnostic and what would reopen it.**

*Deriving a nested composition's fold accumulator by composing the per-level derivations.* Closed by
`43_probes/s3`: sound, and one bit wide on 1201 of 4096 two-level shapes and up to two bits at three
levels, against a flattened derivation that is never wider and whose sufficiency holds at 0 of 8192
overflows including the per-row intermediates. `43_probes/p4` compiles the flattened form and asserts the
gap. **Reopened by:** a demonstration that the flattened capacity is not expressible at realistic
capacities, which is the unpriced compile-time question `p4` and `35` section 8 both leave open. Note
that closing this route closes an implementation shape, not the option that a composition supplies the
accumulator relation at all, which is Q11's and is untouched.

**And one thing the register should not acquire, so the argument is on record before someone proposes
it.** A composition contract carrying a traversal or scheduling engine. Section 8.3.

## 11. What I could not determine

**Which sense of "composition" the canon should carry.** Section 3 measures the collision and does not
resolve it, because both senses have an op statement behind them and the founding sentence uses the one
my dispatch does not. It is op's, and it is cheap to ask.

**Whether arvo ships containers or only the contract.** Section 8.2. The deciding evidence is a survey of
what hilavitkutin and vehje actually hold, and I read neither. `35` names the same gap about its own
requirements and it is still open.

**Indexing, which is the second eliminator and the one I did not investigate.** Section 6.3. A
composition's `len <= capacity` invariant is what would make a bound check removable, and that is the
microkernelling shape the workspace's `small-wins-compound-into-the-program.md` describes: a proof the
typestate holds and lowering discards. `16`'s load-type finding, which reaches me only through
`OPTIONS.md`, is about the same operation and says the load type is a third derivable quantity that is
neither of the two outputs. Whether a composition's capacity bound survives to the backend as a removable
check is a real question, it is measurable, and I did not measure it. It is the first thing I would attack
next.

**Whether a sparse composition's capacity is its dense extent or its nonzero bound.** CSR, the graph
crates and the comb crates all hold structures where the two differ by orders of magnitude, and the
accumulator relation is `W + ceil(log2 C)` for whichever `C` is. If it is the dense extent the
accumulator is needlessly wide; if it is the nonzero bound, something has to establish that bound
statically. I noticed this and did not pursue it, and it is the composition-layer analogue of the
soundness-against-bestness fork `07` names for the numeral.

**Anything about magnitude.** No bench harness run bears on any sentence here and none is claimed to.
`p4`'s type-level product at realistic capacities, the runtime shift a dynamic canonical exponent costs, a
removable bound check's value: all **unpriced**, and I use the word rather than reaching for an
impression.

**Whether the observable/unobservable classification of composition coordinates is complete.** I
classified two (layout unobservable, capacity observable) and I do not know that the list of
composition-level coordinates is two. `40` section 12 flags the same incompleteness about its own
eight-axis list, and the method that found its fourth axis has not been run again on either list.

**Whether `s5`'s coordinate-survival table holds past addition and multiplication.** I measured two
operations. Comparison, division, narrowing and the tropical operations `35` section 3.4 needs are
unmeasured, and the bias's behaviour under comparison in particular is the one I would expect to be
clean and did not check.

## 12. Coverage, bounded honestly

**Read end to end:** `INTENTS.md`, `00_brief.md`, `RULES.md`, `OPTIONS.md` (all 1071 lines, in two
reads), `32`, `36`, `39`, `35`, `40` (all 1257 lines, in two reads), `42`, `08`, `24`.

**Read in the region I cite, by opening the lines:** `archive/CANON_CANDIDATE.md` at `3382-3392` and
`785-815`, for the D52 quotation and the `Number<N, S>` declaration, and nowhere else; `DROPLIST.md` by
grep for composition-related entries, which returned the same D52 material at `DROPLIST.md:400`. `seed/` by grep
only. Every `file:line` in this document was opened and its content checked against my claim, not merely
resolved; the checker is `43_probes/p6` and its output is `43_probes/p6.out`.

**Not read:** `01`, `02` through `07`, `09` through `23`, `25` through `31`, `33`, `34`, `37`, `38`, `41`,
`PERSONA_CALLS.md`, `SETTLED.md`, `MORNING.md`, every probe directory except my own, and all of
`mock/crates`, which is empty. Where I refer to a finding in one of those I rely on `OPTIONS.md`'s
account, or on `35`'s or `42`'s, and say so in the text each time.

**The specific risk, and it is concentrated.** `16` is the file I lean on most and I have not read it. My
section 4.1 rests on `OPTIONS.md`'s account of its two-output finding and of its `Cold` result, and my
section 6.3 rests on its account of the load type. If `OPTIONS.md` misrepresents `16`, those three
paragraphs inherit it. `35` had the same exposure and named it; two files now depend on one unread source
through one compression, which is the shape `a-compression-is-checked-by-someone-else` describes and
which nobody has checked for `16` specifically.

**Not read at all in the repository:** `mock/benches/`. `RULES.md:224-228` names it as the only thing
here that can price anything, and nothing in this file is a price. I say so rather than leaving the
absence to be read as an oversight, and section 11 marks every place a magnitude would decide something.

**Probes:** `43_probes/`, committed with sources, raw compiler output and run logs, per `RUN.md`.

| probe | kind | what it establishes |
|---|---|---|
| `s1` | corpus census over the panel's own prose | three senses of "composition" occur in quantity across two panels; the founding sentence's sense is not recoverable from three lines of its own context |
| `p2` | compiler, 4 arms, gate-free | the carrier, the accumulator reach and the law bound are the same across grids differing in adjustment, bias, phase and canonical exponent; three negative controls, including one proving the equality mechanism is not vacuous |
| `s3` | exhaustive integer arithmetic | flattening is never wider than nesting over 4096 pairs; nesting is one bit wider on 1201 of them and two bits at three levels; the flat accumulator overflows 0 of 8192 including per-row intermediates, against 6502 one bit narrower |
| `p4` | compiler, 3 arms, gate-free | type-level capacity multiplication is expressible; the flattened accumulator reach is a type the derivation consumes; the flat-against-nested gap is one bit at `s3`'s witness, asserted at compile time; two negative controls |
| `s5` | exact rational arithmetic | a raw same-grid add reads the bias and reads neither the adjustment nor the canonical exponent; a fold's effective origin is `n·B` with `n` dynamic; the product's derived grid step matches `A²r^{2e}` at 9 of 9 zero-bias grids and 8 of 18 nonzero-bias grids |
| `p6` | citation checker | opens every `file:line` in this document and tests its content against a word it must contain: **32 checked, 0 failures**, and a cross-check confirms the probe's table and the document's citations are the same set. It failed on its first run, at one wrong range and three citations absent from its own table, and both defects are recorded in section 12 rather than repaired silently |

**Self-checks that fired, reported rather than hidden.** Three, and each changed what I would have
written.

`s5`'s first version asked whether the product **result set** of a grid is a uniform grid, got 0 of 27 at
every bias, and I was one paragraph from reporting it as a finding about the bias. It is a fact about
squares: `{0, A, 2A, …}` squares to `{0, A², 4A², 9A², …}` at any bias. The corrected question, whether
the smallest **containing** grid's coordinates are a function of the operands' coordinates, is Q5 and is
the result worth having. The wrong question is kept in the file with the correction beside it.

`p4`'s first version carried a positive-binary addition whose `Twice` row was wrong and which nothing
called, so the crate compiled and the impl was never exercised. It is deleted with a note, because an
unexercised wrong impl in a committed probe is precisely what a later reader would cite for the shape
rather than for the check.

`p6` failed on its first run, on one citation of mine and on three of my own citations it did not know
about. `08:283-290` opened mid-sentence and missed the phrase the claim needed, which is exactly the
class `RULES.md:126-133` describes: it resolved, and it did not say what I claimed. The range is now
`08:282-290`. And three citations were in the prose and not in the probe's table, so they were silently
unchecked; the cross-check that reported them was in the probe from the start and is the reason they were
found rather than shipped.

**One instance of evidence is never enough, and this file sits badly on that bar.** `s1`, `p2`, `s3`,
`p4` and `s5` share one author, one model and one sitting, so under `RULES.md:116-118` they are one
instance wearing five hats. They are independent in **instrument**: a corpus census, a compiler, an
exhaustive integer sweep, a second compiler construction, and exact rational arithmetic. They are not
independent in **derivation**, since all five come from one reading of the same five files.

The one place with genuinely independent corroboration is section 4.1's shape, which three prior files
reached from three directions before this file existed: `16` from the container derivation, `35` from the
fold, `08` from the membership predicate's locus clause. That is three instances and I did not produce
any of them; what this file adds is the name and the compiled consequence. Everything else here is one
instance, and the two I would most want a second, order-inverted read on are `s5`'s bias result and
section 4.2's claim that capacity-against-count is the defining boundary rather than one boundary among
several.

**Status: COMPLETE.**
