# Panel 120: the contract split, converged

**Persona:** Adam Chlipala, correct-by-construction and proof-automation lens. Dispatched as the second
independent read on the call op reserved at `08b:47-51` and has now declined to make, and as the second
read `118:321-324` names as owed on the bound for `S`.
**Date:** 2026-08-05

**What I read in full, and in the order the dispatch set:** `110_consolidation_eleven.md` (the standing
base), `08b_op_checkpoint_three.md`, `108b_op_checkpoint_twentysix.md`, and
`09_chlipala_enforcement_and_attack.md`. I formed and compiled my own answer before opening
`117_spj_fused_or_split.md` and `118_the_missing_declarations.md`, which I then read in full and reconcile
against in section 7. **What I read in part:** `110` at its trait table (3001-3054), its identity contract
(828-890), and the `Number` declaration and its correction block (752-790).

**Directory listing done.** `ls` across the panel directory including every `NN_probes/`. `git log`
confirms `ee027e1` ("docs: panel file 118, the missing declarations") is the tip and that `118` edited
`110` in the same commit, which matters for section 0.

**Every line number I give for `110` is against `ee027e1`, and I pin it because the file moved under me
while I worked.** A concurrent dispatch is editing the standing base in the working tree, adding blocks
marked "Correction, file 121" from line 278 onward, and it has shifted everything below by 44 lines: the
`Number` declaration I cite at `110:759` currently sits at `803`, and the trait table at `110:3011` sits
at `3062`. The content is unchanged at both. This is the failure mode the workspace's own reference rule
names, a line citation that still resolves and now points at different content, and it is worth a line
here because **two members editing one shared base concurrently is a new thing in this panel** and the
next consolidation inherits both sets of edits with no record that they were written without sight of
each other.

