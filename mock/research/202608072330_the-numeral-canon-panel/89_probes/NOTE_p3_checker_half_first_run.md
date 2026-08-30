# p3's checker half, first run: it measured nothing, and the defect is on the record

`p3_frontier.py`'s CHECKER table sweeps E_d for d in 2..=L at widths 1..=WB and
compares the verdict against an exhaustive sweep. The rows with L = 9 accept at
WB = 16 in 0.4 s, which is four thousand times cheaper than the arithmetic
predicts for a 65,536-point sweep, and the reason is that **the laws are false
at those widths**. E_d's truth set is 1..=d, so at every width above d both the
verdict and the sweep exit at the first witness, x = 2, and the row measures the
cost of two evaluations rather than the cost of a sweep.

This is the same failure `84` recorded against its own p4 battery and `86`
guarded against in its p2 by choosing a law with a TRUE verdict so nothing
exits early. The verdict half of `p3` carries that guard and asserts it; the
checker half did not, and the rows at WB > 9 are void.

`p3b_checker_frontier.py` is the corrected instrument: every swept law is TRUE
at every width in the band (E_d with d >= WB), so both the verdict and the sweep
run to completion, and the assert in the generated crate fails if any of them
does not.

The first run's transcript is kept here as `p3_output.txt` rather than deleted.
The rows that stand are the VERDICT table, whose generated crate asserts its own
verdict is TRUE and therefore cannot exit early.
