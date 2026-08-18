# 169. The chain attacked: the sixth instrument defect, and a claim that survives being tested properly

Kiselyov. Attacking `167`, `168` and `60` at the tenth unit's attack phase. Five probes in
`169_probes/`, each with its case-that-must-fail stated before the run and shown failing.

**The headline, because it is not what an attack usually produces.** I found the sixth instrument
defect, and it is in the strongest measurement of the unit: `168`'s pointwise-optimality search
compares **two** placements while its stated claim quantifies over **all** of them. I widened the
search to every placement. The control moves from 91 win-chains to 317, so the gap was real rather
than theoretical, **and the positive claim comes back at zero on both nearest-point resolutions.**
The finding is not refuted. It is now supported at the strength it actually asserts.

Four further results, one per section: neither heading `168` names as word-for-word identical is
identical; a shared auto-loaded rule states the definitional principle both cold derivations reached
and none of the three declares it; the contest band is entailed by an inequality rather than
measured, and is generic rather than an artifact; and `167` 4.1 is a proof recorded as a sweep with
a closed form behind it.

---

## 0. The two gates

**Test gate: passed.** Thirteen `-shared` crates, crate by crate, `--release`, with
`bitpack-write-contend-shared` serialised and otherwise untouched per the standing instruction:

```
9+12+6+5+3+6+1+3+11+7+15+30 = 108   (twelve crates)
bitpack-write-contend-shared, -- --test-threads=1  = 15
                                             total  123, all passing
```

`bitpack-write-contend-shared` terminated, which is `168`'s point about unserialised records being
records of the runner rather than of the crate.

**Canon gate: passed.** Nothing below touches the RATIFIED rung; I13 is the working method
throughout and is argued with nowhere. The container premise, Q65's marker question and X1 through
X4 are op's and are not resolved here. Where a finding of mine bears on one, it says which branch it
holds under rather than picking one.

---

## 1. The agreement, which is the weakest-looking strong thing and is weaker in one place than stated

### 1.1 Neither heading `168` names as word-for-word identical is identical

`168` section 23 discounts the pair's agreement to "three independent routes plus a corroborating
pair", and supports that with:

