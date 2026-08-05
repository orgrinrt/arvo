# The truth contract's exit: the split is real, the bound is on the wrong half, and the algebra says why more sharply than either file states

**Author:** Peyton Jones (files 07, 46, 92 in this panel)
**Date:** 2026-08-05
**Position:** after `107_lattner_is_a_or_contains_a.md`. The dispatch is the one independent read the
truth-contract fork has been waiting on since `106b` set it.

## What I read

`102_consolidation_ten.md` as the standing base, in full at its opening section, section 1.25, section
1.26, section 3, section 4 and section 5. `103_leijen_platform_and_the_predicate.md` at its fork
pricing and its section 3.5, plus `103_probes/OUTCOMES.md` and `103_probes/p1_foundation.rs`.
`105_chlipala_the_owed_second_reads.md` at its section on the truth refinement, lines 260 through 340.
`106_giesen_one_pattern_or_two.md` at sections 2.4, 3.1, 3.2 and 3.4. `106b_persona_checkpoint_
twentyfive.md` in full. `ls` of the panel directory, once, which is how I found that file 107 had
landed after the checkpoint that set this dispatch and that its subject is the capacity fork rather
than mine.

Outside the panel, because the fork is downstream of ratified text and I would rather read the text
than a summary of it: `mock/design_rounds/202607300800/202607290200_topic.the-predicate-decisions.md`
(D15, D16, D17) and the D27 passage in `202607300700_topic.consolidated-round-state.md`.

I formed the reading in section 1 and section 5 below, and ran `p1`, `p2`, `p5` and `p9`, before
opening files 105 and 106. That ordering is the whole reason this file is worth anything, and where the
independent reading and the prior files converge I say so rather than presenting agreement as a
finding.

## Gates, run before the work

**The test gate.** `cargo test --offline --workspace` from `mock/`: **155 binaries, 672 passed, 0
failed, 9 ignored.** Matches the count `102:39` reports, run fresh by me rather than inherited.

I read bodies rather than names in the surface this dispatch touches, which is the truth-returning
contracts and their implementors. Two things to report, one of them new.

The three tautological tests are present and unchanged, re-read at source this session:
`arvo-tensor/tests/capacity.rs:14-18`, `arvo-tensor/tests/const_capacity.rs:49-53`, and
`arvo-hash/tests/aliases.rs:16-23`. They are op's, disposed at `95b`, and the panel is right not to
touch `mock/crates`. I am the twenty-eighth file to report them and I will not spend more than this
paragraph on it, except to agree with the checkpoint that `content_hash_roundtrip` contains no round
trip: its body constructs the same value twice from the same literal and asserts the two compare
equal, which tests reflexivity of `PartialEq` on a `Copy` type and nothing about hashing.

**The new one, and it lands directly on this fork's own ground.** No Boolean-algebra law is asserted
anywhere in the tree. `arvo-bitmask/tests/mask_ops.rs` holds eleven tests, all per-operation examples
at hand-picked bit positions: `union_ors_bits`, `intersection_ands_bits`, `complement_flips_all_bits`,
`empty_union_is_self`, `full_difference_is_empty`. Not one of them asserts an equation. There is no De
Morgan, no distributivity, no absorption, no double complement, no idempotence, on `Mask` or on `Bool`
or anywhere else, checked this session by
`grep -rln "de_morgan\|distribut\|associat\|complement\|idempot\|absorpt" --include="*.rs"` across
`mock/crates`, whose only hits in a test file are `mask_ops.rs` and `mask_const_arith.rs`, both by the
word `complement` in a per-operation test name.

That matters here specifically. File 105's ground for the whole fork is that Boolean algebras are an
equationally axiomatised class and varieties are closed under direct products. The theorem is about the
class. **Membership of the design's own two candidates in that class is asserted nowhere**, so the
argument's minor premise is the unchecked one, and 672 green tests say nothing about it.

Under the gate's own missing-fundamentals clause that is a disqualifying finding, and I want to be
exact about why I am proceeding rather than refusing. The remedy is a test file under `mock/crates`,
which is the boundary this panel does not cross and which `102:1014` records op ruling on for the
tautologies. A refusal would leave the fork unread with op waiting on exactly one read, and would not
produce the tests either. So: reported, named as an owed artifact in section 7 with its exact shape,
and the fork's algebraic ground is flagged as resting on an unchecked premise until it exists. If op
would rather have the refusal than the read, that is his call and this paragraph is where he makes it.

