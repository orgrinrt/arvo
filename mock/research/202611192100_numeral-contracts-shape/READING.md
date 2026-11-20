# Reading of the numeral-contracts shape

The one expert seat's reading of `SHAPE.md` in this directory, before the design round for step 5 of
`mock/research/202611130950_canon-double-check/PLAN.md` opens. Everything I built for it is committed in
`mock/research/sketches/202611192130_numeral-contracts-reading/` (its `FINDINGS.md`, sources, `run.sh`,
`outputs/`). Below, "the probes" means that directory, and "the sketch" means
`mock/research/sketches/202611192015_numeral-contracts/`.

## 0. The two gates

### The canon gate: passed on direction, with one transcription defect named here and fixed in section 5

I checked the shape against these rows, each queried with `cargo mock query`:
`obligation::the_unstable_machinery_does_not_reach_a_consumer`,
`obligation::the_surface_expressible_as_contracts_before_anything_implements_it`,
`ruling::the_trait_contract_structure_is_a_primary_paradigm` (ratified, op),
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` (ratified,
experts), `ruling::never_a_runtime_check_and_one_lowered_path` (ratified, op),
`ruling::the_overflow_panic_is_permitted_and_bounded` (ratified, op),
`ruling::the_predicate_is_whatever_is_available_at_const_time` (ratified, op),
`ruling::the_additive_and_absorption_verdicts_are_canon` (ratified, experts),
`ruling::the_operating_constraints_are_intents_and_rules` (in_force),
`question::what_the_ingest_row_claims_and_what_turns_on_it` (open), the four law rows the shape cites plus
`law::multiplicative_associativity_under_saturation`, and the three `retirement::dl_*` rows.

The shape does not build on a premise any of these forbids. Contracts as the primary structure is op's
ratified intent. Typing the associated items in the stack's own types, with the bare integer only at a
const parameter, is inside the numeric-door ruling. Keeping construction a compile-time door leaves the
open ingest question alone. On these rows, no ratified row fixes where traits live. The trait-contract
obligation says so in its own `gap`: "Nothing in the tree enforces where traits may live."

The defect is in section "The law suite and its regions" (`SHAPE.md:114-125`). That section binds itself to
the law rows and then misstates them. It reads `holds` and `fails` as complements, which the header of
`mock/registry/law.toml` says they are not: "Absence from `holds` is not a claim that a law fails there".
It also asserts a failure at W = 1 that `ruling::the_additive_and_absorption_verdicts_are_canon` decides
the other way: absorption, over the values the format can hold, decides associativity, and at W = 1 signed
saturation absorbs nothing (measured, section 5). I class this as a wrong transcription of the canon inside
an aligned section, not as a misaligned premise, and so I did not stop at the gate. That classification is
one expert's call, and a second read is owed on it before anyone relies on it.

### The suite gate

The whole workspace suite, `cargo test --manifest-path mock/Cargo.toml --workspace` at `bbe16592`: 1072
passed, 0 failed, 1 ignored, exit 0 (`outputs/workspace-suite-summary.txt` in the probes). The shape touches
no landed test. The one landed test in the surface it names is
`mock/crates/arvo-numeral/tests/a_consumer_crate.rs`, which the shape proposes to grow (`SHAPE.md:111-112`).
I read its body. It builds and runs `tests/consumer` from a scratch directory, and asserts both that the
build succeeds and that the crate directory is resolved at run time. It is not tautological.

The sketch's tests are its four consumers (one body, `shared/consumer_body.rs`) and five twins. I read
every body. None is tautological, and every twin refuses for the reason its doc comment states. Against
what `FINDINGS.md` claims for them:

- Claim 1 (the obligation's test does not separate the arms): established. Each consumer builds a generic
  function over `Sum + Order + Zero` and asserts concrete values, against `Num` and against `Tally`. Two
  weaknesses, neither disqualifying. The `Num` run, `best_running_sum([3, -5, 9, -1])`, never leaves the
  range, so the contract's completion is entered only through `Tally`. And `Tally` is built on `Num`
  (`consumer_body.rs:34`), so the outside instance is not independent of the implementation.
- Claim 2 (a generic `const fn` needs the consumer's gate): established for the declaration, by
  `const-const-bound-no-gate` and `opt-on-const-bound-no-gate`, against the control
  `const-const-bound-with-gate`. The two differ by the one `#![feature]` line, so that is a real control,
  but only for "the refusal is the missing gate". It is not a control for anything evaluated.
