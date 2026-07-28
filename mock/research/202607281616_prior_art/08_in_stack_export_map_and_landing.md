# In-stack: the export map, and where each thing lands under the restructure

**Date:** 2026-07-28
**Kind:** in-stack inventory, from the source. Not design.
**Rests on:** `00_context.md` and `07_in_stack_facade_dependency_audit.md` in this directory.

The prescribed restructure in `202607281220` D2 creates four crates and makes the facade terminal. It
names what three of the four hold. This file inventories what arvo actually exports today, by
declaring crate rather than by re-exporting crate, and walks each group to its destination under that
decision. The purpose is to find the items the decision does not place, before the round that builds
it discovers them one at a time.

Two items have no named destination and one group is ambiguous. Those are the finding; the rest is
inventory.

## What the facade declares, as against what it re-exports

The distinction matters because the restructure turns the facade into re-exports and nothing else, so
only the declared items have to move. Verified by reading `mock/crates/arvo/src/`.

The facade declares, in nine files:

`UFixed<I, F, S>` in `ufixed.rs` and `IFixed<I, F, S>` in `ifixed.rs`, with their impl bodies in the
paired `*_impl.rs` files and `fixed_scale.rs`. The four ergonomic aliases `Fixed`, `Signed`, `Uint`
and `Int` in `aliases.rs`. `FastFloat`, `StrictFloat` and the `Float` alias in `float.rs`. The
`bitfield!` declarative macro in `bitfield.rs`. The marker family `IntegerLike`, `FractionLike`,
`FloatLike`, `BoolLike` and `BitPresentation` in `markers.rs`, alongside the free const fns
`logical_width_unsigned` and `logical_width_signed`. The const fns `ufixed_bits`, `ifixed_bits`,
`is_fractional` and `width_le_64` in `strategy.rs`. The predicate aliases `Pred`, `Pred2` and `Pred3`
in `predicate.rs`. The euclidean division family `ScalarEuclid`, `EuclidDiv`, `EvenShares` and
`EvenSplittable` in `traits/euclid.rs`, plus the free float helpers `abs_f32`, `abs_f64`, `sqrt_f32`
and `sqrt_f64` in `traits/abs.rs` and `traits/sqrt.rs`. And `layout_assertions.rs`, which asserts
rather than declares.

Everything else reachable as `arvo::X` is a re-export. `Bits`, `Bool`, `USize`, `NUSize`, `Cap`,
`IBits`, `FBits`, `Width` and the const bridges come from `arvo-storage`. The strategy markers,
`Identity`, `Bounded`, `SignedIdentity`, `Additive`, `Multiplicative`, `OneRepresentable`, `Picker`,
`Ieee` and the widening family come from `arvo-strategy`. `Abs`, `Recip`, `Sqrt`, `TotalOrd`,
`FromConstant` and the `Predicate` family come from `arvo-numeric-contracts`. The bit aliases and the
refit family come from `arvo-bits` and `arvo-bits-contracts`. `Just`, `Maybe` and `Outcome` come from
`notko`.

## Where each declared group lands

D2 gives `arvo-numeric` the row "`UFixed`, `IFixed`, `Int`, `Uint`, `FastFloat`, `StrictFloat`, and
the default contract impls". Walking the inventory against that row:

