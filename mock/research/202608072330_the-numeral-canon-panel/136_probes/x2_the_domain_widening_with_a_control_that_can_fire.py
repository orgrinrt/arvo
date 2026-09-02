#!/usr/bin/env python3
# x2 (136): reproducing 135's domain-any widening on an instrument that can report a difference.
#
# Why rebuild it. 135's z1 reports every quantity identical between a negative-indexed cell and
# a positive one, and the structural reason it gives (the construction reads frac(x) and never
# k) is sound. But z1 carries NO negative control: its parts 1 and 2 compute a single value and
# assert in prose that it applies to both signs, and its part 3 compares two counts that both
# come out 1 with nothing to show the comparison could have come out otherwise. A sweep that
# cannot report a difference is not evidence that there is none. This probe parametrises by k
# explicitly, sweeps both signs, and carries a control that MUST fire.
#
# Predictions, stated before running:
#
#   P1. Sweeping k over both signs and computing the uniqueness solve per k, every k gives the
#       identical threshold distribution. So the coupling argument is sign-blind and 132 5.6/5.7's
#       `domain closed under negation` is inherited rather than established.
#   P2. CONTROL, and this is what z1 lacks: a deliberately sign-dependent variant must be
#       DETECTED as differing by the same comparison. If the control does not fire, my
#       instrument is blind and P1's confirmation is worthless.
#       *** THE FIRST VERSION OF THIS CONTROL DID NOT FIRE, AND THE REASON WAS THE DEFECT THIS
#       PROBE EXISTS TO CATCH. *** It reversed the subpoint list for k < 0 and then sorted that
#       list by value, so the reversal was a no-op and the control could not have reported a
#       difference at any k, under any convention. The variance control had the same shape: it
#       perturbed f to 1 - f, and f(1 - f) is symmetric under exactly that map, so it too was
#       structurally incapable of firing. Both are replaced below with perturbations that
#       genuinely change the quantity being compared, and the failed versions are recorded here
#       rather than quietly swapped out, because a control that cannot fire is indistinguishable
#       from a passing check and I wrote two of them in the file arguing against one.
#   P3. The variance closed forms, computed per k, are identical across signs; the same control
#       applied to the variance must fire.
#   P4. The widening is CONDITIONAL and z1 does not say on what. The construction is sign-blind
#       because frac(x) = x - floor(x) lands in [0,1) for negative x as well as positive. Under
#       the other common convention, frac(x) = x - trunc(x), which lands in (-1,0] for negative
#       x, the SAME construction becomes sign-dependent. Prediction: under the truncating
#       convention the per-k comparison differs, so `domain any` holds given floor-based frac and
#       not otherwise. If this is confirmed, the widening carries a hypothesis nobody has stated.
from fractions import Fraction

