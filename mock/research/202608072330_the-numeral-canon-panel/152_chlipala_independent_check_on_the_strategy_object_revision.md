# 152. Independent check on the strategy-object candidate's revision

**Member:** Chlipala. I took no part in `139` through `151`. My job is to check `151` against the
sources it claims to entail, working from `139` forward rather than from `151` backward, and to verify
rather than trust every load-bearing claim the brief named.

**Verdict, stated first because the brief asks for the outcome before the method.** `151` is sound. Every
claim I checked by independent computation reproduced. The false clause and its repair are both correct,
and I derived the repair's core fact (nearest-half-even's non-equivariance survives restriction to the
non-negative domain) by hand, from the rounding-mode definitions alone, before opening any probe, and it
matched exactly. The intersection-instrument fix is correct and its diagnosis of *which* dimensions are
empty is right where `148`'s own guess was wrong. The self-accounting number (33 findings carried) is
reproducible and honestly computed with the accounting section excluded from itself. I searched for a
third instance of the verdict-contradicts-its-own-table defect and did not find one in the files I
checked; I say exactly which files those were. One clause in `151` cites a range of numbers that differ
from the file it credits them to, and it is not an error: it is a second, independently-widthed
instrument, correctly attributed as its own. I found nothing that changes what `151` should say.

---

## 0. Gates

### 0.1 Canon gate: passed

