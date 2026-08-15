#!/usr/bin/env python3
"""p5b: 139 says the class count is a function of the numeral's SHAPE. p5's table
says it is a function of two of shape's three coordinates and not the third.

p5's U3 table gives, under the full operation set:

    unsigned, W=3, F in {0,1,2} -> 3, 3, 3
    unsigned, W=4, F in {0,1,2} -> 3, 3, 3
    signed,   W=3, F in {0,1,2} -> 3, 6, 6
    signed,   W=4, F in {0,1,2} -> 3, 6, 6

The integer width does nothing. Signedness and fraction width do everything.

And 139's own table already contains the same pattern without naming it. Every
pair in it that differs only in W agrees:

    W=6 F=3 unsigned  8   against  W=8 F=4 unsigned  8
    W=6 F=3 signed   12   against  W=8 F=4 signed   12
    W=6 F=0 signed    3   against  W=8 F=0 signed    3

That is worth checking properly rather than eyeballing two tables, because if it
holds it changes what the canon has to say. "The count is a function of the shape"
needs three coordinates carried everywhere. "The count is a function of the
signedness and the fraction width" needs two, and both of them are coordinates
every law result in this panel already predicates on.

PREDICTIONS, before running:
  V1. For every (signedness, F) and every witness subset, the class count is
      identical at every W large enough for the shape to be non-degenerate.
  V2. It BREAKS at a degenerate width. At W=2 with F=1 there is one integer bit
      and the value range is four points, so behaviours that separate assignments
      at larger widths have no room to occur. If V1 held even there, the check
      would be insensitive rather than informative.
  V3. Signedness and F do move the count, so V1 is a real invariance rather than
      a sweep in which nothing moves at all.

CONTROLS: V2 is the sensitivity control and V3 is the non-vacuity control. Both
have to fire or V1 says nothing.
"""

from itertools import combinations

ROUND = ("toward_zero", "floor")
OVF = ("wrap", "sat")
INTER = ("stepwise", "exact")


def bounds(sign, w):
    if sign == "u":
        return 0, (1 << w) - 1
    return -(1 << (w - 1)), (1 << (w - 1)) - 1


def reduce_(v, sign, ovf, w):
    lo, hi = bounds(sign, w)
    if ovf == "sat":
        return max(lo, min(hi, v))
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


ASSIGNMENTS = [Asg(r, o, i) for r in ROUND for o in OVF for i in INTER]
BINARY = {"add", "sub", "mul"}
ALL_OPS = ("add", "sub", "mul", "mac", "msb")


def signature(asg, ops, sign, w, f):
    lo, hi = bounds(sign, w)
    rng = range(lo, hi + 1)
    out = []
    for op in ops:
        fn = getattr(asg, op)
        if op in BINARY:
            for a in rng:
                for b in rng:
                    out.append(fn(a, b, sign, w, f))
        else:
            for a in rng:
                for b in rng:
                    for c in rng:
                        out.append(fn(a, b, c, sign, w, f))
    return tuple(out)


def classes(ops, sign, w, f):
    return len({signature(a, ops, sign, w, f) for a in ASSIGNMENTS})


subsets = []
for k in range(1, len(ALL_OPS) + 1):
    subsets.extend(combinations(ALL_OPS, k))

WIDTHS = (2, 3, 4, 5)
FRACS = (0, 1, 2)

print("p5b: is the shape dependence in W at all?")
print(f"assignments = {len(ASSIGNMENTS)}, widths = {WIDTHS}, fractions = {FRACS}")
print(f"witness subsets = {len(subsets)}\n")

counts = {}
for sign in ("u", "s"):
    for w in WIDTHS:
        for f in FRACS:
            if f >= w:
                continue
            for sub in subsets:
                counts[(sign, w, f, sub)] = classes(sub, sign, w, f)

full = tuple(ALL_OPS)
print("=== class count under the full operation set ===")
print(f"{'sign':<6} {'F':>2} " + " ".join(f"W={w:<3}" for w in WIDTHS))
for sign in ("u", "s"):
    for f in FRACS:
        row = f"{sign:<6} {f:>2} "
        for w in WIDTHS:
            row += f"{counts.get((sign, w, f, full), '-'):<5}" if f < w else "  -  "
        print(row)
print()

print("=== V1: W-invariance, over every witness subset ===")
nondegenerate = [w for w in WIDTHS if w >= 3]
mismatch = []
for sign in ("u", "s"):
    for f in FRACS:
        for sub in subsets:
            vals = {counts[(sign, w, f, sub)] for w in nondegenerate if f < w}
            if len(vals) > 1:
                mismatch.append((sign, f, sub, sorted(vals)))
print(f"W in {nondegenerate}: (sign, F, subset) triples whose count moves with W: {len(mismatch)}")
for m in mismatch[:8]:
    print(f"  {m[0]} F={m[1]} {m[2]} -> {m[3]}")
if not mismatch:
    print("  -> V1 CONFIRMED over the non-degenerate widths swept.")
print()

print("=== V2 CONTROL: does the degenerate width W=2 break it? ===")
broke = []
for sign in ("u", "s"):
    for f in FRACS:
        if f >= 2:
            continue
        for sub in subsets:
            a = counts[(sign, 2, f, sub)]
            b = counts[(sign, 3, f, sub)]
            if a != b:
                broke.append((sign, f, sub, a, b))
print(f"(sign, F, subset) triples where W=2 disagrees with W=3: {len(broke)}")
for m in broke[:8]:
    print(f"  {m[0]} F={m[1]} {m[2]} -> W=2 gives {m[3]}, W=3 gives {m[4]}")
print(
    "  -> control FIRES, the check is sensitive to width"
    if broke
    else "  -> control TOOTHLESS, the check cannot see width at all"
)
print()

print("=== V3 CONTROL: do signedness and F move the count? ===")
sgn_moves = sum(
    1
    for f in FRACS
    for w in nondegenerate
    if f < w
    for sub in subsets
    if counts[("u", w, f, sub)] != counts[("s", w, f, sub)]
)
f_moves = sum(
    1
    for sign in ("u", "s")
    for w in nondegenerate
    for sub in subsets
    if len({counts[(sign, w, f, sub)] for f in FRACS if f < w}) > 1
)
print(f"cases where signedness changes the count: {sgn_moves}")
print(f"cases where F changes the count:          {f_moves}")
print(
    "  -> control FIRES"
    if sgn_moves and f_moves
    else "  -> control TOOTHLESS, nothing moves and V1 is vacuous"
)
