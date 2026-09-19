# A manifest rename reaches the same hazard, and no arm here checks it

Hypothesis: a `Cargo.toml` dependency renamed to the key `core`
(`core = { package = "fakecore", ... }`, where `fakecore` is a crate
exporting its own `primitive` module) hijacks the leading-`::` path
`::core::primitive::usize::BITS` under `cargo check` exactly as `extern
crate self as core` does, because both replace what the name `core`
resolves to at the crate root, one through source and one through the
dependency graph. Under `[target.'cfg(...)'.dependencies]` the rename is
target-gated too, so it need not even be unconditional to reach a real
build.

This is not a committed arm. A manifest edit is not `.rs` source, so it
cannot be expressed as a single-file rustc probe alongside its siblings
in this directory the way the other three forms are; checking it needs
a real Cargo workspace with its own `Cargo.toml`, `Cargo.lock` and a
`fakecore` crate, which is a fixture rather than a one-file spike.
Per `evidence-lives-in-the-repo-or-it-never-happened.md`, an ad-hoc
spike outside this repository establishes nothing citable, so the
uncommitted cargo-rename crate this hypothesis was checked against once
is not named here as evidence, and the hypothesis is stated as a
hypothesis rather than as a checked result.

Outcome: NOT CHECKED IN THIS DIRECTORY. What is settled: this lint reads
`.rs` source only (`ctx.all_sources`, `mockspace`'s source-file listing)
and never opens `Cargo.toml`, so a manifest rename is unguarded by this
lint regardless of whether the hijack itself would succeed. `DESIGN.md.tmpl`
and the lint's own module doc both say so.

Next step this unblocks: a fixture crate under this repository's own
`<mock>/research/sketches/` (or a dedicated `Cargo.toml`-bearing probe
directory) that runs `cargo check` against a `core = { package =
"fakecore" }` rename and reads whether the build accepts or refuses it,
turning this hypothesis into a checked result citable elsewhere.
