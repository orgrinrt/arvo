# Debug on every numeral shape: the check, kept where it cannot reattach a dead tier

**Outcome: INCONCLUSIVE, and it cannot be anything else here.** The check does
not run, because the crates it imports do not exist.

## What this is

`numeric_debug.rs` writes `{:?}` for a spread of widths and strategies into a
fixed-size `core::fmt::Write` buffer and asserts the output is non-empty. No
alloc, no std. It is a real end-to-end check on a real surface and it is worth
keeping.

It was written for round `202605111110` against a crate tree that has since been
deleted. It imports `arvo::strategy::{Additive, Identity, Warm}`,
`arvo::{FastFloat, Int, Uint}` and `arvo_storage::{Bits, Bool, Cap, USize}`, none
of which is in the working tree.

## Why it is here rather than under `mock/crates/`

It arrived as a pull request restoring the file to
`mock/crates/arvo/tests/numeric_debug.rs`. That path is inside the tier the canon
work deleted, and putting a file back into it is reattaching the tier that had to
be detached for the canon to be written at all. The repository's own instructions
say so in as many words: do not restore a crate here to make something build.

Two smaller things follow from the same fact and both matter more than the file
does:

- **It names `Warm` and `Additive` as though the strategy set were settled.** It
  is not. Op has stated the four inherited names are a prior attempt at the
  intent rather than the intent, and the number, the names and the decomposition
  are all open. A test asserting over `Warm` is a test asserting the answer to a
  question nobody has closed.
- **It names a crate decomposition** (`arvo`, `arvo_storage`) that the canon work
  is redeciding. Whether there is an `arvo-storage` at all is downstream of a
  question still open.

So the check is preserved and the tier is not. Nothing is lost: git holds the
original path, this file holds the reasoning, and the case comes back the moment
there is something to run it against.

## What has to be true before it runs again

The canon settles what a numeral is, a design derives the crates from it, and
those crates exist. Then this file moves into whichever crate ends up owning the
numeral surface, with its imports and its strategy names rewritten to whatever
the canon actually says. **The rewrite is not mechanical**: if the strategy set
changes shape, the spread of instantiations this test walks changes with it.

## The obligation underneath, which outlives the file

A consumer needs debug output from every numeral shape, at every width and under
every strategy, without alloc and without std. That is a demand on arvo whoever
implements it and whatever the crates end up being called, and it belongs in the
registry's `obligation` namespace rather than only here.
