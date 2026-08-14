# 125. The rounding axis, derived cold

**Member:** Knuth. **Phase:** one, cold. Read before writing this: `INTENTS.md`, `RULES.md`, and the
dispatch brief. Nothing else: no panel file, no register, no probe of anyone else's, and no commit log,
per the independence protocol. The one fact about the preceding topic I was handed by the brief: the
overflow policy selects a structural family, wrapping maps are ring homomorphisms and not monotone,
saturating maps are monotone and not ring homomorphisms, and every predicate in that topic pins
`rounding = trunc`.

**A correction to the brief, received from the dispatcher mid-derivation and before my phase-one
commit.** The brief stated the overflow impossibility unconditionally: "no map onto a finite value set
is both a ring homomorphism and monotone, except a constant one." The dispatcher has since corrected
this: that is the superseded framing, and the corrected form is **domain-conditional**. The
impossibility holds on domains **closed under negation**; on a one-signed domain a saturating map can
be an additive and multiplicative homomorphism and monotone at once, and a concrete counterexample
(sixteen values over `0..47`) exists. I record this here per the dispatcher's instruction that the
error leave a trace, and I note the timing: the correction is part of my phase-one reading set, it
arrived before I committed, and I folded it in rather than receiving it with phase two. Section 2
examines what it does to my own theorem, and the answer turns out to sharpen the derivation rather
than dent it.

**Canon gate:** passed. The question, whether the rounding mode selects character, is inside I13's
programme (predicated arms over regions, universal answers rejected) and conflicts with no entry in
`INTENTS.md`. No intent pins a rounding mode. I7 (accuracy across chains) and I8 (strategies weigh
measurements differently) make the rounding axis a live concern rather than an idle one. I15 is not
touched: everything below is about compile-time-decidable structure.

**Test gate:** run per crate under `mock/benches/variants/`, as the brief instructs, because the
workspace-wide invocation returns a false green. Results recorded in the section "The test gate's
result" below.

**What this file suggests, it suggests.** Op ratifies; nothing here settles anything.

---

## 1. The setting, fixed precisely so the claims can be exact

A fixed-point format has integer width `I`, fraction width `F`, a signedness, and total width
`W = I + F`. Its quantum is `q = 2^(-F)`. Two sets matter and they are different:

- the **grid** `G = q·ℤ`, infinite, the set of values the format could name if it had unbounded range;
- the **representable set** `R = G ∩ [m, M]`, finite, where `[m, M]` is the format's range.

A realisation map takes an exact value (I take the domain to be `ℚ`, which is enough; `ℝ` changes
nothing below) to a representable value. It factors through two decisions:

- **quantisation** `Q : ℚ → G`, what to do when the exact value falls between grid points. This is
  the rounding mode.
- **range policy** `O : G → R`, what to do when a grid value falls outside `[m, M]`. This is the
  overflow policy, the axis the preceding topic classified.

The modes I admit, named to avoid a trap that section 7 is about: `floor` (toward negative infinity),
`ceil` (toward positive infinity), `toward_zero`, `half_up` (nearest, ties toward positive infinity),
`half_even` (nearest, ties to even), and `stochastic` (`floor` with probability `1 - f`, `ceil` with
probability `f`, where `f` is the fractional position within the cell). The first five are functions;
the sixth is a kernel, a function into distributions.

Every mode is a **retraction onto the grid**: `Q(g) = g` for `g ∈ G`, and `Q ∘ Q = Q`. That is what
makes it a rounding and not something else.

## 2. The first theorem: rounding has its own impossibility result, and it is unconditional

The overflow dichotomy rests on a nonexistence theorem, and the dispatcher's correction establishes
that that theorem is domain-conditional: it holds on negation-closed domains and fails on one-signed
ones, where saturation collects both properties. Rounding has a parallel theorem. It is worth stating
carefully, because it turns out to be **stronger in exactly the respect the overflow theorem turned
out to be weaker**: it does not care whether the domain is negation-closed.

**T1 (no exact additive section, negation-closed form).** There is no additive map `Q : ℚ → G` with
`Q(g) = g` on `G`.

*Proof.* `ℚ` is a divisible group. The image of a divisible group under a group homomorphism is
divisible. `G ≅ ℤ` as a group, and the only divisible subgroup of `ℤ` is `{0}`. But the retraction
condition forces the image to contain all of `G`. Contradiction. ∎

