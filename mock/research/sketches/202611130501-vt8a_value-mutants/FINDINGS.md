# Each rung's mutant, against the tests as they stand

Outcome: WORKS. Each of the four mutants turns the tests that run through its rung red, and leaves the
rest green.

`arvo-numeral` carries one mutant per rung, the features `mutant-8`, `mutant-16`, `mutant-32` and
`mutant-64`, each removing that rung's adaptation onto the declared width. The tests read none of these
features: every one asserts agreement with `arvo_format::add`, the slot order or the specified panic,
under every build. So a mutant run is red where a test computes through the mutated rung, and the red
output is what shows the tests notice the mutant.

Each file under `outputs/` is one run of

    cargo test -p arvo-numeral --features mutant-N --no-fail-fast

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, trimmed to each binary's
`Running` line, its result, its failing tests, and each failure's panic message and values. Paths to the
build directory, the hash on each test binary and the thread ids are dropped.

## What each run shows

`mutant-8` fails both rung tests at widths 1 to 8, three tests of the band of widths 1 to 8 (the
exhaustive pass, the saturating associativity counts and `from_raw` at width 4), and four tests of the
assertion binary, whose signature sits at width 4 in the 8-bit carrier: `from_raw` asserts under both
plain markers no longer fire, and the completing twins of `add` and `from_raw` no longer complete.

`mutant-16`, `mutant-32` and `mutant-64` each fail the four rung tests, the two at widths 1 to 8 and
the two at widths 9 to 12, and the one test of their own band. Each rung test names the rung that
disagrees and how many pairs did; each band names how many of its pairs disagreed.

No run fails a test whose signatures avoid the mutated rung, and `a_consumer_crate` passes under every
mutant, because the consumer depends on the crate with no feature line and so builds without one.

## What it does not show

That a mutant of some other kind would be caught. These four remove the adaptation and nothing else; a
mutant of the dispatch, the bit copy or the comparison is not among them.
