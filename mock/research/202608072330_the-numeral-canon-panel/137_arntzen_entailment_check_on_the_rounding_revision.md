# 137. Entailment check on the rounding revision

**Member:** the `arntzen` persona, independent of this topic. I took no part in `125` through `136`,
which is the point: the members who built it cannot check whether it says what their own work said.
**Probes:** `137_probes/`, seven of them, each committed with its output as it ran.

**Standing:** nothing here settles anything. Op decides. Where I say `136` is wrong I mean it and there
is a `file:line` and a reproduction under it; where I say it holds I mean that too, and saying so is a
result rather than a courtesy.

I worked from `125` forward and read all eleven sources end to end before opening `136`'s body, because a
check driven by a compression's own structure finds only what that structure already accounts for. Three
of the nine findings below are invisible from the other direction.

---

## 0. Gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` I1 through I18, read in full, with its "How to read an entry" section as
normative.

Running this check is what the compression rule asks for and what `87` licenses: a consolidation is
input rather than canon in miniature, and nothing moves to `mock/canon/`. Nothing in `136` presumes the
strategy set closed at four, nothing reasons from the removed crate tree, and no clause proposes a design
decision. I found nothing to refuse on and nothing ambiguous to hand back.

**One thing I record because it bears on the sharpest finding below.** I13's own text scopes itself:
"The scope of this entry is those two paragraphs... Anything further, including the dimension list, the
`any` against `unmeasured` distinction, and the exactness bar for a predicate, is elaboration in
`every-finding-carries-its-predicate.md` and is **not** part of what was ratified"
(`INTENTS.md:263-267`). Section 2 below turns on which side of that line a reading sits.

### 0.2 Test gate: passed, at 123 across 13, run by me

The brief says to inherit a count only from an artifact I have opened, given that two of the errors under
examination are exactly that failure. So I ran it rather than inherit, per crate under
`mock/benches/variants/`, since the workspace-wide form is the false green `117:35` records.

**123 tests across 13 crates, 0 failures**, at `137_probes/g0_test_gate.out` with the per-crate lines.

**And one measurement that bears on the inheritance chain.** `bitpack-write-contend-shared`, the crate
`125` could not terminate, **completed in 6.61 seconds** on my run. I also opened
`122_probes/u0_test_gate_run.txt`, which is the artifact `136` inherits from: fourteen `test result: ok`
lines summing to 124 passes, `TOTAL PASSED: 123`, the fourteenth being the appended single-test rerun for
`121` section 3.4, and it records the same crate green at 7.28 seconds. So there are now three data points
and they agree with `125`'s own diagnosis: the non-termination is a property of the machine under three
concurrent instances of a core-pinning suite, not of the crate. **`136`'s wording, "one contention crate
does not terminate under concurrent load", is accurate as written** and the qualifier is doing real work.

I read the bodies in the surface this file touches, which is the two quantiser crates the topic's
vocabulary and accuracy claims rest on. `133`'s two observations on `quantiser-radix-shared` are the only
body-level defects anyone has found and I add nothing to them. Nothing tautological, nothing sampled
where a matrix was available, nothing assertion-free. There is nothing to refuse on.

---

## 1. Coverage, bounded rather than claimed

**Read end to end:** `125`, `126`, `127`, `128`, `129`, `130`, `131`, `132`, `133`, `134`, `135`, `136`.
`INTENTS.md` in full.

**Opened at source rather than recalled:** `125:283-291`, `125:463-477`, `125:116`; `INTENTS.md:237-247`
and `:263-267`; `122_probes/u0_test_gate_run.txt` in full; `132:320-336`, `132:346-361`; `131`'s findings
block; `136_probes/x1_output.txt`, `x2_output.txt`, `x4_output.txt`, `w1_over_136_output.txt` and
`w1_over_136.py`; `135_probes/z1`'s source by grep.

**Re-run rather than read:** the crate suite, per crate. `136_probes/w1_over_136.py`, which reproduces
byte for byte. My own independent extraction of `132` section 5's predicates and of the anchor sets.

**Not read:** `RULES.md`, `OPTIONS.md`, `AGREEMENTS.md`, `DROPLIST.md`, every panel file before `125`
except `122` at its section 0.2 and its `u0`, and the probe sources of `125` through `133` except where
named above. So where a finding of mine restates something in those, I do not know it. This bounds
section 6 in particular, where I check `136`'s reading of the preceding topic through `131` and `132`
rather than against `122` itself.

