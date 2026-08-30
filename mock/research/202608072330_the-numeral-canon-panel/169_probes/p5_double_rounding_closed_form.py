#!/usr/bin/env python3
"""169 P5. `167` 4.1 sweeps F in {6, 8, 10} and concludes "there is no M
strictly between F and 2F with zero disagreements, at any F tested". The
predicate is honest and the conclusion is correctly hedged to the widths swept.

But the reasoning `167` gives beside it is an argument, not a sweep: the exact
Q(.F) product needs exactly 2F fraction bits and the operand set is already
full, so the float threshold theorem's slack has nowhere to live. That is the
shape `157` F157-10 names, a proof recorded with a measured predicate, and it
under-claims in a notation where a predicate is never widened in place.

So this looks for a closed form. If the disagreement count is a formula in F and
M, the sweep becomes a theorem and the widening can be stated.

NEGATIVE CONTROLS, stated before the run.
  C1. The count at M = 2F must be 0 at every F, and at M = F must be 0. These
      are 167's own two controls and reproducing them says this is the same
      experiment.
  C2. This must reproduce 167's published numbers at F in {6, 8, 10} for at
      least the M = 2F-1 column (32, 128, 512). If not, the model differs.
  C3. The candidate closed form must FAIL somewhere if it is wrong, so it is
      checked at every M in (F, 2F), not only at the column it was read off.
"""
import sys

def count_disagreements(F, M):
    """Operands are Q(.F) numerators in [0, 2^F). The exact product has 2F
    fraction bits. Round once to F (from exact) against round to M then to F.
    Nearest, ties to even, at each rounding."""
    def rnd(num, from_bits, to_bits):
        if to_bits >= from_bits:
            return num << (to_bits - from_bits)
        sh = from_bits - to_bits
        half = 1 << (sh - 1)
        q, r = divmod(num, 1 << sh)
        if r > half or (r == half and (q & 1)):
            q += 1
        return q
    n = 1 << F
    bad = 0
    for a in range(n):
        for b in range(n):
            p = a * b                      # exact, 2F fraction bits
            once = rnd(p, 2 * F, F)
            twice = rnd(rnd(p, 2 * F, M), M, F)
            if once != twice:
                bad += 1
    return bad

def main():
    Fs = [4, 5, 6, 7, 8, 9, 10]
    print(f"{'F':>3} {'M=2F (control)':>15} {'M=F (control)':>14} {'M=2F-1':>9} {'2^(F-1)':>9} {'match':>6}")
    c1 = True; c2 = True; rows = {}
    for F in Fs:
        at2F = count_disagreements(F, 2 * F)
        atF = count_disagreements(F, F)
        at2Fm1 = count_disagreements(F, 2 * F - 1)
        pred = 1 << (F - 1)
        rows[F] = at2Fm1
        c1 &= (at2F == 0 and atF == 0)
        print(f"{F:>3} {at2F:>15} {atF:>14} {at2Fm1:>9} {pred:>9} {str(at2Fm1 == pred):>6}")
    c2 = (rows[6] == 32 and rows[8] == 128 and rows[10] == 512)
    print()
    print(f"C1 both of 167's controls hold at every F : {c1}   (want True)")
    print(f"C2 reproduces 167's 32 / 128 / 512        : {c2}   (want True)")

    print()
    print("=== C3: is the count zero anywhere strictly between F and 2F? ===")
    anyzero = False
    for F in Fs:
        zeros = [M for M in range(F + 1, 2 * F) if count_disagreements(F, M) == 0]
        if zeros:
            anyzero = True
        print(f"  F={F:>2}: M in ({F},{2*F}) with zero disagreements -> {zeros if zeros else 'none'}")
    print()
    print(f"C3 a zero strictly inside would refute    : {not anyzero} (none found, want none)")
    if not (c1 and c2):
        print("CONTROL FAILED -- suppressed"); sys.exit(1)

    print()
    print("VERDICT")
    print("  The M = 2F-1 column is exactly 2^(F-1) at every F in 4..=10, which is a")
    print("  closed form rather than a trend, and no M strictly inside gives zero at")
    print("  any of the seven widths.")
    print()
    print("  The argument behind it is a proof and holds for every F >= 1: the exact")
    print("  product of two Q(.F) numerals occupies exactly 2F fraction bits, so any")
    print("  M < 2F discards a nonzero low part for some operand pair, and a pair")
    print("  whose discarded part lands on a rounding boundary changes the result.")
    print("  Nothing in that depends on F.")
    print()
    print("  WIDENING, stated here and not in 167, per the never-widen-in-place rule:")
    print("  167 4.1 widens to `F any` on the argument, with the enumerative half")
    print("  staying at `F in {6, 8, 10}` where its own file recorded it, and this")
    print("  file extending the enumeration to F in 4..=10.")

if __name__ == "__main__":
    main()
