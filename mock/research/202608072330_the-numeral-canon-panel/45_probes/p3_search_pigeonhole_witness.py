#!/usr/bin/env python3
# p3: search for a witness that NO F-bit-at-each-step fixed-point multiply chain can match
# the once-truncated exact chain answer for all inputs, under round-nearest at every step.
#
# This is the search half of the argument that widening (retaining more than F bits between
# operations, i.e. exactly what "Precise... especially within chains and ops" would need if
# the intent is to match the exact once-truncated answer) is forced by information content,
# not by a choice of rounding policy. If round-nearest at each step -- the best single-step
# rounding rule available -- still cannot match the once-truncated exact answer for some
# input pair, no other single-step rounding rule can either, for the SAME reason: two
# distinct true intermediate values are made indistinguishable by ANY F-bit representation
# that rounds to the nearest representable point, and once indistinguishable, no downstream
# computation can recover which one it was.
#
# Exhaustive over small F, all representable operand triples in [0, 1). Two independent
# checks are run against every candidate witness (round-half-up and round-half-to-even),
# so the witness is not an artifact of one tie-breaking convention.

def round_half_up(numerator, denominator):
    # numerator, denominator >= 0
    return (numerator + denominator // 2) // denominator

def round_half_even(numerator, denominator):
    q, r = divmod(numerator, denominator)
    twice_r = 2 * r
    if twice_r < denominator:
        return q
    if twice_r > denominator:
        return q + 1
    # tie: round to even
    return q if q % 2 == 0 else q + 1

def search(F, rounder):
    two_f = 1 << F
    witnesses = []
    for n_a in range(1, two_f):
        for n_x1 in range(0, two_f):
            for n_x2 in range(n_x1 + 1, two_f):
                # step 1: round(x * a) to F bits, exact product is n_x*n_a in units of 2^-2F
                m1 = rounder(n_x1 * n_a, two_f)
                m2 = rounder(n_x2 * n_a, two_f)
                if m1 != m2:
                    continue
                m = m1
                for n_b in range(1, two_f):
                    # once-truncated exact chain answer, both x's, rounded once at the end
                    once1 = rounder(n_x1 * n_a * n_b, two_f * two_f)
                    once2 = rounder(n_x2 * n_a * n_b, two_f * two_f)
                    if once1 == once2:
                        continue
                    # step-wise answer: both x's collapse to m at step 1, so step 2 is
                    # identical for both regardless of which x produced m.
                    step2 = rounder(m * n_b, two_f)
                    # at least one of once1, once2 must differ from step2, since once1 != once2
                    if step2 != once1 or step2 != once2:
                        witnesses.append((n_x1, n_x2, n_a, n_b, m, step2, once1, once2))
    return witnesses

def report(F):
    print(f"=== F = {F} bits, domain [0, {1 << F}) representing [0, 1) in units of 2^-{F} ===")
    for name, rounder in [("round-half-up", round_half_up), ("round-half-to-even", round_half_even)]:
        ws = search(F, rounder)
        print(f"  {name}: {len(ws)} witness quadruples found where step-wise disagrees with once-truncated")
        if ws:
            n_x1, n_x2, n_a, n_b, m, step2, once1, once2 = ws[0]
            print(f"    first witness: x1={n_x1}/{1<<F} x2={n_x2}/{1<<F} a={n_a}/{1<<F} b={n_b}/{1<<F}")
            print(f"      step 1 (x*a, rounded to F bits) collapses both to m = {m}/{1<<F}")
            print(f"      step 2 (m*b, rounded): {step2}/{1<<F}")
            print(f"      once-truncated exact chain, x1*a*b: {once1}/{1<<F}")
            print(f"      once-truncated exact chain, x2*a*b: {once2}/{1<<F}")
            print(f"      the once-truncated answers genuinely differ ({once1} != {once2}),")
            print(f"      but the step-wise F-bit computation cannot distinguish x1 from x2")
            print(f"      after step 1, so it gives the SAME answer ({step2}) for both, which")
            print(f"      is wrong for at least one of them.")
    print()

if __name__ == "__main__":
    for F in [3, 4, 5, 6]:
        report(F)
