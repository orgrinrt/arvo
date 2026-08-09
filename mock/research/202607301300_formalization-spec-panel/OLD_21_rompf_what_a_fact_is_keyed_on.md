# 21: What a fact is keyed on

**Member:** Tiark Rompf. Staging and binding-time analysis: for any computation, which part is known
now and which is deferred, and what the boundary between the two is allowed to see. Lightweight Modular
Staging, the collapse of interpreter towers, reachability types. The habit I bring is that a value's
stage is a property of the term that computes it, never of a declaration written next to it.

**Position:** eighth member of the algebra dive, file 21. Not a synthesis. The dive continues.

**What I read.** The brief's five op files (`16b`, `16c`, `16d`, `17b`, `13c`) first, as instructed.
`11_current_shape_draft.md` in full. Then `18_lamport_say_what_is_claimed.md` and
`19_ringer_the_witness_and_its_upkeep.md` in full, since the question is theirs. Then
`17_orchard_are_these_all_grades.md` sections 5 and 6 line by line, `20_wingo_the_build_layer_contract.md`
sections 5 and 6, and `13_mcsherry`, `14_dolan`, `15_willsey`, `16_fallin` by their section headings and
the passages the later files cite. I listed the panel directory before reading inside it and confirmed
nothing postdates file 20. On source I read almost nothing, per `16b`: the test suite, the nine ignored
tests, `mock/crates/arvo/src/aliases.rs:36-45`, and greps.

**What I compiled and ran, as distinct from what I reasoned about.** Five probe files at `21_probes/`,
all `rustc -O` on `nightly-2026-05-28` (`1.98.0-nightly (57d06900f)`), no `#![feature(..)]` gate opened
anywhere. Sections 3, 4, 5, 6 and 7 are measurements and each cites the probe that produced it. Sections
1, 2, 8, 9, 10 and 11 are argument. Where I hold more than one reading I say so and leave the choice
where it belongs. No timing, no performance claim, so nothing here belongs in `mock/benches/`.

**The test gate, before the assigned work.** `cargo test --workspace` in `arvo/mock`: **654 passed, 0
failed, 9 ignored**, reproducing files 18, 19 and 20 exactly. I read the bodies of the ignored nine
rather than their names. One is a real catalogued red and is exactly right:
`arvo/tests/fixed_point_div.rs:111`, `#[ignore = "catalogue: >64-bit-logical fixed-point divide needs
256/128 long division; tracked #5"]`, which is `catalogue-edge-cases-as-tests.md` working as designed.
The other eight are doctests, and they are a finding I report below rather than a gate failure. The
surface this file is about, law keys, has no tests because it has no implementation: `Magma`,
`Semigroup`, `Monoid`, `AddAssoc`, `Associative`, `Monotone`, `Faithful`, `Kleene` and `LatticeClosed`
each return zero hits over `mock/crates/`. There is nothing to audit and nothing to refuse over.

