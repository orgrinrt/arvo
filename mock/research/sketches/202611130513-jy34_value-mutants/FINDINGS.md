# Each rung's mutant, against the tests on a surface with no carrier door

Outcome: WORKS. Each of the four mutants turns the tests that run through its rung red, and leaves the
rest green, on the surface where `from_raw` and `raw` are gone and the carrier cannot be named.

This takes again the runs of `202611130501-vt8a_value-mutants`, whose outputs were taken while
`from_raw` existed and counted its tests among the red ones. The mutants are the same: the features
`mutant-8`, `mutant-16`, `mutant-32` and `mutant-64` of `arvo-numeral`, each removing that rung's
adaptation onto the declared width. No test reads them.

Each file under `outputs/` is one run of

    cargo test -p arvo-numeral --features mutant-N --no-fail-fast

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, trimmed to each binary's
`Running` line, its result, its failing tests, and each failure's panic message and values. Paths to the
build directory, thread ids and backtraces are dropped. The last result line of each file is the
doc-test binary, which holds no tests.

## What each run shows

`mutant-8` fails both rung tests at widths 1 to 8, two tests of the band of widths 1 to 8 (the
exhaustive pass and the saturating associativity counts), and one test of the assertion binary, the
completing twin of `add`, whose signature sits at width 4 in the 8-bit carrier. The two `add`
assertions under the plain markers still fire, because they ask the format's verdict before the rung
runs, and `adapted` stays green, because it reaches the carrier through the format's own adaptation
and not the rung's.

`mutant-16`, `mutant-32` and `mutant-64` each fail the four rung tests, the two at widths 1 to 8 and
the two at widths 9 to 12, and the one test of their own band. Each rung test names the rung that
disagrees and how many pairs did; each band names how many of its pairs disagreed.

No run fails a test whose signatures avoid the mutated rung. `a_consumer_crate` passes under every
mutant, because the consumer depends on the crate with no feature line and so builds without one, and
`compile_fail` passes under every mutant, since no refusal depends on a rung.

## What it does not show

That a mutant of some other kind would be caught. These four remove the adaptation and nothing else; a
mutant of the dispatch, the bit copy, the slot read behind `Hash` and `Debug`, or the comparison is not
among them.
