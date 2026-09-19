# A manifest rename reaches the leading-colon path, and the lint does not see it

Hypothesis: a `Cargo.toml` dependency renamed to the key `core`
(`core = { package = "fakecore", ... }`) replaces what the name `core`
resolves to at the crate root, through the dependency graph rather than
through source, so the leading-`::` path `::core::primitive::usize::BITS`
reads the renamed crate's `primitive` instead of the real one. Under
`[target.'cfg(...)'.dependencies]` the rename is target-gated as well, so
it need not be unconditional to reach a real build.

The fixture is `manifest_rename/`, ten small Cargo packages, each its
own workspace, four crates standing in for `core` and six dependents,
checked by `run.sh` with `cargo check --locked` beside the single-file
arms. Every stand-in declares its own `primitive` module whose
`usize::BITS` is 8, and every dependent renames its stand-in to `core` in
its manifest and reads `::core::primitive::usize::BITS`. The stand-ins
differ only in what else they carry:

- `fakecore/` does `pub use core::*;`, the real `core` whole. `user/`
  depends on it.
- `fakecore_with_only_the_prelude/` does `pub use core::prelude;`, the
  real `prelude` module and nothing else of `core`.
  `user_of_the_crate_with_only_the_prelude/` depends on it.
- `fakecore_with_an_empty_prelude/` holds nothing of the real `core`, only
  a hand-written `pub mod prelude { pub mod rust_2024 {} }`.
  `user_of_the_crate_with_an_empty_prelude/` depends on it, and so does
  `user_of_the_crate_with_an_empty_prelude_and_a_wrong_width/`, the
  control for its array-length trick.
- `fakecore_without_the_glob/` carries neither, only the `primitive`
  module. `user_of_the_crate_without_the_glob/` and
  `a_std_dependent_of_the_crate_without_the_glob/` both depend on it, the
  first `#![no_std]` and the second carrying no such attribute.

The `no_std` dependents among the first four assert that the path reads
8 and that 8 differs from the pointer width read through the primitive
type `usize`. The empty prelude brings no `assert!` into scope, so those
two dependents state the same two checks as array lengths, which fail as
a length mismatch when false, and the wrong-width one asserts a width the
renamed crate does not have, to show the trick can actually fail. The std
dependent asserts the same two things about the without-the-glob crate
with ordinary `assert!` calls in `fn main`, since nothing about its
prelude is under test.

Outcome, as `run.out` records it:

- `manifest_rename/user`,
  `manifest_rename/user_of_the_crate_with_only_the_prelude` and
  `manifest_rename/user_of_the_crate_with_an_empty_prelude` build, with
  both checks holding in each. The rename takes over `::core`, and the
  path reads 8.
- `manifest_rename/user_of_the_crate_without_the_glob`, `#![no_std]`, is
  refused with `cannot resolve a prelude import`.
- `manifest_rename/a_std_dependent_of_the_crate_without_the_glob`, no
  `#![no_std]`, builds, with both checks holding, against the same
  renamed crate that refuses the `no_std` dependent above.
- `manifest_rename/user_of_the_crate_with_an_empty_prelude_and_a_wrong_width`
  is refused with `mismatched types`: the array-length trick the empty-prelude
  dependent above relies on does fail when its condition is false.

So what the rename needs of a `no_std` dependent is that the path the
edition's prelude import names, `core::prelude::rust_2024` at edition
2024, resolves in the renamed crate. That is because a `no_std` crate's
own prelude is imported through the name `core`, which the rename has
just taken over: the renamed crate is read for it, and nothing of the
real `core`'s contents beyond that path is needed, an empty module at
that path is enough. A std dependent's own prelude is imported through
the name `std`, not `core`, so it needs none of this: the rename still
hijacks the leading-`::` path with no `prelude` module in the renamed
crate at all, shown by `a_std_dependent_of_the_crate_without_the_glob`,
whose renamed crate is the very one that refuses the `no_std` dependent.
The fixture runs at edition 2024 only, so what another edition's prelude
path needs of a `no_std` dependent is not shown here.

So the hazard is real for both, and it needs no source form: none of the
four dependents that build holds any of the three forms the lint reads.
The lint reads `.rs` source only (`ctx.all_sources`) and never opens a
`Cargo.toml`, so a manifest rename stays unguarded by it, whether or not
the dependent is `no_std`. `DESIGN.md.tmpl` and the lint's module doc
both say so, scoped to `arvo-format`, which is itself `no_std`.

What this unblocks: a manifest-reading check, which would have to refuse
a dependency key `core` under `[dependencies]`, `[dev-dependencies]`,
`[build-dependencies]` and every `[target.*]` table, with this fixture as
its firing case.
