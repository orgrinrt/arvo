# In-stack: what the facade dependency actually is

**Date:** 2026-07-28
**Kind:** in-stack audit, from the manifests and the source. Not design.
**Rests on:** `00_context.md` in this directory.

Every other file here surveys the outside world. This one looks inward, at the seven crates that
depend on the `arvo` facade despite every layer rule forbidding it, and asks what those dependencies
are made of. The answer changes one of the arguments the prescribed restructure rests on.

It does not change the restructure. D1 and D2 in `202607281220` are the lead designer's calls and
they stand. What follows is a factual correction to one supporting argument, offered before the
design round builds further on it, and one measurement that was not available when that round ran.

## The direction of the arrow, as designed and as shipped

`mock/DESIGN.md.tmpl:29` states the intent without ambiguity: "Dependency direction runs strictly
upward; the facade sits at the bottom." Line 67 restates it: "`arvo` is the bottom of the dependency
graph. Every other workspace crate is upstream." The `forbidden-imports` rules encode exactly that,
listing `arvo::*` as forbidden for every crate from `arvo-transparent` through `arvo-hash`.

The manifests point the other way for seven crates. `arvo-tensor`, `arvo-bitmask`, `arvo-hash`,
`arvo-graph`, `arvo-sparse`, `arvo-comb` and `arvo-spectral` each carry a Cargo dependency on `arvo`.
Meanwhile `arvo` itself depends on six crates only: `notko`, `arvo-transparent`, `arvo-strategy`,
`arvo-storage`, `arvo-bits-contracts`, `arvo-numeric-contracts` and `arvo-bits`. It names none of the
seven.

There is no cycle, which is why it compiles. But the crate that its own document calls the bottom of
the graph is in fact a middle layer with seven crates sitting on top of it. The lints could not see
this because they read only each crate's `src/lib.rs`, and every one of these imports is in a module
file.

## What those dependencies are made of

Fifty-one `use arvo::` statements across the seven crates. Traced symbol by symbol to the crate that
declares each one:

| Symbol imported through the facade | Declared in | Reachable directly by the importer |
|---|---|---|
| `USize`, `Cap`, `Bool`, `NUSize` | `arvo-storage/src/platform.rs` | yes, all seven may depend on `arvo-storage` |
| `Bits`, `Hot` | `arvo-storage`, `arvo-strategy` | yes |
| `Additive`, `Multiplicative`, `Identity`, `Bounded` | `arvo-strategy/src/identity.rs` | yes |
| `Strategy`, `Signedness`, `Signed`, `Unsigned`, `BitsContainerFor` | `arvo-strategy` | yes |
| `TotalOrd`, `FromConstant`, `Recip`, `Sqrt` | `arvo-numeric-contracts` | yes |
| `Pred2` | `arvo/src/predicate.rs` | **no** |

**Not one of the seven imports `UFixed` or `IFixed`.** The algorithm crates are generic over trait
bounds throughout, exactly as `mock/DESIGN.md.tmpl` and the agent rules require of them.

Fifty of the fifty-one imports name a symbol that lives in a crate the importer is already permitted
to depend on. The facade dependency is import convenience, not structural need.

## The one argument that does not hold

`202607281220` records, under "It is not only drift. One rule is unsatisfiable as written":

> `arvo-tensor`'s rule forbids `arvo::*` and states it "depends on arvo-storage, arvo-bits-contracts
> and arvo-numeric-contracts only". But `Capacity::CAP` is a `Cap`, and `Cap` lives in the facade.
> **The layer rule and the type placement cannot both hold.**

**`Cap` does not live in the facade.** It is declared at `arvo-storage/src/platform.rs:73` as `pub
struct Cap(pub USize)`, and re-exported from `arvo-storage/src/lib.rs:35`. `arvo-tensor` is permitted
to depend on `arvo-storage`, and already does.

What is actually there is `arvo-tensor/src/cap.rs:3` and `arvo-tensor/src/capacity.rs:3`, both reading
`use arvo::{Cap, USize};` when `use arvo_storage::{Cap, USize};` would compile and would satisfy the
rule. The rule is satisfiable as written. The violation is a shortcut through the facade at two
import sites.

The same paragraph continues that "the algorithm crates inherit the same problem through `UFixed` and
`IFixed`, which also live in the facade." Those two types do live in the facade, at
`arvo/src/ufixed.rs` and `arvo/src/ifixed.rs`, but the algorithm crates do not import them. The
inheritance does not occur.

The genuine exception is small and worth naming precisely. `arvo-comb` imports `Pred2` at
`greedy.rs:13` and `dp.rs:16`, and `Pred`, `Pred2` and `Pred3` really are facade-declared, in
`arvo/src/predicate.rs`. They are trait aliases over `Fn(&A, &B) -> Bool` whose only dependency is
`Bool` from `arvo-storage`, so nothing holds them at that layer either; they sit in the facade
because that is where they were written.

## What this does and does not settle

It does not touch D1 or D2. Lifting the capacity system out from under both container crates so they
become peers over one foundation is an argument about what the crates are for, and it survives
untouched. Creating `arvo-numeric` to complete the contracts-plus-sibling pattern rests on
`arvo-numeric-contracts` having no sibling and its own README saying the default impl bodies live in
the facade, which remains true and is the strongest observation in that round.

What it changes is the character of the problem. The round frames the facade situation as a
structural contradiction that the restructure is needed to resolve. The evidence says it is fifty-one
import sites taking a shortcut, plus one three-line trait-alias module that has no reason to be where
it is. That distinction matters for sequencing, because a mechanical fix is available today,
independent of the restructure, and after it the lints would enforce the designed direction rather
than describing a graph that does not exist.

It also means the restructure has to be argued on its own merits rather than on necessity, which is
the more demanding standard and the appropriate one.

## Two further inconsistencies found while checking

`mock/DESIGN.md.tmpl` contradicts itself about `Cap`. Line 43 lists `Cap` among what `arvo-tensor`
ships. Line 86 lists `Cap` among what the facade defines. Neither is right: it is `arvo-storage`.

The same document's layer table places `arvo-bitmask`, `arvo-refit` and `arvo-tensor` at L1 and
`arvo-hash` at L2, while `mockspace.toml` calls `arvo-bitmask`, `arvo-refit`, `arvo-tensor` and
`arvo-hash` all L2, and the `layers` array lists seven entries for sixteen crates with "L2 substrate"
repeated four times. The layer labels are not a single consistent scheme across the two documents
that assert them.

## Method note

This file exists because the round that prescribed the restructure recorded, in its own closing
paragraph, that the agent had proposed a mechanism arvo already shipped in a better form, and that
the existing designs were to be read properly before further design work. Reading them properly
surfaced a claim about source state that a grep contradicts in one command. The general lesson is the
one already in the workspace rules: a claim about what the code does is checkable, so check it, and
do not carry it forward from a summary.
