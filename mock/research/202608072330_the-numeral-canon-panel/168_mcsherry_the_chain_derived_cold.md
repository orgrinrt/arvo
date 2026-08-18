# 168. What a chain is, and what composing operations owes that a single operation does not

**Frank McSherry. Cold derivation, phase one, written blind.**

Phase one was written and committed before reading any numbered panel file, `AGREEMENTS.md`,
`OPTIONS.md`, `DROPLIST.md`, any other member's probes, or the panel's commit log. Phase two is
appended afterwards, in a separate section and a separate commit, and phase one is not rewritten.

---

## 0. The two gates, before the assigned work

### 0.1 The canon gate: passed

Checked against `INTENTS.md` in full, including its "How to read an entry" section, and against
`RULES.md` in full.

There is no ratified canon to be misaligned with. `mock/canon/` does not exist
(`ls: mock/canon: No such file or directory`). Exactly one entry in the catalogue holds RATIFIED,
which is I13; I14 is IN FORCE by lints outside the panel; I18 is explicitly a rule of thumb; the rest
are STATED. I1 is demoted to OPEN on op's own word.

The assigned question is licensed directly by op. I7 is his, it is STATED, and it says of the
accuracy-first concern that it is to be "*accurate* and *precise*, especially within chains and ops,
not only alone" (`INTENTS.md:141`, entry I7 at `INTENTS.md:135`). A question about what a chain is, and what composition owes, is a
question about the term op used in his own statement of an intent. It is not misaligned and it is not
ambiguous in a way that needs handing back.

One thing I flag rather than resolve, because it is a gate observation and not part of my answer.
**I7's own wording contains an unresolved reading.** "Especially within chains and ops, not only
alone" can be read as (a) chains and single operations are two things, and precision applies within
both, or (b) "chains and ops" is one phrase naming composite computations, contrasted with "alone".
Section 5 below argues that under either reading the design needs one concept rather than two, so my
derivation does not turn on which it is. But a canon sentence that has to *quote* I7 will have to
pick, and picking is op's, not mine. I do not resolve it and I do not build on either reading.

### 0.2 The test gate: passed, and the suite is unusually good

Run crate by crate at `--release`, as the brief requires, on the twelve `-shared` crates other than
`bitpack-write-contend-shared`, which I did not touch.

```
cd mock/benches/variants && for d in bitpack-carrier-shared bitpack-contend-shared \
  bitpack-footprint-shared bitpack-plan-shared bitpack-shared bitpack-wide-shared \
  quantiser-fadd-shared quantiser-radix-shared satfold-shared warm-clamp-shared \
  warm-container-shared wide-rung-shared; do (cd $d && cargo test --release); done
```

All twelve build. **108 tests, 108 passed, 0 failed, 0 ignored.** By crate: bitpack-carrier 9,
bitpack-contend 12, bitpack-footprint 6, bitpack-plan 5, bitpack 3, bitpack-wide 6, quantiser-fadd 1,
quantiser-radix 3, satfold 11, warm-clamp 7, warm-container 15, wide-rung 30. The brief's four
non-building crates are not among the `-shared` set and I did not reach them.

**I read bodies, not names**, in full for `warm-clamp-shared` (7 tests) and `satfold-shared` (11), and
by grep across the rest for the failure shapes. There are no tautologies: `grep -rn 'assert!(true'`
returns nothing, and a regex for a value asserted against itself
(`assert_eq!\(([A-Za-z_0-9:]+), *\1 *[,)]`) returns nothing across the whole variants tree.

What is actually there is better than "not decorative", and it is worth naming because it is the
standard the rest of this panel's evidence should be held to:

- **Independent oracles.** `warm-clamp-shared`'s `reference` computes the declared semantics in `u128`
  sharing no carrier type and no code with any arm, and
  `every_arm_agrees_with_the_oracle_on_every_key` runs all six arms against it over all 46 declared
  keys rather than a chosen subset.
- **Real mutation testing.** `satfold-shared` declares four deliberately wrong kernels (`WrongOp`,
  `DropsALane`, `DropsTheRemainder`, `DropsOneElement`) and asserts the instrument catches each, at
  every length where the defect is expressible.
- **A pinned sensitivity boundary that admits a blind spot.**
  `the_one_element_defect_is_caught_up_to_1024_and_not_above_it` asserts the defect is caught up to
  L=1024 **and not caught at 4096**, with the reason in the comment. Its own doc records that an
  earlier version of that comment said 256, and that running the scoped assertion is what corrected
  it. That is a suite reporting the limits of its own instrument, which is rarer than a green run.
- **Anti-degeneracy checks that can fail.**
  `chunked_answer_depends_on_every_element_the_clamp_did_not_absorb` flips a bit in a chunk that does
  not clamp and asserts the answer moves, then flips one in a chunk that does and asserts it does not.
  `the_clamp_fires_on_a_real_fraction_of_chunks_at_every_chunked_key` refuses a run where the clamp
  fires on under 5% or over 95% of chunks.
- **Whole-matrix laws, not samples.** `clamping_is_a_retraction_on_non_negative_addition_at_every_swept_width`
  and `accfit_holds_the_exact_sum_at_every_swept_width_and_arity` both iterate the full swept matrix.

I have no findings against the suite and no work to refuse.

---

## 1. The short answer

**A chain is a maximal run of operations whose intermediate values are not observable.** It is
delimited by observation, not by syntax. Where nothing can look at an intermediate, the resolution
applied at that point is a *choice* the design gets to make; where something can, it is part of the
meaning and there is no choice.

**What composition owes that a single operation does not** is five things, and each is a thing that
provably has no per-operation answer:

1. **An answer.** Composing per-step-resolved operations does not in general compute the same function
   as the resolved composite. Where the two differ, the design must *name* which one is the answer
   before any realisation can be called an optimisation. I quantify how much is at stake (§4.1, p3).
2. **A carrier.** The intermediate value lives somewhere, and the narrowest place that holds it is a
   function of the whole sequence. The same operation needs different widths in different positions,
   so no per-operation rule can produce it (§4.2, p2).
3. **A licence, held conjunctively over every step.** Deleting an interior resolution requires a
   proof, there are at least two independent kinds of proof, and one non-conforming step anywhere
   revokes the algebraic kind for the whole chain (§4.3, p1).
4. **Three predicate dimensions a single operation does not have**: depth, shape and arity. Each is
   measured, on the committed harness, to flip which arm wins (§4.4).
5. **A boundary.** Someone has to say where the chain ends, because that is where the resolution
   fires and where the accuracy claim is made. The design has no way to express I7 without it.

**Accuracy across a chain is a property the design can hold**, and the mechanism is a const predicate
over the chain's own dimensions that licences replacing the chain's realisation with one that provably
agrees at the boundary. Three instances of exactly this shape are already measured on this
repository's harness, worth 23.9x, 33.1x and 177.8x where their predicates hold and 1.00x where they
do not.

**"Chain" is at least three things**, and a canon sentence true of one is false of the others (§5).
**And the chain is probably not the right unit**: the right unit is the region between two
observation points, of which a chain is the path-shaped case (§6). I open that as an option with what
would close it rather than settling it.

---

## 2. What a chain is

### 2.1 The derivation

arvo's world has two levels and everything below turns on keeping them apart.

A **numeral** is a declared thing: a width, a fraction, a signedness, a strategy. It denotes a set of
values `V(N)` and an operation on numerals denotes a function `V(N)^k -> V(N)`. That is the level a
consumer reasons at.

A **container** is a machine integer of some native rung. The emitted code computes a function on
container values. That is the level the machine runs at.

Between them sits a **realisation map**: a container value stands for a numeral value, and a program
on containers realises an operation on numerals when the two agree on the declared domain.

For a single operation the obligation is exactly one sentence: the realisation agrees with the
denotation on `V(N)`. It is local, it is checkable in isolation, and nothing outside the operation
bears on it.

Now compose. Write `pi` for the **resolution**: the map that takes an exact result back into `V(N)`.
Wrapping, clamping and rounding are all `pi` for different declared meanings. A single operation `f`
denotes `pi . f_exact`. So the naive composite of two operations denotes

```
    pi . g_exact . pi . f_exact
```

and the resolved composite denotes

```
    pi . g_exact . f_exact
```

These are different functions in general. That is the whole of it. **Composition creates a question
that does not exist for one operation: does the resolution fire in the middle?**

And the question only has force where the middle is not visible. If a consumer can name the
intermediate, store it into a column, compare it, or hand it across a typed boundary, then the
intermediate is a declared numeral in its own right, `pi` has already fired by definition, and there
is nothing to decide. If the consumer cannot, the intermediate is an artifact of how the computation
was written and the design owes only the value at the end.

So:

> **A chain is a maximal run of operations whose intermediates are not observable. Its boundary is
> where a value becomes observable. Inside it, only the boundary value is owed.**

