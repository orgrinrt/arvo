# 261. Jhala, seat 261: a structural claim can state a region today, and nothing can tell whether it is the right one

Base of this worktree: `5644b8f0`, branch `research/whether-a-structural-claim-carries-a-region`.

This file and its probes were written as seat 259 and are seat 261. Another seat dispatched
alongside this one took 259 from the same base and wrote its own `259_probes/`, and a third took
258, which the trunk already held. None of the three could see the others. Two seats at one number
merge without a conflict, because the probe filenames are disjoint, so every later citation of a
shared probe directory would have named two seats' instruments at once. The number is an
identifier and carries no meaning, so the later two were reallocated. Every bare `258` below names
`258_orchard_what_the_admission_and_standing_sitting_settled.md`, which is the seat that held the
number all along.

## 0. The two gates

**Canon gate: passed.** The question is open, its `decider` is `panel`, and
`ruling::the_panel_finishes_the_canon_without_him` puts every remaining canon question there. What I
checked the work against, in tier order: the ratified rulings
`the_work_is_predicated_arms_composed`, `the_warrant_is_a_token_and_a_clause_on_the_values_side` and
`the_panel_finishes_the_canon_without_him`; the stated rulings `a_predicate_lists_only_what_holds`
and `there_is_no_universal_answer_take_the_win_and_gate_it_out`; the `proposal`, `ruling` and
`dimension` namespace declarations in `mockspace.toml`; the 25 committed `dimension` rows; and the
shipped lints `a-region-agrees-with-the-sentence-kind`, `every-predicate-names-a-declared-axis` and
`an-imposition-rests-on-no-instrument`.

One correction to the brief's framing rather than a refusal, in section 3: the question's own premise
is false as a matter of fact, and the thing it is right about is not what its three options are about.
That does not make the question improper. It makes it a question whose answer reshapes it, which the
`question` namespace's `answered` field says is the most valuable kind.

**Test gate: run, and it caught something bigger than the suite.** `cargo mock test` reported seven
of eight trees green and benches red. The benches red is deliberate and marked: four variants inherit
a workspace `arvo` dependency deleted on 2026-08-08, and `mock/benches/variants/fnv1a/Cargo.toml`
carries a FIXME saying so and saying why repairing it would measure nothing. That is a catalogue red
and I left it.

**What the gate did not report is that the lint pack had not been built from source since the engine
pin moved, and at that pin did not build at all.** Every green in that run, and every green at every
commit hook that day, came off a cached cdylib. Section 8 is that finding, its isolation, and its
repair upstream mid-dispatch. It cost most of this dispatch and it is the reason the decisive probe
took the shape it did.

I read the bodies of the three lints under test end to end, and of `a_standing_is_reachable_from_what_it_cites`
for section 7. They are real: each carries a control arm that must be silent, each reports every
offending row rather than the first, and `a_region_agrees_with_the_sentence_kind` tests both halves of
its contract against each other rather than one twice. I found nothing tautological in them.

## 1. The answer, before the argument for it

**A reasoned claim about the canon's own contents can carry a region today, and two of the three
options are answers to a premise that does not hold.** Twelve `proposal` rows sit on the four topics
that are about the canon rather than about arvo. Eight are `normative` with no region. **Four are
filed `measured`, `enumeration` and `argument` and carry a predicate**, which the question says is not
possible. `261_probes/c_what_can_carry_a_region.out` names all twelve.

**And every one of those four writes a region that is not its own.** Two write the numeric region of
the claims they are talking *about*. Two write `threads: threads = 1` and nothing else. One of the two
says so in its own `note`, in terms:

> The predicate carries almost none of this claim's scope: its coordinates are a named branch head,
> 254 files and 24 producing commits, none of which is a declared axis.

**So the gap is real and it is one layer in from where the question puts it.** It is not that a
structural claim cannot state a region. It is that **whatever it states, nothing can be wrong.** The
planted-row probe shows a structural claim passing every predicate lint with all 25 axes at `any`, and
passing equally with any single axis pinned to a value the claim was never observed at
(`261_probes/e_lintdrive/tests/drive.rs`, arms `d3` and `d8`). A structural claim is handed 25 axes it
may set to anything and no way to be refuted on any of them.

That is a defect in what a checker can falsify, not in the dimension vocabulary and not in the locus.
It is therefore a `sentence_kind` and warrant question, which is where `229` section 4 put it before
me and said it was not its dispatch.

## 2. Breaking the brief and the row first

**The row's `note` states the axis count as twenty-one. It is twenty-five.** `occupancy`,
`association`, `leaf_aliasing` and `phase` were declared after the question was raised, and
`the-axis-set-is-append-only` means the number only goes up. Nothing in the question turns on the
count and I record it because the row's own list is quoted downstream.

