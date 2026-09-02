# 117. Dispatcher note: the bench profile that was never set, and the workspace command that reports a false green

`114`'s test gate reported, in passing, that `cargo test --release --workspace` from `mock/benches`
runs zero tests. Chasing that reported one defect and found a second, larger one underneath it. Both
are recorded here rather than fixed, because the fix is entangled and wants a measured verification
that this sitting is not the right moment for. Neither blocks the panel.

This is the same lineage as `96`, which recorded fourteen bench validators that never executed. Same
shape: a surface that exists, reads as coverage or as discipline, and cannot do the thing it appears
to do.

## The first defect: ninety variant crates are outside the workspace

`mock/benches/Cargo.toml` declares no `[workspace]` table, so it is an implicit single-package
workspace. The ninety-four variant crates under `variants/` are path dependencies of the bench binary,
not members.

```
$ cargo metadata --no-deps --format-version 1 | jq '.workspace_members | length'
1
```

Twenty-three variant source files, across thirteen crates, carry `#[test]` or `#[cfg(test)]`. No
workspace-wide test command reaches any of them.

**Corrected, an hour after this note was first written.** The paragraph here originally went on to call
that the shape `the-test-gate.md` names worse than absent, on the reasoning that the tests read as
coverage while never running. **That was wrong, and it was wrong in the direction that overstates.**
The tests run. Every member of this sitting has run them, per crate, by iterating
`--manifest-path`, which works precisely because each variant is its own workspace root. `115`'s
`s0_test_gate_run.txt` is the tenth such count: thirteen crates, 123 passed, zero failed, zero ignored,
and an attribute count of 123 agreeing with the run.

So the real defect is smaller and differently shaped. It is not an untested surface. It is that the
obvious command returns a **false green**: `cargo test --workspace` from `mock/benches` compiles the
bench binary, runs zero tests, and exits 0. Anyone who trusts the workspace-wide form, rather than
knowing to iterate manifests, is told everything passes by a command that examined nothing. That is a
trap for whoever comes next rather than a hole in what exists today, and the ten counts on record are
the evidence that this sitting's members did not fall into it.

The error is left visible rather than edited away, because a dispatcher note claiming an untested
surface, in a panel whose members had tested it ten times, is exactly the kind of claim the provenance
discipline exists to catch, and it was caught by reading a member's own probe output rather than by
anything I did.

## The second defect, which is the serious one: the documented profile is not set anywhere

The harness builds every variant and the bench binary with a plain release build, one invocation each,
no `--target-dir` and no profile flags (`mockspace/src/bench.rs:283` and `:307`):

```rust
.args(["build", "--release", "--manifest-path"])
```

So the profile comes from whichever manifest is the workspace root for that invocation. Checked:

```
$ grep -l '\[profile' variants/*/Cargo.toml | wc -l
0
$ grep -n '\[profile' Cargo.toml
(no output)
$ grep -n 'lto\|codegen-units\|rustflags' .cargo/config.toml ../../.cargo/config.toml
(no such settings)
```

**Zero of the ninety-four variants declare a profile, the bench root declares none, and no cargo config
supplies one.** Meanwhile mockspace's own starter template declares, for both the bench binary
(`src/bench.rs:505-508`) and every scaffolded variant (`:616-619`):

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
```

So arvo's benches have been built at cargo's **default** release profile: `lto = false`,
`codegen-units = 16`. Not the profile the harness documents, and not the profile
`bench-in-bench-harness-never-sketches.md` names as one of the four properties that make a harness
measurement mean something ("the reproducible profile (fat LTO, one codegen unit)").

`codegen-units = 16` is the part that bears on reproducibility rather than only on speed. Codegen-unit
partitioning is not stable across builds, so two runs of the same unchanged variant can differ in
inlining and layout. That is precisely the contamination the per-variant cdylib isolation exists to
prevent, arriving through a door nobody was watching.

**What this does and does not say.** It says every number this bench directory has produced was taken
under a different codegen profile than the one documented. It does **not** say any particular panel
finding is wrong. Most of this panel's evidence is compiled probes and enumeration scripts rather than
harness runs, and which findings rest on a harness artifact is a separate pass that has not been done.
Anyone about to cite a bench number from this directory should establish that first.

## What was tried, and why it was reverted rather than landed

Adding `[workspace] members = ["variants/*"]` with the four arvo-dependent variants excluded resolves
cleanly and gives 91 members, which fixes the first defect outright.

It cannot be landed on its own, because it also moves every variant's build out of its own workspace
root and into the shared one. That changes which manifest supplies the profile, and it changes cargo's
default target directory from `variants/<name>/target/` to `mock/benches/target/`. The harness resolves
built cdylibs by location, so the second change has to be verified against an actual harness run rather
than reasoned about, and a harness run under a newly-changed profile is exactly the run whose numbers
nobody should trust without a before-and-after.

So the change was reverted. The tree is as it was.

## What a fix has to do, in one pass rather than two

1. Put `[profile.release]` with `opt-level = 3`, `lto = "fat"`, `codegen-units = 1` where it will
   actually be honoured for every variant build, which means the workspace root that each build
   resolves to.
2. Make the variants workspace members so their tests are reachable, excluding the four that inherit
   `arvo` deps through `.workspace = true` and cannot build until a canon-derived `arvo` exists
   (`structural-decomposition`, `spectral-bisection`, `fnv1a`, `xxhash3`).
3. Run the harness before and after on one unchanged bench, and keep both artifact sets, so the
   profile change's effect on the numbers is measured rather than assumed. A profile change that moves
   results is not a problem; a profile change whose effect is unknown is.
4. Confirm the 123 still pass under the workspace-wide form once it reaches them, so the two ways of
   running the suite agree rather than only the per-manifest one being known good.

## The upstream half

The starter template scaffolds the profile into each variant and into the bin, and scaffolds no
`[workspace]` table. A consumer repo that runs `cargo mock bench init` therefore gets the profile it
should have and the unreachable tests it should not, which is the opposite half of arvo's situation.
Fixing membership upstream without hoisting the profile to the root would silently disable fat LTO for
every consumer's variants, since cargo ignores `[profile.*]` in a non-root member. The two changes are
one change upstream as well.

Not filed as a mockspace PR yet. `96`'s validator fix is already merged upstream at `bce17f6`.

## Provenance

Dispatcher's own work, not an expert's, and unratified like everything else here. Every command above
is quoted so it can be re-run rather than believed. The prompting observation is `114`'s, in its
section 0.2.
