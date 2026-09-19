# A manifest rename reaches the leading-colon path, and the lint does not see it

Hypothesis: a `Cargo.toml` dependency renamed to the key `core`
(`core = { package = "fakecore", ... }`) replaces what the name `core`
resolves to at the crate root, through the dependency graph rather than
through source, so the leading-`::` path `::core::primitive::usize::BITS`
reads the renamed crate's `primitive` instead of the real one. Whether the
same holds under a `[target.'cfg(...)'.dependencies]` table, rather than a
plain `[dependencies]` table, is not shown by this fixture; no arm exercises
a target table.

The fixture is `manifest_rename/`, thirteen small Cargo packages, each its
own workspace, five crates standing in for `core` and eight dependents,
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
- `fakecore_with_a_bare_prelude/` holds a `pub mod prelude {}` with
  nothing inside it, no `rust_2024` submodule at all.
  `user_of_the_crate_with_a_bare_prelude/` depends on it, to isolate
  whether the exact `rust_2024` path is needed or any `prelude` module
  satisfies the import.
- `fakecore_without_the_glob/` carries neither, only the `primitive`
  module. `user_of_the_crate_without_the_glob/`,
  `a_std_dependent_of_the_crate_without_the_glob/` and
  `a_std_dependent_of_the_crate_without_the_glob_and_a_wrong_width/` all
  depend on it: the first `#![no_std]`, the other two carrying no such
  attribute, the third the control for the second's checks.

The `no_std` dependents among the first four stand-ins assert that the
path reads 8 and that 8 differs from the pointer width read through the
primitive type `usize`, each as `const _: () = assert!(...)`, evaluated
at check time. The empty prelude and the bare prelude bring no `assert!`
into scope beyond what `#![no_std]` still grants through the language
prelude, so the empty-prelude dependent states the same two checks as
array lengths, which fail as a length mismatch when false, and the
wrong-width one asserts a width the renamed crate does not have, to show
the trick can actually fail. The std dependents assert the same two
things about the without-the-glob crate the same way the no_std ones do,
`const _: () = assert!(...)`, since nothing about their prelude is under
test and a lib crate with no `#![no_std]` still has that form in scope.

Outcome, as `run.out` records it:

- `manifest_rename/user`,
  `manifest_rename/user_of_the_crate_with_only_the_prelude` and
  `manifest_rename/user_of_the_crate_with_an_empty_prelude` build, with
  both checks holding in each. The rename takes over `::core`, and the
  path reads 8.
- `manifest_rename/user_of_the_crate_without_the_glob`, `#![no_std]`, is
  refused with `cannot resolve a prelude import`.
- `manifest_rename/user_of_the_crate_with_a_bare_prelude`, `#![no_std]`,
  is refused with the same `cannot resolve a prelude import`: a `prelude`
  module with nothing in it is refused exactly as an absent `prelude`
  module is, so the exact `rust_2024` path is what a `no_std` dependent
  needs rather than any `prelude` module existing.
- `manifest_rename/a_std_dependent_of_the_crate_without_the_glob`, no
  `#![no_std]`, builds, with both const assertions holding, against the
  same renamed crate that refuses the `no_std` dependents above.
- `manifest_rename/user_of_the_crate_with_an_empty_prelude_and_a_wrong_width`
  is refused with `mismatched types`: the array-length trick the empty-prelude
  dependent above relies on does fail when its condition is false.
- `manifest_rename/a_std_dependent_of_the_crate_without_the_glob_and_a_wrong_width`
  is refused with the const assertion's own message, "the manifest rename
  did not take over `::core`": the std dependent's check fails when its
  condition is false, the same way the no_std dependents' checks do.

So what the rename needs of a `no_std` dependent is that the path the
edition's prelude import names, `core::prelude::rust_2024` at edition
2024, resolves in the renamed crate: a `prelude` module alone is not
enough, shown by `user_of_the_crate_with_a_bare_prelude`'s refusal. That
is because a `no_std` crate's own prelude is imported through the name
`core`, which the rename has just taken over: the renamed crate is read
for it, and nothing of the real `core`'s contents beyond that exact path
is needed, an empty module at that path is enough, shown by
`user_of_the_crate_with_an_empty_prelude`'s build. A std dependent's own
prelude is imported through the name `std`, not `core`, so it needs none
of this: the rename still hijacks the leading-`::` path with no `prelude`
module in the renamed crate at all, shown by
`a_std_dependent_of_the_crate_without_the_glob`'s build and its wrong-width
sibling's refusal, both against the very crate that refuses the `no_std`
dependents above. The fixture runs at edition 2024 only, so what another
edition's prelude path needs of a `no_std` dependent is not shown here.

So the hazard is real for both, and it needs no source form: none of the
five dependents that build holds any of the three forms the lint reads.
The lint reads `.rs` source only (`ctx.all_sources`) and never opens a
`Cargo.toml`, so a manifest rename stays unguarded by it, whether or not
the dependent is `no_std`. `DESIGN.md.tmpl` and the lint's module doc
both say so, scoped to `arvo-format`, which is itself `no_std`.

What this unblocks: a manifest-reading check, which would have to refuse
a dependency key `core` under `[dependencies]`, `[dev-dependencies]`,
`[build-dependencies]` and every `[target.*]` table, with this fixture as
its firing case for the plain tables. Whether a `[target.'cfg(...)'.dependencies]`
rename needs the same check, or reaches the leading-`::` path at all, is
open until an arm exercises one.