**T1b (one-signed form).** There is no additive map `Q : ℚ≥0 → G ∩ [0, ∞)` with `Q(g) = g` on the
nonnegative grid, either. `ℚ≥0` is only a monoid, so divisibility must be run by hand: additivity
gives `Q(x) = Q(n · (x/n)) = n · Q(x/n)` for every `n ≥ 1`, so `Q(x)` is an element of `q·ℤ≥0`
divisible by every positive integer, which forces `Q(x) = 0` for all `x`, contradicting `Q(q) = q`. ∎

Concretely at the smallest case: `Q(q/2) + Q(q/2) = Q(q) = q` would force `Q(q/2) = q/2`, which is not
on the grid. No sign is involved.

So the two impossibility theorems have **different obstructions with different domain sensitivities**.
The overflow theorem's obstruction is order-theoretic and additive-group-shaped (a finite additive
group admits no infinite ascending chain), and restricting to a one-signed domain removes the tension:
saturation slips through. The rounding theorem's obstruction is **divisibility**: the domain is finer
than the grid, halves of grid steps exist on both one-signed and two-signed domains, and no restriction
of sign relieves it. Where the overflow axis offers a domain-conditional two-family choice, the
rounding axis offers no homomorphic family at all among functions, on any domain dense enough to need
rounding. **The hom-versus-monotone dichotomy does not partition the deterministic rounding modes; it
degenerates, unconditionally.** If rounding selects character, it must do so over different
properties. Sections 4 and 5 say which.

> **F1.** No deterministic rounding mode is an additive homomorphism off the grid, on the
> negation-closed domain and on the one-signed domain alike; and all modes are additive on the grid
> trivially, because there they are the identity.
>
> holds for: signedness any, F any, I any, grid infinite (range not involved), rounding ∈ {floor,
> ceil, toward_zero, half_up, half_even}, op = +, domain ∈ {ℚ, ℚ≥0} (the first closed under negation,
> the second one-signed, claimed separately and each proved), threads any (proved property of
> functions; probe verification at threads = 1, sweep named in `125_probes/`).

## 3. The second theorem: monotonicity does not discriminate either

**T2.** `floor`, `ceil`, `toward_zero`, `half_up`, `half_even` are all monotone nondecreasing on `ℚ`.

*Proof sketch.* `floor` and `ceil` are monotone as adjoints (T3). `toward_zero` agrees with `floor` on
nonnegatives and `ceil` on negatives, and the two pieces join monotonically at zero.
`half_up(x) = floor(x + q/2)`, monotone as a composition. `half_even` is a nondecreasing step
function: within each half-cell it is constant, and at every boundary, tie points included, the value
steps up or stays, never down, whichever parity the tie resolves to. ∎

`stochastic` is the exception, and it fails pointwise: for `x = 0.2q < y = 0.8q` the outcome
`Q(x) = q, Q(y) = 0` has positive probability, an inversion no deterministic mode can produce. Its
compensation is T9 in section 6.

> **F2.** All deterministic modes are monotone; monotonicity separates deterministic from stochastic
> and separates nothing within the deterministic set.
>
> holds for: signedness any, F any, I any, grid infinite, rounding ∈ {floor, ceil, toward_zero,
> half_up, half_even}, domain = ℚ (closed under negation), threads any (proved; probe at threads = 1).

Together, F1 and F2 answer one of the brief's listed questions directly: **monotone rounding and
homomorphic rounding are not disjoint the way the overflow families are; the homomorphic class is
empty among deterministic modes, on one-signed domains included, so the disjointness is vacuous.** And
the contrast with the corrected overflow theorem is sharp: saturation finds a one-signed refuge where
both properties coexist, and rounding has no such refuge, because density, not sign, is what it fights.
The interesting structure among deterministic modes lies elsewhere.

## 4. Where the deterministic modes genuinely differ: which exact law survives

T1 says no mode keeps the additive law exactly. What each mode keeps instead is a different exact law,
and the selection among these is, I will argue, the honest sense in which rounding "selects character".

**T3 (adjunction).** `floor` is the right adjoint and `ceil` the left adjoint of the grid inclusion
`G ↪ ℚ`, as monotone maps of posets: for all `g ∈ G` and `x ∈ ℚ`,

```
g ≤ x  ⟺  g ≤ floor(x)        x ≤ g  ⟺  ceil(x) ≤ g
```

