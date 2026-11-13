# Doc changelist: the IEEE suites land through `land`

**Date:** 2026-11-13
**Round:** `202611132305-wqfi`, job `arvo-format`.
**Topics consumed:** `202611132305-wqfi_topic.the-ieee-suites-land-through-land.md`, the only flat topic
on this branch and on every remote ref.
**Scope:** `mock/crates/arvo-format/DESIGN.md.tmpl`.

## Canon

The canon says nothing about how a test harness reaches the crate. The change is to what the landing
section says it owes tests for, which is design.

## CHANGE: the landing's owed tests drop the agreement with the suites' search

The last sentence of "The laws this section owes tests for" under the landing, "And the IEEE parity
suites' own search against `land` over points drawn across each format's exponent range, every
direction.", becomes: "And the IEEE parity suites land through `land` itself, so the host's IEEE
arithmetic checks it over points drawn across each format's exponent range, every direction; only the
overflow region, which `land` does not answer, is adapted at the top magnitude by the suites."
