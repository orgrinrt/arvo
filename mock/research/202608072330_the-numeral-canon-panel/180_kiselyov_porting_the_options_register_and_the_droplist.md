# 180. Porting the options register and the droplist into typed rows

Two namespaces populated: `question`, 78 rows, and `retirement`, 176 rows. Both validate, and the
validation is worth something because I made the checker fail twice on purpose first.

The interesting part of this dispatch was not the transcription. It was that a namespace with a
required `options` field is a predicate over the source, and running it over 2768 lines of register
sorts the entries into two piles: the ones that are questions and the ones that have been living in a
question register without being questions. Twelve numbered entries fall in the second pile. That is
the finding I would lead with, and it is a fact about the register's shape rather than about anyone's
diligence.

## The gates

**Canon gate: passed.** There is no `mock/canon/`, and per `mockspace.toml:31` the canon is
`mock/registry/*.toml`, so writing rows into it is the canon work rather than something beside it. Op's
`87` says the canon is written once at the end and he ratifies that single act, so nothing here is
canon yet and nothing here records an answer. I checked the one thing that could have made this
dispatch illegitimate: whether porting a *prior* panel's droplist into arvo's canon registry
reattaches a dead tier. It does not, and the reasoning is in section 6.

**Test gate.** There is no crate tree and no test suite; `mock/crates/` is empty by design. What stands
in for it here is the registry checker, and I ran the controls on it rather than trusting a pass. Two
fired, one hole is unguarded, and the transcript is at `180_probes/control_runs.txt`.

## 1. What I read, and what I did not

**Read end to end:** `OPTIONS.md` (2768 lines, all 80 headings), `DROPLIST.md` (469 lines, all three
sections), `156` (286 lines), `mockspace.toml`'s registry declarations (lines 300 to 1000 and the
`ref.roots` block), `mock/registry/topic.toml`.

**Read at the sections that carry retirements or options:** `119` 1.3, `122` 7, `132` 1.6 and 1.7,
`136` 2.1, `146` 1.7 and 1.8, `151` 1, 9 and 10, `161` 2, 3, 6 and 7, `164` 5 and 6, `173` 2, 3, 5, 6
and 7, `176` 6, `178` 2 through 10, `53` 4.1, `85` 2, `88` 4, `INTENTS.md` I13's const-time
specification.

**Opened to check one claim each:** `110` (lines 113 and 420), `157_probes/p1b_literal_ties.out`,
`159` F159-1, `78` (line 428), `168` O-168-3, `173` L20 and L21, `132` C1.

**Not read:** the other ~290 files in the panel, every probe directory except the one above, and the
prior panel except its consolidation twelve. That is a real bound and section 13 says what it cost.

## 2. The counts, measured

```
$ grep -c '^\[\[question\]\]' mock/registry/question.toml
78

$ grep -c '^\[\[retirement\]\]' mock/registry/retirement.toml
176

$ grep '^decider = ' mock/registry/question.toml | sort | uniq -c | sort -rn
  45 decider = "panel"
  27 decider = "op"
   3 decider = "measurement"
   3 decider = "coordinator"

$ grep '^kind = ' mock/registry/retirement.toml | sort | uniq -c | sort -rn
 108 kind = "wrong"
  35 kind = "superseded"
  13 kind = "unpayable"
  10 kind = "unpredicated"
  10 kind = "misattributed"
```

Retirement provenance by cited file, measured rather than counted by eye. These are citations and not
rows, because six rows cite two files:

```
$ grep -o 'panel::202608072330_the-numeral-canon-panel::[A-Za-z0-9_]*' \
    mock/registry/retirement.toml | sed 's/.*:://' | sort | uniq -c | sort -rn
  97 DROPLIST
  16 161_leroy_the_canon_candidate_for_the_primitive
  14 173_leroy_the_canon_candidate_for_the_chain
  13 119_leroy_the_canon_candidate_for_the_realisation_map
   9 132_leroy_the_canon_candidate_for_the_rounding_axis
   7 178_leroy_the_restoration_pass
   6 151_leroy_the_candidate_revised_against_four_signatures
   6 146_leroy_the_canon_candidate_for_the_strategy_object
   4 164_leroy_the_candidate_revised_against_two_signatures
   3 OPTIONS
   2 176_leroy_the_candidate_revised_against_two_signatures
   2 122_leroy_the_candidate_revised_against_two_partial_signatures
   1 53_leroy_consolidation_the_container_derivation
   1 136_leroy_the_candidate_revised_against_three_signatures
```

