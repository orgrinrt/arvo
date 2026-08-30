## 126. Wronski. What rounding actually selects.

**Phase one. Derived cold, from `INTENTS.md`, `RULES.md`, and the brief only.** No other panel
file, no register, no probes belonging to anyone else, no git log, has been read before this
paragraph. Everything below is my own derivation, with my own probes, committed as they ran.

### The question, restated

The brief hands me one fact about the preceding topic as background, not as something to
re-derive: overflow policy is a fork between two disjoint, both-achievable algebraic families.
Wrapping realises as the canonical quotient homomorphism `Z -> Z/2^W Z`, genuinely a ring
homomorphism because the target really is a quotient ring and wrap really is that quotient map.
Saturating realises as a clamp, genuinely order-preserving, and genuinely not a ring homomorphism
(saturating addition is not associative, checked below as a sanity anchor before moving to
rounding, since I want my own confirmation of the shape I am told to compare against rather than a
borrowed one). No map onto a finite set is both, except the constant map, by the
finite-additive-group / no-infinite-ascending-chain argument.

The question for me is whether rounding, meaning what a realisation does when an exact value falls
**between** two representable grid points (as opposed to overflow, which is what happens when it
falls **outside** the representable range), plays a comparable character-selecting role: a fork
between disjoint algebraic families, a modifier of the overflow fork, or neither.

**My answer, stated up front and argued below: neither, and the reason it is neither is
structurally informative.** Overflow is a genuine two-way fork where both branches are fully
achievable and mutually exclusive. Rounding is not that shape at all. For any operation that
actually loses precision (multiplication and any inherently-widening or non-closed operation, at
any `F > 0`), the ring-homomorphism branch is not merely harder to reach, it is **unreachable by
any rounding choice**, while the monotone branch is achieved **for free** by every deterministic
rounding rule I tested. Rounding does not offer the overflow-style choice between two live
options; it offers one option (monotone) by default and a second, narrower, genuinely either/or
fork on a different axis entirely: deterministic-and-monotone versus
stochastic-and-unbiased-but-not-monotone-per-draw. Beyond that, rounding modes differ along two
further axes that are not mutually exclusive with each other or with monotonicity: bias (which
direction and how much expectation error a mode introduces) and double-rounding safety (whether
composing two roundings at different widths agrees with rounding once, directly, at the narrower
width).

### Anchor: saturating addition is not associative (sanity check, not new)

Before trusting the brief's framing I checked it myself. Range `[-2, 2]`, `a=2, b=2, c=-1`:
`sat(sat(a+b)+c) = sat(sat(4)+(-1)) = sat(2-1) = sat(1) = 1`. `sat(a+sat(b+c)) = sat(2+sat(1)) =
sat(2+1) = sat(3) = 2`. `1 != 2`. Saturating addition is not associative, confirmed by hand,
matching what I am told. I did not build a probe file for this since it is a two-line hand check
offered only as an anchor, not a finding of mine.

### Finding 1: rounding is vacuous for the closed operations at `F = 0`, hence trivially associative and distributive there, for every mode

At `F = 0`, addition and subtraction of grid points never need rounding at any `F` (the sum of two
grid points is always a grid point, up to overflow, which is the separate axis). Multiplication of
two grid points at `F = 0` is exact (`int * int = int`), so rounding never engages there either.
Every rounding mode agrees at `F = 0` for `+`, `-`, `x`, because none of them ever has anything to
round: they are all the identity function on the relevant domain.

Probe `126_probes/p1_rounding_character.py`, Prediction 1, exhaustive over a `kmax=6` unsigned
grid, all five modes (`floor`, `ceil`, `trunc`, `round_half_up`, `round_half_even`), zero
counterexamples to associativity of rounded multiplication. Output in `126_probes/p1_output.txt`.

```
holds for: F = 0, operation = {addition, subtraction, multiplication}, signedness = unsigned,
domain = no overflow triggered (values chosen away from any bound), rounding mode = any of
{floor, ceil, trunc, round_half_up, round_half_even}, width = swept exhaustively at kmax=6.
```

This does not extend to division, reciprocal, or square root at `F = 0`. Those operations are not
closed over the integers regardless of `F` (`1/3`, `sqrt(2)` are not integers), so rounding engages
for them even when `F = 0`. I have not built a probe for this half since it is definitional rather
than measured: any operation whose exact result can fail to land in the domain even before
fractional bits exist will force rounding regardless of `F`. I flag it as derived, not measured,
and leave it absent from the predicate above rather than folding it in.