There is no ratified canon and `mock/canon/` does not exist on this branch (`ls mock/canon/` returns "No
such file or directory"). The governing material is `INTENTS.md`, `RULES.md`, and `OPTIONS.md` entry
Q51. My assigned work is to verify an entailment and a set of measurements, not to propose a design;
nothing below asserts a design decision, and where `151` itself declines to settle something (the
firewall's unpredicated status, which units a weighting is expressed in, whether the accumulator cell is
stated conservatively or precisely), I leave it exactly as declined.

### 0.2 Test gate: run in full, before any of the assigned work

Per crate, by `--manifest-path`, as the brief specifies. I did not trust the inherited figure; I ran it.

```
$ cargo test --manifest-path variants/bitpack-carrier-shared/Cargo.toml     # 9 passed
$ cargo test --manifest-path variants/bitpack-contend-shared/Cargo.toml    # 12 passed
$ cargo test --manifest-path variants/bitpack-footprint-shared/Cargo.toml  # 6 passed
$ cargo test --manifest-path variants/bitpack-plan-shared/Cargo.toml       # 5 passed
$ cargo test --manifest-path variants/bitpack-shared/Cargo.toml           # 3 passed
$ cargo test --manifest-path variants/bitpack-wide-shared/Cargo.toml       # 6 passed
$ cargo test --manifest-path variants/quantiser-fadd-shared/Cargo.toml     # 1 passed
$ cargo test --manifest-path variants/quantiser-radix-shared/Cargo.toml    # 3 passed
$ cargo test --manifest-path variants/satfold-shared/Cargo.toml          # 11 passed
$ cargo test --manifest-path variants/warm-clamp-shared/Cargo.toml        # 7 passed
$ cargo test --manifest-path variants/warm-container-shared/Cargo.toml   # 15 passed
$ cargo test --manifest-path variants/wide-rung-shared/Cargo.toml        # 30 passed, 108.44s
$ cargo test --manifest-path variants/bitpack-write-contend-shared/Cargo.toml -- --test-threads=1
  # 15 passed, 5.51s
```

108 + 30 + 15 = **123, across 13 crates, all green.** I spot-checked for tautology
(`grep -rn "assert!(true)\|assert_eq!(1, 1)\|assert_eq!(true, true)"` across the thirteen crates: zero
hits) rather than reading every test body, because the assigned surface is the topic's markdown claims
and probes, not this bench-support suite, and five members of this topic have already read bodies in
five different crates and reported the same verdict (real controls, mutation tests, no smoke tests). A
raw `grep -rc '#\[test\]'` gives 124, one more than the executed count; the extra hit is the same doc
comment at `bitpack-write-contend-shared/src/stress.rs:68` that `139`, `141`, `143` and `145` all name,
and I did not need to rediscover it.

I did not audit whether a sixth crate's tests are decorative. That would be a different, larger dispatch
than the one I was given, and the topic's own repeated audits (five members, five crates, one verdict)
already cover more of this surface than I could add to in the time this check has.

---

## 1. What "work from the sources forward" actually required

Read end to end, in order: `INTENTS.md`, `RULES.md`, `OPTIONS.md` Q51 (`OPTIONS.md:2425-2461`);
`139` through `150` in full, including every probe output cited by name (not every probe source, stated
below); `151` in full, last, exactly as instructed. I formed my own reading of the false clause's
mathematics **before** opening `147`'s or `149`'s probes, because a check that reads the answer before
deriving it is a read, not a check, and this is the one place in this dispatch where independence was
cheap to buy and worth buying.

This took most of the dispatch. I am recording it plainly because the brief's checklist cannot be honestly
answered from `151` alone, or from `151` plus a skim of the four signatures: several of the items (the
false clause, the intersection defect, the contamination scoping) only resolve by reading the specific
probe source that produced the number `151` cites, and in two cases the number in the probe disagreed with
the number I expected from the surrounding prose until I traced why.

---

## 2. Entailment: does `151` claim only what `139` through `150` established

I checked this dimension by dimension on the claims the brief flagged as consequential, plus a sample of
others chosen for how far they travel (a wrong count that reaches into a "what op decides" section costs
more than a wrong count that stays local).

**The false clause's repair (`151:81-110`).** Predicate: `signedness in {unsigned, signed}`, `overflow in
{wrap, saturating}`, `rounding in {floor, ceiling, toward zero, away from zero, nearest-half-up,
nearest-half-even}`, all six, argument kind "equivariance, with the domain restriction as the quantifier,
on three independent instruments". I checked this against `147` F147-1/F147-2 and `149` F149-1/F149-2 and
against `151_probes/v1` directly (section 4 below). All three instruments establish exactly this region
and no wider: none of the three sweeps signed saturating over all six modes with the domain restriction
stated as anything other than "unrestricted" (signed values can be negative, so the restriction does not
apply there), and `151`'s predicate correctly does not claim the restriction extends to the signed side.
**Entailed.**

**The contamination scoping (`151:114-149`).** Three corrections, attributed to two signers, with one
further consequence `151` draws itself ("`149` reports against itself" that its own `p4` T1 is not a third
independent mechanism on the unsigned half). I traced each: `147`'s narrower-on-the-mechanism claim is
F147-2, `147`'s wider-on-the-reach claim is `147`'s own prose at `147:104-108`, `148`'s refutation is
`148_probes/p1`. All three are represented at the strength their own files state, not inflated. The one
place I checked hardest, because it is the kind of thing that quietly grows in a revision: `151` says
`148`'s repair is "refuted outright" for the third scoping item, and `148`'s own file does use the word
"Refuted" as its verdict marker for exactly that item (`148:10`). **Entailed.**

**The self-accounting (`151:520-556`).** Checked at length in section 6 below. **Entailed**, and correctly
labelled as a correction against `151`'s own first draft rather than smoothed over.

**`150`'s four items (`151:263-364`).** I checked the numbers in `151`'s tables against `150`'s own tables
and against `151_probes/v3` and `v4`'s outputs directly. `151`'s tables are not literal copies of `150`'s;
they are the same structure with different random draws (`151`'s v3 gives 16.6/16.6/15.2/0.0 where `150`'s
gives 12.0/12.0/11.6/0.0, and `151`'s v4 gives the identical 3.03/1.26/0.77/0.60 sequence `150` reports,
which the source shows is because `v4` fixed a seed on the same 35 committed families rather than drawing
a fresh population). `151` is explicit about which numbers are its own reruns and which are `150`'s,
stated at each table. **Entailed**, with the caveat that a reader skimming only the tables (not the prose
around them) could mistake the differing draws in v3 for a disagreement; they are not one, since the
structural claim (invariance in exactly one cell of four) is what both instruments assert and both agree.

**The count's second argument, `151` section 3.3 and `148`'s qualification (6.3 rests on 6.1).** I checked
that `151` states this as `148` stated it: "this does not weaken 6.3 and `148` does not ask for it to be
withdrawn" (`151:209`), matching `148:280-282` nearly verbatim in substance. `151` does not silently
promote `148`'s dependency claim into a stronger one (e.g. it does not say 6.3 is wrong, only that it is
coupled). **Entailed.**

**What I did not fully re-derive.** `145`'s `z4` (the equivariance-against-the-rounding-topic result) and
`z3`'s main enforceability condition, beyond what `147`, `149` and `150` already checked. Three of
`145`'s five one-expert results remain at one expert after `151`, and `151` says so (`151:496` inherits
`146`'s framing that op decides whether an unpredicated proposition may stand, and does not claim a second
reader for `z4`'s law-table placement). I read `z4`'s source and output and did not build a competing
instrument against it; my reasoning on it is in section 8.

---

## 3. Op's I3 settlement, checked against the topic's own vocabulary

Before trusting any predicate that mentions "rounding" or "toward zero" as a design-meaningful axis, I
checked whether the topic's own vocabulary for it is stable, because a design axis whose name shifts
meaning between files would make every predicate above it unreliable. `142` section 4 and `149` section 7
both flag that "truncation" is ambiguous between bit-drop (floor, on two's complement) and true
toward-zero division, citing `131` F131-3. I opened `131:315-330` and `:535-539` directly rather than
taking either file's account: F131-3 reads "Two's complement bit-drop is `floor`. It is not
`toward_zero`, and the two differ on signed domains and nowhere else." That is exactly what `142` and
`149` report it as saying, and it is the reason `139`'s original "rounding = truncate" predicate cannot be
assumed to mean the same thing as `140`'s and `141`'s "toward zero", which is the mechanism behind section
5's finding below. This is not a new finding of mine; it is a load-bearing citation I verified rather than
inherited, because the whole intersection-instrument fix in section 5 depends on it.

---

## 4. The false clause and the repair, checked at source

I derived this by hand before reading any probe.

**The claim under attack.** `146` section 5.5's first block says fusing a multiply-add on an unsigned
domain is free at all six rounding positions, citing "the one-sided-clamp congruence" as the reason. That
argument (`141` F3, `142` section 3.6, both of which I read) is a statement about **relocating a
reduction**: `R(R(x) + c) = R(x + c)` for reduction `R` modulo a power of two, an identity that holds
regardless of what rounding mode produced `x`. It says nothing about relocating the **rounding step**
itself, which is a different move: `rnd(x) + c` against `rnd(x + c)`. Those coincide exactly when `rnd` is
translation-equivariant under integer `c`.

**My own derivation, before opening any probe.** Nearest-half-even ties to the neighbour with even integer
part. Take `x = 1/2`, `c = 1`. `rne(1/2) = 0` (0 is even). `rne(1/2 + 1) = rne(3/2)`: the two neighbours
are 1 and 2, both integers away by 1/2, tie, round to the even one, which is 2. So `rne(x) + c = 0 + 1 = 1`
and `rne(x + c) = 2`. These disagree, **and both `1/2` and `3/2` are non-negative**, so this is a
counterexample entirely inside the unsigned domain. The mechanism is that nearest-half-even's tie-break
reads the *parity of the candidate integer*, and adding an odd integer `c` flips that parity, which has
nothing to do with sign. The "unsigned means no negatives, so nothing can straddle zero, so equivariance
is free" intuition that makes toward-zero and away-from-zero safe on the unsigned side does not touch this
mechanism at all, because nearest-half-even's failure never needs a sign change.

**Checked against the panel's own instrument.** `147_probes/r1` and `149_probes/y1` both measure this
exhaustively at `W = 6` and both find exactly nearest-half-even nonzero on the unsigned side, at 12.50% of
triples at `F = 1` under wrapping. I reran `151_probes/v1` myself:

```
$ python3 v1_the_false_clause_and_the_domain_restricted_repair.py > /tmp/v1.out
$ diff v1_output.txt /tmp/v1.out
$ echo $?
0
```

Byte-identical. The printed table shows exactly the five-zero, one-nonzero pattern my hand derivation
predicted, at the same percentages `147` and `149` report (12.50 / 12.50 / 9.38 / 6.25 / 3.91 across
`F = 1..5` under wrapping).

**The repair, traced through the source.** `v1`'s `equivariant()` function tests `rnd(x + c) == rnd(x) + c`
over a swept window, with a `restricted` flag that limits the sweep to `x >= 0` and `x + c >= 0`, which is
exactly the domain an unsigned fusion's rounding step actually reaches (both operands of the multiply are
non-negative, so the product is non-negative, and the accumulator addend is non-negative under this
model's `bounds(False) = (0, 2^W - 1)`). I read this function line by line rather than trusting its name:
the restriction is faithful to the scenario it is meant to model, not merely restricted-in-general. Under
the restriction, toward-zero and away-from-zero become equivariant (they coincide with floor and ceiling,
which already have the property, once negatives are excluded), and nearest-half-even does not (my hand
derivation above uses no negative number at all). The result: five equivariant of six on the restricted
domain, matching the measured table at all twelve (mode, signedness) cells, where the unrestricted test
mispredicts exactly the two modes that only become safe once the domain restriction applies.

**Verdict: correct, and it is a genuine improvement over the naive fix.** Striking `nearest-half-even` from
the unsigned list (the naive repair) would also have been correct, but it would have left the candidate
unable to say *why* toward-zero and away-from-zero survive on the unsigned side while failing on the
signed side, which the domain-restricted formulation states as one sentence rather than as a coincidence
in a table. This is the difference between a patch and a canon-shaped sentence, and I confirm `151` (and
`149` before it) reaches the second.

---

## 5. The intersection instrument, re-verified independently against the container row

I did not merely re-run `v2` (it reproduces byte for byte, `diff v2_output.txt /tmp/v2.out` empty). I
independently reconstructed the container row's predicate cross-reference by hand, from the source files,
without reading `v2`'s parser first, to check whether its conclusion (the empty dimensions are `W` and
`rounding`, not `signedness`) is right.

Opening the three predicates directly:

- `139_probes/p6`'s finding (`139:236-241`): `W in {3, 5, 6, 7, 11}`, `signedness = signed` (fixed, no
  unsigned sweep at all in this probe), `rounding = truncate` (fixed).
- `140`'s F3' (`140:855-860`): `W = 4`, `signedness = unsigned` (fixed), `rounding any of the five swept`.
- `141` F1 (`141:620-634`): `W = 4`, `signedness in {unsigned, signed}`, `rounding in {toward zero, floor}`.

`W`: `{3,5,6,7,11} ∩ {4} ∩ {4} = ∅`. Confirmed empty, by inspection, no instrument needed.

`rounding`: `139` contributes `{truncate}`. `140`'s "any of the five" is a universal on this dimension and
contributes nothing restrictive (it is the identity element under intersection, exactly as `v2`'s `merge()`
function treats `None`). `141` contributes `{toward zero, floor}`. So the binding constraint is
`{truncate} ∩ {toward zero, floor}`, and per section 3 above, nothing in this topic establishes that the
literal token "truncate" denotes the same rounding mode as "toward zero" (the topic's own F131-3 says
"truncation" is ambiguous, and could denote either floor-on-two's-complement or true toward-zero). Under a
strict reading, this is genuinely empty: not because the three instruments disagree about what a value
means, but because none of them ever states the equivalence. **Confirmed empty, by hand, matching `v2`'s
computed result exactly.**

`signedness`: `139` never varies it (its probe is signed-only). `140` never varies it (unsigned-only).
`141` varies both. The name-level check (`145`'s `z6`) reported `signedness` as "in the union only", which
reads as "one instance never varied it" and invites a reader to think the other two at least overlap on
one value. They do not overlap at all: `139`'s single value and `140`'s single value are different, so
even a value-level intersection restricted to just those two would be empty on this axis too. What rescues
the clause is that `141` alone spans both values, which is `148`'s repair (cite the single instance that
covers what the intersection cannot) and which `151` adopts correctly.

**So my independent hand-check agrees with `v2`'s printed table on all three dimensions for this row, and
disagrees with nothing.** `148`'s original guess (that `signedness` was the empty dimension) was reasonable
and wrong, exactly as `151` reports, and I verified the "wrong" by finding the real defect myself before
reading `151`'s account of it.

I also traced `v2`'s `merge()` and `parse_dims()` functions (`151_probes/v2:206-244`, `:306-321`) to check
for the failure mode this whole class of instrument is prone to: silently treating "absent from an
instance" the same as "empty after intersecting". They are handled as two different states (`"ABSENT FROM
SOME"` marked distinctly from a computed empty set), which is the correct distinction and is what makes
the container row's finding legible as "genuinely disjoint values" rather than "one instance forgot to
mention it".

**One thing I checked and did not find a problem with, though it looked suspicious at first.** `151`'s
prose (`151:120-124`) cites `v2` B4's two-sided-clamp reproduction as "6.19% to 15.50%, low clamp engaged
at 13792 triples", which does not match `147`'s own numbers ("3.36% to 23.20%, low clamp engaged at
167616"). I traced this to `v2`'s own header, which fixes `W = 5` (`151_probes/v2:49`) against `147`'s
`W = 6`. This is not a misattribution: `151`'s text says "reproduced at `v2` B4", not "matches `147`'s
numbers", and a second instrument at a different model width producing a qualitatively identical shape
(zero under wrap, rising with `F` under a two-sided saturating clamp) is exactly what an independent
instrument is supposed to look like under this panel's own discipline (`RULES.md:116-118`: "independence
means arrived at differently"). Same conclusion, different width, correctly labelled as such. Not a defect.

---

## 6. The self-accounting, verified independently

I reran `v5`:

```
$ python3 v5_a_markdown_normaliser_and_the_anchor_accounting.py > /tmp/v5.out
$ diff v5_output.txt /tmp/v5.out
$ echo $?
0
```

Byte-identical, and the E5 table reads `finding: 50 in union, 33 in 151, 46 including the accounting
section, 17 not carried`, with the stripper firing (`finding +13`), which is the check the brief names:
excluding the accounting section from both sides is exactly what the `strip` versus `full` distinction in
`v2`'s (and `v5`'s) source implements, and the thirteen-finding gap between them is the accounting
section's own list of dropped ids becoming present in the raw text the instant it names them, which `151`
reports about itself rather than hiding.

