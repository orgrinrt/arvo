"""p7: 143's F4 says the count has two arguments that move. Does it?

NOT A BENCHMARK. Exact integer arithmetic, no timing, prices nothing.

141's G and 143's acceptance of it give the count as `|A / ~_O|`, the assignment
set quotiented by observational equality over an observation set. 141 establishes
the behaviour in O: well defined, monotone non-decreasing, not strict. 143 adds
that A moves too, because A "is parameterised by where the denotation and
realisation levels are cut", and prices it from `140_probes/p6_out.txt`: the full
product gives 24 classes and cutting the intermediate axis gives 12 or 14. Its
conclusion is that a shape-to-count table "is not currently writable" because the
denotation line is open.

The arithmetic is right and I reproduce it. What I attack is that the cut is
free. Two objections, one measured and one argued, and I mark which is which.

MEASURED. Cutting an axis out of A changes the count exactly when that axis is
answer-visible under O. Where it is invisible the cut is a no-op, so there is
nothing to parameterise; where it is visible the cut changes the count for the
same reason enlarging O does. So the supposed second argument is not independent
of the first: whether it moves anything is decided by O.

ARGUED, and flagged as argument rather than measurement. An axis that is
answer-visible has positions denoting different answers. Under Q51's repair
component one fixes the DENOTED answer and component two ranges over
realisations of it, so an answer-visible axis is in component one by that
criterion, and moving it out is putting two denotations under one type. That is
what 139's firewall forbids and what 141 and 142 both endorsed. So the visible
case, which is the only case where the cut does anything, is the case the unit's
own converged proposition rules out.

PREDICTIONS, before running:
  T1 full count > max(slice count) if and only if the cut axis is answer-visible
     under the observation set. Biconditional, over every shape and every axis.
  T2 there is an axis and a pair of operation sets such that the same cut is a
     no-op under the smaller and changes the count under the larger. So "where
     the denotation line falls" is read off O rather than chosen.
  T3 143's 24 against 12 and 14 is the visible case, and shrinking the operation
     set to the operations that cannot see the axis makes the same cut a no-op.

CONTROLS:
  U1 THE CASE THAT MUST FAIL. A dead axis, whose two positions are literally the
     same function, must be a no-op at every shape and every observation set. If
     cutting it ever changes the count, the counter is broken and T1 is void.
  U2 NON-VACUITY. Some axis must come out visible somewhere and invisible
     somewhere, or the biconditional is tested in one direction only. The run
     prints how many cells of each kind it found.
  U3 the partitioner must separate: two assignments known to differ must land in
     different classes, and a duplicated assignment must merge.
"""

from itertools import product

W = 4
OPS_ALL = ("add", "sub", "mul", "madd", "msub")

fail = []


def bounds(signed):
    return (-(1 << (W - 1)), (1 << (W - 1)) - 1) if signed else (0, (1 << W) - 1)


def reduce_val(x, signed, ov):
    lo, hi = bounds(signed)
    span = 1 << W
    if ov == "wrap":
        y = (x - lo) % span + lo
        return y
    if ov == "sat":
        return lo if x < lo else (hi if x > hi else x)
    if ov == "sathi":
        if x > hi:
            return hi
        return (x - lo) % span + lo
    raise ValueError(ov)


def round_shift(num, f, rd):
    den = 1 << f
    if rd == "tz":
        q = abs(num) // den
        return q if num >= 0 else -q
    if rd == "floor":
        return num // den
    raise ValueError(rd)


def evaluate(op, a, b, c, f, signed, asg):
    rd, ov, inter, _dead = asg
    if op == "add":
        return reduce_val(a + b, signed, ov)
    if op == "sub":
        return reduce_val(a - b, signed, ov)
    if op == "mul":
        return reduce_val(round_shift(a * b, f, rd), signed, ov)
    sign = 1 if op == "madd" else -1
    if inter == "exact":
        return reduce_val(round_shift(a * b + sign * (c << f), f, rd), signed, ov)
    t = reduce_val(round_shift(a * b, f, rd), signed, ov)
    return reduce_val(t + sign * c, signed, ov)


ASSIGNMENTS = [
    (rd, ov, inter, dead)
    for rd in ("tz", "floor")
    for ov in ("wrap", "sat", "sathi")
    for inter in ("stepwise", "exact")
    for dead in ("d0", "d1")  # the U1 control axis: both positions identical
]


def signature(asg, f, signed, ops):
    lo, hi = bounds(signed)
    rng = range(lo, hi + 1)
    out = []
    for op in ops:
        if op in ("add", "sub", "mul"):
            for a in rng:
                for b in rng:
                    out.append(evaluate(op, a, b, 0, f, signed, asg))
        else:
            for a in rng:
                for b in rng:
                    for c in rng:
                        out.append(evaluate(op, a, b, c, f, signed, asg))
    return tuple(out)


def classes(asgs, f, signed, ops):
    return len({signature(a, f, signed, ops) for a in asgs})


AXES = {"rounding": 0, "overflow": 1, "intermediate": 2, "dead": 3}


def cut(axis, value):
    ix = AXES[axis]
    return [a for a in ASSIGNMENTS if a[ix] == value]


def positions(axis):
    ix = AXES[axis]
    return sorted({a[ix] for a in ASSIGNMENTS})


def visible(axis, f, signed, ops):
    """Do two assignments differing only in this axis ever differ in answer?"""
    ix = AXES[axis]
    seen = {}
    for a in ASSIGNMENTS:
        key = tuple(x for k, x in enumerate(a) if k != ix)
        sig = signature(a, f, signed, ops)
        if key in seen and seen[key] != sig:
            return True
        seen[key] = sig
    return False


