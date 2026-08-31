# 222. The numeric fundamentals: what the ratified canon already decides, and what an instrument had to decide

Seat 222. One half of a blind pair on the open questions of `the_number_system`,
`the_format`, `rounding`, `overflow_policy`, `the_primitive` and
`the_container_premise`.

The short version is that the canon decides considerably more of this than the
question rows suggest, and that most of what it does not decide is decided by a
computation somebody could have run in an afternoon. Of the thirty-seven rows I
was sent at, four are already closed at the governing tier and render as open
because nothing follows the edge that closes them; three more are settled by an
instrument I built here; eight are malformed in a way the canon itself names, so
the useful act is retiring them rather than answering them; and one is not
answerable from the canon at all, because the sentence whose reading it
disambiguates was never admitted to the canon. The rest are derivations of
varying strength and each says which it is.

I want to be exact about what my agreement is worth before any of it. Where a
`one_expert` proposal already answers a question, I read that proposal before I
derived anything, because `mock/registry/*.toml` is the canon and the brief made
it required reading. So my agreeing with such a row is confirmation and not
corroboration, and it does not move it to two experts. I mark those as
confirmations and say so at each one. What I offer as independent are the
entailments from ratified text, which are applications rather than derivations
and do not need a second instance, and the four probes, which are instruments
nobody had built.

## The two gates

**The canon gate passes.** I checked the assigned work against
`mock/registry/ruling.toml` and specifically against
`ruling::the_panel_finishes_the_canon_without_him` (`ruling.toml:1626`), which
says every remaining canon question is the panel's, that a question filed as op's
is now derived from what he has already said, and that nothing is parked awaiting
him. Deriving answers to open question rows is exactly what that licenses. Two
things in the brief's own framing I did have to test and both held: the registry
is the canon by `canon_paths` in `mockspace.toml:32`, and the questions were
ported without answers by that file's own stated policy, which
`question.toml:17` records in the header.