The word "maximal" is doing work: a chain is not a syntactic bracket a consumer draws, it is the
largest region the design can prove nothing looks into. Making it smaller is always safe and always
costs something.

### 2.2 Why this definition and not a syntactic one

Three alternatives, and why each is worse.

**"A chain is a sequence of method calls."** Fails immediately: `a.add(b).mul(c)` and
`let t = a.add(b); t.mul(c)` are the same computation and would be different chains. Worse, it makes
the design's semantics depend on how the consumer spelled it, which is exactly the surprise I3 exists
to prevent.

**"A chain is an expression tree."** Better, but it draws the boundary at the wrong place: an
expression whose root is bound to a named variable that nothing else reads is still a chain by every
argument above, and an expression whose intermediate is stored is not.

**"A chain is a fixed-length pipeline the design provides."** This is the shape a library reaches for
when it wants chains to be tractable, and it fails I11. arvo is a library and does not know how
downstream will compose; a fixed pipeline vocabulary would be arvo choosing the compositions, which is
the policing `arvo-toolbox-not-policer.md` forbids and which I17's "the intent governs, not the
arbitrary amount that made sense back when it was written" warns about in a neighbouring case.

The observability definition survives all three because it names the *property* that makes the
question live, rather than a syntax that correlates with it.

### 2.3 One consequence worth stating immediately

Under this definition **a single operation is a chain of length one**, and it is not a special case
needing its own rules. Its degeneracy (§4.1) is trivially one, its carrier requirement is its own, its
licence is vacuous, and its depth is one. Every obligation below reduces correctly.

That matters for the canon, because it means the design does not need a chain concept *and* an
operation concept with a relation between them. It needs one concept whose short case is an operation.

---

## 3. What is carried along a chain, and what is discarded

### 3.1 Carried

Two kinds of thing, and they have different lifetimes.

**The value.** One container value, per step. This is the only thing that exists at runtime.

**The static facts.** The declared width, fraction, signedness and strategy of the endpoints; the
chain's own depth, shape and arity; and, derivable from those, a **bound on the intermediate**. All of
it is available at const time. Under I13 as op sharpened it, "the above collapses to whatever is
available at const time" (`INTENTS.md:252`), which is exactly the category these live in.

### 3.2 Discarded, and this is where the money is

**What is discarded at each step is the range.**

A step produces a value the design knows is in some interval. The projection back to `V(N)`
re-establishes membership and then the knowledge is *thrown away*: the next step loads a container and
the backend, having no access to the declaration, must assume the container's full range. The bench
tree already states this precisely, in a comment written for a different purpose:

> "The element bound is the declared width, which survives into the type and not into the loaded
> value: LLVM sees a load from a `u16` and must assume the full range. The count bound is the column
> capacity, which arvo carries as a `Cap` and which LLVM sees as a runtime slice length."
> (`mock/benches/variants/warm-container-shared/src/lib.rs`, `run_sat_widening` doc)

So the chain's opportunity is that the range **composes statically** while the value does not, and
that the compiler cannot do the composition because the premises never reach it. That is
microkernelling in the precise sense `small-wins-compound-into-the-program.md` describes: the typestate
holds a proof, the proof does not survive lowering, and the instruction the backend emits in its
absence blocks something much larger than itself.

### 3.3 And a second thing is discarded, which is not the range

The range is not the whole story, and it took a probe to make me stop believing it was. The clamp
retraction lemma the bench asserts,

```
    min(min(a + b, L) + c, L) = min(a + b + c, L)
```

holds **even where the bound is exceeded**. Nothing about a range licenses it; it is a fact about
`min` being a retraction and the accumulation being monotone. So there is a second discarded thing:
the **algebraic relationship between the resolution and the operators**, which the per-step form
destroys by interposing `pi` where the algebra says it does not belong.

This is §4.3, and separating it from the range is the single most useful thing in this file.

---

## 4. What composition owes

### 4.1 It owes an answer, and the size of the debt is computable

Where the resolved-per-step and resolved-once forms differ, there is no such thing as "optimising the
chain" until the design has said which function is the answer. Any realisation is an optimisation of
*something*; the question is of what.

That debt can be measured. For a chain of `D` steps there are `2^(D-1)` placements of an interior
resolution (the boundary one is not optional). Count the **distinct functions** those placements
compute, exhaustively over the declared domain. Call it the chain's **resolution degeneracy**.

- **Degeneracy 1**: every placement computes the same function. The design has nothing to decide and
  may pick the cheapest realisation on any grounds it likes.
- **Degeneracy > 1**: the placements disagree. The design must name the answer *first*.

`168_probes/p3_resolution_degeneracy.rs`, exhaustive over the whole domain at `W = 8`:

| chain | resolution | depth | degeneracy |
|---|---|---|---|
| `+k`, `*3`, `-k` (wrapping subtract) | wrap | 3 | **1** |
| the same, extended | wrap | 5 | **1** |
| `+k`, `+k`, `+k` | clamp | 3 | **1** |
| `+k`, `+k`, `satsub(L/2)` | clamp | 3 | **3** |
| `+k`, `*3`, `-k` | round to `2^3` | 3 | **3** |
| the same, extended | round to `2^3` | 5 | **7** |

Two things fall straight out. Wrapping's degeneracy is 1 **at every depth**, which is what "ring
homomorphism" cashes out to operationally. And rounding's degeneracy **grows with depth**: 3 at D=3,
7 at D=5. So for a rounding resolution the size of the decision is a function of the chain's length,
which is a quantity no operation has.

**Predicate for the table.** `W = 8, F = 0, signedness = unsigned, resolution in {wrap, clamp, round-to-2^3},
depth in {3, 5}, ops as listed, inputs exhaustive over the whole declared domain, threads = 1`.

### 4.2 It owes a carrier, and no per-operation rule can produce one

The intermediate lives somewhere. The narrowest container that holds it exactly is a property of the
**sequence**, and I established that exhaustively rather than by interval reasoning:
`168_probes/p2_the_carrier_has_no_local_answer.rs` runs every value of the declared domain at `W = 8`
and observes the maximum reached at each point.

```
  A: *3 then +200   per-step widths = [10, 10], chain needs 10 bits
  B: +200 then *3   per-step widths = [ 9, 11], chain needs 11 bits
```

Same two operations. Different container. And sharper: **`*3` needs 10 bits as the first step and 11
as the second.** There is no width to attach to `*3`, so a design that only ever names a
per-operation output type cannot express what the chain needs.

The controls that make this mean something:

- **Commuting steps must report the same requirement**, and do: `*3` then `*5` and `*5` then `*3` both
  need 12 bits. If every pair had differed, the instrument would be manufacturing order-sensitivity.
  Note it still reports different *per-step* widths, `[10,12]` against `[11,12]`, so the per-step
  requirement is order-dependent even where the chain requirement is not.
- **The width must be tight**, and is: `fits(10)=true, fits(9)=false` for A, `fits(11)=true,
  fits(10)=false` for B, `fits(12)=true, fits(11)=false` for C. Without this, "needs" would be the
  wrong word.
- **The spread is not an artifact of two cherry-picked orders**: all 24 orderings of one four-step
  multiset span **11 to 13 bits**.

**Predicate.** `W = 8, F = 0, signedness = unsigned, ops in {+k, *k} with k in {2,3,5,200,250},
depth in {2, 4}, inputs exhaustive over the whole declared domain, threads = 1`.

### 4.3 It owes a licence, there are two independent kinds, and it is a conjunction over steps

Deleting an interior resolution is a rewrite and needs a proof. There are at least two, and neither
subsumes the other. `168_probes/p1_two_licences_are_independent.rs` establishes it at `W = 13` over
4096 inputs, with four controls.

**(A) RANGE.** Every intermediate provably lies in the region where `pi` is the identity, so every
interior `pi` is a no-op. Depends on widths and a static bound. **Does not depend on which operations
the chain contains.**

**(B) ALGEBRA.** `pi` commutes with, or is absorbed by, the composition, so interior applications may
be deleted whatever the values are. Depends on the operations and the resolution. **Does not depend on
any bound.**

Independence, both directions, from the probe's output:

```
B-holds-A-fails  wrap, affine chain, 3637/4096 inputs exceed 2^W
  eager vs deferred: AGREE
A-holds-B-fails  round to 2^3, every intermediate on the grid
  eager vs deferred: AGREE
```

The first has 3637 of 4096 inputs leaving the declared range, so (A) is denied and the deletion is
still safe. The second is a rounding resolution, which is not a homomorphism at all, and the deletion
is still safe because every intermediate is exactly representable.

Four controls, all firing:

```
CONTROL  wrap, same chain with ONE saturating step     DISAGREE (0, 0, 5462)
CONTROL  round to 2^3, operand off the grid            DISAGREE (0, 10912, 10920)
CONTROL  clamp, mixed add/subtract chain               DISAGREE (6577, 4096, 7944)
CONTROL  wrap, chain containing a right shift          DISAGREE at x=4962
```

