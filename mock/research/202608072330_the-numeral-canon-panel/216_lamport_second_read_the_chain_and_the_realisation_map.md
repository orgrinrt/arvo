# 216. Second read: `the_chain` and `the_realisation_map`

Seat 216. Fourteen rows standing at `standing = "one_expert"` across two topics, each owed an
independent second reading. The order is fixed: derive blind, commit that, then read the rows.

This file is written in two parts and the parts are committed separately, because the commit
boundary is the only evidence that the first part was written without the second one's inputs.

---

## Part zero: the gates, and what I read to get here

**The canon gate passed.** I checked the assigned work against `mock/registry/ruling.toml`'s
twenty-three `rung = "ratified"` rows and against `INTENTS.md`. Giving a second reading to rows
standing at one expert is exactly what `two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`
describes, and `an_expert_asks_its_peers_before_it_asks_op` says the convergence is where these
settle. Nothing in the assignment asks me to do something the canon forbids.

One thing about the state the work builds on, which is not a refusal but is reported here rather
than in a footnote: `mock/registry/proposal.toml:29` says "`probe.toml` does not exist yet, so
there is no row for a `measured` proposal to point at" and that "the check for that is red on
purpose". `mock/registry/probe.toml` now holds 104 `[[probe]]` rows, all thirty `measured`
proposals carry `evidence`, and the suite is green. The paragraph describes a state that has
passed. Detail in the closing section.

**The test gate passed, and I ran it before anything else.**

```
cargo test --workspace     ->  120 passed; 0 failed; 0 ignored
```

Nineteen test binaries under `mock/checks/tests/`. I read the bodies rather than the names in the
surface I touch, which is the predicate and standing machinery: `every_predicate_names_a_declared_axis.rs`
in full, `what_one_field_obliges_another_to_carry.rs` and `what_reaches_each_obligation.rs` by
their assertions. They are not decorative. Every arm has a planted control that makes it fail:
`an_axis_nothing_declares_is_reported` plants `phase_of_the_moon: waxing` and asserts the
diagnostic kind, `a_laws_failing_region_is_read_as_well_as_its_holding_one` plants an undeclared
axis in `fails` specifically to catch a walk wired to `holds` alone, and
`a_keywords_list_is_not_read_as_a_predicate` pins the negative. A scan for `assert!(true)`, a value
asserted against itself, and test bodies with no assertion at all returned nothing across all
nineteen files. This is a suite that can fail, and the green is worth something.

**What I read before deriving.** The twenty-three ratified rulings in full; `INTENTS.md` entries
I7, I8, I10, I11, I13, I14, I15, I16, I18; `mock/registry/topic.toml` entire;
`mock/registry/dimension.toml`'s axis list; `mockspace.toml`'s `canon_paths`.

**What I did not read, and this is the load-bearing part.** For each of the fourteen rows I read
the `id` and the `topic` line and nothing else. Not `says`, not `because`, not `note`, not
`provenance`, not `predicate`, not `sentence_kind`, not `keywords`. I did not open
`mock/registry/question.toml`, `mock/registry/law.toml`, `mock/registry/probe.toml`,
`AGREEMENTS.md`, `OPTIONS.md`, `DROPLIST.md`, or any panel file in the ranges the two topics were
argued in (166 to 178 for the chain, 114 to 124 for the realisation map). I located the fourteen
rows by `grep -n '^id = "..."$'` and read the line numbers only.

**One contamination to declare, because a coverage statement is worth more than an assertion of
blindness.** `mock/registry/topic.toml` gives each topic a `what` and a `keywords` list, and I read
both. For `the_chain` those keywords include `fold`, `reduction`, `intermediate`, `unobserved
region`, `stretch`, `schedule`, `adaptation point`. That is more vocabulary than the row ids alone
would have given me, and in particular it told me that `schedule` and `adaptation point` are
established terms rather than words the row happened to choose. My derivation of row 1 is
correspondingly less independent than the others. I could not have avoided this without declining
to read the topic registry, which the brief requires.

**The single premise everything below rests on**, and it is ratified rather than mine:
`the_format_spine_is_canon`. A format is a pair, an ambient domain and a representable set, the set
being a constant of the type; and arithmetic on a format is an exact operation in the ambient
domain composed with a named total adaptation onto that set. I write it as

```
x (+)_F y   =   adapt_F ( x + y  in A )
```

and everything I derive about chains is derived from iterating that.

---

# Part one: the derivations, written blind

Fourteen sections. Each names the question I take the id to pose, gives my own answer, and states
what I would have to build to check it. Where an id does not fix a question precisely enough to
derive against, I say so instead of guessing, and I say what the ambiguity is, because the
ambiguity is then itself a finding about the row whatever the row turns out to say.

## C1. What a chain is

`a_chain_is_exact_operations_together_with_a_schedule_of_adaptation_points`

