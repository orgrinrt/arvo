# The mutants of the quotient and the conversion, against the wide sweeps

Taken in round 202611132216-pzi4, which adds sweeps of `div` and `convert` on the edge lattice and
seeded members at widths up to 64 bits.

Outcome: WORKS. `mutant-div` turns the wide quotient sweep red and `mutant-convert` the wide
conversion sweep. With no mutant both are green, as is the wide sweep of the fractional product.

Each file under `outputs/` is one run of

    cargo test -p arvo-numeral --features <mutant> --test <binary> <the wide test>

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, at the source commit
of the round, trimmed to the failing test, its panic message and the result line.

## What the runs show

`mutant-div` is seen on 655250 of 2730960 quotients over six integer formats of 13, 32 and 64 bits
and four fixed-point ones of 16, 32 and 64. `mutant-convert` is seen on 207110 of 539800 members
over thirteen pairs with a source of 32 or 64 bits.

## What they do not show

No mutant reaches the fractional product, for the reason the note of round 202611132143-c8zp gives:
that product is the format's function and the mutants are this crate's. Its wide sweep is green
against the tests' arithmetic and has no red run.

Both mutants are in `arvo-numeral`, so these runs show the sweeps see a value that calls the
format's function wrongly. That the sweeps would see a wrong function in `arvo-format` rests on the
oracle being arithmetic of the tests' own, which is read and not run here.

`mutant-index` does not turn red the new test of where `from_index` aborts, and should not: it
changes what the door completes to, and the test asks only whether the door aborts.
