# 182. Porting the four consolidations, and the eleven region statements they contain between them

**Canon gate: passed.** I checked the assignment against `mockspace.toml`'s declaration of
`mock/registry/*.toml` as `canon_paths`, against the `proposal` and `law` namespace declarations at
`mockspace.toml:658-735` and `:1012-1118`, against `INTENTS.md` for I13, and against `RULES.md`. Porting a
panel's established claims into typed rows is what the `proposal` namespace exists for and its own
`description` says so: *"A claim the panel established and op has not seen. It is canon only once a ruling
stamps it."* Nothing here is stamped, nothing claims to be, and no `ruling` row was touched.

**Test gate: run, and it is red on purpose after this pass.** Baseline before I wrote a row: 42 checks, all
green. After: 41 green and one red, `the_committed_canon_has_an_instrument_behind_every_measurement`,
naming seventeen proposal rows by id. Section 9 is why that is the correct end state and not a defect I left
behind, and section 11 has the run. I read the body of every test in `mock/checks/tests/`, not the names.
Nothing in that suite is tautological, sampled where a matrix was available, or assertion-free; the
conditional-obligation file plants both directions of every arm it checks, including the pair that must be
silent, which is the check most suites skip. There was nothing to refuse on.

The deliverables are `mock/registry/law.toml` and `mock/registry/proposal.toml`, and the instruments are in
`182_probes/`, committed as they ran rather than at the end.

---

## 1. The headline, before anything else

**Across the four consolidations, 4,333 lines of them, there are eleven statements of a region in the
panel's own notation. Nine are in one file.**

```
$ 182_probes/region_notation_census.sh
file                                                     holds for Predicate    total
63_spj_consolidation_the_format_concept.md                      0        0        0
74_giesen_consolidation_the_number_system_concept.md            0        0        0
90_giesen_consolidation_derived_algebraic_laws.md               0        2        2
106_giesen_consolidation_the_strategy_axis.md                   8        1        9
AGREEMENTS.md                                                   0        0        0
```

With both controls run against the same five files, because a zero from a grep is a claim about the grep
until you have shown the grep can produce a non-zero: a pattern that must find nothing found nothing in all
five, and a pattern that must find something found 535, 640, 478, 779 and 441.

Of the eleven, **two are not region statements at all**. `90:564` is a sentence *about* a predicate rather
than one, and `106:1356` is a quotation restored from a member file rather than the consolidation's own.
Of the nine that remain, **three state only bench parameters** (`106:292`, `106:374`, `106:596`: how many
regions a table had, how many arms competed, which committed CSV supplied the costs), and one is a
toolchain and target description with one bench parameter inside it (`106:318`).

**So the four consolidations between them contain four regions a design arm could be gated on**:
`106:324`, `106:460`, `106:530`, and `90:153`. One of the four, `106:1356`'s restored sibling, turns on an
axis nothing declares.

This is not a criticism of the four authors and it is not news to them; three of the four say in their own
coverage sections that they are compressing work whose predicates live in member files. **It is the
measurement the port was for.** The registry demands a region on every established claim, the corpus was
written before that demand existed, and the arithmetic of the two meeting is what section 5 enumerates.

**What I could not do because of it is larger than what I could.** 73 proposal rows landed. 43 of them
carry no region, and every one of those 43 is `normative`, which is the one `sentence_kind` the checker
exempts. **Only 30 rows in the whole file carry a region**, and section 8 is where I say plainly how much
of that 43 is honest and how much is a label doing work a region should have done.

---

## 2. The counts, with the command that produced each

Every number below comes out of `182_probes/counts.sh`, committed, and its output is at
`182_probes/counts.txt`. I ran it rather than remembering it, because a figure in an accounting paragraph
that nobody computed reads as measured and is a belief about one's own document.

```
$ 182_probes/counts.sh

=== rows per namespace, whole registry ===
dimension      16
law            14          <- this pass
proposal       73          <- this pass
question       78
retirement    176
ruling         75
strategy        4
topic          20

=== proposals by sentence_kind ===
  43 normative
  17 measured
   6 argument
   5 theorem
   2 enumeration

=== proposals by standing ===
  48 one_expert
  15 two_experts
   6 cross_topic
   4 three_or_more

=== proposals by kind ===
  47 answer
  23 finding
   3 refusal

=== predicate carried, against not carried ===
rows with a predicate:    30
rows without a predicate: 43
normative rows:           43

=== proposals by topic ===
  18 the_number_system      12 algebraic_laws          9 the_strategy_axis
   6 the_format              6 the_chain               5 the_strategy_object
   4 validation              4 panel_conduct           4 binding_time
   2 the_predicate_notation  1 the_realisation_map     1 canon_form
   1 arvo_identity

=== law rows ===
rows: 14   with holds: 11   with fails: 13   with a witness: 12   with a gap: 12

=== citations, by shape ===
line citations into numbered panel files: 149
heading anchors into living ledgers:        8
distinct source files cited:                6
    46 63_spj_consolidation_the_format_concept
    38 90_giesen_consolidation_derived_algebraic_laws
    38 106_giesen_consolidation_the_strategy_axis
    26 74_giesen_consolidation_the_number_system_concept
     8 AGREEMENTS
     1 108_lamport_the_pair_attacked
```

**The `43` and the `30` are the two numbers to stare at.** 59% of what I could port carries no region, and
the only reason those rows exist at all is that `normative` exempts them.

**Axis usage, and the four axes nothing this pass wrote ever names:**

```
=== predicate entries, by axis, across proposal and law ===
  45 operation      41 total_width     40 fraction_width   39 signedness
  39 overflow_policy 31 arity          12 threads          10 target_features
   6 rounding        6 chain_length     5 build_profile     2 container

=== declared axes never used by any row this pass wrote ===
  integer_width    alignment    access_pattern    strategy
```

`integer_width` is expected: the corpus writes `W` and `F` and the `dimension` row for `integer_width`
already says the two spellings are not independent. **`strategy` being unused is the one worth reporting.**
Sixteen axes are declared, one of them is the strategy axis, and after porting a whole topic *about* the
strategy axis not one row states a region over it. The reason is in that dimension's own `note`: `S any`
quantifies over a set op has stated is open, and the corpus names no individual strategy in any predicate,
so there is nothing to write.

**`threads` appears on 12 predicate entries out of 276.** Section 7 is what that costs.

---

## 3. What I read exhaustively, and what I did not

