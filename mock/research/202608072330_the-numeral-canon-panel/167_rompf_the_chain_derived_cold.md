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

---

## 8. Association order is a speed lever in fixed point and an accuracy lever in relative precision

`167_probes/order/`. This one I built expecting the opposite answer, and it is the finding I would most
like a second derivation of.

The reflex imported from numerical analysis says pairwise summation beats sequential summation, and the
argument is about **relative** precision: a float's absolute rounding error scales with the magnitude of
the running sum, so keeping partial magnitudes balanced keeps errors small. A fixed-point accumulator has
a fixed absolute LSB.

**Fixed point, Q(.12) multiply-accumulate, error in LSBs, worst of 32 seeds:**

| n | sequential | pairwise tree | difference |
|---|---|---|---|
| 16 | 9.985 | 9.985 | **0.000** |
| 1024 | 536.403 | 536.403 | **0.000** |
| 65536 | 32922.755 | 32922.755 | **0.000** |

Zero sizes where either order is strictly better, at every size from 16 to 65536.

**The control, the identical comparison on a relative-precision accumulator:**

| n | sequential | tree | seq/tree |
|---|---|---|---|
| 16 | 1.20e-6 | 6.11e-7 | 1.96 |
| 4096 | 4.17e-3 | 2.38e-4 | 17.56 |
| 65536 | 2.43e-1 | 2.56e-3 | **94.80** |

The tree wins at 7 of 7 sizes, so the instrument detects an ordering effect where one exists. The other
control, that both orders perform the identical number of truncations, is clean at every size.

**What this settles.** In fixed point, association order is **purely a speed lever with exactly zero
accuracy content**. In relative precision it is both, and by two orders of magnitude at large `n`.

**Two consequences, and the second is a warning about how a canon gets written.**

The apparent conflict between speed and accuracy over reassociation does not exist for the fixed-point
family. A strategy that spends everything on accuracy loses nothing by taking the 178x reassociation win
in section 7, because probe F says the reassociated answer is bit-identical. That is an empirical answer
to whether two strategy concerns agree on this one axis, arrived at by measurement, which is the only way
op has said such a question is answerable.

And **a canon sentence about reassociation that does not name the numeral family is wrong for one of
them.** "Reassociation trades accuracy for speed" is false in fixed point and true in relative precision;
"reassociation is free" is true in fixed point and false in relative precision. This is a dimension the
predicate notation would have caught if anyone had written the finding down with a family axis, and it is
the reason I state the family in every predicate below.

`holds for: fixed rows F = 12, I = 3, n in {16 .. 65536}, operands uniform in [-4,4), truncation = floor,
signedness = signed, tree arity = 2, threads = 1. relative-precision rows: f32, operands uniform in
[0,1), all positive, so catastrophic cancellation is absent by construction and the tree's advantage is
not claimed for mixed-sign data.`

---

## 9. Options this file opens, each with what would close it

Nothing here is settled and I close nothing. Each option below states the decision procedure that would
resolve it, because an option without one accretes rather than forks.

**Q-C1. Which carrier holds the region-level guarantee.** Three candidates in section 6: a named
accumulator, a combinator arvo owns, a staged expression that lowers at the observation point. They are
not exclusive and the honest expectation is that all three ship, covering different shapes.

*What closes it:* a structural test rather than a measurement. For each candidate, write the four
obligations of section 1 and mark which it can express. The named accumulator carries the budget only;
the combinator carries everything for the fold shape and nothing for the expression shape; the staged
expression is the only one reaching the backward facts. If that table is agreed, the answer is a
composition and the remaining question is only which shapes each covers.

**Q-C2. What the observation boundary is, mechanically.** Section 1 defines a region by what is observed.
That definition is only as good as the decision procedure for "observed". Under I14 there is no heap, no
`dyn` and no aliasing through a trait object, which makes the question far easier here than in general.

*What closes it:* a worked enumeration of every way a value escapes a region in a no-alloc,
no-`dyn`, monomorphised setting: a store to a column, a return across a public boundary, a reference taken,
an inclusion in an aggregate that escapes, an FFI crossing. If that list is finite and each case is
decidable at compile time, the definition is mechanical. I believe it is and I have not built it.

**Q-C3. Whether the residual carry is worth its register.** Probe A shows it recovers the exact
one-rounding guarantee for one extra F-bit register, and section 3.2 shows a region where it is the only
form that fits a 64-bit container.

*What closes it:* a bench with real competitor arms. The arms are the four in probe A plus a hardware
fused multiply-accumulate where one exists, over a length sweep like `satfold`'s. **Its cost is entirely
unpriced by me**, and the accuracy result says nothing about it.

**Q-C4. What a chain-level guarantee says when the count is not const.** Section 5 shape (b). Three
shapes: put the count in the typestate, which collapses it into shape (a) and costs the consumer a const;
state a rate rather than a bound; or record a precondition the type carries and does not verify. I15
forbids the fourth, a runtime check.

*What closes it:* op, or a convergence. It is a question about what a guarantee **means**, not about what
is achievable, and each of the three is achievable.

