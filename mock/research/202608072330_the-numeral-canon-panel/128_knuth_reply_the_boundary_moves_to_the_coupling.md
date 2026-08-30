# 128. Reply to the attack: F8 falls as stated, and the boundary moves to the coupling

**Member:** Knuth, resumed with `125` still in context, per the reply step of `113`. Read for this
file: `127_dolan_attacking_the_convergence.md` in full, `127_probes/w1` source and output,
`126_wronski_what_rounding_actually_selects.md` in full, and nothing else new.

**Canon gate:** passed. Nothing below conflicts with an entry in `INTENTS.md`; the whole reply is
I13's discipline applied to my own finding, a dimension found missing from a predicate and the
region restated with it named. **Test gate:** inherited from my own run as `125` in this sitting
(12 of 13 crates green at 108 tests, the thirteenth's non-termination documented in `125` section
10), the crates tree unchanged since, and per the dispatcher's disk note (under 2 GiB free) nothing
was rebuilt; every instrument here is exact rational arithmetic in `128_probes/`, committed with
outputs as each ran, two refuted-prediction runs preserved.

**The verdict, stated first.** I reproduced `127`'s construction on my own instruments before
judging it, and it holds exactly as claimed. **I concede F8's second clause and F8's placement of
the boundary, and I do not merely withdraw: the boundary is real and it sits somewhere more precise
than either `126` or I put it, with a uniqueness theorem underneath it that makes `127`'s
construction canonical rather than merely a counterexample.** Sections 3 through 5 are the
synthesis; it goes further than the attack did, which is what a reply owes an attack that is right.

---

## 1. The reproduction, on my own instruments

`128_probes/r1`, exact Fractions throughout, expectation computed as interval measure over `U`
rather than asserted from the formula under test:

- **Monotone on every realisation.** Seven thresholds including two near the extremes and one at
  the tie value, over a negation-closed window: zero violations in 448 adjacent pairs each.
- **Exactly unbiased.** The measure of `{U : Q_U(x) = ceil}` is `frac(x)` on the nose, so
  `E[Q_U(x)] = x`, verified at five points including a negative, a tie, and an on-grid one.
- **Composed with saturate, still monotone on every realisation.** Asymmetric signed range
  `[-8, 7]`, seven thresholds, zero violations in 736 pairs each. **F8's second clause is
  contradicted on my own instrument**, which is the standard the reply step requires.
- **The coupling is the thing my old instrument presupposed.** The same probe recomputes my P5's
  inversion probability both ways: `1/25` under independent draws, which is a **product of
  marginals**, and exactly `0` under the shared threshold, where the inversion event is the empty
  interval `[f_y, 1) ∩ [0, f_x)`. My P5 encoded the independent coupling in its arithmetic and
  never named it as a dimension. That is precisely the failure I13's notation exists to prevent,
  committed by me in the same file that lectured about absent dimensions.

Two additions the reproduction yielded beyond confirmation:

**The wrap gap `127` left unrun is closed, and in the family's favour.** `Q_U` has
`frac(x + q) = frac(x)`, so every realisation is translation-equivariant by `q` and commutes with
wrap **mod span**: zero quotient mismatches at `W ∈ {3, 4}`, both signednesses, both thresholds
tested. Representative-level mismatches behave like the deterministic family's: 105 at `U = 1/3`
(ceil-like top-cell escapes) and zero at `U = 96/97` over this window, because a threshold above
the finest frac present makes the top cell floor-like. `Q_U` therefore interpolates between ceil's
and floor's representative behaviour as `U` rises, which my prediction 4 half-missed (it said
"nonzero" without conditioning on `U` against the window's resolution; the zero at high `U` is the
floor limit showing through, not an instrument defect).

**A tie correction to F127-2.** Under `127`'s own convention (`frac <= U` rounds down), the
midpoint `U = 1/2` is **half_down at ties**, not half_up: at eight tie points `Q_(1/2)` agrees with
half_down on all eight and with half_up on none, while agreeing with half_up on all 435 non-tie
points. F127-2's "is exactly `half_up`" holds off ties and fails on them. Nothing in `127`'s
argument rests on the tie case, so this is precision, not refutation.

## 2. What I concede, exactly

**F8's second clause is false as stated.** "A saturating realisation composed with stochastic
rounding is ejected from the monotone family" holds only for the independent-per-element coupling
and fails for the shared threshold, on `127`'s instrument and on mine.

**F8's boundary placement is wrong.** The deterministic/stochastic line is not where the trade
sits. `126` hedged this honestly (its open item 3 says the exclusion was exhibited, not proved) and
I stated it flat; of the two, mine was the worse epistemic conduct, and `127` was right to note the
difference.

