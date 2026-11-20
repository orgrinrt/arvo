# The shape of the numeral contracts

This is the shape proposed for arvo's step 5, the contracts a consumer bounds on, written after the
three-arm sketch at `mock/research/sketches/202611192015_numeral-contracts/` and before any design
round. One expert seat has read it against probes of its own (`READING.md` beside this file, probes at
`mock/research/sketches/202611192130_numeral-contracts-reading/`), and this revision folds that reading
in. It goes to stack-coord next, and only then does the design round open. Nothing here is canon. The
canon it is bound by is the registry, queried row by row below.

The plan (`mock/research/202611130950_canon-double-check/PLAN.md`, step 5) recommends one contracts
crate below the value crate, each trait placed by the lowest crate that reaches its types, with the
impls for `Bits<N>` held in the contracts crate so the landed design of `arvo-bits` does not change. The
reading shows that last part does change that design, through another crate's impls, so this revision
takes the bit tier out of the step and names where it could go instead.

## What the sketch settled and what it left

All three arms pass the one test `obligation::the_unstable_machinery_does_not_reach_a_consumer` fixes:
a consumer crate with no feature line builds and runs a function generic over a numeral that adds,
orders and has an additive identity, against `Num` and against an instance of its own. So the
obligation does not choose the arm. The reading committed the two controls the sketch left implicit
(`opt-on-gated-control`, `gated-const-eval`), and both hold.

The reading also probed what a consumer with no gate gets at build time. Through a contract, under any
arm, it gets nothing: a concrete call in a const context and a call to a gated generic `const fn` both
refuse with E0658, and the runtime call builds. A consumer that wants a value at build time reaches it
through the value's own inherent `const fn`s, or through a concrete entry point the stack provides. So
the arm decides what arvo's own generic code can do at build time, and nothing a plain consumer can do.

The arm proposed is the third, const traits alone, on the ground of `const-wherever-possible` applied to
arvo's own generic `const fn`s over the contracts: under plain traits none can exist, and under the
second arm their constness depends on whether some crate in the build turns a feature on. The cost is
that `const_trait_impl` becomes unconditional in `arvo-numeral` for `Num`'s `const impl`s, where today
that crate takes the gate only under `hlist-const`. The gate is on the allowed list of
`unstable-features`. It is a change to an existing item of a crate another lane overhauls, so it is
stack-coord's to agree.

Build-time entry points of the families that come later are concrete, for the reason above.

## Where the contracts live

The numeric tier's surface reaches `Self`, `core::cmp::Ordering`, and from `arvo-format` `Bool` and
`Width`. By `arvo-bridge-home-rule` the lowest crate reaching them is `arvo-format`, and the rule says
the declaration lives there. This shape proposes a new crate directly above `arvo-format` instead, and
records that as a deviation from the letter of the rule. The ground is the mutation order: declaring the
contracts in `arvo-format` changes that crate's design, and a design change nukes the code beneath it
first, which is `arvo-format` and every crate depending on it, the whole tree. A new crate adds a design
and nukes nothing. The rule's purpose, refusing inversions, is kept: a crate between `arvo-format` and
`arvo-numeral` inverts nothing.

The earlier draft argued the placement from `arvo-format` gaining a second subject. That is withdrawn:
`arvo-format` already exports `add`, `mul`, `div` and `convert` (`mock/crates/arvo-format/src/lib.rs`).

