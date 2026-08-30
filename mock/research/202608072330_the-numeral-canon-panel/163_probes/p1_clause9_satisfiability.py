#!/usr/bin/env python3
"""
163 P1. Does clause 9 of the candidate remain SATISFIABLE on both branches of the
container premise?

CLAIM UNDER TEST
  `161` section 4 clause 9 states completeness up to weakening as a per-pair obligation
  over "every pair of distinct shipped instantiations", with three outcomes: a separating
  witness, a weakening in exactly one direction, or refusal.

  `161`'s preamble and its section 6 say the container premise is localised: "Everything
  else in the statement is premise-free" (161:673). Clause 9 carries no conditionality
  marker.

  This probe asks what clause 9 says about a pair of shipped instantiations that differ
  ONLY in carrier, which is what a storage-minimising marker and a headroom marker are
  over one (V, R) if neither changes a computed value.

NEGATIVE CONTROLS, stated before the run
  G1. A pair that genuinely differs in denotation (overflow policy) must come out
      witness=YES. If it does not, the witness search is broken and every verdict below
      is worthless.
  G2. A refinement pair (same primitive, different declared bound) must come out
      witness=NO, directions=1. This is 160 F160-1's middle outcome, reproduced on a
      separately written model; if it comes out otherwise this model does not implement
      the clause being tested.
  G3. Under the footprint-OBSERVABLE branch, the carrier pair must come out witness=YES.
      If the branch flag changes nothing, the probe is not modelling the premise at all.
"""
from fractions import Fraction
from itertools import product

# ------------------------------------------------------------------ the model
# A shipped INSTANTIATION is what the type carries: the primitive's coordinates,
# the carrier chosen for it, and any declared refinement.

def value_set(W):
    return list(range(0, 2 ** W))

def realise(exact, W, policy):
    hi = 2 ** W - 1
    return (exact % (hi + 1)) if policy == "wrap" else max(0, min(hi, exact))

def carrier_bits(W, rule):
    for c in (8, 16, 32, 64, 128):
        if W <= c:
            return c if rule == "minimum" else min(128, (8, 16, 32, 64, 128)[
                min((8, 16, 32, 64, 128).index(c) + 1, 4)])
    raise ValueError

def observations(inst, footprint_observable):
    """The declared operation set's denotations, as a comparable object.
    Under footprint-observable the set contains one observation that reads the carrier."""
    W, policy, rule, bound = inst
    V = value_set(W)
    dom = [v for v in V if v <= bound]          # the declared restriction
    out = {"V": tuple(dom)}
    for name, f in (("add", lambda a, b: a + b), ("mul", lambda a, b: a * b)):
        out[name] = tuple(realise(f(a, b), W, policy) for a in dom for b in dom)
    if footprint_observable:
        out["footprint"] = (carrier_bits(W, rule),)
    return tuple(sorted(out.items()))

def witness(p, q, footprint_observable):
    """One input on which the two denote differently, over the SHARED domain."""
    Wp, polp, rulep, bp = p
    Wq, polq, ruleq, bq = q
    shared = [v for v in value_set(min(Wp, Wq)) if v <= min(bp, bq)]
    if footprint_observable and carrier_bits(Wp, rulep) != carrier_bits(Wq, ruleq):
        return ("footprint", None, None)
    for name, f in (("add", lambda a, b: a + b), ("mul", lambda a, b: a * b)):
        for a in shared:
            for b in shared:
                if realise(f(a, b), Wp, polp) != realise(f(a, b), Wq, polq):
                    return (name, a, b)
    return None

def weakens_to(p, q, footprint_observable):
    """A total denotation-preserving map p -> q: every value p admits is admitted by q,
    and every operation agrees on p's domain."""
    Wp, polp, rulep, bp = p
    Wq, polq, ruleq, bq = q
    domp = [v for v in value_set(Wp) if v <= bp]
    domq = set(v for v in value_set(Wq) if v <= bq)
    if not set(domp) <= domq:
        return False
    if footprint_observable and carrier_bits(Wp, rulep) != carrier_bits(Wq, ruleq):
        return False
    for name, f in (("add", lambda a, b: a + b), ("mul", lambda a, b: a * b)):
        for a in domp:
            for b in domp:
                if realise(f(a, b), Wp, polp) != realise(f(a, b), Wq, polq):
                    return False
    return True

def verdict(p, q, fo):
    w = witness(p, q, fo)
    d = int(weakens_to(p, q, fo)) + int(weakens_to(q, p, fo))
    if w is not None:
        return "SEPARATED (both names stay)", w, d
    if d == 1:
        return "REFINEMENT PAIR (ordered, both names stay)", None, d
    return "REFUSED as a spurious split", None, d

# ------------------------------------------------------------------ the pairs
W = 6
FULL = 2 ** W - 1
PAIRS = {
    "G1 policy pair    (wrap vs sat, one carrier rule)":
        ((W, "wrap", "minimum", FULL), (W, "sat", "minimum", FULL)),
    "G2 refinement pair (one primitive, bound 15 vs 63)":
        ((W, "sat", "minimum", 15), (W, "sat", "minimum", FULL)),
    "THE CARRIER PAIR  (one (V,R), minimum vs headroom rule)":
        ((W, "sat", "minimum", FULL), (W, "sat", "headroom", FULL)),
}

for fo in (False, True):
    branch = "footprint OBSERVABLE" if fo else "footprint INTERNAL"
    print(f"=== branch: {branch} ===")
    for label, (p, q) in PAIRS.items():
        v, w, d = verdict(p, q, fo)
        print(f"  {label:<56} directions={d}  witness={'yes' if w else 'no ':<3}  -> {v}")
    print()

print("CONTROLS")
g1 = verdict(*PAIRS["G1 policy pair    (wrap vs sat, one carrier rule)"], False)
g2 = verdict(*PAIRS["G2 refinement pair (one primitive, bound 15 vs 63)"], False)
g3 = verdict(*PAIRS["THE CARRIER PAIR  (one (V,R), minimum vs headroom rule)"], True)
print(f"  G1 policy pair separated under footprint-internal      : "
      f"{'PASS' if g1[1] is not None else 'FAIL'}")
print(f"  G2 refinement pair is witness=no, directions=1         : "
      f"{'PASS' if (g2[1] is None and g2[2] == 1) else 'FAIL'}")
print(f"  G3 carrier pair separated under footprint-observable   : "
      f"{'PASS' if g3[1] is not None else 'FAIL'}")
print()
ci = verdict(*PAIRS["THE CARRIER PAIR  (one (V,R), minimum vs headroom rule)"], False)
print("READING")
print(f"  Under footprint-internal the carrier pair is: {ci[0]}")
print("  So clause 9's refusal branch fires on two shipped instantiations that differ")
print("  only in which carrier a marker selected. Under footprint-observable it does not.")
print("  Clause 9's SATISFIABILITY therefore reads the container premise, and clause 9")
print("  carries no conditionality marker.")
