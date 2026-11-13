# Doc changelist: the consumer test takes its paths when it runs

**Date:** 2026-11-13
**Round:** 202611132123-jozr, branch `work/arvo-values`.
**Topics consumed:** `202611132123-jozr_topic.the-consumer-test-takes-its-paths-when-it-runs.md`, the
only flat topic on this branch or on `origin/dev`.
**Sketches:** none. The source changelist's test is the check.
**Scope:** one sentence of `mock/crates/arvo-numeral/DESIGN.md.tmpl`. No change to `arvo-format` and
no registry file.

## Canon

None is read for the change. `proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature`,
on a provisional stamp, is what the crate is built on, and is not promoted.

## CHANGE: `arvo-numeral`, how the consumer test finds the consumer

File: `mock/crates/arvo-numeral/DESIGN.md.tmpl`, `## What is proved, and how`.

The paragraph on the consumer crate says a test builds and runs it. It gains the statement that the
test takes the consumer's path and its scratch directory when it runs, so that a build directory
shared between checkouts tests the checkout the run was started in.

## What the source changelist will carry

`tests/a_consumer_crate.rs`: the two paths read at run time, and one test that the paths read are the
run's own.