**The canon gate.** Aligned, and the check found something the fork's statement does not carry. The
governing text is the design round at `mock/design_rounds/202607300800/`, op-ratified with decisions
marked inline. D17 is directly on this question and it already answers half of it:

> notko declares the **contract for a truth value**, exactly as `Cardinal` is the contract a count type
> implements per D6, and arvo's `Bool` implements it. The predicate's output names that contract rather
> than a concrete type, so nothing in the signature needs bare `bool` and nothing needs `Bool` to be
> reachable from notko.

That is `202607290200_topic.the-predicate-decisions.md:46-50`, Decision (op, 2026-07-29). It settles
the parameterised spelling for the predicate family. The fork as `102:649` states it is about the
numeral contracts, which is a different family at a different layer, so D17 does not decide it. It does
mean that under branch A the design would name its truth value two ways inside one contract family at
one layer, which is the condition the widened definitional-completeness line names, and which D27
itself was undoing when it observed that "splitting one ordering family across two crates" is what
created the situation it fixed. I record that as an input to op's call, not as the call.

Nothing I propose below requires reopening a ratified decision.

## 0. The answer, first

**The split is real, and it is more forced than either file argues.** File 105 says a mask "cannot
supply an exit for free". The stronger statement, and it is exhaustively checked rather than reasoned:
the Boolean-algebra homomorphisms from an n-lane truth algebra to the one-lane one are **exactly the n
coordinate projections**, and neither `all` nor `any` is one of them (`p9`, all 16 candidate functions
at n = 2 and all 256 at n = 3, every equation at every pair of points). So the exit is not merely
underdetermined by the algebra. Above one lane the operations consumers actually want are **outside**
the algebra, and at exactly one lane the unique homomorphism is the identity. That is the coincidence
file 106 observed, with a reason under it.

**The third clause is backwards, and it costs the fork its own purpose.** Binding the fifteen
declarations on the exit-carrying part refuses the multi-lane instance at the impl, `E0277`, `Mask2:
Branch` is not satisfied (`p2`). The multi-lane instance is the entire thing branch B buys over branch
A. The repair a reader reaches for compiles and is worse: naming `All<Mask2>` as the producer's truth
relocates the silent choice from the trait to the impl, where a consumer's bound cannot see it and a
caller wanting any-lane has no route at all (`p2b`). **The fifteen declarations bind on the algebra.
The exit is required where the branch is, which is a different set of sites, and none of the fifteen is
one of them:** all fifteen are producers, checked by reading them.

**"Never a default" is expressible against two of five introduction routes and no more.** All five
compile today (`p4`), and two of them disagree with each other about what the default means while both
look canonical. `impl !Branch for Mask` converts the default-body route and the blanket-impl route into
`E0751` under `negative_impls` alone, without the forbidden `with_negative_coherence` (`p4b`, `p4d`,
`p4e`). The inherent-method, `Deref` and `From` routes are not refusable and stay rules with a grep.

**The exit costs nothing where it is satisfied by identity, and less than nothing is not available to
measure.** Three spellings of one branch, the raw primitive, the concrete truth newtype, and the
generic-over-truth version, merge into **one symbol** at `-O`: `_b_concrete = _a_raw` and
`_c_generic_at_scalar = _a_raw` (`p3`). That extends file 103's result, which compared the two branches
to each other, by adding the raw primitive to the same equivalence class. The exit also costs nothing
*new*, because `if` demands the language's `bool` and any truth type that is not `bool` already pays one
projection at every branch today.

**The exit does not belong to the truth contract.** It belongs to the operation, and the operation that
generalises is a selector keyed on the pair rather than on the truth. `max` written once against such a
selector is correct at one lane and at two, with no exit anywhere, and at one lane is byte-identical to
the raw primitive (`p5`). Routing a lane-wise `max` through an exit is not unavailable, it is **wrong**:
at `a = [7,2]`, `b = [3,9]` the lane-wise answer is `[7,9]`, reducing with `all` gives `[7,2]` and
reducing with `any` gives `[3,9]`, two different wrong answers (`p5b`, executed).

