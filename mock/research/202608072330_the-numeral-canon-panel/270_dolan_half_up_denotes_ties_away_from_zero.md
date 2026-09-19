# Seat 270. `half_up` denotes ties toward positive infinity

Blind read. Excluded from this read: everything under this directory whose
name starts `228_`, `229_`, `267_`, `268_`, `269_`, and their probe
directories, and the scratchpad. `git log --all` surfaced two commit subjects
from seat 269's branch in the course of a provenance search (`research: audit
two reads of half_up and answer it as ties to +inf`); neither was opened, and
neither is weighed below as evidence.

## Revision

This file originally answered "ties away from zero", filed under the title
this file still carries for the audit trail. The coordinator pointed at
`125_knuth_rounding_cold_derivation.md`, a non-excluded, dev-reachable source
this seat had not read. Having read it, the answer reverses to ties toward
positive infinity. The original five points are kept below with a note on
what each is now worth, because the reversal is a finding about their
relative weight, not a retraction of what they say. The new material is in
"What 125 changes" and "Rows and laws descending from 125", answering the
coordinator's two questions directly.

## Canon gate

`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` is
`rung = "ratified"`, `ratified_by = "op"`. It closes the vocabulary at six
names, `half_up` among them, and its own `note` says the naming was the only
thing open, "no position is overturned by this." It does not say what
`half_up` denotes. Op's ratification transcript
(`.data/op-responses/202608311735_arvo-canon-four-questions.md:88-104`)
answers only the retire-the-word question; the transcript's four questions do
not reach tie direction anywhere. `question::which_operation_half_up_denotes`
is therefore live: the ruling that created the name did not settle what it
means, and nothing else in op's corpus does either. Not misposed, not already
answered. Proceeding.

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
same test pins the mirror. Nothing here to refuse on. One addition on
re-reading with 125 in view: this suite tests `round_slot`'s output against
itself, by construction, everywhere it derives a property (`reads`, `when`,
`reflects`) by sweeping the same function the property is supposedly a fact
about. That is a legitimate regression suite. It is not independent evidence
for what `half_up` should mean, which is the correction section 5 below now
makes explicit.

## The answer

**Ties toward positive infinity.**

## What 125 changes, and why it outweighs what stood before

`125_knuth_rounding_cold_derivation.md` is the traceable origin of the
six-name vocabulary itself. Section 7 states it plainly: "My suggestion... the
canon should retire both spellings and name the modes `floor`, `ceil`,
`toward_zero`, `half_up`, `half_even`, `stochastic`" (line 326). Section 8's
phase-two update repeats the same list (line 597) after resolving the
`trunc`/`toward_zero` half of the naming question against probe evidence.
This is not a document that happens to use the name `half_up`; it is the
document that proposed it, together with the other five, as the replacement
for the retired word.

`125` defines `half_up` once, in the setting section, before any theorem
depends on it: "`half_up` (nearest, ties toward positive infinity)" (line
53), and again as a formula: "`half_up(x) = floor(x + q/2)`" (line 110),
which is the standard closed form for ties-toward-positive-infinity (add a
half quantum, then floor; the addition is translation-equivariant and floor
is translation-equivariant, so the composite is too). Every theorem that
follows, T3 through T8, is proved against that definition. T5's equivariance
table states `half_up` is translation-equivariant and not negation-
equivariant, the opposite of what the shipped `away-from-zero` reading gives
(negation-equivariant, not translation-equivariant, confirmed independently
by this seat's own probe, C4 and C5 below). T7 derives from T5 that `half_up`
commutes with wrapping because it is quantum-equivariant, grouping it with
`floor` and `ceil` rather than with `toward_zero`.

That grouping is not a dead end in `125`'s own file. `131_leroy_formalising_
the_rounding_axis.md`, a second persona's formalising pass, restates it at
R3: "Against wrapping, the modes equivariant under translation by the
quantum commute, which is `floor`, `ceil`, `half_up` and... `half_even`"
(lines 165-166), explicitly citing its source: "measured with the witness
`125` T7 names" (line 173). `151_leroy_the_candidate_revised_against_four_
signatures.md`, later still, derives the same grouping from first principles
again, now checked against three separate instruments (`147`, `149`, and
`151`'s own `v1` probe) rather than carried by citation: "Fusing a
multiply-add is answer-preserving exactly where the rounding position is
translation equivariant on the domain the cell reaches... Under signed
wrapping it is three of six" (lines 90-92), predicate list `rounding in
{floor, ceiling, toward zero, away from zero, nearest-half-up, nearest-half-
even}` (line 99, "away from zero" here is the separate directed mode this
table also sweeps, not a second name for `half_up`'s tie rule; the sitting's
own account of that distinction is in the question row this seat is
answering, not in 151). This is the derivation that is currently sitting,
word for word, in the registry: `law::fusing_a_multiply_add_preserves_the_
answer_under_signed_wrapping`'s `holds` region names exactly `{floor, ceil,
half_up}` as translation equivariant under signed wrapping, which is the
`125`/`131`/`151` claim, unmodified, still standing.

So the ratified naming and the shipped code both post-date `125` by weeks,
but they descend from it on opposite paths. `git log` order, all on `dev`:
`a42f0b17` (`125`'s cold derivation, 2026-08-14 23:28) precedes `807d4b77`
(op's ratification of the six names, 2026-08-31 17:48) by seventeen days,
which precedes `9b612941` (the shipped crate's first `HalfUp` arm, 2026-08-31
23:05, five hours later the same day) by five hours. The naming that was
ratified is the naming `125` proposed. The definition that was implemented,
five hours after ratification, is not the definition `125` gave the name it
was implementing. Nothing in the panel's chain between `125` and the
ratification revisits or contests `125`'s tie direction; `128`, `130` correct
a different clause of `125` entirely (F127-2's midpoint identification, an
off-by-one at a different boundary, not the equivariance table), and `131`
and `151` both reaffirm it. The shipped code is the one point in this whole
chain where the reading changes, and it changes silently, with no panel
document recording the switch.

This is the "design is the oracle, code is not an oracle for design" test
applied directly. `125` through `151` is design-track material: proposal-
tier, unratified, presumed wrong until shown otherwise exactly as this
seat's ladder states, but reasoned from proofs (T1 through T9) and checked
across three independent instruments in `151`, not asserted. The shipped
`round_slot` arm is code, sitting below both the ratified naming and this
unbroken proposal chain in the canon-design-code order, and it is where
`nothing may appear that the design does not say` is violated: nothing in
the design track says `half_up` is away from zero, and the code says it
anyway. Per that chain, the code is what is wrong, not the design track.

### What this does to my original five points

Point 1 (IEEE 754 has no toward-positive-infinity nearest rule) and point 3
(Python and Java spell `HALF_UP` away from zero) are unchanged as facts and
now carry less weight: they are evidence about what a rounding library
outside this repository would call this operation, not about what this
design's own six-name vocabulary, coined and defined in `125`, means by it.
`125` never claims standards parity for the name; it derives a mode taxonomy
and proposes names for what it derived. Point 2 (MATLAB's `Nearest` versus
`Round`) stands as a fact and is now beside the point for the same reason.
Point 4 (the `half_even`/`roundTiesToEven` naming-sibling argument) is
weakened further than "beside the point": it is empirically wrong about what
the coiner of the name intended, since `125` used `half_even` and `half_up`
side by side while defining `half_up` as ties toward positive infinity, the
opposite of the IEEE-pairing heuristic's prediction. Point 5 (four shipped-
code artifacts agreeing) is the one that needed a real correction on
re-reading. `symmetry.rs`'s `reflects` derivation and its regression test
both call `round_slot` (confirmed: `symmetry/tests/the_classification.rs`'s
`rounded()` helper calls `adapt_at`, which reaches `round_slot`), so they are
not independent confirmations of what `half_up` should mean. They are one
implementation choice (`round_slot`'s `HalfUp` arm) and two mechanical
consequences of measuring that same function. Point 5's framing, "four
independent pieces of code converged," overstated the independence; it is
one choice propagated through three checks that cannot help but agree with
it, because they measure it rather than reason about it separately.

## Rows and laws descending from 125, and whether the revised answer holds under them

`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping`
(`holds`: `rounding: in {floor, ceil, half_up}`, translation equivariant
under signed wrapping): traced above through `125` T5/T7 and `131` R3 to
`151`'s three-instrument re-derivation, which is what its `provenance` field
cites. Under ties toward positive infinity this law's `holds` clause is true
exactly as written, confirmed by this seat's own probe (`270_probes/p1_...
rs`, case C5: the toward-+inf reading passes `round(x+k) == round(x)+k` at
the signed tie `x = -0.5, k = 1`). No correction is owed to this row. My
original file recommended correcting it (moving `half_up` from `holds` to
`fails`); that recommendation is withdrawn. It was backwards: the row was
already correct, and it was the shipped code that needed to change to match
it, not the row that needed to change to match the code.

`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned` (`holds`
includes `half_up` among five of six free positions): unaffected by this
question either way, since on a non-negative domain ties-toward-positive-
infinity and ties-away-from-zero coincide (both round up), which is exactly
`125`'s own point in deriving the unsigned case first (T1b, one-signed form)
before deriving the signed case where the two readings first diverge.

`proposal::fusing_a_multiply_add_is_free_exactly_at_translation_equivariance`,
the row consolidating both laws into one equivariance-based statement: same
lineage, same conclusion, holds under the revised answer with no change.

So the revised answer holds cleanly under every recorded row that descends
from `125`'s definition, with zero corrections owed to the registry. What
is owed is a correction to `mock/crates/arvo-format`: `apply.rs`'s
`round_slot` `HalfUp` arm, `rounding.rs`'s doc comment, `apply/tests/mod.rs`'s
regression test (the tie at `Slot::at(-3)` should give `-2`, not `-3`, and
the doc comment on that test needs "away from zero" replaced), and
`symmetry.rs`'s `behaviour_of` classification for `HalfUp` (`reflects` moves
from `true` to `false`, and `the_two_symmetries_partition_the_six_shipped_
names`'s hardcoded `translating == 3` / `reflecting == 3` assertions become
`4` and `2`, since `half_up` joins the translating set with `floor`, `ceil`
and `stochastic`, once the arithmetic is corrected to match). That correction
is design-and-implementation work, out of scope for this canon question, and
is named here as what the answer implies rather than undertaken by this seat.

## Provenance

First implementation of the away-from-zero reading in code: `9b612941`
("feat: apply the adaptation, and delete four tests that checked
declarations", 2026-08-31 23:05:33), which introduces `round_slot`'s
`HalfUp` arm with the comment "A tie on a negative position goes away from
zero, which is down." First definition of the name and its intended
denotation: `a42f0b17` ("research: 125 cold derivation of the rounding axis,
predictions committed before probes", 2026-08-14 23:28:32), seventeen days
earlier, defining `half_up` as ties toward positive infinity and proposing
the name as part of the six. The ratification, `807d4b77` ("canon: ratify the
last four answers and record the handover", 2026-08-31 17:48:37), sits
between the two, five hours before the code and seventeen days after the
definition, and settles only the naming, not the denotation (confirmed by
reading `217_op_the_last_round_and_the_panel_finishes_alone.md`'s "The
rounding vocabulary retires the ambiguous word" section, which discusses only
the bit-drop-versus-`toward_zero` ambiguity and never mentions `half_up`'s
tie direction).

The question row itself: `eb059dec` ("docs: file the question of which
operation `half_up` denotes"), reachable from `d425aff5`. The row's `note`
records that both readings are implemented somewhere in the panel's research
tree today (`226_probes/p6_...rs` computes toward-+inf; the shipped crate
computes away-from-zero), which is what makes this a real open question
rather than a typo, and which this seat now reads as the design track and
the shipped code disagreeing, with the design track earlier, more numerous,
and multiply re-derived.

## What each reading costs, against recorded rows

**Ties toward positive infinity** (this answer). Cost, per the question
row's own option 1: the mode carries a half-LSB bias per tie on a signed
domain, and its mirror under negation (all-ties-down) is not one of the six,
so the vocabulary cannot state the mirror of a claim it can state. Beyond
that: it requires correcting `arvo-format`'s shipped `HalfUp` arithmetic,
doc comment, regression test and symmetry classification, as detailed above,
none of which is a canon-tier cost, all of which is ordinary design-and-
implementation follow-up. It requires no correction to any recorded `law` or
`proposal` row; every row this seat found descending from `125`'s definition
already states this reading and needs nothing changed.

**Ties away from zero.** Cost: it is what `mock/crates/arvo-format` currently
implements, so choosing it costs nothing in the crate today. It costs
`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping`,
whose `holds` region would need `half_up` moved out, undoing a claim that
three separate documents (`125`, `131`, `151`) independently reasoned to and
that the currently-registered row already states correctly under the other
reading. It has no textual anchor in the document that coined the name: `125`
defines `half_up` as ties toward positive infinity in the same file that
proposes calling it `half_up`, so "away from zero" is not what the name's
own author meant by it. External convention (Python, Java) and the shipped
crate both support it, but per this seat's ladder neither outranks a
proposal-tier design document reasoned from proofs and reaffirmed twice.

**Two names, one per reading** (the question row's option 3). Unchanged from
the original file: out of this seat's reach on canon grounds. `ruling::the_
ambiguous_rounding_word_is_retired_for_six_explicit_names` is `ratified_by =
"op"` and closes the vocabulary at exactly six. Widening it is not a call
this question can make on its own; it would need to go back through the same
ratification the six-name closure went through, on its own predicate (the
question row's own note: `(6,2)`, `(6,3)`, `(8,4)`, `(8,5)` at `signedness =
signed` only, sourced to an excluded seat's probe this reader has not opened
and is not relying on).

## Does the shipped `Mode::HalfUp` agree with this answer

No. `mock/crates/arvo-format/src/rounding.rs:44-45` (doc comment),
`mock/crates/arvo-format/src/apply.rs:156-165` (`round_slot`'s `HalfUp`
arm), `mock/crates/arvo-format/src/apply/tests/mod.rs:269-282` (the
regression test), and `mock/crates/arvo-format/src/symmetry.rs:137-143`
(the `reflects` classification) all implement and assert ties away from
zero. Per this seat's ladder, that disagreement is not evidence against this
answer; the design is the oracle for the code, not the reverse, and the
design track (`125` through `151`, unbroken, twice reaffirmed, seventeen
days ahead of the code) says ties toward positive infinity. The shipped
crate is the artifact that needs correcting.

## The text a ruling row would carry

```
id = "half_up_denotes_ties_toward_positive_infinity"
kind = "ruling"
rung = "??? (panel-decided; needs a second blind instance before promotion)"
topic = "rounding"
says = "half_up denotes nearest, ties toward positive infinity: the
  definition 125 gave the name when it proposed the six-name vocabulary
  (half_up(x) = floor(x + q/2)), reaffirmed independently in 131 and derived
  again from three instruments in 151, and currently standing unmodified in
  law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping's
  holds region. It is not IEEE 754's roundTiesToAway, and not Python's or
  Java's HALF_UP, which are away from zero under the same name in those
  ecosystems; this design's own coinage of half_up predates and disagrees
  with that convention, and the design track that coined it governs the
  name inside this vocabulary."