**The row's second option says the panel would then be unable to tell a reasoned structural claim from
an imposed one.** That is true and it is the smaller half of the cost. The larger half is measured at
`261_probes/e_lintdrive` arm `d4` and again through the real gate at
`261_probes/d_can_a_structural_claim_state_its_region.out`: **`an-imposition-rests-on-no-instrument`
refuses a `normative` row that carries `evidence`.** So filing a structural claim `normative` does not
merely blur it with governance, it **strips the instrument that produced it**. Of the eight machinery
rows filed `normative`, not one carries `evidence`, and several of them ran something.

**The row's third option is not a cost, it is forbidden**, and section 5 argues that from the
workspace rules rather than from preference. I am the first read on that call and a second is owed.

**And the brief's word "region" needs one distinction the question does not draw.** A region in this
registry is a value of one of exactly three fields. Everything below turns on which.

## 3. What can carry a region at all, measured

`261_probes/c_what_can_carry_a_region.sh`, five controls, all passing. Two results.

**Three fields in the whole registry are read as a region**, and they are declared in one constant at
`mock/lints/canon_rows.rs`, `PREDICATE_FIELDS`: `proposal.predicate`, `law.holds`, `law.fails`.

**So the canon proper states no region for anything.** The `ruling` namespace declares neither
`predicate` nor `sentence_kind`; its columns are `id, kind, rung, ratified_by, promotion,
ratification, topic, says, because, instead, answers, obligation, ratifies, supersedes, corrects,
note, gap, provenance, keywords`. A claim promoted from `proposal` to `ruling` **loses its region by
construction**, whether it is about a numeral or about the canon.

That reframes the whole question. **The region question is a proposal-tier question and nothing
else.** Asking whether a structural claim can carry a region in the canon is asking whether it can
carry something the canon has no field for, and neither can a claim about saturating addition.

**All 25 declared axes are about a numeral or a machine**, walked one by one in the same probe, with a
control showing the classifier can say otherwise: a planted coordinate reading "the slug of the topic
a row is filed under" classifies as neither. So that half of the question's premise holds exactly.

## 4. What the checker actually accepts, run twice by two instruments

The decisive work. Six candidate spellings of one claim, and the claim is the layering derivation
already in the registry, whose own note says it was "filed `normative` after being written `argument`"
because "no predicate, because none in this registry can express it".

**Instrument one**, `261_probes/e_lintdrive/`: a probe crate that pulls the three lint sources in by
`#[path]`, the way the engine's generated pack does, and depends on `mockspace-lint-rules` once. Eleven
arms, three of them controls, all passing. **Instrument two**,
`261_probes/d_can_a_structural_claim_state_its_region.sh`: the same six rows appended to the real
`mock/registry/proposal.toml`, judged by `cargo mock --lint-only` over the whole real registry, then
truncated back with the tree asserted clean. Four controls including a clean baseline and a
must-fire arm.

The two agree on all six.

| candidate | verdict |
|---|---|
| `argument`, no region | refused, `an-established-claim-carries-no-region` |
| `argument`, region over `registry_state` and `namespace` | refused twice, `undeclared-axis` |
| `argument`, all 25 axes at `any`, tokenless | **accepted, silent** |
| `normative`, carrying `evidence` | refused, `an-imposition-rests-on-no-instrument` |
| `normative`, carrying a region | refused, `an-imposed-proposition-carries-a-region` |
| ordinary numeric row, correctly formed | silent, as the control requires |

Two further arms exist only in instrument one, because they need many registries rather than many
runs of the engine.

**`d6`: a borrowed region passes every lint.** `threads: threads = 1`, `total_width: W in 3..=7` and
`fraction_width: F = 0` are each accepted on a claim about which topic rests on which. That is not a
hypothetical: it is what the four region-bearing machinery rows do.

**`d8`: narrowing any single axis is equally acceptable.** For each of the 25 axes in turn, the same
claim with that axis pinned to "the one value this claim was never observed at" and the other 24 at
`any` draws nothing from any lint. **Twenty-five ways to be wrong, and the checker sees none of them.**

**`d7` runs the region obligation over the whole declared `sentence_kind` set**, six values and both
directions, rather than over the two the question names. The rule holds uniformly: four kinds owe a
region and are reported without one, two owe none and are reported with one.

## 5. The three options, each weighed, and where each is right

Per `never-ask-which-single-rule-governs`, I am not looking for the winner. Two of the three are right
somewhere and the third is not right anywhere.

### Option one, grow an axis: refuted by the checker's own sentence