> **Two of our section headings are word-for-word the same** ("What is carried along a chain, and
> what is discarded at each step"; "'Chain' is at least three things").

Both places opened, `169_probes/p1_heading_identity.sh`:

```
claimed shared: What is carried along a chain, and what is discarded
  167: 3. What is carried along a chain, and what is discarded at each step
  168: 3. What is carried along a chain, and what is discarded
claimed shared: "Chain" is at least three things
  167: 5. "Chain" is at least three things, and they have different binding times
  168: 5. "Chain" is at least three things
```

**In both cases `168`'s heading is a strict prefix of `167`'s, and `168` quoted its own shorter form
as though it were the shared text.** Neither pair is word-for-word identical.

And the one heading that *is* exactly shared across the two files is
`What I settled, what I moved, what I could not`, which the standing rules prescribe in those words,
so it is **zero evidence** of convergence. The extractor finds exactly one exact match and twelve
`167` headings with no counterpart at all, which is the control that keeps this from being a
comparison that matches everything.

**What this does and does not do to the discount.** The discount's real ground is the shared premise
set, which is true and which `168` states independently. That survives untouched. What falls is the
corroborating evidence offered for it, in both directions at once: the two matches quoted are not
matches, and the one real match is dictated. **I sign the discount and refuse its stated evidence.**

`holds for: the two files as committed at this branch, headings extracted by the pattern in
p1_heading_identity.sh, threads any, target features any.`

### 1.2 A shared auto-loaded rule states the definitional principle, and none of the three declares it

`168` asks how much of the shape a shared premise set fixes, and answers it about headings. The
load-bearing convergence is the **definition**, and nobody asked that question about it.

`what-you-can-observe-is-what-you-guaranteed.md` is **line 4** of the auto-loaded rule set that
`157` measured in this same panel (`157_probes/loaded_rules_157.txt`), and its thesis is:

> A guarantee about a type holds only over the operations through which the type can be observed.

That is the observability-perimeter principle, one tier up from a chain. Both cold derivations
produced a definition whose boundary is observation. **None of `167`, `168` or `60` names the rule**
(`169_probes/p2_what_fixed_the_definition.sh`, zero occurrences in each), while all three cite other
workspace rules by name, which is the control that makes the zero a fact about these files rather
than about my pattern.

**Bounded, because this is easy to overstate and I am not going to.** `168`'s derivation is
semantically self-contained: it runs on `pi . g . pi . f` against `pi . g . f` and needs no rule to
get there. What is established is that the rule was in every context and was declared by nobody,
which is a gap in three contamination sections, **not** a demonstration that anyone used it. And for
`60` I claim less still: `157` measured the loaded set at its own dispatch rather than at `60`'s, so
for `60` I have only that the rule existed.

`holds for: the rule set as 157 measured it, the three named files at this branch, threads any,
target features any.`

### 1.3 The definitional split is one claim in two vocabularies, and that is the right reading

`167`: "the **unobserved region**: a maximal stretch of a computation in which no intermediate is
named by anyone outside it. Its boundary is the act of observation rather than the operator."

`168`: "A chain is a **maximal run of operations whose intermediates are not observable**. Its
boundary is where a value becomes observable."

Both are maximal, both are bounded by observation, both explicitly refuse a syntactic boundary.
`167` retains "region" and says "chain" is the wrong unit; `168` retains "chain" and admits a
length-one case. **The vocabulary differs and the claim does not.**

So the panel should record **one finding with two instances**, never two findings. I checked the
obvious inflation and did not find it: neither file counts the other's definition as a separate
result, because neither had read the other. The risk is downstream, in whatever compresses this
unit, and section 1.2 is the reason the two instances should be discounted further rather than
treated as clean.

---

## 2. The sixth instrument defect, and the claim survives it

Five instrument defects are recorded in this unit, all caught by controls. Here is the sixth, and it
is a **scope** defect rather than a mechanical one, which is why no control caught it: every control
in the probe fired correctly on the experiment that was actually run.

`168` 7.1 states:

> Where the boundary resolution is a nearest-point projection onto the representable set, deferring
> every interior resolution to the boundary is pointwise optimal. **There is no input, and no chain,
> on which any other placement is strictly closer** to the exact composite.

"any other placement" quantifies over all `2^(n-1)` interior placements of a depth-`n` chain. The
search is `eager_wins` in `168_probes/p3_resolution_degeneracy.rs`, and its first line is:

```rust
let fm = full_mask(steps.len());
```

It compares **fully eager against fully deferred**. Two placements, of up to sixteen.

`169_probes/p3_is_the_clamp_row_exercised.py` runs both searches over `168`'s own chains,
regenerated from its xorshift seed and alphabet:

```
=== ARM 1: two placements, as 168 ran it ===
nearest (round to 2^3)               0           0                     2737
nearest (clamp)                      0           0                     1183
NOT nearest (truncate)              91        1330                     2296

=== ARM 2: every placement, as the claim states it ===
nearest (round to 2^3)               0           0                     2762
nearest (clamp)                      0           0                     1198
NOT nearest (truncate)             317       13527                     2297

C1 arm 1 reproduces 168's 0 / 0 / 91        : True
C2 truncate wins in both arms               : True   (91 narrow, 317 full)
C3 widening finds strictly more             : True   (317 > 91)
```

Arm 1 reproduces `168`'s published `0 / 0 / 91` and its 1330 winning inputs exactly, which is what
says this is their experiment and not a neighbouring one.

**The gap is real rather than theoretical.** Widening takes the control from 91 win-chains to 317
and from 1330 winning inputs to 13527, so intermediate placements reach cases neither endpoint does.
An instrument that misses 71% of the counterexamples it *can* find is thinner than the claim it was
supporting.

**And the claim holds anyway.** Round comes back at 0 win-chains over every placement, and clamp at
0. `168` was right, and it was right for the reason it gave; what it had was one placement's worth
of evidence for a claim about all of them, and now it has all of them.

`holds for: W = 8, F = 0, unsigned, resolution in {round-to-nearest at grid 2^3, clamp, truncate to
2^3}, depth in 2..=5, ops in {+97, *3, *5, >>1, >>2, xor 182, saturating -127}, the 3000 chains
168's seed generates, inputs exhaustive over 0..=255, all 2^n placements per chain, threads = 1.`

### 2.1 And the clamp row is a second positive with no matched negative

The same run answers the brief's comparability question. The three rows do not have the same
representable set. `RoundTo(3)` and `TruncTo(3)` both project onto the **32 multiples of 8**,
differing in nothing but nearest against not: that is a properly matched pair and it is the
experiment that carries the claim.

`Pi::Clamp` returns `v` unchanged whenever `v <= LIMIT`, so its representable set is **all 256
in-range values**. It is a nearest-point projection, so the positive claim covers it correctly; it is
simply not the truncate control's partner. And it is exercised less: placement changes the output on
**1198** of 3000 chains under clamp against **2762** under round. Both report zero and they are not
equally strong zeros.

`168` does not claim clamp is the control. Its summary sentence, "checked over 3000 chains with a
control that finds 1330 counterexamples", reads as though the control covers both positives, and it
covers one. That is a wording repair rather than a defect in the work.

---

## 3. The bounded contest is entailed rather than measured, and it is generic rather than an artifact

`168` T1 bounds its contest against `60` to "carrier widths **16 through 19** on this construction",
and says it "measured after reading `60` rather than asserting".

**Read the criterion first.** `168_probes/p6_a_fanout_forces_one_schedule.rs`:

```rust
let a_exact_fits    = wa          <= cap_bits;
let a_resolved_fits = wa_resolved <= cap_bits;
if !a_exact_fits && a_resolved_fits { conflict.push(c) }
```

With `wa = 20` and `wa_resolved = 16`, the band is `[16, 19]` **by arithmetic**. It is the interval
`[resolved_need, exact_need - 1]`, entailed by that inequality. Nothing is swept.

`169_probes/p4_is_the_band_measured.py`, reproducing `168`'s construction exactly (its control C2
requires the published 203 inputs and 15504 total at width 16, and gets them):

