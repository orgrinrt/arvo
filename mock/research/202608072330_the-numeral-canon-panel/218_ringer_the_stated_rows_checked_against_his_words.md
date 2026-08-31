# 218. Ringer: the fifty-two stated rows, checked against his words

I was sent to answer one question, fifty-two times: is each `stated` row's `says` a faithful reading of the
`quote` it sits on. That is a compression-fidelity question, and the reason it needs somebody other than the
row's author is that the belief the restatement entails is what produced the restatement. I am that somebody.

The verdicts are below and they are the smaller half of what I found. The larger half is that fidelity to the
quote turns out not to be the thing standing between several of these rows and the canon. Three of them have
already been put to op for ratification and did not get it, one of them because he said in as many words that
he could not bless it. Two more are superseded by rows he has since ratified. None of that shows up in a
says-against-quote check, and none of it shows up in the tool a coordinator would work the list from, which
filters on one field and knows nothing else.

So this file has two halves that have to be read together. The verdicts answer what I was asked. The
promotion blockers answer what the verdicts would otherwise get wrong, and they are the part I would read
first if I were the one holding the gate.

## Both gates, before the work

**The canon gate passed.** I checked against `mock/registry/*.toml`, which `mockspace.toml:31` declares as
`canon_paths`, reading the `rung = "ratified"` rows in `mock/registry/ruling.toml` as the settled part and
`mock/registry/proposal.toml` as explicitly not canon. The assigned work is a fidelity check over a namespace
the canon owns, it changes no `rung`, and nothing in it conflicts with a ratified row. Two ratified rows
govern how I did it: `an_ack_is_not_a_ratification` and `the_intent_is_not_every_clause_of_the_quotation`,
the second of which is itself one of the fifty-two and which I read first, as instructed.

**The test gate passed, and I read the bodies rather than the count.** `cargo test -p arvo-checks
--no-fail-fast` from `mock/`: 143 passed, 0 failed, 0 ignored, 0 warnings, exit 0. Summed off the `test
result:` lines rather than by counting `ok`, per the brief. A green suite is the weakest signal in the room,
so I audited the arms in the surface I touch, which is the `ruling` namespace and its `quote` field:

- `what_one_field_obliges_another_to_carry.rs:457`,
  `the_rulings_with_no_verbatim_are_the_ones_the_corpus_has_no_words_for`. Pins six rows by name and asserts
  the found set equals them exactly. Real: it fails on a seventh, and it fails on a sixth being fixed without
  the list being updated. Its planted-registry sibling at line 479 shows one reported and one not.
- `shape.rs:35`, `rulings_with_no_verbatim`, filters `ratified_by = "experts"` out before checking. I went
  looking for a hole here, because 79 of 86 rows carry a `quote` and the pinned list has six, which leaves a
  seventh unaccounted for. There is no hole. The seventh is
  `the_additive_and_absorption_verdicts_are_canon`, ratified by the experts rather than by op, and the filter
  is documented at `shape.rs:37-48` with the reasoning. **I am recording that I checked and found nothing,
  because otherwise the next reader spends the same twenty minutes.**
- `a_proposal_is_not_asked_for_a_verbatim` at line 507 covers the namespace boundary. Not tautological, not
  sampled, and its assertion is over a planted registry whose answer is known.

**One defect in that surface, and it is in prose rather than in an assertion.** The doc comment at
`what_one_field_obliges_another_to_carry.rs:441-450` splits the six pinned rows into "the bad kind" and "the
tolerable kind", and says of the first four that "their only record is an agent's sentence reporting the
outcome" and "the choice he was choosing among is gone, so nobody can reconstruct what he decided, only that
he decided". **That is false for all four.** `the_canon_is_written_once_at_the_end` and
`the_branch_waits_for_the_canon` both carry their full option sets in `note`, and
`87_op_the_canon_is_written_once_at_the_end.md:14` and `:52` carry them again at the source under the heading
"He was choosing among". `a_proof_and_a_bounded_range_get_markers_the_notation_lacked` and
`the_ambiguous_rounding_word_is_retired_for_six_explicit_names` both carry their options on `ratification`.
The distinction the comment draws between the first four and the last two does not exist in the registry it
reads. The test is correct; its explanation of what it is testing is wrong, and it is the kind of wrong that
gets quoted.