**The first of those is a result, not just a control, and I did not expect it.** It is the identical
affine chain with a single `saturating_sub` in place of the `wrapping_sub`. Nothing about the
endpoints changed, the width did not change, the depth did not change, and the licence is gone. So:

> **The algebraic licence is a conjunction over every step, not a property of the chain's endpoints.**

That has a direct design consequence. Whatever carries a chain's licence has to be able to see every
step, which means it is composed *as the chain is built* rather than computed at the boundary from the
endpoint types. A design that asks "what are the two ends" cannot compute it.

I found that control by writing the probe wrong: my first `affine` chain used `saturating_sub` and the
assertion that it must agree fired. The instrument caught my error, which is the argument for having
the assertion.

**A second thing I got wrong and kept.** My first clamp control put the subtraction in the middle, and
it **agreed**, because the boundary clamp absorbed the difference. A control that cannot fire is worth
nothing, and I record the near miss in the probe source rather than silently moving the step.

**Predicate.** `W = 13, F = 0, signedness = unsigned, resolution in {wrap, clamp, round-to-2^3},
depth = 3, ops in {+k, wrapping -k, saturating -k, *3, >>1}, 4096 inputs drawn across the declared
domain, threads = 1`.

### 4.4 It owes three predicate dimensions, and each is measured to flip the winner

A single operation has width, fraction, signedness, strategy and the operation. A chain has all of
those **and**:

**Depth.** How many steps compose. `satfold`'s committed sweep varies the reduction length `L` with
everything else held, and the crossover is sharp:

| `L` | family row | spread |
|---|---|---|
| 8 | `satfold-length-l1_n1000` | 1.09x |
| 15 | `satfold-length-l1_n2000` | 1.31x |
| 16 | `satfold-length-l1_n3000` | **6.85x** |
| 17 | `satfold-length-l1_n4000` | 6.53x |
| 256 | `satfold-length-l1_n10000` | 140.63x |
| 1024 | `satfold-length-l1_n11000` | **177.81x** |
| 4096 | `satfold-length-l1_n12000` | 163.58x |

The lane counts the arms use are 4, 16 and 64, and the length table deliberately brackets each with a
value one below and one above. Below the lane count the law-licensed rewrite buys nothing; at exactly
the lane count it buys 6.85x. **A chain-level rewrite has a fixed cost that only the chain's length
amortises**, and the threshold is a property of the rewrite rather than of the operation.

**Shape.** Serial against associative, and they are not variants of one thing. A serial chain has a
loop-carried dependence and can only be improved by *shortening* it; an associative fold can be
*reassociated* into lanes. The measured consequence is that the same design lever has opposite signs:

- Fold at W=13, arity 256 (`warm-clamp-arity-w13_n130080`): sizing the accumulator by the chain's own
  interior-safety rule wins, **-63.5%**, spread 51.52x.
- Elementwise chain of four steps at W=13 (`warm-clamp-chain-l1_n130001`): the same widening
  **loses**: `warm-clamp-accfit` at a 374 ns median against `warm-clamp-minimum` at 151 ns, a 2.47x
  spread across the field, and the findings file calls accfit "an outlier: 2.5x slower than the
  field". (`warm-clamp-min-lanes` ties `minimum` at 151 ns and is the same code at this op, which is
  the run's own noise control.)

Same width, same strategy, same resolution, opposite answer, and the only thing that changed is
whether the composition is a fold or a per-element chain.

**Arity.** For a fold, how many terms are combined. At W=13 the arity sweep moves the winner and the
spread continuously:

| arity | row | winner | delta | spread |
|---|---|---|---|---|
| 2 | `n130010` | `minimum` | -69.2% | 16.99x |
| 4 | `n130020` | `accfit` | -61.6% | 7.02x |
| 8 | `n130030` | `minimum` | -11.9% | 2.21x |
| 16 | `n130040` | `accfit` | -47.9% | 3.79x |
| 64 | `n130060` | `accfit` | -61.3% | 34.31x |
| 256 | `n130080` | `accfit` | -63.5% | 51.52x |

And the arity answer is itself gated by width, because at W=64 there is no rung between the minimum
container and `u128`, so the widening has no cheap form and the reassociating arm wins instead:
`warm-clamp-arity-w64_n640060` and `n640080` both go to `min-lanes`, at -51.4% and -59.7%.

**Predicate for the whole of §4.4.** `Apple M1, rustc 1.98.0-nightly (57d06900f 2026-05-27),
build profile = release, threads = 1, F = 0, signedness = unsigned, W as listed per row, arity and
reduction length as listed per row, element count as encoded in each key, resolution = clamp for the
warm-clamp rows and saturating addition for the satfold rows, harness = mockspace-bench-harness`.
Every meta.json in those families carries that cpu and rustc; `bench.toml` names
`../target/release/<variant>` for every arm, which is where the release profile comes from; and none
of `warm-container-shared`, `warm-clamp-shared` or `satfold-shared` mentions `std::thread`.

### 4.5 It owes a boundary

Trivial to state and easy to skip. The accuracy claim in I7 is a claim about a *value*, and the only
value a chain has is the one at its boundary. If the design has no way to say where a chain ends, I7
has nothing to attach to and cannot be expressed at all, let alone honoured.

This is the locus finding in §7 and I put it here so it is not read as an afterthought.

---

## 5. "Chain" is at least three things

A single canon sentence about chains would be false of two of these. They differ in what freedom they
have, what the loop-carried dependence is, and which lever helps.

**(1) The serial chain.** `v <- s_D(... s_1(v))`, per element, no value carried between elements.
Freedom: **fusion**, collapsing the steps into fewer. Elements are independent so it vectorises
freely, and the binding constraint is the lane count the intermediate carrier leaves. Measured
consequence: widening the carrier *costs*, because it halves the lanes.

**(2) The reduction.** `acc <- acc (+) x_i`. A value is carried across elements. Freedom:
**reassociation**, which needs `(+)` associative. Needs an accumulator whose width is a function of the
chain rather than of the operand type. Measured consequence: widening the accumulator *pays*, because
it deletes the interior resolution and turns a serial dependence into a vectorisable one.

**(3) The pipeline across a boundary.** A run of operations whose intermediates are stored, read, or
handed across a typed edge. **No freedom at all**, because every intermediate is observable and every
resolution is part of the meaning.

The third is the important one to name, because it is the one that gets called a chain in casual
speech and it is the one where treating it as a chain would be *wrong* rather than merely suboptimal.
A rewrite licensed for (1) or (2) and applied to (3) changes the answer a consumer can see.

The distinguishing question is the same for all three: **which intermediates are observable?** That is
why I want the definition in §2.1 to be about observation rather than about syntax; it is the one
question that separates all three cleanly, and it is the one a syntactic definition cannot ask.

**One I looked for and did not find.** A fourth candidate is the *fan-out* chain, where an
intermediate feeds two consumers. It is not observable in the §2.1 sense, since both consumers are
inside the region, but its carrier requirement is a join over consumers rather than a per-edge
property. I could not decide whether that makes it a fourth kind or an argument that the unit is
wrong. §6 takes the second horn and I open it as an option rather than settling it.

---

## 6. Whether the chain is the right unit at all

I do not think it is, and I am opening this rather than settling it.

The chain is path-shaped, and real computations are directed acyclic graphs. Everything in §4 that
made composition owe something was actually a consequence of one property, and that property is not
about paths:

> The obligations arise in a **region between observations**, not along a path.

Under that reading:

- A **chain** is the path-shaped region. The common case, and the one every measurement here is of.
- A **fan-out** is a region with a shared node. Its carrier requirement is the join over consumers,
  and its licence is the conjunction over all outgoing paths.
- A **single operation** is a region of one node, and its obligations degenerate to nothing, which is
  why it needs no special rules.

The word matters less than the delimiter. But if the canon says "chain", a reader implementing it will
build a path structure, and the fan-out case will arrive later as a surprise that does not fit. If it
says "region between observations", the path case is the easy instantiation and the graph case is
already covered.

**What would close this.** A probe that exhibits a fan-out region where the joined carrier requirement
strictly exceeds the maximum of the per-path requirements, *or* a proof that it never can. If it never
can, the chain is enough and the region framing is over-engineering; if it can, "chain" is the wrong
word in the canon and would have to be repaired later at design cost. It is one small exhaustive probe
and I did not build it, because I ran out of ground I could cover honestly and I would rather say so.

**What I explicitly do not propose.** I am not proposing a graph type, a builder, an expression DSL, or
any mechanism. Which of those (if any) carries a region is a design question and this is a canon
panel. What I claim is only that the *delimiter* is observation and the *extent* may not be a path.

---

## 7. Can accuracy across a chain be held by anything in the design

Yes, and the mechanism is already visible in this repository, though nothing there was built to be it.