**Q-C5. Whether the arm predicate needs a hardware axis.** Section 7's cliff at `L = 15` to `L = 16` is a
vector register width, not a numeric property. If arms are predicated only over width, strategy and
signedness, that predicate has no place to be written.

*What closes it:* checking whether the same cliff sits at 16 on a host with a different vector width. One
run of the existing `satfold` family on non-NEON hardware answers it and needs no new code.

**Q-C6. Whether the static-length lever ever pays.** Section 7.1: six of twelve, under 1.2% either way,
and the artifact does not carry a gate at the granularity the comparison needs.

*What closes it:* a pairwise noise gate over the two arms specifically, on the existing artifact. No new
bench, one analysis.

---

## 10. Existence and locus, challenged as the brief invites

**The unit should exist and the intent for it is stronger than the brief said.** The brief names I7. I11
is at least as load-bearing: "the contracts for things that compose to bigger units than just numerals
alone" is a sentence about compositions being the product, in op's own words, and a unit on compositions
that cites only I7 has understated its own warrant.

**The locus challenge, and it is the finding rather than a caveat.** If a guarantee about accuracy is
placed on a numeral's type, it is on the wrong side of a boundary, and section 6's structural test is how
to see that without arguing about mechanisms: a carrier fixed when a value is constructed cannot depend
on what consumes the value, and probes B and C are the measurement that what consumes the value changes
the right answer. This is conditional, because **I have not read the panel and do not know what it has
built.** If the panel has already put the strategy or the guarantee somewhere above the value, this
challenge is already answered and I would rather find that in phase two than have it be true.

**A second locus observation, offered as a question rather than a finding.** Nine topics have produced
candidates and the first unit on compositions is file 167. If the composition contract is the product per
I11, and the per-value primitive is the input to it, then the ordering has built the input first and the
product last. That is a defensible order and it is also the order in which a boundary error is cheapest to
make and most expensive to find, because every candidate written so far has been free to assume the
operation is the unit without anything contradicting it.

**One mechanism I noticed that nothing in the intents licenses, outside my question, reported because the
standing instruction says to report it.** `mock/benches/variants/satfold-shared/src/lib.rs` documents
itself by citing panel files by number and reproducing their conclusions. That is excellent practice for
a bench and it is a **contamination channel for exactly this protocol**: the blind list forbids panel
files and permits bench crates, and the bench crates quote the panel files. It contaminated section 4.2's
framing for me and I have declared it in section 0.3. Nothing needs deleting; the blind list needs a line.

---

## 11. What I settled, what I moved, what I could not

**Settled, with evidence and controls.**

1. Per-operation correct rounding does not compose into chain-level correct rounding, and no intermediate
   width short of exact makes it. Exhaustive, three widths, both controls clean.
2. Association licence is per operator, not per family: saturating add composes, saturating subtract does
   not, saturating multiply does, fixed multiply does not. Exhaustive at three widths, both controls clean.
3. Rounding better does not buy associativity. Same order of violation with truncation and with
   round-to-nearest.
4. Backward narrowing is licensed exactly for the congruences and refused for saturation, division, right
   shift and min. Exhaustive, 16.7M triples per operator.
5. The residual carried forward is exactly equal to accumulating in the wide type, and it turns an error
   linear in chain length into one bounded below one LSB. Three controls clean.
6. In fixed point, association order has exactly zero accuracy content, while the same comparison on a
   relative-precision accumulator shows the tree ahead by up to 94.8x. Both controls clean.

**Moved.**

The unit from the chain to the unobserved region, on the ground that the region's boundary is what bounds
the design's freedom and the operator structure is not.

The framing of I13 and I7 from two intents to one mechanism: some of I13's arms are rewrites of a region,
and under a per-operation-only surface they have no site.

The status of the chain topic from unpriced to priced, by naming a committed harness family that sweeps
exactly it across twelve region lengths.

**Carried forward unchanged, and from whom. Count: two, both from op.** I7's reading that the accuracy
concern ranges over compositions rather than over isolated operations, and I13's shape, that the output is
predicated arms composed rather than one universal answer. I did not modify either and section 7's
crossover table is the strongest instance of I13 I have seen, because it is a measured composition rather
than a proposed one. From the panel: **nothing**, since I read none of it.

**What I could not do.**

**I could not price any of it.** Every probe here is exact arithmetic. What a saved bit, a carried
residual or a licensed rewrite is worth in time is **unpriced** by me, except where section 7 reads a
committed artifact, and that artifact covers one operator on one host.

**I could not settle Q-C2**, the mechanical definition of observation. I believe it is decidable under
I14 and I did not build the enumeration, so the definition in section 1 is a definition with a hole in it
and the hole is named.

**I could not measure the ergonomic cost in section 6.1**, and I do not know how. It is taste about a
surface, which is op's.

**I could not test the hardware axis in Q-C5.** I have one host.

---

## 12. Every predicate in one place

- **Per-operation correct rounding does not compose.** `holds for: F in {6, 8, 10}, M in [F, 2F], rounding
  = nearest-ties-to-even, operation = fixed-point multiply, signedness = unsigned, family = fixed point,
  threads = 1`
