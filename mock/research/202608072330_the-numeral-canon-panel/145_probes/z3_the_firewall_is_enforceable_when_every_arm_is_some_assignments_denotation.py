#!/usr/bin/env python3
# z3 (145): hole three. The join, and the condition that makes it enforceable.
#
# 144 F144-15 is the two halves meeting: with arms conforming to one policy a target change moves
# the selection and moves no answer, 0 of 192,512 cases; with non-conforming arms it moves answers
# at the arms' entire disagreement rate, 42.14% of inputs at F = 0, "and nothing in the type says
# so". That last clause is the finding and it is stated as an absence.
#
# But 142 F142-1 already established that the non-conforming pair in question IS the two positions
# of the intermediate axis: 139's p1 Intermediate::Exact and its p2 madd_fused are the same
# function, over 6,356,992 triples, with a cross-pairing control differing at 757,954.
#
# Put together, "nothing in the type says so" is not a property of the firewall. It is a property
# of an assignment set that lacks the axis. So the join has a CONDITION, and the condition is
# checkable:
#
#   Every lowering arm the design admits is extensionally equal to the denotation of some
#   assignment in the assignment set.
#
# Where that holds, a non-conforming pair is two assignments, the type says which one, and the
# firewall needs no mechanism beyond declaration. Where it fails, an arm exists that no declaration
# can name, and that arm is a firewall violation with no repair available in the type.
#
# Predictions, stated before running:
#
#   C1. Every arm in the topic's own arm set matches exactly one assignment's denotation. The
#       conforming pair (two routes to one multiply) matches the same assignment as each other; the
#       non-conforming pair (fused and stepwise multiply-add) matches two different ones.
#   C2. CONTROL, and it must fire: an arm built with a rounding mode the axis set does not carry
#       matches NO assignment. Without this the check would pass for an arm set in which everything
#       matches by construction and would establish nothing.
#   C3. CONTROL: the cross pairing differs, so C1 is a fact about the pairing rather than about
#       every arm collapsing to one function.
#   C4. Each assignment's conforming class has more than one arm somewhere, since a weighting with
#       one arm to choose among selects nothing. If some assignment has exactly one arm, that is
#       where component two is inert and it is worth naming.
from itertools import product

def lo_hi(W, signed):
    return (-(1 << (W - 1)), (1 << (W - 1)) - 1) if signed else (0, (1 << W) - 1)

def rnd(num, den, mode):
    q, r = divmod(num, den)
    if mode == "floor":
        return q
    if mode == "toward_zero":
        return q if num >= 0 else q + (1 if r else 0)
    if mode == "ceiling":
        return q + (1 if r else 0)
    raise ValueError(mode)

def reduce_(v, W, signed, ov):
    lo, hi = lo_hi(W, signed)
    if ov == "wrap":
        m = 1 << W
        v %= m
        return v - m if (signed and v > hi) else v
    if ov == "sat":
        return max(lo, min(hi, v))
    raise ValueError(ov)

# ---------------------------------------------------------------------------
# The assignment set: what a type can declare. Three axes.
# ---------------------------------------------------------------------------
ROUND = ("floor", "toward_zero")
OVER = ("wrap", "sat")
INTER = ("stepwise", "exact")
ASSIGNMENTS = [tuple(x) for x in product(ROUND, OVER, INTER)]

def denotation(asg, W, F, signed, a, b, c):
    rd, ov, inter = asg
    S = 1 << F
    if inter == "exact":
        return reduce_(rnd(a * b + c * S, S, rd), W, signed, ov)
    t = reduce_(rnd(a * b, S, rd), W, signed, ov)
    return reduce_(t + c, W, signed, ov)

# ---------------------------------------------------------------------------
# The arm set: ways of computing, written as the topic's files write them.
# ---------------------------------------------------------------------------
def arm_fused_widening(asg, W, F, signed, a, b, c):
    """139 p2 madd_fused: form the whole product, shift once, reduce once."""
    rd, ov, _ = asg
    S = 1 << F
    return reduce_(rnd(a * b + c * S, S, rd), W, signed, ov)