### 7.1 What "accurate across a chain" has to mean

It cannot mean "each step is accurate", because §4.1 shows the composite of accurate steps is a
different function from the accurate composite, and §4.3 shows the difference can be large.

It has to mean: **the boundary value is close to the exact composite**. And once that is the target,
one placement is distinguished, and it is distinguished by a theorem rather than by taste.

`168_probes/p3_resolution_degeneracy.rs`, at rounding to `2^3`, `W=8`, `D=5`, exhaustive over the
domain:

```
  fully eager  (mask 0b1111): total |err| 7936, worst |err| 64
  fully defer  (mask 0b0000): total |err| 512,  worst |err| 4
  best of all 16 placements:  mask 0b0000, total |err| 512
```

A factor of 15.5 in aggregate error and 16 in worst case, from placement alone, at fixed width and
fixed operations.

And it is not merely the aggregate optimum. The reason is one line: **every placement ends with the
same boundary resolution, so every output is representable; and where that resolution is a
nearest-point projection, the deferred form outputs the nearest representable point to the exact
value by definition, so nothing can be strictly closer.** Checked rather than argued, over 3000
randomly generated chains of depth 2 to 5 over an alphabet including a contracting step (`>>`) and a
non-monotone one (`xor`):

```
  nearest (round to 2^3)   3000 chains searched, 0 with any eager win, 0 winning inputs
  nearest (clamp)          3000 chains searched, 0 with any eager win, 0 winning inputs
  NOT nearest (truncate)   3000 chains searched, 91 with any eager win, 1330 winning inputs
  e.g. [SatSubK(127), AddK(97), MulK(3), XorK(182)] at x=134: eager |err| 2 < deferred |err| 6
```

The truncation row is the control, and it is load-bearing: without it, "the search found no
counterexample" would be indistinguishable from "the search cannot find counterexamples". It finds
1330.

**So the design has a statement it can hold, and it is exact rather than aspirational:**

> Where the boundary resolution is a nearest-point projection onto the representable set, deferring
> every interior resolution to the boundary is pointwise optimal. There is no input, and no chain, on
> which any other placement is strictly closer to the exact composite.

That is what I7's "accurate and precise, especially within chains" cashes out to, and it is a property
of the *chain*, held by the *placement*, priced by the *carrier*.

**Predicate.** `W = 8, F = 0, signedness = unsigned, resolution in {round-to-nearest at grid 2^3,
clamp} for the positive claim and {truncate to 2^3} for the control, depth in 2..=5, ops in
{+k, *k, >>g, xor k, saturating -k}, 3000 chains, inputs exhaustive over the whole declared domain per
chain, threads = 1`.

### 7.2 What holds it, mechanically

A **const predicate over the chain's own dimensions**, licensing a rewrite that provably agrees at the
boundary. All three of the following are already measured on this repository's harness and all three
have that identical shape:

| licence | predicate | where it holds | where it does not |
|---|---|---|---|
| interior-safety of a fold | `W + ceil(log2 n) <= width(M)` | `warm-clamp-arity-w13_n130080`, -63.5%, spread 51.52x | `warm-clamp-arity-w64_n640080`, the widening arm loses |
| no-saturation theorem | `W + ceil(log2 n) <= 64` | `precise-widening-theorem-l1_n80501`, spread **33.13x** | `n600501`, spread **1.00x**; `n640501`, whole field within 4.3% |
| wrapping homomorphism | resolution is `reduce mod 2^W` and every step is ring-affine | `warm-affine-collapse-l1_n130403`, spread **23.95x**; `n80403`, 46.39x | revoked by one non-conforming step (p1's first control) |

The middle row is the one to look at, because the predicate's boundary is **visible in the committed
data**. `theorem_applies(w, n)` is `w + ceil(log2 n) <= 64`, and at n=8192 that is `w + 13 <= 64`, so
it is true at W=8,13,16,32 and false at W=60,64. The measured spreads are 33.13x, 7.80x, 8.52x, 8.15x,
then **1.00x**, then a 4.3% band. The predicate is not an argument about the design; it is a line in
the data.

That is I13 in the flesh at the chain level: "a bunch of arms with const predicates that optimize each
little 'sometimes' so that all 'sometimes' apply on that sometimes and nowhere else"
(`INTENTS.md:222-224`). And it is I15-compatible without strain: the predicate is const, so the unused
arm clears at lowering and no runtime check exists (`INTENTS.md:303-307`).

### 7.3 The honest limit

Where no predicate holds, the honest realisation is the per-step one and it is expensive, and that is
**correct** rather than a failure. `warm-clamp-arity-w64` is what that looks like: at W=64 the
interior-safety predicate cannot be satisfied by any native rung below `u128`, so no arm deletes the
clamps and the field compresses to 1.54x to 2.50x. The design's answer there is the slow one, and
saying so is the difference between a composition of arms and a claim that overreaches.

---

## 8. What I settled, what I moved, what I could not

### Settled

- **The definition.** A chain is a maximal run of operations whose intermediates are not observable
  (§2.1). Three syntactic alternatives are refuted (§2.2), and a single operation falls out as the
  length-one case (§2.3).
- **The carrier has no local answer**, exhaustively, with a tightness control and a commuting control
  (§4.2, p2).
- **Two independent licences**, with four firing controls (§4.3, p1).
- **The algebraic licence is a conjunction over every step**, so it cannot be computed from the
  endpoints (§4.3).
- **Deferral is pointwise optimal under a nearest-point boundary resolution**, checked over 3000
  chains with a control that finds 1330 counterexamples once the resolution stops being nearest-point
  (§7.1, p3).
- **Chain is at least three things** and a sentence true of one is false of the others (§5), with the
  fold/elementwise sign flip measured on the harness.

### Moved

- **"Accuracy across a chain" from a wish to a statement.** It has a precise form, it names a
  distinguished placement, and the theorem that distinguishes it is one sentence.
- **Depth, shape and arity from properties of a workload to predicate dimensions**, each with a
  committed measurement showing it flips the winning arm.
- **The chain question from a semantics question to a locus question.** §4.5 and §7: without a
  boundary the design cannot express I7 at all.

### Could not

- **The fan-out case.** §6. I state the option and what would close it; I did not build the probe. It
  needs one small exhaustive construction and I preferred to say so than to reason about it.
- **Fraction widths.** Everything here is `F = 0`. Rounding appears in p1 and p3 as a grid projection,
  which is the right *shape* for a fixed-point narrowing but is not the same object. Nothing in this
  file holds at `F > 0` and, under this panel's notation, that means it holds nowhere `F > 0` exists.
- **Signed.** Everything is unsigned. Clamping's retraction lemma is stated for non-negative
  accumulation, and I did not check what survives at signed. The workspace already records that
  distributivity results have moved on exactly this axis, so I decline to guess.
- **Whether a chain's licence can be *computed* rather than declared.** §4.3 establishes the licence is
  a conjunction over steps; whether that conjunction can be accumulated at const time as a chain is
  built is a mechanism question I did not attack. I13's "whatever is available at const time" suggests
  the category is right; nothing here shows the construction exists.
- **Everything about magnitudes I did not take from the harness.** My three probes are spikes. They
  establish existence, refute universals and count degeneracies. **They price nothing**, and no timing
  appears in any of them.

---

## 9. Options opened, each with what would close it

**O-168-1. Is the unit the chain, or the region between observations?** (§6)
*Closes:* an exhaustive probe on a fan-out region showing the joined carrier requirement either can or
cannot exceed the max over per-path requirements. If it cannot, "chain" is sufficient and the region
framing costs vocabulary for nothing. If it can, "chain" is the wrong word and the repair is cheap now
and expensive after designs exist.

**O-168-2. Does the design name the chain, or does it fall out?**
Two shapes. (a) The chain is a thing a consumer or the design names, and the boundary is explicit. (b)
The chain is implicit: every value is deferred by default and the boundary is wherever an observation
happens, computed rather than declared.
*Closes:* whether (b) can be given a const-time delimiter at all. If observation cannot be detected at
const time, (b) is not expressible under I15 and (a) is forced. That is a compiled-refutation question,
not a taste question.

**O-168-3. Is the accuracy statement in §7.1 the intent, or is it one reading of it?**
The statement "defer every interior resolution to the boundary" is what an accuracy-first concern
implies *if* the target is closeness to the exact composite. A different target, for instance
reproducing what a named reference implementation does step by step, would pick a different placement
and would be an equally coherent reading of "precise".
*Closes:* op. This is the intent question inside I7 and it is not an agent's to settle. I note that the
two targets **disagree by 15.5x in aggregate error** at D=5 in p3, so it is not a distinction without a
difference.

**O-168-4. Depth as an explicit predicate dimension in this panel's notation.**
`every-finding-carries-its-predicate.md` lists "chain length" among the nameable dimensions, and this
file uses it. `satfold`'s sweep shows a 1.09x-to-177.81x swing along it with everything else held. The
open part is whether the canon treats depth as a first-class dimension of the same rank as width and
strategy, or as a workload property.
*Closes:* one more instance on a different mechanism. Depth is currently established as a
winner-flipping dimension by `satfold` (a reduction) and, weakly, by p3's degeneracy growth (an
algebraic count, not a measurement). A third, on a serial chain rather than a fold, would settle it.

