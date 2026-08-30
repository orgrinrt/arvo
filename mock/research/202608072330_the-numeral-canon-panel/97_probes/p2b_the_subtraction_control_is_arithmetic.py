#!/usr/bin/env python3
"""P2b. Wrapping subtraction's non-associativity is integer arithmetic, not a policy fact.

`94`'s W4 rests on one cell: wrapping subtraction retracts and does not associate, and
that is offered as proof that retraction and associativity are logically independent
permissions, so a design carrying one bit would take the conjunction and lose an arm.

Under P2's criterion the cell says nothing about the policy, because associativity of
subtraction is not an identity of exact arithmetic in the first place. In the integers
(a - b) - c and a - (b - c) differ by exactly 2c. So under wrapping they agree exactly
when 2c is zero modulo 2^W, that is when c is zero or half the modulus, which is two
values out of 2^W whatever W is.

    predicted failure rate  =  1 - 2 / 2^W  =  1 - 2^(1-W)

That is a closed form with no free parameter, derived before running, and it is
checked below against an exhaustive sweep. W = 8 is included because it is the width
`94` reported and is not one this panel's other probes swept, so agreement there is a
prediction rather than a refit.
"""


def measured_failure_rate(W):
    mod = 1 << W
    dom = range(mod)
    bad = 0
    total = 0
    for a in dom:
        for b in dom:
            ab = (a - b) % mod
            for c in dom:
                total += 1
                lhs = (ab - c) % mod
                rhs = (a - ((b - c) % mod)) % mod
                if lhs != rhs:
                    bad += 1
    return bad, total


def main():
    print("P2b. the subtraction control, closed form against exhaustive sweep")
    print()
    print("%3s  %14s  %14s  %s" % ("W", "predicted", "measured", "verdict"))
    worst = 0.0
    for W in (3, 4, 5, 6, 7, 8):
        pred = 1.0 - 2.0 / (1 << W)
        bad, total = measured_failure_rate(W)
        meas = bad / total
        gap = abs(pred - meas)
        worst = max(worst, gap)
        print("%3d  %13.5f%%  %13.5f%%  %s" % (
            W, 100 * pred, 100 * meas, "exact" if gap == 0.0 else "GAP %.3e" % gap))
    print()
    print("largest gap over the swept widths: %.3e" % worst)
    print()
    print("`94` section 4.2 reports 99.22%% at W = 8 for wrapping subtraction.")
    print("The closed form gives 1 - 2^-7 = %.5f%%, which is that number." % (100 * (1 - 2 / 256.0)))
    print()
    print("So the cell `94` cites as evidence that two permissions are independent is")
    print("a fact about subtraction in the integers. Wrapping contributes only the two")
    print("coincidences where 2c vanishes modulo the width. Nothing in it is about the")
    print("overflow policy, and nothing in it needs a permission bit.")


if __name__ == "__main__":
    main()
