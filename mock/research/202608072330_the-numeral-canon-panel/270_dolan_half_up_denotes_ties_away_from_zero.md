# Seat 270. `half_up` denotes ties away from zero

Blind read. Excluded from this read: everything under this directory whose
name starts `228_`, `229_`, `267_`, `268_`, `269_`, and their probe
directories, and the scratchpad. `git log --all` surfaced two commit subjects
from seat 269's branch in the course of a provenance search (`research: audit
two reads of half_up and answer it as ties to +inf`); neither was opened, and
neither is weighed below as evidence.

## Canon gate

`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`
is `rung = "ratified"`, `ratified_by = "op"`. It closes the vocabulary at six names, `half_up` among them, and its
own `note` says the naming was the only thing open, "no position is
overturned by this." It does not say what `half_up` denotes. Op's
ratification transcript
(`.data/op-responses/202608311735_arvo-canon-four-questions.md:88-104`)
answers only the retire-the-word question; the transcript's four questions do
not reach tie direction anywhere. `question::which_operation_half_up_denotes`
is therefore live: the ruling that
created the name did not settle what it means, and nothing else in op's
corpus does either. Not misposed, not already answered. Proceeding.

## Suite audit

Ran `cargo test -p arvo-format --lib symmetry`: 25 passed, 0 failed, this
worktree, this commit. Read the body of every test in
`mock/crates/arvo-format/src/symmetry/tests/the_classification.rs` and
`mock/crates/arvo-format/src/apply/tests/mod.rs` bearing on rounding modes.
None is tautological. `the_control_a_wrong_classification_would_be_caught`
and `the_control_the_two_symmetries_are_not_the_same_question` are real
controls: each names a wrong answer and shows it rejected, and the second
shows the two properties are measured independently rather than one
constructed as the other's negation. `half_up_goes_away_from_zero_on_a_tie_
and_half_even_goes_to_the_even_slot` is a genuine assertion, not a
self-comparison: it names the position (`Slot::at(-3)`, `Fraction::HALF`),
asserts the concrete output (`-3`), and the sibling positive-tie case in the
same test pins the mirror. Nothing here to refuse on.

## The answer

**Ties away from zero.**

### 1. IEEE 754 has no rounding rule this closed vocabulary's other reading
   could denote

`mock/registry/ruling.toml` names the two standards in scope for the
standards-parity bound as MATLAB `fi`/`fimath` and IEEE 754. Fetched primary,
committed at `.data/fetched/0bd303ba9f316ed279c1e6632a977e729e6dfc61857cf5ef77b96d675f4bb193.html`
(`en.wikipedia.org/wiki/IEEE_754`):

> The standard defines five rounding rules. The first two rules round to a
> nearest value; the others are called directed roundings: Round to nearest,
> ties to even ... Round to nearest, ties away from zero (or ties to away) ...

Five rules, named exhaustively: ties-to-even, ties-away-from-zero, and three
directed roundings (toward positive infinity, toward negative infinity,
toward zero). There is no IEEE 754 nearest-rounding rule whose tie behaviour
is "toward positive infinity". Reading `half_up` that way gives it nothing in
IEEE 754 to be in parity with; reading it as ties-away-from-zero gives it
`roundTiesToAway`, one of the standard's own five, by name.

### 2. MATLAB's own vocabulary, read exactly, cuts the other way from what
   its default suggests

Fetched, committed at
`.data/fetched/c3201caf126426b5bdbdfaf6209a4fbb5defe43c5abde902cef87c4bcf07dfd5.html`
(`mathworks.com/help/matlab/ref/fimath.html`):

> RoundingMethod — Rounding method to use
> Nearest (default) | Ceiling | Convergent | Zero | Floor | Round
> Nearest — Round toward nearest. Ties round toward positive infinity.
> Convergent — Round toward nearest. Ties round to the nearest even stored
> integer (least biased).
> ... `F = fimath` ... `F = RoundingMethod: Nearest` ...

MATLAB's default (`fimath`'s default, what `fi(pi)` uses, exactly the calls
`mock/crates/arvo-format/tests/matlab_fi_parity.rs` asserts parity against) is
named `Nearest` and ties toward positive infinity. That is real, and it is
the strongest fact in either direction favouring the other reading. But
`Nearest` is not spelled `half_up`, or anything close to it. MATLAB has a
separate, differently-named method for the away-from-zero rule. Fetched,
committed at
`.data/fetched/b0ffda2a3bb28bac7a7f47fa2510d45c1566468a25bb70a22c35dbb65175cb9b.html`
(`mathworks.com/help/fixedpoint/ref/embedded.fi.round.html`):

> The nearest function rounds ties to the nearest integer toward positive
> infinity. The round function rounds ties to the nearest integer with
> greater absolute value.

MATLAB itself keeps these as two named, distinct functions, and neither is
called `half_up`. The question is not "which does MATLAB's default use," it
is "which operation does the six-name vocabulary's `half_up` denote," and the
word `half_up` is not MATLAB's word at all.

### 3. Where `half_up` is a real name elsewhere, it means away from zero

Fetched, committed at
`.data/fetched/2250296fa1776a289b866d9d315d1a28cad1452b08801553af6d13f1b841b17a.html`
(`docs.python.org/3/library/decimal.html`):

> decimal.ROUND_HALF_UP — Round to nearest with ties going away from zero.

Fetched, committed at
`.data/fetched/dc2b8fa7078c0581090bb1a0e485666657cccc7657fc05c91b94803f82bdf6b6.html`
(Java `RoundingMode` javadoc), from the worked table: input `-2.5` under
`HALF_UP` gives `-3`, input `-5.5` gives `-6`. Away from zero on every signed
row. Python and Java are the two ecosystems that actually spell a rounding
mode `half_up` / `HALF_UP`, and both spell it away from zero. This is the
external evidence that answers the question as posed rather than a nearby
question: not "what does MATLAB's default nearest mode do" but "what does the
name `half_up` denote where that exact name exists as prior art."

### 4. The vocabulary's own naming pattern points the same way

The six names are `toward_zero, floor, ceil, half_up, half_even, stochastic`.
`half_even` is transparently IEEE's `roundTiesToEven` spelled in the crate's
house style. IEEE pairs `roundTiesToEven` with `roundTiesToAway`, not with a
toward-positive-infinity mode, because IEEE 754 has no such mode (point 1).
The natural sibling of `half_even` under the naming pattern the vocabulary
already committed to is the away-from-zero rule, not the toward-infinity one.

### 5. Every part of the shipped crate that touches this agrees with each
   other, and only with the away-from-zero reading

`mock/crates/arvo-format/src/rounding.rs:44-45`, the doc comment on
`Mode::HalfUp`: "To the nearest, and a tie goes away from zero."

`mock/crates/arvo-format/src/apply.rs:156-165`, `round_slot`'s `HalfUp` arm:

```
Mode::HalfUp => {
    if twice > den {
        up
    } else if twice < den {
        down
    } else if exact.is_negative() {
        // A tie on a negative position goes away from zero, which is down.
        down
    } else {
        up
    }
},
```

`mock/crates/arvo-format/src/apply/tests/mod.rs:269-282`,
`half_up_goes_away_from_zero_on_a_tie_and_half_even_goes_to_the_even_slot`:
asserts `round_slot(Mode::HalfUp, Exact::between(Slot::at(-3), HALF), ...) ==
-3` (away from zero, not `-2`).

`mock/crates/arvo-format/src/symmetry.rs:137-143`, `behaviour_of`: `Mode::
HalfUp` classified `reflects: Bool::of(true)`, meaning the mode commutes with
reflection through zero. That classification is only true of the away-from-
zero reading. Toward-positive-infinity does not reflect: negating a tie that
rounds up at a positive position produces a tie that should round down at the
mirrored negative position under reflection, and toward-+inf instead rounds
it up too. Ran the crate's own check,
`symmetry::tests::the_classification::the_reflection_fact_of_every_mode_
agrees_with_the_map`, which independently derives `reflects` by sweeping the
signed band and comparing `rounded(mode, slot)` against `-rounded(mode,
-slot)`: green, in this worktree, at this commit.

And `mock/crates/arvo-format/src/symmetry/tests/the_classification.rs:220-
231`'s own comment, written by whoever shipped this file: "A nearest rule
whose tie went toward positive infinity would read nothing beyond the residue
and still commute with reflection away from a tie, so the day a seventh name
lands this is the arm that reports it." The shipped test suite already knows
about the other reading, names it explicitly as a hypothetical outside the
six, and is built to flag it if it ever arrives. That is not an accident four
independent pieces of code converged on; it is one deliberate choice
propagated consistently through a doc comment, an arithmetic rule, a
regression test, and a symmetry classifier with its own self-check.

## Provenance

First implementation: `9b612941` ("feat: apply the adaptation, and delete
four tests that checked declarations", 2026-08-31 23:05:33), which introduces
`round_slot`'s `HalfUp` arm with exactly the comment quoted above ("A tie on
a negative position goes away from zero, which is down."). Away from zero was
the reading from the first line of code that implemented any tie rule at all;
nothing shows a toward-+inf implementation ever being the crate's committed
state on `dev`.

The question row itself: `eb059dec` ("docs: file the question of which
operation `half_up` denotes"), reachable from `d425aff5`. The row's `note`
records that both readings are implemented somewhere in the panel's research
tree today (`226_probes/p6_...rs` computes toward-+inf; the shipped crate
computes away-from-zero), which is what makes this a real open question
rather than a typo.

## What each reading costs, against recorded rows

**Away from zero** (this answer). Cost, exactly as the question row's own
option 2 states: `law::fusing_a_multiply_add_preserves_the_answer_under_
signed_wrapping` currently
lists `rounding: in {floor, ceil, half_up}` in its `holds` region under
signed wrapping. That is false of the shipped, away-from-zero `HalfUp`.
Probe `270_probes/p1_half_up_breaks_translation_equivariance_at_zero.rs`,
committed with output, transcribes `round_slot`'s `Floor`, `Ceil` and
`HalfUp` arms over integers and checks `round(x + 1) == round(x) + 1` at the
tie `x = -0.5` (slot `-1`, the exact signed-wrapping, at-a-tie region the law
claims). C1 and C2 (floor, ceil) pass equivariant as they must; C4 (shipped
half_up) fails equivariant, i.e. the law's claim is false of the code it is
supposedly a fact about; C5 (the alternate, toward-+inf reading) passes
equivariant, confirming the law's `holds` clause is a true statement about
the *other* reading. The law row needs its `holds`/`fails` corrected to move
`half_up` out of the equivariant three and into the failing set alongside
`toward_zero`, `away from zero`, `half_even`. It is a `law` row (measured,
no `rung`, not human-ratified, not code); correcting a wrong measurement
costs a registry edit and nothing structural.

Same conclusion falls out independently of the FMA law, from the crate's own
symmetry classifier (point 5 above): `the_two_symmetries_partition_the_six_
shipped_names` asserts `translating == 3`, and the shipped away-from-zero
`HalfUp` is not one of the three (it is in the `reflecting` three instead).
Green today. Nobody has to touch `symmetry.rs`; it is already stating the
away-from-zero fact correctly. Choosing toward-+inf would instead put the
FMA law's current text and the symmetry classifier's current, tested, green
verdict in agreement with each other but both in disagreement with the
shipped `round_slot` arithmetic and its own regression test, which is a
larger and more invasive correction: the arithmetic, the doc comment, the
regression test, and the symmetry classifier's `reflects` value would all
need to change together, not one measured law row alone.

**Ties toward positive infinity.** Cost, per the question row's own option 1:
the mode carries a half-LSB bias per tie on a signed domain, and its mirror
under negation (all-ties-down) is not one of the six, so the vocabulary
cannot state the mirror of a claim it can state. On top of that, per this
seat's reading above: it has no IEEE 754 counterpart at all (point 1), the
name `half_up` is not MATLAB's name for it (point 2), no ecosystem that
literally spells a mode `half_up` uses this reading (point 3), and it
requires correcting four independent, internally-agreeing, currently-green
pieces of the shipped crate rather than one measured law row (point 5).

**Two names, one per reading** (the question row's option 3). Not weighed
further here beyond noting it is out of this seat's reach on canon grounds
alone: `ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_
names` is `ratified_by = "op"` and closes the vocabulary at exactly six.
Op's own words are "the six below are the whole of it... closed by the
design rather than because six felt like enough" (crate doc comment,
`rounding.rs:8-9`, itself downstream of the ruling). Widening the set is not
a call this question can make; it would need to go back through the same
ratification the six-name closure went through, on its own predicate (the
question row's own note: `(6,2)`, `(6,3)`, `(8,4)`, `(8,5)` at `signedness =
signed` only, sourced to an excluded seat's probe this reader has not opened
and is not relying on).

## Does the shipped `Mode::HalfUp` agree with this answer

Yes. `mock/crates/arvo-format/src/rounding.rs:44-45` (doc comment),
`mock/crates/arvo-format/src/apply.rs:156-165` (`round_slot`'s `HalfUp` arm),
`mock/crates/arvo-format/src/apply/tests/mod.rs:269-282` (the regression
test), and `mock/crates/arvo-format/src/symmetry.rs:137-143` (the `reflects`
classification, independently checked green by
`symmetry/tests/the_classification.rs:196-210`) all agree with each other and
with this answer. Per this seat's ladder, shipped code is not evidence of
what a name should mean; here it happens to converge with the strongest
external evidence (points 1 through 4) rather than substitute for it.

## The text a ruling row would carry

```
id = "half_up_denotes_ties_away_from_zero"
kind = "ruling"
rung = "??? (panel-decided; needs a second blind instance before promotion)"
topic = "rounding"
says = "half_up denotes nearest, ties away from zero: the operation Python's
  decimal.ROUND_HALF_UP and Java's RoundingMode.HALF_UP name, and IEEE 754's
  own roundTiesToAway, one of its five defined rounding rules. It is not
  MATLAB fimath's default 'Nearest' method, which ties toward positive
  infinity under a different name; MATLAB's own away-from-zero method is
  named 'Round', not 'Nearest', and is the closer match to what 'half_up'
  denotes in every ecosystem that spells a mode that name. IEEE 754 has no
  rounding rule at all whose tie behaviour is toward positive infinity, so
  that reading has nothing in either named standard to be in parity with."