**O-168-5. Whether the three kinds in §5 are three, or two plus a non-chain.**
Kind (3), the observed pipeline, might be better named as "not a chain" than as a third kind, since it
has no freedom at all. Calling it a kind of chain invites someone to look for its optimisations.
*Closes:* whichever way the canon states §2.1's definition. If the definition is observability-based,
(3) is excluded by construction and naming it a kind is a category error. I lean that way and do not
settle it.

---

## 10. Coverage bound

**Read in full:** `INTENTS.md` (395 lines), `RULES.md` (646 lines), arvo's `.claude/CLAUDE.md`,
`mock/Cargo.toml` including its comments, `rust-toolchain.toml`, `mock/benches/Cargo.toml`,
`mock/benches/variants/warm-clamp-shared/src/lib.rs` (1177 lines, including all 7 tests),
`mock/benches/variants/warm-container-shared/src/lib.rs` doc header and all seven transform functions,
`satfold-shared`'s doc header and test module, `warm-clamp-chain-l1_n130001_findings.md` in full.

**Grepped or skimmed:** the remaining `.claude/rules/` files in arvo by name and size, `bench.toml`'s
header and its profile-bearing paths, the "Key findings" and "Fastest" lines of the
`warm-affine-collapse-l1`, `precise-widening-theorem-l1`, `warm-clamp-arity-*`, `satfold-length-l1`
and `satfold-const-gate` families, and every `-shared` crate's test names plus a tautology scan across
the whole variants tree.

**Did not open:** any numbered panel file, `AGREEMENTS.md`, `OPTIONS.md`, `DROPLIST.md`, `HANDLES.md`,
`PRIOR_CALLS.md`, `PERSONA_CALLS.md`, any `SEED_*`, anything under `archive/` or `seed/`, any other
member's probes, `git log`, any commit subject. The 90-odd bench families I did not name. The
`bitpack-*` and `wide-rung-*` and `quantiser-*` families beyond running their tests.

**Which sections would move if something I leaned on were wrong:**

- If the `warm-clamp` and `warm-container` shared crates' arms do not compute what their doc comments
  say, §4.4 and §7.2 move and the licence taxonomy in §4.3 does not, because that rests on my own
  probes. The suites in those crates check the arms against independent `u128` oracles over the whole
  declared key set, which is why I leaned on them.
- If the `satfold` length sweep's `L` decoding is not what `key_l` says, the depth table in §4.4 moves.
  I derived `n1000 -> L=8` from `L_TABLE` and the one-based `key_li`, and did not re-run the bench.
- If op's I7 means the second reading in §0.1, §7.1's framing of the target moves but none of §4
  does, because §4's obligations exist whatever the accuracy target is.
- **Nothing in this file rests on any other panel member's work**, by construction, which is both its
  value and its main risk: if a predecessor already established the carrier point or the licence
  split, I have spent a dispatch reproducing it. Phase two says which.

**A leak I have to declare.** The brief permits reading the variant crates, and their doc comments
cite prior panel conclusions by number and quote them, including a ratified preset table and two files'
verdicts. I read those comments because they are inside the permitted surface. So my phase one is
blind to panel *files* and is not blind to panel *conclusions that were transcribed into bench source*.
Concretely, I learned from `warm-clamp-shared`'s header that some prior file benched a container fork,
that `Warm`/`Cold` were once given clamp and `Hot` reduce-modulo, and that a prior file's saturating
fold was constant-folded. **None of my findings derives from any of that**: §2 is derived from the
premises, §4.2, §4.3, §7.1 are my own probes, and §4.4 and §7.2 are read off committed CSV and findings
artifacts rather than off the comments. But I would rather name the channel than claim a purity the
reading list does not actually give.

**Shared inputs with the other cold deriver of this pair.** We both read `INTENTS.md`, `RULES.md`,
arvo's `.claude/` rules, the auto-loaded workspace rules, and `mock/benches/`. Two of those bear
directly on my conclusions and I discount for them explicitly in phase two:
`every-finding-carries-its-predicate.md` supplies the notation *and* names "chain length" as an
available dimension, which is a large part of §4.4's framing arriving from a shared document rather
than from my derivation; and `small-wins-compound-into-the-program.md` states the
proof-does-not-survive-lowering mechanism that §3.2 uses. Where we agree on those, we are one instance
in two hats.

---

## 11. Every finding with its predicate, in one place

| # | finding | predicate |
|---|---|---|
| F1 | A chain is a maximal run of operations whose intermediates are not observable | definitional; no measurement, no predicate claimed |
| F2 | The narrowest exact intermediate carrier is a property of the sequence, not of any step; the same step needs different widths in different positions | `W = 8, F = 0, unsigned, ops in {+k, *k}, depth in {2,4}, inputs exhaustive over 0..2^8, threads = 1` |
| F3 | 24 orderings of one 4-step multiset span 11 to 13 bits | as F2, depth = 4 |
| F4 | Two independent licences (range, algebra) for deleting interior resolution; neither subsumes the other | `W = 13, F = 0, unsigned, resolution in {wrap, clamp, round-to-2^3}, depth = 3, ops in {+k, wrapping -k, saturating -k, *3, >>1}, 4096 inputs, threads = 1` |
| F5 | One non-conforming step revokes the algebraic licence for the whole chain | as F4 |
| F6 | Resolution degeneracy is 1 for wrapping over ring-affine steps at depth 3 and 5, and 1 for clamping over monotone non-negative additions at depth 3 | `W = 8, F = 0, unsigned, depth in {3,5}, inputs exhaustive over 0..2^8, threads = 1` |
| F7 | Resolution degeneracy for rounding grows with depth: 3 at depth 3, 7 at depth 5 | as F6, resolution = round-to-nearest at grid `2^3` |
| F8 | Full deferral is the pointwise optimum where the boundary resolution is a nearest-point projection, and is not where it is truncation | `W = 8, F = 0, unsigned, resolution in {round-to-nearest 2^3, clamp} positive and {truncate 2^3} control, depth in 2..=5, ops in {+k, *k, >>g, xor k, saturating -k}, 3000 chains, inputs exhaustive per chain, threads = 1` |
| F9 | Deleting the interior projection of a wrapping affine chain is worth 23.95x | `Apple M1, rustc 1.98.0-nightly (57d06900f), release, threads = 1, W = 13, F = 0, unsigned, depth = 3, n = 8192, resolution = reduce mod 2^W, harness = mockspace-bench-harness` |
| F10 | The no-saturation theorem is worth 33.13x where its predicate holds and 1.00x where it does not | as F9, W in {8, 60}, n = 8192, resolution = saturating add, accumulator = 64 bits |
| F11 | A reassociation law's value on a fold is ~1x below the lane count and 6.85x at it, rising to 177.81x | as F9, W = 8, resolution = saturating add, reduction length in {8,15,16,17,256,1024,4096}, column = 32768 elements |
| F12 | Widening the intermediate carrier pays on a fold and loses on an elementwise chain, at the same width and resolution | as F9, W = 13, resolution = clamp, fold arity = 256 against elementwise depth = 4, n = 8192 |
| F13 | The arity of a fold moves both the winning arm and the spread, from 16.99x at arity 2 to 51.52x at arity 256 | as F9, W = 13, resolution = clamp, arity in {2,4,8,16,64,256}, n = 8192 |
| F14 | At W = 64 no native rung satisfies interior safety, so the widening arm loses and the reassociating arm wins | as F9, W = 64, resolution = clamp, arity in {64, 256}, n = 8192 |

F9 through F14 are read off committed harness artifacts in `mock/benches/`; I did not re-run the
harness and I claim nothing beyond what those files record. F2 through F8 are from
`168_probes/`, committed with their output before this file cited them, and they are
spikes: they establish and refute, and **they price nothing**.

---

# Phase two: reconciliation

Written after reading the curated list. Phase one above is unedited and its four citation line
numbers, which I corrected by opening them, were corrected before it was committed.

**Read in phase two, in full:** `166`, `109` section 8, `110` sections 3 to 5 (its P7 and P8 material),
`112` section 8, `164` sections 1 to 4, `113`, `AGREEMENTS.md` sections 6 and 12, `117`, `63` section 5
(reached because `AGREEMENTS.md` section 6 attributes the panel's chain result to it and the register
pointed at `60` through it), and `OPTIONS.md` entries Q11, Q12, Q16, Q42, Q54, Q55. **Grepped:** the
panel for every bench family I cite, to find who had read each first. **Did not open:** everything
else, which is most of the panel.

