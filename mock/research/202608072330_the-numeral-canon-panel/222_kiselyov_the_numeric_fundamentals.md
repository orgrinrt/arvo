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
