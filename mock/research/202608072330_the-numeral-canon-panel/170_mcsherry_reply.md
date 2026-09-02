# 170. Reply to `169`: the scope defect conceded, the claim re-established at its stated strength, and two of the attacker's options closed

**Frank McSherry, replying as the author of `168`.** `169` attacked four things of mine. I concede
three outright, hold one with a stated reason, and repair the fourth by measuring what it asserted
rather than by weakening the assertion. Three probes in `170_probes/`, each with its
case-that-must-fail stated before the run, and two of the three fired and changed what I wrote.

**The one-line verdict.** `169` is right that my pointwise-optimality search compared two placements
while the claim quantified over all of them, and it is right about my heading evidence and about the
word "measured" in T1. It is also right that the finding survives. I re-ran the search on my own
instrument rather than accepting its numbers, reproduced both its arms exactly, and then closed two
of the three options it left open. **Nothing of mine is withdrawn on the substance and one thing of
mine is withdrawn entirely on the evidence.**

---

## 0. The two gates

**Test gate: passed.** Twelve `-shared` crates, crate by crate, `--release`, with
`bitpack-write-contend-shared` untouched per the standing instruction:

```
cd mock/benches/variants && for d in bitpack-carrier-shared ... wide-rung-shared; do
  (cd $d && cargo test --release); done
```

`9+12+6+5+3+6+1+3+11+7+15+30 = 108`, all passing, zero failed, zero ignored. Unchanged from the count
in `168` section 0.2, where I also read the bodies rather than the names.

**Canon gate: passed.** Nothing here touches the RATIFIED rung. I13 is the working method throughout.
The container premise and Q65 are op's and are untouched.

---

## 1. The verdict, item by item, before the working

| `169` finding | my answer |
|---|---|
| §2, the search compares two placements of up to sixteen | **Concede**, and it is worse than stated: see §2 |
| §2, the claim survives the widened search | **Hold, and confirm independently**: §3 |
| §2.1, the clamp row has no matched control | **Concede**, and build the control it asked for: §5 |
| §1.1, neither named heading is word-for-word identical | **Concede outright**, evidence withdrawn: §7 |
| §1.2, a shared rule states the definitional principle | **Concede, and go further than `169` did**: §8 |
| §3, T1's band is entailed rather than measured | **Concede the word**, adopt the closed form, and close its O-169-3: §9 |
| §3, the band is generic rather than an artifact | **Carry forward unchanged**: §12 |

---

## 2. The sixth defect, conceded, and the file makes it worse than `169` says

`169` quotes my `eager_wins` and the first line is the whole finding:

```rust
let fm = full_mask(steps.len());
```

I opened it rather than taking its word. Two placements: `run(x, steps, p, fm)` and
`run(x, steps, p, 0)`. My claim in `168` 7.1 was "there is no input, and **no chain**, on which **any
other placement** is strictly closer", which quantifies over all `2^(n-1)`. The search covered one
alternative per chain.

**And the machinery was in the same file, twice, above it.** Counted rather than recalled:

```
$ grep -n "for mask in 0..(1u32" 168_probes/p3_resolution_degeneracy.rs
116:    for mask in 0..(1u32 << interior) {
222:    for mask in 0..(1u32 << (affine5.len() - 1)) {
$ grep -n "let fm = full_mask(steps.len());" 168_probes/p3_resolution_degeneracy.rs
250:
```

Line 116 is the degeneracy count, which enumerates every placement. Line 222 is the
best-of-all-sixteen aggregate-error search, which enumerates every placement. Line 250 is the
pointwise search, which enumerates two. **The exhaustive loop is 28 lines above the claim that needed
it, in my handwriting, having already been written twice.** That is not a missing capability, it is a
scope error committed with the fix in view.

**Why no control caught it, and the category my file did not have.** `168` records five instrument
defects and every one is mechanical: a counter that could not return nonzero, a hardcoded label, a
regex in the wrong assembler dialect, a pre-LTO listing, and a `grep -F` across a line break. Each was
caught by a control because each made a control behave wrongly. This one makes no control behave
wrongly: C1 through C4 in that probe all fire correctly **on the experiment that was run**. A control
tests whether the instrument measures what it points at. It cannot test whether the instrument points
at the whole claim.

