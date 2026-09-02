# 61. Absorption against coherence, measured

**Date:** 2026-08-09. **Position:** file six of unit two on the format-concept topic, dispatched from
`59`'s P3 ("cheapest item on this list by a distance") to settle a conjecture three files rest on, and
to measure the wrap ring at nonzero fraction, which `59` names "one arm away". Four experts remain
before the consolidation.

**Probes:** `61_probes/`, two of them plus a rerun record of three inherited instruments, committed as
made. `61_probes/RUN.md` is the build line.

**Re-run before argued with.** `56_probes/q1`, `57_probes/p2` and `55_probes/p4` rebuilt on
`nightly-2026-05-28` and diffed against their committed outputs: all three byte-identical
(`61_probes/rerun/`). Every count this file takes from any of them is a count regenerated on this
machine before it was trusted.

**Reading:** `INTENTS.md`, `00_brief.md`, `RULES.md`. Then `56` and `57` in full with their probe
sources (`56_probes/q1`, `57_probes/p1` through `p6`) and outputs, since they are this file's subject.
Then `55b`, `58`, `60` in full. `59` in full (the checkpoint dispatching this file). `OPTIONS.md`'s
Q3, Q5, Q6, Q11, Q12, Q17 entries. **Not opened:** `55` itself (beyond what `55b`, `56`, `57`, `58`,
`59` and `60` quote of it), `08`, `35`, `18`, `20`, `25`, `40`, `43`, `42`, `50`, `DROPLIST.md`,
`seed/`, `archive/`. Where anything below touches those, it is sourced to the register or to a file
that read them, and marked so.

**Register:** nothing here settles anything. Op's explore mode is in force per `00_brief.md`.

## 0. Gates

**Canon gate.** There is no ratified canon; this panel is writing the first one, per `00_brief.md:8-9`
and `INTENTS.md`'s own header, which records that no entry currently holds the ratified rung. The
governing material is `INTENTS.md` plus the acceptance criterion. Nothing below proposes a mechanism;
both probes are plain integer arithmetic with no type-level construction and no feature gate:
`grep -c '^#!\[feature' 61_probes/*.rs` returns 0 on both files. The strategy set is open per I1 and
nothing below presumes a count.

**Test gate.** `find mock/crates -name '*.rs' | wc -l` returns 0; the tree the brief declares nuked is
empty in fact, confirmed directly (`ls -la mock/crates` shows only `.` and `..`). No suite to run or
audit. The substitute is the probe discipline: both new probes carry instrument validation that must
observe both truths, and both fire.

**Repository state.** `git status --porcelain` shows only `61_probes/` untracked. Neither of the two
issues `57` and `58` flagged at their own section 0 (deleted `docs/`, modified bench artifacts) is
present on this tree; `58` already recorded that resolution and I confirm it independently.

## 1. The answer, before the working

**Job one.** Absorption and coherence are not the same law. They are exactly the same law on the
domain that matters for a real fold, and they are two different laws the moment a fold's later
operand is not itself already an element of Q. The dispatch named the crux correctly: "second
operands drawn from outside the representable set." Measured, that condition is not merely relevant,
it is the exact and only source of every disagreement found.

Precisely: define **coherence-ext** as `56`'s C-law, `rho(op(x,y)) == rho(op(rho(x), rho(y)))`,
instantiated over the identical `(x, y)` domain `57`'s absorption predicate uses (`x` a reachable sum
of two box elements, `y` ranging over the operand box), so the only syntactic difference from
absorption is whether `y` is reduced on the right-hand side. Over `57_probes/p2`'s own 4248-
configuration sweep, addition, absorption and coherence-ext disagree on **252 configurations**, and
**every one of the 252** has at least one box element outside `Q`; over the configurations where the
whole box sits inside `Q`, the disagreement count is **exactly zero** (`61_probes/q1_output.txt`).
Multiplication shows the identical pattern: 240 disagreements, all with the box extending past `Q`,
zero when it does not. A second, deliberately widened sweep (7744 configurations, box reaching
`[-20, 20]` against clamps as tight as `[-4, 4]`) reproduces the same exact partition at 220 and 206
disagreements respectively, still zero when the box is a subset of `Q`. This is not a correlation
found by sampling; it is a boolean partition checked on every configuration in both sweeps.