When `every-predicate-names-a-declared-axis` refuses a structural axis it says why, and the reason is
the argument against the option:

> An undeclared axis cannot be absent from any other predicate, so admitting one here weakens every
> predicate in the canon.

**Declaring one does the mirror of that.** Under `ruling::the_work_is_predicated_arms_composed` and
the notation built on it, an axis a predicate does not name is one the claim holds nowhere involving.
Declare `registry_state` and every committed predicate silently becomes a claim about a structural
coordinate it never named. `229` section 5 argues the opposite, that "declaring an axis does not reach
backwards, because a predicate's absence quantifies over the world rather than over this file's
contents, so a row written before a declaration was always exactly as narrow as it now reads". **Both
are true and they are the same fact from two sides**: nothing changes about what the rows meant, and
what they meant becomes visibly stranger, because a numeric law would then be asserting that it holds
in no situation involving a registry.

And it is one-way: `the-axis-set-is-append-only` gates at every commit, so a wrong axis is permanent.

**Where option one is right: nowhere I could find, and I looked at the neighbouring case.** The same
pressure fires on ordinary numeric claims where one axis is inapplicable, which
`ruling::the_additive_and_absorption_verdicts_are_canon` records in its own `note`: the additive row
lists no `rounding` where its multiplicative sibling lists `rounding any`, "addition at a common scale
does not round, so the omission is arguably correct, but the notation has no inapplicable state". A
new axis does nothing for that case, and `229` and `230` between them settled it without one, as
`any` plus a `construction` clause or as a tokenless `any`. An option that helps neither the numeric
case nor the structural one is not an arm.

### Option two, intended: right for governance, wrong for anything that ran an instrument

Split the eight `normative` machinery rows and the option splits with them.

**Right, and no change wanted**: `naming_is_partial_and_injective_or_it_is_broken`,
`a_transfer_argument_is_a_construction_warrant_and_needs_no_new_rule`,
`const_availability_is_the_axis_and_a_trajectory_condition_is_not_an_arm`. These impose. They
establish nothing, they owe no region, and `normative` is exactly what they are.

**Wrong**: a structural claim that walked something. `an-imposition-rests-on-no-instrument` refuses
its `evidence`, so the run that produced it is unciteable, and the row is left asserting a count with
nothing behind it. That is a worse outcome than losing a region, because a region can be narrow and
still true while an uncitable count is exactly the shape
`evidence-lives-in-the-repo-or-it-never-happened` forbids.

The row that raised this question is in the second group. It is the coordinator's derivation over the
`what` sentence of twenty topic rows, which is a walk, and it carries no `evidence`.

### Option three, move it out of the registry: forbidden, and I am the first read

Two grounds, and the first is decisive on its own.

**A workspace rule is op's words by construction.** `every-rule-is-ops-words.md` opens: "All rules
here are op's words, straight from the lead designer." `writing-for-agents.md` makes the mechanism
explicit: "nothing in a rule is attributed, quoted or sourced... The file existing is the
ratification, and a rule is read as his words." **A one-expert panel derivation written into
`.claude/rules/` is therefore laundered into ratified authority in the act of filing it**, which is
the precise drift the whole ratification model exists to refuse and which
`ruling::the_panel_finishes_the_canon_without_him` warns about in its own words, that his absence is
not a licence to invent.

**And the registry already carries structural claims at the ruling tier and cannot stop.**
`the_panel_finishes_the_canon_without_him` is a claim about how the panel is run.
`the_warrant_is_a_token_and_a_clause_on_the_values_side` is a claim about how a predicate is written.
`the_canon_is_written_once_at_the_end` is a claim about when rows are admitted. `255`'s R5 is a claim
about how the obligation rows may be read. Option three would move all of them, and the question's own
statement of the cost, that "the registry loses the ability to state what it rests on", understates
that these are already there and several are ratified.

**Two experts are required for a call about what the canon permits and I am one.** A second read on
this one specifically is owed, and it should come from somebody who has not written on the byline or
ratification questions.

## 6. The shape I think is better supported, stated as a composition

**Nothing about a structural claim's region is missing from the notation. What is missing is anything
that can be wrong about it.** So the useful output is not a fourth option to pick, it is a set of arms
and the region each holds in.

**Arm one, a structural claim that imposes.** `normative`, no region, no evidence. Correct today,
nothing to change, and three of the eight rows are already here.

**Arm two, a structural claim that stipulates a term.** `definition`, no region, `defines` naming the
term. Correct today for the same reason a definition about numerals is: establishment does not apply
to it. `186` reached that independently and it is not disputed.

