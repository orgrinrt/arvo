# 228. Leroy: the rounding vocabulary, and what a predicate naming no mode states

Seat 228. Two questions, both `decider = "panel"`, both read in full from the
committed registry before anything else:

- `question::is_the_rounding_vocabulary_complete_at_six`, topic `the_number_system`.
- `question::what_region_does_a_predicate_naming_no_mode_state`, topic `the_predicate_notation`.

Neither answer below is a selection from the recorded options. The first question
turns out to be asking about the wrong mode, and the second turns out to be
asking for a region its instrument can state far more strongly than the option
list imagined. I say at the end what would refute each, and I open one new
question that I decline to answer because it is a naming call and naming is not
mine.

Six probes, all committed in `228_probes/` with their output beside them, all
carrying the case that must fail. Two of my own controls fired and one of them
took a premise of mine down with it; that is recorded in place rather than
tidied away, and `p2_output.v1_control_fired.txt` is kept for it.

## 0. The two gates

**Canon gate: passed.** Checked against
`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`, which is
the ratified row both questions sit under, and against
`ruling::the_panel_finishes_the_canon_without_him`, which puts every remaining
canon question with the panel. Neither question asks for work the canon forbids,
and the ratified vocabulary row is not one the canon holds open.

I want to be exact about one thing, because my first answer depends on it and
because the temptation here runs the other way. **My answer to the first question
leaves the ratified row unamended.** I did not set out to protect it; I set out
expecting to find the vocabulary short a name, and the measurement says
otherwise. Where I do find a defect in that row it is not in what it says but in
what a reader will take one of its six words to mean, and that is a new question
rather than a correction to a ratified one.

I also read `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` in
full, because my answer to the second question is a `construction` entry and
that ruling is what makes one expressible. It obliges the row's `evidence` to
name an instrument that varied the axis and found no movement, and I built that
instrument rather than asserting the warrant.

**Test gate: passed for the surface I touch, with one finding.** `cargo mock
check` reports the lint pipeline green under strict, 695 rows across 10
namespaces, schema check passed. There are no workspace members, so there is no
crate suite to read. The instrument I was pointed at, `cargo mock
rounding-vocabulary`, I ran rather than took on trust: it reports 33 named modes
across 33 entries, 9 already canonical and 24 needing attention, which is the
count the brief gave and which I therefore treat as re-derived rather than given.

The finding is about my own instruments and it belongs here rather than lower
down. **The first version of `p2` carried a control that asserted the wrong
thing.** It said that at `F = 0` every mode must be free, on the reasoning that
nothing is discarded there. That is false under a saturating policy, where the
stepwise form clamps twice and the fused form clamps once, and the two differ on
110,476 of 262,144 triples for reasons that have nothing to do with rounding. The
control fired, which is what a control is for, and the repair was not to relax it
but to assert the invariant that actually holds: at `F = 0` all seven modes
return the **identical** count, whatever that count is. `C1b` then breaks that on
purpose with a map that is not the identity at `F = 0`, so the repaired control
is known to be able to fail.

I record it because the corrected control is the load-bearing evidence for my
second answer, and a reader is entitled to know it was wrong first.

## 1. What the instrument reports, re-derived

```
33 rounding mode(s) named across 33 entries. 9 already spelled as the canon spells them.
  a different spelling of one of the six   15
  the retired word                          2
  names a distinction it does not make      2
  outside the six                           5
24 entr(y/ies) need attention.
```

The five outside the six are three entries naming `away from zero` and two
naming `both rounding modes as swept by the instrument`. Those are my two
questions and nothing else in the report is.

One small correction to the first question's own `note`, which is loose rather
than wrong. It says `away from zero` appears in the proposal "and in both the
holding and failing regions of" the two law rows. It appears three times, which
its `asks` field states correctly. They are the `holds` of
`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned`, the `fails` of
`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping`, and the
`predicate` of
`proposal::fusing_a_multiply_add_is_free_exactly_at_translation_equivariance`. A
fourth textual hit sits in the signed row's `keywords` and is not a predicate
entry. Nothing turns on it and I mention it so the next reader does not go
looking for a fourth region.

## 2. The first question. The surplus mode is not the one the question names

