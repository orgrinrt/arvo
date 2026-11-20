# Doc changelist: a packed word of named fields

**Date:** 2026-11-20
**Round:** `202611200533-wwxt`, job `arvo-packed-word`. Adds to `arvo-bits`, overhauls nothing.
**Topic:** `202611200533-wwxt_topic.a-packed-word-of-named-fields.md`, the round's one topic, every decision of it taken here.
**Sketch:** `mock/research/sketches/202611200520_packed-word/`, committed at `919c1ebe` before this changelist.

## `mock/registry/obligation.toml`

ADD: `obligation::a_word_divided_into_named_fields_of_declared_widths`, after `an_exact_width_container_a_consumer_can_alias_and_pin`, with `need`, `consumer = "hilavitkutin"`, a `quote` from `mock/crates/hilavitkutin-sym/DESIGN.md.tmpl` on hilavitkutin `dev`, `why`, `topic = "the_format"` and `keywords`. It states the need and names no mechanism (topic decision 1).

## `mock/crates/arvo-bits/DESIGN.md.tmpl`

CHANGE: the title names the packed word beside the bit tier.

ADD: the canon list at the top declares `obligation::a_word_divided_into_named_fields_of_declared_widths`, which the packed word meets (decision 1).

ADD: a section `## The packed word`, after the bit tier and before the byte run, saying: what the word is and why it sits here (decision 2); `Lane` and its two impls, `Bits<N>` through `cast` and `Bool` through the tier's `bit` and `with_bit` (decisions 4 and 5); `Layout`, `Field` with `BELOW`, and the list of fields (decision 3); `Word<L>`, `repr(transparent)`, with `of`, `bits`, `get` and `with`, each a `const fn` with its `[const]` bounds on the function, since an inherent impl takes none (decision 3, the sketch's first attempt); the four refusals and the `const` items forcing them (decision 6); the impls of `Word<L>` written by hand and bound on the container (decision 7); and the cost, stated as what the round's disassembly shows and nothing beyond it (decision 8).

CHANGE: `## What this crate does not ship` says the word is not a numeral and has no arithmetic over a field, no field wider than sixty-four bits, and no field at a shared placement read in place, the lens of the value-stance gap being a later round's (topic, "What the canon says").

CHANGE: `## The surface this design names` adds `Lane`, `Layout`, `Field` and `Word<L>`, which of them `Bits<N>` and `Bool` implement, and the impls of `Word<L>` (decision 3).

ADD: `## The laws this crate owes tests for` gains the word's laws: each field reads back what was written to it, at zero, at its maximum and at alternating bits, for every field of a layout; writing one field leaves every other field and the raw bits outside it unchanged; `of` then `bits` gives the container back and `bits` then `of` gives the word back; the handle's layout reads bit for bit what its fields say; every read and write evaluates in a constant; and the short sum, the overlap and the unlisted field are refused at `cargo build`, pinned as `trybuild` `compile_fail` cases beside a case that builds (decision 6).

## `mock/crates/arvo-bits/README.md.tmpl`

CHANGE: the summary names the packed word: a word of named fields of declared widths over `Bits`, each read and written in a constant, refused at build where the fields do not tile it.

## `mock/crates/arvo-bits/Cargo.toml`

CHANGE: the description names the packed word.

## Not changed

`mock/crates/arvo-bits/src/tier.rs` and the tier's sections of the design, held by `arvo-numeral-doors` (decision 2). `arvo-format`, whose `Bool` and `Width` are used as they stand.
