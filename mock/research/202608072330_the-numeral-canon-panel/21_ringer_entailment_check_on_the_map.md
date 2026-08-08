# 21. An independent entailment check on `MORNING.md`

**Position:** an outside check that the map entails its sources, run by someone who did not write it,
per `RULES.md:145-160` and the workspace rule `a-compression-is-checked-by-someone-else.md`. I did not
contribute to the design and this file makes no design claim.

## The verdict, before the detail

The map's **arithmetic is in very good shape**. I checked fifty-one numeric claims against the files
and probe outputs that produced them, and fifty of the fifty-one reproduce exactly. That is a better
rate than the predecessor compressions this workspace has audited, and the dispatcher's own
self-corrections, which I checked separately rather than taking on trust, are accurate where they
claim to correct.

The defects are not in the numbers. They are of one shape, and it is the shape `19` named at its
section 4.4: **in every case the expert's own hedge is present in the expert file and absent or
weakened in the map.** `19` found three instances and called it systematic. All three are still
standing unrepaired, and the section written *after* `19` landed reproduces the pattern a fourth time,
on the newest and most quotable number in the document. The diagnosis was accepted and the practice
that produced it did not change.

Separately, and mechanically: the map carries **295 numeric tokens and zero `file:line` anchors**,
against 119 unique anchors in the sources. That is not a set difference with losses in it. It is
total.

## 1. Claims I could not verify, and what I found instead

Five. Two are refuted by committed probe output, two are stale sentences that were true when written
and are false now with nothing marking them, and one is arithmetic the map performed and attributed to
a file that does not contain it.

### 1.1 Question four is stated as an iff, and the "only if" half is refuted by `18`'s own table

The most consequential defect I found, because it is one of the six questions the map puts to op, and
because the refuting evidence sits inside the file the claim is drawn from.

The map, at its question four and again at its section six:

> absorbing is sound **exactly while the computation stays at the endpoint**, which additions alone
> satisfy and subtractions do not.

> **an absorbing endpoint is sound exactly while the computation stays at it.**

I read `18_probes/p2.out` directly rather than `18`'s prose. Its section B, verbatim:

```
# B. an operation set that can decrease
add and subtract, 4 steps          chains=   5184  unsound point=   2464  unsound absorbing=    936
add and multiply by zero, 3 steps  chains=    512  unsound point=     48  unsound absorbing=      0
add and multiply by one, 4 steps   chains=   1024  unsound point=    512  unsound absorbing=      0
```

and its witness block:

```
add/mul0   none: absorbing stayed sound over the whole enumeration
```

Multiply by zero decreases. The operation set sits in the file's own "can decrease" category, and
absorbing is sound over the whole enumeration at zero failures. So "stays at the endpoint" is
**sufficient and not necessary**, and the word "exactly" makes the map's sentence false.

`19` reached this first, at its 5.6, by rerunning the same probe. Its statement of the real condition
is better than the map's or `18`'s: every operation must map the absorbed set onto a set the numeral
denotes exactly. Multiplying `[top, inf)` by zero gives `{0}`, which the numeral has; subtracting one
gives `[top-1, inf)`, which it does not.

`18:403-404` says of this result that it "is the single thing in this file I would most want checked
by someone else". It has now been checked twice, by `19` and by me, from the same committed table, and
the map is unchanged. The 936 of 5184 and the 840 of 5184 are both exact; it is the quantifier around
them that is wrong.

### 1.2 "No file in this panel names `mock/benches/` at all"

False as the document currently stands. Measured:

```
grep -c 'mock/benches' 19_persona_checkpoint_three.md          ->  10
grep -c 'mock/benches' 20_fog_what_the_benches_already_know.md ->   8
```

The sentence was true of files `02` through `18` and stopped being true when `19` found the benches
and `20` audited them. Nothing marks it as a statement about a past state, and it sits four lines
above a diagnosis ("nothing told them it was there") that a reader will take as current.

