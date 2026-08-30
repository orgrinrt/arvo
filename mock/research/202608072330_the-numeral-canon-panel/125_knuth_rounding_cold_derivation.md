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

*Written after the probes ran. Nothing above this line was edited when the results landed; the
refutation below is reported here, against the prediction as committed, per the discipline that a
phase-one derivation is never rewritten.*

**One prediction was refuted, and it was one of the two I named riskiest.** Everything else was
confirmed, every negative control failed as required, and two instrument bugs were caught by their own
expectations before any number was believed (both defective runs are preserved beside the corrected
outputs).

**The refutation (P3, wrap commutation).** I predicted zero mismatches for ceil, half_up and half_even
against wrap. Run 1 measured 60, 32 and 32 at W = 3 through 5. The error in the proof: wrap on exact
values is a **piecewise** translation, and my argument silently treated it as a single translation. A
mode that can round upward maps the top cell of the half-open representative window one past its top,
so the two composites differ by exactly one span at boundary cells. The corrected statement, tested in
the same probe's second pass and confirmed at zero mismatches across W ∈ {3, 4, 5} and both
signednesses: **for floor, ceil, half_up and half_even the composites agree in the quotient group
ℤ/spanℤ**, which is where wrap actually lives, **and floor alone agrees at the representative level**,
because floor is the one mode that never rounds upward out of the half-open window. toward_zero fails
even in the quotient (240/480/960 quotient mismatches at W = 3/4/5, identical to its representative
counts, so its failure is structural rather than a boundary artifact). Three consequences:

- **F7 is corrected as follows**, superseding its wording above: the axes still decompose, but the
  commuting pairings hold **mod span**, not at the representative level; only floor × wrap commutes
  representative-exactly; toward_zero × wrap fails at both levels. The saturate column of F7 stands
  unchanged at the representative level (zero mismatches, all five modes, control failed at 15).
  For a design whose wrap semantics is the quotient semantics, mod-span agreement is the meaningful
  equality and the practical content of F7 survives; for any observer of representatives before
  reduction, only floor commutes.
- **The floor-wrap affinity in section 8 strengthens**: floor is not merely the free mode on two's
  complement, it is the unique deterministic mode whose composition with wrap is unobservable at
  every level.
- The prediction's failure mode, a boundary case in a proof over a half-open window, is recorded as
  exactly the class I flagged in section 9's last paragraph.

**P1** (committed with output): five modes monotone over u ∈ [-2000, 2000] subunits at 16 subquanta
per quantum; parity control not monotone (1750 adjacent inversions). Additivity violations of 14884
sampled pairs: floor 6516, ceil 7344, toward_zero 7143, half_up 3770, half_even 3802; one-signed
restriction: 1665, 1800, 1665, 912, 920; on-grid pairs: zero for every mode. T1 and T1b both have
their empirical face: the additive law fails off-grid on the negation-closed window **and** on the
one-signed restriction, which is the point the dispatcher's brief correction made worth checking.

**P2** (committed with both outputs): run 1 carried a units bug, converting add/sub results as if
they scaled like products; its false "refutation" of F4 was caught by the must-be-0 expectation and
the defective output is preserved. Corrected run: ops {+, -} at F ∈ {0, 2} and × at F = 0, both
signednesses, W = 5 exhaustive: **zero mode-differing cells** in every case (F4 confirmed, including
add/sub at F > 0). Controls: ÷ at F = 0 signed differs between floor and toward_zero on 386 of 992
pairs (F5); × at F = 2 differs across modes on 512 of 1024 cells. F6's v₂ predicate: 24576 cells over
F ∈ {0, 2, 3}, W = 6, both signednesses, **zero mismatches in either direction**.

**P4** (committed with output): adjunction laws exact over 4001 points × 281 grid values, both laws,
zero failures. Staged narrowing (fine → quarters → integers versus direct): floor, ceil, toward_zero
zero mismatches; half_up 500, half_even 500 of 4001 (the double-rounding controls, failing as they
must). One instrument bug (integer division producing a float) crashed before producing a number and
was fixed; it could not have corrupted a result.

