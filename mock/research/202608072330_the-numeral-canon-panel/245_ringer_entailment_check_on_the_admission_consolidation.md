# 245. Entailment check: `244`'s consolidation against `241`, `242` and `243`

Seat 245. I did not write `244_orchard_consolidation_admission_and_the_number_system.md` and that
is the whole point of the dispatch: a compression is checked by somebody who does not hold the
original in working memory. Four instruments, run against the three sources
(`241_kiselyov_admission_is_a_resolution_not_a_verdict.md`,
`242_what-admits-a-number-system.md`, `243_seat242_the_resolution_has_no_second_arm.md`), plus
`95`, `181`, `217`, `INTENTS.md` and `AGREEMENTS.md` as the governing texts the brief named.

## Gates

**Canon gate: aligned.** This is an entailment check on panel research against the typed registry
at `mock/registry/*.toml`, which `mockspace.toml:31` declares as `canon_paths`. Nothing here is
canon and nothing here moves a registry row; `244` says so at its own opening and I hold the same
posture. No misalignment to report and no ambiguity to hand back.

**Test gate: run.** `cargo test --workspace --manifest-path mock/Cargo.toml` at my worktree's HEAD
(`98a4b7ee`, a merge of `244`'s branch with unrelated parallel canon work landed on `dev` in the
meantime): 85 in `arvo-format`, 3 doctests, 4 more doctests, 14 in `arvo-placement` with 1 ignored,
19 with 1 ignored in another crate, 10 in `arvo-strategy`, 4 `compile_fail` doctests. All green,
two ignored, zero failing. The population is larger than `244` measured because real work landed on
`dev` between `244`'s commit and my worktree's tip (see "A methodological correction" below); none
of the growth touches the admission topic and none of it is a regression.

## The brief's false premise, re-verified independently

My brief carries the same instruction `244` was given: check whether `242` states that its reading
and `241`'s must not be merged, and that the tier count is to be recorded as contested. **It does
not, and I ran the positive-controlled instrument fresh rather than trusting `244`'s report of it.**

```
merge, combine, synthes, should not be, cannot be read as one : 0 hits, 242 + 243
positive control: resolution = 12 in 243, tier = 17 in 243 and 29 in 242
```

Exact reproduction of `244`'s section 0 numbers. `244`'s refutation is correct, and correctly
narrow: it does not attribute the false instruction to `242`, and it correctly identifies the true
thing near it (`242` section 9's tier-count observation, which is a different claim entirely, about
`question::are_the_level_hierarchies_the_same_cut`).

**Whether the false premise left a trace elsewhere in `244`, per my brief's specific ask: no.**
I read section 3's "What is contested" heading suspiciously, since "contested" is a word from the
false premise. It is the standard consolidation-ledger heading (agreed / contested / closed) that
`how-to-run-a-panel.md` requires of every consolidation, not a residue of the false instruction, and
`244`'s own C1 resolves the actual `241`-vs-`243` disagreement *against* treating it as contested: it
reads it as decided, in `243`'s favour, with the reopening condition stated. The false premise did
not shape the file's structure or its conclusions anywhere I found.

## Instrument 1: entailment, driven forward from the sources

I read `241`, `242` and `243` in full before opening `244`'s corresponding sections, and opened
every registry row `244` quotes rather than trusting its rendering. Everything checked reproduced:

- **R2's `says`** (`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`)
  quoted identically by `241`, `243` and `244`, verified against that row's `says` field,
  word for word, biconditional included.
- **R2's `promotion`** ("the format spine's closed-concept-open-inventory shape one tier down")
  quoted identically by `241` and confirmed against the row.
- **R3's `says` and `promotion`**
  (`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`),
  including the exact sentence `244` uses to demote `241`'s "fixed by a ratified count" claim in
  C3: "Two instances agree about the intersection of their claims and never the union, so the count
  is outside what this ratifies." Present verbatim in the row's `promotion` field. `244`'s reading,
  that the ten appears in `says` as an incidental description inside an arithmetic argument about
  `Width` rather than as a ratified commitment to the arity, is a fair reading of that same field:
  the row's `says` states the arithmetic ("three of the ten associated constants carry values it
  cannot hold at all") and its own scope statement places the count question outside what it
  ratifies. This is `244`'s judgement rather than a bare fact, and `244` says so.