`DROPLIST`'s 97 is 94 from its section 6 and 3 from its closed-by-this-panel section. Of `164`'s four,
two are amendments sharing a row with `161`, which is why the revision's own new entries number two.
The single `136` citation is the corrected attribution on the fabricated-figure row, and the single
`53` is the adjudicating criterion.

The droplist coverage is exact rather than approximate, and I checked it rather than counting by eye:

```
$ sed -n '18,423p' DROPLIST.md | grep -c '^\*\*'
94
```

94 bolded entries in section 6, 94 rows. Every one is ported.

`decider` is the weakest field in the question file and I want that on the record. It is required, and
the source names a decider for only some entries, so I applied one mechanical rule and stated it in the
file's own header: `op` only where the source says the question is his or records him settling it,
`measurement` only where the source names a measurement as what closes it, `coordinator` for the three
calls `156` records as answered in op's stead, `panel` for everything else. Forty-five rows carry
`panel` because the source is silent, not because anybody established that the panel decides them. A
reader treating that column as a finding is reading my default as evidence.

## 3. Twelve register entries are not questions, and this is the report on them

`options` is required, and it is required for the reason the namespace comment gives: an answer
recorded without the choices it was made among is worthless to every later reader. Twelve numbered
entries in the register have no option set that I could state symmetrically, and I did not manufacture
one. Each is named here with why.

```
$ comm -23 <all Q numbers in OPTIONS headings> <Q numbers with a row>
Q17 Q32 Q49 Q51 Q52 Q55 Q56 Q58 Q59 Q60 Q63 Q64
```

They divide into four kinds, and the kinds matter more than the list.

**Findings wearing a question's number.** Q17 (where the fraction boundary falls), Q49 (why the corpus
cannot exhibit three of op's intents), Q51 (what a strategy is, after the pair was attacked), Q52 (what
a primitive is). Each is a paragraph sequence of measured results with a conclusion, sometimes with a
refutation inside it. Q51 opens "It survives as a two-component object", which is an answer. There is
nothing here that is open between anything, and writing two options under them would be inventing a
fork the unit closed.

**How-much questions.** Q55 (what the accumulator width collapse actually costs) says of itself "this
is a how-much question, so an ad-hoc spike cannot answer it". A magnitude has no options; it has an
owed harness run. Q63 (whether any real instance of the composed selector exists) is the same shape as
an existence claim: the honest options are "yes" and "no", which is a degenerate pair that tells a
reader nothing the question did not.

**Owed sweeps.** Q56 (whether two dimensions are missing elsewhere in the panel), Q58 (four predicates
carrying no domain dimension), Q59 (whether the position-keyed arm survives a two-dimensional keying).
Each names a measurement per clause as what closes it. These are obligations with an instrument
attached, not forks.

**Facts about the panel's own instruments.** Q32 (the workload evidence gap), Q60 (whether the
extractor class costs more than it has), Q64 (what a cold derivation's shared inputs cost its
independence). Q32 says of itself, in the source's own words, "Not an option between alternatives. An
evidence gap." Q60's "what would close it" is "Nothing, in the sense of a measurement." Q64 has a
mechanism owed and no alternatives.