**Read in full, every line:** `63`, `74`, `90`, `106` including its sections 16, 17 and 18; `AGREEMENTS.md`
sections 1 through 8; `mockspace.toml`'s reference-root declaration at `:304-327` and its `proposal`,
`law`, `probe`, `topic` and `dimension` namespace declarations; `mock/registry/dimension.toml` and
`mock/registry/topic.toml`; every test file under `mock/checks/tests/`; `checks/src/predicate.rs`; and
the two worked examples `179` and `180` with `180_probes/slugs.sh` and `180_probes/control_runs.txt`.

**Read in the part the assignment turns on:** `108`, section 7 only, plus its heading census. `106`
section 18 states in terms that its own section 4's rendering of the pair is superseded and that *"whoever
writes the canon takes it from there rather than from here"*, so a port that carried only `106`'s version
would carry the version its own author disowned. I did not read `108`'s sections 1 through 6, so my account
of *why* each of the five clauses needed repair is `106` section 18's account of `108` rather than `108`'s
own, and I have marked the row accordingly.

**Not read at all:** `54`, `75`, `91`, `107` as documents. I opened none of the four entailment checks. Where
`106` sections 16 and 17 report what `107` found, I carried `106`'s report of it and said so. That is a real
limit: `107` found four one-file results absent from `106`'s droplist and three further defects that stand
unrepaired, and I know those seven only through the file they were found in.

**Not read, and named because their absence shapes the port:** every numbered member file (`08` through
`62`, `65` through `73`, `76` through `89`, `93` through `105`); `OPTIONS.md`, `DROPLIST.md`, `RULES.md`,
`PRIOR_CALLS.md`, `INTENTS.md` beyond checking I13 exists and what it says.

**Probe directories: not read on the first pass, read on the second, and the change is section 5.2.** The
first pass treated "no probe directories" as a coverage boundary, wrote five claims off as unportable
because their consolidations state no region, and was wrong about all five. On the second pass I opened
`56_probes/q1` and `q2`, `57_probes/p6`, `60_probes/p_d`, `62_probes/p4`, `80_probes/p4` and
`82_probes/p1b`, each in full, and every one states its own parameters. **Those seven are read; the rest
of the panel's probe directories are not**, so the same recovery is available on claims I did not chase.

**The brief's instruction was to derive a predicate from an instrument's own stated parameters where the
consolidation states none, and a probe directory is where an instrument lives.** Reading that as
"where the consolidation reports the parameters" is what cost the five rows, and it is a reading a canon
writer working from the consolidations would make by default, which is why section 5.2 is written the way
it is rather than quietly corrected.

**Built the reading list with a grep rather than from memory**, per the standing instruction. For each
claim's vocabulary I grepped the panel root before deciding nothing else bore on it. That is how `108`
entered the list: `grep -n '108' 106_*.md` returned section 18 pointing at it as superseding. **It is also
what should have caught the probe directories on the first pass and did not, because I grepped for each
claim's vocabulary and never ran `ls` on the directory named beside it.**

---

## 4. The method, stated so the rows can be checked against it

Three decisions govern every row and they are the whole of the porting judgement.

**Where a source states a region, it is carried verbatim, minus bench parameters.** `106:530` goes into
`no_total_join_exists_over_the_observable_axes_so_the_operation_reports` word for word except for the
clause naming which laws the probe enumerated, which is an instrument parameter. `90:153` goes into
`a_law_stated_as_an_author_written_marker_is_checked_by_nothing` complete.

**Where a source states none but names its instrument's parameters, the region is those parameters.**
`63`'s cube states its own axis values in the table (sign domain, operation, policy, two fraction columns)
and section 7 states the widths, so the law rows carry those and nothing more.

**Where neither is available, no row is written.** That is section 5, and it is the largest section here.

**The test for a bench parameter is gateability, and it comes straight from I13:** a predicate exists so an
arm can be built over the region it names, so `fraction_width: 0` is an axis and `arms = 5` is not. Applying
it stripped `regions`, `arms`, `cost coordinates`, `cost source`, `files`, `producing commits`, `seeds`,
`400 random models` and `values exhaustive` out of the predicates that carried them. **Four rows lost
almost their whole stated region to that test** and each says so in its own `note` in the words *"the
predicate carries almost none of this claim's scope"*.

---

## 5. The claims that cannot be predicated

**The headline output, and it is longer than the list of rows I wrote.** Each entry names why, in one of
three ways, and the three are genuinely different problems with different fixes.

### 5.1 The discriminator is an axis no `dimension` row declares

**These cannot be written at all, not merely written weakly.** The cell that holds and the cell that fails
are identical on every declared axis, so a row would put one region in `holds` and the same region in
`fails`, and no reader could tell which governed.

**R7, sign uniformity of a declared operand window** (`90:307-339`). The derived-laws unit's cleanest
positive result: a declared window `[LO, HI]` with `LO >= 0 || HI <= 0` matches associativity on the
window's generated closure exactly, both directions, every interval at widths 2 through 6, with four
weakened predicates each breaking it, and the same predicate necessary and sufficient over every operand
set of any shape exhaustively at widths 2 through 4. Its sufficiency direction was later made a decision
computed at width 64 by a const gate, 25,120 admissible-window checks with zero mismatches against 3,808 of
62,210 straddling checks wrong. **The holding region and the failing region differ in the declared operand
window and in nothing else.** Signed, saturating, `F = 0`, `W in 2..=6`, addition, on both sides.

**`97`'s F-H, restored in `106` section 16** (`106:1350-1358`). A declared non-negative operand window
recovers additive associativity, multiplicative associativity and distributivity over addition at
`W in {4, 5, 6}`, `F = 0`, signed, saturating, arity 3, values exhaustive, `threads = 1`, target features
any. **The consolidation calls this its only positive law result and its cleanest region-scoped arm**, it
was a prediction of the criterion made before running, it independently retrodicts an earlier file's result
its author had not read, and **it is the single claim in the corpus that most wants to be a gated arm.**
The source's own `holds for:` line names the window inline, so it is one of the four usable regions in
section 1's count and it is still not writable, because the axis it turns on has no row.

**The symmetric-clamp restoration of signed multiplicative associativity** (`63:364-368`). Zero
associativity failures and zero coherence violations at every width measured up to `w = 7`, against 28,
160, 780 and 3516 failing triples at `W` 3 through 6 for the two's-complement range. **One code point, the
most negative value, is the entire difference**, and the axis that expresses it, the geometry of the
representable range, is undeclared. It sits in `law::multiplicative_associativity_under_saturation`'s `gap`
rather than on either list.

