# 214. A standing nobody can check is not a standing

**Author lens:** Jhala. Refinement types, what a checker can carry and what it must be handed,
and the habit of asking what would have to be false for a claim to fail.
**Position:** second read of the twenty-one `the_number_system` proposals, dispatched after `213`
made expert convergence a ratification and the coordinator its gate.
**Probes:** `214_probes/`, one instrument with its control, committed with this file. The rest of
the work landed as arms in `mock/checks/src/provenance.rs` with their tests, because this
repository has a harness in its own language and a check belongs in it rather than in a script.

**The answer in one line, before the argument.** I could not raise the standing of a single one of
the seventeen rows I was sent to second-read, and the reason is not that they are wrong: it is that
seventeen of the twenty-one cite the consolidation instead of what established them, so the only
way to evaluate them was to read the files that established them, and having read those files I am
a reader rather than an arrival. What I could do instead was recover the provenance the port
dropped, which is stronger than my agreement would have been, and measure the class it belongs to.

## 0. Gates

**Canon gate: passes.** `mockspace.toml` declares `canon_paths = ["mock/registry/*.toml"]`, so the
registry is the canon. Two rulings bear on this dispatch and neither forbids it.
`ruling::the_canon_is_written_once_at_the_end` says nothing moves into the canon until every topic
is done; raising `standing` is not that move, since promotion is a `ruling` naming a row under
`ratifies`, and I have written no ruling. `213` makes two experts a ratification with the
coordinator gating, quoted in the brief and checked at the source: both quotations are verbatim and
accurate, and the brief compressed away a third clause worth having, that convergence is the bar
and not the trigger, so a promotion carries the judgement that justified it rather than a count.

**Test gate: passes, and the suite is real.** `cargo test -p arvo-checks` was green before I
touched anything (26 + 8 + the rest, 140 aggregate) and is green after. I read the bodies rather
than the names in the surface I touched: `citation.rs` in full, `lib.rs` in full,
`no_line_citation_into_a_living_ledger.rs` in full, and the standing-related arms of
`what_one_field_obliges_another_to_carry.rs`. **The suite is not decorative.** Every arm is a pure
function from rows to findings, every test file plants an input as well as reading the committed
canon, and the controls are the kind that would fail if the arm were vacuous: a heading citation
into a ledger must pass, a line into a numbered member file must pass, two probes with identical
control text differing only in `standing` must be distinguished. That is the discipline this file
tries to match.

**One thing to report outside the question.** `mock/registry_catalogue/panel_050_099.toml:67`
states, as the reason a row was not flagged, that *"op's own standing instruction (rule 87) is that
a consolidation is input rather than a citation-worthy source in its own right until the whole
canon is written, so citing 53 rather than 50 is the correct current shape, not a gap."* The
premise is right and the conclusion is its opposite. `ruling::the_canon_is_written_once_at_the_end`
says a consolidation *"has no standing beyond"* being a compression and *"is input, not canon in
miniature"*. From that it does not follow that a row should cite the consolidation; it follows that
a row citing only the consolidation rests on something with no standing. Two true statements welded
into a connection neither supports, which is the shape that survives review because both halves
check out. That sentence is the recorded reason a whole class went unrepaired.

## 1. What I have to disclose before anything else

**My extraction script leaked the rows' own `says` and `because` for eighteen of the twenty-one
before I opened a single source file.** I was pulling `id`, `standing` and `provenance` out of the
TOML and my record separator was wrong, so the whole block printed. That is the contamination the
dispatch's method exists to prevent, and I am not going to describe it as a partial success.

What it cost, precisely, and what it did not:

- **It cost the blind derivation.** I cannot claim to have reached any of these sentences without
  having seen them. Nothing below claims it.
- **It did not cost the standing analysis**, which is the part that gates promotion. Whether a
  claim was reached independently by two authors is a question about files 65 to 73, and reading
  74's restatement of a claim tells you nothing about how many of those files contain it. That
  question was open when I started reading the sources and is what the rest of this file answers.
- **It did not cost the region analysis**, which is a comparison between a row's sentence and the
  parameters its instrument was run at. Both sides of that comparison are documents.

**And the contamination would have happened anyway, in a milder form, because the row ids are the
claims.** `derivation_is_completion_of_the_sequence_by_the_typestate` is the sentence. There is no
reading order for this registry under which a second reader is blind to what the rows say. That is
worth stating as a fact about the schema rather than as an excuse: **a slug that is a sentence
makes a blind second read of that sentence impossible**, and every future second-read dispatch
inherits it.

## 2. The structural finding: the port carried every claim and dropped every source

