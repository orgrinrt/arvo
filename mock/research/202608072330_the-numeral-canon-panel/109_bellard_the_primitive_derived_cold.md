# What is a primitive, derived cold

**Phase one. Written blind.** Read before writing: `INTENTS.md`, `RULES.md`, `mock/Cargo.toml`,
`mock/benches/` (layout, `bench.toml`, the thirteen test-bearing variant crates), `rust-toolchain.toml`,
and this repository's own `.claude/` rules. No panel file, no consolidation, no `OPTIONS.md`, no
`DROPLIST.md`, no probe of anyone else's, no `git log`.

One contamination to declare, because it is real and small. After a failed `git commit` I ran
`git log --oneline -1` to check whether my commit had landed, and the line that came back was another
member's, not mine. Its subject named a conclusion of theirs. That happened after the derivation below
was complete in outline and after four of my seven probes were already committed; I did not open the
file, did not act on the phrase, and it appears nowhere in what follows. `RULES.md` is right that
informative commit subjects are a leak, and this is one more instance of it.

## Gate one: the canon gate

**Passed.** Checked against `INTENTS.md` I1 through I18, read with its own "How to read an entry"
section as normative.

The dispatch asks what a primitive is, in a repository whose canon is being written and whose crate
tree is deliberately empty. Nothing in I1 through I18 forbids the question or presumes an answer to it.
I1 is demoted to open on op's word, so the strategy set's size and names are mine to challenge; the
brief says the same. I13 governs the shape of every finding here. I14 fixes the operating constraints,
and everything below stays inside them: no probe of mine uses `alloc`, `dyn`, `TypeId`, `std::any`,
`generic_const_exprs`, full `specialization`, or `-Znext-solver=globally`. Six of my seven probes use
no `#![feature(...)]` line at all; the seventh uses `const_trait_impl`, which the workspace's
`unstable-features.md` lists as allowed.

**But I am reporting two misalignments I found while running the gate**, per the standing instruction to
report unlicensed mechanisms even outside the question asked. They are in section 12, and one of them is
in a governing file.

## Gate two: the test gate

**Passed, with a bounded reading and one nit.** `mock/crates/` is empty by design, so the only suite in
the repository is the bench-variant tree.

```
$ grep -rln --include="*.rs" '#\[test\]' variants --exclude-dir=target | cut -d/ -f2 | sort -u | wc -l
13
$ grep -rn --include="*.rs" '#\[test\]' variants --exclude-dir=target | wc -l
124
```

Thirteen crates, **124** `#[test]` annotations, run from `mock/benches/`. The brief says six members
independently counted 123. I get 124 and I am not going to adjudicate the difference: it is one
annotation, either added since or counted with a marginally different exclusion, and it does not bear on
anything.

Per-crate: `wide-rung-shared` 30, `bitpack-write-contend-shared` 16, `warm-container-shared` 15,
`bitpack-contend-shared` 12, `satfold-shared` 11, `bitpack-carrier-shared` 9, `warm-clamp-shared` 7,
`bitpack-wide-shared` 6, `bitpack-footprint-shared` 6, `bitpack-plan-shared` 5, `quantiser-radix-shared`
3, `bitpack-shared` 3, `quantiser-fadd-shared` 1.

**Coverage bound, stated rather than implied.** I read the bodies of the four tests in
`quantiser-fadd-shared` and `quantiser-radix-shared` in full and skimmed the rest. I did **not** run the
suite: `wide-rung-shared` alone is documented at 107 seconds and my question is definitional, so the
surface I touch is not the bench tree. Anyone treating my gate result as a full audit is treating a
sample as a census.

What I read is real testing rather than decoration, and specifically it is guarded against the failure
this gate exists to catch. `quantiser-fadd-shared`'s single test cross-validates a software path against
hardware bit-for-bit and then asserts `assert_eq!(checked, 6 * 64 * N as u64)`, which is exactly the
guard against a setup that silently skips. `quantiser-radix-shared` checks radix two against silicon and
radix ten against the definition, and says in a comment why the second cannot be checked the first way.
That is the shape of a suite somebody meant.

**One nit, with a citation.** `mock/benches/variants/quantiser-radix-shared/src/lib.rs:370` asserts
`assert_eq!(p % 2, 1, "3^{s} should be odd")` and line 372 then asserts `assert!(p % 2 == 1)`. The
second is a verbatim restatement of the first. It is not tautological, because it can fail, so it is not
in the delete-outright class; it is redundant and one of the two is doing the work. Not a gate failure.

## The question, and the shape of the answer

The panel's working assumption: a primitive is a **named composition** of a format, a number system, a
law set and a strategy.

My answer, stated first so the rest can be checked against it:

> **A primitive is a value's compile-time half.** It is the part of an operation's specification that has
> been moved out of the operation and into the type, and it is exactly the part that must be
> const-available in order to decide validity or select a lowering. What remains at runtime is the value.

Under that reading the working assumption is right in shape and wrong in three of its four elements,
each wrong in a different way and at a different level:

- **format** is two independent things, not one: the value set and the realisation. Independence
  established both directions in P1.
- **number system** is not a component. It is a name for a coherent package of choices, which is to say
  it is itself a composition, sitting one level up from where the assumption puts it.
- **law set** is not a component and putting it in the tuple is actively dangerous. A law is a
  consequence of the other components, computable at const time; declaring one is a claim nothing
  constrains. P2.
- **strategy** is not a component either, but it is not disposable. It is a **selector** over the
  components, and it belongs in the primitive as the **request** beside the **resolution** it produced.
  P3. This is the element the panel should keep, re-typed rather than dropped.

So: three resolved components, one request, and nothing else. And a fifth thing that I claim cannot live
in a primitive at all, section 8.

## 1. Derivation: what must be decided before one operation lowers

I started from the smallest possible question, because the definition should fall out of it rather than
be imposed on it. What has to be fixed before `a + b` can emit a single machine instruction?

1. **Which values may `a` and `b` be?** A set of exact mathematical values, in arvo's case a finite
   subset of the rationals. Call it `V`.
2. **How is a member of `V` stored?** An injective map `ρ: V → B` into bit patterns, plus where those
   bits rest. Call it the **realisation**.
3. **What does `+` mean?** The exact operation on the objects `V` denotes. This is not a free choice;
   it is inherited from the mathematics.
4. **What happens when the exact answer is not in `V`?** A total map `π` making the partial operation
   total. Call it the **completion**.
5. **Which instructions compute it?** A lowering `λ`.

That is the whole list, and it is worth noticing that (3) is not a decision at all. Exact rational
addition is what it is. So the decisions are `V`, `ρ`, `π`, `λ`.

Then the first real question: **which of these four is part of the primitive's identity?**

`λ` is not, and the argument is short. Two lowerings that compute the same function on the same
encoding are indistinguishable in the value domain; they differ only in cost. Under I13 the correct
treatment of two lowerings of one function is not two primitives but **one primitive with two arms and
two predicates**, which is op's own words for it. Under `arvo-always-optimal-internals.md` the consumer
never names `λ`. So `λ` is derived and does not enter.

That leaves `(V, ρ, π)`. The rest of this file is checking that triple: that its members are genuinely
independent, that nothing else belongs beside them, and that the two things the panel wants to add are
respectively a consequence and a selector.

