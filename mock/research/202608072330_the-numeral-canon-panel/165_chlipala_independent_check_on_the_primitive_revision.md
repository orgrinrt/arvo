# 165. Independent check on the revised candidate

**Interrupted and resumed.** This dispatch was cut by a session limit partway through, after the
reading phase (109 through 114, 154 through 164, RULES.md, INTENTS.md, all read and logged below)
and partway through independent verification of `162`'s and `163`'s probes. Everything read and
every command run before the cut is either committed to this file's git history (the two `wip:`
commits) or reproduced here by rerunning it after resumption; nothing below rests on an unrecorded
memory of a prior run. The one item genuinely in flight at the cut, F111-5's exception count, is
re-verified in section 3.6 from a fresh rerun rather than from anything held in working memory. I
found nothing that was in my head at the cut and not recoverable from disk: every claim below was
either already committed as a probe output before the cut, or is reproduced fresh in this session.

**Verdict, stated first.** The revision is sound where it repairs the two signatures' findings, and
I verified the load-bearing repairs myself rather than trusting the prose: the lens degeneracy
condition, the two-branch completeness certificate, the `cfg` soundness hole, the container-premise
sweep, and all named "reproduces" claims across `162`, `163` and `164` reproduce bit-for-bit when I
rebuilt them independently. `164`'s own three claimed catches (the line-stale control error, the
shipped-oracle narrowing, the `110` section-6 lineage for reading 3) are each real and each checks
out against source. The container premise, Q65's marker question, and X1 through X4 are genuinely
left open throughout; X3's move from a contested pointer to op's coupled queue item is a relocation,
not a resolution, and the revision says so in its own words ("none is chosen here... decides
nothing").

**One severe finding, verified before being reported as severe.** `157`'s F157-5 and the register
do-not-cite entry built on it (`164`'s R17) both misreport their own committed probe output. The
probe (`157_probes/p1b_literal_ties.out`) shows **three** of the six non-grid literals separating
truncation from ties-to-even and **three** not separating; the prose in `157` and the finding text of
F157-5 both say "four of six... separate nothing; two of six separate." I reproduced the probe
bit-for-bit before writing this down, twice, and recount the table by hand in section 3.5. The
qualitative conclusion the finding exists to support ("non-grid is necessary and not sufficient") is
unaffected by which of the two counts is right, and the candidate's own statement (clause 4, "a full
literal") never repeats the wrong number. But R17 is a do-not-cite record whose entire job is to stop
a wrong number from being cited, and it now carries one, unnoticed through `158`, `159`, `160`, `161`
(silent on it), `162`, `163` and `164`.

---

## What I verified by independent computation, and what I took on report

**Independently rebuilt and confirmed byte-for-byte** (rerun after resumption, from the committed
source, on the committed toolchain pin, without reading the committed output first except where
noted as a diff target): `157_probes/p2_const_certificate/cert.rs` both builds; `157_probes/p1b_literal_ties.py`;
`111_probes/p4_constant_injection_collapses_the_two_degeneracies.py`; `162_probes/p1_how_far_does_the_premise_reach.py`;
`162_probes/p2_cfg_in_const_fn/run.sh` (both builds); `163_probes/p1_clause9_satisfiability.py`;
`163_probes/p2_offset_is_not_the_discriminator/offset.rs` (run and `--cfg oob` control);
`163_probes/p3_audit_the_accounting.py`; `160_probes/p2_lens_degeneracy/lens.rs` (run and `--cfg
control`); `160_probes/p3_packed_weakening/packed_weaken.rs` (run and `--cfg control`);
`164_probes/anchor_accounting/count_anchors.py`; the four-crate `struct/type/trait` grep behind
`157` F157-3 / `159` F159-1; the `grep -n "I15"` behind `162`'s L15 refusal; `109` section 11's text
at source (no entailment argument, grounds only in I13); `154_probes/p1_saturation/sat.s` (the
`cmp w8, #63` at `_b_unsat` against the plain `and` at `_a_sat_47`); `OPTIONS.md:2495-2510` at
source (the literal sentence R17 retires); `warm-container-shared/src/lib.rs:5-11` and `:1356` at
source (the shipped carrier-rule comment and the oracle test `164` cites as its second catch).

**Taken on report, and named as such.** `109`, `110`, `111`, `112`, `114`'s own probe outputs beyond
what topic five's replies already re-verified (I read the files but did not rerun every probe in
them; the panel's own reply chain, `110` R0-R8, `111`'s reply to `112`, already carries several
independent reruns and I did not duplicate all of them). `161`'s and `164`'s citation-check probe
outputs (`citecheck.out` in each probe directory) were read, not rerun; their methodology (open every
`file:line`, test the substring, carry two deliberately-wrong controls) is sound on inspection and I
spot-checked several of the citations they claim to have checked, at source, in the course of
verifying the substantive claims above, and found them accurate every time I checked. `163`'s p4 and
p5 sweeps (its own anchor and findings-survival enumeration) are taken on `163`'s report, per `163`'s
own coverage statement; I did not rerun them. Everything in topics six through eight (`122` through
`152`) is taken on `AGREEMENTS.md`'s and the member files' own account, since the brief scopes this
check to topic five's two sittings and I did not open those files.

**Which sections below would move if something I took on report were wrong.** Section 3's chain
(L15's rung correction) is fully independently verified end to end. Section 3.3's three-catches
verification is fully independent. Section 3.5 (the F157-5 miscount) is fully independent and
reproduced twice. Section 4 (the container-premise sweep) rests on my own read of all thirteen
clauses against the container-premise question, cross-checked against `164`'s own table, and is
therefore a second reading rather than a rerun of an instrument; if `164`'s table is wrong somewhere
I did not independently reconstruct, my agreement with it inherits the error. Section 5 (what remains
open) is a grep-and-read check, not a computation, and its only failure mode is a missed occurrence,
which I bounded by running the greps myself rather than trusting the prior files' claims about them.

---

## 1. Coverage, bounded

**Read in full, this dispatch, across both the pre-cut and post-cut sessions:** `INTENTS.md`,
`RULES.md`, `109` (both phases), `110` (phase one, phase two, and the reply through R8), `111`
(sections 0 through 26, the full reply chain), `112` (sections 0 through 15), `113`, `114` (sections 0
through 14), `154` (both phases), `155` (both phases, including its addendum), `156`, `157` (all
sections including the reply-round-shaped attack and its findings block), `158`, `159`, `160`, `161`
(all nine sections plus the anchor accounting), `162`, `163`, `164`.

**Read in part, at the cited sections, opened at source rather than through another file's
quotation:** `108:820-830`; `OPTIONS.md` Q16, Q51, Q52, and lines 2495-2510 (the R17 sentence);
`AGREEMENTS.md` heading map only (`grep -n '^## '`), to confirm the "no topic-five section" claim
that recurs through the corpus; `warm-container-shared/src/lib.rs` at lines 1-11, 187, 279-283,
1340-1400; `bitpack-shared/src/lib.rs` at its column declaration and pack call, per `163`'s citation;
`154_probes/p2_fibre/` and `154_probes/p1_saturation/sat.s` at source.

**Not opened at all:** `63`, `74`, `90`, `106`, `108` beyond the cited lines, `115` through `152`
(topics six through eight), `122`, `123`, `124` (the propagation-topic ledger `161` and `164` both
point at without restating), `PRIOR_CALLS.md`, `PERSONA_CALLS.md`, `HANDLES.md`, `DROPLIST.md`
beyond its heading map, `seed/`, `archive/`. Where I state something about those, I say so and name
whose account I am relying on.

---

## 2. The two gates

**Canon gate: passed.** Checked against `INTENTS.md`, read in full including its normative "How to
read an entry" section, and `RULES.md`, read in full. This dispatch is an entailment-and-anchor check
on a panel-internal candidate revision; nothing in it touches, argues with, or presumes a ruling on
any RATIFIED entry. I13 is used throughout as the standard every predicate is measured against and is
never treated as open. Aligned; proceeding.

**Test gate: passed, at 123 across 13, run per the brief.** Crate by crate, `--release`,
`bitpack-write-contend-shared` serialised and otherwise untouched:

```
12 crates: 9+12+6+5+3+6+1+3+11+7+15+30 = 108
bitpack-write-contend-shared (--test-threads=1)  =  15
total                                            = 123, all passing
```

`165_probes/run_test_gate.sh`, output at `165_probes/gate_release.out`. This is the fourteenth
independent count in this arc, all agreeing at 123 across 13. `mock/crates/` is empty by design
(`ls mock/crates/` returns nothing); the suite-bearing surface is `mock/benches/variants/`'s thirteen
`-shared` crates and nothing else in the repository. Bodies were not re-read this dispatch; the
reliance is the same three mechanical scans every file in this arc since `157` names: `154`'s
mechanical scan of all 123, `155`'s full read of `warm-container-shared`'s fifteen, `157`'s full read
of `bitpack-write-contend-shared`'s fifteen. I read none of the 108 remaining test bodies myself.

`bitpack-write-contend-shared`'s known soundness bug (stale-pointer write under two concurrent
coordinators, `pool.rs:110-111`) was not touched, per the brief; it is not fixed as of this file.

---

## 3. Verified: the chain from `157` through `164`

### 3.1 L15's rung correction

`157` recorded L15 (the const-availability membership criterion's entailment) at "CONVERGED on the
split; TWO+ INSTANCES on the entailment's ground (`154` blind, `109`'s criterion blind, different
routes)". `162` refused the second half on its own row, arguing that `109` section 11 grounds its
criterion's "must" in I13's scope and never derives the modal's necessity from anything, so `154`'s
entailment (a runtime width forces the `cmp` I15 forbids) is the only derivation of the compulsion and
sits at ONE EXPERT.

I reran the grep myself: `grep -n "I15" 109_bellard_the_primitive_derived_cold.md` returns lines 310,
320, 452, 454, 656, matching `162`'s report exactly. I opened all five: two are the naming-as-validator
paragraph (I15 as the thing that catches an invalid composition, nothing about a modal's necessity),
two are the I18 section (I15 as the rule I18 is a bounded exception to), one is alternative C
(a speculative direction, "the most aggressive reading of I15 available", not an argument for the
membership criterion's necessity). None derives that a const-available parameter is *compelled*. I then
opened `109` section 11 at source (`109:525-557`) and confirmed it grounds "const-available" in I13's
"whatever is available at const time" and says in its own words "I did not choose it": a scope
statement, not a compulsion argument.

I independently rebuilt `154_probes/p1_saturation/sat.s` (already committed, so this is a read rather
than a rebuild, but I opened it fresh at source): `_b_unsat` carries `cmp w8, #63` immediately before a
`csinv`, the runtime range check I15 forbids in as many words, on the arm whose only difference from
`_a_sat_47` (a plain `and`, no comparison) is that the width is a runtime value rather than a const one.
That is the entailment `162` credits to `154` alone, and it is real: no other file in the corpus states
it as an argument from I15.

`163` independently reread the same five I15 occurrences and the same section 11 text and concurred,
marking its own agreement as verification rather than a second derivation. `164` accepted the
correction and rewrote L15 to: CONVERGED on the split, ONE EXPERT on the entailment.

**My own check adds nothing new here beyond independently reproducing the grep and rereading the
cited text myself; the correction chain is sound and each step is checkable, and I checked each
step.**

### 3.2 The lens degeneracy repair and its third failure direction

`157` S-8 (offered, then adopted by `159`): "where the position is const-zero and the carrier is one
machine word, the lens is an identity and the thing is a value." `160` found this insufficient in two
directions (a shared occupant at offset zero is wrongly admitted; a padded sole occupant is wrongly
excluded) and repaired it to sole occupancy. `163` found a third direction `160`'s own probe never
instantiated: a sole occupant at a *nonzero* offset, which S-8's condition also wrongly excludes, and
built the case.

I independently rebuilt `163_probes/p2_offset_is_not_the_discriminator/offset.rs`, both the run and
the `--cfg oob` control. The run reproduces bit-for-bit: a 13-bit sole occupant at offset 3 of a `u16`
round-trips over all 8192 values with zero disagreements, is `Sized` at two bytes, and is
referenceable; the shared occupant at the same offset costs eight bytes (the whole carrier) and leaks
its sibling through the same reference. The control refuses to compile with `E0080` at the exact line
the committed source declares it, once rebuilt from the current, hook-formatted source (see 3.4 below
for the one place this rebuild differed from the committed artifact).

**Verified: sole occupancy, not position, is the correct discriminator in all three of the directions
this arc's probes construct.**

### 3.3 The three catches `164` claims and none of the two signatures stated

`164` claims to have reproduced every dissent before accepting it, and to have caught three things
neither signature stated. I checked each.

**One: `163`'s committed control error is line-stale.** `163_probes/p2_offset_is_not_the_discriminator/offset_control.err`
names line 53; the committed source's `_OOB` assertion is at line 66. I rebuilt from the committed
source myself and the refusal fires at line 66 with the identical message, confirming the artifact is
stale in the direction `160`'s own p1 already demonstrated (the pre-commit hook rustfmts committed
probe sources after the error was captured). **Real, and independently reproduced.**

**Two: the carrier pair's shared denotation is asserted by a shipped test, not assumed by a model.**
`163` names as its own risk that its clause-9 satisfiability probe "assumes two markers over one
`(I, F)` can agree in value set and realisation map." `164` finds that the shipped suite already
asserts exactly this: `warm-container-shared/src/lib.rs:1356`,
`all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`, requires every arm over every
carrier rule (including the `headroom` and `minimum` arms, which differ only in carrier) to produce
byte-identical results against an independent `u128` oracle, over every declared key. I opened the
function at source, confirmed the line number, and confirmed the assertions (`assert_eq!(h, m, ...)`
where `h` and `m` are the headroom- and minimum-carrier arms) are exactly what `164` describes. I also
opened `:5-11`, the shipped rule comment `163` cites for the carrier pair's construction ("Warm and
Precise take one rung above... Hot and Cold take the minimum"), and it matches verbatim. **Real, and
the citation is accurate.**

**Three: `163`'s reading 3 is `110` section 6's second branch, not a new position.** I opened `110`
section 6 at source and found the exact sentence `164` cites: "If it never does, the strategy is a
selector over presentations and is not part of identity at all." `164`'s argument, that clause 9
refusing the carrier pair under footprint-internal-and-no-X3 is that branch stated mechanically, is a
correct reading of the citation; it is an argument rather than a measurement, and `164` labels it that
way. **Real, and correctly labelled as an argument rather than a new empirical result.**

### 3.4 The one place a reproduction did not match its artifact for a reason that is not the finding

`164` reports its own rebuild of `163` p2's control differed from the committed `offset_control.err`'s
line number and attributes it to the rustfmt-shift class `160` already found in its own p1. I
reproduced this myself, independently: rebuilding from the committed source (post-hook-formatting)
gives line 66; the committed `.err` file, captured before the formatting hook ran, holds line 53. This
is exactly the class `154` P2.8 and `160` section 8 both name, and it is now the third instance of it
in this arc. Nothing about the finding moves; the artifact is what needed refreshing, and I confirmed
that refreshing it (rebuilding from committed source) reproduces the claimed message and line.

### 3.5 A finding neither signature nor the revision caught: F157-5's own count is wrong against its own probe

Checking R17 (`164`'s new do-not-cite entry for `OPTIONS.md:2502`'s literal-observability sentence)
against its cited evidence, I opened `157_probes/p1b_literal_ties.out` and reran
`157_probes/p1b_literal_ties.py`; it reproduces bit-for-bit. The output:

```
literal   grid?   separates trunc/near at F=0 (W=3 unsigned sat)
    1/2    no     no   trunc=0 near=0
    3/4    no     SEPARATES   trunc=0 near=1
    1/3    no     no   trunc=0 near=0
    2/3    no     SEPARATES   trunc=0 near=1
    3/2    no     SEPARATES   trunc=1 near=2
    5/2    no     no   trunc=2 near=2
      1    yes    no   trunc=1 near=1
      2    yes    no   trunc=2 near=2
```

Counting the six non-grid rows by hand: `1/2` no, `3/4` SEPARATES, `1/3` no, `2/3` SEPARATES, `3/2`
SEPARATES, `5/2` no. **Three separate, three do not.** `157`'s own prose two lines below its own
table says "Four of six non-grid literals separate nothing", and F157-5's finding text repeats it:
"Four of six non-grid literals tested separate truncation from ties-to-even nowhere; two of six
separate." Both sentences contradict the table printed directly above them in the same file, and both
contradict the probe output I reran.

I checked whether this is a stale-artifact class like 3.4 (an old run before a fix) rather than a
genuine miscount: `157_probes/p1b_literal_ties.py`'s candidate list (`1/2, 3/4, 1/3, 2/3, 3/2, 5/2, 1,
2`) is fixed at the top of the file and is not the kind of thing a formatting hook could move; the
probe has one code path and no cfg branches. It is a hand-count error in the prose, not a stale
artifact.

**Consequence, checked rather than assumed.** The qualitative claim F157-5 exists to support ("non-grid
is necessary and not sufficient") is unaffected: three counterexamples establish "not sufficient" as
well as four would. `111` F111-5, which the revision's R17 correctly names as the surviving basis
(dense rational sample, three named exceptions, reproduced in section 3.6 below), is untouched by
this and does not repeat the error. The candidate's own clause 4 ("a full literal... reaches
saturation at depth one") never quotes the four-of-six figure and is unaffected. What is affected is
the do-not-cite entry itself: R17 is the mechanism built specifically to stop a wrong number from
propagating, and as written it propagates one, unnoticed through `158`, `159`, `160`, `161` (which
does not restate the count at all and is silent on it), `162`, `163`, and `164`.

This is not a finding I am confident belongs at a rung higher than "worth a one-line correction in the
next file that touches R17." It does not touch the RATIFIED rung, does not change any clause of the
statement, and does not change what the candidate is entitled to claim. I report it because it is
real, checkable in thirty seconds by anyone who reads the table two lines above the sentence, and it
is exactly the kind of thing this workspace's own citation-checking discipline exists to catch, and in
this instance did not.

### 3.6 F111-5 itself is correct

Separately, since `164`'s R17 leans on F111-5 as the surviving basis, I reran `111`'s own probe
(`111_probes/p4_constant_injection_collapses_the_two_degeneracies.py`) and it reproduces bit-for-bit:
33 of 36 pairs observable under a dense rational sample, three named exceptions (`W=2, W=3, W=4`
unsigned saturating, truncate against floor). F111-5's own arithmetic is right; only `157`'s later,
separate probe and its own prose miscounted.

### 3.7 The two-branch completeness certificate

`160` repaired `157`'s S-14 (a witness-only completeness clause, which cannot distinguish a refinement
pair from a spurious pair and, taken mechanically, licenses deleting every refinement parameter the
realisation-map topic depends on) to a three-outcome scheme: separated by a witness, connected by
weakening in exactly one direction, or neither (refused). I independently rebuilt
`160_probes/p1_two_branch_certificate/cert2.rs`, and it reproduces the committed run exactly: the
declared-semantics pair gets `witness=true, directions=0`; the refinement pair gets `witness=false,
directions=1`; the spurious pair gets `witness=false, directions=2` and the certificate refuses to
compile under `--cfg carry_spurious`, which I also rebuilt and confirmed fires.

`163` independently reproduced the middle outcome (`directions=1, witness=no`) on a separately written
model inside its own clause-9 probe (control G2), which I reran and confirmed matches. That is a
genuine second instrument for the refinement-pair classification, and I confirm it is separately
constructed rather than a copy: `163`'s probe builds the carrier-pair scenario from the shipped
`warm-container-shared` rule rather than from an abstract model, which is a different construction
from `160`'s.

### 3.8 The `cfg` soundness hole

`157` F157-13 shows a `const fn` realisation map can read `cfg` and produce one type name that denotes
saturation in one build and wrapping in another, both builds individually satisfying I15's "one
lowered path, no runtime check" completely. `158` rebuilt it byte-for-byte from committed source and
confirmed. `159` added the independent argument that I15 cannot see the hazard because it is a
relation between builds, not a property of any one build. `162` independently rebuilt a differently
written model (`162_probes/p2_cfg_in_const_fn/`) rather than reusing `157`'s file, on the ground that
a claim being made against `157` should not rest on `157`'s own instrument.

I reran `162`'s independent build myself, both `base` and `--cfg feature="alt_policy"` arms: the
hazard differs (`8191` against `0`), the stable control agrees (`8191` both times), and neither build's
emitted body branches on the build (`branch on a build value: none` in both). This reproduces bit-for-
bit and is a genuine second, independently-constructed instrument, not a rerun of `157`'s.

---

## 4. The container-premise sweep, checked clause by clause

`156` item 1 (whether footprint is in the operation set the design ships) is the reserved question.
`162` found it reaches clauses 2 and 6 of the statement; `163` found it reaches clause 9 as well, and
that clause 9's *admissibility*, not merely its extension or truth value, moves; `164`'s section 2
replaced the finding-by-finding count with a sweep table over all thirteen clauses and thirty ledger
entries, reporting three clauses conditional in full (2, 6, 9), one by design (4, already marked from
`161`), and two subordinate phrases (clause 10, mirrored at L21; L4's first phrase) that neither
signature had reached.

I reran this sweep myself, independently, against all thirteen clauses of `161`'s statement (section
4), asking of each: does whether footprint is observable change this clause's truth, extension, or
admissibility?

- **Clause 1** (denotation: value set + one realisation map over a declared operation set). No. The
  clause names a variable ("a declared operation set") without committing to its contents.
- **Clause 2** (identity up to isomorphism relative to the operation set). Yes, extension: I
  independently reran `162_probes/p1_how_far_does_the_premise_reach.py` and confirmed 32 primitives
  under container-internal against 64 under container-observable, every identical-`(V,R)` pair split.
- **Clause 3** (law read off, closure prior to law). No; nothing in the definition of a law or of
  closure reads the container.
- **Clause 4** (signature part of the definition, reach theorem). Conditional by design; already
  marked in `161`.
- **Clause 5** (realisation is a lens, sole-occupancy degeneracy). No, and `160` section 2.3's
  invariance argument is the reason: the degeneracy condition reads the placement, not whether the
  placement is part of identity, so it is stable on both branches. I checked this argument rather than
  just citing it, and could not find a way the premise reaches the lens *form* without also reaching
  identity, which is exactly what clause 6 (not clause 5) marks.
- **Clause 6** (realisation not part of identity / part of the surface). Yes, truth value: clause 5
  defines the realisation as "a carrier, an offset and a width"; under footprint-observable the carrier
  is identity-bearing, so clause 6's first sentence is false on that branch, not merely narrower.
- **Clause 7** (refinement: not a coordinate, not declared semantics, a grade). No. A refinement pair
  shares the realisation map on both branches; whether the carrier the map is attached to is itself
  identity-bearing does not change what a refinement over that primitive is.
- **Clause 8** (type is const-available parameters; membership vs. identity criteria). No; neither
  criterion mentions the operation set's contents.
- **Clause 9** (adequacy: soundness, completeness up to weakening, per pair). Yes, admissibility: I
  independently reran `163_probes/p1_clause9_satisfiability.py` and confirmed the carrier pair
  (`warm-container-shared`'s shipped `headroom`/`minimum` rule) is refused as a spurious split under
  footprint-internal and separated under footprint-observable, with all three declared controls
  passing.
- **Clause 10** (verdicts age differently under signature growth). One phrase: "the realisation map's
  whole domain" is the footprint-internal reading of "the largest signature", and needs the branch
  note `164` adds. The aging claims themselves (witness preservation, refinement stability) hold on
  both branches.
- **Clauses 11, 12, 13** (naming, composition, chain accuracy). No; I checked each against the
  container premise directly and found no route by which footprint observability changes any of them.

**This matches `164`'s own table exactly, on an independent pass.** I specifically checked whether the
sweep missed a fourth full-clause dependency or a third subordinate phrase and found none in this
pass; I also confirmed, by grepping `162` and `163` for "clause 10", "L21" and "L4", that neither
signature mentions any of the three (162's grep: no hits; 163's grep: one hit, in its blanket "signed
as written" list for L21, meaning `163` signed L21 *without* the amendment `164` later applies), which
is the concrete evidence that the two subordinate phrases are genuinely new to `164` and not a
restatement of something the signatures already said.

---

## 5. What remains open, checked rather than assumed closed

I grepped `164` (and, where it defers, `161`) for any place the container premise, Q65, or X1 through
X4 might have been quietly resolved rather than merely discussed, and found none:

- **The container premise (`156` item 1).** Every clause `164` marks conditional is phrased as a
  disjunction over both branches, with neither branch asserted. `164`'s own coverage section states
  plainly it "could not choose a branch."
- **Q65's marker question** (whether a proof carries a different marker from a measurement). `164`
  section 5.3 (inherited from `160`) states the `[argument]`/`[sweep]` tags are "interim compliance, not
  a proposal to settle the marker", and I found no place downstream that treats the tags as settled.
- **X1 (O-A against O-B).** Unresolved; `159` F159-3's mapping is carried unchanged and `164` does not
  touch it.
- **X2 (the boundary shape at the wall).** Unresolved; `160` section 5.1 states the two-designer
  discriminator is unrun, and nothing later runs it.
- **X3 (whether a strategy ever changes a computed value).** Relocated, not resolved. `164` section 3,
  verified above, states three readings and says explicitly "none is chosen here" and that the
  narrowing from the shipped oracle test "decides nothing" about which reading is correct.
- **X4 (where the next refinement-propagation dispatch goes).** Untouched by this unit; pointed at
  `122`'s ledger throughout, per `161` section 9's own coverage bound, which I did not verify since it
  is outside topic five.

**No live option was silently dropped.** I diffed `161` section 7's five options (O-A, O-B, O-C, O-D
[closed, retained as a retirement], O-E) plus its two open items (Q157-C, Q157-E) against `164`
section 6, and confirmed `164` is purely additive there (adds O-S13, touches nothing else), consistent
with `164`'s own header statement that sections not named stand unchanged.

---

## 6. A residual, minor, not escalated

`163` section 7 found that `161` section 8's closing sentence ("Zero novel anchors means nothing here
cites material outside the thirteen sources...") contradicts `161`'s own accounting table three lines
above it, which reports one novel anchor (`82:770-774`). I confirmed this against `161`'s own committed
`accounting.out`. `164` section 7 explicitly repairs the *class* of error in its own accounting
("the closing claim is stated over the measured novel set, whatever its size") but does not add an
explicit correcting note to `161`'s own file or to the retirement list saying that `161`'s specific
sentence at line 772 is false as written. Since `161` stays as landed and is not edited in place, a
reader opening `161` directly still meets the false sentence with nothing pointing them to the
correction. This is the same shape as every other repair in this arc (correct in a later file, `161`
itself untouched), so it is consistent with how the arc has handled every other defect; I note it only
because it is the one place the repair is implicit (fixed in kind, in `164`'s own accounting) rather
than explicit (a numbered retirement entry naming the specific false sentence). Not escalating this
to a finding of its own; it is a completeness note about the retirement list, not a defect in the
candidate's substance.

---

## 7. What I did not do

I did not rerun `109`, `110`, `111`, `112`, or `114`'s probes beyond what topic five's own reply
chain already rebuilds and beyond the specific ones this file's claims lean on (`p4_constant_
injection_collapses_the_two_degeneracies.py`, and the `109`/`154` assembly files). I did not verify
`161`'s or `164`'s citecheck probes by rerunning them; I spot-checked the specific citations my own
claims depend on and found them accurate every time. I did not open topics six through eight
(`115` through `152`), so I cannot independently confirm `161`/`164`'s claim that topic eight's ledger
(`151`, checked at `152`) genuinely converged on X3's disposition; I take that on the candidate's own
account, per its coverage bound. I did not price anything; every cost claim in the corpus (C5, Q157-E)
remains explicitly unpriced and nothing in my check changes that.

## 8. Conceding nothing, correcting one thing

The revision is sound. Every load-bearing repair it makes to the two signatures' findings is real and
reproduces under independent rebuild. The one genuine defect I found (F157-5's own miscount, inherited
into R17) is small, does not touch the statement's content, and is the kind of error this workspace's
own discipline is built to catch; that it survived seven files is worth naming, not worth treating as
grounds to distrust the rest of the chain, which held up to every check I ran against it.