```
 carrier  in band  inputs worse  total extra |err|
      14    False           203              15504
      16     True           203              15504
      19     True           203              15504
      22    False           203              15504
```

**Branch B's loss is the same number at every carrier width**, inside the band and outside it, because
nothing in B's computation reads the carrier. So the one thing measured in the band does not vary
across it, and "the conflict exists at 16 through 19" is two observed widths subtracted plus a
measurement that is constant in the variable being bounded.

**But it is not an artifact of the construction, and this is the half that goes the other way.**
Sweeping six constructions:

```
construction                          exact  resolved       band  width
168: t=3x+97, A=t*t, B=t>>2              20        16    [16,19]      4
A=t*t, t=x+97 (smaller node)             17        16    [16,16]      1
A=t*3 (linear branch)                    12        10    [10,11]      2
A=t*t*t (cubic branch)                   30        24    [24,29]      6
A=t>>1 (contracting branch)               9         7      [7,8]      2
t=x (identity node), A=t*t               16        16      empty      0
```

A band appears whenever the exact form needs more bits than the resolved one, and vanishes exactly
when they agree, which is the control that keeps "generic" from being unfalsifiable. **The bound is
right and it has a closed form: the band is `[R, E-1]` and its width is `E - R`.** What is wrong is
one word in how it was reported.

`holds for: W = 8, F = 0, unsigned, resolution = clamp onto [0, 2^8), the six constructions listed,
inputs exhaustive over 0..=255, carrier widths 8..=32, threads = 1.`

---

## 4. `167` 4.1 survives, and it is a proof recorded as a sweep with a closed form behind it

The brief names three `167` survivors that could not have come from reading `60`. I attacked the one
whose shape is most likely to be thin, the intermediate-width sweep, and **it holds and is stronger
than recorded.**

`167` 4.1 sweeps `F in {6, 8, 10}` and concludes "there is no `M` strictly between `F` and `2F` with
zero disagreements, at any `F` tested". The predicate is honest, the hedge is correct, and two
controls are declared. Beside it sits an argument: the exact product needs exactly `2F` fraction
bits and the operand set is already full, so the float threshold theorem's slack has nowhere to
live.

**That argument is a proof, and the finding is recorded with a measured predicate.** This is exactly
`157` F157-10's class, and under a notation where a predicate is never widened in place it
under-claims permanently unless a later file states the widening. `169_probes/p5_double_rounding_closed_form.py`:

```
  F  M=2F (control)  M=F (control)    M=2F-1   2^(F-1)  match
  4               0              0         8         8   True
  6               0              0        32        32   True
  8               0              0       128       128   True
 10               0              0       512       512   True

C1 both of 167's controls hold at every F : True
C2 reproduces 167's 32 / 128 / 512        : True
C3 a zero strictly inside would refute    : True (none found)
```

Two results. The `M = 2F-1` column is exactly `2^(F-1)` at every `F` from 4 to 10, which is a
**closed form rather than a trend**; and no `M` strictly between `F` and `2F` gives zero at any of
the seven widths, extending `167`'s three to seven.

**The widening, stated here rather than in `167`'s file**, per `RULES.md`'s never-widen-in-place
rule: 4.1's conclusion holds for **`F any`** on the argument, because the exact product of two Q(.F)
numerals occupies exactly `2F` fraction bits, so any `M < 2F` discards a nonzero low part for some
operand pair and some such pair lands on a rounding boundary. Nothing in that depends on `F`. The
enumerative half stays at `F in {6, 8, 10}` where `167` recorded it, and this file's enumeration is
`F in 4..=10`.

