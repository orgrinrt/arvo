#!/usr/bin/env python3
# z1 (145): hole one. Does 144's biconditional close 143's second argument of the count?
#
# 144 F144-16: cutting an axis out of the assignment set changes the class count IFF that axis is
# answer-visible under the observation set. 96 cells, no exceptions. 144 then argues that an
# answer-visible axis is component one, so cutting it is forbidden, leaving the second argument a
# no-op or forbidden with no live case between.
#
# The gap I am testing is not the arithmetic, which 144 measured, but the QUANTIFIER. 144's own
# section 8 reports that the same cut is a no-op under one observation set and not under another,
# so "answer-visible" is a property of an (axis, observation set) PAIR and not of an axis. The
# design-level claim needs an axis-only property, so it needs a quantifier over O, and which one it
# gets decides whether the closure holds.
#
# Predictions, stated before running:
#
#   A1. Visibility is MONOTONE in the observation set: if two assignments differing only in axis X
#       are distinguished under O1, they are distinguished under any O2 containing O1. This is the
#       refinement theorem (141 p5) restricted to one axis and should have zero violations.
#   A2. Therefore visibility SATURATES: there is a maximal observation set at which every axis that
#       is ever visible is visible, and visibility under it is an axis-only property. So the
#       quantifier the design claim needs is "visible under the maximal observation set the design
#       admits", and under that reading 144's closure holds.
#   A3. CONTROL, and it must fire or A1 is vacuous: an axis exists that is invisible under some O
#       and visible under a larger one. Without this the monotonicity is trivially satisfied by
#       everything being visible everywhere.
#   A4. CONTROL: a dead axis whose positions are the same function is invisible under every O,
#       including the maximal one, so saturation does not make everything visible.
#   A5. The two definitions of "visible" AGREE: (i) some two assignments differing only in X are
#       distinguished under O, and (ii) cutting X changes the class count under O. 144 states the
#       biconditional with (ii); the monotonicity argument is natural on (i). If they disagree
#       anywhere, 144's statement and my quantifier are about different things and that is the
#       finding.
from itertools import product

# ---------------------------------------------------------------------------
# A compact fixed-point model. Values are raw integers at declared width W with F fraction bits.
# Axes: rounding, overflow, intermediate. Plus a dead axis as a control.
# ---------------------------------------------------------------------------

def lo_hi(W, signed):
    return (-(1 << (W - 1)), (1 << (W - 1)) - 1) if signed else (0, (1 << W) - 1)

def rnd(num, den, mode):
    """Round num/den to an integer under the named mode. den > 0."""
    q, r = divmod(num, den)          # floor division
    if mode == "floor":
        return q
    if mode == "toward_zero":
        return q if num >= 0 else q + (1 if r else 0)
    raise ValueError(mode)

def reduce_(v, W, signed, ov):
    lo, hi = lo_hi(W, signed)
    if ov == "wrap":
        m = 1 << W
        v %= m
        if signed and v > hi:
            v -= m
        return v
    if ov == "sat_both":
        return max(lo, min(hi, v))
    if ov == "sat_high":
        return min(hi, v) if v >= lo else (v % (1 << W)) - (1 << W) if signed else max(lo, v)
    raise ValueError(ov)

def apply_op(op, a, b, c, W, F, signed, asg):
    rd, ov, inter, _dead = asg
    S = 1 << F
    if op == "add":
        return reduce_(a + b, W, signed, ov)
    if op == "sub":
        return reduce_(a - b, W, signed, ov)
    if op == "mul":
        return reduce_(rnd(a * b, S, rd), W, signed, ov)
    if op == "madd":
        if inter == "exact":
            return reduce_(rnd(a * b + c * S, S, rd), W, signed, ov)
        t = reduce_(rnd(a * b, S, rd), W, signed, ov)
        return reduce_(t + c, W, signed, ov)
    if op == "msub":
        if inter == "exact":
            return reduce_(rnd(a * b - c * S, S, rd), W, signed, ov)
        t = reduce_(rnd(a * b, S, rd), W, signed, ov)
        return reduce_(t - c, W, signed, ov)
    raise ValueError(op)