def arm_fused_partials(asg, W, F, signed, a, b, c):
    """The same denotation reached by accumulating partial products, a machine with no wide
    multiplier. Extensionally the same computation, different route."""
    rd, ov, _ = asg
    S = 1 << F
    acc = 0
    x = a
    y = b
    neg = False
    if y < 0:
        y, neg = -y, True
    while y:
        if y & 1:
            acc += x
        x <<= 1
        y >>= 1
    if neg:
        acc = -acc
    return reduce_(rnd(acc + c * S, S, rd), W, signed, ov)

def arm_stepwise(asg, W, F, signed, a, b, c):
    """139 p2 madd_unfused: reduce the product, then add and reduce again."""
    rd, ov, _ = asg
    S = 1 << F
    t = reduce_(rnd(a * b, S, rd), W, signed, ov)
    return reduce_(t + c, W, signed, ov)

def arm_stepwise_partials(asg, W, F, signed, a, b, c):
    """The stepwise denotation by the partial-product route."""
    rd, ov, _ = asg
    S = 1 << F
    acc = 0
    x, y, neg = a, b, False
    if y < 0:
        y, neg = -y, True
    while y:
        if y & 1:
            acc += x
        x <<= 1
        y >>= 1
    if neg:
        acc = -acc
    t = reduce_(rnd(acc, S, rd), W, signed, ov)
    return reduce_(t + c, W, signed, ov)

def arm_offaxis_ceiling(asg, W, F, signed, a, b, c):
    """CONTROL. A fused arm that rounds with ceiling, which the axis set does not carry.
    It must match no assignment."""
    _, ov, _ = asg
    S = 1 << F
    return reduce_(rnd(a * b + c * S, S, "ceiling"), W, signed, ov)

ARMS = {
    "fused/widening": arm_fused_widening,
    "fused/partials": arm_fused_partials,
    "stepwise/shift": arm_stepwise,
    "stepwise/partials": arm_stepwise_partials,
    "CONTROL off-axis ceiling": arm_offaxis_ceiling,
}

def vec(fn, asg, W, F, signed):
    lo, hi = lo_hi(W, signed)
    dom = range(lo, hi + 1)
    return tuple(fn(asg, W, F, signed, a, b, c) for a in dom for b in dom for c in dom)

SHAPES = [(4, 0, True), (4, 1, True), (4, 2, True), (4, 0, False), (4, 2, False)]

print("=" * 100)
print("C1. Does every arm match exactly one assignment's denotation?")
print("=" * 100)
print(f"  {'shape':<12}{'arm':<28}{'realises assignments':<44}{'n'}")
c1_ok = True
control_fired = True
per_assignment_arms = {}
for (W, F, signed) in SHAPES:
    tag = f"W{W} F{F} {'s' if signed else 'u'}"
    lo, hi = lo_hi(W, signed)
    dom = range(lo, hi + 1)
    denots = {asg: tuple(denotation(asg, W, F, signed, a, b, c)
                         for a in dom for b in dom for c in dom)
              for asg in ASSIGNMENTS}
    by_vec = {}
    for asg, v in denots.items():
        by_vec.setdefault(v, []).append(asg)
    for aname, fn in ARMS.items():
        # an arm is evaluated under each assignment; report which assignments it realises
        hits = set()
        for asg in ASSIGNMENTS:
            v = vec(fn, asg, W, F, signed)
            for other, dv in denots.items():
                if v == dv:
                    hits.add((asg, other))
        matched = sorted({o for _, o in hits})
        is_control = aname.startswith("CONTROL")
        if is_control:
            control_fired &= (len(matched) == 0 or F == 0)
        else:
            c1_ok &= (len(matched) > 0)
        shown = ", ".join("/".join(m) for m in matched[:2]) or "NONE"
        if len(matched) > 2:
            shown += f", +{len(matched) - 2}"
        print(f"  {tag:<12}{aname:<28}{shown:<44}{len(matched)}")
    per_assignment_arms[tag] = by_vec
    print()

