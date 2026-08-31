# 207. McSherry: what of op's material in the closed panel the registry does not hold

**Seat:** 207. **Branch:** `research/canon-port-207`.
**Deliverables:** this file, `207_catalogue_op_material_in_the_closed_panel.toml`, four instruments in
`207_probes/`, and one question row.

Both gates ran before the work. `cargo mock` clean, `cargo test -p arvo-checks` green at 26 plus 8 plus the
rest, and I read the bodies rather than the names. The suite is genuinely good, which I say because I spent
the first hour looking for the reason to refuse and did not find one: every conditional arm in
`what_one_field_obliges_another_to_carry.rs` plants both directions, the two ceilings are documented as
ceilings rather than hidden behind an `#[ignore]`, and one of them carries the story of an ignore whose
stated reason had become false. There is one sampled law and I found it, fixed it, and it is section 5.

The canon gate passed against `mock/registry/*.toml` as declared by `canon_paths`, and I read
`206_op_the_canon_test_and_the_standards_bound.md` before assigning a tier to anything.

One correction to the brief, stated first because I was told to read it: **`mock/registry/README.md` does
not exist and never has.** `git log --all -- mock/registry/README.md` is empty. The namespace headers carry
what a README would, and `ruling.toml`'s is the best of them.

---

## 1. The question I was actually able to answer

I was asked what of op's material in the dead panel is not represented in the registry, and what the
registry has to become for it to be.

The second half turns out to be nearly empty, and that is the result rather than a dodge. **The registry
does not have to become anything.** It has the namespaces, it has the fields, and for most of the material
it already holds the correct answer, which is nothing, for a reason op himself gave. What is missing is a
handful of rows and one call that is his.

The first half divides cleanly once you stop treating "op said it in the dead panel" as one category. It is
five categories with five different fates, and the useful thing I can hand the next seat is the map, because
every sweep so far has re-derived it from scratch.

**Route one, and it closes most of the corpus: he demoted it himself.**
`ruling::prior_calls_are_a_historical_log_not_calls` is op saying his prior design calls are collected for
reference, are not calls, not ratified intents, not canon, because the lineage failed. Its own note says
porting one would restore the authority he removed. So for design calls, absence is not a gap. It is the
mechanism working, and a sweep that reports them as losses has misread the corpus.

**Route two: the D-numbered decisions are dead by a separate act.**
`ruling::the_d_numbered_decisions_are_dead`. That single row closes five items which three earlier documents
carry as open, and it closes them correctly.

**Route three: process material moved across the artifact boundary.** Op said the standing rules live in the
rules directory, and several of his closed-panel process instructions are in `.claude/rules/` with his words
quoted. The clearest is conceding-as-an-answer, where he said in the checkpoint itself where it should go,
and it went there. That one is the calibration for the rest of the route: where an instruction has no such
sentence and no rule carries it, the migration reading is a guess.

**Route four, and it is the only way dead material legitimately comes back: he said it again.** The
standards bound is the worked example. It is `13c` from July, it is `D67` in the dead register, and it is in
`obligation.toml` today, not because anybody mined it but because op re-invoked it at `206` as "the rule that
demands we provide first-class matlab and ieee754 compatible apis". Through his own mouth, which the demotion
permits and mining does not.

**Route five: nothing. Those are the losses, and there are nine of them.**

The catalogue is twenty-four rows across all five routes with the evidence per row. I will not restate it
here. What follows is the part a reader would otherwise have to reconstruct.

---

## 2. The roster everybody cites is fourteen files short

`OLD_112_the_op_material_sweep.md` is the definitive index and says so in its own headline. It states that
twenty-three files in that archive are op's own words, and it built that list by opening every file rather
than reading filenames, which is why everyone since has trusted it. `SEED_TALKING_POINTS.md` repeats the
number nine days later.

The archive holds thirty-seven.

```
files matching OLD_*_op_*.md         : 38
  of which the sweep itself (agent)  : 1
op-authored checkpoint files         : 37
roster stated by OLD_112 and by STP  : 23
shortfall                            : 14
```

`207_probes/roster.sh`, with the exclusions asserted rather than trusted, because a count over a glob is a
count over whatever the glob happened to match.

The reason is not carelessness and it is the least interesting thing about it: the sweep was written on
2026-08-05 and fourteen more op checkpoints landed on the sixth and seventh. It could not have seen them. Its
count was right when written and nothing re-ran it, so a stale number became the definitive roster by
citation.

