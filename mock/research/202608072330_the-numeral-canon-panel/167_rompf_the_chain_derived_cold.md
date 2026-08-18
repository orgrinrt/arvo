# 167. The chain, derived cold

**Member:** Tiark Rompf persona. **Unit:** the chain topic, opened by `166`.

**Phase one: written blind.** Premises only, per the cold-derivation protocol in `RULES.md`. No numbered
panel file, no register, no other member's probe, no commit log, no commit subject was read before this
file was committed. What I did read is listed in section 0.3 and includes one item the blind list did not
anticipate and which leaks panel conclusions; I name it there rather than let it pass.

---

## 0. The two gates, and the coverage bound

### 0.1 The canon gate: PASSED

Checked against `INTENTS.md` read in full, including its "How to read an entry" section, and against
`RULES.md` read in full.

The question is licensed and the licence is direct. **I7** is op's, it is STATED, and its wording ranges
over compositions rather than over single operations:

> Precise on other hand is the one that sacrifices as much performance and efficiency as makes sense, to
> be the most precise possible answer, throwing out all cold or hot axis optimisations to be *accurate*
> and *precise*, especially within chains and ops, not only alone.

`mock/canon/` does not exist, nothing is ratified except I13, and I13 is about predicated arms rather
than about compositions. So there is no ratified text this unit could misalign with, and the unit is not
building on a ratified state that would have to be re-derived.

**A second intent bears on this question and the brief did not name it.** I11:

> our main selling point are the algo crates that hilavitkutin, vehje, pretty much every single repo and
> project I have, downstream, use. As well as the contracts for things that compose to bigger units than
> just numerals alone.

"The contracts for things that compose to bigger units than just numerals alone" is a statement about
composition contracts being the point of the library, and it is op's own. It is at least as load-bearing
for this unit as I7 is, and reading the unit as an I7-only unit understates what op has said about it.
I take both as premises below.

### 0.2 The test gate: PASSED, and it reconciles two figures the record disagreed on

Run crate by crate at `--release` per the brief. Commands and raw output in `167_probes/gate/`.

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
| wide-rung-shared | 30 | ok |
| **subtotal, twelve crates** | **108** | ok |
| bitpack-write-contend-shared, `--test-threads=1` | 15 | ok, 2.25s |
| **total, thirteen crates** | **123** | ok |

`holds for: profile = release, threads = 1 for bitpack-write-contend-shared and default for the other
twelve, host = this machine, toolchain = the committed pin`

**This reconciles two counts that have been treated as competing.** 108 across twelve and 123 across
thirteen are both correct and they are not the same measurement: 108 is the twelve crates that run
unserialised, and 123 is all thirteen with the write-contention crate given `--test-threads=1`. The
thirteenth **does terminate** when serialised, in 2.25 seconds at `--release`, so a record saying it does
not is a record of an unserialised run rather than a property of the crate. I did not touch that crate.

Four other variant crates are reported to fail to build on a pre-existing cause. That is outside the
thirteen and I did not investigate it.

**Read rather than counted.** `satfold-shared`'s eleven bodies in full;
`bitpack-shared`'s three and their `check_size` helper; `wide-rung-shared`'s `per_width!` macro. I
scanned every `#[test]` in all thirteen crates mechanically for the tautology shapes: eighteen bodies
contain no `assert` or `panic` token, and every one of them delegates to a helper or a macro that does
assert; I opened four of the eighteen and confirmed this rather than inferring it.

**The suite is not decorative, and `satfold-shared`'s is the strongest I have read in this workspace.**
It carries four deliberately-wrong kernels as negative controls (`WrongOp`, `DropsALane`,
`DropsTheRemainder`, `DropsOneElement`), it asserts each defect exactly where that defect is
*expressible* and skips it where asserting would assert something false, it pins the instrument's own
sensitivity boundary as a two-sided assertion rather than deleting the case that failed, and it checks
the workload is non-degenerate with a range that can fail. `satfold-shared/src/lib.rs`'s
`saturating_addition_is_associative_at_eight_bits` closes the law over its whole domain,
`assert_eq!(total, 1 << 24)`, and its companion proves the false gate is genuinely false. That pair is
directly load-bearing for this unit and I use it in section 5.

### 0.3 Coverage bound, and one leak the blind list did not anticipate