**Gates.** The canon gate passes. The governing material is op's own checkpoint series, and `108b:184-186`
orders the remaining stretch to work the open list down rather than open ground, which is what a
convergence file does. Nothing here touches `mock/crates`, per `108b:188-193`. On the test gate I take
`118:47-56`'s position and my own: the suite covers a tree the canon replaces, op ruled at `108b:174-181`
that the collected tautologies are an implementation-phase checklist rather than something to act on now,
and a fourth report of the same three findings is what that ruling exists to stop. **The gate that binds
this dispatch is my own compiled evidence**, nineteen probes across seven crate topologies, each built
from clean under the pin, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024
--crate-type=lib`, in a scratch directory outside the tree. Every diagnostic quoted below is verbatim
output. The dispatch forbade adding files, so the sources are inline.

**Separation of evidence.** Sections marked *compiled* were built and the diagnostics are real. Sections
marked *reasoned* are argument. Where the evidence does not force one reading I carry both.

---

## 0. Breaking the brief, and one claim in it has already gone stale

The brief says the standing base never states what bounds `S`, zero hits across four searches. **That was
true when `117` ran and is no longer true.** `118` wrote the declaration into `110` in the same commit that
created it, and the base now carries it at `110:759`:

```rust
pub struct Number<N: Numeral, S: Policy + Lowering> {
    datum:    <S as Lowering>::Container,
    _numeral: PhantomData<N>,
}
```

with the provenance stated four lines below at `110:774`: the only statement of the bound anywhere in the
record is `26:28-35`, which is agent output, and `118` marked it owed. So the sentence exists, it is
correctly labelled, and what is owed is not the writing of it but a second reading of it. That is item one
of `118:321-324` and this file discharges it.

The brief's remaining factual claims hold. `09` really does work `Number<N, P, L>` throughout (`09:109`),
three parameters, against the shipped two. `117` really did find its predecessor citing the wrong line for
the split. And op's reservation at `08b:47-51` is verbatim what the brief reports: "The next member is not
asked to rule on whether the split is worth its cost, nor to choose between fused and split. Those are
op's, and they are downstream of whether enforcement is possible."

One thing the brief frames in a way I want to correct before using it. It asks me to establish "what the
split guarantees", and the honest finding is that the split guarantees something **narrower and harder
to defeat** than either `02` claimed or `117` credits it with. The narrowing is not a concession. It is the
part that survives attack, and it survives a wider class of attack than `117`'s account admits.

---

## 1. What file 09 establishes, re-derived at the shipped arity. Compiled.

`108b:11-20` requires a file building on a ratified sentence whose grounds have moved to re-derive it
rather than cite it. `09` is not ratified, but the same discipline applies with more force, because `09`'s
grounds have moved in a specific and checkable way: it worked a topology the design does not ship.

I rebuilt `09`'s question at the two-parameter arity. Five crates, `numeral` / `policy` / `lowering` /
`algebra` / `numeric`, each compiled separately with explicit `--extern` per edge.

**`09` section 1 survives the arity change.** The law crate can bound on the policy half alone and prove
its fact, with no `lowering` edge at all:

```rust
// crate `algebra_split`, compiled with --extern numeral --extern policy and NO --extern lowering
impl<N: Numeral, S: Policy> AddAssoc for Fact<N, S>
where
    S::Quantisation: StableUnderTranslation,
    <S::Quantisation as StableUnderTranslation>::Out: IsTrue,
{}
```

Builds clean. This is worth stating precisely because it was not obvious in advance: at three parameters
the law crate simply never mentions `L`, so blindness is free. At two parameters the law crate must
mention `S`, the very parameter that carries the lowering half, and blindness has to be earned by the
bound. It is earned.

**`09` section 3's gap survives the arity change too.** The crate that owns `Number` must have `Lowering`
in scope, because its own field projects through it (`110:759`), so it can condition the forwarded law on
the cost axis:

```rust
// crate `numeric`, the owning crate, all three linked
impl<N: Numeral, S: Policy + Lowering> algebra_split::AddAssoc for Number<N, S>
where
    algebra_split::Fact<N, S>: algebra_split::AddAssoc,
    S::Layout: IsDense,          // correctness conditioned on the cost axis
{}
```

Builds clean, one dead-code warning on `datum`. So the residual gap is invariant to arity.

**`09` section 5's closure does not survive.** Its mechanism is that `L` is a free parameter with no bound
(`09:229-231`, "the mechanism that actually does the work is that `L` is unconstrained in the type the law
targets"). At two parameters there is no free parameter to leave unconstrained, because the law needs the
policy half out of `S` and `S` is one type. `117:230-237` reaches the same conclusion and states it
correctly. I confirm it independently, and I add the consequence `117` does not draw: **with `09`'s
closure unavailable, the split is not merely one of two ways to get the guarantee. It is the only one the
panel has that transposes to the shipped shape.**

---

## 2. There is a second refusal, it is stronger than the first, and file 117 does not have it. Compiled.

This is the substantive thing I have that the first read does not, and it changes what the design should
write down.

`117` rests its entire case on the crate edge. Its summary sentence is explicit (`117:218-223`): "The
enforcement is the crate edge. The trait split is what makes a crate edge available in a useful place."
Every probe it reports on the split side refuses at `E0432`, unresolved import, which is a statement about
the dependency graph.

**There is a second, independent refusal that does not involve the dependency graph at all.** A bound on
`Policy` does not project a `Lowering` member, whether or not the crate is linked:

```rust
// crate `algebra`, compiled WITH --extern lowering, which it uses for an unrelated purpose
use lowering::StorageLayout;
pub fn unrelated<L: StorageLayout>() -> bool { L::PACKED }

impl<N: Numeral, S: Policy> AddAssoc for Fact<N, S>
where
    S::Quantisation: StableUnderTranslation,
    <S::Quantisation as StableUnderTranslation>::Out: IsTrue,
    S::Layout: lowering::IsDense,      // fully in scope. does it project?
{}
```

```
error[E0220]: associated type `Layout` not found for `S`
  --> alg_lowering_linked.rs:20:8
   |
20 |     S::Layout: lowering::IsDense,
   |        ^^^^^^ there is an associated type `Layout` in the trait `Lowering`
   |
help: consider further restricting type parameter `S` with trait `Lowering`
   |
16 | impl<N: Numeral, S: Policy + Lowering> AddAssoc for Fact<N, S>
   |                            ++++++++++