## The standard I applied, and where I refused to apply it mechanically

`the_intent_is_not_every_clause_of_the_quotation` is the governing row and it is also one of the fifty-two.
Op's words in it:

> It doesn't matter. The important part is whatever came before "for the most part, they probably agree...".
> That's just filler noise I mused on the spot. What it is speculating on ("probably", "perhaps") is the
> important part, not the small talk speculation itself.

Read as a word filter, that retires every clause containing "I think" or "probably". **I did not read it that
way, and I want the reason on the record because several verdicts turn on it.** What he objected to was
having a musing handed back to him as something he owed an answer to. The musing in that case was an
empirical prediction about whether the strategies agree in practice, which is not his to settle and which he
disowned on exactly that ground.

An "I think" attached to a call that is his to make is a different object. He answers questions that way
routinely and expects the answer to bind: at `panel::202608072330_the-numeral-canon-panel::206`, asked where
the canon/design line falls, he answered "I think, if it's a rule that can't be avoided or designed away, a
law, or something that constraints the work of the design, it's canon", and that is a ruling, not a musing.

**So the test I used is not the word. It is whether the hedged clause decides something he owns.** Where it
does, the hedge is politeness and the `says` may state it. Where the hedged clause is a factual claim about a
file, a measurement or the world, it stays in `note` and the `says` may not assert it. That distinction is
what fires on `the_container_derivation_needs_fresh_eyes` and what does not fire on
`the_prior_strategy_split_was_well_enough_defined`, and I would rather be argued with about the line than
have applied a grep.

## What I measured, and the two instruments

Both probes are committed at `218_probes/` with their sources, their output and their controls. Neither
number below counts until its control has been shown failing.

**The dispatch's own census is exact.** 86 rows, 52 at `stated`, 50 of those carrying a `quote` and 2 not,
and the kind split 33 process, 10 ruling, 6 intent, 2 deferral, 1 refusal. Every figure reproduced.
`p1_the_stated_rows_and_their_quotes.rs`, arm A, controlled against a planted registry whose answer is known
by construction. **That control earned its keep immediately**: the first version of the parser closed a row
only on the namespace header, so a following `[[proposal]]` merged its fields into the last ruling and
flipped that row's `rung`. The row count stayed correct throughout, so counting alone would never have found
it.

**Every quote is verbatim against the file its provenance names. Seventy-nine checked, zero missed.** Arm B
of the same probe, whitespace normalised and markdown blockquote markers stripped, with the control mangling
one currently-matching row by a single clause and requiring the miss count to move by exactly one. It moved
0 to 1. **This arm also spent one run being wrong in the most flattering direction available**: leaving the
`> ` markers in reported 70 misses of 79, which reads as a corpus full of fabricated quotes and was entirely
the flattener. A number that large should be disbelieved before it is reported, and the control is what
turned it into a bug instead of a finding.

That result is worth stating plainly, because the brief named the opposite as a hazard that had already
happened here once. **No quote in this namespace has dropped an attributing clause or gained punctuation
against its in-repo source.** I checked a further eight against a second, independent carrier, the verbatim
captures in the workspace repository's `.data/op-responses/`, which were written by a different process from
a different transcript: `20260830_2320_arvo-panel-steering-opening.md` for the five rows drawn from the
session opening, `202608311209_arvo-canon-panel-audit.md` for
`a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon`, and
`202608311355_ruling-batch-arvo.md` and `202608311557_arvo-ruling-ratification.md` for the ratification
rounds. Exact in every case.