**Arm three, a structural claim that reasons and ran nothing.** `argument`, and the region is the
universal on every declared axis, tokenless. `d3` shows the checker accepts it and
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` licenses the spelling: "An entry
carrying no token claims no warrant, which is what every existing entry keeps meaning." It says the
one true thing available, that the claim is not about numerals, and it says nothing false.

**Arm four, a structural claim that reasons and ran an instrument.** `enumeration` where a bounded
list was walked, `measured` where an instrument produced a number, `evidence` naming the probe, and
the same universal region. **This is the arm the corpus needs and the one the current filing pressure
pushes off**, because the alternative filing that keeps the region-free look is `normative`, and that
one refuses the evidence.

**What every arm two through four still cannot do, and this is the residue.** The true coordinates of
a structural claim, the commit, the file set, the row count, live in prose. `229` wrote them into its
own predicate line, "holds for: this registry at commit `0e71955b`", and
`most_committed_bench_regions_predate_the_harness_cross_variant_validation` wrote them into its
`note`. Both are honest and neither is checkable. **A universal region plus prose coordinates is the
best available today**, and a reader should know that the universal part carries no information and
the informative part carries no enforcement.

**When the answer flips.** Every arm above holds while the region-bearing surface is the three fields
in `PREDICATE_FIELDS` and while the `dimension` set is closed to numerals and machines. Grow either
and arms three and four are the ones that move.

## 7. The two second reads that were owed

### 7.1 `258` asked for a reader who opens the declaration before the seats. What the ordering changed

I committed my reading of the `standing` declaration before opening `247`, `248` or `249`. It is
`261_probes/a_standing_read_before_the_seats.md` and it landed in commit `ca9f3d27`; every read of
those three files is after it, which is checkable in the log rather than asserted here.

**On the core claim it changed nothing, and that is the useful result.** From the declaration alone I
derived that the unit counted is an arrival rather than an assent, that the blindness condition is
written into the value rather than left to convention, that the count is regional, that the failure
named is false independence, and that `contested` is orthogonal rather than low. `248` section 3 and
`249` section 1 say the first three in almost the same words. **Under the field's own temporal
criterion my agreement is an arrival and not a confirmation**, which is exactly what `258` could not
supply for itself and exactly what the ordering buys. So the reading now stands at three independent
arrivals rather than two plus a reader.

**I also reached `248`'s locus finding independently**, that the schema is not canon: my committed
note ends "The schema is not ratified canon, it is the configuration the canon is stored under". `248`
section 2 makes the same observation and takes it further, that `standing` therefore cannot be
ratified at its current address. I did not reach the further step.

**What the ordering changed is one thing neither seat has.** Reading the declaration cold, the
asymmetry in the value list is the first thing that stands out: **`two_experts` carries an
independence gloss, `cross_topic` carries one, and `three_or_more` carries none.** Both seats quote
the whole list and neither remarks on it. Reading the seats first, you inherit their focus on the two
glossed values, which is I think why it has not been said.

### 7.2 The floor that does not scale, measured

The gloss gap is not only editorial. `mock/lints/a_standing_is_reachable_from_what_it_cites.rs`
reasons per arrival, in its own doc comment: "Independence is between authors, and a numbered member
file has one author, so one citation cannot exhibit two arrivals however the note describes them."
**Its predicate is `files.len() >= 2` for every member of `MULTI_ARRIVAL`, which includes
`three_or_more` and `cross_topic`.** Two files is the right floor for two arrivals and it is the floor
applied to all three.

`261_probes/b_three_or_more_floor.sh`, five controls, all passing, including one that reproduces the
lint's own measured ceiling of 29 exactly from an independently written reader. The result:

**All twelve rows claiming three or more arrivals cite fewer than three files, and ten of the twelve
cite exactly one.** Six `three_or_more` rows cite one file each. Six `cross_topic` rows cite one or
two. Under the lint's own reasoning none of them is reachable, and under the lint's shipped predicate
ten of them are and the other two are inside the grandfathered ceiling.

I am not proposing a change to the constant. **The finding is that the declaration and the lint agree
with each other and both under-describe `three_or_more`**, and a seat repairing the backlog the
ceiling covers will want to know that clearing a row to two files does not clear it.

Two controls fired while building this and both are kept. The first extractor read the row list out of
the query renderer, which truncates a long `id` with an ellipsis, and silently saw one row of six;
`b_renderer_truncation.sh` reproduces that. The second read `provenance` as a single line and reported
every row citing zero files; `b_second_attempt.out` is that run.

### 7.3 `255` R4: the rung is right and the kind is not

**R4, `the_const_generic_parameter_is_the_one_excepted_position`, filed `kind = "intent"`,
`rung = "in_force"`.** The call it makes about what the canon permits is that a quote of op's, said in
another repository's design round, belongs at arvo's ruling tier.

**On the rung I agree, and the schema decides it rather than judgement.** `in_force` is declared as
"where the workspace and this repo's own lints enforce it independently of convergence". Both halves
hold and I checked both rather than taking `255`'s word. The workspace rule
`no-bare-primitives.md` carries the exception under its own heading, "The const generic parameter is
excepted, op's own exception", with the two bounds. Arvo's own generated `type-surface.md` carries it.
And the repo's lints enforce it: `mock/lints/the_write_hook_excepts_what_the_canon_excepts.rs` exists
because the write hook once denied the excepted position on every const generic parameter, and its
table carries the excepted rows and the denied rows as controls. `stated` would be wrong, because
`stated` is for direction that nothing enforces.

**On the kind I disagree, and the schema decides that too.** `intent` is declared as "a want stated
with no question in front of it"; `ruling` is "a question answered". R4's own `note` records the
provenance: "Said about `kolli-api`, in a design round of that repository, on whether the contracts
crate keeps the bare `usize` it shipped with." **A question was in front of it.** The speech act was
an answer, so `kind = "ruling"`. That there was no question in front of it *in arvo* is a fact about
where the row is filed rather than about what he did.

Small, and worth saying because `kind` is what a later reader uses to tell a want from a decision, and
a decision recorded as a want reads as softer than it is.

### 7.4 `255` R5: one row carrying two sentences with different lifetimes

**R5, `the_primitive_surface_is_cut_by_kind_and_the_demand_rows_are_a_sample`, filed
`rung = "ratified"`, `ratified_by = "experts"`.** The call it makes is that a reading rule over the
demand side is canon rather than bookkeeping.

**My reading is that it is both, and the row binds them.** Two sentences:

**(a) The consumer requests are a demand-side sample and never a decomposition of what to build.**
This is permanent. It follows from the `obligation` namespace's own header, which says the namespace
is "Read from outside the canon on purpose" and that "an obligation nobody enumerated is invisible to
every check that walks what the canon already covers". It survives any rewrite of anything. It is
canon, it is an intent about how the canon is read, and it is exactly the kind of structural claim
section 5 argues belongs in the registry.

**(b) The surface is cut by kind, and the kinds are these five.** This is an enumeration over the
registry at a moment. R5's own `promotion` says so: "Region: the five rows as they stand at
`b34d7a3c`." **That is a region, written in prose, inside the field that carries the promotion
argument, because the `ruling` namespace has no field to put it in.** It is the same move
`most_committed_bench_regions_predate_the_harness_cross_variant_validation` makes in its `note` and
the same move `229` makes in its predicate line, and it is the third independent instance of the
primary question's residue arriving from a direction that was not looking for it.

**Sentence (b) fails the permanence test.** `261_probes/g_r5_permanence.sh`, three controls, all
passing: the `obligation` namespace went from 11 rows to 16 across nine commits in two days, four
distinct counts. It has not moved since `b34d7a3c` **on this branch**, which is a fact about this
branch rather than about the claim's durability, and the probe says so rather than reporting the zero
as reassurance.

**So my second read is: admit (a), and do not admit (b) at the ruling tier as written.** The `ruling`
namespace can hold neither the region (b) needs nor the evidence the census rests on, so admitting it
puts a dated count where nothing can date it, which is what
`a-claim-of-totality-names-what-enforces-it` warns about in the words "a sentence carrying one is a
claim with an expiry nobody sees".

**Where (b) should go is where the primary question bites.** It is an `enumeration` over the canon's
own contents. Arm four of section 6 is its filing: `sentence_kind = "enumeration"`, `evidence` naming
the census, a universal region on every declared axis, and the true coordinates in prose. **That
filing exists and is honest and is not checkable**, which is the residue this whole file is about, and
R5 is a cleaner example of it than the layering row that raised the question.

I have not written on `251`, `252`, `154`, `238` or `235`. `255` section 4.4 asks for a persona that
has not, and that condition holds.

## 8. The finding that cost the most, and it was not in the question

**At engine revision `b57007c`, `cargo mock` could not build this repository's lint pack, and had not
built it from source since the pin moved there.** Every gate that reported green that day, the commit
hooks included, reused a cached cdylib. `rm -rf mock/target/mockspace-lints/target` is what made it
visible, and it made it visible only because I was trying to run a probe.

`261_probes/f_the_url_mismatch.sh` isolates it to one string, four controls, all passing in
`f_the_url_mismatch.out`. The engine wrote the generated crate's mockspace dependency and its
`[patch]` table with the `https://` spelling, from `cargo-mock/src/lib.rs`'s `CANONICAL_URL`.
`mockspace.toml`'s `[lint-crates]`, `mockspace-extra-lints`' own manifest and all five of this repo's
tool crates spell it `ssh://git@`. Cargo keys a git source by the literal URL, so the patch never
reached the ssh dependents, two copies of `mockspace-lint-rules` entered one graph, and the `LintPack`
one handed the other was a different type with the same name. Six `E0308`s. Change the one string in
the generated manifest and it built; the engine rewrote it back on the next run, which is control F4.

