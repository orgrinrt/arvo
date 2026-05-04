# Sketch: bitfield macro mask-construction via `BitPrim::mask_low`

**Date**: 2026-05-04T06:02Z
**Round**: 202605040602 (Round 5, #315)
**Tracks**: src CL Topic 3 Decision 3 (bitfield macro routing).

## Question

The src CL replaces the bitfield macro's hardcoded `(1u64 << $field_bits) - 1`
mask-construction pattern with a routed call:
`<$container as ::arvo_bits_contracts::BitPrim>::mask_low(USize($field_bits
as usize))`. The macro emits inside `const fn` bodies, so the routed call
must stay const-callable on the dispatched container type. The container is
selected by `__bitfield_container_ty!($n)` to one of `u8` / `u16` / `u32` /
`u64`.

Two questions:

- Does `BitPrim::mask_low(n: USize) -> Self` evaluate at compile time when
  invoked on a bare primitive type, given that the trait is `pub const trait
  BitPrim`?
- Does the routed call compose with the macro's existing `<<` shift and `&`
  bitand operations on the same container type, in a const-fn body?

## Hypothesis

Both compose. `BitPrim` is `pub const trait`, the macro's container is a bare
unsigned primitive (u8 through u64), and bare integer `<<` / `&` are
const-stable in `const fn`. The substrate's `narrow_from.rs` already routes
through `<u64 as BitPrim>::mask_low(USize(N as usize))` in similar shape, so
the const-callability is established for at least one container width.

## Probe

`01_macro_routes_mask_low.rs` mirrors the bitfield macro's mask-construction
shape with stub `BitPrim` impls on `u8` / `u16` / `u32` / `u64`. The probe
declares a const fn that builds a slot mask via `mask_low` on the dispatched
container type and shifts it into place at a low-bit index, exactly as the
macro will emit. The probe compiles iff the routing works under rustc
1.96.0-nightly.

## Outcome

`WORKS`. The probe compiles with `pub const trait BitPrim` and `impl const
BitPrim for u{8,16,32,64}`. The const fn body uses `<C as BitPrim>::mask_low`
twice (slot mask plus parent mask), shifts and ANDs the result, and produces
a const-evaluable container value. The const canary (`const _CANARY: ... =
...;`) evaluates the const fn at compile time and asserts the produced mask
matches the hand-computed `(1 << N) - 1` shape.

The src CL applies the macro change. Slot mask emission becomes:

```rust
let mask = <C as BitPrim>::mask_low(USize($field_bits as usize));
let parent_mask = <C as BitPrim>::mask_low(USize($n as usize));
let shifted = (mask << $lo) & parent_mask;
```

where `C = $crate::__bitfield_container_ty!($n)` is the dispatched container
type. The hardcoded `(1u64 << N) - 1` patterns and their associated
`lint:allow(no-bare-numeric)` markers come out.

## Notes

`BitPrim::mask_low` saturates at `WIDTH`: passing `n >= WIDTH` returns the
all-ones value, passing `n == 0` returns zero. This matches the prior macro
behaviour where `if $field_bits == 64 { u64::MAX }` was the special case for
the equal-width branch. The `mask_low` semantics absorb that case; the macro
no longer needs the `if` branch.

The macro is declarative (`macro_rules!`), so the routing is a textual
substitution. No proc-macro rebuild concern. Token tree structure stays the
same; only the mask-construction expression changes.
