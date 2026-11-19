# The shape of the numeral contracts

This is the shape proposed for arvo's step 5, the contracts a consumer bounds on, written after the
three-arm sketch at `mock/research/sketches/202611192015_numeral-contracts/` and before any design
round. It is read by one expert seat, then sent to stack-coord, and only then does the design round
open. Nothing here is canon. The canon it is bound by is the registry, queried row by row below.

The plan (`mock/research/202611130950_canon-double-check/PLAN.md`, step 5) recommends one contracts
crate below the value crate, each trait placed by the lowest crate that reaches its types, with the
impls for `Bits<N>` held in the contracts crate so the landed design of `arvo-bits`, which depends on
nothing in this workspace, does not change. This document takes that recommendation as its starting
point, says where it holds and where it does not, and names the calls that are not the builder's.

## What the sketch settled and what it left

All three arms pass the one test `obligation::the_unstable_machinery_does_not_reach_a_consumer` fixes:
a consumer crate with no feature line builds and runs a function generic over a numeral that adds,
orders and has an additive identity, against `Num` and against an instance of its own. So the
obligation does not choose the arm. Three facts from the sketch bear on the choice instead.

A generic `const fn` over the contracts is open only under a const trait, and only to a consumer that
names `const_trait_impl` itself. Under plain traits it is closed to everyone. A site that names its
signature computes at build time in every arm, through the value's inherent `const fn`s.

Under arm two, a const trait behind an opt-in feature, the constness of a contract becomes a property
of the whole build: a gated consumer leaning on `[const] Sum` breaks when no crate in the build turns
the feature on. A plain-bounding consumer is unaffected either way.

Under arm three, const traits alone, the gate sits unconditionally in the contracts crate, a gated
consumer gets a constness it can rely on, and no consumer's source pays for it. For `Num` to satisfy a
`[const]` bound, its impls are `const impl`, which puts `const_trait_impl` into `arvo-numeral`
unconditionally too. Today that crate takes the gate only under `hlist-const`.

The workspace rule `const-wherever-possible` reads: where a `const fn` will not build, take the vetted
feature gate, never drop the `const`. `const_trait_impl` is on the allowed list of `unstable-features`.
The proposal follows that rule and takes arm three. The cost it moves into `arvo-numeral`'s crate
attributes is a change to an existing item of a crate another lane overhauls, which is stack-coord's
to agree before it is written.

## Where the contracts live

The types the contract surface reaches are `Self`, `core::cmp::Ordering`, and from `arvo-format`
`Bool` and `Width`. By `arvo-bridge-home-rule` the lowest crate reaching all of them is `arvo-format`
itself. The proposal does not put the contracts there, and says why: `arvo-format` declares what
values exist, the representable set, the rounding modes, the overflow policies and the map that
applies one, and is the crate that introduces the numeric category. A contract that a value satisfies
is a statement about operations over values rather than about the set, and putting it there would give
the format crate a second subject. The rule's own question is reach, and one crate directly above
`arvo-format` reaches the same types.