**The question.** What object is a chain, given that a single operation has already been factored
into an exact part and an adaptation?

**My answer.** Iterate the ratified factoring. A composition of n format operations is

```
adapt( adapt( adapt(x1 * x2) * x3 ) * x4 ) ...
```

and the immediate observation is that the two halves of the factoring do not compose the same way.
The exact operations are what the consumer wrote: a sequence, or in general a DAG, of ambient
operations over ambient values. That part is fixed by the program text. The adaptations are not
fixed by the program text at all. An implementation may apply one after every ambient operation, or
one at the end, or at some subset of positions in between, and each choice is a different
computation with a different value and a different error, all of them legitimate realisations of
the same written expression.

So a chain has two components and they have different status. The exact-operation part is given.
The adaptation part is a **choice**, and the object naming that choice is a map from positions in
the exact structure to adaptations. I would call a chain the pair.

**Why the second component has to be first-class rather than an implementation detail.** Because
I7 is unstateable otherwise. "Precise is accurate across chains, not only per operation" is a
claim that two strategies producing identical per-operation behaviour can still differ, and the
only thing left for them to differ in is where the adaptations sit. A model in which a chain is
just a sequence of format operations has already fixed the schedule to "eager, everywhere", and in
that model Precise and Hot have nothing to choose between, so I7 names an intent the model cannot
express. The schedule is exactly the degree of freedom the strategy axis needs at chain scope.

**Where I think the id is underdetermined, and I say so rather than agreeing with it.** "A schedule
of adaptation points" fixes *where* and is silent on *what*. Two schedules that adapt at the same
positions but to different intermediate formats, or under different rounding, are different
computations. Unless the row says an adaptation point carries the whole adaptation (target
representable set, rounding, overflow policy) and not merely its position, two readers satisfy the
definition differently, and one of them builds a chain model that cannot distinguish a round-to-
nearest intermediate from a truncating one. The ratified spine helps here and should be leant on:
it says the adaptation is a first-class object with its own laws, so a *point* in the schedule
should be an occurrence of that object, not a mark on a timeline.

Second underdetermination: "schedule" reads as linear, and a chain in general is a DAG. For
`(a*b) + (c*d)` there is no total order of adaptation points, only a labelling of the DAG. If the
canon means a linear order, it has quietly restricted chains to linear ones; if it means a
labelling, the word "schedule" is doing damage.

**Sort of claim.** Definitional. Under the registry's own convention a definition carries no region
and is `normative`. But it has one testable consequence I can and should check rather than assume:
that varying the schedule while holding the exact-operation sequence fixed varies the value. If it
did not, the second component would be a distinction without a difference and the definition would
be carrying weight it has not earned.

**What I will build.** A probe that fixes one exact-operation sequence and runs it under several
schedules, showing the values differ, and separately showing a case (a coherent operation) where
they do not. The second half is what makes the first half mean something.

## C2. Whether a coherent reduction needs an accumulator

`a_coherent_reduction_needs_no_accumulator`

**The question.** Under what condition does a reduction need no accumulator wider than its element
type, and is "coherent" that condition?

**My answer.** Yes, on one definition of coherent and not on the other, and which one is meant is
the whole content.

Define: the format operation is **coherent** when the exact ambient operation is closed on the
representable set, so that `adapt` is the identity on every result it is ever applied to. Then the
theorem is immediate and is stronger than "needs no accumulator":

> If the ambient operation is closed on R, every accumulator width produces the identical result.
> So the narrowest accumulator, the element type itself, is not merely sufficient but loses
> nothing.

Proof sketch, and it is the reason I trust the statement: with `adapt = id` the format fold is
literally the ambient fold, and folding in a wider format then adapting once at the end is the
ambient fold followed by an adaptation that is again the identity. The two agree pointwise, for
every input, at every length.

The canonical instance is wrapping arithmetic, and it is worth naming because it is the case people
get wrong. With ambient `Z/2^w` and wrapping adaptation, wrapping addition **is** coherent: the sum
of two residues is a residue. Nothing is lost, no accumulator helps, and a wider accumulator
reduced at the end gives the same answer because `Z -> Z/2^w` factors through `Z/2^{2w}`. Min and
max are coherent for the same reason at any width.

Now the other definition. If **coherent** is taken as merely "the operation is a total map
`R x R -> R`", then saturating addition qualifies, and the claim becomes false in the sense that
matters. Saturating addition needs no wider accumulator to *run*, but a wider accumulator
**changes the answer**, and changes it toward the more accurate one. Signed, four-bit, saturating:
`(5 (+) 5) (+) (-5) = 7 (+) (-5) = 2`, while accumulating in eight bits and clamping once gives
`5`. Saying it "needs no accumulator" would be reporting that you can compute *an* answer without
one, which is true of almost everything and is not what a canon sentence should be spending itself
on.