The companion clause, "one mentions a bench round in a single clause", understates by a different
route. Nine of the thirteen expert files contain the string `bench`; `15` has nine occurrences, `10`
four, `17` three. The clause is defensible only under the reading "one names a specific prior bench
round", which `10:190` does. As written it reads as a claim about the panel's bench awareness, and
under that reading it is wrong by an order of magnitude.

### 1.3 "Everything is unpriced. No bench harness run bears on any of it, and no file claims otherwise."

Map line 870, in the method notes, with no qualifier of any kind. It contradicts the map's own opening
correction 800 lines above it, which reports a committed harness run with confidence intervals and
significance, and it contradicts the map's own section seven, which reports `20` auditing 147 findings
files and finding 146 of them taken from a dirty tree.

The neighbouring sentence has the same defect: "**All four experts kept a broken instrument**". Four
was the panel's expert count when that paragraph was written; it is now thirteen, and the sentence is
framed as a method note about the panel rather than about a stretch of it.

That paragraph also carries a sentence with a noun missing, which reads as an excision rather than a
compression:

> `03`'s first reported zero disagreements through setup that helped

I could not reconstruct what was removed, and I did not guess.

### 1.4 "the design's actual inference surface is ten sites, not twenty", attributed to `06`

`06` does not contain the figure. `grep -nE '\bten\b' 06_kiselyov_where_a_numeral_is_inferred.md`
returns three hits, all of them the ordinal "site ten". The map presents "ten sites, not twenty" in
bold as "`06`'s headline rather than a footnote". It is the map's own subtraction, twenty sites minus
eight D0 minus site 16 "not found" and site 20 "absent", and the subtraction is defensible.

What is not defensible is that it contradicts the map's own table three lines below it. `06:341-348`
classifies site 1 twice, annotating it "sites 1 (first step)" under D1 and "sites 1 (second step)"
under D2. The map reproduces `06`'s counts, 8 / 8 / 3 / 0, and drops the annotation. A reader adding
the map's own D1 and D2 gets eleven inference sites on the same screen as the sentence claiming ten,
with nothing on the page to reconcile them.

`06` is itself fragile here in a way the map inherits rather than causes: `06:718` says addition would
join "the six sites that infer nothing" where `06:341` counts eight. I flag it as a source fragility,
not a map defect, because the map took the better-supported number.

### 1.5 The 476-against-461 reconciliation is filed under the wrong expert's section

The map places "A number reconciled before it could become a second contradiction" inside section
seven, whose heading is "your second standing instruction, discharged, and what the repository knew"
and whose every other subsection is `20`'s. The reconciliation is `15`'s, at `15:183-199` and
`15:731`, from probe `q03b_reconcile_461_vs_476.py`.

The sentence itself opens "`15`'s product-overshoot count", so the attribution is not false. The
placement is, and it is the kind of error that survives, because a reader skimming headings files the
result under the bench audit and will look for it there later. The arithmetic is exact: `15:199` gives
`160 + 301 + 15 = 476` against `06:551`'s 461, both correct under different conventions.

## 2. Rung errors

Four, and three of them were already found and flagged by earlier persona checkpoints. What I add is
that they are still standing, and that the fourth is new, arrived after the third checkpoint named the
pattern, and lands on the number the map most wants op to act on.

### 2.1 Two instances called "the bar", against `RULES.md:116`. Fourth flag, still unrepaired

The map, on `07` corroborating `06`'s tight product form:

> That is two instances arrived at differently, which is the bar.

`RULES.md:116`, quoted exactly:

> **One instance of evidence is never enough.** Three independent ones is the bar, and independence
> means arrived at differently, not three probes sharing one model.

`09` flagged this at what was then line 353. `14` flagged it again by the same line number and recorded
that six of `09`'s seven items were repaired and this one was not. `19` flagged it a third time at its
5.1, by then at line 668. It is now at line 780, unchanged, in its fourth stretch.

