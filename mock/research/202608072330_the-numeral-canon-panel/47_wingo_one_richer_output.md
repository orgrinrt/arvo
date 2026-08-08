# 47. One richer output, or two

**Date:** 2026-08-09. **Persona:** Andy Wingo. **Mode:** explore, do not settle (`00_brief.md`,
`04`, `28`). **Position in the unit:** fourth file on one topic, after `44`, `45`, `46`, and `45`'s
reply. Nothing here settles anything.

**Status: COMPLETE.** Written to disk early per `RULES.md:302-304` and extended in place.

My subject is the alternative `46` raised and nobody closed: does "the derivation needs a carrier
and a stride" mean two outputs, or does it mean the carrier was under-specified and one richer
output suffices. `46:62` puts it as "a derivation whose **codomain is the container alone**", and
that qualifier is doing work nobody has cashed out.

## 0. Gates

### 0.1 Canon gate

There is no ratified canon to defend or diverge from. The fixed material is `01`, `04`, `28`,
`INTENTS.md`, the workspace discipline, the forbidden-feature list, and the acceptance criterion in
`00_brief.md`'s "What is fixed". My question sits inside it: the criterion's second noun is exactly
what "how many outputs" is about, and `00_brief.md:145` now reads "the matching container and
numeral **representations**", plural, fixed between `45`'s writing and `46`'s reading (`46:41-48`).
I opened it myself and confirm the plural is present. Nothing below proposes anything the
forbidden-feature list excludes; every probe compiles or refuses gate-free. **Gate: passes.**

### 0.2 Test gate

There is no suite. `mock/crates` is empty by construction. My evidence is eleven probes in
`47_probes/`, six that compile and five that are expected to fail to compile, plus citations I
opened and checked. Five of the eleven are negative controls, because five of my six positive
results would otherwise be the "asserting a value against itself" shape `the-test-gate.md` names
first, and `46` caught exactly that shape in `45_probes/p4` one file ago.

### 0.3 Independence, declared up front

My brief gave a reading order that puts `45` and `46` first, so **I did not derive cold.** Where I
land where they land, that is a third read and not a third independent instance, and I say so
rather than claiming a rung, per `RULES.md:334-350` and following `46:32-39`'s handling of the same
problem. Where I attack, independence is not needed: an attack has to be correct and carry its own
citation.

I did not run `git log` in this repository at any point, per `RULES.md:352-360`.

## 1. Three questions, which the panel has been running as one

Before answering anything I want to separate what "how many outputs" has been used to mean, because
the three readings have different answers and files `15` through `46` slide between them.

**Q-sufficiency.** Is one output enough to determine everything a downstream site needs? This is
what `16:126-141` answers, by injectivity failure, and what `46:50-78` re-derives. Answer: no,
where "one output" means one machine type.

**Q-reducibility.** Given more than one fact, is one recoverable from another? This is what `16`
section 6 answers (`16:255-282`), what `16_probes/p5_recovery_direction.rs` sweeps, and what `45`
section 2.2 and its `p1` are actually about, whatever their prose says. Section 4 below.

**Q-packaging.** Given sufficiency needs more than a machine type, must the derivation's codomain
be a product of several items, or can it be a single richer item? **This is my question, and no
file in the panel has posed it.**

The three are independent. Q-sufficiency's answer does not constrain Q-packaging at all, because
**any product is one thing**: a pair is a single element of a single set, and any single thing with
two projections is a pair. So "two outputs" as a claim about arity is not falsifiable as stated,
and the useful version of it has to be a claim about something else.

`16` already knows this and says so, in the one line of the panel that anticipates my whole file:

> "How many outputs" needs a criterion for what counts as an output, or the answer is
> unfalsifiable: everything downstream is a function of the declaration, so you can always claim
> one output and call the rest recomputation. (`16:95-97`)

Its criterion follows at `16:100-101`: "a component is an output of the derivation when the
consumer did not write it, the machine needs it, and a downstream site that holds the other
components cannot recover it."