**What that cost is measurable.** `SEED_TALKING_POINTS.md` section 1.13 records the fused-versus-split call
as one op reserved and never made, and marks it new for the current panel. Op released that reservation
himself at `127b`, declining to rule and applying his own staleness principle, which is nineteen checkpoints
after the file the sweep stops at. The archaeology's own coverage statement predicts this exactly: it says
the remaining op checkpoints were reached through `112` and `124` rather than opened, and `112` stops at
`108b`. **It named the limitation and the limitation then produced the miss, in the one section where being
wrong makes a live question out of a closed one.**

I am not scoring a point off it. It is the most honest document in either panel and its coverage statement is
why I could find the error at all. The lesson is narrower and it is about counts: a population figure inside a
document that is itself append-only will go stale silently, because nothing about the number knows the corpus
grew.

---

## 3. The nine that are genuinely lost, and the two that are worth a fight

Nine items are in no registry row with nothing replacing them. Six of them are the sort of loss you shrug at:
a bar, a sequence, an end-state sentence, a downstream-evidence correction, a presentation obligation, a
consumer-tier description. All real, all catalogued, none of them keeping me up.

Two are not like that.

### 3.1 Op used the word canon, and it is not in the canon

`OLD_143b`, 2026-08-07, the whole ruling:

> Function can also be a constant. It's not either or there. And all things change and act granularly, not
> just warm. I call this as intent, settled canon, right now. This small bit in this association now governs
> future talks.

The checkpoint reads it out: being constant is the special case and is a claim about a function's behaviour
over a domain, so a design sentence stating a fixed value without naming its domain is underspecified by
construction, and every stated fact owes both halves, what it varies over and where it is constant.

`207_probes/registry_coverage.out` returns zero across all twelve registry files on three independent
phrasings, in a run where four controls known to be carried returned nine, thirteen, five and three. So the
zero is the registry's silence and not my vocabulary.

**It is not covered by I13 and the difference is load-bearing.** I13 is the one ratified row and it requires
every *finding* to name the region it holds in. This requires every *design statement* to name the domain its
value holds over. Same shape, different object, and I13's own note fixes its scope to the two paragraphs op
ratified and says the surrounding elaboration is not part of what was ratified. Stretching it to cover this
is precisely the thing that note forbids.

**And the reason it was dropped is recorded, which makes it the most instructive item in the file.**
`SEED_TALKING_POINTS.md` section 1.16 is the list of rulings already carried, written so the section around it
is checkable. This is in that list, and the same entry says it is quoted in `OPTIONS.md` **only by its first
clause**, and that whether the panel wants the full statement re-ratified is open. Filed as carried and
flagged as partial in one sentence. Nobody turned the flag into a question, the port that built the registry
read `OPTIONS.md` rather than the flag, and the first clause is what survived.

That is not a filing error. **A negative-control list is exactly where a partial carry goes to die**, because
the list exists to be trusted and read quickly, and a hedge inside an entry on it inherits the entry's
"already handled" reading.

### 3.2 The acceptance criterion for the canon is filed as a question about the canon

`OLD_135b`: the consumer expresses usage in bits and bytes, the typestate derives the matching container and
representation, it validates, it erases on lowering, four parts at once, and "anything less than that, no
caveats left, is unacceptable for this design and canon". The checkpoint records it as the standard the canon
is judged against before promotion and as not negotiable.

It reached the live panel intact, through the seed archive, quoted in six member files. And the only place any
registry file holds it is inside the note of `question::what_then_validate_requires`, as the framing of a
question the panel is still answering. `ruling.toml` has nothing.

**The provenance is inverted.** The namespace whose entire job is holding what op has said does not hold the
acceptance criterion for the thing that namespace exists to build, and a reader of it cannot find the gate
their work is measured against. I am not proposing the row, for the reason in section 4, but the shape of the
defect does not depend on who gets to fix it.

---

## 4. What I could not settle, and why it is his

Here is where I stop, and I want to be precise about why, because "hand it back" is the answer an expert
reaches for when it has run out of road and I have not.