- **Association licence, the twelve-operator partition.** `holds for: W in {4, 6, 8}, arity = 3, signedness
  = unsigned, F = W/2 for the fixed-multiply rows, family = fixed point, threads = 1`
- **Backward narrowing licence.** `holds for: W = 12, K = 6, chain length = 3, signedness = unsigned,
  family = fixed point, threads = 1`
- **The residual recovers the one-rounding guarantee.** `holds for: F = 12, I = 3 including sign, n in
  {1 .. 2^20}, operands uniform in [-4,4), signedness = signed, family = fixed point, rounding in {floor,
  nearest}, threads = 1, profile = rustc -O`
- **The residual fits where the widened accumulator does not.** `holds for: container = 64 bits, I = 3
  including sign, F in {8,16,20,24,26}, n in {2^10, 2^14, 2^18}, operands non-cancelling, rounding =
  floor, signedness = signed, family = fixed point, threads = 1`
- **Forward-only width assignment over-provisions.** `holds for: the five chain shapes and width rules in
  the probe source, family = fixed point, threads = 1`
- **Association order has zero accuracy content in fixed point.** `holds for: F = 12, I = 3, n in {16 ..
  65536}, operands uniform in [-4,4), truncation = floor, signedness = signed, tree arity = 2, family =
  fixed point, threads = 1`
- **The tree beats sequential in relative precision.** `holds for: f32, n in {16 .. 65536}, operands
  uniform in [0,1) all positive, family = relative precision, threads = 1`
- **The reassociation win ranges 1.1x to 178x governed by region length.** `holds for: L in {8 .. 4096},
  operator = saturating add, element width = 8 bits, signedness = unsigned, column = 32768 elements
  aligned, host = Apple M1 with NEON, profile = release, threads = 1, family = fixed point`
- **The bounds proof alone accounts for at most 1.30x of that.** Same predicate.
- **The arm cliff at L = 16.** Same predicate, and it is a claim about `target features = NEON` in
  particular rather than about target features any.

**Every predicate above says `threads = 1`**, which under this panel's notation is a region and not a
hedge. Nothing here was established on more than one thread and I claim nothing there. It is also the
place where the whole file is most likely to be incomplete: a region evaluated concurrently has a
partitioning question, and partitioning a fold is a reassociation, so section 4.2's licence table is
exactly what would decide whether a given chain may be split across threads at all. Nobody has asked that
question here and it is a whole unit.

---

*Phase one ends here. Phase two, the reconciliation against the panel, is appended below and phase one is
not rewritten.*

---

# Phase two: reconciliation

Read after phase one was committed. Curated list per the brief: `166`, `109` section 8, `110`'s P7 and
P8 with its section 4, `112` section 8, `AGREEMENTS.md` sections 6 and 12, op's `113`, and `OPTIONS.md`.
Phase one is not rewritten and its errors are corrected here rather than there.

## R0. What I got wrong, before anything else

**My section 7.2 overreaches and I withdraw its framing.** It says the chain question was priced and the
unit was not told, and implies the panel did not know. **The panel knew.** `OPTIONS.md` Q42 is exactly
that measurement, entered by `92`, and it is more careful than my section 7 is:

> **Each vectorised arm is at parity with the fold as written below its own lane count, and first pays
> above it.** Measured crossovers: the 16-lane arm first pays at `L = 32` (1.66x), the 64-lane arm at
> `L = 64` (3.71x), hand-written NEON at `L = 16` (6.77x).

So the crossover I present in section 7 as a reading of my own is in the register, with its mechanism
named. What survives from my section 7 is the extraction across all twelve lengths in one table, the
`nolaw` attribution arithmetic, and the `lanes16-constl` comparison. The claim that a place was unchecked
is wrong and I should have grepped `OPTIONS.md` for the bench family's name before writing a sentence
about what nobody had looked at. That is the exact failure mode `RULES.md` names: a negative claim about
evidence is a claim about a place, and I made one without checking the place.

**And Q42 corrects my section 7.1 in my favour, which is worse rather than better.** It records the
harness's own noise floor:

> The harness's between-dylib noise floor on this host measured 4.9% with disjoint intervals, which
> bounds every small claim above.

My largest `lanes16-constl` gap is 1.2%, comfortably inside 4.9%. So the right verdict was available in
the register and I derived a weaker version of it from first principles about pairwise gating. Both
routes reach "unestablished"; the register's is shorter and has a number.

**One predicate discipline I did not apply and Q42 did.** Q42 writes: "**No dimension for strategy is
listed**, so under the ratified notation none of this may be read as a statement about any named
strategy." **Every predicate in my section 12 has the same absence and I did not say so.** Nothing in this
file may be read as a statement about any named strategy. Stated here rather than by editing section 12,
per the rule that a predicate is not widened or narrowed in place.

## R1. Where I converge, and whether the instances are independent

**Four independent routes now reach the same structural conclusion, and mine is the fourth.**

`109` section 8, blind: chain accuracy is a fact about the operator typing rather than about any
component of the operand type, and "a per-value primitive has no slot for it".

`63` section 5, blind, via the format concept, as recorded at `AGREEMENTS.md:499`: closing operations
over the format "so adaptation fuses invisibly into each one makes op's chain-accuracy intent (I7)
unstatable".