**One unlicensed thing I noticed, outside my lens, reported per the standing instruction.** Four of the
eight ignored doctests are on `arvo::Fixed`, `arvo::Signed`, `arvo::Uint` and `arvo::Int`, and they are
fenced ` ```ignore `, meaning rustdoc neither compiles nor runs them. `aliases.rs:41-43` shows the shape:

```rust
/// ```ignore
/// type Angle = arvo::Fixed<9, 7, Warm>;        // 9.7 unsigned
```

These are the aliases the spec's own diagnostics work calls the intended consumer-facing spelling
(`11_current_shape_draft.md:426-430`). A doc example that does not compile is a claim about the consumer
surface with nothing behind it, which is the same species as the `Monotone` claim file 18 caught at
`11_current_shape_draft.md:699-703` and is in the same place: prose about the design, checked by nobody.
It is cheap to fix and I would fix it, but it is not mine.

---

## 0. Two premise checks, and the first one narrows my own brief

The dive's record is that briefs and drafts here have carried false claims that later members then
reasoned from, so I checked mine before building on them.

**File 18's measurement reproduces, at two arities it did not test.** I rebuilt the accumulator sweep
from scratch rather than cite it. `21_probes/01`, signed three-bit numeral, exhaustive: signed saturating
addition is not Kleene-associative at accumulator scale 1 and is at scale 3, at both arity 3 and arity 4.
`21_probes/02` extends it to arities 3 through 6 and accumulator scales 1 through 8. The finding this
dive is extending is real and I am reasoning from a re-run.

**My brief overstates file 19, and the overstatement merges two different failure modes.** The brief says:

> File 19 then generalised it: a derived fact whose key omits a dimension its proof actually used is the
> mechanism behind every silent break found in this dive. Not some of them, every one.

That is not what file 19 says. Its generalisation is at `19_ringer...md:459-462`, and the thing it
generalises is **an omitted check**, not an omitted key:

> this is true whether the omitted check is a `phi` call nobody wired in (Thread C's original disease), a
> fidelity assert nobody put in the worker (section 2), a `LIBERTIES` array nobody related to a body
> (section 2a), or a grep nobody ran on a claim that read as background (this section).

Three of those four are reachability failures: a check exists and something reaches past it. Only the
third, the declared liberty set never related to the body, is arguably a key failure. So the two
mechanisms are distinct and they want distinct fixes:

| failure | what went wrong | the fix that closes it |
|---|---|---|
| unwired check | a check exists, a path reaches the branch without passing through it | file 19's door: recompute at the branch, so there is no path around it |
| omitted key | the check runs, and quantifies over less than the proof needed | this file: make the key the signature, so the quantifier is visible |

A door does not close a key omission, because a correctly reached check of a wrongly keyed fact is still
wrong, and it now fires confidently. And a complete key does not close an unwired check, because a
perfectly stated fact nobody consults licenses nothing. **Building one mechanism for both is the thing to
avoid**, and the brief as written would send someone to do exactly that. I am answering only the second,
and section 6 is where I found the first reopening underneath my own answer to the second.

---

## 1. Keys are binding times, and three stages fall out of the design as drafted

The design already has stages. It has not written them down, and the two omissions file 18 found are at
exactly the two stage boundaries it has not written down.

A stage, here, is a point at which some parameters become known and stay fixed. There are three in the
design as drafted, and every derived fact belongs to exactly one of them.

**Stage T, when a type is written.** The consumer writes `UFixed<13, 3, Warm>`. Bound: the ten axes.
Fixed for the lifetime of every value of that type.

**Stage O, when an operation is applied.** The consumer writes `a + b`. Bound: which operation, and
therefore what the exact image of that operation on two representable values is. Also bound, though
nothing in this dive has connected it, the other operand's composition, which is what `Resolve<S1, S2>`
is for.

**Stage F, when a fold is run.** The consumer writes a reduction. Bound: the arity, the set of groupings
the combinator will use, and the numeral the running intermediate inhabits.

Sort the design's facts by the stage at which their last free parameter becomes known:

| fact | last parameter bound at | keyed by the draft at | verdict |
|---|---|---|---|
| number-system membership | T | T | correct |
| `AddClosed` (`Bias = Zero`) | T | T | correct |
| translation stability of `phi` | T | T | correct |
| structural class (homomorphism / partial identity / retraction) | **O** | T | one stage early |
| `AddAssoc` / regrouping agreement | **F** | T | two stages early |
| `Deterministic` | see below | T | quantifier is a stage index |
| `ConstantTime` | below every stage arvo has | T | no key exists |

The two errors file 18 found are the two rows marked wrong, and they are wrong by exactly one and exactly
two stages. That is not a coincidence and it is the reason I think the staging reading is worth having:
**a key omission is a binding-time error, in the precise sense that a fact was computed at a stage where
one of its inputs was not yet known.**

Two of the remaining rows sharpen under the same reading.

`Deterministic` is at `11_current_shape_draft.md:303-305` "a blanket marker keyed on the whole
composition", and file 18 section 7.4 says correctly that its quantifier is unnamed. Under the staging
reading the missing quantifier **is a stage index**: determinism across two runs of one binary, across
two builds of one source, and across two targets are three different claims at three different stages,
and only the first is below anything arvo's types reach. Naming the stage names the quantifier.

`ConstantTime` has no key because its last parameter binds at codegen, which is downstream of every stage
arvo has. File 18 section 7.5 says it is "a property of a different object". The staging reading says
which object: the next stage down, the one this tower does not reach. That is why it can only ever be a
measurement, and it is a reason rather than a stipulation.

**The reading I hold against this one, and it is not weak.** The three stages may not be a tower. A fold
`A × E → A` calls the operation on the accumulator's type, so in the dependency order the accumulator is
bound *before* the operation is selected, not after, which reverses stages O and F. If that is right, the
structure is a lattice of dependencies rather than an ordered sequence, and the claim weakens from "the
staging reading makes the right keys fall out" to the much smaller "the staging reading makes the
dependencies visible". I could not settle this from the measurements, and the difference matters for
anyone who wants to write the stages into the vocabulary rather than only use them to check keys.

**And a second reading that reorganises the whole picture, from file 20.** Wingo measured that
monomorphisation does not erase the composition, it prints it into the symbol
(`20_wingo...md:326-341`), and that the intent is legible "exactly at the granularity where the operation
survives as a function, and nowhere else" (`20_wingo...md:371-373`). Under that reading there is one real
stage boundary, monomorphisation, it sits below all three of mine, and my whole tower is one stage. That
is defensible and it is the reading that connects to the downstream contract, so section 11 is written in
it rather than in mine.

---

## 2. Omission is the symptom. Defaulting is the mechanism

This is the reframe I would most like carried forward, because it changes what a mechanism has to catch.

Neither of file 18's two findings is an omission in the ordinary sense. In both, the proof *used* the
parameter. It used a fixed value of it, silently.

File 18's own wording is exact and worth reading for the verb (`18_lamport...md:391-394`):

> It is a fact about a `(numeral, accumulator)` pair in which the accumulator **was silently taken to be
> the numeral itself**.

Taken to be. Not left out. The proof ranged over the accumulator; it ranged over a one-element set, and
the choice of that element was invisible because it was made by the code rather than by the signature.
The same is true of the operation, which file 18's own parameter table records as "implicit, and addition
is assumed throughout" (`18_lamport...md:81`).

So the failure is:

> **A parameter leaves a key by being defaulted, not by being forgotten.** A universally quantified claim
> is reported after a proof that instantiated the quantifier at one point.

Three consequences, and the third decides what a mechanism looks like.

**A key omission is invisible in the artifact that has it.** A law implementation keyed on the
composition alone reads as complete: every parameter it mentions is used, nothing is dangling, the
premise is satisfiable, the impl compiles. There is no dangling reference for a reader to notice, because
the missing parameter was replaced by a value rather than left as a hole. That is why two careful members
built on the `Monotone` claim and why the accumulator survived five files.

**The two directions of key error are not symmetric, and only one is unsound.** A key that omits a
parameter the verdict depends on asserts a false thing, at every setting other than the defaulted one. A
key that carries a parameter the verdict does not depend on asserts a true thing and demands more than it
needs, which is over-strictness. McSherry already measured the cost of over-strictness on this exact
surface and it is one map in 65536 against 1024 (`13_mcsherry...md:210-234`). That is a real cost and it
is a cost, not a lie. So a design here should **err toward key inflation deliberately**, and narrow the
key back only by an explicit, named, checked step. Under-keying cannot be narrowed back at all, because
nothing in the artifact records what was assumed.

**And it tells you what to check.** You cannot check for an omission, because there is nothing there to
inspect. You can check for a default, by evaluating the verdict at two settings of a candidate parameter
and seeing whether it moves. Section 3 is that check, and it is the general form of the sweep file 19
proposed at `19_ringer...md:546` after watching Lamport find the accumulator "by accident of curiosity
rather than by any standing rule".

---

## 3. Scope closes one direction, mechanically and for free

`21_probes/01_the_key_is_the_signature.rs`, four arms, all on the pinned toolchain, no feature gate.

The proposal is the smallest possible one: **a derived fact is a `const fn` whose parameters are its
key.** Nothing else declares a key, and there is no key annotation to keep in sync with anything.

Given that, the omission direction is closed by name resolution and nothing else. The `--cfg omit_the_key`
arm writes the proof body without binding the accumulator:

```
error[E0425]: cannot find value `SCALE` in this scope
   --> 01_the_key_is_the_signature.rs:142:20
    |