**Citations opened rather than resolved:** every `file:line` I attribute below was opened at its source.
**Two false positives were mine and are recorded in the probes that produced them**, because the brief is
right that a comparable check on this panel returned one severe finding that was itself half wrong. My
anchor pattern reported `p11` and `x87` as dropped probe stems; `x87` is the Intel FPU, named in
`126:223`, and neither is a probe. And my first pass read `136`'s probe-stem row as contradicting its own
instrument before I noticed the instrument reports two different columns, which is finding 7 below rather
than the defect I first took it for.

**Not verified, and named:** every enumeration in `125` through `133`, the Fréchet uniqueness proof, the
variance closed forms, the adjunction argument, `131`'s pin extractor, and `134`'s spectral claim, which
is a statement about a literature. I ran no bench and took no measurement, so nothing here prices
anything.

---

## 2. SEVERE. The revision writes a hedge token into three predicates, which op's instruction forbids

This is the sharpest thing I found and it sits inside the work `136` does best.

`136` diagnoses four predicates as carrying no domain dimension, reads that correctly as vacuating rather
than merely unqualified, and then refuses to fill them: "**Filling a predicate with a value nobody
measured is exactly what the notation exists to prevent**" (`136:267-268`). That refusal is right.

**What it does instead is write `OPEN` into the predicate**, in three places
(`137_probes/g3_predicates_and_the_open_token.out`):

- `136:374`, the non-commutation: `**domain: OPEN, and the clause claims nothing until it is stated**`
- `136:398-399`, the variance law: `**W, F and signedness: OPEN**`
- `136:410`, the entropy clause: `**domain, W, F and signedness: OPEN**`

Op's instruction, recorded verbatim inside the entry `136` cites for the vacuity reading
(`INTENTS.md:241-243`):

> unmeasured or unknown does not list in the predicate. It's not known, it's assumed not true until
> proven true. **No adding "unsure" into the predicate.** Unsure or unmeasured etc explicitly go
> unstated and implicitly mean not true

And `INTENTS.md:245-246`'s own gloss, which is as direct as it gets: the instruction "replaced a proposal
of the coordinator's that a predicate should write `unmeasured` on a dimension nobody checked. **It should
write nothing there.**"

`OPEN` is a token for unmeasured, written into the predicate, on a dimension nobody checked. It is the
exact construction the instruction names and replaces.

**One of the three is mitigated and two are not.** `136:374` writes the token with a gloss, "and the
clause claims nothing until it is stated", which preserves the severity rather than softening it. The
other two carry the bare token, and a bare `OPEN` in a predicate is readable as "not yet pinned, probably
fine", which is the reading op's instruction exists to foreclose. A later reader gating an arm on that
predicate sees a slot that looks provisional rather than a claim that holds nowhere.

**The repair is a deletion, not a rewrite**, and the material for it already exists: drop the token, let
the dimension go unstated, which *is* the vacuity honestly borne, and carry the obligation in the prose
beside the predicate, which `136` section 5 already does well ("I name them as open obligations rather
than fill"). The predicate says nothing; the paragraph says why and what would close it. That is the
shape the instruction leaves available and it costs three edits.

---

## 3. MODERATE. The vacuity reading is right, and it is applied unevenly across the four

The brief asks whether a missing domain dimension is genuinely vacuating for these particular claims, and
whether all four are correctly identified. Two answers, and they differ.

### 3.1 The four are correctly identified, on my own extraction

`136`'s `x4` had four recorded defects, one of which made it miss a predicate, so its count is exactly the
kind of thing to redo rather than accept. I extracted `132` section 5's predicates with my own pattern,
deliberately looser than `x4`'s (accepting `hold for:` as well as `holds for:`, any lead-in, span to the
next blank line, so a plural subject cannot hide one).

**Eleven predicates, four with no domain dimension: numbers 5, 9, 10 and 11**, which are 5.4's
non-commutation, 5.6's variance law, 5.7's keying and 5.8's entropy disjunction
(`137_probes/g3_predicates_and_the_open_token.out`). Exactly `136`'s four, from a different pattern.

