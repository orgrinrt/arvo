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