**That criterion is about the observation surface, not about the codomain.** It quantifies over
what a downstream site holds and what it can recover. So `16`'s own definition of "output" already
commits the panel to Reading B (how many facts must be independently observable), and the
two-output finding, read against `16`'s own criterion, says nothing about how those facts are
packaged. Every later file has read it as a claim about packaging anyway, including `OPTIONS.md`'s
account and both of `45`'s and `46`'s framings.

## 2. What I built, and the answer: one richer output suffices if and only if it is a type

The dispatch asked me to build the single richer output and see whether it holds. I built it, and I
built its two natural spellings, because they do not have the same answer.

### 2.1 A single type-valued output holds

`47_probes/p1_single_type_output.rs`. The derivation has exactly one associated item, `type Repr`,
in every strategy. `Repr` is not a machine type; it is a type per (packing discipline, width), and
the carrier, the packed access type, the stride and the width are projections of it through a
`Representation` contract. It compiles gate-free, and it repairs `16`'s collapse by construction:

```
the collapse 16:126-141 names, and its repair under one output:
  width  carrier(collapses)  single output(does not)
  9      u16                 Packed<W9>   stride=9
  ...
  16     u16                 Packed<W16>  stride=16
```

Eight declarations, one carrier, eight distinct single outputs. `47_probes/p1b` asserts three false
type equalities and is refused three times, so the `SameType` bridge doing the distinctness work is
not vacuous:

```
error[E0277]: the trait bound `Packed<W13>: SameType<Packed<W16>>` is not satisfied
error[E0277]: the trait bound `Packed<W13>: SameType<Padded<W13>>` is not satisfied
error[E0277]: the trait bound `u16: SameType<u32>` is not satisfied
```

### 2.2 A single value-valued output is refused by the compiler

This is the reading a reader naturally takes from "a bit count is insufficient": make the count
richer. Encode carrier width, stride and access width into one const. It is **lossless as
information**, so every argument in `16`, `44`, `45` and `46` that turns on information content is
satisfied by it.

`47_probes/p2_scalar_single_output_refused.rs` builds it and is refused, six times, in three
syntactic positions:

```
error: generic parameters may not be used in const operations
  --> p2_scalar_single_output_refused.rs:95:48
   |
95 | pub type CarrierA<W, S> = <Nat<{ carrier_bits(<W as DeriveScalar<S>>::REPR) }> as NativeFor>::T;
   |                                                ^ cannot perform const operation using `W`
   |
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

Same text at the type-alias, function-return and no-arithmetic positions, the last of which
(`Nat<{ <W as DeriveScalar<S>>::REPR }>`) is refused even though it performs no arithmetic at all,
because an associated const projected from a generic is itself a const operation on that generic.
`generic_const_exprs` is forbidden (`00_brief.md:158-159`), so the route is closed.

`47_probes/p2b_kind_asymmetry_positive.rs` is the other half, so the refusal is a statement about
direction rather than about a broken encoding. The same scalar output yields every **const** fine,
generically, in a const fn body:

```
direction: scalar output -> const, generically
  stride_of::<W13, Cold>()  = 13
```

and a type-valued output yields consts and types both:

```
direction: type output -> type, generically
  widen_one::<W13, Cold>(8191u16) -> u32 = 8191
  widen_one::<W13, Warm>(8191u16) -> u16 = 8191