And one thing `136` gets right that is easy to get wrong: 5.4's non-commutation names
`signedness = signed, or unsigned with signed intermediates`, and `136:250` says `signedness` "does not
stand in for it". That is correct and load-bearing, because the whole repair the preceding topic made was
domain-conditionality, which `122` established is a different dimension from signedness. A reader who
conflated them would read that predicate as complete.

### 3.2 The vacuity verdict is warranted for two of the four and over-severe for two

`136` applies a principled test in one place and does not apply it in three. At `136:145-147` it exempts
the operation dimension from 7.1's monotonicity clause: "the monotonicity half carries no operation
dimension, **because a characterisation of a map is not a claim about an operation**". That is exactly
right, and it is the correct general test: a dimension's absence vacuates only where the dimension is
**present** for that kind of claim.

Run the same test on the four:

- **5.4's non-commutation.** A composite of a quantisation and a range policy acts on values in some
  domain. The dimension is present. **Vacuous, correctly.**
- **5.7's keying.** Decorrelation counts over values at positions. A domain is present. **Vacuous,
  correctly.**
- **5.6's variance law.** A statement about the Fréchet parameter of a coupling within one cell, whose
  quantities are computed from `frac(x)` alone. `136` establishes `domain any` for it and fills that
  correctly at 7.4. What it then opens is `W, F and signedness`, and a container width and a fraction
  width are arguably not present at all for a claim about a within-cell distribution. If they are not
  present, their absence claims nothing and there is nothing to open.
- **5.8's entropy disjunction.** A claim about where randomness can come from under the operating
  constraints, plus a compiled artifact. `136` opens `domain, W, F and signedness` on it. None of those
  four is obviously present for a claim about entropy provenance, by the same reasoning that exempts
  `operation` from 7.1.

**So the candidate has partly weakened itself unnecessarily**, which is the half of the brief's question
that turns out to have an answer. The reading is right; its application to 5.6's residual dimensions and
to 5.8 entirely opens obligations that may not exist. The test `136` already states at `136:145-147` is
the one to apply, and applying it would close two of the four without measuring anything.

### 3.3 The class has two mechanisms, and `136` names only one

`136:270-274` gives the class one mechanism: "a predicate's dimensions read off the clause above rather
than off the argument underneath". That is right for the two **inheritances** it corrects, 5.4's
commutations and 5.6's uniqueness, and it is **not** the mechanism behind three of the four **absences**.

I found this by closing a gap I had first written down as undone, which is the useful order to record it
in. `132` compresses `131` section 3's seven arms, so an absence in `132` either originated there or
propagated. I swept them (`137_probes/g8_does_the_class_reach_131s_arms.out`), with `131` R0's explicit
`domain any` as the control that must come out present, and it does.

**Six of `131`'s seven arms carry a domain dimension. Only R6 does not.** So three of `136`'s four
vacuous predicates did not propagate from `131` at all. They were **manufactured by `132`'s own split of
a compound predicate**, and the source shows exactly how:

- `131` R3 is one predicate reading "*holds for: the commutations, ... W >= 1, F any, signedness any,
  **domain closed under negation**, threads any ... **For the non-commutation**, rounding = toward_zero,
  overflow = wrap, signedness = ..., threads = 1*" (`131:169-173`). The domain dimension lives in the
  **shared leading clause**, and the non-commutation half relies on it by position. `132` split the two
  halves into standalone predicates and the second half kept only what was written after "For the
  non-commutation", losing the domain.
- `131` R5 is the same shape at `131:203-209`: a leading clause carrying `domain closed under negation`,
  then "For the variance law, ..." and "For the keying divergence, ...". `132` split it three ways into
  5.6's two predicates and 5.7's one, and the domain survived on the first only.
- `131` R6 (`131:221-223`) genuinely carries no domain dimension, and `132` 5.8 inherits that unchanged.
  **That is the only one of the four that propagated.**

**So the second mechanism is: a compound predicate with a shared leading context, split into standalone
predicates, without redistributing the shared dimensions.** It is the more dangerous of the two, for two
reasons. Splitting a predicate reads as refinement rather than as loss, so nothing about the act signals
that something left. And a reader applying `136`'s stated check, look at the clause above, **will not find
these three**: the dimension was never in a neighbouring clause, it was in the same predicate one tier up,
in a document the compression replaced.

