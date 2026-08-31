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
purpose". `mock/registry/probe.toml` now holds 104 `[[probe]]` rows, twenty-five of the thirty
`measured` proposals carry `evidence`, and the suite is green rather than red. The paragraph
describes a state that has passed. Detail in the closing section, including the five that still
carry none and the one of them that is a row I was sent to read.

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

---

# Part two: what the rows say, and what holds

## The blindness I claimed, corrected

The dispatcher has since established that a blind derivation from a registry row is not
available as the schema stands, and the reason is structural rather than procedural: **the `id`
is the claim, spelled as a slug.** `a_coherent_reduction_needs_no_accumulator` is a sentence.
Reading it to find the question hands over the answer in the same act. My part one was written
under an instruction to derive from the id and it inherits that defect in full, so the honest
account is below rather than a claim of independence I did not have.

**What the ids did give me**, and it is not nothing: the claim. Fourteen conclusions, phrased as
assertions, before I had written a line.

**What they did not give me**, and this is where part one has some value: the definitions the
claims turn on, the region each holds in, the instrument behind it, and whether it is true. Every
one of those was mine to work out, and the record of whether I worked them out independently is
the record of where I got them wrong.

**Three substantive misses, and they are the evidence.**

- **C2.** I derived "coherent" as closure of the exact ambient operation on the representable set,
  so that the adaptation never fires. The row's notion is a **congruence** condition: eager
  adaptation agrees with adapt-once. That is strictly broader and strictly better, and it is what
  makes unsigned saturating addition coherent, which under my definition it is not, because the
  clamp does fire and the ambient sum does leave the set. I reasoned about the wrong hypothesis
  for the whole of C2 and arrived at a narrower theorem than the one that was there.
- **C7.** I wrote out two readings, called the chain reading "false in both directions", said "I
  cannot derive this", and predicted the row meant the pairwise one. The row means the chain
  reading and it is correct, and my argument against it was wrong: I asserted that computing the
  clamped-at-the-end answer needs the exact-sum width plus a sticky flag, and it does not. It
  needs one bit **less** than the exact-sum width, for exactly the reason the row gives.
- **C9.** I read "the grid" as the representable set and predicted a collision with the ratified
  spine and with I13. It is the numeral's **grid coordinates**, adjustment and bias and phase and
  canonical exponent, and there is no collision of the kind I predicted.

**Two lesser misses.** I predicted the `fold` against `reduction` distinction would be unstated
and it is stated, in a different vocabulary than I expected: the row separates a fold from a
reduction not by accumulator type but by whether the operation is **widening**, which is a
sharper cut than mine. And I expected to refuse R3's container half outright; the row keys the
verdict on identity-and-adaptation, which dissolves my counterexample, and I say how below.

**One thing part one got right independently and it is worth naming**, because it is the case
where the derivation and the source agree and neither read the other: the sort lattice has a
direction, `type -> const` is free and `const -> type` is refused, so the rule cannot be "the
sort the consuming site uses" when a fact has two consumers. I derived that from the language and
committed it before reading anything; `47:512-517` had already established it with three probes
by two authors and thirteen compiled refusals; and `p4` here compiles it a fourth time. **That
convergence is real and is the only one in this file I would call one.**

## A. Provenance: where each row's standing actually reaches

The first question the dispatcher put, and the answer is worse than I expected for half the set.

**Seven of the fourteen name only a consolidation.** In each case a member file exists, is
nameable, and was dropped at the port rather than being absent from the record. The consolidations
themselves are scrupulous: every one of them carries the member citation at the very line the row
cites. **The loss is entirely at the registry step.**

| row | provenance names | the member file that established it |
|---|---|---|
| C1 chain definition | `63::692`, `AGREEMENTS` | `60_stam_the_chain_derived_cold`, per `63:698-699` ("where `60`'s derivation lands") |
| C2 coherent reduction | `63::498`, `63::683` | `57_orchard_the_grading_and_the_refutation`, instrument `57_probes/p6`, per `63:497-501` |
| C7 one bit less | `63::506`, `63::683` | `57:479-503` and `57_probes/p6`, named in full at `63:505-511` |
| C8 lifting split | `90::445`, `90::451` | `80_rompf_when_the_deriving_happens` section 4.5, instrument `80_probes/p6_which_chain_laws_reduce_to_arity_three.rs`, per `90:446-448`; the pricing half is `86` F3 |
| R1 carried fact's sort | `53::#4-2` | `47_wingo_one_richer_output:505-510`, judged by `48:388-425`, relocated by `50:374-391`, per `53:369-374` |
| R3 verdict invariance | `90::464`, `90::470` | no member file. `90:465` says "Carried by `80` section 8 from the number-system consolidation, read at source", citing `74:144-147` and `74:507-511`, and `74` is itself a consolidation |
| R4 lowering site premise | `53::#4-2` | no member file for the premise. `53:341-345` cites `45:314-333` and `46:243-251` for two narrower sub-claims and says the premise itself is "a design commitment for op to bless, not a finding" |

