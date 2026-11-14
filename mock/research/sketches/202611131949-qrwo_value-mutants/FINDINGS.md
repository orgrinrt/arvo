# Each operation's mutant, against the tests on the ring operations

Taken in round 202611131949-qrwo, which adds the difference, the negation and the product to the value.

Outcome: WORKS. Each of the three mutants turns red the tests that ask its operation of the oracle, and
leaves the tests of the other two operations and of the sum green.

The mutants are the features `mutant-sub`, `mutant-neg` and `mutant-mul` of `arvo-numeral`. Each changes
one arm of one operation in every rung. `mutant-sub` makes the unsigned saturating difference return the
top of the range where the subtrahend is the larger. `mutant-neg` makes the signed saturating negation
wrap where the declared width is the carrier's. `mutant-mul` pins every signed saturating product that
leaves the range to the top, whichever side it left by. No test reads them.

Each file under `outputs/` is one run of

    cargo test -p arvo-numeral --features mutant-NAME --no-fail-fast

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, restricted to the library
tests and the binaries listed in the file, and trimmed to each binary's `Running` line, its result, its
failing tests, and each failure's panic message. Paths to the build directory, thread ids and backtraces
are dropped.

## What each run shows

`mutant-sub` fails the two unsigned rung tests of the library, the sum-and-order test of each band,
which is where the difference is asked on every pair, and the fixed-point pass at `UFixed<6, 3>`. The
count is in each message, 217175 of 1747600 pairs at widths 1 to 8.

`mutant-neg` fails the signed rung test at widths 1 to 8 and the sum-and-order test of each band, on 5
members in each, which the tally counts and does not name; the mutated arm is reached only by the
bottom of a range whose declared width fills its carrier. The signed rung test at widths 9 to 12 stays green,
since none of those widths fills the 16-bit carrier.

`mutant-mul` fails the two signed rung tests and the product test of each band. The fixed-point product
binary stays green under it, because a product at a non-zero quantum exponent goes through the format's
own multiplication and does not reach the rung.

Under every mutant the debug-check binary passes, because its cases sit away from the mutated arms, and
the product tests pass under `mutant-sub` and `mutant-neg`.

## What it does not show

That a mutant of another kind would be caught. Each of the three changes one arm under one overflow
policy and one sign, so an error in a wrapping arm, or in the other sign's saturating arm, is not shown
caught by these runs.

`compile_fail` and `a_consumer_crate` are not in these runs. The first asks nothing of a rung. The
second is red in the container this was taken in for a reason of the container's, the ticket
`the_container_git_config_rewrites_ssh_git_sources_to_https_a`, and says nothing about a mutant.

The product through the format's multiplication has no mutant here. What `fixed_point_product` carries
in its place is a control inside the binary, a product that ignores the exponent, which the same pairs
reject.
