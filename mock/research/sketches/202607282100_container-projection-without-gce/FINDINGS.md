# Sketch findings: container projection without `generic_const_exprs`

**Date:** 2026-07-28
**Outcome:** **WORKS.** Zero feature gates required.
**Unblocks:** removing `generic_const_exprs` from `arvo-strategy` and the `arvo` facade.

## Hypothesis

The Pattern C container projection's dependency on `generic_const_exprs` comes from computing the
tag and the byte count with const **functions** in const-generic **argument** position:

```rust
const impl<const N: u16, Sign: Signedness> BitsContainerFor<N, Sign> for Hot
where
    Picker: Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>,
{
    type T = <Picker as Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>>::T;
}
```

Carrying the selection as typestate, the way `Capacity` carries `Array`, removes the expression from
type position entirely, so no const-generic feature is needed.

## Result

Confirmed. `src/lib.rs` reproduces the projection's full shape (two tag families, four strategies,
two signs, six buckets including a wide bucket carrying its own byte count) and compiles clean with
**no `#![feature(...)]` gate of any kind**, on the pinned toolchain.

```
$ grep -c '#!\[feature' src/lib.rs
0
$ rustc --version
rustc 1.98.0-nightly (57d06900f 2026-05-27)
$ cargo clean && cargo check
    Checking sketch-container-projection v0.0.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
```

The absence of the gate is the proof, the same way `arvo-spectral/tests/capacity_threading.rs`
records its own GCE escape.

## The move

The tag was an integer computed by a function. It is a closed vocabulary of slots, so it becomes an
**enum of types**: `B8`, `B16`, `B32`, `B64`, `B128`, `BWide<BYTES>`. Width becomes typestate,
`Wid<N>`, the direct analogue of `Dim<N>`. A `WidthFor<F: Family>` trait maps a width to its bucket
as an **associated type**, with the two tag functions becoming two families (`HotCold`,
`WarmPrecise`) rather than two functions.

The projection then reads:

```rust
impl<W, Sign> BitsContainerFor<W, Sign> for Hot
where
    Sign: Signedness,
    W: WidthFor<HotCold>,
    Picker: Project<W::Bkt, Sign, Hot>,
{
    type T = <Picker as Project<W::Bkt, Sign, Hot>>::T;
}
```

`W::Bkt` is a path. Nothing computes in type position, so nothing needs GCE.

## What was verified, not just compiled

Both const blocks in the sketch force the associated types to resolve, so this is resolution rather
than a shape that merely parses:

- `Hot` at 13 bits resolves to `u16`, at 47 bits to `u64`, signed at 7 bits to `i8`.
- `Warm` at 13 bits resolves to `u32` and signed at 32 bits to `i64`, preserving the 2x-logical rule.
- `threaded::<Wid<13>, Hot, Unsigned>` and `threaded::<Wid<64>, Warm, Signed>` instantiate, which is
  the caller-threads-its-own-generic case. That is the exact shape recorded as having overflowed GCE
  under the `const N: Cap` form, and it costs nothing here.

## Cost

Per-width impls, one row per supported width per family. The sketch covers the boundaries plus
representative interior widths; the real crate expands its full range by macro.

This cost is already licensed by a ratified rule. `arvo-compile-time-last.md` states the substrate is
allowed to "spend trait-solver work on per-N const-trait impls (4 strategies x 64+ widths x 2 sign =
hundreds of impls) when the alternative is a runtime container check", and lists exactly this trade
under what the rule licenses. Nothing new is being argued for.

## What this settles

`min_generic_const_args` was never the question. Three builds against the real crate established the
ladder:

| Configuration | Result |
|---|---|
| gate removed | `generic parameters may not be used in const operations`, 16 sites |
| `min_generic_const_args` | `complex const arguments must be placed inside of a const block` |
| `min_` plus `const { ... }` wrapping | escalates to needing `generic_const_args`, the full successor |

So the **inline-expression form** cannot be expressed under `min_`. The recorded justification for
retaining GCE reasoned from that fact and stopped there. It never asked whether the expression had to
be in type position at all, which is the question `Capacity` had already answered elsewhere in the
same crate tree.

The justification on record also names the wrong construct. It defends `cap_size(N)` array-length
patterns, and `cap_size` has **zero live occurrences** in `arvo-strategy` or the facade; both grep
hits are inside the vetting comment itself. `const N: Cap` parameters are at zero across all three
gated crates. The pattern the gate is documented to protect no longer exists.

## Next

The topic file this sketch unblocks prescribes the migration plus the removal of every remaining
forbidden gate. The other two live gates need their own checks, both cheap:

- The `arvo` facade's only live GCE constructs are two static asserts of the form
  `[(); 1 / is_fractional(F)]:` (`ufixed.rs:274`, `ifixed.rs:308`). A stable `const { assert!(...) }`
  block is the obvious replacement and needs no feature.
- `hilavitkutin/src/lib.rs:24` has 76 `cap_size` calls and zero `[(); ...]` bounds, all inspected
  uses being value position (`cap_size(C::CAP) <= 64`, `let n = cap_size(...)`). Its own comment
  describes the const-generic use as sitting in "min-const-generic positions". Likely vestigial, the
  same finding the 2026-05-29 sweep already reached for `hilavitkutin-api` and `hilavitkutin-str`.
