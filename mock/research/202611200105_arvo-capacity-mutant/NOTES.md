# The capacity count mutant

Under `--cfg arvo_mutant_capacity_count` the count `count` reads is one past the array a capacity
projects (`N.wrapping_add(1)`, so the largest `usize` wraps to zero rather than failing the build).

`agreement-mutant-count.txt`, round `202611200049-ol81`, is the agreement binary alone: 4 passed,
3 failed, the three that read the count.

`crate-mutant-count.txt`, round `202611200102-e4xa`, is the whole crate. Every test that reads the count
is red: in `agreement`, the spread and the edges (the constant-read test it once had is gone, covered by
those two); in `a_consumer_crate`, the consumer's build, refused at its `agrees` assertions with
"the capacity's array and its count disagree". The tests that do not read the count stay green: the
filling and nesting tests, the lockfile check, and the compile-fail pairs, whose twins trybuild builds
without the mutant cfg.

Both supersede the run committed under the sketch
`research/sketches/202611200010_capacity-spellings/out/crate-arvo-capacity-mutant-count.txt` for the
crate as it now stands.