- Claim 3, "builds when one does": the positive half was never committed. No twin turns arm two's feature
  on, names the gate, and builds. I supplied it: `opt-on-gated-control` WORKS.
- Claim 4 (arm three "gives the gated consumer a constness it can rely on"): not established by the sketch.
  The control only declares `sum_in_const`. Nothing in the sketch evaluates `const impl Sum for Num` in a
  const context. `arm-const/src/lib.rs:45-46` says the function is "Read by the arm's own test", and
  `outputs/workspace-test.txt` shows `arm_const` running 0 tests. That comment is false. I supplied the
  evaluation: `gated-const-eval` WORKS, with 100 + 100 saturating to 127 in a const item, read back by a
  test.
- `outputs/consumer-alone.txt` is filtered to nine lines, not raw output, so the plain-path rebuild of
  `consumer-opt-off` is inferred from one "Compiling arm-opt" line rather than shown.

Verdict: the sketch's suite is honest and incomplete. Two of its four claims rested on controls it did not
commit. Both are now committed in the probes and both hold, so nothing has to be retracted. I proceeded.

## 1. "What the sketch settled", and the arm (`SHAPE.md:14-38`)

Holds: arm three is the arm to take. The shape's reason for it is wrong, and the shape leaves out the
consequence that matters most to every family built on it.

The shape's reason is that arm three gives "a gated consumer" a constness it can rely on
(`SHAPE.md:29-30`). A gated consumer is the case `obligation::the_unstable_machinery_does_not_reach_a_consumer`
exists to make unnecessary: "no feature gates of its own". Arguing for an arm by what it gives a consumer
the obligation says should not need a gate is arguing from the wrong population. The probes measure what
an ungated consumer gets under arm three:

- a concrete contract call at `Num` in a const item: refused, E0658, "cannot call conditionally-const method"
  (`concrete-const-call-no-gate`);
- a call, in a const item, to a generic `const fn` written in a gated crate (standing in for an arvo
  family): refused, E0658, "cannot call conditionally-const function" (`ungated-call-of-gated-generic`).
  The same call outside a const context builds and runs (`ungated-runtime-call-control`), so the refusal is
  the const context and nothing else.

So, to an ungated consumer, arms one and three are observationally the same. Build-time evaluation reaches
that consumer only through the inherent `const fn`s, and it does so in every arm. The shape says that in
passing (`SHAPE.md:22-23`). It needs to be a stated consequence, because every family from step 10 on is
written against these contracts. A family that consumers are meant to evaluate at build time cannot be
offered only as a generic `const fn` over `[const]` bounds. It needs a concrete `const fn` entry point per
instance, or the consumer has to name the gate, and the second is the obligation's failure.

The correct reason for arm three is `const-wherever-possible`, applied to arvo's own code. Arvo's families,
its const tables and its own generic computations are crates where the gate is allowed. Under arm one,
every generic `const fn` they would write over the contracts fails to build (the sketch's
`plain-generic-const-fn`, E0015). The rule's answer to a `const fn` that will not build is "take the vetted
feature gate, never drop the `const`". Under arm three such a function builds and evaluates (`gated-const-eval`).
The `const_trait_impl` that arm three puts in `arvo-numeral` unconditionally is the rule's prescribed cost.
And the obligation's wording, "whatever arvo uses internally", covers it, because a `const impl` in arvo
asks nothing of a consumer that implements or calls the trait outside a const context (the sketch's
`Tally`, and `ungated-runtime-call-control`).

