#!/usr/bin/env python3
"""
p2. Does a crossing between two systems that differ at several coordinates depend on the order
the coordinates are crossed in?

HYPOTHESIS, written before the run.

`67` identifies a crossing by "the smallest index at which the two terms disagree", which is
well defined only when the terms disagree at ONE index. Two systems routinely disagree at
several. If the composite crossing is order-dependent, then the pair of endpoints does not
determine the crossing, and no canon sentence of the form "the crossing from A to B" is
well formed without naming an order.

The construction that makes each step unambiguous, and it is the point of the design:

    A step moves exactly ONE coordinate. The two terms either side of a step therefore AGREE
    on every other coordinate, so where the step needs a reduction, there is exactly one
    reduction in scope and nothing is chosen. Every single-coordinate crossing is canonical.

So any order-dependence in the composite is not an artifact of an arbitrary choice inside a
step. It is a fact about composition.

PREDICTION: of the six unordered coordinate pairs over {Q, rho, E, C}, exactly one diverges,
the pair {Q, rho}, because a Q-move is the only lossy step and rho is the only coordinate that
changes what the loss does. All ten remaining route comparisons agree at 256/256. For the
three-coordinate case {Q, rho, E} the six orders collapse to exactly 2 distinct functions,
grouped by whether the Q-move precedes or follows the rho-move.

Model width 4 for the narrow set, 8 for the wide. Exhaustive over the wide set.
"""

from itertools import permutations

WIDE = list(range(-128, 128))
NARROW = list(range(-8, 8))


def wrap(v, q):
    n = len(q)
    return ((v - q[0]) % n) + q[0]


def saturate(v, q):
    return q[0] if v < q[0] else (q[-1] if v > q[-1] else v)


REDUCTIONS = {"wrap": wrap, "saturate": saturate}

ENCODE = {
    "twos": lambda v, q: (v - q[0]) if False else (v & (len(q) - 1)),
    "excess": lambda v, q: v - q[0],
}
DECODE = {
    "twos": lambda p, q: (p - len(q)) if p >= (len(q) >> 1) else p,
    "excess": lambda p, q: p + q[0],
}


class Term:
    """A completed telescope term, four movable coordinates below the ambient domain."""

    def __init__(self, q, rho, enc, offset):
        self.q = q
        self.rho = rho
        self.enc = enc
        self.offset = offset

    def moved(self, coord, value):
        t = Term(self.q, self.rho, self.enc, self.offset)
        setattr(t, coord, value)
        return t

    def key(self):
        return (len(self.q), self.q[0], self.rho, self.enc, self.offset)

    def store(self, v):
        return ENCODE[self.enc](v, self.q) << self.offset

    def load(self, byte):
        return DECODE[self.enc]((byte >> self.offset) & (len(self.q) - 1), self.q)


def step(value, src, dst, coord):
    """Cross one coordinate. Every other coordinate is shared, so nothing is chosen here."""
    if coord == "q":
        # The reduction is shared by construction: src.rho == dst.rho.
        assert src.rho == dst.rho
        return REDUCTIONS[src.rho](value, dst.q)
    if coord == "rho":
        # Same representable set on both sides, so the value map is the identity.
        assert src.q is dst.q
        return value
    if coord in ("enc", "offset"):
        # Realisation only: decode under the source, re-encode under the target.
        assert src.q is dst.q
        return dst.load(dst.store(value))
    raise ValueError(coord)


def route(value, start, target, order):
    """Walk the coordinates in the given order, one step each."""
    cur = start
    v = value
    for coord in order:
        nxt = cur.moved(coord, getattr(target, coord))
        v = step(v, cur, nxt, coord)
        cur = nxt
    return v


def compare(start, target, coords):
    """All orders of the differing coordinates; group source values by result tuple."""
    orders = list(permutations(coords))
    results = {}
    for o in orders:
        results[o] = [route(v, start, target, o) for v in WIDE]
    distinct = {}
    for o, r in results.items():
        distinct.setdefault(tuple(r), []).append(o)
    return orders, results, distinct


