#!/usr/bin/env python3
"""
157 P4. The saturation bound is a theorem with a premise, and the premise is a design
decision arvo has not made.

CLAIM UNDER TEST
  111:344-351 (F111-7, and its argument at 111:328-336): "no signature can separate more
  than 'R differs somewhere on Q', and a signature containing a constant injection over Q
  reaches that bound at depth one." Recorded at `W in {2,3,4}` despite the file itself
  saying the bound half "is an argument and not a sweep".

  The argument is: every term's argument to R is a rational. That is true exactly when
  every operation's EXACT semantics is a function of its arguments alone. This probe tests
  what happens when the signature contains an observation that reads something other than
  (V, R) -- specifically the container, which I17's storage-minimising concern makes a
  first-class thing a consumer cares about.

NEGATIVE CONTROLS, stated before the run
  D1. With a p-independent signature, adding EVERY operation to {literal} must split
      nothing. If it splits something, the theorem is false and the probe is wrong.
  D2. With the container observation ADDED, the partition must strictly refine. If it does
      not refine, the container is not observable in this model and the attack fails.
  D3. The container observation must be non-trivial: at least two configurations in the
      grid must share (V, R) and differ in container. If none do, D2 is vacuous.
"""
from fractions import Fraction
from itertools import product

def value_set(W, F, signed):
    step = Fraction(1, 2 ** F)
    lo, hi = (-(2**(W-1)), 2**(W-1)-1) if signed else (0, 2**W - 1)
    return [Fraction(k) * step for k in range(lo, hi + 1)]

def realise(x, W, F, signed, policy):
    step = Fraction(1, 2 ** F)
    q = x / step
    k = int(q) if q >= 0 else -int(-q)
    lo, hi = (-(2**(W-1)), 2**(W-1)-1) if signed else (0, 2**W - 1)
    span = hi - lo + 1
    k = ((k - lo) % span + lo) if policy == "wrap" else max(lo, min(hi, k))
    return Fraction(k) * step

# The container: the treatment picks it. Two treatments, exactly 154 P4b's two rules.
def container_bits(W, treatment):
    if treatment == "minimal":
        for c in (8, 16, 32, 64, 128):
            if W <= c:
                return c
        raise ValueError
    else:  # "headroom": one rung above the minimum, the shipped Warm rule
        for i, c in enumerate((8, 16, 32, 64, 128)):
            if W <= c:
                return (8, 16, 32, 64, 128)[min(i + 1, 4)]
        raise ValueError

LITS = [Fraction(n, d) for d in (1, 2, 3, 4, 5) for n in range(-40, 41)]

def den_R(p):
    """The denotation as (V, R restricted to a dense rational sample). This IS what a
    full literal signature gives, so it is the saturation bound made concrete."""
    V = tuple(value_set(p["W"], p["F"], p["signed"]))
    R = tuple(realise(c, p["W"], p["F"], p["signed"], p["policy"]) for c in LITS)
    return (V, R)

def den_ops(p):
    """Every binary operation's table, plus V. All exact semantics p-independent."""
    V = value_set(p["W"], p["F"], p["signed"])
    out = [tuple(V)]
    for name, f in (("add", lambda a, b: a + b), ("sub", lambda a, b: a - b),
                    ("mul", lambda a, b: a * b)):
        out.append(tuple(realise(f(a, b), p["W"], p["F"], p["signed"], p["policy"])
                         for a in V for b in V))
    return tuple(out)

def den_container(p):
    return (container_bits(p["W"], p["treatment"]),)

AX = ("W", "F", "signed", "policy", "treatment")
grid = [dict(zip(AX, v)) for v in product([3, 4, 5, 6], [0, 1], [False, True],
                                          ["wrap", "sat"], ["minimal", "headroom"])]

def classes(keyfn):
    d = {}
    for p in grid:
        d.setdefault(keyfn(p), []).append(p)
    return d

c_lit = classes(den_R)
c_lit_ops = classes(lambda p: (den_R(p), den_ops(p)))
c_ops = classes(den_ops)
c_lit_cont = classes(lambda p: (den_R(p), den_container(p)))

print(f"grid configurations                              : {len(grid)}")
print(f"classes under {{literal}} alone                    : {len(c_lit)}")
print(f"classes under {{literal, add, sub, mul}}            : {len(c_lit_ops)}")
print(f"classes under {{add, sub, mul}} (no literal)        : {len(c_ops)}")
print(f"classes under {{literal}} + container observation   : {len(c_lit_cont)}")
print()
print(f"D1 (operations add nothing to the literal)   : "
      f"{'PASS' if len(c_lit_ops) == len(c_lit) else 'FAIL'}"
      f"   [{len(c_lit)} -> {len(c_lit_ops)}]")
print(f"D2 (container observation strictly refines)  : "
      f"{'PASS' if len(c_lit_cont) > len(c_lit) else 'FAIL'}"
      f"   [{len(c_lit)} -> {len(c_lit_cont)}]")

split = [(k, len(v)) for k, v in c_lit.items()
         if len({den_container(p) for p in v}) > 1]
print(f"D3 (classes that the container splits)       : "
      f"{'PASS' if split else 'FAIL'}   {len(split)} of {len(c_lit)} classes contain "
      f"configurations differing in container")
print()
ex = None
for k, v in c_lit.items():
    cs = {den_container(p)[0] for p in v}
    if len(cs) > 1:
        ex = (v, cs)
        break
if ex:
    print("worked example of one denotation with two containers:")
    for p in ex[0]:
        print(f"    W={p['W']:>2} F={p['F']} signed={str(p['signed']):>5} "
              f"policy={p['policy']:>4} treatment={p['treatment']:>8} "
              f"-> container {container_bits(p['W'], p['treatment'])} bits")
    print("  identical (V, R). Under the saturation bound these are ONE primitive.")
    print("  Under a design where footprint is observable they are TWO.")
