# Topic: post-#316 doc-drift cleanup

**Date:** 2026-05-03
**Phase:** TOPIC
**Scope:** mechanical drift cleanup driven by senior review of round 202605031400 (#316).
**Source topics:** none (mechanical sweep, no design decisions).

## Why this round exists

Round 202605031400 (#316) deleted `UContainerFor`, `IContainerFor`,
`MultiContainer`, `MultiContainerHalf`, and `AlignedWideBits16` per
the no-legacy-shims-pre-1.0 rule. The deletion landed cleanly at the
type level, but module-doc prose and a couple of DESIGN.md.tmpl
sections still narrate against the deleted types. The
`design-doc-source-mismatch` lint catches type-name presence in
source declarations but not stale references in prose, so the drift
slipped past the close gate.

The senior PR reviewer caught this on the post-close audit. Three
em-dashes also landed in `arvo-strategy/src/container.rs` against
the workspace-wide hard ban in `writing-style.md`.

## What this round fixes

Mechanical cleanup. No design decisions. No new types or traits.

### DOC scope (DESIGN.md.tmpl edits)

- `mock/crates/arvo-strategy/DESIGN.md.tmpl`: remove `AlignedWideBits16<const BYTES: usize>` from the type list; describe the actual `WideBits<BYTES, A: Align = A1>` parametric shape; remove `MultiContainer` family references.
- `mock/crates/arvo-storage/DESIGN.md.tmpl`: same. Top-level `WideBits` and the deleted `AlignedWideBits16` typedef both need correction. Describe the actual `Align` trait + ZST markers `A1`/`A16`/`A32`/`A64`.

### SRC scope (.rs comment edits)

- `mock/crates/arvo-strategy/src/lib.rs`: module doc and `Signedness` doc describe dispatch as routing through "the sealed `UContainerFor` / `IContainerFor` const traits". Rewrite to describe the unified `BitsContainerFor<N, Sign>` + `Project` cascade. Also fix the false claim that `Warm` is "not implemented for the `33..=64` bit range" (it's `65..=128` that has no native Warm bucket; Warm covers `1..=64`).
- `mock/crates/arvo-storage/src/bits.rs`: doc cites `UContainerFor<N>::T` / `IContainerFor<N>::T`; SAFETY notice mentions `MultiContainer<HiT, LoT>`. Rewrite to name the actual inner shapes.
- `mock/crates/arvo-strategy/src/widebits.rs`: module doc narrates against deleted `MultiContainer`. Tighten or relocate.
- `mock/crates/arvo-storage/src/layout_assertions.rs`: module doc references `MultiContainer<HiT, LoT>` `repr(C)` invariant; section comment names `MultiContainer` replacement narrative. Rewrite.
- `mock/crates/arvo-storage/src/meta_bits.rs:45`: comment names `Hot: UContainerFor<{ ufixed_bits(I, F) }>`. Update to `BitsContainerFor<_, Unsigned>`.
- `mock/crates/arvo-strategy/src/arith.rs:28`: comment "Keyed on the same `N` that `UContainerFor` uses". Update.
- `mock/crates/arvo-strategy/src/container.rs:178, 193, 205`: three em-dashes (`—`) freshly introduced in module-internal comments. Replace with periods or colons per the writing-style.md hard ban.

## Lint coverage gap (recorded for follow-up)

The `design-doc-source-mismatch` lint detects deleted type-name
presence in source declarations but not in prose comments. The
senior reviewer flagged this as a follow-up: `cargo mock` should
grep deprecated-CL substrate names against source prose at close
time. Captured under task #318 (cl-claim-vs-source-mismatch lint)
which has the broader scope of structured CL claim grammar; the
prose-grep pass folds in there.
