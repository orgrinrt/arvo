#!/usr/bin/env python3
"""p2. The endpoints DO determine the crossing wherever nothing is lost.

`71` section 4 establishes that a composite crossing moving both the representable set and the
selected reduction is not determined by its endpoints: the two orders are two functions
agreeing on 30 of 256 source values, and the canon must name an order. I accept that finding
and this probe attacks its SCOPE, because the obligation it hands op is currently unbounded
("a canon that says 'the crossing from A to B' without naming an order has said nothing in
that case") and I think the case is much smaller than it reads.

HYPOTHESIS, written before the run. The order-dependence is confined to the region where the
Q-move actually loses something. Concretely:

  H-a. WIDENING the representable set instead of narrowing it makes both orders agree
       everywhere, because an exact step commutes with anything.
  H-b. NARROWING, restricted to source values already inside the target's set, makes both
       orders agree everywhere, for the same reason.
  H-c. The divergence is therefore not a property of the coordinate PAIR. It is a property of
       the pair together with the operand, and it is exactly the operands the crossing cannot
       carry.

`71` states the half of H-c it measured, that 0 of its divergent witnesses have a source value
inside the target's set. H-b is the converse and is what turns a one-directional observation
into a biconditional. H-a is untested by anyone.

If the hypothesis holds, the canon's obligation shrinks from "name an order for every
multi-coordinate crossing" to "name an order for every LOSSY one", and a widening composite,
which is what a promotion or an accumulator entry actually is, needs no sentence at all.

CONTROL: `71`'s narrowing figure must reproduce, or this instrument is measuring something
else. Expected from `71_probes/p2_output.txt`: 30 of 256 agreeing, 2 distinct functions.

Run: python3 p2_order_dependence_is_confined_to_loss.py
"""

from itertools import permutations

WIDE = list(range(-128, 128))
NARROW = list(range(-8, 8))

out = []


def say(s=""):
    out.append(s)
    print(s)


def wrap(v, q):
    n = len(q)
    return ((v - q[0]) % n) + q[0]


def saturate(v, q):
    return q[0] if v < q[0] else (q[-1] if v > q[-1] else v)


REDUCTIONS = {"wrap": wrap, "saturate": saturate}


class Term:
    """A telescope term, in `71`'s shape so the comparison is like for like."""

    def __init__(self, q, rho):
        self.q = q
        self.rho = rho

    def moved(self, coord, value):
        t = Term(self.q, self.rho)
        setattr(t, coord, value)
        return t


def step(value, src, dst, coord):
    """One coordinate moves; every other is shared, so nothing is chosen inside a step."""
    if coord == "q":
        assert src.rho == dst.rho
        return REDUCTIONS[src.rho](value, dst.q)
    if coord == "rho":
        assert src.q is dst.q
        return value
    raise ValueError(coord)


def route(value, start, target, order):
    cur = start
    v = value
    for coord in order:
        nxt = cur.moved(coord, getattr(target, coord))
        v = step(v, cur, nxt, coord)
        cur = nxt
    return v


def compare(start, target, source_values):
    orders = list(permutations(["q", "rho"]))
    results = {o: [route(v, start, target, o) for v in source_values] for o in orders}
    distinct = {}
    for o, r in results.items():
        distinct.setdefault(tuple(r), []).append(o)
    a, b = results[orders[0]], results[orders[1]]
    agree = sum(1 for x, y in zip(a, b) if x == y)
    return len(distinct), agree, len(source_values), orders, a, b


say("p2. order dependence in a composite crossing is confined to the lossy region")
say("=" * 78)

# ------------------------------------------------------------------- the control
say()
say("CONTROL. `71`'s narrowing case, reproduced: wide wrap -> narrow saturate.")
say()

start = Term(WIDE, "wrap")
target = Term(NARROW, "saturate")
d, agree, tot, orders, a, b = compare(start, target, WIDE)
say(f"  distinct functions        {d}")
say(f"  agreeing source values    {agree}/{tot}    (`71`: 30/256)")
say(f"  control reproduces        {d == 2 and agree == 30 and tot == 256}")
wit = [(v, x, y) for v, x, y in zip(WIDE, a, b) if x != y][:3]
say(f"  first three witnesses     {wit}")

# ------------------------------------------------------------------------- H-a
say()
say("H-a. WIDENING instead of narrowing: narrow wrap -> wide saturate.")
say("     Every source value is already in the target's set, so the Q-move is exact.")
say()

start_w = Term(NARROW, "wrap")
target_w = Term(WIDE, "saturate")
d2, agree2, tot2, _, _, _ = compare(start_w, target_w, NARROW)
say(f"  distinct functions        {d2}")
say(f"  agreeing source values    {agree2}/{tot2}")
say(f"  H-a holds                 {d2 == 1 and agree2 == tot2}")