## 2. `V` and `ρ` are two things, established both directions

P1, `109_probes/p1_value_set_and_realisation_are_independent.rs`, output in `p1_output.txt`.

**Direction A: same value set, two realisations.** The integers `0..=8191`, saturating completion held
fixed, realised twice: once per element in a `u16`, once as a dense bit stream at exactly 13 bits per
element crossing word boundaries. Every element of the value set survives both, checked exhaustively
against seven addends, 57344 comparisons, with the neighbouring lanes deliberately saturated so a bleed
across the 13-bit boundary would be caught rather than masked by zeros. The footprints differ (13 bits
against 16) and element 4 straddles a word boundary, so it needs two word reads while element 0 needs
one. The answers are identical and the memory is not.

**Direction B: same realisation, two value sets.** Exactly eight bits in a `u8`, no spare and no tag,
read once as `I=8, F=0` and once as `I=4, F=4`. The bits are the same bits. **64714 of 65536 pairs give
a different product**, and every non-zero pattern denotes a different rational under the two readings.

Both directions hold, so `V` and `ρ` vary independently and "format" is two components wearing one name.

**A defect of mine that is worth more than the result.** The first version of P1 modelled the packed
realisation as four 13-bit lanes in a `u64`. That is 52 bits used of 64, which is 16 bits per element,
which is exactly what the `u16` costs. The footprint assertion failed and it was right to: a pack that
does not pack cannot demonstrate anything about packing. It is a setup that helps, written by reflex,
caught only because the assertion was about the thing rather than about the declaration. The rejected
version is committed at `p1_v1_lane_aligned_pack_was_not_a_pack.rs.rejected`.

**Predicate.** `V` and `ρ` are independent components: `I any, F any, signedness = unsigned, S any,
overflow policy = saturating, rounding = truncate, operation ∈ {add, mul}, arity = 2, chain length = 1,
container ∈ {u16, dense bit stream}, alignment ∈ {aligned, straddling}, access pattern any, target
features any, threads any`. Threads is `any` on the ground that both routes are pure functions of their
inputs with no shared mutable state, which is a property readable off the source rather than an
unmeasured guess.

## 3. `π` is a third axis and not redundant

P3, `p3_three_sameness_relations_and_what_each_licenses.rs`. Holding the value set and the realisation
fixed at `I=8, F=0` in a byte and moving only the completion, **wrapping and saturating disagree on
32640 of 65536 pairs**. So `π` is not derivable from `(V, ρ)` and the decomposition does not collapse to
two.

**Predicate.** `I = 8, F = 0, signedness = unsigned, overflow policy ∈ {wrap, saturate}, rounding =
exact (F = 0), operation = add, arity = 2, chain length = 1, container = u8, S any, target features any,
threads any` (purity, as above).

## 4. The law set is a consequence, and declaring it is a fabrication

This is the element of the working assumption I want to attack hardest, because it is the one that would
do real damage if it shipped.

P2, `p2_a_declared_law_is_a_claim_nothing_constrains.rs`. Take "law set is a component" literally: a
primitive that declares which laws it satisfies. Three questions.

**Q1. Does a false declaration compile?** Yes. `impl DeclaredLaws for SatBoth { const ADD_ASSOCIATIVE:
bool = true; }` for two-sided signed saturating addition, which fails associativity on **952 of 4096
triples**. Nothing between writing the lie and running the program objected. And a reader cannot tell
that row from the `Wrap` row directly beneath it, which declares the same value truthfully: two
assertions in the same position, with the same weight and opposite truth values.

**Q2. Is the declaration load-bearing?** Yes, which is worse than decorative. A reassociating rewrite
gated on it, the shape of every optimisation such a declaration would exist to license, **changes 952 of
4096 answers**, witness `a = -8, b = -8, c = 1`, reference `-7`, rewritten `-8`. The same rewrite on
`Wrap`, whose declaration happens to be true, changes nothing, which establishes that the rewrite is not
the defect. The declaration is.

**Q3. Can the law be computed instead?** Yes, at const time, and this is the constructive half.
`const SAT_BOTH_ASSOC: (u32, u32) = associativity_census::<SatBoth>();` exhausts all 4096 triples during
compilation. There is no position in the source where a wrong answer could be written, because nobody
writes an answer.

This is the test gate's "declarations nothing constrains" failure, hoisted from the test suite into the
design vocabulary. A type declaring `ASSOCIATIVE = true` is a comment with a type, and giving it a slot
in the definition of "primitive" is inviting every implementor to write one. **The law set is not a
component of a primitive. It is a theorem about one.**

### The wall Q3 hit, and what it says about how such a law has to be built

Q3 was first written with the census taking `fn(i32,i32)->i32`. rustc refuses, four times:

```
error: function pointer calls are not allowed in constant functions
```

Recorded verbatim in `p2_blocker_fn_ptr.txt`. So **a law cannot be computed at const time about an
operation supplied as a value; the operation has to be a type.** That is not a detail of my probe, it is
a constraint on any design that wants derived rather than declared laws: the completion must be
reachable through a const trait, which is what the committed version does. The spelling on this pin is
`const trait T`, not `#[const_trait] trait T`; the compiler names it in its own diagnostic and P0
records it, because the workspace's own `unstable-features.md` still writes the old form.

### And a correction of my own, which is the more useful result

I predicted that one-sided saturation associates and two-sided does not, and wrote that as an assertion.
**It failed: 448 of 4096 triples.** The reason is upstream of associativity and I had not thought about
it. Over a *signed* value set, clamping only at the top is **not a completion at all**: `-8 + -8 = -16`
leaves the value set through the bottom, which nothing clamps. The closure census I added afterwards puts
a number on it: **36 of 256 pairs escape**. Asking whether that operation associates is asking a question
about a function of a different type than it claims to be.

**Closure is prior to every law.** With closure restored by moving to an unsigned set `0..=15`, one-sided
and two-sided saturation both associate with zero failures, and they are the *same function* on that set,
because the bottom clamp is unreachable. The discriminator was never the clamp count; it is **whether
both clamps are reachable**, which is a joint fact about the completion and the signedness of the value
set.

**Predicate.** Two-sided saturating addition fails associativity: `W = 4, I = 4, F = 0, signedness =
signed, overflow policy = saturate-both-ends, rounding = exact, operation = add, arity = 2, chain length
= 3, S any, container = i32 model, target features any, threads any`. One-sided saturating addition over
an unsigned set associates: `W = 4, I = 4, F = 0, signedness = unsigned, overflow policy =
saturate-top-only, operation = add, arity = 2, chain length = 3, S any, target features any, threads
any`. One-sided saturation over a signed set is not closed: same predicate with `signedness = signed`.
The width is fixed at 4 and stays fixed: I have no transfer argument to any other width and
`unstable-features.md` is explicit that a model-width check needs one.

## 5. The strategy is a selector, and the request belongs in the primitive anyway

This is where I expected to disagree with the panel and ended up half agreeing.