```

The same refusal fires on `S::Container`, and it fires under `pub const trait`, which is the design's
actual spelling and which neither `117` nor my own first pass had modelled (section 8).

Three things follow, and the third is a correction to the standing account of the trusted base.

**The split's guarantee is a typing fact, not only a crate fact.** `117:116-118` says `02`'s original
ground, "a typing fact rather than a review note", "is gone and does not come back", and `117:440-441`
repeats it. That is more than `08` refuted. `08` refuted the claim that the split *prevents a law from
being conditioned on cost*, and that refutation stands. The claim that a bound on one contract does not
project the other is a different claim, it is a typing fact in the ordinary sense of the term, and it is
the one doing the work. `117` discarded the label along with the overreach.

**The refusal names its own repair, at the line a reviewer reads.** rustc's help line spells the exact
edit an author would have to make, `S: Policy + Lowering`, in the impl header. A widening of a law's own
bound is a visible, reviewable, one-line change in the crate that declares the law. A `Cargo.toml`
dependency edge is not.

**Which makes `117:493-494` false.** Its trusted base says: "If the algebra crate ever acquires a
`lowering` dependency for an unrelated reason, the whole mechanism goes silently, with no diagnostic
anywhere." The probe above is exactly that scenario, the dependency acquired and used for an unrelated
purpose, and the mechanism does not go silently. It refuses, at `E0220`, with a help line. The mechanism
degrades from two barriers to one, and the remaining barrier is the stronger of the two.

This matters beyond correcting one paragraph. `117` proposes writing the dependency edge's absence into
the design as load-bearing (`117:494-495`). It is worth writing down, but as defence in depth rather than
as the mechanism, because a design that believes its guarantee lives in a manifest will treat a manifest
edit as a catastrophe and a bound widening as housekeeping, and the truth is the reverse.

---

## 3. What fusion costs. Compiled.

One trait carrying both halves. The law crate has no weaker bound available, because the members live on
one trait:

```rust
// crate `strategy_fused`
pub trait Strategy {
    type Quantisation: Quantisation;   // semantics half
    type Layout: StorageLayout;        // cost half
    type Container: Copy;
}

// crate `algebra_fused`
impl<N: Numeral, S: Strategy> AddAssoc for Fact<N, S>
where
    S::Quantisation: StableUnderTranslation,
    <S::Quantisation as StableUnderTranslation>::Out: IsTrue,
    S::Layout: IsDense,          // the cost half, conditioned on, in the LAW crate
{}
```

Builds clean, exit 0. There is no `--extern` set that refuses it, because the crate cannot read the
semantics half without linking `strategy_fused`, and `strategy_fused` carries both. `E0220` is unavailable
too, because a single bound projects every member of its trait. Fusion removes both barriers at once.

`117:158-186` reports the same result from a different topology. Two independent constructions, and this
is the one place in this file where I am corroborating rather than extending.

---

## 4. The exact perimeter of what the split guarantees

The brief asks for this precisely, and says naming the boundary is worth more than a verdict. It is.

**Genuinely unrepresentable.** A law whose impl is written in a crate that bounds `S` on `Policy` alone
cannot mention any `Lowering` member. Two independent refusals, `E0220` on the projection and `E0432` on
any attempt to name the crate. Neither depends on review discipline, and the first does not depend on the
dependency graph.

**Genuinely unrepresentable, second clause.** A crate that owns neither the law trait nor the numeric type
cannot add a law impl at all. `E0117`, the ordinary orphan rule, unconditional on anything this round
decides. `09:103-133` established it and I did not re-attack it.

**Merely inconvenient.** The crate that owns `Number` can condition the forwarded law on the cost axis
(section 1). It must have `Lowering` in scope because its own field projects through it, so no bound
discipline reaches it. This is the residual gap. It is one blanket impl per law trait, in one crate, and
it is invariant to fused-against-split, to arity, and to whether `09`'s closure is available.

**Merely inconvenient, second clause.** Whatever the physical type's own struct bound forces the law crate
to name is a discrimination surface. `117:246-286` found this and I reproduce it: a carrier whose
projection lands on a primitive is fully exposed, because primitives need no import.

```rust
// crate `algebra`, compiled with NO --extern lowering. Builds clean.
impl<N: Numeral, S: Policy + Carrier<Store = u16>> AddAssoc for Number<N, S> {}
```

**Trusted rather than proved.** That the law-bearing blanket impl is written in the crate declaring the
law trait. That the fact each `Resolution` computes is the right fact, which is `01`'s and `03`'s leaf
question and orthogonal to all of this. And that no trait in the design has both contracts in its
supertrait closure, which is section 5 and is the one entry on this list that can be converted from
trusted to proved by writing one sentence.

The shape of that list is the finding. **The split's guarantee is sound over the crates that consume the
contracts and silent over the one crate that composes them.** That is a real perimeter, it is worth
having, and describing it as either "enforcement" or "does not bind" gets it wrong in opposite directions.

---

## 5. The supertrait prohibition, confirmed, and it is worse than the first read states. Compiled.

`117:328-377` reports that a `Strategy: Policy + Lowering` convenience trait undoes the mechanism while
passing the `E0432` check. I confirm it and I have to report that its severity is understated.

`117`'s probe demonstrates the leak with `S::Layout: IsDense`, which needs `IsDense` nameable and
therefore still needs something from the lowering vocabulary. That leaves a reader the impression that the
leak is partial, and I initially reached the same conclusion: a bare projection is reachable but
non-discriminating, since a two-hop projection refuses with `E0223` unless the intermediate trait is
named.

**That impression is wrong, and the member that breaks it is `Container`.** `Container` is what `Number`
holds (`110:759`, `110:3027-3028`), and it projects to a concrete storage type. Storage types are
primitives, and primitives need no import:

```rust
// crate `algebra`, bounding on the roof trait, with NO --extern lowering,
// and naming NOTHING from the lowering vocabulary.
pub trait WideEnough {}
impl WideEnough for u16 {}

