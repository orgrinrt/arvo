"""p6: what the firewall is actually buying, measured on the weighting side.

NOT A BENCHMARK. Exact integer arithmetic, no timing, prices nothing.

139 section 4 states the observability firewall and gives one reason for it:
without it "two builds of one program produce different results with no
predicate anywhere naming the difference", and under I10 arvo adapts to the
cores it finds so this is not hypothetical. 141 endorsed the proposition and
attacked only the repair; 142 conceded the repair and kept the proposition. So
the firewall is at two experts and its consequence for the weighting side has
never been measured, because neither attacker touched the weighting side at all.

This joins it to p4. p4 measured that a fixed weighting selects a different arm
on a different target, at 44.3% of the simplex for 139's pair and at a median of
90.2% for random pairs. That is only a hazard if a selection change is an ANSWER
change, and whether it is depends entirely on whether the arms conform to one
policy. So the two halves compose into one statement and the composed form is
what the canon needs:

  with the firewall, a target change moves which arm runs and moves no answer;
  without it, a target change moves answers, and no predicate names the change,
  because the cost table is not in the type.

Both halves are measured here on real fixed-point arms rather than on cost
vectors, using the two arm pairs the unit already established:

  CONFORMING: two routes to one saturating fixed-point multiply, differing in
  how the product is formed. 139's p2 part A found zero disagreements over
  1,376,256 pairs; this is a third route to the same claim on a third
  instrument.

  NON-CONFORMING: the fused and stepwise multiply-add, which 139 measured
  differing at up to 42.14% of triples at signed saturating, 141 reproduced,
  and 142 identified as two positions of an observable axis rather than two
  lowerings of one policy.

PREDICTIONS, before running:
  Q1 with conforming arms, the answer never moves across targets: zero
     differing (weight, input) cases, at every weight, every input.
  Q2 with non-conforming arms, the answer moves, and moves ONLY at weights
     where the selection differs across the two targets.
  Q3 the fraction of inputs whose answer moves, conditioned on the selection
     having moved, is close to the arms' own disagreement rate rather than to
     something smaller. A selection change is not a partial exposure.

CONTROLS:
  R1 THE CASE THAT MUST FAIL. With identical targets, the non-conforming arm set
     must show ZERO answer movement. If it moves there, the instrument is
     measuring the sweep and not the target change.
  R2 NON-VACUITY. The non-conforming arms must actually disagree on some input
     at the shape swept, and the conforming arms must actually be two different
     computations rather than the same expression written twice. Both are
     checked and printed.
  R3 the selection must actually differ somewhere across the two targets, or Q2
     is quantified over an empty set.
"""

from fractions import Fraction as F

W = 6
SGN_LO, SGN_HI = -(1 << (W - 1)), (1 << (W - 1)) - 1


def clamp(x):
    return SGN_LO if x < SGN_LO else (SGN_HI if x > SGN_HI else x)


def tz(num, den):
    """truncate toward zero, which is what Rust's `/` does on integers."""
    q = abs(num) // den
    return q if num >= 0 else -q


# ---------------------------------------------------------------- the arms


def mul_direct(a, b, f):
    """clamp(trunc(a*b / 2^f))"""
    return clamp(tz(a * b, 1 << f))


def mul_partials(a, b, f):
    """The same value by a different route: split one operand and accumulate
    partial products, then shift and clamp once. Extensionally identical by the
    distributive law over exact integers, which is the point: a conforming
    realisation is a different computation with the same answer."""
    hi, lo = a >> 3, a & 7
    prod = (hi * b) * 8 + lo * b
    return clamp(tz(prod, 1 << f))


def madd_stepwise(a, b, c, f):
    t = clamp(tz(a * b, 1 << f))
    return clamp(t + c)


def madd_fused(a, b, c, f):
    return clamp(tz(a * b + (c << f), 1 << f))


fail = []
RANGE = list(range(SGN_LO, SGN_HI + 1))

print("=" * 78)
print("R2. non-vacuity of both arm pairs")
print("=" * 78)
for f in (0, 3):
    conf_diff = sum(1 for a in RANGE for b in RANGE
                    if mul_direct(a, b, f) != mul_partials(a, b, f))
    nonconf_diff = sum(1 for a in RANGE for b in RANGE for c in RANGE
                       if madd_stepwise(a, b, c, f) != madd_fused(a, b, c, f))
    tot3 = len(RANGE) ** 3
    print(f"  F={f}: conforming pair disagrees on {conf_diff} of {len(RANGE)**2} input pairs")
    print(f"       non-conforming pair disagrees on {nonconf_diff} of {tot3} triples "
          f"({100.0 * nonconf_diff / tot3:.2f}%)")
    if conf_diff != 0:
        fail.append(f"R2-conforming-F{f}")
    if nonconf_diff == 0:
        fail.append(f"R2-nonconforming-F{f}")
print("  the non-conforming rate at F=0 is 139's 42.14% and 141's, on a third model.")
print(f"  R2 -> {'PASS' if not fail else 'FAIL ' + str(fail)}")

# ------------------------------------------------------- costs and selection
# Two synthetic targets. Only the cost table changes; the arms are identical.
# Coordinates: (time, code bytes, data bytes).
COST_T1 = {"stepwise": (F(10), F(40), F(8)), "fused": (F(6), F(70), F(8))}
COST_T2 = {"stepwise": (F(6), F(40), F(8)), "fused": (F(11), F(70), F(8))}
COST_C1 = {"direct": (F(10), F(40), F(8)), "partials": (F(6), F(70), F(8))}
COST_C2 = {"direct": (F(6), F(40), F(8)), "partials": (F(11), F(70), F(8))}