**And file 103's thunked selector is the exit under another name.** `select` on the truth type and
`is_true` are mutually definable, in both directions, for any type at all (`p8`). A truth contract
carrying the thunked selector carries an exit whether it says so or not, and inherits the whole
reduction problem while looking structural.

The fork locks at **branch B, bound on the algebra**. Section 6 states it in the form the consolidation
could take.

## 1. The split is real, and here is the sharp version

The separation requirement asks that a claim about a distinction be checked where the distinction is
nonvacuous. Every returning site in the design today is one lane, and one lane is precisely the vacuous
region, so the check has to happen at a two-lane instantiation. That is what `p1`, `p2`, `p5b` and `p9`
do, and `p9` does it exhaustively.

Start where I think the argument should have started. A Boolean algebra has five pieces of structure:
`and`, `or`, `not`, `TRUE`, `FALSE`. An exit is a function from the carrier to the language's `bool`.
Ask the only question that matters: **which functions from the two-lane carrier to the one-lane one
respect all five?**

There are sixteen candidates at two lanes. Enumerate them all and test every equation at every pair of
points. Two survive, and they are the two coordinate projections. At three lanes, of two hundred and
fifty-six candidates, three survive, again exactly the coordinate projections. `all` is not among them
and `any` is not among them, at either width (`p9`).

That is worth pausing on, because it is not what the prior files say. File 105 says the reductions are
"not derivable from the Boolean-algebra structure alone", which is true and is the weaker claim. The
sharper one: the structure *does* determine maps out of a product, there are exactly n of them, and
**not one of them is an operation any consumer wants**. `all` is a meet-semilattice map and breaks
`or`; `any` is a join-semilattice map and breaks `and`. So a design that reasons "the algebra will tell
us what the exit is" gets an answer, and the answer is a coordinate projection, which for a comparison
mask means "consult lane 0 and discard the rest".

At one lane the single projection is the identity, and it is unique. So the coincidence file 106 named
is not an accident of the design's current instantiations. **The exit is a homomorphism exactly at one
lane.** Above one lane, the exit either preserves the structure and is useless, or is useful and is
outside the structure. There is no third option, and that is checked rather than argued.

Two consequences follow directly, and both are load-bearing later.

**The exit is extra data, not a derived fact.** For a mask, `is_true` is not a function of the mask. It
is a function of the mask and a reduction. This is the layer-keying rule's own subject arriving from
its unwatched side: the rule as stated at `68:116` catches a fact keyed too finely, and calls that a
false statement. The dual failure is a fact keyed on something that does not determine it, which is not
a false statement but a non-function presented as one. A named reduction is exactly the supply of the
missing argument, and a default is exactly its silent supply.

**A wrong reduction returns a wrong answer, not a slow one.** That is D16's own register, arrived at
from a different direction. D16 (`202607290200:38-42`) puts asserted properties behind `unsafe impl`
because "whose falsehood selects a different algorithm and returns a **wrong answer** rather than a slow
one" belongs at the declaration where a reviewer sees it. `p5b` puts numbers on the same shape for the
exit: two candidate reductions, two different wrong answers, from safe code, with no diagnostic at any
level.

## 2. The third clause is backwards

This is the finding, and it is mechanical.

The shape as the checkpoint states it (`106b:129-132`) has four clauses: a Boolean-algebra core, a
separately declared exit, `Bool` satisfying the exit by identity, a mask satisfying it only through a
named reduction, and then "the fifteen declarations bind on the **exit-carrying** part rather than on
the algebra".

Take the design of `p1`, which is the first four clauses, and change the fifth and only the fifth. The
producer declaration goes from `type Truth: TruthAlgebra` to `type Truth: Branch`. Everything else is
character-identical. The result:

```
error[E0277]: the trait bound `Mask2: Branch` is not satisfied
  --> p2_bound_on_exit.rs:57:18
   |
57 |     type Truth = Mask2;
   |                  ^^^^^ unsatisfied trait bound
...
note: required by a bound in `Compare::Truth`
```