So my derivation is: the claim is right, the condition is closure of the *exact ambient* operation
on R rather than totality of the format operation, and the row is worth having only if it says
which. And the statement I would rather see is the stronger one, because "needs no accumulator" is
an operational sentence and "every accumulator gives the same answer" is a theorem.

**One qualification I want on the record.** The theorem is about semantics and is silent about
realisation. A coherent operation over a narrow bit-packed field may still be *faster* with a wide
scratch accumulator masked once at the end, because the per-step mask is what the loop is paying
for. So "needs no accumulator" must not be read as "an accumulator is pointless"; it means an
accumulator cannot change the value. Those are different sentences and the realisation-map topic
cares about the second one.

**What I will build.** Exhaustive over all triples and quadruples at small widths: wrapping add
folded in the element width against folded wide then reduced, asserting equality at every length;
and the same shape for saturating, asserting inequality is reachable. The second is the case that
must fail, and without it the first establishes nothing.

## C3. What a fold needs

`a_fold_needs_a_closed_operation_and_a_separately_determined_accumulator`

**The question.** What are the components of a fold, and how does this not contradict C2?

**My answer, and the first thing to say is that C2 and C3 look contradictory and are not, on one
reading only.** C2 says a coherent *reduction* needs no accumulator. C3 says a *fold* needs a
separately determined accumulator. Those are compatible exactly when `reduction` and `fold` are
distinct technical terms in the standard functional sense:

- a **reduction** has type `A x A -> A`, the accumulator is the element type, and closure on A is
  what makes it well-formed;
- a **fold** has type `B x A -> B` with `B` an independent parameter, and the seed is given.

Under that reading C2 is the degenerate case `B = A` of C3, and the pair is coherent and
complementary. Under any reading that treats the words as synonyms, the two rows contradict each
other flatly, and a reader who takes them as synonyms will conclude the canon is inconsistent.

**So my first finding on this row is not about its content.** Two adjacent rows in one topic whose
mutual consistency depends on a vocabulary distinction neither of them states is a defect in the
pair, independent of whether either is true. Somewhere in the topic, `fold` and `reduction` have to
be stipulated apart. If they are not, this is the strongest thing I have to say about C3.

**On the content itself.** "Needs a closed operation" needs the same care C2 needed. If the fold's
step is `adapt_B` composed with an ambient operation, it is closed in B by construction, because
`adapt_B` is total onto `R_B`. So closure read as totality is vacuous here and cannot be what is
meant. Read as *exactness* in B, meaning the ambient result of every step lands in `R_B` so
`adapt_B` never fires, it is a real and demanding requirement, and it is the one that makes the
fold worth doing: it is precisely the condition under which the fold is exact and the only
adaptation in the whole chain is the final one from `R_B` to `R_A`.

"Separately determined" I read as: `B` is not a function of `A`. It cannot be, because the width
that keeps the step exact depends on how many steps there are and how large the operands may get,
and neither of those is recoverable from the element format. That is right, and it is the reason
the accumulator is a parameter of the chain rather than a property of the numeral. It also means
`B` is determined by something, which is C6's subject, so C3 and C6 are two halves and should be
read together.

**What I will build.** Nothing new; C2's probe covers the semantics and C6's covers the widths. What
I will do is check whether the topic stipulates `fold` against `reduction` anywhere, since that is
the finding.

## C4. What a fold's compile-time refusal means

`a_folds_compile_time_refusal_is_the_staging_boundary_reporting_its_own_position`

**The question.** When a fold will not compile, what is the compiler telling you?

**My answer, in two parts, because the second part is a refusal of the first as stated.**

The true and valuable part. Under I14 and I15 the accumulator width is a const, so a fold whose
accumulator width is a function of the element count `n` is well-formed only if `n` is on the
compile-time side of the staging boundary. Where it is not, there is no const to instantiate the
accumulator with, and the fold cannot be monomorphised. The refusal is then genuinely informative:
it is not a limitation of the library and not a gap to be worked around with a runtime path, which
I15 forbids outright. It is the type system reporting that a fact the computation needs sits on the
wrong side of the boundary. That is a good sentence and I would keep it.

The part I do not accept as written. The id quantifies over every compile-time refusal of a fold,
and that universal is false, because a fold can be refused for reasons that have nothing to do with
staging: an unsatisfied trait bound, an ambiguous impl, an arithmetic overflow inside a const
expression, an exceeded recursion limit. Each of those is a compile-time refusal of a fold and none
of them is the staging boundary reporting anything. A universal is falsified by one counterexample
and I expect to be able to compile three.

So my reading is **agree with a narrowed region**: a fold refused *for want of a const it cannot
obtain* is the staging boundary reporting its position. The narrowing is not pedantry. The
unqualified sentence, applied by a later reader to a genuine trait-bound error, says the design is
correct and the consumer's expectation is wrong, when in fact something is missing an impl.