**The congruence closed forms** (`63:370-380`). The reduction's kernel is a multiplicative congruence iff
the range is mirror-symmetric or nonnegative, and an additive congruence iff it is sign-confined, with the
predictor mismatching nothing over all 100 zero-containing intervals at five widths. **The whole content of
this theorem is a statement about range geometry.** I folded it into the law-frame proposal's `because`,
where it is prose, because as a predicate it has no axis.

**Invariance under change of encoding** (`90:464-474`). Half of R12. I wrote the container half and left the
encoding half in prose. **An undeclared axis is invisible to the notation rather than absent from it**, per
`checks/src/predicate.rs`'s own doc comment, so the omission is a blind spot rather than a negative claim,
which is the one thing that made writing the row honest at all.

**The four axes this class needs**, none of which `dimension.toml` declares:
`declared_operand_window`, `representable_range_geometry`, `encoding`, and `constant_embedding_convention`.
`90`'s R1 names three of the four in its own dimension list (`90:105-115`) and says explicitly that the
list *"is a floor, not an enumeration"*. **Declaring them is not mine and I have not touched
`dimension.toml`.**

### 5.2 I got this section wrong, and the correction is the most useful thing here

**The first draft of this section listed six claims as unportable because the consolidation carrying
each states no region. Five of the six are portable, and the parameters were in the committed
instruments the whole time.** They are now rows, added on a second pass and marked as such in
`proposal.toml`'s own comment.

**What I did wrong is worth stating exactly, because a canon writer will be in the same position.** The
brief said to derive a region from the instrument's own stated parameters where the consolidation states
none. I read that as "where the consolidation reports the instrument's parameters", concluded the
consolidations mostly do not, and wrote the claims off. **The instruments are committed, in this panel,
one directory away from the file I was reading, and every one of the five states its parameters in its
output or in the constants at the top of its source.** I did not open them because section 3's coverage
statement said I would not open probe directories, and I wrote that statement before I knew what was in
them.

**It took one grep to find out.** `ls 57_probes/` returned `p6_output.txt`, and its second line reads
`sign W n tuples exact measured gap`.

| claim | where its region actually was | recovered as |
|---|---|---|
| a coherent reduction needs no accumulator | `57_probes/p6_output.txt`, sixteen unsigned rows at `W` 3 to 5, fold lengths 2 to 8 | `a_coherent_reduction_needs_no_accumulator` |
| incoherent clamped addition needs the exact-sum width less one bit | the same table's nineteen signed rows at `W` 3 to 6, all at a gap of one | `an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit` |
| the multiplicative guard grows linearly and the saving is fusion | `60_probes/p_d.out` line 1, `M=15 F=3`; `62_probes/p4_output.txt` line 3, `Q = [-8, 7], F = 3` | `the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion` |
| a nonzero phase leaves the set without an additive identity | `56_probes/q2_output.txt`, sixteen values and 256 pairs, so four bits, stated rather than inferred | `a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity` |
| a trajectory condition lifts exactly when it survives closure | `82_probes/p1b_output.txt`, a width column reading 2, 3, 4, 5 | `a_trajectory_condition_lifts_into_a_declaration_exactly_when_it_survives_closure` |

**And one of the six stays out, for a better reason than the one I gave.** R10's instruction counts are
not missing their parameters: `80_probes/p4_asm_report.txt` line 1 names
`aarch64-apple-darwin, rustc nightly-2026-05-28, -O, --crate-type=lib`. **Its next three lines forbid the
numbers**: *"This is an ad-hoc quick spike with no substance for any how-much question. Nothing is timed
and no bench ran. What it can establish is qualitative: which instructions the backend reached for."* So
the qualitative claim is a row and the counts are not, because the instrument says so about itself, and
that is a stronger reason than the one I had.

**Two things this changes about the rest of this file.**

**Section 1's headline is about the consolidations and it stays true**: eleven region statements across
the four, nine of them in one file. **What it does not mean, and what I had let it mean here, is that the
panel does not know the regions.** The panel knows them. They are in the instruments, and the
compression from instrument to consolidation is where they were lost. **That is a much more repairable
problem than the one section 1 describes, and it is a much worse one for a canon writer**, because a
consolidation reads as complete and the loss is invisible from inside it.

**And section 3's coverage statement was a decision, not a limit.** I wrote that I would not open probe
directories, treated it as a boundary, and it cost five rows on the first pass. **A later pass should
open every probe directory named in a claim it wants to port**, and section 14 now says so.

**What is left genuinely unportable for want of parameters** is smaller than the six and I have not found
a member of it while looking. Every claim I checked on the second pass had its parameters somewhere
committed. **So the honest statement is that I do not know of one, rather than that there are none**, and
a pass that walked all seventy-three rows against their instruments rather than the five I chased would be
the way to find out.

### 5.3 The claim's real coordinates are bench or corpus parameters

**These I wrote, with a predicate carrying almost nothing, and I want the decision challenged.** The
alternative was to leave a load-bearing measurement out of the registry entirely, and a findable row with
an honest `note` seemed better than an absence. A reader who disagrees should delete the four and move them
to this section.

- `the_rationalisability_counts_on_the_committed_carrier_table`. Real region: 6 regions, 5 arms, 2 cost
  coordinates, a named CSV set, exact rational arithmetic. Written region: `threads: threads = 1`,
  `target_features: target features any`.
- `generation_relocates_the_check_rather_than_removing_it`. Real region: 5 regions, 5 arms, 3 coordinates,
  400 random models across two families, five named defect classes. Written region: the same two entries.
- `the_corpus_cannot_exhibit_the_accuracy_intents_because_a_coordinate_is_absent`. Real region: 254
  committed regions, 104080 rows, 94 variant crates. Written region: `threads: threads = 1`.
- `most_committed_bench_regions_predate_the_harness_cross_variant_validation`. Real region: a named branch
  head, 254 files, 24 producing commits. Written region: `threads: threads = 1`.
- `a_coordinate_set_is_a_countable_ceiling_on_how_many_strategies_can_exist`. Real region: the committed
  table, the arm count, the coordinate count. Written region: `threads: threads = 1`.