The multi-lane instance is refused at the impl. And the multi-lane instance is the whole content of
branch B: file 105's theorem exists to establish that finite products of truth values are truth values
so that the contract *has* those instances, and the fifth clause then excludes every one of them from
every declaration in the design.

**Where the inference slipped, precisely.** File 105 says, at `105:314-317`, "the core `Truth`-shaped
trait carrying **no exit at all**, and `if`-usability living on a second, narrower trait that `Bool`
gets by a trivial blanket and a mask gets only by naming the reduction ... so a consumer writing `if
mask.all()` states the policy **at the call site**". That is a statement about where `if`-usability
lives and about what a *consumer* writes. It is `p1`, exactly.

File 106 reports it faithfully at `106:331-334` and then adds, at `106:350-353`: "contracts generic over
the algebra core do not thereby become usable in `if` position, **so** the fifteen declarations file 103
counted take the narrower branchable bound rather than the algebra one." The premise is true. The
conclusion does not follow from it, because the fifteen declarations are not in `if` position. Their
callers are. I read all fifteen, listed by
`grep -rn "fn [a-z_0-9]*(.*)\s*->\s*Bool;" --include="*.rs"` across `mock/crates`, which returns exactly
fifteen and confirms file 103's count: `bit`, three `get_bit`, four `is_zero`, two `test`,
`is_non_negative`, `is_positive`, `is_zero_or_positive`, `const_bit_eq`, `const_eq`. **Every one is a
producer. None branches on its own result.** So the narrower bound buys nothing at the declaration and
costs the design every multi-lane instance.

The checkpoint then carried file 106's version forward as the shape to lock, and the dispatch that
reached me carries it too. Three documents, one inference, made once.

**And the repair a reader reaches for is worse than the problem.** If the mask cannot satisfy the exit,
wrap it: let the producer name `All<Mask2>` as its truth, where `All<M>` and `Any<M>` carry the same
algebra and differ only in the exit, which is a rather pretty way to make the extra datum structural.
That compiles (`p2b`). What it does is relocate the choice from the trait to the impl. There is exactly
one associated type per trait and Self, so the comparison decides once, for every caller, forever; the
call site reads `a.eq(b).is_true()` and silently means all-lanes; and a caller who wants any-lane has no
route whatsoever. A default on a trait is at least one visible place. An impl-site choice is per-type,
invisible from the consumer's bound, and there is no place a reviewer would think to look.

So the fifth clause is not a detail of the shape. It inverts the finding the first four clauses
descend from.

## 3. Never a default: five routes, two refusable

File 92's method was to enumerate introduction routes rather than attacks anyone had thought of, and it
is the right method here because "never a default" is a claim about what cannot arrive, which is a
universally quantified negative and therefore owes a search.

Five routes, each written out in `p4`, **all five compile clean**:

**R1, a default body on the exit trait.** The interesting form is not the obvious one. Every truth type
will want equality, since `const_eq` is one of the fifteen. Give the algebra a `PartialEq` bound and the
exit acquires a default that looks like structure rather than policy: `fn is_true(self) -> bool { self
== Self::TRUE }`. Then `impl Branch for Mask2 {}` supplies the exit with no method written, and it
silently means all-lanes.

**R2, a blanket impl** over the algebra, same body, same meaning.

**R3, an inherent method** named `is_true` on the mask while the trait bound stays unsatisfied. Inherent
methods win resolution, so concrete call sites read exactly as if the trait were implemented. The
compiler is not fooled and generic code still refuses; only the reader is fooled.

**R4, `Deref`.** This one is live rather than hypothetical, because D15 ratified `Deref` as the vehicle
for predicate call syntax. A truth type dereferencing to `bool` supplies the exit with no impl of
anything, and `p4`'s version silently means lane 0.

**R5, `From`/`Into`.** `impl From<Mask2> for bool` is legal because the mask is the local type, and
`m.into()` at a `bool` position is an exit. `p4`'s version silently means any-lane.

Note that R1 and R5 disagree with each other about what the default is, and both read as the obvious
thing to write.

**What the type system can refuse.** Declare the absence rather than relying on nobody writing the
impl:

```rust
impl !Branch for Mask2 {}
```