ROUND = ("floor", "toward_zero")
OVER = ("wrap", "sat_both", "sat_high")
INTER = ("stepwise", "exact")
DEAD = ("d0", "d1")           # the control axis: never read by apply_op
AXES = ("rounding", "overflow", "intermediate", "dead")
POSITIONS = {"rounding": ROUND, "overflow": OVER, "intermediate": INTER, "dead": DEAD}
ASSIGNMENTS = [tuple(x) for x in product(ROUND, OVER, INTER, DEAD)]
AXIS_INDEX = {"rounding": 0, "overflow": 1, "intermediate": 2, "dead": 3}

def answer_vector(asg, W, F, signed, O):
    lo, hi = lo_hi(W, signed)
    dom = range(lo, hi + 1)
    out = []
    for op in O:
        if op in ("add", "sub", "mul"):
            for a in dom:
                for b in dom:
                    out.append(apply_op(op, a, b, 0, W, F, signed, asg))
        else:
            for a in dom:
                for b in dom:
                    for c in dom:
                        out.append(apply_op(op, a, b, c, W, F, signed, asg))
    return tuple(out)

def classes(asgs, W, F, signed, O):
    seen = {}
    for a in asgs:
        seen.setdefault(answer_vector(a, W, F, signed, O), []).append(a)
    return seen

def visible_by_separation(axis, W, F, signed, O):
    """(i): some two assignments differing only in `axis` are distinguished under O."""
    i = AXIS_INDEX[axis]
    cache = {}
    for asg in ASSIGNMENTS:
        for pos in POSITIONS[axis]:
            if pos == asg[i]:
                continue
            other = asg[:i] + (pos,) + asg[i + 1:]
            for k in (asg, other):
                if k not in cache:
                    cache[k] = answer_vector(k, W, F, signed, O)
            if cache[asg] != cache[other]:
                return True
    return False

def visible_by_count(axis, W, F, signed, O):
    """(ii): cutting `axis` (pinning it to its first position) changes the class count."""
    i = AXIS_INDEX[axis]
    full = len(classes(ASSIGNMENTS, W, F, signed, O))
    pinned = [a for a in ASSIGNMENTS if a[i] == POSITIONS[axis][0]]
    cut = len(classes(pinned, W, F, signed, O))
    return full != cut, full, cut

OPS_ALL = ("add", "sub", "mul", "madd", "msub")
OBS_SETS = [
    ("{add}", ("add",)),
    ("{add,sub}", ("add", "sub")),
    ("{add,sub,mul}", ("add", "sub", "mul")),
    ("{mul,madd}", ("mul", "madd")),
    ("{add,sub,mul,madd}", ("add", "sub", "mul", "madd")),
    ("ALL", OPS_ALL),
]
SHAPES = [(4, 0, False), (4, 1, False), (4, 2, False), (4, 0, True), (4, 1, True), (4, 2, True)]

print("=" * 96)
print("A5 first: do the two definitions of 'answer-visible' agree?")
print("=" * 96)
print(f"  {'shape':<16}{'obs set':<22}{'axis':<14}{'separation':<12}{'count-change':<14}{'agree'}")
disagree = []
sep_table = {}
cnt_table = {}
for (W, F, signed) in SHAPES:
    tag = f"W{W} F{F} {'s' if signed else 'u'}"
    for oname, O in OBS_SETS:
        for axis in AXES:
            s = visible_by_separation(axis, W, F, signed, O)
            c, full, cut = visible_by_count(axis, W, F, signed, O)
            sep_table[(W, F, signed, oname, axis)] = s
            cnt_table[(W, F, signed, oname, axis)] = c
            if s != c:
                disagree.append((tag, oname, axis, s, c, full, cut))
            if axis in ("intermediate", "dead") and oname in ("{add,sub,mul}", "ALL"):
                print(f"  {tag:<16}{oname:<22}{axis:<14}{str(s):<12}{str(c):<14}{s == c}")
print(f"\n  cells compared: {len(sep_table)}, disagreements: {len(disagree)}")
for d in disagree[:6]:
    print(f"    DISAGREE {d}")
print(f"  A5: {'CONFIRMED, the two definitions coincide' if not disagree else 'REFUTED'}")

