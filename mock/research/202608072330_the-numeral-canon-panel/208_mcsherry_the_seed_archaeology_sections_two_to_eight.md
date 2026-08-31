# 208. McSherry: what is still live in the seed archaeology's sections 2 through 8

**Seat:** 208. **Branch:** `research/canon-port-208`.
**Deliverables:** this file, `208_catalogue_the_seed_archaeology_sections_2_to_8.toml` at 60 rows, five
instruments in `208_probes/`, four registry rows, and one test file with five arms.

Both gates ran first. `cargo mock` clean at 647 rows, `cargo test -p arvo-checks` green at 17 binaries, and I
read the bodies rather than the names in the surface I touch. That suite is good and I will say where in
section 6, because the one thing I found wrong in it was not a test at all.

Two corrections to the brief, stated first.

**`mock/registry/README.md` does not exist.** Seat 207 already reported this and the brief still names it as
required reading. The namespace headers carry what a README would.

**Section 8 is not untouched.** 207's coverage line records reading "sections 0, 1 and 8 in full". Sections 2
through 7 were untouched, which is what mattered, and I read 7 and 8 first as instructed because they are
where the file says what it thinks its own claims are worth.

---

## 1. The answer to the second half, which turned out to be short

I was asked what the registry has to become to carry this material. **It does not have to become anything, and
that is a measurement rather than an opinion.**

`retirement.toml` cites the closed formalization panel 94 times. Every other namespace cites it zero times. So
the channel exists and is heavily used, and it is used for exactly one thing.

I did not take that on the count, because a citation the engine skips looks identical to one it resolves.
`208_probes/p5_archive_refs_resolve.sh` plants a `lives` entry naming a file that is not in the archive, runs
the registry check, and requires it to report; then restores and requires it to pass. Both fired:

```
ERROR [unresolvable-provenance]: probe::the_structural_backing_array_compiles_and_its_layout_controls_refuse:
`panel::202607301300_formalization-spec-panel::OLD_76_probes::b1_THIS_FILE_IS_NOT_THERE.rs::1`
matches no file under root `panel`.
```

**So a registry row can carry evidence out of the closed panel and the engine holds it to the same standard as
a citation into this one.** What limits the port is not the schema. It is that a `proposal` needs a region over
a declared axis, a `law` needs the same, and a `probe` needs a control, and almost nothing in section 3 has
any of the three.

**The one thing worth noticing about those 94 citations is that they are all the same citation.**
`OLD_124_consolidation_twelve` section 6, the cumulative droplist, once, repeated. The registry's entire
connection to a 320-file archive is one heading anchor pointing at the list of things that must not be cited
again. Until this seat, nothing had ever cited that archive to carry a result forward.

---

## 2. What is genuinely current, and it is two things

The seed names three exceptions to its own no-evidence rule. I checked all three rather than taking them.

**The two re-run probes are real and now reproduce a third time.** `208_probes/p2` runs both on the pin,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, which is the same compiler the 2026-08-09 rerun logged. Both
compile at exit 0. More usefully, the archive shipped two negative controls beside one of them, and I ran those
as arms in the same invocation: `b1b` puts one byte of padding in the odd-arity node and `b1c` reaches the same
corruption through a capacity no assertion names. Both refuse at exit 1. `b1b`'s own header states the stake,
that if it does not refuse then every assertion in `b1` was vacuous and its `unsafe` undischarged.

Those two are now `probe::the_structural_backing_array_compiles_and_its_layout_controls_refuse` and
`probe::the_structural_width_ladder_compiles_gate_free_on_the_pin`.

**They are filed differently and the difference is the point.** The array one is `sound`, because its author
shipped controls and they fired. The ladder is `uncontrolled`, because that source ships none: what reproduces
is that it compiles, and nobody has built a mutation of it that must refuse. I did not let it borrow its
sibling's controls, and the machinery enforces the consequence, since
`measurements_resting_on_an_unusable_instrument` reports any `measured` proposal citing an `uncontrolled`
probe. The compile fact is not in doubt; what is missing is a way for it to have been wrong.

**Both rows say the compile fact and refuse to say the design conclusion.** The archive's claim, that
structural keying is the route the width surface should take, stays the archive's. Op demoted that lineage and a
row asserting it here would restore an authority he removed.

**The third exception is the bench corpus, and its count is wrong.**

---

## 3. The 57 that belongs to nine sections

The seed's T9 states that the container-fork benches are committed at `mock/benches/warm-container-*` as 57
CSV plus meta plus findings triples.

The glob holds 22.

