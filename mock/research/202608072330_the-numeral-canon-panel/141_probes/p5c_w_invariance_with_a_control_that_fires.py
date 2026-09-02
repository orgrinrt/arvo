#!/usr/bin/env python3
"""p5c: p5b confirmed W-invariance and its sensitivity control came out TOOTHLESS,
so the zero it reported was not earned and is not usable.

p5b predicted that a degenerate width W=2 would break the invariance and it did
not: 0 of 31 subsets disagreed between W=2 and W=3. A check that reports "the
count does not move with W" and cannot be made to move with W at all is
indistinguishable from a check that is blind to W, and the honest reading of
p5b's V1 is therefore "unestablished" rather than "confirmed".

What was missing is an axis position whose observability genuinely depends on the
integer width. The three axes I swept do not have one: rounding, overflow and
intermediate placement all behave the same way at every width, which is why the
count could not move and why the finding, if real, is a finding about THOSE axes
rather than about axes in general.

So this probe adds one axis position that is width-sensitive by construction, as
a control rather than as a design proposal: an overflow position that clamps the
high side at the literal constant 6. Below W=3 unsigned that constant is outside
the representable range, so it is indistinguishable from ordinary saturation and
merges; at W=3 and above it is inside, so it separates. If the count then moves
with W, the instrument can see width, and the invariance measured without it is a
statement about the axes rather than about the instrument.

PREDICTIONS, before running:
  V4. With the width-sensitive position present, the class count moves between
      W=2 and W=3 for at least one (sign, F, subset). Control fires.
  V5. With it absent, the count is invariant across W in {3,4,5,6} for every
      (sign, F, subset), extending p5b's range by one width.
  V6. The width-sensitive position is the ONLY source of W-dependence here, so
      with it present the count is still invariant across W in {3,4,5,6}: once the
      constant is inside the range it stays inside, and widening further adds
      nothing. If that also holds, the honest statement is not "the count is
      W-invariant" but "the count is invariant in W above the width at which every
      axis position becomes reachable", which is a different and more useful claim.
"""

from itertools import combinations

ROUND = ("toward_zero", "floor")
INTER = ("stepwise", "exact")


def bounds(sign, w):
    if sign == "u":
        return 0, (1 << w) - 1
    return -(1 << (w - 1)), (1 << (w - 1)) - 1


def reduce_(v, sign, ovf, w):
    lo, hi = bounds(sign, w)
    if ovf == "sat":
        return max(lo, min(hi, v))
    if ovf == "sat_at_six":
        return max(lo, min(min(hi, 6), v))
    m = 1 << w
    r = v % m
    if sign == "s" and r >= (1 << (w - 1)):
        r -= m
    return r


def shift(p, f, rnd):
    if f == 0:
        return p
    d = 1 << f
    if rnd == "floor":
        return p >> f
    q = abs(p) // d
    return q if p >= 0 else -q


class Asg:
    def __init__(self, rnd, ovf, inter):
        self.rnd, self.ovf, self.inter = rnd, ovf, inter

    def red(self, v, sign, w):
        return reduce_(v, sign, self.ovf, w)

    def mul(self, a, b, sign, w, f):
        return self.red(shift(a * b, f, self.rnd), sign, w)

    def add(self, a, b, sign, w, f):
        return self.red(a + b, sign, w)

    def sub(self, a, b, sign, w, f):
        return self.red(a - b, sign, w)

    def mac(self, a, b, c, sign, w, f):
        p = shift(a * b, f, self.rnd)
        if self.inter == "stepwise":
            p = self.red(p, sign, w)
        return self.red(p + c, sign, w)

    def msb(self, a, b, c, sign, w, f):
        p = shift(a * b, f, self.rnd)
        if self.inter == "stepwise":
            p = self.red(p, sign, w)
        return self.red(p - c, sign, w)


BINARY = {"add", "sub", "mul"}
ALL_OPS = ("add", "sub", "mul", "mac", "msb")


def make(ovfs):
    return [Asg(r, o, i) for r in ROUND for o in ovfs for i in INTER]


_PER_OP = {}