**The negative half.** Every strategy in `INTENTS.md` acts by *fixing* one or more of `V`, `ρ`, `π`,
`λ`. I6's storage minimisation is a choice of `ρ`. I7's accuracy across chains is a choice about where
`π` fires. I5's willingness to sacrifice soundness is a choice of `π` and `λ`. I3's imitation of a
native primitive is a choice of `π` and `ρ`. I8 says they weigh different measurements differently,
which is a statement about *how the selection is made*, not about what is selected. I9 says the strategy
changes what the correct answer is, which is the selection of `π`. I looked for strategy content that is
not a choice among the components and did not find any.

So a strategy is a **function from a request to a resolution**, which is a different kind of thing from
the resolution's members. Putting it in the same tuple is a level confusion, the same one that puts
"number system" there.

P3 makes this concrete rather than rhetorical. Two markers, `Speed` and `Space`, that resolve
**identically** at `I=8, F=0` because eight bits is already the container and there is nothing for the
storage-minimising selector to win. The two `Resolved<V, R, C>` types unify: `same_type(&r1, &r2)`
compiles, which it could not if a strategy were a distinguishing component.

**The positive half, and the panel should keep this.** If the marker is discarded and only the
resolution is keyed, `WithoutMarker<V8_0, Speed::PickRealisation, Speed::PickCompletion>` and the
`Space` equivalent are literally one type. They unify, and **nothing in the value's type records that a
consumer asked for speed rather than space**. When the storage-minimising selector later learns to pack
at this width, the consumer's source does not change and the consumer's meaning does.

That is a real cost and it is the argument for keeping the marker. So a primitive is a **pair**: the
**request** the consumer wrote, and the **resolution** it produced, with the resolution a const function
of the request and the target.

**What that pair costs, since it is not free.** With the marker carried, the two identical resolutions
are incompatible types. P3b is the compile-fail proof:

```
error[E0308]: mismatched types
   = note: expected struct `WithMarker<_, _, _, Speed>`
              found struct `WithMarker<_, _, _, Space>`
```

Same value set, same realisation, same completion, same bits, same size, same alignment. A cast is
required for a difference that does not exist. That is the price of keeping the request, and it is worth
naming rather than discovering.

**Predicate.** Two strategy selectors can resolve to one resolution, and the marker distinguishes types
that the resolution does not: `I = 8, F = 0, signedness = unsigned, S ∈ {Speed, Space} as defined in
P3, container = u8, overflow policy = wrap, operation = none (this is a typing result), arity = 0, chain
length = 0, target features any, threads any`. This is a compile-time structural result and threads is
`any` because what the compiler accepts is not moved by a runtime thread count.

## 6. Composing, and what the name buys

Composing, under this reading, is a **const function on resolutions**. `Mul` takes the resolution of its
operands and produces the resolution of its result, at compile time. A chain of operations is a
composition of those functions, evaluated during compilation, and the runtime is what falls out.

So what does naming a composition buy that the composition alone does not?

The honest answer is that it buys **nothing semantically** if the name is a total function of the
parameters. `Fixed<8,4,Hot>` denotes exactly what `(V, ρ, π)` denotes. Naming buys abbreviation and a
locus for impls, which is real and unexciting.

**But naming is interesting exactly when it is partial**, and that is where it earns a place in the
definition. If some compositions have names and others do not, the name is an **existence claim**: this
composition is one the design supports, there is a container for it, and a lowering exists. A
composition with no name is one arvo does not offer.

And under I15 that claim is where invalids get caught. **The naming function is the validator.** P5b
shows this working, and the wanted outcome was a build failure:

```
error[E0080]: evaluation panicked: the carried range does not fit the container
   evaluation of `Ranged::<RSum<Lit<0, 200>, Lit<0, 100>>>::FITS` failed here
```

`0..=200` plus `0..=100` reaches 300; the container stops at 255. Nothing about that is visible in the
values, it is a fact about the composition, and rustc refuses it at compile time and names the exact
composition in the diagnostic. That is I15 satisfied by construction: the invalid never becomes a
runtime concern because it never becomes a program.

So: **the name is the surface, the set of names is the set of supported compositions, and the boundary
between nameable and unnameable is where compile-time validation happens.** That is what naming buys,
and it is a much stronger claim than "abbreviation".

## 7. A carried range removes the completion rather than choosing it

P5, `p5_a_carried_range_eliminates_the_completion.rs`. This started as a check on section 6 and turned
into the most useful thing in the file, so it gets its own section.

If a primitive carries its range, and the ranges of the operands compose to something the container
holds, then the exact answer **cannot** leave the value set, `π` never fires, and the lowering is the
bare instruction. The completion is not chosen. It is eliminated.

The obvious obstacle is that `Ranged<{LO_A + LO_B}, {HI_A + HI_B}>` puts arithmetic in type position,
which needs `generic_const_exprs`, which is forbidden. The workspace's standing reflex is that a refused
bound is a trait nobody has named yet, and here it is exactly right: put the arithmetic in an associated
const's **body**, where arbitrary const expressions are legal, and bound on the trait.

Five questions, five answers:

- **Q1. Does it compile with no forbidden feature?** Yes, and with **no `#![feature(...)]` line at all.**
  `RSum`, `RMul`, `RDiff` propagate through associated consts.
- **Q2. Does it compose to depth?** Yes, to depth six in the probe: `RDiff<RMul<Lit<1,2>, RSum<RSum<RSum<
  Lit<0,100>, Lit<0,50>>, Lit<0,10>>, Lit<5,20>>>, Lit<0,100>>` resolves to `-95..=360`.
- **Q3. Is the derived range correct?** Checked rather than asserted, and checked for **two** properties
  that a declaration would conflate. **Sound**: no result leaves the derived range, zero of 5151 pairs.
  **Tight**: both endpoints are attained, so the rule is not merely conservative. Same for the product
  rule.
- **Q4. Is an over-container range refused at compile time?** Yes, quoted in section 6.
- **Q5. Does the proof survive lowering?** Yes. On aarch64-apple-darwin at `-O`:

```
_proved_add:                       _unproved_add_checked:
	add	w8, w1, w0                     add	w8, w0, w1
	and	w0, w8, #0xff                  mov	w9, #255
	ret                                cmp	w8, #255
                                       csel	w0, w8, w9, lo
                                       ret
```

The `and #0xff` is the `u8` truncation, a container artifact, not a completion. No compare, no select,
no branch. `unproved_add_saturating` folded into `unproved_add_checked` as an alias, correctly, because
`saturating_add` and `checked_add(..).unwrap_or(255)` are the same function on `u8`.

**What this is and is not.** It is a compile-and-inspect check. It establishes that the completion is
absent from the emitted code on the proved path and present on the unproved one. It is an ad-hoc quick
spike as far as magnitude goes: it prices nothing, it did not run on the mockspace bench harness, and
two instructions against four is an instruction count off one body on one target rather than a
measurement. Nobody should read a speed claim out of it, and I am not making one.

**And the bound that matters.** `proved_add` takes raw `u8` parameters and constructs
`Ranged(a, PhantomData)` directly, so the range obligation is discharged nowhere on that path. The
emitted code is what a *proved call site* lowers to; it is not evidence that the proof is enforced at
that entry point. **The guarantee's perimeter is the construction site**, and an `extern "C"` boundary
with primitive parameters walks straight through it. That is a fact about where a carried range must be
established, not a reason the mechanism fails, and it is the perimeter rule this workspace already
carries applied to a new case.

