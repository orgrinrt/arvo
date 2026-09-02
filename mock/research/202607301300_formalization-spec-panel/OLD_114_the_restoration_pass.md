# The restoration pass: what went back into the standing base, and what did not

**Date:** 2026-08-05
**Position in the panel:** after `111_leroy_what_the_restoration_actually_carries.md`,
`112_the_op_material_sweep.md` and `113_the_decision_register_diff.md`, the three audits of
`110_consolidation_eleven.md`. This file is not a fourth audit. It is the record of applying what the
three found, and of what it declined to apply.

Three instruments ran over the eleventh consolidation in one day and each found what the one before it
could not. File 111 checked the restorations against their sources and found two entailment failures plus
six claims resting on repetition. File 112 established that the working list of op's own files named
twelve and was missing eleven, then walked ninety-three items out of the twenty-three that exist. File 113
diffed op's numbered decision register, which no instrument had ever touched, because the standing base's
own enumeration of the ratified rung excluded the two topic files it lives in.

**Twenty-one repairs landed in `110`, each marked inline** in a blockquote beginning "Correction, file
114", stating what was wrong and what the text now says. Nothing was silently overwritten. Two earlier
blockquotes marked "Correction, file 111" were already in the file and are untouched except for one
formatting splice repaired below. The document went from 4,928 lines to 5,867, `wc -l` on 2026-08-05.

**What this pass did not do, stated before what it did, because it is the more load-bearing half.** It
made no design call. Where a repair would have required one, the item went to `110`'s open list addressed
to op rather than being decided. Where an audit named something absent and the source could not be found
saying it, the gap is recorded here as unresolved rather than reconstructed.

---

## 1. The repair the others depend on

`110:154-160` enumerated the ratified rung and omitted op's two topic files. File 113 identifies that as
the structural cause of the entire register drift: **the standing base's own definition of its oracle
excluded op's decision register, so nobody diffed it**, and the agreement rate by number came out at
fifteen of forty-six (`113:14-20`, `113:70-90`). The document cited the talk file once in four thousand
nine hundred lines and the spec file zero times.

Section 0.4 now names three topic files rather than two. The third,
`202607301000_topic.inherited-state-from-the-formalization-round.md`, is added because it is where the
twenty-three prior-round decisions the other two cite by number are declared (`113:44-50`), and because
without it every citation of the form "D23" in the crate table resolves nowhere. **All three are on the
ratified rung and all three are frozen**: they were read at source for this pass and not edited.

The twenty-three op checkpoints were already correctly listed, which file 112 confirms
(`112:88`). What was added beside them is the finding that made the list misleading: **a roster entry is
not a citation.** Seven of the twenty-three (`08b`, `12b`, `13b`, `16b`, `16c`, `17b`, `24b`) appeared in
`110` exactly once each, and the single appearance was the roster (`112:112-128`). A reader had no way to
see the difference between an op file the document draws content from and one it merely lists as
governing. That sentence is now in the provenance section, and all seven now have content cited.

The persona list's trailing "and `101b`'s siblings" is struck (`112:90-92`); the list is nine files and
that is the whole list.

**And the register's own defect is recorded rather than repaired**, because repairing it is op's. The
inherited-state file carries two overlapping `D1` through `D4` sequences, both live and both cited, and
the talk file's question grid runs rows `D1` through `D3` on the same prefix, so there is no decision D3
at all (`113:32-58`). `110`'s crate table cites "`arvo-shape` | D1-D4" and a reader following that into
the talk file reads a forbidden-feature ruling as a shape-crate ratification. **A number that is not
unique is worse than no number.** File 113's round-qualified citation form is carried as its suggestion,
not adopted, and the item is on the open list and on the loudest-for-op list, because every disposition
below is keyed on a number.

---

## 2. What was restored

### 2.1 Op's acceptance test, and the two sentences either side of it

File 112's headline, and it was correct: **op has stated the review's termination criteria three times in
three files and the standing base carried none of the three** (`112:18-23`).

