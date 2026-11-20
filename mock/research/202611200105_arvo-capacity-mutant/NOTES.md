# The capacity count mutant, round 202611200049-ol81

Under `--cfg arvo_mutant_capacity_count` the count `count` reads is one past the array a capacity
projects (`N.wrapping_add(1)`, so the largest `usize` wraps to zero rather than failing the build).
The run in `agreement-mutant-count.txt` turns red the three tests that read the count and leaves the
four that do not read it green: 4 passed, 3 failed. It supersedes the run committed under the sketch
`research/sketches/202611200010_capacity-spellings/out/crate-arvo-capacity-mutant-count.txt` for the
crate as it now stands.