**P5** (committed with output): bit-drop equals floor exhaustively at W = 8 for F ∈ {1, 3, 5}, signed
and unsigned; bit-drop differs from toward_zero on 64/112/124 signed values respectively and on zero
unsigned values (F9). Stochastic expectation exact over 4001 points under exact rational arithmetic,
zero deviations; the inversion pair x = q/5 < y = 4q/5 realises Q(x) = 1 > Q(y) = 0 with probability
1/25; all five deterministic modes show zero inversions on the same sweep (F8).

### The test gate's result

Run per crate by `--manifest-path`, as the brief instructs. **Twelve of the thirteen `*-shared`
crates completed green: 108 tests, 0 failures** (carrier 9, contend 12, footprint 6, plan 5, bitpack
3, wide 6, quantiser-fadd 1, quantiser-radix 3, satfold 11, warm-clamp 7, warm-container 15,
wide-rung 30; wide-rung alone took 202 seconds). **The thirteenth, `bitpack-write-contend-shared`,
did not complete**: my run consumed 88 CPU-minutes without producing a single test result before I
stopped it, and, more telling, a pre-existing sibling process (another member's run of the same test
binary from a different target directory) stood at **910 CPU-minutes, above fifteen CPU-hours,
still running** when I looked, alongside a stale `cargo test --workspace --no-fail-fast` process,
which is the false-green invocation the brief warns about, still alive on this machine. A test that
cannot finish is not a green test, and three concurrent instances of a core-pinning contention suite
fighting for the same P-cores may be why none of them finishes; I stopped only my own processes and
left the other member's untouched. The crate holds 16 `#[test]` functions by
`grep -c '#[test]' src/*.rs` (2 + 9 + 5 across input.rs, kernels.rs, stress.rs), which against my
108 green accounts for the brief's advertised 123 to within one; both counts are stated with their
commands and the discrepancy of one is left as a discrepancy rather than reconciled by guesswork.

I read the test bodies of the two crates nearest my subject, `quantiser-fadd-shared` and
`quantiser-radix-shared`, in full. They are real instruments: the radix-two test compares against
silicon bit-exactly with counted totals asserted; the radix-ten test checks the delivered significand
against the definition (nearest grid point, ties to even) using an independent exact-integer oracle,
explicitly not a second call to the rounding code under test. One redundancy (a duplicated parity
assertion in the odd-radix test), nothing tautological. Two observations that touch this file's
subject: that suite pins **ties-to-even** as the checked mode for its decimal path, while the
preceding topic pins trunc, which is consistent with rounding being per-realisation rather than
global; and its odd-radix test proves that **tie-breaking distinctions are an even-radix phenomenon**,
which bounds my half_even analysis to radix 2 (where arvo lives) and is named in the coverage bound.
I did not read the other eleven crates' 100-odd bodies; they do not touch rounding.

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


---

## 11. Phase two: reconciliation against `122`, `119` section 5, and `116` section 7

*Appended after the phase-one commits, having now read the three files the brief names, and nothing
else of the panel beyond the probe implementations cited below, which I opened to answer the
vocabulary question. Phase one above is unedited.*

### 11.1 The question was asked in the sitting, and my answer lands where `116` guessed

`116` section 7 names rounding as "the first place I would look" for a second character-selecting
axis, on the ground that its own section 6.1 shows the mode "owns a region of the map with its own
algebraic character". `119` section 5 records the question as unsettled and says 4.4 pins rounding at
a fixed value for exactly that reason. My phase one, derived blind, answers: the guess's premise is
confirmed and made precise (rounding owns the off-grid in-range region, section 6 above), and the
conjectured conclusion is refined rather than confirmed. Rounding is **not** a second licence-family
selector in the sitting's sense: the deterministic mode choice cannot move a map between the deferral
and order families (T8), because the additive laws those families rest on never invoke rounding. What
it selects among is a different set of exact laws (adjunction, negation symmetry, staged-narrowing
composition, T3 through T5), and the one place it crosses the sitting's family line is the
deterministic/stochastic boundary, where a saturating map loses pointwise monotonicity (F8).

