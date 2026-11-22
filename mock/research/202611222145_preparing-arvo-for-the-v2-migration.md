# Preparing arvo for the v2 migration

Read on 22 November 2026, after a fetch of every remote branch, against `origin/dev` at eb17dd15. Nothing here migrates: the migration waits for mockspace v2 on its `dev`. This note records what was brought level before it, what was found, and the `include` patterns a migration of this repository would declare.

## The trunk

`cargo mock` exits 0 on `dev` eb17dd15 and on the tip of the preparation branch. The tree is clean afterwards: the run regenerates nothing that differs from what is committed.

## The engine pin

The `mockspace_branch` of `mockspace.toml` is `dev`, which the launcher resolves to the head it builds the lints against (6eb94229 on the day). The lockfiles held older heads of that same branch.

| What | Lockfiles | Resolved before | Resolved now |
|---|---|---|---|
| the tool crates under `mock/tools/` | 7 | 9c36cfa3 | 6eb94229 |
| the bench support crates under `mock/benches/support/` | 13 | 09bc0887 | 6eb94229 |
| the bench arms under `mock/benches/*/arms/` | 95 | 09bc0887 | 6eb94229 |

The probes under `mock/research/` are frozen history and keep the heads they were written against (16 lockfiles, five distinct heads). A grep over the other lockfiles for the mockspace source finds one head only, 6eb94229.

## The branches

53 remote branches, sorted by `branch-classes` on the content of what a merge into `dev` would change.

| Class | Count | What was done |
|---|---|---|
| already on the stale list with their evidence | 35 | nothing here; they wait for `forge-sweep`, which deletes through the console |
| landed | 4 | `chore/supersedes-field`, `salvage/arvo-registry-clean-20261114T093640Z`, `work/arvo-registry`, `work/bench-records-sweep`: queued for deletion through `forge-route delete-branch` |
| record | 0 | |
| source | 3 | see below |
| conflicting | 9 | see below |

Landed by this work: the bench records of `bench/linux-x86_64-12th-gen-intel-core-i5-1240p/20261115T2152Z` (545 new files; its `INDEX.md` merged cleanly), and the eleven new files of `work/arvo-bits-third-review` that sit under `mock/research/` (the third and fourth review passes of the bit tier, and the sketch of a proof across a subtyping coercion). The first branch is then landed whole. The second keeps its source, lint and round files, which it edits against a `dev` that has moved.

Retired: `docs/root-readme`. It rewrites `README.md` by hand on 3 November; the file is generated, and `dev` has regenerated it several times since (the last on 20 November), so the branch conflicts with a newer file of the same meaning.

Left, with what each still holds. The last column counts commits of the branch whose subject has no equal on `dev` (a squash or a rebase leaves other subjects, so a low number is evidence that most of the branch landed in another form, not proof).

| Branch | Ahead | Behind | Last commit | Subject not on dev |
|---|---|---|---|---|
| `work/arvo-bits-third-review` | 14 | 47 | 2026-11-20 | 14 |
| `work/arvo-bits-tier` | 24 | 50 | 2026-11-20 | 24 |
| `work/arvo-digest` | 94 | 70 | 2026-11-14 | 88 |
| `work/arvo-format` | 79 | 70 | 2026-11-14 | 75 |
| `work/arvo-lints-half-up` | 41 | 74 | 2026-11-13 | 36 |
| `work/arvo-numeral-doors` | 25 | 51 | 2026-11-20 | 5 |
| `work/arvo-numeral-exact-step` | 37 | 47 | 2026-11-20 | 5 |
| `work/arvo-values` | 94 | 69 | 2026-11-14 | 90 |
| `work/arvo-values-fin-unpushed` | 64 | 70 | 2026-11-14 | 61 |
| `work/commit-subject-config` | 4 | 55 | 2026-11-16 | 3 |

The two numeral branches landed in another form: the five and the five commits left are the remainder panel and the first review of the numeral doors, which are documents. The others carry source and design rounds that were not landed and that edit files `dev` has changed since, so each needs its own owner to merge or retire it; the branches that carry open work migrate with the repository afterwards.

`work/commit-subject-config` cannot land yet. It writes `{{commit_rules}}`, `{{commit_types}}` and `{{commit_examples}}` into the commit-style skill template and an `extra_types` key under `[lints.commit-subject]`. Neither exists on mockspace `dev`: a search of its source for both finds nothing, so the template would render with the names unexpanded.

## The `include` patterns

The listing of `mock/` on `dev` (11532 tracked files):

| Directory | Files | Goes to |
|---|---|---|
| `registry/` | 15 | the namespace `registry` |
| `registry_catalogue/` | 1 | the namespace `registry`, as the rendering of it |
| `research/` | 7057 | the namespace `mock` |
| `design_rounds/` | 1375 | the namespace `mock` |
| `test-history/` | 11 | the namespace `mock` |
| `benches/results/` | 1955 | the namespace `mock` |
| `crates/**/*.md.tmpl` | 17 | the namespace `mock` |
| `agent/**/*.md.tmpl` | 12 | the namespace `mock` |
| `*.md.tmpl` and `catalogue.md` at the root of `mock/` | 4 | the namespace `mock` |
| `crates/` (the rest), `benches/` (arms and support crates), `lints/`, `tools/`, `agent/` (config and hooks), `Cargo.toml`, `Cargo.lock` | 1085 | stays on the trunk |

The declaration, in the shape muisti runs today:

```toml
[[data_ref]]
namespace = "registry"
path = "registry"

[[data_ref]]
namespace = "mock"
path = "."
include = [
    "research/**",
    "design_rounds/**",
    "test-history/**",
    "benches/results/**",
    "crates/**/*.md.tmpl",
    "agent/**/*.md.tmpl",
    "*.md.tmpl",
    "catalogue.md",
]
```

Two things a dry run has to settle, because this note could not run one. Whether `registry_catalogue/` belongs with the registry or with the rest, and whether the second namespace, whose `path` is the whole of `mock/`, leaves out `registry/` by itself or needs the pattern `!registry/**`. Both are read off the dry run's listing.

The crates' rendered readmes are not under `mock/`, so no pattern names them. The bench arms and support crates stay on the trunk because `cargo mock bench` compiles them from the instance on disk; moving only `benches/results/` keeps the 1955 largest files off the trunk without touching what builds.
