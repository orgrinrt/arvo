# 08. What the one format concept covers, and what it leaves outside

**Date:** 2026-08-08
**Position:** tests the panel's founding premise from outside it. `00_brief.md:8` says the primitives become
"named compositions over one format concept". Every file so far has worked inside that sentence. This one
asks what it excludes, by surveying the representations that exist rather than the ones the panel has been
discussing.
**Probes:** `08_probes/`, nine instruments in two languages, with `RUN.md` carrying every command, its exit
code, the four defects found in my own instruments and kept rather than repaired in place, and the two
compiler refusals that are the negative controls.
**Reading:** `RULES.md`, `01_op_answers.md`, `04_op_no_settlements_tonight.md`,
`07_orchard_the_adjunction_frame.md`, `06_kiselyov_where_a_numeral_is_inferred.md`,
`03_lamport_the_family_question_and_its_consequences.md`, `SETTLED.md`, `DROPLIST.md`, in that order and in
full, plus `seed/SETTLED_laws.md` in the passages bearing on the affine value map, the number-system chain,
the inclusion order and the lattice claim. `CANON_CANDIDATE.md` read in four passages as a starting text to
re-derive from, and cited nowhere. The predecessor panel's tree was not opened.
**Register:** nothing here settles anything, per `04`.

## Verdict, before the argument

**The premise survives, and the panel has been using a two-point sample of it.**

The one format concept is real, it is wider than fixed point and float, and the design already has a name for
the parameter that carries the whole of it. The parameter is the canonical exponent, and `ExponentForm` is an
axis with two instances where the concept has a function space. Everything below is a consequence of that
sentence.

**What the concept is, stated so it can be tested.** A representation is inside it when, at some radix, the
denotable magnitudes falling in each radix-binade form an arithmetic progression whose step is one adjustment
times a power of the radix, at a single phase. Then the step in binade *e* is `Adjustment * radix^fexp(e) `
and the shape of the function *fexp* names the family. This is one mechanical test and it carves the whole
survey. Applied to twenty-one value sets, **eighteen are inside** (`i1b.out`).

**And the shape of *fexp* is where the design's two-instance axis bites.** Constant is fixed point. Slope one
is float. Constant below a knee and slope one above is float with gradual underflow, which the design already
carries and calls `Underflow`. Anything else is unnamed, and **anything else is almost everything**: at a
five-binade window with a five-digit precision floor, thirty of one thousand six hundred and thirty-eight
monotone inhabited shapes are named (`i2e.out`).

**Posits are inside the concept and outside every named shape.** All three posit configurations measured
classify as INSIDE at bias zero, with a canonical exponent of no named form and three distinct step ratios
(`i1b.out`). So the "third family of segmented numerals" `07` priced is not hypothetical. It has a
standardised, hardware-implemented member, and admitting the concept in full admits it for free.

**The thing that surprised me, and it is the sharpest result here.** The three named shapes are closed under
the meet and are not closed under the join. Measured at four windows, the meet closure adds nothing and the
join closure at least doubles the set, fifteen to thirty-one at one window and thirty to seventy-nine at
another (`i2c.out`, `i2e.out`). More precisely, **gradual underflow is the meet
of a fixed-point format and a float**: the meet closure of the two named families is twenty-four shapes and
every one of them lies in the gradual-underflow family (`i2f_meet_lands_in_knee.out`), and the trait solver
computes the same answer independently (`p3_negctl2.out`). The join of the same two is the mirror shape, a
float below and a fixed grid above, and the design has no name for it.

That asymmetry is, on this reading, the whole of the cross-kind join failure. `03` establishes that the join
of a fixed-point numeral and a float has two incomparable minimal upper bounds and no least one, and that no
admission of more uniform-grid shapes repairs it. `03` also names the repair it would take: "a shape strictly
between the operands and the current minimal ones". **That shape exists and is computed here.** The join of
`03`'s own witness pair is `{0, 1/2, 1, 2, 3}`, it contains both operands, and it is **strictly inside both**
of the two minimal upper bounds `03` names (`i2.out` Q2b). It is a segmented grid, which is why no uniform
shape sits between and why `03` could not find it while looking among uniform shapes.

**The cross-kind join is total in the general class.** One hundred and eight of one hundred and eight
cross-kind pairs have a join that contains both operands, and one hundred and eighty-four of one hundred and
eighty-four intersections are again a format (`i2.out` Q1, Q2). So the Moore condition holds and a best
abstraction always exists, which is `07`'s level-two adjunction becoming total once the shape space is the
one the concept describes.

**And closing under intersection is not the same as admitting the concept.** `07` prices the Moore closure at
a sixteen to thirty-four percent enlargement. Measured at the exponent coordinate, intersection takes the
pointwise maximum, so the reachable slopes are the maxima of the operands' slopes, measured as `{0, 1}` over
every pairwise intersection in the pool. Every posit measured has slope 2 somewhere (`i2.out` Q4). **A
tapered format is not in the intersection closure of the two named families and is in the general class.**
That is a refinement of `07`'s pricing rather than a correction of it: the closure buys the glue shapes and
does not buy the tapers.

