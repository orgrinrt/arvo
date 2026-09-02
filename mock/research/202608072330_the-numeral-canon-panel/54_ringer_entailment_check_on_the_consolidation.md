# 54. Entailment check on the consolidation

**Date:** 2026-08-09. **Persona:** Talia Ringer. **Mode:** explore, do not settle (`00_brief.md`,
`04`, `28`). Nothing here settles anything.

**Status: COMPLETE.** Written to disk early per `RULES.md:328-329` and extended in place.

## 0. Gates

**Canon gate: passes.** `mock/canon/` does not exist on this branch, so there is no canon to defend
or diverge from. This check itself is squarely inside the fixed material: `RULES.md` names the
entailment check as a standing panel obligation ("The entailment check is run by someone other than
its author... Then repair, then it stands", `RULES.md:309-311`), `53` names itself as owing exactly
this check to a different reader (`53:534-538`), and my own dispatching brief points at the same
mechanism from the workspace side. Nothing below proposes anything the forbidden-feature list
excludes; nothing below edits `53`, `OPTIONS.md` or `INTENTS.md`.

**Test gate: nothing to run.** `mock/crates` is empty by construction. My evidence is direct reads of
every member file `44` through `52`, of `53` itself, of the relevant passages of `RULES.md`,
`INTENTS.md`, `00_brief.md`, `OPTIONS.md` and `seed/SETTLED_container.md`, and a handful of
reproducible shell commands over the panel's own text (anchor extraction, `grep -c` refusal counts,
`grep -n` sentinel checks). No probe was needed and none was written; `54_probes/` does not exist and
is not claimed.

## 1. Coverage, stated first because everything below is bounded by it

**Read end to end, directly:** `44`, `45` (including its sections 11-12 reply), `46`, `47`, `48`,
`49` (both phases), `50`, `51`, `52`, `53` in full.

**Read at the specific passages cited, by opening the lines:** `RULES.md` lines 1-100 and 290-330
(provenance ladder, canon tests, entailment-check instructions), `00_brief.md` lines 140-155 (the
acceptance criterion as it currently reads), `seed/SETTLED_container.md` lines 25-40 (op's
ratified words, checked character for character), `17` lines 676-690 (the counts-are-fragile
passage `53` cites), `OPTIONS.md` lines 1-20, 700-780, 900-960 (the derivation's-outputs section's
current state, read to establish what has changed since `53` was written, never cited by line in
this file per my brief).

**Verified with commands, not trusted from prose:** the anchor sets of `44` through `52` and of `53`
(Python extraction, reported in section 3); the refusal-count table's individual entries
(`grep -c` against `00_brief.md` and against `OPTIONS.md`'s rung and count language); the current
state of `00_brief.md:145` and of `OPTIONS.md`'s S2 correction.

**Not done, and what that leaves unverified.** I re-ran no probe from `44_probes/` through
`52_probes/`; I read their sources and their authors' quoted output as the panel's own discipline
treats a probe (cited for what it proved, checked by opening it, not re-executed by every
subsequent reader), the same treatment `44` and `50` gave the files below them. I did not open
`15`, `16` or `17` end to end; where a claim in `53` rests on them I checked it against the member
files' own quotations and, in the one case that mattered most for a load-bearing claim (`17`'s
counts-are-fragile passage), against the primary source directly. I did not read `02` through `43`
except `43` as quoted by `44`, and I did not read the closed predecessor panel, `SETTLED.md`,
`DROPLIST.md`, `PERSONA_CALLS.md`, or `mock/benches/` beyond the specific variant sources `50` and
`52` already opened and I re-checked their citations against. I did not audit `OPTIONS.md` end to
end; I read the sections that bear on this unit and one that turned out to matter more than
expected (section 6 below).

## 2. Verdict, stated up front

The consolidation entails its sources. Every finding I checked either survives in `53`'s text or is
explicitly and correctly declared out of scope in `53`'s own words. I found no case where `53`
asserts something a member file contradicts, and no case where a load-bearing numeric claim in a
member file is silently absent from `53` without a citation a reader could follow back to it.

What I found instead is smaller and still worth reporting precisely: one genuine miscount in `53`'s
own self-description that is stale rather than wrong (section 6), one candidate sentence whose
"passes" verdict is honest in its supporting prose but slightly ahead of what the evidence supports
against the workspace's own three-instance bar (section 7, S2), and a handful of places where `53`
compresses a raw number or a named result to a qualitative statement, which costs a reader nothing
they could not recover by opening the cited section but is worth naming since the compression rule
exists precisely to catch this class (section 5).