**And four I did not write on the same grounds, because they carry no design content at all.** *The suite
is 123 tests across 13 crates* (`106:61`), five independent counts and a sixth wrong one the consolidation
caught in its own first invocation. *The corpus's column census*, 17 columns of which 9 carry information
and 3 vary between arms (folded into the ceiling row's `because` instead). *The crate-table cross-check's
evidential worth is zero* (`74`, section 3.1), because it describes a crate tree that has been removed.
*The reproduction chain never broke*, five re-run events across six member files with every rerun matching
to the digit (folded into `an_instrument_is_mutated_and_the_battery_is_made_to_notice`'s `because`).

### 5.4 Not claims at all, and they belong in namespaces that are not mine

**`74`'s five located disagreements, D1, D2, D3, D4 and D6** (`74:654-697`). Where a system's selected
adaptation lives; the `format` and `crossing` word collisions; whether the role set is homogeneous; whether
the ambient operation family is fixed or a parameter, which the consolidation calls the unit's largest open
fork; and whose reduction governs a lossy crossing. **Each has two or three coherent positions and none is
forced by anything measured.** A `proposal` row says one thing, so a located disagreement is N rows at
`contested`, and every one of the five is explicitly op's or explicitly open. **They are questions and
`question.toml` is another seat's.** I have referenced four of the five from the `gap` field of the row
they bear on, which is the most I can do without writing in somebody else's namespace.

**`106`'s standing correction on polarity** (`106:394-400`). The register and the checkpoint read as though the
observable-versus-unobservable split is settled at two instances; the file usually cited as the second says
of itself that it *"did not derive it independently"* and that what it did was point the first file's test
at a list the first file never chose, which is a stress test rather than a derivation. **That is a
retirement of a standing claim, not a proposal**, and `retirement.toml` is another seat's. I carried the
corrected standing, `one_expert`, into the rows it affects.

**`106`'s section 13.2 restorations other than F-H** (`106:1360-1385`). The exchange-rate reading of op's
four intents at three instances, with 4 available behaviours under a priority reading against 58 on the
real table. The polynomial-against-exponential bound and the factor-of-47 ratio, which belong together
because one transfers and the other does not. The five-tier ladder with 144 Pareto-admissible sections of
46656, and the finding that a selection rule minimising one coordinate subject to a bound on another is not
expressible as a weighting at all. **All four are one-file results the consolidation restored precisely
because a one-file result dropped at consolidation is gone**, and all four are stated over bench
parameters, so all four are in 5.3's class rather than 5.1's. I did not write them and they should be
written by whoever writes `probe.toml`, at which point their regions have somewhere to live.

**`90`'s R13 as an observation** (`90:476-502`). Four instruments, four authors, one failure: nobody asked
what the instrument would say if the thing it validates were broken. I wrote the *discipline* as
`an_instrument_is_mutated_and_the_battery_is_made_to_notice` and left the four instances in its `because`.
**The observation itself has no region over any declared axis and is a fact about the panel rather than
about arvo.**

---

## 6. Every `sentence_kind` I assigned against what its source called it

**None of the four consolidations labels its claims with these five words**, because the vocabulary is
younger than all four files. So every one of the 68 is my assignment, and what follows is the ones where a
reader could reasonably assign differently. **The incentive runs one way and I want that said plainly:
`normative` is the only label the checker exempts from carrying a region, so every borderline call I made
had a thumb on the scale pushing it there. Audit these first.**

### 6.1 Results I labelled `normative`, which is the class to attack

Six rows, each of which the source presents as an R-numbered or C-numbered *result* and which I recorded as
an imposed rule.

- **`a_composed_expressions_region_is_never_inherited_from_its_parts`** (R2). The source's first sentence
  is a universal negative that a counterexample could refute. Its second is a rule. I wrote both into `says`
  and labelled the row for the second. **The honest alternative is `theorem` with no region, which the
  checker refuses, so the row would not exist.**
- **`a_const_eval_frontier_is_a_fact_about_the_procedure_rather_than_about_the_law`** (R4's consequence).
  Presented as *"the consequence, owed to whoever quotes any of these numbers"*, which is a rule about
  quotation. I am comfortable with this one.
- **`a_law_layer_answers_whether_a_law_reaches_a_lowering_the_backend_cannot_prove`** (R10). The source
  states it as what the probe established after refuting its own thesis. **This is the one I am least
  comfortable with**: it is a finding about backends dressed as a rule about law layers.
- **`an_instrument_is_mutated_and_the_battery_is_made_to_notice`** (R13). Genuinely the discipline rather
  than the observation, and the observation is in section 5.4.
- **`a_law_is_a_fact_about_an_operation_composed_under_a_fixed_arithmetic_semantics`** (R1). A definition of
  what a law is, reached by two cold derivations. Definitions are stipulations; two people arriving at the
  same stipulation is evidence it is the right one and does not make it measurable.
- **`arvo_owes_laws_as_checkable_facts_and_not_as_a_rewriting_engine`** (R1's locus half). A locus refusal
  is imposed by construction.

### 6.2 `74`'s whole block, on one reading I made and am flagging

**All 23 N-sentences are `normative`, including N5 and N7 which their source calls true by construction.**
The reading is that a consequence of a stipulation is part of the stipulation, so it carries no region and
is voided rather than falsified if the stipulation changes. **That reading is mine.** The alternative is
that N5 and N7 are theorems, and under that reading neither can be a row, because the axes they range over
are the coordinates of the dependent sequence and none of those is a `dimension`. **So the choice is
between labelling them normative and dropping them, and I chose the label**, marked on each row.

The remaining 21 I am confident about: a sentence saying what the numeral concept *is*, what a crossing
*is*, who owns which choice, or what a canon sentence must carry, is a stipulation on any reading.

### 6.3 Claims I did **not** relabel, though it would have been convenient

**Seventeen rows are `measured` and every one of them makes the suite red.** Section 9 is why I did not move
them to `argument`. The brief warned about the failure running the other way, an argument marked as a
measurement, and I found one instance of that class in my own drafting: I had `no_multiplicative_structure_survives_a_nonzero_fraction_width` down as `measured` on the strength of
its three instruments, and `63:413` says in bold *"The boundary is structural, not swept-so-far"* and gives
the route. It is a `theorem` with measured corroboration and it is filed as one.

### 6.4 Two rows whose `kind` is arguable

`the_model_band_transfer_is_defeated_in_both_fragments` is `refusal` rather than `finding`, on the ground
that it says a mechanism must not be used and owes an `instead`, which it carries. `no_total_join_exists_over_the_observable_axes_so_the_operation_reports` likewise. A reader
preferring `finding` would lose the `instead` field, which is where the replacement mechanism lives, so I
would argue against the change.

---

## 7. Every standing I demoted, and the region the instances actually share

### 7.1 Demotions I carried from a source that made them itself

- **The polarity distinction** goes from two instances to `one_expert`. The register and the checkpoint
  treated it as two; the second file states in its own words that it did not derive independently.
  `106:394-400` makes the correction and I carried it.
- **The two-level structure of the strategy object.** `106` records it at no standing; `108` says it is two
  experts, being `40` plus `93`'s blind phase one, and that `93`'s own claim of three instances overcounts.
  I have not written the two-level structure as its own row, because `106` gives it no standing and I would be
  taking `108`'s correction of a claim `106` never made.
- **R1's corroboration count.** Two independent instances plus one earlier same-author instance at lesser
  weight, corrected down inside the unit from an attempted three. Carried as `two_experts` with the third
  named in `note`.
- **`102`'s answer-equivalence claim.** From "every committed region" in bold to 234 of 254, refuted by a
  file dispatched to verify it. The row carries the corrected number and the refutation.

### 7.2 The shared-literature discount, carried rather than resolved

**`63`'s C1, C2 and C3 are `two_experts` and all three carry a discount their own author stated**: the two
derivations draw on one numerical-analysis literature, which both files declared. The consolidation's own
bound is that this is *"worth more than a read and less than two arrivals from nothing"*.

**The schema has no tier between `one_expert` and `two_experts`.** I kept the source's word and put the
discount in `note` rather than demoting, on the reasoning that demoting would be me adjudicating something
the source explicitly declined to adjudicate. **A reader who thinks two instruments over one shared premise
is one instance should read those three rows as `one_expert` and I would not argue hard.** Under the
brief's own rule that two instances agree about the intersection of their dimensions, a shared literature
is a shared premise and the intersection over it is empty.

**The same discount applies to `74`'s N20** (two instruments over one premise set) and to
`the_concept_is_closed_and_the_inventory_is_open`, which carries it in `note`.

### 7.3 The one standing I raised, and why it is not an inflation

Six rows carry `cross_topic`, which is the strongest standing in the enumeration. **Every one comes from
`AGREEMENTS.md` section 6, which states the independence explicitly**, and that section's whole thesis is
that no consolidation names any of them, so they are visible only by reading all four together.

**One of the six I want to flag as weaker than the others.**
`three_topics_independently_terminate_on_the_strategy_axis_as_their_shared_placeholder` carries a predicate
of three `any` entries, which is the weakest region in the file. I wrote it rather than omitting because
omitting all three would say the convergence holds nowhere any of those axes exists, which is the opposite
of what three topics found, and the row's own `note` tells a reader to treat it as a pointer rather than a
gate.

**And `chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type` is the strongest thing
in the file.** Two blind derivations at two parameter settings inside one topic, plus independent arrivals
in two further topics, none citing the other two. Its region is `W in {8, 16}`, `F in {4, 8}`,
`operation = mul`, `chain length any`, which is the union of the two blind settings.

---

## 8. Where the schema fought me

### 8.1 A predicate is a conjunction and a law's region is a union

**This is the structural one and it shaped `law.toml`'s whole layout.** `holds` is one list, one entry per
axis, read as a conjunction. Additive associativity holds under wrapping at both signednesses at any
fraction width, **and** under saturation at unsigned only. That is a union of two boxes and no conjunction
expresses it.

**The workaround is to split the law by the axis on which the verdict inverts**, which is why there is
`additive_associativity_under_wrapping` and `additive_associativity_under_saturation` rather than one row.
The two share a `statement` almost word for word. **A reader searching for "is addition associative" finds
two rows and has to read both**, and there is nothing in either that says the other exists except the
naming convention.

The alternative I rejected was writing the coupling into one axis's value string, `overflow_policy: wrap,
or saturate with signedness unsigned`, which is expressible and makes the notation lie about being per
axis. **The split is uglier and honest.**

### 8.2 An explicitly empty optional list is reported as malformed

`evidence = []` in a field the schema declares `probe[]`, `required = false`, produces:

```
ERROR [malformed-row-reference]: law::additive_associativity_under_saturation: field `evidence`
  holds ``, which is not a slug (snake_case, starting with a letter).
```

once per row, four rows, `registry check failed: 4 error(s)`. **The reference reader turns an empty array
into one empty-string entry rather than into no entries.** So the two states a writer wants to distinguish,
"nobody has looked" and "somebody looked and there is nothing", collapse into one, and the only expressible
state is omission, which reads as the first. Full record at `182_probes/empty_list_is_malformed.txt`.

**Every `evidence` field in both files is omitted rather than empty, and that omission does not mean what I
wanted it to mean.**

### 8.3 `cargo mock --lint-only` passes clean on a predicate naming an undeclared axis

Control 1: a predicate entry mutated to `no_such_axis_at_all: F any`, three substitutions landed, and
`cargo mock --lint-only` reported **`all lints passed`**. The `arvo-checks` arm caught all three.

**So the two validators are not interchangeable and neither is a superset of the other.** The linter
resolves citations and the schema; the checks crate resolves the axis vocabulary and the conditional
obligations. A pass that ran only the first would ship a predicate over an axis nothing declares, which
`checks/src/predicate.rs`'s own doc comment says *"silently converts the strongest negative statement in
the notation into a shrug"*.

### 8.4 The linter resolves that a line exists, never that it says anything relevant

`182_probes/show_cited_lines.sh` prints the line each citation points at. **On its first run over
`law.toml` it found six citations landing on a blank line or on the wrong paragraph**, and six more over
`proposal.toml` across two later runs. Every one passed `cargo mock --lint-only` clean, because the line
number resolved.

**Twelve mis-citations out of 139, caught by an instrument rather than by rereading.** They were the
ordinary kind: I noted a heading's line number while reading, the content was three lines below, and the
paragraph shifted while I was writing. **The linter cannot see this class and nothing else in the toolchain
does either.** The script is committed and takes a registry file as an argument.

### 8.5 No `status` field is a good decision that costs something here

`mockspace.toml:349-353` states the reasoning and I agree with it: whether a proposal is blessed is derived
from what points at it. **The consequence for this pass is that nothing in `proposal.toml` distinguishes a
row I am confident in from one I flagged in section 6 as a call that could go the other way.** Those flags
live in `note` as prose, which no query reads. A reader running `mock query` over the namespace gets 68
rows with no signal about which of them I would defend hardest.
---

### 8.6 The corpus's vocabulary and the workspace's disagree on one word

`vocabulary.md` bans `rung` and directs a writer to `tier`, `level`, or naming the position outright.
**The panel corpus is built on that word**: `RULES.md` states its standings as rungs, `84` names its
validation levels `rung 0` and `rung 3`, and `97` names a policy `exact-in-a-wider-rung`.

I have used `standing`, `tier` and `level` throughout both files and this one, which reads correctly
everywhere and is not a translation problem. **One exception survives on purpose**: the keyword `rung 0`
on `the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times`. Keywords exist so
somebody can find the row, and `rung 0` is the string a reader who has read `84` will type. Renaming it
would make the row harder to find in exchange for consistency nobody searching benefits from.

**A later pass merging this corpus into prose will hit the same collision at every occurrence**, and the
translations that work are: a standing is a `standing`, a validation level is a `level`, and a width tier
is a `tier`.

---

## 9. The suite is red on seventeen rows and that is the correct end state

**`the_committed_canon_has_an_instrument_behind_every_measurement` fails, naming seventeen proposal rows.**
41 checks green, 1 red.

```
proposal::a_compile_time_strategy_selection_leaves_no_residue_in_the_emitted_body
proposal::a_coordinate_set_is_a_countable_ceiling_on_how_many_strategies_can_exist
proposal::a_law_stated_as_an_author_written_marker_is_checked_by_nothing
proposal::absorption_decides_associativity_of_a_clamped_reduction
proposal::chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type
proposal::generation_relocates_the_check_rather_than_removing_it
proposal::headroom_and_intermediate_precision_are_unobservable_inside_a_pure_ring_region
proposal::most_committed_bench_regions_predate_the_harness_cross_variant_validation
proposal::no_total_join_exists_over_the_observable_axes_so_the_operation_reports
proposal::the_const_eval_frontier_collapses_along_arity_and_buys_three_bits_from_the_guard
proposal::the_corpus_cannot_exhibit_the_accuracy_intents_because_a_coordinate_is_absent
proposal::the_rationalisability_counts_on_the_committed_carrier_table
```

**The check is right and the rows are right and the two cannot both be satisfied yet.** `evidence` is
`probe[]`, `probe.toml` does not exist, the brief allocated it to a later pass, and a citation into a
namespace with no rows fails the reference resolver. So the three available end states were:

1. **Write the measurements as `measured` with no evidence.** Red, honest, and the red names exactly what
   is owed and where.
2. **Relabel the seventeen as `argument`.** Green, and it fabricates a claim about what established seventeen
   results. The brief calls this out by name and the check's own message says *"A measurement with no
   instrument is an argument wearing a number"*, which is precisely the sentence I would be making true.
3. **Write `probe.toml`.** Green, and it takes a namespace the brief allocated elsewhere.

**I took the first**, on `strict-by-design-quality-pressure.md`'s reading that a red test whose green path
is a designed-but-unbuilt mechanism marks that mechanism. The namespace is designed, declared at
`mockspace.toml:737-800`, and unbuilt. **It goes green when `probe.toml` lands, by adding one field to each
of seventeen rows, and the assertion needs no change.**

**So this is a catalogue-red, and the next seat should not fix it by editing `proposal.toml`.**

**The instruments the seventeen rows are owed**, tabulated so `probe.toml` can be built from it rather than
from a rediscovery pass. Every one is named in its row's `note` as well.

| proposal row | committed instruments |
|---|---|
| `absorption_decides_associativity_of_a_clamped_reduction` | `57_probes/p2`, `p2b`, `p8`, `p1`; `61_probes/q1` |
| `a_law_stated_as_an_author_written_marker_is_checked_by_nothing` | `80_probes/p1a_declared_law_lies.rs`, `p1b_computed_law_refuses.rs`; `77_probes/probe2_*` |
| `the_const_eval_frontier_collapses_along_arity_and_buys_three_bits_from_the_guard` | `80_probes/p2_frontier.py`, `p2b_swept_verdict_at_shipped_width.rs`, `p5_allow_the_guard.py` |
| `a_compile_time_strategy_selection_leaves_no_residue_in_the_emitted_body` | `93`'s P4; `94`'s probes A, B, F; `100_probes/p3`; `mock/benches/satfold-const-gate_n10000_findings.md` |
| `chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type` | `93`'s F7; `94`'s W7 |
| `no_total_join_exists_over_the_observable_axes_so_the_operation_reports` | `97_probes/p3_does_a_conservatism_order_exist.py` |
| `headroom_and_intermediate_precision_are_unobservable_inside_a_pure_ring_region` | `102_probes/p2_which_of_25s_axes_change_the_answer.rs`, **both versions** |
| `the_rationalisability_counts_on_the_committed_carrier_table` | `97_probes/p9_the_decider.py`; `98_probes/p6_...py`; `101_probes/p4_what_a_coordinate_buys.py`, `p10_the_two_knobs_are_separable.py` |
| `generation_relocates_the_check_rather_than_removing_it` | `100_probes/p2_generation_relocates_the_check.py`, `p3` |
| `a_coordinate_set_is_a_countable_ceiling_on_how_many_strategies_can_exist` | `101`'s coordinate census; `106_probes/p2` |
| `the_corpus_cannot_exhibit_the_accuracy_intents_because_a_coordinate_is_absent` | `103`'s classification instrument; `106_probes/p5` |
| `most_committed_bench_regions_predate_the_harness_cross_variant_validation` | `103`'s F-103-6; `106_probes/p3_prewiring_join.py` |
| `a_coherent_reduction_needs_no_accumulator` | `57_probes/p6_the_adaptation_absorbs_one_bit.rs` |
| `an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit` | the same instrument, its signed rows |
| `the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion` | `60_probes/p_d_rescale_saving_is_adaptation_fusion.rs`; `62_probes/p4_signed_multiplicative_accumulator.rs` |
| `a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity` | `56_probes/q2_affine_membership.rs` |
| `a_trajectory_condition_lifts_into_a_declaration_exactly_when_it_survives_closure` | `82_probes/p1_box_lifting_of_p4.rs`, `p1b_is_every_lifted_box_degenerate.rs` |

**Three of these have a recorded defect** and the `probe` namespace's `defect` field is where it goes:
`102_probes/p2`'s first version swept only additive chains ending in a mask and was proving the ring
homomorphism it stood on rather than testing the axis; `106`'s own test-gate probe read the doc-test result
line and reported zero tests at exit code 0; and `86_probes/p5` could not distinguish its procedure from
one with the breakpoint apparatus deleted, which a later mutant established.

**And the `law.toml` rows carry no evidence either**, but the law namespace has no equivalent check, so
that goes unreported by any instrument. Fourteen law rows, zero probe citations, silent.

---

## 10. Conflicts between consolidations, and the near-miss

**`AGREEMENTS.md` section 7 reports no contradiction among the four and my reading agrees.** I found none
while porting. Three things are worth recording anyway.

**The conflatable pair, which is the one to be careful about.** `63`'s signed saturating **multiplication**
associativity counts (28, 160, 780, 3516 triples at widths 3 through 6) and `106`'s signed saturating
**distributivity** counts (47.72% at width 7, 34.52% at width 6) are two different laws by two different
instrument sets at two points in the timeline. **Neither consolidation checked one against the other,
because they are not the same claim.** They now sit in two law rows,
`multiplicative_associativity_under_saturation` and `distributivity_of_multiplication_over_addition`, and
the row `the_four_consolidations_contradict_each_other_nowhere` carries the union region as its predicate,
which is the region a reader has to be careful in.

**The internal contradiction inside `106`.** Section 4 says component two ranges over the arms that produce
the answer component one fixed; section 8 of the same file says a fidelity column in that region would
measure a constant. **Both cannot hold**, and `106` section 18 resolves it by repairing the first rather
than the second. I have written both the superseded pair and the repaired one, with `supersedes` wiring
them, because a reader meeting the pair in the register or the checkpoint needs to know which version they
are holding.

**The near-miss I want on the record.** `AGREEMENTS.md` section 8 states plainly that its author opened no
member file, no probe directory and no register, so every standing characterisation in it is the
consolidation's own rather than one the ledger derived. **Six of my rows carry `cross_topic` on the
strength of that ledger.** The ledger is one expert reading four documents; the convergences it reports are
real and the *independence* it asserts for each is the four consolidations' own accounts of themselves.
`the_four_consolidations_contradict_each_other_nowhere` is at `one_expert` for exactly this reason and says
so.

---

## 11. The control runs

**Full output at `182_probes/control_runs.txt`**, produced by `182_probes/controls.sh`, run from the
repository root. Eight arms: a baseline, six mutations, and a restore that must return to the baseline
exactly. **Every arm prints how many substitutions actually landed before it runs the validator.**

| control | mutation | substitutions | what fired |
|---|---|---:|---|
| 0 | none | 0 | baseline: lints pass, 41 checks green, 1 red |
| 1 | a predicate names an undeclared axis | 3 | `predicate-names-an-undeclared-dimension`, three findings. **`cargo mock --lint-only` passed clean** |
| 2 | a citation names a file that does not exist | 15 | `unresolvable-provenance`, `registry check failed: 15 error(s)` |
| 3 | a line citation past the end of a real file | 1 | `unresolvable-provenance` naming `99999 requested` |
| 4 | a line citation into a living ledger | 6 | `line-citation-into-a-living-ledger`, six findings |
| 5 | a heading anchor naming no heading | 6 | `unresolvable-heading`, `registry check failed: 6 error(s)` |
| 6 | `normative` retyped as `theorem` | 43 | `an-established-claim-carries-no-region`, 43 findings |
| 7 | `theorem` retyped as `normative` | 5 | `an-imposed-proposition-carries-a-region`, 5 findings |
| 8 | restored | - | back to control 0 exactly |

**Controls 6 and 7 point opposite ways on purpose**, because an arm that reports both shapes is reporting
the rule rather than a breach of it. The 43 and the 5 are the two populations in section 2's table, which
is a second confirmation of those counts from a different instrument.

### The instrument was broken twice, and its own counter is what caught it both times

**This is the part of section 11 worth reading.**

**First break: the delimiter.** The script's first version wrote each mutation as
`sed -i '' '0,|pattern|s||replacement|'`. BSD sed refuses `|` as a range-address delimiter with
`expected context address`, and the error goes to stderr in the middle of a long run. **Five of seven arms
mutated nothing and reported clean validators.** A clean validator after an intended mutation reads exactly
like the checker being correct.

**Second break, and the dangerous one: the range address itself.** Rewritten with `/` delimiters, all seven
arms still reported zero. Measured on a three-line scratch file:

```
$ sed -i '' '0,/aaa/s//ZZZ/' t1.txt ; echo "exit=$?"
exit=0
$ cat t1.txt
aaa
bbb
aaa                          <- unchanged, no diagnostic, exit 0

$ sed -i '' '1,/aaa/s//ZZZ/' t2.txt
sed: first RE may not be empty     <- this form at least fails loudly

$ sed -i '' 's/aaa/ZZZ/g' t3.txt   <- works
```

**`0,/re/` is a GNU extension. GNU sed applies it; BSD sed parses it, matches nothing, exits 0, and says
nothing.** Full record at `182_probes/sed_range_is_a_silent_noop.txt`.

**Two things caught it and neither was reading the validator's output.** The substitution counter, printed
before every run, which was in the script from the first draft because
`180_probes/control_runs.txt` had already recorded the same class of failure on a different sed form. And
the fact that arm 1 had fired correctly earlier when run by hand with a different mutation method, so there
was a known-good result for the script's zero to contradict.

**The final version uses plain `s/pattern/replacement/g`**, which mutates every occurrence rather than the
first. That is fine for a control and the counter says how many landed. No pattern in it may contain a `/`.

---

## 12. Topics and axes I needed and did not have

**Axes.** Four, all in section 5.1, none written by me: `declared_operand_window`,
`representable_range_geometry`, `encoding`, `constant_embedding_convention`. **The first is the expensive
one**, because it is what R7 and F-H both turn on and those are the two cleanest positive law results in
the corpus.

Two more the corpus names that I did not need but a later pass will: the **schedule** (`90`'s R11 splits
chain laws by it and it is the kind op's accuracy intent is stated over) and the **degree** of a
polynomial law (`90`'s R6 states its whole criterion over a degree box).

**Topics.** The twenty declared covered everything I wrote and I needed none that does not exist. Two
placements were forced rather than natural and I am flagging them:

- `a_law_verdict_is_invariant_under_change_of_encoding_and_container` is filed under
  `the_realisation_map`, which is the only topic in the file. It is a claim about what a law verdict reads,
  so `algebraic_laws` is arguable; I chose the topic whose subject the claim is *about* rather than the one
  it belongs to.
- Four rows are under `panel_conduct`, which `topic.toml` says is *"Not a claim about arvo"*. Three of the
  four genuinely are not (the instrument discipline, the 952 mis-attribution, the contradiction census).
  **The fourth, `most_committed_bench_regions_predate_the_harness_cross_variant_validation`, is a claim
  about the bench corpus**, which is neither arvo nor panel conduct. It has no better home.

**One thing I did not do that a reader may expect:** `topic.toml` declares `the_primitive`,
`execution_environment`, `operating_constraints` and `naming`, and this pass wrote no row under any of the
four. That is a fact about which four consolidations I was given rather than about the topics.

---

## 13. Edges I could not wire

**`answers` and `obligation` are empty on all 73 rows**, per the brief. `question.toml` has 78 rows and
`obligation` has none, so half of that is a locus decision and half is that the namespace is empty.

**What the rows would point at, tabulated for whoever wires it.** I have not opened `question.toml`, so
this is by subject rather than by id, and it is a starting list rather than a complete one.

| proposal | would answer, by subject |
|---|---|
| `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | format equality; the wrapping filing question |
| `an_additive_verdict_is_independent_of_the_fraction_width` | the mixed-scale addition question, **partially**: the row depends on it rather than answering it |
| `the_laws_of_a_format_are_derived_from_two_hypotheses...` | how a law verdict's truth is established |
| `the_model_band_transfer_is_defeated_in_both_fragments` | the same question's model-band route, by closing it |
| `inside_a_fragment_with_a_complete_test_set...` | the same question's exhaustive route, by replacing it |
| `no_total_join_exists_over_the_observable_axes...` | cross-strategy resolution; the conservatism-order reading |
| `a_strategy_is_a_declared_semantics_together_with_a_weighting...` | which object the word `strategy` names |
| `the_licensed_category_is_const_available...` | the binding-time question, which op then refused as a ranking |
| `roles_derive_representations...` | whether the role set is homogeneous, **by naming the condition rather than settling it** |

**`supersedes` is wired once**, from the repaired pair to the original. **It should probably be wired a
second time and I did not**: `a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts`
supersedes the sufficient direction of an earlier exposure test while keeping its necessary direction, and
that earlier test is not a row in this file, so there is nothing to point at.

**`law` is wired on 15 proposal rows.** Every wiring points at a law row this pass wrote; nothing points
outward.

**`evidence` on all 87 rows across both files is empty**, and section 9 is the whole of it.

---

## 14. What I would tell the next reader to check first

**Six things, in the order I would check them.**

1. **The seventeen red rows, and resist fixing them in `proposal.toml`.** Section 9. The fix is `probe.toml`
   and the table there is what to build it from.
2. **The 43 normative rows, starting with the six in section 6.1.** If any of those six is really an
   established claim, it needs a region, and if it has no region it should not be a row. **I had an
   incentive to label them `normative` and I am telling you which ones I felt it on.**
3. **The probe directory of every row you doubt, before you doubt it.** Section 5.2 is what skipping that
   costs, and it cost five rows on my first pass. **`ls <NN>_probes/` is one command and the parameters are
   usually in the first ten lines of the output file.** The sixty-odd probe directories this pass did not
   open are where the rest of the regions are, and I have no reason to think the five I recovered are all
   of them.
4. **R7 and F-H.** Section 5.1. Two positive law results, both blocked on one undeclared axis, and F-H is
   the cleanest region-scoped arm in the corpus by its own author's account.
5. **The five rows in section 5.3 with near-empty predicates.** I wrote them and I would not defend the
   decision hard. Deleting them and moving them to the un-portable list is defensible.
6. **`182_probes/show_cited_lines.sh` over both files, after any edit.** It found seventeen mis-citations in
   my own work that `cargo mock --lint-only` passed clean, and it will find yours.

**And one thing I would tell whoever writes the canon rather than the next porter.** The registry's
predicate discipline is exact, its checker is real, and both of its arms fire. But 43 of 73 rows escape it
through `normative`, and the whole of one topic escapes it, because that topic's output is a concept rather
than a set of measurements. **Nothing currently distinguishes a `normative` row that is genuinely a
stipulation from one that is a regionless claim wearing the only label that fits.** A `normative` row could
be asked for something a predicate is not: what would have to be true for it to be wrong. That is not a
region and it is checkable in the same way, and it would close the gap this port fell through 43 times.

---

## 15. Coverage, bounded

**Verified independently rather than taken from a file:** the eleven region statements and their locations,
with both controls; every count in section 2; the seventeen red row ids; the eight control outcomes; that
`cargo mock --lint-only` passes on an undeclared axis; that BSD sed's `0,/re/` is a silent no-op, measured
on a scratch file; that `evidence = []` is reported malformed, measured on four real rows; the line each of
the 149 line citations lands on, by opening it; and, on the second pass, the parameters of seven
instruments read off their committed output and source rather than off any file describing them.

**Taken from a source and not verified:** every measurement quoted in any `because` field except the seven
instruments above. I ran no probe and reproduced no number. Where a row says 47.72% or 13,882,880 or
82.7484%, that is the consolidation's figure carried forward, and the consolidations are themselves
compressions of member files I did not read. **The seven I opened I opened for their parameters, not to
check their arithmetic**, so even there the figures are carried rather than confirmed.

**Not attempted:** re-deriving any standing. Every `standing` value is the consolidation's own or
`AGREEMENTS.md`'s, with the four demotions in section 7.1 carried from the sources that made them. I
adjudicated no disagreement and picked no side, and the located disagreements are in section 5.4 rather
than in rows.

**The largest thing I did not do.** I did not open the four entailment checks. `107` alone found four
one-file results dropped from `106`'s droplist and three further defects standing unrepaired, and I know
those seven only through `106`'s own account of them. **If one dispatch follows this one, opening `107` and
`91` against what I wrote is worth more than another porting pass**, because a compression is checked by
someone other than whoever compressed it, and this file is a compression of four compressions.