So: one new crate, `arvo-contracts` (the name is the round's to settle), depending on `arvo-format`
for `Bool` and `Width` and on `arvo-bits` for the impls of the bit tier on `Bits<N>`. `arvo-numeral`
depends on it and holds the impls for `Num`, the type-declaring crate by rule four of the bridge-home
rule. `arvo-bits` is not changed, so its design's sentence that it depends on nothing in this
workspace keeps standing, and the width contract over `Bits<N>` is the contracts crate's impl rather
than the accessor that design declined.

The obligation's own wording, that a consumer can build against the shape while the implementation
behind it is unfinished, is what a separate crate gives and a trait declared inside `arvo-numeral`
does not: a consumer depends on `arvo-contracts` alone and on no carrier, placement or rung.

## The contracts

Each is a trait whose law is the oracle equality: for `Num`, the slot of the result is the slot the
format's own function gives for the operands' slots (`arvo_format::add`, `sub`, `mul`, `neg`), which is
what `arvo-numeral`'s rustdoc already states for the inherent methods.

| Contract | Surface | Notes |
|---|---|---|
| `Sum`, `Difference`, `Product` | `sum`, `difference`, `product`, each `(self, Self) -> Self` | total, under the instance's own completion |
| `Order` | `order(self, Self) -> Ordering`, with provided `lt`, `le`, `gt`, `ge` returning `Bool` and `min`, `max` | a total order |
| `Identity<M>`, `Additive`, `Multiplicative` | `const IDENTITY: Self` | already in `arvo-numeral`; see below |
| `Sign` | `is_negative`, `is_zero`, `is_positive`, each `-> Bool` | an unsigned instance answers `is_negative` false for every member |
| `Magnitude` | `magnitude(self) -> Self` | the absolute value, under completion at the one member whose negation leaves the range |
| `Constant<const V: i128>` | `const MEMBER: Self` | the member at slot index `V`, refused at build where `V` is none; a compile-time door only |
| `MonotoneSum` | marker, `Sum + Order` | `a <= b` implies `a + c <= b + c`; see the law below |
| `Breadth` | `const WIDTH: Width` | the bit tier's width |
| `BitAccess` | `bit<const I: u32>(self) -> Bool`, `with_bit<const I: u32>(self, Bool) -> Self` | the index a const generic, refused at build past the width, so no index is checked at run time |
| `BitLogic` | `and`, `or`, `xor`, `not` | |
| `BitSequence` | `shift_up<const K: u32>`, `shift_down<const K: u32>`, `rotate_up<const K: u32>`, `rotate_down<const K: u32>` | the amount a const generic for the same reason |
| `Reciprocal`, `SquareRoot` | `reciprocal(self) -> Self`, `square_root(self) -> Self` | named, with no instance until division and floats |

Construction is `Constant<V>`, an associated constant, which needs no const trait and validates
nothing at run time, so it stays on the compile-time side of
`question::what_the_ingest_row_claims_and_what_turns_on_it`, which is open and the panel's.

The associated items are typed in the stack's own types, `Bool`, `Width` and `Self`, with the bare
integer only at the const generic parameter, the one excepted position
(`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`). The
count of types the door carries is that ruling's open count, and this surface adds no type to it.

`Identity<M>` already exists in `arvo-numeral/src/identity.rs`, with `Additive` and `Multiplicative`.
It is a contract whose only reach is `Self`, so by the same placement it belongs in `arvo-contracts`,
and `arvo-numeral` keeps its two impls. Moving it is a change to an existing item of `arvo-numeral`,
which the brief reserves for stack-coord's agreement. The alternative is that `arvo-contracts` declares
no identity and the consumer bounds on `arvo_numeral::Identity`, which puts the one identity contract
inside the implementing crate, against the obligation's wording above.

## Instances

`Num<S, C>` implements the numeric tier, in `arvo-numeral`. `MonotoneSum` is implemented only where
the signature's adaptation names `Saturate` as its overflow (`<S::Adaptation as Adaptation>::Overflow`),
because `law::existence_of_a_translation_invariant_total_order` states a fails region of every width,
both signednesses and the wrap policy: a finite cyclic group admits no translation-invariant total
order. So a wrapping `Num` does not carry the marker, and a function bounding on it refuses a
wrapping value at build. That refusal is a compile-fail test.

`Bits<N>` implements the bit tier, in `arvo-contracts`, through `Bits::masked` and `Bits::raw`. It
implements no numeric contract, since its own design says it has no arithmetic.

The outside instance lives in the consumer crate under `arvo-numeral/tests/consumer`, which depends on
`arvo-contracts` and `arvo-numeral` with no feature line and reaches every new public item.

## The law suite and its regions

The suite is generic over each contract and runs against `Num` and against a deliberately wrong
outside instance that must fail it. Its algebraic laws take their regions from the law rows, never
from intuition about rings:

- additive associativity holds under wrap at any fraction width (`law::additive_associativity_under_wrapping`), holds under unsigned saturation, and fails under signed saturation (`law::additive_associativity_under_saturation`);
- multiplicative associativity under wrap holds at a zero fraction width only and fails above it (`law::multiplicative_associativity_under_wrapping`), which `retirement::dl_wraps_induced_ring_transfers_to_fractional_formats` and `retirement::dl_the_wrap_section_is_a_relabelling_at_every_scale` retire the wider claims of;
- signed two's-complement saturating multiplication is asserted as no semigroup at any width (`retirement::dl_signed_twos_complement_saturating_multiplication_induces_a_semigroup_at_some_width`);
- monotonicity of addition is asserted under saturation and its failure under wrap.

The oracle equality itself is total and is asserted exhaustively at widths 1 to 8 through the value.

## What is not the builder's to settle

1. Arm three, with `const_trait_impl` unconditional in `arvo-numeral`: an existing item of a crate the
   `arvo-numeral-doors` lane overhauls.
2. `Identity<M>` moving from `arvo-numeral` to `arvo-contracts`: an existing item of the same crate.
3. The crate's name.