**Read in full:** `INTENTS.md`, `RULES.md`, `mock/Cargo.toml`, `rust-toolchain.toml`, the repository's
`.claude/` rules and the workspace rules that load automatically, `satfold-shared/src/lib.rs`.

**Grepped or skimmed:** `mock/benches/bench.toml`, the variant directory listing, the thirteen shared
crates' test bodies.

**Not opened:** any numbered panel file, any register, any other member's probes, the git log, the
committed CSV rows, `mock/crates`.

**The leak.** The brief permits reading `mock/benches/` including the variant crates. `satfold-shared`'s
module documentation **cites panel files by number and quotes their conclusions**: it names `80` section
5.3 and `82` section 9, reproduces their instructions-per-element figures, and says which arm lost and
why. So a member told to read the bench crates and not the panel has been handed a panel conclusion
anyway. I read it before I understood what it was, and I cannot unread it.

What it contaminated, named precisely so the discount is applied to the right thing: my section 5 uses
the *existence* of a licensed reassociation and its per-operator asymmetry, and `satfold-shared` told me
that a prior file had studied it. It did not tell me the definition of a chain, the observation-boundary
argument, the residual argument, or anything in sections 1 through 4, all of which I derived before
opening that file. **The honest handling is to treat section 5's framing as contaminated and to hold the
rest at full cold rung**, and to say that the blind list needs a line about bench crate documentation,
because this is a general channel rather than one file's accident.

**Which sections move if something I leaned on is wrong.** Sections 1 through 4 rest on op's own words
and on my own probes and would move only if I misread I7 or I11. Section 5 rests additionally on
`satfold-shared`'s committed law tests, which I re-derived independently in `167_probes/assoc/` rather
than citing. Section 7's fork rests on I15's "never any runtime checks, ever" and would collapse if that
sentence admits a reading I have not found.


---

## 1. The answer, stated once before it is argued

**A chain is not a syntactic object and "chain" is not the right unit.** The right unit is the
**unobserved region**: a maximal stretch of a computation in which no intermediate is named by anyone
outside it. Its boundary is the act of observation rather than the operator, everything inside it is
arvo's to choose, and everything at its edge is the consumer's contract. A chain of three multiplies
whose middle value gets stored in a column is two regions, not one, and two multiplies separated by a
`let` binding nobody reads afterwards are one region, not two.

**What a composition owes that a single operation does not is three obligations, and none of them is the
composition of the step obligations.**

1. **An endpoint contract that its steps cannot add up to.** Strengthening every step to "correctly
   rounded" does not make the chain correctly rounded, and probe D shows there is no intermediate width
   short of exact at which it does.
2. **A choice of association**, which a single operation does not have, whose licence is per operator and
   per width, and which probe E derives exhaustively as a four-against-eight partition.
3. **A budget**, which is a global resource. The bits a chain needs are not the sum of the bits its
   operations need, and probe C counts the gap at 50.9% to 72.7% on ordinary chains.

**And one asset, which a per-operation surface structurally destroys: the residual.** Probe A shows that
carrying the bits an operation's output type has no room for turns an error that grows linearly in the
chain length into one bounded below a single LSB, and that the carried form is exactly equal to
accumulating in the wide type, which is a theorem rather than a coincidence.

**The consequence for locus.** Each of those four is a fact about a region and none of them can be
attached to a value, because a value does not know what will be done to it next. So a design whose only
compositional surface is one operation at a time cannot hold I7's guarantee, however good its operations
are. That is the finding, and it is a locus finding rather than a mechanism one.

---

## 2. Where the binding-time boundary is, and where it has been drawn

The first question about any computation is which part is known now and which part is deferred to the
program that runs. For arvo the answer is unusually clean, and it is clean because of I14 and I15 rather
than by accident.

**Known at compile time:** every operator in the expression, the shape of the DAG, every declared width,
every strategy marker, every association as written, the length of any fold whose count is const, and
therefore every derived quantity over those. I14 puts sizes at const and makes monomorphisation the
dispatch; I15 says everything reaches one lowered path and there is never a runtime check. Between them
the entire structure of a computation is a compile-time object.

**Deferred:** the values, and nothing else.

That is a two-stage program in the strict sense. The structure is stage one and the values are stage two,
and the job of stage one is to emit the code that stage two runs.

