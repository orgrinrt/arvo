# Prior art: algebraic structure hierarchies (identity, combine, and what sits above them)

**Date:** 2026-07-28
**Kind:** research, not design. Nothing here decides anything.

Every language that has tried to give its numeric types a real algebraic hierarchy has run into
the same three problems in some order: a type is a monoid in more than one way and the type system
has to pick which way without the caller repeating themselves at every call site; the identity and
the combine want to live on different traits but the traits want to compose without duplicating the
operation's identity in every downstream structure; and the moment the hierarchy climbs past monoid
toward ring and field, half the concrete types the library actually ships stop satisfying the laws
exactly, so either the hierarchy stops climbing or the laws stop being laws. arvo has already made
the first decision (`arvo-strategy/src/identity.rs:21`, `Identity<Op>` parameterised by a
zero-sized operation marker rather than a newtype wrapper per operation) and the design round that
prompted this pass has already named the second (`Combine<Op>` is the missing sibling, and
`Monoid<Op>: Identity<Op> + Combine<Op>` is a supertrait line once it exists). What follows is what
the field has learned about both decisions, and about the third one, which arvo has not yet made.

## The newtype-wrapper era, and why it does not survive contact with fixed-width numerics

Haskell's `Data.Monoid` is the origin of the newtype-wrapper answer to "a type is a monoid more
than once." `Sum Int` and `Product Int` are distinct types wrapping the same `Int`, each carrying
its own `Semigroup` and `Monoid` instance, so `mconcat` can be told which operation to fold with by
the wrapper the caller chose rather than by an operation parameter on the class itself. Scala's
Cats inherited the identical shape: `cats.kernel.instances`'s `Sum`, `Product`, `All`, and `Any` are
newtypes over numeric or boolean types, each supplying its own `Monoid` instance so that combining
values under addition and combining the same values under multiplication are two different type
class dictionaries rather than one dictionary parameterised by which operation is meant (Typelevel,
*Monoid*, `https://typelevel.org/cats/typeclasses/monoid.html`). NumHask, the actively maintained
2020s successor to Haskell's `numeric-prelude` effort, keeps the same answer under a different name:
it ships a `Wrapped` type specifically so `deriving via` can attach a different instance to the same
underlying representation without hand-written boilerplate for every wrapper (tonyday567/numhask,
`https://github.com/tonyday567/numhask`). Twitter's Algebird, built to give MapReduce aggregation a
principled combiner, does the same for its approximate structures: `HyperLogLogMonoid`,
`BloomFilterMonoid`, and `CountMinSketchMonoid` are each their own type rather than one sketch type
parameterised by which merge operation applies (twitter/algebird,
`https://github.com/twitter/algebird`).

The wrapper answer works when the operation only ever needs one identity and one combine per
wrapped type, and when allocating a new nominal type per operation is free or nearly free. Neither
holds for arvo. A `UFixed<I, F, S>` value already carries a strategy marker, a width, and a
signedness in its type; wrapping it again in `Additive<UFixed<I, F, S>>` to select which identity
applies duplicates the const-generic parameter list on a second type, and every function that wants
to be generic over "any monoid, tell me which operation" now has to be generic over the wrapper
family rather than over the value's own type. Rust has no `deriving via`, so NumHask's escape hatch
from writing the boilerplate by hand does not transfer. The wrapper answer is also silent on arvo's
actual problem, which is not "which operation" but "does the operation's identity exist at all for
this value." `Sum (UFixed<0, 8, Hot>)` still has to answer what `mempty` is for a type that has no
representable zero... except it does, `UFixed<0, F, S>` spans `[0, 1)` and zero is in that range;
the type that has no representable value under the wrapper's operation is `Product (UFixed<0, 8,
Hot>)`, where one is not a member of `[0, 1)` at all. The wrapper answer has no way to express "this
wrapper simply does not exist for this element type," short of not writing the instance and letting
`mempty` fail at the use site with a generic `No instance for Monoid` error that says nothing about
why.

## The operation-as-parameter answer, and the one Rust library that already tried it

The alternative is what arvo has: one trait, `Identity<Op>`, parameterised by a zero-sized marker
type naming the operation, so `Additive` and `Multiplicative` are two instantiations of the same
trait on the same concrete type rather than two traits on two wrapper types. This is not a novel
shape. `alga`, the abstract-algebra crate written for `nalgebra` (Sébastien Crozet, dimforge/alga,
`https://github.com/dimforge/alga`), built its entire hierarchy this way: `AbstractMagma<O>`,
`AbstractSemigroup<O>`, `AbstractMonoid<O>`, `AbstractGroup<O>`, and `AbstractGroupAbelian<O>` are
each parameterised by an operator marker, with `Additive` and `Multiplicative` as the two markers a
concrete numeric type instantiates twice, once per operation, on the same underlying type. `alga`'s
own documentation frames this explicitly as avoiding code duplication while keeping the two
structures distinct at the type level, which is precisely arvo's stated reasoning for choosing one
`IDENTITY` const over two named constants (`202607281547_topic...md:36`). `alga` additionally
generated QuickCheck properties per structure via an `alga_derive` macro, so a type claiming
`AbstractGroup<Additive>` got automatic property tests for closure, associativity, identity, and
invertibility (docs.rs, `https://docs.rs/alga/latest/alga/general/index.html`).