**It fetched fine, and that is why nobody saw it.** The machine carries
`url."git@github.com:".insteadOf "https://github.com/"`, so git rewrote the transport and cargo still
keyed the source by what the manifest said.

**It is fixed, and the same instrument says so.** Mid-dispatch the launcher re-resolved `dev` and the
engine pin moved from `b57007c` to `a7dd8223`. **At `a7dd8223` the engine writes a `[patch]` table for
both spellings**, the pack builds from a clean cache with no workaround at all, and the whole of
`--lint-only` passes. `f_the_url_mismatch_after_the_pin_moved.out` is the same script on the same tree
at the new pin, and F2 and F3 report FAIL there: F2 because the build now succeeds where the probe
required it to fail, F3 because rewriting the https table into a second ssh one is now a duplicate
key. **A control firing on a repaired tree is the control working**, and both outputs are committed
rather than the second replacing the first.

**So this is a finding about a revision rather than a live defect**, and what survives it is worth
more than the defect was.

- **A stale cdylib made a hard build failure invisible to every gate in the repository.** The pack is
  built once and reused, the manifest it is built from is engine-written and moves under it, and
  nothing compares the two. The failure mode is not "the lints are wrong", it is "no lint ran and
  everything said green". That is `a-restored-file-does-not-rebuild`'s second half in a place that
  rule does not name.