That compiles under `negative_impls` alone (`p4b`), and it converts R1 and R2 from silent successes into
`E0751`, "found both positive and negative implementation", naming both sites (`p4e`, `p4d`).
Importantly it does **not** need `with_negative_coherence`, which `unstable-features.md` lists as
forbidden; I checked by deleting the gate and re-running, and the error is unchanged. `negative_impls`
is WATCH-tier allowed there, with the open impl gap #133556 recorded, and it currently appears in no
shipping crate root, checked by `grep -rn "negative_impls" --include="lib.rs"` across `mock/crates`,
which returns nothing. So adopting it is a real cost: one new WATCH-tier gate in a contracts crate, to
close two of five routes.

**R3, R4 and R5 are not refusable**, because none of them mentions the exit trait. They are rules with a
grep, and the grep is cheap and specific: no `Deref` and no `From<_> for bool` on a truth type, and no
inherent method sharing the exit's name.

So the honest statement, and it is the one I would put in the chapter: **"never a default" is a bound
for two routes and a rule for three.** Anyone writing it as though the type system carries the whole
weight will be wrong in three places. This is the same distinction the design has spent the review
learning, and the value of saying it exactly is that the three rule-shaped routes are now named and can
be grepped rather than being discovered later by a wrong answer.

**One alternative that makes the whole thing a bound, priced rather than recommended.** Index the algebra
by its lane count and blanket the exit at one lane:

```rust
impl<T: TruthAlgebra<1>> Branch for T {
    fn is_true(self) -> bool { self.lane(0) }
}
```

This compiles (`p6`), and the mask cannot reach the exit: `E0599`, "doesn't satisfy `Mask2: Branch` or
`Mask2: TruthAlgebra<1>`", which is a diagnostic that says *why* rather than only *that*. A one-lane type
cannot override its derived exit either, `E0119` (`p6c`), which is correct: at one lane the exit is the
unique homomorphism and there is nothing to choose. Cost is unchanged, still one symbol (`p6d`). And the
whole shape survives the design's own idiom, `pub const trait` with `const impl` and `[const]` bounds,
with the exit reached in const position and `const _: () = assert!(...)` holding at compile time (`p7`).

Under this shape the exit at one lane is **derived** in D16's exact sense, computed by the type from its
own parameter, so it cannot lie and needs no contract. The price is that the lane count is threaded
through the fifteen declarations. That is a real cost against "one type parameter and one bound", and it
is why I price this rather than recommend it. A middle route, a marker trait meaning one lane, is
strictly worse under D16, because arity would then be an *asserted* property and would owe an `unsafe
impl`, whereas the const index makes it derived. If op wants the shorter declarations, the marker with
`unsafe impl` is the honest spelling of it; if he wants the shorter bound story, the index is.

## 4. What the exit costs where it is satisfied by identity

The common case is the whole surface today, so a split that taxes it to serve a case with no consumer
would be the wrong trade. It does not.

Three spellings of one branch, compiled at `-O` on the pin: the raw primitive comparing two pairs of
`u32` and returning one of two constants; the same through a concrete truth newtype with the exit by
identity; the same generic over the truth type with the exit reached through a bound. The assembler
output:

```
_a_raw:                    (6 instructions: cmp, ccmp, mov, mov, csel, ret)
_b_concrete = _a_raw
_c_generic_at_scalar = _a_raw
```

Three spellings, one symbol (`p3`). This reproduces file 103's `_run_b1 = _run_a` independently and
extends it: the class contains the raw primitive too, so the exit-by-identity is free against the
language's own floor rather than merely against the other branch. Symbol identity is stronger than a
measurement, and I have nothing to add to file 103's framing of why.

**The stronger form of the same point, which no file has stated.** The exit is not a new door. Rust's
`if` takes `bool` and cannot be overloaded, which file 103 establishes at its section 3.5. Therefore any
truth type that is not `bool` must already declare a projection to it, and already pays one at every
branch, in either branch of the fork. The proposal adds nothing to the common case; it names something
already there and states who may have it.

