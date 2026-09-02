# Sketch findings: keeping the capacity traits separate, with delegated bodies

**Date:** 2026-07-29
**Outcome:** **WORKS.** No supertrait needed, no `Copy` bound forced on the runtime half, `from_fn`
survives, duplicated bodies go. One real cost, measured rather than guessed.
**Unblocks:** what `arvo-capacity` ships under D2.

## Hypothesis

`Capacity` and `ConstCapacity` are two traits with overlapping surfaces, and the round asked whether
the new crate should merge them. Two merge shapes were on the table and each had a cost: making the
const one a supertrait forces `Capacity::Array<T>` to inherit `T: Copy`, and collapsing to one const
trait loses `from_fn`.

op's proposal was that neither is necessary: implement both on the same type, in their own contexts,
and have the non-const bodies delegate to the const ones. No supertrait, so no inherited bound.

This tests that, and the one residue claimed for it.

## What was tried

Run from inside the repository so the pinned toolchain applies. Three probes in this directory,
mirroring arvo's actual shape: `Capacity` with an unbounded `type Array<T>: AsRef<[T]> + AsMut<[T]>`
and `from_fn`, `ConstCapacity` as a `pub const trait` with `type Array<T: Copy>: Copy` and const
accessors, both implemented on `Dim<N>`.

**`01_delegation.rs`. Compiles.** The claim holds in full:

```rust
impl<const N: usize> Capacity for Dim<N> {
    fn filled<T: Copy>(v: T) -> [T; N] { <Self as ConstCapacity>::filled(v) }
    fn from_fn<T, F: FnMut(usize) -> T>(mut f: F) -> [T; N] { core::array::from_fn(|i| f(i)) }
}
```

A const fn is callable from a runtime body, and the concrete type implements both traits, so naming
the other one needs no supertrait relation. Three things were asserted in the same file rather than
assumed:

- `builds_non_copy()` returns `<Dim<4> as Capacity>::Array<NotCopy>` built through `from_fn`, so the
  runtime half really is unbounded in `T`. Under the supertrait shape this would not compile.
- `CONST_BUILT` is a `const` initialised by `<Dim<4> as ConstCapacity>::filled`, so the const half
  works in a const context.
- `slice_access` calls `.as_ref()` through the `Capacity` bound, so the runtime half keeps slice
  access the const half cannot carry.

## The one real cost

**`02_ambiguity.rs`. A consumer bounding both cannot write `C::Array<T>`.**

```
error[E0221]: ambiguous associated type `Array` in bounds of `C`
    ------ ambiguous `Array` from `ConstCapacity`
    ------ ambiguous `Array` from `Capacity`
```

Worth noting the second error it emits: rustc resolves to `ConstCapacity::Array` and then reports that
`as_ref` does not exist on it. So the failure is legible rather than mysterious, which matters because
this is the error a consumer will actually meet.

**`03_disambiguated.rs`. The remedy compiles**: `<C as Capacity>::Array<u8>`. Verbose at the bound
site, and needed only by a consumer that binds both, which today is nobody.

## What this establishes

`arvo-capacity` ships both traits as they are. The merge question dissolves: it was asking which cost
to pay when neither had to be paid. The tidy-up available is delegating the duplicated bodies, which
is an implementation change with no surface effect.

The two traits stay separate for a reason worth stating in the crate's own documentation, since it
will be asked again: `from_fn` takes a closure, closures are not const-callable, and `impl const
Trait` must supply const bodies for every method. That single method is also the only one with no
`Copy` bound on its element type. Remove it and the two traits nearly coincide; keep it and they
cannot.

## What is NOT established

Whether the delegation is worth doing at all. The duplicated bodies are one-liners (`[v; N]` in both),
so this removes very little. The value is that the two impls cannot drift, not that it saves code.

Whether the ambiguity bites in practice. No current consumer binds both, so the disambiguation cost is
theoretical today, and a future one that does bind both may prefer a different arrangement.

Nothing here tests `arvo-shape` inheriting the same split, which `202607281127` question 7 raises and
this sketch does not answer.
