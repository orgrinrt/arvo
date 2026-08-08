#!/usr/bin/env python3
# p6: the fix for p4's vacuous "widening recovers" check, per 46's attack.
#
# p4 computed `wide1` and `once1` as the literal same Python expression
# (`round_nearest_fraction(x1 * a * b, quantum)` twice), so `wide1 != once1` could never fire.
# That is not a check of whether widening recovers exactness; it is `assert X == X`.
#
# This probe replaces it with a genuinely different, finite computation: an intermediate
# with F + k fractional bits, k = 0..F, that rounds ONCE after step 1 (to F+k bits, not to
# infinite precision) and once again after step 2 (to F bits). For k = 0 this is the narrow
# stepwise computation p3 already showed fails. For k = F (full doubling, matching 45's own
# p5 model and 16's p5 model) it is provably lossless for a 2-multiply chain, because the
# exact product of two F-bit values fits in exactly 2F fractional bits with no rounding at
# all, so a 2F-bit intermediate carries every bit the final single rounding needs.
#
# What this probe actually finds out, that p4 did not: for the witnesses p3 found (where
# narrow, F-bit-per-step rounding disagrees with the once-truncated exact reference), what is
# the MINIMUM headroom k that recovers the correct answer? This is a real question with a
# real, non-tautological answer, and it directly answers 46's question 3.

from fractions import Fraction

def round_nearest_fraction(v: Fraction, quantum: Fraction) -> Fraction:
    units = v / quantum
    floor_units = units.numerator // units.denominator
    frac = units - floor_units
    if frac > Fraction(1, 2):
        return (floor_units + 1) * quantum
    elif frac < Fraction(1, 2):
        return floor_units * quantum
    else:
        return (floor_units + 1) * quantum

def finite_widened_chain(x_num, a_num, b_num, F, k):
    """Compute the chain with an intermediate rounded to F+k fractional bits after step 1,
    then rounded to F bits after step 2. k=0 is the narrow stepwise computation. This is a
    GENUINELY DIFFERENT computation from the once-truncated reference: it rounds TWICE (once
    at F+k bits, once at F bits), not once, so it can and does disagree with the reference
    for small k."""
    x = Fraction(x_num, 1 << F)
    a = Fraction(a_num, 1 << F)
    b = Fraction(b_num, 1 << F)
    wide_quantum = Fraction(1, 1 << (F + k))
    narrow_quantum = Fraction(1, 1 << F)
    intermediate = round_nearest_fraction(x * a, wide_quantum)
    final = round_nearest_fraction(intermediate * b, narrow_quantum)
    return final

def once_truncated_reference(x_num, a_num, b_num, F):
    x = Fraction(x_num, 1 << F)
    a = Fraction(a_num, 1 << F)
    b = Fraction(b_num, 1 << F)
    narrow_quantum = Fraction(1, 1 << F)
    return round_nearest_fraction(x * a * b, narrow_quantum)

def find_disagreement_witnesses(F):
    """Reuse p3/p4's search shape to find witnesses where narrow (k=0) stepwise disagrees
    with the once-truncated reference, so this probe has real inputs to test headroom
    against rather than inventing its own."""
    two_f = 1 << F
    witnesses = []
    for na in range(1, two_f):
        for nx1 in range(0, two_f):
            for nx2 in range(nx1 + 1, two_f):
                m1 = round_nearest_fraction(Fraction(nx1, two_f) * Fraction(na, two_f), Fraction(1, two_f))
                m2 = round_nearest_fraction(Fraction(nx2, two_f) * Fraction(na, two_f), Fraction(1, two_f))
                if m1 != m2:
                    continue
                for nb in range(1, two_f):
                    once1 = once_truncated_reference(nx1, na, nb, F)
                    once2 = once_truncated_reference(nx2, na, nb, F)
                    if once1 == once2:
                        continue
                    witnesses.append((nx1, nx2, na, nb, once1, once2))
    return witnesses

def minimum_headroom(nx1, nx2, na, nb, once1, once2, F):
    """The smallest k in 0..F such that BOTH x1's and x2's finite-widened chain match their
    respective once-truncated references. Returns F+1 if even full doubling (k=F) fails,
    which per the argument above should never happen for a 2-multiply chain, and checking
    that it never happens is itself part of what this probe establishes."""
    for k in range(0, F + 1):
        r1 = finite_widened_chain(nx1, na, nb, F, k)
        r2 = finite_widened_chain(nx2, na, nb, F, k)
        if r1 == once1 and r2 == once2:
            return k
    return F + 1  # should not happen; flagged as a failure if it does

def main():
    for F in [4, 5, 6]:
        witnesses = find_disagreement_witnesses(F)
        print(f"F={F}: {len(witnesses)} disagreement witnesses (k=0 stepwise vs once-truncated)")

        # sanity: k=0 must reproduce the disagreement (confirms this is a real, different
        # computation from p4's vacuous one, not another tautology)
        k0_disagreements = 0
        for (nx1, nx2, na, nb, once1, once2) in witnesses:
            r1 = finite_widened_chain(nx1, na, nb, F, 0)
            r2 = finite_widened_chain(nx2, na, nb, F, 0)
            if r1 != once1 or r2 != once2:
                k0_disagreements += 1
        print(f"  k=0 (no headroom) disagrees with the once-truncated reference on {k0_disagreements} of {len(witnesses)} witnesses")

        # sanity: k=F (full doubling) must recover on every witness (the provable-lossless claim)
        kF_failures = 0
        for (nx1, nx2, na, nb, once1, once2) in witnesses:
            r1 = finite_widened_chain(nx1, na, nb, F, F)
            r2 = finite_widened_chain(nx2, na, nb, F, F)
            if r1 != once1 or r2 != once2:
                kF_failures += 1
        print(f"  k={F} (full doubling) fails to recover on {kF_failures} of {len(witnesses)} witnesses (must be 0 if the lossless argument is correct)")

        # the real question: minimum headroom per witness
        min_k_counts = {}
        for (nx1, nx2, na, nb, once1, once2) in witnesses:
            mk = minimum_headroom(nx1, nx2, na, nb, once1, once2, F)
            min_k_counts[mk] = min_k_counts.get(mk, 0) + 1
        print(f"  minimum headroom k needed, distribution across {len(witnesses)} witnesses: {dict(sorted(min_k_counts.items()))}")
        if witnesses:
            nx1, nx2, na, nb, once1, once2 = witnesses[0]
            mk = minimum_headroom(nx1, nx2, na, nb, once1, once2, F)
            print(f"  first witness: x1={nx1}/{1<<F} x2={nx2}/{1<<F} a={na}/{1<<F} b={nb}/{1<<F}, minimum headroom = {mk} bits (of {F} available)")
        print()

if __name__ == "__main__":
    main()