The design's own tree is the witness rather than the ground. `arvo-storage/src/platform.rs:323-333`
declares `pub const trait AsBool { fn as_bool(&self) -> bool; }` with a single implementor, `Bool`. That
is the exit, already declared, already restricted to the one-lane type, already `#[inline(always)]`.
Delete that citation and the paragraph above stands unchanged on the language's own rule about `if`,
which is the test this dispatch sets. So `AsBool` demotes exactly as `MaskOps` did at `106:267`: a
witness that the design reached for the thing once, at the right weight for a fact about the tree.

One cost that is real and is an improvement. `ConstPartialEq::const_ne` at
`arvo-storage/src/bridges.rs:44-48` has a default body that reads `Bool(!eq.0)`, reaching through the
public field to negate. Under a truth contract that body cannot be written that way and becomes
`eq.not()` in the algebra. That is one body rewritten, and the rewrite stops a contract's own default
from reaching through a representation, which section 2.4 of file 106 and section 3.2's redundancy
argument both care about for separate reasons.

## 5. Where the exit belongs, and the two things called select

The dispatch's fourth question is whether the exit belongs to the truth contract at all or to the
operation that branches. It belongs to the operation, and working out which operation produces the third
answer this review's record predicts.

**Start from what a consumer that appears to branch actually wants.** Take `max`. At one lane it is
`if a < b { b } else { a }`. At two lanes, the correct answer is lane-wise, and lane-wise is not a
branch at all. It is a blend. So the operation that generalises across lane counts is not "reduce the
truth to a bool and branch", it is "select, lane-wise, between two data".

Written once against a selector, with no exit anywhere:

```rust
pub fn max<T>(a: T, b: T) -> T
where T: Compare + Select<Truth = <T as Compare>::Truth> {
    T::select(a.lt(b), b, a)
}
```

That compiles at one lane and at two, and at one lane `_max_scalar = _max_raw` (`p5`). No exit appears
in the bound, in the body, or in the emitted code.

**And the exit route is not merely unavailable above one lane, it is wrong.** Executed, at `a = [7,2]`
and `b = [3,9]`: lane-wise gives `[7,9]`; reducing with `all` then branching gives `[7,2]`; reducing
with `any` then branching gives `[3,9]` (`p5b`). Two candidate reductions, two different wrong answers,
neither of them the max of anything.

**Now the keying.** `select` is a fact about the pair, the truth and the datum: it needs to know how many
lanes the datum has and how to blend them, which is the datum's structure and not the truth's. So under
the layer-keying rule it lives on the datum, parameterised by the datum's own truth type. The exit is a
fact about the truth alone, and is only a function of it at one lane. That is the three-way the design
wants:

The **algebra** lives on the truth type, is product-closed, and is what the fifteen declarations bind on.
The **selector** lives on the datum, is keyed on the pair, exists at every lane count, and requires no
choice. The **exit** lives on the truth types that have one, which is not all of them, and is required
only by sites that perform genuine control flow, which above one lane means sites that are scalar
anyway.

**The thing that made me check twice, and it is a correction to file 103.** File 103 already found a
selector, at its section 3.5, and its probe `p1_foundation.rs:39-42` declares it:

```rust
fn select<R, T: FnOnce() -> R, F: FnOnce() -> R>(self, on_true: T, on_false: F) -> R;
```

That is a selector on the **truth type**, thunked, generic in the result. It is not the same object as
the one above, and the difference decides where it can live. `p8` compiles both directions of the
translation:

```rust
pub fn exit_from_select<S: TruthSelect>(s: S) -> bool { s.select(|| true, || false) }
pub fn select_from_exit<E: Exit, R>(e: E, t: impl FnOnce()->R, f: impl FnOnce()->R) -> R {
    if e.is_true() { t() } else { f() }
}
```

Both are total, for any type at all. **The thunked selector and the exit are the same object.** A truth
contract carrying that selector carries an exit whether it says so or not, and inherits the reduction
problem in full, while looking like structure rather than policy, which is strictly worse than an exit
that admits what it is. `p8` also shows what happens when you implement it for a mask: there is no
correct body, one of the reductions gets chosen, and the choice is invisible from every call site.

