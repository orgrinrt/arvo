# Doc changelist: the digest lands on dev, with FNV-1a at 64 bits

**Date:** 2026-11-19
**Round:** `202611191340-whma`, job `arvo-stranded`.
**Topic:** `202611191340-whma_topic.the-digest-lands-on-dev.md`.
**Source:** `work/arvo-digest` at `3d1a3950`, the state after round `202611140316-ys7k`.

The documents below are taken from `work/arvo-digest` as that branch left them, onto the templates
`dev` carries now, and then changed only where `Fnv1a<64>` joins and where the switched digest's hold
moves to a ticket of its own.

## `mock/DESIGN.md.tmpl`

CHANGE, section "Crate topology": six crates; `arvo-digest` sits on `arvo-bits` and `arvo-numeral`.
The table gains the `arvo-digest` row and the `arvo-bits` row names `Bytes`, `xor` and `halves` and
its `notko` dependency, as on the branch. The digest row says FNV-1a at 32 and 64 bits, XXH3 at 64
and 128, and that the switched digest joins once its crossover is measured.

CHANGE, section "Consumers": hashing is `arvo-digest`'s, new source, as on the branch.

## `mock/agent/MAIN.md.tmpl`, `mock/agent/PREAMBLE.md.tmpl`

CHANGE: six crates, `arvo-digest` named, as on the branch.

## `mock/crates/arvo-bits/DESIGN.md.tmpl`, `mock/crates/arvo-bits/README.md.tmpl`

CHANGE: `Bytes`, `Bits::xor` and `Bits::halves`, and the `notko` dependency, as on the branch.

## `mock/crates/arvo-digest/DESIGN.md.tmpl`, `mock/crates/arvo-digest/README.md.tmpl`

ADD: the two documents as on the branch, with these changes.

- The FNV-1a section states both widths: `Fnv1a<32>` and `Fnv1a<64>`, the 64-bit walk over
  `Uint<64, Adapt<TowardZero, Wrap>, Quiet>` at offset basis `0xCBF2_9CE4_8422_2325` and prime
  `0x0000_0100_0000_01B3`. The sentence that `Uint<64>`'s `mul` is refused at build goes, since the
  product is admitted.
- The switched digest section keeps its text and its FIXME, which names ticket
  `the_switched_64_bit_digest_and_its_crossover_bench_in_the_ha` in place of
  `arvo_format_admits_the_wrapping_product_over_the_unsigned_64`, and says it waits on the crossover
  bench, which the harness does not yet carry.
- The tests section checks the FNV-1a reference values at 64 bits as at 32, with a planted wrong
  vector at each width.

## `README.md`

CHANGE: the crate list names `arvo-digest`, as on the branch.

## `deny.toml`

CHANGE: BSL-1.0 on the licence allow list, with op's 13 November allowance, as on the branch.

## Generated

`docs/` regenerates from the templates by `cargo mock`.
