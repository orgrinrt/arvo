"""q3: the two readings of my own work that `145` and `146` ask me to check.

NOT A BENCHMARK. Exact integer arithmetic and a parse of a committed artifact.

PART A: the quantifier. `145` z1 reports that visibility is monotone in the
observation set and therefore saturates, so the axis-only property is visibility
under the maximal observation set, and that both definitions of visible coincide
over 144 cells. My F144-16 is stated per observation set and supplies no
quantifier. Part A checks the monotonicity against `144_probes/p7_out.txt`, the
artifact I committed before `145` existed, rather than against a new model.

PART B: the conformance region. `145` section 2 states, from its own z3 C4, that
"at `overflow = wrap` all four lowering arms conform to every intermediate
position ... at `overflow = sat` the four split two and two", and concludes the
number of arms component two ranges over is a function of component one. The
coordinator asks me to check that reading of my `p6` measurement. Part B builds
the same four arms independently and counts extensional classes per assignment.

PREDICTIONS, before running:
  EE1 zero monotonicity violations in `p7`'s committed table, over every pair of
      observation sets ordered by inclusion.
  EE2 whether the MAXIMAL observation set is needed is itself shape-dependent: at
      unsigned the intermediate axis is invisible under every proper subset I
      swept and visible only under all five operations, while at signed a single
      operation suffices. So `146` section 6.3 is right that the operation set
      matters, and it matters at one signedness rather than uniformly.
  EE3 `145` section 2 is WRONG on the wrapping row. At signed, toward-zero
      rounding, wrapping, F >= 1, the four arms split two and two rather than
      conforming, because toward-zero is not translation equivariant (`142`
      F142-2). The claim holds only for equivariant rounding positions.
  EE4 at signed wrapping with FLOOR the four do conform, which is why the wrong
      claim looked right.
  EE5 at signed saturating the four split at every rounding position and every F.
  EE6 at unsigned the four conform at every overflow position and every rounding
      position, by the one-sided-clamp congruence.

CONTROLS:
  FF1 THE CASE THAT MUST FAIL for part A. Some comparable pair must flip from
      invisible to visible as the observation set grows, or the monotonicity is
      satisfied vacuously by everything being visible or nothing being.
  FF2 the dead axis, if present in the parsed table, must never be visible.
  FF3 THE CASE THAT MUST FAIL for part B. The two fused arms must agree with each
      other at every cell, and the two stepwise arms likewise. If a pair built to
      be the same function disagrees, the class count is measuring my arm
      construction rather than the assignment.
  FF4 the class count must take BOTH values somewhere, or part B is measuring a
      constant.
"""

import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
P7 = os.path.normpath(os.path.join(HERE, "..", "144_probes", "p7_out.txt"))

fail = []

# ---------------------------------------------------------------- part A

# The observation sets p7 swept, and the inclusion order among them.
OPSETS = {
    "binary only": frozenset({"add", "sub", "mul"}),
    "madd only": frozenset({"madd"}),
    "mul and madd": frozenset({"mul", "madd"}),
    "all five": frozenset({"add", "sub", "mul", "madd", "msub"}),
}

print("=" * 80)
print("PART A. monotonicity of visibility, from 144's own committed p7 output")
print("=" * 80)

rows = {}
pat = re.compile(
    r"^\s+(unsign|signed)\s+(\d)\s+(binary only|madd only|mul and madd|all five)"
    r"\s+(\w+)\s+(\d+)\s+(\[[^\]]*\])\s+(True|False)\s+(True|False)")
with open(P7) as fh:
    for line in fh:
        m = pat.match(line)
        if m:
            rows[(m.group(1), int(m.group(2)), m.group(3), m.group(4))] = (
                m.group(7) == "True", m.group(8) == "True")

print(f"  parsed {len(rows)} rows from {os.path.basename(P7)}")
if not rows:
    print("  FAIL: the parse produced nothing, so nothing below means anything")
    fail.append("parse")

pairs = [(a, b) for a in OPSETS for b in OPSETS
         if a != b and OPSETS[a] < OPSETS[b]]
print(f"  observation-set pairs ordered by strict inclusion: "
      f"{[(a, b) for a, b in pairs]}")