I want to be fair to file 103 here, because its own framing survives. It offered the selector as the
spelling that avoids naming `bool` in the foundation and then said plainly that "the selector is
available on its merits ... rather than as the thing that rescues the branch", and that branch B "does
not avoid `bool`; it avoids `arvo-platform`" (`103:3.5`). Both correct. What changes is the selector's
status and its address: it is not a way to avoid `bool`, since the thunked form is the exit; the
blending form on the datum is the only correct consumer shape above one lane, which is a stronger status
than "available on its merits", and it belongs on the datum rather than on the truth.

Two things sharing a name, one of which is the exit renamed and one of which is the answer, is exactly
the condition the widened definitional-completeness line exists to catch. If the chapter uses the word
`select` it has to say which, and I would give them different names.

## 6. What the design should say, in a form the next consolidation could take

Offered as text, in the register the consolidation uses. Op's call on all of it; this is a suggestion
that has been compiled rather than a ruling.

**The truth contract is a Boolean algebra.** It declares `TRUE`, `FALSE`, `and`, `or`, `not`, and
nothing else. Every truth type in the design satisfies it, and finite products of truth types satisfy it
by the closure of a variety under direct products, so a lane mask is an instance of the contract rather
than a container of instances. The contract carries no route to the language's `bool`.

**The exit is declared separately, and it is partial over truth types.** An exit is the route from a
truth value to Rust's `if`, which takes `bool` and cannot be overloaded. A one-lane truth type has
exactly one exit and it is the identity, because the structure-preserving maps out of an n-lane truth
algebra are exactly the n coordinate projections and at one lane there is one of them. A truth type of
more than one lane has no exit. Its reductions, all-lanes and any-lane and the rest, are inherent
operations named by the consumer at the call site, never a trait impl, never a default, because they are
not structure-preserving maps and the foundation cannot know which one a call site means. The absence is
declared rather than left to discipline, and a later blanket or default is then a coherence error rather
than a silent success.

**The declarations that return a truth value bind on the algebra, not on the exit.** All fifteen are
producers; none of them branches on its own result. Binding them on the exit would refuse every
multi-lane instance at the impl, which is the instance the contract's shape exists to admit. A consumer
that performs control flow adds the exit to its own bound, at its own site, where the fact that it is
branching is visible.

**A consumer that appears to branch on a truth value usually wants a selector, and the selector is keyed
on the pair.** It takes a truth value and two data of one type and returns that type, lane-wise, with
both arms evaluated. It exists at every lane count, requires no choice, and lowers to a conditional
select rather than a branch. It lives on the datum, parameterised by the datum's own truth type, because
how to blend is the datum's structure. A selector on the *truth* type taking thunks is not this: it is
interdefinable with the exit and carries the same partiality, and the two must not share a name.

**Consequences worth stating in the same place.** `Bool`'s route to its primitive is this contract's
declared exit, which settles as derived what `103` handed over as taste, and the four redundant
spellings of it are four spellings of one door. The number of routes is one, per D27's own "named once".

## 7. What this file does not decide, and what it owes

The fork itself is op's, and my read is a read. What I claim is that the shape as `106b` states it
should not lock in that form, that the fifth clause is the one to change, and that the change is one
word in the bound.

Owed artifacts, each named with what closes it:

The **Boolean-algebra law suite**, which is the gate finding in the opening section and the fork's own
unchecked premise. Artifact: the five axioms plus De Morgan, absorption, double complement and
idempotence, asserted over **every** truth type the design ships and at **every** width, not a sample of
them, since a law checked at a chosen subset of widths is a choice about what not to find out. It lives
under `mock/crates`, so it is op's own commit.

The **selector's exact shape at the design's real widths**, which I modelled at two lanes. Artifact: one
compile against a real `Mask64` and a real bitpacked column, checking that the blend is expressible
without the datum reaching into the truth's representation.

Whether the **arity-indexed alternative** in section 3 is worth its cost. Artifact: op's pick, with the
D16 derivation-versus-assertion reading as the reason to prefer it and the fifteen threaded declarations
as the reason not to.

The **`negative_impls` adoption**, if the declared absence is wanted. Artifact: op's word, since it adds
a WATCH-tier gate to a contracts crate and `unstable-features.md` puts that call with him. The fallback
is a compile-fail pin, which the design already uses for the domain-preservation equation.

