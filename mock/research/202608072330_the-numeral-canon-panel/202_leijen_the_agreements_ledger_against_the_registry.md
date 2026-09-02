# 202. The agreements ledger against the registry

Porting pass. The answer is that the ledger is very nearly already ported, and the interesting half
is which of its items are not rows and should not become rows.

The short form. `AGREEMENTS.md` holds 100 items across fourteen sections. Op's eighteen intents are
in the registry one to one. The nineteen items in the ledger's own top tier, the five subsections a
source consolidation itself calls independent, are ported at seventeen. All six cross-topic
agreements are ported, and so is section 7. **Two of the nineteen have no row. I wrote one and I
decline the other**, and the two are not distinguished by being about the panel rather than about
arvo, which was my first guess and is wrong: the registry already carries thirty-nine rows at
`topic::panel_conduct` including a measured finding about its own bench corpus.

What the pass turned up that is not about porting: a probe of mine reported thirty-one broken
provenance anchors, and the number was wrong three separate ways before it settled at zero.

---

## 0. What broke

### 0.1 In the brief: the two counts hold, and the description of the ledger does not

`598 rows across 10 namespaces` and `98 tests`: both exact, from `cargo mock --lint-only` and
`cargo test -p arvo-checks` before I touched anything. The porting pass recorded in `179` is real and
I measured its result rather than taking it: `INTENTS.md` carries eighteen entries and `ruling.toml`
carries eighteen rows with an `I` key, `I1` through `I18` with no gap and no extra.

**What does not hold is the ledger's description**, and the brief inherited it from the ledger's own
opening, which says it "reads the four finished consolidations of this panel ... alongside every one
of op's own files". That was true when written. The file now runs to section 13 and covers six
topics rather than four, and **its later sections are a different kind of document**: sections 9
through 13 are pointers rather than compressions, by their own account. Section 9 says so in those
words, and adds that it "postdates section 6's cross-topic pass, which therefore does not cover it".

This matters for the job rather than being a pedantry. **Thirteen of the hundred items are in
sections that decline to compress anything**, so asking whether they have registry rows is asking
the wrong question of them: they point at `122`, `123`, `124` and the other topics' own ledgers,
which are where the claims live and which a different pass would have to walk.

### 0.2 In my own work: a probe that reported thirty-one breaks and should have reported none

`202_probes/do_the_heading_anchors_resolve.sh` was built because the brief tells this seat to cite by
heading anchor rather than by line, and because `mock/checks/src/citation.rs` refuses a line anchor
and does nothing with a heading one. So the half everybody is now told to use looked unchecked.

**The premise was false.** The mockspace engine resolves heading anchors, at
`src/registry/refs.rs:357`, and reports `unresolvable-heading`. I found that by writing a bad anchor
and watching the lint refuse it, not by reading, which is the wrong order.

Then the number moved twice more:

- **Version one reported 31 unresolved of 182.** Reading the detail rather than the count, most were
  apostrophes: `## 3. Warm's headroom rule` is cited `#3-warm-s-headroom-rule`, and neither slug rule
  I had modelled produces that.
- **Version two, with a third rule, reported 30 broken and 1 unwalked**, and I was one paragraph from
  writing that eight of the thirty were into op's own files, which would have been the most alarming
  sentence in this file.
- **The real number is zero of 182**, plus one citation into an archived file my walker never opened.

The engine accepts **two** slug forms and its own documentation says why: a project form where every
non-alphanumeric run becomes one hyphen, and a forge form where punctuation inside a word is dropped,
which is what a browser's address bar shows. **A citation mixing the two resolves under neither**, and
that is exactly what I wrote: `#4-1-` from the project form and `consolidations` from the forge form,
in the row this pass adds. Both pure spellings pass; the mixture is refused, and the error names the
anchor rather than the near miss.

Transcripts kept: `_v1_two_rules.out` for the 31, and the corrected header naming what each version
got wrong.

### 0.3 One thing the engine's own source records, passed on because it is outside this question

`refs.rs`, on the project slug form, in its own words: "a citation written in this project's form
validates green and renders a link a forge will not honour, which accepting a second form at
resolution does not touch. **That is a real defect and it is not fixed here**." So a large share of
the registry's 182 heading anchors validate and render dead. I have not counted which share and it is
not this seat's job; it is worth somebody's.

---

## 1. Method

`202_probes/extract_the_ledger_items.sh` walks the ledger and reports every item with the section and
subsection it sits under, because the tier is what decides whether an item is a row at all. Two item
shapes, since the ledger has two: bulleted items in most sections, bolded paragraph leads in section
6, which is why a bullet-only count reports section 6 as empty. Controls: section 1 must report 24,
counted independently beforehand; section 7 must appear with zero bullets, since an extractor that
omits an empty section cannot distinguish no items from not walked.

