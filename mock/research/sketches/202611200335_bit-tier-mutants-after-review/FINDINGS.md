# The red runs of the bit tier's mutants, after the first review

Round `202611200328` answers the first review of the bit tier: the sweeps take every width from one to
sixty-four, the oracle for writing a bit is built one bit at a time, and the refusal cases cover all six
methods taking an index or an amount, in a constant, in a call at run time and in a generic call at run
time. This sketch runs the five mutant flags of `src/ops.rs` against those tests, and supersedes
`../202611200230_bit-tier-mutants/`, which stays as the record of its own run over the tests before.

**Outcome:** WORKS. Every flag is caught, and each output names the assertion that caught it.

## How it was run

`run.sh`, from `mock/`. The four flags on an operation run as `RUSTFLAGS="--cfg <flag>" cargo test -p
arvo-bits --no-fail-fast --test <binary>`, and each output keeps every line that is not a pass, every
panic with the three lines after it naming the assertion, and the exit status. The earlier runner
dropped the panic lines, so its record could not show which assertion failed.

`arvo_mutant_bits_index` moves the refusal itself, and trybuild cannot see it, since it removes
`RUSTFLAGS` from its own build (`trybuild-1.0.120/src/cargo.rs`, `env_remove("RUSTFLAGS")`). The runner
builds the nineteen index cases as examples of `index/`, whose `[[example]]` paths are the files in
`crates/arvo-bits/tests/ui/` themselves, once as shipped and once under the flag. `cargo build
--example` reaches codegen, so the cases calling at run time are refused as shipped, as `compile_fail.rs`
now has them refused under trybuild's own build.

## What the runs show

| Flag | What it breaks | Caught by |
|---|---|---|
| `arvo_mutant_bits_with_bit` | writing a bit sets it without clearing it first | `the_bit_access`, exit 101, at bit 0 of `0x1` written false at one bit |
| `arvo_mutant_bits_not` | the complement keeps the carrier's bits above the width | `the_bit_logic`, exit 101, at `not 0x0` at one bit |
| `arvo_mutant_bits_shift` | a bit shifted up out of the width stays in the carrier | `the_bit_sequence`, exit 101, at `0x2` up one at two bits |
| `arvo_mutant_bits_rotate` | the rotation wraps at the carrier's sixty-four bits, then masks | `the_bit_sequence`, exit 101, at `0x1` down then up one at two bits, and `the_bit_tier_in_a_constant` at `TOP_SET` |
| `arvo_mutant_bits_index` | an index or amount equal to the width is admitted | the nineteen index cases: each exit 101 as shipped, each exit 0 under the flag |

Each operation mutant is now caught at the narrowest width it can be seen at, one or two bits, where
the thirteen-width sweep caught the same mutants without saying where.

`the_bit_tier_in_a_constant` still does not catch `arvo_mutant_bits_not`: its one complement is of three,
anded with sixteen, which clears the stray high bits, and the width law reads the mask through `masked`,
which does not go through `not`. The logic sweep catches it at every width below sixty-four.