viol = flips = checked = 0
axes = sorted({k[3] for k in rows})
for sgn in ("unsign", "signed"):
    for f in (0, 1, 2):
        for axis in axes:
            for a, b in pairs:
                ka, kb = (sgn, f, a, axis), (sgn, f, b, axis)
                if ka not in rows or kb not in rows:
                    continue
                checked += 1
                va, vb = rows[ka][1], rows[kb][1]
                if va and not vb:
                    viol += 1
                if (not va) and vb:
                    flips += 1
print(f"  EE1 comparable (shape, axis, O1 < O2) triples checked: {checked}, "
      f"violations: {viol} -> {'CONFIRMED' if viol == 0 else 'REFUTED'}")
if viol:
    fail.append("EE1")
print(f"  FF1 pairs where enlarging the observation set makes the axis visible: {flips} "
      f"-> {'PASS' if flips else 'FAIL, the monotonicity is vacuous'}")
if not flips:
    fail.append("FF1")
dead = [k for k in rows if k[3] == "dead" and rows[k][1]]
print(f"  FF2 dead-axis rows reported visible: {len(dead)} "
      f"-> {'PASS' if not dead else 'FAIL'}")
if dead:
    fail.append("FF2")

print()
print("  EE2 which observation sets make the intermediate axis visible:")
for sgn in ("unsign", "signed"):
    vis = [o for o in OPSETS if (sgn, 1, o, "intermediate") in rows
           and rows[(sgn, 1, o, "intermediate")][1]]
    print(f"    {sgn} F=1: visible under {vis}")
u = [o for o in OPSETS if (("unsign", 1, o, "intermediate") in rows
                           and rows[("unsign", 1, o, "intermediate")][1])]
s = [o for o in OPSETS if (("signed", 1, o, "intermediate") in rows
                           and rows[("signed", 1, o, "intermediate")][1])]
ee2 = u == ["all five"] and len(s) > 1
print(f"  EE2 -> {'CONFIRMED' if ee2 else 'REFUTED'}: at unsigned only the full set reveals it,")
print(f"      at signed a single operation does. So the maximal observation set is load-bearing")
print(f"      at one signedness and not the other, which sharpens 146 section 6.3: the table")
print(f"      waits on the operation set, and it waits harder at unsigned.")
print(f"  NOTE ON INDEPENDENCE: this is a re-reading of MY OWN committed artifact, so it is a")
print(f"      second instrument on 145's A1 only in the sense that the model was built before")
print(f"      A1 existed and for another purpose. Our models share W, F, signedness, the")
print(f"      rounding pair, the intermediate pair and the dead axis; mine adds a third overflow")
print(f"      position. The intersection is nearly the union, so this corroborates the")
print(f"      arithmetic and says nothing about generality.")

# ---------------------------------------------------------------- part B

print()
print("=" * 80)
print("PART B. how many arms conform, as a function of the whole assignment")
print("=" * 80)

W = 6


def bounds(signed):
    return (-(1 << (W - 1)), (1 << (W - 1)) - 1) if signed else (0, (1 << W) - 1)


def reduce_(x, signed, ov):
    lo, hi = bounds(signed)
    if ov == "sat":
        return lo if x < lo else (hi if x > hi else x)
    span = 1 << W
    return (x - lo) % span + lo


def rshift(num, f, rd):
    den = 1 << f
    if rd == "floor":
        return num // den
    q = abs(num) // den
    return q if num >= 0 else -q


def fused_widening(a, b, c, f, sgn, ov, rd):
    return reduce_(rshift(a * b + (c << f), f, rd), sgn, ov)


def fused_partials(a, b, c, f, sgn, ov, rd):
    hi, lo = a >> 2, a & 3
    prod = (hi * b) * 4 + lo * b
    return reduce_(rshift(prod + (c << f), f, rd), sgn, ov)


def stepwise_shift(a, b, c, f, sgn, ov, rd):
    t = reduce_(rshift(a * b, f, rd), sgn, ov)
    return reduce_(t + c, sgn, ov)


def stepwise_partials(a, b, c, f, sgn, ov, rd):
    hi, lo = a >> 2, a & 3
    prod = (hi * b) * 4 + lo * b
    t = reduce_(rshift(prod, f, rd), sgn, ov)
    return reduce_(t + c, sgn, ov)


