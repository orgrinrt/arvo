# 110. The primitive, derived cold

**Phase one. Written blind.** Read before writing: `INTENTS.md`, `RULES.md`, the shipped tree
(`mock/crates/` is empty, `mock/Cargo.toml`, the two bannered root templates), `mock/benches/` and its
variant crates. No panel file, no consolidation, no `OPTIONS.md`, no `DROPLIST.md`, no `SEED_*`, no
`PRIOR_CALLS.md`, no git log, no commit subjects.

Probes in `110_probes/`, each committed as it ran, before this file was written.

---

## The two gates

### Canon gate: passes

Checked against `INTENTS.md` I1 through I18 and `RULES.md`. The question I was given is upstream of
every intent op has stated and is not settled by any of them, and nothing in the dispatch asks me to
build a thing an intent forbids. Two places where my answer touches an intent are flagged where they
arise rather than resolved: I9 and I5 bear on whether a strategy is part of a primitive's identity, and
I state the fork instead of picking a side, because picking would be a settlement and this panel is in
explore mode.

**Two corrections arrived mid-dispatch, and I checked whether my derivation leaned on either.** It
does not, and I checked rather than asserting it.

The first is that `RULES.md` misstates the ratification rungs, saying "the only ratified material
remains `INTENTS.md`, I1 through I12", where `INTENTS.md`'s own normative section says exactly one
entry holds RATIFIED (I13), I14 is IN FORCE, I18 is a rule of thumb, and I1 was demoted to open by op
personally. `grep -n "I1 through I12\|ratified\|I14\|unratified"` against this file returns **zero
matches**: I make no claim anywhere about which entries are ratified, I never treat the strategy set as
closed at four, and the one place I lean on `RULES.md` is its predicate discipline, which quotes op
directly and is consistent with I13.

The second is that arvo's generated agent instructions carried a retracted claim that the `no_std` /
no-`alloc` / no-`dyn` / no-`TypeId` constraints are not in `INTENTS.md` and rest on unratified ground.
That version was in my context at session start. I did not build on it: I treated those constraints as
binding throughout, which is what P6 compiles under, and I nowhere argue that anything rests on
unratified ground.

### Test gate: passes, and it took real work to establish

**All 123 tests across 13 crates pass. Getting that answer required finding a hang.**

I ran every test-bearing bench variant crate rather than a filtered subset, and watched output rather
than trusting a summary. What that produced, in order:

**My own count was wrong, and six members were right.** The brief states that six members
independently counted 123 tests. My first count returned **124**, and I was ready to report the
discrepancy as a possible shared-drift signal. It was my error. The pattern I used,
`grep -rn "#\[test\]"`, counts a **doc comment that mentions the attribute in prose**:

```
variants/bitpack-write-contend-shared/src/stress.rs:68:
  /// mixes thread counts within a run). `cargo test` runs every `#[test]` in