### 11.2 Where my blind derivation agrees, and the rungs that creates

Stating what I carry forward unchanged, from whom, with the count: **four things, and two of them now
have independent instances.**

1. **The vacuity argument is a convergence.** `122` 4.4 widens the wrapping map's add and sub half to
   any fraction width "on the argument that the grid is closed under those operations so they never
   enter the rounding region", with `121` section 5 agreeing for the same reason. My F4 is that
   argument, derived and committed before I had read either file, with an exhaustive probe (P2)
   behind it. Under `RULES.md`'s ordering requirement my phase-one commit precedes my reading, so the
   claim now has at least two independent derivations plus an exhaustive verification at W = 5, and
   the consolidation can record the rung accordingly.
2. **The region decomposition is a convergence.** `122` 4.6's locality rule (a reduction's grid part
   fires where an exact result can leave the grid; its range part where the map is not a
   homomorphism) is the operational form of my F7's domain decomposition (rounding governs off-grid
   in-range, overflow governs out-of-range). Independently derived, same structure, no conflict.
3. **The fixed-value pins were the right caution.** `119` pinning rounding rather than writing
   `rounding any` was correct discipline under I13, and my F4 now says exactly which of those pins
   can widen: any finding whose operations lie in {+, -, min} at any F, or additionally x at F = 0,
   holds under every mode, because rounding never fires there. Findings involving x at F > 0, or
   division, do not widen. The widening is stated here and in phase one, per the
   never-widen-in-place rule; the originals stand.