`holds for: F in 4..=10 enumerated here, M in [F, 2F], rounding = nearest-ties-to-even at both
roundings, operation = fixed-point multiply, unsigned, operands exhaustive over [0, 2^F), threads =
1. The argument half holds for F any.`

**Which of my instruments could have refuted this and did not.** The closed-form check would have
failed at any `F` where the count deviated from `2^(F-1)`, and the inside-sweep would have reported
a zero if one existed at any of 49 `(F, M)` pairs strictly inside. Neither fired. I did not test
signed operands, nonzero integer parts, or any operation but multiply, and 4.1 claims none of those.

---

## 5. Replacements owed, addressed to the parties refuted

Per `113`, several each, so the author has material rather than a verdict.

### To `168`, on section 23

**R-1. Replace the heading evidence with the premise-set argument you already have, which is
stronger and true.** The discount does not need the headings; it needs "two files from one premise
set converging is stronger than one file and weaker than two instances", which is your own sentence
and stands on its own.

**R-2. Put section 1.2's rule in your contamination section, and discount the *definition* rather
than the headings.** The definitional convergence is the load-bearing one and it is the one your
accounting does not reach. My bound applies: this is an undeclared shared input, not a demonstrated
dependence.

**R-3. If you want a heading measurement, measure token overlap rather than identity.** The two
pairs are prefix relations, which is a real signal of shared framing and is not identity, and a
metric that reports the difference is more useful than one that rounds it to "word-for-word".

### To `168`, on section 7.1

**R-4. Re-run `eager_wins` over `(1 << n) - 1` masks rather than `full_mask` alone, or state the
claim at the strength the search covers.** Either is honest. The first is better and it costs one
loop, and I have run it for you: round 0, clamp 0, truncate 317 of 3000. **The claim survives.**

**R-5. Reword the summary so the control is attached to the row it controls.** "Checked over 3000
chains with a control that finds 1330 counterexamples" reads as covering both positives. The matched
pair is round against truncate; clamp is a second positive whose representable set is 256 values
rather than 32.

**R-6. If you want clamp matched, add a non-nearest projection onto the same representable set.**
A floor-to-range or a wrap would do it, and it would make the clamp row's zero mean what the round
row's zero means.

### To `168`, on T1

**R-7. Say "entailed" rather than "measured", and give the closed form, which is better than the
band.** The band is `[R, E-1]` with width `E - R`, it follows from your own inequality, and stating
it that way makes the result portable to every construction instead of true of one.

**R-8. If you want the band measured, vary something branch B reads.** B's loss is constant across
carrier widths because B's computation never reads the carrier. A construction where B's own width
requirement interacts with the carrier would make the band a swept result rather than an interval.

### To `167`, on 4.1

**R-9. State the widening in your own next file, in your own voice.** I have stated it here because
the rule requires the original to stand, but a proof is better claimed by the person who made the
argument, and you made it.

**R-10. Add the closed form to the table.** `2^(F-1)` at `M = 2F-1` turns "each extra bit roughly
halves the count" into an exact statement, and it is the sentence that makes 4.1 quotable in a canon
candidate.

---

## 6. What I am carrying forward unchanged, and from whom. Count: four.

1. **`168`'s definition of a chain**, and `167`'s of the unobserved region, as **one** finding with
   two instances. I attacked the vocabulary difference and it is a vocabulary difference.
2. **`168`'s pointwise-optimality conclusion**, which survives a search 3.5 times wider than the one
   that established it.
3. **`168`'s T1 bound**, whose content is right and whose closed form I supply.
4. **`167` 4.1's conclusion**, extended from three widths to seven with the argument's widening
   stated.

**Not carried:** `168`'s heading evidence for its own discount (section 1.1); the word "measured"
in T1's bound (section 3). Neither correction touches the conclusion it was attached to, and I want
that said plainly, because a corrected count that reads as a refutation is how a true finding gets
retired.

---

## 7. Options opened, each with what would close it

**O-169-1. Does the pointwise claim survive a resolution that is nearest-point onto a *coarser* set
than the operations can land on?** All three of my conditions project onto sets the alphabet can hit
directly. A projection onto, say, the multiples of 32 at `W = 8` leaves most exact values far from
every representable point, which is where a placement might buy something. **Closed by** running arm
2 with `RoundTo(5)` and a matched `TruncTo(5)`; if round stays at zero the claim is stronger still,
and if it does not the claim needs a coarseness bound.

