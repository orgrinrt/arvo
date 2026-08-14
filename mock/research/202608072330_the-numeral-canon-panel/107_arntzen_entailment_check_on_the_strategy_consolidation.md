# 107. Entailment check on the strategy-axis consolidation

**Position:** the independent check on `106`, the strategy-axis unit's canon candidate. **Author:** the
`arntzen` persona. **Probes:** `107_probes/`, eleven of them, each committed as it ran.

**Standing:** nothing here settles anything. Op decides. Where I say `106` is wrong I mean it and I have
put a `file:line` and a reproduction under it; where I say it is right I mean that too, and saying so is
a result rather than a courtesy.

**Who I am checking and why it is not `106`.** A compression is checked by someone other than whoever
compressed it, because the belief that it entails is what produced it and cannot also test it. I worked
from the eight members forward, not from `106` backward, and read all eight in full plus op's `95`, `104`
and `105` and the checkpoint `99` before opening `106`'s body. The two instruments the rule names are in
section 2, and the second of them found a class the first is structurally blind to.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` I1 through I18 read in full, and against op's `95`, `104`, `105` and `87`.

Running this check is licensed by `87`, which is op choosing that nothing moves into `mock/canon/` until
every topic is done and that a consolidation is "input, not canon in miniature", and by the standing
compression rule. I found nothing in `106`, and nothing in the work it compresses, that a stated intent
forecloses. The one place `106` itself flagged as a possible refusal, I16's "we shouldn't police what kind
of laws there are or what shapes they take", I read the same way `98` and `100` did and for the same
reason: defining the object the unit was convened to define is not policing, and section 6's four arms are
the form I13 asks for rather than one required shape. Nothing to refuse on and nothing ambiguous to hand
back.

**One thing I record rather than resolve, because it is the dispatcher's.** My brief states that `106`
reports an anchor union of 207 "with 34 carried, classified", and asks me to run the set difference. I did.
But `106` is a canon candidate, and per the tier rule the anchors that must survive are the panel-internal
and probe ones, not the ones into a nuked tier. `106` applies that rule correctly and its four
superseded-tier drops are right and must not be restored. Where I part from the brief's framing is
narrower and is in section 2.2.

### 0.2 Test gate: passed, at 123 across 13, and my own instrument produced a fifth meaningless green

`mock/crates/` is empty by design, so `cargo test --manifest-path mock/Cargo.toml` errors on a virtual
manifest with no members. That is the intended state. The only executable surface this unit touches is the
bench variant crates, and I ran them per crate rather than taking five files' word for it.

**123 tests across 13 crates, all passing.** That is the sixth independent count, after `98`, `100`, `102`,
`103` and `106`, and it reproduces every per-crate figure exactly. Run and output at
`107_probes/p0_test_gate.out`.

I read the bodies in the surface I touch, which is `bitpack-shared`'s three tests and its validator, since
section 3.4 turns on what they assert. Nothing there is tautological, sampled where a matrix was
available, or assertion-free. There is nothing to refuse on, and `103`'s two small observations on
`quantiser-radix-shared` are the only body-level defects in the corpus that anyone has found.

**And the gate caught me the same way `106`'s caught `106`, which is worth a paragraph rather than a
footnote.** `106` section 0.2 enumerates four ways this corpus produces a meaningless green. My first
runner wrote `timeout 600 cargo test ... | grep '^test result'`. **`timeout` is not installed on this
host**, so the pipeline produced no output and exited 0, for all thirteen crates, and printed a clean
table of `NO RESULT` that reads as a completed gate. That is a fifth. A sixth is milder and cost me a
second run: `wide-rung-shared` alone takes 107 seconds, so a batch runner under a two-minute cap reports
twelve of thirteen and looks complete. Both are recorded in the probe. **Six ways, all exiting zero**, in
one corpus, is now the count, and it is the reason `106`'s insistence on running the gate rather than
citing it is correct.

---

## 1. Coverage, bounded rather than claimed

Stated first because this workspace has repeatedly caught unmeasured completeness claims, and a check that
overstates its own reach is worth less than one that does not.

**Read in full:** all eight member files `93`, `94`, `97`, `98`, `100`, `101`, `102`, `103`, both phases of
the two cold derivations included; op's `95`, `104` and `105`; the checkpoint `99`; `INTENTS.md` including
I18; `87` sections 1 and 2; and `106` end to end.

**Read in part:** `22` at the passage `106` recovers, `97`'s and `93`'s probe outputs where a claim of mine
rests on one, `102`'s p1 script, `106`'s p4, p5, p6 and p7. **Not read:** `RULES.md`, `OPTIONS.md`,
`DROPLIST.md`, `25`, `35`, `40`, every panel file before `93` except `22`'s one passage, and the archive.
So where a finding of mine restates something in those, I do not know it. This matters most for section
3.5, which is about a reading of `25` and `40` I have only through `97`, `98`, `101` and `102`.

**Verified by running rather than by reading:** the suite, 123 across 13, per crate. The four
meaningless-green mechanisms `106` reports, all four reproducing. `bitpack-shared`'s test body, extracted
from source. The `-dirty` precondition and the 175/79 pre-wiring join, rebuilt from the meta files and
`git log` with my own extractor. Eight of `106`'s corpus counts. `106`'s own p4, p6 and p7 re-run. Two
committed probe outputs of `93`'s and `97`'s at the cells my severe finding turns on. The satfold medians
at their line in the committed findings file.

**Citations opened rather than resolved:** every quotation I attribute to a member or to op in this file
was opened at its source. Two of my own were wrong on the first pass and are corrected against the file:
a grep of mine reported `106_probes/p2` missing when `106` cites its own probe by stem and the file is
`p2_corpus_counts.sh`, and a grep of mine reported `106` containing a "defect" word about its own drops
when the single hit was a predicate listing generator defect classes. Both false positives are mine, both
are recorded in the probes that produced them, and both are why section 4's severe findings were each
re-checked at source before being written.

**Not verified, and named:** every timing figure, every bootstrap, every exhaustive law sweep beyond the
four cells I opened, the rationalisability counts, and the compile-and-emit claims. I ran no bench and
took no measurement. Where nothing has been priced I have written unpriced.

**What no probe of mine could check**, and it is the reason a second reader exists at all: whether a cited
passage supports the argument put on it. That is the whole of sections 3 and 4 and it is judgement, not
measurement.

---

## 2. The two instruments

### 2.1 The entailment pass: from the members forward

For each member I listed its substantive claims from its own findings sections and its own
what-I-would-keep sections, then asked what `106` does with each: carried at the same strength, carried
weakened, reattributed, or absent. Working forward matters, and the difference is not stylistic: a pass
driven by `106`'s structure can only find what that structure already accounts for, and every one of the
four omissions in section 4 is invisible from that direction because `106` has no heading where they would
have gone.

### 2.2 The anchor set difference, and a correction to the brief's framing

`106`'s own census is `106_probes/p6_anchor_census.sh` and it **reproduces byte for byte**: union 207,
probe files 125, live bench tree 15, workspace rules 13, superseded tier 4, carried 34, superseded-tier
carried 0. The numbers are honest for what they measure.

**But that script computes no set difference.** It has no `comm`, no `diff`, no `join`: it prints two
counts and one grep for `.tmpl`. `106` section 15 says "The census **and the set difference** are
`106_probes/p6_anchor_census.sh`" (`106:1228`) and then, fifty lines later, correctly assigns the set
difference to the check that follows (`106:1263`). The section contradicts itself about its own instrument,
and the script's own comment shows the author knew: "the set difference the check after it should run".
Low severity, one word, and worth fixing because a later reader will otherwise believe the diff was run.

So I ran it, with `106`'s own pattern and `106`'s own exclusion of its accounting section, at
`107_probes/p3_the_set_difference_106_did_not_run.out`.

```
union=207  carried=34  dropped=177
```

**Five anchors are dropped from the body and named in section 15**, so a diff run without the exclusion
would have returned five fewer and looked better for the wrong reason. The exclusion is mandatory and
`106` applied it; I am recording that the instrument would have been disabled without it.

**By class, and the classes are not alike.** Four superseded-tier `.tmpl` anchors: **correctly dropped, and
they must not be restored.** `106`'s reasoning is right and it is the tier rule applied properly, and its
note that the instrument caught it carrying one on the first run is the discipline working. Ten workspace
rules, sixteen live bench-tree paths, and **113 probe anchors**.

**Three members lost their entire probe trail.** By directory:

```
93_probes  15 in the union   0 carried
94_probes  10               0
103_probes 11               0
98_probes  19               1
100_probes 17               3
101_probes 15               2
102_probes 14               2
97_probes  17               4
```

`93` and `94` are the two blind cold derivations, and everything the unit's object rests on at the TWO
EXPERTS rung was established in those two probe directories. A reader of `106` alone cannot reach any of
it.

**And the second measurement, which is the one that matters more.** The eight members carry **157
occurrences of a `file:line` anchor, 116 unique.** `106` carries **two, both the same one**
(`107_probes/p1_anchor_set_difference.out`). Against that, `106` cites member files by bare number 347
times, of which **45, or 12%, carry a locator** (a section, a finding id, or a probe path). The other 302
point at whole files averaging about 1100 lines
(`107_probes/p2b_locatable_including_finding_ids.out`).

`106` has an argument for this and it is a real one: `how-to-run-a-panel.md` prefers a heading anchor over
a line number for anything still growing, and this unit paid for line citations twice, `101` losing
fourteen of thirty-seven on its first run with eight of them because `100` grew underneath it. That
argument is correct and I accept it for the conversion it licenses. **What it licenses is line-to-heading,
not line-to-file**, and 88% of the conversions went to file. A citation to `102` is a citation to 1057
lines.

**Judgement.** The superseded-tier drops are right. The line-to-heading policy is right. The execution
converted most citations to whole-file references, and three members' probe trails went to zero, which
means the claims in `106` that rest on `93`'s, `94`'s and `103`'s probes are present as prose with no route
back to the evidence. Moderate severity, cheap to repair, and section 5 says how.

---

## 3. What `106` got right, checked rather than assumed

Keeping something is a result and this section is longer than it would be if the file were bad. Every item
here was tested before it was credited.

**The corpus counts it says it took itself, all eight, reproduce exactly**
(`107_probes/p10_the_numbers_106_says_it_verified.out`): 94 variant crates, 0 implementing `score_output`,
0 `score_dimensions`, 0 `max_relative_error`, 15 defining `validate_output`, 1 mentioning
`outputs_may_differ`, 254 CSVs, 254 meta files. I measured each independently with `--exclude-dir=target`.

**Both of its own instrument findings reproduce exactly** (`107_probes/p4_the_four_meaningless_greens.out`).
`tail -4` reads the doc-test block, reporting `0 passed` and exiting 0. And
`grep -rl outputs_may_differ variants/` returns **133 after the suite has run and 1 before**, to the file.
The consequence `106` draws is right and is worse than it says: on the same contaminated tree,
`score_output` returns 84 against a clean 0, and `validate_output` returns 428 against a clean 16. **Any
count in this panel taken by grep over `variants/` without excluding `target/` is suspect**, and the two
that would be most damaged are the ones the corpus argument rests on.

**The pre-wiring join reproduces exactly, from an independently written extractor**
(`107_probes/p5_reproduce_the_prewiring_join.out`). 253 of 254 `git_commit` values carry `-dirty`; a naive
join resolves 1 and leaves 253 unresolvable, so the finding does evaporate without the precondition.
`harness::validate` is present in `mock/benches/src/main.rs` at `9db33f8c` and absent at its parent.
**175 before, 79 after, 0 unresolvable.** And the reconciliation `106` offers of `103`'s 24: there are 24
distinct `git_commit` strings and 23 distinct commits, because `defc747` appears once clean and twice
dirty. Exact, including the commit named. That is a third instance of F-103-6 and `103` asked for a second.

**The recovered anchor is real and it is the best single act in the file.** `22:188-193` establishes that
the harness writes its artifacts into the tree it then hashes, so every size row after the first is dirty
by construction and the suffix carries no information about the source. **No file in this unit cites `22`**,
which I checked by grep across all eight. Without that fact the join returns 253 unresolvable and a later
reader concludes the corpus has no resolvable provenance. Recovering an uncited precondition from 81 files
back is exactly what a consolidation is for.

**The `bitpack-shared` refutation is correct at source** (`107_probes/p4`, and the body at
`106_probes/p5`). `check_size` asserts both `extract_aligned` and `extract_zeropad` against
`col.logical[i]` at every index, every size, eight seeds, plus a bijection check. `102`'s "no cross-arm
agreement assertion of either kind, mutual or oracle-backed" is false about the crate.

I add the mechanism, which `106` does not name and which is more useful than the correction.
`102_probes/p1_the_corpus_compares_cost_at_a_fixed_answer.py:87` filters a `#[test]` fn on
`agree|match|same|identical|disagree` matching its name or body, and takes the body as the text up to the
next `#[test]`. `bitpack-shared`'s tests are named `column256_roundtrips` and their bodies are a single
call; the assertions live in a helper **defined above** the first `#[test]`, so they fall outside every
body extent. **`102`'s claim is false about the crate and true about its instrument's output**, and the
instrument is a keyword filter over a text window. That is worth stating because the same instrument
produced `102`'s 48-mutual-and-18-oracle table, which nothing has re-derived.