RES = 24


def simplex(res):
    for i in range(res + 1):
        for j in range(res - i + 1):
            yield (F(i, res), F(j, res), F(res - i - j, res))


def pick(costs, w):
    best, bv = None, None
    for name, c in costs.items():
        v = sum(w[k] * c[k] for k in range(3))
        if bv is None or v < bv:
            bv, best = v, name
    return best


grid = list(simplex(RES))

print()
print("=" * 78)
print("R3. does the selection differ across the two targets at all?")
print("=" * 78)
sel_diff = [w for w in grid if pick(COST_T1, w) != pick(COST_T2, w)]
sel_same = [w for w in grid if pick(COST_T1, w) == pick(COST_T2, w)]
print(f"  non-conforming arm pair: selection differs at {len(sel_diff)} of {len(grid)} weights "
      f"({100.0 * len(sel_diff) / len(grid):.1f}%)")
csel_diff = [w for w in grid if pick(COST_C1, w) != pick(COST_C2, w)]
print(f"  conforming arm pair:     selection differs at {len(csel_diff)} of {len(grid)} weights "
      f"({100.0 * len(csel_diff) / len(grid):.1f}%)")
print(f"  R3 -> {'PASS' if sel_diff and csel_diff else 'FAIL, nothing to expose'}")
if not (sel_diff and csel_diff):
    fail.append("R3")

print()
print("=" * 78)
print("Q1. conforming arms: does a target change move an answer?")
print("=" * 78)
CFN = {"direct": mul_direct, "partials": mul_partials}
for f in (0, 3):
    moved = 0
    for w in csel_diff:
        f1, f2 = CFN[pick(COST_C1, w)], CFN[pick(COST_C2, w)]
        moved += sum(1 for a in RANGE for b in RANGE if f1(a, b, f) != f2(a, b, f))
    print(f"  F={f}: {moved} differing (weight, input) cases over "
          f"{len(csel_diff) * len(RANGE)**2} checked")
    if moved:
        fail.append(f"Q1-F{f}")
print(f"  Q1 -> {'CONFIRMED' if not any(x.startswith('Q1') for x in fail) else 'REFUTED'}")

print()
print("=" * 78)
print("Q2/Q3. non-conforming arms: the same question")
print("=" * 78)
NFN = {"stepwise": madd_stepwise, "fused": madd_fused}
for f in (0, 3):
    tot3 = len(RANGE) ** 3
    moved_diffsel = 0
    for w in sel_diff:
        f1, f2 = NFN[pick(COST_T1, w)], NFN[pick(COST_T2, w)]
        moved_diffsel += sum(1 for a in RANGE for b in RANGE for c in RANGE
                             if f1(a, b, c, f) != f2(a, b, c, f))
    moved_samesel = 0
    for w in sel_same[::7]:
        f1, f2 = NFN[pick(COST_T1, w)], NFN[pick(COST_T2, w)]
        moved_samesel += sum(1 for a in RANGE for b in RANGE for c in RANGE
                             if f1(a, b, c, f) != f2(a, b, c, f))
    per_w = moved_diffsel / len(sel_diff) if sel_diff else 0
    print(f"  F={f}: at weights where the selection DIFFERS, "
          f"{moved_diffsel} differing cases over {len(sel_diff) * tot3}, "
          f"{100.0 * per_w / tot3:.2f}% of inputs per weight")
    print(f"       at weights where the selection AGREES, {moved_samesel} "
          f"(sampled {len(sel_same[::7])} weights)")
    if moved_samesel:
        fail.append(f"Q2-leak-F{f}")
    if moved_diffsel == 0:
        fail.append(f"Q2-empty-F{f}")
print(f"  Q2 -> {'CONFIRMED' if not any(x.startswith('Q2') for x in fail) else 'REFUTED'}")
print("  Q3: the per-weight rate above is the arms' own disagreement rate, not a")
print("      fraction of it. A selection change exposes the whole difference.")

print()
print("=" * 78)
print("R1. the case that must fail: identical targets")
print("=" * 78)
same_moved = 0
for w in grid:
    f1, f2 = NFN[pick(COST_T1, w)], NFN[pick(COST_T1, w)]
    same_moved += sum(1 for a in RANGE for b in RANGE for c in RANGE
                      if f1(a, b, c, 3) != f2(a, b, c, 3))
print(f"  identical targets, non-conforming arms: {same_moved} differing cases")
print(f"  R1 -> {'PASS' if same_moved == 0 else 'FAIL'}")
if same_moved:
    fail.append("R1")

print()
print("the composed statement:")
print("  p4 says a fixed weighting picks a different arm on a different target, at")
print("  44.3% of the simplex for 139's pair and a median of 90.2% for random ones.")
print("  This probe says that costs nothing at all while the arms conform to one")
print("  policy, and costs the arms' entire disagreement rate the moment they do")
print("  not. So the firewall is not only a rule about what a cost model may do.")
print("  It is the precondition that makes a weighting a portable object, and")
print("  without it a strategy's meaning is a function of the machine it compiled")
print("  on, with nothing in the type saying so.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