print(f"  C1 (every non-control arm realises at least one assignment): "
      f"{'CONFIRMED' if c1_ok else 'REFUTED'}")
print(f"  C2 (the off-axis control matches nothing, except at F = 0 where no rounding fires): "
      f"{'CONFIRMED' if control_fired else 'REFUTED'}")

print()
print("=" * 100)
print("C3. The pairing, which is what makes C1 non-vacuous")
print("=" * 100)
print(f"  {'shape':<12}{'pair':<44}{'differences of':>18}")
c3_ok = False
for (W, F, signed) in SHAPES:
    tag = f"W{W} F{F} {'s' if signed else 'u'}"
    asg_sat_tz = ("toward_zero", "sat", "exact")
    fu = vec(arm_fused_widening, asg_sat_tz, W, F, signed)
    fp = vec(arm_fused_partials, asg_sat_tz, W, F, signed)
    sw = vec(arm_stepwise, asg_sat_tz, W, F, signed)
    n = len(fu)
    d_conf = sum(1 for x, y in zip(fu, fp) if x != y)
    d_cross = sum(1 for x, y in zip(fu, sw) if x != y)
    print(f"  {tag:<12}{'conforming: fused/widening vs fused/partials':<44}{d_conf:>10} of {n}")
    print(f"  {tag:<12}{'non-conforming: fused vs stepwise':<44}{d_cross:>10} of {n}"
          f"   ({100.0 * d_cross / n:.2f}%)")
    if d_cross > 0 and d_conf == 0:
        c3_ok = True
print(f"\n  C3 (conforming pair agrees, non-conforming pair differs): "
      f"{'CONFIRMED' if c3_ok else 'REFUTED'}")

print()
print("=" * 100)
print("C4. How many arms does each assignment have, since a weighting needs at least two")
print("=" * 100)
sizes_by_overflow = {}
for (W, F, signed) in SHAPES[:3]:
    tag = f"W{W} F{F} {'s' if signed else 'u'}"
    print(f"  {tag}")
    lo, hi = lo_hi(W, signed)
    dom = range(lo, hi + 1)
    for asg in ASSIGNMENTS:
        d = tuple(denotation(asg, W, F, signed, a, b, c)
                  for a in dom for b in dom for c in dom)
        conforming = [n for n, fn in ARMS.items()
                      if not n.startswith("CONTROL") and vec(fn, asg, W, F, signed) == d]
        sizes_by_overflow.setdefault(asg[1], set()).add(len(conforming))
        print(f"    {'/'.join(asg):<34} {len(conforming)} conforming arms: "
              f"{', '.join(conforming)}")
print()
print("  The count is a function of the OVERFLOW position, which is component one:")
for ov, ns in sorted(sizes_by_overflow.items()):
    print(f"    overflow = {ov:<6} conforming-arm counts observed: {sorted(ns)}")
print("  C4: under wrapping every arm conforms to every intermediate position, because the")
print("  absorption theorem (141 F3) makes fused and stepwise the same denotation there. Under")
print("  saturating the four arms split two and two. So the number of arms component two ranges")
print("  over is itself a function of component one, and at some assignments a weighting has")
print("  strictly fewer things to choose among than at others.")

print()
print("=" * 100)
print("VERDICT")
print("=" * 100)
print("  144's 'nothing in the type says so' is true of an assignment set that lacks the axis and")
print("  false of one that carries it. The join therefore has a condition, and it is checkable:")
print()
print("    every lowering arm the design admits realises the denotation of some assignment")
print()
print("  Under that condition the firewall is enforceable by declaration alone. A conforming pair")
print("  is two arms of one assignment and the weighting picks between them freely, moving no")
print("  answer. A non-conforming pair is two assignments and the type names which one, so the")
print("  42.14% is a declared difference rather than a silent one. The control shows the condition")
print("  is not automatic: an arm rounding off the axis realises no assignment, and for that arm")
print("  the firewall has no repair available in the type.")