**The owed probe is written and it compiles.** `03` section 9, `06` section 9 and `07` section 8 each name
"whether the segmented shapes are expressible in the typestate under the forbidden-feature set" as owed and
none of them wrote it. `p3_segmented_typestate.rs` carries a canonical exponent as a type-level list, orders
two formats pointwise, computes the meet and the join as associated types, and refuses a false ordering at
type check with a diagnostic naming the binade where it fails. **Gate-free**, no `#![feature(...)]` anywhere.
**And it erases**: the emitted assembly is twenty-two lines, one symbol carries a body, the body is a single
`ret`, and six further symbols including the unguarded baseline and all three guarded call sites across a
fixed, a float and a tapered shape are aliases onto it (`p3_asm.out`).

**What is genuinely outside, and each failure names a different layer.** Values not of the form `m * r^q` at
any admitted radix put a representation in a different number system, which is the boundary the record
already draws for a different reason (`seed/SETTLED_laws.md:180-193`, TWO EXPERTS). A value set that is not
an arithmetic progression inside a binade puts it outside the format concept altogether. A value set that
depends on other data puts it in the storage layer. A datum map that is not injective, or whose order is not
the value order, is an encoding on top of a numeral rather than a numeral.

**Five of the eighteen differ from a plain fixed-point numeral in no value-set respect at all.** A residue
number system, a thermometer code, a carry-save form, negabinary and a mixed radix all denote an interval of
integers, and every test in the classifier is blind to what separates them (`i1b.out`, `i3.out`). That is the
second axis, and the design has it in one place only: `SignDomain` against `SignIndexing`.

**The boundary, in one sentence**, is section 5, and it is the thing I would most want a second read on.

## 0. Gates

### 0.1 The canon gate

There is no ratified canon for arvo. This panel is writing the first one, and `01` section 0 carries op's
correction that the rows in `SETTLED.md` marked RATIFIED were classified under a superseded reading. So there
is nothing to defend and the governing material is the narrow set recording op in the loop.

Three items bear on this file directly rather than tangentially, and each is checked rather than gestured at.

**The acceptance criterion**, `SETTLED.md:65-71`, RATIFIED: the consumer expresses usage in bits and bytes,
the typestate derives container and representation, it validates, and it **erases on lowering**. This file's
whole proposal lives at compile time, and section 4.5 measures the erasure rather than asserting it.

**The number-system scoping theorem**, `seed/SETTLED_laws.md:180-193`, TWO EXPERTS: "Every arvo value is
`m . r^q` for integer `m`, `q`, and integer radix `r >= 2`, so every arvo value set is a finite set of
rationals." That sentence is the first clause of my boundary and I did not invent it. Three representations
in the survey fail it and the failure is what puts them outside.

**The toolbox rule.** `arvo-toolbox-not-policer.md` forbids the substrate from hardcoding a consumer policy.
Nothing here removes a consumer's choice. The survey names constituencies rather than dismissing them, per
the brief's own instruction that a representation dismissed because most consumers would not want it is
dismissed on the wrong grounds. Gate passed.

### 0.2 The test gate

Re-run rather than inherited, and the two counts that overlap with `03`, `06` and `07` agree, which is worth
recording as an independent arrival:

    $ find mock/crates -path '*tests*' -name '*.rs' | wc -l
          91
    $ grep -rl '#\[test\]' mock/crates --include='*.rs' | wc -l
          83

There is no suite to audit for this question, because the surface it is about has no source. **I did not run
the suite and I am saying so rather than implying it passed.** The brief declares `mock/crates` nuked and
forbids citing it as evidence about what is correct; a count is the one thing a nuked tree can honestly
report.

### 0.3 The brief's cheap factual claims