**The boundary a per-operation surface draws is in the wrong place**, and it is worth being exact about
where. It draws it at the operation: each operation is separately a small stage-one object that emits its
own lowering. Everything *between* operations then falls out of stage one entirely, because no stage-one
object spans two operations. The consequence is not that the chain is slow. It is that the chain **is not
represented at all**, so every fact about it is recomputed, conservatively, at each step, from the only
thing a step can see, which is its operands' declared types.

That is a binding-time error in the exact sense: information available earlier is being reconstructed
later and worse. And it has the signature such errors always have, which is that the reconstruction is
sound and lossy at the same time, so nothing ever fails and the cost is invisible.

**Two clarifications, because both are ways this argument gets misread.**

It is not an argument that arvo should be lazy or that anything should be deferred to run time. The
opposite: the whole point is that the chain is available *earlier* than the design currently uses it.

And it is not an argument against per-operation lowering. Some of I13's arms genuinely are per operation:
a width-specific lowering of one multiply is an arm, its predicate is over that operation's own
typestate, and nothing above it is needed. The claim is narrower and it is this: **some arms are not per
operation, and under a per-operation-only surface those arms have no site to be applied at.** An arm is a
rewrite of an expression. Where there is only ever one operation in view, there is exactly one thing to do
with it, and the whole reassociation family, the whole width-narrowing family and the whole
residual-carrying family are unreachable, not because they were rejected but because there is nothing for
them to be predicates over.

I13 is the one RATIFIED entry. I read this as I13 and I7 wanting the same mechanism, and that is the
single most load-bearing sentence in this file.

---

## 3. What is carried along a chain, and what is discarded at each step

Four things are computed at every step of a chain and thrown away by an operation whose output type has
room only for a value.

**The residual.** A Q(.F) multiply produces 2F fraction bits and returns F of them. The other F bits
existed, were correct, and are destroyed. Probe A measures what that costs over a chain and what
recovering it buys.

**The exactness bit.** Whether this particular step rounded at all. A chain in which no step rounded is
exact, and nothing per operation records that it happened.

**The achieved range.** The declared type of a result says what values it could hold. The step knows a
much tighter bound, from its operands' achieved ranges rather than their declared ones. That knowledge is
regenerated from the declared type at the next step, which is where the width over-provisioning in probe
C comes from.

**The correlation.** Two intermediates derived from a common ancestor have errors that are not
independent and can cancel exactly. `(a - b) + b` is `a`. Per operation there is no `a` to notice.

Probe A is the measurement of the first of these, and it is the one that admits a clean number.

### 3.1 What the residual is worth, measured

`167_probes/residual/`. A fixed-point multiply-accumulate at `F = 12`, error in LSBs of the Q(.12)
result, worst of 32 seeds:

| n | naive, floor per step | naive_round, nearest per step | widened | comp, residual carried |
|---|---|---|---|---|
| 16 | 10.94 | 2.87 | 0.998 | 0.998 |
| 256 | 136.2 | 8.43 | 0.995 | 0.995 |
| 4096 | 2084.7 | 37.95 | 0.989 | 0.989 |
| 65536 | 32831.3 | 231.6 | 0.984 | 0.984 |
| 1048576 | 524046.5 | 1418.2 | 0.929 | 0.929 |

Three readings, and the third is the design one.

**Per-operation truncation makes the error linear in the chain length.** 524046 at n = 1048576 is n/2,
which is the floor bias accumulating unopposed.

**The best a per-operation design can do still grows.** `naive_round` is round-to-nearest at every step,
which is the strongest per-operation accuracy contract there is, and its error grows as the square root
of the chain length. It is better by a large factor and it is the same shape of answer: an error that
depends on how long the chain is.

**The two arms whose error does not grow at all are both chain-level constructions**, and one of them
needs no wide accumulator. `comp` equals `widened` on every row because `acc * 2^F + carry_n = sum(p_i)`
exactly, so the carried residual reconstructs the wide accumulation. It is one extra F-bit register
against a doubled accumulator type.

Negative controls all clean: at `F = 0` every arm is exact (nothing to discard), on a workload where
every product is exactly representable `naive` is exact (the workload is not rigged), and `fake_comp`,
which computes the residual and discards it, equals `naive` bit for bit at all eleven sizes (the
advantage is the feeding forward and not the computing).