- **`241`'s central quote** ("Given a candidate, it returns either a total assignment ... or the
  name of a coordinate the candidate failed to fix") matches `241` section 2 exactly.
- **`243`'s central quote** ("at the candidate tier, admission is a total map onto a quotient")
  matches `243` section 5 exactly.
- **The three separators against the failure arm**, re-derived from source rather than from `244`'s
  prose:
  - Separator one (`E0046`): `243`'s own committed probe output, quoted accurately.
  - Separator two (R2's grammar): the biconditional-versus-conditional reading holds up on a fresh
    reading of the row; I have nothing to add or subtract.
  - Separator three (mine, `244`'s own instrument): I read `slots.rs:65-165` directly rather than
    trusting the probe's classification, and confirmed by hand that `Slots::ADMITTED`'s five
    assertions reference only `MIN`, `MAX` and `WIDTH`, all three fixed by `Slots` itself. Zero
    reference `PHASE_DEN`, `RADIX`, `SIGNED`, `BASE`, `SLOPE` or `MAGNITUDES`. `244`'s probe,
    `244_probes/no_refusal_names_an_unfixed_coordinate.sh`, reproduces exactly on re-run, both
    controls firing as claimed.
  - `244`'s claim that "nobody has exhibited an inhabitant" of the failure arm: I grepped `241` for
    `"failed to fix"` myself and got one hit, the shape statement itself. Confirmed.
- **Section 1.5's cross-file connection** (241 section 4's "vacuously broad" caveat is present in
  section 4 and absent from section 7, and `probe::the_collapsed_declaration_cannot_be_made_to_fail`
  is `standing = "sound"` and establishes exactly what `244` says it does, verified against
  that row's own fields). This is a real connection neither `241` nor `243` made, and it
  is a legitimate consolidation contribution rather than an invented one: `241` itself states the
  concept/shipped-tier split in section 4 and does not carry it into section 7, and the probe result
  is independently on the record.
- **L1's table** (five of six admission questions carry a standing answer row; none of the five is
  cited by any of the three files): re-ran `244_probes/the_standing_answers_nobody_cited.sh` fresh
  and it reproduces the table and the citation counts exactly, control included.
- **L2's provenance claim** (R2's provenance is `225`/`226`, the standing proposal's is `73`/`74`,
  so `241`'s route is disjoint): verified both provenance lists directly against the registry.
- **L3's mechanism claim** (`answered` resolves only from question to ruling; a `proposal`'s
  `answers` list has no forward-pointing counterpart on the question row): verified by reading
  `question::is_admission_a_predicate_or_a_location` raw. It carries no `answered` field and no
  pointer to the proposal that answers it; the proposal is discoverable only from its own side.
- **Section 7's accounting** (23 source anchors, 30 consolidation anchors, three losses, ten new,
  two of the ten flagged as pattern artifacts): re-ran `244_probes/anchor_diff.sh` and it reproduces
  the exact numbers and the exact lists, including the two false "new" entries from the hyphen and
  digit-run pattern quirks `244` names and does not hide.

**Nothing in this pass contradicts a load-bearing claim.** Every quotation opens to what it claims
to open to, and every re-run reproduces.

## A methodological correction, reported against myself rather than against `244`

My first pass re-ran `244_probes/the_bound_field_and_the_blind_cut.sh` at my worktree's current
HEAD and got `rows carrying a bound, whole file : 23` and `question rows, whole file : 106` against
`244`'s committed `22` and `105`. I read this as a defect for long enough to write it up before
checking the obvious thing: my worktree's HEAD (`98a4b7ee`) is a **merge** of `244`'s own branch tip
(`cc19b122`, itself directly on top of the tree `244` cites throughout, `800e120a`) with unrelated
parallel canon work that landed on `dev` in the meantime (the additive-identity phase repair, the
matlab_fi parity work, several closed design rounds — `86` files, `5890` insertions). That work
added one new `the_number_system`-unrelated question row carrying a `bound`, which is where the
extra `1` came from.