The pin is as stated. `rust-toolchain.toml` carries `channel = "nightly-2026-05-28"` and
`rustc +nightly-2026-05-28 --version` reports `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.

**The brief says the format notion "reduces to a grid, a phase, and two endpoints". That is the inclusion
test and it is not the format notion, and the difference is most of this file.** `SETTLED.md:118` (TWO
EXPERTS) records inclusion as needing the grid, the phase and both endpoint conditions, which is a test for
one uniform grid. The value map the record actually carries is `Adjustment * radix^exponent * k + Bias`
(`seed/SETTLED_laws.md:274`), and `exponent` is a **member**, not a constant, which is exactly what makes a
float a float. So the notion is a grid per exponent rather than a grid, and the brief's summary is the
one-segment case.

**The survey list in the brief is the brief's own and is not authoritative, as it says.** I dropped nothing
from it and added seven: decimal with cohorts, block floating point and the shared-scale microscaling
formats, half-unit-biased formats, unevaluated sums (double-double), Galois field elements, Montgomery and
other transported encodings, and the order-permuting encodings (Gray, one's complement, sign magnitude,
negabinary).

**The ground is new to this panel**, counted rather than assumed. Across every markdown file in the panel
directory:

    posit: 0    posits: 0    generic_format: 0    tapered: 0
    residue number: 0        logarithmic number: 0
    signed-digit: 0          carry-save: 0        continued fraction: 0
    unum: 0                  negabinary: 0        mixed radix: 0

Three hits for `flocq` exist, all in `CANON_CANDIDATE.md`, and one of them matters: a ratified naming call
renamed Flocq's `fexp` to **`canonical_exponent`**. Section 1.3 says what became of it.

## 1. The premise, restated precisely enough to be wrong

"One format concept" is a claim about a parameterisation, and a claim about a parameterisation is testable
only once the parameters are named. So this section names them, from the record, before the survey uses them.

### 1.1 What the record's value map actually says

`seed/SETTLED_laws.md:274` gives the affine value map: the value of a stored integer `k` under a numeral is
`Adjustment * radix^exponent * k + Bias`, and it records why `Adjustment` and `Bias` cannot be folded into
each other, one changing the spacing and the other moving the origin.

Read as a set, that is

$$V(N) \;=\; \bigl\{\, A \cdot r^{\,\varepsilon} \cdot k + B \;:\; k \in K \,\bigr\}$$

for an index set `K`. When `exponent` is a single declared value the set is one arithmetic progression, which
is fixed point. When `exponent` varies per datum the set is a union of arithmetic progressions, one per
exponent, which is float. **The two are the same expression evaluated with one exponent and with many**, and
that is the content of the founding premise, correctly stated.

### 1.2 The parameter that carries the difference has a name in the literature and a shape in the design

Group the values by radix-binade. Inside binade `e`, the step of the progression is `A * r^{f(e)}` for some
integer `f(e)`. The function `f` is Flocq's canonical exponent, and it is the whole of the difference between
the families:

| family | canonical exponent | in the design |
|---|---|---|
| fixed point | constant | `ExponentForm = Implicit<EXPONENT>` |
| float, unbounded exponent | `f(e) = e - p + 1` | `ExponentForm = Ranged` |
| float with gradual underflow | constant below a knee, slope one above | the `Underflow` axis, nested |
| tapered (posit and its relatives) | slope two or more somewhere | **no name** |
| everything else | any monotone `f` with `f(e) <= e` | **no name** |

The first three rows are the design's; the last two are the survey's. And the axis table the design carries
(`ExponentForm`, two instances) is the function space collapsed to two points, with a third point added
separately under a different name because gradual underflow needed one.

### 1.3 The concept was named by a ratified call and never given a home

A ratified naming decision (D56, op's own) renamed `fexp` to **`canonical_exponent`**, with the reason
recorded: "Flocq's own prose calls it the canonical exponent; `fexp` is its Coq identifier, not the concept's
name."

The name occurs exactly once in the candidate consolidation, in that naming decision. It appears in no axis
table, no member list and no trait surface.

**So the design has ratified a name for a concept it does not carry.** That is not a criticism of the naming
call, which is right. It is the tell that the concept was recognised and then represented by two of its
values. I am reporting it because it is checkable, it is one grep, and a reader who believes the design
carries a canonical exponent will misread every question below.

### 1.4 What is testable, and it is one predicate

> Fix a radix `r`. For each binade `[r^e, r^{e+1})`, are the denotable magnitudes an arithmetic progression
> whose step is one adjustment times a power of `r`, all at one phase?

If yes, the representation is inside the concept, and the shape of `f` names the family. If no, the clause
that fails says which layer it belongs to instead. That predicate is `i1b_classify.py` and it is what the
survey runs.

**Two things about the predicate are worth stating before it is used.** It is a test on the **value set** and
is blind to the encoding, which is section 2.2 and turns out to matter for five representations. And it takes
the radix as an argument rather than reading it off the representation, which is why the same decimal value
set is inside at radix ten and outside at radix two (`i1b.out`). The radix is a parameter of the question,
and the design has it as a member, so that is the concept working rather than failing.

## 2. The axes the survey needs

Three, and they are independent. A representation can sit anywhere in the product.

### 2.1 The value set

What the denotable values are, as a set. The predicate of section 1.4 decides it, and it has three failure
clauses rather than one:

**Not rationals of the form `m * r^q`.** The logarithmic number system, fixed-slash and floating-slash
rationals, p-adic and Hensel codes, complex and quater-imaginary radices, level-index and symmetric
level-index, and Galois field elements. This is the record's own scoping theorem doing the work.

**Rationals, but a binade is not an arithmetic progression.** Unevaluated sums, and the rational formats
again if the radix is allowed to vary per datum.

**A progression per binade, so inside.** Everything else in the survey.

### 2.2 The encoding

The map from datum to value. Two properties decide whether the design's machinery still works, and both are
invisible to section 2.1.

**Injectivity.** Measured over eleven encodings of value sets the concept already admits (`i3.out`): plain
unsigned, two's complement, Gray, negabinary and a residue system are injective; one's complement and sign
magnitude carry two data for one value; a carry-save form carries up to four at six bits; a thermometer or
stochastic stream decoded permissively carries up to twenty; decimal cohorts carry up to three.

**Order agreement.** Whether sorting the raw bit pattern sorts the values. Measured: **plain unsigned is the
only one of the eight integer-keyed encodings where it holds.** Two's complement fails it, which is worth
saying out loud because it is the design's own signed encoding, and it is why the record carries the
`TotalOrd` split at `SETTLED.md:116` between a value-level order and a datum-level one.

The design has this axis in exactly one place, as `SignDomain` (identity) against `SignIndexing` (encoding),
a split the droplist records being forced by a different problem: "A single three-instance `Sign` axis
bundling range and zero-count: under-determines the set and mixes a value fact with a datum fact."

**And the design has already accepted a non-injective datum map**, for a third unrelated reason. The droplist
records the crossing contract's two round-trip theorems being replaced because "the second is false the
moment signed zero, NaN payloads, or decimal cohorts exist. Replaced by the section-retraction triple." A
section-retraction triple is exactly the shape a redundant encoding needs. So the machinery a carry-save or
thermometer encoding would want is present, built for signed zero, and used by nothing else.

### 2.3 The locus

Whether the value set is a property of the datum alone.

Block floating point, the shared-scale microscaling formats, frame-of-reference and delta column encodings,
dictionary encodings and run-length encodings all have a value set that depends on **other data**. The
droplist already excludes block floating point on a related ground: "BFP is a different kind of object, a
composite numeral over a shared external exponent."

The sharper reason, and it generalises to the whole class: **no per-datum type can express a constraint that
holds between data.** The per-element value set of a block floating point element is the union over shared
exponents, which is a float and is expressible; what is not expressible is that the block shares one
exponent. So the exclusion is structural rather than a matter of scope, and the layer it names is storage.

That matters more here than it looks, because bitpacked column storage at scale is the workload arvo exists
for, and frame-of-reference and delta encodings are what that workload actually uses. Section 4.7 says what
such a consumer needs instead.

## 3. The survey

Twenty-one value sets classified mechanically, plus the categories where the classification is an argument
rather than a run. Every count is from `i1b.out`, `i2.out`, `i2e.out` or `i3.out`.

### 3.1 The classification, in one table

| representation | value set | canonical exponent | encoding | verdict |
|---|---|---|---|---|
| fixed point, any radix | progression | constant | injective, ordered | **expressed** |
| half-unit-biased (HUB) | progression, phase half a step | constant | injective, ordered | **expressed**, and the only case the `Bias` axis earns |
| ranged grid, count not a power of the radix | progression | constant | injective, ordered | expressed by `Ranged`, not by the anchored family |
| float, no subnormals | union of progressions | slope one | injective, ordered | **expressed** |
| float with subnormals, fp8, bfloat16 | union of progressions | knee then slope one | injective, ordered | **expressed**, and the knee is the meet of a fixed and a float |
| decimal, at radix ten | union of progressions | slope one | **cohorts, not injective** | value set expressed; the encoding is not |
| posit, all three configurations | union of progressions | **slope two, no named shape** | injective, ordered | value set inside the concept, shape unnamed |
| tapered floats, takum, unum type I | as posit | as posit | varies | as posit |
| residue number system | integer interval | constant | injective, **order lost** | value set expressed, encoding not |
| thermometer and unary | integer interval | constant | **not injective as decoded** | value set expressed, encoding not |
| carry-save and borrow-save | integer interval | constant | **not injective** | value set expressed, encoding not |
| signed-digit and non-adjacent form | integer interval | constant | **not injective** | as carry-save |
| negabinary, radix minus two | integer interval, asymmetric | constant | injective, **order lost** | value set expressed, encoding not |
| mixed radix, factorial base, Zeckendorf | integer interval | constant | injective | value set expressed by `Ranged` |
| Gray code, one's complement, sign magnitude, Montgomery | same as their base | same | injective or not, **order lost** | encoding-only |
| stochastic streams | progression | constant | **not injective, denotes a distribution** | value set expressed; the denotation is not |
| block floating point, microscaling, delta, frame of reference | float per element | slope one | injective | **outside: the set depends on other data** |
| unevaluated sums (double-double) | **not a progression per binade** | none | injective | **outside** |
| fixed-slash and floating-slash rationals | **not `m * r^q`** | none | non-injective without gcd | **outside** |
| continued fractions, exact real arithmetic | **infinite, not const-size** | none | streams | **outside** |
| logarithmic number system | **not `m * r^q`**, 8 of 64 rational | none | injective, ordered | **outside** |
| level-index, symmetric level-index | **not `m * r^q`** | none | injective, ordered | **outside** |
| p-adic and Hensel codes | rational, **order is not numeric** | none | non-injective | **outside** |
| complex and quater-imaginary radix | **not in the rationals** | none | injective | **outside** |
| Galois field elements | **no numeric magnitude** | none | injective | **outside**, and it is `Bits<N>` |
| interval and triplex forms | **a datum denotes a set** | inherited | pair of numerals | outside, and buildable above |
| affine forms, Taylor models | **a datum denotes a set, of runtime-varying arity** | none | vector | **outside**, and the arity is the reason |

### 3.2 Where the counts come from

Of twenty-one value sets run through the classifier, **eighteen are inside** the affine-grid-per-binade
concept. Of those, seventeen sit at bias zero and one needs a nonzero bias. By canonical exponent shape: ten
constant, two slope one, three with a knee, three segmented with no named shape (`i1b.out`).

The three outside, with the clause that fails: the decimal set asked at the binary radix and the fixed-slash
set fail `m * r^q`; the unevaluated sums fail the progression-per-binade test. The logarithmic case is
reported separately because its values cannot be constructed exactly: `2^{k/8}` is rational exactly when
eight divides `k`, so **eight of sixty-four** denotable magnitudes are rationals at all.

### 3.3 Posits, which are the case the survey exists for

All three posit configurations classify INSIDE, at bias zero, with a canonical exponent that is neither
constant nor a line. Measured: `posit<8,0>` has six distinct exponent values over ten constrained binades and
three distinct step ratios; `posit<8,1>` thirteen over sixteen; `posit<10,2>` thirty-six over forty
(`i1b.out`).

The step ratios are the finding. **Every posit measured has slopes `{0, 1, 2}`**, and the intersection of any
fixed shape with any float shape in the pool has slopes drawn from `{0, 1}` and never more (`i2.out` Q4).
Intersection takes the pointwise maximum, and a maximum of slopes at most one is a slope at most one. So a
tapered format is not reachable by closing the two named families under intersection, and it is reachable by
admitting the canonical exponent as a function.

That is the refinement I would carry to `07`. Its section 4.3 prices closing the family under intersection at
a sixteen to thirty-four percent enlargement and says every added shape is a segmented grid neither family
names. Both halves hold. What the closure does **not** buy is the tapers, which are a different and larger
region, and the price of those is a different price.

### 3.4 The five that differ only in the encoding

A residue number system, a thermometer code, a carry-save form, negabinary and a mixed radix all denote an
interval of integers. Every test in the classifier says "fixed point, one segment" for all five, and every
test is right.

What separates them is `i3.out`. A residue system is injective and loses the order. Negabinary is injective
and loses the order. A thermometer code as decoded permissively carries twenty data per value. A carry-save
form carries four at six bits. A mixed radix is injective and keeps the order and differs from a plain
integer numeral only in that its count is not a power of the radix.

**So four of the five are questions about `SignIndexing` generalised, and the fifth is a question about
`Ranged`.** Neither is a question about the format concept, and both are cheaper than they look because the
design has already had to build the machinery for an unrelated reason.

### 3.5 What the survey did not find, and it is a result

**No representation in the survey needs a second format concept.** Everything that is not inside the concept
fails a clause that names a different layer: a different number system, a storage encoding, a datum encoding,
or a denotation that is not a point. Not one of them is a rival parameterisation of the same job.

That is the strongest support the founding premise gets tonight, and it is stronger than agreement would be,
because it comes from looking for a counterexample and not finding one.

## 4. What admitting each would cost, and what excluding each costs

The brief asks for both directions per category. An exclusion nobody notices is free and is recorded as free.

### 4.1 The canonical exponent as a function: the largest question here

**What it would cost.** The shape space stops being a small tuple and becomes a function over a bounded
exponent range, which is a list. That collides directly with a ratified call: `SETTLED.md:97` and `:110`
record "No enumeration, ever, if it can be helped", ratified four times and refused against three named
proposals: a width table, a per-width bridge population line, and a macro escape.
A per-exponent precision list looks like an enumeration and a reader will say so.

**What p3 says about that collision.** The list is inductive rather than enumerated: two impls for the
ordering, four for each of the pointwise operations, and no width appears anywhere. It compiles gate-free, it
refuses at type check rather than at monomorphisation, and it erases to one symbol whose body is `ret`. That
does not settle whether it is an enumeration in the sense the refusals meant, and I am not claiming it does.
It removes the assumption that the shape is unbuildable, which is what those refusals were about in the
container case.

**What excluding it costs.** Three things, and they are not the same size.

The join across kinds stays partial, which is `03`'s F2, and the design owes either a refusal or a tie-break,
which are `03`'s readings C and D and are both cheap.

Posits and tapered formats stay outside. Excluding them means an arvo consumer wanting a posit column
builds it outside arvo, and the parts they would need (a bit container of the right width, and a value map)
are exactly the parts arvo would otherwise supply. **The size of that constituency is a literature claim and
I did not verify it**, which matters here specifically: `SETTLED.md:117` and the droplist carry a literature
identification inside the finest-view mechanism as refuted and unrepaired, so an unverified claim of that
kind carries a discount in this record. What I did verify is that the value sets exist and classify as
measured (`i1b.out`); who wants them is a claim from outside the repository and is marked as one.

And the design keeps carrying a ratified name, `canonical_exponent`, for a concept nothing implements.

### 4.2 Gradual underflow, which the design already pays for

Worth stating because it is a cost already borne and it is the meet.

**Measured:** the meet closure of the fixed and float shapes is twenty-four shapes, and every one lies in the
gradual-underflow family (`i2f_meet_lands_in_knee.out`). Independently, the trait solver computes the meet of
a fixed shape and a float shape and prints it in a diagnostic, and it is the gradual-underflow shape
(`p3_negctl2.out`).

So `Underflow` is not a float-specific quirk that the design happened to need. **It is the intersection of a
float with a fixed-point format**, and the design already carries it as an axis because IEEE forced the
question. The general concept would carry it as one point in the function space rather than as a nested axis.

**Excluding the general concept while keeping `Underflow` is a defensible position** and the honest way to
describe it is that the design names one point off the two curves, on hardware grounds, and no others.

### 4.3 The join shape, which is the mirror and has no name

The join of a fixed shape and a float shape, measured over the pool, is a shape with slope one at the bottom
and slope zero above (`i2e.out`, three worked in full). That is a float near zero and a fixed-point grid
above: fine absolute resolution near the origin, uniform absolute step over the working range.

**What admitting it would cost.** One more named point, symmetric to `Underflow`, or the function space.

**What excluding it costs.** The join stays partial across kinds and the design must refuse or pick. That
cost is already on the table as `03`'s readings C and D.

**Who would want it.** A consumer accumulating small signed residuals into a fixed-point working range: audio
near the noise floor, sensor deltas, and any error-feedback loop. Whether that constituency is worth a name
is a taste question and it is not mine.

### 4.4 The encoding axis

**What admitting it would cost.** The stored width stops being a function of the value count. A residue
system at moduli three, five and seven holds one hundred and five values and needs `2 + 3 + 3 = 8` bits where
a positional encoding needs seven. A thermometer code holds seventeen values in sixteen bits where a
positional encoding needs five. So the container derivation gains an encoding-keyed width function, which is
a real change to the ratified acceptance criterion's second clause.

And the datum-level order stops agreeing with the value order for Gray, negabinary and residue encodings, so
comparison stops being a bit comparison. The record already has the vocabulary for that, at `SETTLED.md:116`.

**What excluding it costs.** A consumer wanting any of the five builds the value map themselves over
`Bits<N>`, which arvo ships. That is a small cost and it is the right one for four of the five. The
constituencies: cryptography and fault-tolerant arithmetic for residue systems; hardware adder trees and
deferred carry propagation for carry-save; flash converters and one-hot state for thermometer codes;
sign-free arithmetic for negabinary.

**The one that is not free.** Decimal cohorts are not an exotic encoding, they are IEEE 754-2008, and the
record already had to replace a round-trip theorem because of them. So the design has already priced the
non-injective case once, without generalising it.

### 4.5 Interval and affine forms

**Measured** (`i3.out`), over twenty thousand random interval pairs under addition and multiplication on
`U<3,3>`, with out-of-range results skipped: outward rounding fails **zero** times, using one directed mode
for both ends fails one thousand and thirty-six, and round to nearest for both ends fails nine hundred and
forty-six.

So an interval consumer needs exactly two things from the numeral and neither is a change to it: both
directed rounding modes, reachable per operation. The interval is then a pair of numerals, built above.

**What excluding interval arithmetic costs, therefore, is nothing, on one condition**, and the condition is
worth writing down because it is a constraint on the design rather than an absence: **both directed modes
must be a per-operation choice rather than a per-numeral one.** If rounding is fixed at the numeral, an
interval consumer cannot round its two ends in opposite directions with one type and the whole construction
fails.

That connects to `07` section 2.3 from a different direction. It finds that a rounded datum already denotes
the set of exact values that produced it. Interval arithmetic is that observation made explicit and carried
in the type, and the measurement above says the design can serve it without saying so.

**Affine forms and Taylor models are a harder exclusion and the reason is the arity.** A datum is `x0` plus a
sum over noise symbols whose count grows with the computation, so it is not const-size. That is a hard
collision with the design's own constraint, and no format concept repairs it.

### 4.6 The rational formats

Fixed-slash and floating-slash fail the `m * r^q` clause: `1/7` is not `m * 2^q` and not `m * 10^q`
(`i1b.out`). And the design's `Adjustment` being a signed gcd-normalised rational means **a single rational
grid is already expressible**: a numeral at adjustment one third denotes multiples of one third.

So the precise statement is: **the design admits any one rational grid, and does not admit a datum choosing
its own denominator.** That is a sharper exclusion than "rationals are out", and it says exactly what a
consumer would have to give up.

**Who wants it.** Exact rational arithmetic in bounded memory: continued-fraction algorithms, exact ratios in
music and typesetting, and symbolic work that stays small. **What excluding it costs them:** a pair of
numerals plus a gcd, built above, with the design supplying neither the normalisation nor the ordering.

### 4.7 The storage-layer encodings, which are the ones arvo's own workload uses

Block floating point, microscaling, frame of reference, delta and dictionary encodings are outside because
the value set depends on other data.

**What excluding them costs.** Nothing to express and something to say. The consumer holds two numerals: the
shared or reference value, and the residual. The residual has its own numeral and it is an ordinary one.
`Array<T, N>` already exists to hold the block.

**What the design owes such a consumer**, and it is one sentence rather than a mechanism: the residual's
numeral is what it must declare, and the design has no opinion about the shared value beyond its being a
numeral too. Naming that keeps a reader from looking for a block-floating-point axis and concluding the
design forgot.

### 4.8 The number systems above the rationals

Complex and quater-imaginary radices, quaternions, dual numbers for automatic differentiation, and the
p-adics all leave the rationals, and the record's own vocabulary already anticipates them: ten members fixed
by mathematics, with membership through algebraic structure, and numeral membership "independent of every
branch above ℚ" (`seed/SETTLED_laws.md:180-193`).

**So the format concept's boundary coincides with a boundary the record already draws for a different
reason.** The vocabulary is strictly wider than the concept, deliberately, and the concept covers exactly the
sub-rational part. I am flagging this as agreement rather than corroboration: I read that row before deriving
the boundary, so under `RULES.md` it is inherited.

**What excluding them costs.** A dual number or a complex value is a pair of numerals, and automatic
differentiation is a real constituency in arvo's own downstream. The composition is available and the design
owes only the statement that it is the composition rather than a numeral.

### 4.9 The measurement that would change the ranking, and I did not take it

Everything in section 4 is structural. **Nothing here is priced.** No bench harness run bears on any of it,
and the word is used deliberately rather than reaching for a number.

The measurement I would take first: whether a canonical exponent carried as a type-level list costs anything
at compile time compared with two integers, at realistic exponent spans. `p3` says the runtime cost is zero
because the whole thing erases to a symbol alias. The compile-time question is unanswered and it is the one
that decides whether the general concept is affordable, because a float over a wide exponent range has a
list of a couple of hundred entries and the design's own record has a quadratic curve on a table of that
shape (`DROPLIST.md`, on the bridge table past 4096 rows).

## 5. The boundary

One sentence, offered as the thing to attack:

> **A representation is a numeral when a datum denotes one rational, when the denotable magnitudes in each
> binade of some admitted radix form one arithmetic progression at one phase whose step is that radix to some
> power, and when the set is fixed by the type alone. Everything else fails exactly one of those three, and
> which one it fails names the layer it belongs to instead: a different number system, a different format, or
> a storage encoding.**

Three clauses, and the reason each is where it is.

**"One rational" is the denotation clause.** It excludes intervals, affine forms, stochastic streams and
anything denoting a set or a distribution. Those are compositions above a numeral, and section 4.5 measures
that the composition needs nothing from the numeral that the design does not already have.

**"One arithmetic progression per binade" is the format clause**, and it is the whole of the founding
premise. Fixed point is the one-segment case, float is the constant-slope case, gradual underflow is their
meet, and tapered formats are the rest. It excludes unevaluated sums, the rational formats and the
logarithmic systems.

**"Fixed by the type alone" is the locus clause.** It excludes block floating point, microscaling, delta and
frame-of-reference encodings, which are the storage layer.

**And the encoding is deliberately not in the boundary**, because it is a second axis rather than a
membership test. Two representations with the same value set and different encodings are the same numeral
under two encodings, which is what the `SignDomain` against `SignIndexing` split already says for one case.

### 5.1 What the boundary does not decide

It does not decide which canonical exponent shapes the design admits, which is section 4.1 and is the real
question. It does not decide whether the encoding axis generalises. And it says nothing about the endpoints,
which is the other half of the inclusion test and which I held aside throughout, including in `p3`.

## 6. Does the premise survive

**Yes, and with a correction to how the panel has been reading it.**

`03` finds that "reading A appears to be reading C wearing A's clothes: it is one family only if the family
is already narrow." The same sentence, one level up, is what this file found: **the one format concept is one
concept, and the design's instance of it is narrow enough that the concept's own totality does not reach the
design.** The concept is not one family among several dressed as a universal. The design's `ExponentForm` is
two families dressed as the universal.

Three measurements carry that, and they are independent of each other:

**The concept is closed and total where the design is not.** One hundred and eighty-four of one hundred and
eighty-four intersections are formats, one hundred and eight of one hundred and eight cross-kind joins
contain both operands, and `03`'s own witness has a join strictly inside both of the minimal upper bounds
`03` names (`i2.out`).

**The design's three shapes are meet-closed and not join-closed**, at four windows, with the meet closure
adding nothing and the join closure at least doubling (`i2c.out`, `i2e.out`). And `seed/SETTLED_laws.md:278-288`
records, at TWO EXPERTS, "meets are exact, joins strictly overshoot", within one family and in different
coordinates. Two arrivals at one asymmetry from two directions, and mine explains the record's rather than
restating it: the overshoot is the design rounding an unnamed shape up to a named one.

**And the survey found no rival.** Twenty-one value sets, six categories of exclusion, and not one of them is
a second parameterisation of the same job.

### 6.1 Where the premise is doing less work than it appears to

One honest deduction against my own conclusion. The premise says the primitives become **named compositions
over** one format concept. Everything above is about the format concept and nothing is about the composition.
Whether the primitives are named compositions over it is a separate claim that this file does not test, and
`06`'s enumeration of inference sites is closer to that question than anything here.

## 7. Routes closed, each with the thing that closed it

**"Tapered formats are outside the concept."** Closed by `i1b.out`: all three posit configurations classify
INSIDE at bias zero with a canonical exponent of no named shape. What is outside is the design's two-instance
axis, not the concept.

**"Closing the two families under intersection admits the tapers."** Closed by `i2.out` Q4: every pairwise
intersection in the pool has canonical exponent slopes drawn from `{0, 1}`, and every posit measured has a
slope of two. Intersection takes the pointwise maximum and a maximum of slopes at most one is a slope at most
one.

**"The general concept is Flocq's `generic_format`."** Closed by my own first instrument being wrong. `i1`
tested for a phase of zero, which is `generic_format`, and called a half-unit-biased format OUTSIDE. The
design's map is affine and admits it, and the droplist records the correction that admits it. **The design's
concept is strictly wider than `generic_format` in the phase coordinate and strictly narrower in the exponent
coordinate**, and I would not have found the first half without running the wrong test.

**"The cross-kind join has no least upper bound."** Closed as stated, and true as `03` scoped it. `i2.out`
Q2b computes the join of `03`'s own witness pair, finds it contains both operands, and finds it **strictly
inside both** of the two minimal upper bounds `03` names. `03` predicted the repair would be "a shape
strictly between the operands and the current minimal ones"; this is that shape, and it is segmented, which
is why no uniform shape sits between.

**"A segmented format cannot be carried in the typestate under the forbidden-feature set."** Closed by `p3`,
which compiles gate-free on the pin, orders two segmented formats pointwise, computes both lattice operations
as associated types, refuses a false ordering at type check, and erases to one symbol whose body is `ret`.
Three files named this probe as owed.

**My own assertion that a tapered shape includes into gradual underflow.** Closed by the compiler, at
`E0277`, naming the pair `S<Z>: AtMost<Z>` and therefore the binade where the ordering fails
(`p3_negctl.out`). The false claim is kept in the source as a comment rather than deleted.

**My own enumeration bound, and it is the failure `RULES.md` warns about by name.** `i2b` bounded the
canonical exponent by a quantity that moves with the binade, which silently forbids a constant at any window
above four. `i2d` then reported "fixed alone n=0" and I read it as a fact about fixed-point formats. `i2e`
repairs it and its counts supersede three earlier instruments'. The shape of the answer survived; the counts
did not.

**Two further defects in `i1`, kept.** A binade holding one value pins no step, so `i1` invented a canonical
exponent wherever a range was truncated and reported tapering that was an artifact of the top binade. And its
unevaluated-sum generator drew both parts from the same float family, so the set it classified was the plain
float and the verdict flipped from INSIDE to OUTSIDE once repaired.

## 8. Coverage, stated honestly

**What I read.** Everything the brief named, in full, plus `seed/SETTLED_laws.md` in the passages bearing on
the affine value map, the number-system chain, the four-condition inclusion order and the lattice claim.
`CANON_CANDIDATE.md` in four passages as a starting text, cited nowhere. The predecessor panel's tree was not
opened, so every statement here about `145`, `146`, `148` or `150` is a statement about this panel's carried
files reporting them.

**The largest bound, and it is the same one `06` and `07` name.** Every Python instrument is unsigned, at
radix two except where a radix is the point, with the endpoint half of the inclusion test held aside
throughout. So every claim about the lattice is about the exponent coordinate alone. The endpoints are where
`07`'s sixteen to thirty-four percent enlargement lives, and I did not touch them.

**What the classifier cannot see.** It is a test on value sets. Everything in section 2.2 and 2.3 is argued
from a separate instrument or from structure, and `i3`'s thermometer row decodes every bit pattern rather
than only the legal ones, which is a permissive reading and inflates that row's redundancy. A canonical
thermometer code restricted to its legal patterns is injective.

**Not covered at all.** Whether the sign domain changes any of it. Whether the closure results survive
nonzero bias, which `i2` holds at zero throughout while `i1b` measures that the bias matters for exactly one
representation. Whether a canonical exponent list is affordable at compile time at realistic exponent spans,
which is section 4.9 and is the measurement I would take first. Whether the endpoint half of the inclusion
test composes with the exponent half, which is `p3`'s stated omission. Whether the tapered region has a
parametric sub-shape worth naming, as opposed to being the whole remainder.

**What I could not settle.** Whether "no enumerations, ever" as ratified forbids a type-level canonical
exponent list. `p3` shows the list is inductive rather than enumerated and compiles and erases, and that is
evidence about buildability rather than about what the refusals meant. The refusals were about the container
derivation and a reader who takes `p3` as settling them is taking more than it gives.

**Everything here is unpriced.** No bench harness run bears on any of it. The `p3` assembly read is an
existence claim about erasure, not a measurement. Every number in this file is a count produced by a named
command in `08_probes/RUN.md`, and none of them is a magnitude.

**Owed under the two-expert rule, listed so nothing here is mistaken for agreed.** Every section is a first
read, and where this file agrees with `03`, `06` or `07` I read all three before deriving, so the agreement
is inherited. What is independent is the measurement. Specifically owed a second read: section 1.2's
identification of `ExponentForm` as a two-point sample of a function space; section 1.3's claim that
`canonical_exponent` was named and never given a home; section 3.3's claim that tapered formats are outside
the intersection closure; section 4.2's claim that gradual underflow is the meet of a fixed and a float;
section 5's boundary sentence, which is the thing I would most want attacked; and section 6's reading that
the design's shape set is a sub-meet-semilattice and not a sub-join-semilattice.

## 9. What appears to be op's, and in what order

Stated as questions, per `04`. None of this is a recommendation and none of it settles.

**One, and everything else is downstream of it: is the canonical exponent a member of the design, or are its
two values?** Section 1.2. The design ratified the name and carries two instances plus a nested axis for a
third point. The concept is a function. If the answer is that the two values are the design, then the family
question is settled by fiat and `03`'s readings C and D are the only live ones, which is a clean and
defensible position and should be stated as a choice rather than inherited as an omission. If the answer is
the function, then the join becomes total, the tapers come in free, and the design owes a type-level list and
an answer to whether that is what "no enumerations" forbids.

**Two: is gradual underflow understood as the meet of a fixed-point format and a float?** Section 4.2.
Measured two ways, once by enumeration and once by the trait solver. If it is, then the design already names
one point off its two curves, which makes the third row of section 1.2's table a precedent rather than an
exception, and it changes what admitting a fourth costs.

**Three: does the design want the mirror shape?** Section 4.3. The join of a fixed shape and a float is a
float below and a fixed grid above, and it is what the cross-kind join keeps producing. Naming it costs one
row and would make the two named families join-closed. Not naming it means the join refuses or a rule picks,
which is already on the table.

**Four: is the encoding a second axis, or is `SignIndexing` the only instance the design wants?** Section
2.2 and 4.4. Five surveyed representations differ from a plain fixed-point numeral in nothing but the datum
map, the design has already accepted a non-injective one for signed zero and decimal cohorts, and the
machinery that would carry them is the section-retraction triple the droplist already records adopting. The
cost is on the container derivation's stored-width clause, which is ratified text.

**Five: does the design commit to both directed rounding modes per operation?** Section 4.5. Measured at zero
failures over twenty thousand interval pairs for outward rounding and roughly a thousand for either
single-mode alternative. If it does, interval arithmetic is a consumer-side composition and excluding it
costs nothing. If rounding is fixed at the numeral, the exclusion is real and an interval consumer cannot be
served at all.

**Six, and it is a caution rather than a question.** This file agrees with `03`, `06` and `07` more than it
disagrees, and under `RULES.md` that is worth less than it looks, because I read all three before deriving.
The two places the agreement is genuinely independent are section 4.2, where an enumeration and a trait
solver reach the same meet by different routes, and section 3.3, where a slope argument closes a route that
`07`'s pricing left open. Everything else in sections 5 and 6 is a reframing, and a reframing that reads as
corroboration is the shape this panel has drifted on before.