`git log --diff-filter=D` over `mock/benches/warm-container*.csv` is empty, so nothing was ever deleted and
this is an overcount in the document rather than a loss on disk. The archive's own file 141 states the same 57
and **141 is right**: its wording is "nine harness bench sections in `mock/benches/`, 57 committed CSV plus
meta plus findings triples", and the commit that landed it added exactly 57 CSV:

```
  12 warm-container-width          6 precise-widening-theorem
  10 warm-container-density        6 precise-elementwise-width
   6 warm-elementwise-width        6 precise-container-width
   6 warm-affine-collapse          5 warm-affine-density
  total: 57
```

Five of those nine sections are not `warm-container-*` at all. **The seed carried a nine-section total and
narrowed the scope to one glob.**

This is the same class 207 found on the op roster, where 23 was correct in its source and 37 was the truth by
the time the document carrying it was read. Two instances now, in one file, from two seats working different
populations, which makes it a property of the file rather than a slip. It is retired as
`retirement::stp_fiftyseven_warm_container_triples`, `kind = "misattributed"`, because the figure is somebody
else's and about something else, which is exactly what that value is for.

**While counting, the same probe dated the whole corpus**, which nobody had. Every meta file carries a
timestamp, and the current panel's directory is stamped 2026-08-07 23:30, so the split is clean: 21 bench
families predate this panel and 18 are its own. Two consequences fall out.

The seed's section 0 names four bench prefixes as what may be cited as current evidence. Inside the
closed-panel set those four reach 92 of 147 CSV, leaving **55 in eleven families it does not name**, including
every `precise-*` family and the quantiser sweep. Section 0 says "several" so it is not claiming to be
exhaustive, and it is still the file's own statement of what counts as evidence.

And T29's owed measurement has been taken. Op named concurrent multi-column bandwidth contention as the thing
the small-footprint intent was waiting on; `bitpack-contention`, `bitpack-contend-best`,
`bitpack-contend-decode` and the two write-contend families are all dated to this panel. That is the one item
in section 3 where the archive's owed measurement demonstrably exists now.

---

## 4. The live hazard, and it is reached from inside the registry

This is the finding I would keep if I could keep one, and I found it by following the registry's own pointers
rather than by reading the seed.

`retirement::dl_the_ordered_three_relation_ladder` retires the weak-Kleene-graded ladder as superseded, naming
**the nine-point view lattice** as the replacement. `retirement::dl_partial_associativity_as_a_named_gap`
retires op's adopted name with it, and names the same replacement, specifically at "the position where a view
preserves values and events **while losing definedness**".

Both retirements are correct. The replacement is the problem.

`grep -iE 'view lattice|nine-point|finest view'` across all twelve registry files returns those two rows and
nothing else. **The construction the canon sends readers to appears in no `proposal`, `law` or `probe` row of
its own.** It exists in the registry only as the thing other claims were replaced by.

And the seed's T15 says one of its three named points was compile-refuted. I opened that at source rather than
taking it. `OLD_SETTLED_laws.md:374-405`: the Kleene equation coincides with that lattice point only if
definedness is recoverable from the grade's cause component, and the design's own division chapter refutes it
for every numeral carrying infinity or NaN, because both deliver defined results while carrying a nonempty
cause. The probe that had made it look safe set its own definedness flag from its own cause counter, at
`OLD_136_willsey_the_laws_under_the_laws.md:215-224`, in that file's own words: "The model made the invariant
true rather than testing it." Twenty-four files carried it unchallenged. Three repair shapes were offered and
none chosen.

**So the position the second retirement row names is precisely the position the refutation is about, and
nothing in the registry records the refutation.** A reader following that pointer lands on a construction the
archive itself refuted at that point.

Filed as `question::which_repair_the_refuted_kleene_identification_takes`, written as a question about the
registry rather than about the archive, because what is at stake is whether two live rows say something true.

**I did not use op's archive-era response as authority**, though I verified it at
`OLD_137b_op_checkpoint_thirty.md:87-100`: he treated the calls resting on this as stale rather than patched
and asked for a re-evaluation that never ran. His voice across that panel is demoted by his own word, and the
re-evaluation was asked of a panel that then closed. It is in the row's note as context, and `decider` is
`panel` by the namespace header's own rule.

**Read T15 and T47 together or neither.** The ladder is correctly retired; the thing readers are sent to is not
sound at the point one of the rows names.

---

## 5. The sweep, and what a zero from it is worth

`208_probes/p4` searches the registry for each of the 59 talking points on two or three independent phrasings,
with four carried controls that must return non-zero and one nonsense control that must return zero. All five
behaved.