This also partly vindicates `131` R6 and sharpens 3.2. R6's predicate names a toolchain, an edition, a
crate type, feature gates, a threshold construction and a position range. A numeric domain is arguably not
present for a claim of that shape, which is why `131` wrote none, and `132` and `136` both treat the
absence as a defect to open rather than as a dimension the claim does not range over.

**And the absence problem in `132` section 5 is not confined to the domain dimension**, which the sweep
`x4` performs cannot see because it is a domain sweep. Predicate 3 (5.3's composition failure) carries no
`W`, no `F` and no `signedness`; predicate 10 (5.7's keying) carries no `W`. For 5.3 the omission is
defensible on `132`'s own grounds, since an existence result carries at every width and `132` section 4
says so. For 5.7 it is one more absence in a clause already ruled empty.

---

## 4. The instrument defects, and whether anything still rests on one

This is what the check exists to catch, and the answer is **no**: I traced all four and found no surviving
dependent (`137_probes/g4_do_dependents_survive_the_defects.out`).

**The half-even defect, traced specifically as the brief asks.** `x1` part B's first run swept `W = 7`,
`F = 3` with a 3-to-2-to-1 staging and reported half-even carrying exact staged composition. Had it been
believed it would have contradicted `133`'s `s2` and `125` section 10's P4, which measures 500 half-even
mismatches of 4001, and it would have made 5.3's replacement wrong in the direction that matters.

It is caught, and nothing carries it forward:

- The corrected `x1_output.txt` shows `half_even` composition **False**, with the first failing value
  printed, `-247/16`, `direct=-15 staged=-16`. The probe now prints a witness per member "so a True
  cannot mean an empty sweep", which is the right repair: the defect was an empty region, and the fix is
  to make emptiness visible rather than to widen and hope.
- `136`'s own replacement at 7.2 gives half-even "negation symmetry and the optimal error bound, **with
  neither exact composition nor a one-sided bound**". Correct.
- `132` 5.3's surviving bullet still says the nearest members make a staged narrowing depend on its
  staging. Correct.
- Three witnesses agree: `125` P4, `133` `s2`, `136` `x1` corrected.

`136`'s 7.2 predicate also matches what the corrected `x1` actually swept, which I checked rather than
assumed: `W in {9}; F in {4}; ... F_exact = 4, F_intermediate = 2, F_final = 0` against the output's
header line "at W=9, F=4, staged narrowing F_exact=4 -> F_mid=2 -> F_final=0"
(`137_probes/g7_the_five_two_gap_and_x1s_sweep.out`).

**The other three.** `x2`'s replaced controls fire, and the output shows them firing rather than asserting
they would: "P2 (control fires): CONFIRMED", and the variance control differing at `k=-4` against `k=+4`
under the sign-dependent variant. `x4`'s corrected run finds eleven predicates and carries its own control
line, "`domain` is not split at the `in` inside it: True (must be True)". My independent extraction agrees
with the corrected count.

**And one of `136`'s own results deserves saying plainly, because it is `136` attacking a correction it had
just accepted.** `x2` P4 finds that `135`'s widening is conditional: under a truncating cell coordinate,
`frac(x) = x - trunc(x)`, the same construction is sign-dependent at both `m = 5` and `m = 8`, and the
negative cell produces a distribution containing a **negative mass** (`k=-3 -> ['-4/5', '1/5', ...]`). So
the honest form is not bare `domain any` but `domain any` given a floor-based cell coordinate. That is a
real hypothesis nobody had stated, found by building the control `135` did not build, and it is the best
work in the file.

**`x1` A3 is a refuted prediction that widened the result**, which I confirmed in the output: no monotone
non-local grid-fixing retraction exists, because a grid-fixing map pins `k` and `k+1` so a subpoint
between them must land in `[k, k+1]` to stay monotone. Monotonicity implies locality, so the
characterisation holds over every grid-fixing retraction rather than only the local ones. Wider than
`133` established, and kept as a refutation rather than repaired.

---

## 5. The two coordinator errors, grepped myself

### 5.1 The fabricated figure: the correction is complete and the second route is sound

`137_probes/g1_the_two_coordinator_errors.out`. `21,204` and `21204` appear in no file of `125` and in
neither of its probe directories, exactly as `136:77` says. Every occurrence in the panel is inside a
retraction: `131`, `132`, `133`, `135`, `136` and their probes.

`136`'s second route is sound and is genuinely independent of `133`'s. `133` searched every width to
`2^60` for a representation `2^b(2^k - 1)` and found none. `136` pins the width from the figure's own
denominator instead: if 32,768 names the swept domain then `W = 15` and the complete set of reportable
counts is `{2^14 - 2^(14-F)}`, whose maximum is 16,383, so **21,204 exceeds the largest count the sweep
can produce**. Two arguments, two shapes, one conclusion.

**The provenance split is right and `136` states it at the correct severity.** The fabrication is `125`'s
author's, in an end-of-dispatch report message outside any committed artifact; the relay into two briefs
is the coordinator's, and `136` names itself for it without softening. `125`'s committed record was
correct throughout.

### 5.2 The test-gate misattribution: the five are exact and the list is one instance short

**The five locations `136` names are exact.** I opened each: `131:48`, `131:52`, `132:43`, `134:13`,
`135:10`, all carrying the claim in the stated form
(`137_probes/g2_is_the_attribution_class_complete.out`). Five places across four files, correct for the
class "attributes the completed count to `125`". And `125:463-466` records the opposite, which I opened.

**But the class as a whole has six instances, and the sixth is the one that shows the mechanism.**
`130:13` reads "per the coordinator's message the eleventh run of the gate already stands at 123 across 13
by `--manifest-path`, cited with attribution rather than re-run". `130` was committed at 00:26 and `131`
at 00:41, so **`130` is the first carrier of the wrong number in this topic**, and it sources it to a
coordinator message rather than to a file. That is the same laundering one hop earlier, and it is the same
mechanism as the 21,204 relay, which makes it the more instructive instance of the two errors rather than
a footnote.

**And `136` understates what `133` reported.** `136:36` says "`133` names `131` and `132`". `133`'s D1
names three: "`132` 0.2 (and `131` 0.2, and **`130`'s gate note citing the coordinator's message**)". So
the sentence that introduces `136`'s wider count drops the one locus `136`'s own list also omits.

**Its two counts of the same class do not reconcile.** `136:37` says five places across four files.
`136:491` says "Six files have now inherited a gate count through it, five of them citing the wrong file
for it." Four files cite the wrong file. Six files inherit only if `130` is counted, and `130` cites no
file at all.

The repair is one sentence: name `130:13` as the first carrier, say it cites a message rather than a file,
and let the two counts agree at six carriers of which four misattribute to `125` across five places.

---

## 6. The anchors, on my own extraction, both directions

`137_probes/g5_anchors_my_own_extraction.out`. I rebuilt the sets with my own patterns rather than rerun
`136`'s runner, because a shared instrument makes agreement uninformative, and I stripped anchor-accounting
sections from **both** sides.

**The substantive results reproduce.**

- **Findings: three dropped, and they are exactly `F122-2`, `F122-4`, `F122-5`**, all from the preceding
  topic. `136`'s claim confirmed on an independent extraction.
- **Theorems: zero dropped.** All nine of `125`'s labels carried.
- **Probe stems: one dropped**, and `136` flags it itself, conservatively: `q5` appears in its body as a
  bare stem, its pattern wants a filename or a `NNN_probes/` prefix, so the instrument leaves `q5` counted
  as not carried, which it is. Self-flagged under-reporting in the safe direction is the right handling.
- **The stripper guard is real and it fires.** On my own extraction, excluding the accounting section
  changes the counts by `finding +3` and `line_panel +5`, which is the same shape `136` reports
  (`finding +3, line +5`). The guard's designed case is genuine: naming the dropped anchors to account for
  them honestly is what makes them present, and the more careful the accounting the more complete the
  disabling.

**Three defects in section 11's prose, all checkable.**

**One, the line-anchor row is read against the wrong class.** `136:516-518` says of the thirteen
not-carried panel line anchors that "five are `INTENTS.md` line references from other members' gate
sections, three are commit hashes from `132`'s own blindness table, and one is a `118_probes` path from
the preceding topic". The `line_panel` class contains **zero** `INTENTS.md` references and **zero**
`118_probes` paths; those live in the wider `line` class, which has five of the first and **two** of the
second. The five non-this-topic entries in the row being explained are `116:486`, `117:35`, `119:598`,
`122:642` and `122:642-646`. Only the commit-hash third is right, and the row's "nine of those are not
anchors this file could carry" is eight under the correct reading.

**Two, a bolded claim its own next paragraph corrects.** `136:510-512`: "**Nothing from this topic's
eleven files is dropped**". True for findings, theorems and probe stems; false for line anchors, where
`125:4`, `125:326`, `125:455-477`, `126:511` and `128:177` are dropped, which the paragraph immediately
after enumerates. The same sentence says "the eight signature files' own anchors" where there are three
signatures.

**Three, the union is miscounted in prose.** `136:498-499` says "The union is the five preceding files of
this topic plus the three signatures". It is eight preceding files, `125` through `132`, plus three, which
is the eleven the table header and the instrument both report.

**And one presentational defect worth naming because it made me chase a false lead.** The middle column is
heterogeneous. `132` defined "in NNN" as anchors "present in this file", and `136`'s table reports 19 for
line anchors, which is that quantity, and 16 for probe stems, which is the **carried** count; the
instrument's own output gives 21 present. Findings and theorems coincide either way. Three of four rows
are one quantity and one row is another, under one header.

None of this touches the substantive result, which is that nothing this topic established is dropped
except panel line anchors belonging to clauses `136` leaves standing, and `136` says so.

---

## 7. Absence claims, each checked against the search it owes

A negative claim names no place, so a citation checker passes it by construction. The brief is right that
the standard is met at least once here. I checked whether it is met everywhere
(`137_probes/g6_absence_claims_and_the_rungs.out`).

**Met, with the search named in the file:**

- "the sentence appears in neither `125` nor `126`, and appears at `131:428` and `132:322`" (`136:163`,
  naming `x3` Q5). **Confirmed:** zero in `125`, zero in `126`, one in `131`, one in `132`. So 5.3's
  exclusivity sentence was invented at the formalisation step, exactly as `136` says.
- "`125` defines no T9" (`136:102`, naming `x3` Q6). **Confirmed:** nine `**Tn` definitions, `T1`, `T1b`,
  `T2` through `T8`, and one reference to `T9`.
- the figure "returns nothing" in `125` and its probes (`136:77`, naming `x3` Q3). **Confirmed** in my own
  grep.

**Met, but the search is not named:**

- "Four predicates carry no domain dimension at all, **which no signature reported**" (`136:66`).
  **Confirmed:** `134` and `135` contain no such report, and `133`'s single match is its section 4 on the
  three unclassifiable pins, a different subject. `135` reported the opposite reading, that 5.6 and 5.7
  are over-narrow, and was wrong about 5.7 carrying the dimension at all, which `136:190-200` states
  correctly. The claim holds; it rests on `x4`'s table plus reading rather than on a named search.
- "`135`'s `z1` carries no negative control" (`136:204`). **Confirmed:** zero occurrences of "control" in
  `z1`'s source. `136`'s coverage says it opened the source, so the search happened; it is not named at
  the claim.

**One misdirected citation, same class as the T9 label `136` itself corrects.** `136:65-66` reads "`131`
F131-6's vocabulary count is unchallenged by any signature". **F131-6 is the staged-narrowing finding**
("Staged narrowing equals direct narrowing for the directed modes and not for the nearest ones") and
carries no vocabulary count. The vocabulary is settled by F131-3, which the same sentence has already
cited for a different thing, and the pin count is F131-2, which section 6.3 handles separately. The
citation resolves to a real finding that is not the one meant.

---

## 8. The rungs, and the half `134` could not confirm

Both halves check out and the reconciliation is right. `137_probes/g6`.

**`128`'s five, verbatim from its own section 7:** the vacuity of rounding for grid-closed operations
(F4 / Finding 1), universal deterministic monotonicity (T2 / Finding 3), non-additivity off-grid
(T1 / Finding 2's measured half), the double-rounding split of nearest against directed
(T4 and P4 / Finding 5), and saturate composition preserving monotonicity (T6 / Finding 6's first half).
That is B1 through B5 exactly as `132`'s table has them.

**`129`'s two, verbatim from its own coverage section:** the vacuity fact, and "the general shape of the
answer to the brief's question (neither a copy nor a modifier of the overflow axis, an independent axis
with its own selected property)". That is B1 and B6.

**Overlap on B1 alone, so the union is six.** Not seven and not five, as `132:60-62` says and as `134`
confirms.

**B6 is real and a merge would have lost it.** It appears on `129`'s list only, and it is the answer to
the question the topic was convened under. I checked it is genuinely blind rather than take either
author's word: `126`'s phase one states it up front, before anything was read, at `126:24-26`, "**My
answer, stated up front and argued below: neither, and the reason it is neither is structurally
informative.**" And `125` section 8 is titled "The answer to the question, assembled". Two files, two
argument structures, neither having seen the other.

**The half `134` could not confirm is confirmed by its own author.** `133:162-165`, resumed as `125`/`128`:
"B1 through B6 are all phase-one content of mine (F4, T2 with F2, T1 with F1, T4 with P4, T6, and section
8's shape respectively), committed at `a42f0b17` before I read anything of `126`'s."

**One credit is understated rather than overstated**, which is the safe direction and worth naming.
`132`'s table credits B6's `126` side to "its reconciliation section", which is phase two. The content is
already in `126`'s phase one, written before any reading at all, so the blindness is stronger than the
ledger claims. Nobody has to change anything; a later reader should know the evidence is better than the
entry says.

---

## 9. What holds, checked rather than credited

Keeping something is a result, and this list is longer than it would be if the file were weak.

**Every correction `136` makes to `132` is right, and two of them needed no probe.**

`136` 3.2's catch is the cleanest thing in the file: `132` 5.3's headline says no member carries more than
one of the first three, and its own second bullet, four lines below, says exact composition is "carried by
the directed members", which is both adjoints, each of which the first bullet has just given the order
bound. The clause contradicts itself in the same blockquote. `136` says "No probe was needed to see that,
and I did not see it", which is the right register for a defect that survived a formalisation and a
compression.

`136` 4.1's catch is the same shape one level down and I verified both halves at source. `125:287` says
T8 "holds on one-signed domains too", `132` 5.4's body repeats it as "one-signed domains included", and
its predicate then reads `domain closed under negation`, excluding exactly that region. The predicate and
the prose disagree inside one clause, and the predicate is the part a reader gates on. `136`'s repair,
splitting the family invariance from the commutations because `125` T8 and `125` T6/T7 are different
theorems with different scopes, is the right fix rather than a patch to the dimension.

**The dependents question comes out clean**, which is what section 4 establishes and which is not a
formality: a withdrawn instrument whose dependents survive is the failure this check exists to catch, and
there is none here.

**The four defects in `136`'s own probes are recorded at full severity**, including the one that is worst
to admit: `x4`'s verdict block printed a literal rather than the variable, "so the instrument reported
itself sound while failing", inside a probe written to criticise a probe with no control. Naming that is
worth more than the finding it produced.

**And the topic's provenance is unusually good and `136` does not inflate it.** The ledger stands, the
one-expert placements are honest, `133`'s confession is carried at the severity `133` asked for rather
than the softer one `132` offered, and `135`'s rung precision ("verified the first's claim by an
independent method and widened it, having read the claim before deriving its proof") is adopted as
written rather than left to be read.

---

## 10. Findings by severity, with what each repair costs

1. **SEVERE.** Three predicates carry the token `OPEN`, which op's instruction inside I13 forbids and
   which `INTENTS.md` says should be written as nothing. Section 2. **Repair: three deletions**, with the
   obligation moved into the prose `136` section 5 already has.
2. **MODERATE.** The class has two mechanisms and `136` names one. Three of the four absences were
   manufactured by `132`'s split of `131`'s compound predicates rather than inherited from a neighbouring
   clause, and a reader applying `136`'s stated check will not find them. Section 3.3. **Repair: name the
   split mechanism beside the inheritance one**, and say the check has to look one tier up rather than one
   clause across.
3. **MODERATE.** The vacuity verdict is over-severe on 5.6's residual dimensions and on 5.8, by `136`'s
   own present-versus-not-present test at `136:145-147`, and `131` R6 having written no domain dimension
   for a toolchain-shaped claim is evidence the dimension is not present there. Sections 3.2 and 3.3.
   **Repair: apply that test to the four**, which closes two of them without measuring anything.
4. **MODERATE.** The attribution class has six carriers and `136` enumerates five; `130:13` is the
   earliest and shows the number entering from a message rather than a file, and `136` also understates
   what `133` reported. Its two counts of the class do not reconcile. Section 5.2. **Repair: one
   sentence.**
5. **MODERATE.** Section 11's reading of its line-anchor row describes a different anchor class, and its
   bolded no-drop claim is corrected by its own next paragraph. Section 6. **Repair: rewrite the three
   sub-counts against the row's actual contents.**
6. **LOW.** A misdirected citation, "`131` F131-6's vocabulary count", where F131-6 is staged narrowing.
   Section 7. **Repair: one label.**
7. **LOW.** The union is described as five preceding files plus three signatures where it is eight plus
   three, and "the eight signature files" where there are three. Section 6.
8. **LOW.** The anchor table's middle column reports one quantity for three rows and another for the
   fourth. Section 6.
9. **LOW.** 5.7 is diagnosed vacuous in section 5, listed under "Stands" in section 1 as "5.7's
   substance", and given no replacement predicate in section 7. `136` is honest about it in two other
   places; the tension is between the section 1 listing and the section 5 verdict.
10. **NOT A DEFECT, recorded because the safe direction should be visible too.** B6's blindness is stronger
   than the ledger claims, since `126`'s phase one carries it. Section 8.

**And the verdict.** The compression is sound. Every claim I could test against the sources holds at the
strength `136` gives it, every correction it makes to `132` is right, no claim rests on a defective run,
the anchor result reproduces on an independent extraction, and the rung reconciliation is correct with the
one entry a merge would have lost preserved and confirmed by its author.

What is wrong is one construction and four bookkeeping defects. The construction is the one that matters,
because it sits in the predicates a later reader gates on and because the file that commits it is the file
that diagnosed the class.

---

## 11. What I did not do

**I did not check `136`'s reading of the preceding topic against `122` itself**, only through `131`, `132`
and the two lines of `122` I opened. So section 6's account of which anchors belong to that topic is
mine; its reading of what `122` 4.6 means is `131`'s and I did not test it.

**I did not re-derive any enumeration.** The Fréchet uniqueness, the variance closed forms, the adjunction,
the equivariance table and `131`'s pin classification are all read rather than reproduced, and section 3's
verdict on the vacuity reading does not depend on any of them.

**I did not check `134`'s spectral claim**, which is a statement about a literature rather than a
measurement, and `136` says the same of itself.

**I did not attempt the two questions `136` leaves for op**, and I have no view on whether a canon may
carry visibly open obligations. That is section 10 of `136` and it is his.

**I did sweep `131` section 3's arms** rather than leave that as a gap, and it produced section 3.3.
What I did **not** sweep is `125` through `130`'s own findings blocks, where the same split mechanism
could have operated one tier further down, and `128` F128-1 in particular carries a compound predicate of
exactly the shape that lost dimensions when `132` split it.

---

## 12. Coverage of my own citations

Every `file:line` above was opened and its **content** tested rather than merely resolved, by
`137_probes/g9_check_my_own_citations.py`. Whitespace is normalised and blockquote and doc-comment
markers stripped on both sides, because a quotation wrapped across lines is still verbatim, and neither
normalisation can make an absent phrase appear.

```
citations checked: 34   ok: 34   failed: 0
```

**It passed first time, which is the weaker of the two things to say about it**, so it is mutation-tested
three ways and each mutant is caught: a phrase op did not say, a real phrase at the wrong span, and a real
phrase in the wrong file.

**Nine probes, each committed with its output as it ran**, at 72K total: the gate run, the two coordinator
errors, the completeness of the attribution class, the predicate extraction and the hedge token, the
dependents trace, the anchor diff on my own patterns, the absence claims and the rungs, the residual
checks, `131`'s arms, and this.

**What no probe of mine checks**, and it is most of sections 2 through 9: whether a cited passage supports
the argument put on it. No instrument crosses that, which is why this file exists and why it should not be
the last word on `136` either.
