# 139. The strategy set, derived cold

**Phase one.** Written having read only `INTENTS.md`, `RULES.md`, my dispatch brief, and the workspace
rules that load automatically into every session in this tree. No panel file, no `OPTIONS.md`, no
`AGREEMENTS.md`, no `DROPLIST.md`, no probe directory other than my own, no git log. This file is
committed before any of that is opened. Phase two is appended afterwards and phase one is not rewritten.

**Contamination I have to declare up front, because it is real and it bears directly on my results.**
This repository's `.claude/` rules load into the session automatically, and one of them,
`arvo-always-optimal-internals.md`, contains a paragraph stating that a one-sided clamp is a congruence
and a two-sided one is not, and reporting measured law-failure rates from this panel. I did not choose to
read it and I cannot unread it. Two of my findings land in that neighbourhood. Where they do, I say so
inline and I do not claim independence. My probe still refuted my own prediction on that exact point,
which tells you how much of it I had actually internalised, but the exposure is there and it is not mine
to wave away.

---

## Gate results, both of them, before anything else

**Canon gate: passed.** Checked against `INTENTS.md`, entry by entry. The dispatch asks what the strategy
set is, and I1 is explicitly demoted to OPEN on op's own word (`INTENTS.md:51-61`), which is precisely the
licence to ask. I17 (`INTENTS.md:363-383`) says the count "is besides the point of the intent", which
means a derivation that declines to fix a count is not evading the question but answering it. Nothing in
this file argues for dropping or downgrading the storage-minimising concern, which is what I17 forbids.
I15's "never any runtime checks, ever" (`INTENTS.md:299-310`) is a constraint my p5 probe tests against
rather than around. I14's operating constraints (`INTENTS.md:284-289`) are treated as in force.

One thing I want on the record rather than filed quietly. **The brief tells me a strategy has been
established as a two-component object, and one of my findings is a claim against that as stated.** Section
4 says why: the pair as described is not closed under the thing it has to be closed under, and the missing
piece is a law rather than a third component. That is visible rather than folded in, as the brief
required.

**Test gate: run, and it does not come out clean.** Thirteen crates under `mock/benches/variants/` carry
tests. Twelve of them run green:

| crate | tests | result |
|---|---|---|
| bitpack-carrier-shared | 9 | ok |
| bitpack-contend-shared | 12 | ok |
| bitpack-footprint-shared | 6 | ok |
| bitpack-plan-shared | 5 | ok |
| bitpack-shared | 3 | ok |
| bitpack-wide-shared | 6 | ok |
| quantiser-fadd-shared | 1 | ok |
| quantiser-radix-shared | 3 | ok |
| satfold-shared | 11 | ok |
| warm-clamp-shared | 7 | ok |
| warm-container-shared | 15 | ok |
| wide-rung-shared | 30 | ok, 188 seconds |

108 tests, all passing. The thirteenth, `bitpack-write-contend-shared`, declares 15, which brings the
total to **123 and confirms the brief's figure**.

**My own count said 124 and the brief was right.** The command

```
grep -rc '#\[test\]' --include='*.rs' mock/benches/variants/ | grep -v ':0'
```

sums to 124 because `bitpack-write-contend-shared/src/stress.rs:68` is a doc comment whose prose contains
the token `#[test]`. A grep counts lines and cannot tell an attribute from a mention of one. I am leaving
this in rather than silently fixing the number, because the standing instruction is that counts are
measurements produced by a named command, and the corollary nobody states is that **a named command can
still be the wrong command.** The number that settles it is the one the test binary prints.

**And `bitpack-write-contend-shared` does not complete in a debug build.** I ran it twice, foreground and
background, and each burned over an hour of CPU. It is not deadlocked. Twelve of its fifteen tests pass in
seconds; the harness named the three that do not before I killed it:

```
test stress::guarded_kernel_never_corrupts_under_real_concurrency has been running for over 60 seconds
test stress::naive_kernel_corruption_rate_under_real_concurrency has been running for over 60 seconds
test stress::naive_kernel_never_corrupts_when_the_split_is_aligned has been running for over 60 seconds
```

The arithmetic is in the source. Those three run 500, 3000 and 1000 trials (`stress.rs:88`, `stress.rs:99`,
`stress.rs:121`), `corruption_count` calls `build_bytes` once per trial (`stress.rs:43`), and `build_bytes`
materialises a buffer sized from `input.rs:17` `MAX_N = 4_194_304`, which `input.rs:19-29` sums to roughly
23 MB. Forty-five hundred of those is on the order of a hundred gigabytes of allocation and fill, with four
threads spawned per trial, in unoptimised code, to exercise a race on a 4094-element input.

**That is a gate finding and I am not softening it.** A suite that cannot be run in its default
configuration is a suite that will not be run, and these are the only tests in the tree that exercise
concurrent writes for corruption. Three fixes are each small: size the buffer from the test's own
`STRESS_N = 4094` rather than from `MAX_N`, cut the trial counts, or build the input once per
configuration instead of once per trial. What is not acceptable is the current state, where "123 tests
across 13 crates" reads as something anyone can run and a thirteenth of it is unreachable without a flag
nobody was told about.

**Test quality, on the surface I read.** I read the bodies in `satfold-shared`, `bitpack-shared`,
`bitpack-write-contend-shared/src/stress.rs` and skimmed `warm-container-shared`. They are not decorative.
`satfold-shared/src/lib.rs` checks eleven kernels against an oracle and then, in the same module, runs
**three deliberately wrong kernels** (`WrongOp`, `DropsALane`, `DropsTheRemainder`) against the same oracle
so that the agreement above it is not free, with a comment saying exactly that. `bitpack-shared` checks
both extraction paths against logical ground truth at every index and additionally asserts the permutation
is a bijection. This is the standard I tried to hold my own probes to, and it is higher than most suites I
have been handed. I found no tautology, no assertion-free smoke test, and no sampled law that struck me as
choosing what not to find out.

