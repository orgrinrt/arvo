#!/usr/bin/env python3
"""162 P1. The candidate's statement says "Clause 4 is conditional on op."
Exactly one clause. Does the container premise reach further?

I do NOT reuse 157's p4 grid. A claim I am making against a candidate that
cites 157 should not rest on 157's instrument, so this builds its own from a
different direction: 157 counted equivalence classes over a configuration grid;
this asks, per statement clause, whether the clause's TRUTH VALUE or its
EXTENSION differs between the two branches, and demonstrates the identity split
with an explicit pair rather than a class count.

NEGATIVE CONTROLS, stated before the run.

  C1. Without a container observation, the two container rules must give the
      SAME identity verdict on every pair. If they differ, my grid separates
      things for some reason other than the container and every verdict below
      is about that other reason instead.
  C2. With a container observation, at least one pair must be separated that
      C1 called identical. If none is, the observation is inert in my model and
      the probe shows nothing.
  C3. A pair differing in the VALUE SET must be separated under BOTH branches.
      If it is not, my identity relation is broken outright, and C1 passing
      would only mean it separates nothing at all.
"""
import itertools, sys

# A configuration: (W, F, signed, policy, container_rule)
# Container rules, as 154 P4b identified them.
def container_bits(W, rule):
    rungs = [8, 16, 32, 64, 128]
    minimum = next(r for r in rungs if r >= W)
    if rule == "minimum":
        return minimum
    i = rungs.index(minimum)
    return rungs[i + 1] if i + 1 < len(rungs) else minimum

def value_set(W, F, signed):
    """The denoted rationals, as (numerator, denominator=2^F) pairs."""
    if signed:
        lo, hi = -(1 << (W - 1)), (1 << (W - 1)) - 1
    else:
        lo, hi = 0, (1 << W) - 1
    return (lo, hi, F, signed)

def realise(k, W, F, signed, policy):
    """R : Z -> V, on the integer numerator."""
    if signed:
        lo, hi = -(1 << (W - 1)), (1 << (W - 1)) - 1
    else:
        lo, hi = 0, (1 << W) - 1
    if policy == "wrap":
        span = 1 << W
        return ((k - lo) % span) + lo
    return lo if k < lo else (hi if k > hi else k)

def denotation(cfg):
    """(V, R) as an observable fingerprint: the value set plus R's whole
    behaviour on a fixed probe range covering both sides of both bounds."""
    W, F, signed, policy, _rule = cfg
    probe = range(-(1 << (W + 1)), (1 << (W + 1)) + 1)
    return (value_set(W, F, signed),
            tuple(realise(k, W, F, signed, policy) for k in probe))

def fingerprint(cfg, observe_container):
    d = denotation(cfg)
    if not observe_container:
        return d
    W, F, signed, policy, rule = cfg
    return d + (container_bits(W, rule),)

WS, FS, SG, PO, RU = [3, 4, 5, 6], [0, 1], [False, True], ["wrap", "sat"], ["minimum", "above"]
GRID = [(w, f, s, p, r) for w in WS for f in FS for s in SG for p in PO for r in RU]

def classes(observe):
    seen = {}
    for c in GRID:
        seen.setdefault(fingerprint(c, observe), []).append(c)
    return seen

def main():
    off, on = classes(False), classes(True)

    # C1: same denotation, different container rule -> identical without the observation
    pairs_same_denot_diff_rule = [
        (a, b) for a, b in itertools.combinations(GRID, 2)
        if a[:4] == b[:4] and a[4] != b[4]
    ]
    c1 = all(fingerprint(a, False) == fingerprint(b, False) for a, b in pairs_same_denot_diff_rule)
    # and they must actually differ in container bits somewhere, or C1 is vacuous
    c1_nonvacuous = any(container_bits(a[0], a[4]) != container_bits(b[0], b[4])
                        for a, b in pairs_same_denot_diff_rule)
    c2 = any(fingerprint(a, True) != fingerprint(b, True) for a, b in pairs_same_denot_diff_rule)
    # C3: a value-set difference must separate under both branches
    va = (3, 0, False, "wrap", "minimum")
    vb = (4, 0, False, "wrap", "minimum")
    c3 = (fingerprint(va, False) != fingerprint(vb, False)
          and fingerprint(va, True) != fingerprint(vb, True))

    print(f"C1 container rule is inert without the observation : {c1}   (want True)")
    print(f"C1b and the rules do differ in container bits      : {c1_nonvacuous}   (want True)")
    print(f"C2 the observation separates at least one pair     : {c2}   (want True)")
    print(f"C3 a value-set difference separates on both branches: {c3}   (want True)")
    if not (c1 and c1_nonvacuous and c2 and c3):
        print("CONTROL FAILED -- result suppressed"); sys.exit(1)
    print()
    print(f"configurations                     : {len(GRID)}")
    print(f"distinct primitives, container internal   : {len(off)}")
    print(f"distinct primitives, container observable : {len(on)}")
    sep = [(a, b) for a, b in pairs_same_denot_diff_rule
           if fingerprint(a, True) != fingerprint(b, True)]
    print(f"pairs with identical (V,R) split by the observation: {len(sep)} of {len(pairs_same_denot_diff_rule)}")
    a, b = sep[0]
    print(f"  worked pair: W={a[0]} F={a[1]} signed={a[2]} {a[3]}")
    print(f"    rule={a[4]:8s} -> container {container_bits(a[0], a[4])} bits")
    print(f"    rule={b[4]:8s} -> container {container_bits(b[0], b[4])} bits")
    print()
    print("REACH INTO THE STATEMENT'S CLAUSES")
    print("  clause 2 ('identity is that structure up to denotation-preserving")
    print(f"    isomorphism'): its EXTENSION moves, {len(off)} primitives against {len(on)}.")
    print("    Conditional. The statement gives it no inline marker.")
    print("  clause 6 ('the realisation is not part of identity'): its TRUTH VALUE")
    print("    moves. Under observable, the carrier is part of the realisation")
    print("    (clause 5 defines the lens as carrier, offset, width) and it is")
    print("    identity-bearing, so clause 6's first sentence is false on that")
    print("    branch. Conditional. The statement gives it no inline marker.")
    print("  clause 4 (saturation): conditional, and the statement SAYS SO.")
    print()
    print("VERDICT: the premise reaches at least three clauses, not one. The")
    print("         preamble promises 'Clauses conditional on op's premise say")
    print("         so inline' and the closing note says 'Clause 4 is")
    print("         conditional on op'. Clauses 2 and 6 are conditional and")
    print("         unmarked.")

if __name__ == "__main__":
    main()