**R1 is the sharpest instance**, because the registry already knows how to cite its source: R5's
provenance names `47_wingo_one_richer_output` twice, at lines 62 and 180. R1's source is
`47:505-510`, in the same file, and R1 cites the consolidation instead.

**R3 and R4 are a different and worse shape.** Neither reaches a member file at all. R3's chain is
consolidation to consolidation: `90` carries it from `80` section 8, which carried it from `74`,
which is the number-system consolidation. R4's premise is authored inside `53` and says so. So for
these two the honest statement is not "the provenance lost the member file", it is **"there is no
member file", and a row whose whole provenance is consolidations is one artifact however many
files the chain runs through.**

**Seven reach a member file and are clean on this axis**: C3 (`35_mcsherry`, with a section
anchor), C4 (`80_rompf::552`), C5 (`60_stam::122`), C6 (`199_leijen`, two anchors), C9
(`43_rompf`, with a section anchor), R2 (`51_fog` twice plus `53`), R5 (`47_wingo` twice).

**The standing field itself.** All fourteen carry `one_expert`, and on this evidence that is the
correct value for all fourteen: none names two independently arriving files. So I found no
instance here of the defect the parallel seat measured, where a row asserts multi-expert standing
while naming at most one file. **What I found instead is the axis below it: a row can be honestly
marked `one_expert` and still be unable to say which expert**, which is R3 and R4.

## B. Region: where each predicate reaches, and where the prose reaches past it

### B1. Two predicates state a rectangle where the instrument covered a staircase

**C2 and C7 both do this, and the numbers are checkable against the instrument they both cite.**

`57_probes/p6_output.txt` is a table of cells, and the cells are bounded by the tuple count
`2^(W*L)` rather than by W and L separately. Its signed half has **19 rows**: W=3 at L=2..8, W=4
at L=2..6, W=5 at L=2..5, W=6 at L=2..4. Its unsigned half has **16 rows**: W=3 at L=2..8, W=4 at
L=2..6, W=5 at L=2..5.

The predicates state ranges, and two ranges in a predicate are read as a product:

```
C7:  total_width: W in 3..=6   chain_length: chain length in 2..=8     ->  28 cells claimed, 19 measured
C2:  total_width: W in 3..=5   chain_length: chain length in 2..=8     ->  21 cells claimed, 16 measured
```

**The porter's own file has the right numbers.** `182_orchard_porting_the_four_consolidations.md`
lines 298 and 299 say "sixteen unsigned rows at `W` 3 to 5, fold lengths 2 to 8" and "the same
table's nineteen signed rows at `W` 3 to 6". The counts were in hand and the predicate does not
carry them.

**I do not think this is carelessness, and the useful version of the finding is about the
notation.** A predicate is a list of per-axis entries, so it can say `W in 3..=6` and
`L in 2..=8` and it has no form for "the staircase where `W * L <= 25`". The porter had a choice
between a rectangle that over-claims and dropping an axis, and under I13 dropping an axis is the
more destructive of the two. **So the notation forced the over-claim**, and the fix is a notation
question rather than a correction to two rows.

**And in this instance the over-claim happens to be true.** `p2` and `p3` here cover all nine
signed cells C7 claims and its instrument never reached, `(4,7) (4,8) (5,6) (5,7) (5,8) (6,5)
(6,6) (6,7) (6,8)`, and every one reports a gap of one bit. Same for C2's five. So nothing needs
correcting in either row's content. **The mechanism still wants naming, because the next row it
produces will not be lucky.**

### B2. One sentence states a width and a guard under one verb

**C6 says**: "the operation decides how the width grows: addition at `W + ceil(log2 C)`,
logarithmic in the bound, and multiplication at `(n-1)F`, linear in the length".

Those two quantities are not the same kind of thing. `W + ceil(log2 C)` is an **accumulator
width**. `(n-1)F` is a **guard**, the fraction bits a product needs beyond the ones the format
already declares. The exact product of `n` operands at fraction width `F` has `nF` fraction bits
and total width `n(I+F)`; the guard is `nF - F = (n-1)F`.

**The two-expert row on the same quantity carries the disambiguating word and this one drops it.**
`the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion`, at
`standing = "two_experts"`, says "the exact **guard** grows linearly in fold length at `(n-1)F`
bits". C6 restates it as "multiplication at `(n-1)F`" inside a sentence whose subject is how the
width grows.

**A reader who builds an accumulator from C6 gets the multiplicative case wrong by a factor.** At
`W = 8, F = 2` and three operands, `(n-1)F` is 4 while the exact product needs 24 bits. `p1`'s N4
arm prints that guard for `F` from 0 to 3 and is the check.

**C5 states the same quantity a third way and is correct**: "the sum of every fraction width
involved, `kF` bits at uniform `F`", which is a total over `k` operands. So the topic carries a
total, a guard, and a mixed sentence, with `k` and `n` used for the operand count in two rows and
"fold length" for it in a third. **All three are individually defensible and no two use one
measure.**

### B3. Two predicates omit every axis that would name a format

**C8's predicate** is `chain_length: 2..=5`, `overflow_policy: {wrap, saturate}`,
`operation: {add, mul}`, `arity: 3 for the grouping kind and 2..=5 for the schedule kind`.
**R3's** is `container: container any`, `overflow_policy: {wrap, saturate}`.