**Polarity at ONE EXPERT is a correct and careful rung call.** `102` section 2.5 says of itself, in its own
words, "A second reader who derives polarity without reading `97` would earn the rung. I did not."
(`102:261`). The checkpoint `99` does present it as converged. And `106` states the correction precisely,
as bookkeeping rather than doubt, keeping the distinction in full. Nothing to add.

**I18 against `93`'s F9 is right.** F9's complementary sentence reads "for operands that are not
const-available there is nothing, by I15, not a weaker check and **not a debug-only one**"
(`93:613-614`). I18 licenses exactly a debug-only check. The predicate is untouched and the complementary
sentence is overturned, which is what `106` says.

**The I3 residue analysis is right and is a genuine finding.** `101` section 6's two-with-nothing are I7
and I3/I4; `102`'s two-not-weighting-shaped are I5 and I3/I4. They overlap on one and differ on the other,
so the located disagreement was partly a disagreement about which pair was being counted. `102`'s own table
concedes I5's bar "wants a sound-against-unsound bench that does not exist", verbatim, which is `101`'s
point. The genuine residue is I7. Verified in both files.

**`106`'s own p4 citation checker re-runs at 49 of 49**, and its p6 and p7 reproduce. Its p4 covers the
`bitpack-shared` fact and does not cover the attribution claim in section 4.5, which is the limit `102`
already named: no probe checks whether a passage supports the argument put on it.