`holds for: F = 12, I = 3 including sign, n in {1 .. 2^20}, operands uniform in [-4,4), signedness =
signed, rounding in {floor, nearest}, threads = 1, profile = rustc -O`

### 3.2 The probe that failed first, and what the failure was worth

The companion probe was built to show a region where the one-rounding guarantee is reachable by carrying
the residual and unreachable by widening the accumulator at a fixed container width. **Its first version
found zero such geometries at any size tested**, and the failure is more informative than the fix.

It drew operands from `[-4, 4)`. On decorrelated signed data the accumulated sum is a random walk growing
as the square root of `n`, so the worst-case accumulator width `I + 2F + log2(n)` is never approached and
nothing overflows, however large the geometry. `carrier_bound_v1_FAILED.out` is that run, kept.

**The finding inside the failure: worst-case accumulator width is a fact about correlation between the
terms, not about the chain's length or its declared widths.** A chain sized for the worst case pays up to
`log2(n)/2` bits more than the realised behaviour needs, and nothing in a declared type distinguishes the
two cases. That is a chain-level fact of a third kind, and it is one arvo genuinely cannot derive: it is a
property of the data. What arvo can do is let the region be declared, and refuse to pretend it knows.

With non-cancelling operands the boundary appears exactly where the arithmetic predicts, at a 64-bit
container:

| F | n | widened bits needed | comp bits needed | outcome |
|---|---|---|---|---|
| 20 | 262144 | 61 | 41 | both exact |
| 24 | 16384 | 65 | 41 | **only the carried form is exact** |
| 26 | 262144 | 73 | 47 | **only the carried form is exact** |

Five such geometries, with the control (a geometry where both fit must agree, and does) clean.

`holds for: container = 64 bits, I = 3 including sign, F in {8,16,20,24,26}, n in {2^10, 2^14, 2^18},
operands non-cancelling, rounding = floor, signedness = signed, threads = 1`

---

## 4. The three obligations, each with the evidence that it does not reduce to the steps

### 4.1 The endpoint contract does not compose, at any intermediate width short of exact

`167_probes/doubleround/`. Take the strongest per-operation accuracy contract there is: every operation
returns the correctly rounded result, nearest, ties to even. A Q(.F) product is exact at 2F fraction bits.
A design that stores the intermediate at `M` fraction bits rounds twice; one that keeps the exact product
rounds once. Exhaustive over every operand pair:

| F | M = F+1 | F+2 | F+3 | ... | 2F-1 | 2F |
|---|---|---|---|---|---|---|
| 6 | 832 | 480 | 224 | 96 | 32 | **0** |
| 8 | 15360 | 8064 | 3968 | 1920, 896, 384 | 128 | **0** |
| 10 | 257024 | 130560 | 65024 | 32256 ... 1536 | 512 | **0** |

**There is no `M` strictly between `F` and `2F` with zero disagreements, at any `F` tested.** Each extra
intermediate bit roughly halves the count and it reaches zero only at exactness.

The floating-point literature has a threshold theorem here: double rounding is innocuous once the
intermediate carries enough bits, and the usual figure is `2p + 2`. **The fixed-point analogue has no
threshold below exactness**, and it is worth being explicit that this is not the float theorem failing.
The float theorem quantifies over values that are *sums or products of two p-bit operands*, which is a
constrained set. Here the operand set is already the full one and the exact product already needs exactly
2F bits, so the theorem's slack has nowhere to live.

**What this settles.** A chain-level accuracy guarantee cannot be bought by strengthening the
per-operation guarantee. It is bought only by not rounding the intermediate at all, which means either an
exact intermediate or a carried residual, and both of those are decisions about the region rather than
about the operation.

The `M = F` column is zero because the first rounding is then the identity, which makes it a third
control rather than a finding. Both declared controls (agreement at `M = 2F`, disagreement somewhere
inside) are clean at every `F`.

`holds for: F in {6, 8, 10}, M in [F, 2F], rounding = nearest-ties-to-even, operation = fixed-point
multiply, unsigned, threads = 1`

### 4.2 The association choice exists only above the operation, and its licence is per operator

`167_probes/assoc/`. Derived independently and exhaustively over the whole domain at `W = 4, 6, 8`.