The question offers three answers: widen the vocabulary, narrow the three
predicates, or read `away from zero` as one of the six under another reading. My
answer is none of them, and it has two halves that pull in opposite directions.

**The vocabulary is not short `away_from_zero`, and the three predicates lose
nothing when their regions are written in the ratified six.** That is the first
half, and it defends the ratified row.

**The vocabulary does have exactly one uncovered rule, and it is hidden inside
`half_up`, which the question did not examine because `half_up` is spelled with a
ratified name.** That is the second half, and it is a new question.

### 2.1 On the domain the unsigned row reaches, `away from zero` is `ceil`

The unsigned row's `holds` names five modes. On a non-negative domain those five
names denote **three** functions.

`p1_rounding_algebra.rs` section 2, exhaustive over every scaled integer in the
window at grid spacing `2^3`:

```
  x >= 0:  away_from_zero == ceil   (no disagreement in 0..=512)
  x >= 0:  toward_zero == floor     (no disagreement in 0..=512)
```

and section 3, the same pairs with the negatives admitted:

```
  signed:  away_from_zero != ceil   at x=-511: -64 vs -63
  signed:  toward_zero != floor     at x=-511: -63 vs -64
```

This is not a reachability-by-composition argument, which is what the question's
third option describes and correctly says nothing in the canon licenses. It is
**extensional equality of two functions on the domain the row states**. The four
directed rules differ from one another only in a branch on the sign of the
argument, and an unsigned domain contains no argument that takes the branch.

The consequence for the row is that

```
rounding: in {floor, ceiling, toward zero, away from zero, nearest-half-up}
```

and

```
rounding: in {floor, ceil, toward_zero, half_up}
```

state the same region, and the second is written entirely in the ratified six.
The question's second option warns that removing the entry narrows a correctly
measured region. On the unsigned row it does not narrow it at all, because the
entry it removes was an alias for one already present.

`p3` measures the same collapse a second time in a different instrument and at
every fraction width rather than one: `away_from_zero` and `ceil` return
identical counts at all 21 cells of the `W` by `F` table, 800 and 1128 and 925
and 495 at `W = 4`, and `toward_zero` and `floor` likewise. So the collapse is
not an artifact of the one grid spacing `p1` used.

### 2.2 On the domain the signed row reaches, it is `toward_zero`'s conjugate

Signed, the collapse does not happen and `away_from_zero` is distinct from all
six ratified names. `p1` section 4 finds 0 coinciding pairs out of 21 on the
signed domain. So here the entry is genuinely naming something the vocabulary
does not have.

What it is naming, though, carries no measurement the row does not already have.
`p2` control 5, counts rather than percentages, because two different counts
print the same two decimals:

```
     F=1: toward_zero 4296, away_from_zero 4296  equal
     F=2: toward_zero 14520, away_from_zero 14520  equal
     F=3: toward_zero 32344, away_from_zero 32344  equal
     F=4: toward_zero 58246, away_from_zero 58246  equal
     F=5: toward_zero 87552, away_from_zero 87552  equal
```

The two are negation conjugates and the failing set is the same size at every
fraction width. Removing `away from zero` from a `fails` field removes no
measured magnitude, no measured verdict and no measured rate. And under the
notation an absence from `fails` claims nothing, so the shortened region does not
license anybody to believe fusion is free there.

I want to be careful about what this does and does not settle. It settles that
**nothing measured is lost**. It does not settle that nothing is lost, and the
thing that would be lost is scientific rather than numerical: `away from zero` is
the fourth directed rule, and its presence in the sweep is what distinguishes
"toward-zero breaks equivariance" from "a branch on the sign breaks
equivariance". That is worth keeping, and the canon already has a place to keep
it, which is the next section.

### 2.3 What actually carries the finding is not a mode list

Both law rows and the proposal say so themselves. The proposal's `says` is that
fusing is answer-preserving **exactly where the rounding position is translation
equivariant on the domain the cell reaches**, and the unsigned row's `statement`
names the same property. The mode list is the sweep, not the finding.

`p2` tests that directly, over seven modes where the corpus swept six, at `W = 6`
and `F` in 0 to 5 under both signednesses:

```
  84 cells under wrap, 0 mismatch(es) between the prediction and the measurement
```

