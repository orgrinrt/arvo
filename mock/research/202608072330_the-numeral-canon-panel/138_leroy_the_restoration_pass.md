# 138. The restoration pass on `136`

**Member:** Leroy, resumed. Read for this file: `137` in full with `137_probes/g3`, `g8`, `g2` and `g5`
outputs; `136`, `132`, `131` and `130` at the lines under repair. Nothing else new. Two probes,
`138_probes/y1` and `y2`, each committed with its output as it ran.

**This is a restoration pass.** It repairs what `137` found defective and touches nothing else. `136`
stays as landed, because `137` cites it by line.

## 0. Gates

**Canon gate: passed.** Every edit below deletes a token, corrects a count, or names a mechanism.
Nothing proposes a design decision.

**Test gate: passed, at 123 across 13, and I am not inheriting it this time either.** `137` ran the
suite itself rather than inherit, per crate, and got 123 across 13 with zero failures at
`137_probes/g0_test_gate.out`. That is the third data point on the crate at the centre of this and it
resolves it: `bitpack-write-contend-shared` **completed in 6.61 seconds**, against 7.28 seconds in
`122_probes/u0_test_gate_run.txt` and non-termination in `125`. So `136` 0.2's qualifier, "one
contention crate does not terminate under concurrent load", was doing real work, and the load was three
concurrent instances of a core-pinning suite, one of them an orphaned process of the coordinator's, since
killed. The non-termination was a property of the machine and not of the crate, which is what `125`
itself said and what nobody could confirm until now.

## 1. What this supersedes in `136`, by line

- `136:374`, `136:398-399`, `136:410`: the `OPEN` token in three predicates. **Deleted.** Section 2.
- `136:270-274`: the class given one mechanism. **Extended.** Section 3.
- `136:36-37` and `136:491`: the attribution class's extent and its two counts. Section 4.1.
- `136:498-499`, `136:510-512`, `136:516-518`: the union's size, the no-drop claim, and the line-anchor
  row's reading. Section 4.2.
- `132:70`'s ledger row for B6, on the `126` side. **Upgraded.** Section 4.3.

Everything else in `136` stands. `137`'s verdict is that the compression is sound, that every correction
`136` makes to `132` is right, and that no claim rests on a defective run, and I am not reopening any of
it.

## 2. The three deletions

`137`'s severe finding, and it is correct. `136` reads the vacuity right, refuses to fill the
predicates on the correct ground, and then writes `OPEN` into three of them, which is the construction
op's own instruction inside the entry `136` cites forbids. `INTENTS.md:241-243`: "unmeasured or unknown
does not list in the predicate... **No adding 'unsure' into the predicate.** Unsure or unmeasured etc
explicitly go unstated and implicitly mean not true", glossed at `INTENTS.md:245-246` as "**It should
write nothing there.**"

The three predicates, restated without the token. Nothing else in them changes.

*The non-commutation holds for: rounding = toward-zero; range policy = wrapping; signedness = signed, or
unsigned with signed intermediates; threads = 1. **Argument kind: existence.***

*The variance law (`128` F128-3 on `128_probes/r3`, re-derived and widened by `130` on `130_probes/y1`)
holds for: element count any; fraction = 1/3; coupling in {comonotone, independent}; **domain any**, same
construction and same condition (`136_probes/x2` P3); threads any. **Argument kind: induction.** The
fraction is a fixed value and is not widened.*

*holds for: everything `132` 5.8's predicate lists, unchanged, plus **keying axis = one-dimensional**
(`136` section 8).*

`138_probes/y1` T1 finds the token at exactly the three lines `137` names and in exactly three of `136`'s
predicates. `y2` U1 is the control y1 named in advance: the same scan, widened past the one word I
happened to use to any hedge token, finds three in `136` and **zero in this file**.

