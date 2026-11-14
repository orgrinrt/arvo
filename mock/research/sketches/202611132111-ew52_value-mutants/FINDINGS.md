# The mutant of the count, against its two targets

Taken in round 202611132111-ew52, which implements the count trait of `notko-hlist` for a value.

Outcome: WORKS. The mutant turns red every test that reads a count above one, on both paths, and
leaves green the one compile-fail pair that reads no count above one.

The mutant is the feature `mutant-count` of `arvo-numeral`, and no test reads it. It makes the
successor return a count that is not zero as it is, so every count stops at one.

`outputs/mutant-count.txt` is two runs,

    cargo test -p arvo-numeral --features hlist,mutant-count --test the_count_without_const
    cargo test -p arvo-numeral --features hlist-const,mutant-count --test the_count

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, trimmed to each
target's result, its failing tests, the compile-fail cases' verdicts, and each failure's panic
message. Paths to the build directory, thread ids, colour codes and backtraces are dropped.

## What the runs show

On the plain path all three tests fail: a list of two cells counts one, three cells in two bits count
one where three is asked, and four cells in two bits do not abort, since the count never reaches the
top.

On the const path the two tests that read lengths fail the same way, with four cells counting one as
a constant. The compile-fail pair for a length past the range fails as well: its refused half builds,
because a count that stops at one never leaves two bits. The pair for a format without a one stays
green, which is right, since the refusal there is the missing identity and is raised before any
count is taken.

## What they do not show

That a count is right in a signature the targets do not name. The impl is one line over `add` and the
two identities, each of which has its own evidence, so what could differ per signature is theirs.

Neither target runs in the default test run, since each is built only under its feature.
