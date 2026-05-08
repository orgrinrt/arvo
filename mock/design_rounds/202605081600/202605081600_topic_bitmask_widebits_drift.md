**Date:** 2026-05-08
**Phase:** TOPIC
**Scope:** arvo-bitmask (one paragraph)

# Bitmask: drop stale MultiContainer reference

PR-review follow-up to round 202605051400. The bitmask DESIGN body still
claims `Mask<Bits<256, Hot, Unsigned>>` is "backed by `MultiContainer<u128,
u128>`", but `MultiContainer` was deleted in #316 and replaced by
`WideBits<BYTES, A>` / `AlignedWideBits16<BYTES, A>`. The same paragraph
half-corrects itself ("unroll handled by the WideBits BitPrim impls") so the
reader sees two contradictory claims.

This round corrects the single paragraph to the post-#316 reality.

## Decisions

### Decision 1: Replacement shape

`Mask<Bits<256, Hot, Unsigned>>` projects through
`BitsContainerFor<256, Unsigned>` to `AlignedWideBits16<32>` (32 bytes,
16-byte alignment for the Hot SIMD baseline). The Warm/Cold/Precise
strategies project to `WideBits<32>` (1-byte aligned). Body prose is
updated to name the correct backing.
