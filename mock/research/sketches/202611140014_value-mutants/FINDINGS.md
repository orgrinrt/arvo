# The mutants of the quotient, the product, the conversion and the door, against the sweeps at every shape

Taken in round 202611140014, which runs the quotient, the product, the conversion and the host door
at every width from 1 to 64 where the rounds before it ran them at chosen widths.

Outcome: WORKS, with two sweeps that no committed mutant turns red, named below with what holds each
of them instead.

Each file under `outputs/` is one run of

    cargo test -p arvo-numeral --features <mutant> --test <binary>

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, at source commit
`c97a1af5`, trimmed to the tests' lines, each panic with its assertion, and the result line. A file is
named for the mutant and the binary. With no mutant every one of these binaries is green.

## What the runs show

`mutant-div` turns every band of the quotient sweep red at each of the three radixes, twelve bands,
and the sweep of the integer points beside the binary ones. A band disagrees on between 558799 and
1432320 of its checks, and the integer points on 365 of 1375400, which is the lowest signed member
over minus one at each width.

`mutant-mul` turns the four bands of the binary product sweep red, on 13250 to 26560 checks a band.
The note of round 202611132216-pzi4 said no mutant reached the fractional product. This one reaches
the sweep at the shapes where the product at exponent zero is in the family.

`mutant-convert` turns red the rounded width matrix (four bands), the exponent pairs at widths 1 to
32 (two bands), the three tests of the source signatures and the six ordered radix pairs. It
disagrees on most checks of each, since it drops the two exponents.

`mutant-index` turns red the four bands of the host door and the two tests beside them, at minus 128
into an unsigned saturating range at every width.

## What they do not show

`mutant-convert` leaves the width matrix at exponent zero green, and should: with both exponents at
zero, dropping them changes nothing. What must fail there is the policy, and the matrix carries its
own case for it. On every pair of formats the wrapping conversion is held against the saturating
expectation, and the band asserts that this wrong expectation was caught wherever a member leaves the
target's range. That arm is in `tests/common/crossings.rs` as `Crossing::against`, and it runs in
every run of the matrix, so it is not a file here.

`mutant-index` leaves the sweep of where `from_index` aborts green, for the reason the earlier note
gives: it changes what the door completes to, and that sweep asks only whether the door aborts. No
committed mutant removes the check. The sweep asserts the abort on exactly the host integers outside
the range, and that some door of every signature aborts and some does not, so a check that never
fired or always fired would fail it. A check wrong at one width only has no red run here.

The decimal and ternary product, the wide half of the exponent pairs and the decimal and ternary
width matrices were not run under a mutant. They are the same bodies as the binaries that were, at
other formats.

All four mutants are in `arvo-numeral`. That the sweeps would see a wrong function in `arvo-format`
rests on the expected values being arithmetic of the tests' own, which `tests/the_oracle.rs` holds in
two integers against the host's where the host holds the answer.