A new section 0.5 carries all three, verbatim where short enough, from op's own files.

**The stopping condition and the mode** (`13c:38-42`), verbatim, with the four-step cycle read out at
`13c:44-53`. The third step is **a fresh read: a member given only the consolidation, with the transcripts
withheld.** That is the acceptance test for a standalone consolidation, and `110` independently arrived at
the property that step consumes and had to name it as its own invention, because the procedure that tests
it left the record eight consolidations earlier. The instrument has been run once, at file 12, and
`12b:18-21` records that it produced the widest finding in the review. Judging a consolidation by hand,
which is what files 111 through 113 did, is a different act.

**The post-canon sequence** (`68b:14-21`), verbatim, with `79b:64-69`'s statement of which of its four
phases the verification mandate binds. `110` carried the prohibition half of `68b` repeatedly and none of
the sequence; `stub` occurred eight times in it, every one meaning a documentation stub
(`112:197-199`). The consequence is direct: **the taxonomy round is briefed off the standing base**, so an
op decision about the crate structure that is absent from the standing base is one that round re-invents.

**The end state** (`70b:52-57`): a full spec that is proven, valid, efficient and ergonomic, invisible for
the most part to downstream consumers while doing real work underneath and lowering transparently to
optimal instructions. This is a criterion, not a process note, and it is the consumer-facing half of the
bar. Section 0.1's standard covers optimal, representative and representable; this covers invisible and
ergonomic. **The two together are the standard.**

Section 0.5 also carries op's checkpoint cadence (`04b:42-43`) with both later restatements, and op's
licence to argue against a ratified call (`04b:72-74`), whose operative half is the qualifier "provided
the argument is made rather than asserted". `110` carried the first half of that pair and not the second,
which is why section 1.27's reopening hedges on a re-derivation licence where op's own text authorises it
outright.

### 2.2 D72, the crate split

Absent entirely. `arvo-numeral`, `arvo-policy` and `arvo-lowering` each returned zero hits in `110`,
searched four ways (`113:94-128`), while `110`'s open list restored "what `arvo-numeric` ends up
containing once the numeral, policy and lowering definitions move out", **presupposing a move whose text
was nowhere.**

Section 1.25 now carries D72 with its table transcribed cell by cell from `spec:291-300`, the seven-row
form. Two of its cells name axes ratified out at `39b` after D72 was written (`Growth` on `arvo-policy`,
`Widening` on `arvo-lowering`), and that is marked at the table rather than silently corrected, because
the staleness is on the register's side and the crates are not stale.

**Where file 112 and file 113 disagree, file 113 wins and its correction is recorded in `110`.** File 112
stated D72 as demonstrably carried by section 1.25's crate table; file 113 checked at source and found
that table is the eleven-row periphery taxonomy, keyed on a different numbering, with not one of D72's
rows in it (`113:111-117`). The later reading shows its work.

Restored with it: **D23, D32 and D33**, the placement calls D72 finishes; **file 09's harder enforcement
result**, that the crate owning `Number` can still condition a law on `L` and the split does not stop it,
with the verified `LogicalNumber` shape that does close it and its stated architectural cost; and **op's
reserved call at `08b:47-51`**, which he named as his in those words, made conditional on the enforcement
answer, and has never made. The three-contract split ships in the ratified trait table with no
ratification marked on the split itself.

### 2.3 The rest of the register

Restored from source, each with its `file:line`: **D52** (compositions are public and bindable, so the
presets are the default path and not the only one, without which Thread A's nominal-constructor work is
unintelligible), **D53**'s alias half, **D54**'s axis-sorting test (invoked by name in `110` and stated
nowhere), **D56** (no gratuitous abbreviation, which governs everything the taxonomy round mints),
**D63**'s `Direction` and **D64**'s `Quantisation` and `Resolution` vocabularies, **D66/D67**'s shipping
half, **D68**'s ratified four flat members with its supersession stated, **D70**, **D71**'s two lost
consequences, **D73**'s marker half, **D74**'s accepted trade, **D75**'s rename and the ladder's rung
names, **D31** and **D48**'s public-spelling constraint.