The three tests I could not run I could still read, and they hold the same standard: the corruption-rate
test is paired with `naive_kernel_never_corrupts_when_the_split_is_aligned` (`stress.rs:118`), which exists
so that a corruption count above zero is attributable to the boundary race rather than to a defect in the
pool or the harness, and its doc comment says exactly that. So the verdict that this suite is real extends
to all thirteen crates by reading. What does not extend is any claim that it passes: I ran 120 of the 123
to green and three did not complete.

---

## 1. What the question actually is

The dispatch asks how many strategies there are and what each one is, and says the important part is what
determines the answer. Those are three different questions and only one of them has an answer that is
anybody's to derive.

Start from the one thing I was given. A strategy is a pair: an assignment on observable policy axes, and a
weighting over cost coordinates. Take that seriously for a second, at the level of what the bytes do,
because the shape of the pair settles most of this before any taste enters.

The first component is answer-visible. That is what "observable" means and it is the word in the given
statement. Two numerals whose types differ only in that component can be told apart by running them: there
is an input on which they produce different bits. The second component is cost-visible. It selects among
ways of computing, and the ways of computing are ways of computing **the same thing**, or the word
"cost" in "cost coordinates" is doing no work.

So the pair is not two flavours of the same kind of thing. It is a **specification** and a **selector**,
and they live on opposite sides of the only boundary that matters in a numeric library: whether a change
can be seen in an answer.

That gives the count question three separate answers, and conflating them is why it feels unanswerable:

**How many policy assignments are there?** Determined, and enumerable, once the axis set is fixed. It is a
product. Nobody chooses it; it falls out.

**How many weightings are there?** A continuum, whose observable content is finite and **target-dependent**.
Nobody can write the number down in a document that survives a new machine.

**How many named presets should there be?** Not derivable from anything. It is an ergonomics decision about
vocabulary, and four is as defensible as three or nine, on grounds that have nothing to do with the
mathematics.

I13 is ratified and it says the work is predicated arms composed, with a universal solution rejected by
premise. Read that against the pair and it is the same statement: **the policy assignment is the predicate
and the weighting is what picks the arm inside it.** The strategy axis is not a fifth thing sitting beside
I13's mechanism. It is I13's mechanism, named.

---

## 2. The count is not a constant, and I measured it

The cleanest way to find out how many policy assignments a design actually has is not to argue about
axes. It is to enumerate candidate axes, evaluate every point over the whole input domain, and quotient by
observational equality. Two assignments that agree on every output of every operation are the same
strategy, whatever the labels say.

`p1_policy_classes.rs` does that over a small candidate product: overflow in {wrap, saturating}, rounding
in {truncate, nearest-even, floor}, intermediate in {exact, stepwise}. Twelve syntactic points, five
operations (add, subtract, multiply, a three-term multiply chain, and a multiply-add, the last two present
because the intermediate axis cannot bite on a single operation), the whole domain, and a thirteenth entry
that is a duplicate of the first reached by a different construction.

| shape | syntactic points | observationally distinct classes |
|---|---|---|
| W=6, F=0, unsigned | 12 | **2** |
| W=6, F=0, signed | 12 | **3** |
| W=6, F=3, unsigned | 12 | **8** |
| W=6, F=3, signed | 12 | **12** |
| W=8, F=0, signed | 12 | 3 (lower bound, strided) |
| W=8, F=4, signed | 12 | 12 (lower bound, strided) |
| W=8, F=4, unsigned | 12 | 8 (lower bound, strided) |

**I predicted 3 for the first row and got 2.** The two saturating intermediate values merged at unsigned
`F = 0`. The mechanism is that unsigned saturation clips on one side only, and one-sided clipping of a
monotone operation is a congruence, so reducing early and reducing late land in the same place. Signed
saturation clips both ends and multiplication by a negative reverses order, so it is not a congruence and
the two survive as separate classes. **This is exactly the sentence sitting in the workspace rule I
declared above, so it is corroboration and not discovery.** What is mine is that I predicted the wrong
number anyway, and that the class count is measured rather than argued.

The controls all held. The duplicate merged with its twin at every shape (so the comparator can merge).
Wrap and saturating separated at every shape (so it does not merge everything). And the mutation control:
replacing the comparator with always-equal gives 1 class and never-equal gives 13, at every shape, so the
number moves when the instrument is sabotaged and is therefore measuring the instrument's subject.

**The finding is the variation, not any one number.** The same twelve labels denote two strategies at one
shape and twelve at another. There is no shape-independent count, so "how many strategies are there" has no
answer of the form the question expects, and a design that fixes a number is fixing the number of *names*,
not the number of *things*.