**F8's first clause survives, coupling-invariantly.** `E[Q(x)] = x` forces each element's marginal
and is untouched by the joint law, so expectation-exactness holds for every member of the family,
`127`'s included. And T1/T1b still stand under it: no realisation is exact off-grid; exactness
lives only in the mixture. `127` says the same in its Part 3 control and I confirm.

## 3. The synthesis: unbiasedness fixes the marginals, and everything else is the coupling

Here is the frame that makes the whole family legible, and it is the reply's main content.

An unbiased stochastic rounding assigns each `x` the indicator `B(x) = [Q(x) = ceil(x)]`, and
unbiasedness **forces the marginals**: `B(x) ~ Bernoulli(frac(x))`, no freedom at all. Everything
that distinguishes stochastic schemes is the **joint distribution**, the coupling, and the
properties `126`, `127` and I have been arguing about are properties of the coupling:

**Realisation-monotonicity is exactly within-cell comonotonicity.** For `x < y` in the same cell,
monotonicity needs `B(x) <= B(y)` almost surely. For `x, y` in different cells it is automatic:
`Q(x) <= ceil(x) <= floor(y) <= Q(y)`. So the constraint binds within cells and nowhere else.
`128_probes/r2` part 2 verifies the cross-cell half across the whole Fréchet interval: zero
positive-probability inversions at every admissible joint law.

**Within one cell the trade is total and exact.** For a same-cell pair with fracs `f_x < f_y`, the
joint law has one free parameter `p11 = P[B_x = 1, B_y = 1]` in the Fréchet interval
`[max(0, f_x + f_y - 1), min(f_x, f_y)]`, and the inversion probability is exactly `f_x - p11`:
zero at the comonotone corner `p11 = f_x` and nowhere else. The independent coupling
`p11 = f_x f_y` sits strictly inside whenever both fracs are interior, so **unbiased plus
per-realisation monotone plus within-cell independence is impossible**, not merely untested.
`r2` part 1 sweeps four pairs across thirteen Fréchet points each: the zero is unique every time.

**And the within-cell law is unique, which makes `127`'s construction canonical.** A
realisation-monotone rounding's round-up set within a cell must be an upper set, and a cell's upper
sets are suffixes, indexed by a threshold. The forced marginals then determine the threshold's
distribution through a triangular system whose unique solution is **uniform**. `r2` part 3 solves
it exactly at `m = 8` subpoints (each threshold weight exactly `1/8`), with two controls: a
non-suffix up-set is non-monotone on its own realisation, and a perturbed threshold distribution
misses a marginal. So `127` did not find "a member of a family that happens to work"; within a
cell it found **the only thing that can work**, and the entire remaining design space of unbiased
realisation-monotone rounding is the coupling of per-cell uniform thresholds across cells.

**What F8 becomes**, then, and this is the answer to the dispatcher's question about where the
boundary really is: the either/or that `126` and I both saw is real, and its seat is the
**within-cell coupling**, with determinism never having been the relevant side of it. Order on
every realisation is bought by within-cell comonotonicity; error decorrelation is bought by
independence; within one cell those are the two ends of one Fréchet parameter and cannot be held
together; across cells they can, which none of the three of us had said.

## 4. The price the attack did not measure, which is why the trade looked free

`127` writes that the correlated construction "dominates the independent one on every property
either file measured". That sentence is true, and the reason it is true is that neither file
measured the property independence exists to buy. `128_probes/r3` part 1 measures it, exactly:

For `n` same-cell elements at frac `f`, the summed rounding error has variance **`n² f(1-f)`**
under the shared threshold against **`n f(1-f)`** under independent draws, both verified to match
the closed forms exactly by full enumeration up to `n = 10`, with the `n = 1` control showing the
distinction needs a pair. Coherent errors add linearly; independent errors add in quadrature. The
accumulated-error growth of `O(sqrt(n))` that makes independent stochastic rounding attractive for
long chains, which is I7's territory, is a property of the independent coupling **exclusively**,
and the shared threshold gives it up entirely: every same-frac element moves the same way on every
draw.

So the trade F8 misplaced did not vanish under `127`'s attack. It relocated and became
quantitative: **order on every realisation against concentration of accumulated error**, at equal
and exact unbiasedness. "Dominates" must not be read as Pareto dominance; each end of the coupling
is the optimum of a different objective, which is I13's arms-not-winners shape appearing at a place
none of the three files predicted it.

## 5. The family map, and one more member nobody had named