```

One source line, two return types, chosen by the derivation. `p2` could write none of that.

### 2.3 The asymmetry, stated

```
type  -> const    total, free, gate-free, concretely and generically
const -> type     refused, naming a forbidden feature
```

So **one richer output suffices if and only if it is a type.** As a value it is compiled-refused by
the same wall `16_probes/p5b_const_to_type.rs` hit, reached from the opposite direction: `16` was
trying to recover the carrier from the stride, and I am trying to recover it from a lossless
superset of the stride. The loss of information was never what closed that route. **The kind
boundary was**, and `16:272-282` says so in prose ("recoverable by arithmetic is not the same as
available at the type level") without noticing that this makes information-recoverability the wrong
test for every verdict the panel then reached with it. Section 5.

### 2.4 What that answers, and what it does not

It answers the dispatch's question: a single richer output does hold, and it satisfies `Cold`'s
injectivity failure, the alignment case and a third compute carrier at once (section 6). It is not
a competing design to the pair. Once the single output is a type with named projections, it **is**
the pair wearing one name, and the count that survives is the count of projections, not the count
of associated items on the derivation trait.

So the honest resolution is that the one-versus-two question was never a fork. Both files describe
the same mechanism at different levels of packaging, and choosing between them is choosing a
spelling, which `RULES.md:76-77` puts outside what a canon may contain.

**What is a fork, and this is the part worth carrying:** how many facts must be available **as
types**. That number is at least two and possibly three, it is not the same as the number of facts,
and every "not a third output" verdict in the panel was adjudicated on the wrong one of the two.

## 3. Where I attack `45`, and it survives `46`'s downgrade

`46` downgraded `45`'s wide-rung alignment forcing from unconditional to conditional
(`46:143-158`), `45` conceded (`45:553-583`), and the register now carries it as a real second
forcing waiting on an unratified alignment axis. **I think it is not a forcing of the two-output
requirement at all, conditionally or otherwise, and the reason is visible in `46`'s own words.**

`46:134-137`, describing what survives its attack:

> at widths where the byte counts happen to coincide, the pair **`(declared width, stride)`** is
> identical between the two strategies while the carrier types differ, because alignment is a
> property of the type and not of the byte count.

The pair being collided is `(declared width, stride)`. That is not the derivation's output pair.
The derivation's outputs are `(carrier, stride)` with the carrier **a type**, which is `16`'s own
insistence at `16:610-613`:

> It is recoverable from the **carrier**, because `align_of` is a property of a type, so it rides
> on output 1 and is not a third output. That result has a consequence worth stating: **it is a
> reason the carrier must be a type rather than a width.**

So `45`'s collision is a collision on the key `(width, stride)`, which is Q-reducibility, and `45`
section 5.2 (`45:314-333`) already established nobody proposes a design that drops the strategy and
keys on that. It is not a collision on the derivation's output pair.

I checked this rather than arguing it. `47_probes/p5` reconstructs `45`'s `W = 256` witness at the
same shape, and `47_probes/p5b` asserts the two carriers are the same type and is refused:

```
error[E0277]: the trait bound `AlignedWideBits32: SameType<WideBits32>` is not satisfied
   --> p5b_negctl_forcings.rs:104:29
    |
104 | const _: () = assert_same::<FlatCarrier<W256, Hot>, FlatCarrier<W256, Warm>>();
```

with the size equality and the alignment inequality const-checked in `p5`:

```
F2  alignment divergence, W=256, Hot against Warm
    flat pair:     stride 256 both; carrier size 32 both; ALIGN 16 against 1
                   -> SEPARATES