Arm two is refused on the sketch's own evidence (`opt-off-const-bound-with-gate`): the constness of a
contract becomes a property of the whole build.

What I would put instead. Arm three, argued from `const-wherever-possible` over arvo's own generic
`const fn`s. Add the sentence that no ungated consumer evaluates anything at build time through a contract
under any arm, citing `concrete-const-call-no-gate` and `ungated-call-of-gated-generic`. And add the
obligation it creates for steps 10 and on: build-time entry points to families are concrete.

## 2. Where the contracts live (`SHAPE.md:40-60`)

Wrong in its reasoning, wrong in one of its dependencies, and it leaves out the real cost that decides the
question.

(a) The reason given for leaving `arvo-format` is false about the crate it describes. `SHAPE.md:44-48` says
a contract "is a statement about operations over values rather than about the set, and putting it there
would give the format crate a second subject". `arvo-format` already ships the operations. It exports `add`,
`mul`, `div` and `convert` (`mock/crates/arvo-format/src/lib.rs:77`, `:102-103`, `:116`), and its design
declares `proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`
(`mock/crates/arvo-format/DESIGN.md.tmpl:3-5`). Operations are already its subject. The deviation from
`arvo-bridge-home-rule` that the shape announces as its own call is justified by a premise the tree
refutes.

(b) The shape's own argument does not select a new crate. `SHAPE.md:58-60` sets a separate crate against "a
trait declared inside `arvo-numeral`". `arvo-format` is not `arvo-numeral` either. A consumer bounding on a
contract declared in `arvo-format` depends on no carrier, placement or rung, which is exactly the property
claimed. That passage also never names the row it quotes: the wording is
`obligation::the_surface_expressible_as_contracts_before_anything_implements_it`, which is not
`obligation::the_unstable_machinery_does_not_reach_a_consumer`.

(c) The new crate as specified breaks the shape's own sentence. `SHAPE.md:51-52` makes `arvo-contracts`
depend on `arvo-bits`. `SHAPE.md:60` says a consumer of `arvo-contracts` depends "on no carrier". `Bits<N>`
is a carrier, `repr(transparent)` over a `u64` (`mock/crates/arvo-bits/src/lib.rs:50-52`). Every consumer
of `Sum` would pull in the bit container.

(d) The reason that does decide the question is missing: the mutation order. Declaring the contracts in
`arvo-format` changes that crate's design, and a design change nukes the code under it first. That is all of
`arvo-format`, and everything that depends on it, which is every crate in the tree. A new crate adds a
design and nukes nothing. That is a real, rule-grounded reason to deviate from the letter of the
bridge-home rule ("the lowest crate ... the trait declaration lives in that crate"). The rule's purpose, as
its decision test states it, is to refuse inversions, and a crate directly above `arvo-format` and below
`arvo-numeral` inverts nothing. The rule's "what this rule is NOT" also warns against "put every bridge in
the same crate for consistency". So the deviation is defensible only on the mutation-order ground, not on
"second subject".