ARMS = [("fused/widening", fused_widening), ("fused/partials", fused_partials),
        ("stepwise/shift", stepwise_shift), ("stepwise/partials", stepwise_partials)]

print(f"  {'signedness':<11}{'F':>2}{'overflow':>10}{'rounding':>14}"
      f"{'classes':>9}{'largest class':>15}")
counts = {}
fused_disagree = step_disagree = 0
for sgn in (False, True):
    lo, hi = bounds(sgn)
    dom = range(lo, hi + 1)
    for f in (0, 1, 2, 3):
        for ov in ("wrap", "sat"):
            for rd in ("floor", "toward_zero"):
                sigs = {}
                for name, fn in ARMS:
                    sigs[name] = tuple(fn(a, b, c, f, sgn, ov, rd)
                                       for a in dom for b in dom for c in dom)
                if sigs["fused/widening"] != sigs["fused/partials"]:
                    fused_disagree += 1
                if sigs["stepwise/shift"] != sigs["stepwise/partials"]:
                    step_disagree += 1
                classes = {}
                for name in sigs:
                    classes.setdefault(sigs[name], []).append(name)
                n = len(classes)
                biggest = max(len(v) for v in classes.values())
                counts[(sgn, f, ov, rd)] = n
                print(f"  {'signed' if sgn else 'unsigned':<11}{f:>2}{ov:>10}{rd:>14}"
                      f"{n:>9}{biggest:>15}")

print()
print(f"  FF3 the two fused arms disagreed in {fused_disagree} cells, the two stepwise arms in "
      f"{step_disagree} -> {'PASS' if fused_disagree == 0 and step_disagree == 0 else 'FAIL'}")
if fused_disagree or step_disagree:
    fail.append("FF3")
vals = set(counts.values())
print(f"  FF4 class counts observed: {sorted(vals)} -> "
      f"{'PASS' if len(vals) > 1 else 'FAIL, the count is constant'}")
if len(vals) < 2:
    fail.append("FF4")

ee3 = all(counts[(True, f, "wrap", "toward_zero")] == 2 for f in (1, 2, 3))
ee4 = all(counts[(True, f, "wrap", "floor")] == 1 for f in (0, 1, 2, 3))
ee5 = all(counts[(True, f, "sat", rd)] == 2 for f in (0, 1, 2, 3)
          for rd in ("floor", "toward_zero"))
ee6 = all(counts[(False, f, ov, rd)] == 1 for f in (0, 1, 2, 3)
          for ov in ("wrap", "sat") for rd in ("floor", "toward_zero"))
print()
print(f"  EE3 signed / wrap / toward-zero at F >= 1 splits rather than conforming: "
      f"{'CONFIRMED' if ee3 else 'REFUTED'}")
print(f"  EE4 signed / wrap / floor conforms: {'CONFIRMED' if ee4 else 'REFUTED'}")
print(f"  EE5 signed / saturating splits at every rounding and F: "
      f"{'CONFIRMED' if ee5 else 'REFUTED'}")
print(f"  EE6 unsigned conforms everywhere: {'CONFIRMED' if ee6 else 'REFUTED'}")
for tag, ok in (("EE3", ee3), ("EE4", ee4), ("EE5", ee5), ("EE6", ee6)):
    if not ok:
        fail.append(tag)

print()
print("  the corrected statement:")
print("    the number of arms component two ranges over is a function of the WHOLE assignment,")
print("    rounding included, and not of the overflow position alone. Under wrapping it is one")
print("    class for translation-equivariant rounding and two otherwise; under saturating on")
print("    signed shapes it is two at every rounding; on unsigned shapes it is one everywhere.")
print()
print("  145 section 2's wrapping clause is contradicted by 145's own z3 output, which prints")
print("  `overflow = wrap conforming-arm counts observed: [2, 4]` two lines above a verdict")
print("  saying every arm conforms under wrapping. That is the same verdict-contradicts-its-own-")
print("  table defect 145 section 9 records against its z4, occurring a second time in the same")
print("  file and not caught. The structural claim survives and gets sharper; only the clause")
print("  naming overflow as the argument is wrong.")

print()
print("=" * 80)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 80)
raise SystemExit(1 if fail else 0)