**Predicate.** Range propagation through associated consts compiles and composes: `I any, F = 0,
signedness = unsigned at the leaves and signed in intermediates, range endpoints ∈ i32, operation ∈
{add, sub, mul}, arity = 2, chain depth ∈ 1..=6, container = u8, S any, target features any, threads
any, toolchain = nightly-2026-05-28, edition = 2021`. The edition is listed because it is load-bearing:
the file does not compile under the 2015 default, `core` does not resolve and `repr(transparent)` is
rejected over `PhantomData`. The emitted-code claim narrows to `target = aarch64-apple-darwin, target
features = host default, opt level = 3`.

## 8. Chain accuracy cannot live in a primitive, and that is a hole in the whole framing

P6, `p6_chain_accuracy_is_not_a_property_of_a_value.rs`. This is the finding I did not expect and it is
the one I would most want attacked.

I7 says the accuracy-first concern is accurate "especially within chains and ops, not only alone". A
primitive, in the working assumption and in mine, is a property of a **value**. A chain is not a value.

The experiment holds every per-value component fixed and moves only the operator's target type. Route A
quantises after every multiply, `mul: P × P → P`. Route B never quantises, `mul: P × P → P2`, and
narrows once at the end. Same value set, same realisation, same completion, same stored numerators. The
oracle is the exact rational, so neither route is judged by the other.

```
len  seeds  per-step-worse  deferred-worse  tied   max|err| per-step   max|err| deferred
  1    400               0               0   400                  0                   0
  2    400               0               0   400                254                 254
  3    400             181               0   219             163786               65496
  4    400             256               0   144           79265712            16742448
  5    400             287               0   113        33617975984          4281834240
  6    400             308               0    92     13292080393136       1094722063296
  7    400             311               0    89   6308948376789200      281012935502880
  8    400             313               0    87  2308019703317810688  71623882438406656
```

The deferred route is **never worse**, at any length, on any of the 3200 chains. It is strictly better on
a growing majority from length three onward. And **nothing about the operand type differs between the
two routes**: only where the operator lands.

So "accurate across chains" is a fact about the **operator typing**, not about any component of the
operand type. A per-value primitive has no slot for it, and adding one would be adding a slot for a
property that is not about the value.

Two consequences, and I think the second is more important than the first.

**First, `Mul` is not an endomorphism.** The signature `Mul: P × P → P` is what forces the per-step
quantisation, and the moment the result type is allowed to be a different primitive, the chain story
falls out of the typing without any chain policy existing anywhere. That is the resolution I would
propose: chain accuracy lives in the operator's target, and the operator's target is a const function on
resolutions, which section 6 already needs.

**Second, and this is the structural claim: a primitive is not definable one at a time.** If the
operators land in other primitives, then a primitive is a node in a graph whose edges are the operations,
and naming one node in isolation names nothing usable. What can be named is a **family closed under the
operations**, indexed by its parameters. This is the ordinary algebraic position, that a structure is a
carrier plus operations and not a carrier alone, and it means the canon's unit of definition should be
the family rather than the primitive. I11's "contracts for things that compose to bigger units than just
numerals alone" reads to me as pointing the same way, though I am inferring and say so.

**And the cost, so this is not read as a free win.** The deferred route's accumulator grows with the
chain: at `F = 8` and eight factors it carries 64 fraction bits before the final narrowing, which no
8-bit or 16-bit container holds. The intermediate width is a function of the **chain**, which no
per-value type knows. So this is a trade whose terms only the party that knows the chain can set, which
is the toolbox-not-policer position arrived at from a different direction.

**Predicate.** Deferred quantisation is never worse than per-step and strictly better on a majority of
chains from length three: `F = 8, I unbounded (i128 model), signedness = signed, rounding = truncate
toward zero, overflow policy = none (the model does not overflow i128 at these lengths), operation =
mul, arity = 2, chain length ∈ 1..=8, factors ∈ (0, 2] on a deterministic LCG spread, 400 chains per
length, S any, container = i128, target features any, threads any` (purity). It does **not** extend to
rounding-to-nearest, which I did not test and where the per-step errors would partly cancel; that is the
most obvious attack on this result and I am naming it rather than waiting for someone else to.

## 9. I18 is a bounded exception inside I15, and the object file agrees

P4, `p4_the_bounded_panic_erases_under_lowering.rs`. I15 says never any runtime checks, ever. I18
permits a native-style overflow panic bounded by build and by concern. Those are compatible exactly when
the panic path is gone from a release artifact, and that is checkable rather than arguable.

Two gates, because they are different mechanisms: the **build** gate is `cfg!(debug_assertions)`, and
the **concern** gate is a const on the strategy. The second is the one worth checking, since it is what
makes I18's second bound implementable at all.

Release, at `-O`:

```
_add_imitating   = _add_bare
_add_speed_first = _add_bare
```

Both gated functions are **aliases of the bare three-instruction add**. `grep -c panic p4_rel.s` returns
3, and all three sit inside `_add_ungated_check` and the panic-location string table.

Debug, at `-O -C debug-assertions=on`:

```
_add_speed_first = _add_bare
_add_imitating:  add / tbnz / ... (check present)
```

The concern gate erases **even in a debug build**, so a speed-first path does not pay for a check that
exists for familiarity. That is I18's second bound working, and it is the half I would have guessed
least confidently.

**The control is what makes this a comparison rather than a demonstration.** `add_ungated_check` is the
same check with no const gate, and it keeps the branch in both builds. Without it, the clean release
output would be equally consistent with the optimiser deleting dead arithmetic for reasons having
nothing to do with the gate.

**Predicate.** A panic gated on a const build flag and a const concern flag is absent from the emitted
code: `I = 8, F = 0, signedness = unsigned, operation = add, arity = 2, chain length = 1, container =
u8, overflow policy = wrap-under-the-gate, S ∈ {imitating, speed-first} as defined in P4, target =
aarch64-apple-darwin, target features = host default, opt level = 3, debug-assertions ∈ {on, off},
threads any`.

## 10. Sameness: three relations, three licences, and no single answer

The dispatch asks what makes two primitives the same or different. There is no single answer and asking
for one is the shape `never-ask-which-single-rule-governs.md` names. Three relations, each licensing a
different operation, each with witnesses in P3:

- **Nominal.** The same name applied to the same arguments. Licenses **assignment without a cast**.
- **Representational.** Same value set and same bits. Licenses **reinterpreting memory**: an array of one
  read as an array of the other.
- **Denotational.** Same value set and same answers, whatever the bits. Licenses **a rewrite**: a law
  proved for one holds for the other.

Nominal implies representational implies denotational, and both implications are strict, which is what
makes all three necessary rather than one being enough:

- **W1, denotationally same and representationally different.** Same value set, same completion,
  different resting bits: one in its own byte, one packed at an offset in a shared word with non-zero
  neighbours. Answers agree on all 65536 pairs. A rewrite transfers; a memory reinterpretation does not.
- **W2, representationally same and nominally different.** Two markers resolving identically, same size,
  same alignment, same bits, and rustc refuses the assignment (P3b, quoted in section 5).
- **W3, the ordinary case.** Different value sets differ under all three, all 255 non-zero patterns.

The design choice is which of the three the type system is made to enforce, and the answer is different
per question. That is three arms with three predicates, not a policy to pick.