**And the satfold evidence is exact.** `satfold-const-gate_n10000_findings.md:93-95` gives gate-false at
38391 ns [38374, 38405], gate-true at 1438 ns [1435, 1460] and lanes16 at 1456 ns [1454, 1460], as `93` and
`106` both cite.

---

## 4. Findings, by severity

### 4.1 SEVERE. The law bullet is unpredicated, and as written it is false at signed saturating

`106` section 3.1 states, as a three-or-more-instances finding:

> **Multiplicative associativity and distributivity hold at `F = 0` and fail at `F > 0`.**

It carries **no `holds for:` line**. Of the five bold leads in section 3.1, two carry a predicate; of the
three that do not, one is a sub-note on the lead above it and one is a test count. **This is the only claim
about arvo's arithmetic in the section, and it is the one without a predicate**
(`107_probes/p9_the_defence_quotes_half_its_own_source.out`).

**The finding it compresses carries the missing dimension explicitly.** `93`'s F1 reads
`holds for: W in 3..8, F in 0..2, signedness = unsigned, overflow in {wrap, saturate}, ...`. Dropping
`signedness = unsigned` widens the claim, and under I13, which `106` quotes at the head of the same
section, a predicate is never widened in place: the widening is a new claim needing new evidence. There is
none. Under I13's own absence rule the unpredicated sentence claims something narrower still and equally
wrong, that it holds nowhere signedness is present.