`179_lamport_porting_ops_rulings_into_the_registry.md` section 17 is the seat that built `ruling.toml`
refusing to carry any `PRIOR_CALLS.md` substance into it. It is good work. It reasons from the demotion, it
quotes op in full, it names its own generalisation from twenty-one numbered decisions to the whole corpus as
the step somebody should test, it builds the candidate row it declined to commit so the disagreement has
something concrete to attack, and it names the pricing pillar as the single best argument against itself and
writes that it has no clean answer to it. Section 18 asks for a second reader on exactly this question.

I am that reader, and I agree with the refusal in general. Porting op's dead design calls into `ruling.toml`
would restore an authority he removed, and the three grounds section 17 gives are sound: the rung has no
honest value, the ledger's quotations are elided, and a row built from most of that file would quote a
compression while citing an original nobody opened.

**Where I disagree is bounded, and it rests on a row that same file minted and then never used.**

`ruling::his_voice_is_demoted_except_where_he_frames_it_absolute`, op's words:

> I have demoted my voice in all but the instances where, in this panels transcripts and checkpoints
> **or other files**, I explicitly frame something I say as absolute. Not many have I done so, I remember
> only two, and both of those were very abstract intents only, not specific calls of concrete things

The demotion has a stated exception, the exception reaches "other files", and the closed panel is an other
file. `179` ported that row and then reasoned about `PRIOR_CALLS.md` for a full section without running it
against its own conclusion. Neither porting seat mentions the carve-out anywhere:
`grep -c "demoted my voice"` returns 0 in `179` and 0 in `180`, and the only file in the panel carrying the
sentence is `PRIOR_CALLS.md` itself.

This is the shape `a-governing-claim-is-applied-where-it-hurts.md` describes. The claim was applied where it
closed a gap, which was minting the row, and not where it falsifies a standing conclusion, which was the
refusal in the same document. Nothing about the two acts felt different at the time, which is exactly why the
rule exists.

**What the carve-out would dissolve, if it reaches:** section 17's "there is no honest value for the rung"
argument. For a statement he framed as absolute and never retracted, `stated` is honest on the namespace's own
definition, because `stated` means he has said it and marked it canon-bound and it has not been through
convergence. That is a true description of the erasure gate.

**And here is why I am not writing those rows.** Three reasons, and the third is the one that decides it.

The count is his recollection. He said "I remember only two" and worded it as recall, and the row's note
already says nobody should treat that as exact. If the carve-out reaches by reading rather than by his memory,
three candidates qualify and I would be picking them.

Identifying "framed as absolute" cannot be automated, and I established that by trying rather than by
asserting it. I extracted all 101 of op's verbatim blockquotes from the 37 files and ran a word filter over
them. It selected ten. It caught the erasure gate and the container ruling, it **missed `13c`'s standard
entirely**, which carries no absoluteness marker and is one of the strongest-framed things in the archive,
and it flagged an agent's annotation inside a blockquote as op's words. Both directions, one run. So the class
is a reading, and a reading by one expert about which of op's words retain his authority is precisely the kind
of call the two-expert rule and the human gate exist for.

And the third: **this is a genuine ambiguity in op's own words about the standing of op's own words.** That is
not a design fork I can compose arms over and it is not a category question of the kind
`never-ask-which-single-rule-governs.md` forbids. It is one of the narrow things reserved to him.

So it is `question::which_closed_panel_statements_the_absolute_framing_carve_out_reaches`, with four options
that are regions rather than policies, `decider = "op"`, and the candidates named with their framing so he can
answer by reading three sentences instead of an archive. The fourth option is not a compromise: it asks
whether being reached restores authority or only currency, which is a different question from which
statements are reached, and it is the one I would put money on.

**One thing I checked before adding to his queue**, because the standing instruction is to search first: it is
not in `156_checkpoint_nine_the_queue_for_ops_seat.md`, not in `question.toml`, and not in the 78 files of
`.data/op-responses/`, where a control term returns 22 hits so the zero is real.

---

## 5. The sampled law in the suite, and it is fixed

Two commits prefixed every file in both archives with `OLD_`. One checker watches one of them.

`no_living_ledger_cites_the_archive_by_its_dead_name` and
`every_archive_citation_in_the_panel_names_a_file_that_is_there` both key on the literal string `seed/`,
which is a four-file archive inside the live panel. The formalization archive is 203 files at a different
address and no arm reads it. The test names say "the archive", definite article, over a corpus that has two,
which is the totality claim `a-claim-of-totality-names-what-enforces-it.md` is about, sitting in a test name
where it is least likely to be read as a claim.