**One defect of mine here, and it is the exact one this workspace's test gate names first.** W1's inner
loop originally read `assert_eq!(x, y)` where both sides were the same call to the same function. That
is a tautology, it is not a test, and I wrote it by reflex while writing a file about not doing that.
The committed version routes the two values through the two realisations and compares what comes back.
The failure is recorded in the probe's comment rather than quietly fixed.

## 11. The criterion, which is what I would actually put in a canon

An enumeration of components ages badly and a criterion does not. What I would offer, as a suggestion
rather than a settlement:

> **A property belongs in the primitive iff it must be const-available in order to decide whether a
> program is valid, or to select a lowering. Everything else is an argument to an operation, or an
> internal choice.**

"Const-available" rather than "in the type" is op's own widening at I13: it "collapses to whatever is
available at const time", which reaches const functions and const data from outside the typestate, with
the typestate usable inside a const expression. The criterion inherits that scope and I did not choose
it.

It predicts the answers that already exist, which is the reason to trust it:

- **Width**: needed to decide whether an assignment is valid and to pick a container. In. Matches every
  real design.
- **Fraction position**: needed to decide what a bit pattern means. In. P1 direction B is the
  demonstration.
- **Overflow policy**: needed to select a lowering. In or out, and the criterion genuinely does not
  decide. The discriminator is I3: if it is in the operation, `+` cannot carry a meaning and every
  arithmetic site becomes a method call, which is not how a native primitive behaves. So I3 puts it in.
- **Rounding mode**: same shape, same answer, same reason.
- **Which SIMD width**: not needed to decide validity, and no consumer can get it wrong. Out, internal.
  Matches `arvo-always-optimal-internals.md`.
- **Thread count**: not const-available in general. Out. Matches I10.

The case where it under-determines is instructive rather than embarrassing. Signedness is needed to
select an instruction, `sdiv` against `udiv`, and it can live in the type (C, Rust) or in the operation
(LLVM, where `i32` is signless). Both have shipped, for decades, at scale. The criterion says either
works and the tiebreak is whether a consumer can get it wrong silently. That under-determination is a
true fact about the design space and a canon should say so rather than manufacture a rule.

## 12. Two misalignments I found while running the gate

Reported under the standing instruction, and not softened.

### 12.1 `RULES.md` states a ratified set that `INTENTS.md` contradicts

`RULES.md:571`, under the heading "Op's prior calls are op's voice and are NOT the top rung" (line 551):

> **The only ratified material remains `INTENTS.md`, I1 through I12.**

Against `INTENTS.md` § I1 (line 51), where it reads `~~The strategy set is closed at exactly four~~ **DEMOTED TO
OPEN**`, on op's direct word; `INTENTS.md` § "How to read an entry" (line 28), "**One entry holds this rung: I13**"; and the same
section at lines 29 to 34, which says the three that previously held RATIFIED were imported from the prior
panel's classification, are not to be trusted, and "**Do not import that rung again.**"

So `RULES.md` asserts twelve ratified entries where the catalogue says one, and among the twelve is the
entry op personally demoted within hours. A member who reads `RULES.md` for the provenance ladder, which
is exactly what it is for, and takes that sentence at face value, will treat the strategy set as closed
at four and will defend it as ratified. Two of this panel's governing files disagree about which
material governs, and the wrong one is the one whose job is to say.

The fix is one sentence and I am not making it, because `RULES.md` is not mine to edit mid-flight and
several dispatches are live against it. The dispatching agent should.

### 12.2 The generated agent instructions still carry a claim their own template retracts

`.claude/CLAUDE.md:70`:

> **None of them appears in `INTENTS.md`**, which holds op's ratified intents I1 through I12. [...]
> **do not cite them as ratified**, do not build a canon sentence on them without saying where they
> actually come from, and treat any argument that depends on one of them as resting on unratified ground
> until op says otherwise.

Against `INTENTS.md` § I14 (lines 268 to 297), which is **IN FORCE**, quotes op saying "They are very explicitly
also arvo intents and rules [...] No std, no alloc, all that is explicitly already in place and **not to
be questioned**", and states in its own words that "**The panel was wrong about these and the error
propagated**", naming five files that carried the hedge.

And against the template the generated file comes from, `mock/agent/MAIN.md.tmpl:61-70`, which was already
corrected and now reads:

> A previous version of this section said none of these appears in `INTENTS.md` [...] **That was wrong**,
> and it was wrong in the direction that does the most damage [...] Cite them freely. Build on them. They
> are in `INTENTS.md`.

**So the source was fixed and the generated leaf was not regenerated.** Every agent that opens this
repository loads the refuted version before it does anything else, including every member of this panel,
including me. I read it in my own context at the start of this dispatch. The template retracts it in as
plain language as anybody could ask for and the retraction reaches nobody, because the file that ships is
the one nobody regenerated.

**Two surfaces carry it**, and the class check is what found the second: `.claude/CLAUDE.md` and
`.github/copilot-instructions.md`. Both regenerate from `mock/agent/` on the next `cargo mock`. That is
the whole fix.

### 12.3 The reason 12.2 stayed hidden, which is worth more than 12.2

I nearly missed it, and the way I nearly missed it is a general hazard.

`grep -rn <pattern> . --exclude-dir=target` **does not reach `.claude/` or `.github/`.** `grep` on this
machine is ugrep 7.5.0 and a recursive search rooted at `.` skips dot-directories. Established with a
controlled test rather than inferred, committed at
`109_probes/p7_grep_does_not_reach_the_generated_agent_tree.txt`: the same unique marker written to
`.claude/` and to `mock/`, one found and one not, and naming the directory explicitly finds it.

The brief lists six ways this corpus has produced a meaningless green. **This is a seventh, running the
other way.** The panel's standing instruction is to exclude `target/` from every grep because a match
under it inflated a count 133 to 1. A member following that instruction with the default recursive form
also misses the two trees holding the instructions every agent here loads. "No file in the repository
says X", produced that way, is a description of the search rather than of the repository, and it reads
exactly like a finding.

Name the directories: `grep -rn <pattern> mock .claude .github`. Do not trust a bare `.`.

## 13. Alternatives I considered and did not take

Per the instruction to go wide by category and to describe what was not taken, so the next expert starts
from a list rather than from nothing. Each of these is a genuinely different category, not a permutation
of the axes I already hold.

**A. Primitive as a set with structure, the algebraic reading.** What I took, essentially: a carrier
plus operations. Strong on laws and on I13's predicate work, because a law's region is a region of the
parameter space. **Weak on cost**, and cost is half of what arvo exists for. The algebra says nothing
about why `Hot` and `Cold` differ.

**B. Primitive as a compilation request.** The type is not a thing but a specification handed to the
compiler: this range, this precision, and I care about speed more than size. Strong because it *is* what
the strategy axis is under I8 and I13. I folded its useful half into the request-and-resolution pair.
**The reason I did not take it whole**: if the resolution is target-dependent then a value written on
one machine cannot be read on another, which is fatal for I6's disk storage. That failure yields a real
derived constraint rather than only a rejection: **`ρ` for anything that persists must be
target-independent, and only `λ` may vary with the target.** Which is the always-optimal-internals split
derived instead of assumed.