`19`'s reading of why this one matters more than its size suggests is right and I will not improve on
it: merging the two-expert provenance rung with the three-instance evidence bar is the operation that
produces every other rung inflation in the document, because once the two words are interchangeable
the document can promote either from either.

The underlying claim is fine. `07:57` and `07:465` do report `06`'s corrected form best at 400 of 400
operand pairs by a derivation that does not use `06`'s inequality. That is two instances. It is not
the bar.

### 2.2 The blindness finding is stated at two experts and stands at one

The map:

> The **count** is one expert, by `16`'s own downgrade. The **identity** of the two, the **keying** of
> the second, and the **blindness** of the certifying check were derived independently and stand at
> two.

`16` says the opposite about the blindness, in its own words, twice. `16:632-633`:

> **That same check would be equally green over a derivation that emits only the container**, and `15`
> does not say so.

and `16:648`:

> Mine is about a one-output map passing the certification. They compose: **the second output is both
> harder to get right and impossible to notice missing**, and the second half is not in `15`.

A finding one expert derived and no second expert reached is one expert, however independently it was
derived. `19` established this at its 4.3 with these same citations and offered the accurate form,
which is stronger than the inflated one and which the map still does not carry: **one expert, plus one
checked reproduction**, `17` having reproduced it while stating at `17:758-759` that it read `16` in
full first.

### 2.3 `20`'s self-declared contamination is dropped, where `16`'s was preserved

This is the new one, and it is the reason I would put this section in front of op rather than the
citation count.

The map's question five, one of its six, rests on the clamp family reversing the wrapping answer:

> Under clamping, headroom goes from 2.2x worse at fold arity two to **44x better at arity 256**, with
> the crossover landing exactly where its own safety predicate says.

Both numbers are exact, at `20:205` and `20:198-203`. They come from `20`'s section 1.5, whose opening
paragraph is `20:188-191`:

> Read after I had derived the sections above, and I note that I read the commit subject line
> `docs: panel file 142, excel everywhere, and the headroom is a fold quantity` while establishing
> commit order, which carries a conclusion. Everything above this paragraph predates that; **treat this
> subsection as contaminated and worth an independent read.**

`20` repeats it in its closing coverage statement at `20:674`: "Section 1.5 is flagged contaminated by
a commit subject line and is owed an independent read."

The map carries neither sentence. It carries the section's numbers, in bold, as one of the six things
op is asked to decide.

What makes this a pattern rather than an oversight: the map has a subsection titled "An honest
self-report worth reading" that records `16`'s equivalent contamination declaration faithfully, in
detail, and praises it. The same disclosure from `20`, on a more consequential number, arrived after
`19` had named the tilt, and did not survive the compression. `19`'s count was three for three. It is
now four for four, and the fourth was written after the diagnosis.

### 2.4 A source hedge dropped that changes the question's shape

Adjacent to 2.3 and lesser, but it bears on the same question. The map:

> The ratified preset table gives `Warm` the clamp and gives wrapping to `Hot` alone, and you have
> already declared that cell stale.

Both halves are sourced: `20:161-164` for the ratified table, `20:166-172` quoting
`seed/SETTLED_container.md:405-408` for op having declared the cell stale. What the map drops is
`20:174-177`:

> **So the same round contains a bench built on the premise that Warm clamps and a checkpoint recording
> op killing that premise.** I cannot tell from the artifacts which came first within the round, and it
> does not matter much: what matters is that **the record currently supports both readings** and the
> two instruments give different answers.

The map's version reads as though the record points one way and op has already settled it. `20`'s says
the record supports both and the ordering is unrecoverable. That is a different question to hand
someone.

## 3. The citation set difference

The script is committed at `21_probes/count_anchors.sh` and reruns from the panel directory.

### 3.1 The totals

```
file:line-style anchors, total occurrences
  sources (03, 05..20):   297
  MORNING.md:               0

file:line-style anchors, unique targets, excluding MORNING self-citations
  sources:                119
  MORNING.md:               0

probe filenames named (.rs / .py / .out / .sh)
  sources, unique:         47
  MORNING.md:               1   (count.sh)

numeric tokens in MORNING.md:  295
```