Checked out at the tree `244` actually cites, `800e120a`, the script reproduces `22` and `105`
exactly, matching the committed `output_bound_and_blind.txt` byte for byte. I also confirmed the
six admission rulings and the six admission proposals I had already verified against my current HEAD
are untouched by the intervening merge (`git diff 800e120a HEAD -- mock/registry/ruling.toml`
touches two unrelated rows for banned-word cleanup only; `proposal.toml`, `question.toml`,
`dimension.toml`, `probe.toml` and `retirement.toml` gain only new rows for the unrelated topic),
so none of my other registry-based verifications above needed re-checking at the earlier tree.

**This is not a finding against `244`.** Every predicate in `244`'s section 6 correctly names its
tree (`800e120a`), which is exactly the discipline that makes a claim like this checkable at all; I
had simply re-run the instrument at the wrong ref. Recorded here because a wrong finding half-formed
and then caught is worth more on the record than a clean pass with no trace of the near-miss, and
because it is itself an instance of `a-claim-about-a-merge-is-measured-on-the-merge.md`'s point from
the other direction: a claim measured on an old tree is not automatically wrong just because a later,
merged tree gives a different number.

## Instrument 2: anchor-set difference, and where `244`'s own instrument cannot see

Re-ran `244_probes/anchor_diff.sh` fresh: reproduces `23` source anchors, `30` consolidation
anchors, the three losses, the ten "new," byte for byte against the committed output.

**But the pattern it uses, `P='[0-9]+_[a-z_]+\.md|[a-z_]+\.(rs|toml):[0-9]+|[A-Za-z0-9_]+\.(rs|toml)'`,
cannot match a registry row slug at all.** A row's qualified name, a `ruling` row named `foo` or a
`proposal` row named `bar` in the syntax `cargo mock query` accepts is exactly the anchor kind my
brief names explicitly ("row slugs"), and none of that shape fits the pattern's three alternatives. I wrote a second
instrument, `245_probes/slug_anchor_diff.sh`, committed with its output, extending the same
comm-based diff to a slug pattern with a firing positive control on both sides.

```
slug anchors in the three sources : 13
slug anchors in the consolidation  : 23 (whole file)

LOST (cited by a source, cited nowhere in 244):
  ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves
  ruling::the_canon_does_not_police_what_shape_a_law_takes
  ruling::the_option_set_is_not_a_boundary
  ruling::the_panel_finishes_the_canon_without_him
  ruling::there_is_no_universal_answer_take_the_win_and_gate_it
```

**All five are methodological citations from `241`'s own apparatus, not substantive admission-topic
findings.** `behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` and
`the_option_set_is_not_a_boundary` and `there_is_no_universal_answer_take_the_win_and_gate_it` are
`241`'s precedent for how to handle a "which single policy governs a category" question (dissolve,
do not pick a letter); `the_canon_does_not_police_what_shape_a_law_takes` is `241`'s ground for
declining to attack whether the resolution shape is buildable; `the_panel_finishes_the_canon_without_him`
is `241`'s R4, the epistemic floor for deriving answers where op's words do not reach. None of the
five carries content specific to admission that `244` relies on without attribution, and I checked
each: `244` does not silently reuse the "dissolve rather than pick a letter" move, the
buildability disclaimer, or the derive-from-intent floor anywhere without its own grounding.

**So this is a real, reproducible gap in `244`'s own instrument's coverage, and a real, checkable
absence in `244`'s citation list, but it is not a defect in `244`'s substance.** Worth recording for
two reasons: `244`'s section 7 states "23" and "30" as if the pattern captured everything a reader
would follow, when by its own instrument's construction it could not see the registry-slug half of
the citation graph at all; and the pattern is, by `244`'s own account, "my brief's," meaning the
gap was inherited from the dispatching brief rather than chosen. Either way, a reader trusting
section 7's accounting as complete would be trusting an instrument blind to thirteen of its own
anchors.

## Instrument 3: citations in both directions

Beyond the registry-row checks in instrument 1 (all of which pass in both directions: cited text
matches source text, and source claims used by `244` are quoted or fairly paraphrased rather than
silently reworded into something stronger), I checked the one place a paraphrase could smuggle in
content the source does not carry: `244`'s "carries it forward" line in section 1.4, attributing to
`243`'s section 5 the sentence "a predicate and a location are folds over the pair rather than over
a sum." `243` does not use the word "folds" anywhere in section 5; it says the obligations "compose
with a quotient map cleanly and they cannot be arms of a sum." `244` is explicit that this is a
synthesis ("carries it forward") rather than a quotation, and the synthesis is a fair extension of
what `243` actually argued rather than an invention. Not a defect.