`202_probes/which_agreements_have_rows.sh` then asks, for each item in the top tier, whether a
registry row carries it. **It is row-aware rather than line-aware**, and version one was not: it
filtered to matches on the `says =` line and reported five items absent that are present, because the
phrase sat in `because` or `note`. **It is still a net**, the phrases are my choice, and no verdict
below was reached from the table alone. Every one was reached by opening the row.

---

## 2. What is ported

| tier | items | rows | not |
|---|---|---|---|
| op's intents, section 1.2 | 18 | 18 | 0 |
| op's process words, section 1.3 | 6 | covered at `topic::panel_conduct`, 39 rows | 0 |
| top tier: 2.1, 3.1, 4.1, 5.1, 5.2 | 19 | 17 | 2 |
| cross-topic, section 6 | 6 | 6 | 0 |
| contradictions, section 7 | 1 | 2 rows | 0 |
| contested or located, 2.3, 3.3, 4.3, 5.4 | 13 | the `question` namespace, 79 rows | see 4.3 |
| sections 9 to 13 | 13 | pointers, not compressions | see 0.1 |

The seventeen, each confirmed by opening the row: C1 as
`proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`; C2 as
`proposal::membership_in_the_type_and_identity_are_two_criteria`; C3 as
`proposal::membership_of_the_representable_set_is_one_affine_predicate`; C5 as
`proposal::a_min_plus_computation_needs_monotonicity_as_well_as_an_absorbing_top` beside
`probe::absorption_and_associativity_agree_for_clamped_addition`; the additive column's
scale-independence as `proposal::an_additive_verdict_is_independent_of_the_fraction_width`; the
number-system trio as `proposal::the_numeral_concept_is_a_dependent_sequence_of_choices`; the crate
table's worthlessness subsumed by `ruling::prior_calls_are_a_historical_log_not_calls`, which is op's
and is stronger than the item; the correctness-relation claim as `ruling::the_strategy_is_what_makes_an_answer_correct`, which is I9;
R1 as the row whose `says` is R1 verbatim; the band-transfer defeat in `law.toml`; the compile-time
erasure as `proposal::a_compile_time_strategy_selection_leaves_no_residue_in_the_emitted_body`; the
rationalisability counts across a proposal and two probe rows; the F = 0 qualifier as
`law::distributivity_of_multiplication_over_addition`, which cites `35:311` as the line where the
qualifier was first lost; the argmin definition, the non-closed-operator finding and the
points-in-a-product finding as their own proposals; and the bench-region finding as
`proposal::most_committed_bench_regions_predate_the_harness_cross_variant_validation`.

**One of those closes something I left open.** `199` reported that the canon carried absorption and
not monotonicity, and called it the live item. `proposal::a_min_plus_computation_needs_monotonicity_as_well_as_an_absorbing_top`
has since landed, citing `35` section 3.5a by anchor, and says in its own `says` that "having one of
them is not most of the way to having both". That gap is closed.

---

## 3. The one row I wrote

`proposal::the_derived_laws_units_re_runs_reproduced_and_its_defects_were_all_in_blind_spots`.
`kind = "finding"`, `sentence_kind = "argument"`, `standing = "one_expert"`,
`topic = "panel_conduct"`.

**What it says.** Inside the derived-laws unit, five re-run events across six member files reproduced
to the digit, and every defect that unit found was in what an instrument could not see rather than in
whether it reproduced. So a figure from that unit is at risk from its instrument's blind spots and not
from its rerunnability, and those are different risks wanting different checks.

**Why it is a row rather than a note about the panel.** My first verdict was to decline it as a fact
about the panel rather than about arvo, and the registry overruled that before I wrote it down:
`topic::panel_conduct` exists, holds thirty-nine rows, and its own description is "How this panel is
run and how its output is to be read. Not a claim about arvo." The bench-regions row sits there and is
the negative counterpart of this one, discounting every bench figure in the corpus. **The criterion is
not whether an item is about arvo. It is whether it bears on how much a claim in the canon can be
trusted**, and a corpus whose re-runs reproduce and whose defects are all in blind spots is a corpus
whose `measured` rows should be attacked on coverage rather than on arithmetic.

**Standing, and why not more.** Five re-runs by five different members would look like five instances.
It is one: the consolidation that assembled the chain says plainly it "did not independently
re-execute" the sources and that the chain "is the members' work, not mine". So it is one expert
aggregating five reports, and I did not re-execute them either.

**Predicate, and the one coordinate I checked rather than inherited.** `threads: threads = 1`, and the
note says outright that the predicate carries almost none of the claim's scope, because its real
coordinates are one unit, eight probe directories and five re-run events, none of which is a declared
axis. I checked the thread coordinate rather than assuming it: the eight probe directories hold 185
files and none mentions a thread, a spawn or rayon. **That check was not optional**, because the
corpus retired the claim that every instrument in this panel runs on one thread as false at the moment
it was written, so a blanket single-thread coordinate may not be taken from anywhere.