```
holds for: numeral fixed-point of declared width W and fraction F,
           W in {6, 8}, F in {0, 3, 4}, signedness in {unsigned, signed},
           overflow in {wrap, saturating},
           rounding in {truncate, nearest-even, floor},
           intermediate in {exact, stepwise},
           operations {add, subtract, multiply, multiply-chain-3, multiply-add},
           arity in {2, 3}, chain length in {1, 2},
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

The W=8 rows carry a stride of 8 on the three-argument sweep, which makes them a lower bound: a witness of
difference on a subset is still a witness, but agreement on a subset is not agreement. I have listed them
separately for that reason and would not gate anything on them.

One consequence worth stating plainly, because it bites the naming question directly. **At unsigned
`F = 0`, six of the twelve labels are one strategy.** If a design ships four presets and two of them differ
only in rounding, then for every unsigned integer type in the library those two presets are the same type
wearing two names, indistinguishable by any test anyone can write. That is not necessarily wrong, but it is
a fact the naming has to be chosen in view of.

---

## 3. The two components are told apart by a test, and here it is

The split between the components is only useful if it is checkable. It is, and the check is the one that
should govern which component a candidate axis belongs to.

**Take the candidate, hold everything else fixed, vary it, and look for an input where the answer changes.
If one exists, it is policy. If none exists over the whole domain, it is weighting.** That is the entire
procedure, it is mechanical, and it is what `p1` and `p6` run.

`p6_packing_is_a_weighting.rs` applies it to the storage-minimising concern, which the brief asks about by
name. Three representations of the same declared type: packed at an arbitrary bit offset in a bitstream,
padded into a native slot, and packed one bit short as the control. Same declared policy for all three.
Sixty configurations, `W` in {3, 5, 6, 7, 11}, `F` in {0, 2}, bit offsets in {0, 1, 7, 13, 59, 61},
including offsets that straddle the 64-bit word boundary.

**Zero disagreements between packed and padded, in all sixty configurations.** The round trip held (so the
packed storage is not lossy and the agreement is between two working things), the results were non-vacuous
(up to 48642 of 49152 results nonzero, tens of thousands of saturating inputs), and the one-bit-short
control was detected at 60 to 62 percent of operations everywhere, so the comparator has teeth.

```
holds for: numeral fixed-point signed, W in {3, 5, 6, 7, 11}, F in {0, 2},
           overflow = saturating, rounding = truncate,
           bit offset in {0, 1, 7, 13, 59, 61} including word-straddling,
           operations {add, subtract, multiply}, arity = 2,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**So packing is answer-invisible, which makes the storage-minimising concern a weighting with zero policy
content.** It buys space and costs time and does not change what anything computes. That answers one third
of the brief's four-way question outright, and it has a consequence for I17: the storage concern is not
deprioritised by being a weighting. A weighting-only strategy is a full strategy. It selects arms, it is
what the design exists to let a consumer ask for, and the fact that it cannot be caught by a correctness
test is a property of it rather than a demotion.

---

## 4. The claim against the settled statement

Here is the part the brief told me to make visible.

**The pair as given does not constrain the second component, and it has to.** Nothing in "an assignment on
observable policy axes plus a weighting over cost coordinates" says the weighting may not move an answer.
If it may, then the answer a program computes depends on a cost model, the cost model reads the target, and
two builds of one program produce different results with no predicate anywhere naming the difference. Under
I10 arvo adapts to the cores it finds, so this is not hypothetical: a weighting that could move answers
would make results depend on core count.

So I am proposing a law that the pair must satisfy, and I will call it the **observability firewall**:

> The policy component determines the answer. The weighting selects only among computations that conform
> to the policy. Every difference in an answer traces to the policy, and nothing else may move one.

`p2_firewall.rs` part A tests the "may not" direction where it is easiest to break: two genuinely different
routes to one saturating fixed-point multiply, one widening and shifting, one accumulating four partial
products the way a machine without a wide multiplier would have to. **Zero disagreements over 1,376,256
input pairs across six shapes**, with a wrong-clamp control arm detected at up to 63576 of 65536 pairs, and
up to 65025 nonzero results per shape so the agreement is not vacuous.

```
holds for: numeral fixed-point, W in {8, 10}, F in {0, 3, 5, 7},
           signedness in {unsigned, signed}, overflow = saturating,
           rounding = truncate, operation = multiply, arity = 2,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

One honesty note on that table: at `W=8, F=7` only a single input pair saturates at all, and the
wrong-clamp control fires at three pairs out of 65536. The control technically passes there and it is
nearly toothless, which I would rather say than let a uniform-looking row imply uniform strength.

### The firewall immediately forbids something real, and that is the interesting part

The optimisation a backend most wants on this shape is fusing the multiply-add so the intermediate is never
rounded or reduced. Under the firewall that is legal only if it agrees. Part C of the same probe measures
whether it does, over the whole fraction axis at `W=6`, exhaustively, all 262144 triples per cell:

| | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| unsigned, wrapping | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| unsigned, saturating | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| signed, wrapping | 0.00% | 1.64% | 5.54% | 12.34% | 22.22% | 33.40% |
| signed, saturating | 42.14% | 39.10% | 35.67% | 31.96% | 29.52% | 33.41% |

**Fusing the multiply-add changes the answer at up to 42.14% of triples.** So on signed types fusion is not
a lowering choice at all. It is a different policy, and a design that lets a cost model reach for it has
already lost the property that a type's semantics are stated in its type.

**My prediction here was wrong twice.** I predicted nonzero differences at every `F > 0` and got zero across
the entire unsigned half. I also predicted nonzero at `F = 0` under saturating, which held for signed and
failed for unsigned.

**And my first run of this measurement was itself defective, which I am reporting rather than quietly
fixing.** The three-argument sweep used a stride of 4, which makes every sampled product a multiple of 16,
so at `F <= 4` the shift was exact, truncation never fired, and the arms agreed for a reason with nothing to
do with the question. It printed a clean 0.00% for signed wrapping at `F = 3` and I nearly believed it.
That is textbook setup-that-helps, in my own instrument, and the reason I caught it is that a neighbouring
row disagreed with it in a way the mechanism could not explain. The fix is in the committed probe: odd
strides, exhaustive where affordable, and a control that **counts sampled pairs whose shift is inexact and
fails when that is zero at `F > 0`**. If you take one thing from this file into your own probes, take that:
a sample must be shown to reach the case that can differ, and showing it costs four lines.

```
fusion is answer-preserving:
holds for: numeral fixed-point unsigned, W = 6, F in {0, 1, 2, 3, 4, 5},
           overflow in {wrap, saturating}, rounding = truncate,
           operation = multiply-add, arity = 3, chain length = 2,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)