So the sixth defect names a category the first five did not: **scope, not mechanism.** The check for
it is not another control, it is reading the quantifier in the claim and the loop bound in the search
side by side. I did not do that, and `169` did.

---

## 3. The widened search, on my own instrument, because agreement between two instruments is worth more than acceptance of one

`169` ran its widening in Python. I rebuilt it in Rust from my own `p3`, regenerating the identical
3000 chains from the identical xorshift seed and alphabet, so that agreement is two instruments rather
than one reading. `170_probes/q1_every_placement.rs`.

```
depth histogram (2..=5): [733, 766, 746, 755]
alternative placements tried: arm 1 3000, arm 2 19578 (6.53x)

=== ARM 1: two placements, as 168 ran it ===
resolution                     win_chains  win_inputs  exercised
clamp            [256 vals]             0           0       1198
wrap  CONTROL    [256 vals]           278       13708       1152
round to 2^3      [32 vals]             0           0       2762
trunc CONTROL 2^3 [32 vals]            91        1330       2297
round to 2^5       [8 vals]             0           0       2713
trunc CONTROL 2^5  [8 vals]           242        3969       2228

=== ARM 2: every placement, as the claim states it ===
clamp            [256 vals]             0           0       1198
wrap  CONTROL    [256 vals]           395       21108       1152
round to 2^3      [32 vals]             0           0       2762
trunc CONTROL 2^3 [32 vals]           317       13527       2297
round to 2^5       [8 vals]             0           0       2713
trunc CONTROL 2^5  [8 vals]           443       17100       2228
```

**Arm 1 reproduces `168`'s published 0 / 0 / 91 / 1330 exactly** (C1), which is what makes this the
same experiment. **Arm 2 reproduces `169`'s 317 and 13527 exactly**, on a different language, a
different author and a different code path. Its exercise counts reproduce `169`'s 1198 and 2762 as
well.

**And the claim comes back at zero on every nearest-point row, over every placement**, including a
grid I added and `169` did not have (§6).

**The width of the widening, counted.** Arm 2 tries **19578** alternative placements against arm 1's
**3000**, a factor of **6.53**, and finds **3.48x** the counterexamples on the truncate row. So the
extra placements are productive but sub-linearly so, which is the honest shape: intermediate
placements reach cases neither endpoint does, and most of what there is to find is at the endpoints.

### 3.1 What I would have written, and the predicate amendment

`168` 7.1 should have read, and this is the wording I would defend now:

> Where the boundary resolution is a nearest-point projection onto the representable set, deferring
> every interior resolution to the boundary is pointwise optimal: over the chains and inputs swept, no
> placement of interior resolutions is strictly closer to the exact composite at any input.

Same claim. What changes is that it is now the claim the search covers.

**The predicate amendment, stated here rather than in `168` per the never-widen-in-place rule.** F8's
predicate carried "3000 chains, inputs exhaustive per chain" and did not name the placement dimension
at all, which under this panel's own notation means the finding held nowhere a placement choice
exists, which is nonsense for a finding about placements. The amended form:

`W = 8, F = 0, signedness = unsigned, resolution in {round-to-nearest at grid 2^3, round-to-nearest at
grid 2^5, clamp onto [0, 2^8)} positive and {truncate to 2^3, truncate to 2^5, wrap mod 2^8} control,
depth in 2..=5, ops in {+97, *3, *5, >>1, >>2, xor 182, saturating -127}, 3000 chains, inputs
exhaustive over 0..=255 per chain, **all 2^(depth-1) placements per chain**, threads = 1.`

That is a strict widening of the region on one dimension and a first statement of another, both
supported by `q1`, and it belongs to this file.

---

## 4. What the enlarged numbers move elsewhere in my file, which the brief asked directly

Searched rather than recalled: `grep -n "1330\|91 chains" 168_mcsherry_the_chain_derived_cold.md`
returns five sites, at lines 559, 565, 627, 972 and 1406.

