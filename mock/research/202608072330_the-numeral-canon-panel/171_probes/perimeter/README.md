# P3: is the binding perimeter the same as the observation perimeter

```sh
rustc --edition 2024 -C opt-level=3 -C debug-assertions=off -o p3 perimeter.rs && ./p3
rustc --edition 2024 -C opt-level=3 -o cg perimeter_opaque_MUST_NOT_COMPILE.rs   # must be REFUSED
```
Output: `perimeter.out`.

`167`'s delimiter is **binding**-based: the region ends where an intermediate is named. The
observability rule's is **observation**-based. If an intermediate can be bound, at a type that differs
between two implementations, and still admit no distinguishing context, the two perimeters come apart.

## Result

| binding | types differ | distinguishing context exists |
|---|---|---|
| transparent | yes | **yes** (`size_of_val` 8 against 4) |
| **opaque, `impl Carry`** | yes | **yes** (`size_of_val` 8 against 4) |
| transparent | no | no |

**Opacity does not extend the region.** A caller that cannot name the type can still measure the value.

## Controls, all clean

- **C-G** the opaque type genuinely cannot be named: `perimeter_opaque_MUST_NOT_COMPILE.rs` is refused
  with `E0308 mismatched types ... expected i64, found opaque type`, and **no artifact is produced**.
  Without this, result 2 would be about a type that was never opaque.
- **C-F** a binding at the same type in both implementations does not distinguish.
- **C-H** `size_of_val` distinguishes at least one pair, so the claim is not vacuous.

**So the binding perimeter and the distinguishing perimeter coincide exactly**, at
`debug-assertions = off`, under I14's bans on `dyn`, `TypeId` and `core::any`. Any binding at a
differing type admits `size_of_val`; any binding at the same type has nothing to distinguish.

`holds for: rustc 1.98.0-nightly (57d06900f), edition 2024, aarch64-apple-darwin,
debug-assertions = off, i32/i64 carriers, threads = 1`