fusion changes the answer:
holds for: numeral fixed-point signed, W = 6,
           (overflow = wrap and F in {1, 2, 3, 4, 5})
           or (overflow = saturating and F in {0, 1, 2, 3, 4, 5}),
           rounding = truncate, operation = multiply-add, arity = 3,
           chain length = 2, threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

### The way out, and what it costs, priced

The firewall as stated would ban fusion outright on signed types, which is too strong: it bans a real win
to protect a property. The repair is that **a policy specifies a set of acceptable answers rather than one**.
Then fusion is legal exactly where its result stays inside the declared set, the weighting picks freely
inside it, and the firewall survives with its content intact: nothing outside the *declaration* may move an
answer.

That is only worth having if the required slack is small, and that is a magnitude, so
`p3_declared_slack.rs` measures it. Exhaustive at `W=6`, all triples, the smallest slack that admits fusion
everywhere:

| | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| unsigned, wrapping | 0 | 0 | 0 | 0 | 0 | 0 |
| unsigned, saturating | 0 | 0 | 0 | 0 | 0 | 0 |
| signed, wrapping | 0 | **1** | **1** | **1** | **1** | **1** |
| signed, saturating | **32** | **32** | **32** | **32** | **32** | 1 |

Raw units, on a type whose whole range is 63 units.

**One unit in the last place buys fusion for signed wrapping types. Nothing useful buys it for signed
saturating ones**, where the required slack is 50.79% of the range, which is a policy that has specified
nothing. Both conformance controls held at every cell: the conforming arm was accepted at all 262144
triples, and an arm placed one unit outside the declared set was rejected at all 262144. Both directions
were needed; a checker that rejects everything reports a large slack for free and one that accepts
everything reports zero.

```
holds for: numeral fixed-point, W = 6, F in {0, 1, 2, 3, 4, 5},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           rounding = truncate, operation = multiply-add, arity = 3,
           chain length = 2, threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

This is a design result with a number attached, which is the shape I13 asks for. A strategy that wants both
saturation and fusion on signed types has to give one up, and the design can now say so exactly rather than
as a worry.

---

## 5. The weighting is a continuum whose observable content is finite

The brief asks whether the answer is a continuum, and says to name what the design calls things if it is.

A weighting is a vector over cost coordinates, so there are uncountably many. But a weighting is only ever
*used* to pick one arm from a finite set, and a linear objective over a finite point set has finitely many
distinct argmins. The weight space partitions into cells, one per arm that can ever win, and **two
weightings in the same cell are the same strategy in every way anything can detect.**

`p4_weight_cells.rs` measures this on two cost tables standing in for two targets, with seven arms over
three cost coordinates (time, code bytes, data bytes), swept at resolution 1/400 over the simplex.

**Seven arms, six Pareto-nondominated, five cells.** On both targets. All three controls held: the
strictly dominated arm won zero times, the deliberately narrow-band arm won (so the grid resolves it rather
than under-counting), and the duplicate arm won zero times under index-order tie-breaking so it cannot
inflate the count.

**I predicted the cell counts would differ between targets and they did not.** Both are five. The claim
that survives is about the mapping, so I measured that instead of asserting it: **the same weight vector
selects a different arm at 838 of 1891 grid points, 44.3%.** The "balanced" weighting picks the widened
scalar arm on one target and the compromise arm on the other. So the structure is target-dependent in the
way that matters even where the count coincides, and no document can record which arm a named weighting
resolves to.

**A second prediction of mine was half wrong and produced the more useful half.** I expected the winner
count to equal the Pareto count and said in the probe that I expected that to be wrong. It is: an arm can
be Pareto-nondominated and still unselectable by any linear weighting, because it sits above the line
joining two others. I built the minimal witness rather than leaving it as an expectation: three arms, two
coordinates, the compromise arm at (6,6) between endpoints at (0,10) and (10,0). It is dominated by
neither, and it wins **zero** of 2001 sweep points. The control matters here: pulling the same arm inside
the hull, to (4,4), makes it win 399 times, so the zero is a property of the arm and not of the sweep.

**That is a real limit on the weighting formulation and it should be recorded.** If the selector is linear,
some genuinely reasonable compromise arms are unreachable, and no weight vector a consumer can write will
select them. Whether that matters depends on whether such an arm ever turns out to be the one somebody
wants.

```
holds for: cost coordinates = 3 (time, code bytes, data bytes),
           arms = 7 with the committed cost tables, selector = linear,
           weight grid resolution = 1/400 on the 2-simplex,
           targets = 2 (the committed synthetic cost tables),
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

That predicate is narrow on purpose. **These are synthetic cost tables, not measurements.** The geometry is
real and the numbers in it are invented, so this establishes the shape of the answer and prices nothing.
Anything that depends on how many cells a real arm set has is **unpriced**, and the harness under
`mock/benches/` is where that would be answered.

Two further consequences fall out of the geometry and neither needs a measurement:

**The observable weighting count is bounded by the arm count**, so however continuous the weight space is,
the design's vocabulary for it need never exceed the number of arms.

**Cost coordinates are closed by a measurement obligation, not by taste.** A coordinate exists only if
every arm carries a value on it. Adding one obliges measuring every arm on it, so "should energy be a cost
coordinate" is a question about how much measurement someone will fund, not a question about the design.

---

## 6. The set can be open, and that is doable rather than hoped for

The canon has to be able to say whether a thing is doable, so I checked rather than argued.

`p5_open_set.rs` puts the two components in a consumer-extensible form under the constraints that are in
force (no `dyn`, no `TypeId`, no alloc, monomorphisation is the dispatch, no runtime validation), ships two
presets from a "library" half, and then has a separate module standing in for a separate crate define its
own point that the library never names: the library's overflow, a rounding the library has no preset for, a
slack the library never declared, and its own weighting.