## 12. The largest thing I found, and it changes one of my own findings

`117` records that **every number in `mock/benches/` was taken at cargo's default release profile**,
`lto = false` and `codegen-units = 16`, not the fat-LTO one-codegen-unit profile the harness documents,
and says plainly: "Anyone about to cite a bench number from this directory should establish that
first." I cite six of them. So I established it, and attacking that blocker produced the best result in
this file.

I confirmed `117`'s premise independently: `grep -n '\[profile' mock/benches/Cargo.toml
mock/benches/variants/*/Cargo.toml | wc -l` returns **0**, and `mock/benches/.cargo` does not exist.

**First correction, to my own phase one.** F9 says "deleting the interior projection of a wrapping
affine chain is worth 23.95x" and section 4.4 attributes it to the interior projection blocking the
algebraic collapse of the affine chain. `168_probes/p4` refutes that attribution. I built a chain whose
interposed right shift makes the affine collapse *impossible*, expecting it to show no difference
between the eager and deferred forms. **It shows the same difference**, at both profiles:

```
  affine_deferred      vectorised=YES  vec_operands=22
  affine_eager         vectorised=no   vec_operands=0
  blocked_deferred     vectorised=YES  vec_operands=32
  blocked_eager        vectorised=no   vec_operands=0
```

So the mechanism is not the collapse, because the effect is present where the collapse cannot happen.
The bench family's own title ("what the interior projection prevents the optimiser from doing") invites
the reading I took, and it is the wrong one.

**Second, the mechanism isolated.** `168_probes/p5` separates the projection on the per-element *value*
from the projection on the loop-carried *accumulator*, at `W = 13` in a `u16` container, matching the
`minimum` arm whose eager and deferred forms are that family's 9380 ns and 405 ns rows. Read post-LTO
out of a linked cdylib, which is what the harness builds:

```
  control_plain_sum    vectorised=YES   (no projection anywhere)
  control_indexed_sum  vectorised=YES
  both_deferred        vectorised=YES
  value_eager          vectorised=YES   projection on the value only
  acc_eager            vectorised=no    projection on the accumulator only
  both_eager           vectorised=no    folded with acc_eager, same address
```

**The projection on the loop-carried accumulator is what blocks vectorisation. The projection on the
per-element value costs nothing at all**, and `acc_eager` and `both_eager` land at the identical
address, so once the accumulator projection is present the value projections contribute literally
nothing. LLVM eliminates them itself by the same homomorphism arvo would use.

**Third, and this is what discharges `117` for my citations: the result is identical at both codegen
profiles.** Byte-for-byte the same disassembly at `lto=off, codegen-units=16` and at `lto=fat,
codegen-units=1`. So the *mechanism* behind the family is profile-invariant. The *magnitude* is not
established at the documented profile and I do not claim it is; that needs `117`'s before-and-after
harness run and nothing else.

**Fourth, three instrument defects, each caught by a control and each recorded in the probe rather than
edited away.** They are worth listing because all three produced a confident wrong number and none was
visible without the control:

1. Counting `mul`-class instructions returned **zero on every arm**, because LLVM strength-reduces `*3`
   and `*5` into shift-adds. A counter that cannot return a nonzero is not an instrument.
2. Hardcoding the loop label `LBB*_2` reported a zero-instruction loop for two arms, because LLVM
   numbers labels differently per profile.
3. Matching vector registers as `v[0-9]+\.` matched **nothing**, because that is ARM's assembler syntax
   and Apple's puts the element form on the mnemonic (`add.8h v0, v4, v0`). This reported a fully
   vectorised function as scalar, and it is the one that would have shipped a false finding: I had
   already written down "neither form vectorises" before the positive control fired.

And a fourth, about method rather than about a pattern: **`--emit asm` under `-Clto=fat` reports the
pre-LTO module**, in which nothing is vectorised including the control. An earlier run of p5 concluded
from that "the documented profile suppresses vectorisation entirely", which is a statement about the
instrument. Disassembling the linked image is what fixed it.

**Predicate for the p4 and p5 results.** `W = 13, F = 0, signedness = unsigned, container = u16,
resolution = reduce mod 2^W, chain depth = 3 (p4 affine) and 5 (p4 blocked) and 3 (p5), operations in
{+k, *3, -k, >>1}, host = Apple M1, rustc 1.98.0-nightly (57d06900f 2026-05-27), codegen profiles both
of {opt-level=3 lto=off codegen-units=16} and {opt-level=3 lto=fat codegen-units=1}, crate-type cdylib,
threads = 1`. This is an **ad-hoc quick spike** for any question of magnitude and it prices nothing; it
establishes presence and absence of vector code and nothing else.

### 12b. The predicate amendment F9 through F14 need

Phase one's predicate for the harness findings says `build profile = release`. That is true and it is
not exact, and this panel retired a true finding once already because a profile dimension was stated at
the wrong granularity. The exact form, established by the two commands above and by `117`:

> `codegen profile = cargo default release: opt-level = 3, lto = false, codegen-units = 16`

Phase one is not edited, per the rule that a predicate is never rewritten in place. **The amendment
lives here and the consolidation carries it**, and it is a narrowing rather than a widening: F9 through
F14 hold in a smaller region than "release" names, not a larger one.

One consequence worth stating, because `117` names it and it is not about speed.
`codegen-units = 16` partitions non-deterministically across builds, so two runs of the same unchanged
variant can differ in inlining and layout. That is the contamination the harness's per-variant cdylib
isolation exists to prevent, arriving through a door nobody was watching. It does not make any
committed number wrong; it means no committed number in that directory is reproducible by construction,
and a re-run is the only way to find out whether a given one is.

## 13. Where I agree, and what that does to each rung

I derived phase one blind, so where I land on a prior claim it is a genuine second instance rather than
a read, with two exceptions I name below. **Carrying forward unchanged, ten items:**

**1. `63` section 5, via `60`: "a chain is a composition of exact operations together with a schedule of
adaptation points, and the schedule is part of the function's meaning, because two schedules over the
same ops compute different functions."** This is my section 4.1 arrived at independently, and its
vocabulary is better than mine. **I adopt "schedule" over my "placement"** and I keep their sentence.
`60_probes/p_a` shows three schedules giving three functions; my `p3` gives the same fact a **count**,
the resolution degeneracy, exhaustive over the declared domain: 1 for wrapping over affine steps at
depth 3 and 5, 1 for clamping over monotone non-negative additions, 3 for clamping mixed, and 3 rising
to 7 for rounding as depth goes 3 to 5. Two independent instances, and mine adds the quantity.

**2. `63` section 5: "a concept that closes its operations over the format, adaptation fused invisibly
into each op, can state stepwise correctness and nothing above it, so I7's chain clause has no
expressible form against it."** This is my sections 4.5 and 7 exactly, derived independently, and it is
the sharpest sentence on chains in the panel. Second instance, carried unchanged. `63` marks it ONE
EXPERT, cold, unattacked; on my derivation it is TWO EXPERTS, both cold, neither having read the other.

**3. `109` section 8: "the intermediate width is a function of the chain, which no per-value type
knows."** My section 4.2, derived independently, exhaustively, with a tightness control. Second
instance.

**4. `109` section 8: the deferred route is never worse than the per-step route.** Measured there over
3200 multiplicative chains at `F = 8`. Second instance from me, and see section 14 for the
strengthening.

**5. `20` section 3.2 on `precise-widening-theorem-l1`: "the theorem does not make the loop cheaper, it
changes which loop is compiled."** `20` read that family before me and I did not know. My F10 is
therefore a **re-read of the same artifacts, not a second instance**, and I say so; `20`'s per-arm
ratios (32.3x at W=8, 1.00x at W=60) and my spread figures (33.13x, 1.00x) are the same numbers cut
differently. What p5 adds is a **second instrument** on the mechanism sentence, which `20` took from
the bench's own doc comment: I show it by disassembly, with a positive control, at both codegen
profiles, and for the *masking* accumulator rather than the saturating one.

**6. `112` section 8 and `110` P7/P8: a construction's soundness under composition is not
componentwise**, and only the denotational sameness relation is a congruence (`164` clause 2). This is
the structural twin of my F5, that the algebraic licence is a conjunction over every step rather than a
property of the endpoints. Different objects, same shape, and I read `112` after deriving F5. Carried
unchanged.

**7. `92` and Q42: each vectorised arm is at parity with the fold as written below its own lane count
and first pays above it.** My F11 reads the same committed `satfold-length-l1` artifacts, so it is a
**re-read, not a second instance**, and Q42's crossover figures are the authoritative ones. My reading
agrees. Q42 also corrects something I would have got wrong: `lanes4-idx`, which emits no vector
instruction at all, is faster at every length by up to 14.51x, so "vectorised or not" is not the whole
cost model even for this family.

**8. `110` section 4: "composition" carries two jobs, and configuration is not composition.** Carried
unchanged and extended in section 15.

**9. Q12, from `35`: unsigned wrapping, unsigned saturating and signed wrapping folds are exactly
reassociable, and signed saturating folds are not, with 70.1% of vectors at n=8 depending on the
split.** This fills my "could not: signed" honestly and I carry it unchanged.

**10. `117`, in full.** Carried unchanged and acted on rather than reported.

## 14. Where I add something, or disagree

**A. `109` names its own most obvious attack and my p3 closes it, in the strengthening direction.**
`109` section 8 says its result "does not extend to rounding-to-nearest, which I did not test and where
the per-step errors would partly cancel; that is the most obvious attack on this result and I am naming
it rather than waiting for someone else to."

They do not cancel, and the reason is one line. **Every placement ends with the same boundary
resolution, so every output is a representable point; where that resolution is a nearest-point
projection, the deferred form outputs the nearest representable point to the exact value by definition,
and nothing can be strictly closer.** So deferral is not merely never worse in aggregate, it is
**pointwise optimal**, and rounding-to-nearest is the case where that is easiest to see rather than the
case that breaks it.

Checked rather than argued, over 3000 randomly generated chains of depth 2 to 5 over an alphabet
including a contracting step and a non-monotone one:

```
  nearest (round to 2^3)   3000 chains, 0 with any eager win, 0 winning inputs
  nearest (clamp)          3000 chains, 0 with any eager win, 0 winning inputs
  NOT nearest (truncate)   3000 chains, 91 with any eager win, 1330 winning inputs
```

The truncation row is the control and it is load-bearing: without it "found no counterexample" and
"cannot find counterexamples" are the same output. **And it locates where the theorem stops**: the
property belongs to the resolution being a nearest-point projection, not to chains. A truncating or
directed rounding does not get it.

**B. A narrowing of `63`/`60` and of Q11's third option, and I overstated it before checking.**
`63` section 5 says the multiplicative window's "capacity a static function of container width and
operand formats". Q11's third option says "since capacity is a type, the accumulator is derivable as
the width plus the log of the capacity".

**Both are correct for what they are about**, and I want that said first because my phase one implies
otherwise. `63`'s window is a chain of multiplications, where the bound is the product of the operand
maxima and the order genuinely does not move it. Q11's is a fold of one associative operation, where
`W + ceil(log2 n)` is exactly right and is what `warm-clamp-shared`'s `accfit` implements. Neither is
refuted by anything I have.

**What I establish is that the formula does not extend to a heterogeneous chain**, and the load-bearing
evidence is not the one I led with. `168_probes/p2`, exhaustive over the whole declared domain at
`W = 8`:

```
  C: *3 then *5   per-step widths = [10, 12], chain needs 12 bits
  D: *5 then *3   per-step widths = [11, 12], chain needs 12 bits
```

`C` and `D` compute the **same function**, `15x`, from the same multiset of genuinely commuting steps,
and need the same **chain** width. Their **per-step** requirements differ, `[10,12]` against `[11,12]`.
So even in the most favourable case available, there is no width that belongs to `*3`: it needs 10 bits
in one position and 11 in the other, and no rule keyed on the operation and its operand formats can
produce both.

The pair I led with in phase one, `*3 then +200` needing 10 bits against `+200 then *3` needing 11, is
weaker than I made it sound, and I am correcting my own phase one rather than letting it stand: those
two are **different functions**, `3x+200` and `3x+600`, so it is unsurprising that they need different
containers. It is evidence that the requirement is not a function of the step multiset, and it is not
evidence that order alone moves it. The commuting pair is.

And the spread is real rather than a two-bit curiosity: all 24 orderings of one four-step multiset span
11 to 13 bits, with tightness verified in both directions (`fits(w)` true, `fits(w-1)` false).

So the capacity is a static function of **the ordered sequence**. It is still static, so nothing about
I15 or I13 changes and Q11's third option survives for the case it names; what does not survive is
using that parameter list for a chain of mixed operations.

**And this is a different claim from `60_probes/p_b`, which I checked before writing it.** That probe
establishes that the **value** of a per-step saturating fold depends on the order, which is the
schedule-is-semantics result. Mine establishes that the **width required to stay exact** depends on the
order, on chains where no resolution fires at all. Two different order-dependences; neither implies the
other.

**C. The two-licence split, which I do not find anywhere in what I read.** Deleting an interior
resolution needs a proof, and there are at least two independent kinds: **range** (the intermediate
provably stays where the resolution is the identity; width-dependent, operation-independent) and
**algebra** (the resolution commutes with or is absorbed by the composition; operation-dependent,
range-independent). `168_probes/p1` exhibits each holding where the other fails, with four firing
controls.

This bears directly on `109`'s proposed resolution. `109` proposes that `Mul` stop being an
endomorphism, so "the chain story falls out of the typing without any chain policy existing anywhere".
**That handles the range licence and not the algebraic one.** Widening the result type is exactly a
range argument. It does nothing for the case where the resolution's algebra is what licenses the
deletion, and nothing for the case where **no wider type exists**, which is measured:
`warm-clamp-arity-w64` has no native rung between the minimum container and `u128`, so the widening arm
loses and the reassociating arm wins at -51.4% and -59.7%. A typing-only story has no arm there.

So I would keep `109`'s proposal and say it is half of the mechanism rather than all of it.

**D. The algebraic licence is a conjunction over every step.** `168_probes/p1`'s first control: the
identical affine chain with a single `saturating_sub` in place of the `wrapping_sub`, same endpoints,
same width, same depth, and the licence is gone. So whatever carries a chain's licence must see every
step and be composed as the chain is built. A derivation that asks "what are the two ends" cannot
compute it. I found this by writing the probe wrong and having the assertion fire.

**E. The fold and the elementwise chain take opposite answers to the same design lever, measured.**
Widening the intermediate carrier pays on a fold (`warm-clamp-arity-w13_n130080`, accfit at -63.5%,
spread 51.52x) and loses on an elementwise chain of four steps at the same width and resolution
(`warm-clamp-chain-l1_n130001`, accfit at a 374 ns median against minimum at 151 ns). `40` cites
`warm-clamp-chain-l1` only to say the instrument for its own question does not exist; nobody had read
these two families against each other. Under p5's mechanism the reason is now clear: the fold has a
loop-carried value whose projection blocks vectorisation, and the elementwise chain has none, so
widening buys nothing there and costs the lane count.

**F. `warm-affine-collapse-l1` had not been read at all.** `93` names the family and says explicitly
"I have not read whatever concluded from that bench" (`93:534`); a grep of the panel finds no other
file naming it. So section 4.4's figures for it are a first reading, and section 12's mechanism
correction is a correction to the family's own title rather than to any member.

## 15. "Composition" is now three words wearing one, and the register should say so

`110` section 4 separates two senses, **configuration** (choosing a point in a parameter product) and
**composition of algebras** (a construction taking a base primitive to a composite: product, complex,
dual, interval). Q16 records the same overload from `43` and calls it op's to settle.

**Neither of those is the sense this unit's question uses.** "What does composing operations owe" is
composition of *operations* into a chain, which is a third object: it produces no new primitive, it has
a depth and a shape and an arity, and its obligations are the five in section 4.

That matters for the consolidation, because the tenth unit is briefed as "the chain and the composite"
and those are two different senses under one brief. A canon sentence about "composition" will be true
of at most one of the three. I am not proposing names, per I17's separation of intent from vehicle and
because Q16 is already op's; I am saying the count is three rather than two and that Q16's option list
is missing one.

## 16. What this does to my open options

**O-168-1 (chain or region) is Q54, arrived at from a different direction.** Q54 asks whether consumer
terms are trees or DAGs, reached from leaf-occurrence conditions in the realisation-map topic. I
reached it from carrier joins over a fan-out node. Independent arrivals on the same question, which
raises its priority, and my route supplies a **cheaper closing procedure** than Q54's current one:
Q54 says it needs "a statement of what the consumer-facing term type is, which is a design question at
the tier above". A carrier-join probe does not: exhibit a fan-out region whose joined carrier
requirement strictly exceeds the maximum over per-path requirements, or show it never can. That is one
small exhaustive probe and it settles the modelling question without settling the design question.

**O-168-2 (does the design name the chain, or does it fall out) is `63`'s D-A / D-B / D-C, which that
unit killed none of.** D-C, "the chain as a first-class typed object", is my shape (a); D-A, "closed ops
with chains entirely elsewhere", is close to my shape (b). I withdraw O-168-2 as a new option and
restate my contribution to it: **the const-time delimiter question is the discriminator between them**,
and it is a compiled-refutation question rather than a taste one.

**O-168-3 (which accuracy target I7 means) survives and is sharper.** It is `63`'s note that D-A
"survives only under a reading of I7 that its quoted words do not favour, which only op can rule on".
My p3 adds the price of the distinction: 15.5x in aggregate error and 16x in worst case at depth 5
between the two placements, so it is not a distinction without a difference. And section 0.1's
observation about I7's own wording ("chains and ops" as two things or as one phrase) is a second
unresolved reading in the same sentence.

**O-168-4 (depth as a first-class predicate dimension) is answered better than I asked it.** Q42
already carries the measured crossovers with predicates, `106` already states "chain length is a region
dimension", and `every-finding-carries-its-predicate.md` already lists chain length among the nameable
dimensions. I withdraw the option and carry the answer.

**O-168-5 (are the three kinds three, or two plus a non-chain) survives**, and `63`'s vocabulary
improves it: my kind (3), the pipeline whose intermediates are stored, is **a chain whose schedule is
fully determined**. That is better than calling it a third kind, and it makes the definition in
section 2.1 and `63`'s definition the same definition.

**One new option, from section 14 B.**

**O-168-6. What the accumulator derivation's parameter list is, once the chain is heterogeneous.**
Q11's third option and `63`'s window each derive the intermediate capacity from widths, formats and a
capacity, and each is correct for the shape it is about: one associative fold, and a chain of
multiplications. `p2` shows the parameter list is insufficient once the chain mixes operations, because
the per-step requirement is then order-dependent even for genuinely commuting steps. The
options are: restrict the derivation to single-operation folds and say so; take the ordered chain as
the derivation's input; or take a conservative bound over all orderings, which `p2` prices at 13 bits
against 11 for the four-step multiset it swept, a two-bit over-allocation that at some widths crosses a
container rung.
*Closes:* whichever of the three the design takes is a canon-level call, but the **cost** of the third
is a measurement, and it is the cheapest of the three to establish: sweep orderings across the width
range and report where the conservative bound crosses a rung.

## 17. Coverage of phase two, bounded

**What would move if I am wrong about something I leaned on.** Section 12 rests on `117`'s account of
the profile, which I re-derived with two commands rather than taking on trust, and on my own probes.
Section 13 items 5 and 7 rest on `20` and `92` having read the artifacts I re-read; if their readings
are wrong, mine agrees with them and is wrong the same way, which is the shared-drift risk and it is
why I marked both as re-reads rather than instances. Section 14 B **narrows** a sentence in `63` and a sentence in
Q11 rather than contradicting them, and it says so only because I checked `60_probes/p_b` before
writing it and found my first framing overstated: the pair I had led with computes two different
functions, so it could not carry the claim I hung on it. The commuting pair carries it and the
overstatement is recorded in place rather than edited away. If the two sources intend only the shapes
they name, which on my reading they do, then B is a boundary on their scope and not a defect in them.

**What I did not do.** I did not read `60` itself, only `63`'s account of it, which is exactly the
single-point-of-failure shape `RULES.md` names: my items 1 and 2 both reach `60` through `63`. `63` is
a consolidation and consolidations lose things. Someone should read `60` directly against those two
items. I did not read `35`, `43`, `92` or `20` beyond the passages quoted. I did not run the harness. I
did not attack the composite sense of composition at all, which is half of this unit's brief.

**Shared inputs with `167`, the other cold deriver of this pair.** We both read `INTENTS.md`,
`RULES.md`, arvo's `.claude/` rules, the auto-loaded workspace rules and `mock/benches/`. Two of those
bear on my conclusions and I discount for them: `every-finding-carries-its-predicate.md` supplies both
the notation and the phrase "chain length" as a nameable dimension, so section 4.4's framing of depth
as a predicate dimension arrived partly from a shared document rather than from my derivation; and
`small-wins-compound-into-the-program.md` states the proof-does-not-survive-lowering mechanism that
section 3.2 uses and that p5 then measures. Where `167` and I agree on those two, we are one instance
in two hats. I have not read `167` and am not counting any agreement with it.

**The leak I declared in section 10 stands.** The bench variant crates' doc comments quote prior panel
conclusions, and reading them was permitted and unavoidable. Having now read the panel, I can be
specific about what it cost: `warm-clamp-shared`'s header told me a prior file had benched a container
fork and that a prior saturating fold was constant-folded, and `warm-container-shared`'s header told me
the affine-collapse story including its conclusion. **That last one is the leak that mattered**, and it
is also the claim section 12 refutes, so the leak handed me a wrong conclusion which my own probe then
overturned. I would rather report that than claim the channel was harmless.

---

## 18. O-168-1 attacked rather than left, and phase one guessed the wrong quantity

Phase one opened O-168-1, said it needed one small exhaustive probe, and did not build it. `RULES.md`
is explicit that a blocker reported and left is not a deliverable, so I built it:
`168_probes/p6_a_fanout_forces_one_schedule.rs`, exhaustive over the whole declared domain at `W = 8`.

**Result one, and it closes the question I actually asked, negatively.** I asked whether a fan-out
node's joined carrier requirement can strictly exceed the maximum over its per-path requirements. **It
cannot, and the reason is trivial once stated: a node's value is one value, so its width requirement is
one number, and a join over consumers of one number is a maximum.** Measured rather than argued, on the
construction below: `t` needs 10 bits, branch A 20, branch B 8, the output 20, joined requirement 20,
max over the two per-path requirements 20. Equal, as it must be.

So my phase one guessed the wrong quantity, and Q54's carrier reading, insofar as my section 6 supplied
one, is closed with nothing in it.

**Result two, which is what is actually there.** A shared node has exactly one **schedule**, because it
is one value, and its consumers may want different ones. The construction:

```
    t   = 3x + k        the shared node
    a   = t * t         branch A: squaring, so it needs t narrow
    b   = t >> 2        branch B: contracting, so it wants t wide and exact
    out = a xor b       the boundary
```

At a 16-bit carrier, branch A needs 20 bits with `t` left exact and 16 with `t` resolved, so **branch A
forces the resolution**: the region is not realisable at all otherwise. Branch B is strictly worse for
it, on **203 of 256 inputs**, total extra absolute error 15504 and worst extra 152 against the schedule
it would pick alone.

**And no path-shaped analysis reports that loss**, because along each path in isolation the schedule
chosen is the best one available to that path. The cost appears only where the paths meet, which is
exactly the object a chain does not have.

**The control.** A second DAG whose branches are `t >> 1` and `t >> 2`, both fitting the carrier with
`t` exact, so neither forces a resolution and the region may leave `t` alone. Asserted, and it holds. I
kept the control's own numbers visible and labelled them as not the finding: resolving `t` hurts any
branch, and the finding is that in the first DAG nothing else could be done.

**What this does to O-168-1 and to Q54.** It does not settle whether the canon says "chain" or "region",
which is a naming call and not mine. It settles what is at stake in the answer, and the stake is not
the one I named:

- A design that models only paths gets the **carrier** right for a DAG by construction, since the max
  is the join.
- It gets the **schedule** wrong, silently, whenever two consumers of one intermediate disagree, and
  the disagreement is forced by carrier capacity rather than by anything either consumer says.

**Predicate.** `W = 8, F = 0, signedness = unsigned, carrier = 16 bits, resolution = nearest-point
projection onto [0, 2^W) (the clamp), region shape = one shared node with two consumers and one
combiner, operations in {3x+k, t*t, t>>2, t>>1, xor}, inputs exhaustive over the whole declared domain,
threads = 1`.

**And a residue I am marking as a residue rather than a proposal.** The conflict has exactly three
resolutions and all three cost something: materialise `t` twice in two carriers, paying the recompute
or the storage; resolve `t` and let branch B lose what p6 measures; or refuse the region and make the
consumer split it. Which one a design takes is a call I am not making, and I note only that the third
is the one that keeps the loss visible, which is the property `strict-by-design-quality-pressure.md`
values and the reason I would not want the first two chosen silently.

## 19. What is left that I could not attack

**The composite sense of composition**, which is half this unit's brief. I read `110` P7/P8 and `112`
section 8 and have nothing to add to them. Everything in this file is about composition of operations.

**`F > 0` and signed, still.** p1 and p3 model rounding as projection onto a `2^g` grid, which is the
right shape for a fixed-point narrowing and is not the same object, and every probe here is unsigned.
Q12 supplies the signed fold answer from `35` and I carry it; nothing else here extends.

**The magnitude of anything at the documented codegen profile.** Section 12 establishes the mechanism is
profile-invariant and explicitly does not establish that any number is. `117`'s before-and-after harness
run is the only thing that does, and it is a workspace-level fix rather than a panel dispatch.

**Whether a chain's licence can be accumulated at const time as the chain is built**, which section 14 D
makes necessary and which I did not attempt. I13's "whatever is available at const time" says the
category is right. Nothing here shows the construction exists, and a compiled refutation either way
would be worth more than another measurement.

**`60` read directly.** My two strongest agreements reach it only through `63`, which is a
consolidation, and `RULES.md` names that exact shape as a single point of failure. Someone should open
`60`.