print()
print("=" * 96)
print("A1. Is visibility monotone in the observation set?")
print("=" * 96)
viol = []
pairs = 0
for (W, F, signed) in SHAPES:
    for i, (n1, O1) in enumerate(OBS_SETS):
        for j, (n2, O2) in enumerate(OBS_SETS):
            if not set(O1) <= set(O2):
                continue
            for axis in AXES:
                pairs += 1
                if sep_table[(W, F, signed, n1, axis)] and not sep_table[(W, F, signed, n2, axis)]:
                    viol.append((W, F, signed, n1, n2, axis))
print(f"  ordered (shape, O1 subset O2, axis) triples checked: {pairs}")
print(f"  violations (visible under the smaller, invisible under the larger): {len(viol)}")
print(f"  A1: {'CONFIRMED' if not viol else 'REFUTED ' + str(viol[:3])}")

print()
print("=" * 96)
print("A3. CONTROL: an axis invisible under a smaller O and visible under a larger one")
print("=" * 96)
lifts = []
for (W, F, signed) in SHAPES:
    for i, (n1, O1) in enumerate(OBS_SETS):
        for j, (n2, O2) in enumerate(OBS_SETS):
            if not (set(O1) < set(O2)):
                continue
            for axis in AXES:
                if not sep_table[(W, F, signed, n1, axis)] and sep_table[(W, F, signed, n2, axis)]:
                    lifts.append((f"W{W} F{F} {'s' if signed else 'u'}", n1, n2, axis))
print(f"  instances where enlarging O makes an axis visible: {len(lifts)}")
for l in lifts[:5]:
    print(f"    {l[0]:<12} {l[3]:<14} invisible under {l[1]:<20} visible under {l[2]}")
print(f"  A3: {'CONFIRMED, the control fires so A1 is not vacuous' if lifts else 'REFUTED'}")

print()
print("=" * 96)
print("A4. CONTROL: the dead axis is invisible everywhere, including at the maximal O")
print("=" * 96)
dead_visible = [k for k, v in sep_table.items() if k[4] == "dead" and v]
print(f"  cells where the dead axis is visible: {len(dead_visible)} (must be 0)")
print(f"  A4: {'CONFIRMED' if not dead_visible else 'REFUTED ' + str(dead_visible[:3])}")

print()
print("=" * 96)
print("A2. Saturation, and the quantifier the design claim needs")
print("=" * 96)
print(f"  {'shape':<16}{'axis':<14}{'visible under ALL':<20}{'visible under some smaller O'}")
for (W, F, signed) in SHAPES:
    tag = f"W{W} F{F} {'s' if signed else 'u'}"
    for axis in AXES:
        at_max = sep_table[(W, F, signed, "ALL", axis)]
        at_any = any(sep_table[(W, F, signed, n, axis)] for n, _ in OBS_SETS)
        print(f"  {tag:<16}{axis:<14}{str(at_max):<20}{at_any}")
sat_ok = all(sep_table[(W, F, s, "ALL", ax)] == any(sep_table[(W, F, s, n, ax)]
                                                    for n, _ in OBS_SETS)
             for (W, F, s) in SHAPES for ax in AXES)
print(f"\n  A2 (visible under the maximal O == visible under any O): "
      f"{'CONFIRMED' if sat_ok else 'REFUTED'}")

print()
print("=" * 96)
print("VERDICT")
print("=" * 96)
print("  144's biconditional reproduces and its 'answer-visible' is a property of an (axis,")
print("  observation set) pair. Visibility is monotone in that set and therefore saturates, so")
print("  the axis-only property the design claim needs is visibility under the MAXIMAL")
print("  observation set the design admits. Under that quantifier the closure holds: an axis")
print("  visible at the maximum is component one and cutting it is forbidden; an axis invisible")
print("  at the maximum is dead and cutting it is a no-op at every O.")
print()
print("  What does NOT close, and it is a different question from the one 143 asked: the maximal")
print("  observation set is the set of operations the design ships, which is a design act. So the")
print("  count waits on the operation set rather than on a denotation line, and the two are not")
print("  the same open question.")