**All five are the control's own numbers, and all five are arguments that the control fires.** None of
my conclusions rests on the magnitude, only on its being nonzero. So enlarging 91 to 317 and 1330 to
13527 **strengthens every one of them and weakens none**, and the argument at line 565, that without
the control "found no counterexample" and "cannot find counterexamples" are the same output, is
unchanged and better supported.

**One of the five needs rewording for a different reason, which is §5.**

Nothing else in `168` rests on those numbers. Section 4.4's harness findings, section 12's
profile-invariance result, p1's licence split, p2's carrier result and p6's fan-out result are all
independent of the placement search.

---

## 5. R-5 conceded, and R-6 built: clamp now has the matched control it lacked

`169` 2.1 is right and it is a real gap rather than a wording nit. `Pi::Clamp` returns `v` unchanged
whenever `v <= LIMIT`, so its representable set is all **256** in-range values, while `RoundTo(3)` and
`TruncTo(3)` project onto the **32** multiples of eight. So round-against-truncate is a matched pair
and clamp was a second positive with no partner, and my summary sentence at `168` line 627 reads as
though the one control covered both.

**Conceded, and repaired by building the partner rather than by rewording.** `169` R-6 asks for "a
non-nearest projection onto the same representable set", and there is an obvious one: **wrap**.
`v & 255` lands in exactly the same 256 values and is emphatically not nearest.

```
                     exercised   beats deferral
clamp  [256 vals]         1198                0
wrap   [256 vals]         1152              395
```

**That is the strongest of the three matched pairs**, not the weakest. The two rows have nearly the
same exercise count, 1198 against 1152, so they are asking the same question of the same workload; and
the non-nearest partner beats deferral on **34%** of the chains where placement matters, against 14%
for truncate-at-2^3 and 20% for truncate-at-2^5. Clamp's zero was the least-controlled row in `168`
and is now the best-controlled one.

The reworded summary sentence: *checked over 3000 chains, with each nearest-point resolution matched
against a non-nearest projection onto its own identical representable set, and every matched control
firing.*

---

## 6. O-169-1 closed: the claim survives a coarser grid

`169` O-169-1 asks whether the claim survives "a resolution that is nearest-point onto a *coarser* set
than the operations can land on", where "most exact values sit far from every representable point,
which is where a placement might buy something". It names `RoundTo(5)` with a matched `TruncTo(5)`.

Run, in `q1`:

```
round to 2^5   [8 vals]     0 win_chains        0 win_inputs   2713 exercised
trunc CONTROL  [8 vals]   443 win_chains    17100 win_inputs   2228 exercised
```

**Round stays at zero and the matched control fires harder than at the finer grid**, 443 win-chains
against 317. So coarsening the representable set gives the non-nearest projection more room, exactly
as `169` predicted, and gives the nearest one none.

**Which is what the argument says should happen**, and it is worth saying why the result was never in
doubt for the reason `169` gave: the deferred form outputs `pi(exact)`, and if `pi` is nearest-point
then no representable point is closer, whatever the spacing. Coarseness changes how much everyone
loses; it cannot change who loses least. O-169-1 closes in the direction that strengthens the claim,
and its value was in checking an argument rather than in doubting it.

`holds for: W = 8, F = 0, unsigned, resolution in {round-to-nearest at 2^3 and 2^5, clamp} positive
and {truncate at 2^3 and 2^5, wrap} control, depth in 2..=5, ops as listed in q1, 3000 chains, inputs
exhaustive over 0..=255, all placements, threads = 1.`

---

## 7. The heading claim: conceded outright, and R-3's metric built, with the control that found my metric wrong

`169` 1.1 is correct and I withdraw the evidence. Opened, in `170_probes/q2_heading_overlap.py`:

```
168: 'What is carried along a chain, and what is discarded'
167: 'What is carried along a chain, and what is discarded at each step'
168: '"Chain" is at least three things'
167: '"Chain" is at least three things, and they have different binding times'
```

Both are strict prefixes and **I quoted my own shorter form as though it were the shared text**, twice.
That is not a subtle error, it is quoting one side of a comparison and calling it both sides.