142 |     let lo = NLO * SCALE;
    |                    ^^^^^ not found in this scope
```

That is the whole of it, and its cheapness is the argument for it. There is no new trait, no witness, no
`E0046` obligation, no coherence surface, and no feature this workspace forbids. **The completeness of a
key, in the direction that matters, is the scope discipline the language already has, applied to a proof
that has been written as a term rather than as prose.**

The defaulting direction needs the sweep, and the sweep compiles too. The `--cfg no_sweep` arm is the
draft's own shape, a law keyed on the composition with the accumulator instantiated at the numeral:

```
the drafted key, compiled clean, accumulator defaulted and invisible:
  add_assoc_as_drafted::<SATURATE, 3>() = false
  ... and at an accumulator of scale 3 the same composition IS associative.
```

The `--cfg with_sweep` arm adds the claim that keying on `(R, ARITY)` alone implicitly makes, which is
that the verdict does not move with the accumulator, and evaluates it:

```
error[E0080]: evaluation panicked: this law's verdict moves when the accumulator moves, so the
accumulator belongs in its key; a fact keyed on the composition alone is asserting something false
   --> 01_the_key_is_the_signature.rs:175:9
    |
    | evaluation of `add_assoc_with_the_check::<1, 3>::{constant#0}` failed here
```

`<1, 3>` is `SATURATE` at arity 3. The `WRAP` call on the line above it compiled clean, because wrapping's
verdict genuinely does not move with the accumulator. So the check separates the composition that was
being lied about from the one that was not, names it in the error, and costs one const block.

The mechanism is file 19's, generalised off its own axis. Its section 1 found the wall
(`19_ringer...md:96`, "generic parameters may not be used in const operations") and then found the door
(`19_ringer...md:158-160`): a `const { .. }` block inside a function body is computation rather than a
const generic argument used to construct a type, so the ban on `generic_const_exprs` never applies to it.
File 19 built that door for the fidelity licence. It is the same door, and it works for any fact whose
key is a parameter list.

**What this does not close, and it is the real limit.** The sweep needs somebody to name the candidate
parameter. Section 6 is what happened when I named the ones I knew about and missed one.

---

## 4. The accumulator is a threshold, not a dimension, and it decides whether the recovery map is in the key at all

`21_probes/02_the_threshold_is_predicted_by_the_class.rs`. Signed three-bit numeral, values `[-4, 3]`,
accumulator at scale K holding `[K*-4, K*3]`, exhaustive over every tuple and every grouping.

File 18 left the accumulator with two readings and resolved neither (`18_lamport...md:409-424`): a
combinator parameter, or an eleventh `Policy` axis. Both put a free dimension in the key, and both would
be searched over. I went looking for a third and the measurement gave me one.

First, the structural class, and its stability. The class is what file 18 section 4 defines, and it does
not move with the accumulator:

```
resolution             class               monotone
Wrap     (Hot)         homomorphism           false
Saturate (Warm/Cold)   retraction              true
Refuse   (Precise)     partial identity        true
SubZero                none of three          false