**Associative at every width tested, zero disagreements:** wrapping add, saturating add unsigned,
wrapping mul, saturating mul unsigned, min, max, bitwise or, bitwise xor.

**Not associative:** saturating sub unsigned (827,484 ppm at W = 8), **fixed multiply with truncation**
(125,033 ppm), **fixed multiply with round-to-nearest** (124,744 ppm), average with floor (984,451 ppm).

Both controls clean: eight against four, so the column carries information; and a deliberately broken
reference comparing the left-associated form to itself reports zero disagreements across all twelve
operators, which is what shows the real comparison compares two distinct expressions.

**Two things in that table are worth stating separately.**

**Fixed multiply is not associative and rounding does not rescue it.** At W = 4 the rounding arm is
slightly worse (958 against 878) and at W = 8 slightly better (2,092,854 against 2,097,706), the same
order either way. So per-operation accuracy and chain-level algebraic licence are **independent axes**,
and spending on the first buys nothing on the second. A design that reasons "we round better, therefore
our chains are better behaved" is reasoning about the wrong axis.

**Saturating multiply is associative**, which I did not expect and which matters because it means the
non-associativity of saturating subtraction is not a general property of clamping. The clamp composes on
one side of the operator set and not the other, so the licence has to be derived per operator rather than
inferred from a family.

`holds for: W in {4, 6, 8}, unsigned, F = W/2 for the fixed-multiply rows, arity 3, threads = 1, the
twelve operators listed`

### 4.3 The budget is global, and the forward rule over-provisions by half

`167_probes/widths/`. A per-operation typing rule assigns an intermediate the width its operands imply,
because at the moment it is typed nothing downstream has been seen. A region-level rule may also
propagate the consumer's demand backward and take the smaller.

First, whether the backward propagation is even sound, derived rather than assumed. `167_probes/backward/`
sweeps a three-step chain exhaustively over 16,777,216 triples per operator, working width 12, consumer
keeping the low 6 bits:

**Licensed, zero disagreements:** wrapping add, wrapping sub, wrapping mul, bitwise and.
**Not licensed:** right shift then add (14,680,064), division (8,128), **saturating add at the working
width (2,476,720)**, min (10,812,862).

The saturating row is the one a strategy axis has to care about. A chain of wrapping additions may be
evaluated entirely at the consumer's width; the same chain of saturating additions may not, because
saturation is not a congruence modulo 2^K and the clamp depends on bits the narrowed arm has thrown away.
**The chain-level rewrite available to one strategy is unavailable to another over the identical
expression**, which is I9 as an arithmetic fact rather than as a slogan.

Then the count, using exactly that partition:

| chain | forward bits | forward and backward | saved |
|---|---|---|---|
| MAC x4, 16-bit inputs, 16-bit sink | 228 | 112 | 116 (50.9%) |
| the same chain, consumer keeps everything | 228 | 228 | **0** |
| Horner degree 4, 12-bit, 12-bit sink | 352 | 96 | 256 (72.7%) |
| Horner degree 4 with one right shift in the middle | 365 | 158 | 207 (56.7%) |
| all-blocking chain, div and min, 8-bit sink | 72 | 56 | 16 (22.2%) |

**This is a bit count and not a cost.** No timing was taken and none is claimed; what a saved bit is worth
in cycles or in bytes is **unpriced** by me.

Two sub-findings the table makes visible and which I did not anticipate when I built it. Inserting a
single right shift into the Horner chain drops the saving from 72.7% to 56.7%: **one non-congruence
operator anywhere truncates the region the backward rule reaches**, so the licence is a property of the
whole path rather than of the sink. And in the all-blocking chain the entire 16-bit saving is the sink
node alone, with nothing propagating: the sink always narrows, and what the operator partition governs is
whether anything above it does.

Both controls clean: the chain whose consumer keeps everything saves exactly zero, and the set contains
both a chain that saves nothing and chains that save.

`holds for: the five chain shapes and width rules stated in the source, threads = 1`

---

## 5. "Chain" is at least three things, and they have different binding times

The word covers three shapes whose static content is not the same, and treating them as one is how a
canon sentence ends up true of one and false of another.

