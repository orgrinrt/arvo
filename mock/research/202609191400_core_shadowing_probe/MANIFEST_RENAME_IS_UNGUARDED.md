# A manifest rename reaches the leading-colon path, and the lint does not see it

Hypothesis: a `Cargo.toml` dependency renamed to the key `core`
(`core = { package = "fakecore", ... }`) replaces what the name `core`
resolves to at the crate root, through the dependency graph rather than
through source, so the leading-`::` path `::core::primitive::usize::BITS`
reads the renamed crate's `primitive` instead of the real one. Under
`[target.'cfg(...)'.dependencies]` the rename is target-gated as well, so
it need not be unconditional to reach a real build.

The fixture is `manifest_rename/`, four small Cargo packages, each its own
workspace, checked by `run.sh` with `cargo check --locked` beside the
single-file arms:

- `fakecore/` does `pub use core::*;` and then declares its own
  `primitive` module whose `usize::BITS` is 8.
- `user/` renames `fakecore` to `core` in its manifest and asserts, in a
  `const _`, that `::core::primitive::usize::BITS` is 8 and differs from
  the pointer width read through the primitive type `usize`.
- `fakecore_without_the_glob/` is the same crate with the glob re-export
  taken out.
- `user_of_the_crate_without_the_glob/` renames that one to `core` and
  reads the same path.

Outcome, as `run.out` records it:

- `manifest_rename/user` builds. The rename takes over `::core`, and the
  path reads 8.
- `manifest_rename/user_of_the_crate_without_the_glob` is refused with
  `cannot resolve a prelude import`. The compiler reaches the prelude
  through `core::prelude`, so a crate renamed to `core` has to carry
  core's contents for the dependent to build at all. The glob re-export
  is what the hijack needs, and it is also all it needs.

So the hazard is real, and it needs no source form: `user/src/lib.rs`
holds none of the three forms the lint reads. The lint reads `.rs`
source only (`ctx.all_sources`) and never opens a `Cargo.toml`, so a
manifest rename stays unguarded by it. `DESIGN.md.tmpl` and the lint's
module doc both say so.

What this unblocks: a manifest-reading check, which would have to refuse
a dependency key `core` under `[dependencies]`, `[dev-dependencies]`,
`[build-dependencies]` and every `[target.*]` table, with this fixture as
its firing case.