The coupling frame sorts every construction in the three files, plus one new one:

**Global shared `U`** (`127`'s): fully comonotone. Monotone on every realisation, unbiased,
`q`-translation-equivariant per realisation, wrap-compatible mod span (r1 part 4), saturate-
compatible. Pays maximal error coherence: the `n²` variance law. One draw per pass.

**Per-cell thresholds, independent across cells** (new here, `r3` part 2): monotone on every
realisation for every threshold tuple (243 tuples swept, zero violations), exactly unbiased,
within-cell covariance forced positive (`25/256` at the tested pair, the irreducible coherence),
**cross-cell covariance exactly zero**. For accumulations whose addends spread across many cells,
the coherent term shrinks toward the independent law while every realisation stays monotone. The
price, and my prediction about it was half wrong in an instructive way: `q`-translation
equivariance is lost (r3 part 3 exhibits `Q(x + q) != Q(x) + q`), and wrap compatibility is lost
**only if the threshold table distinguishes cells within one residue class**. My first tuple
alternated by cell parity, the span was even, and wrap commuted perfectly; the refuted prediction
is preserved in `r3_output_run1`. The sharpened statement, both directions verified in r3 part 3b:
**a per-cell table commutes with wrap mod span iff it is constant on residue classes mod span**,
and in wrapped semantics residue-keyed is the natural keying anyway, since the quotient only has
span-many cells. So the per-cell member keeps wrap compatibility for free by keying on the residue,
at the cost of re-cohering cells that share a class.

**Independent per element** (`125` P5, `126` p4, `127`'s Part 1 control): unbiased, minimal
accumulation variance, realisation-monotonicity gone, per the Fréchet theorem necessarily rather
than accidentally.

**Position-keyed dither** (`126`'s aside, not built by anyone): the threshold depends on the
element's position rather than its value, so the realised map is not a function of value at all and
"monotone" only makes sense per position. In the coupling frame: it decorrelates by position the
way per-cell decorrelates by value cell, and it sits outside the realisation-monotone class
whenever two positions carrying same-cell values get different thresholds. `126`'s instinct that
"stochastic" was underspecified was exactly right, and the specification it wanted is the coupling.

**The deterministic modes** are the degenerate members: point-mass thresholds (floor at `U -> 1`
limit, ceil at `U = 0`, half_down at `U = 1/2` exactly, per the tie correction), monotone, biased,
entropy-free. The five-mode set and the stochastic family are one picture: a distribution over
thresholds, with determinism as the point masses and unbiasedness as the uniform mixture.

**What no member escapes:** entropy at runtime. One draw per pass, span-many draws, or one per
element is a cost gradient, but every non-degenerate member needs a runtime draw, so the
const-availability concern in `125` F8's coda stands for the whole family unchanged, and `127`
says the same.

## 6. Findings, each with its predicate, the coupling now a named dimension

> **F128-1 (concession and replacement).** `125` F8's second clause is false as stated and its
> boundary misplaced. The corrected statement: an unbiased stochastic rounding is monotone on
> every realisation iff its within-cell coupling is comonotone; the within-cell realisation-
> monotone unbiased law is unique (uniform threshold per cell); and composition with saturate
> preserves per-realisation monotonicity for every comonotone-within-cell member.
>
> holds for: signedness any, F any, I any, grid infinite for the theorem and the swept windows
> named in `128_probes/` for the enumerations (negation-closed), rounding = the unbiased
> stochastic family, coupling ∈ {comonotone within cells: monotone on every realisation;
> any coupling with within-cell independence at interior fracs: monotone on no
> realisation-almost-surely, inversions with positive probability}, overflow = saturate for the
> composition clause, threads = 1 for the enumerations, threads any for the Fréchet and
> uniqueness arguments (properties of distributions, not executions).

> **F128-2 (the impossibility that survives).** Within one cell, unbiased + monotone-on-every-
> realisation + pairwise independence is impossible at interior fracs: inversion probability is
> exactly `f_x - p11`, uniquely zero at the comonotone corner, and the independent coupling is
> interior. Across cells every coupling is monotonicity-safe.
>
> holds for: fracs interior to (0, 1) with f_x < f_y for the within-cell clause, any cell pair for
> the cross-cell clause, coupling any point of the Fréchet interval (swept at 13 points per pair,
> 4 pairs), threads any (exact distribution arithmetic).

> **F128-3 (the price).** Summed-error variance for n same-cell elements at frac f: exactly
> `n² f(1-f)` comonotone, exactly `n f(1-f)` independent. The concentration property of stochastic
> rounding over chains belongs to the independent coupling exclusively.
>
> holds for: n ∈ 1..10 enumerated exactly and the closed forms proved by the enumeration matching
> them at every n, f = 1/3 swept (the closed forms are algebraic in f; other fracs not enumerated),
> coupling ∈ {comonotone, independent}, threads any (exact distribution arithmetic).

> **F128-4 (the per-cell member and its wrap law).** Per-cell independent thresholds keep
> realisation-monotonicity and unbiasedness, zero cross-cell error covariance, forced positive
> within-cell covariance; translation equivariance by q is lost; wrap commutation mod span holds
> iff the threshold table is constant on residue classes mod span, and residue keying is the
> natural per-cell definition under wrapped semantics.
>
> holds for: W ∈ {3} for the wrap sweeps and the five-cell window at E = 4 subquanta for
> monotonicity (243 tuples), signedness = unsigned for the wrap table sweep (the periodicity
> argument is signedness-free; only unsigned was enumerated), coupling = per-cell thresholds,
> threads = 1 for the enumerations.

> **F128-5 (tie correction to F127-2).** Under the `frac <= U` convention, `Q` at `U = 1/2` is
> half_down at tie points and half_up elsewhere; F127-2's midpoint identification holds off ties
> only.
>
> holds for: the identification is definitional; enumerated at 8 tie and 435 non-tie points,
> E = 5, window [-7q, 7q], threads = 1.

## 7. What I carry forward unchanged, with counts

**From `127`, eight things.** Its six carried items (my T1/T1b and F2 and F4 and the F7-corrected
commutation and the vocabulary confirmation, each re-stated there with instruments), adopted back
unchanged; F127-1, now reproduced on a second independent instrument (r1), so the shared-threshold
construction's properties carry two instruments from two members; and F127-2 as corrected by
F128-5, which changes its tie clause and nothing else.

**From `126`, and which agreements were independent now that I have read it.** Five convergences
between `126` and my phase one were reached blind by both of us, with the commit ordering as the
audit trail: the vacuity of rounding for grid-closed operations (my F4, its Finding 1), universal
deterministic monotonicity (my T2, its Finding 3), non-additivity off-grid (my T1, its Finding 2's
measured half), the double-rounding split of nearest against directed modes (my T4/P4, its Finding
5, on two genuinely different instruments), and saturate-composition preserving monotonicity (my
T6, its Finding 6 first half). Those are independent instances in `RULES.md`'s sense, and `127`
carries four of them with the same reading. Three things of `126`'s I did not derive and take by
reading only, at their stated rungs: its tie-bias magnitude measurement (its Finding 5, adversarial
all-ties construction), its overflow-verdict divergence (its Finding 6 second half, a genuinely
new observable I had not considered, inert unless a saturation flag ever becomes observable), and
its position-keyed dithering aside, which section 5 above places in the coupling frame. And one
disagreement to record rather than smooth: `126`'s vocabulary suggestion (keep "truncation" as
prose, `trunc` as token) predates the evidence, which `127` and I independently confirmed at
source, that the word is ambiguous on signed domains between two distinct modes; my `toward_zero`
recommendation stands, and the disagreement is now between two files that have both seen the same
facts, which is exactly what a consolidation should weigh rather than what I should settle here.

**Untouched by the attack, and not relitigated:** T1/T1b and the divisibility obstruction, the
region decomposition with the toward_zero exception, F4's vacuity and its widening rule, F6's
valuation predicate, F9's bit-drop identity. One composition update rather than a change: the
comonotone family joins the translation-equivariant modes in the wrap-compatible class (mod span,
per realisation), the per-cell member joins it under residue keying, and the independent coupling
remains outside the per-realisation order story entirely. The entropy/const concern of F8's coda
transfers to the whole family unchanged.

## 8. Coverage, bounded

Read in full: `127`, `126`, `127_probes/w1` source and output. Not read: anything else new; my
account of `114` through `122` remains what `125` phase two recorded. Built: three probes, six
outputs, two preserved refuted-prediction runs (r3's periodic-tuple wrap zero, reclassified by
3b; and the r1 representative-count refinement at high U, noted inline). Not done, named
honestly: no enumeration of the variance law at fracs other than 1/3 (the closed forms are
algebraic and the n-scaling is the claim); no signed wrap sweep for the per-cell table (the
periodicity argument does not read the sign, but only unsigned was enumerated, so F128-4's
predicate says so); no measurement of the cost of one draw against span-many against per-element,
which is a bench question and stays unpriced; and no position-keyed construction was built, `126`'s
aside remaining unbuilt by all three of us.
