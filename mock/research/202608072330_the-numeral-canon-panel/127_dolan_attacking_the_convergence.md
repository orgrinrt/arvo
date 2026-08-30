# 127. Attacking the convergence

Dispatched to attack `125` and `126`, which derived rounding's role blind to each other and to `122`,
and agreed. Read in full: `125`, `126`, `122`, `INTENTS.md`, `RULES.md`. Not read: anything else in the
panel; per the brief, this is a targeted attack on one topic's convergence rather than a fresh sweep.

**Canon gate: passed.** The question is inside I13's programme and touches no closed entry. I15 bears
on nothing here; everything below is compile-time-decidable structure or a fact about a stochastic
kernel's distribution, and I17 and I1 are not touched by a topic about rounding modes.

**Test gate.** `125` already ran it per crate by `--manifest-path`: twelve of thirteen `*-shared` crates
green at 108 tests, `bitpack-write-contend-shared` unterminated because of concurrent contending
processes, one of which was mine and has been killed. I checked the machine before starting: `ps aux`
still shows a stale `cargo test --workspace --no-fail-fast` process at 0:00.33 CPU, the exact
false-green invocation the brief and `125` both warn about, alive but not consuming resources. Disk is
at 1.9GiB free, tighter than when `125` ran. I did not re-run the twelve green crates or attempt the
contending thirteenth: doing so would spend disk this dispatch cannot safely spare on a result already
established once, cleanly, with a named and now-partially-resolved cause. I record the stale process as
further confirmation of `125`'s finding rather than as a fresh discovery, and treat the gate as passed at
twelve of thirteen, inherited, with the thirteenth's blocker unchanged from `125`'s account.

## The verdict, stated first

**The convergence is real on its main claim and wrong on its second-most load-bearing one.** Both
derivations are right that rounding is not a second copy of the overflow axis and not a modifier of it:
the homomorphic class is empty among deterministic modes off-grid, at every domain including one-signed,
and monotonicity is free for all of them. That part I reproduce independently below and it holds.

But both files, independently, build a second claim on top of the first: that the deterministic/
stochastic boundary is "the one place rounding echoes the overflow axis's trade" (`125` F8), a "genuine
either/or fork" parallel in shape to wrapping-versus-saturating (`126` Finding 4). **That claim is false
as stated, and I built the counterexample rather than only arguing against it.** A stochastic rounding
scheme exists that is monotone on every single realisation, globally, and exactly unbiased in
expectation, at once. The two properties both files present as mutually exclusive are not exclusive at
all; the independent-per-element construction both of them tested is one member of a family, and it is
the member that happens to give up monotonicity, not a structural necessity of leaving the category of
functions.

## What I am carrying forward, with counts

**Six things, unchanged, from both files independently, now with a third instrument.**

1. **No deterministic rounding mode is additive off-grid, at any domain, including one-signed.** `125`
   T1/T1b (proof) plus P1 (probe, includes a one-signed restriction column). `126` Finding 2 (probe,
   grid already one-signed by construction) plus phase two's explicit re-check against `122`'s domain
   dimension. Two independent derivations, one of them (`126`) derived on a domain it did not know was
   the interesting one, which makes the agreement stronger rather than weaker: `126`'s grid function
   happened to be non-negative and its author only later realised, in phase two, that this was exactly
   the domain `122` needed tested. TWO EXPERTS, and I did not re-derive this one myself; I read both
   proofs and both probes and find no defect in either.

2. **All deterministic modes are monotone, unconditionally.** `125` T2/F2, `126` Finding 3, both proved
   by the same structural argument (a threshold-per-cell rule is nondecreasing by construction) and both
   swept. I independently re-derive and extend this below (my own construction is a monotone mode too,
   by the identical argument), so this is now a third instance on a fourth instrument.

3. **Rounding is vacuous for `{+, -}` at every `F` and for `{+, -, x}` at `F = 0`.** `125` F4, `126`
   Finding 1, independently derived from the same grid-closure fact, and `125`'s phase two separately
   notes the same fact underlies `122` 4.4's C2 exemption. TWO EXPERTS on the underlying grid-closure
   claim; I did not re-derive it, it is definitional and neither file disputes it.