class stability across accumulator scales 1..8:  stable = true, all four
```

Then the threshold, meaning the smallest accumulator at which every grouping of an n-element fold agrees
under Kleene equality:

```
resolution                 n=3     n=4     n=5     n=6
Wrap     (Hot)              K1      K1      K1      K1
Saturate (Warm/Cold)        K2      K3      K4      K5
Refuse   (Precise)          K2      K3      K4      K5
SubZero                     K2      K3      K4      K5
```

**I predicted this from the structural class and monotonicity, and the measurement refuted the
prediction.** My reasoning was that a monotone recovery has clamped paths that converge, so `Saturate`
and `Refuse` should reach agreement strictly earlier than a non-monotone one. `SubZero` is neither
monotone nor any of the three classes and it reaches agreement at exactly the same scale. Monotonicity
separates nothing here. The refuted prediction is section 3 of the probe and is kept in it.

What does predict it mentions no recovery map at all:

> **Interior safety.** The accumulator covers every partial sum of at most `n - 1` numeral values.

At that width no interior node of any grouping can leave the accumulator, so `phi` is applied at most once
per grouping, at the root, to the exact sum. A map applied once to a grouping-independent argument cannot
depend on the grouping, whatever the map is. That is a theorem rather than a measurement, and the measured
threshold is `K = n - 1` at every arity, for every non-homomorphism row:

```
resolution                     n=3         n=4         n=5         n=6
Wrap     (Hot)             K1 < B2     K1 < B3     K1 < B4     K1 < B5
Saturate (Warm/Cold)        K2 = B      K3 = B      K4 = B      K5 = B
Refuse   (Precise)          K2 = B      K3 = B      K4 = B      K5 = B
SubZero                     K2 = B      K3 = B      K4 = B      K5 = B
```

So there are exactly two sufficient conditions, and they are the two ends of one axis, which is the
staging axis:

**The recovery map commutes with the operation.** It may be applied at every stage and the answer is
unchanged. This is file 18's homomorphism theorem and it is `Wrap` alone. Threshold 1.

**The recovery map is deferred to the last stage.** It is applied once, so its own properties are
irrelevant. Threshold `n - 1`, every map.

Nothing was measured strictly between the two, across four maps and four arities. Which gives the third
reading I would put next to file 18's two:

> **Reading three: the accumulator is neither an eleventh axis nor a free combinator parameter. It is a
> side condition on the law, stated as a closed form in the arity, and it decides whether the recovery
> map is in the key at all.**

Below the threshold, the map's class is the whole answer and the map is in the key. At or above it, the
map drops out of the key entirely and the fact is about the accumulator alone. That is exactly the
draft's own unresolved question at `11_current_shape_draft.md:688-692`, whether quantisation fires per
operation or is deferred, arriving as a computed bound rather than as a choice somebody has to make.

The cost of this reading against file 18's two: the accumulator is still in the key, so the signature
still names it, so section 3's scope discipline still applies. What it buys is that nothing searches over
it. The bound is `acc ⊇ (n-1) × numeral`, which is an inequality on consts, which file 18 already noted is
expressible here (`18_lamport...md:400-407`). It is conservative, and section 5 measures by how much.

---

## 5. The operation does not join the key. It sets the growth rate of everything else in it

`21_probes/03_the_operation_scales_the_threshold.rs`. Signed Q2.2, raw in `[-8, 7]`, value `raw/4`,
`Precise` per the preset table at `11_current_shape_draft.md:327` (nearest ties-to-even in range, refuse
out of range). Exact values carried as `i128` in units of `2^-24`, exact for every product of up to five
Q2.2 values.

File 18 proved the classification is a property of the pair (`18_lamport...md:357-358`), and file 19
restated it as the limit of the classification's own robustness (`19_ringer...md:490-495`). Both left the
question of what it costs. File 19 called a second operation "the most expensive of all" and had no
number. Here is the number.

The precondition first, reproducing `18_probes/04` on my model:

```
  Add  rounding fired on   0 of 256 in-range operand pairs
  Mul  rounding fired on 128 of 256 in-range operand pairs
