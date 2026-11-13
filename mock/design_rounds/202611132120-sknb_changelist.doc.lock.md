# Doc changelist: landing a point on a format's magnitudes

**Date:** 2026-11-13
**Round:** `202611132120-sknb`, job `arvo-format`.
**Topics consumed:** `202611132120-sknb_topic.landing-a-point-on-a-format-s-magnitudes.md`, the only
flat topic.
**Scope:** `mock/crates/arvo-format/DESIGN.md.tmpl`.

## Canon

`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` makes a result's place
a function of the declared signature. A magnitude-indexed format's signature declares a union of grids,
so where a value lands among them is a function of it as well, and the design names that function rather
than leaving it to each caller. `ruling::never_a_runtime_check_and_one_lowered_path` is why the nesting
the landing rests on is a consequence of the two obligations and not a check. The canon is silent on what
completion means past the top of a union of magnitude ranges, and the landing leaves that region empty
under the open ticket rather than filling it.

## CHANGE: the IEEE section names the landing

In `### The rounding directions, and where a result lands`, the paragraph saying "where a result lands
is the caller's composition" says instead that `land` answers it, and keeps the argument that the finest
magnitude in range is the standard's. The pointer to the sketch where it was first shown stays.

## CHANGE: a section on landing, before the open items

A new `## Landing a point on a format's magnitudes` before `## Open items`, with the subsections the topic
derives: the finest grid in range as the nearest member, and why nothing checks the nesting; the position
at a magnitude; where the walk starts; what it does not land, the region past the top under
`arvo_landing_a_value_on_a_magnitude_indexed_format_is_a_call`; the laws it owes tests for; and the
surface it names, `Scaled`, `Scaled::of`, `Landing`, `Landing::magnitude`, `Landing::exact` and `land`.