```

An attribute-only pattern returns 123, and the difference is exactly that one line:

```
grep -rnE '^[[:space:]]*#\[test\][[:space:]]*$' --include='*.rs' variants/ | grep -v '/target/' | wc -l   # 123
```

Recording this because a count is a measurement and mine was the defective one. Six agreeing
measurements were not shared drift; my single command was.

**`cargo test` hangs in `bitpack-write-contend-shared`, and passes in 2.6 seconds serially.** This is
the finding, and it is a real defect. Full write-up with the reproduction, the stack sample and the
citation in `110_probes/p9_the_suite_hangs_under_the_default_runner.md`.

```
cargo test --release                        # hangs. killed at 45 minutes, no progress.
cargo test --release -- --test-threads=1    # ok. 15 passed; 0 failed; finished in 2.60s
```

The cause is in the crate's own comment at `src/stress.rs:66-72`, which observes that every test in the
file shares one process-wide `OnceLock` thread pool and concludes they must agree on a thread count.
They do. What it does not conclude is that they must not run **at the same time**, and libtest runs
tests concurrently by default. `sample(1)` on the hung process shows two test threads,
`stress::naive_kernel_corruption_rate_under_real_concurrency` and
`stress::naive_kernel_never_corrupts_when_the_split_is_aligned`, both live, both bottoming out in
`pool::write_pass`, at **1387 of 1387 samples**, sharing three pool workers. The sample is committed at
`110_probes/p9_hang_sample.txt`.

**And the brief's stated hazard is in the wrong place.** `RULES.md` warns that a batch runner under a
cap silently reports twelve of thirteen crates, attributing the long pole to `wide-rung-shared` taking
107 seconds. On this host `wide-rung-shared` runs its 30 tests in **4.05 seconds** in the batch run and
4.48 seconds standalone. The real long pole is the hang, and a hang is worse in the way that matters
here: a slow crate finishes if you wait, so a runner that times out records a crate that is silently
absent rather than one that failed.

**I am not refusing on this.** The gate refuses a suite that is decorative. This one is the opposite,
and I want to be as specific in praise as the gate requires me to be in criticism, because the two
judgements ride on the same reading:

- `satfold-shared/src/lib.rs` runs its arms against an oracle and then runs **four deliberately broken
  kernels** against the same oracle, so the agreement is shown to have been capable of failing. One of
  them, `the_one_element_defect_is_caught_up_to_1024_and_not_above_it`, asserts the instrument's own
  sensitivity boundary, including that the defect is **not** caught above 1024, with the Bernoulli
  argument for why in the comment. That is a test asserting a limitation of its own suite.
- `wide-rung-shared/src/shape.rs::wordround_alias_is_never_a_distinct_stride` asserts its property over
  `1u32..=512` where the six swept widths would have sufficed. That is the opposite of a sampled law.
- `stress.rs:105-111` deliberately refuses to assert a threshold on a scheduler-dependent corruption
  rate and says why, which is a harder call to make than adding an assertion.

Checked for and not found: `grep -rn 'assert!(true)'` returns nothing, and a pass over every
`assert_eq!` looking for a self-comparison returns nothing, across 359 assertion lines.

Tally, all observed rather than inferred: 78 tests across 11 crates in the batch run, 15 in
`bitpack-write-contend-shared` serially, 30 in `wide-rung-shared`. 123.

## Contamination declared, before anything rests on it

This workspace auto-loads `arvo-always-optimal-internals.md` into every agent context. That rule
already carries a claim about **when a law holds**, attributed to another panel member's probe, with
percentages. I did not go looking for it and could not have avoided it.

Two consequences I hold myself to. My probes do not test that claim and I make no claim of independence
from it. And where my results touch it, I downgrade my own agreement to what it is. Concretely, P2's
table shows distributivity over subtraction failing at unsigned `F = 0` saturating, which that rule
also reports. **That is not corroboration from me.** I had read it before I wrote the probe.

The question this file actually rests on is one that rule does not address: not when a law holds, but
whether a law set is a component of a primitive at all.

---

## The answer, stated first

**A primitive is a finite algebra over a declared signature: a set of values, and a total
interpretation of every operation symbol in the signature.** Its identity is that algebra up to
denotation-preserving isomorphism, and everything else that gets called part of a primitive sorts into
three bins:

- **What determines the algebra.** The value set, and the realisation map that lands exact results back
  in it. These are the primitive.
- **What merely presents it.** The bit encoding, the layout, the instruction selection. These are not
  the primitive and must not be, or nothing can ever be rewritten.
- **What is read off it.** The laws. Not components. A lossy projection.

The working assumption, that a primitive is a **named composition** of a format, a number system, a law
set and a strategy, is wrong in shape while being mostly right in material. It lists one thing that is
not a component (the law set), splits one thing wrongly (the number system is two axes under one name),
omits the thing that decides the answer (the signature), and uses two words, "named" and "composition",
that each carry a defect. I take these in order, and I say at the end what I keep, which is more than
what I drop.

---

## 1. What a primitive names

Not a set of values. Not a bit layout. An algebra.

The set is not enough because two primitives can have the same values and disagree on what an operation
computes. P2 sweeps 48 configurations and finds 40 distinct operation tables.

The layout is not enough, and this is the direction a systems reader gets wrong. **P1 puts four
different code assignments through the identity criterion and every one comes back the same
primitive**: offset binary, Gray code, order-reversed, and an arbitrary bijection `i -> (7i + 3) mod m`
with no structure in it at all. The criterion was not told that encodings do not count. It quotients
them out because nothing in the algebra reads them.

That is load-bearing. A definition that admits the encoding into identity makes every representational
choice semantic, and then no lowering can be substituted for another. A definition that admits nothing
but the values makes overflow policy invisible, and then a wrong answer can be substituted for a right
one. The algebra is the level that gets both right.

```
axis varied                            full signature   {add} only
1. total width W                       different        different
2. fraction width F at fixed W         different        different
3. signedness                          different        different
4. overflow policy                     different        different
5. radix 2 vs 10                       different        different
6a-d. encoding (4 kinds)               SAME             SAME
7. rounding mode at F>0                different        SAME
7b. rounding mode at F=0               SAME             SAME
```

`110_probes/p1_output.txt`. The separations are witnessed, not asserted: overflow policy is separated
by `add` at `(1/2, 15/2)` giving `15/2` against `0`, and rounding at `F > 0` by `mul` at `(1/2, 3/2)`
giving `1` against `1/2`.

## 2. What it is made of

**A value set, and one map.**

The value set is a finite set of rationals with a spacing. Whether it was described as fixed-point or
floating is a fact about the description, not about the thing.

The map is the part I want to argue for, because the vocabulary currently has two names for it.
**Overflow policy and rounding policy are one map**, `R : Q -> V`, taking an exact result to a
representable value. Rounding is what `R` does between grid points; saturation and wrapping are what it
does outside the range. Not two mechanisms that interact, two regions of one mechanism. I built both
models that way (`p2`, `p6`) and neither needed them separated; in the Rust probe it is two lines:

```rust
let k = R::round(a * b, S::DEN);
P::realise(k, LO, HI)
```

This is not tidiness. A design carrying them as two axes will eventually ask which one a given
behaviour belongs to, and that question has no answer. It also makes the identity criterion harder to
state: "the value set, the rounding and the overflow policy" says what "the value set and `R`" says,
and stops saying it when somebody adds a third region.

**And the signature, which the four-part list omits entirely.** P3 counts the same 288 configurations
under five signatures:

```
signature                 primitives   names   second names
{add}                             84     288            204
{add,sub}                         84     288            204
{add,mul}                        186     288            102
{add,sub,mul,neg}                186     288            102
{add,sub,mul,neg,le}             186     288            102
```

**The number of primitives more than doubles when `mul` joins the signature.** Not the number of useful
ones, the number of distinct ones: configurations that are literally the same algebra under
`{add, sub}` are different algebras under `{add, mul}`. So "how many primitives are there" is not a
question about the axes, and it is not well posed until somebody says which operations a primitive is
required to interpret. A canon that enumerates axes without fixing a signature has not defined the
thing it is enumerating axes of.

## 3. What makes two primitives the same or different

**A denotation-preserving isomorphism, relative to a declared signature.** Two primitives are the same
when a bijection between their carriers preserves what each value denotes and commutes with every
operation.

One criterion, decidable without search (preserving denotation forces the map, so there is nothing to
hunt for), and P1 shows it reproduces every answer a hand-written table of "which axes count" would
have to state, without being told any of them. **A list has to be relitigated every time somebody
invents an axis; a criterion does not.** P1 fed it an axis nobody put on the four-part list, the
rounding mode, and it classified it correctly and conditionally with no special handling.

### An axis can vanish two ways, and only one is safe to name away

P4 was built to confirm a hypothesis of mine and **falsified it**. I had claimed that at `F = 0` both
the rounding mode and the radix vanish structurally, arguing that a grid step of 1 means no result ever
lands between grid points so the rounding mode is unreachable. I added a halving operation and all
twelve classes I had called structural broke immediately. My argument held only for operations
**closed on the grid**, and `add`, `sub`, `mul` and `neg` are; division is not.

The falsification is in `110_probes/p4_output.txt` and stays there. The corrected distinction, which P5
then tested properly:

**Definitional degeneracy.** The axis disappears from the definition of the value set and of `R`. At
`F = 0` the step is `radix^0 = 1` whatever the radix is, so nothing mentions the radix again. No term
over any signature can separate two primitives differing only in it, because nothing is left that
reads it.

**Reachability degeneracy.** The axis is still a parameter of `R`, but no term in the *current*
signature produces an argument on which it matters. A fact about the operation set, not about the
primitive, and it evaporates when the operation set grows.

P5 separates them over 432 configuration points:

```
                    grid-closed    + half     + half, recip, fma
