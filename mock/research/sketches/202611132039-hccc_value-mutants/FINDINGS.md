# The mutants of the quotient, the conversion and the host door, against their tests

Taken in round 202611132039-hccc, which adds `div`, `convert` and the door for a host integer to the
value.

Outcome: WORKS. Each of the three mutants turns red the tests of its own operation and leaves the
other two binaries of the round green.

The mutants are three features of `arvo-numeral`, and no test reads any of them. `mutant-div` returns
the dividend where the quotient leaves the range. `mutant-convert` reads the source's slot index as
the target's. `mutant-index` makes the saturating door send a negative host integer to the top of an
unsigned range.

Each file under `outputs/` is one run of

    cargo test -p arvo-numeral --features <mutant> --no-fail-fast \
        --test the_quotient --test the_conversion --test the_host_door

from `mock/`, in the dev profile, on the toolchain in `outputs/toolchain.txt`, trimmed to each
binary's `Running` line, its result, its failing tests, and each failure's panic message. Paths to the
build directory, thread ids and backtraces are dropped.

## What the runs show

`mutant-div` fails four of the five quotient tests. On the integers it is wrong on twenty quotients
in each sweep, the lowest signed member over minus one under saturation in each of the five modes at
each of four widths, which is the whole of where an integer quotient by a member leaves the range.
Under wrapping that quotient wraps back onto the dividend, so the mutant agrees there, and the named
test says so by asking for `-128` under wrapping and `127` under saturation. On the fixed-point
formats it is wrong on 2454 of 7040, since a quotient by a small fraction leaves the range often.

`mutant-convert` fails every test that crosses two exponents and passes the two that do not: between
integers the slot index is the position, so reading it as the target's is right, and over one format
the conversion is the crossing.

`mutant-index` fails the two sweeps at their first negative host integer into an unsigned saturating
range, `-128` as an `i8`, and the named test at `-1`. The outward door and the constant test take no
negative host integer and pass.

## What they do not show

That the quotient or the conversion is right on a signature the sweeps do not name: widths above 8
for the quotient, and conversions other than the fourteen pairs listed in the test. Both call the
format's own function with nothing between, so what could differ is the call's arguments, and the
sweeps and the named cases cover those.

The run is restricted to the round's three binaries. The debug check's binary takes these operations
under `Quiet` and under `Abort` on inputs the mutants also change, and is not in these files.
`mutant-64`, the 64-bit rung's, removes the adaptation the host door calls and so turns the door's
sweeps red as well; that is its own claim at one more caller and is not rerun here.
