# Predicates: the arity question, and what a typed predicate is actually for

**Date:** 2026-07-29
**Kind:** research summary. Points at two sketches; carries no decision.
**Sketches:** `sketches/202607282230_hlist-arity-dissolution/` and
`sketches/202607290030_typestate-algorithm-selection/`.

This exists so the two sketches are findable from one place. Both came out of one question about
`arvo/src/predicate.rs`, and the answer moved a long way from where it started.

## How it began

`Pred<A>`, `Pred2<A, B>` and `Pred3<A, B, C>` are three trait aliases over `Fn(..) -> Bool` at
arities one, two and three. Only `Pred2` has callers, twice, both in `arvo-comb`. `Pred` and `Pred3`
are declared, re-exported through the facade, and used nowhere.

That is the shape D4 of `202607281220` already rejected for capacities: "needs impls generated per
arity and caps rank at whatever is written". The round rejected it for shapes and left it standing
for predicates in the same crate.

## What the first sketch found

Seven constructions, 41 assertions, mutation-checked. Full ladder in that sketch's FINDINGS.

Two are complete, enforced and gate-free, and the choice between them is one trade:

- **Shape F**, a `Deref` wrapper carrying the list as typestate. Consumers write `f(a, b)`, ordinary
  call syntax, because **call position autoderefs** and `Deref` is stable to implement. Validation is
  a marker trait whose per-arity impls collapse to one macro line.
- **Shape G**, a recursive `Describes` with two impls and zero arity anywhere, at the cost of curried
  call sites, `f(a)(b)`.

**Shape F is the semantically correct one**, and not for ergonomic reasons. `greedy_group`'s
`feasible(&acc, &item)` is a joint atomic test, not a chain of refinements; currying it would express
structure the domain does not have.

Two walls are recorded there because they are properties of the language rather than gaps in effort:
`Fn(&A, &B) -> Bool` names all its argument types in **one bound**, and this toolchain has **no
variadic generics** among its 247 unstable features. And `unboxed_closures` / `fn_traits`, which would
give literal call syntax directly, is vetted **forbidden** (#29625, `S-tracking-design-concerns`, open
since 2015).

## The finding that changed the question

Checking the canon for why `Pred` exists surfaced a design-versus-source drift.

The locked src changelist of round `202604470000` specifies **no supertrait**, blanket impls with
real bodies, and a doc comment stating that "custom callable types implement `test` directly for
stateful predicates". The shipped file has `: Fn(&A, &B) -> Bool` as a supertrait on every trait,
empty blanket impls, and a rewritten doc comment describing the family as "equivalent to
`impl Fn(&A, &B) -> Bool`, but reads as a named concept".

**The supertrait forecloses the capability the round designed for.** A struct cannot implement the
shipped `Pred2`:

```
error[E0277]: expected a `Fn(&u32, &u32)` closure, found `Budget`
```

So the family's one functional justification was removed after the CL locked, and the documentation
was updated to describe what was left rather than what was decided. Same failure class as the
`generic_const_exprs` WATCH entry found the same evening.

Stripped of that, the remaining justification is naming, and naming does not survive: the parameter is
already called `feasible`, and `feasible: impl Fn(&A, &T) -> Bool` names the concept where a reader
looks.

## What the second sketch found, which is the justification that holds

A closure type carries no properties. A named predicate type can. That lets a library **select an
algorithm from a property in the type**, with no runtime branch and without the consumer ever naming
an algorithm.

Verified from emitted assembly:

- The discarded algorithm is **absent**, not cold. Each instantiation references only its own.
- Correctness, not only speed: binary search is wrong for a non-monotone predicate, and the type
  system is what keeps it away.
- At **one** choice, typestate, const-generic `bool` and `const fn` are **byte-identical**; LLVM
  aliases them to a single symbol. "Typestate gives better codegen" is false.
- At **breadth**, three properties and nested branches, the instantiations diverge completely: 14, 18
  and 98 instructions for three predicates through one entry point.