```

**So the flat pair, as `15` and `16` actually state it, separates `Hot` from `Warm` at `45`'s own
witness.** `45_probes/p1`'s 40-of-640 count is a count of collisions in a `(width, stride)` key, and
`45_probes/p2`'s "0 against 40" is the demonstration that a bit-count carrier cannot see the
difference, which is a fact about a bit-count carrier and is the thing `16:610-613` had already
refused to build.

**What I would put to `45` and `46`.** The conditional the register now carries ("if any two
strategies diverge in alignment, the pair's irreducibility is forced a second, independent way")
should be narrowed once more, to Q-reducibility: it forces `(width, stride)` not to determine the
carrier. It does not bear on whether one output suffices, because the carrier that would have to be
collapsed is a type and alignment rides on it. `45_probes/p7`'s abstract alignment lemma is correct
and its subject is the same key. I am not asking either of you to retract a compiled result; I am
saying the result answers Q-reducibility and the register files it under Q-sufficiency.

## 4. A forcing nobody has stated: under `Precise`-widens, TWO is already insufficient

This one goes the other way and strengthens `45`.

`16` section 6 (`16:264-270`) frames the `Precise` question as: if `Precise` widens, "two
declarations with the same extent have different carriers, and the map from extent to carrier is
not a function." That is Q-reducibility again. `16_probes/p5_recovery_direction.rs` sweeps it and
reports 64 of 251 extents mapping to two carriers.

The sharper statement is about Q-sufficiency and nobody has made it. Take `Warm` and `Precise` at
the same width, under the widening reading. `Precise` stores exactly what `Warm` stores, so:

- same stride,
- same storage carrier,
- **different compute carrier.**

The pair `(carrier, stride)` does not separate them. `47_probes/p5` compiles this as an assertion
that holds:

```
F3  Precise widening, W=13, Precise against Warm
    flat pair:     stride 16 both; carrier u16 both -> DOES NOT SEPARATE
```

and `47_probes/p5b` includes the same equality as a claim that **must not** be refused; it produces
no error, which is the positive half of that control. The two claims that must be refused are, and
the file's error list contains exactly them.

So under the widening reading the two-output shape is not merely irreducible, **it is
insufficient**, and the register's sentence "if `Precise` does widen, the pair is irreducible as a
matter of arithmetic, full stop" understates what happens: the pair stops separating declarations
that behave differently, which is the same defect `Cold` produced against a one-output derivation,
one level up.

`45` section 4 (`45:252-288`) showed the third output is mechanically free and correctly concluded
the remaining uncertainty is about op's intent. I agree with the conclusion and I would add the
finding it did not state: the third output is not an addition to a sufficient pair, it is the
repair of a pair that has become insufficient. That is a stronger reason to settle `Precise` than
"the character of the finding turns on it".

## 5. Where I attack `16`: information is not the right test, and its own criterion says so

`16` dismissed two third-output candidates. Both dismissals used the same test and the test is
wrong for one of them.

**Alignment: correct dismissal.** `align_of::<T>()` is a const from a type, which is the free
direction of section 2.3. It genuinely rides on the carrier and needs no slot. `16:605-613` is
right and I confirm it.

**The packed access width: dismissed on information, and the information is not the quantity
needed.** `16:186-189`:

> the maximum byte span of a `W` bit field at unknown phase is `floor((W + 6) / 8) + 1`, a function
> of `W` alone... So a site holding `(carrier, extent)` can compute the access width without
> re-entering the derivation.

Compute, yes, as a number. What a lowering site needs is the **type** it loads into. `16` knows the
carrier is the wrong answer for it at 28 of 64 widths (`16:384-386`) and that reaching for the
carrier is the trap that produces its own 24-of-64 truncation. So the site must name a different
type, and `16`'s closed form has to cross the kind boundary to supply one.

`47_probes/p3_access_type_from_const_refused.rs` transcribes `16`'s closed form verbatim and tries
to reach a type from a const-carried width, which is the kind arvo's own consumer surface writes
(`UFixed<13, 3, S>`: two const generic arguments). Refused three times:

```
error: generic parameters may not be used in const operations
  --> p3_access_type_from_const_refused.rs:70:57
   |
70 | pub type AccessOf<const W: u32> = <Bytes<{ access_bytes(W) }> as AccessFor>::T;
   |                                                         ^ cannot perform const operation using `W`
   |
   = help: const parameters may only be used as standalone arguments here, i.e. `W`