# ------------------------------------------------------------------------- H-b
say()
say("H-b. NARROWING, restricted to source values already inside the target's set.")
say()

inrange = [v for v in WIDE if v in NARROW]
d3, agree3, tot3, _, _, _ = compare(start, target, inrange)
say(f"  source values in range    {tot3}/{len(WIDE)}")
say(f"  distinct functions        {d3}")
say(f"  agreeing source values    {agree3}/{tot3}")
say(f"  H-b holds                 {d3 == 1 and agree3 == tot3}")

# and the complement, so the partition is exhibited rather than implied
outrange = [v for v in WIDE if v not in NARROW]
d4, agree4, tot4, _, _, _ = compare(start, target, outrange)
say(f"  complement (out of range) {tot4} values, distinct functions {d4}, "
    f"agreeing {agree4}/{tot4}")

# ------------------------------------------------------------- the exact partition
say()
say("THE PARTITION, stated as a biconditional and checked value by value.")
say()

diverge = {v for v, x, y in zip(WIDE, a, b) if x != y}
lossy = {v for v in WIDE if v not in NARROW}
say(f"  values where the two orders diverge   {len(diverge)}")
say(f"  values outside the target's set       {len(lossy)}")
say(f"  the two sets are equal                {diverge == lossy}")
say(f"  divergent is contained in lossy       {diverge <= lossy}")
say()
say("  H-c AS A BICONDITIONAL IS REFUTED BY THIS PROBE'S OWN OUTPUT, and the prediction is")
say("  kept with the correction. Containment holds in one direction only: every divergent")
say("  value is lossy, and some lossy values agree anyway.")
say(f"    lossy but agreeing: {len(lossy - diverge)} values, {sorted(lossy - diverge)}")
say()
say("  The closed form for the exceptions, derived after seeing them and then checked here.")
say("  Order one is wrap onto the target; order two is clamp onto the target. Off range they")
say("  agree exactly when the wrapped representative lands ON the bound the clamp would pick:")
say("  above the window when v is congruent to hi, below it when v is congruent to lo.")
lo_t, hi_t, n_t = NARROW[0], NARROW[-1], len(NARROW)
predicted = {v for v in WIDE if v not in NARROW
             and ((v > hi_t and (v - lo_t) % n_t == hi_t - lo_t)
                  or (v < lo_t and (v - lo_t) % n_t == 0))}
say(f"    predicted agreeing-but-lossy set: {len(predicted)} values, {sorted(predicted)}")
say(f"    predictor is exact:               {predicted == (lossy - diverge)}")

# ------------------------------------------------------- is it symmetric in the pair?
say()
say("BOTH DIRECTIONS of the reduction pair, so the finding is not about saturation.")
say()
for rs, rt in (("wrap", "saturate"), ("saturate", "wrap"), ("wrap", "wrap"),
               ("saturate", "saturate")):
    s = Term(WIDE, rs)
    t = Term(NARROW, rt)
    dd, ag, tt, _, _, _ = compare(s, t, WIDE)
    note = "  (single coordinate, no order to name)" if rs == rt else ""
    say(f"  {rs:9s} -> {rt:9s}  distinct {dd}  agreeing {ag}/{tt}{note}")

# ------------------------------------------------------------------------- verdict
say()
say("=" * 78)
say("VERDICT")
say()
say("  `71` section 4's finding stands: a composite crossing over {Q, rho} is not determined")
say("  by its endpoints, and the canon must name an order.")
say()
say("  Its SCOPE is narrower than the section states, in one direction and not two. The two")
say("  orders are the same function on every source value the crossing carries WITHOUT LOSS,")
say("  measured here at 16 of 16 in range and 16 of 16 under widening. Off range they usually")
say("  differ and sometimes coincide, so the divergent set is contained in the lossy set and")
say("  is not equal to it.")
say()
say("  The containment is the direction that matters for what op is asked. The obligation is")
say("  not 'name an order for every multi-coordinate crossing'. It is 'name an order for every")
say("  LOSSY one', and a crossing that loses nothing is endpoint-determined and needs no canon")
say("  sentence at all.")
say()
say("  That matters because widening composites are the common case in the material this")
say("  panel already carries: an accumulator entry, a promotion into a wider intermediate,")
say("  and the window mechanism a chain uses are all Q-growing.")
say()
say("  What this does not establish: anything about three or more coordinates (`71` measured")
say("  that and found two classes, which this file does not re-derive); anything about a")
say("  crossing whose ambient domain also moves; any magnitude. One model pair of widths,")
say("  exhaustive within it.")

with open("p2_order_dependence_is_confined_to_loss.out", "w") as f:
    f.write("\n".join(out) + "\n")