4. **half_even groups with half_up against the directed modes on double-rounding safety, while being
   distinct from half_up on bias.** `125` T4/T5/P4 (proved and swept: floor, ceil, toward_zero exact
   under staged narrowing; half_up, half_even both show nonzero mismatches). `126` Finding 5 (a
   dedicated isolation probe, `p3_double_rounding_isolated.py`, with the overflow bound physically
   removed to isolate the effect). Two different probes, two different constructions (a staged-grid
   sweep against an isolated accumulator-to-final sweep), same split. TWO EXPERTS, genuinely independent
   instruments, and I add nothing beyond noting the independence explicitly since neither file credits
   the other with this convergence (both were phase-one blind to each other).

5. **The rounding axis and the overflow axis decompose over disjoint input regions**, meeting only where
   an exact result is both off-grid and out of range, where the composite order is unobservable for
   every deterministic pairing except toward_zero with wrap. `125` T6/T7/F7, corrected by its own P3
   after a refuted representative-level prediction, to: floor alone commutes with wrap at the
   representative level, the rest commute mod-span, toward_zero fails at both levels. I reproduced this
   correction on the exact asymmetric two's-complement-style signed range arvo would actually use
   (`lo, hi = -(span/2), span/2 - 1`), which is what `125`'s own P3 already swept and I re-ran by reading
   the source and its output rather than by re-executing it, since the code and the output already
   settle it and re-running would spend disk this dispatch cannot spare. The signed and unsigned rows in
   `125_probes/p3_output.txt` are numerically identical at every width, which is itself informative: the
   floor-uniquely-commutes result does not depend on signedness, only on which representative window is
   chosen, so it generalises to the real container shape without further work. `126`'s Finding 6 studies
   a different order-of-operations question (round-then-clamp versus clamp-then-round with saturate, not
   wrap) and does not test wrap-commutation across the mode set the way `125`'s P3 does, so this specific
   floor-uniquely-commutes-with-wrap claim stays at ONE EXPERT even after my check; I confirm it holds
   and add no second derivation of it.

6. **The `trunc`/`truncation` vocabulary pin in the settled overflow topic (`119`/`122`) names
   toward-zero, not floor, and the two differ on signed values.** `125` F9 makes this claim and cites
   `118_probes/q3`'s `q_of` and `118_probes/q5`'s `quantise`. **I checked this at source before carrying
   it forward, per the brief.** `q5_one_rule_with_two_locality_conditions.py:107-109`:

   ```python
   def quantise(P, q):
       k = int(q / P.step) if q >= 0 else -int(-q / P.step)
       return k * P.step
   ```

   That is toward-zero: Python's `int()` truncates toward zero, and the explicit negative branch
   confirms it is not an accident of `int()`'s behaviour on positive-only inputs, it is deliberately
   symmetric truncation. `q3_the_fraction_width_splits_my_arms_too.py:407-413`'s `q_of` carries the same
   default with `floor` as a named, separate branch, and `q3`'s `R_of` (line 415) calls `q_of` before
   reducing, confirming `125`'s claim that the sitting's probes use the canonical quantise-then-reduce
   order. I also grepped `119_leroy_the_canon_candidate_for_the_realisation_map.md` directly: the pin is
   spelled `rounding = truncation` at six sites, never `floor`. **This is now independently confirmed at
   source by a second reader, and it is a fact about the settled candidate rather than about this topic**:
   `122`'s wrap clauses (4.3, 4.4, 4.6, 4.7) are pinned at exactly the mode `125`'s own P3 (item 5 above)
   identifies as the one deterministic mode that fails to commute with wrap even in the quotient group.
   Nobody who wrote `122` knew this when writing it, because the rounding axis had not been derived yet.
   I do not propose editing `122`; it is locked and out of scope for this dispatch. I flag it as a fact
   the rounding topic's consolidation should carry forward explicitly, because a reader of `122` alone
   would have no way to know its own wrap clauses rest on the single worst-interacting rounding pin
   available.

## The attack: the deterministic/stochastic fork is not a fork

### Reproducing the shared claim before refuting it

`125` F8: "Stochastic rounding restores the additive law in expectation, `E[Q(x)] = x` exactly... it is
the one mode with a homomorphism, at the price of leaving the category of functions... a saturating
realisation composed with stochastic rounding is ejected from the monotone family... The
deterministic/stochastic boundary is therefore the one place the rounding axis reproduces the overflow
axis's trade, in mirrored form: pointwise order against expectation-level algebra."