```

`47_probes/p3b` builds the two routes that do work and separates who closed which:

- **Route A**, width stays a const, drop the arithmetic, key the trait directly on the width. Legal.
  One impl per width, 128 of them for the range, which is the enumeration the design refused four
  times (quoted at `16:485-488`). **Closed by the design, not by the compiler.**
- **Route B**, width as a type, access as an associated type on the derivation result. Legal, one
  blanket impl per strategy, no new enumeration. **Open**, and it is the same
  refused-bound-wants-a-trait move `45` section 4 used for the third output.

So `16`'s verdict survives, and it survives **because 15's design happens to carry the width as a
type**. It is a fact about the width's kind, not about the closed form, and any arrangement that
carries the width as a const literal loses it. This connects to `10`'s bridge problem, which `44`
section 5 (`44:298-325`) correctly flags as sitting underneath the two-output finding and unsolved:
the bridge from a written const literal to a structural nat is what makes route B reachable at all.

### 5.1 And the verdict costs a second ladder, which nobody has named

If the access type is a projection of the width, the next question is whether it comes off the
ladder that already exists. `47_probes/p6_two_ladders_not_one.rs` computes both partitions of
widths 1 to 128 exhaustively:

```
native rung jumps at W = [9, 17, 33, 65]
access rung jumps at W = [2, 10, 26, 58, 122]
jump points shared by both partitions: []
classes: native 5, access 6, common refinement 10
widths where the native carrier is the WRONG load type: 35 of 128
```

**Zero shared jump points.** Neither partition refines the other, so one width ladder cannot key
both: a design needs two ladders, or one over the common refinement, which has ten classes against
five and six. That is a real cost of the "not a third output" verdict and it is unstated anywhere I
read.

One cross-check worth recording: my `access_bytes`, transcribed from `16:187` and rounded to a
power of two, reproduces `16`'s reported 28-of-64 figure exactly over widths 1 to 64. That is
agreement between two codings of one closed form, which is one instance wearing two hats, not two
independent derivations.

## 6. Does the answer change if a third output turns out to be needed

The dispatch flagged that a framing generalising badly to three is suspect at two. I tested it and
**the worry is mostly not borne out at the mechanism level**, which is worth saying since keeping
something is a result.

`47_probes/p4` compares. Adding a compute-carrier projection changes the contract and its impls in
both forms, equally. A downstream function bounded on the derivation does not change in either.
There is no arity churn to speak of.

Two things do differ, and one of them matters.

**The subject of a type-level property.** The two-output finding rests on injectivity. `p4` arm A
shows a componentwise assertion is satisfied by the very collapse it is meant to catch:
`assert_same::<u16, u16>()` compiles and is true of all eight colliding `Cold` widths. Eight green
assertions over a derivation that has already lost the distinction, which is `16`'s own tautological
`size_of` check (`16:229-232`) wearing a different name. The joint assertion needs **one subject**,
and `p4b` refuses it correctly once one exists:

```
error[E0277]: the trait bound `Pair<u16, 13>: SameType<Pair<u16, 16>>` is not satisfied
error[E0277]: the trait bound `Pair<u16, 9>: SameType<Pair<u16, 16>>` is not satisfied
```

while the carrier-only assertion on the same two declarations produces no error at all, and its
absence from the error list is half the result. **So the property the whole finding rests on is
statable at the type level only if a single type carrying the whole result exists.** Whether you
call that "one output" or "the pair reified" is naming; the type has to exist either way, and the
flat form as written does not have one.

**Reification by tuple churns; reification by declaration does not.** `p4` arm C: `Pair<C, STRIDE>`
becomes `Triple<C, STRIDE, X>` when the compute carrier lands, and every assertion naming it is
rewritten. A result keyed on the declaration (`Packed<W>`) does not move: `p4`'s three `assert_same`
lines are character-identical to `p1`'s, and `p1`'s contract carried two projections where `p4`'s
carries three.

## 7. Supporting the converged claim, by a route neither `45` nor `46` used

`RULES.md:212-214` says support counts as much as attack, and my brief asked for an independent
derivation if one was available. Mine is not independent in the strict sense (section 0.3), so this
is a third read with its own reasoning rather than a third instance. The reasoning is different
enough to be worth writing down.

`16` and `46` both derive more-than-one from **non-injectivity**: the map from declarations to
machine types collapses fibres. My route is dual and starts from the codomain rather than the map.

Ask what a machine type makes observable. For a native Rust type `T`, exactly three things:
`size_of::<T>()`, `align_of::<T>()`, and `T` itself as a type. Rust's array layout law fixes
`size_of::<[T; N]>() == N * size_of::<T>()` with no exception, which `16_probes/p2` const-asserts.
So the stride of an aggregate of `T` **is** `8 * size_of::<T>()`, by definition, and there is no
other slot on a bare native type for a different number to live in. A packed stride is a different
number by construction. Therefore a derivation whose codomain is the native machine types cannot
carry a packed stride, and the reason is not that the map loses information; it is that **the
codomain's elements have no slot for the fact**.

That reaches the same place from the other end, and it explains why the repair is what it is: the
fix is not a bigger codomain in the sense of more distinguishable elements, it is a codomain whose
elements have somewhere to put the answer. Which is section 2.1's `Repr`, and which is also why
`16:719-721`'s attack ("could the second output be folded into the carrier by making the carrier a
packed type? No. There is no Rust type of size 13 bits") is correct about the candidate it names and
does not close the general question: it refutes folding the stride into `size_of`, and folding it
into an associated const on a richer type was never tried.

## 8. Bearing on the live options

Per `RULES.md:239-241`. I cite `OPTIONS.md` by section and quoted phrase, never by line, per my
brief.

**The derivation's outputs section.** *Corrects a framing, kills nothing, adds one forcing and
narrows another.* The converged claim (more than one, forced unconditionally by `Cold`) stands and
I support it by the codomain route in section 7. What I would change: the section states the
finding as a count of outputs, and the count is not falsifiable as stated; it should state the count
of facts that must be available **as types**. Under that statement the wide-rung alignment
conditional moves out of this section into the reducibility one (section 3), and a new
Q-sufficiency item arrives: under `Precise`-widens the pair itself stops separating declarations
(section 4).

**The `Precise`-on-inexact open item.** *Sharpened, in the same direction `45` sharpened it, on
different evidence.* `45`'s pigeonhole argument prices the widening reading's information cost. My
section 4 prices its structural cost: it is not a third fact added to a sufficient pair, it is the
repair of an insufficient one. `45`'s question for op is unaffected and I would keep it worded
exactly as it stands.

**Whether the two-output shape is forced by arithmetic or only by the type system.** *Fits badly,
and I would retire the dichotomy rather than answer it.* `45` section 5.3 already argues "forced by
semantics" is the honest label and I agree with its reasoning. My addition is that the "type
system" half is not a weaker kind of forcing at all: the kind boundary in section 2.3 is a fact
about what a compile-time derivation can hand a lowering site, and it would hold in any language
where types and values are different sorts, which is nearly all of them. Calling it
"type-system-shaped" and therefore softer has the polarity backwards.

**`10`'s bridge problem, which `44` section 5 flags as sitting underneath.** *Newly load-bearing.*
Section 5 shows the access type is reachable only because the width is carried as a type. The
bridge is what makes that true for a consumer who writes a literal. So a finding the register
currently files as adjacent to the two-output thread is a precondition of one of its verdicts.

**Q5, is the arithmetic column one axis or two.** *No bearing.* My work is about the container
derivation's codomain, not about the arithmetic axes.

**Q6, `Warm`'s overflow policy.** *No bearing.*

**Q11, what the numeral guarantees to a fold.** *A small addition.* `35`'s accumulator reach is
another quantity needed as a type at a lowering site rather than as a number, so it lands on the
same side of section 2.3's boundary as the carrier and the access type. I have not checked it and I
am naming the connection rather than claiming it.

## 9. What I would put to `45` and `46`, since both may be resumed

Concrete, located, falsifiable.

**To `45`, one question.** `46:134-137` describes your wide-rung collision as a collision on the
pair `(declared width, stride)`, and your own `p1` output reports the two carriers as "different
carrier TYPE". `16:610-613` insists the carrier must be a type precisely so alignment rides on it.
Given that, does your section 2.2 finding bear on whether one output suffices, or only on whether
`(width, stride)` determines the carrier, which your own section 5.2 says nobody proposes keying
on? I built `47_probes/p5b` expecting the flat pair to separate your `W = 256` witness and it does,
by refusal. If you agree, the conditional you and `46` converged on should move from the sufficiency
thread to the reducibility one, and its epistemic weight goes down again rather than staying where
the register now files it.

**To `46`, one question.** You wrote that the general mechanism "does not need `Precise` to produce
it" and belongs in the register as a conditional result. I agree it is a real mechanism. Do you
agree that its subject is the `(width, stride)` key rather than the derivation's output pair, and
that with a type-valued carrier the pair separates the two strategies at every witness either of
you built? If so, the conditional is about a keying nobody proposes, which is a smaller claim than
"a second, independent forcing waiting on an unratified axis".

**To both, one question that goes the other way.** Under the `Precise`-widens reading, `Warm` and
`Precise` at the same width have the same carrier and the same stride and different compute types
(`47_probes/p5`, `p5b`). Does that make the pair **insufficient** rather than merely irreducible,
and if so, does the register's sentence about `Precise` deciding "the character of the finding"
need replacing with one saying `Precise` decides the finding's **arity**?

## 10. What I would add to the register

I am not editing `OPTIONS.md`, `INTENTS.md` or `00_brief.md`, per my brief.

**Replace the count of outputs with a count of kinds.** The sentence that survives a rewrite is not
"the derivation has two outputs". It is: *the derivation's result must make available, as types,
every fact a lowering site cannot recompute from a const; facts recoverable as consts from those
types are not further outputs.* That statement is arity-free, survives a third projection, passes
`RULES.md:79-83`'s permanence test where the count does not, and it is what all four files in this
unit have actually been establishing.

**A kind-asymmetry entry, with its two compiled refusals.** `type -> const` is total and gate-free
generically (`47_probes/p2b`); `const -> type` is refused, naming a forbidden feature
(`47_probes/p2`, six refusals in three positions; `47_probes/p3`, three more; `16_probes/p5b`, four
more, from the opposite direction). Three probes by two authors now hit the same wall from three
different starting points, which is closer to the three-instance bar than anything else on this
thread.

**A new Q-sufficiency item for `Precise`.** Under the widening reading the pair `(carrier, stride)`
does not separate `Warm` from `Precise` at the same width (`47_probes/p5`, `p5b`). The register
currently carries only the reducibility consequence.

**A narrowing of the wide-rung alignment conditional**, per sections 3 and 9.

**A cost on the access-width verdict.** `16`'s dismissal stands and carries an unstated cost: the
native and access rung partitions of widths 1 to 128 share no jump point, so a design needs two
width ladders or one over their ten-class common refinement (`47_probes/p6`). The ladder is the part
of the derivation the design has refused to enumerate, so a second one is not bookkeeping.

**And a dependency the register does not draw.** The access type is reachable as a type only because
the width is carried as a type, so `10`'s bridge, which `44` section 5 files as adjacent, is a
precondition of a verdict inside the two-output thread rather than a neighbouring problem.

## 11. What I could not determine

**Whether any lowering site in arvo's actual design needs the packed access width as a type at
all.** A packed read can be written as a width-generic byte loop, which needs no per-width load
type and is correct. What it costs against a fixed-window load is a codegen-quality question, and
no bench harness has run in this panel, so it is **unpriced** and I will not reach for a number. My
section 5 establishes that the type is unreachable from a const-carried width; it does not
establish that anyone needs it. That gap is the honest boundary of the finding.

**Whether the access rung partition stays coprime to the native one above 128 bits.** `47_probes/p6`
sweeps 1 to 128 only. The wide rung's access story depends on a multi-limb payload's load strategy,
which `16:539-543` also declined to derive, and I did not invent one.

**Whether `35`'s fold-accumulator reach falls on the type side of section 2.3's boundary.** I named
the connection in section 8 and did not check it. `35` is on my not-read list.

**Whether a single type-valued output has a cost I have not found.** I looked for one. The
candidates I considered and could not make bite: an extra projection layer at every use site (real,
and it is one associated-item hop, which monomorphises away); a name in the canon (real, and small);
and the possibility that `Repr` types proliferate one per (discipline, width) rather than per
discipline (real in my probe, and an artifact of my scaffolding ladder rather than of the form,
since `Padded<W>` is one generic type). None of those is a compiled refutation and I would not
present them as one. Someone attacking this form rather than building it may find what I did not.

**Whether the three-instance bar is met for anything here.** It is not, for anything of mine alone.
The kind asymmetry is the closest: `16_probes/p5b`, `47_probes/p2` and `47_probes/p3` reach the same
refusal from three different starting points, by two authors, which is two instances at best under
`RULES.md:116-118`. A third author attacking the wall rather than confirming it is what would move
it.

## 12. Coverage, bounded honestly

**Read end to end, directly:** `INTENTS.md`, `00_brief.md`, `RULES.md`, `45` in full including its
sections 11 and 12 reply, `46` in full, `44` in full, `16` in full.

**Read at the specific passages I cite, by opening the lines:** `15` lines 260 to 559 (sections 3.2
through 5, covering the map, both stride defects, the wide rung, and the two passages `46` used to
downgrade `45`), `OPTIONS.md` lines 690 to 819 (the derivation's-outputs section as it currently
reads, read last per my brief and never cited by line), `16_probes/p6_trait_form_recovers_both.rs`
in full as source.

**Every `file:line` in this document was opened and its content checked against my claim**, not
merely resolved. I checked eight passages this way in one pass before writing sections 3 through 7,
and one of them changed what I wrote: I had `46`'s attack filed as being about the carrier pair
until I opened `46:134-137` and found it names `(declared width, stride)`, which is section 3.

**Not read:** `02` through `14`, `17` through `43`, `DROPLIST.md`, `PERSONA_CALLS.md`, `SETTLED.md`,
`archive/`, `seed/`, the closed predecessor panel, `mock/benches/`. I did not open `45_probes` or
`16_probes` beyond `16_probes/p6`; where I refer to `45_probes/p1`, `p2`, `p4`, `p6` or `p7` I am
relying on `45`'s and `46`'s accounts of them, both of which quote their outputs inline and one of
which (`46` on `p4`) audited the source directly. **That is a real exposure**: my section 3 argues
`45_probes/p1` collides a different key than the register files it under, and I argue that from
`45`'s and `46`'s prose descriptions of the probe plus my own reconstruction of its witness in
`47_probes/p5`, not from `p1`'s source. If `p1` actually keys on `(carrier, stride)` with a
type-valued carrier, section 3 is wrong and my reconstruction is the thing that misled me.

**Not verified:** `43`'s grid apparatus, `35`'s fold accumulator, `10`'s bridge beyond `44`'s
account of it, and every figure in `16` section 7 except the 28-of-64 access-width count, which
`47_probes/p6` reproduces from `16`'s stated closed form.

**Probes:** `47_probes/`, eleven files with sources, outputs and committed compiler errors, plus
`verify.sh` which rebuilds and reruns the lot and reports each expected refusal's error count.
Toolchain `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`. Zero `#![feature]`
gates: `grep -c '^#!\[feature' 47_probes/*.rs` returns 0 for all eleven, and `verify.sh` prints it.
Five of the eleven are negative controls, and the two that matter most (`p4b`, `p5b`) each contain
one claim that must **not** be refused, so its absence from the error list is part of the result
rather than an oversight.

**No bench harness ran.** Every magnitude question this file touches, what a byte-loop packed read
costs against a fixed-window load in particular, is **unpriced**.