- **`cargo mock test` has a second cache beyond the one `cargo clean` reaches.** It redirects several
  trees into a shared `mock/target/mockspace-test/`, and that directory held objects from the broken
  resolution, so the lints tree kept failing after the graph was fixed. Removing it took the suite
  back to seven of eight.
- **The engine pin moves during a session, silently.** It moved once in this dispatch and it is what
  turns a green tree red between two commits with no edit in between. The tool locks lag it each time,
  `the-tool-locks-pin-one-mockspace` reports it precisely, and the repair is `cargo update -p
  mockspace-lint-rules` in each tool directory. I ran that twice, once per pin.
- **Arvo's tool manifests are right and the old engine was wrong**, not the other way round.
  Switching the five tools to `https://` took the error count from six to one and is the wrong
  direction: the workspace rule on private repositories is why they are spelled `ssh://`, and the
  https spelling works here only because of a git config on one machine. The repair that landed
  upstream is the other one, both tables, which is the right shape.

  **The reasoning in that bullet is wrong on both halves, and the correction was made after this
  file was pushed.** `gh repo view hiisi-digital/mockspace --json visibility` answers `PUBLIC`, so
  the private-repository rule does not reach this dependency at all; that rule's own text names
  the public repositories in the stack as the reason the https spelling has always worked. Nor
  does https work here only through a machine-local git config: an anonymous fetch of a public
  repository succeeds with no credential, and the `insteadOf` rewrite exists to keep a credential
  helper out of the way rather than to make the fetch possible. So the direction the bullet calls
  wrong is the one the repository took, at `1ac3c8de`, moving all five tool manifests and the lint
  pack onto `https`, which is also the engine's own `CANONICAL_URL`. **The bullet's conclusion is
  unaffected**: both patch tables upstream is the right shape and is what landed, and it is what
  makes every consumer immune rather than this one.

**What I committed from all of this is one `.gitignore` line** saying a local cargo config is not
tracked, which the workspace's own cross-repo iteration rule already assumes and which stands whether
or not this defect exists. The workaround it was written for is gone and no `.cargo/config.toml`
remains in the tree. **One thing about it is worth keeping in view**: while it existed, a root-level
patch table reached every lockfile in the tree and `cargo mock test` rewrote 97 of them with a
`[[patch.unused]]` stanza. `h_lockfile_churn.txt` names them. They were reverted, not committed, and
the next person reaching for a root `[patch]` should know it costs that.
## 9. The instances behind section 1, and how independent they are

Seven, six routes. `evidence-lives-in-the-repo-or-it-never-happened` asks for three or more and asks
that they be arrived at differently, so the routes matter more than the count.

1. **`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`**, `note`. The row that
   raised the question. Self-report, so the weakest.