Neither names `total_width`, `signedness` or `fraction_width`. Under the reading `proposal.toml`
states for itself in its own header, "an absent axis says the claim holds in no situation where
that axis exists", **each of these is a measured claim about arithmetic that holds at no width and
at neither sign**, which is a region containing no format at all.

**The header says `182` names every instance of this narrowing. Neither of these two is in it**:
`grep` for both ids in `182_orchard_porting_the_four_consolidations.md` returns zero. C8 and R3
came through the derived-laws port rather than that one, so the guarantee the header offers does
not cover them, and a reader who trusts the header will not go looking.

**C8 additionally names `chain_length: 2..=5` and `arity: 2..=5` as separate entries** for what
its own sentence treats as one quantity, the number of operands in the chain. If those are one
axis the predicate names it twice, which the `predicate-names-one-axis-twice` check exists to
catch and does not catch here because the two entries have different axis names.

### B4. One predicate excludes the operation its sentence is half about

**C5's predicate is `operation: operation = mul`**, and its `says` opens with "Addition composes
headroom logarithmically ... a fold of `k` adds at fraction width `F` is held exactly by
`ceil(log2 k)` extra integer bits". Under I13 the predicate says this row holds in no situation
involving addition. **The row's own note is honest about the mechanism** and says the predicate
carries only the probe's tested region because the width arithmetic is elementary rather than
measured. That is the right instinct and it leaves the sentence and the predicate contradicting
each other, which the notation has no way to express and the note has no standing to fix.

## C. The fourteen verdicts

**Weighting, per the dispatcher's ordering.** An agreement I cannot ground independently is worth
less than a refusal, so I say for each one what I actually did, and where the answer is "I read it
and could not arrive at it separately", I say that instead of agreeing.

### C1. `a_chain_is_exact_operations_together_with_a_schedule_of_adaptation_points`

**Agree, independently reached, with one wording finding.** My part one derived the same object
from the ratified factoring and for the same reason the row's `because` gives, that a concept
fusing adaptation into each operation cannot state I7 at all. I did not read `60` before writing
it and the argument is the same argument.

**The wording finding stands after reading.** "A schedule of adaptation points" fixes where and is
silent on what, and the row does not say that a point carries the whole adaptation rather than a
position. The ratified spine makes the adaptation a first-class object with its own laws, so a
point should be an occurrence of that object; the row leaves two readers able to satisfy the
definition with schedules that round differently. **The row's own `gap` field names a different
open question, where a chain lives, and not this one.**

**My linear-against-DAG worry is unresolved by the row and I now think it is real.** The row says
chains factor into windows, which is a decomposition of a term rather than of a sequence, so the
DAG case is in scope and "schedule" is the wrong word for a labelling. Small, and worth one word.

`holds for: nothing. This is a definition and carries no region, per the registry's own
convention, and I am not supplying one.`

### C2. `a_coherent_reduction_needs_no_accumulator`

**Agree within the stated region, on a second instrument, and I got the hypothesis wrong first.**

`p1` sweeps unsigned saturating addition exhaustively over every tuple at 21 cells, W from 3 to 7
and L from 2 to 8 as the budget allowed, and the format's own width sufficed in every one. That
covers C2's 16 measured cells and adds W=6 and W=7, which its instrument did not reach.

**What my agreement is worth is limited by what I varied.** I swept the same operation, the same
policy and the same sign domain. So this is a second instrument, not a second angle.