```

Then the smallest accumulator at which every grouping that returns agrees:

```
op        n=2    n=3    n=4    n=5   total bits (1 + int + frac)
Add        5      5      5      5
Mul        5      7      9     11
```

Addition needs nothing. Multiplication needs two more bits per additional element, and the two bits are
the numeral's own `F`. Reading the fractional column directly: `af` is 2, 4, 6, 8 at `n` = 2, 3, 4, 5,
which is `(n-1) * F` exactly. That is section 4's interior-safety condition, unchanged, read at the other
operation: the interior nodes hold products of at most `n-1` elements, a product of `j` elements carries
`j*F` fractional bits, so interior exactness needs `(n-1)*F`.

**So it is one side condition, not two.** The accumulator must hold the exact image of the operation
applied to at most `n-1` elements. The operation does not add a term to that statement; it changes what
the statement evaluates to, and it changes it by an exponential in the arity:

| operation | interior-exactness bound | growth in arity |
|---|---|---|
| addition | `ceil(log2(n-1))` extra integer bits, quantum unmoved | logarithmic |
| multiplication | `(n-1) * I` integer and `(n-1) * F` fractional bits | linear in both directions |

That is a sharper statement than "multiplication is a different problem", which
`11_current_shape_draft.md:776-779` already says, and it is sharper than "the classification earns zero
free transfer", which file 19 already says. It says the transfer is not merely absent, it is quantified,
and the quantity is the reason. **An operation is not one more parameter in a key. It is the parameter
that sets how the others scale.**

One further separation, which I did not go looking for and which lands on the seam files 17 and 18 already
cut. Section 2 of the probe measures existential agreement, meaning every grouping that returns returns
the same number, and under it the integer width is invisible: addition needs no widening at any arity.
Section 4 re-runs the same search under Kleene agreement:

```
op        n       smallest (int, frac)
Add       2                     (2, 2)
Add       3                     (2, 2)
Add       4                     (3, 2)
Mul       2                     (2, 2)
Mul       3                     (3, 4)
Mul       4                     (3, 6)
```

The two halves of the accumulator buy different things, for different reasons. **Fractional bits buy value
agreement**, because a truncated quantum is a wrong number. **Integer bits buy definedness agreement**,
because a range exit is a refusal. So the side condition is two bounds, along exactly the seam file 17
section 5.2 cut when it separated grade-invariance from value-agreement, and file 18 section 2 measured
when it found no single relation separates the four resolutions. A design that reports one accumulator
bound is fusing the two the same way reporting one relation fuses them, and it will put `Precise` in the
wrong column for the same reason.

And it is the answer to file 18's own untested prediction at `18_lamport...md:716-720`, that
`Growth::Exact` with a `2F`-fractional accumulator recovers the partial-identity property for
multiplication. It does, for one multiplication. For a fold of `n` it needs `(n-1)*F`, so the prediction
holds at `n = 2` and understates from `n = 3` upward. Whoever runs the multiplication dive should carry
the arity.

---

## 6. I committed the exact bug, in the probe written to forbid it, and the mechanism did not catch it

`21_probes/04a_the_shape_with_my_own_key_omission.rs`, kept unmodified as the audit trail, with a header
naming both defects. This is the most useful thing in this file and I would not have found it by
reasoning.

I wrote the shape from sections 3, 4 and 5, compiled it, and it printed two false rows:

```
resolution op     arity acc frac     agrees  why
Wrap       Mul        5       2       true  commutes
Saturate   Add        5       2       true  deferred to the root
```

`Wrap Mul` is false because I wrote `recovery_class<R>()`, keyed on the resolution alone, when file 18
proved the class is a property of `(phi, Op)` and I had quoted that proof three sections earlier.
Wrapping is a congruence for addition; for fixed-point multiplication the unconditional `>> FRAC` is not a
homomorphism at any range, which file 17 measured with the range removed entirely
(`17_orchard...md:443-446`).

`Saturate Add` is false because my `interior_exact_frac` returned only the fractional half of section 4's
bound, with a comment saying the integer half "is the same shape and is elided here to keep the probe
short". Addition's entire requirement lives in the integer half, which section 5 had just measured.

Both are the same defect and neither was caught, and the reason they were not caught is the finding:

> **Scope constrains what a proof body may name. It says nothing about whether a helper the body calls
> has a complete key of its own.** The law's own signature was complete. It called two helpers whose
> signatures were not, and a defaulted parameter one call deep is exactly as invisible as a defaulted
> parameter at the top, because the same mechanism hides it: a value where a quantifier should be.

Which fixes section 3's mechanism rather than refuting it, and the fix is small: the candidate sweep runs
at every function in the fact's call graph, not only at the fact. The candidate set for a given function
is the union of the parameters of everything it calls, which is mechanically readable from the call graph
and is what I should have written down as the procedure rather than as an instinct.

**And a third instance, of the other species, found in the last minutes of this dispatch.** The corrected
probe's own header listed four `--cfg` arms, one of which, `lie_at_the_leaf`, did not exist: the header
claimed it, the arm-selection guard in `main` named it, and nothing anywhere implemented it. Section 7 of
this file then described its refusal as verified. That is not a key omission. It is the *other* thing this
file criticises, a claim about an artifact with nothing behind it, and it is the same species as the
`Monotone` sentence file 18 caught at `11_current_shape_draft.md:699-703` and as the eight ` ```ignore `
doctests in the header above. I built the arm rather than delete the claim, and its refusal is quoted in
section 7. The general point is the one file 19 already made at `19_ringer...md:459-462`: a check that
never ran and a check that would have passed look identical from outside, and that is true of a `--cfg`
arm in a research probe exactly as it is true of a witness in a substrate.

I am reporting all three rather than smoothing them because this dive's own record, stated in file 19's
own section 0, is that each unbuilt shape had a hole the next member found by compiling it. These had
holes I found by compiling them, inside one dispatch, having gone in knowing precisely which failure
shapes to expect and having just written two sections about them. Read section 7 with that suspicion.

---

## 7. The shape, compiled

`21_probes/04_the_shape_compiled.rs`, the second attempt, four arms.

Five properties, and the second is 04a's correction:

1. A derived fact is a `const fn` whose parameters are its key. Nothing else declares a key.
2. Every helper the fact calls has a complete key of its own, checked the same way.
3. The operation is in the key of every law and enters through two derived predicates, the structural
   class at that operation and the interior-exactness bound. It does not multiply the vocabulary,
   because everything downstream reads it through those two.
4. The accumulator is in the key and is not searched over. The bound is a closed form in the arity,
   evaluated as a const bound, and it is deliberately conservative: section 5 measured signed addition at
   arity 4 needing one extra integer bit where the closed form asks for two.
5. The use site recomputes the fact from its own parameters. There is no fact object to pass around, so
   a fact proven at one key cannot be consumed at another.

The default arm, at a Q2.2 numeral:

