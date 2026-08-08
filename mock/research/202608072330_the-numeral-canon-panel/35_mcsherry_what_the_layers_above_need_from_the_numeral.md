# 35. What the layers above need from the numeral

**Date:** 2026-08-08. **Position:** after `34`. **Mode:** explore, do not settle (`00_brief.md`, `04`,
`28`). Nothing here is a ruling, and where I say a route is closed I give the diagnostic that closed it.

## The gates

**Canon gate: passed, and there was nothing to defend.** There is no ratified canon for this panel to
align against; the fixed set is op's own files (`01`, `04`, `28`, `32`, `34`), the workspace discipline,
and the forbidden-feature list. I checked my work against all five op files and against
`~/Dev/clause-dev/.claude/rules/unstable-features.md`. Every probe below compiles on the pinned
`nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, with no feature gate of any kind:
`grep -c 'feature(' 35_probes/*.rs` returns zero on every one of the eleven files. Nothing here proposes
work op's files forbid, and where my finding bears on a question he answered, I treat his answer as
direction rather than as a lock, per `28`.

**Test gate: run, and reported rather than reasoned from.** `cargo test --workspace` in `mock/`: 204
suites, **751 passed, 0 failed, 9 ignored**, against 417 `#[test]` functions in `crates/`
(`grep -rn "#\[test\]" crates/ | wc -l`). An all-green suite is supposed to trigger the audit, and **I did
not audit those 417 bodies**, deliberately. `mock/crates` is the dead tier for canon purposes and reading
its tests to decide whether they are good tests is reasoning from a document already declared dead. I
report the number so nobody reads its absence as an unexamined green, and I claim nothing from it.

The tests that matter for this file are my own, and I state their controls, because a probe without one is
a claim about its own setup. `p1` carries four negative controls; `p2b` is a negative control on `p2`'s
positive result, on the axis `p2` held fixed; `p5` carries an in-range control that is the entire reason
its numbers mean anything; `p8` carries a **mutation check on its own checker** (break the recurrence, 33
assertions fail; leave it, clean); `p9` exists to refute a shape I myself proposed, and does; `p10`
carries two controls that isolate the mechanism. My first version of `p1` failed for a reason that turned
out to be my own shortcut rather than the design's, and that failure is kept in the file header because it
is the more useful half.

## 1. The finding, in one paragraph

**The panel has spent thirty-four files designing a widening binary operation, and the layer above it
cannot use one.** Every algorithm in the algorithm layer is a fold; a fold's accumulator is loop-carried,
so it has exactly one type; a widening operation gives it a different type on every iteration. Four
independent formulations of a widening fold are refused by rustc with the same diagnosis (`p1`,
`35_probes/p1.out`). What the layer above needs instead is a **closed** operation plus a **separately
determined accumulator**, and the accumulator's width is not a function of the operand widths at all. It
is a function of the element width and the **capacity**, which is a quantity that lives in the composition
rather than in the numeral. That single fact reorganises which of the panel's live options matter, and it
puts a derivation nobody has written down at the centre of what op called "the contracts for things that
compose to bigger units than just numerals alone".

The second finding is separable and I think it is the larger one. **Which algebraic laws a numeral obeys
is a per-strategy, per-sign-domain fact, and it is what decides whether the layer above is allowed to
reassociate, vectorise, thread, or algebraically rewrite a computation.** Op's `32` intent that arvo adapts
to the cores it finds, and his `34` correction that this may not cost soundness for any strategy except
`Hot`, together impose a precondition named nowhere: a reduction may only be split when the operation is
associative. Measured exhaustively, three of arvo's four sign-and-policy combinations are exactly
reassociable and the fourth is not, at 70.1% of inputs (`p3`).

## 2. Method, and what I refused to count

I derived the requirements from the mathematics of folds, then built instruments. Where the shipped tree
appears below it is cited for **what arvo used to do**, per the brief, never as evidence about what is
correct. Nothing in section 3 rests on it: every claim there is a compiled refutation, an exhaustive
count, or arithmetic.

I read the algorithm crates' trait bounds once, early, and they agree with the derivation so precisely
that I want to flag the risk rather than lean on it. `arvo-graph/src/path.rs:36` bounds its weight as
`W: Add<Output = W> + TotalOrd + Copy + FromConstant`, and `arvo-spectral/src/power.rs:44-50` bounds its
scalar as `F: Add<Output = F> + Mul<Output = F> + Sqrt<Output = F> + Recip<Output = F> + TotalOrd + Copy
+ FromConstant`. Every operation is closed (`Output = Self`) and there is no widening operation in either.
**That is corroboration I am deliberately not counting**, because it is one dead artifact and because a
design that shipped only closed operations would produce exactly this whether or not closure is right. It
is a fact about the old tree; the argument stands without it.

## 3. What the layers above require

Ten requirements. Each is stated so it would survive a rewrite, then the instrument that establishes it.

### 3.1 A fold's operation must be closed, or its trip count must be static

This is `p1`, and it is the one place here where the strongest available result was reachable: a contract
test that does not compile.

The probe models the panel's derivation surface in miniature, `Num<A> + Num<B> -> Num<S<max(A,B)>>`, a
total function from operand widths to a strictly wider result, with a real type-level max so the
derivation is heterogeneous rather than restricted to equal widths.

| arm | shape | outcome |
|---|---|---|
| A | widening op in a fixed-arity expression | compiles |
| C | widening op right-folded over a static-length hlist, 3 and 4 elements | compiles, width tracks the element count (`W3`, `W4`) |
| B1 | widening op in a loop over a runtime-length slice | **refused, E0308** |
| B2 | same, output named as the once-widened type | **refused, E0308, twice** |
| B3 | same, output left to inference as `impl Sized` | **refused, E0308** |
| B4 | same, written as recursion instead of a loop | **refused, E0308** |
| D | closed op in a loop over a runtime-length slice | compiles |
| E | closed op into a separately named wider accumulator | compiles |

Every arm B formulation gives the identical diagnosis, `expected Num<W>, found Num<S<<W as Max<W>>::Out>>`
(`35_probes/p1.out`). Arms A and C are what stop this being the too-strong claim that widening does not
work: it composes perfectly well when the arity is a compile-time fact, and arm C's width tracks the
element count exactly. **The boundary is the runtime trip count and nothing else.**

So the design's width algebra is available in expressions and unavailable inside every loop in the layer
above. That is not a defect in the algebra. It is where the algebra's domain ends, and a canon describing
the algebra without describing that boundary will be read as describing something the algorithm crates can
use.

`06` reached the same wall from a different direction and deserves the credit. `06:260-262`: "A fold's
accumulator cannot have its numeral grow per iteration, which the droplist records as impossible in
principle rather than merely unbuilt, since a type cannot depend on a runtime value." I agree; I got there
by compiling rather than by census; 3.2 is where I go past its conclusion.

### 3.2 The accumulator is derivable, from the element numeral and the capacity

`06:263-264` concludes from that wall that "the accumulator numeral is **consumer-written**, and the
design's contribution is a **verdict** rather than a numeral", filing it as `D0` plus a check (`06:233`).
That is one of three options, and the other two were not on the table, because `06`'s taxonomy classifies
a site by which **operands** determine the numeral and the missing input here is not an operand and not a
numeral.

It is the **capacity**, and in arvo a capacity is already a type. The exact trip count is runtime;
sufficiency needs only the bound; a sum of at most $C$ values each below $2^W$ is below
$2^{W + \lceil \log_2 C \rceil}$. So:

$$\mathrm{acc\_width}(W, C) \;=\; W + \lceil \log_2 C \rceil$$

`p7` compiles this as a two-input contract with an associated accumulator type, plus a generic fold that
names the derived accumulator without ever naming a width or a container. The derived widths are checked
by const assertion against arithmetic rather than trusted (4-bit elements at capacity 16 give 8; 8-bit at
256 give 16; 3-bit at capacity **3**, the non-power-of-two row, give 5; capacity 1 adds nothing), and the
sufficiency claim is checked the same way with negative controls showing one bit narrower is not enough. A
capacity with no derivation has no accumulator and the fold is refused (`--cfg arm5`), so the bound is
load-bearing rather than decorative.

**`p7`'s first version was inadmissible and I say so before anyone else has to.** Its `ceil(log2)` was one
impl per capacity, which is exactly the enumeration `SETTLED.md:110` refuses ("No enumeration, ever, if it
can be helped", RATIFIED four times). A construction that clears the mechanism bar by violating a ratified
rule has cleared nothing.

`p8` closes it. `ceil(log2)` is computed **inductively, with no table**, in three impls, over a positive
binary representation whose three constructors are pairwise disjoint by construction:

```
One                value 1
Twice<N>           value 2N       (N >= 1)
TwiceP1<N>         value 2N + 1   (N >= 1)

lg(One)        = 0
lg(Twice N)    = 1 + lg(N)
lg(TwiceP1 N)  = 1 + lg(inc N)
```

The disjointness is the whole trick and is worth carrying as a technique. The reason a table looks
necessary is a coherence collision at the base case: with an ordinary binary tower, `1` is both "the base"
and "a number ending in a set bit", so the base impl and the inductive impl overlap. A representation
carrying only naturals at least one has no such collision. This is the same move as
`a-refused-bound-wants-a-trait-not-a-feature.md`, applied one level further in: the obstacle was not the
arithmetic but the representation the arithmetic was written over.

The third line is the one to check rather than believe. `p8` checks the whole function against integer
arithmetic at forty values written out by hand, including every value 1 through 33 and both sides of 64,
256 and 1024, plus a sufficiency sweep over widths 1 to 24 against capacities 1 to 64, plus tightness
controls showing one bit narrower fails. Under `--cfg mutate`, which drops the increment from the third
line, **33 assertions fail**; without it the build is clean. `p8_scale_check.rs` carries it to capacity
65536, including the all-ones 65535 which walks the increment at all sixteen levels: compiles. That is an
**existence** result. How long it takes is unmeasured, and the word for that here is unpriced.

**The convergence worth reporting.** `20:208-210` records the bench crate's own interior-safety predicate
as `W + ceil(log2 n) <= width(accumulator)`, with the measured crossover landing at arity 8 "to the row",
and `20:216` reports that `accfit`, the arm picking the narrowest accumulator satisfying it, "is at or near
best at every arity. It is the design's own rule and it beats both the shipped rule and its proposed
deletion." Same formula, reached from a committed harness run rather than a type-level derivation, and it
says the derivation is not merely sound but is what wins. Two instruments, opposite directions, one
formula. I did not know `20` had it when I derived it, and found it while verifying citations; I would
rather say that than present it as a prediction.

### 3.3 A fold's seed must be representable, and it is not always

Every fold needs its operation's identity: `0` for a sum, `1` for a product, the top for `min`, the bottom
for `max`. A fold whose seed is not representable in the numeral it folds is wrong, silently.

`p4` sweeps the whole `(W, F)` box from width 2 to 10. The additive identity is always available. **The
multiplicative identity is absent at exactly the purely fractional shapes**, `F == W`, which is 18 of 126
rows, and in every one the seed a fold reaches for corrupts `2^W - 1` of `2^W` values on the first step:
the whole domain but one.

Not hypothetical and not mine. The shipped tree documents the same case at `arvo/src/ufixed.rs:90-95`:
"`UFixed<0, F, S>` spans `[0, 1)`, which does not contain one, and the raw encoding `1 << F` does not fit a
container of `F` bits: it wrapped to zero on Hot and saturated to just below one on Precise, and `x * ONE`
silently annihilated or shrank." I measured it rather than quoting it because the quote is from the dead
tier, and the measurement agrees.

The requirement that survives a rewrite: **the composition contract is keyed on the algebraic structure,
not on the numeral.** A product fold requires a multiplicative monoid, and `UFixed<0, F>` is not one. Keyed
on the numeral, the missing identity is a runtime surprise; keyed on the monoid, the fold does not
typecheck, which is where it belongs.

### 3.4 A min-plus fold needs an absorbing top, and only saturation supplies one

Graph algorithms are semiring computations, and the ones the algorithm layer holds are mostly tropical:
shortest path is `(min, +)`, longest path is `(max, +)`, widest path is `(max, min)`, reachability is
`(or, and)`. The additive identity of the tropical semiring is infinity, a bounded numeral has none, so an
implementation stands infinity on the top. That works only if the top **absorbs**: `TOP + x == TOP`.

`p4` measures it over 63 `(W, F)` cells per policy. Under saturation the top absorbs at **63 of 63**.
Under wrapping, at **0 of 63**.

### 3.5 Order-compatibility, and what it costs when it is missing

Every relaxation step rests on monotonicity: `a <= b` implies `a + c <= b + c`. `p2` counts it exhaustively
over the whole `(W, F)` box unsigned, `p2b` signed. It holds at **33 of 33** cells under saturation and
fails at **33 of 33** under wrapping, up to 33.07% of triples, both signednesses.

`p5` is a different kind of instrument and it is the one that decides whether any of this reaches an
answer. It runs two real DAG dynamic programmes, the shape `arvo-graph/src/path.rs` already has, over every
DAG on four nodes respecting a topological order and every weight assignment, against exact unbounded
arithmetic. **The control that makes it fair**: an instance is counted only when the exact answer and every
exact intermediate fit inside the numeral, so a disagreement cannot be blamed on the numeral being too
narrow for the problem.

| width | policy | routine | in-range instances | wrong answers |
|---|---|---|---|---|
| 3 | wrap | longest, max-plus | 11,785,152 | 0 |
| 3 | wrap | **shortest, min-plus** | 11,932,446 | **5,414,255 (45.4%)** |
| 3 | saturate | shortest, min-plus | 11,932,446 | 0 |
| 4 | wrap | longest, max-plus | 736,300,800 | 0 |
| 4 | wrap | **shortest, min-plus** | 832,398,764 | **407,293,133 (48.9%)** |
| 4 | saturate | shortest, min-plus | 832,398,764 | 0 |

The witness is minimal and it is 3.4's mechanism in one line: edge mask `001000`, only the edge `(1,2)`
present with weight 1. Node 1 is unreachable so its distance is the top; node 2's distance is `top + 1`,
which under wrapping is **zero**. A genuinely unreachable node reports the shortest distance there is.

Note what does **not** fail: max-plus is correct under both policies at every one of 736 million in-range
instances, because it seeds at zero and never uses the top as a sentinel. So this is not "wrapping breaks
graph algorithms". It is that **wrapping breaks the ones needing an absorbing top**, and the split is
total.

Three instruments, arrived at differently, agreeing: a pairwise monotonicity count, a per-cell absorption
count, and an end-to-end algorithm run with an in-range control.

### 3.5a The two properties are two, and conflating them cost me an option

I proposed, in an earlier draft of section 5, a shape nobody had written down: a numeral wrapping in its
interior (keeping the additive inverse, and behaving "like native primitives in regular old rust would")
with a **reserved absorbing top** so min-plus gets its infinity. One value spent, both consumers served.

`p9` builds it and refutes it, on the same harness and the same in-range control, with the three policies
compared on identical instance sets.

| policy | absorbing top | monotonicity of + | shortest path wrong, w=4 |
|---|---|---|---|
| wrap | fails 15 of 16 | fails 680 of 2176 | 48.1% of 622M in-range |
| saturate | 0 of 16 | 0 of 2176 | **0** |
| reserved top | **0 of 16** | fails 560 of 2176 | **12.6% (78.2M)** |

The reserved top buys the absorbing property outright and about three quarters of the improvement, and
still gets the answer wrong on one instance in eight. The shape is dead, and its corpse says something
sharper than the proposal did: **min-plus needs two properties, not one, and the register's wrap-or-clamp
framing names neither of them.** It asks which policy `Warm` takes; what the layer above needs to know is
whether the top absorbs and whether addition is monotone. Those are separable, one of them is buyable
without the other, and buying only that one is not enough.

### 3.6 A reduction may only be split when the operation is associative

This is what bears on `32` and `34`, and nobody has named it.

Splitting a reduction across lanes or cores changes the association order. `p3` computes the same fold four
ways per input, exhaustively over the whole input space: a strict left fold, a balanced pairwise tree, two
strided partial sums combined, and four. The last two are what a vectorised reduction and a per-core
partial-sum reduction actually do.

| numeral | policy | vectors whose answer depends on the split, n=8, exhaustive over 16,777,216 |
|---|---|---|
| unsigned fixed | wrap | 0 |
| unsigned fixed | saturate | 0 |
| signed fixed | wrap | 0 |
| **signed fixed** | **saturate** | **11,760,675 (70.1%)** |
| f32 | IEEE | 554,034 of 1,000,000 (55.4%, sample, not exhaustive) |

At n=4 the signed saturating figure is 43.2%; at n=8, 70.1%. It grows with the trip count.

**The positive, and nobody in the panel has stated it.** Three of four sign-and-policy combinations are
**exactly reassociable**, at zero failures over sixteen million vectors. Fixed-point addition is exact:
there is no rounding in an add at a fixed `F`, so the only thing that can break associativity is the
overflow policy, and for wrapping (a group) and unsigned saturating (a monotone clamp at one end only) it
does not. Float never has this property, at 55.4% here. **A fixed-point fold is splittable across any
number of cores and lanes with a bit-identical answer, and a float fold is not.** That is exactly what
op's `32` intent wants to be true, and it is true for most of the box.

**The negative, and it is precise.** `p2b` finds unsigned saturating addition associative at 33 of 33 cells
and **signed** saturating addition non-associative at 33 of 33, first witness at width 2: `a = -2, b = -2,
c = 1` gives `(a+b)+c = -1` and `a+(b+c) = -2`. Unsigned saturation clamps at one end and both orders
collapse to `min(a+b+c, MAX)`; signed clamps at both, and the classic counterexample needs exactly that.
**I predicted the opposite for the unsigned case and the probe corrected me**, which is why `p2b` exists:
it is the negative control on `p2`'s positive result, on the axis `p2` held fixed.

So the sign domain, which the panel has treated as a representation question, is also a **reassociation
licence** question, and `34`'s general principle that each strategy's "own purpose, intent, that shapes
what the answer is" has an instance on an axis `34` does not name.

**The consequence for `32` and `34` together.** Op's adaptation intent is conditional on two proofs, and
`34` makes the soundness one hard for every strategy except `Hot`. For a signed saturating numeral,
splitting a reduction across the cores arvo detects changes the answer on 70% of inputs, which is a
soundness sacrifice by any reading, so under `34` it is available to `Hot` and forbidden to the rest.
**As things stand, op's adaptation intent is blocked for signed saturating folds by op's own soundness
condition**, and nothing in the panel records that. I do not think it is a defect in the intent. I think it
is a missing sentence, and section 6 proposes candidates.

### 3.7 Algebraic rewriting is a soundness trade, and the fractional part is what makes it one

`p2` and `p2b` also count the ring laws, and the result surprised me enough that I went looking for which
cells survive.

**Multiplicative associativity and distributivity hold exactly at `F == 0` and fail everywhere else.**
Unsigned, per policy over 33 cells each: mul-assoc holds at 6 of 33 under wrapping and 7 of 33 under
saturation; distributivity at 6 of 33 under both. **Every holding cell has `F == 0`**, with exactly one
exception across all four columns, a tiny-box row at saturating `w=2, f=1`. Worst failure rates 87.5% and
72.6%. Signed: the same picture. (Counted with `awk` over `35_probes/p2.out`, filtering `$5==0` per law and
policy, then re-filtering on `$2!=0` to check the `F == 0` claim; that second count is 0 in three of the
four columns and 1 in the fourth.)

So what destroys the ring laws is not the overflow policy. It is the rescaling shift by `F`, which
truncates, and truncation neither associates nor distributes. **A fixed-point numeral with a fractional
part is not a semiring under any overflow policy**, and an integer numeral under wrapping is a ring.

**On folds the bite is smaller than the number suggests.** Multiplication is usually pointwise and then
summed, as at `arvo-spectral/src/power.rs:71`, so what a matvec or a dot product needs is associativity of
`+`, which 3.6 gives in three of four cells. Non-associative `×` bites only where `×` is itself the fold's
operation: path products, continued products, multiplicative DP costs. Real, and a minority.

**On rewriting the bite is the whole point.** Distributivity is what licenses `a*b + a*c -> a*(b+c)`, which
a fused kernel, a common-subexpression pass and a hand-written microkernel all do.
`~/Dev/clause-dev/.claude/rules/arvo-always-optimal-internals.md` licenses arvo's internals to reach for
exactly that, freely. Measured, that rewrite changes the answer at up to 70.5% of triples once `F > 0`.
Under `34` it is a soundness trade available to `Hot` against a proven meaningful gain and unavailable to
the other three.

The canon-shaped statement, offered as a shape rather than a sentence: **which rewrites the internals may
perform is a per-strategy fact, and it is the same axis as which reassociations a scheduler may perform.**
The internals rule and the adaptation intent are asking one question in two places.

### 3.8 What a composition needs that a value does not

A vector, a matrix, a CSR structure, a graph weight column and a spectral iterate each need, from the
numeral, things a single value never asks for.

**A per-aggregate quantity, not only a per-value one.** `16` established this independently and from both
directions, and it survives my derivation intact: the container derivation has **two** outputs, a carrier
and a stride, and `16` section 12's framing that "the derivation answers a per-value question and a
per-aggregate question" is exactly the distinction a composition contract needs. **This is a keep**, and it
is the strongest existing result for the question I was sent to ask, because it is the only place in the
panel where the composition layer has already forced something into the numeral's answer.

**A fold accumulator keyed on capacity**, per 3.2, which is a *third* thing keyed on the aggregate rather
than the value. `16`'s two outputs are about layout; this is about arithmetic. So the count of
aggregate-keyed outputs is at least three: carrier, stride, and the accumulator relation. I am not
proposing they are one mechanism.

**An identity that is the fold's, not the numeral's.** A sparse structure omits elements, and an omitted
element means the fold's identity: zero for a sum, **infinity** for min-plus. A CSR contract keyed on "the
numeral's zero" is wrong for every tropical algorithm; keyed on "the monoid's identity" it is right for all
of them. Same conclusion as 3.3, from the aggregate side.

**A total order**, for every routine that sorts, ranks, selects a minimum, or decides a DP.

### 3.9 Retraction: I went looking for the requirement and found the opposite

I expected **retraction** to matter: an aggregate maintained under updates needs an additive inverse to
withdraw a contribution, and `p2` measures that wrapping preserves it at 33 of 33 cells while saturation
loses it at 33 of 33, up to 49.6%.

Rather than leave that as a question I could not answer from inside arvo, I read the downstream engine,
read-only, no writes, and the answer is informative in a way I did not expect.

Its incremental machinery is **incremental skip**, not incremental aggregate:
`hilavitkutin/mock/crates/hilavitkutin/src/plan/dirty.rs:1-5` describes a dirty mask that "tracks which
stores changed since last frame so downstream fibers can skip when their inputs are clean". That is coarse
invalidation over a bitmask. It needs no arithmetic inverse and creates no retraction requirement.

The one running aggregate is an exponential moving average
(`hilavitkutin/mock/crates/hilavitkutin-providers/src/adapt_ema.rs`), and an EMA is the canonical structure
you reach for **precisely because it does not need an inverse**: old contributions decay rather than being
withdrawn.

So the requirement is not established, and the reason it is not is worth more than the answer: the one
place downstream that maintains a value under updates has already chosen the structure that avoids needing
a group. **Retraction stays an open question about what the composition contract forecloses**, not a
requirement. It is cheap to state now and expensive to retrofit: a contract distinguishing a monoid
(maintainable only by recomputation or a tree) from a group (maintainable in place) costs one named
structure today, and a downstream engine wanting incremental maintenance later cannot add it without
changing every contract. I flag it and claim nothing.

### 3.10 The same measurement, landed on a named downstream invariant

Reading that EMA provider produced a checkable prediction, so I checked it.

Its module documentation states the update as `dst[i] = dst[i] * NORM_7_OVER_8 + src[i] * NORM_1_OVER_8`,
with the blend factors carried as sixteen-bit fractional constants (raw `0xE000` and `0x2000`), and states
that they "sum to exactly `1.0` (`0x10000` in the same repr), **preserving the EMA invariant**".

The two constants do sum to one. What is inferred from it does not follow, and the reason is 3.7 exactly.
The chain `x*a + x*b = x*(a+b) = x*1 = x` has three steps and **the first one is distributivity**, which a
truncating fixed-point multiply does not have. Two constants summing to one is a fact about the constants;
it does not make the computation fix its input.

`p10` measures it over samples 0 to 4095:

| check | result |
|---|---|
| one update returns its input, `dst == src == x` | **fails at 3584 of 4096 (87.5%)** |
| the failure holds exactly when `x` is not a multiple of 8 | 0 violations of that structure, 4096 tested |
| steady state under a constant stream equals the input | **fails at 3584 of 4096 (87.5%)** |
| worst steady-state shortfall | **7 at x = 7, a 100% relative error: a constant stream of 7 settles at 0** |
| same, with round-to-nearest instead of truncation | fails at 512 of 4096 (12.5%) |
| **any** pair of 16-bit fractional constants summing to exactly one | **65,535 of 65,537 pairs fail** |

The last row is the generalisation and the one worth carrying: **"the weights sum to one" never implies
"the update fixes its input" under truncating fixed-point multiplication, for any pair but the two
degenerate ones.** The round-to-nearest row is the control that says the mechanism is the rounding rather
than the constants.

**What this is and is not.** It is a statement about the formula as documented, under truncating
fixed-point multiplication. It is **not** a claim about shipped behaviour: that provider's body is marked a
stub in its own source, so there is nothing shipped to be wrong. It is not a claim about another
repository's design either, since that source is agent output like everything else. It is a prediction,
with the arithmetic that produces it, handed over for whoever implements the body.

For this panel the interest is different and it is about `34`. The documented carrier is `Hot`, and `34`
says `Hot` may sacrifice soundness for a **proven meaningful gain**. A 100% relative error at small samples
might well be an acceptable price for `Hot`. What it must not be is unnoticed, and a comment asserting the
invariant is preserved is the shape of a trade nobody priced. `34` names that exact failure: losing
soundness "for nothing". So this is a worked instance of why `34`'s per-strategy framing needs to reach the
law layer and not stop at the strategy table.

## 4. Where the requirements conflict

Five tensions, none resolvable by picking a better numeral. They are tensions **between consumers**, which
is why a strategy axis exists.

**Wrapping against saturation is a straight trade, not a preference.** Wrapping buys the additive inverse
(0 of 33 cells fail) and costs monotonicity (33 of 33) and the absorbing top (63 of 63). Saturation buys
both of those and costs the inverse (33 of 33). No policy has all three, and 3.5a shows a hybrid buying the
absorbing top alone does not close the gap.

**Signedness against reassociation.** Unsigned saturating folds split freely; signed saturating folds do
not, at 70.1%.

**The fractional part against the ring laws.** `F > 0` is what fixed point is *for*, and it is what makes
the numeral not a semiring. Every algorithm reasoning algebraically about a fixed-point expression is
reasoning about something that is not a ring, and 3.10 is one instance of the consequence.

**Genericity against declared outputs.** Section 5's Q9 is the whole of this one: the arrangement with the
best diagnostics at a concrete call site has the least to offer a generic algorithm crate, and the
algorithm crates are the largest population of generic consumers arvo has.

**The tightest-answer question does not arise in a fold.** `07`'s soundness-against-bestness fork is about
the derived numeral being the smallest containing the result. In a fold the accumulator is determined by a
capacity bound rather than an exact result set, so the tightest honest answer is `W + ceil(log2 C)` and
there is nothing to tighten: `p8`'s controls show one bit narrower is genuinely insufficient. The bestness
question is an expression-layer question and the fold layer answers it for free. Small, real, and a
simplification rather than a cost.

## 5. Bearing on the live options

Per `OPTIONS.md`'s own instruction, each gets fits-well, fits-badly, or kills. I cite `OPTIONS.md` by
section and quoted phrase rather than by line, per my brief.

### Q1, what "then validate" requires

**Fits well, and adds instruments the register asks for.** Q1's admissibility part records "Panel evidence:
none" and owes "a two-directional sweep". `p7`'s `--cfg arm5` is an admissibility refusal in exactly that
sense: a capacity with no derivation has no accumulator, so the declaration is refused rather than served
wrongly. `p6`'s `a5r` and `a5c` are two more, in the over-permission direction. Three admissibility
refusals now exist where the register records none. That does not discharge the sweep, which is a much
larger thing; it moves the count off zero.

**And it sharpens self-validation.** Q1's self-validation is "the derived container actually holds the
declared range, checked at derivation time rather than assumed", evidence "incidental". `p7` and `p8` do the
fold-layer analogue and check it: the derived accumulator holds what the fold can produce, asserted against
integer arithmetic across a sweep, with negative controls and a mutation check. That is a different
self-validation from the container one and the register has no slot for it.

### Q2, which coordinates a consumer writes

**Neutral, and I tried to make it decide.** `p8`'s derivation adds `ceil(log2 C)` to the **total** width:
a sum of fixed-point values keeps its fractional bits and grows only its integer part, so the accumulator
is `(W + lg C, F)` in one coordinate system and `(I + lg C, F)` in the other. One addition either way. It
genuinely does not decide Q2.

The **fourth reading** (grid and reach, `24`) fits marginally better, for a reason `24` did not have: the
accumulator relation is the **reach** growing while the **grid** stays fixed, one coordinate moving in that
vocabulary against a compound statement in the other. That is an aesthetic argument and I mark it as one.

### Q3, is there a mixed-numeral addition

**A fold is evidence for the "none exists" reading, and I want to be careful how much.** A fold adds values
of one numeral into an accumulator of another, which looks like mixed addition and is not: it is a closed
addition in the accumulator's numeral, preceded by a widening conversion of each element. `p7`'s contract
has exactly that shape, a `lift` then a closed add, and `p1` arm E the same. So the fold layer, the largest
consumer of addition in the design, needs **no** mixed addition and would not use one. It needs the
**conversion**, which is the third option, and it needs the conversion to be where the lossless predicate
fires.

A real vote for the first or third option over the second, from the largest caller. Not a kill: an
expression-layer consumer adding two differently-shaped values is a separate population I have no evidence
about.

### Q4, what a datum stands for

**The absorbing-top reading gains an argument nobody has made, and it is not a soundness argument.** `18`
evaluated the absorbing top for whether it makes a denotation sound, found the "exactly" qualifier false,
and left the necessary condition open. Independent of all that, the absorbing top is **what min-plus
arithmetic requires**: 63 of 63 cells absorb under saturation, 0 of 63 under wrapping, and `p5` shows the
consequence is a 49% wrong-answer rate in a real routine. So the absorbing top is not only a candidate
denotation, it is a load-bearing algebraic property of the top that the algorithm layer cannot do without.

**And 3.5a sharpens what the algorithm layer is actually asking for**, which is not the absorbing top alone:
`p9`'s reserved-top numeral absorbs perfectly and still gets 12.6% of shortest paths wrong. Any denotation
reading that delivers absorption without monotonicity has delivered less than half of it.

**The "a set, admitted generally" reading fits very badly, and I stop short of killing it.** `OPTIONS.md`
Q4 records it "costs the total order, multiplicative associativity outright... and the additive inverse
except on degenerate data". 3.8 says the composition layer needs a total order at essentially every routine
and 3.5 says the DP relaxations need monotonicity, so admitting sets generally removes the preconditions of
most of the algorithm crates. Not a kill, because a coherent answer remains: sets are a separate family the
algorithm contracts do not accept. But then the canon owes that sentence, and the option's cost is bigger
than "the consumer writes the law layer themselves" makes it sound. It is "the algorithm layer does not
accept this type", which for a library whose selling point is the algorithm crates is a different size of
cost.

**The soundness-against-bestness sub-fork:** section 4's last paragraph. In a fold, bestness is free.

### Q5, is the arithmetic column one axis or two

**Fits the two-axis and product readings well; fits one-axis badly, on a new ground.** The register's
evidence for two axes is that the presets answer two different questions and that a bench family shows
accumulator and container are independent. Mine is different in kind: the **laws** partition along one axis
(overflow policy: wrap gives the inverse; saturate gives monotonicity and absorption) and the
**reassociation licence** partitions along a second thing not in the preset table at all, the sign domain.
An arithmetic column with one axis cannot state which laws a strategy carries, because the laws are not a
function of a single value.

**And I would add an axis to the product reading, or at least a question about one.** Q5's product entry
says "the axis list itself is open past the two contested here" and names SIMD lane count (argued derived)
and rounding (absent from arvo). I would add: **which algebraic laws the numeral guarantees, and therefore
which rewrites and reassociations are licensed.** It may be derived from the overflow policy and the sign
domain rather than primitive, which is exactly the question worth asking. But it is the axis a downstream
algorithm's bound has to name, and the axis `34`'s soundness condition is quantified over, so if it is
derived the canon owes the derivation.

**And rounding, which the register lists as absent from arvo, is not absent from the consequences.** 3.7
shows the rescaling shift is what breaks distributivity, and `p10`'s round-to-nearest control drops a
downstream invariant's failure rate from 87.5% to 12.5%. Whatever else rounding is, it is an axis that
moves the law layer, which is a stronger reason to carry it than "the prior art has one".

### Q6, does `Warm` wrap or clamp

**My evidence bears on this hardest, and it does not settle it. It re-prices it and then re-frames it.**

If `Warm` wraps, `Warm` values cannot carry min-plus algorithms: 0 of 63 cells absorb and a real DAG
shortest path is wrong on 48.9% of in-range instances. Longest path is unaffected. If `Warm` clamps,
min-plus works everywhere and the additive inverse is lost at up to 49.6% of pairs, which matters only to a
consumer maintaining an aggregate incrementally, and 3.9 finds no such consumer downstream and finds the
near-miss deliberately structured to avoid needing one.

So the algorithm layer's interest is **asymmetric and points at clamping**, which is what the ratified
preset table says. The register also records op declaring that cell stale under his restated intent that
`Warm` behave "like native primitives in regular old rust would", and native primitives wrap in release.
That is a genuine conflict between op's restated intent and what the layer above needs, and I would rather
name it than resolve it.

**I proposed a fourth entry to dissolve it and then killed it myself**, which is 3.5a: a numeral wrapping in
its interior with a reserved absorbing top. Built in `p9`, it absorbs correctly and still gets 12.6% of
shortest paths wrong, because min-plus needs monotonicity too and interior wrapping destroys it. It belongs
in `DROPLIST.md`, and section 6 states it with what would reopen it.

**What survives from the attempt is a reframing rather than an option.** The question "does `Warm` wrap or
clamp" is at the wrong granularity for the consumer that cares. What min-plus needs is that **the top
absorbs** and that **addition is monotone**, and the wrap-or-clamp axis names neither. A canon stating the
policy states one thing and leaves the consumer to derive two; a canon stating the two properties per
strategy says what a downstream bound needs. That is a candidate shape for the register rather than a
proposal, and it is the same shape as Q5's suggested law axis.

**One caution on my own evidence.** `p5`'s in-range control makes its numbers a statement about arithmetic
rather than range exhaustion, which is what I wanted, and it also means the 48.9% is measured on instances
where a wrapping implementation had every value it needed and still got the answer wrong. Outside the
control both policies are wrong in different ways and I did not count them.

### Q7, which carrier the packing claim is about

**No bearing, and I checked.** My requirements are about arithmetic and typing; packing is about bytes and
bandwidth. The one connection is structural: `32`'s regime-sensitive answer, "the claim is about whichever
regime is detected, with at least two arms behind that detection", is the same shape as 3.6's problem, an
arm chosen by detected core count. If the packing arm and the reduction arm key on the same detection, the
canon has one mechanism to describe rather than two. An observation, not evidence.

### Q8, one numeral family or several

**The algorithm layer does not care how many families there are, and that is a finding rather than a
shrug.**

Every requirement in section 3 is a requirement about an **algebraic structure**: a closed associative
operation, an identity, an absorbing top, an order compatible with the operation, a widening relation into
an accumulator. Not one mentions family membership. A contract keyed on structure is satisfied by a numeral
from any family that has the structure and unsatisfied by one that lacks it, and the family relation never
enters. `p6`'s A1 arm is this compiled: a generic routine bounded on an accumulation trait works for any
element type whose impl exists, and never learns a width, a container or a family.

So the family question is load-bearing for **inference** (what numeral a mixed-operand expression produces)
and not for **algorithms**. Given that op names the algorithm crates as the selling point, that reweighs
it: whichever way it goes, the layer he named is unaffected. It fits every live option equally, which is a
boring result and I believe it.

**One exception, the step-set reading (E).** E computes family membership from nested step sets rather than
declaring it. A composition contract saying "these two numerals may be summed into this accumulator" is also
a computed relation over numerals, and `p6`'s `AtLeast` and `p7`'s `SumAccum` are both that shape: a
relation, inductively defined, no declaration. So E is the only family reading whose **mechanism** is the
mechanism the composition contracts need, which is a point in its favour on the "serves all other parts of
arvo best" criterion, and a weak one, because sharing a mechanism is not evidence of correctness.

### Q9, the crossing at the width surface

**Where my evidence changes something concrete, and it is a fits-badly rather than a kill.**

`13` names the gap and nobody has attacked it. `13:624`: what arrangement D "does to tier one, whose premise
is `T: Add` with no typestate at all. That is the first thing I would" attack next. The algorithm crates are
the largest population of that generic tier, so I attacked it from their side.

`p6` builds six arms:

| arm | how a generic routine gets an accumulator | outcome |
|---|---|---|
| A1 | associated type on a trait the numeral implements | compiles; names `T::Acc`, never sees a width |
| A2 | extra type parameter, bounded by a trait **relation** | compiles; refuses a too-narrow accumulator |
| A3 | const-generic widths, comparison **in a where clause** | **refused: "generic parameters may not be used in const operations", help: add `generic_const_exprs`** |
| A4 | const-generic widths, comparison as a post-mono `const { assert!(..) }` | compiles at the definition site |
| A5c | A4 instantiated too narrow | refused at instantiation, message "accumulator narrower than element width", pointing at the call |
| A5r | A2 instantiated too narrow | refused, `W0: AtLeast<S<S<S<W0>>>>` not satisfied |

**A3 is the finding.** Arrangement D is "declare the output width explicitly; check it is wide enough by a
free type-level comparison". At a concrete call site the comparison is between literals and is free. At a
**generic** definition site the widths are parameters, and the comparison in a bound position terminally
names `generic_const_exprs`, which is forbidden. So D's check, in its const-generic spelling, does not
reach the algorithm layer at all.

Not dead, and this is the composition rather than the winner. Two spellings survive and they are not
equivalent:

- **A2, the trait relation.** The check lives in the signature, so it composes: a caller's own bound can
  rest on it and the refusal lands at type-check. Its diagnostic is a digit tower, the exact cost `13`
  identified for the nat-keyed arrangements.
- **A4, the post-monomorphisation const assertion.** The message is genuinely good, carrying the
  consumer's own words and pointing at the instantiation. But the check is invisible in the signature, so
  no caller can rely on it, and it only fires if the function is actually instantiated and codegen'd.

A per-site answer rather than a global one: **the trait relation where the constraint must compose upward,
the const assertion where it need only hold locally and the message matters.** They coexist; `p6` compiles
both in one file. This also extends `a-refused-bound-wants-a-trait-not-a-feature.md` to a case it does not
currently cover: the refused bound here is a **comparison** rather than a derived quantity, and the trait
spelling is a relation with two impls rather than an associated type. Same move, different shape.

**Bearing on the other Q9 entries.** C4/A (cross once at literals, in one direction) fits my requirements
**well** and is what `p7` implicitly assumes: the numeral keyed on nats, the consumer writing literals, a
generic routine working in nat-land where the derivation lives. `13`'s proposed canon sentence, **cross
once, at literals, in one direction**, survives my derivation untouched and I record it as a keep: every
probe here obeys it without my aiming at it, which is what `16` also reported.

### Q10, the order's predicate on singletons

**No bearing found, and I looked.** My requirements use the order as a relation on pairs, the same use
`06:467-470` identifies for the sufficiency check. A numeral carrying fewer than two values is not a case a
fold's accumulator reaches, since the accumulator is at least as wide as the element. `03`'s request for a
second read is still unmet by me.

### The questions op has not been asked

**"Does the canon carry a numeric threshold at all, or only the inequality":** fits well. Everything in
section 3 is an inequality or a law, and the only number in `p7`'s derivation is a `ceil(log2)`, a function
rather than a threshold.

**"Is the derived numeral required to be the tightest honest answer":** section 4's last paragraph. In a
fold, tight and sound coincide.

**"What a strategy is":** `25`'s definition, at `25:528`, "**A strategy is a consumer-written name for one
coherent policy over how a numeral is represented and** how its arithmetic behaves", with strategies as
named sections over a product of axes, **survives my derivation and I would keep it**. My addition is that
one of the things a section fixes is **which laws hold**, and therefore which reassociations and rewrites
are licensed, and that this is the property `34`'s soundness condition is actually quantified over. `34`
itself says the sections differ in what they may trade and that `25`'s definition does not currently carry
it. I agree; 3.6, 3.7 and 3.10 measure what the trade is.

## 6. What the register should gain

I am not editing `OPTIONS.md`, per my brief. These are for whoever does.

**A new question, and the most valuable single item here. Q11: what does the numeral guarantee to a fold,
and what does a composition supply?** None of Q1 through Q10 is about arity-n or about laws under
reassociation; all ten are about a single value or a single binary operation. That is a gap in the register
rather than a wrong ordering (section 7). Its live options, in full:

- **The numeral carries nothing extra; a fold is the consumer's problem.** Cheapest. Costs the algorithm
  crates the ability to state their preconditions, so each re-derives sufficiency by hand and the
  wrong-answer classes in 3.4, 3.5 and 3.10 are undetectable at compile time.
- **The numeral names its algebraic structure**, so a contract keys on "an ordered monoid with an absorbing
  top" rather than on a numeral. Buys the fold-seed and sparse-identity requirements (3.3, 3.8) and makes
  `UFixed<0,F>` fail to typecheck as a product fold's carrier rather than annihilate at runtime. Costs a
  vocabulary of structures the canon must name and keep.
- **The numeral names its accumulator relation**, keyed on a capacity, per `p7` and `p8`. Buys sufficiency
  by construction and makes `20`'s `accfit` rule the default rather than an arm. Costs a second input to a
  derivation that currently takes only numerals, which `06`'s D0/D1/D2/D3 taxonomy has no cell for.
- **Both**, which is what `p7` compiles: the structure names what a fold may do, the capacity how wide the
  result is.
- **The composition supplies everything and the numeral stays a value type.** The mirror image. Puts the
  accumulator relation in `arvo-tensor`-shaped code rather than in the numeral, at the cost of every
  composition re-deriving it.

**A new question, Q12: is the reduction order specified, or is associativity required?** The escape from
3.6, and a genuinely different shape from anything in the register:

- **Require associativity.** A fold may be split only where the operation is associative, which by `p3` is
  three of four sign-and-policy cells. Signed saturating folds run in one lane, or run under `Hot`.
- **Specify the reduction shape in the canon**, as a fixed tree over the index range, independent of the
  detected lane and core count. Then the answer is a function of the input and the numeral alone,
  deterministic at any thread count, and `32`'s adaptation is unblocked for every strategy. Costs the
  sentence that a fold is not a left fold, and costs a single-core implementation a tree it does not need.
- **Make the reduction shape part of the strategy**, so `Hot` splits freely and takes what it gets and the
  others take the specified shape. Fits `34`'s per-strategy framing exactly, and costs an axis.
- **Say nothing**, and let the answer depend on the core count. Recorded so the space is not silently
  three-sided; under `34` it is a soundness sacrifice for every strategy but `Hot`.

**A candidate reframing rather than an option, under Q6 and Q5 together.** State per strategy **which
properties the arithmetic has** (does the top absorb, is addition monotone, is it associative, is it
invertible, does it distribute) rather than which policy it takes. 3.5a is the argument: the policy names
one thing and the consumer needs two, and a hybrid buying one of the two is measurably not enough.

**Two additions to existing questions:** an axis candidate under Q5's product reading (which laws the
numeral guarantees, possibly derived from overflow policy and sign domain, with rounding shown by `p10` to
move it); and under Q9, the finding that arrangement D's check has two surviving spellings at generic sites
and that the const-generic one is not among them.

**Two droplist entries, each with its diagnostic and what would reopen it.**

*The const-generic spelling of a width comparison in a `where` clause, at a generic definition site.* Closed
by `p6` arm A3: rustc refuses with "generic parameters may not be used in const operations" and terminally
names `generic_const_exprs`, forbidden. **Reopened by:** `min_generic_const_args` reaching the point where a
comparison of two const parameters is expressible in a bound. Restating it as a relation is A2, a different
mechanism rather than a repair of this one.

*A numeral wrapping in its interior with a reserved absorbing top.* Proposed in this file and closed in this
file by `p9`: it absorbs at 0 of 16 failures, matching saturation, and still gets DAG shortest path wrong on
12.6% of 622 million in-range instances, because interior wrapping destroys monotonicity (560 of 2176) and
min-plus needs both. **Reopened by:** a use case needing the absorbing top and the additive inverse but not
monotonicity, which I could not construct and do not believe exists among the tropical algorithms.

## 7. Is the panel asking the wrong question by starting at the numeral

Partly, and not in the direction the question suggests.

**The ordering is fine.** The numeral is the base and the base has to work, which is op's own framing at
`32`. Deriving the compositions first and the numeral from them would produce a numeral shaped by whichever
compositions happened to get written first, which is the failure the panel is already avoiding elsewhere.

**The question set is incomplete, and that is the real answer.** Q1 through Q10 are without exception about
a datum or a binary operation: what a datum denotes, what two operands produce, whether two numerals are
comparable, what a width is called. The layer op named as the selling point never performs a binary
operation in isolation. It performs folds, over aggregates, at capacities, with laws it needs and seeds it
needs and an order it needs. **A register with ten questions and none about arity-n is not asking the wrong
question first. It is missing a column.** Q11 and Q12 are what I would add.

**One thing the panel is doing that I would call unlicensed, and I was asked to report these even outside
my question.** The register carries the derived-numeral machinery (the tight product form, the meet and
join, the closure conditions, the negative-integer-width corner) as though it were the design's arithmetic
surface. `06` already found the extrema have no located caller after two independent looks, and `p1` adds
that the widening derivation is unavailable in every loop in the layer above. So machinery whose consumer
count is **zero located callers for the extrema** and **zero folds** occupies a large share of the register.
I am not saying it is wrong. I am saying that on op's own criterion, which shape serves all other parts of
arvo best, it has not been shown to serve any part, and the panel has not asked it to.

## 8. What I could not determine

**Whether `Precise` widens compute past storage**, which `16` names as genuinely undetermined and which
bears on 3.2: if it does, the accumulator relation and the container derivation interact and I do not know
how. I did not attack it because `16` already established it is undetermined rather than unmeasured.

**Whether the accumulator relation is one mechanism with `16`'s stride**, or a third aggregate-keyed output.
I said "at least three" and I do not know whether they unify.

**The float family.** Every fixed-point number here is exhaustive; every float number is a sample and says
so. Sections 3.5 through 3.7 are about fixed point, and a numeral concept covering both (Q2's fourth
reading, `08`'s general canonical exponent) needs the law layer re-derived for the float case, where I have
one sampled instrument and know associativity fails.

**What `ceil(log2)` at the type level costs.** `p8_scale_check.rs` establishes it reaches capacity 65536,
including the all-ones case. Trait-solver depth, compile time and behaviour at larger capacities are
unmeasured, and `arvo-compile-time-last.md` says compile time is last but not ignored. Unpriced.

**Whether the reassociation licence extends past addition.** `p3` measures folds over `+` only. A fold over
`min` or `max` is associative unconditionally and I did not measure it; a fold over `×` is not, per 3.7, and
I did not measure the split-dependence rate for it.

## 9. Coverage, bounded honestly

**Read end to end:** `00_brief.md`, `RULES.md`, `01`, `04`, `28`, `32`, `34`, `29`, `33`, `05`,
`OPTIONS.md` (all 912 lines).

**Read in the region I cite, by opening the lines:** `06` (sites table, sections 2.1, 2.2, 5.1), `20`
(section 1.5 and the interior-safety predicate), `13` (the tier-one gap at 410 and 624), `SETTLED.md` (the
enumeration row at 110), `18` (the total-order cost), `25` (the strategy definition at 16 and 528). **Every
`file:line` in this document was opened and its content checked against my claim**, not merely resolved.

**Read read-only, in another repository, no writes:** `hilavitkutin/mock/crates/hilavitkutin/src/plan/
dirty.rs` (header) and `hilavitkutin/mock/crates/hilavitkutin-providers/src/adapt_ema.rs` (documentation and
constants). Both are agent output on the suspect rung, and 3.9 and 3.10 mark what I take from them: a
located absence, and a prediction about a formula whose body is a stub.

**Not read:** `02`, `03`, `07`, `08`, `09`, `10`, `11`, `12`, `14`, `15`, `16`, `17`, `19`, `21`, `22`,
`23`, `24`, `26`, `27`, `30`, `31`, `CANON_CANDIDATE.md`, `DROPLIST.md`, `MORNING.md`, `seed/`. Where I
refer to their findings I rely on `OPTIONS.md`'s account and say so in the text each time. **The specific
risk:** `16` and `24` are the two I lean on most (the two-output derivation, the grid-and-reach reading) and
I have read neither. If `OPTIONS.md` misrepresents them, 3.8 and my Q2 remark inherit it.

**Not verified:** that my requirements match everything hilavitkutin and vehje ask of arvo. I derived them
from the mathematics of folds and checked two downstream files against one question. I did not read vehje at
all. Op named both consumers at `32`, so this remains the largest gap, and a dispatch that reads them would
be checking my premise rather than extending my result.

**Not measured:** anything about performance. There are no benches here and nothing in this file prices
anything. Where a magnitude would decide something I have said it is unpriced.

**Probes:** `35_probes/`, committed at `10cdb47` and `98b7807` with sources, raw compiler output and run
logs. `p1` (compiled refutation, 8 arms), `p2` and `p2b` (exhaustive law sweeps, unsigned and signed, whole
`(W,F)` box to width 7), `p3` (reduction order, exhaustive to 16.7M vectors plus a declared float sample),
`p4` (identities and absorption, box to width 10), `p5` (end-to-end algorithm, 832M in-range instances),
`p6` (generic accumulator, 6 arms), `p7` (capacity-derived accumulator), `p8` plus its scale check
(table-free `ceil(log2)` with a mutation check), `p9` (the refutation of my own proposed shape), `p10` (the
downstream EMA prediction with two controls). All on `nightly-2026-05-28`, no feature gates.