## Instrument 4: predicate-dimension diff

Walked every predicate in `244`'s section 6 against the corresponding source predicate, where one
exists. The only apparent widening I found (`244`'s "Proof" marker replacing `241`'s
`total_width: W any: construction` / `radix: radix any: construction` phrasing for the ten-coordinate
count) is not a widening at all: it is `244` correctly adopting the notation op ratified at `217`
("A proof carries a marker saying the argument is width-free by construction, rather than being
dressed as a sweep that happened to stop at three widths"), which postdates `241`'s filing. Section
2's agreement entries (A1 through A10) intersect over **values**, not dimension names, exactly as the
workspace rule demands, including the one place it matters most: A2 correctly credits `241` alone
for the four-algebra sweep and states `242`'s contribution as a source-reading agreement over an
empty measured intersection, rather than folding the two into one shared sweep. This is the strongest
mechanically-disciplined section of the file and I found nothing to correct in it.

## An absence claim I re-executed: L4's fourth phrase

`244` claims `242`'s absence check ("`MAGNITUDES = 0`", "`empty representable set`", `RADIX = 1`"
appear nowhere in the panel directory) holds for those three and fails for a fourth ("radix one"),
which appears in `archive/OLD_CANON_CANDIDATE.md`. Re-executed at tree `800e120a` (the file is
byte-identical between `800e120a` and my current HEAD, so the tree choice does not matter here):
confirmed exactly. `"radix one"` appears in `242`, `244` and `archive/OLD_CANON_CANDIDATE.md` and
nowhere else; the other three phrases appear only in `242` and `244`. The positive control (files
matching `"representable set"`) is `59` at tree `800e120a`, matching `244`'s stated figure exactly
once the tree is fixed correctly (my first, uncorrected pass found `60`, which is `59` plus `244`'s
own file, present only at the later tree).

**The quoted passage from the archive is accurate**, word for word against `archive/OLD_CANON_CANDIDATE.md`
around line 990, elision marks in the right places.

## A real, low-severity defect: the term-frequency census in L4

`244` states the archive "carries 61 uses of `sealed`, 8 of `value-unique`, 6 of `NonZero` and 6 of
`AtLeastTwo`." I counted all four with a plain substring `grep -o` over the file:

```
sealed        : 61   (matches)
value-unique  :  8   (matches)
NonZero       : 13   (does NOT match; claimed 6)
AtLeastTwo    :  6   (matches)
```

Three of four reproduce exactly under plain substring counting, which is strong evidence that is
the method used. `NonZero` does not reproduce under that method (`13`, not `6`), nor under a
stricter word-boundary count that excludes `NonZeroCarrier`, `NonZeroUsize` and `NonZeroable`
(`5`, not `6`). Neither reading gives `6`. This does not touch the substance of L4: the quoted
passage stands, the general claim that the archive built the same obligation class as type-level
bounds is true regardless of the exact count, and the finding this census merely corroborates
(that `242`'s and `241`'s obligations are a rediscovery rather than a discovery) does not depend on
whether `NonZero` appears 5, 6 or 13 times. It is worth naming because it is exactly the class of
hand-check `a-hand-check-becomes-a-test-every-time.md` asks to be committed as a script with its
output, and every other count in the file's evidence sections is; this one, alone among them, was
not, and it is the one that turned out wrong. Severity: low, corroborating rather than load-bearing.

## The most consequential thing I was asked to check: is L2 really "the sitting's one promotable result"

`244`'s L2 argues, correctly and with a well-supported disjoint-route case, that `241`'s central
shape claim (admission returns a coordinate assignment rather than a boolean) is an independent
second instance of the standing proposal `proposal::admission_returns_a_coordinate_rather_than_a_verdict`
(`standing = "one_expert"`, provenance `73`/`74`), and that nobody in the sitting named the row. It
then writes: "So the sitting's one promotable result is a promotion nobody proposed."

**That sentence overclaims. There is a second one, built from the exact apparatus `244` itself
uses for L2, sitting in the same L1 table `244` already constructed, that `244` did not check.**