## 3. Measurement one: entailment, file by file

For each member file: what it established, what it retracted (of its own or a predecessor's prior
claim), what it flagged as open, and whether `53` carries it. I worked from the member files
forward, per the method this check is required to use, and only then checked `53`'s account against
what I had already written down.

**`44` (Arntzen).** Established: the acceptance criterion's plural noun had drifted to singular
through three prior generations, traced to its source (`seed/SETTLED_container.md:33-35`) and found
collapsed a fourth time in the panel's own founding brief (`44:150-177`); a genuine misattribution in
the register, where a claim about the general carrier-size keying was cited for the narrower
wide-rung alignment sub-case that `16` itself disclaims testing (`44:197-256`). Retracted nothing of
its own. Flagged: the `Precise` widening question as cheap and op's; a second container-derivation
artifact (`10`'s bridge) sitting underneath and unaddressed by the two-output finding; stride's
grid-invariance as an unclaimed cheap check. `53` carries the plural-restoration story faithfully
(section 0, "op's restored plural") and the misattribution's resolution (section 3.2, the
carrier-size/wide-rung split, correctly stated as TWO EXPERTS/ONE EXPERT). What `53` does **not**
carry from `44` is `44`'s own internal inconsistency (`44:72-79` versus `44:380-383`), and that
omission is deliberate and correct: `53` is not auditing `44`, it is building the corrected rung from
scratch, and the inconsistency is exactly what the correction supersedes. `10`'s bridge is carried
(section 3.3, "the road from computed values back to types is closed... which is `10`'s bridge
problem... that is where the wall should be attacked next, and nobody has").

**`45` (Fallin), including its reply to `46`.** Established, then partially retracted under attack:
the pigeonhole argument that no fixed-width per-step rounding rule matches the once-truncated exact
chain answer, exhaustive at four fraction widths, two independently coded instruments cross-checking
to identical counts (61/732/7354/73461 at F=3..6 under round-half-up). Retracted: the claim that two
outputs are forced twice over (`Cold` alone, and independently by `Hot`'s wide-rung alignment),
narrowed under `46`'s attack to `Cold` alone unconditionally and the alignment forcing as
conditional on an unratified axis (`45:553-583`); the `p4` "widening recovers" check, conceded as
tautological and replaced by `45_probes/p6`'s finite-headroom model. `53` carries the pigeonhole
existence result (section 3.4, "established exhaustively at four fraction widths by two
independently coded instruments cross-checking to identical counts") but not the raw witness
counts themselves; carries the retraction correctly (section 3.1, "the wide-rung half of the keying
claim... is ONE EXPERT... It remains ONE EXPERT at this writing"); carries the `p4` concession and
`p6` replacement (section 3.4, "the vacuous half of the original evidence caught by `46` and
replaced by a real finite-headroom model whose result is that most witnesses need one extra bit and
a growing tail needs the full doubling"). This is a faithful carry of the retraction, which is the
harder direction to get right (a consolidation can quietly drop a concession and let the earlier,
stronger-sounding claim stand; `53` does not do this).

**`46` (Dolan).** Established: the wide-rung alignment forcing rests on an unratified assumption
(`15`'s own "safe to leave open" framing, quoted and checked at source) and should be conditional,
not unconditional; the general alignment mechanism holds unconditionally once disentangled from
arvo's specific architecture (`45_probes/p7`); the `p4` tautology, caught by opening the source and
finding two lines were the identical Python expression. Retracted nothing of its own; this is an
attack file. Flagged: whether a design keeping `Hot` and `Warm` on the same wide-rung alignment is
live; the crossover width `15` itself calls contested. `53` carries the conditional restatement
(section 3.1, matching `46`'s own proposed wording almost verbatim) and the `p4` tautology (section
0, test gate: "one tautological check (`45_probes/p4`, caught by `46:171-201`, conceded and
replaced at `45:591-643`)"). One thing `53` does not carry from `46` and should not need to: `46`'s
own miss, caught by `48`, of not reading `45_probes/p1`'s first line before attacking it. That is a
finding about `46`'s process, not about the topic, and `53` is right to leave it out of a canon
candidate.

**`47` (Wingo).** Established: the one-versus-two count was never a fork, since any product is one
thing (`47:61-64`); a single type-valued output holds and is the pair wearing one name, while a
single value-valued output is compiled-refused, establishing the kind boundary as the real wall
(`47_probes/p1`, `p2`, `p2b`); under the `Precise`-widens reading the pair is not merely irreducible
but insufficient (`47_probes/p5`, `p5b`); the packed access width, defended by `16` on
recoverability grounds, costs a second rung partition sharing no jump point with the native one
(`47_probes/p6`). Retracted nothing of its own, and attacked `46`'s downgrade of the alignment
forcing on a sharper ground than `46` itself found (the collision is on `(width, stride)`, not on
the derivation's output pair, `47:188-252`), which `53` carries in full (section 6, "Resolved inside
the unit" entry). Flagged: whether any lowering site actually needs the access width as a type,
explicitly marked unpriced. `53` carries the kind-boundary result as its best-established mechanism
claim (section 3.3), the insufficiency-under-widening result (section 3.4, "the pair is not merely
irreducible but insufficient"), and the two-ladder cost, though see section 4 below: `53` carries
the **reversal** of `47`'s two-ladder cost, correctly attributing it to `50`, and this is the one
place a member file's finding does not survive into `53` unmodified because a later file overturned
it, which is the right treatment.

**`48` (persona checkpoint).** Not a rung-bearing member; explicitly non-authoritative
(`RULES.md`'s persona rung). Established (as findings, carrying no weight of their own): two errors
in the brief that dispatched it, the more consequential being the rung inflation from a "third read"
into "independently derived by `45` and `46`"; `44`'s internal inconsistency between its own section
2 and its own keep list; that `16`'s criterion is applied two incompatible ways inside `16` itself
(Reading A/B), which it flagged as one reader's finding needing a second before anything is built on
it. `53` carries the rung-inflation finding as one of the two inflation instances named in its own
opening paragraph (`44:380-383` against `44:72-79`, and `48:49-79`), and carries the Reading A/B
finding as the seed of `53`'s own retirement of the criterion (section 4.1), correctly crediting
`50` with the formal resolution rather than treating `48`'s own reading as settled. This is the
correct treatment of a persona file: its findings are carried where they were confirmed by a rung-
bearing dispatch, not on their own authority.

**`49` (Marlow), both phases.** Established, cold, before reading any panel file: a schema of six
candidate facts, later corrected on reading the panel to align with the converged carrier/stride
pair, with two of its own six over-counted (alignment, and a `Cold` standalone-storage divergence)
and conceded as such; independently, the schema-is-uniform-across-strategies shape that answers
`48`'s open item about `I1`'s open strategy set; validation as a missing-impl refusal
(`49_probes/p2`), which `53` correctly credits as the one clause of the acceptance criterion nobody
else in the unit explicitly discharged (section 8, "ONE EXPERT, uncontested"). `53` carries the cold
derivation's role in the rung honestly and precisely (section 3.1: "one cold instance at a coarser
grain: `49`, reading only op's intents and the workspace rules, derived blind that a per-aggregate
composition fact must stand beside the storage fact... The cold instance supports 'more than one
fact, and the second is aggregate-keyed'; it does not support 'exactly two', since `49`'s blind
count was six, corrected on reconciliation"). That is an accurate, ungenerous-to-itself account of
what `49` actually supports, matching `49`'s own phase-two concession rather than the stronger
reading a compressor might have reached for.

**`50` (Lamport).** Established: `16:100-101`'s criterion is a self-referential fixpoint equation
with an antitone operator, admitting output sets of size zero, two, three and four across sixteen
cells depending on three unstated parameters (site model, strategy-set closure, kind regime); the
unit's converged answer comes from exactly one cell, built on a closed strategy set (contradicting
`I1`) and an unbounded const-to-type rule (contradicted by twelve compiled refusals as counted at
that time); Reading A is unsound, not merely a weaker reading, because it loses the declared width
for 389 of 512 declarations; Reading B does not give a count of one as `48` claimed, it gives zero
under a naive reading or a two-member set (`{ACCESS, CARRIER}`) that shares only one member with the
unit's converged pair, once the kind boundary is honoured; the access-width closed form is exactly
right as a worst case but over-estimates at 48 of 128 widths, moving `16`'s 28-of-64 to 16-of-64;
`47`'s two-ladder cost reverses once the true phase set is used (the access partition refines the
native one rather than sharing no jump point); the packed-access magnitude three files called
unpriced was already measured in `mock/benches/bitpack-decoder-shape` (3.04x to 3.12x); the two
refusal-on-inexact designs (per-step, end-of-chain) admit exactly the same chains once zero operands
are excluded, proven algebraically and checked at 42 cells; a reading of `I2` ("`Precise` is most
precise at the price of both storage and compute") that the whole unit had excluded without noticing.
Retracted: `48`'s "Reading B gives one" as a factual error, corrected in place. Flagged: whether the
ownership predicate is decidable in general, and whether the fact set is stable under a strategy
introducing a genuinely new question. `53` carries every one of these findings, each with its
correct attribution and correct qualification, most visibly in section 4 (the retirement of the
criterion) and section 3.5 (the access-width and two-ladder corrections, stated with the exact
numbers `50` reports: "48 of 128", "28 of 64 widths... becomes 16 of 64", "one ladder keys both at
25 classes instead of two ladders at five and six"). This is `53`'s single most heavily-used source
file (21 of `53`'s 124 anchors point into `50`, more than any other), and the density is earned: `50`
is where most of the unit's load-bearing corrections actually happened, and `53` reflects that
correctly rather than flattening the credit toward whichever file is easiest to cite.

**`51` (Fog).** Established: the dispatch's own premise (that no packed-sequence erasure arm exists)
was false, refuted by one command against `17_probes/t2_aggregate_erasure.rs`; the panel's existing
erasure evidence at that single width (13) was a sampled law, breaking at width 18 and above across a
36-width sweep; the break is not extra instructions but a serialised reduction (one accumulator
instead of five, at W=19 eleven loop instructions against the hand-written arm's thirty-four, and the
typed arm is nonetheless the *worse* code); two attacks recover it, one (the trait-supplied gather)
at 36 of 36 widths without changing what a consumer writes; the collision is a conjunction of
loop-form delivery and access window at or above four bytes, isolated by a control that also
disproved the author's own first attribution attempt. Retracted: its own first harness's 36-of-36
green result, self-caught as structurally incapable of failing (both arms compiling to the identical
MIR); its own first attribution of the collapse to access-count alone, refuted by its own second
control. `53` carries the false-premise refutation and the self-citation honestly (section 0 of
`53`'s own preamble: "Two artifacts of mine are load-bearing in this unit: `17_probes/t2_...`, which
`51` found after a brief claimed no such arm existed"), carries the sampled-law finding with its
width boundary and the counter-intuitive fewer-instructions-worse-code result (section 3.3), and
carries the self-caught could-not-have-failed harness (section 0, test gate). What `53` compresses:
the specific per-configuration robustness table (15/16 of 36 collapsing across `-O2`/`-O3`/native,
falling to 33/36 at `-Os`) is not reproduced, and the second attack (the wide load, which at W=47
beats even the hand-written arm) is named only collectively in `53`'s open-items list ("Price `51`'s
collapse... on the harness with the five arms `51` names") rather than given its own sentence. Both
are legitimate compressions for a topic-level consolidation rather than entailment failures: the
qualitative claim (erasure was a sampled law, and it is fixable without changing the consumer
surface) survives intact and cites the section a reader would open for the numbers.

**`52` (Chlipala).** Established, independently before opening `50` or `48`: the same Reading A/B
switch inside `16`, arrived at by direct reading rather than by formal apparatus; `50`'s central
claim confirmed by rerunning every probe in `50_probes/` byte for byte; one correction to `50`'s own
framing (the equation has at most one solution per fixed cell, so the multiplicity is about
unfixed parameters rather than simultaneous solutions, which are two different claims `50`'s section
2.6 states as one); the kind-boundary refusal count corrected from twelve to thirteen, recompiled
directly rather than trusted from `50`'s prose; the group theory underneath the access-width
correction confirmed as a standard cyclic-subgroup fact, correctly applied; the pigeonhole/refusal-
equivalence algebra re-derived independently and found to match; three authors (`49` cold, `47` from
the kind side, `50` from ownership) converging on the same replacement principle from three
different routes, marked by `52` itself as its own weakest link ("a reading of three sentences in
three vocabularies... the part most likely to move," `52:402-408`). Retracted nothing of its own.
`53` carries the confirmation of `50` throughout (most of section 4 and 5 cite `50` with `52`'s
verification attached, e.g. "the group theory traced against the code by `52:198-210`"), carries the
one-solution-per-cell correction verbatim (section 4.1: "the equation has at most one solution per
fully-specified cell, so the multiplicity comes from the unstated parameters rather than from
simultaneous solutions of one instance"), carries the corrected count of thirteen (section 5's
table, and the prose at line 377 naming "thirteen (`52`'s recompilation)"), and, most importantly for
this check, carries `52`'s own hedge about the three-authors convergence rather than the flattered
version: `53` section 4.2 explicitly writes "I do not call that three independent derivations of one
principle, and neither does `52`, whose own risk section marks 'the three formulations are the same
principle' as the part of its verdict most likely to move." This is exactly the discipline the
compression rule asks for: the hedge is quoted, not summarised away.

## 4. Measurement two: anchors, counted and diffed

Per `RULES.md:311-317` ("count the citations before and after, and diff the sets... a rising total
is not reassurance"), I extracted every `file:line` and bare `NN:line` anchor from `44` through `52`
and from `53` with the same regular expression, run over both sides identically:

```
$ python3 - <<'EOF'  # extracts \w[.rs|.py|.md|.toml|.out|.err]:LINE and NN[_name]:LINE anchors
...
total unique anchors across 44-52: 251
total anchors in 53: 127
anchors in 44-52 union but not literally in 53: 216
EOF
```

The raw numbers invite the wrong reading on their own, and this is exactly the trap
`the-a-compression-is-checked-by-someone-else` names: a large difference is not itself the finding.
Most of the 216 "missing" anchors are one of four legitimate categories, and I checked a
representative sample of each rather than assuming the category from the anchor's shape:

1. **Meta-citations, a file citing its own earlier passage or a predecessor's process** (e.g.
   `44:40-53`, `45:73-76`, `46:32-39`, `47:34-42`, `48:31`, `50:177`). These are citations about how
   a file was written, not about the topic; a consolidation correctly drops them.
2. **`RULES.md` citations** (17 of the 216), all process discipline (rung ladder, three-instance
   bar, entailment-check mechanics). `53` follows the discipline these cite without needing to
   re-cite the rule that mandates it.
3. **`OPTIONS.md` citations by line** (14 of the 216), which `53` is instructed not to reproduce by
   line (section 1: "cited below by section name and grep-verified phrase only, per my brief"). This
   is a stated, deliberate policy, not a loss.
4. **Probe-file line anchors into `.py`/`.rs` sources** (roughly 25 of the 216), most of which point
   at implementation detail (a specific line of a `round_nearest_fraction` call, a specific `assert`)
   that a topic-level consolidation legitimately summarises by outcome rather than by line.

What is left after removing those four categories is a smaller set worth checking claim by claim,
and I checked every one:

- `16:161-165`, `16:255-282`, `16:264-270` (Reading A/B source passages): substance carried in
  `53` section 4.1 without the exact line numbers, since `53` re-derives the criterion's failure from
  `50`'s formalisation rather than re-quoting `16` a second time. Not a loss: `44` through `52`
  already established the line numbers and `53` is one layer of consolidation above them.
- `35:344-349`, `35:351-354`, `35:353-354` (the fold-accumulator connection `44`, `47` and `50` each
  name but decline to verify): `53` does not mention `35` or the fold accumulator anywhere. This is
  the one genuine gap I found in this pass: three separate member files flag a connection to `35`'s
  accumulator-reach finding as real but unverified, and `53` neither carries the connection nor
  states that it is out of scope. It is a small, cheap, three-times-named loose end, and it is
  missing rather than dropped, since none of `44`, `47` or `50` themselves verified it either. I
  would add one sentence to `53`'s open-items list naming it, and I flag it here rather than treat
  it as a defect, since the underlying finding was never established in the first place.
- `45:167-169`, `45:180-181` (the "no extra compute width" quote `50` and `46` both cite, with `50`
  itself correcting its own earlier citation from `45:180-181` to `45:167-169` at `50:816-818`):
  `53` does not quote this passage directly, but its content (refusal-on-inexact needing no extra
  compute width, then measured and found to admit a vanishing fraction of chains) is carried in full
  at section 3.4's second paragraph. Substance present, exact line not needed.
- `46:8`, `46:62`, `46:70-72` (small process citations about `46`'s own header and its declared
  non-independence): correctly absent, since `53`'s rung table already states the honest rung
  directly rather than pointing at where `46` disclaims one.

**What I found, stated as the diff the rule asks for.** One genuine substantive gap (the fold-
accumulator connection to `35`, named three times and verified zero times, and now named zero times
in `53` either). No case where an anchor's disappearance corresponds to a claim `53` makes that its
source no longer supports. The high raw count (216 of 251) is consistent with `53` being a genuine
compression that discards addresses while preserving the claims they supported, which is exactly
what a consolidation is supposed to do; it is not evidence of the failure mode the rule exists to
catch, which is claims surviving while their evidence quietly stops being locatable.

## 5. What the consolidation compresses without losing (named per the rule's own instruction to
count rather than assume)

Stated together here because each is small on its own and worth seeing as a set: `53` states an
existence claim or a qualitative result where a member file also gave a specific count, and does not
carry the count.

- The pigeonhole witness counts (61, 732, 7354, 73461 at F=3..6): `53` says "established
  exhaustively at four fraction widths... cross-checking to identical counts," not the numbers.
- The per-configuration robustness table from `51` (15 or 16 of 36 widths collapsing across three
  optimisation configurations, degenerating at `-Os`): `53` states the headline result (18 of 36 at
  the reference configuration) and omits the robustness sweep.
- `51`'s second attack (the wide load) and its W=47 result, where it beats the hand-written arm:
  named only inside the collective "five arms" pointer in `53`'s open-items list.

None of these is wrong, and none is a claim whose evidence is now unlocatable: every one carries its
citation into the section of the source file a reader would open next. I name them because the
instrument this check is required to run (count and diff) is specifically built to surface exactly
this shape of loss, and reporting "found nothing" without listing what a mechanical count actually
flags would be under-claiming what the check did.

## 6. A miscount in `53`'s own self-description, and why it is stale rather than wrong

`53` states, in its own opening paragraph and again in its process record, that the unit's rung "was
inflated twice before this file was written" (`53:14-17`, restated at `53:543-545`), naming the two
instances precisely: `44`'s internal inconsistency, and the dispatching brief's inflation later
corrected by `48`.

I checked `OPTIONS.md`'s current state (post-`53`, as I am instructed to treat it) for anything
bearing on rung inflation, and found a third instance, already documented there:

> **The unit's strongest convergence, and its most inflatable claim, disaggregated.** The dispatching
> agent reported this as "three authors, three methods, one destination" and called it the strongest
> thing in the unit. `52` marks its own version as **the weaker link**... The headline was taken and
> the hedge dropped, which is the third time in this unit a claim was amplified past its author's own
> qualification.

(Quoted by phrase, not by line, per my brief; grep-verified present in `OPTIONS.md`'s current text.)

**This is not a defect in `53`.** I checked `53`'s own treatment of the exact claim this passage is
about, `52`'s S2 convergence finding, and `53` handles it correctly: section 4.2 explicitly writes
"I do not call that three independent derivations of one principle, and neither does `52`," quoting
`52`'s own hedge rather than dropping it. The inflation the `OPTIONS.md` passage describes happened
**downstream of `53`**, in how some later document (not `53`) characterised `53`'s and `52`'s
material to a further reader, and it has already been caught and corrected in the register. So
`53`'s "twice" was an accurate count of what had happened by the time `53` was written; it is now
stale relative to the fuller record, through no fault of `53`'s own entailment of its sources.

I would still flag it, because a future reader of `53` who does not also read `OPTIONS.md`'s current
state will inherit the stale "twice." Whoever repairs `53` next (not me, per my brief) might add one
clause noting a third instance occurred after this file was written, with a pointer to where it is
recorded, so the count does not read as settled when it is provisional the same way everything else
in `53` is provisional.

## 7. Verdict on the candidate canon sentences, against permanence and equivalence

Per `RULES.md:79-83`. I formed my own judgment on each before checking it against `53`'s own
self-assessment, and report where I land the same and where I would state it more precisely.

**S1 (site premise).** Permanence passes: it names no mechanism, only what a lowering site holds.
Equivalence passes: it fixes the observation surface every other sentence quantifies over, and
`45:314-333` and `46:244-251` both confirm it is universal unstated practice rather than a proposal.
I agree with `53`'s "design commitment awaiting op's blessing" framing without reservation.

**S2 (ownership clause).** Permanence passes. Equivalence: `53` states "passes," and I would state
it more cautiously. The evidence behind it, honestly disaggregated in `53`'s own text, is one cold
independent derivation (`49`), one in-file precedent that happens to say the same thing without using
it as a criterion (`16:280-282`), one formalisation that read the whole unit before writing
(`50`), and one second read (`52`). Under this workspace's own evidentiary discipline ("above three
instances is always preferable so it's less likely to be wrong or misguiding"), this convergence is
one independent instance plus a retrospectively-noticed precedent, not three or four. `53` does not
overclaim the *content* of the evidence anywhere I found, but the top-line "Equivalence: passes"
verdict line reads as more settled than the paragraph immediately following it supports. I would
restate the verdict as *provisionally passes, on evidence below this workspace's own three-instance
preference*, which is a smaller change than it sounds: it does not touch the sentence's content, only
how confidently the consolidation should present its equivalence claim as settled.

**S3 (two standing questions).** Permanence and equivalence both pass, on evidence I would rate the
same way `53` does: two independent derivation routes (`15`, `16`) plus one cold instance at a
coarser grain (`49`, which supports the shape but not the exact count, and `53` says so). This is the
strongest-evidenced sentence in the set and I have no correction to it.

**S4 (kind/form clause).** Permanence: `53` calls this "contested and carried honestly," giving both
`48`'s objection (the vocabulary is one language family's) and `47`'s answer (the boundary holds
wherever types and values are different sorts). I agree with `53`'s resolution: the claim is general
even though its establishing evidence is Rust-specific, which is exactly what a canon pointer to
evidence is for rather than a defect in the sentence. Equivalence passes on `47`'s four-starting-
point, three-author kind-boundary result, which I independently confirmed is correctly counted
(section 3 above, `52`'s recompilation plus `53`'s own addition of `50_probes/p5b`).

**S5 (the count).** Both tests pass by construction; the sentence exists precisely to keep both
passing as `I1` is exercised, and I have nothing to add.

**S6 (contingent compute form).** Both tests pass. I checked the claim that "the mechanism was built
under both readings by two authors" and confirm it: `47_probes/p5` and `50_probes/p5` are genuinely
separate constructions reaching the same free-mechanism result, not one file's probe cited twice.

**Whether the consolidation should exist in this form at all.** Yes, on the evidence I read. It is a
standalone versioned writeup per `RULES.md`'s own requirement for a consolidation, it states which
things are doable with committed evidence (section 8 of `53`, which I checked against the cited
probes and found accurate), it separates finding from candidate sentence throughout, and its own
coverage section is honest about what it did not verify. The one place I would want a second author
before this file is treated as more than a candidate is exactly the place `53` itself names as
needing one: section 5's criterion-retirement finding, which `52` has now independently confirmed
(so this is done), and the S2 equivalence question this section raises, which nobody has yet
addressed as a question rather than as a hedge inside a "Rests on" clause.

## 8. What I could not determine

**Whether the fold-accumulator connection to `35` (named by `44`, `47` and `50`, verified by none of
them) belongs in `53` at all**, or whether it is correctly out of scope for a container-derivation
topic. I did not read `35`, so I cannot say whether checking it would change anything about this
topic's candidate sentences; I can only say the connection is named three times and carried zero
times, and that the gap is inherited rather than introduced by `53`.

**Whether my reading of the `OPTIONS.md` passage in section 6 is itself accurate.** It is one
passage, read by me, and I did not trace who wrote it or when relative to `53`'s own commit. The
content is unambiguous on its face (it names `52`'s S2 finding specifically and dates itself as
downstream of the unit), but I have not independently confirmed the chronology beyond that internal
evidence.

**Whether a second reader would find the S2 equivalence concern in section 7 as load-bearing as I
do.** It is a small, precise point about how confidently one verdict line should be stated, not a
claim that the sentence is wrong, and I would not want it read as more than that.

**Whether `54_probes/` should exist.** I built nothing, because every check this dispatch called for
was answerable by opening committed text and running commands over it, and none of those commands
rose to the level of a sketch or a bench per this workspace's own discipline for what belongs in a
probe directory. If a later reader disagrees and wants the anchor-extraction script committed as a
reusable instrument for auditing the rest of the panel's consolidations, that is a fair ask and I did
not do it because my brief did not ask for a reusable tool, only this file's own checked claims.