I checked this arithmetic by hand against the printed list of 17 not-carried finding ids: `F142-1`,
`F142-6`, `F144-1`, `F144-3`, `F144-7`, `F144-8`, `F144-9`, `F144-14`, `F144-17`, `F144-19`, `F4`, `F5`,
`F6`, `F7`, `F8`, `F9`, `F99` (the last one is `149`'s own deliberate nonexistent-id control, correctly
absent). 17 items. `50 - 17 = 33`. Confirmed.

**One imprecision I found, worth naming though it changes nothing.** The verdict-text line
"`line_panel: 12 carried, 88 not`" and "`probe_stem: 16 carried, 22 not`" report **different carried
counts** than the table's "in 151" column two lines above them (14 and 21 respectively). I traced this:
the table's "in 151" column is the raw count of that class inside `151`'s stripped body, while the
verdict-text line computes `carried = len(union) - len(not_carried)`, i.e. the size of the intersection
with the union. These differ whenever `151` cites something in its own text that is **not** in the
139-through-150 union at all: for `probe_stem` this is exactly `151`'s own five new probes (`v1` through
`v5`), which cannot appear in a union drawn from files that predate them, and for `line_panel` it is a
handful of citations `151` makes (self-references, or a line range whose start digit the union's own
citations never happened to hit at that exact number) that are real and correct but were never cited at
that literal line by any of `139` through `150`. **This is not an error in the instrument or in `151`'s
prose**, both numbers are computed correctly for what they measure, but a reader taking the "in 151"
column at face value as "carried from the union" for every row would be misled on two of the four rows.
I checked the same shape in `146_probes/w2`'s own accounting (`w2`'s `probe_stem: 21 carried` against a
raw count that likewise counts `146`'s own new probe) and it is the identical, benign pattern, present
since the first anchor-accounting instrument in this panel's line of descent. I am reporting this as a
documentation gap in the probe's own output labelling, not as a defect in `151`'s claims: `151`'s markdown
text never quotes the "12" or "16" figures, only the table's "in 151" / "not carried" pair, so nothing in
the shipped document is wrong.

