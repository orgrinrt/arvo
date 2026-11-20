# Source changelist: arvo takes a capacity

**Date:** 2026-11-20
**Doc changelist:** `202611200029-bfwv_changelist.doc.lock.md`.
**Scope:** the new crate `mock/crates/arvo-capacity/`, the workspace manifest and lockfile, and the
mutant run committed beside the sketch.

## CHANGE: the workspace

`mock/Cargo.toml`: `crates/arvo-capacity` joins `members`, and `workspace.dependencies` gains
`arvo-capacity` by path at the workspace version. `mock/Cargo.lock` follows.

## CREATE: `mock/crates/arvo-capacity/`

`Cargo.toml`: depends on `arvo-format` and `arvo-numeral`, no features, `trybuild` for the tests, and
`cfg(arvo_mutant_capacity_count)` declared for the mutant.

`src/lib.rs`: `#![no_std]`, re-exports `Capacity`, `Dim` and `count`.

`src/capacity.rs`: `Dim<const N: usize>`; the crate-private `seal` module with the trait `Sealed`
holding `const LEN`, typed as the alias `Len`, an `arvo_numeral::USize`, so no bare integer is written
outside the const parameter; `Capacity: seal::Sealed` with `type Array<T>: AsRef<[T]> + AsMut<[T]>` and
`fn filled<T: Copy>(value: T) -> Self::Array<T>`, carrying
`FIXME(is_arvo_s_capacity_trait_sealed_to_its_one_marker)`; the impls for `Dim<N>`, with `LEN` one past
`N` under the mutant cfg.

`src/count.rs`: `pub const fn count<C: Capacity, A: NamesAdaptation<points::USize>, K: DebugCheck>() -> USize<A, K>`,
reading `LEN` through its host door into the consumer's signature.

`tests/agreement.rs`: the array's length and the count agree over eighteen counts from zero to 65536,
for a one-byte and a multi-word element; the count in a constant; `filled`; a nested projection.

`tests/compile_fail.rs` with `tests/ui/`: an outside impl of `Capacity` refused (E0277 on the seal)
beside `Dim<4>` used from outside; a `Dim` sized by a run-time value refused (E0435, E0747) beside one
sized by a constant.

`tests/a_consumer_crate.rs` and `tests/consumer/`: the outside crate with no feature line, its
lockfile committed, built and run into its own scratch directory; the harness is the one
`arvo-numeral`'s consumer test uses, with the crate's names.

## CREATE: the mutant run

`mock/research/sketches/202611200010_capacity-spellings/out/crate-arvo-capacity-mutant-count.txt`:
`tests/agreement.rs` under `--cfg arvo_mutant_capacity_count`, two of four red, the two that read the
count.
