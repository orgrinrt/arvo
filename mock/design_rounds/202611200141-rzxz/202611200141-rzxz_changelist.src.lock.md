# Src changelist: the capacity crate after its third review

**Date:** 2026-11-20
**Round:** `202611200141-rzxz`, job `arvo-capacity`.
**Doc changelist:** `202611200141-rzxz_changelist.doc.lock.md`.
**Topics consumed:** `202611200141-rzxz_topic.arvo-capacity-third-review.md`, D1 to D4.

## Source

`mock/crates/arvo-capacity/src/capacity.rs`: the rustdoc on `seal::Len` says what the private length
closes and that `count` is this crate's reader of it (D1).

`mock/crates/arvo-capacity/src/lib.rs`: `#![forbid(unsafe_code)]` (D4).

## Tests

`tests/compile_fail.rs` and `tests/ui/`: a new pair, `C::LEN.0` read from outside refused (E0616)
beside the read through `count` (D2).

`tests/agreement.rs`: `a_nested_array_of_a_non_copy_element_is_built_in_place` becomes
`a_nested_filled_with_makes_its_elements_row_by_row_in_order`, numbering every element from one counter
and asserting six calls and the rows in order (D3).
