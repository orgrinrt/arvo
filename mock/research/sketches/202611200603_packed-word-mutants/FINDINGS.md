# The red runs of the packed word's mutants

Round `202611200556-vw98` answers the first review of the packed word, which found that two hand mutants
the job had run were recorded nowhere. They are now flags in `src/word.rs`, off unless a build sets them
with `--cfg`, as the bit tier's are, and this sketch runs each against the word's law tests.

**Outcome:** WORKS. Both flags are caught, each by several tests, and each output names the assertions
that caught it.

## How it was run

`run.sh`, from `mock/`, on `nightly-2026-05-28`. Each flag runs as `RUSTFLAGS="--cfg <flag>" cargo test -p
arvo-bits --test the_packed_word --no-fail-fast`, and `outputs/<flag>.txt` keeps every line that is not a
pass or a compile line, with the exit status at the end. The `compile_fail` cases are not run under a
flag: neither flag touches a refusal, and `trybuild` removes `RUSTFLAGS` from its own build.

## What the runs show

| Flag | What it breaks | Red, of ten | Exit |
|---|---|---|---|
| `arvo_mutant_bits_word_get` | reads a field from one bit above its `BELOW` | seven: the three read-back tests, the two that pin raw bits, the constant test and the other-fields test | 101 |
| `arvo_mutant_bits_word_with` | writes a field without clearing its old bits | four: the three read-back tests and the 64-bit edges test | 101 |

The three tests the read mutant leaves green are the round trip of `of` and `bits`, the agreement of
`Lane` with `cast`, and the alias test, which compares two reads that move the same way. None of them
reads a field against an expected value.

The write mutant leaves the other-fields test green, as it must: a write that keeps the field's old bits
moves no bit outside the field. It also leaves the constant test and the 32-bit raw-bits test green,
because both build their words up from zero, where an or and a cleared write agree; the 64-bit edges test
writes over a word with bits already set where it reads, and catches it.