2. **`proposal::every_canon_sentence_names_the_prefix_it_quantifies_over`**, `note`. Different topic,
   different consolidation, and it names the shape in one sentence: "This sentence and the predicate
   notation are two halves of one discipline over two disjoint sets of axes, and neither names the
   other." Reached from the number-system unit, not from the notation.
3. **`ruling::the_additive_and_absorption_verdicts_are_canon`**, `note`. The inapplicable `rounding`
   asymmetry. **Reached from a numeric claim**, which is what shows the class is not about the canon's
   structure at all.
4. **`proposal::most_committed_bench_regions_predate_the_harness_cross_variant_validation`**, `note`.
   A `measured` structural claim saying its predicate carries almost none of its scope.
5. **`255` R5's `promotion`**, writing a region into a promotion argument.
6. **`229` section 4**, writing "holds for: this registry at commit `0e71955b`" as a predicate line in
   prose.
7. **This file's probes**, which are mechanical rather than testimonial and are the only ones that
   could have come out the other way.

**Independence is real between 2, 3, 4 and 7** and weaker between 1, 5 and 6, which are three seats
noticing the same thing about their own writing. All seven are unratified agent output and several
agreeing is shared drift rather than corroboration, which is why arm 7 exists.

## 10. What I could not establish, and what would close it

**Whether the universal-region spelling is honest under `ruling::a_predicate_lists_only_what_holds`.**
That ruling is op's, at `rung = "stated"`, and says what is unmeasured goes unstated and implicitly
means not true. Writing `threads: threads any` on a claim nobody ran at two thread counts looks like
writing an unmeasured `any`. **`230` section 6.3 isolated this before me and named it precisely**: it
is not uncontrollable, it is inapplicable, and those are different, so "a universal true by
inapplicability has no warrant, and the notation offers a spelling for it that the discipline above
the notation is uneasy with". `230` said that needs two experts and is not one seat's. **I am the
second arrival at the same collision and I still cannot settle it**, because settling it means
ranking a ratified expert row against a stated row of op's, and nothing I found ranks them.

**Whether a lint could tell a claim's own region from a borrowed one.** I think not, and I could not
build the counter-instrument. The information that would distinguish them is what the claim's subject
is, and no field carries that; `topic` is the nearest and it is about subject matter rather than about
which axes the subject instantiates. **A seat wanting to attack this should start by asking whether a
row can declare that its subject instantiates no declared axis**, which is a one-bit fact a checker
could then hold it to, and which is not a new dimension.

**Whether the four region-bearing machinery rows should be corrected.** I have not proposed
retirements. Each is true in its `says` and wrong in its `predicate`, and
`every-finding-carries-its-predicate` says a predicate is never widened in place and a correction goes
in the later expert's own deliverable. This is that deliverable and the correction is stated; whether
it reaches those rows is the coordinator's.

**Whether `three_or_more` wants a floor of three.** I measured that nothing meets one. I did not
establish that raising it is the right repair rather than lowering the value on twelve rows, and the
lint's own ratchet reasoning cuts both ways.

## 11. Findings, with what carries each

The notation is `every-finding-carries-its-predicate`'s and I am using it against its own grain, which
is the subject of the file. **Every finding here is structural: its coordinates are this repository at
`5644b8f0`, the files named, and the shipped lints at the revision the launcher pins. No declared
dimension carries any of that.** Writing the universal on all 25 axes would be admissible and would
carry no information, so I have written the true coordinates instead and said what they are, which is
arm three of section 6 practised rather than described.

**F1.** Three fields in this registry are read as a region, and `ruling` declares none of them.
*holds for: this registry at `5644b8f0`; `PREDICATE_FIELDS` in `mock/lints/canon_rows.rs`; the twelve
namespaces `mockspace.toml` declares. No declared axis bears on it.*

**F2.** Twelve `proposal` rows sit on the four machinery topics; eight are `normative` with no region
and four carry one. *holds for: this registry at `5644b8f0`; the machinery topic set read out of
`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`. No declared axis bears on
it.*

**F3.** Every one of those four writes a region belonging to the run or to the claim it discusses,
and one says so in its own note. *Same coordinates as F2.*

**F4.** A structural claim with all 25 declared axes at `any`, tokenless, passes every predicate lint,
and so does the same claim with any single axis pinned to a value it was never observed at. *holds
for: the three lints at the revision `mock/target/mockspace-lints/Cargo.toml` pins; the 25 axes
declared at `5644b8f0`; both instruments in `261_probes`. No declared axis bears on it.*

**F5.** A structural claim filed `normative` may not cite the instrument that produced it. *Same
coordinates as F4.*

