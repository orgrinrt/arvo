# The red runs of the member step's two flags

Round `202611200734` moves the exact step formed in the host's integer out of `arvo-numeral`'s `room.rs` and
into `arvo-format`, as `Member::product`, `Member::quotient` and `Member::converted` in `src/member/step.rs`.
The two mutant flags `room.rs` carried move with it and are renamed for the format. This sketch holds one run
of each flag, and one run as shipped, against the member step's own tests, so the tests that moved are shown
to still see what the flags break in the format's home.

**Outcome:** WORKS. Each flag fails tests of the member step, and the shipped run passes all six.

## How it was run

`run.sh` runs `cargo test -q -p arvo-format --lib --no-fail-fast member::step` from `mock/`, once with no flag
and once under each of `--cfg arvo_mutant_member_step` and `--cfg arvo_mutant_member_step_edge`, and keeps
every line that is not a pass, each panic with the three lines after it, and the exit status in
`outputs/<name>.txt`.

## What the flags do

`arvo_mutant_member_step` takes the floor of a negative ratio one step too high: `Ratio::exact` returns
`-whole` rather than `-whole - 1` beside the residue `den - rest`, so a negative position that is not whole
lands on the slot above its floor with the remainder still measured from below.

`arvo_mutant_member_step_edge` lets `holds` admit any ratio with a positive denominator, so the host arm takes
a quotient up to `2^128` rather than one below `2^127`, past the edge, where the floor no longer fits the
index, and the bound over the range reads true where it should read false.

## What the run shows

As shipped, `outputs/shipped.txt`: 6 passed, exit 0.

Under `arvo_mutant_member_step`, `outputs/arvo_mutant_member_step.txt`: 2 of 6 fail, exit 101. The sweep of
every pair at six bits fails first at `BinaryRationals` at exponent -5 on the quotient of -32 by 3, and the
64-bit corners fail on the signed quotient at exponent -1 of `-2^63` by `2^63 - 2`. Both compare the member
step against the slot step's position, which is the law the round says the format owes for a second formation
of the same position.

Under `arvo_mutant_member_step_edge`, `outputs/arvo_mutant_member_step_edge.txt`: 3 of 6 fail, exit 101. The
edge test at radix two fails on `!Sides::<UFixed<64, 0>>::PRODUCT`, and the one at radix ten and three on
`!Sides::<UTen<Unsigned<64>, 0>>::PRODUCT`: the unsigned square past `2^127` is admitted. The 64-bit corners
fail on the product the flag lets into the host's integer, the unsigned product at exponent zero of
`2^64 - 2` by itself.

The tests that stay green under each flag are the ones that do not reach what it changes. Under both, the
landing against `adapted`, which forms no ratio, and the 64-bit division's choice, which reads the terms and
not the bound. Under the first, the two edge tests too, which read the bound and form no floor. Under the
second, the sweep at six bits, whose ratios all sit far below the edge.

## What this does not establish

The records of the flags as `room.rs` carried them stay in `202611192130-room_value-mutant` and
`202611200125_room-edge-mutant`. This run is of the format's flags against the format's tests alone; the
value's suites, which now call the format, are run by `cargo mock test` and not under these flags.