`alga` is also the field's clearest cautionary tale about how far this can go before the mechanism
stops paying for itself, and it is worth being precise about what actually happened rather than
reaching for "abandoned." `alga` has not been updated since a `0.9.3` release roughly six years
before this pass, and its GitHub repository last received a push in February 2023. It was not
formally deprecated in a README notice this pass could locate, but `nalgebra`, its principal
consumer and the reason it existed, stopped depending on it. The replacement, `simba`
(dimforge/simba, `https://github.com/dimforge/simba`), kept a handful of concretely useful traits
(`SimdRealField`, `SimdComplexField`, `SimdValue`) and dropped the `Magma` / `Quasigroup` / `Loop` /
`AbelianGroup` tower entirely. The traits `simba` kept are not more abstract than `alga`'s; they are
less abstract and more concrete, specialised to "a value that might be a SIMD lane of several
scalars" rather than to "an element of an abstract algebraic structure." The direction of travel
away from the general hierarchy and toward the specific consumer need is the finding, not the exact
cause, which this pass could not establish beyond the trait set itself: nobody wrote a retrospective
naming compile time, maintenance burden, or the depth of the hierarchy as the reason. `noether`
(warlock-labs/noether, `https://github.com/warlock-labs/noether`), a newer and still-active attempt
at the same problem (pushed April 2025, 68 stars, MIT/Apache-2.0), rebuilds almost exactly `alga`'s
tower from `Magma` through `Field`, `FiniteField`, and `ExtensionTower`, and its own documentation
cites `alga` as the thing it is modernising, without stating why a fresh crate was written instead
of reviving the original. Two independent Rust attempts have now built the same deep tower twice;
neither attempt's public material explains what the first one lacked that made the second necessary,
which is itself worth recording as a gap in the record rather than filling in with a guess.

