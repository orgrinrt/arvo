# Sketch: does a width above the bound refuse with something a reader can act on

**Hypothesis.** A declared width wider than a slot index carries should be refused at compile time
with a message naming the width and the bound, per
`ruling::never_a_runtime_check_and_one_lowered_path`, rather than producing a raw arithmetic error or
a wrong value at runtime.

Cited by `mock/design_rounds/202608312054_changelist.src.md`.

## Outcome

**REFUSES, and names why.** `output.txt` carries the diagnostic. It names the bound, the reason the
bound is 62, and the instantiation that failed.

## What it was before

Two failures, both reproduced before the change:

`Signed<64>` gave `error[E0080]: attempt to negate i64::MIN`, an arithmetic error about a constant
with nothing in it about widths.

`Signed<63>` **compiled**, panicked in debug, and in release returned a derived width of **zero bits**
for a declared 63-bit numeral, because the slot count wrapped negative and the deriving loop never
entered. That is the case this round exists for: a wrong answer produced at runtime where a refusal
belonged.

## What must fail

The consumer here instantiates `Signed<63>`. If it ever builds, the bound is not being enforced and
this sketch establishes nothing. The control is that widths at and below the bound build, which the
crate's own 37 tests cover across 1 through 62.

## Toolchain

The repository's pinned nightly, copied in as `rust-toolchain.toml` so the sketch resolves the same
way the workspace does.
