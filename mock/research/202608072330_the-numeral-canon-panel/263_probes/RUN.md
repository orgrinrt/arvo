# 262 probes

Everything seat 262 rests on, in the form it was run. All of it is an ad-hoc spike in the sense
`evidence-lives-in-the-repo-or-it-never-happened` gives the phrase: qualitative, one instance each,
no bench harness, and named as such in the seat file. Tree: arvo at
`b544c82cf66536bfd19e3d3f7bdf995a4a813c52`, engine `mockspace` at
`a7dd822305629e54c6ed4ed2c5670840ecab2677`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`aarch64-apple-darwin`.

## The four-cell probe

`probe_262.rs` is one test that plants the same structural sentence in five filings and runs the two
shipped predicate lints, `a-region-agrees-with-the-sentence-kind` and
`every-predicate-names-a-declared-axis`, against each. It was compiled into a copy of the engine's
generated lint pack rather than into the pack itself, so that nothing under `mock/lints/` was
touched: copy `mock/target/mockspace-lints/{Cargo.toml,src}` somewhere, append

```
#[path = "<absolute path>/probe_262.rs"]
mod probe_262;
```

to the copied `src/lib.rs`, and run `cargo test --manifest-path <copy>/Cargo.toml probe_262 --
--nocapture` with the working directory inside the repository. `probe_262.out` is that run. The
controls are stated in the file's header: cells A and B must fire, cell E must not, and the test
asserts all five cells so a lint changing its behaviour fails it.

## The suite

`pack_test.log` is the whole generated lint pack's unit suite, 685 tests, run from a copy placed at
`mock/target/probe-pack/` so that the testkit's `repo_root()` walk finds `mockspace.toml`. Result:
669 passed, 0 failed, 16 ignored.

`pack_test_from_scratch_location.log` is the same suite run from a scratch directory outside the
repository, kept because it reports seven failures and every one of them is `no mockspace.toml above
the working directory`: the testkit walks up from the working directory to find the repository, and
from outside it there is nothing to find. It is a fact about where the copy sat and not about any
lint, and it is committed so the next reader does not spend a dispatch on it.

The three source crates' suites were run with `cargo test` in `mock/`; the counts are in the seat
file's section 0 and were read off that run rather than recorded here, since nothing in the seat
rests on them.

## The engine's pack at this pin

`cargo_mock.log` is the first `cargo mock` in this worktree, which exited 1 with six `E0308`
mismatches on `LintPack` and the engine's `BLOCKED` notice. `generated_pack_Cargo.toml` is the
manifest that run left behind and `cargo_tree_d.out` is `cargo tree -d` in that directory afterwards.
`cargo_mock_at_the_resolved_pin.log` is the next full run in the same worktree at the same pin, which
exited 0 with `all lints passed`. `launcher_registry.toml` is the launcher's own record of which
engine build served which consumer, copied from `~/.cache/mockspace/registry.toml`. The seat file's
section 0 says what these establish and what they do not.
