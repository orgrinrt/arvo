# What of the canon was read, to what depth, and what was read beside it

This is the reading record of job `arvo-canon-check`, taken at `origin/dev` tip
`2c8e5343` on 13 November 2026. It says what was read, by whom, and to what
depth, so that every claim in `CHECK.md`, `ANSWERS.md` and `PLAN.md` beside it
can be checked against a named source rather than taken on trust.

The registry is the canon, and all fifteen of its files have now been read in
full by the seat, in two passes. The first pass read nine in full, the dimension
file in part, and five as an index: the two large proposal files, the questions,
the retirements and the probes. The first version of this file opened by saying
the registry was read whole, which its own depth table contradicted, and a
review caught it. The second pass, after that review, read the six remaining
files row by row, every field, and what it found is `CHECK.md` section 1.10 and
the amendments section 7 lists. The depth table below keeps both passes apart,
because the findings of the first three documents were written on the first.
Nothing in the registry was read by a sub-agent. The designs under `mock/` were
read by the seat, four of the five crate designs whole and the format crate's in
part, as their table says. The corpus around them, the research notes, the closed rounds' topic files,
the committed bench findings and the old arvo at the
revision hilavitkutin pins, is large enough that a full reading of it costs more
than this job has, so it was transcribed in batches by sonnet sub-agents, each
told to quote and never paraphrase, with their output committed beside this file
as `ledger-<group>.md`. The last section says exactly which batches ran and which
did not, because an unread batch is a hole in every absence claim made here.

Two things about depth are worth stating before the tables. A row read in full
means its `says`, `because`, `quote`, `note`, `gap` and every edge field were
read. A row read as an index means its id, its tier field and the opening of its
`says`, which is enough to know what the row is about and not enough to quote it.
Where a finding in `CHECK.md` rests on an index reading, the row was opened in
full before the finding was written, and the finding says so.

The second thing is the citation form. Rows are cited by namespace and slug
(`ruling::the_format_spine_is_canon`), never by line, because the registry gains
rows constantly and a line citation keeps resolving while naming something else.
The repo's own lints refuse the line form.

## The registry

Every file under `mock/registry/`. The counts are mechanical, taken at
`2c8e5343`.

| File | Rows | By tier |
|---|---|---|
| `ruling.toml` | 104 | ratified 39, stated 59, open 5, in_force 1 |
| `proposal.toml` | 104 | one_expert 74, two_experts 19, three_or_more 5, cross_topic 6 |
| `proposal-the-later-topics.toml` | 23 | one_expert 18, two_experts 4, three_or_more 1 |
| `proposal-the-value-stance.toml` | 24 | one_expert 24 |
| `proposal-the-value-type.toml` | 5 | one_expert 5 |
| `proposal-the-exact-operations.toml` | 3 | one_expert 3 |
| `question.toml` | 110 | panel 86, op 13, measurement 8, coordinator 3 |
| `retirement.toml` | 196 | wrong 122, superseded 36, misattributed 14, unpayable 13, unpredicated 11 |
| `probe.toml` | 116 | sound 105, uncontrolled 8, defective 2, withdrawn 1 |
| `obligation.toml` | 17 | no tier field |
| `law.toml` | 18 | no tier field |
| `law-the-later-topics.toml` | 6 | no tier field |
| `dimension.toml` | 25 | no tier field |
| `topic.toml` | 20 | no tier field |
| `strategy.toml` | 4 | prior_attempt 4 |

Of the 39 ratified rulings, 18 carry no `ratified_by`, which the schema calls
required on a ratified row. They are listed in `CHECK.md`.

By depth and by pass, since the files differ:

| File | First pass | Second pass |
|---|---|---|
| `ruling.toml` | every row in full | not reread |
| `obligation.toml`, `strategy.toml`, `topic.toml` | every row in full | not reread |
| `law.toml`, `law-the-later-topics.toml` | every row in full | the gaps naming an undeclared axis reread |
| `proposal-the-value-stance.toml`, `proposal-the-value-type.toml`, `proposal-the-exact-operations.toml` | every row in full | not reread |
| `dimension.toml` | header and every id and `what`; nine rows in full | every row in full |
| `proposal.toml`, `proposal-the-later-topics.toml` | every row's id, standing, topic and the opening of its `says`; about a dozen rows in full | every row in full, header and block comments included |
| `question.toml` | every row's id, decider, topic, `answered` and `bound`; one row in full | every row in full |
| `retirement.toml` | every row's id, kind and claim | every row in full |
| `probe.toml` | every row's id and standing | every row in full |