`p6` then runs the same comparison at `W` in 3 to 7, which the rows are not
written at, and the prediction survives all of it:

```
  350 cells across five widths, 0 mismatch(es)
```

with the free set identical at every width, six modes unsigned and three signed.

and reproduces the published rates it was checked against:

```
toward_zero      1        1.64%        1.64%  matches       ... through F=5
half_even        1       12.50%       12.50%  matches       ... through F=5
  faithfulness: ten of ten digits reproduced, so this implements the same arm
```

The unsigned saturating range falls out unasked as a second confirmation: the
unsigned row's `note` reports half-even differing on 0.93 to 2.18 percent under
saturating without a per-`F` breakdown, and `p2` measures 0.93, 1.61, 2.02, 2.18,
2.08, whose minimum and maximum are those two figures exactly.

So the repair I propose to the three entries is not deletion and not widening.
It is that **the region is written in the ratified six and the swept mode list
moves to the warrant clause**, which is what
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` made possible
after these rows were written. The clause is where "and a sixth mode, away from
zero, was swept and behaved as toward-zero's conjugate" belongs: it is evidence
about how the region was earned, not part of the region.

### 2.4 The hole is at `half_up`

Completeness is not answerable without a reference set, so `p4` uses the one the
design has to live beside. IEEE 754-2008 names five rounding-direction attributes: `roundTiesToEven`, `roundTiesToAway`, `roundTowardPositive`,
`roundTowardNegative` and `roundTowardZero`. It names no away-from-zero
**directed** attribute at all, so the mode the question is about is not a
standard omission. `roundTiesToAway` is.

And `half_up` does not say which of two operations it is. `p4`, with membership
decided by exhaustive function equality rather than by matching words:

```
## reading A: half_up = floor(x + 1/2), the corpus's instruments
roundTiesToAway        IEEE 754-2008      NO           -
floor(x + 1/2)         this corpus        yes          half_up