### Finding 2: at `F >= 1`, no rounding mode makes multiplication associative or distributive

Same probe, Predictions 2 and 3. Exhaustive over `kmax=6` grids at `F in {1,2,3}`, all five modes,
both associativity and distributivity over addition: every single mode fails, with a concrete
counterexample recorded for each, see `126_probes/p1_output.txt` for the full table.

```
holds for: F >= 1 (tested F in {1,2,3}), operation = multiplication (and distributivity of
multiplication over addition), signedness = unsigned, domain = no overflow triggered, rounding
mode = any of {floor, ceil, trunc, round_half_up, round_half_even} (each fails individually,
not just in aggregate), width = swept exhaustively at kmax=6.
```

This corroborates, independently and from a different angle, the associativity/distributivity
finding already cited to me in `arvo-always-optimal-internals.md` ("F == 0 is necessary... and it
is not sufficient"). I did not read that finding before deriving this; I derived it fresh from
first principles and it agrees. I record this as independent agreement, not as having checked a
citation, since I built my own grid, my own five rounding modes, and my own exhaustive sweep rather
than reading anyone's prior probe.

**The structural reason, argued rather than only measured:** a ring homomorphism from an infinite
or growing-precision source (the exact rationals reachable by chaining multiplications, whose
denominators grow without bound as more multiplications compose) into a finite target is
impossible unless the map collapses to a constant, by the same finite-structure argument used for
the overflow axis. For a single isolated multiplication the source is finite (bounded by the input
widths), so this argument alone does not forbid a homomorphism at one operation; what forbids it in
practice is that any consistent discard rule, applied at every multiplication in a chain, has to
make different information-loss decisions depending on which sub-products were computed in which
order, and reconciling those decisions to agree exactly for every possible triple is
over-constrained for any grid with more than a couple of points. This is the same negative result
well known in the floating-point literature (floating-point multiplication under any IEEE rounding
mode, including round-to-nearest-even, is not associative either, for the identical reason:
rounding discards information, and the information discarded depends on the order sub-results are
computed in). I am stating this as inherited domain knowledge plus my own exhaustive small-grid
confirmation, not as a fresh formal proof; a formal proof (an explicit counting argument over how
many distinct rounding decisions a chain of length `n` induces versus how many distinct answers
agree) is open and would be worth someone building.

### Finding 3: all deterministic rounding modes are monotone; this is not a fork, it is the default

Probe Prediction 4: `floor`, `ceil`, `trunc`, `round_half_up`, `round_half_even` all pass an
exhaustive monotonicity sweep over 296 non-grid-aligned rationals (denominator 37) at `F=3`. No
mode failed. This is not a surprising result mathematically (any function defined purely by "which
grid point is `x` closest to, with a fixed tie-break rule" is weakly increasing by construction),
but it is worth stating as measured rather than assumed, because it is the fact that makes
rounding's relationship to the overflow axis asymmetric rather than parallel: overflow's monotone
branch (saturation) costs you the homomorphism branch. Rounding's monotone branch costs you nothing
you were going to get anyway, since Finding 2 already establishes the homomorphism branch is
unreachable at `F >= 1` regardless of mode.

```
holds for: rounding mode = any of {floor, ceil, trunc, round_half_up, round_half_even}, domain =
rationals with denominator 37 in a bounded range, F = 3, monotonicity = weak (x <= y implies
round(x) <= round(y)).
```

### Finding 4: stochastic rounding is the one place rounding echoes overflow's shape, and it is a narrow echo

Stochastic rounding (round up with probability proportional to distance from the lower grid point,
down otherwise) is the one rounding mechanism I found that gives up monotonicity for something in
return, which is the same trade shape as overflow's fork, but on a completely different axis and a
much narrower one.

**Non-monotone per realisation, existence claim, confirmed.** `126_probes/p4_stochastic_nonmonotone_fixed.py`:
`x=1/10`, `y=9/10`, both in the same grid interval `[0,1)` at `F=0`. With draws `u_x=0.01` (below
`x`'s fractional part 0.1, rounding `x` up to 1) and `u_y=0.95` (above `y`'s fractional part 0.9,
rounding `y` down to 0), `x < y` but `round(x)=1 > round(y)=0`. My first attempt at this
construction (in `p1_rounding_character.py`, Prediction 5) picked `x, y` in different grid
intervals and failed to find a violation; that failure is itself informative and I record it as a
refuted first attempt rather than deleting it, since it shows the violation specifically requires
`x, y` to share a grid interval, which makes sense once stated: only within one interval do `x` and
`y` compete for the same pair of candidate grid points.

**Unbiased in expectation, exactly, confirmed analytically and by Monte Carlo.** For `x` in
`[k, k+1)`, `E[round(x)] = k*(1 - frac(x)) + (k+1)*frac(x) = k + frac(x) = x`, exactly, by
construction of the probability. Verified for five sample values including two already-integer
points (trivially exact, no randomness engages) and three fractional ones, against a 200,000-draw
Monte Carlo estimate agreeing to within sampling noise (`126_probes/p1_output.txt`, Prediction 5).

```
holds for: rounding mode = stochastic (independent draw per value), domain = any rational,
non-monotonicity = exists (there exist adversarial draw pairs within one grid interval producing
a violation; not a claim that every draw violates), unbiasedness = exact, in expectation, for
every x, independent of F.
```

This is the genuine either/or fork on the rounding axis, and it is worth naming as the actual
analogue of the overflow fork, not the homomorphism question. Deterministic rounding buys
monotonicity and gives up nothing on the homomorphism side (nothing was there to give up).
Stochastic rounding gives up per-realisation monotonicity and buys exact unbiasedness, a property
no deterministic mode achieves for a generic input (every deterministic mode has some nonzero
expected error somewhere in its domain, since it always makes the same choice at the same input).
This is a real trade, both branches are achievable, and they are mutually exclusive in every
construction I tried. I have not proven this exclusion in general, only exhibited the trade; a
rigorous statement of it as a real dichotomy, parallel in shape to the overflow one, is open.

I note for whoever reads this after me, since it is the field I actually work in: this is the
identical trade image processing calls dithering. A deterministic quantiser is monotone and
biased (it always makes the same rounding decision for the same input, so a smooth gradient
banding through a coarse quantiser shows visible steps and a consistent directional error).
Ordered or blue-noise dithering trades that monotonicity for decorrelated, near-zero-mean error,
exactly the stochastic-rounding trade above, with one refinement: a fixed per-position dither
pattern (rather than fresh independent randomness per value) restores monotonicity as a function
of value, for a fixed position, while still breaking it across positions. Whether arvo's strategies
want that refinement (a position-keyed deterministic pseudo-random rounding, rather than a fresh
draw per call) is a real design option I am not resolving here; I flag it because "stochastic" in
the brief's list may be underspecifying a design space that actually has at least two shapes
(fresh-draw and position-keyed) with different monotonicity properties, and conflating them would
be a mistake later.

### Finding 5: round-half-to-even is structurally different from directed modes on bias, and structurally the SAME as round-half-up on double rounding

The brief asks specifically whether round-half-to-even is structurally different from the directed
modes "rather than merely fairer." My answer is split across two axes, and the split itself is the
finding: it is genuinely different on one axis and genuinely not different on another, and treating
"round-half-to-even" as one undifferentiated alternative to "directed rounding" loses that.

**Bias axis: real, measured, large.** `126_probes/p1_output.txt`, Prediction 6. Constructing 2000
consecutive exact ties (`i + 0.5` for `i = 0..1999`, so every single input is a boundary case) and
summing the per-step rounding error: `trunc` gives mean error `-0.5000` (always rounds every tie
down, maximal one-directional bias), `round_half_up` gives `+0.5000` (always up, maximal bias the
other direction), `round_half_even` gives exactly `0.0000` (ties alternate between even and odd
integers, so the tie-break alternates direction and cancels exactly on this construction). This is
an adversarial, all-ties input chosen to make the effect as visible as possible; on a
non-tie-heavy, more realistic input distribution the directed modes still carry a nonzero
systematic bias (they always discard a positive quantity in the same direction), just not the full
half-step magnitude shown here. I have not measured the non-adversarial case; the adversarial case
is sufficient to establish the axis is real and the direction of the effect, not sufficient to
state a general bias magnitude for typical arvo workloads.

**Double-rounding axis: round-half-even groups with round-half-up, not with the directed modes.**
This is the one I did not expect going in, and it directly complicates the brief's phrasing.
`126_probes/p3_double_rounding_isolated.py` isolates double rounding (round to a wider accumulator
grid, then round again to a narrower final grid, versus rounding directly from the exact value to
the final grid in one step) with the clamp bound removed entirely, so nothing about overflow can be
responsible for what is measured. Result, swept exhaustively over five `(F_acc, F_final)` pairs and
all five modes: `floor`, `ceil`, `trunc` never diverge, zero instances across the whole sweep.
`round_half_up` diverges 64 times, `round_half_even` diverges 48 times. Concrete instance:
`F_acc=3, F_final=1, exact=-89/24`, double-rounded via `round_half_even` gives `-4`, direct single
rounding gives `-7/2`.

```
holds for: rounding mode = {floor, ceil, trunc}, property = double-rounding-safe (round-to-coarser
then round-again-to-coarser-still equals rounding directly to the final width in one step),
domain = rationals with denominator 24, F_acc in {2,3,4,5}, F_final in {1,2}, overflow = not
present (bounds removed entirely to isolate the effect).

holds for: rounding mode = {round_half_up, round_half_even}, property = NOT double-rounding-safe
(counterexamples exist), same domain as above.
```

The mechanism is the classic double-rounding failure from floating-point history (the reason x87's
80-bit intermediate rounding could disagree with computing directly at 64-bit double): a "nearest"
rule is only locally optimal at each rounding step, and the intermediate rounding can move a value
across a tie boundary for the next rounding step that the direct single rounding would never have
crossed. Directed rounding (always toward the same fixed direction) has no such tie-sensitivity: it
is transitive under composition of widths, essentially because dropping the low `k` bits then
dropping the next `j` bits is the same as dropping `k+j` bits at once, and the equivalent statement
for floor/ceiling follows the same way once the direction is accounted for consistently.

So: the brief's implicit binary of "round-half-to-even vs the directed modes" is real on the bias
axis and wrong on the double-rounding axis, where the actual split is "nearest-family (half-up,
half-even) vs directed-family (floor, ceil, trunc)," with round-half-to-even on the vulnerable
side. A design that chains a wide accumulator into a narrower typed result (which arvo's own
strategy markers, with their different container widths per strategy, will do routinely) and picks
round-half-to-even for its bias properties should know it is also picking up a double-rounding
hazard the directed modes do not have.

### Finding 6: rounding does not change which family the composed realisation map belongs to; it can change whether overflow is judged to have fired, without changing the stored value, when the boundary is grid-aligned

Two separate probes here, kept distinct because they answer different parts of "does rounding
interact with overflow."

**Composition preserves the overflow policy's family.** `126_probes/p1_output.txt`, Prediction 7.
`saturate(round_half_even(x))` is monotone over an exhaustive sweep (composition of two monotone
functions is monotone, a basic order-theory fact, confirmed rather than merely cited). Bare `wrap`
(mod 8, integers -3..10) is not monotone, with a concrete violation (`-3 <= 0` but `wrap(-3)=5 >
wrap(0)=0`), and nothing about composing it with a rounding step in front of it can restore
monotonicity, since the violation lives inside `wrap` itself and rounding, being monotone, cannot
un-cross an ordering that wrap already crosses.

```
holds for: overflow policy = saturating, rounding mode = any deterministic mode (tested:
round_half_even; monotone composed with monotone is monotone regardless of which monotone
rounding mode, so this generalises across the mode set already shown monotone in Finding 3),
property = monotone, F = 2.

holds for: overflow policy = wrapping, property = NOT monotone (independent of rounding mode,
by the composition argument above rather than by testing every mode against wrap specifically).
```

**Numeric agreement at one fixed width, when the boundary is grid-aligned.**
`126_probes/p2_order_of_round_and_clamp.py`, Search A. At a single target width with the clamp
boundary itself a representable grid point (true for any real type's own `MAX`/`MIN`, since those
are by construction representable values of the type), round-then-clamp and clamp-then-round agree
numerically, exhaustively, over 288 sampled exact values, five modes, three values of `F`: zero
divergences. This is a genuine positive compositionality result and I want to flag that my own
first hypothesis (that these orders would diverge, framed as Prediction 8 before I had run
anything) was wrong as originally stated; I record the correction rather than quietly rewriting the
prediction.

**Verdict divergence, real and separate from numeric divergence.** Same search: 100 cases where the
final stored value agrees but whether the mechanism is recorded as having overflowed does not. The
mechanism: a directed mode (`floor` in the recorded examples) can round a value that genuinely
exceeds the boundary down to exactly the boundary before the overflow check ever sees it, so
"clamp-then-round" reports overflow (the exact value did exceed) while "round-then-clamp" does not
(the rounded value landed exactly on the boundary, not past it). Concrete instance: `F=1`, `floor`,
`exact=73/24 ~ 3.04`, boundary `hi=3`; round-then-clamp reports no overflow and stores `3`;
clamp-then-round reports overflow and stores `3`. Same stored value, different verdict.

```
holds for: single fixed target width, overflow policy = saturating, boundary = grid-aligned at
that width, rounding mode = any of {floor, ceil, trunc, round_half_up, round_half_even}, F in
{1,2,3}, property = numeric agreement between round-then-clamp and clamp-then-round.

separately holds for: same domain, property = overflow-verdict agreement, does NOT hold (100
counterexamples found, concentrated in directed modes rounding toward the boundary from outside).
```

Whether this verdict distinction matters depends entirely on whether arvo's design ever surfaces an
observable "did this saturate" signal separate from the numeric result (a flag, a diagnostic, a
warning per `arvo-toolbox-not-policer.md`'s "warn, never police" posture). If it never does, the
verdict divergence is inert. If it ever does, the order of rounding versus overflow-checking is not
a free implementation choice, and this file is where that gets flagged.

**Numeric divergence, real, but not actually about overflow at all.**
`126_probes/p2_order_of_round_and_clamp.py`, Search B, chaining an accumulator width into a
narrower final width: 8 genuine numeric divergences, all in `round_half_up` and `round_half_even`,
zero in the directed modes. I isolated this in `126_probes/p3_double_rounding_isolated.py` by
removing the overflow bound entirely and reproducing the same divergences with the same mode split
(64 and 48 instances respectively across a wider sweep, zero for directed modes). This means Search
B's numeric divergence is Finding 5's double-rounding phenomenon wearing an overflow-shaped
costume, not a genuine rounding-overflow interaction. My own first framing of this (an
overflow/rounding "order dependence" question) was pointing at the wrong mechanism, and the
corrected finding is narrower and cleaner: rounding and overflow compose without numeric surprise
at a single width; the numeric surprise that does exist comes entirely from chaining two different
widths, and it is a property of the rounding modes alone, present or absent independent of whether
any clamp is involved.

### Answering the brief's checklist directly

- **Is any rounding mode a homomorphism for any operation, over what domain?** At `F=0`, every
  mode is trivially "homomorphic" for `+, -, x` because rounding never engages (Finding 1); this is
  vacuous rather than a genuine achievement. For `F >= 1`, no mode I tested is a homomorphism for
  multiplication (Finding 2), and I gave a structural (not fully formal) argument for why none can
  be. Division, reciprocal, and square root need rounding even at `F=0` and I have not tested
  whether any mode is homomorphic for those in any domain; that is open.
- **Is any rounding mode monotone, and are monotone and homomorphic rounding disjoint the way
  overflow's families are?** All deterministic modes are monotone (Finding 3). They are not
  disjoint from homomorphism in the overflow sense, because homomorphism is not a live option for
  any of them at `F>=1` in the first place; there is nothing to trade away. The real either/or fork
  on this axis is deterministic-monotone versus stochastic-unbiased-not-per-draw-monotone (Finding
  4), a genuine dichotomy but a narrower and differently-shaped one than overflow's.
- **Does rounding interact with the overflow policy at all, or do the two decompose?** Both,
  precisely separated: they decompose numerically at a single fixed width with a grid-aligned
  boundary (Finding 6, first half), they do not decompose on the overflow verdict near the boundary
  for directed modes (Finding 6, second half), and the numeric divergence that appears once two
  widths are chained is not an overflow interaction at all, it is double rounding (Finding 5/6,
  isolated in probe 3).
- **Does the answer differ at `F=0` from `F>0`?** Yes, sharply, for the closed operations: rounding
  is a no-op at `F=0` and non-vacuous at `F>0` (Finding 1 vs Finding 2). It does not differ for
  non-closed operations (division etc.), which need rounding at any `F` including zero; untested by
  me, flagged as open.
- **What does rounding do to the ordering properties the preceding topic leans on?** Nothing, when
  overflow is saturating: composition of monotone functions is monotone, so the composed map stays
  monotone regardless of rounding mode (deterministic). When overflow is wrapping, rounding cannot
  rescue the non-monotonicity that wrap already has (Finding 6, first paragraph).
- **Is round-half-to-even structurally different from the directed modes, rather than merely
  fairer?** Yes on bias (real, measured, large on adversarial ties), no on double-rounding safety
  (it groups with round-half-up, both vulnerable, against the directed modes, which are immune)
  (Finding 5). Treating it as a single undifferentiated alternative to "truncation" loses this
  split.

### What I am carrying forward, what I am refuting, and what is open

I have read no prior file on this topic, so there is nothing of a predecessor's to carry forward or
refute; the two things I refuted in this file are my own predictions, stated before running the
probes that refuted them (Prediction 5's first construction, Prediction 8 as originally framed),
and I kept both refutations visible rather than only keeping the corrected version, per the
prediction-then-report discipline this dispatch asked for.

**Open, for the reconciliation pass or a later member:**

1. Whether any rounding mode is a homomorphism for division, reciprocal, or square root, in any
   domain, at any `F` including zero. Untested.
2. A rigorous (not merely exhaustive-small-grid) proof that no rounding mode is ever associative
   for multiplication at `F >= 1`, in the shape of an explicit counting argument. I gave the
   informal shape of one; it wants formalising.
3. Whether stochastic rounding's monotone-vs-unbiased trade is a genuine, provable dichotomy (no
   function can have both properties simultaneously) or merely a fact about the constructions I
   tried. I exhibited the trade; I did not prove the exclusion.
4. Whether a position-keyed (dithered) stochastic rounding, which restores monotonicity as a
   function of value at a fixed position while still breaking it across positions, is a design
   option worth naming as a fourth or fifth mode alongside the five I tested, given I15's "never a
   runtime check" and I18's build-bound overflow-panic precedent both suggest arvo is comfortable
   with build-time-selected, non-uniform-across-the-codebase behaviour where it earns its keep.
5. The magnitude of directed-mode bias on a realistic (non-adversarial, non-all-ties) input
   distribution. I measured the adversarial worst case; I did not measure a representative case.
6. Whether the overflow-verdict divergence (Finding 6) matters to arvo's actual design, which
   depends on whether a saturation signal is ever surfaced as observable state, which I do not know
   and did not find stated anywhere in `INTENTS.md`.

**Vocabulary, as asked.** I would settle on truncation as the prose noun (matching "floor" and
"ceiling" as full words rather than abbreviations) and reserve trunc for a short
code-identifier-style token where one is wanted, rather than mixing the two in prose. This is a
light preference, not a ruling; per I16 the canon does not police shape, and this is smaller than
that, it is just spelling consistency.

### Coverage

I read `INTENTS.md` and `RULES.md` in full, wrote and ran four probe scripts from scratch, and
derived every finding above from those probes plus first-principles reasoning about grid
arithmetic, order theory, and floating-point/fixed-point rounding literature I already know. I did
not read any panel file, any register, any prior probe, or the git log before writing this
paragraph. I did not test signed arithmetic (everything above is unsigned), and I did not test any
interaction between rounding and the strategy markers themselves (`Hot`/`Warm`/`Cold`/`Precise`),
since I13/I17 leave the strategy set open and nothing in my brief asked me to presume a
decomposition. Both are gaps a reconciliation pass should weigh.

---

## Phase two. Reconciliation against `122`, `119` section 5, and `116` section 7.

Read after phase one was committed and not before. Phase one above is unedited from this point.

### The question was already asked, and left explicitly open

`119:598` states it almost verbatim: "Whether the rounding mode has the same character-selecting
role as the overflow behaviour. `116` section 7 names it as the first place it would look. Nothing
measured it, and section 4.4 lists rounding at a fixed value everywhere for that reason." `116:486`
is the pointer: "I would look at the rounding mode first, because section 6.1 shows it owns a
region of the map with its own algebraic character." I have not read section 6.1; my brief did not
assign it and I am flagging the gap rather than reading past my dispatch. Whatever "its own
algebraic character" names there, I cannot say whether my findings already cover it or add to it.

### Where my brief's background was the superseded framing, not the current one

My brief told me, as given background: "A wrapping map is a ring homomorphism and is not monotone.
A saturating map is monotone on a suitable domain and is not a ring homomorphism. No map onto a
finite value set is both, except a constant one." Reading `122` now, this is `119`'s **original**
4.2 and 4.4, both explicitly `[REPLACED]` rather than `[STANDS]`. The corrected statement, forced by
`121`'s counterexample and reproduced at `122` F122-2 and F122-4: the mutual exclusion holds **only
on a domain closed under negation**. On a one-signed domain, a saturating map is a homomorphism for
addition and multiplication (not subtraction) **and** remains monotone, at once, with no
declaration needed. The brief's "monotone on a suitable domain" phrase already gestured at
domain-sensitivity on the monotone side; it did not carry the same hedge on the homomorphism side,
and that is exactly the half `122` corrected. I record this as a fact about which version of the
finding reached my brief, not as an error to relitigate; `122` is the panel's own current, two-way
dissented-and-reconciled state and outranks what I was handed as background.

This matters directly for how my own findings should be read, because it sharpens rather than
weakens them.

### My grid was already one-signed, and rounding still fails there

`122`'s central discovery is that the domain-and-operation-set pair `(one-signed, {add, mul})` is
exactly where saturating overflow escapes the exclusion and gets both licences at once. My probe's
grid function (`126_probes/p1_rounding_character.py`, `grid(F, kmax)`) generates `k/2^F` for
`k` in `range(0, kmax+1)`, which is non-negative only. **Finding 2's counterexamples to associativity
and distributivity of rounded multiplication were already measured on a one-signed domain, using
exactly the operation set `{+, x}` that rescues overflow's homomorphism.** Rounding is not rescued
there. `a=1/2, b=1/2, c=2` at `F=1` under `round_half_even`, all three values non-negative, still
gives `(a[X]b)[X]c != a[X](b[X]c)`.

So the sharper, corrected version of my answer to the brief's question is not merely "no rounding
mode is a homomorphism at `F>=1`" (which is what phase one said); it is: **the specific
domain-conditional escape that rescues overflow's homomorphism does not exist for rounding's.**
Rounding fails to license deferral for multiplication in the exact region where overflow succeeds.
This is a stronger and more precisely targeted claim than phase one made, and it is the direct,
corrected answer to `119:598` and `116:486`.

```
holds for: F >= 1, operation = multiplication, domain = one-signed (non-negative), the domain
region where saturating overflow is a homomorphism per 122 F122-4, rounding mode = any of
{floor, ceil, trunc, round_half_up, round_half_even}, width = swept exhaustively at kmax=6.
property = NOT a homomorphism, no domain restriction found (or looked for beyond one-signed)
that rescues it.
```

I have not tested whether some **narrower** domain restriction (a bound tighter than "non-negative",
analogous to `122` 4.5's "declare a restriction" mechanism, which discharges by making the map the
identity on the restricted range) rescues rounding's homomorphism. That is a real and different
question from the one-signed test above: `122` 4.5 says a restriction where the map does not move at
all (the identity there) trivially has both characters, for **any** behaviour, rounding included,
since an identity map is trivially homomorphic and monotone. Whether that is the *only* way to
rescue rounding (paralleling `122`'s finding that a declared restriction is not overflow's only
mechanism, since the one-signed domain is a second) is open and I did not chase it.

### `122` 4.6's "grid part must always be applied" is the same claim as my Finding 2, stated as a rule rather than measured as a sweep

`122` 4.6, unchanged from `119`, splits a reduction into a **grid part** (applied at the result of
every node whose exact result can leave the grid, meaning: rounding) and a **range part** (applied
at the operands of every node the map is not a homomorphism for, meaning: overflow discharge, which
`122` section 2 shows can be deferred to the root under the domain-conditional escape). The clause
states the grid part **must** be applied at every such node, with no exemption listed anywhere in
section 4, in contrast to the range part, which gets an explicit domain-conditional exemption in
4.5 and 4.7.

I read this as the same claim my Finding 2 makes, in the panel's vocabulary rather than mine, and
asserted rather than measured the way I measured it. My probes give it independent, exhaustive
small-grid evidence from a direction nobody in the `114`-through-`122` chain built: not a
term-shape sweep over declared bounds and discharge certificates, but a bare associativity and
distributivity check on the rounded operation itself, with the domain deliberately matched to the
region where the range part's exemption is at its widest. I record this as **independent agreement
with 4.6's grid-part rule**, arrived at before I read it, on a different instrument (my own
exhaustive multiply sweep versus whatever established 4.6, which I have not read and do not cite).
Whether this is enough for a TWO EXPERTS rung on 4.6's grid-part clause specifically is not mine to
call; I flag the instrument and the independence and leave the rung to whoever consolidates.

### Where my Finding 1 independently reproduces `119`'s C2 exemption

`122` 4.4 (carrying `119`'s unchanged C2 answer forward): "the wrapping map's addition and
subtraction half holds at **any** fraction width, on the argument that the grid is closed under
those operations so they never enter the rounding region." This is exactly my Finding 1, derived
independently, before reading either file: addition and subtraction of grid points never need
rounding at any `F`, because the sum (or difference) of two grid points is always itself a grid
point. I built this from first principles about how a fixed-point grid is closed under `+`/`-`; C2
states the identical fact as the reason a different clause (the wrapping homomorphism's fraction-
width independence) is exempt from needing a width sweep. Two different investigations landing on
one fact, from different directions, is the shape `RULES.md` asks a reconciliation to name plainly
rather than bury: this is a genuine second, independent derivation of the same underlying grid
closure fact, not a citation of one by the other.

### The order-licence family is never at risk from rounding, for any deterministic mode, at any width or domain: worth stating more strongly than phase one did

`116`'s order-preserving licence family (`8`, third bullet: "any construction or rewrite resting on
the map preserving order, which includes comparison-carrying composites, min and max folds, and
clamping reassociation") is exactly what my Finding 3's monotonicity result protects. Having now
read that this licence family is treated in the panel's framework as something a design can lose
(by choosing wrapping) or gain conditionally (saturation, domain-dependent per `122`), I want to
restate Finding 3 more sharply than phase one did: **rounding never threatens this licence family,
for any deterministic mode, at any `F`, at any domain, at any width.** Every deterministic mode is
monotone unconditionally (Finding 3's sweep found no violation and my argument for why is
structural, not domain-dependent: a nearest-or-directed rule is weakly increasing by construction).
This is the one place where rounding is strictly *safer* than overflow with respect to a licence
family the panel already cares about: overflow's order licence has to be earned (saturating, and
even then only unconditionally; wrapping never has it), while rounding's is free by default and
only lost by choosing stochastic rounding.

### Double rounding is a hazard the panel's carrier/discharge apparatus has not yet named, as far as I have read

`122` 4.6's grid-part rule, and `119`/`122` section 4.10's carrier-cost accounting (recursion depth,
associated-const cells, per-node state), are both framed around a **single** target width per
derivation. My Finding 5/6 (double rounding: `round_half_up` and `round_half_even` disagree between
"round at accumulator width, then round again at final width" and "round directly at final width",
zero disagreements for `floor`/`ceil`/`trunc`, confirmed with overflow bounds entirely removed to
isolate the effect) is about what happens when a derivation's carrier changes width partway through,
which is exactly the situation arvo's own strategy markers create routinely (a wider accumulator
narrowing into a typed result). I did not find this named in 119 section 5, `122`, or `116` section
7, and I do not know whether it is covered by material I was not assigned (`116` section 6.1 in
particular, or the carrier-cost sections 4.10/8 of `119`/`122`, which I read only in the reproduced
form 122 carries and not at their `114`/`115` sources). I flag it as new rather than claim novelty I
cannot check: **a carrier that changes fraction width partway through a derivation is a second place
the grid part's "must always apply" rule needs a qualifier, and the qualifier is mode-dependent
(directed modes are safe under width-chaining; nearest-family modes are not), which is a genuinely
different axis from the domain-sign qualifier `122` found for the range part.**