**What I will build.** Three refusals of a fold, compiled, with their diagnostics captured: one
from a non-const length, one from an unsatisfied bound, one from const-eval overflow. Only the
first is the staging boundary. If all three produce the same class of diagnostic I am wrong and I
will say so.

## C5. Whether a multiplicative chain can avoid an ever-growing intermediate

`a_multiplicative_chain_is_writable_without_an_ever_growing_intermediate_by_windowing`

**The question.** A chain of exact multiplications grows without bound. Can it be written in const
storage, and is windowing how?

**My answer.** The growth is real: an exact product of two width-`w` values needs `2w`, and a chain
of `n` needs `n*w`, which under I14 ("sizes are const, no runtime growth") is not writable for
unbounded `n` at all. So something must give, and windowing is the right name for what gives.

Windowing means keeping a fixed-width window of the exact product together with the window's
position, discarding what falls outside. Two forms, and the distinction matters more than the word
does:

- **Static window.** In fixed point the window position is a compile-time constant: multiply into
  `2w`, shift right by `F`, keep `w`. That is ordinary fixed-point multiplication, and it is
  windowing with a position that costs nothing because it is const. Available whenever the operand
  magnitudes are known to stay in the format's range.
- **Dynamic window.** Keep the window position as a value that moves, which is a running exponent,
  which is block floating point. Available always, costs an exponent and a normalise per step.

Which one suffices is decided by whether the integer part can grow, which is decided by whether the
operands are bounded away from magnitude one. For operands in `[0, 1)`, a common and important case
(gains, probabilities, attenuations), the exact product's integer part is always zero and only
fraction bits are lost, so the static window is enough and the error is a pure truncation tail. For
operands that may exceed one, the integer part grows and a static window overflows, so the dynamic
one is forced. That is exactly the declared `operand_window` axis, and it is the axis the arms split
on.

**What must not be claimed.** Windowing makes the chain *writable*, not *exact*. Windowing at every
step and windowing once at the end give different answers, and the gap grows with the chain length.
So the sentence is about representability under a const size bound, and if it is read as saying an
unbounded multiplicative chain can be computed exactly in fixed storage, it is false, and provably
so by a counting argument: the exact products of `n` `w`-bit values take more than `2^k` distinct
values for any fixed `k` once `n` is large enough.

**What I will build.** A sweep computing, for a chain of multiplications, the exact product in a
wide type against a per-step statically windowed product, reporting where the windowed one is exact
and where it is not, and separately showing the exact intermediate exceeding a fixed container while
the windowed one does not. The failing case is the exactness claim; I want it to fail visibly.

## C6. What decides the accumulator

`an_accumulator_is_decided_by_the_operation_and_by_whether_a_bound_exists`

**The question.** What inputs determine the accumulator width?

**My answer.** The two named inputs decide the right thing, which is whether an *exact* accumulator
exists and how wide it is, and they do not decide what happens when it does not. So the row is
correct about one branch and silent about the other, and the silence is the finding.

The operation gives the growth law. For unsigned width `w` and `n` operands:

```
add          exact accumulator width  =  w + ceil(log2 n)      and this is tight
multiply     exact accumulator width  =  n * w                 and this is tight
min, max     exact accumulator width  =  w
```

Tight meaning achieved, by the all-maximum operand vector in each case, so the formula cannot be
improved without an operand bound. Whether a bound exists gives the second input: if `n` is a
compile-time constant, the formula has a value and that value is a const, so the exact accumulator
can be instantiated and the whole chain is exact with a single final adaptation. If `n` is
unbounded, no finite exact accumulator exists at all, and the question changes from arithmetic to
policy.

**Where the pair runs out.** On the unbounded branch, something still has to choose a width, and
the pair does not choose it. I8 and I13 say what does: the strategy, weighing the measurements it
weighs. So the honest statement has three terms, not two, and the third is only reached when the
first two fail to produce a value.

**A second gap, and I am less sure of it, so I state it as a question about wording rather than as
a refutation.** "The operation" has to mean the *format* operation, adaptation included, not the
ambient one. Otherwise the row misses that a wrapping addition needs no accumulator at all, by C2,
which is a fact about the adaptation rather than about `+`. If "operation" is the ambient one, the
row is incomplete on a case its own neighbour row settles. The ratified spine's factoring makes
this exactly the sort of thing that has to be said, since the whole point of the factoring is that
the ambient operation and the adaptation are separable and different things.

**A third input the pair may or may not subsume.** An operand window narrows the exact accumulator
below the formula, because `n` operands each below `2^k` sum below `2^{k + ceil(log2 n)}`. If "a
bound" means a bound on the accumulated magnitude then this is included; if it means a bound on `n`
then it is a separate input. The two readings give different accumulator widths for the same chain,
which makes this an exactness defect rather than a quibble.