(e) The impls of the bit tier on `Bits<N>` violate rule clause 4 ("per-substrate-type impls in the
type-declaring crate"), and they launder a decision the bits design made. `mock/crates/arvo-bits/DESIGN.md.tmpl:26-34`
declines a width accessor as "a second name for one already in scope", because "nothing in this design
erases `N`". The shape ships that same width as `Breadth for Bits<N>` in another crate (`SHAPE.md:55-56`),
and it ships `and`, `or`, `not`, shifts and rotates on `Bits<N>` too, against the bits design's "the whole
public surface" (`DESIGN.md.tmpl:131`). A generic `B: BitAccess` is precisely what erases `N` from a caller's
scope, so the bits design's premise stops being true once this tier exists. Routing the change through
another crate's impls does not avoid the design change. It makes it without the round. The plan itself
named "the bit crate's design takes a round of its own first" as one of two routes. The shape took the
other route and called it changing no landed design.

What I would put instead. One new crate for the numeric tier, depending on `arvo-format` alone, with the
deviation from the bridge-home rule recorded and argued from the mutation order. The bit tier comes out of
this step. It goes either to the round on `arvo-bits`'s own design, so that the impls live in `arvo-bits`
by clause 4, or to step 10, where `obligation::set_operations_over_a_fixed_size_bit_set` actually names
what is wanted. That obligation asks for intersection, union and an emptiness test, each lowering to one
instruction, up to `Mask256`. The shape's `BitLogic` has no emptiness test, adds `not`, which nobody asked
for, and `Bits<N>` stops at 64 bits. Which side of the dependency the bit tier sits on is stack-coord's
call, and should be listed with the other three.

## 3. The contracts (`SHAPE.md:62-97`)

Overall: the section claims each trait's law is "the oracle equality", meaning the slot of the result is
the slot the format's function gives (`SHAPE.md:64-66`). No contract in the table exposes a slot or a
format. So that law can be stated only for `Num`, by its inherent `slot()`, and not over the contract,
which is what a generic suite and an outside instance need. A contract whose law is not expressible in its
own vocabulary is not "checkable before an implementation of it exists". The suite needs a second
vocabulary per instance: its coordinates (W, F, signedness, policy, rounding), so it can pick a verdict
from the law rows, and an enumeration of its members, so it can be exhaustive. That belongs to the test
side. Each instance supplies it: `Num` from `S` (`Sole::WIDTH`, `Sole::POLICY`, `Sole::SIGNED`, the
quantum exponent), and the outside instance by declaration. The shape has to name it, or "generic over each
contract" does not mean anything.

Per contract:

- `Sum`, `Difference`, `Product`, "total, under the instance's own completion". This is false for the impl
  the sketch writes, which covers every `C: DebugCheck`. Under `Abort`, the default, a debug build panics on
  a pair whose sum leaves the range (`abort-oracle`, 3 tests: `Quiet` gives 127, `Abort` panics, an
  in-range `Abort` sum returns). The panic is ratified (`ruling::the_overflow_panic_is_permitted_and_bounded`),
  so the fix is to the contract's statement, not to the value. The law holds under `Quiet`, or in release,
  and under `Abort` in debug the call aborts exactly where the exact result is not a member. The surface
  also omits what makes `Abort` usable generically: `Num` exposes `fits_add`, `fits_sub`, `fits_mul` and
  `fits_neg`, each returning `Bool` (`mock/crates/arvo-numeral/src/num.rs`), so a caller can ask ahead. Over
  `T: Sum`, a generic caller has no way to stay inside the debug contract. Add a `fits` member to each
  operation contract, and state the law per `C`.
- `Order`. It holds as a surface. It owes the sentence `law::existence_of_a_translation_invariant_total_order`
  says the canon owes: on a wrapped numeral the comparison is the representative order, which arithmetic
  does not respect.
- `Identity<M>`. It holds. Its reach is `Self` only.
- `Sign`. It holds as a surface. For an unsigned instance, `is_negative` is constantly false, which is a law
  to assert, not a note.
- `Magnitude`. Incomplete. Under wrap the one member whose negation leaves the range is its own magnitude,
  and it is negative, so "magnitude is not negative" fails at `MIN` under wrap and holds under saturation.
  The law has to carry that region.
- `Constant<const V: i128>`, "refused at build where `V` is none". As a statement this is true and as a
  bound it is empty. The impl has to cover every `V`, so `T: Constant<300>` is satisfied at an 8-bit type,
  `cargo check` is green, and the refusal arrives only at monomorphisation, as E0080 from
  `Num::constant`'s assertion (`constant-bound-check`). It is also instance-relative: the member at slot
  index `V` is the value `V` only at F = 0. At a fixed point, `Constant<1>` is one quantum. A generic
  family that wants the number two cannot get it from this contract, and the contract's law ("the slot is
  `V`") needs a slot accessor the surface does not have. Rename it for what it is (a slot member), or state
  it over value. Either way, say that its refusal is post-monomorphisation, as `Bits`'s own design says of
  `ADMITTED`. On the bare `i128`: `adt-param-consumer` shows an ungated consumer writing
  `T: Constant<{ Index::at(3) }>` over a private-field stack type, so the bare integer is an ergonomic choice
  and not forced. Op's exception for const generics is bounded by "only when truly painful otherwise"
  (quoted in `obligation::a_primitive_for_every_position_a_bare_number_would_take`). The shape has to make
  that ergonomic case explicitly, not lean on the exception as if it were unconditional.
- `MonotoneSum`. See section 4.
- `Breadth`, `BitAccess`, `BitLogic`, `BitSequence`. Out of this step (section 2e).
- `Reciprocal`, `SquareRoot`, "named, with no instance until division and floats". Division has landed:
  `Num::div(self, by: Divisor<S::Format>)` (`mock/crates/arvo-numeral/src/ratio.rs`), with `Divisor` "not
  zero by its type" (`mock/crates/arvo-format/src/division.rs:70-78`). `reciprocal(self) -> Self` takes any
  member, zero included. So it needs a run-time zero check, which `ruling::never_a_runtime_check_and_one_lowered_path`
  forbids, or a completion of 1/0 that nobody has declared. The signature contradicts the landed design of
  the operation it reciprocates. If it stays, it takes the divisor type. It also has no stated law, and a
  contract with no law and no instance is a name, not a checkable shape. Drop both from this step, or give
  each a law and a signature that cannot meet zero.

The paragraph on `Identity<M>` moving (`SHAPE.md:92-97`) holds as reasoning. It misses that implementing
the contracts for `Num` inside `arvo-numeral` (`SHAPE.md:53-54`) is already a change to `arvo-numeral`'s
design, with a new dependency and new impls, whether or not `Identity` moves. Section 6 picks this up.

## 4. Instances (`SHAPE.md:99-112`)

`MonotoneSum` keyed on `Overflow = Saturate` is a soundness bug in the selection. `Sole` admits a policy by
its declared value: `matches!(<OverflowOf<S> as Overflow>::POLICY, Policy::Wrap | Policy::Saturate)`
(`mock/crates/arvo-numeral/src/admit.rs`, in `ADMITTED`). `Overflow` is open by design
(`mock/crates/arvo-format/src/overflow.rs:24-26`). `monotone-saturate-control` builds a policy `Pinned` of
another crate with `POLICY = Policy::Saturate`, and shows its sums equal `Saturate`'s at all 65,536 pairs at
W = 8. `monotone-outside-policy` hands that value to a function bounding on the shape's marker, and is
refused, E0271, "expected `Saturate`, found `Pinned`". So a value that saturates is denied the marker,
because the selection is by type and the admission is by value.
`ruling::the_predicate_is_whatever_is_available_at_const_time` puts the axis at const-available, not at
type-identity, which is what the shape's selection silently narrows it to.

What I would put instead (one instance, `monotone-policy-marker`, so a hypothesis to test in the round). The
property belongs to the completion, so the marker sits on the policy: a `MonotoneCompletion: Overflow`
trait that `Saturate` implements and an outside policy opts into, with
`impl MonotoneSum for Num<S, C> where OverflowOf<S>: MonotoneCompletion`. Both `Saturate` and `Pinned` reach
it (WORKS), and `Wrap` is refused, E0277 (`monotone-policy-marker-wrap`). The generic suite, run against
each marked instance, is what checks that an opted-in policy told the truth.

The outside instance is in the wrong place for the obligation it is supposed to serve. `SHAPE.md:111-112`
puts it under `arvo-numeral/tests/consumer`, depending on `arvo-contracts` and `arvo-numeral`, and the
sketch's `Tally` is built on `Num`. The obligation `the_surface_expressible_as_contracts_before_anything_implements_it`
is about a consumer building against the shape before an implementation exists. That needs a consumer that
depends on the contracts crate, and `arvo-format` for `Bool` if needed, and on no implementing crate, with
an instance that uses no `Num`. Keep the numeral consumer for the "reaches every public item" test, and add
that one under the contracts crate.

The deliberately wrong outside instance (`SHAPE.md:116-117`) has to be one mutant per law. A single wrong
instance that fails one law shows nothing about the others. The value crate already uses this pattern, one
`arvo_mutant_*` cfg per operation (`mock/crates/arvo-numeral/Cargo.toml`, `[lints.rust]`).

## 5. The law suite and its regions (`SHAPE.md:114-125`)

Three of the four bullets are wrong as written, and the fourth cites no row. I measured them exhaustively
on the landed `Num` (`law-regions`, 240 cells). The instrument reproduces the registry's own counts exactly
where they overlap: 952 and 4,177,792 for signed saturating addition at W = 4 and 8, and 28, 160, 780 and
3516 for signed saturating multiplication at W = 3 to 6.

- Additive associativity. Under wrap it holds at every cell, W 1 to 8 and every F, which agrees with the
  row and with the ratified scale-blindness. "Fails under signed saturation" (`SHAPE.md:120`) is false at
  W = 1, where it holds at both F. The row's fails region is W = 4 alone. The shape also drops the row's
  `gap`, the recovery under a declared non-negative operand window, which has a row of its own:
  `law::additive_associativity_recovered_under_declared_nonnegative_operand_window`.
- Multiplicative associativity under wrap, "holds at a zero fraction width only and fails above it"
  (`SHAPE.md:121`). Holds at F = 0 everywhere: right. "Fails above it" is false at (W, F) = (1, 1) in every
  combination of sign and rounding, at (2, 2) signed under both roundings, and at (3, 3) signed under
  `TowardZero`. The last cell lies inside the row's own fails region (W in 3..=7, F > 0, both
  signednesses), so the row is wrong there too, because it carries no rounding axis and the verdict at
  (3, 3) signed depends on rounding (holds under `TowardZero`, 8 failures under `HalfEven`). That is a
  registry finding, not mine to edit. The registry owner should take it.