radix    at F=0     0/108          0/108      0/108      observable
rounding at F=0     0/108          108/108    108/108    observable
```

And the direct test, which is the part worth keeping because it needs no signature sweep at all: probe
`R` on a dense sample of the **whole rational line**, including arguments no term need ever produce.
The radix at `F = 0` does not change `R` anywhere on that line. The rounding mode does, at `F = 0`,
even though no grid-closed term can reach where it does.

**So the test is: does the definition read the axis, not does any current term observe it.** Cheap,
decidable, stable when the signature moves, and right on both cases where the naive test is wrong on
one.

### The consequence, which is a soundness statement

Only a definitional degeneracy may be canonicalised away. P4 prices doing it anyway: **6 of the 42
non-structural collapse classes broke** under operations built from the same realisation map. A design
that merged those on observed table equality would have to un-merge them the day somebody adds `half`,
so two types that were one type become two with nothing in the design edited to cause it.

This is the union-find discipline and it is the one thing I will not soften. A canonicalisation that
**splits** where it could have merged costs names and nothing else. One that **merges** two primitives
that are not the same hands one name to two different answers, and every consumer that substitutes
along that name gets a wrong value rather than a slow one. P3 scores both candidate rules in both
directions separately for exactly that reason; both return zero merges, neither is exact, and I would
ship the conservative one.

### And the criterion is a congruence, which I checked rather than assumed

An identity criterion licenses substitution, so it is only safe if it survives every context a
primitive appears in. P8 attacks my own answer at that joint: for each of three criteria, take every
pair it calls the same, build both composites under four constructions, and ask whether the composites
are still the same.

```
criterion                           pairs called same   congruence failures
weak (value set only)                             202                   131
medium (value set + add)                           88                    17
full (denotation-preserving iso)                   71                     0
control: encoding-differing pairs the full criterion merges: 21, failures: 0
```

The weak criterion is what a reader reaches for when a primitive is a set of representable values, and
it is unsound in 131 places. The medium one is what checking a single operation buys, and it fails
exactly where P1 predicted: on a rounding mode that `add` cannot see and `mul` can. The full criterion
has no failures, and the encoding control shows it is not surviving by being too strict to merge
anything.

**The first run of P8 was defective and the defect is recorded in the probe.** It swept no rounding
modes, so `medium` and `full` merged the same 23 pairs and both scored zero, making `medium` look
sound. A criterion cannot be tested against a population that excludes the case it fails on. That is
"setup that helps", occurring in my probe rather than in somebody's test.

## 4. What composing means, and why the word carries two jobs

Two different operations share the word, and only one of them is composition.

**Configuration** is choosing a point in a product of parameter spaces: pick a format, pick a number
system, pick a strategy. That is what "a named composition of four things" describes. Nothing is
composed; a record is filled in.

**Composition** is a construction taking an algebra to an algebra. That is what I11 asks for with
"contracts for things that compose to bigger units than just numerals alone": vectors, complex numbers,
dual numbers, intervals. P7 and P7b test four, and three things come out.

**A composite is a primitive.** Same definition, no amendment. So a canon should carry one concept and
not two, and every contract written for a primitive applies to a composite unchanged. Worth stating,
because a separate "composite" concept duplicates every rule about identity, laws and naming.

**A construction transforms the theory computably, and does not choose it.** The componentwise product
reproduces its base's law set exactly on every base tested, which is the classical fact that equational
theories survive products, and which means a lane-wise composite inherits every rewrite its scalar was
licensed for with no separate measurement. The twisted constructions do not: complex breaks `mul_assoc`
and `distrib` over a saturating base and preserves both over a wrapping one, which is what a
ring-preserving construction does to a base that is a ring in one case and not the other.

**A construction carries a predicate on its base.** P7 found this by failing: `interval` is not closed
over a wrapping base, because adding two well-ordered intervals returned `(1, 0)`, whose lower bound
exceeds its upper. P7b then tested the obvious candidate against 16 bases:

```
interval   vs predicate 'monotone'                 agrees on 16/16 bases  <== exact
interval   vs predicate 'always (no precondition)' agrees on  8/16
product2   vs predicate 'always (no precondition)' agrees on 16/16 bases  <== exact
complex    vs predicate 'always (no precondition)' agrees on 16/16 bases  <== exact
```

So `interval` is an arm with a const predicate on its base, and the predicate is monotonicity, which
wrapping does not have. The same shape as every other arm in this design, arrived at from the
composition side rather than the rewriting side, and I was not looking for it.

## 5. What giving a composition a name buys

**Nothing, unless the naming is canonical.**

A name is worth having when it is the unique name of its equivalence class. Two names for one thing is
not a convenience, it is a missed merge, and in a nominally typed language a missed merge is a wall
rather than a slow path. P6b is that wall, compiled:

```
error[E0308]: mismatched types
66 |     takes_binary(d)
   |     ------------ ^ expected `2`, found `10`
   = note: expected struct `Num<FxAxes<_, _, 2, _, _, _>>`
              found struct `Num<FxAxes<_, _, 10, _, _, _>>`