**What I will build.** A sweep establishing the three growth laws and their tightness exhaustively
at small `w` and `n`, including the operand-window narrowing, and a case where the sum of two
`w`-bit values does not fit in `w` bits, which is the case that must fail.

## C7. How wide a clamped addition needs to be

`an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit`

**The question.** A saturating addition's accumulator: how wide?

**This is the row I am least confident I have read correctly, and I am going to write down both
readings before checking, because picking one and being right by luck is not a second reading.**

Reading A, pairwise. Two `w`-bit operands. The exact sum needs `w+1` bits. The claim then says a
clamped addition needs `w`, one less. And that is **true and is a real fact about saturation**:
saturation needs the overflow *detected*, not *represented*. Unsigned, in `w` bits:
`s = x.wrapping_add(y); if s < x { MAX } else { s }`, where the comparison recovers the carry
without ever materialising a `w+1`-bit value. Signed, in `w` bits: wrapping add, then the classic
sign test on `(x ^ s) & (y ^ s)`, which detects signed overflow from the sign bits alone. So the
clamped operation lives one bit below the exact one, and the exact one genuinely cannot: `w` bits
cannot hold `2^{w+1} - 2`.

Reading B, over a chain. `n` operands, exact sum width `w + ceil(log2 n)`, claim says
`w + ceil(log2 n) - 1`. I cannot derive this and I think it is false in both directions. A chain of
eagerly clamped additions needs no growth at all, `w` suffices at every length, which is far less
than the claim. And a chain whose *clamped-at-the-end* answer is wanted needs to distinguish sums
below `MAX` exactly from sums above it, which needs `w+1` bits plus a sticky flag rather than
`w + ceil(log2 n) - 1`. So under Reading B the number is too large for one target and the wrong
shape for the other.

I therefore expect the row to mean Reading A, and I expect to agree with it under that reading and
to want the sentence to say "pairwise" out loud, because Reading B is what a reader arrives at when
the row sits in a topic whose subject is chains and whose neighbouring rows are all about chain
accumulators. Sitting in `the_chain`, a sentence about a per-operation width is going to be read as
a chain width by default.

**The word "incoherent" is load-bearing and connects this to C2.** It is presumably C2's predicate
negated: the clamped addition is incoherent because the exact ambient sum is not always in `R`,
which is exactly why the width question arises at all. If so, C2 and C7 are a matched pair, coherent
needs nothing and incoherent-clamped needs one less than exact, and that is a tidy piece of canon.

**What I will build.** Exhaustive at every width from 1 to 8 bits, unsigned and signed: a saturating
add implemented with `w`-bit arithmetic only, against a reference computed in a wide type and
clamped, asserting agreement on every input pair at every width. And the case that must fail: exact
addition attempted in `w` bits, shown to disagree with the wide reference, which is what makes the
"one bit less" claim mean something rather than being true of nothing.

## C8. How chain laws divide

`chain_laws_split_by_whether_a_lifting_theorem_exists`

**The question.** Is there a structural division among laws about chains, and is it this one?

**My answer.** Yes, and I think this is the strongest row of the nine.

A law about a single format operation is a statement about `adapt(x * y)`. A law about a chain is a
statement about the iterated form. The question of whether the first implies the second is exactly
the question of whether the law survives being pushed through the adaptation, and there are two
regimes:

- **A lifting theorem exists.** When the adaptation is a homomorphism for the operation, every
  identity-shaped law in the ambient transports to the format at every chain length, by induction
  on the length. Wrapping is the instance: `Z -> Z/2^w` is a ring homomorphism, so associativity,
  commutativity and distributivity all hold at every length and the finding carries
  `chain_length: any` legitimately. The chain law is a corollary and no chain was ever examined.
- **No lifting theorem exists.** When the adaptation is not a homomorphism, and clamping and
  rounding are not, the per-operation law says nothing about chains, and the chain law has to be
  established directly, quantified over chains of each length. Then `chain_length` is a live
  dimension of the finding and a verdict at length three is not a verdict at length four.

The consequence I care about is notational and is why the split is worth being canon: it tells you
when `chain_length: any` is earned and when it is a widening. A finding that swept triples and wrote
`chain_length: any` has claimed a lifting theorem it did not prove. Under I13's own rules that is
the exact failure the predicate notation exists to prevent, and the split is what makes it visible.

**The ratified canon already contains one instance of each side**, which is part of why I believe
the row. `the_additive_and_absorption_verdicts_are_canon` says an additive verdict is independent of
the fraction width, which is a lifting-style statement, and in the same breath says that whether a
reduction's induced operation is associative is decided by absorption *quantified over the values
the format can hold*, which is the direct-establishment side.

**Where I would sharpen it.** "A lifting theorem exists" is an existential over proofs, which is not
checkable: you cannot establish that no lifting theorem exists, only that you have not found one.
The usable criterion is a sufficient condition, and the one that does the work is: the adaptation is
a homomorphism for the operation in question. That is decidable from the format and the operation
and can be a const predicate, which is what I13 wants. I would want the row to name the sufficient
condition, otherwise the split is real and nothing can be gated on it.