So: one new crate, `arvo-contracts` (the name is the round's to settle), depending on `arvo-format`
alone. A consumer bounding on a contract depends on it and on no carrier, placement or rung, which is
the wording of `obligation::the_surface_expressible_as_contracts_before_anything_implements_it`.
`arvo-numeral` depends on it and holds the impls for `Num`, the type-declaring crate by clause four of
the rule.

## The bit tier is not in this step

The brief lists the bit tier's width, access, logic and sequence, with the `Bits<N>` impls held in the
contracts crate. The reading gives three reasons that does not stand as written. The impls on
`Bits<N>` in another crate break clause four of the bridge-home rule. `Breadth for Bits<N>` ships the
width accessor `mock/crates/arvo-bits/DESIGN.md.tmpl` declines, and the logic and sequence tiers widen
what that design calls its whole public surface, so the bits design changes without its round. And
depending on `arvo-bits` would put a carrier, `repr(transparent)` over a `u64`, under every consumer of
`Sum`.

Two homes are open: a round on `arvo-bits`'s own design, so the impls live there by clause four, or
step 10, where `obligation::set_operations_over_a_fixed_size_bit_set` names intersection, union and an
emptiness test up to `Mask256`. The surface sketched earlier (`Breadth`, `BitAccess`, `BitLogic`,
`BitSequence`) lacks the emptiness test that obligation asks for. Which home is stack-coord's call.

## The contracts

Each numeric contract's law is the oracle equality: for `Num`, the slot of the result is the slot the
format's own function gives for the operands' slots. No contract exposes a slot or a format, so a
generic suite cannot state that law over the contract alone. Each instance supplies a test-side
vocabulary instead, its coordinates and an enumeration of its members, and the suite states the law
through it.

| Contract | Surface | Notes |
|---|---|---|
| `Sum`, `Difference`, `Product` | `sum`, `difference`, `product`, each `(self, Self) -> Self`, with `fits_*` predicates returning `Bool` | the law is stated per debug check: under `Abort` in a debug build an operation whose exact result leaves the range aborts, so "total" holds only under the quiet checks |
| `Order` | `order(self, Self) -> Ordering`, provided `lt`, `le`, `gt`, `ge` returning `Bool`, and `min`, `max` | owes the representative-order sentence `law::existence_of_a_translation_invariant_total_order` says the canon lacks |
| `Identity<M>`, `Additive`, `Multiplicative` | `const IDENTITY: Self` | already in `arvo-numeral`; see below |
| `Sign` | `is_negative`, `is_zero`, `is_positive`, each `-> Bool` | an unsigned instance answers `is_negative` false for every member |
| `Magnitude` | `magnitude(self) -> Self` | states the wrap region at the least member, where the magnitude is negative |
| `Constant<V>` | `const MEMBER: Self` | a compile-time door only; see below |
| `MonotoneSum` | marker over `Sum + Order` | selected by a marker on the overflow policy; see below |
| `Reciprocal`, `SquareRoot` | named with no instance | `Reciprocal` takes an operand typed so zero cannot be met, as division's `Divisor` does |

`Constant<V>` refuses a `V` that names no member only after monomorphisation, as E0080 at build, and
`cargo check` stays green, so the bound itself carries no information a caller can rely on early. `V`
names a slot rather than a value. A stack-typed const parameter builds with no consumer gate
(`adt-param-consumer`), so the bare integer at that position is not forced, and the round takes the
stack type where it builds. Construction stays on the compile-time side of
`question::what_the_ingest_row_claims_and_what_turns_on_it`, which is open and the panel's.

`Reciprocal(self) -> Self` over every member would need a run-time zero check, against
`ruling::never_a_runtime_check_and_one_lowered_path`, or a completion nobody has declared. Typing its
operand as nonzero is what the landed division does.

`MonotoneSum` is not selected by equality of the overflow policy to `Saturate`. That form refuses an
outside policy that saturates (E0271, `monotone-outside-policy`). A marker on the policy side,
implemented by `Saturate` and open to an outside policy, builds, and its `Wrap` twin is refused with
E0277 (`monotone-policy-marker`, `monotone-policy-marker-wrap`). That is one instance of proof. The
refusal under wrap rests on `law::existence_of_a_translation_invariant_total_order`, which fails for
every width and both signednesses under wrap. Monotonicity of saturating addition has no law row of
its own yet.

Associated items are typed in the stack's own types, `Bool`, `Width`, `Ordering` and `Self`
(`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`), and this
surface adds no type to that ruling's open count.

`Identity<M>` already exists in `arvo-numeral/src/identity.rs`, with `Additive` and `Multiplicative`.
Its only reach is `Self`, so by the same placement it belongs in `arvo-contracts`, with `arvo-numeral`
keeping its two impls. Moving it is a change to an existing item of `arvo-numeral`. The alternative is
that the consumer bounds on `arvo_numeral::Identity`, which puts the one identity contract inside the
implementing crate.

## Instances

`Num<S, C>` implements the numeric tier, in `arvo-numeral`, and carries `MonotoneSum` where its
signature's overflow policy carries the monotone marker. A function bounding on `MonotoneSum` refuses a
wrapping value at build, and that refusal is a compile-fail test.

The outside instance lives in the consumer crate under `arvo-numeral/tests/consumer`. It depends on
`arvo-contracts` with no feature line, implements the contracts for a type of its own that uses no
`Num`, and reaches every new public item. The suite carries one deliberately wrong instance per law,
each of which must fail exactly that law.

## The law suite and its regions

The suite asserts a law only inside the region a law row states for it, and records what it measures
outside that region as a measurement rather than an assertion. `holds` and `fails` are not
complements, as the `law.toml` header says. The reading's exhaustive sweep
(`mock/research/sketches/202611192130_numeral-contracts-reading/outputs/law-regions.txt`) reproduces
the registry's own counts and shows where the rows stop:

1. Additive associativity under wrap holds at any fraction width (`law::additive_associativity_under_wrapping`).
   Under saturation it holds unsigned at `W in {4, 8}` and fails signed at `W = 4`
   (`law::additive_associativity_under_saturation`). Signed saturating addition holds at `W = 1`,
   as `ruling::the_additive_and_absorption_verdicts_are_canon` decides, so the suite asserts no
   failure there.
2. Multiplicative associativity under wrap holds at `F = 0`, `W in 3..=8`
   (`law::multiplicative_associativity_under_wrapping`). Its fails region, `F > 0`, is contradicted by
   the sweep at `W = 3, F = 3`, signed, `TowardZero`, where it holds. The row has no rounding axis.
   That is a registry defect, ticket `c7dfe82db2d5`, and the suite asserts the failure only in the
   cells measured to fail.
3. Multiplicative associativity under saturation holds unsigned at `F = 0`, `W in 3..=8`, and fails
   signed at `F = 0`, `W in 3..=6`, arity three (`law::multiplicative_associativity_under_saturation`).
   The sweep also measures it holding at `W = 1` and at `F = W` under `TowardZero`, outside the row.
4. Monotonicity of addition fails under wrap at every width
   (`law::existence_of_a_translation_invariant_total_order`). Under saturation it is measured, and
   asserted only once a row states its region.

The oracle equality is total and is asserted exhaustively at widths 1 to 8 through each instance's
test-side vocabulary.

## What is not the builder's to settle

1. Arm three, with `const_trait_impl` unconditional in `arvo-numeral`.
2. `Identity<M>` moving from `arvo-numeral` to `arvo-contracts`.
3. The `Num` impls in `arvo-numeral`, which are a change to that crate's design on their own.
4. The deviation from `arvo-bridge-home-rule`, argued from the mutation order. One expert seat has read
   it, so it does not yet have the two independent agreements a call about what the canon permits
   needs.
5. The bit tier's home: a round on `arvo-bits`, or step 10, against the brief's placement of it here.
6. The crate's name.