**Twelve subjects are ABSENT on every phrasing tried.** T1, T12, T21, T26, T27, T33, T42, T45, T49, T54, T55,
T56. Those are the reliable verdicts.

**"Present" is not a verdict and I have said so in the probe's own output.** It means a word appears. The
registry reached most of these subjects by its own route and may hold a different claim about them, a narrower
one, or the opposite one. Fourteen catalogue rows are `covered` and several say plainly which half may still be
unported.

My instrument had a bug worth confessing because it is the kind that flatters. T10's second phrasing included
the bare word `ring` to catch the modular-ring argument, and it returned 343. It was matching `during`,
`lowering`, `bearing`, `neighbouring`, `ordering`. Tightened to 39, which on reading is the current panel's own
vocabulary. **A count that large should have read as wrong on sight and instead read as coverage**, which is
the direction these errors always go.

The catalogue's verdict spread over 60 rows: 41 `unported`, 14 `covered`, 2 `reproduced`, 1 `retired`, 1
`miscarried`, 1 `live_hazard`.

**41 unported is not 41 losses and the catalogue says so per row.** `why_dropped` is a separate field from
`verdict` for the reason 207 gave: "no reason found" must never be readable as "superseded". Where a document
records a decision not to carry something, the row cites it. Where nothing does, the row says so in those
words, and that is most of them.

---

## 6. The catalogues were TOML nobody read, and my test for that was wrong twice

Nothing validated the panel catalogues. Not `arvo-checks`, which names none of them; not the engine, because
they sit outside `canon_paths`; not `cargo mock`. Four of them existed before mine. **A stray quote in any one
would have sat there indefinitely, and the file it breaks is the one a later seat reads instead of re-sweeping
the archive by hand.**

`mock/checks/tests/a_panel_catalogue_is_readable.rs`, five arms: every catalogue parses, every row can be
named, no two rows claim the same identifier, a confidence is a number in zero to one, and a control arm that
plants each defect and requires the readers to find it.

**Both of the first two arms were wrong when written, and each was corrected by a catalogue rather than by an
argument.**

The first demanded an `id` field and went red on `catalogue/100-149.toml`, which keys on `path`. That
catalogue is right: its rows are panel files and the path is the identity, and an `id` beside it would be a
second name for one thing. Then it went red on `203_catalogue_001_049.toml`, which keys on `file`.

The second then demanded uniqueness of whatever key it found, and went red on
`203_catalogue_001_049.toml` carrying `22_xu_the_bench_that_was_missing.md` three times. **That is not a
defect either.** Seat 203 split one panel file into three rows because its section groups have three different
verdicts, `live`, `lost` and `lost`, with different ported-as targets. Splitting it is better work than
flattening three dispositions into one row would have been.

So the property is: a row can be named, by `id` or `path` or `file`; and only `id` promises uniqueness,
because the other two name a subject rather than a row. **Both corrections narrowed the assertion onto the
property and away from the spelling, which is the only direction a test may be loosened in.** I have written
that into the file so the next person does not re-tighten it.

The suite is 18 binaries green, up from 17.

**On the rest of the suite: I looked for the reason to refuse and did not find one.**
`what_one_field_obliges_another_to_carry.rs` plants two probes with **identical control text** differing only
in `standing` and asserts that exactly one is reported, with the failure message saying that if both are
reported the prose matcher has started catching it and the test is measuring something else. That is a real
discriminating arm and it is why my `uncontrolled` filing has teeth.

---

## 7. What I could not settle

**Whether this namespace should reach the closed panel at all.** I added two `probe` rows citing the archive
and I am one expert. The rule wants two, and I am saying so rather than letting the rows stand as though the
question were closed.

The argument for them: what is recorded is a reproduction performed here, under current discipline, on
committed sources, with controls that fired in my own run. The argument against: `retirement` has been the only
namespace touching that archive, always to close a route, and a citation carrying a result forward is new.

**And there is a sentence bearing on it that I could not resolve.**
`ruling::the_prior_strategy_split_was_well_enough_defined` carries op's verbatim: "Warm is what regular old
rust would do. In fact, **this is the one thing you can check from the prior panel.**" The registry's `says`
renders that as "is the one thing worth checking from that panel". His wording reads more restrictively than
the paraphrase. Under the restrictive reading, porting anything from a subsection other than the strategy axis
is not licensed, and T1 and T2 are the container derivation.

I opened `37_op_warm_imitates_rust_and_strategy_is_not_orthogonal.md` in full to see whether context settles
it. It does not: the antecedent of "this" is arguably the Warm claim and arguably the strategy split, and
either way the sentence is about what survives from that panel rather than about what a registry row may cite.

