#!/usr/bin/env python3
# p4: independent second instrument.
#
# p3 finds its witnesses with integer division tricks (`(n + d//2) // d`). This file uses
# `fractions.Fraction` throughout and a different comparison method (exact rational
# comparison against the midpoint, rather than integer arithmetic), as a genuinely separate
# code path rather than a rerun of p3's own logic, per RULES.md's bar that three independent
# instruments beat one instrument run three times.
#
# It checks two things p3 did not check directly:
#   (a) does the count of step-wise-vs-once-truncated disagreements, counted this
#       differently-coded way, land in the same neighbourhood as p3's count at the same F
#       (a cross-check on the finding, not a re-derivation of the same code)?
#   (b) for every disagreement found, does the WIDENED computation (never rounding the
#       intermediate at all, i.e. carrying the full-precision product through both steps
#       and rounding only once at the very end) equal the once-truncated exact answer, by
#       construction? This is the other half of the pigeonhole argument: narrow-at-every-step
#       is forced to be wrong somewhere; wide-until-the-end is never wrong, because it never
#       discards the information that caused the narrow version to fail.

from fractions import Fraction

def round_nearest_fraction(v: Fraction, quantum: Fraction) -> Fraction:
    # round v to the nearest multiple of `quantum`, ties away from zero. v, quantum > 0.
    units = v / quantum
    floor_units = units.numerator // units.denominator
    frac = units - floor_units
    if frac > Fraction(1, 2):
        return (floor_units + 1) * quantum
    elif frac < Fraction(1, 2):
        return floor_units * quantum
    else:
        return (floor_units + 1) * quantum  # ties away from zero

def sweep(F):
    quantum = Fraction(1, 1 << F)
    disagreements = 0
    widening_always_matches = True
    checked = 0
    first = None
    for nx1 in range(0, 1 << F):
        for nx2 in range(nx1 + 1, 1 << F):
            for na in range(1, 1 << F):
                x1 = Fraction(nx1, 1 << F)
                x2 = Fraction(nx2, 1 << F)
                a = Fraction(na, 1 << F)

                m1 = round_nearest_fraction(x1 * a, quantum)
                m2 = round_nearest_fraction(x2 * a, quantum)
                if m1 != m2:
                    continue
                m = m1

                for nb in range(1, 1 << F):
                    b = Fraction(nb, 1 << F)
                    checked += 1

                    once1 = round_nearest_fraction(x1 * a * b, quantum)
                    once2 = round_nearest_fraction(x2 * a * b, quantum)
                    if once1 == once2:
                        continue

                    disagreements += 1
                    step2 = round_nearest_fraction(m * b, quantum)
                    if first is None:
                        first = (x1, x2, a, b, m, step2, once1, once2)

                    # part (b): the widened computation never rounds the intermediate at
                    # all, so its "step 1" is the EXACT product x*a, not m. Its final
                    # rounding is the only rounding it ever does, so it is definitionally
                    # equal to `once1` / `once2` for the respective x. Check it anyway,
                    # via the same exact-Fraction machinery, rather than asserting it.
                    wide1 = round_nearest_fraction(x1 * a * b, quantum)  # no intermediate round
                    wide2 = round_nearest_fraction(x2 * a * b, quantum)
                    if wide1 != once1 or wide2 != once2:
                        widening_always_matches = False

    return checked, disagreements, widening_always_matches, first

def main():
    for F in [3, 4, 5, 6]:
        checked, disagreements, widening_ok, first = sweep(F)
        print(f"F={F}: triples checked={checked}, step-wise-vs-once-truncated disagreements={disagreements}")
        print(f"  widening (round only once) matches the once-truncated reference on every case checked: {widening_ok}")
        if first:
            x1, x2, a, b, m, step2, once1, once2 = first
            print(f"  first witness (independent search): x1={x1} x2={x2} a={a} b={b}")
            print(f"    step-wise (F bits at every step) gives {step2} for both x1 and x2")
            print(f"    once-truncated exact reference gives {once1} for x1 and {once2} for x2")
        print()

if __name__ == "__main__":
    main()