**And the widened claim is measurably false, on two independently written models, at the cell the
predicate was excluding** (`107_probes/p6_the_law_bullet_is_false_at_signed_saturating.out`):

```
93_probes/p7_signedness_breaks_the_congruence.out, signed W = 7, F = 0, saturate
    (a+b)+c == a+(b+c)     FAILS  520128/2097152 = 24.80%
    (a*b)*c == a*(b*c)     FAILS   15036/2097152 =  0.72%
    a*(b+c) == a*b+a*c     FAILS 1000674/2097152 = 47.72%

97_probes/p2_congruence_predicts_the_laws.out, signed saturate F=0 truncate
    add_assoc      fails  (23.24% of 4096)
    mul_assoc      fails  ( 3.91% of 4096)
    distrib        fails  (34.52% of 4096)
```

**`106` cites as support the very work that refutes it.** It names `97`'s criterion and its 552 cells as
one of the instances behind the sentence. `97` section 6.3 is titled "A hazard in a live workspace rule,
which is what the criterion found first", and inside the region the sentence *is* right about, unsigned
`F = 0` saturating, the criterion measures distributivity over **subtraction** failing at 34.94% of 4096
in the committed output and 45.79% at `W = 6` as `97` reports it. `97`'s conclusion is that "a law
permission has to name the **operations** it covers and not just the fraction width". `106` names neither
the operations nor the signedness.

