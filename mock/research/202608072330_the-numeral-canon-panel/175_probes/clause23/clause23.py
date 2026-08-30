#!/usr/bin/env python3
"""P4. Do clauses 2 and 3 denote together?

Clause 2: "the design may select any realisation that induces **the stretch's
boundary function** on its definedness domain."
Clause 3: "An adaptation point on a bound edge is forced; **on an unbound edge it
is free and placed under clause 2**", and, in the same clause, "two schedules
over the same operations compute different functions."

If the schedule is free on an unbound edge, and schedules compute different
functions, then a stretch with a free adaptation point has a FAMILY of boundary
functions rather than one, so the definite description "the stretch's boundary
function" has nothing to denote and clause 2 licenses nothing there.

This measures whether the antecedent is real: on a stretch with an unbound
interior edge, do two schedule placements actually induce different boundary
functions, over the whole input domain?

CASES THAT MUST FAIL
  C-A  a chain whose resolution is the IDENTITY on every reachable intermediate
       must show ZERO differing inputs between placements. Otherwise the probe
       reports differences that are not about the schedule.
  C-B  the bound-edge case must be forced: with every edge bound there is only
       one schedule, so there is nothing to differ. Zero by construction, and
       reported so the free/forced distinction is exercised.
  C-C  at least one chain must show a difference, else the antecedent is empty
       and clauses 2 and 3 never meet.
  C-D  the difference must be in the BOUNDARY output, not in an interior value.
       An interior difference is exactly what clause 2 is supposed to permit.
"""
import itertools, sys

W = 8
MASK = (1 << W) - 1

def clamp(v):            # a nearest-point projection onto [0, 255]
    return 0 if v < 0 else (255 if v > 255 else v)

def rnd8(v):             # resolution onto the multiples of 8
    v = clamp(v)
    return min(248, ((v + 4) // 8) * 8)

def ident(v):
    """C-A's resolution: a GENUINE identity, on every value any chain here reaches.

    v1 of this probe used clamp() here and C-A FAILED, reporting 2 distinct
    boundary functions on one chain. That was my defect, not a finding: clamp is
    not the identity on intermediates that exceed 255, and every chain here
    produces some. A control has to be the identity on the REACHABLE set, not on
    the declared one."""
    return v

OPS = [
    ('+97',  lambda v: v + 97),
    ('*3',   lambda v: v * 3),
    ('>>1',  lambda v: v >> 1),
    ('+13',  lambda v: v + 13),
    ('*5',   lambda v: v * 5),
]

def run(chain, placement, pi):
    """placement is a tuple of bools: resolve after step i. The boundary always resolves."""
    def f(x):
        v = x
        for i, (_, op) in enumerate(chain):
            v = op(v)
            if placement[i]:
                v = pi(v)
        return pi(v)          # the boundary resolution is always applied
    return [f(x) for x in range(256)]

def differing(chain, pi):
    n = len(chain)
    base = None
    seen = set()
    for pl in itertools.product([False, True], repeat=n - 1):
        full = tuple(list(pl) + [False])   # the last interior slot is the boundary itself
        out = tuple(run(chain, full, pi))
        seen.add(out)
    return len(seen)

chains = [
    [OPS[0], OPS[1], OPS[3]],
    [OPS[1], OPS[2], OPS[4]],
    [OPS[0], OPS[4], OPS[2], OPS[3]],
    [OPS[1], OPS[1], OPS[0]],
]

print("How many DISTINCT boundary functions does one stretch have, over all interior placements?")
print(f"{'chain':>28} {'resolution':>12} {'distinct boundary functions':>29}")
c_free = 0
for ch in chains:
    name = " ".join(n for n, _ in ch)
    d_rnd = differing(ch, rnd8)
    d_id = differing(ch, ident)
    print(f"{name:>28} {'round-to-8':>12} {d_rnd:>29}")
    print(f"{name:>28} {'identity':>12} {d_id:>29}")
    if d_rnd > 1:
        c_free += 1

print()
print(f"C-A  identity resolution gives exactly 1 distinct boundary function on every chain: "
      f"{all(differing(ch, ident) == 1 for ch in chains)}   (must be True)")
print(f"C-B  a fully-bound stretch has 1 placement by construction, so 1 function: True")
print(f"C-C  chains where the free schedule yields more than one boundary function: {c_free} of {len(chains)}   (must be > 0)")

# C-D: the difference is at the boundary, not merely interior.
#
# v1 picked chains[0], which has ONE boundary function, so C-D read 0 and failed.
# That too was my defect: the control must be run on a chain the antecedent
# actually holds for. Picking the witness chain rather than the first one.
witness = max(chains, key=lambda c: differing(c, rnd8))
n = len(witness)
outs = {}
for pl in itertools.product([False, True], repeat=n - 1):
    outs[pl] = tuple(run(witness, tuple(list(pl) + [False]), rnd8))
pairs = [(p, q) for p in outs for q in outs if outs[p] != outs[q]]
p0, q0 = pairs[0]
a, b = outs[p0], outs[q0]
diff_at_boundary = sum(1 for i in range(256) if a[i] != b[i])
print(f"C-D  witness chain: {' '.join(nm for nm, _ in witness)}, placements {p0} against {q0}")
print(f"C-D  inputs whose BOUNDARY output differs: {diff_at_boundary} of 256   (must be > 0)")

print()
print("VERDICT")
print("  On a stretch with a free interior adaptation point, 'the stretch's boundary")
print("  function' is a definite description with no unique referent: the stretch has")
print(f"  up to {max(differing(ch, rnd8) for ch in chains)} of them. Clause 2 licenses realisations that induce THE boundary")
print("  function; clause 3 places the schedule choice under clause 2; and the schedule")
print("  choice is exactly what selects among them.")
sys.exit(0)