```
resolution op     arity        acc class at this op   kleene   exist  why
Wrap       Add        5     (2, 2)     homomorphism     true    true  commutes with the operation
Wrap       Mul        5     (2, 2)     unclassified    false   false  nothing licenses it
Wrap       Mul        5     (8, 8)     unclassified     true    true  deferred to the root
Refuse     Add        5     (2, 2) partial identity    false    true  nothing licenses it
Refuse     Add        5     (4, 2) partial identity     true    true  deferred to the root
Refuse     Mul        5     (8, 8)     unclassified     true    true  deferred to the root
Saturate   Add        5     (2, 2)       retraction    false   false  nothing licenses it
Saturate   Add        5     (4, 2)       retraction     true    true  deferred to the root
Saturate   Mul        5     (8, 8)       retraction     true    true  deferred to the root
```

The law returns two verdicts rather than one, because section 5 measured that they come apart and file 18
section 2 measured that no single relation separates the four resolutions. Row four is the whole reason:
`Refuse Add` at the numeral's own accumulator is existentially true and Kleene false, which is
`Precise`'s entire regrouping story (file 17's diameter-0 finding, `17_orchard...md:353-357`) and is
invisible to any design that reports the conjunction.

Three refusals, each verified:

A combinator regrouping at a key the fact does not hold at, `--cfg consume_wrong`:

