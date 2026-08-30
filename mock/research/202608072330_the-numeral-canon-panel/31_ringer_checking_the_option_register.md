# 31. Checking the option register, from the member files forward

**Position:** an outside check on `OPTIONS.md`, run from the member files toward the register rather
than from the register outward, per the dispatch and per `a-compression-is-checked-by-someone-else.md`.
I contribute no design opinion in this file, and I did not edit `OPTIONS.md`.

## The verdict, before the detail

I found the register in substantially better shape than a document rebuilt from twenty-six files by one
pass has any right to be. Of roughly forty load-bearing numeric and quoted claims I checked against the
member file they cite, essentially all reproduce exactly. Of 152 section-level citations I checked
mechanically against the actual heading structure of the cited files, 146 resolve cleanly. All sixteen
probe citations resolve to files that exist on disk. `30`'s rebuild is a real fix, not a relabelled
repeat of the defect it replaced.

Two things are wrong, and one of them is the same shape of failure this dispatch exists to catch a
second time.

**One design question, independently checked by two member files with real evidence, is missing from
the register entirely.** File `02` names it as one of exactly two things "genuinely undetermined, and
is op's". The other made it into the register. This one did not, under any wording I searched for.

**Three citations misdirect a reader to content that is not where they say it is**, one of them serious:
a quoted phrase is attributed to two files as though both used it, when only one does, and both of the
citation's section pointers are wrong.

I found no basis to challenge the register's existence or its locus, and I looked, per the dispatch's
invitation to. `RULES.md` mandates the mechanism in terms that leave no ambiguity, every member file from
`03` onward treats it as the working surface it is supposed to be, and `30`'s account of building it
matches what I found reading the sources it cites.

## 1. Coverage: the finding that reproduces the panel's known failure mode

### 1.1 What `02` names as op's, and what happened to it

`02` (a carried file from the closed predecessor panel, present in this panel's own numbered sequence
and cited throughout as source material) closes with an explicit list:

> **What is genuinely undetermined, and is op's:** whether `Precision` counts the sign digit; and
> whether the order's own predicate is amended to identify shapes that denote the same value set, which
> turns out to be a precondition for `150`'s open question two rather than a separate matter.
> (`02:49-51`)

Two items. The first is in `OPTIONS.md`, in full, as its own entry: "Whether `Precision` counts the sign
digit... (`02` section 1.4)." I checked the citation; `02:299-336` (headed "1.6 A correction owed to the
four-condition order") is where the precision question and the predicate question actually sit together,
and the register's separate handling of the precision half is accurate to `02`'s content.

The second item is not in `OPTIONS.md` anywhere. I grepped for every wording I could construct:
`singleton`, `four-condition`, `grid and phase`, `vacuous`, `predicate.*amend`, `same value set`. One
hit, for `singleton`, at `OPTIONS.md:485`, and it is about a different topic (whether the singleton is
realisable under the ambient-and-realisation technique, `03:667-669`), not this one.

### 1.2 The question itself, and why it is not a minor omission

`02` section 1.6 (`02:299-334`) derives, with a compiled cross-check reported at 48 disagreements out of
576 ordered pairs, that the design's four-condition inclusion order (grid, phase, both endpoint
conditions, per `SETTLED.md:77`) is **vacuous on a numeral carrying fewer than two values**: a singleton
lies on every grid and in every phase, so the predicate reads a declaration it cannot see and gets
inclusion wrong in the direction that matters for `150`'s zero-width-numeral question.