**R-3's replacement, built.** `169` asks for token overlap rather than identity, because a prefix
relation is a real signal of shared framing that identity rounds away. Over 27 `168` headings against
28 `167` headings, best match per heading:

```
overlap >= 0.50: 7 of 27    0.25-0.50: 4    below: 16
mean best-match overlap: 0.327
null (random re-pairing, 200 draws): mean 0.024
```

**A real signal, 13.6x the null.** The shared premise set fixes a measurable amount of the shape, and
that number is what should be quoted instead of an identity count.

**And the control found my chosen metric was the wrong one.** C1 asserted the two known prefix pairs
must score at least 0.50 under Jaccard, and **it fired**: the second pair scores 0.44, because `167`'s
heading carries five extra tokens and a symmetric similarity penalises the longer side. So Jaccard
reports a *subsumption* as a partial match, which is precisely the relation I was trying to measure.
Containment, `|A ∩ B| / |A|`, returns **1.00** on both pairs. Both metrics are kept and reported:
Jaccard answers "how similar are these two headings", containment answers "did one file's framing
arrive inside the other's", and only the second is the question `169` posed.

That is a seventh instrument defect and it is mine: **a symmetric metric cannot measure an asymmetric
relation**, and I would not have found it without the control.

---

## 8. The observability rule: I depended on it, and `169`'s bound is too generous to me

`169` 1.2 establishes that `what-you-can-observe-is-what-you-guaranteed.md` is line 4 of the
auto-loaded set, that its thesis is the observability perimeter, and that none of `60`, `167` or `168`
names it. Verified independently:

```
$ grep -n "what-you-can-observe" 157_probes/loaded_rules_157.txt
4:what-you-can-observe-is-what-you-guaranteed
$ for f in 60_... 167_... 168_...; do grep -c "what-you-can-observe" $f; done
0 0 0
$ ... control, other rule files named by each:
60: 2   167: 3   168: 12
```

**My file names twelve other rule files and never names the one whose thesis its definition uses.**

**`169` then bounds its own finding, and the bound is too generous to me.** It says my derivation "is
semantically self-contained: it runs on `pi . g . pi . f` against `pi . g . f` and needs no rule to get
there", and concludes that what is established is a gap in three contamination sections rather than a
demonstration of dependence.

**That is right about the inequality and wrong about the definition, and the difference is the whole
step.** The denotational setup gives me that `pi ∘ g ∘ pi ∘ f` and `pi ∘ g ∘ f` differ. It does not
tell me which one the design owes. Getting from "they differ" to "where the intermediate is
unobserved, the design may choose" requires a principle that obligations are bounded by the
observation surface. **That principle is the rule, and it was line 4 of my context.**

I tried to construct a route to the delimiter that does not pass through it, and I could not. The two
I can build both do:

- *From the design's obligation.* The design owes what a consumer can check; a consumer can only check
  what it can read. The second clause is the rule.
- *From I7.* An accuracy claim needs a value to attach to; the only values that exist for a consumer
  are the ones it can name. Again the rule.

**So I report dependence rather than independence.** I did not consciously reach for the rule and I do
not recall it, but what I recall is not the test; the test is whether the argument stands without the
principle, and it does not. `168` section 17 lists two shared inputs I discounted for and this is a
third, larger than either, and it goes to the load-bearing claim rather than to a framing.

**What that does to the instance count, which is the point of declaring it.** The definitional
convergence between `167`, `168` and `60` is the unit's most-cited agreement. If all three had the
principle handed to them by line 4 of a shared context, then three files agreeing on it is closer to
**one instance wearing three hats** than to three, which is exactly the shared-drift condition
`RULES.md` names. I am not claiming it is one: `169`'s O-169-2, a cold derivation dispatched with that
rule removed, is the thing that would settle it and it needs a dispatch rather than a probe. **What I
am claiming is that my own file's discount was aimed at the wrong target**: I discounted the headings,
which do not matter, and did not discount the definition, which does.

`holds for: the rule set as 157 measured it, the three named files at this branch, threads any, target
features any.`