**One widening I establish separately, as a new claim rather than an edit to the row.** `p1`'s Q3
runs the identical construction at four common scales and locates the same minimum at every one,
with N4 as the control showing the harness does see a scale, since the multiplicative guard moves
with it. Addition at a common scale is raw addition and the bounds scale with it, so the located
minimum is invariant under the point position. **The ratified `the_additive_and_absorption_verdicts_are_canon`
already says this** ("an additive verdict, survival and breakage alike, is independent of the
fraction width"), which makes my measurement a confirmation of a ratified sentence rather than a
new result.

**And it names no instrument.** C2 is `sentence_kind = "measured"` and carries no `evidence` field,
one of five such rows in the registry, under a ratchet whose ceiling is six. Its instrument is real
and committed and named in its `note`, and neither of the two `probe` rows in that directory covers
the arm it rests on. **So this row cannot be checked by anything, and my second instrument does not
change that**, because a second instrument is not an edge. Section D carries the detail.

`holds for: signedness = unsigned, overflow policy = saturate, operation = add, W in 3..=7,
L in 2..=8 subject to 2^(W*L) <= 4*10^7, F = 0 measured and scale-invariant across scales
{1,2,4,8}, threads = 1, toolchain nightly-2026-05-28, build profile opt level 3.`

### C3. `a_fold_needs_a_closed_operation_and_a_separately_determined_accumulator`

**Agree, and my part-one anxiety about it contradicting C2 was misplaced for a better reason than
the one I expected.** I predicted the two rows were consistent only under the standard fold/reduce
type distinction and that the distinction was unstated. The row makes a sharper cut than mine: a
fold cannot take a **widening** operation, because the accumulator is loop-carried and therefore
has exactly one type, while a widening operation gives it a different type every iteration. That
is a better statement than "closed in B", it is a fact about the language rather than about
algebra, and it makes C2 the case where nothing widens.

**I could not arrive at that separately.** I had the type-level shape and not the loop-carried
argument, and the loop-carried argument is the whole of why it is true.

**The gap field is right and understates itself.** It says the row names a capacity as the second
input and does not say where a consumer writes one or what the derivation from element width and
capacity is. `p4`'s B4 arm shows the capacity-as-a-type shape compiling gate-free, which is a
demonstration that such a derivation is expressible and not a statement of what it should be.

`holds for: toolchain nightly-2026-05-28 edition 2021, build profile opt level 3, threads = 1.`
My own arm ran on one thread and I decline to carry the row's `threads any`, which its own note
says is argued rather than swept.

### C4. `a_folds_compile_time_refusal_is_the_staging_boundary_reporting_its_own_position`

**Agree with the row as written, and refuse the reading its id invites.**

The row's sentence is careful and is about a specific keying: capacity is stage zero, length is
stage one, and a capacity-keyed accumulator relation compiles while the same relation keyed on
length is refused. **The id drops all of that and reads as a universal over a fold's compile-time
refusals.** That universal is false and `p4` compiles three refusals of a fold with three distinct
causes:

```
b1  accumulator width keyed on a runtime length     E0435  attempt to use a non-constant value in a constant
b2  a perfectly staged fold, unsatisfied bound      E0277  the trait bound `String: Addable` is not satisfied
b3  fully stage-zero, const evaluation faults       E0080  attempt to compute `...+3_usize`, which would overflow
b4  the same fold keyed on a static capacity        compiles, gate-free
```

**B4 is what makes B1 mean anything.** Without it, B1's refusal could be about the widening rather
than about the staging. With it, the boundary is located at the count's binding time and nowhere
else, which is the row's claim, reproduced independently.

**One arm of mine failed before it worked and the failure is committed.** B3's first version made
the accumulator width a const fn of a const generic, and rustc refused it for the same reason it
refuses A2, so it demonstrated the generic-const wall a second time instead of a third cause. The
diagnostic is at `216_probes/p4_v1_b3_was_refused_for_the_wrong_reason.out`.

**So: the row is right and the slug is not, and the slug is what the registry is searched by.**

`holds for: toolchain rustc 1.98.0-nightly (57d06900f 2026-05-27) edition 2021, build profile
no feature gates, threads = 1.`

### C5. `a_multiplicative_chain_is_writable_without_an_ever_growing_intermediate_by_windowing`

**Agree on the width arithmetic, which I derived independently, and cannot ground the cost
comparison.**

Part one derived the growth (`n` operands at width `w` need `n*w`, unbounded, so under I14 an
unbounded exact multiplicative chain is not writable at all) and identified windowing with a
static position for fixed point and a moving one for a running exponent. The row's three-way cost
comparison, per-op against windowed against fully exact, is a real result and I did not reach it
and have not checked it.

**The `operand_window` split I proposed in part one is not in the row and I still think it is
right.** For operands in `[0,1)` the integer part of an exact product never grows, so a statically
positioned window suffices and the loss is a pure truncation tail; above one the window must move.
`operand_window` is a declared axis, so this is expressible as an arm. **I have not measured it
and I am not claiming it**; I am recording it as the shape a later expert should attack.

**The predicate excludes addition while the sentence is half about addition.** Section B4.

`holds for: nothing I established. The elementary counting I did (a product of n operands at
fraction width F has nF fraction bits, checked in p1's N4 at F in 0..=3 and n = 3) supports the
guard formula and not the cost comparison, and the cost comparison is the row's contribution.`

### C6. `an_accumulator_is_decided_by_the_operation_and_by_whether_a_bound_exists`

**Agree with the two axes, which I derived independently, and refuse one clause of the sentence on
a measure error.**

The two axes are right and part one reached them: the operation gives the growth law, and whether
a bound exists decides whether that law has a value at all. My part-one worry that the sentence is
silent on the unbounded branch is answered by the row, which says a hand-chosen accumulator "is
correct only up to the length its author guessed", and that is better than what I had.

**The refusal is section B2**: "multiplication at `(n-1)F`" is a guard and the addition clause
beside it is a width, and the sentence presents both as what "the operation decides how the width
grows" produces. The two-expert row on the same quantity says "guard" and this one does not.
**Fixing it is one word.**

**A second thing the row does that I want on the record because it is right.** Its note says no
dimension declares whether a bound exists, that every value ever written on `chain_length` is a
number or a range or `any`, and that the condition therefore lives in the sentence rather than the
predicate. **That is the correct handling of a missing axis** and it is the opposite of what B3's
two rows do with their missing axes, in the same registry.

`holds for: operation = add, arity = 2, and W any, established by counting rather than by
measurement: n operands of unsigned width W have maximum sum n(2^W - 1), which needs
W + ceil(log2 n) bits and no fewer, the bound being attained by the all-maximum vector. The
multiplicative half I did not establish and do not carry.`

### C7. `an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit`

**Agree inside the stated chain-length region. Refuse any extension of the constant, with an
exhaustive counterexample at the first fold length outside it.** This is the row where the second
read produced something.

**Reproduction first.** `p1` locates the minimum sufficient accumulator width exhaustively at 21
signed cells, W from 3 to 7, and the gap is exactly one bit in all 16 cells above fold length two.
That reproduces the row independently, on an instrument written from the spine rather than from
`57_probes/p6`, and extends it to W=7 which its instrument never reached.

**Then the constant breaks.** `p2` built two routes past the exhaustion wall, a derived predictor
from the clamp-event inequality and a search over the extremal family that p1's witnesses all
belong to, and validated both against **all 42** exhaustively checkable cells with an off-by-one
control that agreed on only 11 of them. Both routes then report a gap of **two** at signed
`W = 3, L = 9`. `p3` settled it by exhaustion rather than by extrapolation, over all `2^27` tuples:

```
   W    L           tuples   exact exhaustive    gap  witness at measured-1
   3    9        134217728       7          5      2  [3, 3, 3, 3, 1, -4, -4, -4, -4]
```

**The exact-sum width is 7, the minimum sufficient accumulator is 5, and the gap is two bits.**
Further cells at gap two: `W=3` at `L=17` and `L=18`, and `W=4` at `L=17`.

**Why the constant looked like a law.** The minimum and the exact-sum width are both step
functions of the fold length and their steps do not land in the same places. `ceil(log2 L)` and
`1 + ceil(log2(ceil(L/2)))` agree for every `L` from 2 to 8 and part company at 9. **The region the
instrument measured is exactly the region where the two agree**, and the first fold length outside
it breaks the constant.

**So the consolidation's own instruction was right and is now more than a precaution.** `63:689`
says the sentence survives without the constant, as "at most the exact-sum width", "which is how a
canon should hold a constant that is measured and not proven". It is no longer merely unproven. It
is **false one step outside the region**, and the weaker form is the only one that may be
promoted.

**A caution on my own S3.** I fitted a candidate closed form and it turned out algebraically
identical to the row's constant, so the table did not discriminate between them and both miss the
same four cells. `p3` prints that as a failed discrimination rather than hiding it. **I have no
closed form to offer**, and the honest output is the table plus the refutation.

`holds for: signedness = signed, overflow policy = saturate, operation = add, F = 0, W in 3..=7,
L in 2..=8 subject to 2^(W*L) <= 4*10^7, threads = 1, toolchain nightly-2026-05-28, build profile
opt level 3. Refuted at signedness = signed, W = 3, L = 9, exhaustively.`

### C8. `chain_laws_split_by_whether_a_lifting_theorem_exists`

**Agree with the split, which I derived independently, and refuse the predicate.**

Part one derived the same two families and the same instance on each side, and proposed the
sufficient condition I still think the row should carry: **a lifting theorem exists for an
identity-shaped law exactly when the adaptation is a homomorphism for the operation.** The row
names a different and better sufficient condition for the grouping half, the generalised
associative law from arity three, which is sharper than mine because it is about the law's shape
rather than about the adaptation. It has nothing corresponding for the schedule half, where "no
lower-arity statement implies them" is a statement about what is not available rather than a
criterion. **An existential over proofs is not checkable and nothing can be gated on it**, which
matters under I13 because the split is only useful if an arm can test it.

**The predicate refusal is section B3.** No `total_width`, no `signedness`, no `fraction_width`, on
a row whose `because` reports measured divergence counts for wrapping against saturating addition.
Under the registry's own stated reading that region contains no format. **And it is not in `182`'s
list of narrowing artefacts**, which the header offers as the place every instance is named.

**The consequence I care about is the one the row states and it is the reason to keep it.** It
tells you when `chain_length: any` is earned and when it is a widening. A finding that swept
triples and wrote `chain_length: any` has claimed a lifting theorem it did not prove, and this row
is what makes that visible.

`holds for: nothing I established. I did not build the arity sweep and I am not carrying the row's
measurement.`

### C9. `no_derivation_reads_the_grid_so_a_composition_may_hold_it_at_run_time`

**Cannot settle, and my part-one objection is withdrawn as based on a misreading.**

I read "the grid" as the representable set and predicted a collision with the ratified spine's
"the representable set is a constant of the type" and with I13's const predicates. **The row means
the numeral's grid coordinates**, adjustment and bias and phase and canonical exponent, and its
claim is that the carrier, the stride and the fold accumulator derive to the same types across
numerals differing in those. That is a compiled type-equality result, not a claim about the
representable set, and my objection does not reach it.

**What I would want checked, and did not check.** The row says a coordinate no derivation reads
may move to run time for free. Its own note records that the **operations** do not agree with the
derivations, that a multiplication reads the canonical exponent, and that this is measured in the
same file and not filed. **So a composition holding its grid at run time has derivations that are
unaffected and at least one operation that is not**, and the row states the first half. Whether
that is sound for a consumer depends on the half that is not filed.

**I am not calling that a refusal.** The row is scoped to derivations and says so in its own note.
I am saying that the row and the unfiled half are a pair, that the row is the safer-sounding one,
and that a reader taking it alone will conclude more than it says.

`holds for: nothing. I did not reproduce the type-equality result and have no instrument here.`

### R1. `a_carried_fact_takes_the_sort_its_consuming_site_uses_it_in`

**Agree, independently derived, independently compiled, and incomplete in a way I can fill.**

`p4` compiles the asymmetry on the pinned toolchain:

```
a1  fact as a const generic, used as a const generic argument and as a const value   compiles
a2  the same fact as an associated const, used as a const generic argument           refused
      error: generic parameters may not be used in const operations
      error: constant expression depends on a generic parameter
a3  the same associated const, used only as a const value                            compiles
a4  fact as an associated type, projected to a const generically                     compiles
```

**A3 is the control that makes A2 mean anything.** Without it, A2 shows only that a program does
not compile. With it, the refusal is located at the sort the consuming site wants.

**The incompleteness.** "The sort its consuming site uses it in" is underdetermined the moment a
fact has two consumers wanting two sorts, and facts of this kind routinely do: a width is a const
generic argument at one site and a const value at another. A1 and A4 say the lattice has a
direction, so the well-founded rule is **carry it in the strongest sort any consumer needs**, from
which the weaker uses are free.

**That refinement is already established and the row lost it.** `47:512-517` records it as a
"kind-asymmetry entry, with its two compiled refusals", `type -> const` total and gate-free,
`const -> type` refused naming a forbidden feature, across three probes by two authors and
thirteen refusals from three starting points. The row's source is `47:505-510`, five lines above
it. **So the fix is not new work; it is restoring a sentence from the same passage.**

`holds for: toolchain rustc 1.98.0-nightly (57d06900f 2026-05-27) edition 2021, build profile
no feature gates, threads = 1, on the four carrier shapes in p4_arms and no others.`

### R2. `a_fact_delivered_as_a_const_a_generic_body_loops_over_costs_the_reduction`

**Cannot settle, and I decline to try with the instrument available to me.**

Part one predicted this would be substantially false at `-O` because LLVM unrolls a known trip
count, and that its region would collapse to debug builds. **The row is not vulnerable to that
prediction**, because its finding is not "the loop survives": it is that the const-and-loop form
serialises the reduction onto one accumulator and is worse code **despite emitting fewer
instructions**, at and above the width where the live access window reaches four bytes, isolated
by sweeping width with shape held fixed and repaired by moving the fact onto a per-width trait.
That is a sharper claim than the one I was ready to attack.

**And it is a claim about how much, so it needs the harness.** The row's own note says it is
"priced only in instruction shape on one host and one toolchain (ONE EXPERT, counts read off
emitted assembly, nothing timed)". Anything I produced here would be an ad-hoc quick spike with no
substance, by this workspace's own naming, and it could not decide the question either.