**The test gate passes and the suite is real.** `cargo test --manifest-path
mock/checks/Cargo.toml` runs 152 tests across nineteen files and all pass. I read
the bodies rather than the names in the surface I touch, which is
`every_predicate_names_a_declared_axis.rs` and
`a_settled_question_does_not_sit_in_the_queue.rs`. Both are genuine: every arm
runs against a planted input as well as against the committed canon, the planted
inputs are the failures the arm exists to catch, and the files say in their own
prose why an arm that has only seen a clean canon establishes nothing. The
declaration class the workspace's test gate warns about is absent here: no test
asserts a constant against the literal its own definition sets, and the predicate
checker's assertions reach through to the parsed rows rather than to another
declaration. I found one gap and it is already recorded rather than hidden:
`predicate.rs:19` states outright that the values side of a predicate is not
checked and gives the reason, and
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` names two live
violations that the slug-side-only arm cannot see. That is a known hole with a
stated reason, which is not the same thing as a decorative suite.

## Blindness, and what I read

I read, in this order and nothing else before committing this file: the four
`.claude/rules` directories generated into my worktree, `mockspace.toml`, every
file under `mock/registry/`, `mock/checks/src/` and `mock/checks/tests/`,
`mock/Cargo.toml`, the probe sources and outputs under
`mock/research/202608072330_the-numeral-canon-panel/*_probes/` that I name below,
and one `awk` range over `OPTIONS.md` covering the heading of Q57 alone.

I did not read any numbered member file, I did not run `git log`, and I did not
look for the parallel seat's branch. The one place my blindness came close to
the edge is that `OPTIONS.md` range, which is a living ledger rather than a
member deliverable; I took it because Q57's option set in the registry is
uninterpretable without it, and it turned out to be uninterpretable with it too,
which is the finding in section 9.

Two things the coordinator flagged as having landed on the trunk while I worked
turn out to have been in my tree already, so neither changes anything here.
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` is at
`rung = "ratified"`, `ratified_by = "experts"` at `ruling.toml:1544` in the tree I
cut, and I wrote every predicate below under it. The three questions closed
through `ruling::the_format_spine_is_canon` I found by walking the same two hops
myself, before the message arrived; that walk is section 3.1 and it is the first
thing I did after counting the rows.

## The count

Thirty-seven, matching the brief exactly: `the_number_system` fifteen,
`the_format` ten, `rounding` eight, `overflow_policy` two, `the_primitive` one,
`the_container_premise` one. Measured by walking `[[question]]` blocks in
`mock/registry/question.toml` and counting those with no `answered` key.

Three further rows carry no `answered` field and no `topic` at all
(`should_phase_collide_across_two_vocabularies`,
`the_cross_repo_strategy_name_collision`,
`is_the_non_terminating_contention_crate_a_defect`). They are outside my
assignment and I did not touch them, but a topicless question is invisible to any
roster built by topic, which is worth somebody's attention.

**What "no `answered` field" means is weaker than it reads**, and this matters for
how the thirty-seven should be understood. `what_then_validate_requires` (Q1) has
no `answered` field and is nonetheless closed: `ruling::validate_means_all_three_readings`
(`ruling.toml:1185`) carries op's verbatim answer, names the row in its `answers`
edge, and the question's own `note` says "Recorded as answered at `28` batch one".
So the field's absence marks rows the schema's newer answer mechanism has not
reached rather than rows nobody has answered. Reading the count as thirty-seven
open questions overstates it, and by more than one.

## 3. Four structural findings, which change what several of the questions are

These come first because three of them move questions out of my answer list
before any derivation starts, and the fourth changes the shape of eight more.

### 3.1 Three questions are closed at the governing tier and the queue cannot see it

`ruling::the_format_spine_is_canon` (`ruling.toml:1436`) is ratified, marked
`ratified_by = "both"`, and carries a `ratifies` list of four propositions. Three
of those four carry `answers` edges, and every one of the three lands in my
assignment:

| ratified proposition | line | closes |
|---|---|---|
| `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | `proposal.toml:65` | `adaptation_in_identity_or_realisation` (Q18) |
| `membership_of_the_representable_set_is_one_affine_predicate` | `proposal.toml:82` | `which_width_coordinates_a_consumer_writes` (Q2) |
| `the_concept_is_closed_and_the_inventory_is_open` | `proposal.toml:576` | `is_the_number_system_inventory_open` (Q20) |

**The mechanism that hides them is one line of the checker.**
`checks/tests/a_settled_question_does_not_sit_in_the_queue.rs:80` defines
`answered_by`, and its walk at line 82 is `for ruling in reg.of("ruling")`. It
reads `ruling.answers` and nothing else. A question closed by a ratified
*proposition* rather than by a ruling is therefore invisible to the arm whose
whole purpose is finding settled questions still in the queue, and the file's own
module doc says why that matters: "a question op has already answered renders as
open, reads as open, and counts as open in every roster built by reading
`decider`". It is right about the failure and wrong about the extent, because it
guards one of the two edges that produce it.

The fix is two lines in `answered_by`: walk `reg.of("proposal")` as well, and
report only where the answering proposition appears in some ruling's `ratifies`.
That last condition is what keeps a `one_expert` proposal from closing anything,
which it must not.

**A fourth row is in the same class and I am less sure of it**, so it is
separate. `staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`
(`proposal.toml:1579`) answers `does_narrowing_compose` at `standing =
"three_or_more"` with an exact predicate and a named instrument. That is not a
ratified proposition, so it does not close the question the way the three above
do; what it does is make the question's binary option set false, which is section
6.1.

### 3.2 A live self-contradiction in the registry, about the general form of a rounding law

`proposal.toml:1603` names the instrument
`07_probes/p4_composition_and_forced_adjoint.py` and line 1604 says of it, in
the same sentence, "and no `probe` row names it, so the general form is not
writable and what is written is the one instance whose instrument is
registered". The preceding line says, in bold, "**The general rule behind it is
in the corpus and cannot be written here.**"

`probe.toml:982` is a row with `id =
"narrowing_composes_where_the_modes_direction_switches_at_coarser_grid_points"`,
whose `lives` list at line 987 is exactly
`07_probes/p4_composition_and_forced_adjoint.py`, and whose `establishes` states
the general rule verbatim: narrowing composes exactly when the mode's direction
switches only at coarser-grid points. It carries `standing = "sound"` and a
control whose note is the best statement of method in this corpus.

So the blocker is gone and the sentence recording it is still there. The general
form is writable now, and section 6.1 writes it. I did not check which row is
older, because I may not run `git log` and because it does not matter: both are
in the committed canon at the same commit and they disagree.

### 3.3 One `answers` edge reaches past its own `says`

`membership_of_the_representable_set_is_one_affine_predicate` closes Q2, "which
pair of width coordinates does a consumer write at the surface?" Its `says`
establishes that membership is one affine predicate over a slot function, a
quantum and a phase, of which integers, fixed point, scaled integers and floats
are points. That is Q2's fourth option's *reading*, which the row's own question
describes as "the numeral is one object, a grid cut down to a reach, and the
width pair is what that object is called". It settles that the width pair is a
projection of one definition rather than a definition.

It does not say which projection a consumer writes, which is the question's first
three options and its literal text. So Q2 is closed at the definitional half and
open at the surface half, and the residue is a strictly narrower question than
the row as written. That is the same failure the ratification gate exists to
catch, in a field the gate does not read: the `promotion` note on
`the_additive_and_absorption_verdicts_are_canon` says the gate is that "neither
prose reaches past its predicate", and an `answers` edge is prose about scope
that nothing audits.

### 3.4 Eight of the thirty-seven ask which single policy governs a category

The canon names this shape and rejects it.
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it`
(`ruling.toml:909`) carries op's words: "we don't need to settle for one
universal solution, it's the anti-pattern I've already named. Case by case ...
Take the win where it applies, gate it out from where it does not. No single
one-fits-all solutions, it's impossible." The ratified
`ruling::the_work_is_predicated_arms_composed` (`ruling.toml:847`) and
`ruling::arms_over_regions_are_the_fundamental_heart` (`ruling.toml:1459`) say
the same thing positively.

The eight, and what each is really asking once the shape is removed:

| row | asks for one policy over | the answer the canon already gives |
|---|---|---|
| `does_warm_wrap_or_clamp` | a whole strategy's overflow behaviour | the mode is declared per site; section 5.1 |
| `do_arvos_consumers_want_value_keying_or_position_keying` | a whole consumer base | both, each gated; its own `bound` says so |
| `does_narrowing_compose` | a whole design | a predicate on the rounding mode; section 6.1 |
| `is_the_derived_numeral_required_to_be_tightest` | every derived numeral | soundness in the canon, tightness as arms; 7.3 |
| `is_the_cross_kind_join_closed_or_priced` | the whole shape space | priced and unclosed, arms per region; 7.4 |
| `does_the_position_keyed_dither_arm_ship` | whether an arm exists at all | it ships; 7.6 |
| `does_a_consumer_supplied_seed_surface_exist` | whether a surface exists at all | it exists; 7.7 |
| `whose_reduction_governs_a_lossy_crossing` | every lossy crossing | the target's, by the ratified factoring; 6.4 |

Three of those eight already say so in their own `bound` fields, which were
written after op returned them. That is the discipline working. What it has not
done is change the `options` arrays, so each still reads as a live fork to anyone
who reads the row rather than the bound, and the option arrays are what a
consolidation compresses. Rewriting an option set is not a port's call and is not
mine either, so I am naming them rather than editing them.