### 3.2 The set difference is the whole set

There is nothing to diff. The map cites **zero** line-precise anchors, so every one of the 119 unique
targets in the sources is unreachable from it. The distinct files behind those 119, which a reader of
the map alone cannot get to for any claim:

```
00_brief.md                          10_probes/p12_improved_full.rs
01_op_answers.md                     12_probes/p11_diag_battery.rs
124_consolidation_twelve.md          12_probes/p14_lazy_type_alias.rs
137b_op_checkpoint_thirty.md         16_probes/p3_blind_suite.rs
140b_op_checkpoint_thirtythree.md    b01_table_caps_the_algebra.rs
141_xu_the_container_fork_benched.md bench-harness/src/harness.rs
202607281616_prior_art/02_...md      generic-array-0.14.7/src/lib.rs
PERSONA_CALLS.md                     ladder.rs
RULES.md                             mock/crates/arvo-storage/src/layout_assertions.rs
SETTLED.md                           mockspace/bench-core/src/counter.rs
SETTLED_container.md                 p13_where_the_door_error_lands.rs
SETTLED_laws.md                      p3_blind_suite.rs
seed/SETTLED_container.md            p4_cross_family_join.rs
seed/SETTLED_laws.md                 p5_merged.rs
seed/SETTLED_surface.md              p5_total_ladder.rs
unstable-features.md                 q07_three_input_map.rs
```

Three of those are worth naming individually, because they are the establishing sources for claims the
map presents in bold and op is being asked to act on:

- `seed/SETTLED_container.md:405-408`, the record that op declared the `Warm` clamp cell stale. The map
  asserts this in question five and the reader has no route to the text.
- `124_consolidation_twelve.md:2604-2612`, the ratified preset table itself.
- `RULES.md:116`, the three-instance bar the map contradicts. The map cites `RULES.md` once, by name,
  in a different context.

### 3.3 The 13 probe stems the map does carry are ambiguous by construction

The map names probes as bare stems: `p02`, `p03`, `p07`, `p08`, `p12`, `p13`, `p14`, `p23`, `p24`,
`b01`, `b02`, `b03`, `e01`. I checked each against disk. **Seven of the thirteen resolve to three
different files each**, in three different probe directories:

```
p02 -> 10_probes/p02_assoc_const_min_gca.rs
       12_probes/p02_const_door_alias.rs
       13_probes/p02_mgca_assoc_const_path.rs
p12 -> 10_probes/p12_improved_full.rs
       12_probes/p12_first_day_errors.rs
       13_probes/p12_surface.rs
```

and so on for `p03`, `p07`, `p08`, `p13`, `p14`. The map disambiguates in prose, writing "(`12`, probe
`p02`)", so a careful reader can resolve them. But the stem carries no information on its own, and
`10_probes/p12` and `12_probes/p12` are unrelated probes about different questions. The sources name
full filenames; the map named stems. That is the same compression operating on a smaller unit.

All thirteen exist on disk. None is fabricated. I checked.

### 3.4 The repair `19` prompted was applied to the text and not to the practice

`19:534-548` measured the third stretch at zero anchors and zero probe names against 60 numeric tokens,
and quoted the workspace rule at the finding. The section written after that measurement, the map's
section seven on `20`:

```
sec7 numeric tokens: 35
sec7 probe stems:     0
sec7 file:line:       0
```

Thirty-five new numbers, no new anchors. `19`'s own sentence for this is the one I would keep: the
document was patched, and the practice was not.

## 4. What a reader of the map alone would not learn

Ranked by whether the absence changes a decision.

### 4.1 `17`'s deliverable is not in the map at all

`17:95-96` states what its file is: "That is what a trusted-base list is for, and it is why the
deliverable below is a list rather than a verdict." Section 6 is that list, nine items.