with the corollaries `floor(x) ≤ x ≤ ceil(x)`. Adjoints are unique, so **these are the only two modes
carrying an exact one-sided order law**. This is the structural foundation of interval arithmetic: a
`(floor, ceil)` pair realises certified enclosures, and no other mode pair does, because no other mode
has a one-sided guarantee at all. For a strategy in the spirit of I7, accuracy across chains, this is
the difference between an error estimate and an error bound.

**T4 (composition across precisions).** Right adjoints compose: rounding `floor` to a fine grid and
then `floor` to a coarser grid equals `floor` to the coarse grid in one step, exactly, and likewise
`ceil` with `ceil`, and `toward_zero` with `toward_zero` (it is the adjoint pair glued along the sign,
and magnitude commutes with the gluing). The nearest modes do **not** compose: rounding to nearest
twice is not rounding to nearest once, the classical double-rounding failure. So chains that narrow in
stages are semantically identical to narrowing once under the directed modes and are not under the
nearest modes. For a design that wants one lowered path regardless of how a chain was associated
(I15's spirit), this is a real structural asymmetry: **directed modes make staged narrowing
associative; nearest modes make it depend on the staging.**

**T5 (equivariance groups).** Which symmetries of the grid a mode respects:

| mode | translation by `q·ℤ` | translation by `2q·ℤ` | negation |
|---|---|---|---|
| floor | yes | yes | no (floor and ceil swap) |
| ceil | yes | yes | no (dual to floor) |
| half_up | yes | yes | no |
| half_even | **no** | yes | yes |
| toward_zero | **no** | no | yes |

Two entries deserve the emphasis. `half_even` is not equivariant under translation by one quantum,
because shifting by `q` flips the parity of the two neighbouring grid points and the tie flips with
it; it is equivariant under `2q`. And `toward_zero` respects no translation at all; its symmetry is
the point reflection at zero. These are not aesthetic footnotes; they are exactly what decides the
interaction with the overflow axis in section 6.

> **F3.** Among deterministic modes the exact surviving laws are: adjunction (one-sided order bounds)
> for floor and ceil alone; negation equivariance and magnitude non-increase for toward_zero (and
> negation equivariance for half_even); exact staged-narrowing composition for floor, ceil,
> toward_zero and not for half_up, half_even; translation equivariance by q for floor, ceil, half_up,
> by 2q only for half_even, by nothing for toward_zero.
>
> holds for: signedness any, F any, I any, grid infinite, domain = ℚ (closed under negation), threads
> any (proved; probes at threads = 1 over the sweeps named in `125_probes/`, windows symmetric under
> negation).

**On the brief's question about `half_even`:** it is structurally different from the directed modes,
not merely fairer, in three provable ways: it carries no adjunction law (no exact one-sided bound), it
does not compose exactly across precisions, and its symmetry group is coarser (`2q`, not `q`). What it
buys is the optimal error bound `|Q(x) - x| ≤ q/2` shared with all nearest modes, plus zero tie bias,
plus negation equivariance, which `half_up` lacks. "Fairer" names the tie behaviour and misses the
symmetry and composition differences.

## 5. Where rounding lives at all: the operation decides, and F = 0 is the degenerate case

Rounding fires only when an exact result leaves the grid. That single observation settles two of the
brief's questions at once, because the grid is a group but not a ring in general:

- `+` and `-` of grid values land on the grid, for every `F`. **Rounding never fires for addition and
  subtraction of representable operands, at any fraction width.** The additive structure, which is the
  structure the overflow dichotomy's proofs live on, is rounding-free by construction.
- `×` of grid values has, in scaled-integer terms, `k_x · k_y / 2^F` as its exact value, off the grid
  unless `2^F` divides the product. At `F = 0` the division is by 1 and multiplication is exact;
  rounding never fires. At `F > 0` it routinely fires.
- `÷` leaves the grid even at `F = 0`, and so do right shifts read as division and square roots.
  **Integer division is where the rounding axis lives at F = 0**, and Rust's own `/` versus `>>` split
  (toward_zero versus floor) is the visible everyday consequence.

> **F4 (vacuity).** For operations in {+, -, ×} at F = 0, and for {+, -} at every F, the realised
> operation is identical under every rounding mode, stochastic included, because rounding is never
> invoked: the exact result is on the grid. Any finding predicated `rounding = trunc` whose operations
> lie in that set widens to `rounding any`. The widening is stated here, in my file, per the
> never-widen-in-place rule; the originals stand.
>
> holds for: signedness any, F = 0 for op ∈ {+, -, ×} and F any for op ∈ {+, -}, I any, W any,
> overflow ∈ {wrap, saturate}, rounding any, domain = representable operand pairs (closed under
> negation where signed), threads any (proved; exhaustive probe at threads = 1, W ≤ 5 both
> signednesses, ranges named in the probe output).

> **F5 (division is the residue).** At F = 0 with signedness = signed, realised division differs
> between toward_zero and floor on operand pairs with negative exact quotient off the grid; the
> rounding axis does not degenerate at F = 0, it merely withdraws to the non-ring operations {÷, >>,
> sqrt}.
>
> holds for: signedness = signed, F = 0, I any, op = ÷, rounding ∈ {floor, toward_zero} (the pair
> exhibited; other pairs differ too where the probe says so), domain = operand pairs with nonzero
> divisor (closed under negation), threads = 1 (measured; counts in `125_probes/`).

And the exactness region for multiplication has a clean arithmetic name. Writing `v₂` for the 2-adic
valuation of the scaled integer (the count of trailing zero bits, `v₂(0) = ∞`):

> **F6 (the roundless-multiplication predicate).** Realised multiplication is exact, and therefore
> mode-invariant, precisely on the pairs with `v₂(k_x) + v₂(k_y) ≥ F`. The valuation is additive, so
> the predicate is exact in both directions, not merely sufficient. Notable const-recognisable
> subregions: either operand an integer multiple of one (`v₂ ≥ F`), either operand a power of two
> times such, and all of F = 0. On this region every mode is a multiplicative homomorphism up to the
> range policy; off it, none is.
>
> holds for: signedness any, F any, I any, op = ×, rounding any, overflow any (the claim is about the
> quantisation step; range effects are the overflow axis's own), domain = representable operand pairs
> (closed under negation where signed), threads any (proved via valuation additivity; exhaustive probe
> at threads = 1, F ∈ {0, 2, 3}, W ≤ 6).

This is I13 material in its purest form: not "multiplication rounds", but a named region where it
provably does not, gate-able at const time for constants and for types whose fraction widths make the
condition structural (for example, multiplying by an integer-valued operand is roundless in any mode).

## 6. The interaction with overflow: the axes decompose, with one named exception

The two axes govern **disjoint regions of the input**: rounding acts on the off-grid part of the
in-range domain, overflow on the out-of-range part. They meet only on values that are both off-grid
and out of range, where the composite `O ∘ Q` applies, and there the question is whether the order of
the two decisions is observable. It is a commutation question and it has an exact answer.

**T6 (commutation with saturation).** Saturation clamps to `[m, M]` whose endpoints are grid points.
For any monotone grid-fixing `Q`: if `x` is in range then `Q(x)` is in range (monotonicity against the
fixed endpoints), so the clamp does nothing in either order; if `x > M` then both orders give `M`;
symmetrically below. So **every deterministic mode commutes with saturation.** The saturating family's
character, monotonicity of the composite, is preserved under every deterministic mode, since a
composition of monotone maps is monotone.

**T7 (commutation with wrapping).** Wrapping is translation by an integer multiple of the span
`s = 2^W · q`. A mode commutes with it iff the mode is equivariant under translation by `s`. From T5:
`floor`, `ceil`, `half_up` are `q`-equivariant, hence commute; `half_even` is `2q`-equivariant and `s`
is an even multiple of `q` whenever `W ≥ 1`, hence commutes; **`toward_zero` does not commute**, and
the failure is concrete: with span 16 and representative window `[0, 16)`, the exact value `-0.5q`
rounds toward zero to `0` and then wraps to `0`, but wraps to `15.5q` and then rounds to `15q`. The
non-commutation is observable exactly where a negative off-grid exact value meets a wrap, which
requires signed off-grid intermediates (signed multiplication at F > 0, or division).

> **F7 (decomposition).** The rounding and overflow axes decompose: they govern disjoint input
> regions, compose canonically as quantise-then-range-reduce, and the composition order is
> unobservable for every pairing except wrap with toward_zero. A canon that admits toward_zero
> alongside wrapping must state the order (the canonical order is Q first, on the exact value, then O
> on the resulting grid value); for every other pairing the sentence is unnecessary because the maps
> commute.
>
> holds for: signedness any (the toward_zero × wrap failure needs negative off-grid exacts, so it
> manifests at signedness = signed, or unsigned with signed intermediates), F any, I any, W ≥ 1,
> rounding ∈ {floor, ceil, half_up, half_even} × overflow ∈ {wrap, saturate} commuting, toward_zero ×
> saturate commuting, toward_zero × wrap not commuting, domain = ℚ over the swept windows (closed
> under negation), threads any for the proved commutations, threads = 1 for the counted
> non-commutation.

**T8 (mode cannot move a map between overflow families).** Family membership is decided by the
additive laws, and rounding never fires on addition (F4). So for every deterministic mode, a wrapping
realisation keeps its additive homomorphism and a saturating realisation keeps its monotonicity:
**the deterministic rounding choice neither rescues nor breaks the overflow character.** This holds on
one-signed domains too, where the corrected overflow theorem lets saturation hold both properties:
both are additive-law properties, rounding does not fire on addition, so both survive every
deterministic mode unchanged. At F > 0 the multiplicative half fails on the complement of F6's region
for every mode alike; no mode rescues it, so this too discriminates nothing.

The one genuine cross-term is stochastic:

> **F8 (stochastic selects character).** Stochastic rounding restores the additive law in expectation,
> `E[Q(x)] = x` exactly, hence `E[Q(x) + Q(y)] = x + y`: it is the one mode with a homomorphism, at
> the price of leaving the category of functions. Pointwise it is not monotone, so a saturating
> realisation composed with stochastic rounding is **ejected from the monotone family**: the property
> the preceding topic's saturating character rests on holds for none of its realisable outcome
> functions, only in expectation. The deterministic/stochastic boundary is therefore the one place the
> rounding axis reproduces the overflow axis's trade, in mirrored form: pointwise order against
> expectation-level algebra.
>
> holds for: signedness any, F > 0 or op ∉ {+, -, ×-at-F=0} (where rounding fires at all), I any,
> rounding = stochastic, overflow ∈ {wrap, saturate}, domain = ℚ off-grid (closed under negation),
> threads any for the expectation identity (proved, exact Bernoulli algebra; probe verifies with exact
> rational arithmetic at threads = 1). Also to note and not part of the claim: stochastic requires
> runtime entropy, so its selection predicate is not fully const-available; I13's instruction (what is
> not const-available cannot be in the predicate) bears on whether it can be an arm at all, and that
> is a question for the panel rather than a finding of mine.

## 7. The vocabulary hazard, which is worse than a spelling fork

The brief asks that `trunc` versus `truncation` be settled. Having derived the mode taxonomy, I must
report that the choice is worse than orthographic: **on signed domains the word is ambiguous between
two distinct modes.**

- **Bit truncation**, discarding the low `F` bits of a two's complement value, is `floor`. This is the
  mode the hardware gives away free: an arithmetic right shift is floor division, for signed and
  unsigned alike.
- **Arithmetic truncation**, rounding toward zero, is what Rust's integer `/` does, and on negative
  values it differs from bit truncation and costs a corrective step on most hardware.

On unsigned domains the two coincide, so if the preceding topic's probes swept unsigned domains its 35
pins are unambiguous. On signed domains a pin spelled `trunc` does not name a mode until one knows
whether the probe implemented `>>` or `/`. My suggestion, and it is a suggestion: the canon should
retire both spellings and name the modes `floor`, `ceil`, `toward_zero`, `half_up`, `half_even`,
`stochastic`, with `toward_zero` carrying a note that it is Rust's `/` and **not** the bit-drop
operation. If one of the two existing spellings must win in the interim, neither should, precisely
because the winner would inherit the ambiguity.

> **F9.** Two's complement bit-drop equals floor on signed and unsigned; it differs from toward_zero
> exactly on negative values with a nonzero dropped bit; on unsigned domains floor = toward_zero.
>
> holds for: signedness ∈ {signed, unsigned} as stated per clause, F ≥ 1 (bits exist to drop), I any,
> rounding ∈ {floor, toward_zero}, domain = all W-bit two's complement values (closed under negation
> up to the asymmetric minimum), threads any (proved; exhaustive probe at threads = 1, W ≤ 8).