```

Those two types are the same primitive: P5 established that at `F = 0` the radix is definitionally
absent, 0 of 108 points made it observable under any of three signatures, and `R` does not read it
anywhere on the rational line. **And there is no repair.** No impl, no blanket, no const predicate
turns two type constructors applied to different arguments into one type without `generic_const_exprs`,
which is forbidden.

So the canonicalisation cannot be applied after the fact. It has to be built into how the type is
spelled, which means it is decided once, when the parameter list is chosen, and it is load-bearing for
every consumer that later wants one function over both.

P6 shows the version that works, and the move is small: **parameterise by what `R` actually reads.**
`R` reads the grid step, and it reads the radix and the fraction width only through the step. So take
the step. Then there is one spelling of a step-of-one primitive, no radix parameter to disagree about,
and the two intents that were two types are one type:

```rust
pub type U8BinaryNoFraction  = Fx<0, 255, Unit, Near, Sat>;
pub type U8DecimalNoFraction = Fx<0, 255, OneOver<1>, Near, Sat>;
```

`same_primitive_accepts_both` typechecks, and `p6_run` confirms it computes what it claims
(`canonical_by_construction` returns 255, saturating and wrapping return 255 and 44, `half(7)` returns
4 and 3 under the two rounding modes). Compiled on `nightly-2026-05-28`, `rustc 1.98.0-nightly
(57d06900f 2026-05-27)`, no forbidden feature, no arithmetic in type position. `p6_build.txt`,
`p6_run_output.txt`.

The general rule I would offer: **an axis that the value set and the realisation map do not read must
not be a type parameter.** Not "may be ignored", must not be present. Presence creates a nominal
distinction nothing can erase, and the erasure is what Rust will not give you. That is the same
discipline as canonicalising at construction rather than afterwards, and here it is the only option
rather than the tidier of two.

## 6. Where the working assumption survives, and where it does not

Keeping something is a result, so the ledger before the disagreements.

**Format survives, and wants re-cutting.** Everything the four-part list means by format is
identity-bearing: width, signedness, fraction width, overflow behaviour. P1 separates each with a
witness. What I would change is the cut: what `R` reads is the grid step, and radix and fraction width
enter only through it, so carrying them separately manufactures the duplicate names P6b walls off.

**Number system does not survive as one axis. It is two things under one name.** The part that changes
the value set (the radix, at `F > 0`) is identity-bearing. The part that changes only which bit pattern
stands for which value is presentation, and P1 quotients four kinds of it out automatically. Naming
them together guarantees a discussion about one is heard as a claim about the other.

**Law set does not survive as a component, and this is the clearest of the four.** P2 tests it as a
coordinate, which is what a component is:

- It is a **function** of the algebra. 40 algebra classes, 0 carrying more than one law set.
- It is **not injective**. One law set is shared by 8 different algebras; 40 algebras collapse to 7 law
  sets. So it carries strictly less information than the primitive and cannot reconstruct it.
- It **cannot be varied** with every other coordinate held fixed. 0 of 48 configurations.

A coordinate you cannot vary is not a coordinate. **The correct residue is real:** a law set read as a
*demand* rather than a component is a query over the configuration space, and P2's fourth test runs it
as one. Demanding `distrib_add` selects 12 of 48 configurations, all at `F = 0`. Demanding
`distrib_sub` selects 8, all at `F = 0` and all wrapping. That is a predicate in I13's sense and a
useful surface. It is just not a coordinate of a point in the space, and calling it one will make
somebody try to set it.

**Strategy: I am not going to settle this, and I want to say precisely why.** Under my criterion an
axis is identity-bearing exactly when it changes a computed value. I9 says strategies "are the
variables that change what the correct answer is", and I5 says Hot "can sacrifice soundness ... for
provable meaningful gains". Both read as licence for a strategy to change a value. If it does, the
strategy is identity-bearing and two strategies are two primitives. If it never does, the strategy is a
selector over presentations and is not part of identity at all.

Both are coherent and they are different designs. What distinguishes them is measurable and needs no
ruling from op: pick a strategy pair and ask whether any operation at any width computes a different
value. What I will say, in my own register, is the warning:

> **The strategy is a cost function, and I5 licenses a cost function to change an answer. That is
> exactly the configuration in which substitution-based optimisation stops being sound.**

Choosing a lowering by cost is safe precisely because the candidates all denote the same thing. The
moment the cost function may prefer a candidate that denotes something else, every rewrite in the
system inherits the doubt, and no local inspection recovers it. That is not an argument against I5,
which is op's intent and not mine to weigh. It is an argument that **the licence has to be declared and
scoped**, so the region where a strategy may change an answer is a named predicate rather than a
general permission, and everything outside it keeps the substitution property the rest of the design
depends on.

**"Named composition" is the wrong shape twice**, and both defects are priced above. "Named" promises
what a non-canonical naming cannot deliver, and the delivery failure is a compile error with no repair.
"Composition" names configuration, while the thing I11 asks for is a different operation this
vocabulary then has no word left for.

---

## Findings, with predicates

Per I13 and `RULES.md`. Everything below was established by exact arithmetic over exhaustively
enumerated small domains on a single thread. **`threads = 1` is on every finding** rather than omitted,
because that is what was established; widening to `threads any` is a separate claim only measurement
makes. Target features are absent from every predicate, carrying the strict meaning: nothing here is
claimed to hold where target features are in play, because nothing here measured them.

Model-width caveat, stated once for all of them: exhaustive at `W <= 5`, and an exhaustive check at a
model width transfers to a real width only with an argument. I have not made one. Where a finding is
structural rather than enumerative I say so in its line.

**F1. Identity as denotation-preserving isomorphism reproduces the axis classification without being
given it.** `W in {4,5}, F in {0,1,2}, signedness any, policy in {sat,wrap}, rounding in {near,trunc},
radix in {2,3,10}, encoding in {identity,offset,gray,reversed,arbitrary-bijection}, signature in
{{add}, {add,sub,mul,neg,le}}, threads = 1`. `p1_output.txt`.

**F2. Every pure code assignment is presentation.** Four encodings including a structureless bijection
return the same primitive under both signatures. `W = 4, F in {0,1}, unsigned, policy = sat, rounding
in {near,trunc}, radix = 2, threads = 1`. `p1_output.txt`.

**F3. A law set is a function of the algebra, is not injective on algebras, and cannot be varied with
the other coordinates fixed.** 48 configurations, 40 algebras, 7 law sets, 0 free variations. `W in
{3,4}, F in {0,1,2}, signedness any, policy in {sat,wrap}, rounding in {near,trunc}, radix = 2,
signature = {add,sub,mul,neg}, laws = the ten enumerated in p2, threads = 1`. `p2_output.txt`.

**F4. The number of distinct primitives is a function of the signature, not of the axes alone.** 84
under `{add}`, 186 under `{add,mul}`, over the same 288 names. `W in {2,3,4}, F in 0..=W, signedness
any, policy in {sat,wrap}, rounding in {near,trunc,floor}, radix in {2,3}, signature in the five
enumerated in p3, threads = 1`. `p3_output.txt`.

**F5. Radix at `F = 0` is definitionally degenerate.** Unobservable at 0 of 108 points under every
signature swept, and `R` does not read it anywhere on a dense sample of the rational line. `W in
{2,3,4}, F = 0, signedness any, policy in {sat,wrap}, rounding in {near,trunc,floor}, radix in {2,3,5},
signature in {grid-closed, +half, +half+recip+fma}, threads = 1`. `p5_output.txt`. The rational-line
half is structural rather than enumerative: `step = radix^0 = 1` removes the axis from the definition,
which is an argument rather than a sweep.

**F6. Rounding at `F = 0` is reachability-degenerate, not definitional.** Unobservable at 0 of 108
points under the grid-closed signature, observable at 108 of 108 the moment a halving operation is
present. Same predicate as F5. `p5_output.txt`, with the falsification that produced it in
`p4_output.txt`.

**F7. Both candidate canonicalisations are sound and conservative; neither is exact.** Zero unsound
merges and 8 conservative splits for the better rule on a disjoint held-out sweep. `W = 5, F in 0..=5,
signedness any, policy in {sat,wrap}, rounding in {near,trunc,floor}, radix in {2,3,5}, signature =
{add,sub,mul,neg,le}, threads = 1`. `p3_output.txt`.

**F8. Two names for one primitive is a compile error with no in-language repair.** `nightly-2026-05-28,
rustc 1.98.0-nightly (57d06900f 2026-05-27), edition 2021, no forbidden features, threads = 1`.
`p6b_expected_failure.txt`, E0308.

**F9. Parameterising by the grid step instead of by radix and fraction width gives the degenerate
primitive exactly one spelling.** Compiles and runs; every asserted value matched. Same toolchain
predicate as F8. `p6_build.txt`, `p6_run_output.txt`.

**F10. A composite is a primitive under the same definition.** Closure holds for the componentwise
product, complex and dual over every base tested, and for interval over every monotone base. `W in
{2,3}, F in {0,1}, signedness any, policy in {sat,wrap}, radix = 2, signature = {add,sub,mul},
constructions in {product2, complex, dual, interval}, threads = 1`. `p7_output.txt`, `p7b_output.txt`.

**F11. The componentwise product preserves its base's law set exactly; the twisted constructions do
not.** Product2 diverged from its base on 0 of 20 law entries. `W = 2, F in {0,1}, signedness any,
policy in {sat,wrap}, radix = 2, laws = the five enumerated in p7, threads = 1`. `p7_output.txt`.

**F12. The interval construction is closed exactly on monotone bases.** The predicate agrees on 16 of
16 bases; the no-precondition hypothesis agrees on 8 of 16. `W in {2,3}, F in {0,1}, signedness any,
policy in {sat,wrap}, radix = 2, signature = {add,mul}, threads = 1`. `p7b_output.txt`.

**F13. Denotation-preserving isomorphism is a congruence with respect to the four constructions
tested; the two weaker criteria are not.** 0 failures over 71 merged pairs, against 131 over 202 and 17
over 88. `W in {2,3}, F in {0,1}, signedness any, policy in {sat,wrap}, radix in {2,3}, rounding in
{near,trunc}, encoding in {identity,offset,gray,shuffled}, constructions in {product2, complex,
interval, product3}, threads = 1`. `p8_output.txt`.

**F14. `cargo test --release` does not terminate in `bitpack-write-contend-shared` under the default
test runner, and passes in 2.60s under `--test-threads=1`.** `nightly-2026-05-28, this host, threads =
1 for the passing arm and libtest default parallelism for the hanging arm`.
`p9_the_suite_hangs_under_the_default_runner.md`, `p9_hang_sample.txt`.

**Unpriced.** Everything about cost. I measured no time, no space and no instruction count, and the
word for every magnitude in this file is unpriced. In particular, the claim that a conservative
canonicalisation "costs names and nothing else" is a soundness statement, not a cost statement: what
the extra monomorphisations cost in compile time and code size has not been measured by me and is not
measured in `mock/benches/`. The two wall-clock numbers in the gate section are observations about a
test suite, not benches, and they decide nothing about a design.

## Options this fits, fits badly, and kills

I have not read `OPTIONS.md`, so I cannot name its entries. What my evidence bears on, stated so
whoever holds the register can match it up:

**Fits well.** Any option making the primitive a parameterised algebra with the strategy as a declared
consumer-facing axis. Any option treating laws as measured facts about a configuration rather than as
declarations. Any option parameterising by a grid step, a scale, or an exponent-and-significand pair
rather than by a radix and a fraction width separately.

**Fits badly, at a nameable cost.** Any option carrying both a radix parameter and a fraction width
parameter. It survives, and it pays P6b's wall at `F = 0`, and the payment is a hard compile error
rather than a slow path.

**Killed, with the diagnostic.** Any option making a law set a settable component of a primitive. P2
TEST 3: 0 of 48 configurations can vary it with the others fixed. It is not a knob, and offering one is
offering a control connected to nothing.

**Killed, with the diagnostic.** Any option canonicalising primitive names by observed equality of
operation tables. P4: 6 of 42 such collapses broke when operations were added that use the same
realisation map, so the resulting type identity is a function of the current signature and changes
under an addition nobody would read as breaking.

**Added, and I think this one is new.** A construction on primitives carries a const predicate on its
base, and `interval` requires monotonicity (16/16). That makes the composite layer the same shape as
the arm layer, so one mechanism could serve both.

## What I did not cover

Bounded honestly, because a claim of completeness in this workspace has repeatedly turned out to be
unmeasured.

I read no panel file, no consolidation, `OPTIONS.md`, `DROPLIST.md`, `PERSONA_CALLS.md`,
`PRIOR_CALLS.md`, the `SEED_*` files, or the archive. That is the protocol and it is also a real limit:
if the panel has already established any of the above, this file is a second instance rather than a
first, and should be recorded as whichever it turns out to be.

I measured nothing. No bench, no timing that decides anything, no claim depending on a magnitude.

I did not test non-uniform value sets. Mine are uniformly spaced, so every finding is about a uniform
grid. The realisation-map framing should carry to a non-uniform spacing unchanged, because `R` never
assumed uniformity, but I did not run it and do not claim it. This is the single largest gap in the
file, because the float side of arvo lives there.

I did not test a signature containing division as a primitive operation rather than as the probe
operations `half`, `third` and `recip`. Division introduces a partiality at zero that my model handled
by returning zero, which is a probe shortcut and not a design decision.

I did not test widths above 5, and I have no transfer argument to real widths.

I did not check whether arvo's strategies actually do change computed values. That is the fork in
section 6, measurable by someone with the strategy definitions in front of them, which I deliberately
do not have.

I did not verify my own reading of `stress.rs` against its author's intent. The hang is reproduced and
the mechanism is sampled; whether the remedy is a mutex, serial sequencing, or a separate test binary
is not mine to call.

## Two things outside my question

**One unlicensed mechanism.** `RULES.md` names `mock/benches/` as "the only thing in this workspace
that can price anything", and the panel cites its committed artifacts. **Sixteen bench variant crates
have untracked `Cargo.lock` files**, against a dependency declared as a git branch rather than a pinned
revision:

```
mockspace-bench-core = { git = "ssh://...mockspace.git", branch = "dev", features = ["std"] }
```

21 lockfiles are tracked and 16 are not (`git ls-files mock/benches/ | grep -c Cargo.lock` against
`git status --porcelain mock/benches/ | grep -c Cargo.lock`), and the untracked set includes
`wide-rung-shared`, `warm-container-shared`, `warm-clamp-shared`, `bitpack-carrier-shared`,
`bitpack-footprint-shared`, `bitpack-plan-shared`, `bitpack-shared`, `bitpack-wide-shared`,
`quantiser-fadd-shared` and `quantiser-radix-shared`.

The consequence is narrow and real: for those crates a rerun resolves `dev` at rerun time, so a number
disagreeing with a committed CSV cannot be attributed between a real change and a moved dependency.
That invalidates no committed artifact and I do not claim it does. It means the artifacts are not
reproducible on their own terms, which is the property `RULES.md` leans on. The state predates this
dispatch; my own test run refreshed those files rather than creating them. I did not fix it, because
committing sixteen lockfiles touches other members' in-flight work and is not a call to make from
inside a phase-one dispatch.

**And the hang above is the other.** It is in `110_probes/p9_*` rather than only here because the next
member to run the suite will otherwise lose the same forty-five minutes, and because a check performed
by hand and left as prose is a check that has to be redone.

---

# Phase two: reconciliation

Appended after the phase-one commit `eadbc1cc`. **Nothing above this line is edited.** Its value is that
it was written blind, and the blind version is the thing being checked.

## What I read, bounded

`109` in full, which is the parallel cold derivation on the same question; `99`, the checkpoint;
`63`, the format-concept consolidation, sections 0 through 9 as far as line 796; `90`, the derived-laws
consolidation, sections 0 through 2; `98` at the lines that bear on the test count; `OPTIONS.md` Q16 and
its heading index; `DROPLIST.md` grepped rather than read; `SEED_THEORY_91_UP.md` grepped only.

**Not read:** `43` itself (I have it only through Q16's account of it), `53`, `74`, `106`, the members
of any unit, the `SEED_*` files beyond one grep, `PRIOR_CALLS.md`, `PERSONA_CALLS.md`, the archive. So
where I say something is new below, the claim is bounded by those greps and not by a reading, and I say
which grep.

## 16.1 Where my file is a second or third instance rather than a finding

Three of my headline results were already established, and finding that out is what phase two is for.

**The realisation map is not new, it is `63`'s C1 and it has two prior cold arrivals.** My "a value set
and one map `R`" is the panel's `computed = adapt(exact)`, derived cold by `55` and again by `60` at the
chain level, and stated as a candidate canon sentence at `63` C1. Mine is a **third independent
instance**, arrived at from the identity question rather than from error analysis, and it should be
counted as corroboration of an existing claim rather than as a contribution.

**Encoding is not part of identity: `63` C2 already says it**, and says it in almost my words: "Adaptation
choice and encoding are realisation, observable in computed values and in pattern-level properties
respectively, and not part of identity." Their witness is two's complement against offset binary on the
same sixteen values. Mine adds two encodings theirs did not use, a Gray code and a structureless
bijection, which widens the instance without changing the claim.

**The law set is not a component, and it now has three independent methods behind it.** `90` R1 states
that a law is a fact about an operation composed under a fixed arithmetic semantics, at TWO EXPERTS from
`76` and `77`. `109` establishes it by showing a false declaration compiles and is load-bearing, changing
952 of 4096 answers through a rewrite gated on it. I establish it by testing it as a coordinate: it cannot
be varied with the others held fixed, 0 of 48. **Three different attacks, one conclusion, none of us
having read the others on it.** That is the strongest form this panel's rungs can carry, and the useful
observation is that the three are genuinely independent: a declaration census, a provenance argument and a
coordinate census have almost nothing in common as instruments.

**"Composition" is overloaded, and `OPTIONS.md` Q16 already carries it** from `43`, with the two senses
named exactly as I found them: sense one, the founding sentence's "primitives become named compositions
over one format concept", and sense two, op's "contracts for things that compose to bigger units than
just numerals alone". My section 4 rediscovers Q16. What I add to it is in 16.4.

**And the componentwise-product law preservation is a classical theorem the seed already cites.**
`SEED_THEORY_91_UP.md:137-139` names Birkhoff and varieties being closed under direct products. My F11 is
a measured instance of that at four bases, not a discovery, and I should have recognised it while writing
it. It is still worth having as a measurement, because the fixed-point operations are not obviously a
variety in the relevant sense once the reduction is in play, but the credit is Birkhoff's.

## 16.2 Where I was wrong, over-stated, or slid a word

**My test count was wrong and I raised it as a possible drift signal.** I counted 124, said so, and
suggested six agreeing measurements were the shape to be suspicious of. `98:55-56` had already found the
answer and names the same file and the same line I eventually found: the literal `#[test]` inside the doc
comment at `bitpack-write-contend-shared/src/stress.rs:68`. **123 is correct.** My contribution is a
second independent arrival at an explanation that already existed, which is worth something and is not
what I presented it as.