**F6.** Twelve rows claim three or more independent arrivals and none cites three files; ten cite one.
*holds for: `proposal.toml` and `proposal-the-later-topics.toml` at `5644b8f0`; the lint's own
`file_named` notion of a citation, reproduced to its measured ceiling of 29. No declared axis bears on
it.*

**F7.** At engine revision `b57007c` the generated lint crate did not build, for one URL spelling,
and the gates were green on a cached object. At `a7dd8223` it builds with no workaround. *holds for:
mockspace at `b57007c` and at `a7dd8223`; this repository at `5644b8f0`; one machine carrying
`url."git@github.com:".insteadOf`. No declared axis bears on it.*

## 12. Options this opens, and what closes each

**O1. Whether option three of the question is forbidden or merely costly.** My reading is forbidden,
on `every-rule-is-ops-words` and `writing-for-agents`. **Closed by a second expert forming the same
reading from those two files before reading this one.** It does not go to a third if we disagree.

**O2. Whether a row can declare that its subject instantiates no declared axis.** A one-bit field
would make arms three and four checkable and is not a new dimension. **Closed by somebody deriving
whether that bit is derivable from `topic` instead, which would make it free.**

**O3. `255` R5, admitted whole or split.** My read is split. **Closed by the coordinator's gate, since
the question is whether the promotion paragraph states the evidence and region well enough and the
region is the thing I say it cannot state.**

**O4. `255` R4's `kind`.** `ruling` rather than `intent`, on the schema's own words. **Closed by
reading the `kind` field's declaration; nobody needs to re-derive the provenance.**

**O5. The tokenless universal against `a_predicate_lists_only_what_holds`.** Open since `230` and
still open. **Closed by two experts ranking a ratified expert row against a stated row of op's, or by
finding that the two do not meet at all because inapplicable is not unmeasured.** I lean to the
second and did not establish it.

**O6. Whether `three_or_more`'s floor should be three.** **Closed by whoever works the ceiling down,
because the answer changes what "cleared" means for each row they touch.**

## 13. Carried forward unchanged, and from whom

- **`229`, that this is a `sentence_kind` question rather than a `dimension` question.** I reached it
  from the lints and it reached it from the corpus; the relocation is its, not mine.
- **`229`, that an inapplicable axis is the widest positive state plus a clause rather than a fourth
  region state.** Not re-derived, and section 5 rests on it.
- **`230` section 6.3, the whole of it**: that the class is an axis whose values do not index any
  observation of the claim, that a differential which cannot fail is not a control, that a tokenless
  `any` is admissible under the ratified marker ruling, and that the residue is two ratified rows
  meeting. My section 10 adds an arrival and no result.
- **`248` section 2, that `standing` is defined outside `canon_paths` and cannot be ratified where it
  lives.** I reached the first half independently and not the second.
- **`249` section 2, that `cross_topic` is count-shaped.** My section 7.2 counts it as one and would
  have missed it.
- **`255` sections 2 and 3 entire**, the agreement ledger and what moved the candidate at `161`. Not
  re-derived; my second read is scoped to R4 and R5 as `255` asked.

## 14. Coverage, bounded

Read end to end: `223`, `224`, `229` sections 4 and 5, `230` section 6, `247` section headings only,
`248` sections 0 to 4 and 8 headings, `249` sections 0 to 2 and its outline, `255` sections 4 and 4.4,
and `220` sections 8 and its warrant inventory. **I did not read `247` in full**, and its census of
where `73`'s content reached the cold seats bears on section 7.1 in a way I have not checked.

Opened and quoted from the registry: 14 rows, each by slug. Opened raw in the TOML rather than through
the renderer: 4, after the renderer's truncation cost me one instrument.

**Not verified**: anything about `251`, `252` or the primitive surface itself beyond what `255` states
about R4 and R5, and I have taken `255`'s account of the two blind derivations behind R5 on its word
rather than opening either.

**Every number in this file is in a committed probe output beside it.** The counts in sections 1, 3, 4
come from `c_what_can_carry_a_region.out`, `d_can_a_structural_claim_state_its_region.out`,
`e_lintdrive.out` and `b_three_or_more_floor.out`. The 97 lockfiles in section 8 are named one
per line in `h_lockfile_churn.txt`, which is the record of a run rather than a rerunnable
probe, because rerunning it means dirtying 97 tracked files to produce a number already there.

## 15. The one sentence

A claim about the canon's own contents can state a region today and four rows already do, so the
question's premise is false; what is true is that no checker can tell whether the region is the
claim's own, which makes it a question about what a `sentence_kind` and a warrant can be held to
rather than about a missing axis or a wrong address.