## 8. The answer to the question, assembled

**Is rounding another character-selecting axis, a modifier of the overflow axis, or neither?** The
derivation supports a three-part answer, and none of the brief's three offered shapes fits alone:

1. **It is not a second copy of the overflow axis.** The hom/monotone pair that classifies overflow
   policies degenerates on rounding: the homomorphic class is empty among deterministic modes (T1,
   T1b) and the monotone class is all of them (T2). And where the corrected overflow theorem is
   domain-conditional, with saturation collecting both properties on one-signed domains, the rounding
   impossibility is unconditional: its obstruction is the domain's density against the grid, not its
   closure under negation. The axes are not the same theorem wearing two names.
2. **It is not a modifier of the overflow axis either.** Deterministic mode choice cannot move a
   realisation between the overflow families (T8), on one-signed domains included, and the two axes
   decompose as maps governing disjoint domain regions, commuting in seven of the eight deterministic
   pairings (T6, T7).
3. **It is an independent axis that selects character over a different property set**: which exact
   law survives quantisation. Directed modes keep the order bounds (adjunction) and exact staged
   narrowing; toward_zero keeps negation symmetry and magnitude contraction at the price of breaking
   translation structure, which is exactly the structure wrapping is built on; nearest modes keep the
   optimal error bound and (half_even) tie fairness at the price of composition; stochastic keeps the
   additive law itself, in expectation, at the price of pointwise monotonicity, and it alone crosses a
   character boundary the preceding topic cares about, by ejecting saturating realisations from the
   monotone family.