**C. Primitive as a refinement type.** The type carries a range and arithmetic propagates it. I did take
this, in P5, and it worked better than I expected. What I did **not** do is take it all the way and let
it *replace* the completion component: a design where every primitive carries a range and `π` exists
only where the range cannot be proved. That is the most aggressive reading of I15 available and I think
it is the most interesting unexplored direction in this file. What would decide it: whether ranges can
be established at enough construction sites in real consumer code without the obligation becoming
noise, which is a question about hilavitkutin and vehje rather than about arvo.

**D. Primitive as a pair of a resting form and a working form.** Nobody computes on 13 bits; they widen
to a register. So `ρ` is not one map but a pair, plus the transport between them, and the
storage-minimising concern is the one that makes them differ. I think this is **right and I did not
develop it**, because P1 only needed one direction of it. It matters because it is what the
`bitpack-carrier-*` benches in this repository actually measure, and because it makes the storage and
speed concerns one axis (which realisation rests) rather than two unrelated ones. Whoever picks this up
should start from `mock/benches/` rather than from theory.

**E. Primitive as nothing at all; only operations exist.** The LLVM position: `i32` carries no
signedness and `sdiv`/`udiv` carry it. Radical, shipped for decades, and instructive precisely because
it is the opposite of arvo's instinct. It destroys the ergonomics I3 demands, so I rejected it, but it
is what made me realise the real question is not "what is a primitive" but **"how much of an operation's
specification has been moved into the type"**, which is where section 11's criterion came from. The most
valuable dead end in the file.

**F. Number system as a component.** Rejected as a level confusion, but there is a version I did not
kill. Residue number systems have a value set, a realisation and a completion like anything else, and
what makes them a "system" is that the **cost profile of the operations is radically different**:
addition cheap, comparison expensive. If "number system" means "which operations are cheap", it is a fact
about `λ` and belongs with `λ`. I did not probe this and the radix work in
`mock/benches/variants/quantiser-radix-*` is the nearest committed evidence to it.

## 14. What I did not cover

- **I did not run the bench suite.** No claim of mine is a measurement of speed, and none should be read
  as one. The two assembly sections are compile-and-inspect checks and say so in their own artifacts.
- **I did not read any panel file**, by construction. Where my findings duplicate or contradict earlier
  work, phase two is where that gets reconciled.
- **Every numeric result is at a model width**: 4 bits in P2, 8 in P1/P3/P4, `F = 8` in P6. I have **no
  transfer argument** to any other width for any of them, and `unstable-features.md` is explicit that a
  model-width check needs one and that the enumeration of routes by which behaviour can vary per
  instantiation is not exhaustive. Every predicate above lists the width as a fixed value for this
  reason.
- **P6 tested truncation only.** Round-to-nearest is where the per-step errors partly cancel and the gap
  should narrow. That is the obvious attack on section 8 and it is untested.
- **P5's range propagation covers add, subtract and multiply on non-negative operands.** The general
  product rule needs the min and max over all four corner products; mine takes the corners, which is
  correct only for non-negative ranges, and the probe says so in a comment rather than pretending
  otherwise.
- **I did not test whether the request-and-resolution pair survives contact with inference.** Every P3
  type is written out by hand. Whether a consumer can write `let x = a * b;` and have the resolution
  inferred without annotation is a real question and I did not touch it.
- **Nothing here is settled.** Op decides, and per I12 an opinion given before the experts converge is
  an ack. This is one expert, cold, with seven committed probes and two corrections of its own
  predictions along the way.

## 15. Probe index

All under `109_probes/`, all committed with their output, each committed as it ran.

- `p0_const_trait_spelling_on_the_pin.rs` and `p0_output.txt`. `const trait T`, not `#[const_trait]`.
- `p1_value_set_and_realisation_are_independent.rs`, `p1_output.txt`, and the rejected first version
  `p1_v1_lane_aligned_pack_was_not_a_pack.rs.rejected`. Value set and realisation vary independently.
- `p2_a_declared_law_is_a_claim_nothing_constrains.rs`, `p2_output.txt`, `p2_blocker_fn_ptr.txt`. A
  declared law compiles false and changes 952 of 4096 answers; a computed one cannot lie; closure is
  prior to every law.
- `p3_three_sameness_relations_and_what_each_licenses.rs`, `p3_output.txt`, and the compile-fail
  `p3b_marker_makes_identical_resolutions_incompatible.rs` with `p3b_output.txt`. Three sameness
  relations, three licences, both implications strict.
- `p4_the_bounded_panic_erases_under_lowering.rs`, `p4_output.txt`, `p4_asm.txt`, `p4_rel.s`,
  `p4_dbg.s`. Both of I18's bounds erase, with an ungated control.
- `p5_a_carried_range_eliminates_the_completion.rs`, `p5_output.txt`, `p5_asm.txt`, `p5_full.s`, and the
  compile-fail `p5b_overflowing_range_must_be_refused.rs` with `p5b_output.txt`. Range propagation with
  no feature gate, sound and tight, refused at compile time when it does not fit, and the completion
  absent from the emitted code.
- `p6_chain_accuracy_is_not_a_property_of_a_value.rs`, `p6_output.txt`. Deferred quantisation never
  worse over 3200 chains, with nothing about the operand type differing.
- `p7_grep_does_not_reach_the_generated_agent_tree.txt`. A controlled test that a recursive grep from
  the repository root misses `.claude/` and `.github/`.

---

# Phase two: reconciliation

Appended after the phase-one commit, which is untouched above. What I read for this: `110`
(the parallel cold derivation on the same question) in full, `106` sections 0, 1, 2 and 11, `63`
section 3.5, `90`'s result headings, `74`'s headings, and targeted greps across the live panel for
carried ranges, declared operand windows and completion elimination. I did **not** read `OPTIONS.md`,
`DROPLIST.md`, `PRIOR_CALLS.md`, the `SEED_*` files, the archive, or the bulk of the member files. That
bound matters and I state it rather than implying coverage: where I say "nobody has done X", read it as
"nothing I read did X", and the grep that backs it is named.

One phase-two probe, `p8_when_an_encoding_is_observable.rs`, built to test a reconciliation rather than
to derive anything. It is marked as phase two in its own header.

## 16. What survives, what I withdraw, and what was already known

### 16.1 Already established, so my file is a second or third instance rather than a finding

**The law set is not a component.** `90` R3 has it: "A law stated as an author-written marker is a
declaration checked by nothing." `110`'s F3 has it three ways at once, and its census is stronger than
mine: 48 configurations, 40 algebras, 7 law sets, and **0 configurations in which the law set can be
varied with the other coordinates held fixed**. A coordinate you cannot vary is not a coordinate, which
is a cleaner kill than my argument.

My P2 is a **third independent instance by a different method**, and I claim only that. What it adds
that neither has: the declaration is not merely unchecked, it is **load-bearing and wrong**, priced at
952 of 4096 answers changed by a rewrite that trusts it, with the witness `(-8, -8, 1)`; and the
constructive half, that the law computes at const time parameterised by the completion, with the
`function pointer calls are not allowed in constant functions` wall recorded as the reason the
completion has to be a **type** rather than a value for anything to be computable about it.

