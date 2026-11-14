# The mutants of the quotient, the conversion and the product, against the rewritten tests

Taken in round 202611132143-c8zp, after the sweeps of `div`, `convert` and the product stopped
comparing the value with the function of `arvo-format` it calls and started comparing it with
arithmetic written in the test crate, `tests/common/oracle.rs`.

Outcome: WORKS for what a mutant exists for, and one path has no mutant. `mutant-div`,
`mutant-convert` and `mutant-mul` each turn red the sweep of their own operation against the new
oracle. `mutant-mul` sits on the machine-integer path and does not reach the product at a non-zero
exponent, so `fixed_point_product` stays green under it, as it has to.

Each file under `outputs/` is one or two runs of

    cargo test -p arvo-numeral --features <mutant> --test <binary>

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, at the source commit
of the round, trimmed to the failing tests, each failure's panic message and the result line. Thread
ids, build lines and one unrelated dead-code warning from `arvo-format` are dropped.

## What the runs show

`mutant-div` is seen on the same quotients as before the rewrite: twenty in each integer sweep and
2454 of 7040 at the fixed-point formats. The count is now the number of quotients on which the value
differs from a rounded quotient the test computes with `div_euclid` and completes by the policy, and
no function of `arvo-format` that computes is on the expected side.

`mutant-convert` is seen on 3754 of 3840 members between two exponents and on 5352 of 6400 between
an integer and a fixed-point grid, against a position the test scales and rounds itself.

`mutant-mul` is seen on 197950 of 1747600 pairs at widths one to eight, where `check_product` now
reads the oracle for the slot and for `fits_mul`.

With no mutant every one of these sweeps is green, which is the other half of what was asked: the
format's quotient, conversion and scaled product agree with the tests' arithmetic on every case
swept.

## What they do not show

No mutant breaks the product at a non-zero exponent, because that product is the format's function
called with nothing between and the mutants are features of `arvo-numeral` alone. What stands in for
one is the control in `fixed_point_product`, which counts the pairs on which the slots' product with
no scaling differs from the value's and requires some at each format, so the oracle the sweep reads
is known to differ from an answer that left the exponent out.

That the oracle is right. It is a second derivation of the same arithmetic, short enough to read,
and the named cases beside each sweep are literals written by hand. An error the oracle and the
format shared would pass.

The oracle asserts a radix of two and is not written for another. The decimal radix is not swept by
any value test.