**D65's supersession was recorded under D69's number.** `110:488` attributed to D69 a sentence that is
D65's own reasoning verbatim; both were overturned at `30b`, and D69's content is the ten-axis table
(`113:183-205`). Both are now named.

### 2.4 Op's material outside the register

`Cold`'s cold-path meaning, elided by an ellipsis inside the passage `110` flags as its flagship
restoration of ratified material (`112:324-346`). Op's own sentence is "Cold also tells us it's seldom
computed or used, it's on a cold path", and `68b:76-78` draws the consequence: `Cold` carries two
meanings, cold storage and cold path, and the second is what licenses it paying more compute than `Warm`.
The bench target survived in `110` and the reason for it did not.

The **`WideBits` hole** op named at the fourth checkpoint (`12b:46-54`), rediscovered forty files later
and credited to file 68. The gating half is genuinely superseded by `68b:23-28`; the technical half is the
same hole at the same mechanism. **The cost of the drop was not the sentence, it was the rediscovery.**

The **four `16b` and `16c` posture directives**, absent by seven searches (`112:232-234`): the existing
code is irrelevant and everything is being rewritten; the spec is the subject; every member owes its
boundary a design rather than an observation; and novel answers to a boundary outrank observations that
the boundary exists. The technical half of `16b` had survived into `110` unattributed as section 1.25's
"arvo grows no build harness of its own"; the instructions that produced it had not.

**`12b`'s hold on the arithmetic-fidelity axis, and his statement that the ten-axis completeness claim
stays attackable.** The mechanisms were droplisted correctly and in detail; the hold and the
attackability sentence were separate and absent.

**`17b`'s fidelity principle**, that a fidelity grant is checked rather than asserted. File 112 partially
corrected file 111 here and the correction stands: the mechanism that discharges it is present as a
droplist entry's closing clause, so the adoption is satisfied in substance, **and no sentence stated the
principle**, so a reader could learn only that one particular way of not checking failed. The principle is
now stated at the entry it governs.

**Three attribution repairs on op's own text.** The constructive-deliverable directive was cited to
`40:610-612` and is `24b:18-30`; the convergence directive to `58:962` and is `30b:40-57`; the novelty
posture to `78:790-791` and is `34b:38-69` (`112:414-427`). Sources attached, last carriers kept beside
them, per the rule file 111 states at `111:544-551`.

### 2.5 The claims resting on nothing but repetition

**The spine rule's eleven firings are enumerated rather than downgraded**, from `63:106-123`. File 111
found the count had no list behind it and traced through `78:120-129` to `68:98-101`, which reads "Nine
occurrences stand from the sixth consolidation's count", so the chain terminated in a count rather than a
list. **The list did exist**, one consolidation below the count that replaced it, which is the same shape
of loss the rest of the document repairs. Enumerating was therefore possible and downgrading the
conclusion was not necessary.

**The transfer refutation's second compiled support**, absorption-freedom at exponent span `p` against
`p + 1` with the bans in force, restored from `68:451-455` along with the four-legs analysis it rests on
and the necessary-promoted-to-sufficient statement of what it refutes. This is the sharper of the two
supports and it is the evidence for the first of the three `unstable-features.md` wording edits `110`
calls its largest single item for op. **Op was being asked to edit a ratified workspace rule on half the
evidence the panel produced for it.**

**The ten axes are tabled from D69 at `talk:1621-1641` with the three ratified out at `39b` marked**, and
the two live uses of the stale count are rewritten so neither rests on it. One of them was a premise in a
live finding (`113:414-425`). **No replacement count is asserted**, because the trait table's members and
D69's axes are different populations and a count that cannot be checked against a list is exactly what
`110`'s own seal section refuses.