`244`'s L1 table lists, against `one_word_or_two_for_is_a_number_system`:
`proposal::membership_and_hosting_are_two_questions` (`standing = "one_expert"`, provenance
`73`/`74`, the identical provenance pair as L2's proposal), cited zero times across `241`, `242`
and `243`. That row's `says`: "Whether something is a number system and whether this implementation
can carry one are different questions. The first is about structure and is answered by locating the
candidate on the chain of choices; the second is about residue at runtime and is answered by what a
value at rest may carry. ... **a system the implementation cannot host is still a system**."

`241`'s Q31 answer, reached blind (its first two commits precede opening anything under
`mock/research/`): "Two, and the second is not a second admission procedure. It is a predicate over
the first one's output. Under the resolution reading, being a number system is fixing the
coordinates. That is target-free ... Being hostable is a predicate over the values those coordinates
take, against a particular target's realisation ladder. `Slots::ADMITTED` in the shipped crate is
exactly this ... a width of 63 bits is a perfectly good coordinate assignment that this stack cannot
carry."

Read those two side by side. "Fixing the coordinates" against "locating the candidate on the chain
of choices" is the same structural half. "A predicate over the values those coordinates take,
against a particular target" against "residue at runtime ... what a value at rest may carry" is the
same hosting half. And `241`'s own worked example, a 63-bit width that **is** a valid coordinate
assignment and **is not** hostable by this stack, is a direct instance of "a system the
implementation cannot host is still a system," independently constructed rather than borrowed.

**The route is disjoint by the same standard `244` applies to L2.** `241` reaches Q31 from R2 (the
admission-rule ruling, provenance `225`/`226`) plus the shipped `Slots::ADMITTED` mechanism, neither
of which `73` (provenance `73`/`74`, predating `225`/`226`) could have used. This is exactly the
argument L2 makes for the *other* question, reused here with the names changed.

I checked whether `241` cites this proposal anywhere in its own reconciliation, the way it names
`admission_returns_a_coordinate_rather_than_a_verdict`'s content for Q30 without naming the row:
it does not. Its reconciliation's "What this does to my own answers" section lists Q30, Q22, Q29
and Q21; **Q31 is absent from that section entirely**, so `241` itself never connected its own Q31
answer to `73`/`74` even after reading them. The connection is real and nobody in the sitting, `244`
included, made it.

**I checked the other two uncited proposals in the same table for the same possibility and found
neither holds.** `proposal::the_concepts_edge_is_not_an_order_and_wrapping_is_the_test` argues from
order-compatibility and wrapping addition specifically; neither `241`'s Q21 route (the ambient
algebra is not a coordinate, measured over four algebras) nor `242`'s (the two-element Boolean set
is `Unsigned<1>` and already admitted) touches order or wrapping at all. No overlap.
`proposal::a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts` argues that a
system must expose its ambient's own law inventory conjoined with reduction verdicts; `242`'s fourth
Q29 option (`RADIX >= 2`, `MAGNITUDES >= 1`, `PHASE_DEN != 0`) is about what must hold of the
*declarations* for the ratified predicate to be meaningful, a different object from an exposed law
inventory. `244`'s own C5 treatment of this pair ("not competing and neither has a second instance")
is right. Only the `membership_and_hosting` pairing survives the check.

**One caveat that keeps this from being a clean second L2.** The proposal's own `note` says "the
hosting half is a different author's and is conditional on an open question of op's." I did not
track down which open question that is; it may bear on whether the hosting half is ready to be
raised to `two_experts` even with a genuine second instance in hand. That does not touch the
disjoint-instance finding itself, only what a later seat does with it.

**What this means for `244`'s file.** Not a wrongness in anything `244` asserts; `244` never claims
to have checked all four remaining uncited rows for the L2 treatment, only L1's citation table and
L2's specific case. But the sentence "the sitting's one promotable result is a promotion nobody
proposed" is stated as a completeness claim and it is not complete. A later seat working from `244`
should record `241`'s Q31 answer as a second candidate promotion for
`proposal::membership_and_hosting_are_two_questions` alongside L2's, with the same "needs an
independent second reader" caveat `244` correctly applies to L2's own tier.

## What I checked and could not fault