`74` is the topic's consolidation and its section 4 is careful. Each of its twenty-three candidate
sentences names, in parentheses, the member files and line ranges that established it: N1 at
`67:613-617` and `72:214-219`, N4 at `71:658-665` and `71:711-718`, N10 at `73:667-675`, N15 at
`70:352-367`, N19 at `65:502-504`, N20 at `65:519-521` and `66:255-270`, and so on across eight
distinct member files.

**All twenty-three reached the registry. Not one of those citations did.** Seventeen of the
twenty-one rows in this topic carried a single provenance entry pointing at a line inside `74`.

This is the shape `a-compression-is-checked-by-someone-else` measured once already, in its own
words: *a rewrite carrying 17 source citations where its predecessor carried 78, where every claim
survived and only the ability to check them died.* A claim-by-claim entailment check scores it
clean and is right to, because content and verifiability are different axes.

**And the class had already been found once and fixed once.**
`proposal::conversion_and_resolution_are_one_obligation_at_two_arities` carries a note recording
exactly this repair: *"Provenance previously cited only 74's consolidation; `66` is added here per
a same-day entailment check on the consolidation finding this credit dropped."* One instance
repaired, and the grep that would have found the other sixteen was never run. That is
`fix-the-class-not-the-instance-named` in its plainest available form, and it cost this dispatch
most of its budget, because recovering a dropped citation means reading the file it was dropped
from.

### The measurement

`mock/checks/src/provenance.rs` carries two arms and their tests. Over the committed registry,
before this file's repair:

- **65 proposal rows rest only on a consolidation.** Against 122 proposals, that is a bit over
  half of the namespace.
- **30 proposal rows assert more than one independent arrival** (`two_experts`, `three_or_more` or
  `cross_topic`) **while naming at most one distinct file.**

**The second number is the one that matters now.** Under `213`, those thirty are precisely the rows
eligible for promotion without op, and not one of them lets a reader reach the second arrival it
claims. A row saying `cross_topic`, which the schema glosses as *separate topics arriving without
citing each other*, **cannot be exhibiting that while citing one file**, because one file has one
author. The claim may be true. It is not checkable, and under the new rule it is checkability that
the coordinator's gate needs.

After the repair below: **48 and 29.**

## 3. `standing` is a declaration nothing constrains, and here is the mutation that shows it

Stated as a hypothesis before the run, with the case that must fail run first, because a probe with
no failing case establishes nothing. Transcript at `214_probes/standing_is_unconstrained.txt`.

**Control.** Set `standing` on a proposal row to `seventeen_experts`. The schema refuses it by
name: `error: "seventeen_experts" is not one of ["one_expert","two_experts","three_or_more",
"cross_topic","contested"]`. So the gate is live and a green below is a measurement rather than an
absence.

**Measurement.** Set the same field on the same row to `cross_topic`, the strongest tier the panel
produces, leaving its single citation into `74` untouched. `cargo mock --lint-only` reports
`schema check passed` and `all lints passed`. The whole of `arvo-checks` reports 140 passed, 0
failed.

**So the answer to "what value of `standing` would make anything fail" was: only a misspelling.**
That is the `the-test-gate` clause about declarations nothing constrains, arriving in the one field
that decides how close a row is to canon. The arms in `provenance.rs` are the repair, and they are
necessary conditions rather than sufficient ones: neither can tell whether two authors really
arrived separately, and neither pretends to. What they establish is that a reader can go and find
out.

A third arm went in and found nothing, which is worth recording as a clean negative:
`an_imposition_resting_on_an_instrument` reports a row filed `normative` or `definition`, both of
which carry no region because they are imposed rather than established, while pointing at a probe.
Zero in the committed registry. It stays as a guard.

## 4. The repair, and why it is stronger than my agreement

For each row I opened the member file `74` names, checked the anchor resolves to the sentence
`74` attributed to it, and added it to `provenance` beside the existing citation into `74`. **The
repair is additive on purpose.** The consolidation is where the wording came from and that is worth
recording; what was missing is where the claim came from.

Twenty of the twenty-one are repaired. The twenty-first,
`every_dispute_in_the_number_system_topic_was_a_dispute_about_an_address`, still trips the arm and
**should**: it is the consolidator's own contribution, `74` genuinely is its source, and its own
note already says it sits at the most suspect tier available. The flag there is honest signal
rather than a defect.

**Three anchors `74` gives do not land where its parentheses say**, which is worth naming because a
later reader copying them would cite the wrong paragraph. `67:613` lands inside K1's rung
statement rather than on K1, whose sentence begins at `67:608`. `67:447` is a blank line; K5 is at
`67:641` and its supporting observation at `67:448`. `66:255` is a blank line; the section is
`66:245`. I cited the corrected anchors and checked each one resolves.