**Why the token appeared, since a file that states the rule correctly and then breaks it in the same
clauses owes the next reader a sentence.** Having just established that an absent dimension is the
strongest negative in the notation, I wanted a reader to see that the absence had been *noticed* rather
than merely left. So I wrote a note to that reader into a slot that is not for readers. A predicate with a
dimension missing looks unfinished, and writing something there feels like diligence rather than like the
hedge it is; that pull is exactly what op's instruction names, and the note belongs in the prose beside
the predicate, which `136` section 5 already carries. **The severity survives the deletion and only the
reassurance goes.**

## 3. The class has two mechanisms, and the second is `137`'s finding

`136:270-274` attributes the whole class to one mechanism, "a predicate's dimensions read off the clause
above rather than off the argument underneath", and offers looking at the neighbouring clause as the
check. That is right for the two **inheritances** and wrong for three of the four **absences**.

`137` swept `131`'s seven arms with `131` R0's explicit `domain any` as the control that must come out
present, and it does. **Six of seven carry a domain dimension; only R6 does not**
(`137_probes/g8_does_the_class_reach_131s_arms.out`). So three of the four absences did not propagate
from `131`. They were manufactured by `132`'s own split of `131`'s compound predicates: `131` R3 at
`131:169-173` and R5 at `131:203-209` each carry the domain dimension in a **shared leading clause**,
followed by "For the non-commutation, ..." and "For the variance law, ...", and `132` split those halves
into standalone predicates keeping only what was written after the lead. R6 at `131:221-223` is the one
that genuinely propagated.

**So the second mechanism: splitting a compound predicate distributes its dimensions to every part, and a
split that does not is how a dimension disappears without anyone deleting it.**

It is the more dangerous of the two and it wants its own sentence for two reasons. Splitting reads as
refinement rather than as loss, so nothing about the act signals that something left. And **the check
`136` states will not find these three**, because the dimension was never in a neighbouring clause; it was
in the same predicate one tier up, in the document the compression replaced. The check has to look one
tier up, at the source predicate, not one clause across.

**This correction is `137`'s and not the candidate's.** The inheritance framing came to me in the revision
brief and I carried it as though `136` had found the mechanism. It had found one of two.

## 4. The three bookkeeping repairs

### 4.1 The attribution class has six carriers, and the earliest cites a message

`136:36` says "`133` names `131` and `132`". `133`'s D1 names three, including `130`'s gate note citing
the coordinator's message; `y1` T3 reads the D1 block and returns `125`, `130`, `131`, `132`. And
`136:37`'s five places against `136:491`'s six files do not reconcile.

`y1` T2 classifies every line in the topic asserting the count, separating retractions from carriers, and
the corrected statement is:

**Six carriers across five files. Five of those places, across four files, cite `125`, which records the
opposite: `131:48`, `131:52`, `132:43`, `134:13`, `135:10`. The sixth is `130:13`, committed at 00:26
ahead of every other, and it cites "the coordinator's message" rather than any file at all.**

That makes `130:13` the origin instance rather than another copy, and it is the more instructive of the
two coordinator errors precisely because it shows the number entering from a message, one hop before it
acquired a file citation. It is the same laundering as the 21,204 relay, caught at the moment it happens.
`133:11`, `133:199` and `133:205` also contain the figure and are retractions, not carriers.

### 4.2 Section 11's line-anchor row, and two counts in the same paragraph

`136:516-518` explains the line-anchor row by describing a different anchor class. The row reports
`line_panel`; the `INTENTS.md` references and the `118_probes` path it attributes to that row live in the
wider `line` class. `y1` T4 confirms: `line_panel`'s thirteen not-carried entries contain **zero**
`INTENTS.md` references and **zero** probe paths.

The corrected reading of those thirteen: **three are commit timestamps** from `132`'s own blindness table
(`23:20`, `23:28`, `23:29`), **five belong to the preceding topic** (`116:486`, `117:35`, `119:598`,
`122:642`, `122:642-646`), and **five belong to this topic**, at clauses `136` leaves standing (`125:4`,
`125:326`, `125:455-477`, `126:511`, `128:177`).

