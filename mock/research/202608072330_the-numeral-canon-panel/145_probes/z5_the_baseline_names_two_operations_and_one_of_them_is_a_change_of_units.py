#!/usr/bin/env python3
# z5 (145): hole five. The shared-baseline obligation, and the tension inside its own repair.
#
# 144 F144-13 measures two placements of one named shared baseline: per coordinate before the
# weighting changes which arm a fixed weighting picks at 24.6%; applied once to the weighted scalar
# it changes it at 0.0%. Its repair (section 6) is that the comparison is made after the weighting,
# on the weighted scalar, because the per-coordinate form "is a change of weighting wearing a
# normalisation's clothes".
#
# But 144 section 4.3 says the per-coordinate form is exactly what makes a weighting travel across
# a rescaled target: un-normalised, a pure per-coordinate rescale switches the selection at a mean
# of 18.0%; normalised per coordinate, at exactly 0.0%. So the same file forbids the operation in
# section 6 and requires it in section 4.3, and nothing states how a design does both.
#
# Predictions, stated before running:
#
#   E1. The 0.0% is a THEOREM and not a measurement. Dividing every arm's weighted scalar by one
#       positive constant preserves the argmin, so nothing could have come out otherwise, and the
#       obligation's scalar half needs no evidence.
#   E2. The per-coordinate form is exactly the weight substitution w -> w/b, up to positive scale.
#       So it is a reweighting by identity rather than by measurement, and the 24.6% is how often
#       that substitution crosses a cell boundary rather than what makes it a reweighting.
#   E3. A uniform baseline is not merely sufficient for the per-coordinate form to be a no-op, it
#       is NECESSARY. If two coordinates of the baseline differ, an arm set and a weight exist on
#       which the selection moves.
#   E4. The two obligations CONFLICT as stated: on one fixed target, per-coordinate normalisation
#       changes which arm a declared weighting selects, so buying portability by section 4.3 costs
#       exactly what section 6 forbids.
#   E5. And the conflict dissolves if the per-coordinate division is read as a declaration of the
#       weighting's UNITS rather than as a transformation of the cost table. A weighting declared
#       in baseline-relative units travels exactly across a per-coordinate rescale AND is the arm
#       the consumer asked for, because they asked in those units. Predicted: switch rate exactly
#       zero across rescales, and by construction no discrepancy against "the arm asked for".
#   E6. CONTROL: the reading in E5 must NOT make selection travel across an arbitrary target
#       change, or it is not a change of units but a claim that targets do not matter.
from fractions import Fraction as Fr
import random

random.seed(20260815)

def pick(costs, w):
    """argmin of the weighted sum, ties to the lowest index."""
    best, bi = None, None
    for i, c in enumerate(costs):
        s = sum(wk * ck for wk, ck in zip(w, c))
        if best is None or s < best:
            best, bi = s, i
    return bi

def weights(n, res):
    """Every weight vector on the (n-1)-simplex at resolution `res`, as exact rationals."""
    def rec(k, left):
        if k == 1:
            yield (Fr(left, res),)
            return
        for i in range(left + 1):
            for rest in rec(k - 1, left - i):
                yield (Fr(i, res),) + rest
    return [w for w in rec(n, res) if any(x > 0 for x in w)]

def table(arms, coords, hi=60):
    return [tuple(Fr(random.randint(1, hi)) for _ in range(coords)) for _ in range(arms)]

ARMS, COORDS, RES = 7, 3, 12
W = weights(COORDS, RES)

print("=" * 100)
print("E1. The scalar placement, and why it needs no measurement")
print("=" * 100)
moved = 0
cases = 0
for _ in range(60):
    C = table(ARMS, COORDS)
    b = C[random.randrange(ARMS)]
    for w in W:
        bw = sum(wk * bk for wk, bk in zip(w, b))
        if bw == 0:
            continue
        cases += 1
        raw = pick(C, w)
        scaled = pick([tuple(ck for ck in c) for c in C], w)   # same table
        # the scalar normalisation divides every arm's weighted sum by bw, so recompute directly
        sums = [sum(wk * ck for wk, ck in zip(w, c)) / bw for c in C]
        norm = min(range(len(sums)), key=lambda i: sums[i])
        if raw != norm:
            moved += 1
print(f"  cases: {cases}, selections changed by the scalar normalisation: {moved}")
print(f"  E1: {'CONFIRMED' if moved == 0 else 'REFUTED'}, and the reason is that dividing every")
print("      arm's weighted sum by one positive constant preserves the argmin. Nothing could have")
print("      come out otherwise, so 144's 0.0% is the theorem showing rather than evidence.")

print()
print("=" * 100)
print("E2. Is the per-coordinate form exactly the substitution w -> w/b?")
print("=" * 100)
mismatch = 0
checked = 0
for _ in range(60):
    C = table(ARMS, COORDS)
    b = C[random.randrange(ARMS)]
    if any(bk == 0 for bk in b):
        continue
    for w in W:
        checked += 1
        per_coord = pick([tuple(ck / bk for ck, bk in zip(c, b)) for c in C], w)
        substituted = pick(C, tuple(wk / bk for wk, bk in zip(w, b)))
        if per_coord != substituted:
            mismatch += 1
print(f"  (arm set, weight) pairs checked: {checked}, disagreements: {mismatch}")
print(f"  E2: {'CONFIRMED, they are the same operation' if mismatch == 0 else 'REFUTED'}")
print("      So 'per-coordinate normalisation' and 'change the weighting to w/b' are one thing")
print("      written two ways, and no measurement is needed to call it a reweighting.")