because = "Standards-parity evidence and the vocabulary's own naming pattern
  (half_even echoing IEEE's ties-to-even, its natural sibling being IEEE's
  ties-away-from-zero rather than a mode IEEE does not define) both point one
  way; the shipped crate's arithmetic, doc comment, regression test and
  symmetry classification already agree with each other and with that
  reading."
corrects = ["fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping"]
note = "The correction moves half_up from the law's holds region to its
  fails region, confirmed by 270_probes/p1_half_up_breaks_translation_
  equivariance_at_zero.rs: the shipped away-from-zero HalfUp fails
  round(x+k)==round(x)+k at the signed tie x=-0.5, k=1, where floor and ceil
  both pass and the alternate toward-+inf reading also passes. The
  unsigned twin, fusing_a_multiply_add_preserves_the_answer_under_unsigned,
  is untouched: away-from-zero and toward-+inf coincide wherever every value
  is non-negative."
provenance = ["panel::202608072330_the-numeral-canon-panel::270_dolan_half_up_denotes_ties_away_from_zero"]
keywords = ["rounding", "half_up", "tie", "away from zero", "toward positive
  infinity", "IEEE 754", "MATLAB", "fimath", "roundTiesToAway", "python",
  "java", "decimal", "RoundingMode", "translation equivariance", "reflects"]
```
