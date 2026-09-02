# Arvo arbitrary-limitations audit catalog

Fulfils the catalog exit-condition of design round `202605111741` (topic: arvo-arbitrary-limitations-audit). That round's verdict: a CSR/spectral round had accepted a "capacity cap at 64 nodes" as if it were a property of the foundations, when it was an implementation choice. The principle the round stated: the foundations ship generic surfaces and named defaults, never hardcoded literals masquerading as foundation properties.

This document is the catalog only. It does not fix any signature. Each open category drives a separate follow-up round that cites this note. The catalog applies the round's definition of "arbitrary" at every site and marks each one open or already-resolved.

The 202605111741 survey was authored 2026-05-11 and is partly stale: at least one site it flagged (the spectral fiedler gate) has since been fixed. This catalog re-ran the surveys and reflects source state as of 2026-05-28.

## What counts as arbitrary

A limitation is arbitrary, and a candidate for lifting, when all of these hold:

1. It is a single hardcoded value or width where the surrounding type or trait is already generic over that axis.
2. No mathematical or representational necessity forces the value (it is not a property of the bit width, the algorithm, or the IEEE/native primitive ladder).
3. A plausible consumer could need the value lifted (a wider hash, a graph past N nodes, a bitfield past 64 bits).
4. The value is expressed as a runtime literal or assert rather than a type-level bound the call site picks.

A limitation is justified, and stays, when it traces to a real necessity: the native primitive ladder topping out at u128, an adjacency matrix being N-by-N for N nodes, a hash algorithm whose state width is the specification.

## Catalog

### 1. Width hardcoding where the chassis is generic (arbitrary, open)

The largest category. The L2 layer provides W-generic `Bits<W, S, Sign>`, but algorithms hardcode `Bits<64, Hot, Unsigned>` instead of carrying a `W: BitPrim` parameter. Survey as of 2026-05-28: dozens of `Bits<64, Hot, Unsigned>` literal occurrences across `mock/crates/*/src/` (a volatile aggregate that moves with every commit, so the catalog records the pattern and the open sites rather than a precise count). Still-open sites include `arvo-graph` (rank, topo, path, waist, components, spanning), `arvo-sparse` (rcm, block, dm), and `arvo-comb`. Each hardcodes the word width where the algorithm is width-agnostic. Criterion 3 applies: liftable to `W: BitPrim` with the default named, not baked. Follow-up: a per-crate W-generic round for arvo-graph, then arvo-sparse, then arvo-comb.

### 2. ContentHash width alias (arbitrary, open)

`mock/crates/arvo-hash/src/aliases.rs:12`: `pub type ContentHash = Bits<64, Hot>;`. The `Hasher` / `ConstHash` traits are now generic over `const N: u16` (`arvo-hash/src/algo.rs`), so the alias is the only remaining hardcode in the hash surface. A consumer wanting 128-bit content hashes must abandon the alias and name the width directly. Criterion 3 applies; the fix is a named default over a generic alias rather than a fixed one. Follow-up: a hash-width round (pairs with #181, the arvo-hash ContentHash workload confirmation).

### 3. Hash state-width gates (implementation choice, open, minor)

`mock/crates/arvo-hash/src/fnv1a.rs:23` and `xxhash3.rs:36` document a `1 <= N <= 64` gate tied to the 64-bit hash state. This is closer to a specification of the chosen algorithm state than to an arbitrary literal, but it is a ceiling a 128-bit-state variant would lift. Catalogued as a known boundary; lower priority than 1 and 2.

### 4. Bitfield width ceiling (arbitrary-shaped, open)

`mock/crates/arvo/src/bitfield.rs:203`: `assert!($n <= 64, "bitfield N must be <= 64")` (carries `lint:allow ... tracked: #127`). The bitfield primitive is capped at 64 bits by a runtime assert. Criterion 4 applies: the ceiling is a runtime assert rather than a type-level where-clause the call site satisfies, and the cap itself is a candidate to lift (a wider bitfield over a wider container). Follow-up: a bitfield-ceiling round (lift the cap or convert the assert to a bound), coordinated with #127.

### 5. Container-tier width ceilings (justified, recorded)

`mock/crates/arvo-strategy/src/container.rs:60-91` and `width.rs:104-130`: Hot and Cold project to a native bucket up to N=128; Warm and Precise up to N=64; both fall to the wide bucket above. The `#[diagnostic::on_unimplemented]` at `container.rs:112` explains there is no native u256 ladder. This is justified under criteria 1 and 2: it traces to the native primitive ladder topping out at u128, and the `Project` trait plus WideBits is the documented escape above 128. Recorded as a knowingly-chosen boundary, not a fix target.

### 6. Adjacency capacity cap (justified N, arbitrary W, partly resolved)

`mock/crates/arvo-tensor/src/cap.rs:16` `cap_size` (carries `lint:allow ... tracked: #121`) backs the `BitMatrix<W, N>` row count at `mock/crates/arvo-bitmask/src/matrix.rs:34`. The N-node "cap" that triggered round 202605111741 is justified: an adjacency matrix of N nodes is N-by-N by necessity (criterion 2). The arbitrary part was the W=64 word width baked alongside it, which category 1 covers. Recorded: N stays, W is the smell already tracked under category 1.

### 7. Resolved since the 2026-05-11 survey (recorded for accuracy)

`arvo-spectral` fiedler previously carried `assert!(n <= 64)`; it is GONE as of 2026-05-28 (the W-generic spectral fix landed in round 202605111719). The spectral instance of category 1/3 is closed. No `recursion_limit` directives exist in arvo source (the `recursion_limit_for_kits!` concern is hilavitkutin's #396, not arvo).

## Follow-up rounds this catalog drives

| Category | Open sites | Follow-up |
|---|---|---|
| 1. W-generic algorithms | arvo-graph, arvo-sparse, arvo-comb | per-crate W-generic round (graph first) |
| 2. ContentHash width | arvo-hash aliases.rs:12 | hash-width round (with #181) |
| 3. Hash state width | fnv1a.rs:23, xxhash3.rs:36 | folds into 2 or a wide-state variant |
| 4. Bitfield ceiling | arvo/src/bitfield.rs:203 | bitfield-ceiling round (with #127) |
| 5. Container tiers | container.rs, width.rs | none (justified) |
| 6. Adjacency cap | cap.rs:16, matrix.rs:34 | W part folds into 1; N stays |

Categories 5 and 6's N axis are justified and need no follow-up. Categories 1 through 4 are the arbitrary limitations the foundations should lift so that generic surfaces and named defaults replace the baked literals.