**And the counter-instance is in the row.** A later unit records a re-run that did not reproduce:
twelve of thirteen crates at 108 tests with the thirteenth non-terminating under concurrent load,
against a completed run at 123 across 13. The note carries it, because a row saying "the reproduction
chain never broke" without it would generalise a one-unit result across a panel that has a
counter-example.

---

## 4. What I decline, and why

### 4.1 The corpus's test suite is 123 tests across 13 crates

Ledger section 5.1, filed under "three or more independent instances", supported by five counts.

**Declined, and the reason is not that it is about the corpus.** Section 3 is the argument that
corpus-facing findings are canon when they bear on how a claim is read. This one does not. It is the
size of a test suite belonging to a crate tree that was deleted when the canon work opened, no row
cites it, and nothing in the canon rests on it. The bench-regions row is the contrast: it discounts
figures the canon carries. A test count discounts nothing.

**And it is unpredicated over the coordinate that decides it**, which is the sharper objection and
would apply even if the tree still existed. `133` records the two figures as two runs rather than as a
disagreement: `122`'s `u0` records a completed run at 123 across 13, and `125` section 10 records a
per-crate run reaching twelve of thirteen at 108 with `bitpack-write-contend-shared` non-terminating
under concurrent load, "after 88 CPU-minutes in my run and above fifteen CPU-hours in a sibling
process". **The ledger cannot know this**, because `133` is later than the consolidation it read, and
that is the honest description: not a false item, a stale one whose predicate is missing the load
coordinate. Porting it would put "five independent counts" into the canon over a quantity that
depends on something none of the five varied.

### 4.2 What I did not treat as candidates at all

**Sections 9 through 13**, thirteen items, for the reason in 0.1: they are pointers to other topics'
own ledgers by their own declaration, and porting a pointer ports nothing. Whether those five topics'
ledgers are themselves ported is a real question and it is a different pass.

**Section 2.4, 3.4, 4.4 and 5.5, explicitly closed inside a unit.** These are prose rather than
bullets, which is why the extractor reports zero for them, and they are retirements. `retirement.toml`
already holds the ones I spot-checked. A systematic walk of the closed tier against `retirement` is
owed and I did not do it; I say so rather than let the extractor's zero read as coverage.

### 4.3 The contested tier is questions, and I did not port any of it

Thirteen items across 2.3, 3.3, 4.3 and 5.4. **A located disagreement is not an agreement**, and the
ledger files them separately for that reason. They belong in the `question` namespace or nowhere, and
79 questions exist. I matched by concept rather than by phrase and found the ones I checked present:
the wrap-order fork as `where_wrapping_lives`, the chain-carrier fork as two questions, Q41 on whether
the strategies are partially ordered, and the generate-versus-check fork as the winner-table question.

**I did not walk all thirteen** and I am not reporting the contested tier as covered. What I am
reporting is that the tier maps to a namespace that exists and is populated, so the porting question
for it is a different shape from the one this brief asked.

---

## 5. What is genuinely open

**Whether the closed tier is ported.** Section 4.2. It is the one tier I left with a spot-check.

**Whether the later topics' own ledgers are ported**, which sections 9 to 13 point at and which is
the larger remaining pass.

**How many of the registry's 182 heading anchors render as dead links.** Section 0.3, from the
engine's own recorded defect. All 182 validate; that is a different property from resolving in a
browser, and the engine says so about itself.

**Whether `topic::panel_conduct` should be in the canon at all.** I used its existence as the
precedent licensing the row in section 3, and precedent is the weakest kind of licence: thirty-nine
rows about how a panel is run, in a document whose stated job is what arvo is. Op's `181` says the
canon must be exhaustive enough for a full design and implementation to follow from it, and none of
those thirty-nine serves that. The counter-argument is the one I made in section 3, that a reader
needs to know how far to trust a `measured` row. **Both readings are live and I did not settle it**,
and if it goes the other way my row goes with the other thirty-nine, which I would not argue against.

---

## 6. Probes

| probe | establishes | the case that had to fail |
|---|---|---|
| `extract_the_ledger_items.sh` | 100 items across fourteen sections, each with its tier | section 1 must report 24, counted independently first; section 7 must appear with zero bullets |
| `which_agreements_have_rows.sh` | seventeen of the nineteen top-tier items have rows | a phrase from a `note` field must be found, since the line-aware version one missed exactly those; a phrase in no row must report nothing. `_v1_line_aware` is not kept as a separate transcript because the rewrite was structural, and the header records what it got wrong |
| `do_the_heading_anchors_resolve.sh` | all 182 heading anchors resolve; one citation is into a file this walker does not open | an apostrophe heading must resolve under the project form and not under the forge form, which is the case version one got wrong. `_v1_two_rules.out` kept, and its number is wrong |

Two of the three caught a defect in their own first version, and in both cases the thing that caught
it was reading the detail rather than the count. That is the third time in this arc, and the pattern
is specific enough to state: **a count is what an instrument reports, and the detail under it is what
says whether the instrument could have been right.**