4. **The stochastic aside in 4.3's vocabulary.** The deferral licence has an expectation-level
   sibling: under stochastic rounding, deferring quantisation across additions is exact in
   expectation (F8's identity). I did not measure anything about it beyond the identity and leave it
   as an opening.

### 11.3 The sharpest cross-result: the sitting's domain dimension does not act on my axis

`122`'s central repair is that the overflow impossibility is domain-conditional: closed under
negation, exclusive; one-signed, saturation collects both characters. The dispatcher forwarded this
correction mid-derivation, and T1b (proved, then verified empirically in P1's one-signed column) is
its counterpart on my axis: **the rounding impossibility has no one-signed refuge.** The obstruction
is divisibility, not order against negation, so the homomorphic class of deterministic modes is empty
on one-signed and negation-closed domains alike. The dimension the sitting just learned to carry in
every predicate is provably inert for this one question, and that inertness is itself the predicate
discipline working: my F1 lists `domain ∈ {ℚ, ℚ≥0}` because both are proved, not because the
dimension does not matter.

### 11.4 What my derivation adds to `122`'s clauses, stated as suggestions

**4.4's wrapping-mul condition has an exact form.** `122` 4.4 licenses multiplication "only where the
fraction width is zero or the operands are declared on the unit grid". That is a sufficient syntactic
condition, and `122` 4.7 itself distinguishes such conditions from the condition itself. F6 supplies
the exact one: multiplication is roundless, hence mode-invariant and grid-part-free, precisely on
`v2(k_x) + v2(k_y) >= F` (2-adic valuations of the scaled integers; unit-grid declaration is the
special case `v2 >= F` on one operand). The exact condition is per-value; the declared forms of it
that a type system can carry (unit grid, or a declared trailing-zero budget) are the const-checkable
arms. Verified both directions on 24576 cells (P2).

**The grid part's deferral region is mode-invariant, and the residue is not.** Where every
intermediate satisfies F6's condition, deferring the grid part is exact under every mode; outside
that region it is inexact under every mode, with per-mode magnitudes I did not count. So `122` 4.6
needs no mode dimension for the soundness of its locality rule, and would need one for any future
claim about error sizes.

**The pinned mode is the structurally worst partner for wrapping, and the sitting should know it.**
The probes implement `trunc` as genuine toward-zero (`118_probes/q3` at its `q_of`, with `floor` as a
labelled alternative; `118_probes/q5` at its `quantise`), so the 35 pins name my `toward_zero`. My P3
measures that toward-zero is the unique deterministic mode that fails to commute with wrap **even in
the quotient group** (240 to 960 quotient mismatches per width against zero for floor, ceil, half_up,
half_even), while floor commutes at every level and costs nothing on two's complement (F9). Two
consequences offered as suggestions: any canon clause pairing wrap with toward-zero must state the
quantise-then-reduce order, because it is observable there and only there (the sitting's probes do
use the canonical order, `q3`'s `R_of` quantising before reducing); and the wrap-family arm may
simply prefer floor, which dissolves the caveat and is the free mode in hardware.

**The vocabulary matter, now answerable with evidence.** The brief asked that `trunc` versus
`truncation` be settled. Phase one found the deeper hazard (F9: on signed domains the word is
ambiguous between bit-drop, which is floor, and toward-zero); phase two resolves which one the
sitting means: toward-zero, per the probe sources cited above. My suggestion stands as in section 7:
retire both spellings for `toward_zero` (with `floor`, `ceil`, `half_up`, `half_even`, `stochastic`
as the mode vocabulary), and note beside it that bit truncation of two's complement is floor, so
nobody reads the hardware operation into the word. If the panel prefers to keep one existing
spelling, it should define it as toward-zero in one place and expect the floor confusion to recur.

### 11.5 What I got wrong, per the brief's required confession

**In phase one, one prediction**: representative-level wrap commutation for ceil, half_up and
half_even, refuted by my own P3 and corrected to quotient-level in section 10. Phase two adds
nothing to that list: none of the three files contradicts a claim of mine, and the one premise I was
handed wrong (the unconditional overflow impossibility) was corrected by the dispatcher mid-phase and
is recorded in the preamble. I note for the record that `122` reached me only in phase two, so my
sections 2 and 3 were derived against the corrected premise as relayed by the dispatcher, not against
`122`'s own text; having now read `122` sections 1.5 and 4.2, the relay was faithful.

**One near-miss worth confessing**: my section 5 wrote "the additive structure, which is the
structure the overflow dichotomy's proofs live on", with the finite-ascending-chain argument in mind.
`122` 4.2's replacement predicate shows the proof also needs the domain's closure under negation,
which my sentence did not carry. Nothing in my file rests on the overflow theorem's proof, so no
claim moves, but the sentence as I wrote it repeats the sitting's superseded framing and should be
read with `122` 4.2's predicate instead.

### 11.6 Suggested consolidation shape for the rounding axis

For whoever writes this topic's candidate, the shape my findings suggest, all of it suggestion:

- Rounding is an independent axis, not a sub-case of overflow: own impossibility theorem
  (unconditional where overflow's is domain-conditional), own surviving-law classification, own
  operation-selectivity (it exists only where an exact result leaves the grid).
- The axes decompose with a compatibility table: saturate composes invisibly with every deterministic
  mode; wrap composes invisibly (mod span) with the translation-equivariant modes and floor alone at
  the representative level; wrap with toward-zero requires an order clause; stochastic with saturate
  forfeits the monotone character.
- Per I13, the arms: `{+, -, min}` any F and `{x, F = 0}` carry `rounding any`; `{x, F > 0}` splits
  on F6's valuation predicate (roundless arm, mode-free) against the rounding region (mode named per
  arm); division carries a mode always, F = 0 included; certified-enclosure arms require the directed
  pair; wrap arms prefer floor or state the order.

### 11.7 Coverage of this reconciliation, bounded

Read in full: `122`, `119` section 5, `116` section 7. Read in part: `116` sections 6 through 7
headings for context; `118_probes/q3` at its `quantise`, `q_of` and `R_of`, and `118_probes/q5` at
its `quantise`, opened for the vocabulary question only. Not read: everything else in the panel,
including `114`, `115`, `120`, `121`, the registers, and the consolidations; my account of the
sitting is `122`'s account, which is exactly the single-point-of-failure shape `RULES.md` warns
about, so where this reconciliation attributes a position to `120` or `121` it is `122`'s report of
them, not my reading.