**What a second read can say without measuring.** The predicate is `W in 18..=128` and
`operation = packed_access_gather`, which is one gather shape, and the row's `gap` field says so.
**The general rule its gap field floats**, that a fact whose consuming loop bound is itself a fact
the derivation owns should be delivered as a contract rather than a const, is the interesting
sentence and is untested. It is also the one a later reader will quote.

**This needs someone who will run `mock/benches/` with real competitor arms**, not another reader.
The arms are named in the row: the const-and-loop form, the per-width contract, the hand-written
twin, and the single fixed-width load that beats the twin at W=47.

`holds for: nothing. I established nothing about this row.`

### R3. `a_law_verdict_is_invariant_under_change_of_encoding_and_container`

**Agree, and my part-one refusal is withdrawn, with a note on what makes it non-vacuous.**

I planned to refute the container half with signed saturating addition at declared width four
against the same values in an eight-bit container, where associativity fails in the first and
holds in the second. **The row is not vulnerable to it.** It keys the verdict on the pair of
identity and **selected adaptation**, and saturating onto `[-8,7]` against saturating onto
`[-128,127]` is a change of adaptation, not a change of container. My counterexample changes the
thing the row says moves the verdict, which is the row being right.

**What keeps it from being a tautology.** The claim has content precisely because a map that reads
the container is constructible and would break it. That is what the row's `container: container any`
asserts and what the design must therefore not do: the container derivation may read the container,
and the law layer may not. **The row states that consequence and states it as the interesting
half**, that the law layer's compile-time computation is keyed on strictly less than the container
derivation's.