**Two further arms, in `p2_how_far_a_says_travels_from_its_quote.rs`, aimed at classes I first found by
reading and then had no way to bound.** Arm C sweeps `supersedes` and `corrects` edges running from an
unratified row at a ratified one. Arm D counts the content words in a `says` that appear nowhere in its
`quote`, and splits them by whether the source file carries them, which is the signature of a quote clipped
short of the sentence the `says` leans on. Arm D's control needed a fixture file, and **the first fixture
defeated itself in a way I am keeping in the record**: it explained which word it did not contain, thereby
containing it. A file cannot both lack a word and name it. The fixture now says so without spelling the
token.

Arm D ranked seven of my eight overreach findings inside its top twelve, which is the corroboration I wanted
and is also the honest bound on it: **the two it ranks low are the two whose overreach is a hardening of the
same words rather than an import of new ones**, which that arm cannot see by construction.

## The verdicts

**41 faithful, 8 overreaching, 1 narrow, 2 unsupported.**

The non-faithful eleven follow. Everything not named here I read and judged faithful.

### Overreaching

**`ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`** (ruling). The `says` opens with "Data
entering from outside the program is the consumer's boundary, not arvo's", and the quote contains no such
sentence. What he said was that everything ends up in a C ABI eventually, that this is not a problem, and
that "This is something the writer handles by defining the apis with the arvo shapes and generics". That
supports a claim about who writes the API. The `says` turns it into a claim about where arvo's
responsibility ends, which is a boundary rule, and the row then hangs an `obligation` edge off it. Drop the
first sentence and the row is faithful. **Do not promote it on that repair alone; see the blockers.**