- Signed saturating multiplication, "no semigroup at any width" (`SHAPE.md:122`). False at W = 1, and at
  (2, 2) and (3, 3) under `TowardZero`, where the induced operation is associative. The claim was drawn from
  a retirement: retiring "induces a semigroup at some width" does not license asserting the negation at
  every width, and the retirement's own `why` says "closed exhaustively at four widths". The row that
  states the region is `law::multiplicative_associativity_under_saturation` (fails: W in 3..=6, F = 0,
  signed). The shape does not cite it. Its holds side, unsigned at F = 0, is also missing from the suite.
- Monotonicity of addition (`SHAPE.md:123`). Measured: it holds in every saturating cell and fails in every
  wrapping cell, W = 1 included. It is the law the marker rests on, and no law row states it.
  `law::existence_of_a_translation_invariant_total_order` is about the existence of some compatible order.
  It says saturating addition "admits some". It does not say the slot order is that order. So this bullet
  is from intuition, the very thing `SHAPE.md:118` forbids. It happens to be right, at one instrument. It
  needs a row before a marker is gated on it.
- "Asserted exhaustively at widths 1 to 8" goes past every row's region (W in {4, 8}, 3..=8, 3..=7). That is
  fine, provided the suite reports what it finds outside a row's region as new evidence and never asserts a
  verdict there. Otherwise the suite is red against a correct value. `law-regions` shows it would be, at
  W = 1 and at F = W.

