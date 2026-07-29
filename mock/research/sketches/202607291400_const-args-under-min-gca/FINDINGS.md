# Sketch findings: expressing arvo's const-argument shapes without a forbidden gate

**Date:** 2026-07-29
**Outcome:** **WORKS**, two ways, both with **zero feature gates**. The shape in use today has no
expression under any allowed feature, so it must change rather than migrate.
**Unblocks:** D3 of `202607282100` (remove every forbidden gate), and the placement question for the
ergonomic aliases.

## Hypothesis

arvo's ergonomic aliases are `pub type Uint<const N: u16, S> = UFixed<{ ibits(N) }, { fbits(0) }, S>`:
a const function applied to a generic const parameter, in const-generic argument position. The
`202607282100` survey lists the facade's only live GCE constructs as two static asserts and does not
mention these, so either they do not need the gate or the survey missed them.

If they do need it, the question becomes what replaces them, given `generic_const_exprs` is forbidden
by D1 and `min_generic_const_args` is the only allowed successor.

## What was tried, in order

Run from inside the repository so the pinned toolchain applies. All five probes are in this directory.

**`01_alias_bare.rs`, the shape as it ships, no gate.** Refused:

```
error: generic parameters may not be used in const operations
  = help: const parameters may only be used as standalone arguments here, i.e. `N`
  = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

So the survey missed them. **The aliases are a live GCE dependency**, and the facade's gate is
load-bearing for more than the two static asserts.

**`02_alias_min.rs`, the same shape under `min_generic_const_args`.** Refused, differently:

```
error: complex const arguments must be placed inside of a `const` block
```

**`03_alias_const_block.rs`, taking rustc's own suggestion.** Refused, and escalated:

```
error: generic parameters may not be used in const operations
  = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

`generic_const_args` is the full successor, not the minimal one, and is not on the allowed list. So
the remedy the compiler offers leads out of the permitted set.

**Conclusion from the first three: there is no way to compute a const from a generic const parameter,
in type position, under any feature this workspace permits.** The computation has to leave type
position entirely, which is the same conclusion `Capacity` reached and for the same reason.

## The two escapes, both gate-free

**`04_escape_assoc_table.rs`: put the computation in an impl table and project the result.**

```rust
pub trait Sel<const N: u16> { type T; }
impl Sel<3> for Tbl { type T = Foo<4>; }
pub type A<const N: u16> = <Tbl as Sel<N>>::T;
```

The const parameter appears only as a standalone argument, which is exactly what the first error said
is permitted. Compiles. **Verified with no feature gate at all**, not merely under `min_`: the file as
committed declares none.

This is Pattern C, the shape `BitsContainerFor` and `Project` already use, so it is the mechanism
arvo has rather than a new one. The cost is one impl row per supported N, macro-generated, which
`arvo-compile-time-last` already licenses by name.

**`05_escape_typestate.rs`: the width stops being a const parameter and becomes a type.**

```rust
pub trait Width { const BITS: u16; type Wider: Width; }
pub type Widened<W> = Foo<<W as Width>::Wider>;
```

Nothing computes in type position because there is no const argument left. Compiles, no gate. This is
the `Capacity` move applied to widths, and matches the container-projection sketch at
`202607282100_container-projection-without-gce/`.

## What this establishes

The aliases cannot be migrated by adjusting syntax. Either they are deleted, or they are re-expressed
through one of the two escapes above. Both escapes are proven to need nothing, so the choice is about
ergonomics and impl-table size, not about feasibility.

It also corrects the gate inventory: `202607282100` D3 says the facade's only live GCE constructs are
two static asserts at `ufixed.rs:274` and `ifixed.rs:308`. **That is wrong.** `aliases.rs` carries four
more, one per alias, and any plan that assumes only the asserts need attention will fail at the first
build.

## What is NOT established

Whether the four aliases should survive at all. They are barely used: `Fixed` and `Signed` appear only
in two test comments, and `Uint` and `Int` have one use between them outside their own file. Deleting
them removes the problem instead of solving it, and that is a design call rather than a sketch's.

Whether the impl-table escape scales to the full width range arvo supports. Five rows compile; the
real table is per-width across a range the round has not fixed, and `202607282100` explicitly leaves
"whether the per-width macro expansion covers the full 1 to 128 range plus wide, or a narrower set
with a documented cap" open.

Nothing here touches the container projection in `arvo-strategy`, which is the other GCE dependency
and the larger one.
