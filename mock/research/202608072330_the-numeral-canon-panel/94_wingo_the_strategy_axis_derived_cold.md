# 94. The strategy axis, derived cold

**Phase one. Written blind.** The only things read before this commit were `INTENTS.md`, `RULES.md`,
arvo's own `.claude/` rules, the workspace rules that arrive with every dispatch, and the repository
itself: `mock/DESIGN.md.tmpl`, `mock/PRINCIPLES.md.tmpl`, `mock/WORKFLOW.md.tmpl`, `mock/Cargo.toml`,
`mock/crates/` (empty), and `mock/benches/`. No panel file, no consolidation, no option register, no
droplist, no probe of anyone else's, no git log, no commit message. A reconciliation section is
appended in a separate commit after reading the panel.

hey hackfolk. The question I was handed is "what is a strategy", with the shape of the answer left
open, including whether a fixed set of named markers is the right shape at all and whether the word
is covering one thing or several. What follows is a derivation from op's intents and from the
machine, in that order, with seven committed probes in `94_probes/`. I will tell you up front where
I landed, because the rest is long: I think **"strategy" is currently three separable things wearing
one name**, I think **two of the three do not belong on the value**, and I think the question "how
many strategies are there" is not well formed once they are separated, which is a satisfying place
for I1's demotion to land.

I could not settle what a mixed-strategy operation should resolve to, and I will say why: I found
evidence that the thing being resolved does not form a lattice, so both of the available answers are
wrong in different ways and the fork is real rather than a matter of taste.

## 0. The two gates