`num-traits` (rust-num/num-traits, `https://github.com/rust-num/num-traits`), the Rust ecosystem's
de facto standard numeric-trait crate, never adopted the operation-marker shape at all, and instead
reproduced Haskell's pre-`Semigroup`-split `Num`-class problem in Rust terms: `Zero` and `One` are
two separate traits, each with a `zero()` / `one()` *method* rather than an associated constant. An
open issue asks directly why these are not associated constants, and the crate's own maintainer
answer is structural, not aesthetic: at the time `Zero` and `One` were written, associated constants
either did not exist on stable Rust or could not be used the way the API needed, so the methods
shipped and stayed for backward compatibility (rust-num/num-traits issue #54, "Associated constants
for One and Zero," `https://github.com/rust-num/num-traits/issues/54`). This is the same defect
arvo's own design-round note names for the pre-`Identity<Op>` shape ("The earlier shape carried
`ZERO` and `ONE` together"), except `num-traits` also never merged the two constants under one
operation parameter, so a generic function bound on `Zero + One` gets both whether or not the type
actually has both, with no mechanism to express a type that only has one. `num-traits` is the
crate most Rust numeric code actually depends on today, and it is the one that solved neither
problem this pass is asking about.

## How far the ladder is worth climbing, with two independent data points against climbing far

`numeric-prelude`, the Haskell effort that predates `alga` by roughly a decade and set out to
replace `Num`, `Real`, `Integral`, and `Fractional` with an algebra-shaped `Additive`, `Ring`,
`Field` hierarchy plus QuickCheck-backed law tests, never displaced `Prelude`'s numeric classes in
practice; the Haskell wiki's own retrospective on the broader "mathematical prelude" reform effort
describes the discussion as having "never gelled" across years of scattered mailing-list threads and
competing proposals (`https://wiki.haskell.org/Mathematical_prelude_discussion`). What did land from
that era was narrower than a full algebra tower: the 2015 Applicative-Monad Proposal and the earlier
`Semigroup`/`Monoid` split into `base`, which fixed exactly the identity-and-combine layer arvo is
now filling and stopped there. Nobody after that shipped a ring-and-field hierarchy into `base`.

NumHask (tonyday567/numhask, currently at 0.13.x on Stackage LTS 24.49) is the most direct evidence
available for what happens when a maintained library actually climbs the ladder and then has to live
with the climb for several years. Its own release history records collapsing `Ring`, `Field`,
`Distributive`, and `Module` from classes into type synonyms, and removing `Ring` and `Field` as
superclasses of `QuotientField` in favour of a narrower `SemiField` constraint defined as
`(Distributive a, Divisive a)`, specifically so that types without subtraction (`Infinity`, `NaN`,
and a `Positive` newtype using the truncated "monus" operator) could still be quotient fields
without inheriting a `Group`-under-addition requirement they cannot satisfy. The library's own
stated tension is instructive for arvo directly: a computation involving magnitude is usually one
underlying type, so demanding that every intermediate value be re-proven as a `Positive` to keep the
subtraction-free guarantee produces enough unwrapping at call sites that the library chose ergonomics
over the tighter type-level guarantee (tonyday567/numhask, same URL as above). This is the same
shape of tradeoff as arvo's own overflow-policy axis: the type system *could* refuse to let a
`Wrapping`-strategy value participate in an operation that assumes a group, and NumHask's own
experience says that refusal has a real ergonomic cost that a maintained library eventually walked
back from once, in the specific case of subtraction-adjacent structure.

Between `alga`'s full tower going effectively unmaintained while its consumer moved to a narrower
successor, and NumHask's own maintainers collapsing three of their four top-tier classes into type
synonyms after using them in production, the two clearest available data points both point the same
direction: the ladder pays for itself at monoid and stops paying reliably somewhere around ring or
field, at least for libraries whose concrete types include the kind of partial, policy-dependent
arithmetic arvo ships (saturating, wrapping, bitpacked, IEEE-tagged). Neither source states this as
a general law, and this pass found no controlled comparison (a bench, a user study, a retrospective
naming compile time explicitly) that would let the claim be stronger than "two independent
maintained libraries backed away from the top of the tower after using it." That absence of stronger
evidence is itself worth recording rather than papering over.

Spire (typelevel/spire, `https://spire-math.org/`), the Scala ecosystem's numeric-tower library, is
the counter-data-point and needs stating honestly: it is not abandoned (an active maintainer team,
a Typelevel Discord channel, and a changelog with 2020s-era entries), it ships the full
`Semiring` / `Rig` / `Rng` / `Ring` / `Field` / `Module` / `VectorSpace` ladder, and it has survived
over a decade in production use inside the Typelevel ecosystem. This pass could not fetch Spire's
own algebra-module documentation directly (the page returned an access error on every attempt), so
the claim that Spire proves the full ladder can pay for itself rests on secondary sources
(`https://typelevel.org/blog/2013/07/07/generic-numeric-programming.html`,
`https://www.baeldung.com/scala/spire-intro`) rather than the primary documentation, and should be
weighted accordingly. What those secondary sources agree on is that Spire's performance story leans
heavily on Scala-specific mechanisms, `@specialized` and macro-based inlining, that erase the
abstraction at compile time for the JVM's primitive numeric types. Rust's monomorphisation gives a
comparable erasure for free per the section below, but Spire is evidence for "the ladder survives
when the language can erase it aggressively," not evidence that the ladder is cheap in the absence
of that erasure.

## Multi-operation folding: the motivating literature, and what it actually requires

The reason `Combine<Op>` matters beyond tidiness, per the design round, is that a fold along one axis
of a rank-N shape reduces to an identity and a combine, and arvo's tensor and cascading-fold work
depends on that reduction existing generically. The clearest published statement of why this
composes cheaply, rather than merely why it is elegant, is Jimmy Lin's short paper on monoids as a
MapReduce design principle: local aggregation before shuffle is only sound when the combining
operation is associative, and framing a MapReduce reducer as folding with a monoid rather than as an
ad hoc accumulator function is what licenses the compiler or runtime to reorder, batch, or partially
apply the combine without changing the result (Jimmy Lin, "Monoidify! Monoids as a Design Principle
for Efficient MapReduce Algorithms," arXiv, submitted April 2013, `https://arxiv.org/abs/1304.7544`).
Algebird's `sum(items: TraversableOnce[T]): T` method on both its `Semigroup` and `Monoid` traits is
the direct implementation of this idea: a fold that only needs to know the identity and the combine,
generic over what T actually is (twitter/algebird,
`Monoid.scala`). This is the same shape arvo's design round names for a cascade fold across shape
ranks: the fold does not need to know the element type's arithmetic, only that it has an
`Identity<Op>` to seed with and a `Combine<Op>` to reduce with.

What the fold literature does not by itself settle is whether the combine has to be genuinely
associative for the fold to give a reproducible answer, and that turns out to be arvo's actual open
question, addressed below rather than in the abstract.

## Whether the laws can be stated at all under a conditional overflow policy

This is where the design round's own framing of the gap is sharpest, and where the published prior
art thins out the fastest. Property-based law testing is the field's standard mechanism: Cats uses
`discipline` to express each type class's laws as a reusable `RuleSet`, and `ScalaCheck` to generate
the arbitrary inputs the law is checked against, requiring an `Eq` instance for the type under test
so the law's two sides can actually be compared
(`https://typelevel.org/cats/typeclasses/lawtesting.html`). `alga` did the equivalent in Rust via a
derive macro generating QuickCheck properties. Agda's standard library takes the opposite,
proof-carrying route: `Algebra.Structures`'s `IsMonoid` (and the sibling `IsSemigroup`,
`IsGroup`) records carry the associativity and identity laws as fields the instance author must
literally construct a proof term for, not test against generated inputs
(`https://agda.github.io/agda-stdlib/v2.0/Algebra.html`). Both approaches assume the same thing arvo
cannot assume unconditionally: that the law, once stated, is either true of the whole type or false
of the whole type. Neither approach as published has an answer for a type where the law's truth
depends on which of two zero-sized policy markers the type is additionally parameterised by, because
neither `discipline`'s `RuleSet` nor Agda's `IsMonoid` record takes "under policy P" as an argument
distinct from the type itself; the type IS the unit the law is checked against.

This pass found no published treatment, in Haskell, Scala, Rust, or a proof assistant, of exactly
arvo's shape: one type, two overflow-policy instantiations, where the algebraic structure the type
satisfies genuinely differs between the instantiations rather than merely the runtime behaviour
differing. Absence of a citation is itself the finding here; this is a case where saying so plainly
is more useful than reaching for tangential material to fill the space.

What can be checked directly, and is worth doing rather than asserting, is whether arvo's own two
overflow markers (`arvo-strategy/src/axes.rs:38`, `OverflowPolicy` sealed by `Wrapping` and
`Saturating`) actually differ in which laws they satisfy for addition. Wrapping addition on a fixed
width is addition in `Z / 2^N Z`: it is fully associative, has an identity, and every element has an
inverse, so `UFixed<I, F, Hot>` under `Wrapping` addition is a genuine abelian group under addition,
not merely a monoid. Saturating addition clamped at only one end (the unsigned case, clamped at
`MAX` and never below zero) is also associative: writing `f(x) = min(x, MAX)` for the clamp and
`a ⊕ b = f(a + b)`, the identity `f(f(a+b)+c) = f(a+b+c) = f(a+f(b+c))` holds because once the
running sum exceeds `MAX` every subsequent clamp also saturates to `MAX`, and this is exactly the
tropical-semiring shape NumHask's `Positive`/monus type relies on. Saturating addition clamped at
*both* ends, which is arvo's actual `IFixed` case since `Saturating` applies to signed types too,
is where it breaks: with a four-bit signed range `MIN = -8`, `MAX = 7`, and `a = 7, b = -8, c = -8`,
`(a ⊕ b) ⊕ c = f(f(7 + (-8)) + (-8)) = f(-1 + (-8)) = f(-9) = -8`, while
`a ⊕ (b ⊕ c) = f(7 + f(-8 + (-8))) = f(7 + (-8)) = f(-1) = -1`. `-8 ≠ -1`: two-sided-clamped
saturating addition is not associative. This derivation is this pass's own, checked by direct
calculation rather than found in a source, and it is offered as a worked example in the spirit the
context document asked for rather than as a citation. The consequence for the design question is
concrete: `Combine<Additive>` cannot be one blanket impl over "any type with `OverflowPolicy`."
Under `Wrapping` it is a full group operation. Under `Saturating` with one-sided clamping it is an
associative monoid. Under `Saturating` with two-sided clamping (arvo's actual signed case) it is not
associative at all, and a `Combine<Additive>` impl that claims otherwise would be claiming a false
law. Whether that means `Combine` should not be implemented for signed-saturating types, should be
implemented with a documented caveat, or should be gated behind a different sealed witness the way
`OneRepresentable` gates `Identity<Multiplicative>` is a design question this pass does not answer;
it only establishes that the question is real and has a concrete counterexample rather than being
speculative.

The nearest published parallel to "the operation is genuinely correct but the value it represents is
approximate" is Algebird's sketch monoids. `HyperLogLogMonoid`, `BloomFilterMonoid`, and
`CountMinSketchMonoid` merge their underlying sketches with an operation that is exactly associative;
what is approximate is the cardinality or membership estimate the sketch represents, not the algebra
merging it (twitter/algebird, `https://github.com/twitter/algebird`). That is a different axis from
arvo's saturating case: Algebird's approximation lives in what the value means, arvo's non-
associativity lives in what the operation does. IEEE-754 floating point is the more direct parallel
to arvo's fixed-point case in that the non-associativity is in the operation itself: floating-point
addition and multiplication are commutative but not associative because of rounding, a fact stated
plainly enough to need no derivation
(`https://en.wikipedia.org/wiki/Floating-point_arithmetic`, "Accuracy problems"). arvo's own
`FastFloat` and `StrictFloat` wrappers inherit this from the underlying `f32`/`f64` regardless of
any strategy marker, which means the non-associativity problem for `Combine<Additive>` is not unique
to saturating fixed point; it already exists for the float primitives the substrate wraps, and
whatever mechanism resolves it for `Saturating` almost certainly needs to resolve it for `Ieee`-
tagged floats too.

## Bounded structures and the lattice neighbourhood

arvo already ships `Bounded` (`MIN`/`MAX` as associated consts) alongside `Identity`, and the design
round's framing groups lattices and semilattices as the natural neighbours of that pairing. The
clearest applied literature for what a semilattice buys once you have one is the distributed-systems
literature on state-based CRDTs: a state-based CRDT's merge function must compute the join of two
replica states and, together with the type's initial state as the neutral element, form a
semilattice, which the formal definition states directly as requiring the merge to be commutative,
associative, and idempotent, with the update function monotone with respect to the semilattice's
partial order (Shapiro, Preguiça, Baquero, Zawirski, "Conflict-free Replicated Data Types," in
*Stabilization, Safety, and Security of Distributed Systems*, LNCS vol. 6976, Springer, 2011, DOI
10.1007/978-3-642-24550-3_29, with the companion technical report "A Comprehensive Study of
Convergent and Commutative Replicated Data Types," INRIA RR-7506, 2011). An idempotent commutative
monoid is, by definition, a join-semilattice; `Identity<Min>` and `Identity<Max>`, which arvo's own
design round already names as impls that arrive for free once the operation parameter generalises
past addition and multiplication, are exactly the identity elements of the two semilattice
operations (min as meet, max as join) that this literature is built on. This gives a second,
independent motivating use case for the `Combine<Op>` gap beyond folding: a `Combine<Min>` and
`Combine<Max>` pairing with their respective `Identity` impls is not a hypothetical extension, it is
the exact algebraic shape the CRDT literature already depends on and already has thirteen years of
production use behind it, in a different field applying the same structure arvo's design round
gestures at.

## The cost of climbing without higher-kinded types

Every library surveyed here that reached for a deep hierarchy in a language without higher-kinded
types (Rust's `alga` and `noether`, and to a lesser extent NumHask, since GHC's type classes give
Haskell HKT but NumHask still had to hand-write the `Additive`/`Multiplicative` split rather than
deriving it) paid the cost in one of two currencies: either the trait surface duplicates itself once
per operation family by hand (NumHask's parallel `Additive`/`Multiplicative` module trees, stated
directly in its own documentation as "the hierarchy is repeated for the Additive and Multiplicative
structures"), or the operation becomes a type parameter and every downstream trait bound has to
carry it explicitly (`alga`'s `AbstractGroup<Additive>` appearing in every bound that wants "a group
under addition specifically"). arvo's `Identity<Op>` already pays the second cost, which is the
correct one to pay given the design round's own reasoning: it is one trait, not a duplicated family,
and it generalises to new operations (`Min`, `Max`, `BitOr`) as new impls rather than new trait
declarations.

Rust's monomorphisation is what makes this affordable at runtime regardless of how deep the trait
bound stack gets: the compiler stamps out a distinct copy of a generic function's code for every
concrete type it is instantiated with, which is exactly the mechanism that erases the
`Identity<Op>`/`Combine<Op>` indirection to a compile-time-resolved constant and a direct call,
paying nothing at the call site (rustc dev guide, "Monomorphization,"
`https://rustc-dev-guide.rust-lang.org/backend/monomorph.html`). The documented tradeoff is on the
build side, not the runtime side: "the result is fast programs, but it comes at the cost of compile
time... and binary size," in the dev guide's own words. This pass found no measurement, from any of
the surveyed libraries or from arvo's own bench history, of how much compile time or binary size a
`Semiring`/`Ring`/`Field`-depth trait hierarchy actually costs versus a `Monoid`-depth one for a
crate with arvo's shape (const-generic width, multiple strategy markers, multiple overflow
policies, all crossed against each other). Given that arvo already multiplies four strategies by
dozens of widths by two signs by two overflow policies into hundreds of concrete impls for its
existing arithmetic traits, adding a ring-and-field tier would multiply that combinatorial space
again, and this pass has no bench evidence, from arvo or from the field, for what that costs. The
crate's own convention (`bench-and-sketch-discipline.md`: any performance claim belongs in
`mock/benches/`, not asserted) applies directly here and this pass defers to it rather than guessing.

Generic associated types, stabilised in Rust 1.65, were the most recent Rust-language-level move in
this direction and are worth naming because they are exactly the kind of feature that could
eventually let a `Combine<Op>`-shaped trait express something closer to a real higher-kinded
abstraction (a fold that is generic over which container it folds, not only which element type).
The stabilisation announcement is explicit that GATs are a foundational piece for "a vast range of
patterns," not a solved higher-kinded-types story on their own, and separately notes several
concrete limitations shipped with the initial stabilisation (GAT-bearing traits are not yet
object-safe, and higher-ranked trait bounds could incorrectly imply `'static`) that a design
depending on GATs for this hierarchy would need to check are still live on the pinned nightly (Rust
Blog, "Generic associated types to be stable in Rust 1.65,"
`https://blog.rust-lang.org/2022/10/28/gats-stabilization/`). Nothing in the design round as written
asks for GATs, and this pass found nothing in the surveyed prior art suggesting a monoid-and-above
hierarchy needs them; the note is here because the brief asked specifically about the HKT-less
encoding cost, and GATs are the nearest thing Rust has shipped toward closing that gap without
actually closing it.

## What this pass could not find

Stated plainly, per the context document's instruction that a negative result is a real finding:

No source, in any of the languages surveyed, publishes a treatment of an algebraic law that holds
conditionally on a second, independent type-level parameter (arvo's overflow-policy axis) rather
than conditionally on the base type alone. `alga`'s and `noether`'s hierarchies, and Agda's
proof-carrying records, all assume the law is a property of the type, full stop.

No retrospective, blog post, or issue thread from either `alga` or `nalgebra`'s maintainers states
directly why `alga` was not maintained forward instead of being succeeded by the narrower `simba`.
The direction of the change (dropping `Magma`/`Quasigroup`/`Loop`/`AbelianGroup` in favour of
SIMD-lane-specific traits) is verifiable from the two crates' current trait surfaces; the stated
reasoning is not.

No source explains why `noether` was written as a new crate in 2024 rather than as a revival of
`alga`, despite `noether`'s own documentation naming `alga` as its predecessor.

No controlled comparison (bench, case study, or retrospective naming a number) was found for the
compile-time or binary-size cost of a ring-or-field-depth trait hierarchy against a monoid-depth
one, in Rust specifically or in any language using monomorphisation rather than dictionary-passing.
NumHask's and `alga`/`simba`'s narrowing are both circumstantial evidence that the deeper tiers did
not earn their keep in practice; neither states a measured cost as the reason.

Spire's own primary algebra-module documentation could not be fetched during this pass (the page
returned an access error on repeated attempts); the claims about Spire above rest on secondary
sources and should be treated as less certain than the claims sourced to primary documentation
elsewhere in this file.