For the arms this feeds (suggestions): a wrap-family arm pairs naturally with floor, whose
equivariance matches and whose cost on two's complement is zero; a certified-bounds arm (I7's
concern) requires the directed pair and cannot be built on any other mode; an accumulation arm that
wants zero drift wants half_even or stochastic, and the latter drags in runtime entropy, which the
panel must weigh against I13's const-predicate instruction; and every `{+, -}` path, and every
`{×, F = 0}` path, is mode-invariant and should be predicated `rounding any` so no arm is needlessly
split by a dimension that provably does not act on it.

## 9. Predictions, stated before the probes ran

Recorded per the brief, so refutations are visible. P-numbers match the probe files in `125_probes/`.

1. **P1**: the five deterministic modes pass monotonicity on the full sweep; the deliberately broken
   parity mode fails it (control). Every deterministic mode fails additivity off-grid with a nonzero
   violation count, on the negation-closed sweep **and** on its one-signed restriction (T1b's
   empirical face); on-grid pairs pass (instrument sanity).
2. **P2**: at F = 0, ops {+, -, ×} produce identical results across all modes on the exhaustive
   sweep; op ÷ at F = 0 signed differs between floor and toward_zero (control, must fail the
   all-equal check); op × at F = 2 differs across modes (second control). The v₂ predicate of F6
   matches exactness in both directions with zero mismatches.