**What I will build.** Exhaustive associativity over chains of length three through six, at four
bits, for wrapping and for saturating: wrapping holding at every length, which is the lifting side,
saturating failing at length three already, which is the non-lifting side. And I want the *counts*
of failing tuples per length, since a law that fails on a vanishing fraction is a different object
from one that fails everywhere.

## C9. Whether a derivation reads the grid

`no_derivation_reads_the_grid_so_a_composition_may_hold_it_at_run_time`

**The question.** Do the derivations depend on the representable grid itself, and if not, may a
composition let the grid move at run time?

**This is the row I am most worried about, and I want to state the worry before I read it.**

First, "the grid". The registry uses the word in `dimension.toml` for the representable lattice: the
rounding axis is "how a result that is not representable is placed on the representable **grid**",
and `radix`'s keywords carry it too. `INTENTS.md` I15 also has a "binding-time grid", which is a
different object entirely. Given the topic is `the_chain` and the sentence is about run time, I take
the representable-set reading: no derivation depends on where the representable values actually sit,
only on the structural parameters, so the position may vary at run time.

If that is the reading, the row is the licence for C5's dynamic window: a running exponent is
precisely a grid held at run time, and C5 and C9 are one mechanism stated twice from two angles.
That is coherent and I like it.

**Now the three problems.**

**One, the universal is contradicted by ratified canon.** `the_additive_and_absorption_verdicts_are_canon`
says whether a reduction's induced operation is associative "is decided by absorption, **quantified
over the values the format can hold**". That derivation reads the grid. It does not read a width; it
reads which values are present, and absorption is a relation between actual magnitudes. So "no
derivation reads the grid" is false against a ratified sentence unless "derivation" is a narrower
term of art than it appears, and if it is, the row has to say so, because the plain reading of a
canon sentence is what a later reader uses.

**Two, the consequence collides with the ratified format identity.** The same ratified spine says
"the representable set is a constant of the type: a value set that depends on other data is not a
format but storage." An object whose grid moves at run time is therefore, by ratified canon, not a
format. If the row is read as licensing a runtime-varying grid *for a format*, it contradicts the
spine directly. If it is read as licensing it for a composition, which is not a format, there is no
contradiction and the row is fine, but only if it says so, because the thing a reader wants next is
a floating-point-shaped numeral and the row as titled looks like permission to build one.

**Three, the consequence collides with I13 and I15.** If the grid is a run-time value, then any
predicate that reads the grid is a run-time predicate. Absorption reads the grid. So the
associativity verdict for a composition holding its grid at run time is not a const, and an arm
gated on it is not const-selectable, which is what I15 forbids and what I13's const predicates
assume. The escape is that the verdict is taken conservatively over all grid positions, which is
sound and is a real answer, but it is a *different* claim from "no derivation reads the grid" and it
is weaker: it says the derivations can be made grid-independent by taking a worst case, at a cost in
sharpness.

So my derivation is that the row is pointing at something real, that its universal is too strong as
titled, and that it sits on a ratified boundary in a way that needs saying out loud. My prior is
**refuse as stated, agree with a much narrower and explicitly stated region**, and I expect the
narrowing to be roughly: the derivations of accumulator width and of adaptation-point placement read
only the structural parameters, and therefore a composition may hold the grid position at run time
provided the verdicts it relies on are the grid-independent ones.

**What I will build.** An exhibition of a derivation that reads the grid, namely absorption, showing
the verdict changing with the grid position while every width parameter is held fixed. If I can
produce that, the universal is refuted by construction and the argument is over.

## R1. What sort a carried fact takes

`a_carried_fact_takes_the_sort_its_consuming_site_uses_it_in`

**The question.** A fact established at one site and needed at another: in which syntactic category
should it travel?

**My answer.** The available sorts in this language, under I14's no-`dyn`, no-`TypeId`, everything-
monomorphised constraint, are: a type parameter, a const generic parameter, an associated type, an
associated const, a marker trait bound, and a runtime value. I15 removes the last one for anything a
decision is taken on. The claim is that the choice among the remainder should be made by the
consumer rather than the producer, and the reason is that the conversions between sorts are not all
available.

That is right, and the reason it is right is sharper than the claim: **the sorts are not
interchangeable and the asymmetry has a direction.** A fact held as a const generic parameter can be
read as a const value for free. A fact held as an associated const cannot be used as a const generic
argument without `generic_const_exprs`, which is unstable and which I14's spirit and the workspace's
pinned toolchain make a real constraint rather than a theoretical one. So the map from type-level to
const-level is total and free; the map back is not.