```
grep -ciE 'trusted base|trusted-base|ban list|specializ|TypeId' MORNING.md  ->  0
```

Zero. The map carries `17`'s findings about the instruments and drops the artifact `17` says it was
for. `19` found this at its 5.7 and it is unchanged.

The item with a consequence outside this panel is the one about the bans on full `specialization` and
`TypeId`, which hold because the transfer from a bounded check to the full width range rests on them.
`unstable-features.md` in this workspace has a whole section on that dependency. A reader of the map
would not know the panel had touched it.

Related and also absent: `17:236-237` marks dispatch erasure as proved by construction "**conditional
on the ban list holding**". The map carries the split and not the condition. A proof by construction
with an unstated side condition reads as unconditional, and that is the strongest-sounding sentence in
the map's section five.

### 4.2 `20`'s coverage bound

`20:670-674`, its own "Not covered":

> Roughly 30 findings files in the quantiser, spectral, structural and hash families, read only through
> the impossible-throughput sweep. **No re-run of any bench.** No reading of `141`, `142` or the
> predecessor panel's files beyond the two paragraphs `SETTLED_container.md` quotes and the bench crate
> doc comments I cite. Section 1.5 is flagged contaminated.

The map's section seven opens with "The verdict is that the bench body is sound" and closes without any
of this. A reader takes away that the benches were audited. What happened is that the committed CSVs
and findings files were read and nothing was re-run, which is a different and weaker thing, and `20` is
scrupulous about saying so.

### 4.3 One of `16`'s four green checks is a self-declared tautology

The map:

> `16` ran that check over a deliberately one-output derivation whose storage is **23.1 percent over**,
> and it came back **four of four green**.

`16:639-641`:

> four of four green over a carrier-only derivation of `UFixed<13,0,Cold>`, including the per-value
> erasure check and **including a `size_of` check that is a tautology**, while the stored form is 23.1%
> larger than the strategy promised.

The tautology is the point of the demonstration, and `16` flagged it. This workspace's own test gate
calls a tautological test not a test at all and says it inflates a count that gets cited as coverage.
The map cites the count.

### 4.4 The widths are gone from the width-banded result

The map:

> The harness answer is **width-banded and not monotone**: 44.2x, 0.98x, 21.0x, 7.0x, 0.99x, 2.45x
> across the widths measured. The split is exactly filled versus sub-rung widths.

`20:99-104` attaches those to W = 8, 13, 16, 32, 60, 64. The map states the mechanism and strips the
mapping, so a reader who wants to know which two widths are the near-1.0 cases, which is the entire
content of "the split is exactly filled versus sub-rung", cannot recover it. They are 13 and 60.

Same shape, smaller: "a million small values are 625 KB at `Cold` against 1 MB at `Warm`" drops that
the row is `UFixed<5,0>`, and `15:329-332` has four other rows including one where `Hot` is *larger*
than `Warm` at W = 200, which is the wide-rung alignment finding the map mentions separately without
connecting.

### 4.5 The 96 denominator on the cross-radix count

The map: "radix produces 60 structurally unreachable cross-radix joins." `03:273-275`: 96 cross-radix
pairs with no upper bound, of which 60 are structural in a sense no enlargement touches. Sixty out of
ninety-six is a different fact from sixty. `17`'s own classification says counts are the most fragile
class and the fragility is always the unstated domain. This is that, in the map, about a count.

## 5. What verified clean, because a clean result is a result

Fifty of fifty-one numeric claims reproduce exactly against the file or probe output that produced
them. I am listing these rather than only the failures, because a check that reports only defects gives
no way to judge how hard it looked.

**The `Cold` correction, which I re-derived rather than relayed.** I opened
`mock/benches/bitpack-sequential-sum_n16384_findings.md` myself. Line 45: spread 4.61x, fastest 1667.3
ns, slowest 7678.9 ns. Line 86: baseline `bitpack-aligned-seq` 5570 ns. Line 88: `bitpack-zeropad-seq`
7679 ns, +37.9%, CI `[+2107, +2116]`, significant. Line 146: `bitpack-zeropad-seq` won 0/40, lost
40/40. The map's table is exact and its "zero of forty passes" is exact.

