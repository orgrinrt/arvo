# The red run of the member step's guard on the quotient

Round `202611201340` holds the member step's bound at its edge above a zero exponent and adds a third
mutant flag, `arvo_mutant_member_step_guard`, which drops the check in `Sides::QUOTIENT` that the widest
member times `u` is a 128-bit integer. This sketch runs the member step's tests as shipped and under each
of the three flags, so the guard is shown to have a test that sees it go. It follows
`202611200745_member-step-mutants`, which ran the first two flags before the positive side was held.

**Outcome:** WORKS. Each of the three flags fails tests of the member step, and the shipped run passes all
eight.

## How it was run

`run.sh` is the runner of `202611200745_member-step-mutants` with the third flag added. It runs
`cargo test -q -p arvo-format --lib --no-fail-fast member::step` from `mock/`, once with no flag and once
under each `--cfg`, and keeps every line that is not a pass, each panic with the three lines after it, and
the exit status in `outputs/<name>.txt`.

## What the run shows

As shipped, `outputs/shipped.txt`: 8 passed, exit 0.

Under `arvo_mutant_member_step`, `outputs/arvo_mutant_member_step.txt`: 3 of 8 fail, exit 101. The sweep of
every pair at six bits fails at `BinaryRationals` at exponent -5 on the quotient of -32 by 3, and both
64-bit corner tests fail on a signed position at exponent -1. The negative floor is still seen where the
earlier sketch saw it.

Under `arvo_mutant_member_step_edge`, `outputs/arvo_mutant_member_step_edge.txt`: 5 of 8 fail, exit 101.
Beside the failures the earlier sketch recorded, the edge test now fails too, first on the quotient one
past its edge at one bit, signed, radix two.

Under `arvo_mutant_member_step_guard`, `outputs/arvo_mutant_member_step_guard.txt`: 1 of 8 fails, exit
101. The edge test fails on the quotient one past its edge above zero, first at eight bits, signed, radix
two, exponent 121, where the guard is the only part of the bound that refuses. No other test sees the guard
go, which is what the second review found: before this round its deletion went unseen.