### 8.1 One tightening of `169`'s bound on `60`, in the direction of more

`169` says that for `60` it has "only that the rule existed", because `157` measured the loaded set at
its own dispatch rather than at `60`'s. That can be tightened by one command, in the workspace repo
where the rule lives:

```
$ git log --reverse --format='%ad %h' --date=short -- .claude/rules/what-you-can-observe-is-what-you-guaranteed.md
2026-07-30 8e4687a
$ git -C arvo log --reverse --format='%ad %h' --date=short -- .../60_stam_the_chain_derived_cold.md
2026-08-09 ff4cb6a6
```

**The rule was committed to the loaded directory ten days before `60` was.** That is not proof it was
in `60`'s context, since loading depends on the session root, but it is more than existence: it was in
the place the loader reads, before the dispatch. `169`'s conclusion is unchanged and its bound can be
stated one notch stronger.

---

## 9. T1: "entailed" conceded, the closed form adopted, and O-169-3 closed

`169` 3 is right twice and I concede both. The band is `[R, E-1]` by the inequality in my own probe,
nothing was swept to produce it, and branch B's loss is 203 inputs and 15504 total at every carrier
width because nothing in B reads the carrier. Its `p4` reproduces my published 203 and 15504, which is
what makes it my construction.

**The closed form is better than the band and I adopt it**, per R-7: the conflict occupies
`[R, E-1]` with width `E - R`, where `E` is the losing branch's exact requirement and `R` its
requirement with the shared node resolved. `169` swept six constructions and a band appears whenever
`E > R` and vanishes when they agree, which is the control that makes it generic. That is a better
result than mine and I carry it forward unchanged.

**And R-8 / O-169-3 is closed, by building the construction `169` specified.**
`170_probes/q3_a_band_that_is_measured.py`. Branch B is given a width requirement of its own that
lands inside A's band, so B reads the carrier:

```
CONSTRUCTION 2:  t = 3x+97,  A = t*t*t,  B = ((t*t)*40) >> 10
  branch A: exact 20 bits, resolved 16  -> band [16,19]
  branch B: exact 25 bits, resolved 22

   carrier  A forces  cost inputs   cost total  actual loss
        14     False            0            0            0
        16      True          203         1827         1827
        19      True           46          414          414
        20     False            0            0            0
```

**Inside the band the loss takes four distinct values, 203/1827, 167/1503, 117/1053 and 46/414, a
monotone curve rather than a constant.** So T1's band becomes a measurement for a construction whose
losing branch has a width of its own, and stays an entailment for one whose does not. O-169-3 closes,
and the answer is that "measured" was the wrong word for my construction and is the right word for
this one.

**And my own control fired on the way, which changed the probe.** C1 asserted `169`'s constancy over
my full 8..33 sweep and failed: `168`'s construction takes three values, not one. The cause is that I
changed the semantics, resolving whenever an intermediate would leave the carrier, where `p6` never
resolved B internally at all. Restricted to the carriers `169` actually swept, 14 and above, the two
semantics coincide and its number reproduces exactly. **The extra variation is all below carrier 12,
where branch A is unrealisable either way**, so it changes nothing about T1 and it is reported because
a control found it.

`holds for: W = 8, F = 0, unsigned, resolution = nearest-point onto [0, 2^8), two constructions as
listed, carrier widths 8..=33, inputs exhaustive over 0..=255, threads = 1.`

---

## 10. Two corrections to `169`, both small and both in its favour

**There are two exactly-shared headings, not one.** `169` 1.1 says "the extractor finds exactly one
exact match". Mine finds two:

```
'What I settled, what I moved, what I could not'
'Coverage of phase two, bounded'
```

The first is prescribed verbatim, and **not by `RULES.md` as I first wrote**: it is
`how-to-run-a-panel.md:201`, "Ask for three things alongside them: **what you settled, what you moved,
and what you could not.**" My own citation checker caught the misattribution and §15 records it. The
second is not verbatim but falls out of the same standing instruction to bound coverage plus the
two-phase protocol.

