# The value's ratio in the host integer, timed

Round `202611192047` forms the exact position of the value's product at a non-zero exponent, of its
quotient and of its conversion in the host's integer where a constant bound over the range holds
(`arvo-numeral/src/room.rs`), and leaves the format's four-limb step for the rest. This note reads the
bench `mock/benches/numeral-ratio`, which prices the arm against that four-limb step and against the
operation a consumer writes on the machine's integers. It is the bench A-new-1a and A-new-1b of the
stack roadmap ask for (`.shared/research/202611150830_stack-path/40-the-roadmap-final.md:55-57`).

## What was measured

One signature, `Fixed<16, -8>` under half to even and saturation. Each arm reads 64 records of two
`i16` slot indices and folds its 64 results into one 64-bit word in order, which the harness compares
across the arms on its validation seeds before timing; every arm agreed. `hand` is the baseline: the
scaled numerator, `div_euclid` and `rem_euclid` on `i32`, the residue rounded half to even, the result
clamped. `room` is the value's own operation. `wide` is `arvo_format::div`, `mul` or `convert` on the
same slots, which is what the value called before this round. The quotient's divisor is read from the
record at run time and landed in a format of the slots 1 to 32767 on the same grid by
`Member::adapted`, then taken by `Divisor::member`; `hand` clamps to the same range. The conversion goes
onto `Fixed<16, -4>`.

Profile: the harness's pinned release profile, each arm its own cdylib with fat LTO and one codegen
unit, in its own subprocess, four passes of 1000 runs in batches of 100. Machine class
`linux-x86_64-c4-container-intel`, an Intel Xeon at 2.30 GHz in a shared container. The medians below
are for all 64 records; divide by 64 for one operation.

## The run

The run is `20261119T224053Z-1dd38711`, taken from the commit that adds the arms with nothing else
on disk changed. The arm it times is the arm as it lands: the residue handed to `adapt` unreduced,
since a value only rounds and completes the position, and the division in the machine's 64-bit
integer where the bound keeps both terms there, which it does at 16 bits for all three operations.
An earlier version reduced the residue by a gcd loop and divided every term in `u128`; it was timed
only from a dirty tree, so those runs are not kept and this note quotes nothing from them.

| sweep | hand | room | wide |
|---|---|---|---|
| quotient | 152.6 ns | 1.37 us (9.0x) | 12.10 us (79.3x) |
| product | 153.0 ns | 171.5 ns (1.12x) | 10.92 us (71.4x) |
| conversion | 112.0 ns | 177.3 ns (1.58x) | 8.36 us (74.6x) |

Each run directory is
`mock/benches/results/numeral-ratio/<sweep>_n256/linux-x86_64-c4-container-intel/20261119T224053Z-1dd38711/`,
with its `samples.csv`, `summary.tsv` and `report.md`.

## What it decides

On this signature and this machine the arm takes the product from 10.92 us to 171.5 ns for 64
records, 18.5 ns over the hand version in all, and the conversion from 8.36 us to 177.3 ns, 65.3 ns
over it. Both are equal to the hand version's output and to the oracle's on every member the suites
run. That is the fast path A-new-1a and A-new-1b ask for at the 16-bit point, within 1.12 times the
hand product and 1.58 times the hand conversion. The roadmap states no factor, so these are the
factors, not a pass against one.

The quotient is 10.72 us faster than the four limbs and still 1.22 us behind the hand version over 64
records, about 19 ns a quotient. The run does not say where that goes. The arm's own division is the
same 64-bit one the product takes, so the likely place is the divisor's landing, `Member::adapted`,
which runs the format's applied map over a four-limb position for every record while the hand version
clamps an `i32`. That is a reading of the code, not something this bench measured, and it is ticket
`member_adapted_lands_an_on_grid_slot_through_the_four_limb_adapt`.

## What it does not decide

One signature at one width and one exponent, one input length, one machine class. The arm's bound, the
64-bit narrowing and the four-limb fallback are constants of the instantiation, so a 64-bit point takes
the `u128` division and a deep exponent the four limbs; neither is timed here. Nothing is said about a
signature under wrapping or another mode, where `adapt`'s own cost differs.