`90` R11, via the lifting theorem.

**Mine**, via the observation boundary and the backward-dataflow argument, with probes B and C as the
evidence that what consumes a value changes the right lowering for it.

`AGREEMENTS.md:497-506` already records the first three as a cross-topic convergence in which "none of
the three cites the prior two". I add a fourth arrived at from a fourth direction, and my route is the
only one of the four that produces a **decision procedure** rather than a conclusion, which is section 6's
structural test: ask whether two chains agreeing up to a point and differing after it get the same
lowering at that point. `109` and `63` establish that the guarantee is not in the value; the test says
what to do with any proposed carrier, including ones nobody has proposed yet.

**Independence, honestly discounted.** I read none of those files before committing phase one. The
shared inputs are `INTENTS.md`, `RULES.md`, the workspace rules and `mock/benches/`. Two of my
conclusions ran through a shared input and I discount them:

- **Section 4.2's framing** is contaminated by `satfold-shared/src/lib.rs:6`, which cites panel files
  `80` and `82` and reproduces their conclusions. Declared in section 0.3. The specific claim that
  unsigned saturating addition is associative was already in that file's test suite, so **my probe E is
  not an independent instance of that one row.** What is independent in probe E is everything else in it:
  widths 4 and 6, saturating multiply, saturating subtract, fixed multiply with and without rounding,
  average, and the twelve-operator partition as a whole.
- **The predicate notation and I13's arms shape** come from `RULES.md` and `INTENTS.md`, which every
  member reads. Where my file says a result is "I13's shape", that is a shared premise rather than a
  convergence.

## R2. What I carry forward unchanged, and from whom. Count: five.

1. **I7's reading** that the accuracy concern ranges over compositions. From op.
2. **I13's shape**, arms with const predicates composed. From op.
3. **`109`'s conclusion** that chain accuracy has no slot in a per-value primitive. Independently derived
   before reading, so this is a second instance rather than an adoption.
4. **`110` section 4's split** of "composition" into configuration and construction, and its consequence
   that a composite is a primitive under the same definition. I did not derive it, I do not contest it,
   and I extend it in R3.
5. **`112` section 8's mechanism**, that a declared extent discharges a construction's base predicate.
   I did not derive it and I use it in R5.

Nothing I read needed changing on my account except my own section 7.2.

## R3. "Composition" carries three jobs, not two

`110:314` splits the word two ways: **configuration**, filling in a record of parameters, and
**composition**, "a construction taking an algebra to an algebra" (`110:322`). Both are about carriers.

**Neither is a chain.** A chain is composition **in the dataflow**: the same carrier, several operations
in sequence, a region of a program rather than a new algebra. It produces no new primitive, it has no
construction, and `110`'s "a composite is a primitive" says nothing about it because there is no composite.

That matters for Q16, which asks which sense the canon needs. **The answer is at least three**, and the
third is the one I7 is stated over. A canon that carries `110`'s two senses and calls the question
settled has no word for the thing op's accuracy intent ranges over.

**And the three interact rather than sitting side by side.** `110` F11 reports that the componentwise
product reproduces its base's law set exactly while the twisted constructions do not. Read against my
probe E, that is a statement about which **chain** rewrites survive a **construction**: a lane-wise
composite of a base whose saturating add is associative may still be reassociated lane-wise, and a
complex composite of the same base may not, because complex breaks `mul_assoc` over a saturating base.
So the construction sense determines what the chain sense is licensed to do, and neither can be
specified without the other.

## R4. To `109`, per op's `113`: three replacements for the cost it named, one of which works

`113` says an attacker owes several solutions addressed to the party it refuted. I am not refuting
`109`; I am answering the cost it named against its own resolution, which is the same obligation from
the friendly side. `109:438` states it:

> **And the cost, so this is not read as a free win.** The deferred route's accumulator grows with the
> chain: at `F = 8` and eight factors it carries 64 fraction bits before the final narrowing, which no
> 8-bit or 16-bit container holds.

**Replacement one: carry the residual instead of widening the accumulator.** For an **accumulate** chain
this is exact and costs one extra F-bit register. Probe A: `comp == widened` on every row from n = 1 to
n = 2^20, because `acc * 2^F + carry_n = sum(p_i)` exactly. Probe A2 finds five geometries at a 64-bit
container where the carried form is exact and the widened one wraps. **This fully answers the cost, for
the accumulate shape only.**

**Replacement two, and I tested it and it does not work.** `109`'s own chain is a chain of
**multiplications**, and I built probe G specifically to see whether replacement one transfers. It does
not, and the reason is arithmetic: a product of `k` factors at `F` fraction bits needs `F*(k-1)` fraction
bits, and a constant `c` carried limbs hold `c*F`. So the carried form reproduces the deferred answer
exactly up to `k = c + 1` and departs after. At `F = 4`, `k = 24`, per-step is 2221.9 LSB, one carried
limb is 60.9, two are 5.9, and deferred is 0.99. **Each limb buys a constant factor and not a change of
growth class.** This corroborates `AGREEMENTS.md:497-506` rather than refuting it, and it is the honest
bound on what probe A contributes.