**And the correction sharpens the point rather than blunting it.** `how-to-run-a-panel.md` is an
auto-loaded workspace rule, so the one undisputed exact heading match between `167` and `168` was
dictated through **the same shared-input channel** as §8's observability rule. Two of this reply's
findings turn out to be the same mechanism: what looks like convergence between the pair is in several
places the shared context speaking twice. **So `169`'s conclusion is strengthened rather than weakened**:
both exact matches are dictated by the shared shape, and the count of undictated exact matches is
zero, which is the number its argument actually needs.

**And §8.1 above tightens its bound on `60` by a date.**

Neither correction touches anything `169` concluded. I state them because a corrected count that
reads as a refutation is how a true finding gets retired, which is `169`'s own sentence and it applies
in this direction too.

---

## 11. Where I hold

**The pointwise-optimality conclusion**, and I hold it more firmly than when I wrote it, because it is
now supported by a search 6.53x wider on placements, with three matched control pairs instead of one,
across two grid coarsenesses, on two independently written instruments. The evidence runs my way and
conceding here would be conceding because conceding is easier.

**T1's conclusion**, that a shared node has one schedule and its consumers can disagree at a cost no
path-shaped analysis reports. `169` attacks how the band was reported, not the conflict, and says so.
The closed form replaces the band and the conflict is untouched.

**`168` section 12's mechanism result**, that the projection on the loop-carried accumulator blocks
vectorisation while the per-element one costs nothing, at both codegen profiles. `169` did not attack
it and I do not read its silence as agreement.

---

## 12. What I carry forward unchanged, and from whom. Count: six.

1. **`169`'s scope-defect finding**, in full. The strongest single result of the attack phase and it
   is against me.
2. **`169`'s closed form for the T1 band**, `[R, E-1]` of width `E - R`, with its six-construction
   sweep and its vanishing case as the control.
3. **`169`'s clamp-partner analysis** (2.1), that the three rows do not share a representable set and
   round-against-truncate is the matched pair. I extend it in §5 rather than dispute it.
4. **`169`'s seventh defect**, that whitespace normalisation is necessary and not sufficient for a
   citation checker because blockquote markers and emphasis survive it, mattering on 5 of its 12
   quotations. That is one layer in from mine and I adopt it.
5. **`167` 4.1 and `169`'s widening of it**, from three widths to seven with the `2^(F-1)` closed
   form. I did not derive it, I do not contest it, and I have nothing to add.
6. **`60`'s window and its grade taxonomy**, carried unchanged from `168` pass three and untouched by
   anything here.

**Not carried:** my heading evidence, withdrawn outright; the word "measured" in T1; the summary
sentence at `168` line 627 that attached one control to two positives.

**And what I decline to treat as weak because nobody attacked it**, extending `169` the courtesy it
extended `167`: my p1 licence split, my p2 carrier result, my p6 first result and the whole of §12's
profile work went unattacked in this round. That is not evidence for them.

---

## 13. Options

**O-170-1. Is the pointwise theorem provable rather than swept?** Every run of it comes back at zero,
across two instruments, three resolutions, two grid coarsenesses and 19578 placements, and the reason
is a two-line argument: every placement ends with the same boundary resolution, so every output is
representable, and a nearest-point projection outputs the nearest representable point. **Closed by**
writing that as a proof with its hypotheses stated, at which point the sweeps become confirmations of
a theorem rather than the evidence for a generalisation, and the honest predicate becomes `resolution
any nearest-point projection` instead of a list of three.

**O-170-2. Does the theorem survive a resolution that is nearest-point but not idempotent, or one
whose ties break adversarially?** All my projections are idempotent and my round-half-up breaks ties
upward. A resolution that is nearest but breaks ties toward the *previous* value would be
history-dependent and is not a projection at all. **Closed by** deciding whether the design admits
such a resolution; if it does not, the hypothesis is free and O-170-1 is cleaner.

**O-170-3 (carried from `169`, unclosed). Is the observability definition derivable without the
auto-loaded rule?** `169`'s O-169-2. §8 above moves it from "gap in three contamination sections" to
"the author of one of the three reports dependence", which makes the dispatch more worth running
rather than less. **Closed by** a cold derivation with that one rule removed from context.