def frac_floor(x):
    """The convention the construction actually uses: result in [0, 1) for every sign."""
    return x - (x.numerator // x.denominator)

def frac_trunc(x):
    """The other common convention: result in (-1, 0] for negative x."""
    t = x.numerator // x.denominator if x >= 0 else -((-x).numerator // (-x).denominator)
    return x - t

def cell_of(x):
    return x.numerator // x.denominator

# ---------------------------------------------------------------------------
# The uniqueness solve, restated per cell so a per-k difference COULD appear.
# A realisation-monotone unbiased law on m subpoints: the round-up probability at subpoint j
# must equal frac at that subpoint, and monotonicity forces the round-up sets to be nested,
# so the law is the uniform threshold. Solving per k means: build the m subpoint values that
# actually live in cell k, take their frac under the chosen convention, and read off the
# threshold distribution as the successive differences.
# ---------------------------------------------------------------------------

def threshold_distribution(k, m, fracfn, sign_dependent=False):
    # sign_dependent is the CONTROL: for k < 0 the subpoints are placed quadratically rather
    # than uniformly, which changes the successive differences and so must be detected. This
    # perturbs the quantity being compared rather than the order it is read in, which is where
    # the first version of this control failed.
    if sign_dependent and k < 0:
        pts = [Fraction(k) + Fraction(j * j, m * m) for j in range(m)]
    else:
        pts = [Fraction(k) + Fraction(j, m) for j in range(m)]
    fr = [fracfn(p) for p in pts]
    order = sorted(range(m), key=lambda j: fr[j])
    dist = []
    prev = Fraction(0)
    for j in order:
        dist.append(fr[j] - prev)
        prev = fr[j]
    dist.append(Fraction(1) - prev)
    return tuple(d for d in dist if d != 0)

def variance_pair(k, n, m, fracfn, sign_dependent=False):
    """Comonotone and independent variance of the sum of n realisations at one point of cell k.

    sign_dependent is the CONTROL: for k < 0 the fractional part is halved. The first version
    mapped f to 1 - f, and f(1 - f) is invariant under that map, so it could never fire.
    """
    x = Fraction(k) + Fraction(1, 3)
    f = fracfn(x)
    if sign_dependent and k < 0:
        f = f / 2
    return (n * n * f * (1 - f), n * f * (1 - f))

KS = (-5, -4, -3, -1, 0, 1, 3, 4, 5)

print("=" * 90)
print("P1. The uniqueness solve, computed PER k over both signs, floor-based frac")
print("=" * 90)
for m in (5, 8):
    dists = {k: threshold_distribution(k, m, frac_floor) for k in KS}
    uniq = set(dists.values())
    print(f"  m={m}: {len(uniq)} distinct distribution(s) across k in {KS}")
    print(f"        value = {[str(d) for d in next(iter(uniq))]}")
p1 = all(len({threshold_distribution(k, m, frac_floor) for k in KS}) == 1 for m in (5, 8))
print(f"\n  P1 (sign-blind under floor-based frac): {'CONFIRMED' if p1 else 'REFUTED'}")

print()
print("=" * 90)
print("P2. THE CONTROL. A deliberately sign-dependent variant must be DETECTED.")
print("=" * 90)
for m in (5, 8):
    dists = {k: threshold_distribution(k, m, frac_floor, sign_dependent=True) for k in KS}
    uniq = set(dists.values())
    print(f"  m={m}: {len(uniq)} distinct distribution(s) across k (must be > 1 for the")
    print(f"        instrument to be capable of reporting a difference at all)")
p2 = all(len({threshold_distribution(k, m, frac_floor, sign_dependent=True)
              for k in KS}) > 1 for m in (5, 8))
print(f"\n  P2 (control fires): {'CONFIRMED' if p2 else 'REFUTED, and P1 is then worthless'}")

print()
print("=" * 90)
print("P3. The variance closed forms per k, and the same control")
print("=" * 90)
for n in (5, 10):
    vals = {k: variance_pair(k, n, 3, frac_floor) for k in KS}
    uniq = set(vals.values())
    c = variance_pair(-4, n, 3, frac_floor, sign_dependent=True)
    p = variance_pair(4, n, 3, frac_floor, sign_dependent=True)
    print(f"  n={n}: {len(uniq)} distinct (comonotone, independent) pair(s) across k, "
          f"value = ({next(iter(uniq))[0]}, {next(iter(uniq))[1]})")
    print(f"        control at k=-4 vs k=+4 under the sign-dependent variant: "
          f"({c[0]}, {c[1]}) vs ({p[0]}, {p[1]}), differ = {c != p}")
p3 = (all(len({variance_pair(k, n, 3, frac_floor) for k in KS}) == 1 for n in (5, 10))
      and variance_pair(-4, 5, 3, frac_floor, True) != variance_pair(4, 5, 3, frac_floor, True))
print(f"\n  P3 (variance sign-blind, control fires): {'CONFIRMED' if p3 else 'REFUTED'}")

print()
print("=" * 90)
print("P4. What the widening rests on: the frac convention, which nobody has stated")
print("=" * 90)
for m in (5, 8):
    dists = {k: threshold_distribution(k, m, frac_trunc) for k in KS}
    print(f"  m={m}, truncating frac: {len(set(dists.values()))} distinct distribution(s) "
          f"across k (floor-based gave 1)")
    neg = threshold_distribution(-3, m, frac_trunc)
    pos = threshold_distribution(3, m, frac_trunc)
    print(f"        k=-3 -> {[str(d) for d in neg]}")
    print(f"        k=+3 -> {[str(d) for d in pos]}")
    print(f"        differ = {neg != pos}")
p4 = any(threshold_distribution(-3, m, frac_trunc) != threshold_distribution(3, m, frac_trunc)
         for m in (5, 8))
print(f"\n  P4 (the same construction IS sign-dependent under truncating frac): "
      f"{'CONFIRMED' if p4 else 'REFUTED'}")

print()
print("=" * 90)
print("VERDICT")
print("=" * 90)
print(f"  135's dissent reproduces on an instrument that can report a difference: {p1 and p2 and p3}")
print(f"  and the widening carries a hypothesis 135 did not state: {p4}")
print()
print("  So the honest predicate is not bare `domain any`. It is `domain any` GIVEN that the")
print("  cell coordinate is taken by flooring, which is the convention the construction and")
print("  every probe in this topic uses, and which is the same convention 125's F9 identifies")
print("  the bit-drop operation with. Under a truncating cell coordinate the argument does not")
print("  transfer, and the region where the widening applies is exactly the region where the")
print("  grid decomposition is floor-based. That is a condition on the construction rather than")
print("  on the domain, which is why it belongs in the clause and not in the predicate's domain")
print("  dimension.")