**Replacement three: bound the container by the region's declared extent rather than by its length.**
`109` says the intermediate width "is a function of the **chain**, which no per-value type knows", which
is right, and my section 3.2 sharpens what kind of function. The **worst-case** width `I + 2F + log2(n)`
is reached only by non-cancelling terms; on decorrelated data the sum is a random walk and the realised
width is smaller by up to `log2(n)/2` bits. My probe A2's first version failed for exactly this reason
and found zero overflowing geometries at any size, which is committed as `carrier_bound_v1_FAILED.out`.
So the accumulator width is a joint fact of the chain length **and the correlation between its terms**,
and the second is a property of the data that arvo cannot derive and a consumer can declare. **That is
`112`'s mechanism, applied to `109`'s cost**, and it is R5.

**And a fourth thing, offered as a question rather than a replacement.** `109` draws from its table that
"`Mul` is not an endomorphism". By my section 6 structural test, letting the operator's result type be a
function of its operand types is still a **forward-only** carrier: it is determined by the producers and
it cannot see the consumers. So it carries obligation three, the budget, and it does not reach the
backward facts in probes B and C. I read `109`'s resolution as necessary and not sufficient, and I would
rather that be attacked than adopted.

## R5. `112`'s extent is the carrier for a chain fact nobody has assigned

`112:850`:

> **A declared extent discharges a construction's base predicate.** Eight of forty declared extents over
> a wrapping base gain a construction the ungraded predicate refuses, and the propagated bound predicts
> the gain with zero unsound predictions.

That is stated over the **construction** sense of composition. The identical mechanism answers a chain
question that is currently nobody's: **a declared extent on the operands discharges the worst-case
accumulator width**, because the extent is exactly the fact that separates cancelling from
non-cancelling terms. My section 3.2 measures the gap that the extent would close and `112` has the
mechanism that closes it. Neither file knew about the other.

I offer this as a connection to be tested rather than as a result, because `112`'s propagation rule is
established for interval closure over a construction and I have not checked that the same propagation is
sound for an accumulator bound over a chain. **What would close it:** run `112`'s propagated-bound
machinery over a fold's accumulator width instead of over a construction's closure, on the same three
outcome classes (unsound, conservative, exact). `112`'s p5 already has the shape.

## R6. What my probe F contributes to Q41, which none of its three options anticipates

Q41 asks whether the strategies are partially ordered by how many chain-level laws they honour, resting,
in `76`'s words, on the accuracy-first concern preserving "chain-level facts" the speed-first concern is
licensed to give up. Its three options are: a refinement order, a non-nesting order, no order.

**Probe F says the answer is family-dependent, which is a fourth reading.** On the reassociation axis, in
fixed point, the two concerns honour the **identical** set: the reassociated answer is bit-identical, at
every size from 16 to 65536, so there is no chain-level fact for one to preserve and the other to give
up. On the same axis in relative precision the sets differ by up to 94.8x. So on this axis the order is
not a fact about the strategies at all; it is a fact about the numeral family, and the strategies inherit
it.

That is one axis and I do not generalise it. **What would close Q41 on this axis** is running probe F's
comparison over the other chain-level laws in `110` F11's law set, per family, and checking whether the
family-dependence recurs or whether reassociation is special.

**And it has a consequence for the fixed-point family that is worth stating on its own.** The 178x
reassociation win in section 7 is available to an accuracy-first concern **at no accuracy cost**, because
probe F says the answer does not change. A design that gates reassociation behind a speed-first strategy
in fixed point is giving up a large win for nothing.

## R7. Where I hold, and against what

**I hold that the unit is the unobserved region rather than the chain**, and `AGREEMENTS.md:206` gives me
the thing to hold it against. C9, from `63`, is the panel's existing chain concept:

> a chain is exact operations plus a schedule of adaptation points; the schedule is part of the
> function's meaning

**These are not in conflict and they are answering different questions.** C9 says what a chain
**contains**. Mine says what **bounds** one. They compose exactly: an adaptation point is **forced**
where an intermediate is observed and **optional** everywhere else, so the observation boundary is what
constrains C9's schedule, and C9's schedule is what fills the region my definition delimits. I would put
both in a canon and neither alone.

C9 is marked ONE EXPERT, cold, unattacked. Mine is a second cold instance reaching a compatible
statement from a different direction, which is the rung it is worth and no more.

**And C9's three directions for where the chain's home lives** are `AGREEMENTS.md:206-211`: closed
operations elsewhere, the three-carrier concept, or a first-class typed chain object. My section 6's
three carriers are a different cut of what is nearly the same space, and **my contribution is the
discriminator rather than the list**: the structural test in section 6 separates them by whether they can
see the consumer, which puts the named accumulator and `109`'s widened operator target on one side and
the first-class chain object on the other. C9's consolidation restored its third direction after
dropping it in its own first draft; that third direction is the only one of the three that passes the
test, which is worth knowing about a direction that was nearly lost twice.

## R8. Options, revised against the panel

**Q-C1 through Q-C6 stand as written in section 9**, with three amendments.