### What I am carrying forward, and what remains genuinely open after this reconciliation

**Carried forward unchanged from phase one, now with tighter grounding:** Findings 1, 3, 4, 5, and
6 all stand as measured; nothing in `122`/`119` section 5/`116` section 7 contradicts any of them.
Finding 2 is sharpened (see above) rather than corrected.

**Refuted nothing of a predecessor's**, since the material I was assigned to read agrees with
everything I derived and, where it overlaps at all, states the same facts in a different vocabulary
(4.6's grid part, C2's width exemption) that I reached independently first.

**Genuinely open, now including what the assigned reading surfaces on top of phase one's list:**

7. Whether `116` section 6.1's "region of the map with its own algebraic character" for rounding is
   the same territory my findings cover, a subset, or something I have not touched. Unread by me
   under this dispatch's scope.
8. Whether a restriction narrower than "one-signed" (paralleling `122` 4.5's non-domain mechanism)
   rescues rounding's homomorphism for multiplication anywhere short of the trivial identity case.
9. Whether double rounding across a chained-width carrier belongs in `122` 4.6's grid-part rule as
   a stated qualifier, or is a separate clause entirely, and whether it interacts with the
   recursion-depth carrier-cost accounting in section 4.10 the way the domain-sign condition
   interacts with the discharge-check certificate in 4.7.
10. Whether my brief's background statement of the overflow disjointness theorem (pre-`122`, no
    domain qualifier on the homomorphism half) reaching me as "given" rather than as the panel's
    current state is a pattern worth a standing note for how backgrounds get written into future
    dispatch briefs on this topic; not mine to fix, only mine to have hit and to say so.