**This is the sentence the workspace rule was corrected away from during this unit.** `99` records it:
"`arvo-always-optimal-internals.md` told every agent that distributivity holds exactly at `F == 0`. Two
members refuted the 'exactly' independently from different models... That was a live licence to emit a
wrong rewrite." The rule now reads "**`F == 0` is necessary and it is not sufficient, and this rule
previously said it was**"
(`.claude/rules/arvo-always-optimal-internals.md:65`). `106` restores the corrected sentence into the
document the canon is written from.

**The word `signed` does not appear in `106`'s own prose anywhere.** Its four occurrences are two inside
quoted predicates, one in "both signednesses" inside a quoted result, and one in the word "assigned".

**Why this is the most consequential thing in the file.** Every other defect here costs a later reader time.
This one is a licence to fuse `a*b + a*c` into `a*(b+c)` on a signed saturating type, where it is wrong on
34.52% of triples, and to factor `a*b - a*c` on an unsigned saturating one, where it is wrong on 34.94%.
The canon is written from the consolidations. This sentence would go in.

**Repair.** One line, and the material exists: restore `93`'s F1 predicate verbatim, add `93`'s F10 and
`97`'s F-G as the two cells that bound it, and state `97`'s criterion rather than its score (see 4.4). The
finding survives whole once it carries its region; it is `93`'s F1, not a weaker thing.

### 4.2 SEVERE. Four one-file results are missing from the droplist, and the droplist is where they were supposed to be caught

`106` section 13.2 exists precisely because "the options most likely to be lost are the ones the panel most
needs carried", and it lists thirteen. My brief asked me to test the list for completeness rather than its
entries. Four substantive one-file results are absent from `106` entirely, each verified by hand and not by
grep alone (`107_probes/p8_what_the_droplist_missed.out`).

**One. `97`'s F-H: a declared non-negative operand window recovers three laws that two-sided signed
saturation loses.** `holds for: W in {4,5,6}, F = 0, signedness = signed, overflow = saturate, operand
window = declared non-negative, operations {add, mul}, arity 3, values exhaustive`. This is the only
**positive** law result in the unit: a nameable, const-checkable predicate under which a lossy policy
regains additive associativity, multiplicative associativity and distributivity over addition. It was
predicted by the criterion before running, and `97` records that it independently retrodicts `82`'s
declared-window result. The word `window` appears **zero** times in `106`. I13 is about collecting exactly
this shape, and this is the unit's cleanest instance of it.

**Two. `97`'s F-B: the rationalisability gap is polynomial against exponential in the region count**, over
`regions in 2..20`, by a hyperplane-arrangement bound plus exact enumeration over 2000 random tables with
zero violations. This is what makes 72-of-15625 a fact about what a weighting is rather than about one
table, and `98` says so in those words, calling it "a better argument than mine and reaches further".
Without it, `98`'s own F-98-5 bounds the counts hard: the ratio varies by a factor of **47** across tables
of one shape, "so a particular value of it is a fact about one table". `106` carries the counts as its
lead three-instance finding and carries **neither** the structural bound that generalises them nor the
measurement that limits them. `polynomial`, `hyperplane`, `47x` and `fact about one table` all return zero.

