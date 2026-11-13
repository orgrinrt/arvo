# Each rung's mutant, against the tests on the reworked value

Taken again in round 202611130720-wub5, after the fixed-point pass joined the band of widths 1 to 8, the
release twin of the debug check joined the suite, and a compile-fail pair joined the list; the shape is
unchanged otherwise.

Outcome: WORKS. Each of the four mutants turns the tests that run through its rung red, and leaves the
rest green, on the surface where the value reads its container from the placement's `SoleCarrier`, the
sign from `Sole::SIGNED`, and takes its debug check as a second parameter.

This takes again the runs of `202611130513-jy34_value-mutants`, whose outputs were taken while the value
kept its own carrier table and the plain markers carried the abort. The mutants are the same: the
features `mutant-8`, `mutant-16`, `mutant-32` and `mutant-64` of `arvo-numeral`, each removing that
rung's adaptation onto the declared width. No test reads them.

Each file under `outputs/` is one run of

    cargo test -p arvo-numeral --features mutant-N --no-fail-fast

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, trimmed to each binary's
`Running` line, its result, its failing tests, and each failure's panic message. Paths to the build
directory, thread ids and backtraces are dropped.

## What each run shows

`mutant-8` fails both rung tests at widths 1 to 8, three tests of the band of widths 1 to 8 (the
exhaustive pass, the fixed-point pass at `UFixed<6, 3>` and the saturating associativity counts), and one test of the debug check binary,
`add_completes_under_quiet`, whose signature sits at width 4 in the 8-bit carrier. The `Abort` tests
still fire, because they ask the format's verdict before the rung runs.

`mutant-16`, `mutant-32` and `mutant-64` each fail the four rung tests, the two at widths 1 to 8 and the
two at widths 9 to 12, and the one test of their own band.

No run fails a test whose signatures avoid the mutated rung. `a_consumer_crate` passes under every
mutant, because the consumer depends on the crate with no feature line, and `compile_fail` passes
under every mutant, since no refusal depends on a rung.

## What it does not show

That a mutant of another kind would be caught. These four remove the adaptation and nothing else. The
debug check's `ABORTS` and the admission's whole-range test were each mutated by hand during the round
and caught by `the_debug_check` and the compile-fail pairs, and those runs are not committed here.