```
error[E0080]: evaluation panicked: this combinator regroups, and at its own operation, arity and
accumulator the composition's groupings do not agree; widen the accumulator, or pick a resolution
that commutes with this operation
    | evaluation of `regrouping_fold::<2, 2, 2, 1, 2, 2, 5>::{constant#0}` failed here
```

A helper with an incomplete key, `--cfg omit_a_helper_key`, which is 04a's first bug reintroduced:

```
error[E0080]: evaluation panicked: this helper is keyed on the resolution alone, and its verdict
moves with the operation; the operation belongs in its key
    | evaluation of `main::_CAUGHT` failed here
```

A leaf declaring a structural class the model refutes, `--cfg lie_at_the_leaf`, checked at the declaration
site by exhaustive const evaluation against the recovery map itself:

```
error[E0080]: evaluation panicked: this resolution declares a structural class the model refutes:
it is not a homomorphism, and the exhaustive check at the model width says so
    | evaluation of `main::_LIE` failed here
```

That third one is Thread C's own shape (`11_current_shape_draft.md:281-284`) and I only ported it; it is
not new here. It is also the arm section 6 records me claiming before building.

**What this shape gives up, honestly.** Every consumption is a monomorphisation of a generic function, so
there is no propagatable `T: AddAssoc` bound and no `E0277` naming an unsatisfied trait one level up.
This is file 19's own trade at `19_ringer...md:181-187` and I take it for the same reason: the bound-shaped
diagnostic was never sound, and file 19's section 2a shows the friendlier error can be bought back with an
unconditional blanket plus a truth marker if someone wants it. I did not wire that here, and file 19 flags
the same wiring as its own open item.

**And the cost I would want priced before anyone adopts it.** Every fact is now a function call with five
to seven const generic arguments at every use site. `08_fog...md` measured the const-eval wall at
quadrupling per bit, and none of these facts is a search over values, so none of them is near it. But the
monomorphisation count is the product of the key's cardinality, and nobody has measured what a real
consumer's composition set does to that. It is a measurement, it belongs in `mock/benches/`, and it is not
in this file.

---

## 8. Whether a law belongs to a type at all

Two readings, and I do not resolve them.

**Reading one: laws belong to magmas, and the design already has the noun.** Associativity is not a
property of a set, it is a property of a set paired with an operation. `11_current_shape_draft.md:385`
renames `Combine<Op>` to `Magma<Op>` and calls it "the precise term for a set with a binary operation and
no law claimed", and then the design attaches its laws to `Number<N, S>`, which is the set. That is the
category error that let the operation fall out of the key, and it is visible in the spec's own vocabulary:
the right noun is declared and not used. Under this reading a law attaches to the pair, a fold's law
attaches to a *different* pair (the accumulator's magma, with an embedding from the element's), and
sections 4 and 5 are what you get when you write that down.

The consequence I would follow up: a fold does not need its element type to be a semigroup. It needs its
accumulator to be one, plus a map from elements into it. That reframes file 13's over-strictness finding
and McSherry's observation that `arvo-spectral/src/power.rs:71` is refused at every strategy
(`13_mcsherry...md:490-496`), because the question stops being "does this numeral fold" and becomes "is
there an accumulator into which it folds", which section 4 answers with a closed form.

**Reading two: laws belong to nothing, and are edges of a rewrite system.** File 15's frame
(`15_willsey...md:44-57`): a law is a licence to rewrite, its key is the free variables of the rewrite
rule, and there is no object it is a property of. This reading is cheaper, it needs no noun, and it
reaches the same answer to my question by a different route, since the free variables of a rule are again
a scope question. Its cost is that it gives the const fn nowhere to hang, and file 15 argues at
`15_willsey...md:234-297` that arvo should not ship the rewrite engine that would make the frame
operational.

I lean to reading one and the lean is weak. What makes me hold reading two is that the two agree about the
*key* while disagreeing about the noun, which is corroboration of the mechanism and not of the vocabulary,
and the mechanism is what I was sent to look at.

---

## 9. Whether a provably complete key is expressible here

Directly, because the brief asks and the answer has a sharp boundary.

Completeness of a key means the key contains every parameter the proof depends on. For a proof that is a
**term**, that is decidable and trivially so: the free variables of a term are computable by scope, and
sections 3 and 6 are that computation being performed by rustc, once at the top and once per helper. So
for the checkable half of a proof, a complete key is not merely expressible, it is what you get for free
the moment the proof stops being prose.

For the half that is not a term, it is not expressible, and it never will be. The design's proofs are
split and the split is deliberate. Two of file 18's four rows follow from a structural argument that
quantifies over every arity and every width in three lines (`18_lamport...md:264-286`), and no const block
reaches that. The draft is honest about the general form at `11_current_shape_draft.md:861-868`: a claim
quantified over values in a type is dependent typing and Rust does not have it.

Which localises the whole problem, and this is the part I would act on:

> **A key omission is always an omission in the prose half of a split proof.** The const half cannot omit
> a parameter, because scope. So the design does not need a general key-completeness mechanism. It needs
> the prose half's parameters to appear in the const half's signature *even where the const half does not
> read them*, so the type system carries what the argument forgot.

That is key inflation, deliberately, and section 2 is why it is safe: inflation is over-strict and sound,
omission is unsound. It costs a parameter that nothing reads, and it buys that the parameter is visible to
the sweep, visible in the error message, and visible in the symbol table per section 11. Rust will not
help here, because an unused type parameter on a *function* is legal (unlike on a struct, where `E0392`
fires), so this is a discipline rather than a mechanism. It is a discipline with a check, which is more
than the design has now.

The residual, stated so nobody claims more than it: after all of this, what is trusted is the derivation
step, "homomorphism implies Kleene associativity at every arity" and its two siblings, which is three
lines of mathematics in prose. That joins the draft's own trusted bin at
`11_current_shape_draft.md:832-834` and it belongs there. The apparatus does not shrink that bin. It
shrinks what feeds into it.

---

## 10. Where staging does not transfer, argued from what it needs against what this substrate gives

The brief asks for honesty here and two members have delivered exactly this verdict about their own
fields. Mine is a partial one: the analysis transfers, the payoff does not, and one of the three claims I
would ordinarily make is false here.

**There is no residualisation, so this is not staging in the technical sense.** In a two-level language the
point of a binding-time separation is that stage-0 computation disappears and stage-1 computation is
emitted as a residual program. Every stage in section 1 is compile time; nothing is emitted, nothing is
specialised away that was not already going to be, and there is no generated program to inspect. What
transfers is the **binding-time analysis**, which is bookkeeping about which parameters are known when.
What does not transfer is the **specialisation**, which is where the value normally is. A member expecting
the LMS argument to arrive with its usual payoff should not.

**Monomorphisation is not a stage boundary in the sense that would make the tower pay.** A real stage
boundary erases, and erasure is what makes the earlier stage free. Wingo measured that this one prints
instead (`20_wingo...md:326-341`). So section 1's three stages are a tower of *scopes*, not of stages, and
I have been careful above to claim only what scope gives.

**And the thing staging would buy is already present, in a harsher form than this design wants.** This is
the verdict I would most like recorded. The discipline "a stage-0 computation may not depend on a stage-1
value" is exactly what rustc enforces when it refuses a const expression computed from a generic parameter
in type position. File 19 hit it head on (`19_ringer...md:96`) and read it as a wall imposed by the
`generic_const_exprs` ban. Read as a binding-time rule it is not a wall, it is the substrate stating where
facts are allowed to live:

> **arvo is not missing a staging discipline. It is fighting one it did not choose.** The ban means the
> only way to run a computation over generic parameters is inside a function body, which means every
> derived fact must be a call at a use site rather than a projection in a type. Sections 3 and 7 are that
> constraint followed rather than worked around.

I arrived at the same mechanism from taste before I understood this, and I trust it considerably more
having found the constraint underneath it. But it is worth being clear that the constraint is doing the
work, not the analysis, and someone who dislikes the analysis can keep the mechanism.

**The one place I would not push the frame.** Section 1's tower may be a lattice, per the alternative I
recorded there. If it is, "which stage does this fact belong to" is not a well-posed question and only
"which parameters does this fact depend on" is. Everything in sections 3 through 7 survives that, because
none of it uses the ordering. Only section 1's table uses it, and that table is a diagnostic device rather
than a result.

---

## 11. The downstream contract, designed

`16c`'s obligation is that every boundary the design stops at gets a design rather than an observation.
File 18 designed the specification side of the regrouping boundary (`18_lamport...md:651-697`) and file 20
designed the channel (`20_wingo...md:390-437`). The piece neither designed is the one my question owns:
**a downstream reader of a fact needs to know which key it was proven at**, because a fact proven at
`A = S` licenses nothing for a fold at `A ≠ S`, and section 4 measured how much nothing.

This is written in file 20's reading of the stage boundary rather than mine, since that is the one that
reaches a build layer.

**What arvo guarantees, and it is three sentences on top of file 20's three.** Fact-producing functions are
`#[inline(never)]` at the granularity where the key is complete, so the symbol carries the whole key.
Wingo's measurement is that every generic argument of every instantiation is in the symbol table exactly,
with its value, under v0 mangling which is the default on the pinned toolchain
(`20_wingo...md:338-341`). So the symbol **is** the key, without arvo emitting a manifest, registering
anything, or growing a build harness, which `16c` is explicit about not wanting. A reader recovering
`regrouping_fold::<2, 2, REFUSE, MUL, 8, 8, 5>` from `llvm-nm` knows the numeral, the resolution, the
operation, the accumulator and the arity the fact was proven at, and knows them exactly, because they are
the same parameters the const block evaluated.

**What arvo needs back, which is one clause.** A combinator must instantiate the fact at the parameters it
will actually use, not at a representative set. That is not an honour system: because the fact is a call
rather than an object, a combinator that regroups at arity 5 with a narrow accumulator and calls the fact
at its own parameters gets `E0080` at its own call site, which is section 7's `consume_wrong` arm. The
clause is therefore "call the fact at your own parameters", and it is checked rather than asked for.

**And the failure this makes visible, which nothing else in the dive catches.** File 18 already noted that
a build layer choosing an unroll factor is choosing a parameter of the law's statement
(`18_lamport...md:690-697`). The symbol channel makes that mechanically checkable, and I would state it as
the contract's one hard prohibition:

> A build layer may not emit a body whose regrouping arity or accumulator differs from the `ARITY` and
> accumulator in that body's own symbol. The symbol records what was proven; the body is what runs; and a
> transformation that changes one without the other is Thread C's fourth-pass gap
> (`11_current_shape_draft.md:606-617`) reintroduced after type checking, where nothing internal to arvo
> can see it.

That is checkable by exactly the reader file 20 already built, in the same pass, and it is the only new
obligation this file puts on the boundary. Every other liberty file 20's section 6 designs is unaffected,
because none of them touches a law's key.

**The reading I hold against this one.** The symbol channel closes under inlining
(`20_wingo...md:365-373`), and a fact-producing function that must survive as a symbol is a function that
must not inline, which is a real runtime cost on a substrate whose whole compile-time policy exists to buy
runtime (`arvo-compile-time-last.md`). If the key never needs to be read downstream, none of this is worth
paying for, and the honest position is that nobody has produced a downstream consumer that wants to read a
law's key. File 18's regrouping combinator is the candidate and it does not exist yet. So I would design
the contract, per `16c`, and not build it until something asks.

---

## 12. What I would flag for the next member, unresolved

**The brief's merge of two failure modes, section 0, is the thing I would fix before the next dispatch.**
An unwired check and an omitted key are different, they want different mechanisms, and file 19 built the
first while this file builds the second. A brief that presents them as one will produce a member who
builds neither properly.

**Section 1's three stages may be a lattice rather than a tower and I did not settle it.** The accumulator
is bound before the operation is selected in a fold's dependency order, which reverses two of them.
Everything in sections 3 through 7 survives either way; only the table in section 1 depends on it.

**The accumulator's third reading, section 4, is mine and nobody has attacked it.** It says the
accumulator is a side condition with a closed form rather than an axis or a combinator parameter. It rests
on interior safety, which is a theorem, and on the measurement that nothing lands strictly between the two
sufficient conditions, which is four maps at four arities on one model. A fifth map that lands between
them would not refute the theorem but would refute the claim that the two conditions are the whole story,
and searching a wider space of partial recovery maps for one is a cheap next probe of exactly the shape
file 18's `18_probes/02` already runs.

**Section 5's growth rates are measured on one numeral at one precision.** `(n-1)*F` fractional and
`(n-1)*I` integer for multiplication, `ceil(log2(n-1))` integer for addition. The fractional column is
clean and I would trust it; the integer column is fiddly on a signed asymmetric range and my closed form
is conservative by one bit at the one arity where I could compare. Whoever writes the real bound should
derive it rather than fit it to my table.

**Division is untouched and it is the operation that breaks section 5's frame, not multiplication.**
Interior exactness requires the accumulator to hold the operation's exact image, and
`11_current_shape_draft.md:694-697` already notes that the exact quotient of two representable values is
generally not expressible at any finite width. So there is no interior-exactness width for division at any
arity, the second of section 4's two sufficient conditions is simply unavailable, and the only remaining
route is a recovery map that commutes with division, which no map in the design does. I did not probe
this and I state it as a prediction rather than a finding. If it holds, division's law is a genuinely
different problem from both addition's and multiplication's, and the dive has been assuming there are two
cases when there are three.

**Section 6's three failures mean section 7 should be read as the fourth or fifth shape in a sequence where
each predecessor had a hole the next member found by compiling.** I found all three inside one dispatch,
and the third was a claim in a header rather than a defect in code, which is the failure mode with the
worst ratio of cost to detectability in this whole review. The candidate-sweep-over-the-call-graph
procedure is the correction for the first two and I applied it by hand rather than mechanically, which is
exactly the condition under which I made the original error. For the third there is no procedure, only
the discipline of running every arm you name, and a probe file is small enough that running all of them
is a shell loop.

**I did not price the monomorphisation cost of section 7's shape**, which is the product of the key's
cardinality across a consumer's composition set, and it is the one thing about the proposal that could
make it unaffordable. It is a measurement and it belongs in `mock/benches/` per
`bench-in-bench-harness-never-sketches.md`.

**And I did not read `arvo-num-systems` or `notko-hlist`**, which files 17, 18 and 19 have each now
flagged as possibly changing the cost picture. Three members in a row naming the same unread thing is
itself a signal, and it is a cheaper dispatch than any of the probes above.