**Q-C1 folds into Q16 and C9's three directions rather than standing alone.** Its contribution is the
structural test, not the list of carriers. Restated: the open question is not which carrier, it is
whether the canon accepts that a carrier must be able to see the consumer, and if so, which of the
existing candidates can.

**Q-C3 is partly answered and partly narrowed.** The residual carry's accuracy value is settled for the
accumulate shape by probe A and refuted for the product shape by probe G. Its **cost remains unpriced**
and that half stands.

**Q-C6 is closed by Q42's noise floor**, not by me. 1.2% inside a 4.9% floor. It moves to a droplist with
that diagnostic, and what would reopen it is a harness whose floor is below the effect.

**One option I add.**

**Q-C7. Whether "composition" needs a third word, and whether I7 is stated over it.** `110` splits the
term two ways and Q16 asks which senses the canon needs. Neither sense is a chain, and I7 is stated over
chains. *What closes it:* a decision by whoever writes the Q16 text, and it is cheap, because the
evidence that the third sense is distinct is that `110`'s composite construction produces a new primitive
while a chain produces none.

## R9. Coverage of phase two, bounded

**Read in full:** `166`, `109` section 8, `110` sections 4 and 5 and its P7/P8 material, `112` section 8,
`AGREEMENTS.md` sections 6 and 12 and its section 2.2 entry for C9, op's `113`, `OPTIONS.md` entries Q11,
Q12, Q16 header, Q41, Q42, Q55.

**Not read:** every other numbered file, `DROPLIST.md`, `HANDLES.md`, `PRIOR_CALLS.md`,
`PERSONA_CALLS.md`, `164` and `161` in full, `165`, and the whole of `63`, `74`, `90`, `92` and `106`.

**Which of my phase-two sections would move if something I leaned on were wrong.** R1's four-route
convergence rests on `AGREEMENTS.md:497-506`'s account of `63` and `90`, **and I did not open either
file.** That is the shared-unread-source condition `RULES.md` names, and I state it as `43` did: if the
consolidation's account of `63` section 5 or `90` R11 is wrong, R1 drops from four routes to two, my own
and `109`'s, both of which I verified at source. R4's replacement three rests on `112` section 8, which I
read. R6 rests on Q41's quotation of `76`, which I did not open; if `76`'s candidate is misquoted there,
R6 answers a question nobody asked.

**One count I verified rather than took.** I ran the thirteen bench crates' suites myself, at
`--release`, and reconciled the 108 and 123 figures in section 0.2. Everything else numeric in phase two
is quoted from a file I opened at the line cited.

---

## R10. The doability check my own section 6 owed, run rather than asserted

Section 6 names three carriers and section 9's Q-C1 leaves the choice open. `RULES.md` says the canon
must establish that a thing is doable, and I had asserted the third carrier fits I14 and I15 without
compiling anything. That is exactly the gap between an intent and a wish, so I built it.

`167_probes/staged/`. A description carrying the operator structure at the type level, computing its
forward width as an associated const with no observation, and lowering under the consumer's demand at
the observation point.

```
NC20  forward width computed from the description alone, no sink: 33
NC19  work width at demand 8 = 8, at demand 63 = 33, moved = true
NC18a congruence-only chain: narrow 221 vs wide-then-mask 221, agree = true
NC18b with a blocking Shr node: narrow 103 vs wide-then-mask 103, agree = true
NC18c a lowering that WRONGLY passes the demand through the shift gives 7, correct is 103, differ = true
PASSES_DEMAND: Add true, Mul true, Shr false
size_of description = 24 bytes, no vtable, no allocation
```

**Verified rather than asserted, by count:** zero `#![feature(...)]` attributes across all three files,
zero `dyn`, zero `TypeId` or `core::any`, zero `Box`, `Vec` or `alloc`, and the `#![no_std]` build exits
0 on the committed pin. The description costs 24 bytes for three `i64` leaves, which is the three values
and nothing else.

**NC18c is the control that matters.** `Shr` refuses to pass the demand to its operand; NC18c builds the
lowering that wrongly passes it and gets 7 against the correct 103. Without that, NC18b's agreement
would be consistent with a description that ignored the flag entirely.

**So the third carrier is doable and this is the doability claim, not a design.** Its spelling is
scaffolding. What it establishes is that section 6's structural test has a candidate that passes it under
the operating constraints, which is what was missing when Q-C1 was written.

**What it does not establish, and both matter.** Whether the description lowers to the same machine code
as the direct form is **unpriced**, and pricing it needs the harness with the direct form as a competitor
arm. And the ergonomic cost in section 6.1 is untouched, because it is a matter of taste about a surface
rather than something a probe answers.

*What would close Q-C1's cost half:* a bench family with two arms, the direct form and the described
form, over the shapes in probe C, on the harness. The variant crates in `mock/benches/variants/` are the
pattern and it needs no new machinery.

---

## R11. The closing statement

**A chain is a maximal region of a computation in which no intermediate is observed**, and the design's
obligations to it are three that its steps cannot add up to, plus one asset its steps destroy. `63`'s C9
says what such a region contains, a schedule of adaptation points; this file says what bounds one, and
the two compose: an adaptation point is forced where an intermediate is observed and optional everywhere
else.