**Three. The exchange-rate-against-priority reading of op's four intents, which is three-instance and is
about op's own words.** `40` section 5.3 reached it from op's four intent statements; `98` section 4.1
derived it independently from the four intents' wording and measured it (a priority is realisable as a
weighting, 1200 of 1200; the converse fails, 4 sections against 58); `102` reached it a third time from the
intents' text and lists it under what it keeps. The content: **every one of op's four intents names a
primary concern and then explicitly refuses to make it absolute**, which is exactly the difference between
a lexicographic priority and a finite exchange rate, and it is measurable. It carries a consequence `98`
said out loud so it would not be discovered later, F-98-7: **a hard bound on a measurement is not
expressible as a weighting at all**, so a canon saying "a strategy is a weighting" has thereby said no
strategy is a hard bound. `102` then resolves that under the pair, since a hard bound is a policy
assignment rather than a weight.

In `106`: `lexicographic` 0, `priority` 0, `threshold` 0, `hard bound` 0. Its section 1 definition says "a
**weighting** over cost coordinates" and nothing about what kind of weighting or what it excludes. **This is
the canon candidate's central noun, the members established its shape against op's own four statements
three times over, and none of it survives.**

**Four. `98`'s five-rung ladder, and the Pareto-admissible rung inside it.** L0 46656, L1 144, L3 72, L4 9,
L5 9, with L1 = L2 exactly on structured data and L4 = L5 exactly, plus the union-graph acyclicity
criterion that predicts the coincidence and was exact in 120 of 120 models. `98` says the ladder is "the
part I most want carried", on the ground that op's "mostly option 1, a little bit of option 3" has no
reading on a binary and an obvious one on a ladder: **a rung strictly between the ends**. `106` carries 72
and 9 and drops 144, the ladder, and the reading. `ladder` 0, `admissible` 0, `144` 0. `106` uses the word
`rung` eighteen times, exclusively in the provenance sense, which is a collision worth avoiding in a
document that dropped the other meaning.

**Repair.** Four entries in 13.2, each two or three sentences, and F-H arguably belongs in the body rather
than the droplist because it is an arm with a predicate rather than an option.

### 4.3 MODERATE. The defence for the drops quotes half of its own source

`106` section 15 defends the 177 dropped anchors:

> They are not lost: every member file is intact beside this one, and `87` fixes that the canon is written
> from the consolidations **read alongside the members they compress**. A dropped probe anchor here is one
> hop away rather than gone.

The sentence in `87` it rests on, whole, at `87:26-29`:

> A finding that a consolidation dropped is recoverable at the end from the file it came from, **which is
> why the droplist and the compression checks matter and why a dropped item is a defect rather than a
> closed question.**

`106` carries the recoverability clause and drops the clause of the same sentence that says a dropped item
is a defect. No occurrence of "defect" in `106` refers to its own drops; I listed all twelve to be sure,
after a naive grep of mine gave a false positive
(`107_probes/p9_the_defence_quotes_half_its_own_source.out`).

Both halves are the coordinator's reading rather than op's verbatim, since `87` marks his words with
blockquotes and this bullet sits outside them. That cuts both ways and it does not license taking one half:
a file may not cite the half of a sentence that absolves it and leave the half that convicts it. This is
the compression failure occurring inside a compression's own defence of itself, which is the smallest
possible scale and the most instructive.

**Repair.** One sentence, restating both halves, and the four entries from 4.2 are what makes it true.

### 4.4 MODERATE. `97`'s criterion is cited by its score and never stated

`106` section 3.1 says "`97`'s criterion predicts every verdict in 552 cells with zero mismatches in either
direction" and, in the same sentence, that it agrees with 659 of `35`'s 660 committed rows. **It never says
what the criterion is.** `realisation map` 0, `identity of exact` 0, `ordered nesting` 0, `congruence` 0,
`quotient` 0.

The criterion, at `97:708-709`:

> a law holds in the representable set **iff** it is an identity of exact arithmetic **and** `pi` respects
> every ordered nesting of operations the law contains.

`97` calls it "the one I would most like carried" and the reason is stated: it makes an **infinite** family
of laws at every arity decidable from a **finite** table nobody has to grow, and it answers, in general
form, the route `OPTIONS.md` records as never asked of any law this panel measured. A reader of `106` alone
learns that some criterion scored well and cannot use it for anything.

This is the classic shape the compression rule names: the number survives and the content does not, because
the number is what reads as evidence. It is also what made 4.1 possible: `106` kept the criterion's score
as support for a sentence the criterion refutes, which it could not have done had it stated the criterion.

