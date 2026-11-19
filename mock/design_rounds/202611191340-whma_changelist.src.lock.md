# Src changelist: the digest lands on dev, with FNV-1a at 64 bits

**Date:** 2026-11-19
**Round:** `202611191340-whma`, job `arvo-stranded`.
**Doc changelist:** `202611191340-whma_changelist.doc.lock.md`.
**Source:** `work/arvo-digest` at `3d1a3950`, everything it changed after it merged `c97a1af5`.

## `mock/Cargo.toml`

CHANGE: `crates/arvo-digest` joins the members; `arvo-digest` and `xxhash-rust` 0.8.19
(`default-features = false`, features `xxh3` and `const_xxh3`) join the workspace dependencies, as on
the branch.

## `mock/crates/arvo-bits`

CHANGE, `Cargo.toml`: depends on `notko`, and the description names xor and the byte run.
CHANGE, `src/lib.rs`: `Bits::xor`, `Bits::<64>::halves`, and the `bytes` module with `Bytes`.
ADD, `src/bytes.rs`, `src/bytes/tests.rs`; CHANGE, `src/tests.rs`: as on the branch, the xor laws at
every width included.

## `mock/crates/arvo-digest`

ADD: `Cargo.toml`, `src/lib.rs`, `src/digest.rs`, `src/xxh3.rs`, `tests/bits.rs`, `tests/refused.rs`,
`tests/xxh3.rs`, `tests/ui/` as on the branch.

ADD, `src/fnv1a.rs`: the walk is written once, in a `macro_rules!` instantiated at each width with its
state type, offset basis and prime, so `Fnv1a<32>` and `Fnv1a<64>` share it. `Fnv1a<64>` walks
`Uint<64, Adapt<TowardZero, Wrap>, Quiet>`.

CHANGE, `src/lib.rs`: the FIXME on the switched digest names
`the_switched_64_bit_digest_and_its_crossover_bench_in_the_ha`.

ADD, `tests/fnv1a.rs`: the branch's tests at 32 bits, and the same at 64: the reference values for
`""`, `"a"` and `"foobar"`, every prefix of the 512-byte run against the host's wrapping arithmetic,
the const evaluation, and a planted wrong vector shown failing.

## No change

`arvo-format`, `arvo-numeral`, `arvo-placement`, `arvo-strategy`. The branch's `mock/benches/` changes
do not land (the topic says why).
