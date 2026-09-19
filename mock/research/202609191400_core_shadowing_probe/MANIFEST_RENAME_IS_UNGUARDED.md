# A manifest rename reaches the leading-colon path, and the lint does not see it

The question is whether a `Cargo.toml` dependency renamed to the key `core`,
`core = { package = "fakecore", ... }`, changes what the leading-`::` path
`::core::primitive::usize::BITS` reads, with nothing in the dependent's source
binding the name `core`. The fixture answering it is `manifest_rename/`, run by
`run.sh` with `cargo check --locked` beside the single-file arms, and every
statement below about an arm is either in the table or checked against
`run.out` and that arm's source.

## The arms, as `run.sh` records them

`arms.md` is the table, written by `run.sh` in the same run that writes
`run.out`: one row per arm and one per generated control, with what the arm
checks, what it is expected to do and what it did. The "what it checks" column
is counted from each arm's source by the same scan that generates the controls,
so it is not a description anybody wrote. A generated control is the arm with
one checking item's comparison flipped, `==` to `!=` or the reverse, and it
counts only when it is refused by that item alone; `run.sh`'s header says how
that is told.

## The fixture

- Thirteen Cargo packages, each its own workspace: five stand-ins for `core`
  and eight dependents.
- Every stand-in declares a `primitive` module whose `usize::BITS` is 8.
- Every dependent renames one stand-in to the key `core` in its
  `[dependencies]` table, and no other table.
- The stand-ins differ in what else they carry: `fakecore` does
  `pub use core::*;`, `fakecore_with_only_the_prelude` does
  `pub use core::prelude;`, `fakecore_with_an_empty_prelude` holds
  `pub mod prelude { pub mod rust_2024 {} }`, `fakecore_with_a_bare_prelude`
  holds `pub mod prelude {}`, and `fakecore_without_the_glob` holds only
  `primitive`.
- Every dependent is at edition 2024, and every one but
  `a_std_dependent_of_the_crate_without_the_glob` is `#![no_std]`.

## What each dependent shows

- `user`, onto `fakecore`, builds, and its two const asserts hold: `W == 8`
  and `W != usize::BITS`. `run.out` shows both generated controls refused
  with their own messages.
- `user_of_the_crate_with_only_the_prelude`, onto
  `fakecore_with_only_the_prelude`, builds with the same two const asserts
  holding, and both its generated controls are refused.
- `user_of_the_crate_with_an_empty_prelude`, onto
  `fakecore_with_an_empty_prelude`, builds with the same two comparisons
  written as array lengths, and both its generated controls are refused with
  `mismatched types` on the flipped line.
- `user_of_the_crate_with_an_empty_prelude_naming_assert`, onto the same
  stand-in, is refused with ``cannot find macro `assert` in this scope``, so
  there is no `assert!` in scope under the empty prelude.
- `user_of_the_crate_without_the_glob`, onto `fakecore_without_the_glob`, is
  refused with `cannot resolve a prelude import`. It checks no value.
- `user_of_the_crate_with_a_bare_prelude`, onto
  `fakecore_with_a_bare_prelude`, is refused with the same
  `cannot resolve a prelude import`. It checks no value.
- `user_of_the_crate_with_a_bare_prelude_naming_assert`, onto the same
  stand-in, is refused with both `cannot resolve a prelude import` and
  ``cannot find macro `assert` in this scope``, so there is no `assert!` in
  scope under the bare prelude either.
- `a_std_dependent_of_the_crate_without_the_glob`, onto
  `fakecore_without_the_glob`, builds with the same two const asserts
  holding, and both its generated controls are refused with their own
  messages.

So four of the eight dependents build, and in each of the four `::core`
reads the stand-in's 8 rather than the pointer width.

## What that establishes, and where

At edition 2024, with the rename under `[dependencies]`:

- A `no_std` dependent's build reaches its own source when the renamed crate
  has a `prelude::rust_2024` path, whether that is the real `core` whole, the
  real `prelude` module alone, or an empty module at that path.
- A `no_std` dependent is refused at the prelude import when the renamed crate
  has no `prelude` module, or a `prelude` module with no `rust_2024` in it.
- The std dependent builds against the stand-in with no `prelude` module, the
  one that refuses the `no_std` dependent.
- None of the four dependents that build holds a module, a `use` item or an
  `extern crate` item binding the name `core`.

The lint reads `.rs` source only, through `ctx.all_sources`, and never opens a
`Cargo.toml`, so none of these renames is refused by it.

## Not shown

- A rename under `[dev-dependencies]`, `[build-dependencies]` or a
  `[target.*.dependencies]` table. No arm has one, so this fixture says
  nothing about whether any of them reaches `::core`, and so nothing about
  which tables a manifest-reading check would have to read.
- Any edition but 2024.
