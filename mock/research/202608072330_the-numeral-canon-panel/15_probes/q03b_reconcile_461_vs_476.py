#!/usr/bin/env python3
# q03b. Reconcile my product-overshoot count against 06's.
#
# 06 section 7.1: "tight at 6100 of 6561 pairs and wastes exactly one bit on the
# other 461", with the region given as 160 + 301, plus "16 pairs where the clamp
# hides the waste, which are the 15 negative-width pairs plus the doubly
# degenerate one".
#
# My instrument reports 476. This panel already carries one unreconciled count
# that 09 and 14 both call poison, so this diagnoses the gap rather than picking
# a side. Neither number is wrong; they answer different questions.
#
# Reproduce:  python3 q03b_reconcile_461_vs_476.py
# Toolchain:  python3 only. Exact integer arithmetic, not a measurement.

from collections import Counter

BOX = 9


def bits_for(u):
    return u.bit_length()


def exact_W(a, b):
    W1, W2 = a[0] + a[1], b[0] + b[1]
    if W1 == 0 or W2 == 0:
        return 0  # the product set is {0}, which needs zero bits
    return bits_for((2**W1 - 1) * (2**W2 - 1))


def naive_W(a, b):
    return (a[0] + b[0]) + (a[1] + b[1])


def main():
    shapes = [(I, F) for I in range(BOX) for F in range(BOX)]
    pairs = [(a, b) for a in shapes for b in shapes]

    over = [(a, b, naive_W(a, b) - exact_W(a, b)) for a, b in pairs
            if naive_W(a, b) > exact_W(a, b)]
    print("pairs where the sum-of-widths form exceeds the exact width:", len(over))
    print("by number of bits wasted:", dict(sorted(Counter(d for _, _, d in over).items())))

    deg = [(a, b, d) for a, b, d in over if a[0] + a[1] == 0 or b[0] + b[1] == 0]
    print("  at least one operand is the zero-only numeral:", len(deg))

    narrow1 = [(a, b, d) for a, b, d in over
               if not (a[0] + a[1] == 0 or b[0] + b[1] == 0)
               and min(a[0] + a[1], b[0] + b[1]) == 1]
    print("  narrower operand has total width 1, non-degenerate:", len(narrow1))

    negI = [(a, b, d) for a, b, d in narrow1
            if exact_W(a, b) - (a[1] + b[1]) < 0]
    print("    of those, the exact answer has I < 0:", len(negI))
    print("    of those, the exact answer has I >= 0:", len(narrow1) - len(negI))

    print()
    print("06's decomposition, read with the overlap removed:")
    print(f"  160 (degenerate) + 301 (narrow-1, I >= 0) + 15 (narrow-1, I < 0)"
          f" = {len(deg)} + {len(narrow1) - len(negI)} + {len(negI)}"
          f" = {len(deg) + len(narrow1)}")
    print("06's 461 is its own first two terms. The 15 it accounts for")
    print("separately, as the pairs where the clamp hides the waste, and its")
    print("'16' includes the doubly degenerate pair already inside the 160.")
    print()
    print("Both numbers are right and they answer different questions. Neither")
    print("should be quoted without its convention.")


if __name__ == "__main__":
    main()