What I would put instead. The suite asserts `holds` only inside a row's `holds` and `fails` only inside its
`fails`. Outside both it records the measured verdict with no assertion, and feeds it back as a candidate
row. Each bullet cites the row that carries its region, never a retirement. Add the missing rows to the
suite's list (`multiplicative_associativity_under_saturation`, the two distributivity rows, the three
operand-window recoveries). And open a registry note against `law::multiplicative_associativity_under_wrapping`
for the rounding-dependent cell at (3, 3) signed.

## 6. The calls it reserves (`SHAPE.md:127-132`)

The three listed calls are the right kind of call. The list is incomplete:

1. Arm three, with `const_trait_impl` unconditional in `arvo-numeral`. It holds as a reserved call. Its
   argument needs replacing, as in section 1.
2. `Identity<M>` moving. It holds.
3. The crate's name. It holds.

Missing:

4. The contracts' impls for `Num` in `arvo-numeral`, with the new dependency. That is a design change to
   `arvo-numeral` independent of item 2.
5. The deviation from `arvo-bridge-home-rule`'s "lowest crate", argued from the mutation order (section 2d).
   It is a workspace-rule deviation, and the agent that wrote the shape may not grant it to itself.
6. The bit tier's home, which is `arvo-bits`'s design question that the plan already reserved (section 2e).