because = "The document that proposed the six-name vocabulary defines
  half_up as ties toward positive infinity in the same breath as proposing
  the name, every later design-track document that touches the claim
  reaffirms it rather than revising it, and the currently-registered FMA law
  already states it correctly. The shipped crate's away-from-zero
  implementation has no design-track document behind it and first appears
  five hours after ratification with no recorded reasoning for the switch."
corrects = []
note = "No registry row needs correcting under this reading; the crate does.
  mock/crates/arvo-format's round_slot HalfUp arm, its doc comment, its
  regression test half_up_goes_away_from_zero_on_a_tie_and_half_even_goes_
  to_the_even_slot, and symmetry.rs's behaviour_of classification for
  HalfUp (reflects: true -> false, and the_two_symmetries_partition_the_
  six_shipped_names's translating/reflecting counts move from 3/3 to 4/2)
  all need updating to match. Confirmed by 270_probes/p1_half_up_breaks_
  translation_equivariance_at_zero.rs: the shipped away-from-zero HalfUp
  fails round(x+k)==round(x)+k at the signed tie x=-0.5, k=1, where floor,
  ceil, and the toward-+inf reading of half_up all pass. The unsigned
  twin law is untouched either way: away-from-zero and toward-+inf coincide
  wherever every value is non-negative."
provenance = ["panel::202608072330_the-numeral-canon-panel::125_knuth_rounding_cold_derivation", "panel::202608072330_the-numeral-canon-panel::131_leroy_formalising_the_rounding_axis", "panel::202608072330_the-numeral-canon-panel::151_leroy_the_candidate_revised_against_four_signatures", "panel::202608072330_the-numeral-canon-panel::270_dolan_half_up_denotes_ties_away_from_zero"]
keywords = ["rounding", "half_up", "tie", "toward positive infinity", "away
  from zero", "translation equivariance", "reflects", "IEEE 754", "python",
  "java", "decimal", "RoundingMode", "design track", "shipped code"]
```

## Appendix: the original reading, kept for the audit trail

The sections below are the file as it stood before this revision. They are
left in place rather than deleted because `undo-by-fixing-forward-not-by-
reverting` applies to a seat's own file as much as to anything else: the
reasoning was real, the citations check out as stated, and the reversal is
about which body of evidence outranks which, not about any fact here being
false.

### 1. IEEE 754 has no rounding rule this closed vocabulary's other reading could denote

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
is "toward positive infinity". This is true and, per the revision above, is
evidence about standards parity, a different concern from what this design's
own vocabulary, coined in `125`, denotes by the name.

### 2. MATLAB's own vocabulary

Fetched, committed at
`.data/fetched/c3201caf126426b5bdbdfaf6209a4fbb5defe43c5abde902cef87c4bcf07dfd5.html`
(`mathworks.com/help/matlab/ref/fimath.html`):

> RoundingMethod — Rounding method to use
> Nearest (default) | Ceiling | Convergent | Zero | Floor | Round
> Nearest — Round toward nearest. Ties round toward positive infinity.
> Convergent — Round toward nearest. Ties round to the nearest even stored
> integer (least biased).
> ... `F = fimath` ... `F = RoundingMethod: Nearest` ...

MATLAB's default (`fimath`'s default, what `fi(pi)` uses) is named `Nearest`
and ties toward positive infinity, matching this seat's revised answer,
though under a different name than `half_up`. MATLAB's own away-from-zero
method is separately named `Round`. Fetched, committed at
`.data/fetched/b0ffda2a3bb28bac7a7f47fa2510d45c1566468a25bb70a22c35dbb65175cb9b.html`
(`mathworks.com/help/fixedpoint/ref/embedded.fi.round.html`):

> The nearest function rounds ties to the nearest integer toward positive
> infinity. The round function rounds ties to the nearest integer with
> greater absolute value.

### 3. Where `half_up` is a real name elsewhere, it means away from zero

Fetched, committed at
`.data/fetched/2250296fa1776a289b866d9d315d1a28cad1452b08801553af6d13f1b841b17a.html`
(`docs.python.org/3/library/decimal.html`):

> decimal.ROUND_HALF_UP — Round to nearest with ties going away from zero.

Fetched, committed at
`.data/fetched/dc2b8fa7078c0581090bb1a0e485666657cccc7657fc05c91b94803f82bdf6b6.html`
(Java `RoundingMode` javadoc), from the worked table: input `-2.5` under
`HALF_UP` gives `-3`, input `-5.5` gives `-6`. Away from zero on every signed
row. Real, and per the revision above, evidence about an external
convention this design did not adopt when it coined the name.

### 4. The vocabulary's own naming pattern

The six names are `toward_zero, floor, ceil, half_up, half_even, stochastic`.
`half_even` echoes IEEE's `roundTiesToEven`. The original file argued this
made away-from-zero the "natural sibling" for `half_up`. Per the revision
above, that argument is empirically wrong about what the coiner of both
names intended: `125` defines `half_even` and `half_up` side by side (line
53), with `half_up` as ties toward positive infinity, not away from zero.

### 5. The shipped crate

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
-3`. `mock/crates/arvo-format/src/symmetry.rs:137-143`, `behaviour_of`:
`Mode::HalfUp` classified `reflects: Bool::of(true)`. Per the revision
above, the last two are not independent of the first two: both call
`round_slot` through `adapt`/`adapt_at`, so they are downstream consequences
of one implementation choice rather than four independent confirmations.