**I acted on the permissive reading and built the narrowest rows I could**, which say only that a named source
compiles and that two controls refuse. If the restrictive reading is right, those two rows should go, and
nothing else I did depends on them. **I would rather have that argued than have it pass unnoticed because the
rows read as modest.**

**The member-file sweep.** T55 records three instruments the archive named and never built, the first being a
sweep of the ninety-nine numbered member files for material no consolidation absorbed. The seed states plainly
that it is not that instrument: it swept consolidations, sweeps, op files and the late stretch. **Seat 207
independently reached the same conclusion from the op-material side and named the same missing sweep.** Two
seats, two populations, one instrument nobody has built.

That is the coverage bound on this catalogue and I have written it into the file rather than leaving it to be
inferred: **every `unported` verdict here rests on the seed's rendering of a consolidation, and the seed's own
section 8 says a point sourced only from a consolidation inherits that document's compressions.** An
`unported` row may be unported because the material never reached the seed either.

**Six of 127 cited passages were opened.** T15's refutation and its probe, T24's discrepancy, the 57 at both
ends, and the two re-run sources. What was checked mechanically is different and is stronger: every citation
resolves and is in range, both current claims reproduce with the archive's own controls, the bench corpus is
counted and dated, the sweep ran with controls, and the archive channel is shown to resolve rather than skip.

---

## 8. What I would attack next, from a different angle

**The 81-versus-zero discrepancy, T24.** It is the cheapest high-value thing left. One file reports 81 decided
join failures in a slice where two instruments in another find zero; the archive says a third instrument is
owed and that "it will poison a consolidation that quotes either number", which I verified verbatim at
`OLD_SETTLED.md:126-137`. Nothing in the registry records it. **And there is an obvious first move nobody has
made**: T22 says the inclusion order needs four conditions where the establishing sweep used two, and that the
two-condition form produced 17,037 false positives invisible because every numeral in that sweep had bias zero.
If one of the two join counts came from the two-condition predicate, the discrepancy may dissolve without a
third instrument.

**T54, the owed-artifact list.** Not a claim, so it does not decay: a list of the cheapest attacks on the
archive's own conclusions, and it survives the demotion because an undone thing stays undone whoever said so.
Its truth-suite item is the sharpest, because 672 green tests over a contract whose foundational property is
asserted nowhere is exactly what this workspace's test gate calls missing fundamentals, and T35 is the contract
those tests do not check.

**Setup-that-helps appears four times in this archive**, at T15, T22, T25 and T46, each found by a different
instrument and none by the author. The archive's own fourth statement of it is the sharpest thing in it: a
model that undercounts refusals fails loudly, **a model too narrow to see a value disagreement returns a
quietly wrong number, and no mechanical guard for the second was ever found.** That is a standing hazard for
every model-width sweep this panel runs, it is in no registry row, and it is one finding split across the
seed's sections 2 and 4 so neither half reads as load-bearing alone.

---

*Grounded on: `SEED_TALKING_POINTS.md` sections 0 and 2 through 8 in full; `207_mcsherry_op_material_in_the_dead_panel.md`
and its catalogue in full; `mock/registry/*.toml` headers in full and `ruling.toml`, `retirement.toml`,
`probe.toml`, `question.toml`, `dimension.toml` at the cited rows; `mockspace.toml`'s registry schema for
`probe`, `retirement` and `question`; `mock/checks/src/{citation,corpus,predicate}.rs` and the bodies of
`what_one_field_obliges_another_to_carry.rs`; `37_op_warm_imitates_rust_and_strategy_is_not_orthogonal.md` in
full. Opened in the archive: `OLD_SETTLED.md:126-137`, `OLD_SETTLED_laws.md:374-405`,
`OLD_136_willsey_the_laws_under_the_laws.md:215-224`, `OLD_137b_op_checkpoint_thirty.md:87-100`,
`OLD_141_xu_the_container_fork_benched.md:8`, `OLD_76_probes/b1b_layout_law_negative_control.rs`.
Verified by instrument: `208_probes/p1` (127 citations, three controls), `p2` (two re-runs, two negative
controls), `p3` (bench corpus counted and dated, one control), `p4` (registry sweep, five controls), `p5`
(archive refs resolve, mutation control both directions). Canon gate: `canon_paths` names
`mock/registry/*.toml`; this seat adds two `probe` rows, one `retirement`, one `question`, one catalogue, five
probes and one test file, and touches no crate source. Test gate: 17 binaries green before, 18 after.*