**Chain accuracy is not served by an operator closed over its operand type.** `106` section 11 has this
from `93`'s P6 and `94`'s probe E, both cold, both with more measurement than mine: 8.1% of triples at
`W = 8, F = 4`, a maximum absolute error of 238 raw units in a domain of 256, closed-form width growth,
and **11,922,158 ulps** divergence at a five-multiply chain where the widening construction stays at 1.

My P6 is a **third independent instance**. What it adds is the controlled framing: the two routes were
compared with **every per-value component held fixed**, so the result is specifically that no component
of the operand type moved, which is the form the answer to "is this part of a primitive" needs. And a
different shape of result: deferred was **never worse** on any of 3200 chains at lengths 1 to 8.

**And `106` carries a correction I did not have and could not have had.** `101` found that accuracy
rankings **cross**: a finer grid with truncation is twice as accurate at one operation and four times
worse at sixty-four, because bias accumulates linearly and unbiased error as a random walk, crossing at
`k = 4`. My "never worse" is at truncation, one grid, length at most 8. I named rounding-to-nearest as
the obvious attack on it in section 14; `101` has run something in that neighbourhood and the honest
reading is that my result is a narrower claim than it sounds, and that `101`'s region framing (chain
length as a **region dimension**, like thread count) supersedes any per-arm scalar reading of mine.

**The trait-instead-of-a-feature move for a const quantity.** `102_probes/p4_is_chain_depth_const_available.rs`
carries chain depth as a type with the number as an associated const, no feature gates, `E0080` on a
wrong depth. That is the identical mechanism as my P5, applied to a different quantity. So the mechanism
is not mine and is already at two instances before me; **what P5 adds is the target**, which is ranges
rather than depth, and the consequence, which is elimination of the completion rather than selection of
a policy.

### 16.2 Where I was wrong, and `106` is why

**I withdraw section 5's recommendation that the strategy marker be carried in the type as the
request.**

`106` section 1 states the object as a **pair**: a first component that is an assignment on the axes a
consumer can observe, "supplied and never derived, because a consumer of a value cannot recover it from
the bits", which **travels with the value**; and a second component that is a weighting over cost
coordinates, "resolved and never observed", supplied where the operation happens. Two components, two
carriers.

That is a better answer than mine and it dissolves the tension I posed rather than trading it off. My
section 5 treated the strategy as one thing and asked whether to carry it, and both answers had a real
cost: carry it and pay P3b's unrepairable nominal split, drop it and lose the intent. The pair says the
question was malformed. **Only the answer-fixing half has to travel**, because only it is unrecoverable
from the bits. The cost half never enters the type, so it can never split it.

And my own evidence lands on the pair's side rather than mine. In P3, `Speed` and `Space` resolve to the
identical realisation and completion; they differ only in what `106` would call the second component.
P3b then shows what putting that second component in the type costs:

```
error[E0308]: mismatched types
   = note: expected struct `WithMarker<_, _, _, Speed>`
              found struct `WithMarker<_, _, _, Space>`
```

**So P3b is a compiled demonstration of the failure mode the pair exists to avoid**, and it is
independent support for `102`'s object rather than an objection to it. That is the shape I would want it
recorded in. `110`'s F8 is the same wall arrived at from the canonicalisation side with a different
parameter, which makes three independent instances of "a nominal split that nothing in-language can
repair", from three different causes.

**I also withdraw the worry that motivated section 5.** I wrote that discarding the request means "the
consumer's source does not change and the consumer's meaning does". That is wrong under the pair, and it
is wrong under my own P3. If the resolution changes, the resolved type changes, and every downstream
signature naming the old resolution stops compiling, at exactly the sites that need review. The failure
is loud, not silent. I reasoned about it and did not check it, which is the error, and the check was
available in the file I had already written.

### 16.3 Where `110` and I agree, independently, and where I concede to it

Neither of us read the other. On my side that is the protocol; `110` declares the same and declares one
contamination of its own from an auto-loaded workspace rule.

**Agreed, three times over, that the law set is not a component.** By three different methods: `110`'s
coordinate census, my declaration-and-rewrite census, and `90` R3's earlier statement.

**Agreed that the operation set is part of the definition, arrived at from opposite directions.** `110`
names it the **signature** and prices it: 84 distinct primitives under `{add}`, 186 under `{add, mul}`,
over the same 288 configurations, so "how many primitives are there" is not well posed until somebody
fixes the signature. I arrived at the same place in section 8 from the chain result, concluding that a
primitive is not definable one at a time because the operators land in other primitives, so what can be
named is a family closed under the operations. Same claim, two vocabularies, two routes, neither having
read the other. **`110`'s is the better form**, because "declare the signature" is actionable and "it is
a family" is a description, and because it comes with a count.

**Agreed that a criterion beats a list.** `110`: "a list has to be relitigated every time somebody
invents an axis, and a criterion does not", demonstrated by feeding its criterion an axis nobody listed.
Mine is section 11's const-availability test. These are **different criteria at different levels** and I
do not think they compete: `110`'s decides *identity* (are these two the same primitive), mine decides
*membership* (does this property belong in the primitive at all). A design wants both, and I would not
merge them.

**Agreed that overflow and rounding are one map.** `110` argues it explicitly, `R : Q -> V`, rounding
between grid points and saturation outside the range being two regions of one mechanism. I called it one
component with internal structure and gave it one name. Same position, and `110`'s reason for insisting
is better than my reason for assuming: a design carrying them as two axes will be asked which one a
behaviour belongs to, and that question has no answer.

**Conceded: `110`'s treatment of "number system" is sharper than mine.** I said it is a name for a
package, one level up. `110` splits it: the radix is identity-bearing at `F > 0`, and the encoding is
presentation. That is a cut where I offered a category. Take theirs.

**Conceded, and this is the substantive one: I called the realisation a component of the primitive and
should not have.** `110`'s P1 puts four encodings including a structureless bijection through its
identity criterion and every one comes back the same primitive, and its reason is the one that matters:
a definition admitting the encoding into identity makes every representational choice semantic, "and
then no lowering can ever be substituted for another".

My P1 established that the value set and the realisation vary independently, which is true and which I
over-read. Independence is not identity-bearing-ness. **The realisation is a component of the
resolution and is not part of denotational identity**, and my own section 10 already says exactly that
and I failed to apply it to my own section 2. The correction is one word: format is two things, one
identity-bearing and one selectable, rather than two components of identity.

**And the realisation still has to be selectable, which is where I would push back on `110`'s wording.**
Its section 1 says the encoding and layout "are not the primitive and must not be, or nothing can ever
be rewritten". The rewriting argument is right. But I6 and I17 require a consumer to be able to *ask*
for the storage-minimising realisation, and I17 is explicit that this is not to be deprioritised. So
"not part of identity" must not slide into "not part of the surface". The three-relation lattice in my
section 10 is the vocabulary that keeps those apart, and I think it is the useful thing my file has to
offer here: **denotational sameness quotients the realisation out and licenses rewriting;
representational sameness does not and licenses reinterpreting memory.**

### 16.4 A conflict that is not one, tested rather than argued

`63` section 3.5 says the encoding "is not part of identity **and it is observable**", citing raw-order
agreement and raw-adder correctness. `110`'s F2 says every pure code assignment is presentation and
quotients four of them out. A reader could file those as a contradiction between a consolidation and a
cold derivation, and it would take a while to unpick.

