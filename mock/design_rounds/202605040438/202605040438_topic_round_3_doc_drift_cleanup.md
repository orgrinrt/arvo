**Date:** 2026-05-04
**Phase:** TOPIC
**Round:** 202605040438
**Scope:** Tiny doc-drift cleanup. Carries no design decisions; aligns prose with shipped state from round 202605031748 (#313).

# Round 3 doc-drift cleanup

The senior PR review on PR #44 (round 202605031748, task #313) flagged two source-side doc references that did not get updated alongside the chassis collapse. Both are factual-state corrections, not design choices:

1. `mock/crates/arvo-mask-contracts/README.md.tmpl:3-4` describes the trait as an "abstract surface over `Mask64`, `Mask256`, `BitMatrix` and other concrete masks from `arvo-bitmask`". `Mask64` and `Mask256` were deleted by the prior round per Decision C (Strategy/Sign discoverability). The README must name the chassis form.

2. `mock/crates/arvo-bitmask/src/node.rs:5` says `NodeId` is "Used by `BitMatrix64` / `BitMatrix256`". Both deleted. Should name `BitMatrix<W, N>`.

For comparison, `mock/crates/arvo-bitmask/src/ops.rs:1-14` and `mock/crates/arvo-bitmask/src/mask.rs:11-15` already framed their references historically with explicit "Round 202605031748 collapsed the prior..." language. The two outliers above did not get the same treatment in the in-flight pass.

## Decisions

This round carries **no design decisions**. Every change is a factual correction to align prose with shipped state.

The choice of "rewrite as chassis form" vs "rewrite as historical reference" is taken from prior precedent inside the same crate: `ops.rs` and `mask.rs` use the historical-with-prefix shape ("Round 202605031748 collapsed..."), so we follow suit for `node.rs`. The `arvo-mask-contracts/README.md.tmpl` is a forward-pointer surface (rendered into DESIGN.md), so it gets the chassis form directly without a historical preface.

## Why this is a separate round

Per the mockspace state machine, the prior round's source files are frozen post-close (TOPIC phase blocks all crate edits). Per `cl-claim-sketch-discipline.md`, locked CLs cannot be re-edited; corrections flow through new rounds. This round is the smallest possible unit that gives the doc-drift fix a clean audit trail.

## What is NOT in scope

- Reviewer findings 1+2 (locked-CL claim/source mismatches in `202605031748_changelist.src.lock.md`). Those CL claims are frozen by the state machine. They are recorded leaks until mockspace #318 (cl-claim-vs-source-mismatch lint) ships; #318 will catch the same class at lock time before merge. No corrective edits to a locked CL are permitted under `cl-claim-sketch-discipline`; they would themselves be discipline violations.

- `MaskOps` concrete trait impls and `bitfield!` macro routing through `BitPrim::mask_low`. Deferred to Round 5 (#315) per round 202605031748 src CL final notes.

## Cross-references

- Prior round: `mock/design_rounds/202605031748/` (DOC + SRC locked, round closed). PR #44 squash-merged into `dev` at 2026-05-03T20:39:34Z (`8c77af4`).
- Related rule: `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`.
- Related rule: `~/Dev/clause-dev/.claude/rules/local-pr-review-flow.md` (advisory-only review framing).
- Mockspace task #318: cl-claim-vs-source-mismatch lint (will catch reviewer findings 1+2 class).