It compiles with no change to the library half. The two arms agree on all 131072 inputs with 129773
nonzero results, so the weighting is not moving the answer here either. And the emitted code, scanned by
`p5_scan.sh` against the committed `p5.s`:

| monomorphisation | body | arm_fast | arm_small | conditional branches |
|---|---|---|---|---|
| `mul::<LibraryPreset>` (time-weighted) | 5 lines | 1 | 0 | **0** |
| `mul::<consumer::MyStrategy>` (space-weighted) | 5 lines | 0 | 1 | **0** |
| `mul_runtime_selected` (the control) | 9 lines | 1 | 1 | **1** |

Each monomorphisation is a `mov` and an unconditional tail branch to exactly one arm. The runtime-selected
control keeps both arms and a `tbz`, which is what makes the two rows above it mean something: the scan can
see a surviving arm, so its absence is a result rather than a limitation of the scan.

**My first version of that scan reported zero arms for all three functions and I nearly wrote it up as a
failure.** It was looking at the `extern "C"` entry points, which are thunks that tail-call the
monomorphised symbol. The arms were one hop further down. The corrected scan is committed and the note
about why is in it, because the failure mode (a scan that finds nothing and reads as evidence of absence)
is worth more to the next person than the fixed version alone.

```
holds for: strategy expressed as a trait with associated consts for the policy
           and the weighting, selection by const comparison, arms marked
           inline(never), consumer strategy declared outside the library module,
           opt-level = 3, no gates beyond the stable language,
           threads = 1,
           target = aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f
```

So **"the set is open" is doable**, it costs nothing at runtime, and it honours I15 in the strong reading:
one lowered path, no conditional branch, the unused arm gone. Note what the probe does **not** establish: it
uses one spelling, and per the spike rule that spelling is scaffolding rather than a design decision. What
transfers is that a consumer-supplied point resolves at compile time. How it is spelled is a design
question and not this file's to answer.

---

## 7. What follows, stated as suggestions

Op ratifies. These are what my derivation supports, offered as suggestions and not as rulings.

### The set is a space, and what a design names is presets

There is no finite strategy set to enumerate. There is a space, with a discrete first component whose
distinguishable size is a function of the numeral's shape, and a continuous second component whose
distinguishable size is a function of the target. A design names points in it, and a name is a convenience
rather than a boundary.

That composes with I1's demotion exactly. Op said the set is not closed at four and that the count is
beside the point. The derivation says something slightly stronger and in the same direction: **the count is
not a property of the design at all.** It is a property of the vocabulary, so asking how many strategies
there are is asking how many names to ship, which is an ergonomics question with an ergonomics answer.

### The axes are discovered; the presets are chosen

This is the answer to "what determines the answer", and it is the sentence I would most like carried
forward.

**An axis earns its place by a test, not by an argument.** Vary the candidate, hold everything else fixed,
sweep the domain, and see whether an answer moves. If one moves it is policy; if none moves over the whole
domain it is weighting; if it is neither, it is not an axis. That procedure is `p1` and `p6`, it is
mechanical, and it settles membership without anybody's taste.

**A preset earns its place by being recognisable to a consumer.** Nothing determines it. It is a name for
an intent, which is exactly what I2 already says presets are.

### What a strategy must not determine

- **Not the declared width.** Independent, and op's I3 settlement points the same way: the imitation is
  ergonomics, and where boundaries land is answered by the width and the overflow policy.
- **Not an answer its policy did not declare.** The firewall, section 4.
- **Nothing read at runtime.** I15, and p5 shows the const form costs nothing.
- **Not the representation.** Packing is answer-invisible (p6), so it belongs to the weighting, and a
  *policy* naming a storage layout has put a cost decision on the semantic side.
- **Not whether a type compiles on a given target.** Derived, not measured: since the policy is a
  specification over integers and the weighting only picks among conforming arms, a target that can compute
  the specification can host every strategy, and target features change which arm wins rather than whether
  one exists. I flag this as an argument. It would be established by building one policy's general arm and
  showing it needs nothing beyond the base instruction set.

### What the canon owes a consumer picking one

Two different kinds of knowledge, two different ways to decide, and an order between them.

**The policy is chosen from the consumer's semantics.** They know whether overflow is a bug in their
domain, whether a chain must be exact, whether a wrong answer is a wrong answer. The substrate cannot know
this and should not guess, which is `arvo-toolbox-not-policer.md` at the semantic layer.

**The weighting is chosen from the consumer's measurements.** Nobody else can take them, because they are
facts about a workload and a target, and section 5 shows the answer moves between targets at 44.3% of the
weight space.

**Pick the policy first.** A wrong policy is a wrong answer; a wrong weighting is a slow answer. The order
follows from the cost of being wrong, and it costs the canon nothing to say it.

And the default preset follows from the same asymmetry: **the default is the one whose policy is least
surprising, not the one whose weighting is most balanced.** Which is I3 and I4 restated in the derivation's
own terms, and I take it as support for op's position rather than as a new claim.

### The four concerns are a mix, and the mix is asymmetric

The brief asks whether they are four strategies, four axes, or a mix. None of them is an axis. They are
named directions in a space whose actual dimensions are the policy axes and the cost coordinates, and they
are not the same kind of thing as each other:

- **storage-minimising**: a weighting. Zero policy content, measured (p6).
- **speed-first**: a weighting, plus a policy. I5 says it may sacrifice soundness for a proven gain, and
  under the firewall that sacrifice has to be declared in the policy rather than taken by the cost model.
  So I5 reads as a licence for the design to *ship* a looser policy, not as a licence for the lowering to
  choose one.