BASE = Term(WIDE, "wrap", "twos", 0)

PAIRS = [
    ("q", "rho"),
    ("q", "enc"),
    ("q", "offset"),
    ("rho", "enc"),
    ("rho", "offset"),
    ("enc", "offset"),
]

TARGET_VALUES = {"q": NARROW, "rho": "saturate", "enc": "excess", "offset": 2}

print("=" * 78)
print("PAIRS OF COORDINATES: does the composite crossing depend on the order?")
print("=" * 78)
print(f"{'pair':<18}{'orders':<8}{'distinct functions':<22}{'agreeing source values'}")

divergent = []
for pair in PAIRS:
    target = BASE
    for c in pair:
        target = target.moved(c, TARGET_VALUES[c])
    orders, results, distinct = compare(BASE, target, list(pair))
    a = results[orders[0]]
    b = results[orders[1]]
    agree = sum(1 for x, y in zip(a, b) if x == y)
    label = f"{{{pair[0]}, {pair[1]}}}"
    print(f"{label:<18}{len(orders):<8}{len(distinct):<22}{agree}/{len(WIDE)}")
    if len(distinct) > 1:
        divergent.append((pair, agree, a, b, orders))

print()
print(f"Divergent pairs: {[p for p, *_ in divergent]}")

for pair, agree, a, b, orders in divergent:
    print()
    print("-" * 78)
    print(f"THE DIVERGENT PAIR {pair}, in detail")
    print("-" * 78)
    print(f"  order {orders[0]} : the {pair[0]}-move happens under the SOURCE's other")
    print(f"  order {orders[1]} : the {pair[0]}-move happens under the TARGET's other")
    print(f"  agreement {agree}/{len(WIDE)} source values, "
          f"{len(WIDE) - agree} divergent")
    ex = [(v, x, y) for v, x, y in zip(WIDE, a, b) if x != y]
    print(f"  first three witnesses (source value, route 1, route 2):")
    for w in ex[:3]:
        print(f"    v = {w[0]:>5}   ->  {w[1]:>4}   against  {w[2]:>4}")
    inrange = [w for w in ex if -8 <= w[0] <= 7]
    print(f"  witnesses whose source value is already inside the target's set: {len(inrange)}")

# --------------------------------------------------------- the three-coordinate case

print()
print("=" * 78)
print("THREE COORDINATES AT ONCE: {q, rho, enc}")
print("=" * 78)
target3 = BASE
for c in ("q", "rho", "enc"):
    target3 = target3.moved(c, TARGET_VALUES[c])
orders3, results3, distinct3 = compare(BASE, target3, ["q", "rho", "enc"])
print(f"  orders                  {len(orders3)}")
print(f"  distinct functions      {len(distinct3)}")
for i, (res, os) in enumerate(distinct3.items()):
    print(f"  class {i + 1}: {len(os)} orders, e.g. {os[0]}")
    q_before_rho = [o.index("q") < o.index("rho") for o in os]
    print(f"          q-move precedes rho-move in all of them: {all(q_before_rho)}; "
          f"in none: {not any(q_before_rho)}")

print()
print("=" * 78)
print("VERDICT")
print("=" * 78)
if len(divergent) == 1 and divergent[0][0] == ("q", "rho") and len(distinct3) == 2:
    print("PREDICTION HELD in both halves.")
else:
    print("PREDICTION REFUTED. Divergent pairs and the three-coordinate class count above.")
print()
print("Reading. Every single-coordinate crossing is canonical, because the coordinate not")
print("moving is shared and so nothing is chosen inside a step. The composite is not: a")
print("crossing that both narrows the representable set and changes the selected reduction")
print("computes a different function depending on which move happens first, and the two")
print("answers are exactly 'the source's reduction governs the loss' and 'the target's does'.")
print("So the endpoints do not determine the crossing, and 'the crossing from A to B' is not")
print("a well-formed phrase unless the systems differ at one coordinate, or an order is named.")
