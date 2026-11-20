# Doc changelist: arvo takes a capacity

**Date:** 2026-11-20
**Round:** `202611200029-bfwv`, job `arvo-capacity`.
**Topics consumed:** `202611200029-bfwv_topic.arvo-capacity.md`, the only flat topic on this branch, D1 to D5.
**Scope:** a new crate's design and summary, and the root documents that count or list the crates.

## Canon

`obligation::a_count_fixed_at_build_that_sizes_arrays_from_generic_code` is the row the crate answers,
a proposal asked of op as `does_the_capacity_obligation_row_stand_as_arvo_s_need`. No ruling places a
capacity crate or decides its seal; the seal stands in for `is_arvo_s_capacity_trait_sealed_to_its_one_marker`.

## Sketch

`mock/research/sketches/202611200010_capacity-spellings/`, committed on this branch in `fcb300a1`
before this changelist, with `FINDINGS.md` and every attempt's output under `out/`.

## CREATE: `mock/crates/arvo-capacity/README.md.tmpl`

The summary: a count fixed at build carried as a type, `Capacity` and `Dim<N>`, bundling and nesting,
`count` in a constant as a `USize` under the consumer's adaptation, no feature line, depends on
`arvo-format` and `arvo-numeral`.

## CREATE: `mock/crates/arvo-capacity/DESIGN.md.tmpl`

Topic D2 to D5 as the shipping contract: the trait and its projection with the slice bridge and
`filled`, bundling and nesting with no product, the `Copy` residue the consumer states, the seal and
why it stands in, the marker and its one bare integer at a const parameter, `count` and why the crate
picks no adaptation for it, what the crate does not hold, and the tests.

## CHANGE: `mock/DESIGN.md.tmpl`

Six crates become seven: the topology paragraph names `arvo-capacity` on `arvo-numeral`, the table
gains its row, the dependency paragraph says what it depends on and why, and the consumers paragraph
says a count fixed at build is `arvo-capacity`'s, new source rather than the removed tensor crate.

## CHANGE: `mock/agent/MAIN.md.tmpl` and `mock/agent/PREAMBLE.md.tmpl`

The tree holds seven crates, `arvo-capacity` among them, and the sentence saying there is no capacity
type yet goes.

## CHANGE: `README.md`

One sentence after the digest's, saying what `arvo-capacity` carries.