- **accuracy-first**: principally a policy. I7's "especially within chains and ops, not only alone" is a
  statement about the intermediate axis, which p1 confirms is a real and separate axis that only chains can
  observe. Its weighting is a near-zero weight on time and is the least interesting part of it.
- **imitate-the-native**: neither, on op's own settlement that it is ergonomics. It is a criterion for
  which preset is the default and what it is called, and its policy content is inherited from whatever
  Rust's primitives do rather than chosen.

One consequence I want to name because it is easy to trip over. **Two presets that share a policy
assignment are semantically identical and can only be told apart by cost.** If the storage concern is
weighting-only, then a storage-minimising preset and any other preset with the same policy compute the same
answers on every input, and no test distinguishes them. That is fine and it is worth knowing, because a
reviewer who expects four presets to be four semantics will keep looking for a difference that is not
there.

### A consequence of op's I3 settlement that I think has not been drawn

If the imitation is ergonomics rather than semantics, then **a width cap on the imitating preset has no
basis**. A cap of the shape "this preset is unavailable above N bits because there is no native primitive
to imitate" only makes sense if the preset's *policy* refers to a native primitive. Under the settlement it
does not; it refers to the experience of using the type. So the imitating preset extends to every width the
library supports, and where Rust has no primitive at that width the imitation is vacuous rather than
violated. I flag this because it is a live consequence rather than a finding, and because
`arvo-toolbox-not-policer.md` independently says no width cap below the largest container the substrate
dispatches through.

---

## 8. Options I am opening, each with what would close it

Per the panel's rules, each states its decision procedure.

**O-139-A. Is a strategy a property of the value or of the computation site?**
Under the value reading, a mixed-strategy binary operation needs a meet function on strategies. Under the
site reading, the result's strategy comes from the context and each operand is converted, so no meet exists.
**This matters because the meet is not derivable.** I looked for one: policies that pin different answers
have disjoint specifications, so no result conforms to both operands, and whichever policy is declared the
winner is a convention rather than a consequence. Spec inclusion does not order wrapping against saturating
in either direction. So if the value reading is taken, the design owes an admittedly-arbitrary rule, and it
should say that it is arbitrary rather than dressing it as a lattice.
*Closes on*: writing the usage code for both and reading the call sites. Whether `let x: T<A> = a + b` with
`a: T<B>` is a conversion or a warned coercion is visible in five lines of consumer code, and the one that
reads like what the user is trying to do wins.

**O-139-B. Does a policy pin one answer or declare a set?**
Section 4 prices this: a one-ulp declaration buys fusion for signed wrapping types, and no useful
declaration buys it for signed saturating ones. Pinning is simpler and forecloses the win. Declaring costs a
slack field on every policy and a conformance obligation on every arm.
*Closes on*: whether anything downstream needs fused multiply-add on signed wrapping fixed-point badly
enough to pay for the mechanism. That is a bench question on `mock/benches/`, and the magnitude is currently
**unpriced**.

**O-139-C. Is the selector linear?**
Section 5 witnesses a Pareto-optimal arm no linear weighting can select. A non-linear selector (lexicographic
priority, a constraint plus an objective, an explicit arm preference) reaches it.
*Closes on*: finding one real arm in `mock/benches/variants/` that is Pareto-optimal and inside the convex
hull of its neighbours. If none exists in the arm sets the library actually has, linear is enough and the
limit is theoretical. That is a computation over committed bench artifacts and it is cheap.

**O-139-D. Is a preset a fixed assignment or a function of the shape?**
A fixed assignment means a preset denotes the same axis values at every width and fraction, and p1 shows
those values sometimes collapse to the same behaviour. A shape-indexed preset could pick nearest-even where
rounding is live and not care where it is dead, which is the same *intent* expressed differently per shape.
*Closes on*: whether any preset's stated intent requires different axis values at different shapes to remain
true to itself. The accuracy-first concern is where to look, since I7's chain clause may want a different
intermediate rule at `F = 0` than at `F > 0`.

**O-139-E. Are the cost coordinates named in the canon, or is the coordinate set itself open?**
Section 5's measurement obligation argues they are closed by cost rather than by principle: a coordinate is
only usable if every arm carries a value on it.
*Closes on*: whether the harness can carry a coordinate a consumer adds without every existing arm being
re-measured. If it cannot, the set is closed in practice whatever the design says.

---

## 9. Coverage, bounds, and what I did not do

**What I carried forward unchanged, and from whom: nothing, and that is the protocol rather than a
judgement.** This is a cold derivation. I have read no member's file and therefore cannot have kept or
discarded anybody's position. The count is zero by construction and phase two is where it stops being zero.

**Everything here is `threads = 1`.** Every probe runs on one thread and none of them touches concurrency,
so under the panel's own notation these findings do not hold anywhere threads exist. That is the strong
reading and it is the intended one.

**Every measurement is at model widths.** `W` in {3, 5, 6, 7, 8, 10, 11}. I have no transfer argument to
64 bits and I am not offering one. The domain of an exhaustive check grows as `2^(W*k)` for arity `k`, so
the three-argument sweeps were already at the affordable edge at `W = 6`.

**The cost tables in section 5 are synthetic.** They establish geometry and price nothing. Anything in this
file that depends on how much something costs is unpriced, and I have used that word rather than reaching
for a number.

**I did not measure anything on the bench harness.** No claim here is a bench result and none is called one.

**I did not read a single panel file, so I do not know what has been settled**, which means any of this may
duplicate work or contradict a converged result. That is what phase two is for and it is the cost of the
protocol rather than a defect in it.

**I ran 120 of the 123 tests to green.** The three stress tests in `bitpack-write-contend-shared` did not
complete in a debug build, so nothing here rests on them passing. I read them and they are well built; that
is a statement about their construction and not about their result.