**And where they disagree, coherence-ext is the one that is wrong.** For addition, absorption matches
measured associativity on all 4248 (and all 7744) configurations, zero mismatches, reproducing
`57`'s own finding exactly. Coherence-ext mismatches measured associativity on **exactly the same 252
configurations** it disagrees with absorption on (252 and 252, identical counts, both sweeps). So
`57`'s identification is correct in the case that matters and false as a general statement, and
the failure mode is not symmetric: absorption is never the one that gets associativity wrong.

The mechanism is mechanical rather than mysterious, and section 2 works a witness by hand: coherence,
as `56` literally states it, independently reduces **both** operands before combining, which models a
schedule where every leaf of the fold is itself the output of a prior adaptation. A real fold's
operand is a stored value, already an element of `Q` by construction (every arvo numeral in memory is
format-typed), so `rho(y) = y` always holds for a genuine fold operand and the two laws collapse to
one identically. They separate only when the sweep tests a `y` that could never occur in a real fold:
a raw, unreduced value fed straight into the operand box. `57_probes/p2`'s own sweep does this by
construction (`blo, bhi` range independently of `lo, hi`), which is exactly the gap `59` section 1a
flagged and nobody had run the check on.

**A second, weaker mechanism affects `56`'s literal window-based coherence separately.** `56_probes/
q1`'s own shape (ambient window used directly for both operands, no separate concept of a box of
reachable sums) disagrees with absorption far more often, **681 of 4248** for addition, and **429**
of those disagreements occur even when the box is entirely inside `Q` (`61_probes/q1_output.txt`,
"mechanism check" line). This is a domain-size defect distinct from the reduced-y mechanism: a
literal per-config instantiation of `q1`'s window as the box never tests values beyond the box (the
reachable sum `x = op(a,b)` can exceed the box, and a coherence check restricted to the box alone
never examines it), so it is testing a strictly smaller domain than the associativity check draws
from. This is a caution about instantiating `56`'s window-based coherence naively at a config's own
box; it is not the crux the dispatch named, which is fully accounted for by the reduced-y mechanism
above.

**Job two.** The wrap ring does not survive a genuine rescaling multiply. At every `F` from 1 to 3,
across `M = 15, 31, 63`, wrap's multiplicative associativity and distributivity both fail, with
counts growing from hundreds to hundreds of thousands as `M` grows (`61_probes/q2_output.txt`,
section 2: 9 of 9 fractional configurations are NOT rings). The additive half survives unconditionally
and is stronger than saturation's: wrap keeps a full **abelian group** under addition (identity,
inverses, associativity, commutativity, all zero failures at every `F` and every `M` measured), where
saturation only ever had a monoid. The multiplicative half breaks by the **identical mechanism**
`57`/`58` already isolated for saturation: the rescale `(a*b) >> F` is a lossy step shared verbatim by
both policies' `mul`, and both policies' multiplicative associativity break under it side by side at
the same configuration (`61_probes/q2_output.txt`, section 3: wrap `*assoc` 1712, saturation `*assoc`
398, at `M=15, F=1`, both nonzero under the same coarsening code). Multiplicative commutativity,
identity and zero-annihilation all survive at every row measured; only associativity and
distributivity break, which is exactly the semiring's failure signature transplanted onto wrap. So
the honest statement is symmetric with `57`'s original semiring finding: **wrap induces a ring only
at `F = 0`. At `F > 0` it degrades to an abelian additive group whose multiplication is neither
associative nor distributive, which is worse than "not a ring" and fails to clear the bar for a
semiring too**, for the same reason saturation's did.

Both results are unconditional in Q3. Neither probe performs, or needs, a mixed-numeral addition
anywhere; every operation in both sweeps is a single numeral's own operation at a fixed, common scale.
Section 5 states this precisely and why it is not merely an assumption.

## 2. Job one, worked

### 2.1 The three predicates, stated exactly

**Absorption**, `57_probes/p2:90-102`, reproduced faithfully in `61_probes/q1_absorption_versus_
coherence.rs`'s `absorbing()`:

```
for a, b in box:
    x = op(a, b)                          // UNREDUCED
    for y in box:                         // y UNREDUCED on both sides
        rho(op(rho(x), y)) == rho(op(x, y))