**Repair.** Two sentences, quoting the criterion and its one extension for order laws (monotone quotients
inherit order facts), which is what let `97` reach `35`'s battery.

### 4.5 LOW. The `bitpack-shared` doc comment is attributed to the module and is the test helper's

`106` section 0.3, having quoted the body correctly, says "**the module's own doc comment says so in its
first line**: 'Cross-checks both extraction paths against the logical ground truth, every index, every
size, 8 seeds each.'"

The quotation is verbatim and exists. It is **`check_size`'s own `///` doc comment at
`mock/benches/variants/bitpack-shared/src/lib.rs:264`**, inside `mod tests`. The module's `//!` doc comment
begins "Shared data model for the `Layout::Bitpacked` access-pattern bench." A reader who greps the module
header, which is what "module doc comment, first line" instructs, does not find it, and the module header
is 40 lines of packing rationale that says nothing about cross-checking.

The correction survives whole; only the address is wrong. `106`'s own p4 checks the fact and not the
attribution, which is the limit no probe crosses.

### 4.6 LOW. The heading over 4.5's finding is stronger than its own body

`106` section 0.3's heading reads "`bitpack-shared` is characterised **wrongly** in four of the eight
member files". Its body is careful and distinguishes them: `102`'s claim is false; `103` inherits the false
half; `94` "names the bijection and does not name the ground-truth cross-check", which is incomplete rather
than wrong; `97` is "right about the extraction property and says nothing about the bijection check", which
is over-broad rather than wrong. Two are wrong and two are incomplete. The body is accurate and the heading
is not, and section 12 item Four repeats the heading's framing.

### 4.7 The consequence `106`'s own amendment introduces and does not trace

Not a defect, and I nearly filed it as one, which is why it is stated with both readings.

`106` section 9.2 amends the definition of observable, correctly in my view, so that it covers "whether a
value is produced at all", and calls the fix "one clause rather than a mechanism". `98` section 3.2, citing
`40` section 5.3, states the build-arm rule: an arm may resolve an **unobservable** coordinate however it
likes and a later arm may resolve it differently, and "for an **observable** one it is forbidden by the
same section" (`98:479-484`).

Under the amended definition, I18's debug-only panic is an observable difference between two builds of
identical source, which that rule forbids. Two readings are available:

- The amendment does create the conflict, and `106` does not name it.
- Or the build profile is a consumer input rather than a resolver output, so the panic axis is
  consumer-supplied and polarity is satisfied. That is probably right.

Which is right turns on whether a build condition is an input to the resolver or an output of it, and
**`102` already named that as unresolved and unnoticed**: "A build condition is a `cfg`. A region is a
property of the consumer's workload. These are disjoint sets of things and nothing has noticed"
(`102:155-156`). So the one-clause amendment lands exactly on a seam the unit identified and left open, and
saying so is a sentence rather than a mechanism. It belongs in section 14's list of what the unit did not
establish.

### 4.8 A note on `106`'s own ratio, which is offered as the test of whether the file is any good

`106`'s opening says the accounting is "explicit and countable rather than asserted" and `p7` counts it
from the file. The counter reproduces exactly, and it counts **bolded lead-ins**. A file that stated 31
things and dropped 15 one-file results would score identically to one that dropped none, because the
instrument measures headings the author wrote and cannot see an absence. The numbers are honest; what they
are offered as evidence for is not something they can bear. This is not a defect in the count and it is a
reason not to treat it as the file's own check.

---

## 5. What each repair costs

Ordered by severity, and every one of them is short because the material exists in the members.

1. **The law bullet.** Restore `93`'s F1 predicate verbatim; add the two bounding cells, `93`'s F10 and
   `97`'s F-G; state the criterion per 4.4. Roughly a paragraph. **This one should not wait**, because the
   sentence as it stands is a licence to emit a wrong rewrite and it is in the document the canon is
   written from.
2. **The four droplist entries.** Two or three sentences each in 13.2, except `97`'s F-H which wants to be
   in the body as an arm with a predicate.
3. **The `87` half-quotation.** One sentence.
4. **The criterion.** Two sentences in section 3.1.
5. **The anchors.** Not a rewrite. Restoring a section or a finding id to the highest-traffic citations
   would recover most of it: `93`, `94` and `103` at zero probe anchors is the part that matters, and the
   claims resting on them are identifiable because they are the ones in sections 3.2, 5 and 11.