This also **closes `109` section 16.6**, which records that two independent phase-one counts returned 124
against the consolidation's 123 and asks whoever holds the register to re-run rather than pick a side. It
is re-run: the naive pattern returns 124, the attribute-only pattern
`grep -rnE '^[[:space:]]*#\[test\][[:space:]]*$'` returns 123, and the set difference is exactly that one
doc-comment line. No test was added; the consolidation is right and both of us were counting a comment.

**I asked for a single sameness relation, which is the shape this workspace has a rule against.** My
section 3 presents denotation-preserving isomorphism as *the* identity criterion. `109` section 10 gives
three relations, nominal, representational and denotational, strictly nested, each licensing a different
operation: assignment without a cast, reinterpreting memory, and rewriting respectively. That is better
factored than mine and it is what `never-ask-which-single-rule-governs.md` says to produce.

**I concede the framing and keep the result, and the two fit together cleanly.** My criterion is `109`'s
denotational relation. What P8 adds is that **that relation, and not the weaker ones, is the one closed
under composition**: over 202 pairs merged by a value-set-only relation, 131 fail to survive a
construction. So the three-relation lattice gains a property on its bottom rung, which is the reason the
bottom rung is the one that may license a rewrite. Stated as one arm rather than as a policy: *the
denotational relation licenses substitution, including inside a composite; the weaker relations do not,
and the count of where they break is 131 and 17.*