`03` was independently dispatched to check this exact claim ("`02_carried` section 1.6 claims..." is the
opening line of `03`'s section 6, `03:491-494`) and confirms it with **three separate instruments,
arrived at differently**, which clears this workspace's own three-instance bar:

- Python, unsigned fixed point, radices 2 and 3, 1936 ordered pairs: 188 disagreements, **all 188**
  attributed to a source carrying fewer than two values, zero unexplained (`03:508-509`, probe `i2.out`
  Q7).
- Rust, scaled integers, a different containment algorithm, 484 ordered pairs: 28 disagreements, all 28
  attributed to the same cause, zero unexplained (`03:513-514`, probe `i3.out` C1).
- A negative control: `03`'s own first instrument reported zero disagreements over 1024 pairs, which
  would have refuted the claim, and `03` diagnoses why (a shape list containing exactly one degenerate
  numeral, at the coarsest declared step in the box, so the predicate was never offered the case that
  breaks it) and keeps the wrong instrument in the record with the diagnosis attached (`03:517-521`).

`03` then goes further than confirming: it shows the amendment is owed under **every** family-question
reading, not only the one `02` filed it under (`03:522-533`, "6.1 Where the amendment lands in the
consequence lists"), and offers two named, non-equivalent candidate repairs:

> **Amend the predicate.** State inclusion as the four conditions where the source carries two or more
> values, and as membership of the source's single value where it carries one.
>
> **Amend the quotient.** State that the order is on value sets... with the degenerate cases decided by
> the value sets directly.
> (`03:539-547`, condensed)

`03` states a preference (the quotient form survives the permanence test better) and explicitly flags it
as one read, wanting a second (`03:549`).

This is not a stray remark. It is a load-bearing definitional claim about the order the whole family
question (Q8) runs on, checked by two independent files with three instruments and zero unexplained
disagreements between them, cross-cutting every reading in the register's own Q8 section, with two named
candidate repairs and an explicit request for a second read on which. Under this panel's own mode ("a
member finding a shape nobody wrote down adds it here, and that is the most valuable single act
available", `RULES.md:152-156`), this is exactly the kind of thing the register exists to carry, and it
is not carrying it.

### 1.3 Why I read this as the same failure recurring, not a new one

`OPTIONS.md`'s own preamble states the register was rebuilt because the first pass, built from
`MORNING.md`, omitted an entire question (the dispatch brief for this file says the same). `30`'s rebuild
worked from the member files directly and still produced a register missing a second op-owed question
that two member files, working independently, spent real effort establishing. I do not think this is a
criticism of `30`'s method, which I found careful and honestly bounded (see section 5.4 of `30`'s own
file, and my citation checks below). I think it is evidence that a single read of twenty-six dense files
by one pass, however careful, will keep missing things, which is exactly why `RULES.md:116-118`'s
three-instance bar and this dispatch's own existence assume a second and third check are needed rather
than optional.

### 1.4 What else I looked for and did not find missing

I checked the following threads specifically, because they are dense, recent, or structurally likely to
have material that arrived after an earlier pass closed: Q7 (files `26`, `27`, the packing-under-
contention material), the derivation's-outputs section (files `15`, `16`, `17`), Q8's fork on the
canonical exponent (files `08`, `24`), Q2's fourth reading (`24`), Q5's strategy-definition material
(`25`), and the persona checkpoints' own audit sections (`09`, `14`) for anything they flagged as owed
that never landed. All of these are represented in `OPTIONS.md`, and where I checked their numeric
content against source it held (section 3 below). File `22`'s finding that the ratified "three
instructions per operation" and "seven bytes per value" figures do not survive the harness
(`22:561-580`) is not in `OPTIONS.md`, but I read this as correctly out of scope: it is a correction to
the evidence beneath a ratified row, not a design option, `22` itself says explicitly it is not op's to
bring as a question ("Not op's, and mine to state plainly", `22:643-648`), and the register's own stated
purpose is live design shapes, not evidentiary corrections to ratified rows.

## 2. Citation integrity, checked mechanically and then by hand

### 2.1 Method

I extracted every citation of the form `` `NN` section N.N `` from `OPTIONS.md` with a script (152
individual section references across 139 citation sites, `31_probes/check_sections.py`), parsed the
actual numbered headings (`##`/`###`/`####` lines matching `\d+(\.\d+)*`) out of every cited member file,
and checked each citation resolves to a real heading. I then hand-verified every mismatch by reading the
actual content at that location, because a heading mismatch can be a real citation error, a numbering
convention difference (some files use lettered clauses or spelled-out numbers instead of numeric
headings), or content genuinely present nearby under an unnumbered heading. I separately hand-checked
roughly forty load-bearing numeric or quoted claims against the cited section's actual text, across
essentially every question in the register (Q1 through Q9 plus the derivation-outputs section), by
opening the source and reading the surrounding paragraph rather than trusting the resolution.

### 2.2 What is not a defect, so it is not double-counted below

Six of the 152 mechanical mismatches resolved, on inspection, to citation-style differences rather than
wrong pointers:

- `` `22` section 10 `` and `` `22` section 11 ``: file `22` headings are spelled out ("## Ten: what I
  had to decide...", "## Eleven: what is op's..."). The content matches exactly; my parser only reads
  numeric headings.
- `` `16` section 0 ``: file `16` has no heading literally numbered "0", but its opening section (the
  contamination declaration) is conventionally "section 0" the way every other file's actual "## 0.
  Gates" section is, and the content the citation supports (the ONE-EXPERT downgrade on the count) is
  exactly there.

Four of the six are the three real defects below, one phrase cited twice under the same wrong pointer.

### 2.3 Defect one: `` `17` section 2.2 ``, cited three times, does not exist

`OPTIONS.md`'s Q1 section cites `` `17` section 2.2 `` three times (`OPTIONS.md:66,69,73`) for the claim
that fifteen expected-to-fail probes exist in the panel and not one tests whether a *declaration* is
admissible. File `17` has no heading numbered `2.2`. Its section 2 ("## 2. The proved, validated and
trusted split, clause by clause", `17:116`) is subdivided into lettered clauses C1 through C4
(`17:130,152,173,221`), not numbered subsections. The content itself is real and I found it: the
fifteen-probes claim and the count `grep -rlniE 'EXPECTED TO FAIL|...' */*.rs | wc -l -> 15` sit at
`17:205-215`, inside the range governed by heading `` ### C3, which is the least instrumented clause and
is also ambiguous `` (`17:173-221`). Under the natural mapping (C1=2.1, C2=2.2, C3=2.3, C4=2.4) the
citation should read `` `17` section 2.3 ``, not `2.2`. This is a small, mechanical, but real pointer
error, repeated three times in one paragraph.

### 2.4 Defect two: `` `15` section 9 ``, for a claim that is in section 8

`OPTIONS.md`'s derivation-outputs section states: "Nobody has built `Precise` as anything but the default
strategy under a different name in any probe in this panel (`15` section 9, `16` section 12)"
(`OPTIONS.md:713-714`). File `15`'s section 9 ("## 9. What is op's, and it is one thing rather than a
menu", `15:692-716`) is entirely about the `(W, F)`-versus-`(I, F)` consumer-surface question and does
not mention `Precise` at all. The actual sentence, verbatim, is at `15:669-670`: "I did not build
`Precise` as anything but `Warm` with a different name," inside `` ## 8. What I did not cover ``
(`15:654-691`). This is not a rebuild-introduced error: file `30`'s own writeup makes the identical
citation ("`15` says so of itself explicitly (`15` section 9)"), so the wrong pointer originated in `30`
and carried through unchanged into the register. The `16` half of the citation checks out: `16`'s section
12 (`16:682-716`) does discuss the `Precise`-dependent irreducibility question.

### 2.5 Defect three: a quoted phrase attributed to two files, present in one

The register's Q8 section: "it is explicitly marked dropped by `09`'s and `14`'s persona checkpoints as
"the direction most likely to eat a week and produce nothing", though this is persona judgement carrying
no authority (`09` section 6, `14` section 9)" (`OPTIONS.md:803-805`).

I searched both files for the exact phrase. It is in `09`, verbatim, once: `09:253`, inside the `### 07`
subsection of `` ## Per file: what holds, what is thin, what I would push on ``, not under `` ## What I
would refuse `` (which is `09`'s sixth heading and the section the citation names). It is **not** in `14`
at all; `grep -n "eat a week" 14_persona_checkpoint_two.md` returns nothing.

`14`'s section 9 ("## 9. What I would drop", `14:594-613`) does discuss the same substance and reaches
the same conclusion ("The Moore completion, the `canonical_exponent` naming debt, and `03`'s reading F
stay dropped," `14:612-613`), so the register's underlying claim, that both checkpoints agree the Moore
completion should stay off the map, is true. What is wrong is presenting it as a shared direct quotation
when the wording is one file's, and citing the wrong section of the file that does contain it. A reader
who opens `14` at section 9 looking for the quoted sentence, per the citation, will not find it and may
reasonably doubt whether `14` said this at all.

### 2.6 What I checked and it held: a representative sample

Numbers and quotes I verified exactly against the cited source, spanning every question:

| claim | cited at | source |
|---|---|---|
| `24`'s grid-and-reach set equality, 121 of 121 | Q2 | `24:417-461`, confirmed |
| `06`'s tight product form, negative-width corner 7/625, 11/2401, 15/6561 | Q8, standing item | `06:591-618` |
| `25`'s preset table as an exact 2x2 of headroom and layout | Q5 | `25:219-260`, confirmed, probe `p2_decompose_the_preset_table.py` |
| `25`'s 34-run bench family, zero wins for the shipped doubled container | Q5 | `25:472-521`, "34 committed runs", "wins zero of 34" |
| `20`'s clamp headroom, 2.2x worse at arity two to 44x better at arity 256 | Q6 | `20:186-222` |
| `11`'s `typenum` figures, 1148 rows, 4758 generated lines, capped at 1024 dense | Q9 | `11:362-441` |
| `27`'s break-even carrier table, one core against four | Q7 | `27:653-691` |
| `27`'s `u16` figure, -59 to -111% superseded by 8.9 to 15.9% packing win | Q7 | `27:653-691`, `27:920-1017`, confirmed with the sign convention resolved by reading section 15.1 |
| `27`'s UADALP mechanism, 23.2-24.6% at one core, +0.9% at four | Q7 | `27:595-617` |
| `26`'s eight-byte carrier, wins past 2M records by 13-21% | Q7 | `26:652-683` |
| `16`'s carrier-only derivation, 23.1% overshoot, four of four green | derivation outputs | `16:284-401` |
| `24`'s knee shape, two integers not a list | Q8 | `24:573-663`, "max(K" pattern confirmed |
| `18`'s absorbing-top counterexample, 936/5184 and 840/5184 | Q4 | `18:349-421` |

Every one of these reproduces. I did not find a fabricated or inflated number anywhere I checked.

### 2.7 Probe citations

All sixteen `` `NN_probes/...` `` citations in `OPTIONS.md` resolve to files that exist on disk
(`31_probes/check_probes.py`). None fabricated.

## 3. A gap the register itself half-names: `DROPLIST.md` is stale against it

`RULES.md:147-150` states the procedure for a closed route: "move it to `DROPLIST.md` with the
diagnostic that closed it... then strike it here with a pointer." `OPTIONS.md` currently carries at
least three fully-written closed-route entries that this procedure says should have moved: Q8's reading
G (ordering by something other than inclusion, `OPTIONS.md:474-479`, the only one explicitly labelled
"belongs in `DROPLIST.md`"), and two closed Q9 routes (the bare byte-count carrier, `OPTIONS.md:611-619`;
the macro-generated bridge table, `OPTIONS.md:621-626`). I grepped `DROPLIST.md` for every distinguishing
phrase from all three ("byte-count carrier", "macro-generated bridge", "ordering the numerals",
"inclusion") and found none of them. `DROPLIST.md` is entirely the predecessor panel's inherited content;
nothing from this panel's own closed routes has been added.

This is not a fresh discovery. `30`'s own file says so directly: "flagged all four in `OPTIONS.md` as
closed-with-citation rather than removing them silently... `DROPLIST.md` maintenance was out of scope for
this dispatch" (`30:145-151`). I confirm the gap is real, unchanged, and still open, and I note the
compounding point `30` did not: `OPTIONS.md` is not just holding these entries pending a move, it is
holding them in full prose rather than "struck... with a pointer" as `RULES.md` specifies, which is a
reasonable and stated deliberate choice (a member reading the register should see what was tried) but is
itself a small deviation from `RULES.md`'s letter that nothing has named as a deviation until now.

## 4. On the instrument itself

Existence and locus, both asked for explicitly by the dispatch: I looked for a reason to say the register
should not exist, or sits on the wrong side of the panel's own boundaries, and did not find one. `RULES.
md:143-165` mandates it in terms that leave nothing to infer, op's own words at `00_brief.md:62-67`
describe the mechanism directly, and every member file from `03` onward reads and is read against it as
working infrastructure rather than as an artifact anyone treats as authoritative. Nothing in it carries
the concrete spelling of an implementation; the container-derivation and width-surface entries describe
what was compiled and what it established, never a snippet a reader would mistake for the required shape.
I find this consistent with `30`'s own account and see no basis to add to or subtract from it.

## 5. Coverage, stated as a bound

**Read in full**, opening the cited lines rather than trusting a summary: `RULES.md`, `OPTIONS.md`,
`DROPLIST.md`, `30`, `21` (my own prior entailment check on `MORNING.md`, read for method and for the
citation-checker pattern I extended). Full text of `02`, `03` sections 6 and 6.1, `06` sections 7.2 and
8-10, `07` sections 7-8, `08` section 7, `09` (the "What I would refuse" and "Per file" sections in
full), `14` section 9 and its heading list, `17` section 2 in full (all four clauses), `18` section 6-7,
`22` sections 9-12.

**Read the section headings of every member file `03` through `27`** (`31_probes/section_headings.txt`),
and read substantially, meaning several hundred lines each at the passages a mechanical check flagged or
that bear on Q7, Q9, the derivation-outputs section, and Q8's canonical-exponent fork: `10`, `11`
(sections 7, 10), `12` (sections 3, 6, 7), `13`, `15` (sections 1.5, 8, 9), `16` (sections 10, 12), `20`
(sections 1.4, 1.5, 2.1), `24` (sections 2.1, 2.2, 3.5), `25` (sections 4.1, 6.2), `26` section 10, `27`
(sections 9, 10, 10.4, 15, 15.1).

**Checked mechanically, exhaustively**: every section-style citation in `OPTIONS.md` against the real
heading structure of the file it cites (152 individual references), and every probe-file citation against
disk (16 references), per the committed scripts.

**Checked by hand, sampled, not exhaustive**: roughly forty numeric or quoted claims, chosen to cover
every question and to weight toward the newest and densest material (Q7, the derivation outputs, Q8's
fork) rather than uniformly. I did not check every claim in `OPTIONS.md`; I did not open every probe
output file the register cites, only the ones bearing on a claim I was checking; I did not re-verify any
probe's own arithmetic, per the dispatch's scope (that is `30`'s stated gap, not mine to close here). I
did not read files `04`, `05`, `19`, `23`, `28`, `29` beyond what earlier checkpoints (`09`, `14`, `21`,
`30`) quote from them, on the grounds that `28` and `29` are explicitly not option-bearing (`OPTIONS.
md:834-838` says so and I have no reason to doubt it) and `23` is a 1206-line inventory `30` already read
in full and cross-checked, which I chose not to duplicate.

**One instance, not three.** This is my read alone, run from the sources forward as the dispatch asked.
`30`'s own account of building the register is one instance of the entailment check it needs; I am a
second, and the predicate-amendment gap in section 1 is exactly the kind of thing a third, independent
pass would be likeliest to catch that I did not: I stopped after finding one clear miss and did not keep
searching for a second at the same depth once I had confirmed it against `02` and `03`'s own words.

## Relevant files

`/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel/OPTIONS.md`
(the file checked, not edited), `DROPLIST.md`, `RULES.md`, `02_carried_what_replaces_the_two_refutations.md`,
`03_lamport_the_family_question_and_its_consequences.md` (section 6), `15_giesen_the_axes_the_ladders_left_out.md`,
`17_leroy_what_would_actually_certify_this.md`, `09_persona_checkpoint.md`, `14_persona_checkpoint_two.md`,
`30_willsey_rebuilding_the_option_register.md`, and the probes at `31_probes/`.