Two consequences, both of which `136` states wrongly:

**`136:510-512`'s "Nothing from this topic's eleven files is dropped" is false for line anchors**, and its
own next paragraph enumerates the five that are. The claim holds for findings, theorems and probe stems
and it is not general. The same sentence says "the eight signature files" where there are three
signatures.

**`136:498-499` miscounts the union**: it is eight preceding files, `125` through `132`, plus three
signatures, which is the eleven the table header and the instrument both report.

### 4.3 B6's blindness is stronger than the ledger claims

`132:70` credits B6's `126` side to "its reconciliation section", which is phase two. `137` places the
content in phase one, and `y1` T5 confirms it by locating the boundary rather than assuming it: the
answer sentence sits at `126:24`, "**My answer, stated up front and argued below: neither, and the reason
it is neither is structurally informative**", and the first line mentioning a phase-two boundary is at
`126:393`.

So B6 was written before `126` read anything at all, not after. **The entry is upgraded: `126` phase one
at `126:24-26`, alongside `125` section 8.** A rung understated is a real error and this one is the
topic's headline result, the answer to the question the topic was convened under, on the only entry a
merge of the two convergence lists would have lost.

## 5. What `137` found that this pass does not repair

Named rather than dropped, since a finding absorbed silently is the failure the whole exchange is about.

**`137` section 3.2, the vacuity verdict is over-severe on two of the four.** By `136`'s own
present-versus-not-present test at `136:145-147`, a container width and a fraction width are arguably not
present for a claim about a within-cell distribution, and none of `domain`, `W`, `F` or `signedness` is
obviously present for a claim about entropy provenance. `131` R6 having written no domain dimension for a
toolchain-shaped claim is evidence in the same direction. **The three deletions above are correct under
either reading**, because either way the predicate writes nothing; what is unresolved is whether the prose
beside them is right to call the obligations open. That turns on a judgement about which dimensions a
claim ranges over, which is substantive rather than bookkeeping, and I am not making it in a restoration
pass.

**`137` section 7, one misdirected citation.** `136:65-66` cites "`131` F131-6's vocabulary count"; F131-6
is the staged-narrowing finding. The vocabulary is F131-3. Same class as the dead T9 label `136` itself
corrects, one label wrong, and it is left standing here only because it was not in the repair list.

**`137` section 6, the anchor table's middle column** reports anchors-present for three rows and
carried-count for the fourth, under one header, which cost `137` a false lead.

**`137` finding 9**, that 5.7 is listed under "Stands" in `136` section 1 while section 5 rules it vacuous.

## 6. Coverage

**Read in full:** `137`. **Opened at source rather than recalled:** `130:10-16`, `126:22-28` and `126:393`,
`131:169-173`, `131:203-209`, `131:221-223`, `132:70`, `133`'s D1 block, `136:36-37`, `136:270-274`,
`136:371-412`, `136:496-520`, `INTENTS.md:241-246`.

**Not read:** `137_probes` sources except `g8`'s output, `g3`'s output and `g0`'s; every file before `125`;
`OPTIONS.md`, `AGREEMENTS.md`, `DROPLIST.md`, `RULES.md`.

**Built:** two probes, two outputs, eight predictions, all confirmed, and **two defects in the first
recorded rather than repaired quietly**. Its predicate-span pattern was non-greedy to the next `*` and every one of these
predicates writes the token inside a bold run, so the first run reported zero predicates carrying a token
its own line-level count had just found three times. That is the third extractor in three files defeated
by markup inside the span it was reading, and the line-level count is what caught it. And its carrier
classifier matched any line containing the figure, counting `133`'s own retractions as carriers and
returning nine; a carrier asserts the count as its own inherited result, and a line naming the error is
the opposite of one.

**Not done.** No measurement, so nothing here prices anything. No re-derivation of anything in `125`
through `131`. No repair of the four findings in section 5, each left with its `file:line`.