**And three of the unnumbered entries go the same way.** The strategy definition ("What a strategy
*is*, as a definition rather than a table") is a proposal awaiting a second read, which is the
`proposal` namespace's business rather than this one's. The necessary condition for a sound
absorbing-top denotation is an owed check with one named candidate and no rival. The seam sentence
between the two vocabularies of "numeral" is a pointer to Q2's fourth reading rather than a question.
So is "the name of the third sense of composition", from `156` item 8: nobody has named a candidate,
and a question whose options are the empty set is not one.

**The locus reading, which is the useful half.** A register whose entry criterion is "a live option"
has accumulated four other kinds of thing, and each of the four has a home the registry already
declares. A how-much question is a `probe` that has not run. An owed sweep is an `obligation`. A
finding is a `proposal`. A fact about an instrument is a `probe` row with a `defect`. None of them is a
`question`, and the reason they ended up in a question register is that the register was the only
artifact the panel had that carried anything forward. That is worth saying plainly, because the
migration will otherwise be read as having lost twelve questions when what it did was decline to
fabricate options for twelve non-questions.

## 4. Q39 is a fork op rejected, and the register still carries it as live

This is the sharpest thing I found and it is checkable in three greps.

`OPTIONS.md` Q39, "Whether an arm's predicate may read data", carries three options: typestate only,
typestate or data, and typestate only for selection with data admitted at an ingest boundary. It
describes its first option as "I13 read literally, since it says const predicates". It cites no answer.

Op answered that fork on 2026-08-13. `INTENTS.md` records it verbatim under I13:

> Let me just add there that the above collapses to whatever is available at const time: Making the
> predicates const expressions for example, allows using const functions and pipe in some data that is
> outside the typestate. However, being const time expressions, typestate is usable there too

and the entry's own gloss says: "It was answering a two-way fork the panel had built, typestate against
values flowing through, and it rejected the fork rather than picking a side."

`88` section 4 then counts the rejections and names this one explicitly: "Before that he declined to
pick a side on typestate against value predicates, saying the axis was const-availability." That is the
third of three questions of the shape `never-ask-which-single-rule-governs.md` exists to stop.

Q39 was added by `80`. `83` is op's answer. The register was never amended, so:

- Option (a) describes itself as the intent read literally, and the intent was sharpened past the
  literal reading in the answer that rejected this very fork. The description is stale in the direction
  that makes the option look better founded than it is.
- The entry reads as an open three-way fork when op has said the axis is const-availability.

**It is not fully dead, and I did not kill it.** `INTENTS.md` is explicit that what the answer does not
settle is "what happens to a condition that is genuinely not const-available", and Q39's subject is the
trajectory predicate, which is exactly that residue. So a question survives, and it is narrower than
the one written down. The row carries all of this in its `note` and states no answer.

**Q43 is the same class and the register caught it itself**, which is the encouraging half: its entry
says outright that it was written as a binary, that the binary was the dispatcher's error, and that a
canon gate corrected it. That correction is in the row.

**Q38 is borderline and I flagged rather than excluded it.** "Where a law verdict's truth is
established" asks one question over the whole category of law verdicts, and `85` section 2 is op saying
"We shouldn't police what kind of laws there are or what shapes they take." It survives here because
what it turns on is a single canon-level willingness to carry a trusted-base item, which is a
canon-form question and therefore genuinely one decision, rather than a rule about the shape any
individual law takes. The row says so and points at the coupling.

## 5. Duplicates, and entries that have silently become two

**Q21 and Q33 are one question.** The Q21 amendment says so in as many words: its stated discriminator
was measured not to cut, and "What remains of Q21 is `67:440-446`'s question, whether the ambient
operation family is fixed at (+, x) or is a parameter, and the two entries should be read as one." I
kept both rows, because each is separately cited elsewhere and merging them would break those
citations, and each row names the other.

**Q61, Q62, `156` item 2 and the tenth unit's normative-sentence question are two decisions wearing
four numbers.** `151` records that the firewall and the operation-set question are one decision. `173`
section 6 records the canon-form question as "coupled to `156` item 2 as one decision". `178` section 7
lists all six reserved items and two of the six are these. Four rows, each naming the coupling. A
consolidation that treats them as four open questions will report the queue as longer than it is.

**Q4's soundness-against-bestness sub-fork and the unasked tightness question are the same fork twice.**
Q4 carries it as "a live sub-fork inside every reading above" and the unasked list carries it as "Is
the derived numeral required to be the tightest honest answer?" with three options. I gave the second a
row and left the first in Q4's `note`, because that is how the source has it, and both rows say so.

**Three entries had genuinely become two and I split them**, marking each split as my call in the row:

- **Q37** is two questions under one label, and the source admits it: "two entries with one subject,
  consolidated with both authors' options intact". The naming half and the ordering half have disjoint
  option sets and disjoint discriminators. Two rows, both keyed `Q37`.
- **Q47** is the reading question and the imitation-covers-the-panic question. They got different
  answers from op on the same day: the first dissolved, the second entered the intent catalogue. Two
  rows, both keyed `Q47`.
- **The accuracy-target item** in `156` item 8 carries "a second unresolved reading in the same
  sentence", whether "chains and ops" names two things or one. Two rows.

**And two entries carry a second question I could not give a row.** Q2 carries, inside itself, `24`
section 9's question of whether the design admits numerals its coordinates cannot name, which `24`
names as op's own and which is separate from and prior to the surface-pair choice. No options are
stated for it anywhere I looked, so it has no row and it is named here instead. Q4 carries the
necessary-condition question, which is in the unasked list and is excluded above for the same reason.

## 6. On porting a dead panel's droplist into a live canon registry

`DROPLIST.md` section 6 is 94 entries extracted from the predecessor panel at the moment it closed,
and most of them retire claims about a crate tree that has since been deliberately deleted. Facade
migration routes, `Bits` and `UFixed` call sites, the width chain's second spelling of zero. Under the
canon-design-code chain, code is the tier that gets nuked, and a retirement of a claim about nuked code
looks at first like a fact about a dead tier being reattached.

It is not, and the distinction is worth stating because the next reader will hit it too. A retirement
row is not a claim about the code; it is a claim about a *sentence*, and the sentences are still in the
tree and still greppable and still get cited. Several of them are cited by the current panel: the
generic-const-expression walls, the value-uniqueness findings, the tightness result. The namespace's own
preamble settles it: "Nothing is deleted for being wrong later. A retired claim is part of the record of
how the thinking went." So they belong here, and the ones about the deleted tree belong here most,
because those are the ones nobody can check against source any more.

`DROPLIST.md` section 7 is the one thing I deliberately did not port, and I want the reason on the
record rather than discovered as an omission. Its own preamble says these are "proposals born and
retired inside the stretch that absorbed them", that they "are not removals from a standing base", and
that mixing them with section 6 "is what made the droplist read as a standing record while reporting on
nothing that left it". They are recorded in one paragraph, in brief, without individual claim
sentences. A `claim` field demands "the sentence itself, or close enough that a grep finds it", and
there are no sentences to take. Porting them would mean writing 15 claims nobody wrote, for material
the ledger says never left the base, so nothing cites it and nothing needs stopping.

## 7. Where I checked a retirement at source and found the retirement wrong

The dispatch was right to ask for this and it was right about which one to worry about.

**`132` section 1.7's own attribution of the 21,204 figure is false.** The entry retires
"21,204 of 32,768 signed negative cases at `W = 8`" and says "The figure originated in `125`". It did
not.

```
$ grep -rn '21,204\|21204' .
```

returns eleven hits and none of them is in `125` or either of its probe directories. `133` says so
directly ("The figure was never in `125` or in any committed artifact"), `136` section 2.1 corrects the
candidate's own wording ("The candidate's wording should read: originated in `125`'s author's report
message, outside any committed artifact"), and `137` reproduces the grep. So the entry whose whole
purpose is to stop a fabricated figure being cited **misattributes it to a file that refutes it**, and
a reader following that attribution lands on `125`'s committed record, which is correct throughout at
64, 112 and 124 of 256.

The row carries the corrected attribution and both impossibility arguments. I put the correction in
`why` rather than filing a second retirement of the retirement, because the entry's substance is right
and only its provenance sentence is wrong; the class of error is the one `161` R8 names, where a
correction that reads as a refutation retires a true finding.

**Four other retirements I checked at source and found accurate**, so the ledger is not generally
unreliable and I do not want this report read as saying it is:

- `161` R1's "0 of 48". `110` line 420 and line 560 both carry it, in the form the retirement quotes.
- `161` R12's quoted sentence. `110` line 113 reads, verbatim, "This workspace auto-loads
  `arvo-always-optimal-internals.md` into every agent context." The retirement calls it false as
  stated. I can add one independent instance from this seat: that file exists at the workspace root's
  `.claude/rules/` and is **not** in this repository's own generated `.claude/rules/` (17 files), and it
  was not in the rule set loaded into this dispatch's context. One counterexample is enough against an
  "every", so the retirement holds and now has a second witness.
- `164` R17's "three separate and three do not". `157_probes/p1b_literal_ties.out` shows six non-grid
  literals, three marked SEPARATES and three not, with both controls passing, and the tie case named in
  the output's own closing paragraph.
- `161` R2's "three, not five". `159` F159-1 states the four packed-end crates are one dependency
  family.

**And one stale citation, which is not a retirement defect but is in the same file.** `DROPLIST.md`
line 3 says it was extracted from `202607301300_formalization-spec-panel/124_consolidation_twelve.md`.
The file on disk is `OLD_124_consolidation_twelve.md`; the unprefixed name resolves to nothing. Every
droplist row's second provenance entry uses the real name, so the registry is correct and the ledger's
own sentence is not. Cheap to fix and not mine to fix, since `DROPLIST.md` is not a file this dispatch
owns.

## 8. Topics I needed and did not have

Six rows carry no `topic` because nothing in `topic.toml` fits and the field is optional. I would
rather leave it empty than file a row under a topic that is merely nearby, because a wrong topic is
worse than an absent one: it makes "everything the canon says about X" return a row that is not about X,
which is the exact query the namespace exists to serve.

What is missing, in the order I would add it:

- **`naming_and_vocabulary`.** The largest gap. It would carry Q46 (the cross-repo `Strategy`
  collision), the phase collision across the two vocabularies, the rounding mode vocabulary, the chain
  against region call, and the third sense of composition's name. Five questions and at least three
  retirements. Naming calls are a real subject here, they are consistently op's, and they currently
  have nowhere to live.
- **`the_width_surface`.** Q9 and the container derivation's outputs both sit under `the_format`
  because that is the least wrong choice. The subject is the crossing from a written const to a
  type-level natural, which is neither what a numeral's format is nor how a numeral is realised onto
  storage, and it produced its own thread of six compiled arrangements.
- **`the_bench_corpus`.** Q45, Q48, Q49, Q63 and several retirements are about what the committed bench
  corpus can and cannot express, which is a fact about the panel's instruments rather than about arvo.
  `panel_conduct` is close and is described as being about how the panel is run, which is not the same
  thing.
- **`doability_and_workload_evidence`.** Q32's subject: whether a shape shown to work at one thread has
  been shown to work. It is filed under `panel_conduct` in the one retirement that touches it, which
  undersells it, since the entry itself calls it the sharpest open problem the canon faces.

I also want to flag that `the_strategy_axis` and `the_strategy_object` are doing a lot of work between
them and the boundary is not obvious from their `what` sentences. Thirteen questions went to the first
and one to the second, which is lopsided enough that I suspect I put some in the wrong one.

## 9. Where the schema fought me

**`retirement` has no `note` field**, and it is the only namespace here without one. Three things had
nowhere else to go and are now inside `why`: a reopen condition (the closed-by-this-panel entries all
carry one, and the droplist's own rule is that an entry states what would have to be overturned), a
qualification that is neither the reason nor the replacement, and the item's location inside an
enclosing section. The last is the one that matters: `DROPLIST.md` has four headings for 94 entries, so
every droplist row cites the same anchor and `claim` is what locates it. That works, because `claim` is
required to be greppable, but it works by accident rather than by design.

**`retirement` has no `key` field either**, where `question` and `ruling` both do. The candidates number
their retirements (`R1` through `R18`, `R-a` through `R-o`) and those labels are cited across files;
`164` section 5 is entirely about amending `R10`, `R16`, `R17` and `R18` by name. I put the labels in
`keywords` so a grep finds them, which is a workaround rather than a fit.

**`kind`'s five values do not cleanly cover a refused proposal.** A large fraction of the droplist is
not "a claim that turned out false" but "a shape somebody proposed and the compiler or a measurement
refused". I mapped those to `wrong` where the ground was measured false and `unpayable` where the thing
cannot be delivered, which is why `wrong` is 108 of 176. That number reads as a corpus riddled with
falsehoods and it is really a corpus with a lot of closed routes. A sixth value, `refused` or
`closed_route`, would separate them and would make the count mean something.

**`decider` being required forces a claim on 45 rows where the source makes none**, covered above.

**The heading anchor rule in the brief is not the rule the checker implements.** The brief says "forge
slug form: lowercase, spaces to hyphens, punctuation dropped". Punctuation is not dropped: a run of
non-alphanumeric characters collapses to one hyphen, so an apostrophe becomes a hyphen and
"the derivation's outputs" is `#the-derivation-s-outputs`, not `#the-derivations-outputs`. I found this
by generating all 80 `OPTIONS.md` slugs under the documented rule and feeding them to the checker: 14
failed, all 14 containing an apostrophe. The corrected rule is in `180_probes/slugs.sh` with the
reasoning in its header, and it is worth keeping because every future porting dispatch will hit it.

## 10. The control runs

Full transcript at `180_probes/control_runs.txt`. Summary:

**Control 1, a heading anchor naming no heading.** One citation mutated,
`#6-the-droplist-cumulative` to `#6-the-droplist-cumulative-BROKEN`. Fires:
`ERROR [unresolvable-heading] ... names heading '6-the-droplist-cumulative-BROKEN', which
202608072330_the-numeral-canon-panel/DROPLIST does not contain.` This is the check the whole citation
discipline rests on and it works.

**Control 2, a citation naming no file.** `DROPLIST` to `DROPLIST_NO_SUCH_FILE`. Fires:
`ERROR [unresolvable-provenance] ... matches no file under root 'panel'.`

**And the control caught my own instrument first, which is the part worth reporting.** My first attempt
at control 1 used `sed -i '0,/re/s//../'`, which BSD sed does not support. It changed nothing, the run
reported "all lints passed", and that is indistinguishable from the checker being silent about a broken
anchor. I only knew because I expected red and got green. Every run in the transcript therefore prints
the substitution count before the result, and I would recommend the next person porting into this
registry do the same: a control that silently does not mutate is a control that certifies whatever you
wanted.

**One hole, and it is unguarded.** `mockspace.toml:316` says a line citation into a living ledger is
"checked rather than declared: `mock/checks` refuses a line citation into a living ledger by name".
`mock/checks/` does not exist in this repository. I put a row citing `panel::...::OPTIONS::57`, a line
number into the most-edited living ledger in the panel, and it passed clean. `mockspace.toml` names
`mock/checks` three times, at lines 316, 404 and 1066, for three different checks, and none of them
exists. That is a claim of totality in the config with nothing behind it, and it is the exact shape
`a-claim-of-totality-names-what-enforces-it.md` names: the sentence reads as a guard and the guard is
absent. Every citation in both my files is a heading anchor, so nothing I wrote depends on it, but the
next author who reaches for a line number into `OPTIONS.md` will not be stopped.

## 11. What is not ported, enumerated so nobody has to rediscover it

Two bodies of material sit outside what this dispatch was sent for. I did not port them, and I am
naming every one so the next dispatch starts from a list.

**Live options in the candidates that are in no register.** The register was last extended for the
strategy-set topic and stops at `152`. Everything after is in the candidates only:

- `161` section 2, contested items X1 through X4, of which X1 and X2 are op's premise decisions.
- `161` section 7, live options O-A, O-B, O-C, O-E, plus Q157-C and Q157-E.
- `164` section 6, option O-S13, entered there specifically because it was "the unresolved proposal
  with nothing for a compressor to grip".
- `173` section 2, contested X-A through X-F.
- `173` section 7, live options O-1 through O-8.
- The contested sections of the earlier candidates: `119` 1.2, `132` 1.6 (C1 through C4), `146` 1.7.

That is the loss mechanism `RULES.md` records three times, happening a fourth time: an unresolved option
has no result attached, so nothing grips it, and it falls out. I ported `156`'s items because `156` is a
named source, and several of those items are these options seen from op's side, so the two lists
overlap and neither contains the other.

**Retirement-shaped material outside the candidates.** Every one of these carries closed routes with
diagnostics, in the droplist's own register:

```
06 §8, 07 §7, 08 §7, 13, 18 §6   Routes closed, each with the thing that closed it
100 §10b, 101 §9, 102 §8, 108 §10  Shapes found and not taken, with what closed each
103 §8                             What I tried that did not work, with what closed each
93 §15, 94 §2, 109 §16, 154 §P2.1  What I withdraw
106 §13.3                          Options this file closes, with what closed them
89 §7                              Two routes closed
```

Fourteen files, and I would guess two hundred entries. I ported `53` section 4 out of this set because
it retires the criterion that adjudicated a whole unit's verdicts, which is load-bearing in a way the
others are not, and left the rest. A dispatch that wants them should budget for a full second pass, and
should note that these are mostly *routes* rather than *claims*, which is the `kind` gap in section 9.

## 12. What I would tell the next reader to check first

**One: whether Q39 should be closed.** Section 4. It is the only entry I found where the register
contradicts a recorded answer of op's rather than merely lagging it, and the residue that survives is
worth stating cleanly because `INTENTS.md` names it precisely: what happens to a condition that is
genuinely not const-available. That is a real question and it is smaller than the one written down.

**Two: whether the twelve non-questions get rows somewhere else.** Section 3. If they simply vanish
from the migration, the queue looks twelve items shorter and four owed measurements disappear with
them. `obligation` and `probe` are declared and empty, and at least eight of the twelve belong in one
or the other.

**Three: the four coupled canon-form questions.** Q61, Q62, `156` item 2 and the tenth unit's
normative-sentence question. Two of them are one decision by `151`'s own reading and two more by
`173`'s. If op is asked four times what is two questions, the answers will not obviously compose.

**Four: `decider` on the 45 default rows.** It is the field an implementer will act on and it is the
field I had least evidence for. A pass by somebody who has read the units rather than the register would
move a good number of them, and moving one is cheap because nothing downstream is keyed on it yet.

**Five: the missing `mock/checks/`.** Section 10. Three declared checks, none present. The line-citation
one is the load-bearing one for a registry whose entire provenance discipline is citations into a tree
that is half frozen and half living.

## 13. What I would have done differently

I ported `DROPLIST.md` section 6 as 94 flat rows in one pass, and about fifteen of them are entries
whose claim sentence I had to reconstruct from a bolded lead that was written as a noun phrase rather
than as a sentence ("A pushed, registered build-layer manifest", "Route X for the facade migration").
Those rows are greppable on the words the ledger used, which is the bar `claim` sets, but they are not
sentences somebody would meet in the wild, because the thing somebody would meet in the wild is the
*proposal* rather than the ledger's name for it. Finding the original proposal would mean reading the
prior panel, which I did not do. If a later reader finds a retirement whose `claim` does not match what
they were looking at, that is the class, and the repair is to widen the claim from the establishing
source rather than to doubt the retirement.

I also state plainly that I did not verify 171 of the 176 retirements against their sources. I checked
five, found one wrong, and that ratio is not reassuring. A one-in-five error rate on a must-not-cite
list would be serious, and one-in-five is what a sample of five gives you with one hit; the honest
reading is that the sample is too small to say anything except that the class exists and is not
hypothetical.