**(a) The bounded expression.** `d = (a * b + c) / e`. The operator DAG, its depth, every width and every
strategy are compile-time objects. Nothing about its structure is deferred. Every fact in section 4 is a
const expression over it, so every guarantee in section 4 is const-decidable and I15 is satisfied without
argument.

**(b) The fold.** One operator, one accumulator, `n` steps. The per-step contribution is const; the
accumulated bound is a function of `n`, which is const only if `n` is. What is const regardless of `n` is
the **rate**: whether the error grows linearly, as a square root, logarithmically, or not at all. Probe A
is exactly a measurement of that rate for four accumulation shapes, and the rate is the thing a design can
promise when the count is not known.

**(c) The iterated map.** `x_{k+1} = f(x_k)`, trip count possibly data-dependent. Neither the count nor
the accumulated bound is const. What can still be const is an **invariant**: that `f` is a contraction on
the declared range, that the error does not grow, that a value is a fixed point. That is a different
species of guarantee and it is the only one available here.

**So the honest general answer to "can the design hold accuracy across a chain" is a predicate rather than
a yes or a no.** A bound is holdable where the structure and the count are compile-time facts. Where the
count is not, a **rate** is holdable. Where neither is, an **invariant** may be, and a bound is not. Those
three are different promises and the canon owes different words for each.

**The uncomfortable corollary, and it is a fork rather than a finding.** For shape (b) with runtime `n`,
a statement of the form "this chain's error is below E" is a claim about a value, and I15 forbids the
runtime check that would discharge it: "Never any runtime checks, ever. We catch invalids on compile time."
So either the count enters the typestate and shape (b) collapses into shape (a), or the guarantee is
stated as a rate rather than as a bound, or the obligation is pushed onto the consumer as a precondition
the type records and does not verify. I do not settle which; it is Option Q-C4 in section 8.

---

## 6. Where the guarantee has to live, and why this is a locus finding

Everything in sections 3 and 4 is a fact about a region. Not one of them can be attached to a value,
because a value does not know what will be done to it next, and probes B and C are the demonstration that
what will be done to it next changes the right answer.

That gives a structural test, which is worth more than any of the individual measurements:

> Take any candidate carrier for a guarantee. Ask whether two chains that agree on everything up to a
> given point, and differ only in what happens after it, would be assigned the same lowering at that
> point. If yes, the carrier is forward-only, and every backward fact in section 4.3 is invisible to it.

A strategy marker on a value's type fails that test, by construction: it is fixed when the value is
constructed and it cannot depend on the value's consumers. A width on a value's type fails it. An error
bound on a value's type fails it, and this is the subtle one, because such a bound looks like exactly the
right mechanism: it is a compile-time refinement, it composes forward correctly, and it is genuinely
useful. It is still forward-only, so it can express "how far off is this" and cannot express "how many of
these bits does anyone need".

**Three carriers pass the test, and they are the three shapes worth having in front of op.** All three
satisfy I14 and I15 without strain, since all three are const and all three monomorphise away.

**A named accumulator.** The consumer declares the intermediate's type once and the operations write into
it. Cheapest by a distance, entirely idiomatic, and it is exactly the `widened` arm in probe A. It carries
the budget and nothing else: the association choice and the demanded-bits fact are still invisible, and
the consumer does the sizing. Ships today with no new machinery.

**A combinator arvo owns.** The consumer hands arvo the operator and the data, and arvo owns the
accumulator, the association, and the residual. This is exactly I11's "contracts for things that compose
to bigger units than just numerals alone", it is the shape the algo crates already have, and it carries
everything in section 4 for the fold shape (b) and nothing for the expression shape (a).

**A staged expression.** The operators build a compile-time description of the region rather than a value,
and the description lowers at the point of observation, where both what produced each intermediate and
what consumes it are in view. This is the only one of the three that carries the backward facts, because
it is the only one where the sink is what triggers the lowering. It is also the one where I13's non-per-
operation arms have a site: an arm becomes a const-predicated rewrite of the description, which is exactly
the phrasing op used.

### 6.1 The cost of the third, stated honestly, because it collides with an intent

Under a staged expression, `let t = a * b;` binds a description and not a value. **That is a direct cost
against I3**, which op sharpened on 2026-08-14 to be about ergonomics rather than about where arithmetic
boundaries land: "Neither, it's ergonomics." A native Rust primitive's `let` binds a value, and a reader
who knows Rust's primitives is caught out by one that does not.