impl<N: Numeral, S: Strategy> AddAssoc for Fact<N, S>
where
    S::Container: WideEnough,
{}
```

Builds clean, and it discriminates:

```
error[E0277]: the trait bound `u8: WideEnough` is not satisfied
  --> check_discriminates.rs:12:33
   |
12 | pub fn packed() { needs::<Fact<Fix13_3, ColdPacked>>(); }
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `WideEnough` is not implemented for `u8`
   |
   = note: required for `Fact<Fix13_3, ColdPacked>` to implement `AddAssoc`
```

Two strategies with identical `Policy`, differing only in layout, disagreeing on whether addition is
associative, from a crate holding no lowering name whatsoever and passing every check that would be used
to verify it. A single roof trait restores the full fused-shape hole while the crate graph still reports
closed.

**And the prohibition wants stating directionally, because the direction is what makes it checkable.** I
compiled both directions:

A bound reaches every member of its trait's supertrait closure and nothing below it. `S: Strategy` where
`Strategy: Lowering` projects `S::Layout`, exit 0. `S: Carrier` where `Lowering: Carrier` refuses
`S::Layout` with `E0220`. So `117:371-372`'s claim that its topology B runs the edge the safe way is
correct, and I supply the negative it asserted without compiling.

This also confirms `118:161-166`'s free consistency check on the truth contract from the other side. The
edge there runs from the exit up to the algebra, so a crate bounding on `Truth` never gains a route to
`Branch`. Same rule, same direction, and the two now stand on one compiled statement rather than two
analogies.

---

## 6. Sealing, which op named and nobody had tested. Compiled.

`08b:34` lists the mechanisms nobody had probed: "Sealing, module privacy, a crate boundary, coherence
structure, a witness on the law itself, a marker that only a `Policy` member can produce." `09` tested the
crate boundary, coherence, and macro export. It did not test sealing, and neither did `117`. Since the
residual gap of section 4 is the design's one open enforcement question, sealing is the obvious next
proposal and someone will spend a round on it. I spent an hour instead.

Seal the law trait so the owning crate cannot impl it, and give the type a declared route in:

```rust
// crate `algebra_sealed`
mod private { pub trait Sealed {} }
pub trait AddAssoc: private::Sealed {}
pub trait CarriesFact { type N: Numeral; type S: Policy; }
impl<T: CarriesFact> private::Sealed for T {}
impl<T: CarriesFact> AddAssoc for T where Fact<T::N, T::S>: AddAssoc {}
```

**Sealing works, against the attack it is aimed at.** The owning crate cannot impl the law directly:

```
error[E0277]: the trait bound `Number<N, S>: algebra_sealed::private::Sealed` is not satisfied
  --> numeric_sealed_direct.rs:8:69
   |
 8 | impl<N: Numeral, S: Policy + Lowering> algebra_sealed::AddAssoc for Number<N, S>
   |                                                                     ^^^^^^^^^^^^ unsatisfied trait bound
```

**And it relocates the gap rather than closing it.** The owning crate conditions the carrier route
instead, one level down, and the discrimination reappears intact:

```rust
impl<N: Numeral, S: Policy + Lowering> algebra_sealed::CarriesFact for Number<N, S>
where S::Layout: IsDense
{ type N = N; type S = S; }
```

```
error[E0277]: the trait bound `Bitpacked: IsDense` is not satisfied
   = note: required for `Number<Fix13_3, Packed>` to implement `CarriesFact`
   = note: required for `Number<Fix13_3, Packed>` to implement `AddAssoc`