**Two things it does not carry.** The encoding half cannot be predicated because no `dimension` row
declares an encoding axis, which the note says plainly and correctly. And no `total_width`,
`signedness` or `operation`, per section B3.

**And it sits on the container premise without saying so.** `topic.toml` records
`the_container_premise` as blocking, with "no wording of several downstream clauses is true on both
branches". This row is true on both branches and **means different things on each**: under
declared-width semantics it says the container is storage and the verdict cannot see it; under
container semantics the container is part of the identity, so the sentence is true by construction
and says nothing. **A row whose content collapses on one branch of an open premise should say which
branch it is interesting on.**

`holds for: nothing measured by me. I constructed the intended counterexample, found it changes the
adaptation rather than the container, and withdrew it; that is a reason to believe the row and not
a measurement of it.`

### R4. `a_lowering_site_holds_the_numerals_full_type`

**Agree, and it should not be promoted, and its own note says why.**

The row is a design commitment for op to bless, in its source's own words, not a finding. It has no
member file behind it, it is authored inside a consolidation, and `53:341-345` supports two narrower
sub-claims from `45` and `46` rather than the premise itself. Under the ratification model where two
converging experts promote, **a design commitment is exactly the kind of sentence that model does not
cover**: I can agree with it and my agreement is not evidence, because there is nothing to have
independently derived. It is a choice.