**`Quantisation`, `Direction` and the `Resolution` members** are declared at section 1.23. Of the six
terms file 111 found used and undefined under `110`'s own completeness line, three are now defined from
source (`Quantisation`, `Direction`, the `Resolution` members), one is tabled ("the ten axes"), one is
enumerated (the spine rule's eleven) and one is restored (the second support). A seventh, D54's
axis-sorting test, was found by file 113 under the same line and is restored with them.

**The `Ranged` coordinates' lost figures**: both negative controls' numbers, the sentence recording that
two of the six coordinates collapse into one (which is what makes the index set six rather than seven, and
is load-bearing for anyone re-running the argument), and the sentence recording that the sixth
consolidation's own models cleared the saturation threshold by luck rather than by design.

**Thread B's two positive results** (`11:509-520`), restored so that a member opening the thread reads a
mechanism rather than three costs alone.

### 2.6 Three silent supersessions made explicit

A supersession that is stated is legitimate under `108b:11-20`; one that is merely omitted is a drop
wearing better clothes, and a later reader cannot tell it from attrition. Three are now stated: **D68**
(the nested `Numeral` shape against op's ratified four flat members, with the disposition itself left to
op), **partial associativity** (retired with the three-relation ladder, resolving from inside `110` the
pair file 111 could not distinguish), and **`ffl`** (removed from the physical-grounds row, probably
correctly, silently).

---

## 3. What was deliberately left for op

Each of these is a call this pass could have made and did not. All are on `110`'s open list and the first
four are on its loudest-for-op list.

1. **The register's disambiguation convention.** Two live `D1`-`D4` sequences plus a colliding question
   grid. Every disposition in `110` is keyed on a number, so this gates the rest.
2. **The fused-versus-split call reserved at `08b:47-51`.** Two readings survive and the evidence does not
   force one: practice has exercised the split through eighty files without producing file 08's failure,
   and a call op explicitly reserved cannot be made by practice. File 112 leans to the second and holds
   the first as live; this pass carries both.
3. **Whether the nested `Numeral` shape supersedes D68's ratified flat members.** Both shapes are op's.
   The nesting's argument (`Underflow` has no bottom to fall off under a constant exponent) postdates
   D68 and is sound; whether that constitutes a supersession of a ratified call is not this pass's to say.
4. **The round's seventeen open rows, never reconciled with the panel's open list.** Two reach `110` only
   because a different route rediscovered them, cited to file 11 rather than to the round.
5. **Which checkpoint cadence is current**, every two experts or the two later four-shaped restatements.
   The record cannot show whether the drift was op's or the dispatcher's, which is the point.
6. **The `unstable-features.md` 28.45-second figure's wording**, a fourth item alongside the three already
   on the loudest list, with the proposal stated as `63:456-460` states it.
7. **The ordering of the four `Resolution` members "by how much they lie."** `110` asserts an ordering;
   no source states one. The members are enumerated from source and the ordering is left marked as the
   document's own reading rather than promoted or deleted.

---

## 4. What could not be resolved

Recorded as unresolved rather than reconstructed, because guessing at op's call is worse than a labelled
gap.

**Nothing an audit named as absent was found to be unrecoverable.** Every item files 111, 112 and 113
cited by `file:line` was read at that range and restored from it. That is a smaller claim than it sounds:
all three audits carry citations on both sides of every finding, which is why the work was largely
mechanical.

**What remains genuinely open is instrumental rather than textual, and it is three sweeps nobody has
run.** They are keyed on different populations and none substitutes for another. First, the member-file
and probe-directory sweep for material no consolidation ever absorbed, named owed at `109:643-646`,
sampled six times by file 111 and confirmed untouched at `112:541-546`. Second, the register diff run
against the inherited-state topic file, which carries fifty-two decisions plus the duplicate sequence and
of which `110` cites twenty-one by number; **it has never been diffed against anything** (`113:450-457`).
Third, file 111's restoration ledger, one row per restored item carrying the source range that
*established* the statement rather than the last carrier (`111:474-501`). **The ledger was not built by
this pass.** Building it faithfully means re-deriving the establishing source for roughly one hundred and
twenty rows, which is a dispatch and not an edit, and a ledger assembled from the last carriers would
reproduce the defect it exists to catch. All three are on `110`'s open list with artifacts named.

**Two smaller things this pass declines to settle.** The nine items `110` already names and does not
answer (`110`'s section 8) are unchanged; nothing in the three audits bears on them. And the line-number
discrepancy between `112:498` and `113:307` over where D54 is invoked is moot after this edit, since every
line number in the document has moved; it is noted only because it is the kind of thing that reads as a
disagreement when it is arithmetic.

**One formatting defect was repaired rather than reported.** The "Correction, file 111" blockquote in
section 1.7 had its closing sentence spliced outside the quote mid-paragraph, so a sentence of ordinary
prose read as part of the correction. The blockquote boundary is fixed and no word changed.

---

## 5. Does `110` now satisfy the property it claims

The property, at `110:29-35`: "A reader can reconstruct the design from this file alone, without opening
any prior consolidation. That is the whole point of the exercise, and it is the one measurable property
this document should be judged on."

**Against the vocabulary bar, yes, and it did not before.** File 111 answered "not yet, and by a small
margin", and named the margin: four of the design's own vocabulary items used and not defined, of which
`Quantisation` is the sole content of `Policy` in the ratified trait table. Those four are defined, the
fifth (the spine rule's count) is enumerated, the sixth (the transfer refutation's second support) is
restored, and the seventh file 113 found under the same line is restored. A reader can now reconstruct
`Policy` from this file.

**Against the register bar, it is better and it is not clean.** Fifteen of forty-six decisions agreed by
number before this pass; the absent eleven and the weakened four are restored, and the seven superseded
now carry their dispositions. But the register's identifiers still collide, which means **a reader
following a D-number out of `110` into the topic files can still land on the wrong decision**, and no edit
to `110` fixes that. The measurable property is intact within the document and leaks at its citations.

**Against op's own bar, the question is not answerable from inside.** Op's acceptance test is step three of
his cycle: a member given only the consolidation with the transcripts withheld. That has never been run on
any consolidation, and running it is the one thing that would settle this. It is now in the standing base
to be run, which it was not before, and this file is not it: this pass read the audits, the sources and
the document together, which is the opposite of the conditions the test names.

**And the earmark is still not due.** `108b:184-193` sequences it as consolidation eleven, then the queue
worked down, then completeness, then the earmark, and file 111 reached the same reading from the same
text. Three of the seven items in section 3 above are op's calls that the queue cannot close without him.

---

*Grounded on: ratified (`110`'s three topic files read in full at source on 2026-08-05 and not edited;
`04b:15-50,65-80`, `08b:40-60`, `12b:25-60`, `13b:8-25`, `13c:30-60`, `16b` in full, `16c` in full,
`17b:15-55`, `24b:14-34`, `30b:38-60`, `34b:36-72`, `68b:10-30,60-82`, `70b:45-62`, `79b:58-75` read at
the ranges quoted), settled shapes (`109:643-646`, `110` in full, `111` in full, `112` in full, `113` in
full, `09:133-240`, `11:150-216,505-525`, `63:104-124,452-462`, `68:438-492`, `78:118-132`), verified at
source (both canon-gate greps re-run 2026-08-05 over `mock/crates/`, `grep -rln
"Adjustment\|Bias\|Numeral"` and `grep -rln "FullRange\|UTerm\|AddWidth"`, both exit 1 empty; `git status`
showing `110` as the only modified file; zero em-dash occurrences in the edited document). Canon gate:
`108b:190-193` places `mock/crates` out of bounds and gives the panel `mock/research/` and
`mock/benches/`; `mock/design_rounds/` is in TOPIC phase and its files are frozen, so they were read and
not edited. This pass edited one file in `mock/research/` and wrote this one. No test gate was re-run,
because no source changed. Only op's calls are final, and nothing above is a design call.*