def per_op(asg_i, asgs, op, sign, w, f):
    """Signature of ONE operation, memoised. Recomputing the whole signature per
    subset was quadratic enough to outrun a ten minute budget at W=6; the answer
    functions do not depend on which subset is being asked about."""
    key = (id(asgs), asg_i, op, sign, w, f)
    v = _PER_OP.get(key)
    if v is not None:
        return v
    asg = asgs[asg_i]
    lo, hi = bounds(sign, w)
    rng = range(lo, hi + 1)
    fn = getattr(asg, op)
    if op in BINARY:
        out = tuple(fn(a, b, sign, w, f) for a in rng for b in rng)
    else:
        out = tuple(fn(a, b, c, sign, w, f) for a in rng for b in rng for c in rng)
    _PER_OP[key] = out
    return out


def classes(asgs, ops, sign, w, f):
    return len(
        {tuple(per_op(i, asgs, op, sign, w, f) for op in ops) for i in range(len(asgs))}
    )


subsets = []
for k in range(1, len(ALL_OPS) + 1):
    subsets.extend(combinations(ALL_OPS, k))

BASE = ("wrap", "sat")
WITH_CTRL = ("wrap", "sat", "sat_at_six")
FRACS = (0, 1, 2)
full = tuple(ALL_OPS)

print("p5c: W-invariance re-checked with a control that can fire")
print(f"base axes: rounding x {BASE} x intermediate = {len(make(BASE))} assignments")
print(f"with control: rounding x {WITH_CTRL} x intermediate = {len(make(WITH_CTRL))}\n")


def table(ovfs, widths):
    c = {}
    asgs = make(ovfs)
    for sign in ("u", "s"):
        for w in widths:
            for f in FRACS:
                if f >= w:
                    continue
                for sub in subsets:
                    c[(sign, w, f, sub)] = classes(asgs, sub, sign, w, f)
    return c


print("=== V4: control present, does the count move between W=2 and W=3? ===")
ctrl = table(WITH_CTRL, (2, 3))
moved = [
    (sign, f, sub, ctrl[(sign, 2, f, sub)], ctrl[(sign, 3, f, sub)])
    for sign in ("u", "s")
    for f in FRACS
    if f < 2
    for sub in subsets
    if ctrl[(sign, 2, f, sub)] != ctrl[(sign, 3, f, sub)]
]
print(f"(sign, F, subset) triples whose count moves: {len(moved)}")
for m in moved[:6]:
    print(f"  {m[0]} F={m[1]} {m[2]} -> W=2 gives {m[3]}, W=3 gives {m[4]}")
print("  -> control FIRES" if moved else "  -> control TOOTHLESS AGAIN, V5 unestablished")
print()

WIDE = (3, 4, 5, 6)

print("=== V5: base axes, invariance across W in {3,4,5,6} ===")
base = table(BASE, WIDE)
mm = []
for sign in ("u", "s"):
    for f in FRACS:
        for sub in subsets:
            vals = {base[(sign, w, f, sub)] for w in WIDE}
            if len(vals) > 1:
                mm.append((sign, f, sub, sorted(vals)))
print(f"triples whose count moves with W: {len(mm)}")
for m in mm[:6]:
    print(f"  {m[0]} F={m[1]} {m[2]} -> {m[3]}")
print("  -> V5 CONFIRMED" if not mm else "  -> V5 REFUTED")
print()

print("=== V6: control present, invariance across W in {3,4,5,6} ===")
ctrl_wide = table(WITH_CTRL, WIDE)
mm2 = []
for sign in ("u", "s"):
    for f in FRACS:
        for sub in subsets:
            vals = {ctrl_wide[(sign, w, f, sub)] for w in WIDE}
            if len(vals) > 1:
                mm2.append((sign, f, sub, sorted(vals)))
print(f"triples whose count moves with W: {len(mm2)}")
for m in mm2[:6]:
    print(f"  {m[0]} F={m[1]} {m[2]} -> {m[3]}")
print(
    "  -> V6 CONFIRMED: invariance resumes once every position is reachable"
    if not mm2
    else "  -> V6 REFUTED: the count keeps moving with W even above the threshold"
)
print()

print("=== class counts under the full operation set, base axes ===")
print(f"{'sign':<6} {'F':>2} " + " ".join(f"W={w:<3}" for w in WIDE))
for sign in ("u", "s"):
    for f in FRACS:
        print(f"{sign:<6} {f:>2} " + " ".join(f"{base[(sign, w, f, full)]:<5}" for w in WIDE))