Seven citations into the closed panel are written in the dead spelling. All seven resolve to nothing. Three
are in living ledgers, which are the files the existing arm exists to protect, including `DROPLIST.md` line 3
and two in `PRIOR_CALLS.md`.

I did not take that on the count. `207_probes/archive_citation_gap.sh` plants a citation of the covered form
in a living ledger, runs the arm, requires it to go red, and restores the file from a copy rather than through
git. It went red. So the arm works and the gap is scope, not function.

The fix and the arm that pins it are in section 7.

---

## 6. What I settled, what I moved, what I did not

**Settled.** The roster is 37, not 23, and the fourteen are named. The absence of op's dead design calls from
`ruling.toml` is correct and has an op-sourced reason, so the dispatch's premise that this is a loss is wrong
for most of the corpus. Five items three documents carry as open are closed by
`ruling::the_d_numbered_decisions_are_dead`. The fused-versus-split reservation was released by op at `127b`,
which corrects `SEED_TALKING_POINTS.md` section 1.13. The standards bound is the worked example of the only
route by which dead material legitimately returns.

**Moved.** Nine items from "unexamined" to "lost with the reason cited or the absence of one stated", and
`why_dropped` is a separate field from `verdict` in the catalogue precisely so "no reason found" can never be
read as "superseded". The carve-out is now a live question rather than an unrun argument. The citation gap is
closed with a control.

**Did not.** Which closed-panel statements the carve-out reaches, which is op's. Whether `143b` should be
re-ratified, which is downstream of that. Whether the erasure gate wants a `ruling` row, same. And I did not
open the 99 numbered member files of the closed panel: my population was op's own words and the member-file
material no consolidation absorbed is a third instrument that `OLD_112` section 7 named and nobody has built.

**What I would attack next, from a different angle than mine.** The registry has never cited
`SEED_TALKING_POINTS.md` at all, in any namespace, which I verified by grep across all twelve files. It is
945 lines of archaeology whose section 1 is literally "op material the current panel's `INTENTS.md` does not
carry", and the port consumed `INTENTS`, `PRIOR_CALLS`, `OPTIONS` and `DROPLIST` and walked past it. My
catalogue works section 1. **Sections 2 through 7 are untouched by anybody**, they are about measured and
refuted claims rather than op material so they were outside my dispatch, and section 3 alone is four hundred
lines of things proved or measured in the archive with the instruments named. That is the next seat's, and it
is a bigger population than mine was.

---

## 7. The fix to the sampled law

Stated separately because it is engineering rather than archaeology.

The two archive arms take the archive's address as a parameter rather than assuming `seed/`, and the corpus
module gains the second archive. The arm that refuses a dead-name citation in a living ledger and the arm that
refuses a dangling `OLD_` citation anywhere both now run over both archives.

The seven citations in section 5 are repointed, since three sit in living ledgers, which are editable by
definition, and the other four sit in member files, which are the record. Member files are not edited to make
a checker green, which is the position the existing module already takes for the same reason, so those four go
under a ceiling that reports rather than an exemption that hides.

The control is the mutation, not the count: the planted citation in a living ledger must turn the arm red, in
both archives, and a planted anchor that does not exist must be rejected while a real one is accepted.
`207_probes/anchors_resolve.sh` carries the second pair and checks all 22 of the catalogue's own citations,
which is the only thing checking them until the arm lands.

---

*Grounded on: op's own words, all 37 op-authored files of the closed panel opened at source, and `206`,
`181`, `95`, `88`, `87`, `85`, `28` and `01` in the live panel. Settled shapes: `OLD_112` in full,
`SEED_TALKING_POINTS.md` sections 0, 1 and 8 in full, `179` sections 17 and 18 in full,
`PRIOR_CALLS.md` sections 0 and 1, `ruling.toml` and `question.toml` in full, the `arvo-checks` suite bodies.
Verified by instrument: `207_probes/roster.sh`, `archive_citation_gap.sh` with a mutation control,
`registry_coverage.sh` with four carried-item controls, `anchors_resolve.sh` with both controls. Canon gate:
`canon_paths` names `mock/registry/*.toml`; this seat writes one question row, one catalogue and four probes
under `mock/research/`, and touches no crate.*
