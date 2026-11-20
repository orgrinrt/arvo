# The red runs of the bit tier's mutants, after the second review

Round `202611200402` answers the second review of the bit tier: the guard of an index or an amount is
the tier's `Within::NEW`, once for every container; the refusal cases take the edges of the width, past
it, and a container of its own; the algebraic assertions that could never fail first are gone; and
seven mutants join the five, one for every operation. This sketch runs all twelve flags against the
tests, and supersedes `../202611200335_bit-tier-mutants-after-review/`, which stays as the record of its
own run over the tests before.

**Outcome:** WORKS. Every flag is caught, and each output names the assertion that caught it.

## How it was run

`run.sh`, from `mock/`. The eleven flags on an operation run as `RUSTFLAGS="--cfg <flag>" cargo test -p
arvo-bits --no-fail-fast` over the oracle test of the operation's trait and `the_bit_tier_in_a_constant`,
and each output keeps every line that is not a pass, every panic with the three lines after it, and the
exit status.

`arvo_mutant_bits_index` sits in `Within::NEW`, and trybuild cannot see it, since it removes `RUSTFLAGS`
from its own build (`trybuild-1.0.120/src/cargo.rs`, `env_remove("RUSTFLAGS")`). The runner builds the
thirty-one index cases as examples of `index/`, whose `[[example]]` paths are the files in
`crates/arvo-bits/tests/ui/` themselves, once as shipped and once under the flag.

## What the runs show

| Flag | What it breaks | Caught by, first |
|---|---|---|
| `arvo_mutant_bits_bit` | reads whether any bit at or above the index is set | `the_bit_access`, bit 0 of `0x2` at two bits; and the constant test |
| `arvo_mutant_bits_with_bit` | writes a bit without clearing it first | `the_bit_access`, bit 0 of `0x1` written false at one bit |
| `arvo_mutant_bits_and` | ands with the complement of the other side | `the_bit_logic`, `0x1 and 0x0` at one bit; and the constant test's `APART` |
| `arvo_mutant_bits_or` | gives the exclusive or | `the_bit_logic`, `0x1 or 0x1` at one bit |
| `arvo_mutant_bits_xor` | gives the or | `the_bit_logic`, `0x1 xor 0x1` at one bit |
| `arvo_mutant_bits_not` | keeps the carrier's bits above the width | `the_bit_logic`, `not 0x0` at one bit |
| `arvo_mutant_bits_is_empty` | reads the low thirty-two bits only | `the_bit_logic`, `0x100000000` at thirty-three bits |
| `arvo_mutant_bits_shift` | keeps what a shift up moves past the width | `the_bit_sequence`, `0x2` up one at two bits |
| `arvo_mutant_bits_shift_down` | rotates rather than shifts | `the_bit_sequence`, `0x1` down one at two bits |
| `arvo_mutant_bits_rotate` | rotates up within the carrier, then masks | `the_bit_sequence`, `0x2` rotated up one at two bits; and the constant test's `TOP_SET` |
| `arvo_mutant_bits_rotate_down` | rotates down within the carrier, then masks | `the_bit_sequence`, `0x1` rotated down one at two bits |
| `arvo_mutant_bits_index` | admits an index or an amount equal to the width | the index cases: 31 refused as shipped, 28 of them building under the flag |

The eleven operation runs each exit 101, and the line each panics at is in its output.

Under the index flag three cases stay refused, and each says why. `a_bit_at_the_largest_index_is_refused`
and `a_shift_down_past_the_width_is_refused` are past the width rather than at it, so `I <= w` still
fails. `a_container_of_its_own_past_its_width_in_a_constant_is_refused` is let through by the proof and
then refused by the constant evaluation of the container's own array read, `index out of bounds: the
length is 2 but the index is 2`. Its twin at run time, `a_container_of_its_own_past_its_width_is_refused`,
builds under the flag: with the tier's guard moved, nothing stands between that container's
`self.0[I / 64]` and a bounds check at run time, which is the case the guard exists for.

The edge cases close what the second review found open. A guard refusing only `I == w`, and admitting
everything past the width, now fails the two cases past it; a guard that held at eight bits alone now
fails the eight cases at one bit and at sixty-four.
