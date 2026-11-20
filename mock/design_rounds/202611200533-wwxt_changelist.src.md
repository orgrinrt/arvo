# Src changelist: a packed word of named fields

**Date:** 2026-11-20
**Round:** `202611200533-wwxt`, job `arvo-packed-word`. Adds to `arvo-bits`, overhauls nothing.
**Doc changelist:** `202611200533-wwxt_changelist.doc.lock.md`.
**Topic:** `202611200533-wwxt_topic.a-packed-word-of-named-fields.md`.
**Sketch:** `mock/research/sketches/202611200520_packed-word/`, committed at `919c1ebe`.

## `mock/crates/arvo-bits/src/word.rs`

ADD: the module, as the design's section "The packed word" states it and the sketch built it. `pub const trait Lane: Copy` with `WIDTH: Width`, `from_low(Bits<64>) -> Self` and `low(self) -> Bits<64>`; `const impl<const N: u32> Lane for Bits<N>` through `cast`; `const impl Lane for Bool` through `bit::<0>` and `with_bit::<0>` (topic decisions 4 and 5). `pub trait Field` with `Layout`, `Value: Lane` and `BELOW: Width`; `pub trait Layout` with `Container: Lane` and `Fields`; the list traits folding a list into its sum, union and overlap, and asking it whether it holds a field at a place and width, implemented for `()` and `(H, T)` (decision 3). `pub struct Word<L: Layout>(L::Container)`, `repr(transparent)`, with `of`, `bits`, `get` and `with`, each a `const fn`, the `[const] Lane` bounds on `get` and `with` (decision 3); the tiling and listing assertions in `const` items forced by every entry point (decision 6); `Clone`, `Copy`, `PartialEq`, `Eq` and `Debug` by hand, bound on `L::Container` (decision 7). The carrier's own `u64` appears inside the module's bodies and constants only, as everywhere in this crate.

## `mock/crates/arvo-bits/src/lib.rs`

ADD: `mod word;` and `pub use word::{Field, Fields, Holds, Lane, Layout, Word};`, the list traits public because a consumer's `Layout::Fields` names a list that must implement them. CHANGE: the crate doc names the packed word.

## `mock/crates/arvo-bits/tests/the_packed_word.rs`

ADD: the word's laws, as the design's laws section states them, over the handle layout (28, 3 and a one-bit `Bool` in a `Bits<32>`, listed out of bit order) and its wide-tag twin (26, 5 and 1): each field round-trips at zero, at its maximum and at alternating bits; writing one field keeps every other field and the bits outside it; `of` and `bits` invert each other; the handle's raw bits hold each field where its layout puts it; every read and write evaluated in a constant; `Lane` agreeing with `cast` and with `bit` and `with_bit` at index zero.

## `mock/crates/arvo-bits/tests/ui/` and `tests/compile_fail.rs`

ADD: `a_packed_word_whose_fields_fall_short_is_refused`, `a_packed_word_whose_fields_share_a_bit_is_refused`, `a_field_its_layout_does_not_list_is_refused`, each in a constant, and `a_call_of_get_on_a_short_word_is_refused`, `a_call_of_with_on_a_field_not_listed_is_refused`, each a call at run time, with their `.stderr`; and `a_packed_word_that_tiles_builds` among the cases that must build. Each named in `REFUSED` or `BUILT`.

## `mock/benches/packed-word/`

ADD: a bench of two arms, `packed-word-host`, the handle's fields read and written by hand on a `u32`, and `packed-word-word`, the same through `Word<L>`, each exporting the round trip to the container, one field's read and one field's write as `extern "C"` functions, with a loop over records for the timing the harness requires. Its disassembly, `cargo mock bench disasm`, is the evidence the design's cost paragraph points at, written under `mock/benches/results/packed-word/disasm/` and committed (decision 8).

## `mock/test-history/default/arvo-bits.json`, `docs/`

CHANGE: what `cargo mock test` and `cargo mock` write for the above.

## `mock/research/202611200533_packed-word/REPORT.md`

CHANGE: the job's report gains the rounds, the tests, the disassembly and what is left.