| Declared group | Named in D2 | Destination |
|---|---|---|
| `UFixed`, `IFixed` and their impls | yes, explicitly | `arvo-numeric` |
| `Fixed`, `Signed`, `Uint`, `Int` aliases | `Int` and `Uint` named; `Fixed` and `Signed` not | `arvo-numeric`, by obvious extension |
| `FastFloat`, `StrictFloat`, `Float` | yes, explicitly | `arvo-numeric` |
| default impls of the numeric contracts | yes, explicitly | `arvo-numeric` |
| marker family and `logical_width_*` | no | `arvo-numeric` on subject matter, unstated |
| `ufixed_bits`, `ifixed_bits`, `is_fractional`, `width_le_64` | no | `arvo-numeric` on subject matter, unstated |
| `bitfield!` macro | no | unstated. It expands to `Bits`, not to `UFixed`, so `arvo-bits` is as plausible as `arvo-numeric` |
| `ScalarEuclid`, `EuclidDiv`, `EvenShares`, `EvenSplittable` | ambiguous | contracts or concretes, see below |
| `abs_f32`, `abs_f64`, `sqrt_f32`, `sqrt_f64` | no | `arvo-numeric`, being float helper bodies |
| `Pred`, `Pred2`, `Pred3` | **no** | **unplaced** |

And from `202607281220` D2's closing line, plus the round's own note:

| Item | Current home | Status under D2 |
|---|---|---|
| `Enumerator` | `arvo-tensor` | leaves, "where it lands is not settled here" |
| `Cap`, `Capacity`, `ConstCapacity`, `Dim`, `cap`, `cap_size` | `arvo-storage` and `arvo-tensor`, split | all to `arvo-capacity` |
| `Identity`, `Bounded`, `SignedIdentity` | `arvo-strategy` | open, per `202607281547`: stay or move to the algebra crates |

## The three that are actually open

**`Pred`, `Pred2` and `Pred3` are not mentioned anywhere in the restructure.** They are trait aliases
over `Fn(&A) -> Bool` at arities one through three, declared in `arvo/src/predicate.rs`, whose only
dependency is `Bool` from `arvo-storage`. They are also, per the audit in `07`, the single symbol
among fifty-one facade imports across seven crates that is not reachable from a lower crate: `arvo-comb`
imports `Pred2` at `greedy.rs:13` and `dp.rs:16`. So when the facade becomes terminal, `arvo-comb`
either loses them or the facade stops being terminal for exactly this one item. They are naming
conveniences over `Fn` with no numeric content, which makes `arvo-numeric` a poor fit and
`arvo-numeric-contracts` a plausible one, since it already hosts the `Predicate` trait these sit
beside.

**The euclidean family is ambiguous between the contracts crate and the concretes crate.**
`ScalarEuclid`, `EuclidDiv` and `EvenSplittable` are trait declarations, which by arvo's own
contracts-plus-sibling convention belong with `Abs`, `Recip`, `Sqrt` and `TotalOrd` in
`arvo-numeric-contracts` rather than with their impls in `arvo-numeric`. `EvenShares` is a struct,
which is a concrete. D2's phrase "the default contract impls" covers the impls and does not say where
the declarations go. The convention answers it, but the decision does not.

**`bitfield!` has no obvious home.** It is a declarative macro that expands to `Bits` accessors and
never mentions `UFixed`, so it belongs with the bit-storage surface rather than the numeric one, which
puts it in `arvo-bits`. D2 does not name it, and it is the only macro in the workspace, so it will not
be caught by any pattern that places types.

## One thing the inventory settles that was open

`Cap` is currently split from the rest of the capacity system. It is declared at
`arvo-storage/src/platform.rs:73`, while `Capacity`, `ConstCapacity`, `Dim`, `cap` and `cap_size` are
in `arvo-tensor`. D2's reasoning for putting all six in `arvo-capacity` is that they are one tight
semantic bunch and that placing `Cap` with the const-generic carriers would separate it from the
traits that give it meaning.

The inventory supports that on its own evidence. `Cap` sits in `platform.rs` alongside `Bool`,
`USize`, `NUSize` and `AsBool`, which are platform-shaped wrappers, and it is not one: it is a
capacity, and the only reason it is there is that `USize` is. Moving it is a correction independent of
whether the rest of the restructure proceeds.

## Method note

This inventory reads declarations, not documents. The two documents that describe arvo's layer
structure disagree with each other and with the source on where `Cap` lives, which is recorded in
`07`, so nothing here was taken from either.