**Where I would want a second pair of eyes first.** Section 4's firewall is the load-bearing claim in this
file and it is a *proposal*, not a measurement. The measurements around it are solid; the claim that the
pair must satisfy it is an argument, and it is the kind of argument that sounds obviously right and might be
wrong in a way I cannot see from inside it. If one thing here gets attacked, it should be that.

---

## Appendix: the thirteenth crate

`cargo test --release --manifest-path bitpack-write-contend-shared/Cargo.toml`, run in the background while
this file was written. The result is recorded in the commit that follows this one, so that the number in
this file is one somebody can check rather than one I remembered.

---

# Phase two: reconciliation against Q51

Written after committing everything above and reading `OPTIONS.md` entry Q51 and nothing else. Phase one
is untouched. What follows is where I agree, where I do not, and what I got wrong.

## Correction to my own gate report, before anything about Q51

**The gate section above names the wrong cause and I am leaving it standing with this correction beside
it**, because the record of a wrong diagnosis and the evidence that closed it is worth more than a clean
page. Phase one is not rewritten.

I wrote that `bitpack-write-contend-shared` does not complete because 4500 trials each rebuild a 23.6 MB
buffer, "on the order of a hundred gigabytes of allocation and fill". **That is wrong.** I attacked it
rather than shipping it, and `p7_gate_diagnostic.rs`, an ad-hoc quick spike with no substance and named as
such, times the buffer directly:

```
per-build as-is      : 723.972us
per-build right-sized:   5.435us
projected cost of the 4500 builds the three stress tests perform: 3.3 s
```

The buffer is oversized by 1024x and it accounts for **3.3 seconds of a sixty-minute run**. My control
technically passed, because right-sizing is 133x cheaper, and the projection refuted the hypothesis anyway.
A control that only asks "is A cheaper than B" cannot tell you whether A was ever the term that mattered,
and that is a lesson about my own instrument rather than about the crate.

**The real mechanism is a livelock, and it is in `pool.rs`.** The worker pool is process-global, sized on
first use, and its workers never exit: `pool.rs:87-122` is an unbounded `loop` spinning on a `generation`
counter. `write_pass` (`pool.rs:143-160`) drives one pass by storing the arguments, zeroing `done`, bumping
`generation`, and then spinning until `done` reaches `threads - 1`. That protocol has exactly one
coordinator in it.

`cargo test` runs tests concurrently by default, in one process. All three stress tests call
`pool(STRESS_THREADS)` and drive the same pool at the same time, so a second coordinator's
`p.done.store(0)` resets a counter the first is waiting on and two `generation` bumps coalesce into one
wakeup. The first coordinator then spins forever in `while p.done.load(..) != threads - 1`. The crate's own
comment predicts this without quite naming it: `pool()` asserts on the thread count with the message that
"the harness runs one thread count per worker process; a second count in the same process means that
contract changed". The pool was built for the bench harness, where a process runs one configuration, and
`cargo test` is not that.

**The decisive experiment, and it takes one flag:**

```
cargo test --manifest-path bitpack-write-contend-shared/Cargo.toml -- --test-threads=1
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.97s
```

**Seven point nine seven seconds, in a debug build, for the suite I had twice killed after an hour.** So
the crate is not slow, it is not badly written, and its stress tests are fine. It is serialisation-required
and nothing says so.

**The corrected gate result: all 123 tests pass, 120 under the briefed invocation and 15 more once one
crate is given `--test-threads=1`.** The finding that stands is that the requirement is undocumented and
presents as an unbounded hang, which is the worst possible failure shape: a reader who follows the brief
concludes the suite is broken or that their machine is. **The brief itself is incomplete on this point.**
It says the suite runs per crate by `--manifest-path` or `-p` and warns that the workspace-wide form
returns a false green; it does not say that one of the thirteen additionally requires
`-- --test-threads=1`. Every future member running this gate as briefed will hit it, and the cheapest fix
is one clause in the brief. The next cheapest is a serialising lock in the crate, which would make the
constraint self-enforcing rather than folklore.

I spent roughly two hours of wall time on this, most of it waiting for a run that was never going to
finish, and the whole thing would have been one flag away had the requirement been written down anywhere.


## The one correction that reaches furthest, and it is against me

Q51 says **observability is a property of the chain, not of the axis**, and reports that the definition in
play gave 0% against 89.081% depending on whether the limit is read at **the declared width or the
container width**.

**My entire corpus is `container width = declared width`, and I never noticed.** Every probe I wrote
reduces to the declared width after every operation. There is no container in my model at all: `p1`, `p2`,
`p3` and `p6` all clamp or wrap to the declared range at each reduction point, and the intermediate is
either exact-in-`i128` or reduced to the declared width, with nothing in between. The case arvo exists for,
a 13-bit declared value living in a wider container where the extra bits change where a boundary lands, is
absent from every measurement above.

So **every predicate in phase one is missing a dimension**, and under this panel's own notation that means
those findings do not hold anywhere a container wider than the declared width is present. Which is nearly
everywhere in the real library. I am not editing the predicates, because a predicate is not widened or
narrowed in place; I am stating the correction here, in my own file, which is where the rules put it:

```
every finding in phase one additionally requires:
           container width = declared width W
```

That is a large narrowing and it is the right one. The 0% against 89.081% figure Q51 quotes is exactly the
magnitude of the gap I did not model, so this is not a technicality.

**And the framing correction lands too.** Section 3 of phase one states the membership procedure as a
per-axis test: vary the candidate, hold everything else, sweep for an answer change. Q51 says observability
belongs to the chain. My probes half-obey that already, since `p1`'s intermediate axis is invisible on a
single operation and only shows up because I included a three-term chain and a multiply-add. But I wrote
the *procedure* as per-axis, and that is the wrong granularity, and my own data was already telling me so:
an axis with no effect at chain length 1 and a large effect at chain length 2 is not a property of the axis.
**Conceded.** The procedure should be stated over chains, with chain length a dimension of the sweep rather
than an incidental choice of mine.