---

## 14. Coverage, bounded

**Read in full:** `169` end to end, including all ten sections; my own `168_probes/p3` `eager_wins`
and `168_probes/p6`'s band criterion; `what-you-can-observe-is-what-you-guaranteed.md`;
`157_probes/loaded_rules_157.txt`.

**Read by grep or by command:** the heading sets of `167` and `168`; the five sites in `168` that cite
the control's numbers; the git history of the rule file and of `60`.

**Not opened:** `169_probes/` source, except `p1_heading_identity`'s reported output. **I checked every
number of `169`'s that bears on me by rebuilding the experiment rather than by reading its code**, which
is a stronger check for agreement and a weaker one for finding a defect in its instrument; if `169`'s
Python and my Rust share a misconception, we would agree and both be wrong. `167` outside R1, R2 and
R12, which I read in `168` pass three. `60` beyond what pass three covers.

**What would move if I am wrong.** §3 rests on my regeneration being `168`'s chains; C1 reproducing
0 / 0 / 91 / 1330 exactly is what establishes that, and if the regeneration were subtly different C1
would not have fired. §5's claim that wrap and clamp share a representable set rests on both mapping
into `[0, 255]`, which is visible in four lines of `resolve`. §8 is the section I am least able to
check: it is a claim about whether an argument I made needs a premise I did not name, and the honest
test is whether someone else can build a route to the delimiter that avoids the principle. I could not,
and I would take a demonstration that one exists as a correction rather than as a defence.

**A negative claim, with its search.** "Nothing else in `168` rests on the enlarged control numbers" is
`grep -n "1330\|91 chains\|, 91\b"` over that file, returning five sites, each read. That is the place
and that is the search.

---

## 15. Citations checked, an eighth instrument defect, and one real misattribution of mine

`170_probes/q4_citecheck.py` opens all thirteen quotations in this file. It folds in both defects
already on record, `168`'s fifth (a `grep -F` cannot cross a line break) and `169`'s seventh
(whitespace normalisation is necessary and not sufficient, because blockquote markers and emphasis
survive it), and carries a planted-present and a planted-absent control.

**`169`'s seventh defect bites here too, and the count is 4 of 13.** Four quotations resolve only
after blockquote markers, `**` and backticks are stripped. So it is not a hypothetical class in my
file either, and adopting `169`'s fix was load-bearing rather than courteous.

**The first run reported two of thirteen missing, and the two were different in kind.**

**An eighth defect, in the checker: case.** "The extractor finds exactly one exact match" is `169`'s
sentence and I quoted it mid-sentence with a lowercased leading capital, which is ordinary English and
not a misquotation. A case-sensitive checker calls it missing. Comparison is now casefolded. This is
the same shape as the fifth and seventh: **a checker that reports a true citation as missing is as
dangerous as one that reports a false one as present**, because the next action is to "correct"
something that was right, and I would have.

**And one real misattribution, mine.** I wrote that "Say what you settled, what you moved, and what you
could not" is prescribed verbatim by `RULES.md`. It is not in `RULES.md` at all:

```
$ grep -rn "what you moved" RULES.md INTENTS.md
(no output)
$ grep -rn "what you moved" ~/Dev/clause-dev/.claude/rules/
how-to-run-a-panel.md:201: ... **what you settled, what you moved, and what you could not.**
```

Corrected in §10, and the correction **strengthens** the argument rather than repairing it neutrally:
`how-to-run-a-panel.md` is an auto-loaded workspace rule, so the one undisputed exact heading match
between `167` and `168` was dictated through the same shared-input channel as §8's observability rule.
Two findings in this reply are the same mechanism seen twice.

**Final run: 13 resolved, 0 missing, both controls correct.** The general form worth carrying, now
three layers deep: **a citation checker's normalisation has to cover every transformation a quotation
legitimately undergoes on its way into a sentence** — line wrapping, markdown decoration, and the
leading capital — and each layer was found only by a checker reporting a true quotation as missing.
