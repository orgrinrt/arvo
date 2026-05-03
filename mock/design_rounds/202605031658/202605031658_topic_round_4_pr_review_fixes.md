**Date:** 2026-05-03
**Phase:** TOPIC
**Scope:** Round 4 follow-up — senior PR #43 review fix-ups
**Source topics:** PR #43 senior review on `feat/const-hash-and-narrowing`

# Round 202605031658 — round 4 PR review fixes

## Why this round exists

Round 202605031548 (#314) closed and PR #43 opened, then the senior reviewer flagged two blockers and three should-fixes. Per `local-pr-review-flow.md`, blockers must land before merge. Per the workspace state machine, source edits require IMPL phase, so this thin follow-up round opens to ship the fixes on the same branch.

## Senior-review findings to fix

**Blockers** (must land before merge):

1. **Test crate breakage**: `tests/algo.rs` and `tests/fnv1a.rs` use the deleted `HasherExt` and per-N `hash_const` inherents. `cargo +nightly check --tests --workspace` produces 13+ errors. The lock criterion `cargo test passes` was not exercised pre-merge.
2. **`XxHash3` streaming silent truncation**: `update()` drops bytes beyond the 256-byte buffer with no diagnostic. Pre-existing bug surfaced by the round 4 cleanup. Add `debug_assert!` so debug builds catch the overflow; document the limitation explicitly in the doc comment.

**Should-fix**:

3. **Em-dash in `narrow_from.rs:1`**: hard-banned by `writing-style.md`. New file; no excuse.
4. **`arvo-storage/src/meta_bits.rs:299-301` stale Width references**: doc references `Hasher<const N: Width>` and `Fnv1a<const N: Width>` from the pre-#316 era. Mechanical refresh.
5. **Bridge family ships with zero impls**: `ConstFrom`, `ConstTryFrom`, `ConstDeref`, `ConstAsRef` declared in round 4 with no implementor anywhere in the workspace. Land at least one canonical impl per bridge so the trait shape is exercised. Targets: `ConstFrom<u16> for Width`, `ConstAsRef<[u8]> for WideBits<BYTES, A>`, `ConstDeref` on a representative substrate type.

**Nit (defer)**:

6. The `_bound::<Signed>()` workaround in `algo.rs:91-95` is a smell. Cleaner: `pub use arvo::strategy::Signed` so `Signed` is part of the public re-export surface. Defer to a follow-up; not in this round's scope.

## Decisions captured

### Decision A: Round shape

**Single thin follow-up round on `feat/const-hash-and-narrowing`**, mirroring the round 202605031530 drift-cleanup pattern that ran post-#316. Topic + doc CL + src CL + close. Same PR #43 carries the fixes after the branch is updated.

### Decision B: XxHash3 streaming truncation handling

**Add `debug_assert!(self.pos + bytes.len() <= self.buffer.len(), ...)` to `update()`.** This is the correct shape for a contract violation: visible in debug, no perf cost in release. The streaming API surface stays unchanged; consumers who want to chunk above 256 bytes either chunk manually or reach for `ConstHash::hash_const(bytes)` (which has no buffer limit).

The alternative (delete the streaming impl entirely) is too disruptive for a fix-up round; the streaming contract was already public pre-round-4.

### Decision C: Bridge canonical impls

Ship one per-bridge canonical impl this round to exercise the trait shapes:

- `ConstFrom<u16> for Width` (arvo-strategy/src/width.rs)
- `ConstAsRef<[u8; BYTES]> for WideBits<BYTES, A>` (arvo-strategy/src/widebits.rs)
- `ConstDeref` on `WideBits<BYTES, A>` with `Target = [u8; BYTES]` (arvo-strategy/src/widebits.rs)
- `ConstTryFrom` defers — no obvious canonical impl in this round; lands when a consumer surfaces the need.
