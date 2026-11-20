# The red runs of the bit tier's mutants

Round `202611200155` adds the bit tier to `arvo-bits` and five mutant flags to `src/ops.rs`, each off
unless a build sets it. This sketch holds one run of each against the tests that hold its operation, so
each of those tests is shown to fail on the wrong operation it exists to catch.

**Outcome:** WORKS. Every flag is caught.

## How it was run

`run.sh`, from `mock/`. The four flags on an operation run as `RUSTFLAGS="--cfg <flag>" cargo test -p
arvo-bits --no-fail-fast --test <binary>`, and each output keeps the run less its passing lines.

The fifth, `arvo_mutant_bits_index`, moves the refusal itself, and its tests are the `compile_fail` cases.
trybuild cannot see it: it removes `RUSTFLAGS` from its own build and passes only its own flags
(`trybuild-1.0.120/src/cargo.rs`, `env_remove("RUSTFLAGS")`, and `src/rustflags.rs`, which keeps only an
instrument-coverage flag), so a first run of `compile_fail.rs` under the flag passed all nine cases while
proving nothing. The runner builds the five refusal cases as examples of `index/`, a crate whose
`[[example]]` paths are the files in `crates/arvo-bits/tests/ui/` themselves, once as shipped and once
under the flag.

## What the runs show

| Flag | What it breaks | Caught by |
|---|---|---|
| `arvo_mutant_bits_with_bit` | writing a bit sets it without clearing it first | `the_bit_access`, exit 101 |
| `arvo_mutant_bits_not` | the complement keeps the carrier's bits above the width | `the_bit_logic`, exit 101 |
| `arvo_mutant_bits_shift` | a bit shifted out of the width stays in the carrier | `the_bit_sequence`, exit 101 |
| `arvo_mutant_bits_rotate` | the rotation wraps at the carrier's sixty-four bits, then masks | `the_bit_sequence` and `the_bit_tier_in_a_constant`, exit 101 |
| `arvo_mutant_bits_index` | an index or amount equal to the width is admitted | the five refusal cases: each exit 101 as shipped, each exit 0 under the flag |

`the_bit_tier_in_a_constant` does not catch `arvo_mutant_bits_not`: its one complement is of three,
anded with sixteen, where the stray high bits are cleared by the `and`. The logic sweep catches it at
every width below sixty-four.