**My F2's wording is right and its reach is narrower than it sounds.** I wrote that every pure code
assignment is presentation. `63` section 3.5 says the encoding is not part of identity **and is
observable**, through raw-order agreement and raw-adder correctness. `109` section 16.4 then tested the
apparent conflict rather than arguing it and found the discriminator: an encoding is observable through an
operation exactly when that operation is defined on the representation rather than on the denotation, with
all four encodings agreeing on all 256 pairs under denotation-defined operations and separating
immediately under representation-defined ones.

My predicate listed only value-level signatures, so under the notation's own rules F2 claims nothing about
representation-defined operations and is not wrong. But the sharper sentence is theirs, and I would rather
have written it. This is my own signature-relativity finding applying to a case I did not think to
instantiate, which is a good demonstration that the criterion is more general than its author.

**And one word of mine slid, which `109` caught.** I wrote that the encoding and layout "are not the
primitive and must not be, or nothing can ever be rewritten". The rewriting half is right. The wording
slides from *not part of identity* to *not part of the surface*, and I6 and I17 require a consumer to be
able to **ask** for the storage-minimising realisation, with I17 explicit that this is not to be
deprioritised. The correction is accepted in full: the realisation is quotiented out of denotational
identity and is emphatically part of what a consumer selects.

## 16.3 Where `109` and I agree, having each derived blind