**Which is why I would state the rule the other way round.** "The sort its consuming site uses it
in" is underdetermined the moment a fact has two consumers using two sorts, and facts of this kind
usually do: a width is used as a const generic argument to size an array *and* as a const value in
an arithmetic comparison. Determined by the consumer, that fact has two answers. Determined by the
lattice, it has one: **carry it in the strongest sort any consumer needs**, from which the weaker
uses are free. For the width, that is the const generic parameter, and the associated const is a
projection of it rather than a rival.

So: the row's instinct is right and its rule is underdetermined in exactly the case that matters. I
expect to agree with a correction rather than refuse.

**What I will build.** A compile-fail demonstration, which is the strongest evidence available here.
The same fact carried as an associated const and consumed as a const generic argument, which I
expect rustc to refuse on the pinned nightly; the same fact carried as a const generic parameter and
consumed both ways, which I expect to compile. If the first compiles, my refinement is wrong and the
row's rule is sufficient, and I will say so.

## R2. What a const a generic body loops over costs

`a_fact_delivered_as_a_const_a_generic_body_loops_over_costs_the_reduction`

**The question.** If a fact arrives as a const that a generic body then iterates over, what is lost?

**My answer, with lower confidence than the others, and I will say why.**

I read "the reduction" as the compile-time reduction: the const folding and monomorphisation-time
collapse that I15 requires everything to reach ("const-time ifs that get erased via
monomorphisation and just const time solving and ultimately llvm"). The claim is then that
delivering a fact as data a loop walks, rather than as a parameter the instantiation reads, defeats
that collapse: the loop survives into the emitted code and the one-lowered-path guarantee is not
met.

Structurally this should be true at low optimisation and I expect it to be **substantially false at
`-O`**, because LLVM unrolls a loop with a known trip count over a known array and folds the result,
and after monomorphisation the trip count is known. If that is what happens, the row's region is
`build_profile` debug, which under I15's release-oriented reasoning is close to no region at all.
That would make this the weakest of the fourteen.

But there are shapes where it does not fold even at `-O`: a trip count above the unroll threshold, a
body with a data-dependent branch, an array large enough that materialising it is cheaper than
folding it. So I expect the honest answer to be an arm, not a verdict: it costs the reduction in a
nameable region, and the region is about trip count and body shape rather than about the sort the
fact arrived in.

**Two things I will not do here.** I will not report an instruction count as a measurement: that is
`mock/benches/` work on the harness with real competitor arms, and I do not have that in scope, so
anything I produce is an ad-hoc quick spike with no substance and I will call it that. What a spike
*can* settle is the qualitative question, whether a loop survives at all, and that is what I will
ask it.

**What I will build.** Two functions differing only in how the fact arrives, compiled to assembly at
`-O0` and `-O3`, inspected for a surviving backward branch. Qualitative only, named as a spike, and
if the answer is "both fold at `-O3`" that is the finding and it narrows the row hard.

## R3. Whether a law verdict survives a change of encoding and container

`a_law_verdict_is_invariant_under_change_of_encoding_and_container`

**The question.** Two of them, and they have different answers, which is my main point about this
row.

**Encoding: yes, and it is close to a corollary of ratified canon.** The spine says two's complement
and offset binary "denote the same sixteen values through different pattern maps and are one format
filed two ways", so encoding is realisation and not identity. A law is a statement about values. A
change of encoding is a bijection commuting with denotation. So the verdict cannot move, and the
proof is one line. I agree with this half without reservation.

**Container: this is where I stop.** `mock/registry/topic.toml` carries a topic called
`the_container_premise`, described as "Whether a declared numeral's behaviour is stated over its
declared width or over the container that carries it", and marked, in the registry's own words,
**"Blocking: no wording of several downstream clauses is true on both branches."**

A claim that a law verdict is invariant under change of container is one of those downstream
clauses, and it is not true on both branches:

- **Declared-width semantics.** The container is storage. Operations saturate, wrap and overflow at
  the declared width whatever carries the bits. The verdict is invariant, trivially, and the row is
  right.
- **Container semantics.** The container is where the arithmetic happens. Then a four-bit numeral in
  a `u8` saturates at 255 and the same numeral in a `u4`-shaped field saturates at 15, and those are
  different operations with different verdicts.

And I can exhibit the flip rather than argue it. Signed saturating addition, declared width four,
so `MAX = 7`: `(5 (+) 5) (+) (-5) = 7 (+) (-5) = 2` while `5 (+) (5 (+) (-5)) = 5 (+) 0 = 5`.
Associativity fails. The same three values in an `i8` container never reach 127, so no clamp fires,
and associativity holds on them. Same numeral, same law, two containers, two verdicts.

So my derivation is: the encoding half is right; the container half is true on one branch of a
premise the canon marks as blocking and open, and a row that asserts it without naming the branch
has silently settled a blocking question in passing. That is worse than being wrong, because it
will be cited as settled by everyone downstream who never opens the container topic.

**What I will build.** The exhaustive verdict flip: associativity of signed saturating addition over
all triples at declared width four, and over the same triples embedded in an eight-bit container,
with the failing-triple counts for both. The case that must fail is the container run, and if it
also shows failures my example is wrong and I will report that.

## R4. What a lowering site holds

`a_lowering_site_holds_the_numerals_full_type`

**The question.** How much of the numeral's type must be present where it becomes machine
operations?

**My answer.** Enough to evaluate every arm predicate, and the argument is short. I13 says the work
is arms with const predicates over regions. I15 says the selection among them is const and erased.
A predicate can only read what is in scope. So a lowering site that holds less than the axes its
arms' predicates mention cannot select, and the only alternatives are to select at run time, which
I15 forbids, or to not have the arm, which is I13's whole programme abandoned. So the site must hold
every axis any arm reads: widths, signedness, strategy, overflow policy, rounding, and whatever the
arms have since grown to gate on.

**Where I would not follow the word "full".** "Full type" and "every axis its arms read" are not the
same set, and the difference is paid for. Every axis carried is a monomorphisation dimension, and
this workspace has the receipt: `mock/Cargo.toml` records `libarvo_strategy.rlib` at 19 MB and 8.9 GB
of incremental sessions in an afternoon, from exactly this multiplication of per-width, per-strategy,
per-sign instantiations. Carrying an axis nothing gates on buys nothing and costs a factor. So the
demanding, and I think correct, statement is the second one: the lowering site holds every axis some
arm's predicate reads, and "full" is right only if the type is defined as exactly that.

**A consequence worth stating because it constrains the design rather than describing it.** If the
set of axes is "whatever the arms read", then adding an arm that gates on a new axis changes what a
lowering site must hold, which is a change to the type. Either the type surface is extensible along
axes, or every new axis is a breaking change, or arms are confined to axes fixed in advance. Those
are three different designs and the row picks none of them. `operand_window` is a live example: it is
a declared axis in `dimension.toml`, an arm gated on it is exactly the kind of win C5 identifies,
and if the numeral's type cannot carry an operand window then that arm is unreachable no matter how
real the win is.

**What I will build.** A demonstration that a lowering site holding only the container cannot select
an arm that a site holding the parameters can, expressed so that the failure is a compile error
rather than a wrong answer. A compile error is the honest form: the point is that the information is
absent, not that the selection is slow.

## R5. What a derivation's output is, relative to downstream

`an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover`

**The question.** Once a derivation has established something, can whoever consumes the lowered code
work it out again?

**My answer.** Sometimes, and "sometimes" is the whole finding, because the row as titled says never.

The true core is the structural fact this workspace already states in
`small-wins-compound-into-the-program.md`: the typestate proves things the backend never learns, the
proof does not survive lowering, and the compiler then emits a check, a clamp, a branch or a mask
that is provably dead. That is real and it is the reason facts must be carried rather than
recomputed, and it is why R1 and R2 exist at all.

But "cannot recover" is a universal over downstream sites and it is false in the easy direction.
LLVM does range propagation. A value derived from a mask, a modulo, or a bounded loop induction
variable carries a range the backend re-derives without help, and in those cases the fact was
recoverable and carrying it bought nothing while costing a type parameter and a monomorphisation.

So the correct statement is an existential with a region, not a universal: a derivation's output
**may** be unrecoverable downstream, the recoverable and unrecoverable cases are distinguishable,
and which is which is a measurement rather than an axiom. That is also the posture the workspace's
own optimisation rule takes: the characteristic win is noticing the *particular* instruction the
compiler could not prove away, which presupposes that in other places it could.

The practical difference between the two statements is not academic. The universal licenses carrying
every fact always, and under I14's monomorphisation-as-dispatch that is precisely the thing that
produced a 19 MB rlib.

**What I will build.** Two facts of the same shape, one the backend recovers and one it does not,
shown by the presence or absence of a bounds check or a mask in the emitted assembly. If both are
recovered, the row is weaker than I think; if neither is, the universal survives my attack and I
will say so. Qualitative, and named as a spike rather than a measurement.

---

## What I expect to find, recorded now so it can be scored later

Writing this down before reading the rows, because a prediction made afterwards is not one.

- Agree, possibly with a wording sharpening: C1, C2, C3, C6, C8, R4.
- Agree with a narrowed region I expect the row may already carry: C4, C5, C7, R1, R5.
- Expect to refuse or heavily narrow: C9 on the universal and the ratified collision, R3 on the
  container half.
- Genuinely unsure and expect the answer to be a narrow region or nothing: R2.
- Expect to find the `fold` against `reduction` distinction unstated anywhere in the topic, which
  would be a defect in the C2 and C3 pair rather than in either row.

If I come back agreeing with all fourteen, that is evidence the second read was not independent, and
it should be read that way.
