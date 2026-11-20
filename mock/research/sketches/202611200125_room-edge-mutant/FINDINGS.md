# The red run of the room's bound past its edge

Round `202611200114` adds the flag `arvo_mutant_room_edge` to `arvo-numeral`'s `room.rs`. Under it,
`holds` admits a ratio whose quotient reaches up to `2^128` rather than one below `2^127`, so the value forms
the product, the quotient and the conversion in the host's integer past the edge, one radix power at
radix two, where the floor no longer fits the index. At radix ten and three it does so wherever the next
power lands below `2^128`, which the run shows it does at the edges the tests take. The shape
suites cannot see this, since none of their exponents is past the edge, and the first mutant of the room
(`202611192130-room_value-mutant`) never crossed it. This sketch holds one red run of the new flag against
the two tests that do: the `bound` module, which holds the predicates at the edge, and
`the_conversion_at_the_room_edge`, which converts onto the first exponent past it.

Outcome: WORKS. The flag fails both, at all three radixes.

## How it was run

`run.sh` calls the crate's mutant runner, `../202611140253-gl7r_value-mutants/run.sh`, with the one name
`arvo_mutant_room_edge` and `MUTANT_OUT` set to this sketch's `outputs/`. The runner runs it from `mock/` as
`RUSTFLAGS="--cfg arvo_mutant_room_edge" cargo test -p arvo-numeral --no-fail-fast --lib --test
the_conversion_at_the_room_edge` and keeps the run less its passing lines in
`outputs/arvo_mutant_room_edge.txt`.

## What the run shows

In the library's tests three of eighteen fail, all three in `room::bound`: the conversion's edge at 64
bits, the quotient's edge at 64 bits, and the edges of the product, the quotient and the conversion at 8,
16, 32 and 64 bits over the three radixes. The product's predicate is not the one the flag moves, since past
its edge the radix power itself leaves 128 bits and `holds` is never reached with a numerator; the test of it
fails on the quotient and the conversion it holds in the same macro. `the_conversion_at_the_room_edge` fails
its binary, decimal and ternary tests, where converting the range's ends onto the first exponent past the
edge disagrees with the oracle; the three that pass in that binary are the shared self-tests of
`common::big`.