print()
print("=" * 100)
print("E3. Is a uniform baseline NECESSARY for the per-coordinate form to be a no-op?")
print("=" * 100)
print(f"  {'baseline':<22}{'arm sets tried':<18}{'selection moved somewhere':<28}")
uniform_moved = nonuniform_all_moved = True
for tag, b in (("uniform (5,5,5)", (Fr(5), Fr(5), Fr(5))),
               ("non-uniform (1,2,4)", (Fr(1), Fr(2), Fr(4))),
               ("non-uniform (3,3,7)", (Fr(3), Fr(3), Fr(7))),
               ("non-uniform (1,1,2)", (Fr(1), Fr(1), Fr(2)))):
    found = False
    tries = 0
    for _ in range(40):
        C = table(ARMS, COORDS)
        tries += 1
        if any(pick([tuple(ck / bk for ck, bk in zip(c, b)) for c in C], w) != pick(C, w)
               for w in W):
            found = True
            break
    print(f"  {tag:<22}{tries:<18}{str(found):<28}")
    if tag.startswith("uniform"):
        uniform_moved = found
    else:
        nonuniform_all_moved &= found
print(f"\n  E3: uniform never moves anything: {not uniform_moved} (must be True)")
print(f"      every non-uniform baseline moves something: {nonuniform_all_moved} (must be True)")
print(f"      {'CONFIRMED' if (not uniform_moved) and nonuniform_all_moved else 'REFUTED'}")

print()
print("=" * 100)
print("E4. Do 144 section 6 and section 4.3 conflict on one fixed target?")
print("=" * 100)
diff = tot = 0
for _ in range(40):
    C = table(ARMS, COORDS)
    b = C[random.randrange(ARMS)]
    if any(bk == 0 for bk in b):
        continue
    for w in W:
        tot += 1
        if pick([tuple(ck / bk for ck, bk in zip(c, b)) for c in C], w) != pick(C, w):
            diff += 1
print(f"  (arm set, weight) cases: {tot}, per-coordinate normalisation picks a different arm "
      f"than the declared weighting: {diff} ({100.0 * diff / tot:.1f}%)")
print(f"  E4: {'CONFIRMED, the two obligations conflict' if diff > 0 else 'REFUTED'}")
print("      Section 4.3 requires the per-coordinate form to make a weighting travel. Section 6")
print("      forbids it because it changes which arm a fixed weighting picks. Both are correct")
print("      about the same operation, and nothing in 144 says how a design does both.")

print()
print("=" * 100)
print("E5/E6. The dissolution: read the division as the weighting's UNITS, not as a transform")
print("=" * 100)
print("  If a weighting is DECLARED in baseline-relative units, then the selection on a target is")
print("  argmin over arms of sum_k w_k * c_ik / b_k by definition, and there is no other weighting")
print("  it should have agreed with. Two things then have to hold, and they are opposite:")
print()
rescale_moved = arbitrary_moved = 0
rescale_cases = arbitrary_cases = 0
for _ in range(40):
    C = table(ARMS, COORDS)
    bi = random.randrange(ARMS)
    r = tuple(Fr(random.randint(1, 8)) for _ in range(COORDS))
    C_rescaled = [tuple(rk * ck for rk, ck in zip(r, c)) for c in C]
    C_other = table(ARMS, COORDS)
    for w in W:
        b1, b2, b3 = C[bi], C_rescaled[bi], C_other[bi]
        if any(x == 0 for x in b1 + b2 + b3):
            continue
        rescale_cases += 1
        arbitrary_cases += 1
        p1 = pick([tuple(ck / bk for ck, bk in zip(c, b1)) for c in C], w)
        p2 = pick([tuple(ck / bk for ck, bk in zip(c, b2)) for c in C_rescaled], w)
        p3 = pick([tuple(ck / bk for ck, bk in zip(c, b3)) for c in C_other], w)
        if p1 != p2:
            rescale_moved += 1
        if p1 != p3:
            arbitrary_moved += 1
print(f"  E5: across a pure per-coordinate RESCALE of the target, the relative-units selection")
print(f"      moves at {rescale_moved} of {rescale_cases} cases "
      f"({100.0 * rescale_moved / rescale_cases:.1f}%) -> "
      f"{'CONFIRMED' if rescale_moved == 0 else 'REFUTED'}")
print(f"  E6: across an ARBITRARY target change it moves at {arbitrary_moved} of "
      f"{arbitrary_cases} cases ({100.0 * arbitrary_moved / arbitrary_cases:.1f}%) -> "
      f"{'CONFIRMED, so it is a change of units and not a claim that targets do not matter' if arbitrary_moved > 0 else 'REFUTED'}")

print()
print("=" * 100)
print("VERDICT")
print("=" * 100)
print("  One named shared baseline names two operations and the word does not distinguish them.")
print()
print("    Applied once to the weighted scalar it is a REPORTING normalisation. It preserves the")
print("    argmin as a theorem, so it changes no selection ever, and it is what makes two")
print("    strategies' cost claims comparable. This is 144's repair and it is right.")
print()
print("    Applied per coordinate before the weighting it is a CHANGE OF UNITS, identical to the")
print("    substitution w -> w/b. It is what makes a weighting travel across a per-coordinate")
print("    rescale of the target, and it is a no-op for every weight exactly when the baseline is")
print("    uniform across coordinates.")
print()
print("  The 24.6% is the disagreement between a weighting read in absolute units and the same")
print("  numbers read in relative ones, which is two questions rather than one question answered")
print("  wrongly. So the design owes a declaration of which units a weighting is expressed in, and")
print("  once it declares that, both of 144's requirements hold at once and neither is violated.")