So there is a real tension between I7 and I3, and locating it precisely is worth more than resolving it.
I do not resolve it. Three things narrow it, and I state them because a tension that is smaller than it
looks should not be priced as though it were large.

The classic failure of this technique in C++ is a description outliving the temporaries it refers to, and
**Rust's borrow checker rejects that outright**, so the historically worst cost does not transfer.

The collapse is one annotation: `let t: Q<..> = a * b;` materialises, so a consumer who wants a value
says so, and the surprising case is the unannotated binding rather than every binding.

And the tension is not total. The named accumulator and the combinator cost nothing against I3 at all;
they simply carry less. So the fork is not "ergonomics or accuracy", it is a choice among three carriers
with different coverage, and the third is the only one that reaches the backward facts.

I have not measured the ergonomic cost and I do not know how to. It is a matter of taste about a surface,
which is op's rather than mine.

### 6.2 What LLVM does and does not recover, which decides whether any of this is needed

The strongest objection to this whole file is that the backend already does it: LLVM has a demanded-bits
analysis that narrows, and a reassociation pass that reassociates, so the region-level facts are
recovered below arvo and arvo need not represent them.

That objection is partly right and the part it is wrong about is the part that matters.

A backend reassociates where it can prove the operator associative in its own model. `add nsw` it can.
**A saturating add lowered to a compare and a select it cannot**, because the proof does not exist at that
level: the pass would have to rediscover a property of the operator over its whole domain from the
lowered form. Probe E establishes that unsigned saturating add is associative exhaustively at 4, 6 and 8
bits. **That proof exists in the typestate and does not survive lowering.** Supplying it is the entire
content of the work, and it is the shape this workspace calls microkernelling.

The same holds for demanded bits: a backend narrows what it can see, and what it can see stops at the
first operation whose lowered form it cannot prove is a congruence. Probe C's Horner row with one right
shift in it measures exactly that boundary from the other side.

So the answer is not "the backend does it" and not "the backend does nothing". It is that **the backend
recovers the region-level facts precisely where the operator's law is visible in the lowered form, and
arvo's operators are chosen so that theirs are not.** That is not a defect in arvo's operators. It is what
makes the typestate worth having, and it is the reason a region has to be represented above the lowering
rather than left to be rediscovered below it.

---

## 7. The chain question is already priced, in this repository, and nobody in this unit was told

`mock/benches/` is the only thing in this workspace that can price anything, and it holds a committed
harness family whose subject **is** a chain: `satfold`, twelve reduction lengths, nine arms per length,
40 samples per arm. Its meta records `Apple M1`, `Darwin 25.5.0`, `rustc 1.98.0-nightly (57d06900f)`, and
the harness loads its variants from `target/release`, so **profile = release** throughout.

I did not run it. The numbers below are read out of the committed `_findings.md` files by the script in
`167_probes/satfold_read/`, and the extraction is worth the paragraph it costs, because my first version
of it was wrong: splitting on the section heading without stopping at the next one let the bridge-overhead
table overwrite the medians, and the whole table came out as single-digit nanoseconds. Reported here
because a plausible table is what that error produces.

**Median nanoseconds of the algo, 32 KiB column, aligned, saturating add. `L` is the reduction length the
reassociation is applied over.**