**`the_imitation_is_ergonomics_not_an_arithmetic_boundary`** (ruling). Quote in full: "Neither, it's
ergonomics". The `says` closes with "Where the arithmetic boundaries land is answered by the width and the
overflow policy instead", which is an answer he did not give. He said the imitation does not decide it. The
panel then decided what does. The gloss on "ergonomics" as "unsurprising and does the expected thing" I
accept, because he independently reached the same word later ("for the most part it should be *intuitive* to
users", `202608311355_ruling-batch-arvo.md`), but the final sentence belongs in `instead` or `note`. Arm D
ranks this first of fifty-two, 18 of 19 words absent from the quote and present in the source.

**`what_is_ratifiable_is_the_intent_not_the_concretes`** (process). Quote: "we don't ratify these as
absolutes, rather, *intent* as stated by me here and prior". The `says` renders that as "The per-strategy
conclusions drawn from an intent are not ratifiable and were never his". Two moves. "Not ratified as
absolutes" becomes "not ratifiable", and "were never his" is a claim about authorship the quote does not
make. **The first one now contradicts a ratified row**: under
`two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`, concretes are exactly what
gets ratified, by the experts. Corrected: "What may be ratified is the intent as he stated it, here and
prior. The per-strategy concretes are not ratified as absolutes."

**`a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon`** (process). The `says` carries a
two-limb test, "constrains the work and is needed to know" plus "is a law, or cannot be avoided or designed
away". The `quote` carries only the first limb. The second limb is real and is his, from his fourth answer
in the same sitting, and I have it verbatim in the independent capture: "I think, if it's a rule that can't
be avoided or designed away, a law, or something that constraints the work of the design, it's canon"
(`202608311209_arvo-canon-panel-audit.md`). **So this is a filing defect rather than an invention, and the
repair is to extend the `quote` by his second statement rather than to shorten the `says`.** As it stands, a
reader diffing the row against its own evidence finds half the test unsupported. The third `says` sentence,
that the kind of statement and the tier do not decide the question, is the panel's inference from him
sidestepping the three options he was given, and reads to me as sound but is not his words.

**`steer_aggressively_to_convergence_without_rushing_it`** (process). Same class. The `says` ends with "The
work has run for months, the theoretical side can be argued in every direction without end, and settling
somewhere inside the constraints already formed is what is wanted", and the `quote` stops one sentence
earlier. His next sentence, in the independent capture, is "But it's been literal months ongoing by now, and
there seems to be no end to how much the theoretical side can be argued on all directions and we just want
to settle somewhere within the constraints we already kind of have formed"
(`20260830_2320_arvo-panel-steering-opening.md`). **The clause is operative rather than decorative: it is
what licenses closing something.** Extend the quote by that sentence and the row is faithful. Note that
`work_autonomously_and_batch_the_questions_for_him` has a clipped quote of the same shape and I did **not**
fire on it, because there the missing sentence is a reason and the `says`'s operative content is entirely
inside the quote.

**`the_container_derivation_needs_fresh_eyes`** (process). The `says` asserts "The latest attempt already had
the contracts and typestate working fully with no enumeration and no forbidden features, which establishes it
is doable". He said "If I remember correctly", and "And pretty sure that did not contain any enumeration".
**This is the hedge case where the hedge governs**, because the hedged clause is a factual claim about the
contents of a file, which is not his to settle by saying so, and he flagged the recall as approximate twice
in four sentences. His call is the last clause, "I said fresh eyes on it, that holds", and that is firm and
belongs in the `says` exactly as written. The `note` already records the caveat honestly; the `says`
overrides it. Corrected: keep the standing call firm, attribute the properties to his recollection, and say
whoever takes this verifies them.

**`the_option_set_is_not_a_boundary`** (process). "A shape nobody has written down is admissible **and is
what he is asking for**". He said "don't even restrict the panel to these three. Free reign to converge by
theory and logic to the best one that serves all other parts of arvo best". That licenses a fourth shape; it
does not ask for one. **The difference matters in this corpus specifically**, because
`the_prior_strategy_split_was_well_enough_defined` is the counterweight against churning a decomposition
that is already adequate, and a canon sentence saying he wants unwritten shapes cuts against it. Drop four
words.

**`the_question_was_already_answered_by_an_intent`** (process). The quote is two words, "Already answered."
The `says` builds a general pre-ask filter out of them: "A question whose answer is implied by an intent he
has already stated is not a question for him." The `note` says outright that this is "the coordinator's
wording rather than his". I agree with the rule and it should be a workspace rule; it should not be a canon
sentence carrying `ratified_by = "op"`, because two words do not establish it. The first `says` sentence,
which reports what he said about that one question, is faithful and is all the row can carry. **Its
`supersedes` edge is separately wrong; see the blockers.**

### Narrow

**`the_trait_contract_structure_is_a_primary_paradigm`** (intent). He said "The trait contract based
structure is a primary paradigm **we** uphold in future too", in a message about the stack, naming
hilavitkutin as the live instance. The `says` renders "we" as "arvo". This is the mildest finding in the set
and the narrowing is defensible, since a row in arvo's registry cannot bind hilavitkutin anyway. But it is a
loss: what he stated was a stack-wide commitment, and the canon sentence derived from this row will say arvo
only. Either keep his word and let the scope be read off the registry it sits in, or say in `note` that the
narrowing is the port's choice.

### Unsupported

**`the_canon_is_written_once_at_the_end`** and **`the_branch_waits_for_the_canon`** (both process). No quote
exists, so no reading of one can be faithful. What a reader **can** conclude is more than the phrase "he took
the third" suggests, and I want that on the record against the doc comment I corrected above: both rows carry
their complete option sets in `note`, and `87_op_the_canon_is_written_once_at_the_end.md` carries them again
at the source, written out as "He was choosing among: (1) ... (2) ... (3) ...". So the decision is
reconstructible and the alternatives are visible. What a reader **cannot** do is check anybody's reading of
his words against his words, because there are none, and neither row may carry `ratified_by = "op"` on the
strength of a quotation.

`the_branch_waits_for_the_canon` is otherwise sound and nothing has overtaken it.
`the_canon_is_written_once_at_the_end` has been overtaken in part; see below.

## The promotion blockers, which the verdicts do not carry

**Five rows must not be promoted, and only one of them is non-faithful.** A verdict of faithful means the
`says` reads the `quote` correctly. It does not mean the row's claim still stands, and for these five it does
not.

**`ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`. Op was asked to bless this and refused, in
terms.** In the second ratification round he was offered it as an option and answered:

> I need elaboration on the Ingest vs C ABI. That explanation does not explain what this is, so I can't
> bless it or comment on it in any manner

Recorded at `211_op_the_second_ratification_round.md:106`, and independently at
`.data/op-responses/202608311557_arvo-ruling-ratification.md`. Panel file `211` says at line 128 that a real
statement of this row is one of two things owed. **Promoting it would attach his authority to the exact
sentence he said he could not comment on.** The other owed item, which successor carries the strategy-shape
ratification, has a question row. This one has nothing: I grepped `question.toml` for `ingest` and for `C
ABI` and found no row, and there is no obligation row either. **It exists as one sentence in a panel file, and
the row itself carries no mark at all.**

**`a_strategy_is_a_preset_naming_a_point_with_a_weighting_in_it`. He ticked it and sent the stamp elsewhere.**

> Again, on the option 1, I think there's a better wording for that since that one you mention, so the
> ratification goes for the actual succeeding one rather than this, but the lineage is ratified and blessed
> thus

**The registry already handles this correctly**, and I want that said as loudly as the failures:
`question::which_successor_carries_the_strategy_shape_ratification` records it, names the three candidate
successors, and states that the row "stays at `stated` deliberately: stamping it would take the ratification
he explicitly sent past it". Nothing to fix on the row. The problem is that nothing carrying that hold is
visible from the row or from the tool.

**`warm_behaves_as_a_native_rust_primitive_would`. Superseded by a ratified row.** I first judged this one
overreaching and I was wrong, and I am recording the correction rather than quietly landing the right answer.
The `says` is near-verbatim faithful to its quote. What made me suspicious was that op declined to ratify it
when asked, saying intuition trumps Rust convention. He then sent that ratification to
`warms_objective_is_the_intuitive_best_choice`, which is `rung = "ratified"`, carries his redirection
verbatim, and carries `supersedes = ["warm_behaves_as_a_native_rust_primitive_would"]`. **The corpus had
already done the work; I had not looked far enough before forming a verdict.** Faithful, superseded, and a
promotion would stamp a historical statement rather than a live one.

**`the_run_ends_at_ratification_or_when_he_clears_it`. Overtaken by two ratified rows.** It says the goal is
not met until the full canon is ratified by him. He has since handed ratification to the experts and the
coordinator, and then ended his involvement:
`two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate` and
`the_panel_finishes_the_canon_without_him`, both ratified. Promoting this row reinstates a gate he removed.
Faithful to its quote and dead as a rule.

**`the_canon_is_written_once_at_the_end`, in part.** Its final clause, "and he ratifies that act", is
overtaken by the same two ratified rows. The rest of it, that nothing moves into the canon until every topic
is done and the canon is then written fresh as one act, is untouched and still governs.

**And one that is expired rather than superseded: `the_explore_mode_runs_for_the_first_hundred_stretches`.**
Its horizon was roughly the first hundred expert stretches. This file is 218. It is faithful, it is his, and
promoting it as a live rule would be promoting a spent one. It reads correctly as history.

## Arm C: four backward edges, two of which are defects

An edge running from an unratified row at a ratified one says an unstamped statement overrides a stamped one.
I found two by hand and built the sweep because two is exactly the count that makes you want one. It found
four. **Two are defects and two are legitimate lineage, and the instrument cannot tell them apart, which is
why I read all four rather than reporting the number.**

**Defects, both the same shape.** `the_question_was_already_answered_by_an_intent` carries `supersedes =
["the_d_numbered_decisions_are_dead"]`, and `the_explore_mode_runs_for_the_first_hundred_stretches` carries
`supersedes = ["nothing_settles_during_the_breadth_pass"]`. Both targets were ratified by op in the second
round, on 2026-08-31, **after** the statements that claim to supersede them, and on
`the_d_numbered_decisions_are_dead` he added "Most of these have been well established and blessed by me
earlier several times". A later ratification confirms a row; it does not get superseded by an earlier remark.
A reader traversing the graph is currently told that two rows op has stamped are dead. Both edges should go.

**Lineage, not defects.** `the_intent_is_not_every_clause_of_the_quotation` corrects
`the_strategies_weigh_measurements_differently`, and the correction is baked into the target's ratification:
he was asked whether the reading, that the intent is the first sentence only, was right, and answered "Yes,
ratify as written" (`202608311355_ruling-batch-arvo.md`). **That is the strongest promotion argument
available for any of the fifty-two**, because op endorsed this row's substance in the act of ratifying
another. `the_panic_bound_names_a_concern_not_a_marker` corrects `the_overflow_panic_is_permitted_and_bounded`,
whose `says` and `ratification` were both rewritten around his much longer later statement, including "even
on hot". The correcting row is consistent with it and fully absorbed by it. Promoting it adds nothing;
marking it absorbed would.

Zero dangling edges. Every `supersedes` and `corrects` target names a row that exists.

**A side result the sweep closed for free.** `the_panic_bound_names_a_concern_not_a_marker` carries a `gap`
saying it is unsettled whether the panic may appear in dev builds of the speed-first concern. It is settled.
Op: "panic is the go-to for things that can't be caught on compile time via typestate, when on debug builds,
even on hot", already recorded verbatim on the ratified row it corrects. The `gap` can be closed by pointing
at its own neighbour.

## The tool a coordinator would work this list from does not know any of it

`cargo mock awaiting-a-ruling` lists all fifty-two, in one flat list, with a citation count beside two of
them. `mock/tools/awaiting-a-ruling/src/lib.rs:120` filters on `rung == "stated"` and on nothing else.

**So `ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` appears on that list with no mark saying op
has already looked at it and declined. `a_strategy_is_a_preset_naming_a_point_with_a_weighting_in_it` appears
with no mark saying its hold is deliberate and recorded in a question row. `warm_behaves_as_a_native_rust_
primitive_would` appears at the top, with no mark saying a ratified row supersedes it.** Under the old model
that cost a round trip. Under the model settled on 2026-08-31, where the coordinator holds the gate and
expert convergence ratifies, **a coordinator working this list top to bottom promotes all three, and there is
nobody left to catch it.**

The tool is doing what it says it does. What is missing is that three states exist which its one field cannot
express: superseded by a ratified successor, held on a recorded question, and put to op and declined. **The
first two are already derivable from the registry**: a row named in a ratified row's `supersedes`, and a row
named in an unanswered question row. The third is not derivable, because nothing records it, which is the
next finding.

## Op's refusal is nowhere in the canon, and neither are three of his statements

**A refusal to bless is a statement of his and there is no place for it.** `ruling.toml`'s own header says
the namespace is "What op has said, one row per statement". His words on the ingest row are a statement about
that row, they were captured by two independent processes, and they live only as a blockquote in a panel
file. The row carries no field for it and the tool cannot see it.

**And three of his statements from `212_op_the_alias_scope_the_demotion_and_observability` have no ruling row
at all.** I checked: `grep -n "212_op" mock/registry/` returns three hits and all three are in
`question.toml`, on `answered` fields. They are:

- **"It becomes your intent"**, on whether the licence clause about unobserved stretches is his. **He
  designated that an intent, in those words**, and it has no row in the namespace whose header says one row
  per statement. It sits at `question.toml:1314`.
- **"It's a rule of thumb that absolutes will fit but not itself an absolute. There must be exceptions. But
  as a general rule. Otherwise yes, demoted a bit, but not entirely. My calls and remarks and statements are
  still valid for the time, and valuable as precedence"**, at `question.toml:1475`. **This directly qualifies
  one of my fifty-two.** `his_voice_is_demoted_except_where_he_frames_it_absolute` states the demotion as a
  binary with one carve-out. He has since said it is partial, that the carve-out is a rule of thumb admitting
  exceptions, and that his prior statements remain valid and carry precedent value. Nothing points at that
  row: `grep -c` finds its id exactly once in `ruling.toml`, which is its own `id` line. I judged the row
  faithful to its quote and it is. It is also, as canon, out of date, and a reader has no way to learn that
  from the ruling namespace.
- **"Let's start with the two named, and reserve the rest named for future, so it can't close them out"**, at
  `question.toml:1801`.

I am not proposing where these should live; that is a schema question and it is the coordinator's. **What I
am reporting is that the ruling namespace's own stated contract, one row per statement of his, is not met for
four statements**, three of which are answers he gave and one of which is a refusal, and that one of the four
silently ages a row I was asked to check.

## Outside the question entirely: `mock/checks` and what op said about it

Reporting under the standing instruction to name unlicensed mechanisms whether or not they are in scope. **I
could not establish that this is drift, and I am not asserting that it is.** Here is what I have.

On 2026-08-31 at 10:53, captured verbatim in `.data/op-responses/202608311053_arvo-canon-mockspace-
discipline.md`, op said, across several messages in one stretch: "There shouldn't be a mock/checks though.
I'm pretty sure it's not allowed even on kamu, was it allowed and there?"; "Read the current mockspace code
and find out why you would default to a check dir, which doesn't seem right. Lints and tools are the two
mechanisms to use"; "I think I disallowed the checks dir. What's the mtime and git history on that? Because
if they've been writing those recently, that's fucking annoying because I am pretty sure I clearly stated
it's not a mockspace standard dir".

The tree today has `mock/checks/` as the sole Cargo workspace member (`mock/Cargo.toml:31`), 18 test files,
6602 lines, 143 tests, and three references in `mockspace.toml` at lines 316, 404 and 1122. `mock/tools/`
also exists with two tools, so some of that stretch was acted on. `mock/Cargo.toml:18` carries a considered
justification for keeping `checks`: that it implements nothing the design says and is a harness over the
canon rather than source.

**That justification is agent-authored and states no human in the loop. His statement is captured verbatim.
Under the provenance ladder the captured statement outranks the manifest comment.** But the capture ends
without a resolution on the checks directory specifically, moving on to the mockspace builtins, and I found
no ruling row, no question row and no panel file recording that the question was settled either way. So the
honest report is that a captured instruction and a shipped structure disagree, that nothing in the repository
resolves the disagreement, and that **I could not establish which way it went.** With nobody left to ask, it
derives from the canon or it does not get decided, and either way somebody should decide it rather than
leaving 143 tests standing on an open question.

## What I did not do, and what I would want next

**I did not check the fifty-two against each other for mutual consistency**, beyond the four supersession
edges the sweep surfaced. Arm C looks at declared edges; two rows can contradict each other with no edge
between them and nothing here would see it. That is a real gap and it is a different dispatch.

**I did not verify the twenty-eight ratified rows.** Two of them turned out to matter to my verdicts and both
were correct when I opened them, which is one data point about that population and no more.

**Arm D is a ranker and I read all fifty of its rows, but its precision is unmeasured.** It found seven of my
eight overreaches and it also ranks `which_half_of_the_pair_i9_attaches_to_is_not_his` and
`a_queue_batch_inherits_the_scope_of_the_work_in_flight` high, both of which I read carefully and judged
faithful: in the first the extra words are the deferral naming what was deferred, and in the second the
derived rule is disclosed in `note` and the `says` stays inside what his outburst supports. **Somebody who
disagrees with either of those two has a reasonable disagreement and the arm is why they can find it.**

**What would move the remaining uncertainty**, in the order I would spend it: a field or a row shape for a
statement of his that refuses rather than decides, since that is what leaves the ingest row indistinguishable
from an unasked one; the three statements from `212` given rows; the two backward `supersedes` edges deleted;
and the `awaiting-a-ruling` tool taught the two states it can already derive.

## Files

- `218_probes/p1_the_stated_rows_and_their_quotes.rs` and `p1.out`. The census and the verbatim check.
- `218_probes/p2_how_far_a_says_travels_from_its_quote.rs` and `p2.out`. The edge sweep and the distance
  ranking.
- `218_probes/control_fixture.md`. Arm D's fixture, and the note about why it must not name the word it
  lacks.

Both probes take the repository root as their first argument and name no path outside it. Build and run lines
are in each file's module documentation.