The second pass ran on the branch that carries this file, whose registry is
`2c8e5343` plus this job's own rows and notes.

The question table is the one derived artifact of this reading worth naming: for
each of the 110 questions it records the decider, whether the row carries an
`answered` field, and every ruling and proposal that names it in an `answers`
edge. 62 questions carry no answering edge of either kind. All 13 of op's own
carry one or an `answered` field, so no question the registry reserves for him is
sitting unanswered in the record. None of the 8 measurement questions and none of
the 3 coordinator questions is answered.

## The designs

Read by the seat, at `2c8e5343`.

| File | Lines | Read |
|---|---|---|
| `mock/PRINCIPLES.md.tmpl` | 363 | whole |
| `mock/DESIGN.md.tmpl` | 119 | whole |
| `mock/crates/arvo-numeral/DESIGN.md.tmpl` | 365 | whole |
| `mock/crates/arvo-strategy/DESIGN.md.tmpl` | 189 | whole |
| `mock/crates/arvo-format/DESIGN.md.tmpl` | 2122 | every heading, and the sections on the spine, the adaptation, the standards and the closing inventory in full |
| `mock/crates/arvo-placement/DESIGN.md.tmpl` | 268 | whole, on the second pass |
| `mock/crates/arvo-bits/DESIGN.md.tmpl` | 130 | whole, on the second pass |

Source was read where a design claim turned on what the tree does: the whole of
`arvo-numeral/src` except its tests, `arvo-format/src/standards.rs`, and greps
for unstable features and for an IEEE 754 name across every crate. Those readings
are quoted in `CHECK.md` and `ANSWERS.md` at `file:line`, which is the permitted
form for source.

Hilavitkutin was read at `c42d707` for two things only: the revision of arvo its
`mock/Cargo.toml` pins, `1b2796e9`, and a count of the old arvo names its `.rs`
files under `mock/crates` still import, which `PLAN.md` quotes. Occurrences were
counted and no use site was read.

## The corpus around the canon

Transcribed in batches. Each entry in a batch ledger carries the file's date, who
wrote it where that is stated, what it holds in one or two sentences, and the
lines worth quoting, verbatim.

| Group | Files | Covers | Ledger |
|---|---|---|---|
| `old` | 262 | the old arvo at `1b2796e9`, the revision `hilavitkutin/mock/Cargo.toml` pins: all sixteen crates, their designs, readmes, backlogs, public items and test counts | `ledger-old-01.md`, `ledger-old-02.md` |
| `topics` | 236 | the closed design rounds' topic files | `ledger-topics-01.md`, `ledger-topics-02.md`, `ledger-topics-03.md` |
| `rest` | 117 | the research notes outside the two panels, including the November value-stance and datum reads and the bench-harness findings | `ledger-rest-01.md`, `ledger-rest-02.md` |

The seat spot-checked one file of each batch against its entry and found no
paraphrase in place of a quotation. Which files it opened was not written down,
so the spot check cannot be repeated from this record and is worth what an
unrecorded check is worth. The `old` batches also carry no date per file, since
every file in them is one tree at one revision, `1b2796e9`.

One reading in the table above failed where it was claimed in full. `law.toml`
is listed as read row by row, and the first version of `CHECK.md` still proposed
moving a cell into `holds` that three existing rows already carry. The file was
read again for the correction, and the miss is the reason the depth of every
other file is stated here without rounding up.

## What was not read

Named because an absence claim is only as wide as the reading under it.

The two panel corpora are unread: `mock/research/202608072330_the-numeral-canon-panel`
(498 files) and `mock/research/202607301300_formalization-spec-panel` (320 files).
They are the argument behind the canon rather than the canon, and the registry
carries their conclusions as rows with `provenance` citations into them, which is
what was read instead. The cost of reading them whole is several times this job's
budget. The consequence is stated plainly: where this job says the registry holds
no row on a subject, that is a claim about the registry, every file of which has
been read in full, and not about the panel files. A sentence of that kind written
on the first pass was rechecked on the second only where `CHECK.md` section 1.10
or `PLAN.md` says so.

Also unread: the 254 committed bench findings under `mock/benches`, of which the
five November ones were read through
the `rest` batches; `mock/research/INTENTS.md` as a file, though every intent it
carries reaches the registry as a `ruling` row with its `key`, and those were read
whole; and the bodies of the test files in every crate, old tree and new, which
were counted rather than read.
