# Op: the verification mandate, recorded rather than dispatched

**Date:** 2026-08-04
**Position:** alongside `79_dolan_what_capacity_is.md`.
**Status:** a standing obligation on the implementation phase, not a panel topic.

Op stated this and explicitly framed it as out of scope for the panel's dispatches:

> This is unrelated to the work at hand and as such, should not become something the experts talk and
> brainstorm here. However, deserves being mentioned somewhere I suppose.

So it is recorded here verbatim and carried in the consolidation's standing section. **No dispatch takes
it as a question**, and no member is briefed to design it. It binds the design rounds that follow the
canon, not the canon's own derivation.

## Parity suites for every convention alias

Op:

> for the matlab, ieee745, systemc and others we have any api aliases for first party, as designed so
> far and kept as a full intent pillar, however it might evolve in future: we need to establish full
> suite of parity tests for the APIs we write for them. As in actual tests where we run same stuff in
> our api and then on matlab/sysc/etc APIs, and assert that both actually return the same. This has to
> be done on a wide selection of different usages of the api, different kinds of computations, and
> likely best we run them via macros to actually make it maintainable too at the volume we need here;
> these will ensure that the APIs do produce the namesake's behavior, even in being expressed through
> arvo's internal typestate and contracts aliased to the namesakes. This is crucial step for each.

The load-bearing part is that these are **differential tests against the namesake's own implementation**,
not tests against a reading of the namesake's specification. The design's stated standard has always been
that representability of MATLAB, IEEE 754 and SystemC is the test that the abstractions are right; this
makes that test executable rather than argued. An alias that expresses a convention through arvo's
typestate and contracts is a claim about behaviour, and the claim is verified by running both and
comparing, across a wide selection of usages and computation kinds, macro-driven so the volume stays
maintainable.

Note what the panel has already found that this would have caught independently: a primary source whose
own definitions section transposed two biases, and a clause characterisation that held for exact results
only. A parity suite catches both without anyone reading the document correctly.

## Exhaustiveness on the mathematical side, in both directions

Op:

> Similarly, the same robustness and exhaustiveness rule guides our test batteries on general
> mathematical theory side too; we need to ensure things that should express and manifest in a certain
> way, do so, and those that should simply be inexpressible in our typestate and contracts, actually do
> fail to compile. We need the kind of exhaustiveness we've never done before ever, on any crate, and as
> such, I thought it better to spell it out here explicitly. We have to cover everything, and as we
> impl, we'll have to start from a huge enormous amount of red tests, TDD style, and build up to
> stabilizing everything we deemed and the canon deems we do.

Two directions, and the second is the one usually skipped. A property that should hold is asserted and
must pass. **A state that should be unrepresentable is asserted and must fail to compile.** The panel has
built compile-fail artifacts for exactly this shape several times (the seal adversary, the
projection-chain pair, the empty-capacity refusal one file above), and this generalises that practice
from a handful of perimeters to the whole surface.

The exhaustiveness bar is stated as beyond anything the workspace has done on any crate, and the starting
state is explicitly an enormous body of red tests, worked down as the canon's pieces land. That is the
workspace's existing red-is-the-lifeblood discipline at a scale it has not been run at, and it means an
implementation round's first act is the failing suite rather than the first passing piece.

## Where this binds

Not here. The canon is settled first; then a design round creates the settled taxonomy and its documents;
then stubs; then design rounds implement the canon piece by piece into those stubs. **This mandate governs
that last phase and the round that scaffolds it**, and it is recorded now so it is not rediscovered late
or watered down when the volume becomes apparent.