The core located disagreement (section 1, C1): sound, and the reopening condition ("an exhibited
candidate that has fixed some coordinates, failed to fix one, and receives that coordinate's name")
is precisely stated and would in fact reopen it if met. C3's demotion of `241`'s "fixed by a
ratified count" overclaim: sound, correctly caveated as needing a second independent reading of a
ratified row rather than presented as settled. L5's resolution of `242`'s provenance-hole worry:
I spot-checked two of the eighteen no-`ratified_by` rows (R2's row itself carries `ratified_by =
"experts"` and a `quote` field is not expected of it under `244`'s own classification, which is
correct; I did not re-run `244_probes/the_ratified_rows_with_no_ratifier.sh` against all eighteen
individually, so this is a spot check rather than a full re-derivation). The two self-disclosed
instrument defects (the one-sided control, the pre-run accounting block): both match what the
committed scripts and their headers actually show.

## Coverage

**Read in full:** `241`, `242`, `243`, `244`, `95`, `181`, `217`, `AGREEMENTS.md` sections 0 and 1,
`INTENTS.md` I13 (already loaded from workspace memory, cross-checked against the file). **Read from
the registry, raw and by query:** `question.toml` (all eighteen `the_number_system` rows, raw, plus
the two rows this file's L2/promotion analysis turns on),  `ruling.toml` (R1, R2, R3, and a diff of
the whole file between `800e120a` and HEAD), `proposal.toml` (all six admission-topic answer rows,
raw, plus a diff of the whole file), `probe.toml` (`the_collapsed_declaration_cannot_be_made_to_fail`,
raw), `dimension.toml` and `retirement.toml` (diffed, not read whole). **Read from source:**
`arvo-format/src/slots.rs` lines 1-200 in full, including `Slots::ADMITTED` and `is_admissible`
verbatim, to check separator three by hand rather than trusting the probe alone. **Read from the
repository:** `archive/OLD_CANON_CANDIDATE.md` around line 990, and its full text for the four
term-frequency counts. **Not read:** the other roughly 440 panel files `244` itself declined to
open; `73` and `74` in full (I worked from their provenance-listed content and from `244`'s own
quotations, the same limitation `244` names against its own L2); any consumer survey; the mockspace
bench harness question, out of scope for this dispatch.

**What a reader should distrust most in this file.** The L2-adjacent promotion finding (the
"membership and hosting" second instance): built the same afternoon, by one reader, and it wants the
same two-independent-agreements bar `244` correctly applies to its own section 1.2 and L2. And the
low-severity NonZero count, which I am confident is wrong but have not asked anyone why it was
written that way, since the author is not resumable from here.

## Verdict

**The consolidation can stand as the sitting's compression.** Every load-bearing entailment,
citation and predicate I checked, across all four instruments and against the sources and the
registry directly rather than against `244`'s rendering of them, reproduced. The two defects `244`
disclosed about its own instruments are handled correctly. The false premise in my own brief (which
was also `244`'s brief) is correctly refused and does not leak into the file's structure.

**Two corrections are owed before a later seat treats section 4 as exhaustive.** L2's "one
promotable result" is better stated as "at least one, and a second candidate of the identical shape
sits uninspected in the same table this file built" (`241`'s Q31 answer against
`proposal::membership_and_hosting_are_two_questions`), and L4's term-frequency aside should read `13`
or `5` for `NonZero`, not `6`, whichever counting convention a later seat prefers, committed as a
script this time. Neither correction touches the file's central argument, the located disagreement,
or any of the five other agreed findings in section 2.

## Paths opened

`241`, `242`, `243`, `244` in full, `95`, `181`, `217` in full, `AGREEMENTS.md` sections 0-1 and
`87` directly, `INTENTS.md` (workspace-memory cross-check). `mock/registry/{ruling,proposal,
question,probe,dimension,retirement}.toml`, raw and diffed between `800e120a` and HEAD.
`mock/crates/arvo-format/src/slots.rs` lines 1-200. `archive/OLD_CANON_CANDIDATE.md` around line
990 and its full-text term-frequency counts. All of `244_probes/` re-run; `241_probes/` and
`242_probes/` listed but not re-run (their outputs are quoted and re-derived by `243` and `244`
already, and my own separator-three check reads the source directly rather than trusting any of the
three probe layers stacked on it). `cargo test --workspace` at HEAD. My own
`245_probes/slug_anchor_diff.sh`, committed with `output_slug_anchor_diff.txt`.