```

**Coherence-ext**, the same `(x, y)` domain, differing in exactly one place:

```
for a, b in box:
    x = op(a, b)                          // UNREDUCED
    for y in box:
        rho(op(x, y)) == rho(op(rho(x), rho(y)))   // y REDUCED here
```

**Coherence-direct**, `56_probes/q1`'s literal shape, using the operand box as the ambient window:

```
for a, b in box:                          // a, b range over box directly, no reachable-sum step
    rho(op(a, b)) == rho(op(rho(a), rho(b)))
```

The three are genuinely different statements. Absorption and coherence-ext share every quantifier and
differ only in whether `y` is reduced before combining; coherence-direct additionally never tests a
value beyond the box, so it covers a strictly smaller domain than the other two. `59` section 1a
identified the first difference correctly; this file adds that the second difference exists too and
matters on its own.

### 2.2 A hand-worked witness, taken from the sweep

`lo = None, hi = Some(-6), box = [0, 0]`, addition. `Q` here is everything at or below `-6`; the box
has one element, `0`, which sits outside `Q`.

`x = add(0, 0) = 0`. `rho(x) = -6` (clamped, since `0 > -6`). `y = 0`, the only box element.
`rho(y) = -6` too, since `y` is the same value.

**Absorption:** `rho(op(rho(x), y)) = rho(-6 + 0) = rho(-6) = -6`. `rho(op(x, y)) = rho(0 + 0) =
rho(0) = -6`. Equal. Absorption holds for this (degenerate, one-element) box.

**Coherence-ext:** `rho(op(x, y)) = rho(0 + 0) = -6`. `rho(op(rho(x), rho(y))) = rho(-6 + (-6)) =
rho(-12) = -12` (no floor is present, so nothing stops `-12`). `-6 != -12`. Coherence-ext fails.

**Measured associativity**, over the single triple the box admits: `l = rho(rho(add(0,0)) + 0) =
rho(-6 + 0) = -6`. `r = rho(0 + rho(add(0,0))) = rho(0 + (-6)) = -6`. `l == r`; associativity holds.

So on this configuration absorption is correct (predicts associative, and it is) and coherence-ext is
wrong (predicts non-associative, and it is not). The reason is visible in the arithmetic: coherence-ext
independently reduces `y` from `0` down to `-6` before combining, a reduction that no actual fold ever
performs on a raw operand that was never itself the output of a prior adaptation. Absorption's `y` is
never touched, which matches what a fold literally does with its later operands.

`61_probes/q1_output.txt` prints this exact configuration first among 252 for addition and confirms
`y_outside_q_count = 1`, meaning the box's one element is outside `Q`, which is the condition under
which the two predicates can differ at all.

### 2.3 The condition is exact, not merely correlated

`61_probes/q1_absorption_versus_coherence.rs` partitions every disagreement by whether the whole
operand box sits inside `Q`. The result, over both sweeps and both operations:

| sweep | op | absorption vs coherence-ext disagreements | ...with box entirely inside Q | ...with box extending past Q |
|---|---|---|---|---|
| p2's 4248 configs | add | 252 | 0 | 252 |
| p2's 4248 configs | mul | 240 | 0 | 240 |
| widened, 7744 configs | add | 220 | 0 | 220 |
| widened, 7744 configs | mul | 206 | 0 | 206 |

Zero disagreements, in every sweep, every time the box is a subset of `Q`. This is the exact form of
`57:277-278`'s bridging step ("with `b` drawn from `Q` so that `rho(b) = b`") stated as a measured
biconditional rather than an assumption: **absorption and coherence-ext are the identical predicate
exactly on the domain where every fold operand is already representable, and they can diverge, with
coherence-ext being the wrong one, the moment that ceases to hold.**

For a real arvo fold this domain restriction is not a restriction at all. Every stored value of a
numeral type is already an element of `Q` by construction; there is no way to hold a raw,
un-adapted value in a variable of format type. So the condition under which the two laws coincide is
the condition that always holds for actual arvo data. **The identification `57` made in one paragraph
is correct for what the panel needs it for, and it is correct because of a fact about arvo's typing
that neither `56`'s nor `57`'s probe encodes**, since both sweep an operand box that is free to
extend past the clamp bounds precisely because nothing in either probe's construction models "this
value is already format-typed."

### 2.4 What this means for the register's identification claim

`57:286-288` states "coherence is not a coarsening of a finer true statement. It is the statement",
and recommends the register carry absorption as the criterion with coherence as the same fact. Both
halves stand, corrected in scope:

**Absorption should be the criterion the register carries**, unconditionally, exactly as `57` and
`59`'s P3 both point toward. It matched associativity perfectly across every configuration in both
sweeps, including the ones where coherence-ext did not.

**Coherence, as `56` literally states the C-law**, is a **stronger** statement than absorption: it
additionally asserts something about values that are not fold operands at all (raw, un-adapted
inputs). That stronger statement can be false while absorption, and the actual associativity of a
real fold, are true. So coherence-as-literally-stated is not interchangeable with absorption as a
general mathematical fact; it is interchangeable with absorption **restricted to the operand set
being a subset of Q**, which is a condition worth writing into the canon sentence rather than leaving
implicit, because the two probes that carry the current identification both sweep past it silently.

**This does not reopen `57`'s absorption biconditional or `58`'s F-independence argument for
addition**, since both stand on absorption directly and neither depends on the coherence
identification to be true. It narrows what may be said about `56_probes/q1` specifically:
`q1`'s stated coherence law quantifies `a, b` over the ambient window `[-64, 64]` against
`Q = [-8, 7]`, which is the unrestricted form this file shows can diverge from absorption. But the
check that actually validated coherence against real fold behaviour in that same file,
`chain_divergences`, restricts `a, b, c` to `Q` itself rather than to the window (verified by
reading `56_probes/q1_two_law_families.rs`'s `profile()` function directly), so `56`'s own strongest
piece of evidence was already inside the safe domain by construction, even though its stated law was
not. `55b`'s induced-algebra material rests on the same `chain_divergences`-shaped window-restricted
check (`55_probes/p4`'s `window_chain_failures`), which I have not opened at the source level this
dispatch and do not claim to have verified the same way; I flag it as likely to share the pattern
rather than asserting it does. What changes for `56` is only the register's phrasing of the
identification, from an unconditioned "they are the same law" to "they are the same law on Q, and
coherence as literally quantified is strictly stronger off it."

## 3. Job two, worked

### 3.1 The machinery already existed and had never been driven

`57_probes/p3_semiring_across_widths_and_scales.rs` declares `Policy::Wrap` with
`reduce(Wrap, m, x) = x.rem_euclid(m + 1)` and a genuine rescaling multiply, `mul = |a, b|
reduce(p, m, (a * b) / scale)` with `scale = 1 << f` (`p3.rs:63-64, 110-111`). `grep -n
"Policy::Wrap" 57_probes/p3_semiring_across_widths_and_scales.rs` shows exactly one call site,
`check(Policy::Wrap, 15, 0)`, inside the file's own instrument-validation section (`p3.rs:302-310`),
always at `f = 0`. `59` section 1e and `58:441-445` both name this the same gap independently, and
`59`'s P3 calls it "one arm away". It is closer than that: the rescale code for wrap already exists
and needed only a loop over `F`.

### 3.2 The ring, at F = 0, reproduced first

Before trusting anything at `F > 0`, `61_probes/q2_wrap_ring_at_nonzero_fraction.rs` reproduces the
existing claim: `M = 15, F = 0` gives zero failures on every one of nine axioms (associativity,
commutativity and identity for both operations, distributivity, zero-annihilation, and additive
inverses), matching `55_probes/p4` and `57_probes/p3` section 4's wrap-is-a-ring reading exactly
(`61_probes/q2_output.txt`, section 1).

### 3.3 At F > 0, the ring collapses, in the same two properties that broke the semiring

Nine configurations swept (`M = 15, 31, 63` at `F = 1, 2, 3`), all nine fail ring status
(`61_probes/q2_output.txt`, section 2). Every failure is in exactly two of the nine axioms:
multiplicative associativity and distributivity. Additive associativity, commutativity and identity
hold at every row; multiplicative commutativity, identity and zero-annihilation hold at every row;
additive inverses exist at every row (`no-inv` column reads 0 throughout). The counts grow with `M`
(1712 associativity failures at `M=15, F=1` rising to 208,686 at `M=63, F=3`), which is the expected
shape of a fixed proportion of a growing space, not a change in which laws break.

**The mechanism is shared code, not an analogous but separate defect.** Section 3 of
`61_probes/q2_wrap_ring_at_nonzero_fraction.rs` runs wrap and saturation side by side at the
identical configuration, `M = 15, F = 1`: wrap's `*assoc` fails 1712 times, saturation's fails 398
times, both under the exact same `(a * b) / scale` rescale, differing only in which `reduce` runs
afterward. This matches `58` section 2.2's argument precisely, transplanted to wrap: there is no
fixed-width eager multiply at `F > 0` that supplies the exactly-associative ambient operation either
policy's absorption/coherence proof needs, because the rescale is baked into every pairwise step
regardless of which reduction follows it. `58` proved this for saturation; this section shows the
same structural fact holds for wrap by direct measurement rather than by re-deriving `58`'s argument
from scratch, since the shared code makes the transfer a fact about the code rather than an inference.

### 3.4 The additive half is stronger than saturation's, and survives for the same reason

`60`'s phase one derived the general fact `58` used for addition: raw integer addition never
rescales, so an `F = 0` additive result transfers to every `F` by inspection rather than by sweep.
The same argument holds for wrap and is checked directly: `wrap_reduce`'s `add` closure in
`61_probes/q2_wrap_ring_at_nonzero_fraction.rs` never reads `f` or `scale` anywhere in its
definition (`grep -c 'scale\|f)' ` against the `add` closure shows zero), and every row measured
confirms zero additive failures at every `F` (`61_probes/q2_output.txt`, section 4). Wrap's additive
structure is a full abelian group at every `F` measured (identity, inverses, associativity,
commutativity all hold), which is strictly stronger than saturation's additive monoid (no inverses
except at the identity, per `55b`/`57`'s own material): wrap's group structure comes from
`rem_euclid`'s own algebraic content, which section 3 shows is untouched by the fraction axis, exactly
as it is untouched for saturation's weaker monoid.

### 3.5 Wrap does not merely lose "ring"; it fails to reach "semiring" either

`61_probes/q2_output.txt` section 5 checks the same eight-axiom semiring predicate `57_probes/p3`
used for saturation (associativity, commutativity, identity for both operations, distributivity,
zero-annihilation, no inverse requirement), against wrap at `F > 0`: **false at every one of nine
fractional configurations**. So "wrap is a ring at `F = 0` and loses ring status at `F > 0`"
understates the degradation. What survives is the additive group (section 3.4) and the multiplicative
monoid's identity/commutativity/annihilation properties; what is lost is exactly the same pair of
laws (`*assoc`, distributivity) the unsigned saturation semiring lost, landing wrap's `F > 0` induced
algebra at the same structural place a near-semiring collapse lands saturation's, not at some weaker
but still-coherent intermediate structure.

## 4. Both branches of Q3, and why this file needs neither

`59` section 2b found that not one of unit two's five files mentions Q3, and that the unit's
strongest unconditional result (addition's F-independence) is silently premised on same-scale
addition, which Q3's second option (inferred mixed-numeral addition) could threaten if the inferred
result narrows below the join. `60`'s phase two, section 3b, sharpened this: aligning to the join is
an exact widening, and the coarsening threat enters only when the inferred result format is coarser
than the finer operand, which is a fact about the inference rule and the schedule, not about
mixedness alone.

**Neither of this file's two results performs a mixed-numeral operation at any point.** Absorption
and coherence in job one are single-numeral properties: one clamp, one box, one operation, checked
against itself. The wrap ring in job two is one modulus, one fraction width, checked against itself.
`grep -cE "op\(.*,.*\).*op\(" 61_probes/*.rs` (the pattern a mixed-format composition would need,
two different reductions composed) returns zero in both files; every reduction in both probes is the
same `rho` or `wrap_reduce` applied throughout a single sweep.

So both results hold **under all three of Q3's options identically**, because the premise Q3's
options vary (whether an operation exists that mixes two different numerals, and if so what its
result format is) never arises in either probe. This is the "same under all three options" branch
`59`'s P2 dispatch asked for as one of its two possible done-states, and it is the honest answer for
this file's specific results: they are orthogonal to Q3 rather than resolved by it, and stating that
plainly is worth more than silently assuming it, since `59` found the unit's other strongest result
was doing exactly that.

## 5. What I put to `56` and `57`, for the resumption

**To `56`.** Your coherence law, as stated (`56_probes/q1_two_law_families.rs:10-12`, quantifying
both operands over the ambient window), is a **stronger** statement than absorption, and the
strengthening is exactly the independent reduction of the second operand. Measured: they coincide
exactly when the operand box is a subset of `Q` (zero disagreements, both sweeps, both operations),
and off that domain coherence is wrong about associativity in every case it disagrees on for
addition. Two questions. First, was the window `[-64, 64]` in your own probe, chosen well outside
`Q = [-8, 7]`, doing implicit work here, since your `chain_divergences` check (the one that actually
validated coherence against measured fold behaviour) restricts `a, b, c` to `Q` itself rather than to
the window, so your own strongest validation was already inside the safe domain by construction while
your stated law was not? Second, given this, would you restate the C-law with the domain restriction
made explicit (`b` in `Q`, per `57`'s own bridging assumption), or is there a reading under which the
unrestricted form is the one the design actually needs, and I am missing what it buys?

**To `57`.** Your identification (`57:277-278`) is correct exactly where you needed it and false as
a general statement, and the direction of the error matters: where the two diverge, coherence is
wrong and absorption is right, never the reverse, in every configuration measured. Would you accept
the register carrying the identification as "absorption, and coherence restricted to operands already
in Q, are the same law" rather than as an unconditioned identity? And a second item, since it fell out
of the same probe: your `p2`'s own operand box was never restricted to `Q`, so your reported 4248-
configuration biconditional for absorption itself never depended on the identification holding; it
is absorption's own result, standing on its own, and the corrected scope of the identification does
not touch it. Do you read it the same way, that the identification's narrower scope costs nothing
already built on absorption directly?

## 6. What the register should gain

Reported; I have edited neither `OPTIONS.md` nor `INTENTS.md`.

**Q12's mechanism paragraph, again.** It already carries the caution from `55b`'s `p5` and `57`'s
adjudication of the `42` refutation. It should additionally state that the coherence-as-criterion
recommendation `57` makes (and `59` records as owed a second read) is now measured: coherence and
absorption are the identical predicate exactly when the operand box is a subset of `Q`, with the
counts above, and the register's phrasing should say "absorption (equivalently, coherence restricted
to `Q`)" rather than "coherence" unqualified, since the unqualified form is a different and in these
cases a wrong predicate.

**Q17 should gain the wrap-ring row it has been missing.** As of this file, `OPTIONS.md`'s Q17
entry carries the additive-independence argument, the multiplicative-gap finding, `60`'s probe D
correction to the accumulator saving, and the rounding-mode result; it does not currently carry a
row for the induced ring at all. The gap was named before, but not yet as a register line: `59`
section 1e states "`58:441-445` correctly records that wrapping multiplication at `F > 0` is
unmeasured, an inference rather than a result", and `59` section 2a's own table marks the induced
ring "**unmeasured**, multiplicative half only", both inside `59`'s checkpoint file rather than in
`OPTIONS.md` itself. It should now read **measured, and it collapses**: wrap induces a ring only at
`F = 0`; at `F > 0` the additive group survives unconditionally (stronger than saturation's monoid)
but the multiplicative half fails associativity and distributivity by the identical shared-code
mechanism that broke the semiring, landing wrap below even semiring status at every `F > 0`
configuration measured.

**Q11's structure-naming option** (the register's second Q11 option, "the numeral names its algebraic
structure") should carry the same scale condition for wrap that `57` (section 5) and `59` (section 6)
each proposed adding for saturation's semiring, and that `OPTIONS.md`'s current Q11 text does not yet
carry for either policy: an induced structure is a fact about the policy **and** the scale jointly,
never the policy alone, and this file's measurement extends that requirement from saturation, where
it was proposed, to wrap, where it is now also measured.

**A droplist candidate, not yet an entry.** "Wrap's induced ring transfers to fractional formats"
should be recorded as closed on the same structural grounds as the semiring's droplist candidate
(`58` section 5): the mechanism is a fact about what a fixed-width rescaling multiply is, not about
which policy consumes its output, and it is now measured directly for wrap rather than only argued by
analogy. What would reopen it: the same thing that would reopen the semiring's entry, a total
translation-covariant rounding rule under which the rescale loses no information on some nontrivial
operand set, which `58` section 2.2 argues cannot exist and has not proven as a theorem.

## 7. Bearing on the live options

**Q5 (one axis or two).** No new bearing beyond what `57` and `58` already carry. Job two's finding
that wrap and saturation share the identical multiplicative failure mode under the identical rescale
code is mild additional evidence that the scale axis's law consequence is a property of the rescale
step itself, independent of which overflow policy runs after it, which is consistent with (not
decisive for) the two-axis reading.

**Q6 (Warm wraps or clamps).** Sharpens the cost on both sides rather than deciding between them, in
the direction `57:624-626` already named: whichever overflow policy `Warm` takes, if it multiplies at
`F > 0` (which almost every `UFixed`/`IFixed` does), its multiplicative chains inherit the F > 0
boundary regardless of the choice. Wrapping no longer has an advantage over clamping on the
multiplicative axis at nonzero fraction; both lose associativity and distributivity by the same
mechanism, at different but comparable magnitudes (job two section 3.3's side-by-side count). What
remains distinct is the additive axis, where wrap keeps a full group and clamping only a monoid, and
the coherence result (job one), where wrap remains additively coherent at every `F` (per `58`'s
established F-independence of addition) and signed clamping does not.

**Q11 and Q12.** Section 6.

**Q17.** Section 6, the wrap-ring row directly.

**Kills nothing.** No option in the register is closed by anything here. One non-option claim inside
Q12 is sharpened (the identification's scope), and one droplist candidate is offered, not yet an
entry.

## 8. What I could not determine

**Whether the absorption/coherence divergence transfers to n-ary folds beyond triples.** Both sweeps
test the two-argument absorption predicate and its associated three-term associativity check. A
four-or-more-term fold's schedule has more places a "reduced-y" mistake could enter, and I have not
built that case.

**Whether the divergence mechanism generalises to operations other than addition and multiplication.**
Both probes cover the same two operations every prior file in this unit covered. A subtraction, a
shift, or a mixed-operation chain is untested.

**Whether coherence-direct's second, domain-size mechanism (section 2.1, the 429/477/75/150
disagreements that occur even inside `Q`) has a clean characterisation of its own.** I identified
that it exists and is distinct from the reduced-y mechanism, and named its cause (never testing values
beyond the box), but did not build a probe isolating it the way section 2.3 isolates the reduced-y
mechanism. It is a secondary finding rather than the crux the dispatch named, and I have not chased
it further.

**Whether wrap's semiring-collapse magnitude (the specific counts in section 3.3) has a closed form
in `M` and `F`**, the way `57`'s one-bit additive accumulator finding does. Nine data points show the
count growing with `M`; I have not fit or argued a formula.

**Whether round-to-nearest, rather than truncation, changes wrap's F > 0 collapse the way `58`
measured for saturation (magnitude moves, existence does not).** Both of `61`'s probes use truncating
division (`/`) for the rescale, matching `57_probes/p3` and `55_probes/p4`'s own construction. I did
not rerun wrap under a different rounding rule, and given `58` section 3.1's result for saturation
(rounding mode changes the count, never whether the count is zero), I expect the same here but have
not measured it.

**Anything about `08`, `35`, `18`, `20`, `25`, `40`, `43`, `42` or `50`.** Not opened. Where this
file's results touch material those files carry (per the register's citations of them), I have said
so and have not reasoned about their contents directly.

## 9. Coverage, bounded honestly

**Read in full:** `INTENTS.md`, `00_brief.md`, `RULES.md`, `56` (with `56_probes/q1` source and
output), `57` (with `57_probes/p1` through `p6` sources and outputs), `55b`, `58`, `59`, `60`
(both phases), `OPTIONS.md`'s Q3, Q5, Q6, Q11, Q12, Q17 entries. **Not opened:** `55` directly, `08`,
`35`, `18`, `20`, `25`, `40`, `43`, `42`, `50`, `DROPLIST.md`, `seed/`, `archive/`.

**Re-run before relied on:** `56_probes/q1`, `57_probes/p2`, `55_probes/p4`, all byte-identical
against their committed outputs. **Not re-run:** `57_probes/p1`, `p3`, `p4`, `p5`, `p6`; I read their
sources and outputs but did not rebuild them, since neither job depends on their specific counts, only
on the probe-discipline pattern they demonstrate (which I checked by reading, not by re-execution).

**Built:** two probes, both exhaustive within their stated domains, both with instrument validation
that fires (both boolean values of every predicate observed somewhere in the sweep, both a ring and a
non-ring row present), both committed with sources and outputs.

**Everything measured here is plain integer arithmetic**, no arvo types, no type-level construction,
deliberately, matching the discipline every file in this unit has followed, so that nothing here is
an artifact of a representation choice.

**Nothing here is priced.** Every number is a count from a committed probe; no bench harness ran on
any of it, and the wrap-versus-saturation magnitude comparison in section 3.3 is a count comparison,
never a timing.

**First-read here, owed a second:** the exact subset-of-Q partition for the absorption/coherence
disagreement (section 2.3); the identification of coherence-direct's second, domain-size mechanism as
distinct from the reduced-y mechanism (section 2.1); the wrap ring's collapse at `F > 0` and its
shared-code mechanism with saturation's semiring collapse (section 3). **Seconded here, from a
different instrument:** `57`'s absorption-predicts-associativity result for addition, reconfirmed
under both of this file's sweeps including the widened one (`61_probes/q1_output.txt`, "disagreements
with MEASURED associativity: absorption 0" in every addition row); `58`'s F-independence argument for
addition, reconfirmed structurally for wrap's `add` closure rather than only for saturation's
(section 3.4).

**Nothing here settles anything.** The mode is explore, there is no canon, and `56` and `57` should
be resumed to answer section 5 before either of this file's results is carried past this unit's
consolidation.