They do not contradict, and the discriminator is testable. **An encoding is observable through an
operation exactly when that operation is defined on the representation rather than on the denotation.**
P8, `p8_when_an_encoding_is_observable.rs`:

```
encoding                     raw-order disagreements   raw-adder correct
two's complement                     128/256                256/256
offset binary                          0/256                  0/256
gray over two's complement           184/256                 62/256
arbitrary bijection 7i+3             112/256                  0/256
```

Under **denotation-defined** addition and comparison, all four encodings agree on **all 256 pairs**,
across four encoding pairs, zero differences. Under **representation-defined** raw comparison and raw
pattern addition they separate immediately, as above. So `110`'s quotient is a consequence of its
signature containing only denotation-defined operations, and `63`'s observability is a consequence of
naming two representation-defined ones. Both are right about their own level and neither needs
correcting.

**And `63`'s reported figures reproduce independently.** It reports, from `56_probes/q3`, "two's
complement 256 of 256 adder-correct and order-disagreeing, offset binary order-agreeing and 0 of 256
adder-correct with constant defect 8". At `W = 4` I get two's complement adder-correct 256 of 256 and
order-disagreeing on 128 of 256; offset binary order-agreeing 256 of 256, adder-correct 0 of 256, and
the defect set is **exactly `{8}`**, a single element, which is the constant-defect claim rather than a
paraphrase of it. `63` records that row at ONE EXPERT. This is a second, and it was run without reading
`56`.

**One number nobody I read has reported.** Gray code is adder-correct on **62 of 256** pairs: neither 0
nor 256. The exclusivity `63` states is about having *both* properties, and Gray has neither fully, so
this does not disturb it. It is on the record in case the redundant-encoding hole `63` flags as "wholly
unexamined" gets picked up, since a partial adder property is the kind of thing that hole would turn on.

**Predicate.** An encoding is invisible to denotation-defined operations and visible to
representation-defined ones: `W = 4, I = 4, F = 0, signedness = signed, overflow policy = saturate for
the denotation-defined arm and wrap for the representation-defined arm, operation ∈ {add, le}, arity =
2, chain length = 1, encoding ∈ {two's complement, offset binary, gray, 7i+3}, S any, target features
any, threads any` (purity). Four bijections, not all of them; `63`'s uniqueness argument is for
bijections and nothing here widens it.

### 16.5 What of mine I think still stands and is not duplicated

Stated conservatively, since three of my seven phase-one sections turned out to be second or third
instances.

**P5's completion elimination.** File `82` already has the region concept, "no clamp can fire", at
21.98% of one box, and uses it to license a **law lifting**. My P5 uses a carried range for a different
end: to establish that the completion is **absent from the emitted code**, with the compile-time refusal
when the range does not fit (`E0080`, naming the exact composition
`Ranged::<RSum<Lit<0,200>, Lit<0,100>>>::FITS`), and with the range rule checked for **tightness as well
as soundness**, which is the property a declaration would conflate. I found no file claiming the
elimination on the emitted-code side; the grep was `no clamp|clamp is gone|eliminat.*completion|
overflow check.*absent|removes the clamp|csel` across the live panel, and `82:494` is the nearest hit,
which shows the clamp chain **intact** on the unproved side.

**P4's I18 erasure.** `106` section 9.2 says I18 is new and that reading it against the object finds a
gap; I have not read that section. What P4 establishes is narrow and mechanical: both of I18's bounds
erase, the build bound in release and **the concern bound even in a debug build**, with an ungated
control that keeps the branch in both so the result is a comparison rather than a demonstration. If
`106`'s gap is about something else, this stands beside it; if it is about the same thing, this is a
second instance.

**The three-relation lattice.** Section 10, and section 16.3 is the case for it: it is the vocabulary
that keeps `110`'s quotient and `63`'s observability from reading as a conflict, and it names which
operation each relation licenses.

**Section 11's membership criterion.** Different question from `110`'s identity criterion, as argued
above.

**And the two misalignments in section 12**, which are about the panel's own governing files rather than
about primitives, and which nothing I read addresses.

### 16.6 One count where three independent measurements now disagree with a locked consolidation

`106` section 0.2 records the test gate "at 123 tests across 13 crates". `110` counts **124** across 13
and says so explicitly, by a different command from mine. I count **124** across 13. Two independent
counts, taken without either of us reading the other, against the consolidation's 123 and against the
brief's claim that six members counted 123.

Neither of us can say from inside a phase-one dispatch whether a test was added since, because that
means reading the log. It is one annotation and it is load-bearing for nothing. It is worth a line only
because `RULES.md` says counts are measurements and that six agreeing measurements are exactly the
shared-drift shape to be suspicious of. Whoever holds the register should re-run it rather than pick a
side.

### 16.7 What I would hand to whoever writes the canon text on this topic

Three sentences, offered as suggestions, none settling anything.

**A primitive is an algebra over a declared signature.** `110`'s form, not mine, because it comes with a
count showing the signature changes the answer. My contribution to it is that the family, not the
individual, is the unit of definition, which is the same claim.

**What is in the primitive is what must be const-available to decide validity or select a lowering; what
is identity-bearing is narrower still, being only what the value set and the realisation map read.**
Two criteria, two questions, both needed, and `110`'s section 5 rule ("an axis that the value set and
the realisation map do not read must not be a type parameter") is the sharp end of the second.

**Sameness is three relations, not one, and each licenses a different operation.** Denotational for
rewriting, representational for reinterpreting memory, nominal for assignment. Collapsing them is what
makes `63` and `110` look like they disagree when they do not.

And one thing I would put in the canon that is not about primitives at all: **closure is prior to every
law.** My P2 asked whether an operation associated before asking whether it stayed inside its own value
set, and the answer to the second question made the first meaningless. `110` hit the same wall from the
other side, its interval construction returning `(1, 0)` over a wrapping base, and resolved it as a
predicate on the base. Two instances, two directions, one statement.

## 17. Coverage of phase two, bounded

I read `110` in full, `106` sections 0 through 2 and 11, `63` section 3.5, and the result headings of
`90` and `74`. I read **none** of: `OPTIONS.md`, `DROPLIST.md`, `PERSONA_CALLS.md`, `PRIOR_CALLS.md`,
the `SEED_*` files, the archive, `108` (which is the immediate predecessor and is titled "the pair
attacked", so it bears directly on section 16.2 and I do not know what it says), or the ninety-odd
member files I did not name.

So section 16.5's claims of non-duplication are claims about a grep and about what I read, not about the
panel. Each names its grep. Anyone who knows a file that already has one of them should say so, and the
right handling is that my instance drops to a second rather than that anything of mine is defended.

I did not re-run anyone else's probe except through P8's independent reconstruction of `63`'s two
figures, and I did not verify `110`'s counts, `93`'s, `94`'s, `101`'s or `102`'s. Where I quote them
above I am quoting a file, not corroborating it.

## 18. Probe index, phase two addition

- `p8_when_an_encoding_is_observable.rs` and `p8_output.txt`. An encoding is invisible to
  denotation-defined operations and visible to representation-defined ones, over four bijections
  including a structureless one; and `63` section 3.5's two figures reproduce independently at `W = 4`,
  with the offset-binary defect set having exactly one element, `{8}`.