- **`const fn` cannot discriminate on a type at all.** `const fn is_monotone<P>()` compiles and
  returns the same value for every `P`. The only construct that makes a compile-time constant vary by
  type is an associated const, which is typestate.

**So the value is not performance. It is that the property cannot desync from the thing it
describes.** With a `const fn` the property is tied to nothing; with a const-generic `bool` it is
restated at every call site; with typestate it lives on the predicate type and a consumer cannot pass
the wrong flag because there is no flag to pass.

This is `Strategy` one level up: a property in the type, read by the trait solver, selecting the
implementation before runtime, without the substrate hardcoding a threshold or policing the consumer.

## What is open, and where it belongs

**The properties are unpopulated.** `MONOTONE`, `SORTED_OK` and `CHEAP` were invented to exercise the
mechanism. Whether any arvo algorithm has a property that legitimately unlocks a better variant is a
**bench question** and belongs in `mock/benches/` per `bench-in-bench-harness-never-sketches.md`. The
existing arvo and hilavitkutin benches already show variants dominating in different bands; the
properties worth encoding are the ones that **name a band boundary**.

**There is a soundness obligation.** An asserted property is an unchecked promise, and a false one
produces a wrong answer rather than a slow one. That is a different risk class from `Strategy` and
closer to an `unsafe` contract. It wants a stated contract per property and probably a debug-mode
cross-check against the naive algorithm.

**And the narrow question remains.** Whether `Pred` is restored to its locked shape, replaced by
Shape F, or deleted, is a decision for the round. Nothing here decides it.

## The part that reshapes the stack, not just the predicate question

The predicate question turned out to be a doorway. Pushing the same mechanism further showed it
selects **microkernels from the shape**, at a granularity that hand-written dispatch cannot reach.

One generic kernel, `sum<C: Capacity>(v: &C::Array<u32>)`, instantiated four ways: `Dim<16>` compiles
to 8 instructions with **zero** conditional branches, `Dim<17>` to 10 with zero, `Dim<1024>` to 20
with one, and the runtime-length `&[u32]` to **65 with nine**. Same source. `Dim<17>` has no tail
loop at all; its remainder is two inline instructions.

Then properties the type **derives from itself**, `LANE_ALIGNED = N % 4 == 0` and `SMALL = N <= 8`,
select among three explicit hand-written kernel bodies. `Dim<8>` gets the flat kernel at 5
instructions, `Dim<64>` the unrolled-by-four at 26, `Dim<66>` the pairwise at 39. Three distinct
bodies, no merging, zero conditional branches in any. **The consumer wrote `Dim<8>`, `Dim<64>`,
`Dim<66>` and nothing else.**

Hand-written microkernel dispatch means writing the cross product by hand, extent band times
alignment times strategy times sign times rank, as cfg branches or a table, every entry a place to
get it wrong and to keep in sync. Here **the type is the cross product**, the properties are derived
from it, and impls exist only for combinations that earn one.

**This is the mechanism notko's `#[profile]` rewriting reaches for and the one arvo's `Strategy`
markers already are.** `Strategy` is a property in the type selecting storage; `Capacity` is typestate
carrying an associated array; `AccessSet` is an hlist already proving disjointness. Nothing new is
needed for solver-driven selection to reach kernel choice: it is the trick the stack uses in three
places, pointed at a fourth. It sits inside `arvo-always-optimal-internals` (which licenses asm
microkernels chosen by bench) by adding a per-shape axis alongside the per-target one, and inside
`arvo-toolbox-not-policer` (which forbids hardcoded thresholds) because a derived property guesses
nothing: the consumer states a shape and the substrate reads a fact about it.

**Two things this makes newly important.** Monomorphisation cost is now a real bench question, since
every distinct shape can instantiate a distinct kernel. And derived properties (computed, cannot lie)
must stay visibly separate from asserted ones (promised, can lie), because from a call site the two
look identical.

Evidence and reproduction in `sketches/202607290030_typestate-algorithm-selection/FINDINGS.md`,
Results 5 and 6.
