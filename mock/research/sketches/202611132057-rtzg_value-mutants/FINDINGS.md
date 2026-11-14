# The mutant of the accumulator's capacity, against its tests

Taken in round 202611132057-rtzg, which adds the fixed-point aliases and the exact accumulator to the
value.

Outcome: WORKS. The mutant turns red the three tests that hand `take` a slice longer than the
capacity and leaves the other seven of the binary green.

The mutant is the feature `mutant-accumulator` of `arvo-numeral`, and no test reads it. It makes
`take` sum the whole slice whatever the capacity, which is the fold written by hand that
`probe::a_runtime_length_fold_compiles_at_every_length_and_computes_at_one` measured.

`outputs/mutant-accumulator.txt` is one run of

    cargo test -p arvo-numeral --features mutant-accumulator --test the_accumulator

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, trimmed to the binary's
`Running` line, its result, its failing tests, and each failure's panic message. Paths to the build
directory, thread ids and backtraces are dropped.

## What the run shows

`the_capacity_bounds_what_a_slice_puts_in` reads 224 where it asks for 240. Thirty-two fifteens are
480, the 8-bit wrapping accumulator holds that as 224, and the capacity of sixteen is what keeps the
first block at 240 with sixteen terms handed back. This is the probe's number through the real type.

`take_sums_a_block_and_hands_back_the_rest` fails at the first slice longer than its capacity of
eight, nine terms, whose whole sum is 0 where the first eight give -4.
`an_accumulated_sum_is_a_constant` fails the same way in a constant, 110 for three terms where the
first two give 200.

## What it does not show

That the bound on the accumulator's signature is right. The mutant leaves the bound alone and breaks
what the bound is stated over, the number of terms that enter. The bound has its own evidence: the
compile-fail pair that admits eight bits over four-bit terms at sixteen and refuses them at
thirty-two, and the sums at capacity of the highest and of the lowest member in the test binary.

The run is one binary. `of` and `plus` keep the capacity by a constant the build evaluates, so no
feature can lift it at run time, and their refusals are held as compile-fail pairs.