```

The reasoning generalises past this one construction, in the same way `09:171-196` generalised past its
one macro. Any route by which the owning crate attaches a law to `Number` is a route the owning crate
authors, and a route it authors is a route it can condition. Sealing moves the authoring site; it does not
remove it. **The gap is not closable by any mechanism that leaves the owning crate authoring the
attachment**, which is a sharper statement of the limit than `09` or `117` reached, and it closes off
sealing, module privacy, and marker-token schemes together.

---

## 7. Where I agree with the first read, where I do not

**Agree, and this is the second independent agreement the panel's rule requires: keep the split. Three
traits, two parameters, unchanged.** I reached it before reading `117`, from my own probes, and the
grounds are the same ones `117` gives in section 3 plus the stronger ones in my section 2. Fusion removes
both refusals; the split is the only shape that transposes to the shipped arity; and the split is the
incumbent, so rewrite cost is the tiebreaker and it points the same way.

**Agree on the bound.** `S: Policy + Lowering` is right, and my section 2 supplies a reason the record did
not previously have. The conjunction is not merely notation for "S does both". It is what makes the
weaker bound `S: Policy` a meaningful and enforceable thing for a law crate to write. A design that named
the pair with a single identifier would lose exactly that. So the bound and the supertrait prohibition are
the same decision seen twice, and they belong in one sentence.

**Agree that the supertrait convenience trait must be forbidden in the text.** More strongly than `117`
puts it, for the reason in section 5.

**Disagree on where the enforcement lives.** `117:218-223` locates it entirely at the crate edge. It is at
the bound first and the crate edge second. This is not a quibble about emphasis: it decides what the canon
writes down, and `117:493-494`'s trusted-base sentence is false as a consequence of getting it the other
way round.

**Disagree that `02`'s ground is dead.** `117:116-118` and `117:440-441` retire it. A narrower version of
it is exactly what section 2 compiles, and it is load-bearing. What died is the overreach, that the split
prevents cost-conditioned laws. What survives is that a bound on one contract does not project the other.

**Disagree, mildly, on rider two.** `117:452-458` wants the carrier extracted into its own crate below
`Lowering` now. The direction is right and my section 5 strengthens the case, since `Container` is the
member that breaks the supertrait barrier. But `118:95-101` observes that `Number`'s declaration is
unchanged under either home, and my probes agree: the extraction changes where a projection travels, not
what the type is. It is a real improvement and it is not on the critical path, so it belongs on the open
list rather than in the ratifying sentence, and the spelling rule matters more than the crate does.

**No disagreement at all on the diagnostic measurement.** `117:395-408` measures the trait-count question
at zero diagnostic cost and frees the coupling from the open list. I did not re-measure it and I have no
reason to doubt it.

---

## 8. What neither of us checked

**The const-traits interaction, which I then checked.** The design's contracts are `pub const trait`
(`110:3011-3030`). Both `117`'s probes and my own first pass modelled them as plain traits, and the whole
of section 2 rests on `E0220` behaviour that could in principle differ under a const trait. It does not:

```rust
pub const trait Policy { type Quantisation: Quantisation; }
impl<N, S: Policy> AddAssoc for Fact<N, S> where S::Container: Copy {}
```

```
error[E0220]: associated type `Container` not found for `S`
```

One incidental fact worth recording next to `118:218-221`'s. A `[const]` bound is refused on a non-const
impl ("this impl is not `const`, so it cannot have `[const]` trait bounds"), so law impls take plain
bounds on const traits, which is the shape used above and is the shape the design will actually write.

**Whether arvo's real `Container` is spellable from the law crate.** `117:317-324` flags it and declines
to resolve it; I flag it too and can be more specific about the test, because my section 5 probe is that
test. The check is one attempted equality constraint in a crate with no lowering edge, `S: Policy +
Carrier<Store = ...>`, which either resolves or does not. **It should be run at the moment the carrier is
declared and not before**, since the answer depends on a spelling nobody has written.

**Whether the law impls proliferate.** `117:430-434` names this as the number that would decide its
section 7 fork and could not find it. I could not either. I add only that my section 6 result narrows why
it matters: since no mechanism closes the gap while the owning crate authors the attachment, the review
surface is exactly the set of attachment sites, so the count is the whole question and not merely an input
to it.

**Leaf truth.** Untouched here, as in `09` and `117`. A closed enforcement story around a wrong fact is a
well-defended wrong answer.

---

## 9. The sentences the canon should carry

Written as the canon should state them, per the dispatch. Three are ratifying, one is a prohibition, one
is a perimeter statement, and the last is an open-list line.

> **The split.** The design declares three contracts. `Numeral` names what a number is, `Policy` names how
> it behaves, and `Lowering` names what it costs. `Numeral` is separate because what the number is does not
> change through strategies, which is D54 and is ratified. `Policy` and `Lowering` are separate because a
> bound on one does not project the other, and that is what lets a crate read the semantics of a number
> without being able to name its cost.

> **The bound.** `Number<N: Numeral, S: Policy + Lowering>` carries two parameters, and the second
> implements both strategy contracts. The bound is written as this conjunction wherever both halves are
> read, and the weaker bound `S: Policy` is written wherever only the semantics are. **The weaker bound is
> a declaration that the code does not read the cost axis, and the compiler checks it**: a `Lowering`
> member projected off a `Policy` bound is refused at `E0220` whether or not the lowering crate is linked.

> **The prohibition.** No trait in the design may have both `Policy` and `Lowering` in its supertrait
> closure. A bound reaches every member of its trait's supertrait closure, so a convenience trait
> `Strategy: Policy + Lowering` hands the cost axis to every crate that bounds on it, silently, while a
> dependency-graph check still reports closed. If the name `Strategy` returns it returns as a type alias
> for a bound at consumer-facing positions, or as prose, never as a supertrait. Edges running the other
> way are safe and are how the carrier is factored: `Lowering: Carrier` is permitted, `Carrier: Lowering`
> is not.

> **The perimeter, stated because a guarantee whose limits are unstated is a claim rather than a
> guarantee.** The contract split is enforced over every crate that consumes the contracts and is silent
> over the one crate that composes them. A crate bounding on `Policy` cannot mention a `Lowering` member
> (`E0220`, and `E0432` if it names the crate). A crate owning neither the law trait nor the numeric type
> cannot impl the law at all (`E0117`). **The crate that owns `Number` can condition a law on the cost
> axis, and no mechanism prevents it**, because its own field projects through `Lowering` and any route by
> which it attaches a law is a route it authors. Sealing the law trait moves that route without removing
> it. The residual surface is one blanket impl per law trait, in one crate, and it is a review obligation
> with a compile-fail test, named rather than closed.

> **On the open list.** Whether the container projection moves to a one-member carrier contract below
> `Lowering`. The declaration of `Number` is unchanged either way; what changes is the path the projection
> travels and whether the law crate can spell the carrier's inhabitants. The spelling rule is the
> load-bearing half: the carrier's inhabitants must not be nameable from the law crate, which a primitive
> always is and a generic form parameterised by lowering-side markers is not.

---

## What this changes, and what it costs

Nothing that ships. The split stays, the arity stays, the consumer's call site is identical, and the
declaration `118` wrote at `110:759` is correct as written and now has its second read. What changes is
that the bound acquires a reason that survives `08`, `09` and `117`; one prohibition gets written before
somebody builds the thing it forbids; one false sentence in a trusted-base section gets corrected; and one
family of proposed repairs gets closed off with a compiled reason rather than left for a future round to
rediscover.

On the dispatch's convergence pressure, plainly: **this is settled and downstream is not blocked on it.**
Two independent reads agree on the split, on the bound, and on the prohibition, each from its own compiled
evidence, and where we disagree the disagreements are about which mechanism to write down rather than
about what to build. The residual gap is real, invariant to the choice, and correctly handled as a named
review obligation. If op wants the one-line ratification it is the first sentence in section 9; if he
would rather not spend the line, the design is not waiting on him for anything except the carrier's home,
which is already on the open list where it belongs.

---

## Trusted base

Section 2's refusal trusts nothing beyond the trait solver's own projection rule, which is why I prefer it
to the crate-edge account: it holds under an accidental dependency, under `pub const trait`, and under any
manifest.

Section 4's perimeter trusts that the design's law impls are the enumerable set section 8 says nobody has
counted. If they are generated per numeral or per preset, the review obligation in the perimeter sentence
is not dischargeable by review and the carrier extraction stops being optional.

Section 5's prohibition trusts that `Container`'s eventual spelling is a type, which it must be, and that
some lowering will project it to something nameable from the law crate, which is true today for every
primitive and is the reason the prohibition is not merely tidiness.

And this file trusts, as `09` and `117` both did, that `StableUnderTranslation` computes the right fact
for each `Resolution` constructor. Nothing in the enforcement story touches that, and it remains the
largest unexamined thing under all of it.