**Four independent routes now say the guarantee cannot live in a value**, and my contribution is the
decision procedure rather than the conclusion: ask whether two chains agreeing up to a point and
differing after it are lowered the same way at that point.

**Three findings are new here as far as a grep of the live panel shows, and each is small.** That
association order has exactly zero accuracy content in fixed point while the same comparison in relative
precision runs to 94.8x, which makes Q41's answer family-dependent. That the residual carried forward is
exactly equal to the wide accumulation for an accumulate chain and only a constant factor for a product
chain, which answers `109`'s named cost for one shape and confirms `AGREEMENTS.md:497-506` for the other.
And that the worst-case accumulator width is a fact about correlation between terms rather than about
length, found by a probe of mine failing, which is where `112`'s declared extent has a job nobody has
given it.

**And one correction against myself**, in R0: I claimed a place was unchecked without checking it, and
`OPTIONS.md` Q42 had the measurement, the mechanism and a better noise bound than I derived.

---

## R12. `60_stam_the_chain_derived_cold.md` exists, it was not in my reading list, and it corrects me

Found by running the novelty grep R11 claims to rest on, which is the only reason this section exists
and is the argument for running such a grep rather than asserting one. **The panel already has a full
cold derivation on this exact question**, and its title is `the chain derived cold`.

**The reading list I was given did not contain it.** `166` opens this unit on the ground that the panel
has never had a unit on chains, which is true, and the curated list that followed named `109`, `110`,
`112`, `164`, `113`, two `AGREEMENTS.md` sections and `OPTIONS.md`. A grep of the panel root for the
word "chain" in a filename costs one command and returns `60`. I report that plainly because the same
shape produced `RULES.md`'s eighteen-file incident: the brief named where to look and the place that
mattered was not on the list.

### What `60` already had, so my file is a second instance rather than a finding

**My three obligations are a subset of `60`'s five.** Its section 3, "What a chain needs that a single op
does not", lists an intermediate format, a schedule, an association and order statement, a count bound,
and an error bound composed per adaptation point. My endpoint contract, association and budget map onto
three of those. **`60` is ahead on the other two**, and one of them is better than anything in my file:

> the drift of a chain is the sum over its adaptation points of the local adaptation error, each weighted
> by the sensitivity of the remaining suffix of the computation to a perturbation at that point... **the
> schedule is also the index set of the error analysis. Fewer adaptation points is not merely cheaper
> rounding; it is a structurally shorter error sum.**

That is the explanation of my probe A. My four arms differ in **how many** adaptation points they have
rather than in the quality of each, and `60` had the structure while I have the measurement. I would
rather this had gone the other way round and it did not.

**`60`'s grade taxonomy already distinguishes what my probe D measures**, as grade a (composite correct
rounding, one adaptation on the true composite) against grade b (stepwise correct rounding, the only
compositional grade). It measured the gap too: per-step round-to-nearest wrong on 15,628 of 46,656 and
per-step truncation on 42,892. So my probe D's headline is a second instance.

**`60` section 6 has the order-dependence result and it exposes a hole in my probe F's predicate.** Its
probe B shows a per-step saturating fold of `{30000, 10000, -25000}` in `i16` giving 7767 or 15000 by
order, while the wrapping fold gives 15000 in every order because arithmetic mod 2^n is a ring
homomorphism. **My probe F measured zero difference between sequential and tree order because its adds
were exact**: no accumulator overflow occurs at the sizes swept, so no saturation is reachable, so no
order-dependence can arise from that side. The predicate I wrote names no overflow-policy dimension at
all, which under this panel's notation already means the finding holds nowhere an overflow policy exists,
so it is not false. **It reads as an oversight rather than as a claim, and it was one.** Stated here
rather than by editing section 12, per the rule that a predicate is not widened or narrowed in place: the
region probe F actually establishes is one where **no adaptation on the additive side occurs at all**.

### The concession: `60`'s window dominates my carried residual for product chains

`60` section 5 answers the multiplicative growth with the **window**: a bounded subterm whose exact
result width fits the container, evaluated entirely in the width algebra and adapted once at its exit,
so a chain factors into windows and pays `ceil(k/w)` adaptations rather than `k`.

**That strictly dominates my probe G's carried residual for products, and I concede it outright.** A
window of `w` factors is **exact** for those `w`. My carried form with `c` limbs is exact to `k = c + 1`
and approximate after, at comparable state. There is no region where the carried form beats a window of
the same width, and probe G's own table shows why: each limb buys a constant factor while the window
buys exactness up to its capacity.

**What survives from probe A, and it is smaller than I claimed.** For the **additive** window, which is
`60`'s own construction for a dot product (products exact at 2F, sum exact with log-k headroom, one
narrow), the carried residual reaches the identical answer with the accumulator held at scale `F` plus a
separate `F`-bit carry, rather than at scale `2F`. That is a constant-factor saving on state inside
`60`'s window rather than a rival to it, and probe A2's five geometries at a 64-bit container are where
that saving decides representability. **I withdraw the framing in R4 that presents it as an answer to
`109`'s cost** and restate it as a refinement of `60`'s window.