`126` Finding 4: "the one rounding mechanism I found that gives up monotonicity for something in return,
which is the same trade shape as overflow's fork... This is the genuine either/or fork on the rounding
axis, and it is worth naming as the actual analogue of the overflow fork."

Both are honest about not having proven the exclusion. `126`'s own open item 3: "Whether stochastic
rounding's monotone-vs-unbiased trade is a genuine, provable dichotomy... or merely a fact about the
constructions I tried. I exhibited the trade; I did not prove the exclusion." `125` does not hedge this
way; F8 states the mirrored-trade framing flat, as a finding rather than as an open question.

I reproduced the base facts on my own instrument first, independently of either probe file, using exact
`Fraction` arithmetic rather than either file's construction: `127_probes/w1_output.txt`, Part 1's
control and Part 2. Independent per-element draws produce non-monotone pairs (194 of 20000 trials on the
`x=1/10, y=9/10` construction, matching `125` P5 and `126`'s `p4_stochastic_nonmonotone_fixed.py`), and
the exact-expectation identity `E[Q(x)] = x` holds symbolically for every tested `x`, matching both
files' F8/Finding 4 exactly. **So the base facts both files report are correct and I confirm them on a
third instrument before attacking the conclusion drawn from them.**

### The construction that breaks it

A single shared threshold, drawn once and held fixed across an entire evaluation pass rather than redrawn
per element:

```
Q_U(x) = floor(x)  if frac(x) <= U
       = ceil(x)   if frac(x) > U
```

for a single `U ~ Uniform[0, 1)` drawn once. `127_probes/w1_shared_threshold_stochastic_is_monotone_and_unbiased.py`:

- **Part 1, monotonicity**: for seven values of `U` including two near the extremes, zero monotonicity
  violations over 769 consecutive exact points spanning twelve quanta at subquantum resolution `E = 6`.
  This is not "monotone as a function of value for a fixed position" the way `126`'s own
  position-keyed-dithering aside describes; it is monotone over the whole swept domain, for every fixed
  `U`, the same global sense `125` T2 and `126` Finding 3 prove for floor and ceil.
- **Part 2, unbiasedness**: exact symbolic expectation `E_U[Q_U(x)] = k(1-f) + (k+1)f = k + f = x` for
  five test points including a negative one and an on-grid one, matching to the last digit, plus a
  500,000-draw Monte Carlo cross-check on a second, independent random-number path (Python's `random`
  rather than the exact partition used for the symbolic check) agreeing to within sampling noise at
  every point (largest deviation 0.00096 of 500,000 draws).
- **Part 3, control**: `Q_U` for a fixed `U` is an ordinary deterministic rounding mode and is not
  additive off-grid (30 of 1000 sampled pairs violate additivity at `U = 1/3`), confirming this is not a
  loophole in T1/Finding 2. Unbiasedness is a property of the distribution over `U`, never of a single
  realisation, exactly as both files' own unbiasedness claim already requires; I am not disputing that
  part.
- **Part 4, composition with saturate**: for the same seven values of `U`, `saturate(Q_U(.))` shows zero
  monotonicity violations over 1536 consecutive points on an asymmetric two's-complement-style signed
  range `[-8, 7]`, by the same composition-of-monotone-maps argument `125` T6 and `126`'s Prediction 7
  already use for the other five modes. **This directly contradicts F8's second clause**: under this
  construction, a saturating realisation composed with stochastic rounding is not ejected from the
  monotone family. It stays in it, on every draw.

All four predictions were stated in the probe's header comment before it was run, and all four were
confirmed exactly, at zero violations where zero was predicted. Nothing here was found by accident after
a failed run; I predicted the shared-threshold construction would work before building it, because once
I noticed `Q_U` is literally `floor` and `ceil` connected by a continuum (`U = 1` recovers floor, `U = 0`
recovers ceil, `U = 1/2` recovers `half_up` exactly), and each fixed `U` is a threshold-per-cell rule
identical in shape to the ones `125` T2 and `126` Finding 3 already proved monotone, the monotonicity
half was not really in doubt. What needed checking was whether the mixture over `U` still integrates to
`x` exactly, and it does, by the same linearity-of-expectation argument that makes independent stochastic
rounding unbiased.

### Why this is not a quibble about which stochastic scheme somebody happens to have tested

The overflow fork (`122` 4.2) is a genuine dichotomy because both branches are separately achieved by
nonconstant, coherent, deterministic functions occupying the same category (maps `G -> R`), and the
theorem is a real combinatorial fact about that shared category (a finite additive group admits no
infinite ascending chain). Giving up one property to get the other is a forced move within one kind of
object.

What both files call "the rounding fork" is not that shape. **No deterministic function was ever a live
candidate for exactness off-grid**, by T1/Finding 2, at any domain, unconditionally. So "monotone" was
never traded against anything; it was free, and both files say so elsewhere (`126` Finding 3: "Rounding's
monotone branch costs you nothing you were going to get anyway"). What F8/Finding 4 then does is notice
that leaving the category of deterministic functions (going to a distribution) recovers exactness in
expectation, and frames the loss of PER-DRAW monotonicity in the ONE construction tested (independent
per-element draws) as the price of that move, as though it were structurally necessary. It is not. The
shared-threshold construction stays inside "a distribution over deterministic functions" exactly the way
independent stochastic rounding does, and recovers both properties, because the two properties were never
in tension to begin with: monotonicity is a fact about ONE fixed realisation being a nondecreasing
function, and unbiasedness is a fact about the DISTRIBUTION integrating correctly. Nothing forces the
distribution to be independent per element, and the independent choice is what manufactures the
monotonicity violation, not the act of randomising itself.

### The replacement finding, in I13's notation

> **F127-1.** A stochastic rounding scheme exists that is monotone on every realisation and exactly
> unbiased in expectation, simultaneously: the shared threshold `Q_U(x) = floor(x)` if `frac(x) <= U`
> else `ceil(x)`, for a single `U` held fixed across an evaluation. The claimed either/or fork between
> deterministic-monotone and stochastic-unbiased-not-per-draw-monotone is false as a general statement
> about "stochastic rounding"; it is true only of the independent-per-element construction, which is one
> member of a family rather than the only shape stochastic rounding can take.
>
> holds for: signedness any, F any (the construction only touches whether `x` is on-grid, which is
> domain-generic), I any, rounding = the shared-threshold family parametrised by `U`, overflow ∈ {wrap
> (untested directly, monotonicity argument applies regardless since it is about the rounding step
> alone), saturate (tested, composition stays monotone)}, domain = ℚ (closed under negation; the swept
> range in Part 4 is asymmetric signed, per Part 4's own note), threads = 1 (measured; the argument for
> monotonicity and for the exact-expectation identity is structural and does not depend on thread count,
> but I have not swept `threads any` and do not claim it), target features any (exact rational
> arithmetic, no instruction selection can move it).

> **F127-2.** `Q_U` at `U = 1/2` is exactly `half_up`; at `U -> 1` it approaches `floor`; at `U -> 0` it
> approaches `ceil`. The shared-threshold family is a one-parameter continuum connecting three of the
> five modes both files already swept, and every member of it is monotone by the identical
> threshold-per-cell argument `125` T2 and `126` Finding 3 already give for the endpoints. This is not a
> new proof technique; it is the same one, applied to a value of the threshold neither file happened to
> randomise.
>
> holds for: the parametrisation is exact and definitional, no sweep needed beyond confirming the two
> endpoints and the midpoint algebraically, which is immediate from the definition of `Q_U` and the
> definitions of floor, ceil, and half_up already given in `125` section 1.

### What this changes for the arms, offered as a suggestion

Both files' section on arms treats "stochastic" as a single tradeoff: exactness at the price of
monotonicity, with `125` explicitly flagging that a saturating arm composed with stochastic rounding
loses its monotone character and that the choice must be weighed against I13's const-predicate
requirement given stochastic needs runtime entropy. **The monotonicity loss is not a property of
"stochastic" as an axis; it is a property of independent-per-element draws specifically.** A
shared-threshold (or more generally, a correlated-draw) stochastic mode keeps monotonicity, keeps
exactness in expectation, and composes cleanly with saturate the way every deterministic mode already
does.

This does not remove the const-availability problem I13 raises for either construction (both need a
runtime draw; a shared threshold needs only one draw per evaluation rather than one per element, which
is a smaller ask but not a const-time one). What it changes is the framing: an arm wanting both
order-preservation and unbiasedness is not choosing between two mutually exclusive properties on the
rounding axis the way it is choosing between wrap and saturate on the overflow axis. It is choosing
between two different **correlation structures** for the same underlying randomness, and the correlated
one dominates the independent one on every property either file measured. I have not measured whether a
shared-threshold scheme is cheaper or more expensive to generate than independent per-element draws, and
say so as a gap rather than guess: that is a bench question, not a structural one, and nothing here
prices it.

## What I checked and did not find a defect in

I looked for a second attack angle before settling on this one and want the dead ends on the record,
since they were real work and the next reader should not redo them.

**Whether the asymmetric two's-complement signed range breaks the floor-uniquely-commutes-with-wrap
claim.** It does not; `125`'s own P3 already tests it and the signed and unsigned rows agree numerically
at every width. Reported under "carried forward," item 5.

**Whether either file tested a domain restriction narrower than one-signed, parallel to `122` 4.5's
declared-restriction mechanism, that rescues rounding's homomorphism nontrivially.** `125`'s F6 (the
roundless-multiplication predicate, `v2(k_x) + v2(k_y) >= F`) is exactly this mechanism, correctly
identified by `125` itself as parallel to `122` 4.5. `126` leaves it as an open question (item 8) without
constructing it. I do not think there is a gap here worth chasing further: F6 already answers the
question `126` leaves open, in the file `126` did not read at the time it was writing phase one, and both
files converge on it once `126`'s phase two reads `122` 4.6. I add nothing beyond noting the convergence
holds and is not a false one.

**Whether rounding interacts with the specific one-signed subtraction-free deferral region `122` F122-5
establishes for saturation's overflow discharge.** Both files address this directly and correctly:
rounding fires per-node inside that region whenever `F >= 1` and the multiplication is not covered by
F6's roundless predicate, so the grid part cannot be deferred there even though the range part can. `125`
section 11.4 and `126`'s reconciliation both state this; I checked both statements against `122` 4.6's
text and neither misreads it.

## Coverage, bounded

Read in full: `125`, `126`, `122`, `INTENTS.md`, `RULES.md`. Opened at source and checked directly:
`118_probes/q3_the_fraction_width_splits_my_arms_too.py` at `q_of` and `R_of`;
`118_probes/q5_one_rule_with_two_locality_conditions.py` at `quantise`;
`119_leroy_the_canon_candidate_for_the_realisation_map.md`, grepped for `rounding = trunc`;
`125_probes/p3_commutation_with_overflow.py` and its output, in full;
`warm-clamp-shared/src/lib.rs` not opened (not needed for this attack; `122`'s own account of it at
`122:642-646` is a claim about a shipped test I take on `122`'s citation rather than re-verifying, since
it bears on the overflow topic rather than this one). Not read: `114` through `118` and `120`, `121`
themselves; my account of the overflow topic's history is `122`'s account of it, the same
single-point-of-failure both `125` and `126` disclose about their own phase-two sections.

Built and committed: one probe, `127_probes/w1_shared_threshold_stochastic_is_monotone_and_unbiased.py`,
with predictions stated in its header before it ran, four parts each with a control, and its output
committed alongside it before this file was written.

**What I did not do.** I did not sweep `Q_U` against wrap directly (only against saturate, in Part 4); I
expect it to hold by the same composition argument `125` T7 uses (wrap fails to be monotone regardless of
what is composed in front of it, per `126`'s own Finding 6 first paragraph, so `Q_U` composed with wrap
inherits wrap's non-monotonicity the same way every other mode does, and there is nothing left to check
that either file has not already established), but I did not run it and say so rather than assert it. I
did not measure the cost of drawing one shared threshold versus one per element; that is unpriced. I did
not check whether a position-keyed (rather than globally shared) variant of `Q_U`, the construction `126`
gestures at without building, gives a weaker or different monotonicity guarantee; my construction is
strictly stronger than that one (global rather than per-position) so I did not need to separately confirm
the weaker case to make my point, but a full account of the whole family is a further reconciliation and
not this dispatch's.