Recorded because independent agreement is the only thing that reaches the middle rung, and because `109`
wrote its own version of this section without either of us having read the other at derivation time.

We agree that the law set is not a component; that the operation set is part of the definition, which
`109` reached from the chain result and I reached by counting; that overflow and rounding are one map;
and that a criterion beats an enumeration.

Two places where we differ usefully rather than conflicting:

**Our criteria are at different levels and should both survive.** `109` says so first and I agree on
reading it. Its const-availability test decides **membership**, whether a property belongs in the
primitive at all. Mine decides **identity**, whether two primitives are the same. A design wants both, and
merging them would lose the question each answers.

**`109` concedes the realisation point to me and I concede the sameness framing to it.** Both concessions
are in the same direction, which is that identity is thinner than the specification: the primitive's
identity is the denotation, and the specification carries more than the identity does.

## 16.4 What of mine appears to be new, bounded by grep rather than by reading

**The definitional against reachability distinction.** `grep -rli "definitional degenerac"` across the
live panel returns nothing outside my file, and I found no member drawing the line between an axis that
has left the definition and one the current signature merely fails to reach. It is the result I would most
want attacked, because it was my own falsified hypothesis that produced it: P4 broke the version I
believed, and the corrected version is the one that survives adding operations. Its practical form is a
test anybody can run in five minutes: probe the realisation map over the whole rational line rather than
over the terms the signature reaches.

