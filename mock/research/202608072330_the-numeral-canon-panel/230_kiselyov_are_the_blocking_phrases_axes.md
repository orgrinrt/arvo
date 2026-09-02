# 230. Kiselyov: which of the blocking phrases are axes, and what the ranking is a census of

## 0. The two gates

**Canon gate: passed.** I read `mockspace.toml`'s `canon_paths`, which is
`mock/registry/*.toml`, and reasoned from the rows rather than from the panel.
The rows that bear on this work and that I treat as governing are
`ruling::a_predicate_lists_only_what_holds` (stated, op's own words),
`ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked` (ratified
by op), `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`
(ratified by experts) and
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
(ratified by experts). Nothing I was asked to do conflicts with any of them. One
thing I was *not* asked about does, and it is in section 5.

**Test gate: run, with one part of it blocked and said so rather than skipped.**
`cargo mock check` reports the lint pipeline green at strict, and `build` and
`tests` as `no workspace members yet`, which is the honest report rather than a
count of zero dressed as a pass. I read the bodies of the two lints that guard
the surface I touch, `mock/lints/every_predicate_names_a_declared_axis.rs` and
`mock/lints/a_predicate_names_an_axis_once.rs`. Both suites are real: negative
controls that would fire, a differential arm walking `holds` as well as
`fails`, an arm for the separator inside a value, and registration checks.
Nothing tautological, nothing sampled. **And the suite ran, after queueing for
most of the sitting behind another worktree's cargo: 540 passed, 0 failed, 13
ignored.** Every one of the thirteen carries an `#[ignore = "catalogue: ..."]`
reason naming the case it holds open, which is the convention for a known gap
rather than a suppressed failure, and all thirteen sit in the five source-side
enforcer lints.

**One prior finding is closed by that run and worth saying so.** `226`'s section
5.1 reports that `no_std_enforcer.rs`, `no_alloc_enforcer.rs`,
`no_dynamic_dispatch.rs`, `no_runtime_grow.rs` and `arvo_bits_traits_only.rs`
carry no `#[cfg(test)] mod tests` at all. They do now, and they are the files
the thirteen catalogued cases live in. The finding was true when written and is
not true now.


One consequence of that lock worth stating because it shaped the sitting: every
commit here took minutes rather than seconds, so the probes are committed in
larger batches than I would otherwise have made.

## 1. What I did first, which was try to break the brief

Two claims in the brief and in `dimension.toml`'s own header are quotations, so
I opened the files rather than trusting them.

**Both committed outputs reproduce.** `183_probes/axis_census.sh` re-run against
the tree gives byte-identical output except for the six rows declared since it
ran: `declared axes (22)` against `(16)`, `radix` and `ambient domain` flipping
from `UNDECLARED` to `declared`, and the summary line moving from `19` declared
and `96` undeclared to `21` and `94`. `183_probes/unblock_value.sh` re-runs
byte-identical, and `183_probes/span_verdicts.sh` reproduces its 4 portable and
60 blocked. So the header's *"115 distinct keys the corpus predicates over, 19
of them declared here and 96 not, and 60 predicate spans that cannot be written
as a registry predicate at all"* is exact against the artifacts, and the
artifacts are the ones the scripts produce. Working tree clean after all three
re-runs, which is the check that the committed `.out` files are the ones those
scripts emit.

That is the good news and it is worth saying plainly before the rest: the
instruments do what they say, they carry controls that can fail, and one of
their controls (`U2` in `unblock_value.sh`) is documented as having been unable
to fail in its first version and fixed with a committed negative control. That
is better discipline than most of what I have read in this panel.

The rest of this section is about what those instruments were pointed at.

### 1.1 The census is a census of one persona

`axis_census.sh` names twelve governing files. Four of them produce **zero**
spans, which its own `C3` arm prints before aggregating, exactly so this is
visible rather than absorbed. The four are `161`, `164`, `173` and `176`, and
the reason is not a defect: `161:70-71` says it in its own words, *"Predicates
are carried at the establishing file and not restated in full here; the ledger
names the anchor that holds each predicate, per the never-widen-in-place
rule."* So topics nine and ten appear in the file list and contribute nothing,
and the files that actually hold their predicates were never in the sweep.

The eight that do produce spans are `119`, `122`, `132`, `136`, `138`, `146`,
`151` and `178`. **Every one of them is Leroy's.**

Against that: **60 files in this panel write a `holds for:` predicate, by 21
distinct personas, carrying 372 predicate paragraphs.** Leroy writes 12 of the
60. `230_probes/wide_census.out` and `230_probes/who_out/paras.tsv` carry both
numbers, and `wide_census.sh`'s `W1` arm checks that its extractor reproduces
`183_probes/keys.txt` byte-identically first, so the difference between the two
readings is a difference in the corpus read rather than in the instrument.

**So "the keys the corpus predicates over" is really "the keys eight files by
one persona predicate over."** That does not make the ranking wrong. It bounds
it, and the bound is not small, and nothing on the page says it.

I tried the obvious repair, which is to run the same extractor over all 60
files, and it does not work: the corpus writes predicates in at least three
dialects, italic paragraphs, blockquoted lines, and set notation with `∈` and
`×`, and the extractor is tuned to the first. The wide arm returns keys like
`> rounding`, `target > features` and `rounding ∈ {floor, > ceil, ...}`. I did
not build a multi-dialect extractor; I built an author-counter instead, which
does not need to split keys correctly because it counts who wrote a phrase
rather than what the phrase is. That is `230_probes/who_writes_it.sh`, and its
numbers are what section 3 rests on.

### 1.2 The family grouping ranks a regex, not an axis

`unblock_value.sh`'s second table groups phrases *"by the axis a phrase is a
spelling of"*, and the grouping is an awk regex over the phrase text. The
top-ranked family, `the term and declaration shape` at 27 spans, contains
`term shapes`, `declarations`, `restrictions`, `construction`, `structure
constants`, `declared grid step`, `placements`, `chains of depth` and
`carriers`.

Reading the spans those come from, at `230_probes/spans_for.sh`:

- `declarations = one-sided`, `declarations = a uniform magnitude bound on
  every component` and `restrictions = upper bounds in {1, 3, 7}` are **the
  operand window**, which `dimension::operand_window` now declares.
- `term shapes = every term at 2 and 3 leaf slots with every leaf
  identification` is a different thing entirely and is discussed in 3.4.

So the top family welds a live candidate to a spelling of an already-declared
axis, and its 27 is the sum of two unrelated quantities. The same holds of the
second family, `the cost-model population` at 25, which puts `selector`
alongside `120 tables per cell`. **A ranking whose unit is a regex over
spellings ranks spellings.**

### 1.3 The ranking cannot find the kind of axis that was last declared

`occupancy` is the most recent row in `dimension.toml`. It appears in **zero**
of the 372 `holds for:` paragraphs in this panel and in **zero** of the 527
predicate entries in the registry (`230_probes/who_writes_it.out`,
`230_probes/axis_uptake.out`). It was reached by two seats from opposite
directions, from what a derivation's output count does, and its own note says
so: *"a region something genuinely moves along, found by measurement rather
than posited."*

An axis found that way has rank zero on a frequency ranking of what the corpus
already wrote, by construction. **So the ranking answers "which spelling, if
declared, would let the most already-written spans be ported", which is a
useful question and is not the question "which axes does the design have."**
The header treats it as the second. It is not, and `occupancy` is the proof
inside the same file.

### 1.4 Two smaller instrument defects, both citable

**`axis_census.sh`'s `C2` message asserts something it never checks.** It
prints `radix: PASS, present as a bare key and no dimension row declares it`,
and the arm's whole body is `grep -qE "^ +[0-9]+ radix\$" keys.txt`. It never
reads `dimension.toml`. It printed that sentence for me, today, with
`dimension::radix` sitting in the file. What the arm actually checks, that a
short key is not being swallowed by an over-capturing span, is the check that
matters and it works; the sentence around it is now false.

**`span_verdicts.sh`'s hand-written correction at its own tail is wrong.** It
says three spans block only on the fragment `or unsigned with signed`, that
this is the tail of the single value `signedness = signed, or unsigned with
signed intermediates`, and therefore *"the honest portable count is 5 of 64
rather than the 4 printed above."* The splitting diagnosis is right; the
conclusion is not. Opening the source at `132:359-360`, `136:373-374` and
`138:49-50`, the value is a disjunction over **two** axes: the declared
signedness, and the signedness of the intermediate. `signedness: signedness =
signed` drops the second disjunct, and `signedness: signedness in {unsigned,
signed}` claims the non-commutation at plain unsigned, which the source denies.
There is no declared axis for the intermediate's signedness, so the span is not
portable. **The correction moves the count in the direction that overstates
portability, on the three spans that most need an axis.** More on this in 3.7.

## 2. The criterion I used, and where it differs from the file's own

`dimension.toml`'s header gives the test: *"An axis indexes a situation the
world can be in ... A parameter indexes a run."* I used it and it decides most
cases. It does not decide two, and I want the refinement stated as mine and
unratified before I use it.

**The refinement: an axis is a coordinate something is free to sit at more than
one value of.** The container premise is the worked example of why the header's
test alone is not enough. *"Behaviour stated over the declared width"* and
*"behaviour stated over the container"* are both situations a world could be
in, and a claim is true or false at each, so the header's test passes them
both. `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
is ratified and closes it: *"every operation the design declares is a function
of the declared width and never of the machine carrier."* The design is no
longer free there. **A ratified ruling closing a fork removes an axis**, and
what is left is a constant of the design, which a predicate does not carry any
more than it carries the fact that arithmetic is deterministic.

The freedom can sit in three places and all three count: the consumer's choice
(`signedness`, `rounding`, `strategy`), the consumer's data (`ambient_domain`,
and `coupling` in 3.6), or the environment the thing was built and run in
(`toolchain`, `build_profile`, `target_features`). What does not count is a
choice the canon has already made, and what does not count is a number that
indexes a run.

This is the interventionist criterion, and it is worth saying so because it is
also, exactly, what
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` already
obliges: *"a `construction` entry obliges the row's `evidence` to name an
instrument that varied that axis and found no movement."* Varying a coordinate
and watching whether the outcome moves is Woodward's invariance-under-
intervention in the form a shell script can run. The registry got there without
the vocabulary, which is a point in the registry's favour and is why I am not
proposing to change it, only to use it deliberately: **the way to settle
whether a phrase is an axis is to vary it and see, and where nobody can vary it
because it has no values in the situation, that is a different finding and
section 4 is about it.**

## 3. The verdicts, one phrase at a time

The `pers` and `files` numbers below are from `230_probes/who_writes_it.out`,
counting distinct personas who write the phrase as a predicate key across all
60 predicate-bearing files rather than occurrences in Leroy's eight. Its
controls: `threads` at 19 personas passes the arm that the paragraph grab is
not broken, `phase_of_the_moon` at zero passes the arm that a phrase nobody
wrote produces nothing, `discharge check` at one passes the arm that the count
does not saturate, and every keyed count is a subset of its loose count.

**One arm of that probe failed and the failure was mine.** I wrote down before
the run that `weight grid resolution` was `146`'s alone, from the narrow sweep,
and the wide read shows three personas write it. The arm stays in the script
with its failure visible, and the purpose it served, that the count does not
saturate, is carried by `discharge check` instead. I mention it because it is
the one place my expectation about the corpus was wrong in a checkable way, and
that is what a control is for.

### 3.1 `declarations`, `restrictions`: an axis, and it is already declared

**2 personas, 3 files** for `declarations`; 1 and 2 for `restrictions`.

The values are `one-sided`, `one-sided exhaustive`, `a uniform magnitude bound
on every component`, `upper bounds in {1, 3, 7}`. Every one of them says which
sub-range of the representable set the operands are declared to be drawn from.
That is `dimension::operand_window`, word for word: *"Whether the operands are
declared to lie within a restricted sub-range of the format's representable
domain."*

**Verdict: not new axes. Two spellings of one declared axis.** What it rests
on: the four values above against the `operand_window` row's own `what`.

**But the grammar as declared cannot write any of them.** The row admits
`operand window = declared non-negative`, `= full range`, and `any`. That is a
two-value enumeration and the corpus writes bounds (`upper bounds in {1, 3,
7}`), per-component bounds (`a uniform magnitude bound on every component`) and
one-sidedness without a side. **Widening a grammar invalidates nothing**: no
written span becomes unparseable, so it is an append in the sense the header's
own append-only rule means, unlike a rename or a delete. That is a repair a
second reader can agree to cheaply and it unblocks nine spans across three
files.

**What would change my mind:** a reading on which `one-sided` names something
other than a restriction on the operand range. I looked for one at `119:426`
and `119:542`, which discuss what a declared restriction can discharge, and
found the opposite: the whole discussion is about operand ranges.

### 3.2 `overflow limit read at the declared width`: not an axis, and declaring it would be worse than useless

**4 personas, 7 files**, and notably **zero** of them write it with an operator
after it. It is written as a bare assertion inside the span, not as
`key = value`, which is itself the reading: the authors were stating a premise,
not naming a coordinate.

`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
is ratified by experts and says *"every operation the design declares is a
function of the declared width and never of the machine carrier, so arithmetic
and encoding are stated over the declared width."* Its `answers` field names
`the_container_premise`. The design is fixed at one value.

**Verdict: not an axis. A constant of the design, restated in seven files that
predate the ruling that fixed it.** And declaring it would be actively harmful,
which is a stronger statement than "unnecessary": it would make writable a
region, *overflow limit read at the container width*, that a ratified ruling
forbids the design from occupying. The dimension vocabulary would then be able
to express a predicate the canon says is uninhabited, and a checker would
accept it.

**What would change my mind:** a demonstration that a declared numeral's
overflow limit can honestly sit at the container width somewhere the ruling does
not reach. I do not think one exists after 225, and 225's section 4 works
through the downstream clauses one at a time.

### 3.3 The whole cost-model family: no new axes, and the region-bearing part is `strategy`

**`cost coordinates` 7 personas / 9 files. `arms` 9 / 10. `selector` 3 / 5.
`weight grid resolution` 3 / 5.**

I expected the header's classification of `cost coordinates` and `arms` as
parameters to be under-supported, because it is illustrated with a phrasing
that traces to one note (`proposal::the_rationalisability_counts_on_the_committed_carrier_table`, *"every one of which is a bench
parameter rather than a design axis"*), and one unratified note is not evidence.
Reading the wider population settles it the header's way and adds the
mechanism. The non-Leroy usages are `100_xu` and `101_wronski`, and they read:

> regions = 6, arms in {5, 6}, cost coordinates = 3 (median algo_ns, declared
> bytes per element, interquartile range), cost source = committed
> bitpack-carrier-width_n CSVs, 80 samples per arm

`arms = 5`, `80 samples per arm`, `2000 bootstrap resamples`, `seed 20260814`.
Nothing about the world is different at five arms rather than six. **These are
run parameters and nine personas write them that way.**

The interesting half is what happens to the part of the family that *is*
region-bearing. `146 #18` writes `cost coordinates any; arms any; weights any
positive; baseline any arm with positive weighted cost`, and its own tag says
*"Argument kind: order-preservation theorem, which is why this predicate lists
no width, no population and no resolution."* So `weights` and `baseline` there
are the theorem's hypotheses about the weighting, and the weighting is not a
free-floating quantity: `proposal::a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it` says a strategy **is** *"a declared
semantics together with a weighting over the cost coordinates of the arms that
realise it."*

**So the weighting, the baseline, the coordinate set and the selector are
components of a strategy, and the axis that indexes them is `strategy`, which
is declared.** The `selector` fork is the one place this is genuinely open, and
I cannot settle it; see 6.1.

**Verdict on the 25-span family: zero new axes.** Its parameters are
parameters, and its region-bearing content is a declared axis that nobody
writes. That last clause is not rhetorical, and section 5.2 is what it costs.

**What would change my mind:** evidence that the design ships one selector and
one weighting form for all strategies, in which case those are design constants
like 3.2 rather than strategy components, and the family contains zero axes for
a different reason.

### 3.4 `term shapes`: a real axis is in here and it is not term shape

**2 personas, 3 files** (`116` Leijen, `119` and `122` Leroy).

The values are compounds: `every term at 2 and 3 leaf slots over the signature
in play`, `every term at 2 and 3 leaf slots with every leaf identification`,
`every term at 2 and 3 leaf slots with every leaf identification and 120 of
2025 sampled at 4`. Each welds a region to a warrant, and the ratified marker
notation splits them cleanly:

```
<axis>: leaf slots in {2, 3}: exhaustive, every term over the signature in play
<axis>: leaf slots = 4:      swept, 120 of 2025
```

Two of the three coordinates inside are already declared. Depth is
`chain_length`. Operator arity is `arity`, and `119 #6` writes `arity in {2, 3}`
in the same span, so the corpus already treats them as separate. Leaf slot count
is close enough to `chain_length` that I would not declare a row for it alone.

**What is left is not close to anything declared, and it is load-bearing.**
`leaf identification` means whether a term's leaves are distinct occurrences or
the same value repeated: `(a+b)*(a+b)` against `(a+b)*(c+d)`, same depth, same
arity, different aliasing. And the corpus has a law whose stated condition is
exactly that:

> a certificate that a refusal by that check is honest needs only that every
> leaf occurs at most once

`119:491`, repeated verbatim at `122:475`, derived at `114:380` and `114:974`,
and used at `111:1237` and `111:1383` as a condition on a twelve-row result.
Four files, two personas at minimum.

**Verdict: `leaf_aliasing` is an axis, on the corpus's own evidence, and `term
shape` is not the right name for it.** Values would be `distinct` and
`repeated`, or a finer count. Under the absence rule the certificate claim as
written holds nowhere leaf aliasing exists, which is everywhere terms exist,
and the aliasing appears in the predicate only inside a coverage phrase welded
into a compound value.

**The one thing that could have killed this is settled, and it settles the
other way.** I wrote the paragraph above expecting to leave it open, on the
worry that the design's term language might not admit aliasing at all, in which
case the axis has one value and is a constant like 3.2. It admits it, and the
corpus says so at `111:1391`, in Jhala's file rather than in any of the three
this phrase was ranked from:

> Sufficient and not necessary: it does not fire on `x - x` or `(x + y) - x`,
> both of which are exact.

`x - x` and `(x + y) - x` are terms with repeated leaves. So the design's terms
take both values of the axis; F111-15's condition *"every leaf occurs at most
once"* restricts a stated finding to one of them; and the same file names two
witnesses in the other where the property holds anyway and the predicate does
not fire. **A coordinate the corpus's own terms take both values of, on which a
stated finding is restricted to one, is an axis under any of the three tests in
this file.** Two personas write about it, Jhala at `111` and Leroy at `114`,
`118`, `119` and `122`, which is the count `ambient_domain` and `radix` cleared.

**What would change my mind now:** a showing that leaf aliasing is a function of
depth and arity, so that `chain_length` and `arity` together already pin it.
They do not, on `(a+b)*(a+b)` against `(a+b)*(c+d)`, which agree on both.

### 3.5 `discharge check`: the claim's own content, sitting in the region slot

**1 persona, 2 files.**

`discharge check = root under a homomorphism and per node otherwise` appears in
`119 #7` and `122 #6`, and the claim those spans predicate is, from `119:495`:

> So neither the check nor its certificate is chosen. Both are consequences of
> the character.

The claim is that the check follows from the character of the map. The span puts
the check's shape in the region. **That is the conclusion written into its own
hypothesis**, and the predicate then says nothing, because the region already
assumes what the sentence asserts.

**Verdict: not an axis. A category error in three files, and the axis behind it
is a different thing:** whether the realisation map is a homomorphism for every
operation in the derivation, which `119` calls the *character*. That is a
situation a design sits at, claims are true at one value and false at the other,
and the corpus states one of its halves in a prose heading rather than in a
predicate: `119`'s second predicate is introduced as *"The non-homomorphic half
alone"*, and `183_probes/axis_census.out` shows that as a key,
`The non-homomorphic half alone holds for: as 119 4.7's second predicate`, which
is the extractor faithfully recording that the region lives in the sentence.

**I am not proposing `map_character` on this evidence.** One persona, two files,
and I have not read `112` or `114`, which is where the character was derived.
**What I am reporting is that a region is being written in prose next to a
predicate that cannot hold it, which is the exact failure mode
`dimension::occupancy`'s own note names**, and that the phrase blocking those
spans is the wrong end of it.

### 3.6 `coupling`: an axis, at the same author-count the declared set already clears

**2 personas, 5 files** (`128` Knuth, `131`/`132`/`136`/`138` Leroy).

Values: `coupling = any point of the Fréchet interval` and `coupling in
{comonotone, independent}`. That is the dependence structure of the operand
values: whether the elements of a workload move together or are unrelated.

It is a situation the consumer's data is in, not a run parameter. A claim about
whether errors cancel or accumulate across a reduction is true under
independence and false under comonotone dependence; that is the whole content of
`132`'s two spans, one of which is a construction over the entire Fréchet
interval and the other a sweep at two of its points. **An axis taking both `any`
and a two-element set in the same file, from the same author, over the same
subject, is behaving exactly the way a declared axis behaves.**

Nothing declared carries it. `access_pattern` is *"how the values are reached"*,
which is about the access; `ambient_domain` is what the values approximate.
Neither says how the values are jointly distributed.

**Calibration, so this is not a lone judgement:** `dimension::ambient_domain` is
written by 2 personas in 2 files, `dimension::radix` by 2 keyed personas, and
`dimension::operand_window` by 3. **The declared set already contains axes at an
author-count of two.** `coupling` clears the bar the existing rows clear.

**Verdict: an axis.** `element count` in the same spans is `chain_length` under
another name and wants a keyword rather than a row; `fraction = 1/3` is a
parameter of the particular distribution swept and belongs in a `swept` clause.

**What would change my mind:** a showing that arvo never sees a workload, only
individual operations, so a dependence structure has nowhere to live in its
model. That would be a real objection and I have not tested it. It would not
make the finding false, it would move it downstream, which is a different
answer with a different fix.

### 3.7 The intermediate is a second format and one of its coordinates is declared

**`F_intermediate` 1 persona, 3 files.** And this is where 1.4's second defect
lands.

Two things that look unrelated are the same gap:

```
132 #3   rounding in the nearest members; F_exact in {4, 5};
         F_intermediate in {2, 3}; F_final in {1, 2}; ...
132 #5   ... signedness = signed, or unsigned with signed intermediates; ...
```

A staged narrowing has three fraction widths and the declared format has one
`fraction_width`. A mixed-sign chain has a declared signedness and an
intermediate signedness, and `dimension::signedness` has one entry. In both
cases the corpus is trying to state a coordinate of the **intermediate**, and
the notation has one entry per axis for the **declared** format.

`dimension::accumulator_width` is already the first coordinate of that second
format: *"How wide the intermediate a chain accumulates into is, where it
differs from the declared width."* It carries a total width and nothing else.

**Verdict: one gap, not two, and the shape of the fix is a design call I will
not make alone.** Either the intermediate gets its own coordinates row by row
(`intermediate_fraction_width`, `intermediate_signedness`, beside the declared
`accumulator_width`), or the notation gains a way to state a second format's
whole coordinate set, which is a bigger change and touches the ratified entry
grammar. **The first is additive and needs no ruling; the second needs one.**

What makes this more than a spelling problem: `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`'s
own note already records *"six values sides bind more than one thing, two
packing an undeclared axis inside a legitimate slug and the rest stating two
regions on one axis inside one array element"* and says *"the shipped arm checks
the slug side only and sees none of them."* The three `or unsigned with signed`
spans are exactly the first of those two classes, and `span_verdicts.sh`
declared them portable.

### 3.8 `feature gates`: an axis, on the `toolchain` row's own precedent

**6 keyed personas, 9 files.** Written as `feature gates = none` and
`feature gates = 0`, always beside `toolchain = the pinned nightly` and
`edition 2021`.

`dimension::toolchain` covers the compiler and the edition. It does not cover
which unstable features the compilation was allowed to use, and neither does
`target_features`, which is what the machine offers rather than what the
language surface was.

The precedent is the `toolchain` row's own note, which is the argument for this
one restated: *"Whether it should instead fold into those two neighbours was
raised and left as a fork; it is declared because the corpus writes an edition
and a rustc version as their own coordinates and neither neighbour has anywhere
to put them."* The corpus writes feature gates as their own coordinate, six
personas do it, and neither neighbour has anywhere to put it.

It is region-bearing rather than decorative: *"compiles gate-free on the pin"* is
a probe row's name in this registry, and the whole content of that finding is the
value `feature gates = none`. A construction needing `generic_const_exprs` and
one needing nothing are different situations and a claim is true at one and false
at the other.

**Verdict: an axis.** `crate type = library`, `recursion limit in {16, ...}`,
`#![no_std]` and `float types = none` sit in the same spans and I am **not**
taking them: `crate type` and `#![no_std]` are fixed by the operating
constraints, `recursion limit` reads as a swept parameter in the one span that
varies it, and `float types = none` is a statement about the construction rather
than about the environment.

### 3.9 `observation sets` and `assignment set`: one decomposes, one is the coordinate a ratified ruling names

**2 personas, 3 files** for each.

`assignment set = rounding {floor, toward zero} x overflow {wrap, saturate
both, saturate high only} x intermediate {stepwise, exact}` is a cross-product
of `rounding`, `overflow_policy` and something close to `accumulator_width`. It
is three declared axes packed into one entry, which is the second violation
class the ratified note names. **Verdict: not an axis; decompose it.**

`observation sets = every subset of {add, subtract, multiply, multiply-add,
multiply-subtract}` is not that. It says which operations the *signature*
exposes, which is different from `operation`, which says which operation the
claim is about. And it is the coordinate a ratified ruling puts behaviour
relative to:
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
says *"Identity saturates at the declared signature set"* and
`ruling::observability_is_relative_to_a_declared_signature` is in its
`ratifies` list.

**If behaviour is stated per declared signature, then the declared signature is
a coordinate every behavioural claim is relative to, by exactly the argument
that makes `operation` an axis.** Whether two markers are one primitive is true
at one signature set and false at another; `146 #2` sweeps the whole powerset
of five operations precisely because the answer moves.

**Verdict: an axis, and the strongest warrant in this file, because it comes
from a ratified row rather than from a phrase count.** It is also the clearest
case of the ranking under-serving: three spans, buried in a nine-span family,
against a ratified ruling that makes it the thing behaviour is stated per.

**What would change my mind:** a reading on which the declared signature set is
part of the claim's *subject* rather than its region, the way a theorem is
about a group rather than holding in a region of groups. I considered it and
came down the other way by analogy with `operation`, which is a subject in
exactly the same sense and is declared. **A second reader should push here,
because the analogy is the whole of my argument.**

### 3.10 `value set finite with at least two elements`: a hypothesis, not a region

**2 spans**, `119 #1` and `122 #1`, both opening a theorem stated abstractly:
`value set finite with at least two elements; operations including addition;
domain containing a complete residue system`.

This is the hypothesis of a general theorem, not a region of arvo's space: every
arvo format is finite with at least two elements. Inside arvo it reduces to
`total_width: W >= 1`, which is a declared axis at a stated span.

**Verdict: not an axis.** What it exposes is a different thing worth naming: the
notation has no way to say *this holds in a wider setting than arvo, of which
arvo is an instance*, so a general theorem's hypotheses get written where a
region goes. That is adjacent to 3.7 and to section 4 and I have not pursued it.

### 3.11 What I did not take

`weight grid resolution`, `cost tables drawn uniformly from`, `200 independent
target pairs`, `120 tables per cell`, `decision procedure`, `no exact duplicate
arms`, `positions 0 to 255`, `2000 bootstrap resamples`, `seed 20260814`: run
parameters under the header's test, and the wide read supports it rather than
resting on one note. `structure constants in {-1, 0, 1}`, `multiplier schedules
{[1,-1,2], [2,-1,1]}`, `dimension in {2, 4}`: parameters of one linear-algebra
sweep in `119 #10`; possibly a region for that claim, one file, not enough.
`threshold`, `keying axis`, `decorrelation measured both as ...`, `input shape`:
the stochastic family's remainder, which I read as parameters of the particular
family swept rather than coordinates, on one persona's spans.

## 4. The compile-time item: two questions welded, and they have different answers

The open item, from `226`'s section 5.2: *"The `dimension` vocabulary has no
occupancy axis and no compile-time axis ... The second means every compile-only
finding in this panel is written in a notation that says it holds nowhere."*
The argument is at `226`'s F2:

> F2 is a compile result: nothing runs, so `build_profile` has no value to take,
> and `threads` and `target_features` have nothing to range over. Under the
> discipline an absent axis says the finding holds nowhere that axis exists,
> which reads as a much stronger negative than I mean. The notation has three
> region-free spellings and none of them is compile-only.

**That sentence contains two problems and they want opposite answers.** The
brief asks whether the repair is an axis, a value on an existing axis, or
something the notation already provides. It is the third for one half and the
first for the other, which is the shape this canon's forks keep turning out to
have.

### 4.1 The half the notation already handles

*Axes that have nothing to range over in the situation the claim is about.*

`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` is ratified and
gives `<axis>: <span>: construction, <clause>`, where `construction` marks *"an
axis that cannot enter the argument at all, with the clause saying what makes it
unable to."* An axis with no values in a compile-time situation cannot enter the
argument. That is what the token is for.

**And the corpus already writes it, twice, in the pre-ratification form.** From
`230_probes/who_writes_it.out`'s source data:

```
"threads: threads any, the equalities being decided at compile time"
"threads: threads any, the refusal being a type-check outcome that precedes execution"
```

`dimension::access_pattern`'s own note says the same thing from the other side:
*"A correctness claim untouched by it writes `access pattern: any`, with the
structural argument as the warrant, exactly as a compile-time result writes
`threads any`."*

**So the spelling exists, is ratified, and has been used.** `226` did not miss a
notation feature so much as arrive at the same conclusion the notation had just
reached, from a direction that made it look like a gap.

**What genuinely remains is narrow and real: the `evidence` obligation.** A
`construction` entry obliges `evidence` to name an instrument that varied that
axis and found no movement. **You cannot vary the thread count of a type-check.**
The obligation is unsatisfiable exactly where the warrant is most obviously
correct, and `226` says so in its own words about a different axis: *"plainly-
cannot is not a warrant."* The ratified row anticipates part of this, *"Some
constructions have no differential control available and never will, and the
`evidence` obligation forces those to either a weaker warrant or a probe written
to satisfy it"*, but it treats that as a price rather than as a class. **The
class is nameable: an axis with no values in the situation is not merely
uncontrollable, it is inapplicable, and those are different.** **And it is
narrower than it looks: two of the four axes `226` names are not in it at all**,
because `target_features` and `build_profile` are varied by compiling twice and
that is an instrument. 6.3 works it through and reports where the residue
actually is.

### 4.2 The half that is a genuine undeclared axis, with a compiled witness

*A claim whose truth differs between const evaluation and execution.*

This is not the same question and it is not answered by a warrant clause,
because here the axis has two values and the claim's truth moves between them.

`dimension::build_profile` is the only declared axis that mentions const
evaluation, and it welds two channels into one sentence in its `moves` field:
*"the overflow panic plus a const-eval refusal is such a channel and neither
exists in a release artifact."* That is a claim about rustc and it is checkable
in one file. `230_probes/p1_binding_time/matrix.sh` checks it, on the pinned
nightly, with the same expression written on both sides and a negative control
per operation:

```
op     site               debug-assertions     outcome
add    const evaluation   on                   REFUSED error[E0080]
add    const evaluation   off                  REFUSED error[E0080]
add    execution          on                   PANIC attempt to add with overflow
add    execution          off                  value 44
sub    const evaluation   on/off               REFUSED error[E0080]
sub    execution          on                   PANIC attempt to subtract with overflow
sub    execution          off                  value 255
shl    const evaluation   on/off               REFUSED error[E0080]
shl    execution          on                   PANIC attempt to shift left with overflow
shl    execution          off                  value 2
```

The negative controls, the same expressions with operands that do not overflow,
produce values in every cell. Every stderr is committed beside the script and
its byte count printed, so an empty one is visible rather than assumed.

**The finding that survives everything below.** At `debug-assertions = off`, the
same expression at the same width, signedness, container, overflow policy,
toolchain, target features, build profile and thread count is **refused** at one
evaluation site and **evaluates to 44** at the other. Every declared coordinate
is held fixed by construction, since it is one file built twice with one flag
changed. **So no declared axis separates const evaluation from execution**, and
that is a universal refuted by a witness rather than a positive claim needing
three.

**Verdict: an axis, and I would spell it `evaluation_site`, with values
`{const evaluation, execution}`.**

**Not `binding_time`, and the distinction matters enough to state.** The registry
already uses that word, and uses it correctly, for something else:
`ruling::the_predicate_is_whatever_is_available_at_const_time` sits on
`topic = "binding_time"` and is about *const-availability*, which is exactly
binding-time analysis in the partial-evaluation sense, the static/dynamic split
of Jones, Gomard and Sestoft. Binding time asks **when a value is known**.
`evaluation_site` asks **where an operation ran**, and the two come apart: an
expression can be fully static and still be evaluated at runtime, which is
precisely the pair the probe builds. Using one word for both is the failure the
vocabulary rules exist to stop, and it would be the worse of the two to
introduce, because the registry's existing use is the older and better-attested
one.

#### 4.2.1 I attacked my own result with a second mechanism and it broke one sentence

The matrix's three operations all route through the same const-evaluation
arithmetic check, so they are three witnesses and not three independent ones. I
went looking for a second mechanism rather than leaving that as a caveat, and
`230_probes/p1_binding_time/second_mechanism.sh` is what came back. It is a
different shape rather than a different operation: **one `const fn` body, called
at both sites with the same arguments**, which is arvo's own shape and not the
free-standing const the matrix used.

```
args     site               da     outcome
200,100  const context      on     REFUSED error[E0080]
200,100  call at runtime    on     PANIC attempt to add with overflow
200,100  const context      off    value const 44
200,100  call at runtime    off    run 44
```

**Read the third line.** A `const fn` whose body contains a bare `+`, evaluated
in const context, **produces 44 at `debug-assertions = off`**. The matrix says a
literal in a const item refuses at both profiles. These are two different
mechanisms and I had generalised from one of them.

**What that costs me, stated plainly rather than absorbed.** The sentence *"the
two coordinates are independent"* was in my first draft of this section and it
is **false**. It is true of the literal-in-a-const shape and false of the
`const fn` shape, where the two sites agree at `off` and differ at `on`. The
finding above survives, because it needs one witness and has one. The
independence claim does not, and I withdraw it. **This is the same defect I
report against the ranking in 1.1 and against my own reader in 5.6: a
generalisation from a population of one.** Three instances of that in one
sitting, one of them found by a second mechanism I built specifically to attack
myself, which is the only reason it was found at all.

#### 4.2.2 The flag that acts is not the flag the declared axis names

Arm C separates `debug-assertions` from `overflow-checks`, which the first sets
by default and which arms A, B and the matrix therefore cannot tell apart:

```
debug-assertions       overflow-checks        const-context outcome
on                     on                     REFUSED error[E0080]
on                     off                    value const 44
off                    on                     REFUSED error[E0080]
off                    off                    value const 44
```

**The outcome tracks `overflow-checks` in all four cells and is constant in
`debug-assertions`.** Arm D runs the same 2x2 on the literal shape and refuses in
all four, so the two shapes are demonstrably different mechanisms rather than one
seen twice: the literal is caught by a deny-by-default lint no flag reaches, the
`const fn` body by const evaluation of MIR that only carries an overflow check
when `overflow-checks` is on.

`dimension::build_profile`'s grammar names `debug-assertions` and `opt level`.
**It does not name `overflow-checks`, which is the flag that moves the answer.**
The two are confounded by default, so a corpus that only ever sets the profile
cannot tell which it measured, and the declared axis names the proxy.

#### 4.2.3 What this says about the compile-time premise, reported outside the question

arvo's stated posture is that invalids are caught at compile time and there is
never a runtime check. **Arm C says the compile-time catch, for a `const fn`
body containing a bare operator, is conditional on a codegen flag**: a consumer
writing `const X = a.add(b)` in a release build gets the wrapped constant with
no diagnostic at all.

**This is not a complaint, because arm E shows the guarantee is recoverable by
construction and shows exactly what recovers it.** The same operation, written
so the overflow is a value the body inspects rather than a check the backend may
not have emitted:

```rust
match a.checked_add(b) {
    Some(v) => v,
    None => panic!("operand sum leaves the representable set"),
}
```

refuses in **all four** cells of the same 2x2, and its control, the same body
with operands that fit, produces a value in all four. So the design obligation
is precise: **a compile-time refusal has to be carried by the construction and
may not be inherited from const evaluation of a bare operator**, and which of
those two a body does is invisible in the source until somebody compiles it at
`overflow-checks = off`.

I did not go looking for this and it is the largest thing the probe found. It
belongs to whoever owns the primitive rather than to an axis review, and the 2x2
plus arm E is enough to act on without re-deriving anything.

## 5. Findings outside the question, harshly where they deserve it

### 5.1 Three registry rows carry a stated reason that is false against the registry

`230_probes/stale_absence.sh`, with a planted stale sentence and a planted live
one as controls, and a silence check against `strategy.toml`:

- `law::distributivity_of_multiplication_over_addition`, `gap = "... is on neither list, because the
  declared operand window is not a declared axis ..."`
- `law::coherence_of_a_reduction_onto_its_induced_operation`, `gap = "... is on neither list, because the
  operand window that distinguishes it is not a declared axis."`
- `proposal::the_realisation_map_factors_into_quantisation_and_range_policy`, `"... its region is
  stated over the ambient domain and nothing declares that axis."`

All three are false. `dimension::operand_window` and `dimension::ambient_domain`
exist. Two further sentences match the same pattern in `dimension.toml` itself,
on `dimension::operand_window` and `dimension::occupancy`, and both are **correct**, because they are written in the
past tense: *"no axis existed to state it on"*, *"neither could gate on it"*.

**Tense is the discriminator and it is mechanical enough for a lint.** A
past-tense account of why a row was written stays true forever. A present-tense
claim about what this file contains is falsified the moment the file gains a
row. **This is the corollary to the semantics the header settles**, and the
header does not draw it: declaring an axis reveals a narrowness rather than
creating one, which is right about predicates, and says nothing about prose,
which is where the damage lands instead.

**Two lints are owed and I could not land either.** The round is at `TOPIC` and
a `mock/lints/*.rs` file is a source edit, which the phase gate refuses and
should.

The first is this section's. Its predicate: for every registry field, a sentence
matching *"is not a declared axis"*, *"no declared axis"* or *"nothing declares
that axis"* in the present tense, whose subject resolves to a slug in
`dimension`, is a finding. Its controls are in the shell probe already,
including the one where the automated axis-attribution arm gets two of five
wrong and the tense arm gets five of five, which is left visible in the output.

The second I owe because of something that happened while writing this file, and
it is worth the paragraph. **`no-line-citation-into-a-registry-file` refused my
first commit of this file**, because I had written six citations of the form
a registry file name followed by a line number, and the ceiling is 45. The lint is right: the
registry gains rows constantly, a line citation keeps resolving while naming
something else, and its message says the repair is to write slugs rather than to
raise the number. I repointed all six. **Then I checked by hand that every
`ns::slug` in this file resolves to a row, and one did not**: I had written
`proposal::a_strategys_weighting_is_rationalisable_from_the_arms_it_selects`,
which exists nowhere. I had invented it while converting a line citation,
sitting inside a file whose section 5.6 is about instruments agreeing with
themselves.

**Nothing would have caught it.** `a_citation_names_a_target` reads citation
fields on registry rows and does not read prose in `mock/research/`, which its
own module documentation is explicit about. So the second owed lint is: **a
`<namespace>::<slug>` written in a panel file resolves to a row in that
namespace.** One grep, thirty slugs in this file, and it found a fabricated one
inside a minute. The repository has a hard gate against a citation *shape* it
distrusts and none against a citation that names nothing, in the tier where the
argument lives.


### 5.2 No predicate field in the canon names a strategy, and one entry pretends to

`230_probes/axis_uptake.out`, over 527 entries in 91 predicate fields:

```
access_pattern   0      ambient_domain   0      occupancy        0
strategy         1      accumulator_width 1     alignment        2
radix            2      operand_window   3      integer_width    4
toolchain        7      build_profile   11      container       14
rounding        17      chain_length    20      target_features 25
threads         36      arity           53      overflow_policy 61
total_width     65      fraction_width  66      signedness      66
operation       73
```

The single `strategy` entry, at `proposal::a_resolved_strategy_never_inherits_a_composed_operations_laws_but_dominates_them_monotonically`, reads
`strategy: strategy = product order over two independent axes, generic labels
rather than Hot/Warm/Cold/Precise`. `dimension::strategy`'s grammar admits
`S = <name>` or `S in {<set>}`. That value is neither. **So the registry's only
strategy entry does not name a strategy, and no predicate field in the canon
states which strategy any finding was established under.**

Under `ruling::a_predicate_lists_only_what_holds`, in op's words, *"unmeasured
or unknown does not list in the predicate ... unstated ... implicitly mean not
true"*, and `dimension::threads` states the consequence in the registry's own
voice: an absent axis means *"the claim holds in no situation where threads
exist at all."* Every arvo numeral carries a strategy. **So every predicated row
in the canon currently claims to hold nowhere in arvo.**

I want to be careful about what that is and is not. **It is not a notation
defect.** The notation is working: it is reporting, accurately, that the corpus's
findings were established on models without strategies and that nobody has
established them under one. **It is an unpaid debt**, and the honest spelling
exists, `strategy: strategy in {the ones you ran}`, and it costs a measurement
per row rather than a ruling.

**One part of it is a notation defect, and it is small and fixable.**
`dimension::strategy`'s grammar bans `S any` outright, on the ground that it
*"quantifies over a set op has stated is open."* That reasoning is right about a
**swept** universal and wrong about a **constructional** one. A `construction`
clause saying the strategy cannot enter the argument does not quantify over the
set; it says the axis is irrelevant, which is what the ratified token was
introduced to express. **So the ratified warrant ruling licenses a spelling that
`dimension::strategy`'s grammar forbids**, and the same holds of
`dimension::operation`. The gap is real and it is exactly the class of finding
that has nowhere to go today.

### 5.3 A ratified ruling dissolved the container premise and two rows still say it is blocking

`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
is `rung = "ratified"`, `ratified_by = "experts"`, `answers =
["the_container_premise"]`, and its own note says *"there are no branches now,
and the clauses can be written."*

- `topic::the_container_premise` still reads *"Blocking: no wording of several
  downstream clauses is true on both branches."*
- `dimension::container`'s note, still reads
  *"the container premise, and it is blocking: no wording of several downstream
  clauses is true on both branches. See `topic::the_container_premise`."*

Both are pointers to a fork that a ratified row closed. **A `dimension` row's
note telling a reader an axis sits on a blocking premise is the worst place for
this**, because it is the file every predicate is written against, and it will
be read as a live reason not to state a region.

### 5.4 The declared-and-unused set, and what one of those rows was declared for

`access_pattern`, `ambient_domain` and `occupancy` have zero entries.

`ambient_domain` is the one that stings. Its own `moves` field says *"It is the
single largest blocker in the corpus's unwritable predicates, sole blocker of
four spans and present in eleven."* It was declared to unblock those spans.
**Nothing has been ported onto it.** Meanwhile `proposal::the_realisation_map_factors_into_quantisation_and_range_policy`
still says nothing declares it, so a reader of that row is told the axis does
not exist while a reader of `dimension.toml` is told it is the biggest unblocker
in the corpus.

`operand_window` is the counter-example and shows the act done properly: three
new `law` rows were written carrying `operand_window: declared non-negative`.
But the three older rows the declaration's own note names as motivating it,
`additive_associativity_under_saturation`,
`distributivity_of_multiplication_over_addition` and
`coherence_of_a_reduction_onto_its_induced_operation`, still carry the `gap`
fields from 5.1. **Half the act, on the one axis where the act was attempted.**

### 5.5 What a predicate actually names, measured

91 predicate fields, 22 declared axes, **mean 5.8 named, so 16.2 silent on
average, and the richest field in the canon names 13**. The distribution has a
mode at 6 (28 fields) and a tail at 11 (9 fields) and 13 (2 fields).

Under the absence rule that is what the canon currently claims. I am not
proposing that every row name 22 axes; several of the silences are correct and
the notation gives no way to be quiet politely, which is the design and op
refused a fourth state for good reasons. **I am reporting the number because
nobody had, and because a discipline whose central rule is that silence is the
strongest negative statement available should know how much of it there is.**

### 5.6 Three instrument defects of my own, reported because they are the interesting ones

**My first uptake reading said `strategy` had zero entries and the registry had
517.** It was wrong. My reader matched an entry by its own line, `^ *"<slug>: `,
which cannot see a TOML array written on one line, `predicate = ["a: x", "b:
y"]`. Three such arrays exist, holding ten entries, one of which is the only
`strategy` entry in the registry.

**Two of my controls passed on that wrong reading**, including one comparing my
per-field total against a flat grep and finding 517 = 517. They agreed because
they shared the blind spot. What caught it was opening the four fields the
distribution reported at zero axes, which is an arm neither script had, and
which I ran only because zero looked odd.

The corrected reader gives **527**, and
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`'s own note says
*"139 of 527 entries drop the axis word from the values side entirely."* That
number was arrived at by a different seat with a different reader, so agreeing
with it is a check rather than a restatement, and it is now a control in
`230_probes/axis_uptake.sh`.

**I am reporting this at length because it is the same failure the ranking has**
and I walked into it while writing about it: two readings agreeing is not
corroboration when they share a premise. Mine shared a regex. The panel's share
a persona.

**And it happened three times in one sitting, which is the number worth
carrying.** This one. The generalisation from one program shape in 4.2.1, found
by a second mechanism I built to attack myself. And the fabricated registry slug
in 5.1, found by a hand check no lint performs. **None of the three was caught
by a control I had written in advance**; each was caught by an arm added after
something looked odd. That is the honest report of how this file was produced,
and it is the reason section 7 exists.

## 6. What I could not settle, and what dissolved when I attacked it

### 6.1 Whether `selector` is an axis

`selector = linear` appears in three of Leroy's spans and the phrase across 3
keyed personas in 5 files, and `question::which_selector_does_the_design_ship_linear_or_augmented_chebyshev` asks *"Does the strategy object's
selector ship as a plain linear weighting, which reaches every hull vertex, or
as an augmented Chebyshev form, which reaches every non-dominated arm at the
same measured compile-time and portability cost?"*

Two readings and the corpus does not decide between them:

- **Per-strategy**: the selector is the form of a strategy's weighting, so it
  folds into `strategy` and is not a new axis. `proposal::a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it`'s *"The
  weighting is a support ... together with a rate"* points this way.
- **Design-wide**: the design ships one selector for everything, in which case it
  is a fork the canon has not closed, and under my 2 it is an axis **now** and
**What would decide it:** whether two strategies may use different selector
forms. I went looking for the row that would say, and the answer is narrower and
more useful than "the corpus does not decide". The only two rows describing what
a weighting *is* are `proposal::a_strategy_is_a_pair_of_an_observable_assignment_and_a_weighting`
and `proposal::a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it`,
which supersedes it. **Both sit at `standing = "one_expert"`, which is the
weakest standing the registry has**, so neither can decide a fork without
becoming exactly the drift the provenance ladder exists to stop.

If the second row is right, it points at design-wide: it says a weighting is
*"a support, the set of coordinates demanded, together with a rate, the exchange
among them"*, and a linear scalarisation and an augmented Chebyshev form are two
functions consuming the same support and rate rather than two rates. On that
reading the selector sits outside the strategy and is one design-wide choice.

**So the call turns on ratifying one `one_expert` proposal, and that is the
thing to do rather than to ask me.** I have not formed a view and would not want
a second reader agreeing with me on the strength of this section.

### 6.2 Whether `map_character` is an axis

3.5 establishes that a region is being written in prose and that `discharge
check` is the wrong phrase for it. Whether the character of the realisation map
is the right axis needs `112` and `114` read, which I did not do. One persona,
two files in the swept set, and the underlying result is `116`'s. **A seat
attacking this should start at `116:315`, which `119` cites for the mutual
exclusion of the two characters being a theorem.**

### 6.3 What a `construction` warrant does when the axis is inapplicable

I said in 4.1 that this was the residue and I want to report how far I got,
because two thirds of it dissolved and the third is smaller than `226` states.

**For `target_features` and `build_profile` there is no problem at all, and my
own probe is the counter-example.** The obligation is to name an instrument that
varied the axis and found no movement. Both of those are varied by *compiling
twice*, which is an instrument anybody can build in a minute:
`230_probes/p1_binding_time/matrix.sh` varies `build_profile` across a
compile-time claim and finds movement on one arm and none on another, which is
precisely the shape the obligation asks for. **So `226`'s F2 could have written
`build_profile: release` from what it already knew, or compiled both and written
`any: construction, <clause>` with the second compilation as evidence.** It
wrote nothing on the ground that the axis has no value to take, and the axis has
a value: the profile the compilation ran under.

**What is left is `threads` and `access_pattern`, and there the obligation is
genuinely undischargeable**, for a reason worth naming precisely rather than
calling it a hard case. You can observe a compile-decided fact from n threads,
and the observation cannot move with n, because nothing about a type equality is
reachable from a thread count. **So the differential exists and cannot fail,
and by this repository's own test-gate standard a control that cannot fail is
not a control.** That is the class: not an axis nobody controlled, an axis whose
values do not index any observation of the claim.

**And for that class the notation already admits a spelling, which I missed on
the first pass through 4.1.** The ratified ruling says *"An entry carrying no
token claims no warrant, which is what every existing entry keeps meaning."* So
`threads: threads any, <clause>` with **no token** is admissible today, says
exactly what a compile result means, and claims no warrant for it. Both corpus
instances I found are that shape, written before tokens existed.

**Where it still bites, and this is the whole of the residue.**
`ruling::a_predicate_lists_only_what_holds` is op's own, and it says what is not
known is not true and goes unstated. A tokenless `threads any` on a compile
result is *true* and *unmeasurable*, and his mechanism has three states with no
room for a fourth. So the honest statement of the gap is not `226`'s *"none of
them is compile-only"*; it is **that a universal true by inapplicability has no
warrant, and the notation offers a spelling for it that the discipline above the
notation is uneasy with.** That is one sentence rather than a missing axis, and
it is a question about two ratified rows meeting, which needs two experts and
not this one.

### 6.4 What I could not reach

The wide corpus in a form the extractor can read: 48 of the 60 predicate-bearing
files are in dialects the sweep does not parse, so every author-count in section
3 is a **file-level** count from a substring search, not a key-level one. It is
enough to establish that a phrase is not one persona's habit. It is not enough to
rank.

Outside the repository I looked for prior art on the question *"is this
coordinate a region or a run parameter"* and found the general answer rather
than a numerics-specific one: the interventionist criterion, which section 2
uses, and binding-time analysis, which section 4.2 uses to argue against a name.
I did not find anything on how a corpus of measured numeric findings should
notate its regions, and I do not think it exists; the closest neighbours are
experimental-design vocabularies that separate factors from nuisance parameters,
which is the same distinction with the freedom question left implicit. **Said
plainly so nobody re-runs it: I searched, and the absence is a real absence
rather than an unsearched one.**

## 7. What a next seat should attack in what I wrote

- **3.9 is my strongest claim and rests entirely on an analogy** between the
  declared signature set and `operation`. Break the analogy and the finding goes.
- **3.4's `leaf_aliasing` was open on whether the term language admits aliasing
  and is no longer.** `111:1391` names `x - x` and `(x + y) - x` as terms in it.
  What is left to attack is whether `chain_length` and `arity` pin the axis
  between them, which I say they do not on one example and have not swept.
- **3.6's `coupling` assumes arvo's model reaches a workload.** If arvo only ever
  sees one operation at a time, a dependence structure has nowhere to live and
  the finding moves downstream rather than dying.
- **4.2 has been attacked once, by me, and one sentence in it did not survive.**
  A second mechanism is in `second_mechanism.sh` and it broke the independence
  claim. A third is still owed and floating point is where I would look, because
  const evaluation and runtime float behaviour would be genuinely independent of
  the arithmetic-overflow checker that arms A through E all route through.
- **4.2.2 says `overflow-checks` is the flag that acts and `debug-assertions` is
  a proxy for it.** That is a 2x2 on one operation at one width. Whether it holds
  across the operation set is unswept and I would expect it to, which is not a
  predicate.
- **My criterion in section 2 is mine and unratified.** It adds "free to sit at
  more than one value" to the header's test. If it is wrong, 3.2 is wrong with
  it, because 3.2 is the only verdict that turns on the addition.
- **Everything in section 5 is outside the question I was asked** and 5.2 is the
  largest thing in this file. If a second reader confirms that no predicate names
  a strategy, that is a bigger finding than anything in section 3 and it should
  not be filed under an axis review.

---

**Region.** The findings in sections 1, 3 and 5 are about this repository as of the commit that lands this file on
`research/declare-the-blocking-axes-2`, with
`dimension.toml` at 22 rows and the registry at 527 predicate entries. **The
declared vocabulary has no axis over the canon's own contents**, which
`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` already
records and `226` cites as the other side of its own gap, so these regions are
stated in prose rather than as predicate entries and that is the second instance
of that gap in this file.

The finding in 4.2 has a region and I write it in full, including the entry the
current set cannot accept, because a predicate that demonstrates its own gap is
worth more than one that hides it:

```
toolchain:        rustc = nightly-2026-05-28, edition 2021
build_profile:    debug-assertions in {on, off}: exhaustive, the whole axis
overflow_checks:  {on, off}: exhaustive, the whole axis
program_shape:    {literal in a const item, call to a const fn}: exhaustive, the two
                  shapes a const-evaluated operation can be written in here
operation:        operations {add, sub, shl}
total_width:      W = 8
signedness:       signedness = unsigned
container:        container = u8
target_features:  host aarch64-apple-darwin
threads:          threads = 1
evaluation_site:  {const evaluation, execution}: exhaustive, the whole axis
```

**Three of those entries name axes nothing declares**: `evaluation_site`,
`overflow_checks`, and `program_shape`. `overflow_checks` is what
`dimension::build_profile` should have named and did not, per 4.2.2.
`program_shape` is the coordinate 4.2.1 discovered by breaking my own claim, and
I am **not** proposing it as a row: it is a property of the source a finding is
about rather than of the design, and it is here because leaving it out would
widen the finding in exactly the way this notation exists to prevent. Writing
them separately rather than packing two into one value is the discipline
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` names as a live
violation in six existing entries, and I nearly committed a seventh in this very
block.

`every-predicate-names-a-declared-axis` would refuse this predicate at
`HARD_ERROR`, three times, and it is correct to. That refusal is the finding.

**Instruments.** `230_probes/spans_for.sh` (span reader),
`230_probes/wide_census.sh` (the twelve-file sweep against all 60,
`W1` byte-identical to `183_probes/keys.txt`),
`230_probes/who_writes_it.sh` (author counts, loose and keyed arms),
`230_probes/entries.sh` and `230_probes/axis_uptake.sh` (527 entries, per-axis
uptake, per-field silence, checked against the ratified note's own 527),
`230_probes/stale_absence.sh` (present-tense undeclared-axis prose),
`230_probes/p1_binding_time/matrix.sh` (the evaluation-site matrix) and
`230_probes/p1_binding_time/second_mechanism.sh` (arms A to E: one `const fn` at
two sites, the profile visible at const time, `overflow-checks` against
`debug-assertions` on two program shapes, and the construction that keeps the
refusal at every flag setting). Every source, every captured stderr and every
panic message is committed beside its script; the compiled binaries are not, and
a `.gitignore` beside them says which and why.