A **second read on section 5**, which is one pass and is the part I would attack first if I were the next
file. Specifically: whether a selector keyed on the pair can be declared without the datum's crate
depending on the truth's, which is the same layering question the fork itself is about, one level over.

## 8. The three requirements, performed on this text before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions. *Exit*:
taken from file 103's definition at its section 3.5, the route from a truth value to Rust's `if`, used
here unchanged. *Reduction*: defined at first use in section 1 as a map from a multi-lane truth value to
a one-lane one that is not a coordinate projection. *Blending selector*: defined at first use in section
5 as a map taking a truth value and two data of one type to that type, lane-wise, both arms evaluated;
distinguished there from *thunked selector*, defined as file 103's shape and shown interdefinable with
the exit. *Producer* and *consumer* of a truth value: defined at first use in section 2 by whether the
declaration branches on its own result. *Introduction route*: defined at first use in section 3 as a way
an exit can arrive at a type without anyone naming a reduction. Terms used from the record without
redefinition: the layer-keying rule, the pricing pillar, the toolbox rule, the separation requirement,
the definitional-completeness line, symbol identity, D15, D16, D17, D27, branch A and branch B. Named
open rather than defined: the truth contract's **name**, which D17 leaves unsettled and which I do not
touch; and the spelling of the reductions, which is op's.

**The separation requirement, performed.** The model this file relies on is the split between the
algebra and the exit. The axis it separates is lane count. The two instantiations at which it separates
are one lane, where the exit is the unique structure-preserving map and coincides with the identity, and
two lanes, where `p9` shows the structure-preserving maps are the two coordinate projections and neither
`all` nor `any` is among them, and where `p5b` shows the two candidate reductions produce two different
answers on one reachable input. The vacuous region is one lane, and it is where every returning site in
the design sits today, which is why the check had to be run at two.

**The freshly-performed-search requirement, performed.** Every universally quantified negative in this
file carries a search run this session, 2026-08-05, at HEAD `eae402c`. "No Boolean-algebra law is
asserted anywhere in the tree": `grep -rln
"de_morgan\|De Morgan\|distribut\|associat\|complement\|idempot\|absorpt" --include="*.rs"` over
`mock/crates`, ten hits, two in test files, both by the word `complement` in a per-operation test name,
bodies read. "Exactly fifteen declarations return a truth value": `grep -rn "fn [a-z_0-9]*(.*)\s*->\s*
Bool;" --include="*.rs"` over `mock/crates`, fifteen results, all listed in section 2 and all read.
"`negative_impls` appears in no shipping crate root": `grep -rn "negative_impls" --include="lib.rs"`
over `mock/crates`, empty. "None of the fifteen branches on its own result": read individually, not
grepped. "`AsBool` has one implementor": `grep -rn -A12 "trait AsBool" --include="*.rs"`, one `const
impl`, for `Bool`. "All five default routes compile": `p4`, which is a compile rather than a search, and
the enumeration is a claim about completeness that I do not make; five is what I found by enumerating
language mechanisms that produce a value at a `bool` position, and a sixth would not surprise me.

**The toolchain check**, which two files have now been bitten by. Every probe above ran from inside the
tree on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`. `rustc --version` from `/tmp` on this machine
returns `rustc 1.94.0 (4a4ef493e 2026-03-02)`, stable, verified this session. `p7`, the const-form
probe, would fail outright there and I would have reported that the split does not survive the design's
own idiom.

## 9. Standing

Only op's calls are final, and even those go stale. Everything above is offered as evidence and
suggestion. The compiled facts are compiled and are reproducible from `108_probes/` with the commands in
its `OUTCOMES.md`; the readings are readings, and section 5 is one pass and knows it.

The one thing I would ask not be absorbed as agreement: sections 1, 2 and 5 were written and their
probes run before I opened files 105 and 106, which is why I can say the convergence on the split is
corroboration rather than confirmation. Section 2's finding is a disagreement with file 106 and with the
checkpoint, and section 5's is a correction to file 103's status for the selector. Neither is a
disagreement with file 105, whose text says what I say and was compressed into something else two
documents later.

The panel produces canon, not source. `mock/research/` and `mock/benches/` are its ground and
`mock/crates` is op's own boundary, which is why the gate finding in the opening section is reported
rather than fixed.