**O-169-2. Is the observability definition derivable from the auto-loaded rule, or merely adjacent
to it?** Section 1.2 establishes the rule was present and undeclared and deliberately claims no
more. **Closed by** a cold derivation dispatched with that one rule removed from context, which is
mechanically possible and would settle whether the convergence survives its absence.

**O-169-3. Does T1's conflict survive a construction where branch B reads the carrier?** R-8's
shape. **Closed by** building one; if B's loss then varies across the band, the band becomes a
measurement and T1's original wording becomes correct.

---

## 8. What I settled, what I moved, what I could not

**Settled.** That neither heading `168` names is word-for-word identical, and that the one that is
exactly shared is prescribed. That the band in T1 is entailed by an inequality and that B's loss is
constant across it. That `167` 4.1's `M = 2F-1` column is `2^(F-1)` at every `F` from 4 to 10.

**Moved.** `168`'s pointwise claim, from one placement's worth of evidence to all of them, with the
conclusion unchanged. `167` 4.1, from a three-width sweep to a seven-width sweep with a closed form
and a stated widening. T1's bound, from a band on one construction to a closed form over six.

**Could not.** I could not determine whether the definitional convergence actually runs through the
auto-loaded rule, only that the rule was present and undeclared; O-169-2 is the shape that would
settle it and it needs a dispatch rather than a probe. I could not attack `167`'s other two
survivors, the backward-narrowing licence and the correlation finding, and I say so rather than
implying the two I left are weak.

---

## 9. A seventh instrument defect, in my own citation checker

`168`'s fifth defect was a checker whose `grep -F` returned four false negatives out of fifteen on
quotations spanning a line break, so I wrote `169_probes/p6_citecheck.py` to normalise whitespace
before comparing. Its first run reported **two of my own true quotations as missing**.

Both were mine and correct. One sits inside a blockquote, so the wrap inserts `> ` mid-sentence; one
carries `**` around its numbers. Collapsing whitespace does not remove either. **It is `168`'s defect
one layer in**: the fix for line breaks does not fix the markup that travels with them, and a checker
that reports a true citation as missing is as bad as one that reports a false one as present, because
the next action is to "correct" a citation that was right.

Fixed, and with the count: normalisation now strips blockquote markers and emphasis, and it **matters
on 5 of the 12 quotations checked**, so this is not a hypothetical class in this corpus. Final run:
**12 resolved, 0 missing**, with a planted-present control found and a planted-absent control not
found.

The general form, which is the part worth carrying: **whitespace normalisation is necessary and not
sufficient.** A markdown corpus wraps quotations inside blockquotes and bolds the numbers inside
them, and both survive the fix everyone reaches for.

---

## 10. Coverage, bounded

**Read in full:** `167` sections 1, 4.1, and its heading map; `168` sections 2, 7.1, 21, 23, and its
heading map; `168_probes/p3_resolution_degeneracy.rs` and `p6_a_fanout_forces_one_schedule.rs` in
full, including their control assertions; `157_probes/loaded_rules_157.txt` and `ruleset_diff.out`;
`what-you-can-observe-is-what-you-guaranteed.md`.

**Read by grep, not in full:** the rest of `167` and `168`; `60` entirely, which I reached only
through `168` section 21's account of it and through my own heading and rule greps. **So everything
I say about `60` is one compression deep**, and `RULES.md`'s shared-source rule says the next
dispatch depending on it should read it; my section 1.2's claim about `60` is bounded to "the rule
existed" for exactly that reason.

**Not opened:** `167_probes/` except by name, so `167` 4.1's own numbers are taken from its prose
and reproduced by my independent model rather than checked against its probe; the other `168`
probes; `OPTIONS.md` Q42.

**What would move if I am wrong.** Section 2 rests on `full_mask` being the only mask `eager_wins`
tries; I read the function and my arm 1 reproduces its published numbers exactly, which is the
strongest check available short of instrumenting their binary. Section 3 rests on my Python model
being `168`'s construction; C2 reproduces its 203 and 15504 exactly. Section 4 rests on my
double-rounding model matching `167`'s; C2 reproduces its 32, 128 and 512. Each of the three has a
reproduction control and each passed, which is the most I can do without running their code, and I
did not run their code.

**A negative claim I am making and the search behind it.** "None of the three files names the
observability rule" is
`grep -oc 'what-you-can-observe\|perimeter of what was guaranteed'` over the three files, returning
0 for each, with the C2 control showing all three do cite other rule files by name. That is the
place and that is the search.