**The canon gate passes.** I checked the assigned work against `INTENTS.md` and found nothing in the
question, or in the state it builds on, that conflicts with op's stated intents. The question is open
by I1 (the set is not closed at four, demoted to OPEN on op's word, `INTENTS.md:51-61`), and deriving
what a strategy is is exactly what the panel is for.

**The test gate is more complicated, and I want to be precise rather than dramatic about it.**

There is no arvo test suite, because there is no arvo. `mock/crates/` is empty and `mock/Cargo.toml`
declares `members = []`, so `cargo test --workspace` in `mock/` does not run zero tests, it errors:

```
error: manifest path `/Users/orgrinrt/Dev/clause-dev/arvo/mock` contains no package:
The manifest is virtual, and the workspace has no members.
```

That is the intended state (`mock/Cargo.toml:1-16`) and it is not a defect. It does mean the gate has
to be run somewhere else, and the only executable surface bearing on my question is `mock/benches/`.
So I ran that, and read the bodies rather than the names.

The run is committed at `94_probes/test_gate.out.txt` with the command that produced it, because a
count is a measurement and has to be reproducible like any other.

One trap on the way, worth recording because it would silently pass: `cargo test --workspace` **inside
`mock/benches/` runs zero tests** and reports ok, because the variant crates are path dependencies
rather than workspace members. The 108 tests that exist are reached only by running each variant
crate. Running them one at a time:

```
warm-clamp-shared            7 passed
warm-container-shared       15 passed
wide-rung-shared            30 passed
satfold-shared              11 passed
bitpack-shared               3 passed
bitpack-carrier-shared       9 passed
quantiser-radix-shared       3 passed
quantiser-fadd-shared        1 passed
bitpack-plan-shared          5 passed
bitpack-footprint-shared     6 passed
bitpack-wide-shared          6 passed
bitpack-contend-shared      12 passed
```

`bitpack-write-contend-shared` exceeded my command budget on this machine and I did not get a result
for it; that is recorded in the artifact rather than left as an implication that I ran everything.

**The tests I read are good, and I want to say so plainly, because "keeping something is a result".**
Not one of the strategy-relevant ones is decorative. `warm-clamp-shared` cross-validates six arms
against an independent `u128` oracle at every declared key rather than a sample, and it does not stop
there: it perturbs one bit in a chunk that does not clamp and asserts the answer moves, and one bit in
a chunk that does clamp and asserts it does not, which is the check that catches a constant-folded
workload masquerading as a measurement
(`mock/benches/variants/warm-clamp-shared/src/lib.rs`, `chunked_answer_depends_on_every_element_the_clamp_did_not_absorb`).
It asserts the retraction lemma the whole bench rests on over the full swept matrix rather than at a
chosen width, and it asserts that its own quoted noise-floor controls really are the same
instantiation, which is a fact about which const the accumulator selector returns rather than a hope.
`satfold-shared` decides its associativity law exhaustively over `1 << 24` triples and separately
asserts that its two const verdicts differ and are computed, so the gate pair is not two names for
one thing. That is the discipline working.

I did find one weak spot and it is minor: `bitpack-shared` has three tests, all
`check_size::<N>()` roundtrips at 256, 4096 and 16384
(`mock/benches/variants/bitpack-shared/src/lib.rs`). The body does assert a real property (the
permutation is a bijection), so it is not tautological, but three sizes of one shape is a sample and
the crate is a shared workload builder that a lot rests on. I am recording it, not refusing on it.

**So the gate passes, with the coverage bound stated: 108 tests read and run in the bench tree, one
variant crate not run, no arvo suite in existence to run.**

## 1. What I am reporting outside my question

Two things the intents do not license, and I was told to report them even out of scope.

**Both root design documents are live, unbannered, and assert as settled the thing I1 demoted.**
`mock/crates/` was nuked so the canon could be written, and `mock/Cargo.toml:1-16` explains exactly
why in the mutation-order terms the chain rule uses. `mock/DESIGN.md.tmpl` and
`mock/PRINCIPLES.md.tmpl` were not nuked and carry no marker at all. `grep -cin 'nuked\|dead
tier\|superseded\|stale'` returns 0 for both. And they are not neutral: `mock/DESIGN.md.tmpl:3`
opens with "Numeric primitives + analysis algorithms with Hot / Warm / Cold / Precise strategy
markers", `mock/PRINCIPLES.md.tmpl:271` states "Strategy markers (`Hot`, `Warm`, `Cold`, `Precise`)
are type parameters", and `mock/PRINCIPLES.md.tmpl:294` states "Default strategy is `Warm` unless
called out."

That is precisely the failure the chain rule names: a lower tier that survives a change above it
becomes a claim about something that no longer exists, and it gets defended because it is concrete
and detailed next to the abstract statement that replaced it. Op demoted I1 on 2026-08-08. These
documents still read as the settled answer to the question this panel is convened to open, they sit
flat in `mock/` where a reader looking for the design will find them first, and nothing on the page
says they are dead. `mock/DESIGN.md.tmpl:35` even names a specific resolution mechanism,
`Resolve<S1, S2>`, which is one of the two live answers to a question I could not settle below.

**And `mock/PRINCIPLES.md.tmpl:33` declares a nightly feature set including
`feature(generic_const_exprs)`**, which `unstable-features.md` lists in its FORBIDDEN table on op's
direct call, superseding an earlier WATCH resolution. A live design document naming a forbidden
feature as part of the design is not a stale detail, it is an instruction to a future implementer to
do a thing the workspace forbids.

I am not proposing what to do about either. Both are the dispatcher's to handle, and the cheap remedy
for the first is a banner rather than a deletion, since canon is demoted rather than deleted and the
same courtesy makes sense one tier down.

## 2. Going wide before going deep

Before deriving, I enumerated the categories a "strategy" could belong to. Permuting the four markers
would not be exploring anything; these are meant to be genuinely different kinds of thing.

**A preset.** A named tuple of policy values, resolved by table lookup. This is what the shipped
design was: `mock/PRINCIPLES.md.tmpl:288-292` lists what the markers drive (container width, codegen
decisions, tradeoff clarity) as a fixed bundle per name. Op's own word for it is "preset" (I2), which
is worth noticing, because a preset is by definition a named point in a larger space, and naming one
implies the space.

**An objective.** A weighting over measurements, with the implementation chosen by taking the best
candidate under that weighting. I8 is almost literally this: "All of them should be decided by
measurement, just measuring different things, and, this is I think the mental unlock: They weigh
different measurements differently."

**A permission set.** Which rewrites you are allowed to invoke. Not the same as an objective: in any
real optimiser the cost model and the rewrite-rule set are two separate artifacts, and confusing them
is how you get a compiler that is fast and wrong. I5 reads naturally here: Hot "can sacrifice
soundness, that is its explicit purpose", which is a statement about permitted moves, not about what
is being minimised.

**A denotation.** Different strategies mean the numeral denotes a different mathematical object, so
they are different types sharing a syntax and the mixing question is coercion rather than selection.

**An error domain.** A refinement carrying an accuracy bound, with the marker a coarse summary of a
lattice of bounds. This is the reading under which I7's "accurate within chains" is a compositional
property in the ordinary abstract-interpretation sense.

**A layout decision.** Where the value lives and what the memory traffic is, with arithmetic policy
downstream of that. I6 and I17 both read most naturally here.

**A property of the code region rather than of the value.** notko ships `#[profile(Hot | Warm |
Cold)]` as a lexical attribute that retargets a scope, described in
`arvo-always-optimal-internals.md`. Under this reading, putting the strategy in the value's type is a
category error, and the fact that the stack currently carries both mechanisms is a clue rather than a
redundancy.

Six and seven are the ones I would have skipped if I had been permuting, and they turned out to be
where the derivation went.

## 3. The derivation

### 3.1 The spine, from two intents

I9 and I8 together very nearly define the thing.

I9: "strategies are the variables that change what the 'correct' answer is for what we choose as the
path" (`INTENTS.md:162-163`). So a strategy is upstream of correctness: it is not a choice among
implementations of one specification, it is part of the specification.

I8: all strategies are decided by measurement, they measure different things, and they weigh
different measurements differently (`INTENTS.md:136-139`).

Put together: **a strategy names a preference over outcomes, and that preference is what makes one
answer correct rather than another.** Not a set of implementation choices. A criterion by which
implementation choices are made.

Test it against the rest.

I5 (Hot may sacrifice soundness for a proven meaningful gain): a weighting that puts near-zero weight
on the answer and high weight on cycles, with "proven" meaning the gain is measured rather than
asserted. Fits.

I6 (Cold minimises storage, has leeway on efficiency, and "can use the same paths Hot uses, not
because it needs to by intent, but nothing in its intent would fight it", `INTENTS.md:117-119`): this
sentence is the strongest single piece of evidence for the whole model, and I want to dwell on it.
It is unintelligible if each strategy owns its own implementations, because then "using the same
path" would be duplication. It is obvious if there is **one shared set of arms and each strategy is a
choice function over it**: two strategies pick the same arm exactly when their weightings agree in
that region, and nothing has to be written down to make that happen.

I7 (Precise is accurate "especially within chains and ops, not only alone", `INTENTS.md:126-127`):
does **not** fit, and that is section 5.

I17 (the storage-minimising path is not deprioritised): a statement that footprint is a first-class
measurement axis rather than a nice-to-have. Fits.

I13 (predicated arms composed, universal rejected, RATIFIED): falls out rather than being an extra
rule. If the weighting is evaluated per region and the winner differs by region, then what you ship
is a set of arms with const predicates. I13 is what the choice-function model produces, which is a
good sign for both.

### 3.2 The correction I had to make to my own model

There is a hole in "a strategy is an objective evaluated at compile time", and it is important.

**A const function cannot measure anything.** I8 says the strategies are decided by measurement, and
nothing at compile time can take a measurement. So the objective is not evaluated by the compiler at
all. What happens is:

The objective is evaluated **offline, on the bench harness, by a person**, who finds where each arm
wins under that weighting. What is evaluated at compile time is **the predicate naming the region the
answer came out**. That is exactly I13, and it is exactly what `mock/benches/` is full of: 200-odd
committed findings files whose whole job is to say which arm wins where.

This reconciles I8, I13 and I15 without strain. The objective is real and it is about measurements;
the measuring happens on the harness; the result is a predicate; the predicate is const; one path is
lowered. Op's later specification that "the above collapses to whatever is available at const time"
(`INTENTS.md:238-240`) is about the predicate, not about the objective, and the distinction matters:
the predicate is const-bounded, the objective is not bounded at all because it never runs.

So the mechanical definition, as distinct from the intent:

> A strategy is a **compile-time choice function over a shared set of arms**, whose value at each
> region was decided offline by measurement under a stated weighting.

**Probe A establishes that this is expressible and that it erases.** Three arms written once, three
strategies contributing nothing but an associated const indexed by the region, one generic dispatcher.
On the pinned nightly, edition 2024, `-O`, with **zero feature gates**, each entry point compiles to a
single tail branch with no compare and no conditional
(`94_probes/a_choice_function.out.txt`):

```
_entry_exact:      b  ...arm_accfit
_entry_footprint:  b  ...arm_minimum
_entry_speed:      b  ...arm_accfit_lanes
```

And the part I did not plan and would not have predicted:

```
_entry_speed_short = _entry_exact
```

The Speed strategy at arity 4 selects the same arm the Exact strategy selects, so the linker aliased
the two symbols. That is I6's sentence, mechanised, with the duplication cost measured at zero.

Note the shape, because it matters for what the canon can say: the region sits in the trait's own
parameters rather than in a const function's arguments, so the derivation lives in an impl where
arbitrary const expressions are legal and the bound names a contract. That is
`a-refused-bound-wants-a-trait-not-a-feature.md` applied before hitting the wall rather than after.

**Finding W1.** A strategy is expressible as a compile-time choice function over a shared arm set;
the selection erases entirely; and two strategies whose choice functions agree in a region collapse to
one emitted body at zero cost.
*Holds for:* `W = 13, ARITY in {4, 64}, arms = 3, signedness unsigned, target = aarch64-apple-darwin,
target features baseline, rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024, opt = -O,
panic = abort, feature gates = 0, threads any.` The `threads any` is claimed rather than omitted
because the finding is about an emitted static artifact, whose content does not vary with the
program's runtime thread count; a reader who rejects that reasoning should read it as `threads = 1`.

### 3.3 The const-availability limit, which W1 quietly dodged

Probe A and probe F both select on a const arity. Real folds take a slice whose length is a runtime
value. So: what happens when the region fact the choice function reads is not const-available?

I15 forbids a runtime check, so there are only two honest answers, and probe G shows them side by
side. The same selection rule, over the same two arms, with the arity const in one case and runtime
in the other (`94_probes/g_const_availability.out.txt`):

```
_site_const_long:    mov w1, #64
                     b   ...arm_lanes

_site_runtime:       cmp x1, #15
                     b.ls LBB4_2
                     b    ...arm_lanes
LBB4_2:              b    ...arm_seq
```

Two instructions and zero compares against four instructions and a compare. The runtime version is a
runtime check, which I15 forbids in those words: "Never any runtime checks, ever."

**So the choice function's domain is bounded by what is const, and the bound is real.** Two arms
follow, and they are arms rather than a policy question:

Where the region fact is const, which under I14's "sizes are const" is the normal case for anything
arvo declares, the full choice function is available and erases.

Where it is genuinely runtime, the choice function may not read it. The arm is then selected on the
const facts alone, and it is selected for correctness rather than for profitability. That is not a
degraded version of the same thing, it is a different and weaker predicate, and the honest statement
is that the selection is made with less information rather than that it is made later.

There is a distinction hiding in there that I think the canon will want, and I have not seen it named
anywhere: **a correctness predicate must be const, and a profitability predicate merely wants to be.**
Choosing the lane arm when the law does not hold is wrong at every length. Choosing the lane arm when
the run is too short is slow and right. The first can never be deferred; the second can be resolved
pessimistically without lying.

**Finding W2.** A selection on a const region fact erases to a direct branch; the identical selection
on a runtime fact emits a compare and a conditional branch, which I15 forbids. The choice function's
domain is therefore the const facts.
*Holds for:* `ARITY in {4, 64} const and ARITY runtime, arms = 2, signedness unsigned, target =
aarch64-apple-darwin, rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024, opt = -O, panic =
abort, threads any (compile-time artifact, as W1).`

## 4. Is "strategy" one thing?

No, and this is the part of the derivation I would most like attacked.

### 4.1 Three components, and they are not the same kind of thing

Pull apart what actually varies when the marker changes.

**Cost.** Which of the shared arms to select in a region. Answered by measurement, offline, per I8.

**Policy.** What the operation does at the declared boundary: wrap, clamp, round, or widen so the
question does not arise. This is not a cost question at all. It changes the function being computed,
not the way it is computed.

**Licence.** Which algebraic rewrites may be invoked. This is not a cost question either, and it is
not a policy question: it is a permission, and permissions are what make a rewrite sound or unsound
rather than fast or slow.

The shipped design fuses all three into one marker (`mock/PRINCIPLES.md.tmpl:288-292`), which is why
there is a "set" of four and why the set feels closed even after op said it is not. Unbundle them and
the question "how many strategies are there" stops being well formed: there is no set, there is a
product, and a named marker is shorthand for a point in it. Which is the word op used: **preset**.

**Probe B establishes the unbundling is expressible and cheap.** Three independent traits, four named
presets recovered as points, plus a fifth point nobody named in advance (`ColdExact`: the
storage-minimising carrier with the accuracy-first licence, which under a closed set of four has no
spelling at all). Zero feature gates. Five points emit three distinct bodies
(`94_probes/b_unbundled.out.txt`):

```
_p_cold:        b  ...arm_seq_sat
_p_hot:         b  ...arm_lanes_wrap
_p_precise:     b  ...arm_wide_sat
_p_cold_exact = _p_cold
_p_warm       = _p_hot
```

And the affordability number, which is the one that decides whether opening the axes is a real option:

```
four named points  :  3232 bytes,  9 text symbols
plus one unnamed   :  3240 bytes, 10 text symbols
```

Eight bytes and one symbol. **The emitted cost tracks the number of distinct arm selections the
consumer actually instantiates, not the size of the product and not the number of names.** That is the
whole affordability argument for opening the axes, and it is the argument the closed set was
implicitly resting on.

**Finding W3.** The three components are separable, expressible with no unstable features, and the
named markers are recoverable as points in the product. Adding a point outside the named set costs one
alias symbol.
*Holds for:* `points = 5, axes = 3, region W = 13 and ARITY = 64, arms = 4, signedness unsigned,
target = aarch64-apple-darwin, rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024, opt =
-O, panic = abort, feature gates = 0, threads any (as W1).`

### 4.2 Licence is not one bit, and this is where probe C landed hardest

I expected "may I be clever" to be a single permission. It is not, and the measurement is
unambiguous.

Probe C decides two different laws exhaustively over their whole small domains, swept over the model
width so the region is not pinned to one convenient choice (`94_probes/c_retraction.out.txt`).

**Retraction**, which licenses applying the boundary policy at each step rather than once at the end:

```
policy     chain          verdict
saturate   add>add        RETRACTS at every swept W
saturate   add>sub        retracts at no swept W      (49.61% of triples differ at W=8)
saturate   add>mul        RETRACTS at every swept W
saturate   sub>add        retracts at no swept W      (49.61%)
saturate   sub>sub        RETRACTS at every swept W
saturate   sub>mul        RETRACTS at every swept W
saturate   mul>add        RETRACTS at every swept W
saturate   mul>sub        retracts at no swept W      (96.62%)
saturate   mul>mul        RETRACTS at every swept W
wrap       (all nine)     RETRACTS at every swept W
```

**Associativity**, which licenses splitting a fold into lanes:

```
saturate   add            ASSOCIATES at every swept W
saturate   sub            associates at no swept W    (82.75% at W=8)
saturate   mul            ASSOCIATES at every swept W
wrap       add            ASSOCIATES at every swept W
wrap       sub            associates at no swept W    (99.22% at W=8)
wrap       mul            ASSOCIATES at every swept W
```

Look at `wrap`/`sub`. It **retracts** and does not **associate**. So the two permissions are logically
independent: knowing one tells you nothing about the other, and a design carrying a single "clever"
bit would have to take the conjunction and lose the arm that only needs the weaker permission.

The saturation pattern is worth naming because it has a shape rather than being a list. Saturation
retracts on exactly the chains that are monotone in one direction, and fails on exactly
`add>sub`, `sub>add` and `mul>sub`. The reason is mechanical: a clamp may be applied early only if the
rest of the chain cannot move the value back across the boundary it clamped at. Subtraction after an
upward operation is the escape; subtraction after subtraction is not, because the lower clamp at zero
is absorbing. So the criterion is not a table, it is a predicate over the chain shape, and it is
compile-time visible wherever the chain is.

Wrapping retracts on all nine because it is a ring homomorphism, which is a theorem rather than a
measurement; the exhaustive run confirms the implementation matches the theorem, which is what a check
of a known law is for.

**Finding W4.** Retraction and associativity are independent permissions with different regions. A
strategy's licence component is a vector of permissions, not a bit.
*Holds for:* `W in {2, 3, 4, 5, 6, 8}, F = 0, signedness unsigned, policy in {saturate, wrap},
operations in {add, sub, mul}, chain length = 2 operations for retraction and arity = 3 for
associativity, values exhaustive over the declared domain, threads any (a numeric identity over
values, independent of execution).`

**Finding W5.** Saturation retracts on `{add>add, add>mul, sub>sub, sub>mul, mul>add, mul>mul}` and on
none of `{add>sub, sub>add, mul>sub}`. Wrapping retracts on all nine. Saturation and wrapping both
associate on `add` and `mul` and on neither `sub`.
*Holds for:* the same predicate as W4.

I did not measure distributivity, and by the notation that means I am claiming nothing about it.

### 4.3 A defect in my own probe, recorded because it is instructive

The first revision of probe C part 1 swept only homogeneous chains: `add>add`, `sub>sub`, `mul>mul`.
Every policy "retracted" under it, at 0 differing triples out of 16777216, and I nearly wrote that
down as the finding.

It is setup that helps, in the exact sense `the-test-gate.md` names. A chain monotone in one direction
can never escape a clamp, so the clamp's position cannot matter, so the test could not have failed.
The full 3x3 matrix is what produced W5, and the three failing cells are all mixed. I am recording
this because the corrected probe is committed and the wrong one is not, and a reader should be able to
see that the correction happened rather than infer that I got it right first time. The note is in the
probe's own header (`94_probes/c_retraction.rs:24-27`).

## 5. Chain strategies are a different kind of thing, and I7 is one

Here is the place where the choice-function model breaks, and I think it is the most useful thing in
this file.

I7 says Precise is "the most precise possible answer... especially within chains and ops, not only
alone". That clause is not decoration. **An objective over a chain cannot in general be implemented by
a per-operation selection rule**, because the composition of locally optimal choices is not the
globally optimal choice. Probe C part 2 gives the exact region:

```
policy       W   F      triples         differ       pct   max |diff|   verdict
truncate     8   0    16777216              0     0.00%            0   RETRACTS
truncate     8   1    16777216        4136960    24.66%           64   does not retract
truncate     8   4    16777216       11988992    71.46%           15   does not retract
truncate     8   8    16777216        3969848    23.66%            1   does not retract
nearest      8   0    16777216              0     0.00%            0   RETRACTS
nearest      8   4    16777216       11514240    68.63%            8   does not retract
```

Swept over `W in {4, 6, 8}` and `F in 0..=W`, for both truncation and round-to-nearest: **rounding
retracts exactly at `F = 0` and nowhere else.** Every row with `F > 0` fails, at every swept width, for
both rounding modes.

So for any fractional fixed-point type, a per-operation rounding marker cannot deliver chain accuracy.
Not approximately, not usually. The region where it works is `F = 0`, where "rounding" is not an
operation at all.

**Finding W6.** A rounding policy applied per operation retracts over a multiply chain exactly at
`F = 0`.
*Holds for:* `W in {4, 6, 8}, F in 0..=W, signedness unsigned, operation = multiply, chain length = 2
multiplies, rounding in {truncate, nearest}, values exhaustive over the declared domain, threads any
(numeric identity).`

### 5.1 Attacking it rather than reporting it

That is a blocker on I7 as stated, so the job is to find the construction that serves it. Probe E
tests the obvious one from exact fixed-point arithmetic: **do not quantise in the interior.** Let the
declared width grow through the chain and quantise once, where the consumer asks.

Under that construction, "accurate within chains" stops being a policy and becomes a representation
discipline, and the only remaining questions are what it costs and where it runs out. Both are
answerable exactly.

Accuracy first (`94_probes/e_chain_exactness.out.txt`, a deterministic stride sweep of 9363 chains at
`W = 16, F = 8`, and I am labelling it a sweep because the domain of a 5-tuple over 2^16 is 2^80):

```
  k   per-op != exact   per-op max ulp   widen != exact   widen max ulp
  1                 0                0                0               0
  2              4684                1             4684               1
  3              9041              250             4681               1
  4              9216            53222             4802               1
  5              9252         11922158             4759               1
```

Per-operation rounding diverges by up to **11,922,158 ulps** at a five-multiply chain. The widening
construction stays at 1 ulp, and that residual is not accumulated loss: it is truncation against a
round-to-nearest reference at the single final quantisation, which is a choice of rounding mode rather
than interior error.

The mechanism behind the explosion is the same one probe D found independently, and I like that they
corroborate from different directions: an operand's error is **scaled by the other operand**, so
losing one ulp early and then multiplying by values up to 256 three times gives you 256^3.

The cost is the width growth, and it is exact rather than empirical. A chain of `k` multiplies at
`I.F` grows to `kI.kF`. A fold of `n` additions grows to `(I + ceil(log2 n)).F`. So the availability
predicate against a 128-bit widest rung:

```
 start I.F   total   max mul-chain k   max add-fold n
       1.7       8                16   unbounded in practice
       8.8      16                 8   unbounded in practice
      4.12      16                 8   unbounded in practice
     16.16      32                 4   unbounded in practice
     32.32      64                 2   unbounded in practice
      13.0      13                 9   unbounded in practice
```

**The add fold is essentially free and the multiply chain is what runs out**, linearly in the chain
length, and the chain length is compile-time visible wherever the chain is written out. That is a
clean const predicate for gating the arm.

**Finding W7.** Chain accuracy is achievable by never quantising in the interior, at a width cost of
`k` times the declared width for a `k`-multiply chain and `ceil(log2 n)` extra integer bits for an
`n`-term fold. Against a 128-bit widest rung the multiply chain is the binding constraint.
*Holds for:* `W = 16, F = 8, signedness unsigned, operation = multiply, chain length k in 1..=5,
deterministic stride sweep rather than exhaustive, threads any (numeric identity)`, with the width
arithmetic itself derived rather than measured and instantiated at `I.F in {1.7, 4.4, 8.8, 4.12,
16.16, 32.32, 1.15, 13.0}`.

### 5.2 What that means for the shape of the answer

Two kinds of strategy, and they are structurally different rather than differently parameterised.

**A local strategy** has an objective that is a function of one operation's inputs and outputs. Its
selection composes freely, because per-operation optimality is chain optimality. Hot, Cold and Warm as
op states them are all local: I5 is about cycles, I6 about bytes, I3 and I4 about behaving as a native
primitive, and none of the three mentions a composition.

**A chain strategy** has an objective over a composition. Its selection composes only where the policy
retracts, and where it does not, the strategy needs the chain in view. Precise as op states it is a
chain strategy, and it is the only one in the current set.

This is a much more interesting decomposition than four names, and it is derived from op's own
wording rather than imposed. It also explains something that would otherwise look like an
inconsistency: it is not that Precise is "more" of the same axis than Warm, it is that Precise is
quantified over a different thing.

The design consequence is not that Precise needs a special mechanism nobody else uses. It is that
**the chain has to be expressible**, and section 5.1 says the cheapest way to express it is to let the
type carry the grown width and let the consumer say where the quantisation happens. Under that, the
chain is visible because the widths are, and no expression-tree machinery is needed.

## 6. What relates two strategies, and why I could not settle it

`mock/DESIGN.md.tmpl:35` names `Resolve<S1, S2>`, and `arvo-toolbox-not-policer.md` describes its
behaviour: a cross-strategy operation resolves toward the more conservative side, with a warning. That
is a join over an order on the markers.

I went looking for the order, and I do not think it is over the markers.

**Objectives are not ordered.** Hot weighs cycles, Cold weighs bytes. Neither is more conservative
than the other; they are incomparable weight vectors, and any ordering imposed on them is extra
structure rather than a fact about them.

**Guarantees are ordered, naturally.** Guarantee A is below guarantee B when B's promises imply A's.
So if there is a lattice, it is over what the value is worth, not over what the strategy prefers.

Which raises the question probe D was built for: **what is the resolved marker a claim about?** There
are two readings and they point opposite ways.

Under the **operational** reading, the marker says what this operation does. Join is sound, because
the addition really was performed under Precise's policy.

Under the **value-level** reading, the marker says what this value is worth. Join is unsound, because
an operand that already lost accuracy cannot have it restored by a later operation's policy.

Probe D measures the gap (`94_probes/d_resolution.out.txt`). Take one operand off a lossy path,
combine it with an exact one under an operation performed exactly:

```
case 1: y = a + b, addition performed EXACTLY, `a` off a lossy path
  drop     y != exact   max |err| (ulp)   max |err| in a
     0              0                 0                0
     1         225044                 1                1
     4         421894                15               15
     8         448310               255              255
```

The result's error column and the operand's error column are equal at every row. The operation had no
opportunity to introduce or remove any of it. A type saying Precise on that result is saying something
false under the value-level reading and something true under the operational one.

**And then case 3 killed the whole framing for me**, which is why this section ends without an answer:

```
case 3: does the operand's ulp bound bound the RESULT's ulp error?
        op   drop      operand bound     max result err   meet tight
       add      1                  1                  1          yes
       add      8                255                255          yes
       mul      1                  1                256          NO
       mul      2                  3                766          NO
       mul      8                255              65055          NO
```

**An ulp bound survives addition and does not survive multiplication**, because the operand's error is
scaled by the other operand. So the guarantees, stated as bounds a finite marker set could carry, are
**not closed under the operations the type supports.** There is no lattice. The meet is not merely
conservative, it is unsound too, in the sense that no fixed marker from a finite set can be attached
to the result of a multiplication and be right.

**Finding W8.** An operand-level accuracy bound expressed in ulps is preserved by addition and is not
preserved by multiplication, by a factor up to the other operand's magnitude.
*Holds for:* `W = 16, F = 8, signedness unsigned, operations in {add, mul}, operand loss in {1, 2, 4,
8} dropped fraction bits, stride sweep over the declared domain rather than exhaustive, threads any
(numeric identity).`

So where does that leave resolution? With a fork I am not going to pretend to close.

**Option R1: the marker is operational, and join is correct.** The marker never claims anything about
the value, only about the last operation. Cheap, matches the shipped behaviour, and the cost is that
I7's chain accuracy is not tracked by the type at all: a chain of exactly-performed operations on a
degraded operand carries the strongest marker in the set. Under this option the marker must never be
read as a numeric property by any consumer, and that prohibition has to be stated in the canon or it
will be violated within a year.

**Option R2: the marker is value-level, and join is wrong.** The meet is what a mixed operation gives,
so a mixed expression degrades toward the weakest operand. Honest about what the value is worth, and
it surprises everyone the first time, and W8 says it is still not sound for multiplication because
the bound itself does not survive.

**Option R3: accuracy is not a marker at all.** W8 says a finite marker set cannot express a bound
that composes, so a genuine value-level accuracy claim needs a **carried quantity** that grows, which
is exactly the widening construction of section 5.1: the declared width **is** the accuracy claim, it
composes exactly, and it needs no lattice because arithmetic on widths is the lattice. Under this
option there is no accuracy strategy, there is a representation discipline, and `Resolve` is only ever
about policy, where R1's join is unproblematic.

I lean toward R3 and I am not going to dress that as a conclusion, because I have one probe on it and
one instance decides nothing. What would distinguish them is a question about consumers rather than
about arithmetic: **does any consumer ever read the strategy marker on a value and conclude something
numeric about that value?** If yes, R1 is unsafe as it stands and needs the prohibition written down.
If no, the marker is operational, R1 is fine, and the accuracy question is R3's and belongs to the
width rather than to the marker. I cannot answer that from arvo alone. It is a fact about
hilavitkutin, vehje and loimu.

**And one shape I am explicitly not proposing, because it is already refused:** making a mixed-strategy
operation a compile error and requiring an explicit cast. `arvo-toolbox-not-policer.md` names "Refuse
to compile cross-strategy ops 'for safety'" as an incorrect shape in those words. I considered it,
because I15's "we catch invalids on compile time" makes an ambiguous resolution look like an invalid,
and the rule refuses it. Recording that I went there and turned back rather than leaving it for
someone else to walk into.

## 7. Where each component is carried, which I think is the sharpest thing here

If the three components are separable, the next question is which of them belongs on the value.

**Policy travels with the value.** A value whose overflow semantics is wrapping is still wrapping when
it is passed to a function that knows nothing about where it came from. The callee cannot supply it.
It has to be in the type.

**Cost is a property of the site.** Which arm is cheapest depends on this loop's arity, this target's
features, this access pattern. The value has no opinion about any of that.

**Licence is a property of the site too.** Whether this fold may be split into lanes is a fact about
the fold, not about the numbers in it.

So **only one of the three needs to be in the type**, and the stack already contains the evidence: the
lexical `#[profile(Hot | Warm | Cold)]` attribute from notko, described in
`arvo-always-optimal-internals.md`, sits alongside arvo's type parameter and nobody has said why there
are two mechanisms. On this reading they are not redundant, they are carrying different components,
and the reason `#[profile]` has three tiers against arvo's four (that rule notes Precise has no tier)
is that the fourth is the one that is not a site property.

Probe F tests the claim directly. Policy on the value's type, cost and licence at the call site. Four
sites, three of them sharing one value type (`94_probes/f_where_carried.out.txt`):

```
_site_tight:          mov w2, #8191 ; b ...arm_tight_sat
_site_faithful:       mov w2, #8191 ; b ...arm_wide_sat
_site_fast:           mov w2, #8191 ; b ...arm_lanes_sat
_site_fast_wrapping:  mov w2, #8191 ; b ...arm_lanes_wrap
```

Zero conditional instructions in every body. Zero casts in the source. Three different arms for one
value type, and the policy still travels: `site_fast` and `site_fast_wrapping` run the same plan and
get different arms because the values differ.

**That last property is the discriminator against type-carried cost.** Under type-carried cost,
folding the same data two ways at two sites requires changing the value's type, which means a cast.
The cast is free at runtime and is not free in the design: it puts a conversion in the source saying
the value changed when the only thing that changed was the plan, and it means every function signature
that takes a numeral has to name a cost policy it has no opinion about.

**Finding W9.** Policy carried on the value and cost plus licence carried at the site reaches one
lowered path with no conditional and no cast, and the same value type folds three different ways at
three sites.
*Holds for:* `W = 13, signedness unsigned, policy in {wrapping, saturating}, plans = 3, arms = 6,
target = aarch64-apple-darwin, rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024, opt =
-O, panic = abort, feature gates = 0, threads any (as W1).`

I want to flag the counter-argument, because it is strong and I am not dismissing it. **A preset exists
so a consumer states one intent and does not answer three questions.** I2 and I4 both stress the
intent and the intuition, and `arvo-toolbox-not-policer.md` says defaults are good. Splitting the axes
across two carriers means a consumer who just wants "fast" has to say it in two places. The answer is
presumably both: the axes exist and are addressable, and named presets bind a point across both
carriers so that `Hot` still means one thing at a call site. That is a composition rather than a
winner, and W3's eight bytes says the composition is affordable.

## 8. What this says about the set of four

Under the derivation above, the question "how many strategies are there" has no answer, and that is
the good outcome rather than an evasion.

There is a product of components. Each component's own space is small and open: policies are however
many boundary behaviours the design supports, licences are a vector of permissions with one bit per
law the design cares about, and cost is however many measurement weightings anyone has bothered to
name. A **named strategy is a point** in that product, chosen because it is worth naming. W3 says a
new point costs one alias symbol when it duplicates an existing selection and one body when it does
not.

So I1's demotion is not a hole to be filled by finding the right number. It is the observation that
the number was never a design parameter.

**Keeping is a result, and here is what I would keep.** The four names are good names for four points
people actually want. Op's four intents (I5, I6, I3 and I4, I7) are four coherent objectives, they
cover the plausible span, and nothing in my derivation says any of them should go or be renamed. What
I would change is what a name *is*: a binding of a point rather than a member of a closed set, so that
a fifth point does not require a design round. Rewrite cost matters and this change is additive: every
existing spelling keeps working.

## 9. So, what is a strategy

Stated as intents, in the register a canon would use, offered as a suggestion rather than a ruling.

A strategy is a **stated preference over outcomes** that determines which of several correct-by-
construction implementations of an operation is the right one, and in some cases determines which
answer is correct at all. It is not a set of implementations and it does not own any code.

It is realised as a **choice function over a shared set of arms**, total on the const facts of a
region, whose value at each region was decided offline by measurement under that preference. The arms
are shared: two strategies select the same arm exactly when their preferences agree there, and that
costs nothing.

It has **three separable components**. What the operation does at the boundary, which travels with the
value because no consumer can supply it. Which measurement is weighed, which belongs to the site
because only the site knows the workload. Which rewrites are permitted, which belongs to the site
because a permission is about a computation rather than about a number, and which is a **vector** of
permissions with a region each rather than a single bit.

A **named strategy is a point** in the product of those components, named because it is worth naming.
The set of names is open and a new name costs one symbol.

Two strategies are related by an order on their **policies**, which exists, and not by an order on
their **preferences**, which does not. Whether they are also related by an order on accuracy is open,
and the evidence says a finite marker set cannot carry an accuracy claim that survives multiplication.

A strategy whose preference is over a **chain** rather than an operation is a different shape and needs
the chain in view. The cheapest way to put it there is to let the declared width grow and quantise
once, under which chain accuracy is a representation discipline rather than a policy.

## 10. What I did not do, and what I could not settle

Coverage, bounded honestly rather than claimed complete.

I read `INTENTS.md` and `RULES.md` in full, and arvo's three root `.md.tmpl` documents in full. I read
`mock/Cargo.toml` and `mock/benches/Cargo.toml`. I listed `mock/benches/` and read the test bodies in
the strategy-relevant shared variant crates. **I did not read a single findings file** in
`mock/benches/`, of which there are over two hundred, and every one of them is committed harness
output that could price something I have left unpriced. That is the largest gap in this file and it is
deliberate: reading them is a phase-two act, because several are cited throughout the panel and I did
not want the panel's framing arriving through them.

**Everything about cost in this file is unpriced.** I have taken no timing measurement, on the harness
or off it. Every emitted-code observation here is a shape or a count from an ad-hoc quick spike, and
where I have said an arm is "cheapest" I am describing what a weighting would select, not what a
measurement found. The bench harness is where that question lives and I did not run it.

What I could not settle, stated as a concession rather than dressed as a finding:

**Which direction a mixed-strategy operation resolves in.** Section 6. I found three options and a
measurement that says the value-level lattice does not exist, and the discriminator is a fact about
consumers that arvo cannot answer from inside itself. I would want someone who has read
hilavitkutin's and vehje's use of the marker to answer whether any consumer reads it as a numeric
property. That is a different specialism from mine.

**Whether the site-carried cost is ergonomically acceptable.** W9 says it works and costs nothing at
the machine level. Whether a consumer will tolerate naming a plan at call sites is a taste question
and op's, and it is exactly the sort of thing I2 and I4 are about.

**Whether the licence vector's permissions are the right ones.** I measured retraction and
associativity because they are the two the arms in front of me needed. Distributivity, commutativity,
the identity and zero laws, and monotonicity are all plausibly separate permissions with separate
regions, and I measured none of them, so by the notation I am claiming nothing about any of them.

Routes I opened and closed inside the work, for whoever comes next:

The first probe A used a `const fn` in a trait, which needs `const_trait_impl`. Moving the region into
the trait's own const parameters made it stable-expressible with zero gates, and that is why W1 can say
"feature gates = 0". A later expert reaching for a const-callable trait method to express a selector
should try the indexed-trait shape first.

The first probe C part 1 swept homogeneous chains and produced a clean, wrong, universal answer.
Recorded in section 4.3 and in the probe header.

Probe D case 2 originally carried a control comparing an expression to itself. It is deleted rather
than repaired, because there was nothing there to repair; the real control is the `drop = 0` row,
which is in the table.

Until next time, happy hacking.

---

## Phase two: reconciliation

Appended after reading the panel. Phase one above is unedited.

**Not yet written.** This section is the second commit.