## 7. Dead routes and what I tried

- Running the gated generic through an ungated const item, to rescue the shape's constness claim for
  obligation-compliant consumers: refused, E0658 (`ungated-call-of-gated-generic`). The runtime control
  builds. No route found.
- A concrete contract call in an ungated const item: refused, E0658 (`concrete-const-call-no-gate`).
- Building on the sketch's `arm-const` two path hops away: cargo would not resolve its inherited
  `workspace.package` fields. I fixed this with a byte-identical copy (`arm-const-copy`, diff exit 0).
- The shape's `MonotoneSum` selection against an outside saturating policy: refused, E0271. Replaced with a
  policy-side marker. That WORKS, and its `Wrap` twin is refused, E0277.
- `Constant<V>` with a range-restricted impl: not expressible without `generic_const_exprs`, so the refusal
  can only be post-monomorphisation (`constant-bound-check`: check green, build E0080).
- Replacing the bare `i128` const parameter with a stack type: WORKS with no consumer gate
  (`adt-param-consumer`).
- The law regions, read off rows and retirements: three of the four sentences refuted by the exhaustive
  sweep, and one row (`multiplicative_associativity_under_wrapping`) refuted at one cell.
- Not tried, and out of scope: disassembly of the generic sum against the concrete one, which the plan
  assigns to the round as an ad hoc reading. Nothing here is a bench. None of these figures are timings.

## Summary of changes to the shape

- The arm: keep arm three, argue it from `const-wherever-possible` over arvo's own generic `const fn`s, and
  state that no ungated consumer reaches build-time evaluation through a contract under any arm.
- Placement: a new crate for the numeric tier on `arvo-format` alone, justified by the mutation order and
  recorded as a bridge-home deviation. The "second subject" reason is refuted.
- The bit tier: out of this step, to `arvo-bits`'s own round or to step 10, with impls in `arvo-bits` by
  clause 4, and with the emptiness test the bit-set obligation names.
- The oracle-equality law: add a test-side coordinate and enumeration vocabulary per instance, so the
  generic suite is expressible.
- `Sum`, `Difference`, `Product`: state the law per debug check, and add the `fits` members.
- `Magnitude`: the wrap `MIN` region. `Order`: the representative-order sentence.
- `Constant<V>`: say that its refusal is post-monomorphisation, and that it is a slot member and not a
  value, and argue the bare `i128` as ergonomics.
- `Reciprocal`, `SquareRoot`: out, or typed so that zero cannot be met, each with a law.
- `MonotoneSum`: select it by a policy-side marker, not by equality to `Saturate`, and get a law row for the
  monotonicity before gating on it.
- The outside instance: one under the contracts crate with no implementing dependency, and one mutant per
  law.
- The law regions: assert only inside a row's region, cite rows rather than retirements, add the missing
  rows, and report the W = 1 and F = W cells and the one row defect to the registry.
- Reserved calls: add the `Num` impls in `arvo-numeral`, the bridge-home deviation, and the bit tier's home.

A second read is owed on the canon-gate classification in section 0, and on the placement call in section 2.
