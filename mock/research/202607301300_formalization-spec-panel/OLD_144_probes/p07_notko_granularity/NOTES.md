# p07: granularity and tier resolution of the shipped attribute

Cargo project against the real `notko-macros` (path dep). Each case is a commented
block in `src/main.rs`; uncomment one at a time. Recorded results on the pin,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`:

| Case | Result |
|---|---|
| free fn | builds and runs; prints `Ok(6)` |
| `impl` block | `error: expected `fn`` at the `impl` line |
| `mod` | `error: expected `fn`` at the `mod` line |
| trait method | attribute EXPANDS, then `error[E0053]: method `t` has an incompatible type for trait` |
| `#[profile(Precise)]`, no tier file | `error: unknown profile tier `Precise`. built-ins: Hot \| Warm \| Cold. custom tier expected at `<CARGO_MANIFEST_DIR>/notko-optimizers/Precise.rs` (crate-local) or `$NOTKO_OPTIMISERS_PATH/Precise.rs`` |
| `#[profile(Precise)]` with `notko-optimizers/Precise.rs` (`based_on = "Cold"`) | builds; expands to `pub fn precise_fn(x: u32) -> ::notko::Outcome<u32, E> { ::notko::Outcome::Ok(x) }` |

Two incidental observations from the same builds.

Every build of a crate using `#[profile(Hot)]` emits `unexpected_cfgs` for
`feature = "internal"`, "originates in the attribute macro `profile`". The gate
the macro emits (`rewrite/hot.rs:25`) is expanded into the CONSUMER's crate, so
`feature = "internal"` names a cargo feature of whichever crate is being compiled.

The trait-method row is the one worth keeping: the attribute is not refused there.
It parses (a trait method is `ItemFn`-shaped), rewrites the return type to
`::notko::Outcome`, and the impl then no longer matches the trait it implements.