3. **P3**: wrap commutes with floor, ceil, half_up, half_even (zero mismatches each); wrap fails to
   commute with toward_zero (nonzero count, control); saturate commutes with all five deterministic
   modes; the artificial boundary-violating mode fails saturate commutation (control).
4. **P4**: adjunction laws hold exhaustively for floor and ceil; staged narrowing equals direct
   narrowing for floor, ceil, toward_zero with zero mismatches; half_up and half_even show nonzero
   double-rounding mismatches (control).
5. **P5**: bit-drop equals floor exhaustively signed and unsigned; differs from toward_zero on a
   nonzero count of signed negatives (control) and on zero unsigned values; stochastic expectation
   equals the exact value under exact rational arithmetic; a monotonicity-inverting outcome pair
   exists for stochastic and none exists for the deterministic modes.

The riskiest predictions, where I am most likely to be refuted: the `half_even`-commutes-with-wrap
claim (it rests on the span being an even multiple of `q`, and on my parity bookkeeping being right),
and the toward_zero staged-narrowing exactness (the sign-gluing argument is the kind that fails at a
boundary case one did not think of). If either falls, the falling is the finding.

## 10. Results

*Pending. This section is filled in only after the probes in `125_probes/` have actually run, each
committed with its output as it runs, per the evidence rules. Nothing above this line is edited when
the results land; a prediction refuted is reported here as refuted.*

### The test gate's result

*Pending with the same commit; the per-crate runs were in flight while this derivation was written.*

### Coverage bound

What I did not do, named per `RULES.md`: I did not sweep `half_down` or ties-away-from-zero (their T5
rows would mirror half_up's and half_even's respectively, and I did not verify that); I did not probe
`sqrt` or any transcendental; I did not measure anything on the bench harness, so every magnitude here
is a count from an exhaustive enumeration, not a price, and nothing in this file prices anything; I
did not examine saturation to asymmetric bounds (the signed minimum's magnitude exceeding the
maximum's) beyond noting that T6's proof only needs the endpoints to be grid points, which holds; and
stochastic × wrap commutation-in-distribution is stated nowhere above because I could not settle in
what sense expectation survives the quotient, and I chose to leave it absent rather than hedge, which
per the predicate discipline means it holds nowhere until someone establishes it.