---

## 7. Third instance search: none found, and here is exactly where I looked

The brief asks whether a third instance of the verdict-contradicts-its-own-printed-table defect exists
beyond `136` section 9's `x4` (self-caught, a different topic entirely, same author) and `145`'s `z3`
(found by `150`, this topic). I searched rather than assumed absence, because the panel's own rule
(`RULES.md:412-414`) is that a negative claim about evidence is a claim about a place, checkable in one
command, and I intend this one to be checkable.

**What I checked.** Every probe output in this topic (`139_probes` through `151_probes`) carrying the
string `VERDICT` (`find ... -iname "*out*.txt" | xargs grep -l -i VERDICT`, 26 files). I read the full
`VERDICT` section of each of the six I judged least scrutinised: `145`'s `z1`, `z2`, `z4`, `z5`, `z6`
(four of which are still, by `151`'s own admission, at one expert with no second reader for their main
result), and all five of `151`'s own `v1` through `v5` (which nobody but me has yet checked, since I am
the check). I additionally spot-checked `146_probes/w2` and `150_probes/q3`, the latter because it is the
file that caught the second instance and is therefore the most likely place for the same author's habit
to have repeated.

**What I found.** None of the eleven files I read in full carries a hand-written categorical claim ("every
arm conforms", "at every cell", "all six modes") that a printed enumeration two or three lines above it
contradicts. Every `VERDICT` block I read states a conclusion that matches the table immediately preceding
it, cell for cell, in the files I checked.

**What I did not check.** I did not read the full `VERDICT` section of the remaining fifteen files
(`139_probes/p5_scan_out.txt`, `141_probes/p3b_out.txt`, `142_probes/q3_out.txt`, `143_probes/p2_out.txt`
and `p3_out.txt`, `144_probes/p5`, `p10`, `p10b`, `p10c`, `p11`, `147_probes` (none matched the grep),
`148_probes/p1_out.txt` and `p2_out.txt`, `149_probes/y2_out.txt` and `y4_out.txt`, `150_probes/q4_out.txt`
and `q5_out.txt`). Several of these are the subject of independent second reads already (`144`'s `p10`
sequence is `144`'s own withdrawn-and-corrected headline, which is a documented instance of a *different*
self-caught defect, not this one). I did not exhaustively read every probe *source* for this defect
pattern either, only the printed outputs, which is where the defect (a hand-authored sentence disagreeing
with computed data) would actually surface to a reader.

**So the honest answer is: I found no third instance in the surface I checked, and I checked the surface I
judged most likely to hold one (files nobody had yet been the second reader on) rather than a random or
complete sample.** That is a bounded negative, not a proof of absence, and I am naming the bound rather
than letting the search read as exhaustive.

---

## 8. The rounding-topic warning, checked against the closed candidate directly

`151` section 7 repeats `149`'s warning that adopting `142`'s O-142-A ("record per-mode equivariance in the
rounding candidate") as worded would introduce an error, because the unrestricted partition mispredicts
two of twelve cells. I opened `132` and `136` at the sections `145`'s `z4` cites, rather than taking `z4`'s
account of them on faith, since this is the one place in this dispatch where a wrong reading would recommend
against touching a **different, closed** topic, which is a higher-cost mistake than a wrong reading inside
this one.

`132` section 5.3 (the exclusivity result `145`'s `z4` says its own negation-symmetry exclusion mirrors) is,
as reported, an argument of the same two-line shape: assume both properties, derive a contradiction with
"not an integer". I did not re-derive `132`'s own instance of it, but the shape matches what `z4` claims for
it, and the argument itself (reproduced independently in `145_probes/z4` and confirmed again by my own
reading of the definitions in section 4 above) is elementary enough that I trust it without a second probe.

I agree with `151`'s conclusion not to reopen `132`, on the ground it states: the property equivariance
reads is a function of a **(mode, domain)** pair, and a per-mode table in the rounding candidate has no
column for domain. That is a structural fact about what a per-mode table can express, not a judgment call,
and it does not depend on trusting any single probe's numbers.

---

## 9. What `151` gets right that I would flag if it did not

Two things `151` does that a weaker revision would not, and I want them on the record because they are the
difference between a repair and a compression.

**It states its own new defects rather than only reporting the ones signers found.** Section 9's "what I
got wrong, collected" names six things, and item 6 (`z6` intersecting names rather than values) is `151`'s
own instrument's failure, diagnosed and fixed in the same file that reports it, with the fix's own defects
(`v2`'s first false-positive run, its first control's over-correction) also kept rather than silently
repaired. That is the discipline `strict-by-design-quality-pressure.md` and the panel's own `RULES.md`
ask for, and it is rare enough across this panel's history that its absence would have been the more
notable finding.

**It refuses to widen a predicate past what its own probe measured, even where widening would look better.**
Section 5.3 states the arm's optimum "at every point of the simplex" only for the two-coordinate embedding,
and separately states the three-coordinate zero as a different, weaker fact rather than folding the two
into one sentence that would read as stronger than either alone. That is the discipline this whole check
was built to verify, and `151` holds it against itself, not only against the signatures it is revising.

---

## 10. What only op decides, checked for scope creep

`151` section 10 restates `146`'s "what only op decides" list with two additions (the second declaration
in the baseline units question, the sharpened operation-set dependency). I checked that neither addition is
dressed as a finding: both are phrased as open questions with the evidence that narrows them, not as
recommendations. Nothing in `151` proposes a canon sentence that was not already licensed by a measured
result in `139` through `150`, and nothing in it closes a question the brief or `RULES.md`'s explore-mode
reserves for op.

---

## 11. Coverage, bounded honestly

**Read end to end:** `INTENTS.md`, `RULES.md`, `OPTIONS.md:2425-2461`, `139` through `151` in full,
including every table and every predicate block. `131:315-330` and `:535-539` at source, for the
truncation-ambiguity citation both `142` and `149` depend on.

**Reran and diffed against committed output, byte for byte:** all five of `151_probes/`'s probes
(`v1` through `v5`). All five reproduced exactly.

**Traced by hand, line by line, against the source rather than the prose description:** `v1`'s
`equivariant()` function and its restriction logic; `v2`'s `parse_dims()`, `merge()`, and the container-row
predicate cross-reference (independently reconstructed from `139`, `140`, `141`'s own predicate blocks
before reading `v2`'s printed answer); `v5`'s accounting arithmetic against its own printed 17-item list.

**Verified independently, outside any probe:** the git commit-ordering claim (`git log --format='%h %cd |
%s'` on the three specific commits `148` and `151` cite; timestamps match to the second); the whole test
suite, run myself rather than inherited.

**Derived independently before reading any instrument:** the mathematical mechanism behind the false
clause (nearest-half-even's non-equivariance on the non-negative domain), by hand, from the rounding-mode
definitions, before opening `147_probes/r1`, `149_probes/y1`/`y2`, or `151_probes/v1`.

**Read at source but not independently re-derived:** `145_probes/z3_output.txt` in full (the "[2, 4]"
table `150` catches); `z4`'s output and the shape of its argument, without building a competing instrument;
`146_probes/w2`'s accounting output, spot-checked for the same benign carried-vs-in-file discrepancy found
in `v5`.

**Not read:** the probe sources of `139` through `144` and of `147` through `150`, except where an output
was quoted and I opened the specific lines to check the quotation; `40`, `93`, `102`, `106`, `107`, `108`,
which this whole topic relies on `OPTIONS.md` Q51 to summarise and which I did not open, matching every
member of this topic's own stated coverage.

**The third-instance search is bounded, not exhaustive**, per section 7, and I named exactly which files I
read in full for it and which I did not.

**I built no new probes.** Every verification in this file is either a rerun of a committed probe with a
diff against its committed output, a hand derivation checked against a committed probe's result, or a
direct read of committed source and committed git history. Nothing here rests on anything uncommitted, and
nothing here needed a new instrument to answer the question asked. Where I would have wanted one and did
not build it: an instrument sweeping the truncation-ambiguity question itself (whether "truncate" in
`139`'s vocabulary was ever meant to denote "toward zero" specifically, as opposed to genuinely being
underspecified) is not something a probe over arvo's arithmetic can answer, since it is a question about
what an author meant rather than about what a computation does, and I have treated it as unresolved rather
than guessed.

**Where I would want a second pair of eyes.** The benign carried-vs-in-file discrepancy in section 6. I am
confident it is not a defect in `151`'s claims, because `151`'s own prose never quotes the smaller number,
but the probe's own printed labelling ("X carried") is genuinely ambiguous between two things ("carried
from the union" and "carried in my own stripped text") and a future reader of `v5`'s raw output, rather
than of `151`'s prose, could draw the wrong conclusion from it. That is worth a one-line fix to the probe's
print statement whenever someone next touches it; it is not worth a correction to `151`.

I priced nothing and no claim above is a bench result. Everything in this topic is `threads = 1` and at
model widths, and nothing I checked changes either bound.