6. **Two words:** "module's own doc comment" to "the test helper's doc comment", and the heading in 0.3.

**And one thing not to repair.** The four superseded-tier `.tmpl` anchors stay dropped. `106` is right
about them and a later reader working from the compression rule alone might restore them.

---

## 6. Verdict

**The compression is sound on its central content and it has one false sentence in it.**

Everything I could test about the object `106` states, the pair with its two carriers, the
measured-versus-computed split with `103`'s predicate, the region-against-cost-vector line, the three
answers on three regions, the four arms, the rung classification, and the corpus correction, holds against
the members and against source. Its rung calls are more careful than the checkpoint's, its polarity
correction is right and is the kind of bookkeeping that only gets done by someone looking for it, its
recovery of `22`'s uncited precondition is the single best act in the file, and eight of its independently
claimed counts reproduce exactly.

Against that: one arithmetic claim is stated without the predicate its source carries and is false in the
region the predicate excluded, while citing the work that refutes it; four one-file results are gone
including the only positive law result in the unit and the whole three-instance reading of op's four
intents; the criterion is present as a score and absent as content; and the defence for the drops quotes
half of its own source.

The severe two are the same failure in two places, and it is the one the compression rule names: **prose
survives and the thing that qualifies it does not**, because the qualification is what a compressor has no
room for and what a later reader has no way to miss the absence of.

None of it is unrecoverable, `87` says so, and `87` also says a dropped item is a defect rather than a
closed question. Both halves.

---

## 7. What I did not do

**I did not attack the pair**, per the brief. Section 4 of `106` separates its three claims so they can
fail independently and that separation is good; a dispatch that has not read it is what it needs, and
`106` says so itself.

**I did not re-derive any measurement.** Every number in section 3 is a reproduction of somebody else's
instrument or a count over committed artifacts, and where I opened a probe output I opened it to check a
sentence rather than to re-run the sweep.

**I did not read `RULES.md`, `OPTIONS.md`, `DROPLIST.md`, `25`, `35` or `40`.** So my section 4.2 claim
that four items are missing is a claim about the members and `106`, not about the register: if the option
register already carries `97`'s F-H or `98`'s ladder, they are less lost than I have said, and `106`'s own
13.2 opening says the register lost a live option in each of the two prior consolidations. Somebody should
check that, and it is one grep.

**I did not check the entailment of `106` section 10**, its Q50 contribution, against anything, because it
is marked as a contribution rather than a compression and there is nothing to entail it against. It needs
the second read every one-expert claim needs and this is not it.

**And I did not verify `106`'s claim that the unit "converged more than its own located-disagreement
sections suggest"** beyond the three collapses it names, two of which I checked and both of which hold.
Whether that is the honest summary of eight files is a judgement about the whole unit and I only tested its
parts.

---

## 8. Coverage of the citations

Every `file:line` in this file was opened and its **content** tested rather than merely resolved, by
`107_probes/p11_check_my_own_citations.py`. Whitespace is normalised and blockquote and doc-comment
markers stripped on both sides, because a quotation wrapped across lines or carried inside a `>` block is
still verbatim, and neither normalisation can make an absent phrase appear.

```
citations checked: 23   ok: 23   failed: 0
```

**It passed on the first run, which is the weaker of the two things to say about it.** `103` records the
lesson: a citation checker that has never failed has not been tested either. So it is mutation-tested three
ways, and each mutant is caught: a phrase op did not say, a real phrase at the wrong span, and a real
phrase in the wrong file. The output carries the mutation run beside the pass.

**Two false positives of my own, both mine and both recorded in the probes that produced them**, because
the brief is explicit that a severe finding is verified before it is reported and one of nine was half
wrong on a prior check of this panel. A grep of mine reported `106_probes/p2` missing when `106` cites its
own probe by stem. A grep of mine reported `106` containing a "defect" word about its own drops when the
single hit was a predicate listing generator defect classes; I listed all twelve occurrences by hand
before writing 4.3. Both corrections are in `p10` and `p9` rather than deleted.

**What the instrument does not check**, and it is the whole of sections 3 and 4: whether a cited passage
supports the argument I put on it. No probe crosses that, which is why this file exists and why it should
not be the last word on `106` either.
