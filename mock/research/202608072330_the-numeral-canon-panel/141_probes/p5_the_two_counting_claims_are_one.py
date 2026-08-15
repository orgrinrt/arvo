#!/usr/bin/env python3
"""p5: are 139's counting claim and 140's counting claim one finding or two?

The dispatch asks me to establish this by construction rather than by comparing
prose, so this is a single instrument that varies BOTH dimensions.

  139 varies the SHAPE with the operation set held fixed, and reports the class
      count moving 2 / 3 / 8 / 12 across shapes. It concludes "the count is not a
      property of the design at all".

  140 varies the OPERATION SET with the shape held fixed, and reports the count
      moving 15 -> 24 by adding one operation. Its F2 states the count is "a
      strictly increasing function of the witness set".

Both are quotients of one assignment set by observational equality. The thing
being quotiented by is the OBSERVATION SET, and shape and operation are both
components of it: an observation is a (shape, operation, input) triple, and
fixing a shape or fixing an operation set are two ways of restricting the same
set. So the two findings should be instances of one statement, and the way to
show that is to make both movable on one instrument and check the structure.

This is written in Python over exact integers, so it shares no code with my
Rust probes p1 to p4 and no code with either cold derivation.

PREDICTIONS, before running:
  U1. MONOTONICITY IS A THEOREM, not a measurement. If O1 is a subset of O2 then
      equality on O2 implies equality on O1, so the O2 partition refines the O1
      partition and cannot have fewer classes. Zero violations expected over
      every subset pair at every shape. A violation would mean the instrument is
      broken, not that the theorem is false.
  U2. 140's F2 as written, "a strictly increasing function of the witness set",
      is FALSE. Monotone non-decreasing is the theorem; strict increase is a
      property of the particular operation added. I expect to find at least one
      (shape, subset, operation) where adding an operation adds zero classes.
  U3. 139's "the count is not a property of the design at all" is TOO STRONG.
      With the shape fixed and the operation set taken to be the whole design's,
      the count is determined. What is not a property of the design is a single
      shape-free number, which is a weaker and true statement.
  U4. NEW, and the reason this is worth building: the shape-dependence is itself
      witness-set-dependent. I expect to find two shapes whose ORDER BY CLASS
      COUNT REVERSES between two operation sets. If that happens, neither
      variation is prior to the other and the two findings are symmetric halves
      of one statement rather than one being a special case of the other.

CONTROLS:
  C-null. An operation that returns a constant must add zero classes at every
      shape and from every subset. If it adds one, the partition is keyed on
      something other than the answers.
  C-live. The partition must be non-trivial somewhere, that is more than one
      class at some shape under the full set. Otherwise every count is 1 and
      every claim above is vacuous.
  C-dup. A duplicate assignment reached by a different construction must merge.
"""

from itertools import combinations

# ---------- the model ----------

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
    # truncate toward zero
    q = abs(p) // d
    return q if p >= 0 else -q


class Asg:
    def __init__(self, rnd, ovf, inter):
        self.rnd, self.ovf, self.inter = rnd, ovf, inter

    def key(self):
        return (self.rnd, self.ovf, self.inter)

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

    def zero(self, a, b, sign, w, f):
        """C-null: a constant operation."""
        return 0


ASSIGNMENTS = [Asg(r, o, i) for r in ROUND for o in OVF for i in INTER]

BINARY = {"add", "sub", "mul", "zero"}
TERNARY = {"mac", "msb"}
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


def classes(ops, sign, w, f, extra=()):
    seen = set()
    for asg in list(ASSIGNMENTS) + list(extra):
        seen.add(signature(asg, ops, sign, w, f))
    return len(seen)


# ---------- the sweep ----------

SHAPES = [(s, w, f) for s in ("u", "s") for w in (3, 4) for f in (0, 1, 2)]

print("p5: are the two counting claims one finding?")
print(f"assignments = {len(ASSIGNMENTS)} (rounding x overflow x intermediate)")
print(f"shapes = {len(SHAPES)}, operations = {ALL_OPS}\n")

# U1: monotonicity over every subset pair, at every shape.
print("=== U1: monotonicity of the class count in the observation set ===")
violations = 0
checked = 0
subsets = []
for k in range(1, len(ALL_OPS) + 1):
    subsets.extend(combinations(ALL_OPS, k))