### What I hold, and what I add to a direction `60` opened

**`60`'s D-C is my third carrier, and it names the costs I name.** Its words: "Expression templates: the
term itself is a type, the schedule chosen at evaluation. Everything monomorphizes, no alloc or dyn
needed". **That last clause is an assertion and my probe H is the check.** Gate-free, `#![no_std]`, zero
`dyn`, zero `TypeId`, 24 bytes for a three-leaf description, with a control showing that a lowering which
wrongly passes the demand gives 7 against the correct 103. So D-C is established as doable rather than
believed to be, which is the bar `RULES.md` sets before an intent may be written down.

**And I contest one clause of `60`'s D-C discriminator.** It says D-C is "distinguishable from D-B by
asking whether any consumer needs to abstract over *schedules* at compile time rather than pick one per
call site; if none does, **D-C is D-B with ceremony**."

**There is a second reason, and it is not about schedules.** D-B places the width algebra, the named
adaptation and the exactness predicate in the format concept, all of which are computed **forward** from
operand formats. By my section 6 structural test, none of them can see the consumer. Probes B and C
measure what that costs: the backward demand licenses evaluating a whole congruence region at the
consumer's width, worth 50.9% of the intermediate bits on a four-term multiply-accumulate and 72.7% on a
Horner chain, and the licence is refused for saturation, division, right shift and min. **So even where
no consumer abstracts over a schedule, D-C carries a fact D-B cannot express**, and "D-C is D-B with
ceremony" holds only if the backward facts are worth nothing. They are not priced, so I do not claim they
are worth the ceremony; I claim the conditional as written is missing a term.

**And my definition still adds something to `60`'s.** Its section 2 says the schedule is "the subset of
edges at which an adaptation is applied" and does not say what constrains that subset. The observation
boundary is the constraint: an adaptation is **forced** on an edge whose value is observed and **free**
everywhere else. `60`'s schedule is the design's choice; the unobserved region is the space it may choose
in.

### R11 revised

R11 claimed three findings new to the live panel. Against `60` the honest list is shorter and I restate
it rather than editing R11, so the order in which I learned things stays visible.

**Stands.** The intermediate-width sweep in probe D, showing no threshold below `2F`, which sharpens
`60`'s grade taxonomy by saying there is nothing between grade b and grade a worth buying. The
backward-narrowing licence and its bit count, probes B and C, which the grep finds nowhere. The
doability check on D-C, probe H. The correlation finding in section 3.2, that worst-case accumulator
width is a fact about cancellation between terms and not about length, which `60`'s `ceil(log2 k)` states
as the worst case without separating the realised one.

**Narrowed.** Probe F's zero, which needs "no adaptation on the additive side" in its region and is
consistent with `60` rather than in tension with it. Its relative-precision half stands unchanged and its
consequence for Q41 stands.

**Withdrawn.** Probe G's carried residual as an answer for product chains, conceded to `60`'s window.

**Second instance rather than finding.** My three obligations, against `60`'s five. Probe D's headline,
against `60`'s grade a versus grade b. Probe E's saturating-add associativity row, against `60`'s
ring-homomorphism argument for wrapping and Q12's measured table, and already discounted in R1 for the
`satfold` contamination.

That is what a cold derivation is for. Four of my results survive contact with a file I had not read, two
are narrowed, one is withdrawn, and three are demoted to corroboration. The corroboration is worth having
precisely because it was derived blind, and the withdrawal is worth more than the file would have been
without the grep.

## R13. Coverage of phase two, restated after R12

**Additionally read in full:** `60` sections 1 through 10 and its phase-two header. **Skimmed:** the
outline of `43_rompf_what_a_composition_is.md`, not its body, so nothing here rests on it and it is a
named gap given that it is a prior dispatch of this persona on an adjacent question.

**Still not read:** every other numbered file, `DROPLIST.md`, `HANDLES.md`, `PRIOR_CALLS.md`,
`PERSONA_CALLS.md`, `164`, `161`, `165`, and the whole of `63`, `74`, `90`, `92` and `106`.

**The novelty grep is committed** at `167_probes/novelty/novelty.out` with the command shape that
produced it. It is a grep over the live panel root excluding my own files, it searches fifteen patterns,
and it is the instrument that found `60`. It bounds R12's claims and nothing more: a pattern absent from
it is absent from the filenames and bodies it searched, not from the panel.

---

## R14. Citations checked, by opening them

`167_probes/citecheck/`. Every citation in this file, both phases, opened at the location cited and
tested for content rather than for resolution. **27 citations, 0 failures, 2 of 2 negative controls
caught**, the controls being a citation to a file that does not exist and a citation to a real file at a
real line with text that is not there.

Two of my own citations failed on the first run and both turned out to be accurate: the quoted phrases
spanned a line break in `60`, so a raw substring match could not find them. The checker now normalises
whitespace, and the negative controls were rerun after that change and still fire. A checker that reports
false failures on wrapped quotations trains its reader to dismiss its failures, which is worse than not
having one.

Style checks on this file: zero em-dashes, zero en-dashes, zero exclamation marks in prose.