**The retraction of the scale crossover.** `20:409-437`. No committed run contains both a packed and a
dense arm; the families are disjoint; the two runs subtracted were at different source commits. The
real figures, `20:433-435`, are 4.10x at n=256 falling to 3.34x at n=16384. The map's "falls with n and
stays large: 4.10x down to 3.34x, not 1.43x, and not a crossover" is right, and the retraction is
honest about being the dispatcher's own error.

One internal tension worth a line and not a defect: those two ratios are the *byte-aligned* arm against
native. The map's own table three sections earlier identifies the *zeropad* arm as the packed one,
whose ratios are 5.52x to 4.61x. `20:582` uses the aligned reading as "the" packed penalty, so the map
followed its source. A reader holding both passages will not be able to tell which arm "the penalty"
names.

**Verified exact, by file and line:**

| claim | source |
|---|---|
| 936 of 5184, 840 of 5184 | `18_probes/p2.out`, `p2b.out`, read directly |
| 42.05% falling to 35.45% decidable | `18:193`, `18:199-200` |
| 24 rung / 2 sign / 4 stride / 0 width-keyed impls | `15:288-293` |
| 4 strategies x 2 signs x width 0 to 200, 1608 triples, zero features | `15:299-306` |
| 625 KB against 1 MB at a million elements | `15:329-332` |
| 6400 + 6561 assertions over the 81-shape box | `15:734` |
| 7 of 625, 11 of 2401, 15 of 6561 | `15:83`, `06:609` |
| 476 = 160 + 301 + 15 against `06`'s 461 | `15:199`, `06:551` |
| `Cold`/`Precise` zero, `Hot`/`Warm` 648 | `15:263`, `15:267-268` |
| 23.1 percent over, four of four green | `16:321`, `16:371`, `16:639` |
| carrier wrong at 28 of 64 widths | `16:384`, `16:515`, `16:675` |
| 174 of 420 against the naive suite's 41% | `17:306`, `17:322` |
| 66 instructions each, 14 opcodes each | `17:489` |
| stable across three toolchains, three months | `17:548`, `17:625` |
| D0 8 / D1 8 / D2 3 / D3 empty | `06:341-348` |
| 6100 of 6561 tight, 461 wasting one bit | `06:48`, `06:551` |
| 512 of 1024 point, 55,085 of 65,536 wrapping | `07:74`, `07:599` |
| 34,976 pairs zero failures, 184 out of range | `07:40`, `07:274` |
| 400 of 400 operand pairs | `07:57`, `07:465` |
| 670 of 1326 pairs, bias | `03:264` |
| 60 structural cross-radix (of 96) | `03:273-275` |
| 18 of 21 value sets inside | `08:32` |
| 16 to 34 percent enlargement | `08:69` |
| `{0, 1/2, 1, 2, 3}` strictly inside both bounds | `08:58` |
| 1148 rows, 4758 generated lines (typenum) | `11:43`, `11:406`, re-confirmed at `14:369` |
| 1636-bit numeral at 208 bytes | `12:91-92`, `12:311` |
| `UInt<5>` 7 chars, `UFixed<13, 3, Hot>` 18, identical to C0 | `12:136`, `12:140`, `12:142` |
| six-row door `0, 3, 5, 8, 13, 24`, no 48/96/192 | `12:161-163` |
| 4711-bit numeral, three crates | `12:442` |
| 4225-pair matrix | `13:189`, `13:573` |
| six bodies, 95 instructions | `10:108`, `10:365` |
| ten hits for "fresh eyes" | `10:72`, re-confirmed at `14:334` |
| 8193 rows compile, 3.11 s | `10:484` |
| thirteen routes, six attacked | `12:356` |
| 468 lines with no "strategy" | `14:70-71` |
| 44.24x / 0.98x / 21.02x / 7.03x / 0.99x / 2.45x | `20:99-104` |
| 21.6x projection effect at W=13 | `20:130` |
| 2.2x at arity 2 to 44x at arity 256 | `20:205` |
| 48x to 66x above roofline | `20:248-250` |
| 6 impossible of 691 cells | `20:31`, `20:318` |
| 33x affine, 7.5x widening, 16.8x to 1.03x const arity | `20:486`, `20:516`, `20:538`, `20:545` |
| 0 of 55,280 rows with a digest, 147 findings files, 146 dirty | `20:369-373`, `20:22-23`, `20:662` |
| 1600 corrected to 339 against 81 | `20:88-91` |
| ratified table gives `Warm` clamp, wrapping to `Hot` alone | `20:161-164`, `20:633` |
| arm matrix asymmetric, one config against four | `20:142` |
| 81-versus-zero between `148` and `150` | `SETTLED.md:143-146` |