**The congruence check on the sameness relation.** `grep -rin "congruence.*construction\|substitut.*composite"`
returns nothing outside my file. "Congruence" is used widely in the panel, but in `57b`'s sense of the
reduction's kernel being a congruence for an ambient operation, which is a different object. What P8 asks
is whether the *sameness relation between primitives* survives the constructions, which is the soundness
condition on the relation itself rather than on any reduction.

**The naming wall, and its remedy.** `grep -rli "canonicalis"` finds only an unrelated use in
`DROPLIST.md`. The result is that two spellings of one primitive cannot be merged after the fact under the
forbidden-feature list, compiled as `E0308` in `p6b_expected_failure.txt`, and the remedy is to
parameterise by what the realisation map reads so the second spelling never exists. This composes with
`109`'s naming result rather than competing: `109` finds that naming is interesting when it is **partial**,
because a name then becomes an existence claim and the naming function is the validator. Mine finds that
naming is dangerous when it is **non-injective**. A design wants the naming function to be partial and
injective, and neither of us had both halves.

**And a gap in Q16 that I can fill, from the wrong direction.** Q16 lists three ways out, ending with "to
find that they are the same concept at two scales, **which nobody has argued**". I did not argue that
either, and what I have is a fourth option Q16 does not list:

> **Sense one is not composition at all.** Choosing a format, a system and a strategy is **configuration**:
> filling in a record, with nothing composed. Only sense two is composition, in the ordinary algebraic
> meaning of a construction taking an algebra to an algebra. And composites are **primitives by the same
> definition**, so the concept is closed under sense two and the canon needs one concept rather than two.

P7 is the evidence for the closure half: the componentwise product, complex, dual and interval
constructions all return a carrier with a total interpretation of the signature. If that holds, "primitive"
and "composite" are one concept and every contract written for one applies to the other, which is a
stronger statement than either of Q16's first two options and cheaper than the third.

## 16.5 A conflict that dissolves, and it sharpens both sides

`63` C10 says: "Compositions over formats, stored pairs, intervals, error-carrying values, are not format
instances; they consume these three things and owe their own laws." My P7 says a composite **is** a
primitive under the same definition. Read side by side those look opposed, and a later reader would spend
real time on it.

They are not opposed, and the resolution is that **format and primitive are different concepts and the
panel has been careful about one of them.** A format is `(D, Q)`, an ambient domain and a representable
set of numbers. An interval over a format has a carrier of *pairs*, which is not a set of rationals, so it
is not a format and `63` is right. It is still a carrier with a total interpretation of the signature, so
it is a primitive and P7 is right.

What falls out is a containment rather than a conflict: **every format induces a primitive, and not every
primitive comes from a format.** That is worth having explicitly, because the working assumption puts
"format" inside "primitive" as a component and the containment says something different and more useful
about how the two relate.

## 16.6 One of my findings was already explained, and the explanation is better than my measurement

P7b measures that the interval construction is closed exactly on monotone bases, 16 of 16, and reports
that wrapping is not monotone. `63` section 3.6 carries the reason: **a wrapped numeral has no
arithmetic-compatible order, because a finite cyclic group admits no translation-invariant total order.**

So my empirical predicate has a structural explanation sitting in a consolidation I had not read, and the
finding upgrades from "monotone, 16 of 16 at `W in {2,3}`" to a statement that needs no width predicate at
all on the wrapping side, because the group-theoretic fact does not depend on the width. I do not restate
the predicate on my phase-one finding, per the rule that a predicate is never widened in place; the
widening is claimed here, in my own words, and it rests on `63`'s citation rather than on my sweep.

That is also the cleanest example in my file of why phase two exists. The measurement was right, the
region was right, and the reason was available and better.

## 16.7 What I would hand whoever writes the canon text on this topic

Suggestions, settling nothing, and each one carries what it rests on.

**One.** A primitive is a carrier with a total interpretation of a **declared signature**, and the
signature is part of the definition rather than context for it. Rests on my F4 and `109`'s section 8,
independently.

**Two.** Identity is **denotational**, and denotational sameness is the relation that licenses rewriting,
including inside a composite. The other two relations in `109`'s lattice license other operations and are
not competitors. Rests on `63` C2, my P1 and P8, and `109` section 10.

**Three.** An axis belongs to identity when the value set or the realisation map **reads** it, and the
test is over the whole domain of the map rather than over the terms the current signature reaches. Rests
on my P5, with the falsified version in P4 kept as the reason the wording is what it is.

**Four.** A law is read off, never declared. Three independent methods, `90` R1, `109` P2 and my P2.

**Five.** The concept is **closed under composition**, and configuration is not composition. Rests on P7
and P7b, and it fills the option Q16 names as unargued with a fourth shape rather than the third.

## 16.8 Coverage of phase two, bounded

I did not read `43`, `53`, `74` or `106`, so my statements about the composition senses come through
Q16's account of `43` rather than from `43`, and my statement that the definitional-degeneracy distinction
is absent rests on a grep of the live panel rather than on having read those four files. `109` section
16.5 flags that `106` section 9.2 bears on I18; I did not read it and nothing of mine turns on I18.

I did not re-run any other member's instrument. `63` re-ran thirty-two of them and reports all diffs
empty; I take that on its report and did not verify it, which is a dependency worth naming since two of my
reconciliations lean on `63`'s numbers.

The one number I did verify at source rather than through an account is `98:55-56`, because it is the one
I used to withdraw a claim of my own.