## 5. The twenty-one, one at a time

Format per row: what I derived from the sources, how it compares, the outcome, and the region.
**Where I say "not an independent instance" I mean it in the sense this unit's own members used
it**: `70`, `71` and `73` each state in their own opening that they read the panel before writing
and are therefore reads rather than arrivals, and `71` refuses to be counted on Q21 in exactly
those terms. I am held to the same standard and fail it in the same way.

### N1. `the_numeral_concept_is_a_dependent_sequence_of_choices`

**Derived.** The telescope is `67`'s, section 2, with `p1_telescope.rs` enforcing the dependency
through associated types and a refusal (`E0271`) when a component declared over one identity is
attached to a term at another. Two prior discoveries are the same discovery: `55b`'s concession
that the reduction space is derived, and `56`'s finding that the encoding is ordered after the
value set. `72` amends it: the components range over sets of relations rather than functions,
because `71`'s `p8` shows a redundant encoding gives 81 strings onto 31 values.

**Comparison.** Converged with the row. **And the row understates its instruments.** `74` records
"ONE EXPERT, two instruments" citing `67` and `72`. `70` section 5 supplies a third and says so:
*"Supported with a new instrument: `67`'s K1 sequence claim. p3's forced re-instantiation is the
dependency observed from the change side, independent in mechanism from `67`'s p1."* `70` read `67`
first, so it is not a third arrival, but it is a third instrument by a different mechanism.

