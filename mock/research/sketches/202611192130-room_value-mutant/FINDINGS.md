# The red run of the ratio formed in the host's integer

Round `202611192047` forms the exact position of the value's product at a non-zero exponent, of its
quotient and of its conversion in the host's 128-bit integer where a constant bound over the range
holds, in `arvo-numeral`'s `room.rs`, and adds the flag `arvo_mutant_room`. Under the flag a negative
ratio that leaves something over is floored to the negated quotient with no step down, the floor a cast
toward zero gives, and the residue is counted from the slot above as before. This sketch holds one red
run of the flag against every suite that reaches the arm.

Outcome: WORKS. The flag fails the tests of the product, the quotient and the conversion that cross a
negative non-integral ratio, and the test that divides a value by a value read at run time.

## How it was run

`run.sh` calls the crate's mutant runner, `../202611140253-gl7r_value-mutants/run.sh`, with the one
name `arvo_mutant_room` and `MUTANT_OUT` set to this sketch's `outputs/`. The runner runs it from
`mock/` as `RUSTFLAGS="--cfg arvo_mutant_room" cargo test -p arvo-numeral --no-fail-fast --test
the_quotient --test the_quotient_by_a_value --test fixed_point_product --test the_conversion` and keeps
the run less its passing lines in `outputs/arvo_mutant_room.txt`.

## What the run shows

`fixed_point_product` fails 2 of its 8 tests, the signed product at eighths on every pair and the tie
below zero under each mode. `the_conversion` fails 4 of its 10, the fixed point between two exponents,
the integer onto a fixed-point grid and back, the named conversions and the wide members on the lattice.
`the_quotient` fails 2 of its 5, the named quotients and every member over every non-zero member at
widths 1 to 4. `the_quotient_by_a_value` fails its sweep, the third of its three tests; the other two in
that binary are the shared self-tests of `common::big`.

The tests that pass under the flag are the ones whose ratios are never negative with a residue: a
non-negative sweep, a constant whose quotient is on the grid, a conversion that only widens.
