# The crossing's mutant, against the tests on a value under a sibling signature

Taken in round 202611132028-qn1r, which adds `under`, `with_overflow` and `with_rounding` to the value.

Outcome: WORKS. The mutant turns all six tests of `tests/the_crossing.rs` red and leaves every test of
the four operations and of the debug check green.

The mutant is the feature `mutant-under` of `arvo-numeral`. In a crossing it sends the member at the
top of the range to the bottom and hands every other member over as it is. No test reads it.

`outputs/mutant-under.txt` is one run of

    cargo test -p arvo-numeral --features mutant-under --no-fail-fast

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, restricted to the library
tests and the binaries listed in the file, and trimmed to each binary's `Running` line, its result, its
failing tests, and each failure's panic message. Paths to the build directory, thread ids and backtraces
are dropped.

## What the run shows

The exhaustive test fails at its first signature, a one-bit signed integer whose top is slot 0, and
the fixed-point one at slot 31 of `Fixed<6, -3>`. Both fail on the slot, the first of the three things
asked of each member. The four named tests each cross the top of an 8-bit range and each reads the
wrong member afterwards.

The shorthands fail with it, so they go through `under` and have no body of their own to be wrong in.

## What it does not show

That a crossing wrong at a member other than the top would be caught by the named tests. The
exhaustive test asks every member, and the mutant moves one.

`compile_fail` and `a_consumer_crate` are not in the run. The first asks nothing of a crossing's value.
The second is red in the container this was taken in for a reason of the container's, the ticket
`the_container_git_config_rewrites_ssh_git_sources_to_https_a`.