**Outcome: converged, standing unchanged, instrument count raised.** One expert, three instruments
(`67`'s p1 by enforcement, `70`'s p3 by unsatisfiability of "all else equal", `72`'s amendment).
Provenance gains `67:608`, `70:269`, `72:270`.

**Region.** `normative`, no region, and correct: it stipulates the shape of the concept. Nothing an
instrument could refute, and if the coordinates change the sentence is voided rather than
falsified.

### N3. `derivation_is_completion_of_the_sequence_by_the_typestate`

**Derived.** `67` section 3, and the row's `says` is `67`'s K3 **word for word**. `67` states its
own support: section 3, `p1`'s projections and size assertions, and `68`'s split of validate into
two verbs, *"which this sentence must be read with rather than against"*.

**Comparison.** Converged. Nothing to add and nothing to correct.

**Outcome: converged, standing unchanged at one expert.** Provenance gains `67:627` and `68:281`,
the second because the row's own note already says it must be read with that split and the row
could not reach it.

**Region.** `normative`, correct. It is a reading of op's acceptance criterion, not a measurement.

### N4. `a_crossing_carries_two_relations_and_a_verdict_per_law_family`

**Derived.** `71` section 2, five independences each measured: the value and pattern relations are
independent (`p1`), the two preservations are the two law families (`p3`), the count of adaptation
points is part of the meaning (`p3` regime 2), the endpoints do not determine a composite (`p2`),
and the pattern relation need not be a function at all (`p8`).

**Comparison.** Converged on content. **The region is where it parts from the row.** The row is
`normative` and therefore carries no region, and under I13 a claim with no region reads as holding
wherever it is quoted. The section-and-retraction clause is not an imposition: `p8` measured it, at
four digits over the digit set `{-1, 0, 1}`, 81 strings onto 31 values, only 2 of 16 values with a
unique image, maximum multiplicity 5, and the retraction property exhaustive over all 54 reachable
strings. That is a measurement wearing a stipulation's clothes.

**Outcome: converged narrower.** The five-class enumeration is structural and holds as written. The
independence of the three carried things, and the section-retraction case, hold at `total_width:
W = 4`, `fraction_width: F = 0`, digit set `{-1, 0, 1}`, four digits, `threads = 1`. Provenance
gains `71:658`.

### N5. `a_crossing_preserves_an_operation_exactly_when_it_moves_no_coordinate_that_operation_reads`

**Derived.** This is the strongest row in the block and the row undersells it. `67:252` carried a
universal, *"No crossing preserves operations at 100%"*, drawn from three of the five classes its
own telescope defines. `71` section 1 measured all five and refuted it. `72` section 1 accepted the
refutation and offered this biconditional as the repair, and **`72` section 2 then did the thing
that makes it a canon sentence**: it showed by mutation that `71`'s index-4 and index-5 value rows
**cannot fail**, because `System.add` reads neither the encoding nor the offset, so the check
compares a computation with itself. Giving the index-4 target an encoding mapping all sixteen
values onto the single pattern zero still returns 256 of 256.

**Comparison.** Converged, and I correct the row's note. It reads: *"`normative` is the porter's
call rather than the source's word."* **It was the source's word.** `72:90-118` is the mutation
that licenses it, in its own terms: *"A 4-bit sweep would license 'does not, at four bits'. A row
that cannot fail licenses 'never', at any width, in any language."* And `74`'s N5 says the same,
citing `72:66-74` licensed by `72:90-118`.

**Outcome: converged, and the row's own account of its support is too weak.** Standing stays one
expert, `72`'s. Provenance gains `72:66` and `72:90`. **The note should be corrected in
consolidation**, not by me editing the row.

**Region.** `normative` with no region is correct here and is *earned*, which is the rare case. The
by-construction argument plus a mutation control showing the check cannot fail is exactly the
warrant a region-free sentence needs. The two "256 of 256" figures must never be quoted as
measurements, and `72` says so.

### N6. `meaning_is_decided_by_the_first_three_coordinates_and_cost_by_the_last_two`

**Derived.** `71` section 3, its own headline. `72` section 4 calls it *"correct, and it is the best
sentence in the file"* and supplies two supports `71` does not cite, both from `67`'s `p3`, which
was built for a different question and so did not select for this: a packed run's stride leaves the
representable set constant at one distinct set over four strides, and block floating point's shared
exponent moves it, eight distinct sets over eight exponents. Both are cases where a reader would
expect the last coordinate to change meaning and in both the meaning moves only where the first
three do.

**Comparison.** Converged, and the row's note is right that this is the sentence most wanting
attack. It has now had one: `72` attacked it with the two hardest cases available and it held.

**Outcome: converged, standing unchanged.** The row's note says *"It has not had a second
derivation"*, which is true and remains true; what it has is a second reader's two independent
hard cases. Provenance gains `71:667`, `72:203`.

**Region.** The meaning half is structural. The cost half rests on `p1`'s index-4 and index-5 rows,
which `72` established are constructional at the value level and measured at the pattern level, the
pattern column at `W = 4`, `F = 0`.

### N7. `a_crossings_preservations_are_the_two_law_families`

**Derived.** Two files, two halves. `67` section 5 established the two consumer classes and the
monotone-distributivity biconditional in 6 of 6 cells with a deliberate neither-cell control.
`71`'s `p3` established the crossing pairing over twelve cells, two windows by three reductions by
two operations, with all four combinations of (monotone, coherent) inhabited so neither implies the
other. `71` also says the operation half is **definitional** once the source's own reduction is
held out of the schedule, and its probe header says so rather than presenting arithmetic as a
discovery.

**Comparison.** Converged. The row welds `67`'s consumer-class finding to `71`'s crossing pairing
and adds "the crossing is the two families' third consumer", which is `71`'s.

**Outcome: converged narrower.** One expert on the pairing, as the row says. **The order half is
measured and the row carries no region for it.** It holds at `total_width: W = 4`,
`fraction_width: F = 0`, `signedness: {unsigned, signed}`, reductions `{wrap, saturate,
opposite-bound mutant}`, operations `{add, mul}`, `threads = 1`. The operation half is definitional
and needs none. Provenance gains `71:684`, `67:647`.

### N8. `an_order_is_named_exactly_where_a_crossing_is_lossy`

**Derived.** `71` section 4 built the construction that makes the answer mean something: a step
moves exactly one coordinate, so every single-coordinate crossing is canonical and nothing is
chosen inside a step. Six unordered pairs over `{Q, rho, E, C}`, both orders each, and **exactly
one diverges**, `{Q, rho}`, agreeing on 30 of 256 source values, with 0 of the divergent witnesses
having a source value already inside the target's set. The three-coordinate case collapses to
exactly two functions and the probe checks the grouping mechanically. `72` section 5 offered the
scoping half as its own K7.

**Comparison.** Converged. The row's *"One expert on each half"* is exactly right.

**Outcome: converged narrower.** Measured at `total_width: W = 4`, `fraction_width: F = 0`,
`signedness: signed`, coordinates moved `{Q, rho, E, C}`, `threads = 1`. The row is `normative` and
carries none of it. Provenance gains `71:693`, `72:273`.

**And `71`'s own bound belongs in the record**: both routes are well typed, `p4` builds both, so
**the typestate cannot break the tie** and only a canon sentence says which the notation means.
That is the part that is genuinely op's, and the row's `gap` says so.

### N9. `conversion_and_resolution_are_one_obligation_at_two_arities`

**Derived.** `66` filed conversion and resolution as **two separable questions** and said conflating
them is the likelier mistake. `71` section 5 measured the reason they are one: three schedules for
one mixed expression, four cells, 2048 operand pairs each, with two implications asserted rather
than eyeballed. In the wrap-wrap cell all three coincide at 2048 of 2048, and `71`'s own prediction
that they always differ was refuted and kept.

**Comparison.** Converged, and this row is the one that was already repaired, which is how I found
the class. Its note is exemplary: it credits `66` for the half `66` got right and states plainly
that the unification is `71`'s and **against** `66`'s filing. A reader could otherwise take the
`66` citation as support for the unification, which it is not.

**Outcome: converged, standing unchanged.** Provenance gains `72:221`, which accepts the
unification and supplies the telescope mechanism (*an n-ary operation whose operands sit at
different terms is n crossings into a common term plus one operation performed there*).

**Region.** Measured at `W = 4`, `F = 0`, `signedness: signed`, reductions `{wrap, saturate}`,
operation `add`, `threads = 1`.

### N10. `membership_and_hosting_are_two_questions`

**Derived.** `73` section 1, M1, with the two-by-two inhabited: windowed integers and GF(2)^n host
and are systems; unbounded exact rationals and continued fractions of unbounded depth are systems
and do not host; a Gray code, two's complement and a container width host and are not systems. The
bottom-left cell is the one `73` says nobody had named.

**Comparison.** Converged. Nothing to correct.

**Outcome: converged, standing unchanged at one expert on the split, hosting half `68`'s.**
Provenance gains `73:667`.

**And this row refutes another row in the same block.** See section 6.

### N11. `a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts`

**Derived.** `73` sections 2 and 3. The sufficient direction of the prefix-3 exposure test fails
because every system has a second declaration of itself: take its own induced operation and declare
the ambient domain to be that, with the identity as the reduction. It satisfies the exposure list
verbatim and computes the identical function. Two repairs: the retraction clause for the
constant-zero hole, and the ambient law inventory for the collapse, with `p1d`'s biconditional in
16 of 16 cells and neither conjunct alone predicting it.

**Comparison.** Converged. The reasoning is careful in a way I want to name: `73` did not forbid
the collapse, because five of eight collapsed rows keep ambient associativity and for GF(2)^4 the
collapsed form is the **only** honest description, so forbidding it would forbid `65`'s K5.

**Outcome: converged, standing unchanged.** Provenance gains `73:677`, `73:184`.

**Region.** The row is `normative`, and its own refutation row is `measured` with a full predicate
at `W = 4`, `F = 0`, both signednesses, both policies, `operation = add`, `arity = 3`. **A claim and
its refutation should not be filed at two different sentence kinds**, and here the refutation has
the region while the positive statement built on it has none.

### N12. `a_closed_ambient_operation_makes_the_reduction_the_identity`

**Derived.** `67`'s K5, from `p4` measuring that min needs 0 adaptations over Q squared in all six
cells, and `73`'s M3 using it as a **construction** rather than a remark: `p3`'s two `const fn`
loops of identical shape return `true` for xor on the window and `false` for integer addition, both
asserted at compile time, **so neither assertion is a tautology because one of them is false.**
That last detail is the discipline this panel keeps rediscovering and it is done right here.

**Comparison.** Converged. Two instruments, one arrival: `73` read `67` and says so.

**Outcome: converged, standing unchanged at one expert with a construction.** Provenance gains
`73:688`, `67:641`.

**Region.** `normative`, and defensible: closure leaves the space of total reductions with one
member, which is an argument rather than a sweep.

### N13. `the_concepts_edge_is_not_an_order_and_wrapping_is_the_test`

**Derived.** `73` section 4. `p2` enumerates **every total order** on the carrier at widths 2 and 3,
24 and 40320 of them, and tests compatibility in both arguments. Wrapping addition and xor admit
**zero** compatible orders each; min admits 8 and 128; saturating addition admits 2 and 2. And the
probe prints the structural reason: both are finite groups, every non-identity element has finite
order, and a translation-invariant total order on a group forces `a > e` to give `a^k > e` for
every k, which finite order contradicts.

**Comparison.** Converged, and this is the best-evidenced row in the block. The exhaustive
enumeration is not a sample and the group argument covers every width.

**Outcome: converged, standing unchanged.** Provenance gains `73:697`, `73:723`.

**Region.** The row is `normative` and correctly carries the measurement in a law row instead,
`law::existence_of_a_translation_invariant_total_order`, *"because the law has a region and the
boundary claim does not"*. **That is the right filing and it is the only row in the block that does
it.** The enumeration holds at `total_width: W in {2, 3}` exhaustively with a structural argument
at `W any`; the natural-order spot check is at `W = 4`.

### N14. `admission_returns_a_coordinate_rather_than_a_verdict`

**Derived.** `73` section 7. Every disputed case in the register is a candidate fixing a coordinate
at an unexpected index or an unexpected owner, and `73` walks them: Gray code, two's complement,
offset binary and signed-digit at coordinate 4; stride, alignment and housing at coordinate 5;
wrapping against saturating at coordinate 3; a block exponent and a platform width at coordinate 2
with different owners.

**Comparison.** Converged. It is a composition rather than a new claim, as the row says.

**Outcome: converged, standing unchanged.** Provenance gains `73:705`.

**Region.** `normative`, correct.

### N15. `the_concept_commits_to_its_choices_and_to_no_count_of_levels`

**Derived.** `70` sections 2 and 3. `p3` applies `65`'s own change-test exhaustively at the 4-bit
model width over one baseline chain and five single-component variants, measuring four observables.
Four distinct signatures, and **the effect partition does not respect the dependency order**: `D`
and `rho` at positions one and three are observationally identical, with `Q` between them carrying
a different signature. A partition with a non-contiguous class cannot be a coarsening of the order.
`70` also kept a refuted prediction of its own in the output.

**Comparison.** Converged. And `70` is explicit that it read the panel first, so the caller-census
criterion it shares with `67` is a read.

**Outcome: converged narrower.** Provenance gains `70:352`.

**Region.** **The row welds two sentence kinds.** "A level is a partition class relative to the
observables a client brings, and a cut earns a name by having a caller" is structural. "Both
partitions are real and neither refines the other" is `p3`'s measurement, at `total_width: W = 4`,
`fraction_width: F = 0`, baseline `D = (Z, +)`, `Q = [0, 15]`, `rho = wrap`, `E = binary`,
`C = 8-bit`, observables `{V, M, O, L}`, `threads = 1`. `70` says in its own fifth finding that
the count is observable-relative and that adding `56`'s raw-pattern observables splits encodings
further, so **the four is a fact about that observable set** and the row carries no trace of it.

### N19. `one_container_hosts_many_systems_so_the_canon_types_the_system`

**Derived.** `65`'s candidate 4, and the row's `says` is close to verbatim. `65` section 1 gives the
construction: one eight-bit container under one identity map is simultaneously a numeral of Z/256,
a window on Z, a bounded chain, an element of GF(2)^8, and a mask in a Boolean lattice.

**Comparison.** Converged, and **three separate files refused to be counted as a second instance on
the broad reading**, which is the most disciplined thing in this unit. `65` section 5 calls it *"ONE
EXPERT plus an independent posing of the question"* because `66` raised the scope question and
declined to answer. `71` section 7 supplies a different argument (a narrow concept cannot type the
index-1 crossing, which is a real consumer act) and says *"Still one instance on the conclusion. A
second cold derivation is what Q21 needs and this is not it."* `73` sections 4 and 5 measured the
discriminator empty and says *"I am not a second instance and will not be counted as one."*

**Outcome: converged, standing unchanged at one expert, and it should stay there.** I read `65`
before forming a view and am the fourth reader in that queue. Provenance gains `65:503` and
`67:488`, the second because `67` attached a vocabulary caution the row's note already quotes and
could not reach.

**Region.** `normative`. The arithmetic cells behind it are measured elsewhere at `W = 4`.

### N20. `the_concept_is_closed_and_the_inventory_is_open`

**The only `two_experts` row in the block, and the one place my second read changes something.**

**Derived.** `65` section 7 and `66`'s section on open or closed. Both are blind cold derivations,
committed in that order, neither having read the other.

**Comparison: converged narrower, and the narrowing is not small.** They closed different things.

- **`65` closes the membership concept.** *"The canon defines once what a number system is
  (section 2), what a representation is (section 3), and what admission requires."* Its admission
  contract is carrier, operation family with totality statements, law inventory with bounded
  failures, correctness relation. Nothing about arvo enters.
- **`66` closes the hosting contract.** *"the concrete set of number systems **arvo will ever
  host** cannot be closed... What can and should be closed is not the set of systems, but the
  **contract** a new system must satisfy **to plug into the pipeline described above**"*, and its
  contract is a decidable self-contained validity predicate over a const-sized container plus a
  derivation function from (shape, strategy) to (container, representation). `66` derives it from
  `#![no_std]` and const sizing, and says in the same file that unbounded exact rationals are
  excluded **by that constraint alone**.

**`73` then proved those are two different questions.** N10, in this same block: membership and
hosting have different answers and an inhabited two-by-two, and *"Exact rationals with unbounded
denominators fail it while remaining a perfectly good number system."*

So under `expert-dispatch-defends-the-canon`'s rule that **two agreeing instances agree about the
intersection of their dimensions, intersected over values rather than over dimension names**, the
dimension here is *which contract is closed* and the values are disjoint. What survives at two
experts is the shape: **the inventory of instances is open, and what is closed is a contract rather
than a list.** The row's own sentence, *"The canon defines once what a number system is and what
admission requires"*, is `65`'s half and `66` does not support it.

**Outcome: converged narrower. The row states at `two_experts` a clause only one of its two
instances reached.** I have not edited it: a predicate is never widened in place and by the same
reasoning it is not narrowed in place, so this goes to consolidation. **It should not be promoted
in its current wording.** Provenance gains `65:519` and `66:245`, which is what lets the next
reader check this in two file opens rather than in the day it took me.

**Region.** `normative`, correct for the surviving half.

### N21. `roles_derive_representations_and_a_realisation_variant_computes_nothing_new`

**Derived.** `65` proposes storage, compute and interchange and carries chain extent open. `71`
section 6 argues three of the four differ at indices 4 and 5 only, so they preserve the value-level
operation and cannot change what anything computes, while chain extent changes the number of
adaptation points and therefore changes the function. `73` section 7 supports it with its own
derivation.

**Comparison: converged narrower, and the row inherits a narrowing from `74`.** `72` section 4 says
this is the one place `71` over-reads, with two specific defects: the register already carries a
live reading under which the compute role differs at index 2 (`OPTIONS.md:991-994`, 64 of 251
extents mapping to two carriers), and `65`'s own text for the compute role says *"a native-width
two's complement **or a redundant intermediate**"*, which is a form holding what the format cannot.
And `65` does not file chain extent as a fourth role separate from compute, so the clean two-kind
split runs through the middle of one of `65`'s three roles rather than between them.

**Then `72` states the criterion's real shape**: `71` poses one question, *may a role change the
selected reduction*, and **it is three**: may a role change the ambient domain, the representable
set, or the selected reduction.

**The row's `gap` names one of the three.** It reads *"whether any role may widen the representable
set before the encoding"*, which is `72`'s middle question. `74`'s N21 narrowed it the same way and
the row inherited it. `72` also says the role set is **mixed on evidence the panel already holds**
rather than open on a question nobody has posed, which is stronger than the row's framing.

**Outcome: converged narrower, with a stated gap that is one third of the real gap.** Standing
unchanged. Provenance gains `65:185`, `71:405`, `72:232`, `73:415`. **The gap should be widened to
`72`'s three questions in consolidation.**

### N23. `every_dispute_in_the_number_system_topic_was_a_dispute_about_an_address`

**Derived.** Nothing to derive. It is `74`'s own composition of N2, N14, N15 and N16, marked as such
by its author and offered for attack.

**Comparison.** The row's note is correct and unusually honest: most suspect tier, a composition
rather than a claim, and it falls with any of the four.

**Outcome: correct as filed, and correctly still flagged by the consolidation arm**, because it
does rest on a compression. Provenance unchanged; `74` is genuinely its source.

### The refusal: `an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing`

**Derived.** `73` sections 2 and 3, `p1` and `p1c`. This is the best-constructed evidence in the
block and the probe row behind it is exemplary: `probe::the_collapsed_declaration_cannot_be_made_to_fail`
carries `lives` pointing at all four artifacts and a `control` field naming the positive arm, in
terms worth quoting: *"a row that cannot fail is not a measurement, and the way to show it cannot
fail is to corrupt what it is nominally about and watch it not notice."*

**Comparison.** Converged. The mutation set breaks the honest verdicts in 3 of 4 cases and reaches
the collapsed verdict in 0 of 4, which is exactly what makes the zeros mean something.

**Outcome: converged, standing unchanged.** Provenance gains `73:119` and `73:184`.

**Region.** Already stated in full on the row, and it is the only proposal in the block carrying
one. Note that `threads` and `target_features` are absent, so under I13 it holds in no situation
where those exist. The file header names that as a known artefact of who was writing.

### The two rows that were already correct

`the_construction_pipelines_trusted_base_is_nine_named_items_not_all_checked` cites `68:226`
directly and never went through the consolidation. Its enumeration of nine trust items with each
one's checked-or-not status stated alongside it is the shape I would want every soundness claim in
this canon to take, and its `gap` naming items 4 and 6 as currently unaddressed rather than merely
unchecked is the honest form.

`the_change_test_survives_only_as_a_crossing_and_compatibility_classifier_not_as_a_boundary_definition`
cites `75:58` and `70:284`. It also carries a placeholder predicate with a note explaining that the
claim genuinely ranges over no numeric dimension, which is the right way to handle a schema
requirement that does not fit.

**Both are one expert, both are correct as filed, and neither needed anything from me.** Keeping
something is a result.

## 6. Two cross-row findings the sentence-by-sentence port could not see

**First: N10 refutes N20's standing.** They sit in the same topic, seven rows apart. N10 establishes
that membership and hosting are different questions with different answers; N20 claims two experts
for a closure clause whose two instances closed one each. A port working sentence by sentence,
which is what `74`'s section 4 is and what the registry rows are, cannot see a relation between two
sentences. **The only reader positioned to see it is one holding both, which is the second read.**

**Second: the block's sentence kinds do not agree with its own header.** `proposal.toml`'s header
states the test in its own words: *"A claim that could be measured false is not `normative` however
definitional its grammar, and it carries the region it was established in or it is not here at
all."* Applied to this block, at least six rows fail it: N4, N7, N8, N11, N15 and the measured half
of N6 all rest on instruments that returned counts and could have returned others.

**The consequence is a silent widening, and it is the exact widening I13 forbids.** A `measured` row
at `W = 4, F = 0` holds there and nowhere else. Filing the same claim `normative` gives it no
region, so it reads as holding everywhere, and it gets there **without anybody touching the
predicate the widening would have been visible in**. `every-finding-carries-its-predicate` guards
the predicate field; nothing guards the field that decides whether a predicate is required at all.

I built the arm that catches the unarguable half of this, a row imposed and pointing at an
instrument at once, and it finds zero. **The rest is a judgement per row and I have made it above,
one at a time, with the instrument's parameters attached.** It is not mechanical and I do not think
it can be made mechanical, which is why it is stated as a list rather than as a check.

## 7. What I could not do

**I could not raise a single standing, and I want to be exact about why, because "second read" as a
dispatch shape presumes something this topic cannot supply.**

To evaluate a row I had to read the file that established it. Having read it, my agreement is a
read. This is not a scruple I invented: `70`, `71` and `73` each declare it about themselves in
their own openings, and `71` and `73` each refuse to be counted on the broad-reading question in
those terms while making real contributions to it. **The standard is the unit's own and I am held
to it.**

**So a second read of a topic can raise standing only in two ways.** It can find a pre-existing
independent arrival the consolidation missed, which is a fact about the corpus rather than about
the reader; that is what I looked for and it exists for N1 as a third *instrument*, not a third
arrival. Or the second reader can be dispatched **before** reading the sources, derive cold from
the premises, and then compare. **That dispatch is available for this topic and was not what I was
sent to do.** For N19 specifically, `71` and `73` both name it as the thing the question needs: a
second cold derivation on whether the concept reaches past magnitude. Neither of them could be it,
and neither can I.

**The other thing I could not settle** is whether the six normative-but-measured rows should be
refiled. Refiling them means giving each a predicate, and a predicate is a claim; writing one from
a probe's parameters is transcription, but deciding which parameters were load-bearing is not. I
have written each region above as I read it and I would not put any of them into a row without the
author of the instrument or a third reader agreeing.

## 8. What is owed, in order

1. **The 29 rows asserting multi-expert standing on one citation.** Under `213` these are exactly
   the promotion-eligible set and none of them is checkable. Each needs the same additive repair I
   did here, which means reading its topic's consolidation and recovering the citations. This is
   the single highest-value remaining pass and it is mechanical to find and slow to do.
2. **N20's wording**, before it is promoted. It states at two experts a clause one expert reached.
3. **The six sentence-kind refilings**, with the region each instrument was run at.
4. **N21's gap widened** to `72`'s three questions.
5. **N5's note corrected** to say the source licensed the by-construction reading, because the row
   currently reads weaker than its evidence.
6. **The catalogue sentence at `panel_050_099.toml:67`**, which records the opposite of what the
   ruling says and is the reason the class went unrepaired.

**And one for whoever writes the next second-read brief**: the row slugs are the claims, so a blind
second read of a registry row is not possible as the schema stands. Either the second reader is
dispatched at the sources with the registry withheld, or the dispatch asks for the region and the
provenance rather than for agreement. **This one should have asked for the second.**

## 9. Bounds

Everything above rests on reading, on one mutation with its control, and on two arms with thirteen
planted-input tests. **I built no bench and nothing here is priced.** I did not re-run any member's
probe: every count I quote is transcribed from the file that reported it, and where two files
report the same number I say which I took it from. I did not open `67_probes/`, `70_probes/`,
`71_probes/`, `72_probes/` or `73_probes/` at the source, so every measurement above inherits its
reporting file's accuracy; `71` did reproduce all eight of `67`'s crossing numbers independently
and `72` re-ran `71`'s `p1`, which is why I trust those two chains more than the rest.

**Nothing here settles anything.** The repair is provenance, which changes what a reader can check
and no claim's content. The corrections in sections 5, 6 and 8 are for consolidation.