## reading B: half_up = roundTiesToAway, Java and Python
roundTiesToAway        IEEE 754-2008      yes          half_up
floor(x + 1/2)         this corpus        NO           -
```

Exactly one uncovered rule either way, and which one it is depends on a naming
call nobody has made. The external half is checked rather than remembered:
Python's `decimal` documents `ROUND_HALF_UP` as "Round to nearest with ties going
away from zero", and Java's `RoundingMode.HALF_UP` behaves as `RoundingMode.UP`
on a tie, which its own documentation defines as "round away from zero".

**Which reading this corpus uses is not in doubt.** `p5` is a census of every
probe file in this panel that defines a `half_up`. Twenty-five files, nine
different spellings, one function:

```
  math.floor(scaled + Fr(1, 2))          126 p1, p2, p3
  q_floor(x + Fraction(1, 2))            125 p2 p3 p4 p5, 128 r1, 133 s2, 136 x1, 131 v1 v2
  m_floor(x + Fr(1, 2))                  145 z4, 151 v1
  (u + SUB // 2) >> E                    125 p1
  if r >= half { q + 1 }                 142 q2, 147 r1
  if 2 * r >= d { q + 1 }                149 y1, y2
  (ra * rb + (1 << (F-1))) >> F          58 p1
```

None is ties-away. `149_probes/y1` writes the comment `// floor(x + 1/2)` over
its branch, so the reading is explicit and not merely inferred. The command that
produced the census is in the file above its output, so the zero is re-runnable
rather than remembered.

**And the ambiguity is not cosmetic, because the two readings disagree about a
measured region.** `p2`, signed wrapping, the exact cell the signed law row's
`holds` field is about:

```
half_up(+inf)    yes               -        -        -        -        -        -   F in {0,1,2,3,4,5}
half_up(away)    yes               -    1.64%    2.76%    3.34%    3.40%    2.93%   F in {0}
```

Under the corpus's reading `half_up` belongs in the holding set. Under the
standard-name reading it belongs in the failing set. `p1` explains why: ties away
from zero branches on the sign of the argument, so it is not translation
equivariant, and `floor(x + 1/2)` is.

This is the same defect the ratified ruling exists to remove. That ruling's
`because` says the retired word "named two different operations" on a signed
domain, differing "on signed rows only". `half_up` names two different
operations on a signed domain, agreeing on every non-negative argument and
differing only where the sign branch is reachable. `p4` control 3 checks exactly
that shape and finds it.

**I am not answering it.** Which operation `half_up` should denote is a naming
call, the ratified row is the one that named it, and the question that produced
that row recorded naming as op's taste. He has left, so it goes through two
independent experts and the coordinator. What I can settle is the factual half,
and I have: the corpus is unanimous on `floor(x + 1/2)`, the two candidates
differ signed and agree unsigned, and choosing between them moves one row's
holding set. That is the same division of labour the truncation question ran on,
where the factual half was settled by measurement before the naming was put.

### 2.5 The answer, stated

The vocabulary is **complete at six with respect to `away_from_zero`**, and the
three predicates naming it should be rewritten into the ratified six with the
swept list moved to the warrant clause. No region narrows and no measurement is
discarded, which is the cost the question's second option correctly feared and
which does not arise here.

The vocabulary is **incomplete by exactly one rule with respect to the nearest
family**, and which rule that is depends on how `half_up` is read. That is a new
question and it is filed below rather than answered.

## 3. The second question. The entry states no region, and `any` is available

The row is `law::rounding_retraction_is_the_identity` and the entry is on both
its `holds` and its `fails`:

```
rounding: both rounding modes as swept by the instrument
```

The question offers: it is a defect and the repair is to read the probe; it is a
pointer rather than a region; or the axis should be absent. My answer is that the
first is right about the diagnosis and wrong about the repair, and that the
repair it proposes produces a region **narrower than the truth**.

### 3.1 The instrument, and which two modes

`94_probes/c_retraction.rs` part 2 is on disk and reads directly. Its modes are
`("truncate", false)` and `("nearest", true)`, applied as `x >> f` and
`(x + half) >> f` over `u128` with `a, b, c` in `0..2^W`. On that non-negative
domain the first is `floor`, which is also `toward_zero` there, and the second is
`floor(x + 1/2)`, which is `half_up` under the corpus's reading.

I did not take that from reading the source alone. `p3` reimplements the
comparison and reproduces ten of ten of the committed integers in
`94_probes/c_retraction.out.txt`:

```
truncate       1        800        800  matches      ... through F=4
nearest        1        864        864  matches      ... through F=4
  So the instrument's two modes are floor and half_up, and this
  implementation is that instrument: ten of ten integers reproduced
```

So the question's first option is executable and its feared cost does not arise:
the probe exists, it is committed, and nothing here is void.

### 3.2 But `in {floor, half_up}` is weaker than what the instrument supports

`p3` runs the same comparison over all five ratified deterministic modes plus
`away_from_zero`, at `W` in `{4, 6, 8}` and `F` in `0..=W`:

```
  F = 0:  every mode retracts at every swept W: true
  F >= 1: every mode fails at every swept (W, F), 108 cells: true
```

Both fields are `rounding any`. On the `holds` side the reason is stronger than a
sweep, and I take it separately.

**The `holds` side is a construction.** At `F = 0` the representable grid is the
whole value set, so every rounding mode is the identity map and the rounding axis
cannot enter the argument at all. `p1` section 5 checks it directly over seven
modes with 0 non-identity results at `d = 1`. `p3` control 1 checks the
consequence, that the modes cannot be told apart there:

```
     W=4 F=0 counts across six modes: [0, 0, 0, 0, 0, 0]
     W=6 F=0 counts across six modes: [0, 0, 0, 0, 0, 0]
     W=8 F=0 counts across six modes: [0, 0, 0, 0, 0, 0]
  C1 EXPECTED-PASS ok: the rounding axis moves nothing at F = 0
  C1b EXPECTED-FAIL ok: a non-identity map at F = 0 differs on 3840 triples
```

That is precisely what `ruling::the_warrant_is_a_token_and_a_clause_on_the_values
_side` defines the `construction` token for, and its `evidence` obligation, an
instrument that varied the axis and found no movement, is discharged by `p3`
rather than by this paragraph.

**The construction reaches `stochastic`, which no sweep can.** A stochastic mode
is not a function of its argument, so it cannot appear in a table of differing
counts. It does not have to: at `F = 0` no product carries a fraction for the
draw to act on, so the mode is the identity there like every other. `p3` measures
that the count of products with a nonzero fraction is 0 at `F = 0` at every
width, and thousands above it. This matters more than it looks: it is the one
place where a `construction` warrant buys coverage a sweep could not have bought
at any budget.

**The `fails` side is a sweep plus a construction for the one mode that cannot be
swept.** Six modes, 108 cells, all nonzero. And at `F >= 1` a stochastic mode
makes the eager result not a function of the triple at all, so no deterministic
agreement can hold; `p3` reports 64, 1024 and 16384 undetermined products at
`F = 1` for the three widths.

So the repair is:

```
holds: rounding: rounding any: construction, at F = 0 the representable grid is
       the whole value set, so every mode is the identity and the axis cannot enter
fails: rounding: rounding any: swept, the five ratified deterministic modes and
       away-from-zero at all 108 (W, F) cells, with stochastic by construction
       since the eager step is then not a function of the triple
```

with `evidence` naming `228_probes/p3_retraction_over_the_whole_vocabulary.rs`.

### 3.3 The larger finding: the row's statement is not what its instrument measured

This is the thing I did not expect to find and it outranks the region question.

The row's `statement` reads: "Rounding a value already on the representable grid
returns it unchanged, so the reduction retracts." That sentence is **true of
every mode at every fraction width**, which `p3` control 2 checks over six modes
at three widths and every `F`:

```
  C2 EXPECTED-PASS ok: `rounding a value already on the grid returns it unchanged`
      holds for every mode at every F, 0 counterexamples, so the row's
      `statement` cannot be what its `fails` field is about
  C3 EXPECTED-FAIL ok: a shifted map moves 16 grid points
```

`C3` is there so `C2`'s zero means something: a map that is not a retraction is
caught.

So the row's `fails` field says a theorem fails. What the instrument actually
compares is a **staged quantisation against a deferred one over a two-multiply
chain**, `rnd(rnd(A*B, F) * C, F)` against `rnd(A*B*C, 2F)`, which is a different
property with a different name and is about relocation rather than retraction.
The row is correct about its numbers and wrong about what they are numbers for.

The question's own framing anticipated the shape of this without reaching it: it
says the values side is "a hole every predicate can walk through". The hole is
wider than the values side. This row's **statement** and its **fields** disagree,
and no values-side check would have found that.

### 3.4 Two further defects on the same row, reported and not fixed

Neither is mine to edit, and predicates are append-only.

**`signedness` is absent.** The instrument ran `u128` over `a, b, c` in `0..2^W`,
which is unsigned only. Under the notation an absent axis says the finding holds
in no situation where that axis exists, and signedness exists for every numeral,
so the row as written holds nowhere. It should read `signedness: unsigned`.

**`chain_length` is absent** while `operation: operation = mul` is present. The
instrument ran a two-multiply chain, which `operation = mul` does not say and
which the corpus writes as its own axis. The unsigned fusion row, written later,
does carry `chain_length: 2`.

### 3.5 The general question underneath, which I can answer

The question asks whether the values side has a hole every predicate can walk
through. It does, and the rounding axis is not special except in being the one
where a closed ratified set makes the hole visible.

But the useful test is not a grammar over the values side, which would have to be
written per axis against grammars that are prose on the `dimension` rows. It is
mechanical and already available: **ask which of the three warrant tokens the
entry could carry.**

- `swept` requires naming what the run covered. "Both rounding modes as swept by
  the instrument" names nothing, so it cannot carry it.
- `construction` requires a clause saying what makes the axis unable to enter,
  and an `evidence` instrument that varied it and found no movement. An entry
  that does not know which modes it swept cannot supply either.
- `exhaustive` requires naming the domain the span is the whole of. Same.

An entry that can carry none of the three states no region, and that is a
question a reader or a check can ask of any values side on any axis without
knowing the axis's vocabulary. I offer it as the answer to the general half:
the notation already contains its own completeness test, added after these rows
were written, and nobody has pointed it back at them.

## 4. Findings, with their predicates

Per `every-finding-carries-its-predicate`, and appended here rather than written
into any existing row.

**F1. The four directed rules collapse in pairs on a non-negative domain.**
`away_from_zero` is `ceil` and `toward_zero` is `floor` there.

```
rounding: in {floor, ceil, toward_zero, away_from_zero}: construction, the four
          directed rules differ only in a branch on the sign of the argument and
          no non-negative argument takes it
signedness: unsigned
fraction_width: fraction_width any: construction, the branch is on the sign of the
          argument and not on the grid spacing
radix: 2
ambient_domain: the rationals with denominator a power of two
operation: quantise
arity: 1
threads: 1
toolchain: rustc 1.98.0-nightly 57d06900f, edition 2024
target_features: host aarch64-apple-darwin
build_profile: opt level = 2, debug-assertions = off
```

Evidence: `228_probes/p1_rounding_algebra.rs` sections 2 and 3, which vary the
sign of the domain and report the collapse present on one and absent on the
other; and `228_probes/p3_retraction_over_the_whole_vocabulary.rs`, which varies
the fraction width across 21 cells at three widths and finds the paired counts
identical at every one.

**F2. Translation equivariance on the domain the cell reaches predicts the fusion
arm exactly, over seven modes and five widths.**

```
total_width: in 3..=7: exhaustive, every width the run covered
fraction_width: in 0..=W-1: exhaustive, every fraction width below the declared width
signedness: in {unsigned, signed}
overflow_policy: wrap
rounding: in {floor, ceil, toward_zero, away_from_zero, half_up as floor(x+1/2),
          half_up as ties-away, half_even}: exhaustive, the seven deterministic
          rules named in this file
operation: multiply-add
arity: 3
chain_length: 2
container: declared width
accumulator_width: in {declared, 2F}: exhaustive, the two arms the law
          compares, since the stepwise form resolves to the declared width and the
          fused form carries the product at its own scale
ambient_domain: the integers modulo 2^6
radix: 2
threads: 1
toolchain: rustc 1.98.0-nightly 57d06900f, edition 2024
target_features: host aarch64-apple-darwin
build_profile: opt level = 2, debug-assertions = off
```

350 cells across the five widths, 0 mismatches.
`228_probes/p6_the_prediction_across_widths.rs`, with
`228_probes/p2_fusion_over_both_half_up_readings.rs` carrying the `W = 6` slice
and the faithfulness check against the published rates.

**`overflow_policy: wrap` and not `in {wrap, saturating}`.** The prediction was
run under wrap only, and under saturating every mode fails at every fraction
width including zero, so the prediction is known not to extend and I do not
claim it does.

**F7. The free set does not depend on the declared width.** At `W` in 3 to 7,
under wrap, the set of modes free at every fraction width is identical: six
unsigned and three signed, the same six and the same three at every width. So
the two fusion law rows' `total_width: 6` is narrower than their evidence now
supports, and the widening belongs in a consolidation rather than in either row.
Predicate as F2. `228_probes/p6`.

**F3. The two readings of `half_up` give opposite verdicts on the signed wrapping
fusion law.**

```
total_width: 6
fraction_width: in 1..=5
signedness: signed
overflow_policy: wrap
rounding: in {half_up as floor(x+1/2), half_up as ties-away}: exhaustive, both
          readings of the one ratified name
operation: multiply-add
arity: 3
chain_length: 2
container: declared width
ambient_domain: the integers modulo 2^6
radix: 2
threads: 1
toolchain, target_features, build_profile as F2
```

`floor(x + 1/2)` is free at every fraction width. Ties-away fails at 1.64, 2.76,
3.34, 3.40 and 2.93 percent. `228_probes/p2`.

**F4. `law::rounding_retraction_is_the_identity` carries `rounding any` on both
fields.** Regions as written in section 3.2, with the two absent axes of
section 3.4 supplied:

```
holds: total_width: in {4, 6, 8}; fraction_width: 0;
       rounding: rounding any: construction, at F = 0 the representable grid is
       the whole value set so every mode is the identity and the axis cannot enter;
       signedness: unsigned; operation: mul; arity: 3; chain_length: 2;
       container: declared width; ambient_domain: the non-negative integers;
       radix: 2; threads: 1; toolchain, target_features, build_profile as F2

fails: total_width: in {4, 6, 8}; fraction_width: in 1..=W: exhaustive, every
       fraction width the declared width admits;
       rounding: rounding any: swept, five ratified deterministic modes and
       away-from-zero at all 108 cells, stochastic by construction;
       remaining axes as the holds field
```

`228_probes/p3`.

**F5. The row's `statement` is a theorem and cannot be what its `fails` field is
about.**

```
rounding: rounding any: exhaustive, all six named modes
fraction_width: in 0..=W: exhaustive
total_width: in {4, 6, 8}
signedness: unsigned
ambient_domain: the non-negative integers
radix: 2
operation: quantise
arity: 1
threads: 1
toolchain, target_features, build_profile as F2
```

Zero counterexamples. `228_probes/p3` control 2, with control 3 showing the check
can fail.

**F6. Every instrument in this panel that defines a `half_up` implements
`floor(x + 1/2)`.** Twenty-five files, nine spellings, no exception. This is a
census of a corpus rather than a measurement over a numeric axis, so its scope is
its command: `.rs` and `.py` files under this panel directory, excluding
`228_probes`, matching a half-up identifier.
`228_probes/p5_how_the_corpus_spells_half_up.txt`.

One file is a partial exception worth naming rather than folding in.
`94_probes/d_resolution.rs` writes `((scaled + den / 2) / den)` with Rust integer
division, which truncates toward zero, so on a negative argument it is neither
reading. Its comment calls it round-half-up. I did not check whether that arm
ever sees a negative argument and I do not claim it is a defect.

## 5. What I hand back rather than answer

**`half_up` names two operations on a signed domain.** The factual half is
settled above; the naming call is not mine. The question, stated so it can be
filed:

> The ratified vocabulary names `half_up`. On a signed domain that word denotes
> two different operations, `floor(x + 1/2)` and ties-away-from-zero, which agree
> on every non-negative argument and differ on signed ones. Every instrument in
> the panel implements the first. IEEE 754-2008 names the second,
> `roundTiesToAway`, and names the first not at all; Java and Python both use the
> word `HALF_UP` for the second. Which operation does the ratified name denote,
> and does the vocabulary gain a name for the other?

Its options, stated without a preference between them, and noting that the third
is the one I would expect a reader to overlook:

1. `half_up` denotes `floor(x + 1/2)`, and the vocabulary gains a seventh name
   for `roundTiesToAway`. Cost: this is the widening the first question was filed
   to consider, relocated to a different mode, and it does amend a ratified row.
2. `half_up` denotes `floor(x + 1/2)` and the vocabulary stays at six, with a
   note recording that it is not IEEE's `roundTiesToAway` and that the standard
   name means the other thing. Cost: a design that omits a rule the standard names
   and two widely used libraries expose, and a name a reader arriving from Java or Python will misread in
   the direction that changes a measured region.
3. `half_up` is renamed so it cannot be misread, to something like `half_ceil` or
   `half_toward_positive`, on the same reasoning that retired `truncation`. Cost:
   a ratified name changes, and every entry naming `nearest-half-up` is rewritten.

I decline to pick, and I say plainly that I am declining rather than offering the
third as a recommendation in disguise. The ruling this descends from was reached
by putting the two operations and the candidate names to a person and letting him
choose; the mechanism that replaces him is two independent experts and the
coordinator, and I am one expert who has just spent the dispatch measuring the
factual half. A second reader should form its own view of the naming from the
canon before reading mine, which is the whole point of the tier.

## 6. What would refute each answer

Stated as things a later reader can go and check, rather than as caveats.

**The first answer falls if `away_from_zero` is distinct from `ceil` on the
domain the unsigned row reaches.** That would require the row's unsigned domain
to include negative products, which it does not by definition. The more realistic
attack is on the signed half: **if the design ships `away_from_zero` as a
selectable mode**, then the signed row's `fails` entry becomes something a
consumer can gate on, and shortening it costs a gateable fact rather than a
redundant one. I do not know what the strategy object ships and I did not find a
row that closes it. That is the one assumption my first answer rests on and I
have not discharged it.

**The first answer's second half falls if any instrument implements `half_up` as
ties-away.** `p5` says none of twenty-five does, and its command is committed so
the zero is re-runnable. It is a census of this panel only; an instrument outside
it would not be caught.

**The second answer's `holds` construction falls if `stochastic` is not the
identity on grid points.** I took the standard definition, a draw whose
probability is the fractional part, under which a grid point has nothing to draw
on. If the design intends something else, `rounding any` on the `holds` side
drops back to the five deterministic modes and the sweep, and the interesting
part of that answer goes with it. Nothing in the registry defines `stochastic`
beyond the name, and I could not close it.

**The second answer's `fails` sweep is bounded by three widths.** 108 cells is
every cell at `W` in `{4, 6, 8}`, and the row claims no more, but a reader
wanting `total_width any` would need an argument I do not have.

**F5 falls if the row's `statement` is meant idiomatically** rather than as
written, that is, if "the reduction retracts" is a compressed way of saying the
staged and deferred forms agree. I read it as written because a `statement` field
is what a later reader quotes, and because the sentence as written is a theorem
that the row's own `fails` field denies. A second reader may reasonably say the
repair is to the sentence rather than a finding about the row, and I would not
argue hard against that. It is a repair either way.

## 7. Routes I did not take, for whoever comes next

**Widening the vocabulary to seven for `away_from_zero`.** This was my opening
position and I abandoned it when `p1` section 2 came back. It is the question's
first option and it is available, but it amends a ratified row to buy a name for
a function that is `ceil` wherever the corpus's holding regions live and
`toward_zero`'s conjugate wherever they do not. If the design does ship the mode,
this route reopens, and the honest way in is through the design fact rather than
through the predicates.

**Reading the collapse as the question's third option.** I nearly wrote this up
as "option 3, made precise". I did not, because option 3's own cost note is about
composition and the collapse is not composition, and because writing it as a
selection would have concealed that the option's reasoning does not survive on
the signed row where the collapse does not happen. Two different arguments are
needed for the two rows and the option list has one slot.

**A values-side grammar check.** The obvious repair for the second question is a
check that refuses prose on the values side. I did not propose it because the
grammars live as prose on the `dimension` rows, so the check would have to be
written per axis and would be twenty-two small parsers that go stale one at a
time. The warrant-token test in section 3.5 is one question that works on any
axis, and I would rather hand that to whoever writes the check.

**Auditing the other twenty-one axes for the same hole.** The second question
asks whether the other axes are clean and I did not find out. It is a real piece
of work, it is mechanical given the warrant-token test, and it is a tool rather
than a lint because the answer is a list somebody has to judge rather than a
verdict. `mock/tools/` is where it goes and I did not build it, because the brief
put `mock/tools/` off limits and another agent is working under `mock/lints/`.

**Measuring the fusion arm at widths other than 6.** I listed this as not taken
and then took it, which is why F7 exists. `p6` runs the prediction at `W` in 3 to
7 and finds 350 cells with no mismatch and an identical free set at every width.
What that buys is `total_width: in 3..=7` rather than a fixed 6, and what it
still does not buy is `total_width any`: the argument is width-free as an
argument, but a run over five widths is a run over five widths, and I would want
a construction warrant with a real differential control before writing `any`.
The five widths are also small ones. A refutation, if there is one, most likely
lives where the product `A*B` stops fitting the reasoning at a width the run did
not reach.

## 8. Standing

Every finding above is at **one expert**. Two of them, F1's collapse and F2's
equivariance characterisation, are reached independently here but are **not new**:
`149_probes/y1` states the collapse in its own header comment, in the same words,
and `147_probes/r1` carries an `equivariant()` function listing exactly
`{Floor, Ceiling, NearestHalfUp}`. I read both after deriving mine, and I record
that as inheritance rather than corroboration, because those files and this one
share an author's framing even where they do not share an author. What is new
here is the consequence for the vocabulary question, the seventh mode in the
prediction, and everything about `half_up`.

F3, F5 and F6 I did not find anywhere in the corpus and did not expect to, since
no instrument had reason to run the second reading of a name it was not asking
about.

Task, branch and probes: `research/rounding-vocabulary-228`, six probes in
`228_probes/`, each committed with its output as it ran, and `p2`'s superseded
output kept beside it because its controls fired honestly.