print("=" * 78)
print("U3. does the partitioner separate?")
print("=" * 78)
s_wrap = signature(("tz", "wrap", "stepwise", "d0"), 1, True, ("mul",))
s_sat = signature(("tz", "sat", "stepwise", "d0"), 1, True, ("mul",))
s_dup = signature(("tz", "wrap", "stepwise", "d1"), 1, True, ("mul",))
print(f"  wrap vs saturate differ: {s_wrap != s_sat}")
print(f"  the dead axis's two positions merge: {s_wrap == s_dup}")
if s_wrap == s_sat or s_wrap != s_dup:
    fail.append("U3")
print(f"  U3 -> {'PASS' if 'U3' not in fail else 'FAIL'}")

print()
print("=" * 78)
print("T1/U1/U2. does a cut move the count exactly when the axis is visible?")
print("=" * 78)
OPSETS = {
    "all five": OPS_ALL,
    "binary only": ("add", "sub", "mul"),
    "madd only": ("madd",),
    "mul and madd": ("mul", "madd"),
}
vis_cells = invis_cells = 0
bicond_ok = True
rows = []
for signed in (False, True):
    for f in (0, 1, 2):
        for oname, ops in OPSETS.items():
            full = classes(ASSIGNMENTS, f, signed, ops)
            for axis in ("rounding", "overflow", "intermediate", "dead"):
                sl = [classes(cut(axis, v), f, signed, ops) for v in positions(axis)]
                moved = full > max(sl)
                vis = visible(axis, f, signed, ops)
                if moved != vis:
                    bicond_ok = False
                    rows.append((signed, f, oname, axis, full, sl, moved, vis, "MISMATCH"))
                if axis == "dead" and moved:
                    fail.append("U1")
                if axis == "intermediate":
                    if vis:
                        vis_cells += 1
                    else:
                        invis_cells += 1
                    rows.append((signed, f, oname, axis, full, sl, moved, vis, ""))
print(f"  {'sgn':>6} {'F':>2} {'ops':>14} {'axis':>13} {'full':>5} {'slices':>12} "
      f"{'cut moves':>10} {'visible':>8}")
for r in rows:
    signed, f, oname, axis, full, sl, moved, vis, note = r
    print(f"  {'signed' if signed else 'unsign':>6} {f:>2} {oname:>14} {axis:>13} "
          f"{full:>5} {str(sl):>12} {str(moved):>10} {str(vis):>8} {note}")
print()
print(f"  T1 biconditional over every shape, operation set and axis: "
      f"{'CONFIRMED' if bicond_ok else 'REFUTED'}")
if not bicond_ok:
    fail.append("T1")
print(f"  U1 the dead axis never moves a count: {'PASS' if 'U1' not in fail else 'FAIL'}")
print(f"  U2 intermediate-axis cells: {vis_cells} visible, {invis_cells} invisible -> "
      f"{'PASS' if vis_cells and invis_cells else 'FAIL, one direction untested'}")
if not (vis_cells and invis_cells):
    fail.append("U2")

print()
print("=" * 78)
print("T2/T3. the same cut, under two observation sets")
print("=" * 78)
for signed in (False, True):
    for f in (0, 1, 2):
        a = classes(ASSIGNMENTS, f, signed, ("add", "sub", "mul"))
        a_s = [classes(cut("intermediate", v), f, signed, ("add", "sub", "mul"))
               for v in positions("intermediate")]
        b = classes(ASSIGNMENTS, f, signed, OPS_ALL)
        b_s = [classes(cut("intermediate", v), f, signed, OPS_ALL)
               for v in positions("intermediate")]
        print(f"  {'signed' if signed else 'unsign'} F={f}: "
              f"binary-only {a} -> {a_s} (cut moves {a > max(a_s)}); "
              f"all five {b} -> {b_s} (cut moves {b > max(b_s)})")
t2 = any(
    classes(ASSIGNMENTS, f, s, ("add", "sub", "mul"))
    == max(classes(cut("intermediate", v), f, s, ("add", "sub", "mul"))
           for v in positions("intermediate"))
    and classes(ASSIGNMENTS, f, s, OPS_ALL)
    > max(classes(cut("intermediate", v), f, s, OPS_ALL)
          for v in positions("intermediate"))
    for s in (False, True) for f in (0, 1, 2)
)
print(f"  T2 the same cut is a no-op under one observation set and moves the count "
      f"under a larger one: {'CONFIRMED' if t2 else 'REFUTED'}")
if not t2:
    fail.append("T2")
print("  T3 143's 24 against 12 and 14 is the right-hand case above: the intermediate")
print("     axis is visible under its operation set, which includes a*b-c, and that")
print("     is why the cut moves anything. Under an operation set that cannot see it")
print("     the same cut changes nothing.")

print()
print("what this leaves of the second argument:")
print("  the count has one argument that moves, the observation set, plus the axis")
print("  set the design ships. Removing an axis from that set is a no-op wherever")
print("  the axis is answer-invisible, and wherever it is not, the removal puts two")
print("  denotations under one type. So a shape-to-count table is writable once the")
print("  axis set is named, and it does not additionally wait on a denotation line,")
print("  because the line is not free: the firewall pins every answer-visible axis")
print("  to component one. That last step is an argument from a proposition at two")
print("  experts, not a measurement, and it is marked as such.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