| L | seq | iterfold | nolaw | lanes4-idx | lanes16 | lanes64 | neon | neon8 | winner | seq/winner |
|---|---|---|---|---|---|---|---|---|---|---|
| 8 | 8003 | 7913 | 8075 | 7378 | 7901 | 7998 | 7991 | 7946 | lanes4-idx | 1.1x |
| 15 | 10241 | 10152 | 10159 | 8024 | 10304 | 10255 | 10123 | 10492 | lanes4-idx | 1.3x |
| 16 | 10411 | 10473 | 10422 | 7869 | 10499 | 10469 | 1537 | 1533 | neon8 | 6.8x |
| 17 | 10699 | 10740 | 10732 | 7842 | 10788 | 10766 | 1651 | 1667 | neon | 6.5x |
| 32 | 15128 | 15125 | 14636 | 8815 | 9103 | 14639 | 886 | 873 | neon8 | 17.3x |
| 63 | 26827 | 26736 | 21166 | 2455 | 10315 | 26799 | 3932 | 4082 | lanes4-idx | 10.9x |
| 64 | 28746 | 28495 | 21983 | 1981 | 4080 | 7749 | 518 | 537 | neon | 55.5x |
| 65 | 28276 | 27175 | 21845 | 2004 | 4533 | 7633 | 542 | 539 | neon8 | 52.5x |
| 128 | 35216 | 35064 | 26921 | 2534 | 2239 | 5121 | 342 | 339 | neon8 | 103.9x |
| 256 | 38368 | 38496 | 29419 | 7115 | 1436 | 2782 | 284 | 274 | neon8 | 140.0x |
| 1024 | 41341 | 41170 | 31626 | 13075 | 834 | 1098 | 310 | 232 | neon8 | 178.2x |
| 4096 | 41713 | 41596 | 31997 | 14989 | 1528 | 670 | 362 | 255 | neon8 | 163.6x |

**Four readings, and every one of them is about the region rather than the operation.**

**The chain-level rewrite is worth between 1.1x and 178x, and the governing dimension is the chain
length.** At `L = 8` the findings file reports **seven of eight arms with no significant difference from
the baseline**; the whole family of chain rewrites is worth nothing there. At `L = 1024` the same family
is worth 178x. There is no single answer and the dimension that decides it, the length of the region, is
not a property any operation in the region has.

**The winner changes with `L`**, and it changes back: `lanes4-idx` at 8, 15 and 63, `neon` at 17 and 64,
`neon8` at 16, 32, 128, 256, 1024 and 4096. That is I13's composition of predicated arms, measured, in a
committed artifact rather than as a proposal.

**The law is the lever, not the bounds proof.** `nolaw` supplies the identical bounds proof with the
accumulation left strictly serial. It tracks the sequential arm to `L = 32` and separates to at most
1.30x at `L = 4096` (41713 against 31997). So of a 178x win, the bounds proof accounts for roughly 1.3x
and reassociation for the rest. **That is an attribution and it is the reason the associativity proof in
probe E is worth carrying in a typestate**: it is where nearly all of the win lives.

**The cliff at `L = 15` to `L = 16` is a mechanism boundary, not a gradient.** `neon` goes from 10123 to
1537, 6.6x, for one extra element. Sixteen bytes is one vector register on this host: below it the
hand-written kernel cannot fill one and pays the prologue for nothing. **The predicate that selects the
arm is `L >= 16`, and it is a hardware fact rather than a numeric one**, which is worth saying because a
canon that predicates arms only on width and strategy has no place to write it down.

### 7.1 One static lever that did not pay, and one caution about how these files are read

`lanes16-constl` is `lanes16` with the fold length lifted from a runtime value to a const generic:
exactly one static lever, everything else held. Across the twelve lengths it is faster at **six** and
slower at **six**, and the largest gap anywhere is 126 ns out of 10662, or 1.2%.

By the artifact's own per-variant confidence intervals the two do not overlap at `L = 17`
(`[10762, 11133]` against `[10657, 10709]`). **I do not read that as a result**, and the reason is
methodological and applies to anyone else reading these CSVs. Those intervals are computed for each
variant against a common baseline. `lanes16` against `lanes16-constl` is a **pairwise** comparison between
two non-baseline arms, and nothing in the artifact gates it. A significance figure computed at one
granularity does not license a verdict taken at a finer one.

So: **the static-length lever is unestablished in this arm shape**, not measured to be zero. What is
established is that the effect, if any, is under 1.2% and its sign flips half the time, which is a real
bound on how much anyone should expect from it. Deciding it needs a pairwise gate the artifact does not
carry, and that is what would close it.

### 7.2 What this section is evidence of, beyond the numbers

`RULES.md` records that eighteen files of this panel reported a trade as unpriced while
`mock/benches/` held the measurement, because no brief named the directory. **The same thing was about to
happen to this unit.** The brief for `166` opens the chain topic on the ground that the panel has never
had one, which is true of the panel's files and not true of its repository: the largest chain-level result
in the tree, an entire committed family sweeping the reassociation question across twelve region lengths,
was already there.

A negative claim about evidence is a claim about a place. This one is checkable in one command, and it
comes out the other way.