**My part-one refinement stands and is a real objection to the word "full".** Every axis carried is a
monomorphisation dimension, and this repository has the receipt in `mock/Cargo.toml`: 19 MB for one
strategy rlib and 8.9 GB of incremental sessions in an afternoon, from exactly this multiplication of
per-width, per-strategy, per-sign instantiations. **The demanding statement is "every axis some arm's
predicate reads"**, which is what makes arm selection possible under I13 and I15, and which costs
nothing for axes nobody gates on.

**The consequence that makes it a design question rather than a wording one.** If the axis set is
"whatever the arms read", adding an arm on a new axis changes what a lowering site holds, which is a
type change. `operand_window` is the live case: a declared axis, the natural gate for C5's windowing
arm, and unreachable if the numeral's type cannot carry one. Either the type surface is extensible
along axes, or every new axis is breaking, or arms are confined to axes fixed in advance. **The row
picks none of the three and it is op's pick, not mine.**

`holds for: nothing. This is normative and carries no region, correctly.`

### R5. `an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover`

**Refuse the row's continued existence in this shape, and it is the strongest finding in the
realisation-map half.**

**The id does not name the claim.** The sentence is about **packaging**: any product is one thing, a
pair is a single element of a single set, and the count of associated items on a derivation trait is
not the count the predicates produce. Nothing in it is about recoverability. The row's own note
explains why: the opening sentence that did name recoverability was retired as incapable of defining
anything, and what stands is the packaging argument the retirement does not touch.

**So the row was rewritten and its id was not.** Everything the registry is searched and cited by is
the id, `keywords` still carries "recoverable", and a reader looking for the recoverability principle
lands on an argument about pairs.

**And three rows still declare they supersede it.** `a_lowering_site_holds_the_numerals_full_type`,
`a_fact_is_carried_when_producing_it_applies_a_rule_the_strategy_owns` and
`a_carried_fact_takes_the_sort_its_consuming_site_uses_it_in` each carry
`supersedes = ["an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover"]`. Those edges
were true of the retired sentence. **They are not true of what the row now says**, which the three
clauses do not replace and which its own note says the retirement does not touch. A reader following
`supersedes` concludes the packaging argument is dead. It is not.

**Where the recoverability principle actually lives now**, and this is the constructive half:
`a_fact_is_carried_when_producing_it_applies_a_rule_the_strategy_owns`, at `two_experts`, which says
"a pure function of what every site already holds is recomputed rather than carried". **That is the
correct form of what I derived in part one**, and it is correct in the way the id is not: it is a
criterion for when a fact must be carried, rather than a universal claim that downstream can never
recover anything.

**My part-one attack on the universal is therefore an attack on a sentence that has already been
retired**, which I could not know from the id. The attack was right about the universal: LLVM does
range propagation and recovers easy facts, so "cannot recover" is false in the easy direction and
would license carrying every fact always, at the monomorphisation cost above. **That is now an
argument for keeping the two-expert row's wording rather than a finding against anything live.**

`holds for: nothing. This is normative and carries no region, correctly.`

## D. Things I found that the question did not ask about

**A canon file describes a state that has passed, and one sentence of it is still true in a way
that matters more than the stale ones.** `mock/registry/proposal.toml:29` says "`probe.toml` does
not exist yet, so there is no row for a `measured` proposal to point at", and line 31 says "The
`measured` rows carry no `evidence` and the check for that is red on purpose".

`mock/registry/probe.toml` now holds 104 `[[probe]]` rows, **twenty-five** of the thirty `measured`
proposals carry `evidence`, and `cargo test --workspace` is 120 green. So the first sentence is
false, and "the check is red on purpose" is false too: the check is
`no_new_measurement_lands_without_an_instrument` and it is a **ratchet**, asserting
`found.len() <= 6` against a ceiling whose own doc comment enumerates why each of the six is stuck.
It is green because five is under six.

**And one of the five is a row I was sent to read.** `a_coherent_reduction_needs_no_accumulator`
is `sentence_kind = "measured"`, stands at `one_expert`, and names no instrument. Its `note` names
`57_probes/p6_the_adaptation_absorbs_one_bit.rs` in prose, and `57_probes/` does have probe rows:
`the_accumulator_width_is_the_exact_sum_width_less_one_bit`, which is p6's **signed** half and is
the neighbouring row's evidence, and `a_coherent_reduction_diverges_nowhere_at_any_fold_length`,
which lives at `p4` and measures divergence counts rather than sufficiency. **Neither is p6's
unsigned half**, which is what C2 rests on, and the second row's own note says as much about why it
had to be written separately. So C2 is the check's own "a row moves before an edge can" case,
sitting under a grandfathered ceiling.