**The three self-corrections the map makes about itself are accurate.** The brief-error count moved
from three to five and five is what `19:605-616` counts, three to experts and two to the persona. The
"four probe directories" number was removed rather than repaired, which is the right move given that
`19:560-565` found four available counts and none of them was four probe directories. The `SETTLED.md`
"either" defect and the ergonomics-bar qualifier drop are both narrated with their establishing
sources named.

**Every probe the map names exists on disk.** Thirteen stems, all resolving, none fabricated. That is
worth stating explicitly because the panel rules exist partly because a predecessor cited five probe
files by name that existed nowhere.

## 6. Coverage, stated as a bound

**Read in full:** `MORNING.md` (894 lines), `20` (725), and `18_probes/p2.out` and `p2b.out` in full.

**Read substantially, meaning several hundred lines each at the passages that bear on map claims:**
`19` (944 lines, sections 4 and 5 closely), `16` (section 11), `15` (sections 1.5, 3.2, 3.3, 5), `12`
(sections on C0 to C4 and the door), `06` (sections 2, 2.1, 3), `17` (sections 3.4, 4.3, 6).

**Read only through targeted grep:** `02`, `03`, `05`, `07`, `08`, `09`, `10`, `11`, `13`, `14`.
Claims sourced to those files were verified by opening the cited line and its surrounding paragraph,
not by reading the file.

**Claims checked:** fifty-one numeric claims plus roughly twenty non-numeric attributions. **Sampled
rather than exhaustive:** I did not verify every sentence of the map. I worked from the map's bolded
claims and its six questions outward, which is a bias toward the load-bearing text and away from
connective prose.

**Where I ran out of confidence, in three places, each stated so nobody reads silence as clearance:**

First, I did not open the CSVs behind `20`'s medians. I verified that the map reproduces `20`'s tables
and that `20` states its commands. Whether `20`'s medians are correct against `mock/benches/*.csv` is
unchecked by me, and it is the layer where a number would be hardest to catch.

Second, the map's historical self-claims ("an earlier version of this section said", "an earlier
version of this line said three") I checked only against the `19` and `14` quotations of those earlier
versions, not against `git show` of each of the eight commits touching the file. Where `19` or `14`
quoted the old text I could compare; where they did not, I took the map's account of its own history.

Third, `08`'s and `03`'s sections of the map got the lightest treatment. Their headline numbers verify,
but I did not read either file's argument closely enough to say whether the map's *reasoning* summaries
entail them. If a fourth entailment check is run, those two are where I would point it.

**What I did not do at all:** I did not check `CANON_CANDIDATE.md`, `DROPLIST.md`, `PERSONA_CALLS.md`
or `SETTLED.md` for consistency with the map beyond the two lines cited above. I did not re-run any
probe other than reading `18_probes/p2.out` and `p2b.out`. I did not evaluate any design question and
this file contains no design opinion.

**Committed evidence:** `21_probes/count_anchors.sh` and `21_probes/src_anchors.txt`, both reproducing
the counts in section 3 from the panel directory.