## Where the unit and I converged independently, and where the two repairs differ

Q51's furthest-reaching repair is that **component one fixes the denoted answer, not the computed one**,
because with the computed answer fixed a fidelity column measures a constant and op's accuracy intent is
expressible in neither component.

I arrived at the same *problem* from the opposite direction and produced a different *repair*. My section 4
found that a policy pinning one computed answer forbids fusion outright, which is too strong, and proposed
that **a policy specifies a set of acceptable answers rather than one**. Same defect, same diagnosis that
the pair as first written is too tight on component one, two different loosenings:

- **Q51's**: component one fixes the ideal, and arms differ in fidelity to it, so fidelity is a **cost
  coordinate that can be weighed**.
- **Mine**: component one declares a set, and arms must land inside it, so fidelity is a **bound that must
  be satisfied**.

**These are compatible and I think the composition is strictly better than either alone.** A policy
declares a bound; inside the bound, fidelity is a coordinate the weighting may trade against time and
space. That keeps the property my firewall exists for, which is that nothing outside the declaration can
move an answer, and it restores the expressiveness Q51's repair exists for, which is that an accuracy-first
intent has somewhere to live. Under Q51's repair alone the weighting can trade accuracy for speed with no
stated floor, which is exactly the case where a program's results depend on a cost model and therefore on
the target, and I10 makes that concrete rather than hypothetical.

**And my `p3` supplies the number that makes the bound operational rather than a slogan.** The bound is
measurable per shape, exhaustively, and it is small enough to be worth having in the region where it is
small: 0 raw units unsigned at every `F`, 1 raw unit for signed wrapping at `F >= 1`, and 32 of 63 for
signed saturating, which is a bound that has declared nothing. A design taking the composition can state
that floor exactly instead of gesturing at it.

## A question Q51's repair raises that I cannot answer from Q51 alone

**If component one fixes the denoted answer, what separates wrapping from saturating?**

Both denote something other than the exact result, deliberately, and neither is an approximation of the
other. Two readings and both have a cost:

- If "denoted" means the mathematically exact result, then the overflow axis is not in component one, which
  contradicts it being an observable policy axis.
- If "denoted" means the result the type specifies, then it is what I called the pinned computed answer, and
  the fidelity column goes constant again, which is the defect the repair was made to fix.

My measurement says whatever "denoted" means, it has to be finer than exact: `p1`'s control C2 held at all
seven shapes, so wrapping and saturating never merged into one class anywhere I looked, including the shapes
where six of the twelve labels did merge. So the separation is real and the definition has to deliver it.

**The set formulation delivers both without the ambiguity.** Component one is a set of acceptable realised
values, per operation, per input. Wrapping and saturating are different singletons, so they separate.
A policy with declared slack is a wider set, so fidelity varies inside it and an accuracy coordinate is
non-constant. That is the property Q51's repair was reaching for, obtained without needing "denoted" to
carry two jobs.

I am not asserting this beats the unit's converged statement, which I have not read; `108` section 7 is
where that statement lives and I have read only Q51's summary of it. **It is offered as the thing to check
first, and if `108` already carries it, then this paragraph is one more instance of the same reading rather
than a correction, which is worth as much.**

## What I carried forward unchanged, with a count

**One: the pair.** Q51 records that the two-component object survived being attacked, and my derivation
assumed it and found nothing that unseats it. Six probes, none of which produced a candidate for a third
component. What I found instead is that the pair needs a **law** relating its components rather than a
third member, which is a claim about the pair and not against it.

**Zero positions carried from any member file**, because I have read none. That is the protocol working
rather than a contribution.

## What I am not touching

Q51's rung corrections, the union-on-supports result, and the finding about `88` having five incompatible
readings are all material I have not reached. The supports-and-rates result in particular describes a
structure on component two that my `p4` never encountered, because combining two weightings never arose in
what I measured. I have no view and am not manufacturing one.

## The six probes, offered to whoever picks this up

Each proves one thing once, and none of them decides anything on its own.

1. `p1_policy_classes.rs`: the number of observationally distinct policy assignments is 2, 3, 8 or 12 out
   of the same 12 syntactic points, as a function of shape alone. Mutation-controlled.
2. `p2_firewall.rs`: two routes to one policy agree over 1,376,256 pairs; fusing the multiply-add changes
   the answer at up to 42.14% of triples, with the full fraction-axis table. Carries the record of my own
   setup-that-helps defect and the control that now catches it.
3. `p3_declared_slack.rs`: the slack that admits fusion, exhaustively, per shape. 0, 1, or 32 raw units of
   63.
4. `p4_weight_cells.rs`: the weighting continuum quotients to five cells over seven arms; the same weight
   vector picks a different arm at 44.3% of the simplex across two cost tables; a Pareto-optimal arm that
   no linear weighting can select, with the control that proves the zero is the arm's and not the sweep's.
5. `p5_open_set.rs` and `p5_scan.sh`: a consumer-defined strategy compiling against an unchanged library
   and lowering to one unconditional branch, with the runtime-selected control that keeps both arms.
6. `p6_packing_is_a_weighting.rs`: packing is answer-invisible across 60 configurations, so the
   storage-minimising concern has zero policy content.

The three predictions of mine that fell, which is the part worth reading: the unsigned `F = 0` class count
(3 predicted, 2 measured), the fusion difference rate for unsigned (nonzero predicted, exactly zero
everywhere), and the weight-cell counts across targets (different predicted, identical measured, with the
mapping difference being the claim that survived).