**Under the model where two agreeing experts promote, that is a promotion blocker rather than
bookkeeping.** A `measured` row whose instrument is named only in prose cannot be checked by
anything, and my agreement with it does not repair that: I built a second instrument, and a second
instrument is not an edge. **The repair is a `probe` row for p6's unsigned half and an `evidence`
edge from C2 to it**, and it is a judgement for whoever owns that registry rather than mine to make
from outside.

**The class, stated once.** There is a check for a registry comment counting its own rows and none
for a comment describing a state that moved. That is the harder class, and it is the one that
misleads a reader who trusts a header. I made the same error reading it: my first note in this file
said all thirty `measured` rows carry `evidence`, which I had inferred from two separate counts
that happened to agree, and it is exactly the shape my own rules call two true statements welded
into a connection neither supports. The check that caught it was running the join instead of the
two counts.

**The header offers a guarantee it cannot keep.** It says `182` names every instance of the
absent-axis narrowing. C8 and R3 both have that narrowing and neither appears in `182`, because they
came through a different port. **A reader who checks `182` and finds nothing will conclude the row is
not affected.**

**A predicate cannot express a staircase, and the notation forces an over-claim.** Section B1. Two
rows here state rectangles their instruments covered as staircases, and the porter had the correct
counts in hand at `182:298-299`. This is not fixable row by row; it wants either a joint-constraint
form in the predicate grammar or a convention that a bounded region is written as the bound.

**`chain_length` and `arity` are used for one quantity in one predicate.** C8 names both, at the same
range, for what its sentence treats as the number of operands in the chain. The
`predicate-names-one-axis-twice` check cannot see it because the two entries carry different axis
names. Whether they are one axis is a question for whoever owns `dimension.toml`, and if they are,
the check has a blind spot shaped exactly like this.

**A pipeline into `head` truncated one of my own artifacts and reported success.** `p1` was first run
as `./p1 | tee p1.out | head -70`; `head` closed the pipe at 70 lines, the program took SIGPIPE, and
`p1.out` ended mid-section looking like a completed run that had stopped reporting. `echo $?` was 0,
because that is the last command in the pipeline. **The artifact was wrong and every signal said it
was fine.** `216_probes/RUN.md` records it.

**Three tautologies in my own instruments, caught by controls rather than by care.** A Q3 arm whose
`fbits` parameter was unused, so it asserted a value against itself. A replacement whose value-domain
path computed `raw * s / s`, which is the identity. And a candidate closed form in `p3` that turned
out algebraically identical to the rival it was being compared against, which the probe printed as
"the table discriminates between the two: false" rather than as a match. **The first two I caught by
re-reading before running; the third the instrument caught for me**, and that is the argument for
building the control before the measurement rather than after.

## E. Summary of the fourteen

Ordered as the dispatcher asked, provenance first.

| row | provenance reaches | region | verdict |
|---|---|---|---|
| C1 chain definition | consolidation only, member is `60` | none, definitional | agree, independently reached; "schedule" underspecified on what a point carries |
| C2 coherent reduction | consolidation only, member is `57` | rectangle over a staircase | agree in region, second instrument, 21 cells exhaustive; **names no instrument**, so unpromotable as it stands |
| C3 fold needs a closed op | member file, anchored | toolchain only | agree; I could not reach the loop-carried argument separately |
| C4 fold refusal is staging | member file, anchored | toolchain only | agree with the row, refuse the id's universal, three causes compiled |
| C5 windowing | member file, anchored | excludes the operation half its sentence is about | agree on the counting, cannot ground the cost comparison |
| C6 accumulator's two axes | member file, anchored | narrow and honest about it | agree on the axes, refuse "multiplication at (n-1)F" as a width |
| C7 one bit less | consolidation only, member is `57` | rectangle over a staircase | agree in region; **refuted at L=9 exhaustively**, constant must not be promoted |
| C8 lifting split | consolidation only, member is `80` | names no width and no sign | agree with the split, refuse the predicate, criterion not gateable |
| C9 no derivation reads the grid | member file, anchored | toolchain only | cannot settle; my objection withdrawn as a misreading |
| R1 carried fact's sort | consolidation only, member is `47:505-510` | toolchain only | agree, compiled independently; the asymmetry that completes it is at `47:512-517` |
| R2 const the body loops over | member file, anchored | one gather shape | cannot settle; needs the bench harness, not another reader |
| R3 verdict invariance | consolidation to consolidation, no member | names no width and no sign | agree, my refutation withdrawn; sits on the blocking container premise |
| R4 lowering site premise | consolidation, no member | none, normative | agree and do not promote: a design commitment, op's call |
| R5 derivation's output | member file, anchored | none, normative | refuse the shape: id names a retired claim, three stale `supersedes` edges |