counts = {}
for sh in SHAPES:
    sign, w, f = sh
    for sub in subsets:
        counts[(sh, sub)] = classes(sub, sign, w, f)
for sh in SHAPES:
    for a in subsets:
        for b in subsets:
            if set(a) <= set(b):
                checked += 1
                if counts[(sh, a)] > counts[(sh, b)]:
                    violations += 1
print(f"subset pairs checked: {checked}, monotonicity violations: {violations}")
print("(a violation would mean the instrument is broken; the statement is a theorem)\n")

# U2: is the increase strict?
print("=== U2: is the count STRICTLY increasing in the witness set? ===")
zero_gain = []
for sh in SHAPES:
    for sub in subsets:
        for op in ALL_OPS:
            if op in sub:
                continue
            bigger = tuple(o for o in ALL_OPS if o in set(sub) | {op})
            if counts[(sh, bigger)] == counts[(sh, sub)]:
                zero_gain.append((sh, sub, op, counts[(sh, sub)]))
print(f"(shape, subset, added operation) triples that add ZERO classes: {len(zero_gain)}")
for row in zero_gain[:8]:
    sh, sub, op, c = row
    print(f"  shape {sh}: {sub} + '{op}' stays at {c} classes")
if zero_gain:
    print("  -> 140's F2 wording 'strictly increasing' is REFUTED as a universal.")
    print("     Monotone non-decreasing is the theorem; strictness is a fact about")
    print("     the operation added, not about the witness set.")
else:
    print("  -> no counterexample found in this sweep; F2's wording survives here.")
print()

# U3: the full-set count per shape.
print("=== U3: the count at a fixed shape under the full operation set ===")
full = tuple(ALL_OPS)
print(f"{'shape':<16} {'classes':>8}")
for sh in SHAPES:
    print(f"{str(sh):<16} {counts[(sh, full)]:>8}")
print("  -> a well-defined function from shape to count. Not a single number, and")
print("     not 'not a property of the design': it is a property with an argument.\n")

# U4: does the ORDER of two shapes by class count reverse between witness sets?
print("=== U4: does the shape ordering reverse between witness sets? ===")
reversals = []
for i, sh1 in enumerate(SHAPES):
    for sh2 in SHAPES[i + 1 :]:
        for a in subsets:
            for b in subsets:
                c1a, c2a = counts[(sh1, a)], counts[(sh2, a)]
                c1b, c2b = counts[(sh1, b)], counts[(sh2, b)]
                if c1a < c2a and c1b > c2b:
                    reversals.append((sh1, sh2, a, b, c1a, c2a, c1b, c2b))
print(f"reversing (shape pair, witness set pair) instances: {len(reversals)}")
for r in reversals[:6]:
    sh1, sh2, a, b, c1a, c2a, c1b, c2b = r
    print(f"  {sh1} vs {sh2}: under {a} -> {c1a} < {c2a}; under {b} -> {c1b} > {c2b}")
if reversals:
    print("  -> U4 CONFIRMED. Neither variation is prior to the other: which shape")
    print("     admits more strategies depends on which operations you look at.")
else:
    print("  -> U4 REFUTED in this sweep: the shape ordering is witness-set stable here.")
print()

# controls
print("=== controls ===")
null_gain = 0
for sh in SHAPES:
    sign, w, f = sh
    for sub in subsets:
        with_null = classes(tuple(sub) + ("zero",), sign, w, f)
        if with_null != counts[(sh, sub)]:
            null_gain += 1
print(f"C-null: constant operation changed the count in {null_gain} cases (must be 0)")

live = max(counts[(sh, full)] for sh in SHAPES)
print(f"C-live: maximum class count over all shapes under the full set = {live} (must be > 1)")

dup = ASSIGNMENTS[0]
sign, w, f = SHAPES[0]
n_dup = classes(full, sign, w, f, extra=(Asg(dup.rnd, dup.ovf, dup.inter),))
n_nodup = classes(full, sign, w, f)
print(f"C-dup : {n_dup} with a duplicate vs {n_nodup} without (must be equal)")
