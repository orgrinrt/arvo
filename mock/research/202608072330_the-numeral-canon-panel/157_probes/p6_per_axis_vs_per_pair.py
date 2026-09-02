#!/usr/bin/env python3
"""
157 P6. Is a per-axis separating witness sufficient, or is the obligation per pair?

CLAIM UNDER TEST
  157 Q157-B, opened in my own file and left unattacked. Per-axis is the cheap form: one
  witness per carried axis, showing the axis is read somewhere. Per-pair is the strong
  form: one witness per pair of distinct parameter assignments, which is injectivity.

  Per-coordinate separation does NOT imply injectivity in general: two assignments
  differing in two axes can agree everywhere if the effects cancel. This probe looks for
  such a cancellation in the model.

NEGATIVE CONTROLS, stated before the run
  F1. The per-axis check must PASS for every axis in the grid; if some axis has no witness
      anywhere the grid is degenerate and the comparison is vacuous.
  F2. The per-pair check must be able to FAIL: a deliberately spurious axis (rounding at
      F = 0 with no literal) must produce unseparated pairs, so a report of zero on the
      honest grid is a result rather than a dead branch.
"""
from fractions import Fraction
from itertools import product, combinations

def value_set(W, F, signed):
    step = Fraction(1, 2 ** F)
    lo, hi = (-(2**(W-1)), 2**(W-1)-1) if signed else (0, 2**W - 1)
    return [Fraction(k) * step for k in range(lo, hi + 1)]

def realise(x, W, F, signed, policy, rounding):
    step = Fraction(1, 2 ** F); q = x / step
    if rounding == "trunc":
        k = int(q) if q >= 0 else -int(-q)
    elif rounding == "floor":
        k = q.numerator // q.denominator
    else:
        fl = q.numerator // q.denominator; frac = q - fl
        k = fl + 1 if frac > Fraction(1,2) else (fl if frac < Fraction(1,2) else (fl if fl % 2 == 0 else fl + 1))
    lo, hi = (-(2**(W-1)), 2**(W-1)-1) if signed else (0, 2**W - 1)
    span = hi - lo + 1
    k = ((k - lo) % span + lo) if policy == "wrap" else max(lo, min(hi, k))
    return Fraction(k) * step

AX = ("W", "F", "signed", "policy", "rounding")
OPS = {"add": lambda a,b: a+b, "sub": lambda a,b: a-b, "mul": lambda a,b: a*b}

def den(p, sig, lits):
    V = value_set(p["W"], p["F"], p["signed"])
    out = {"V": tuple(V)}
    for name in sig:
        if name == "literal":
            out[name] = tuple(realise(c, p["W"], p["F"], p["signed"], p["policy"], p["rounding"]) for c in lits)
        else:
            f = OPS[name]
            out[name] = tuple(realise(f(a,b), p["W"], p["F"], p["signed"], p["policy"], p["rounding"])
                              for a in V for b in V)
    return tuple(sorted(out.items()))

def sweep(Ws, Fs, signs, pols, rnds, sig, lits, label):
    grid = [dict(zip(AX, v)) for v in product(Ws, Fs, signs, pols, rnds)]
    dens = {id(p): den(p, sig, lits) for p in grid}

    # per-axis: for each axis, is there SOME base point where moving only that axis separates?
    per_axis = {}
    for ax in AX:
        found = False
        for p in grid:
            for q in grid:
                if p is q: continue
                if all(p[a] == q[a] for a in AX if a != ax) and p[ax] != q[ax]:
                    if dens[id(p)] != dens[id(q)]:
                        found = True; break
            if found: break
        per_axis[ax] = found

    # per-pair: pairs of DISTINCT assignments that denote the same thing
    same = []
    for p, q in combinations(grid, 2):
        if dens[id(p)] == dens[id(q)]:
            diffs = [a for a in AX if p[a] != q[a]]
            same.append((p, q, diffs))

    multi = [s for s in same if len(s[2]) >= 2]
    print(f"--- {label}")
    print(f"    grid={len(grid)}  signature={sig}  literals={[str(l) for l in lits]}")
    print(f"    per-axis witness found for: "
          + ", ".join(f"{a}={'yes' if per_axis[a] else 'NO'}" for a in AX))
    print(f"    distinct assignments that denote the same thing : {len(same)}")
    print(f"      of those, differing in >= 2 axes             : {len(multi)}")
    for p, q, d in multi[:3]:
        print(f"        {[p[a] for a in AX]}  ==  {[q[a] for a in AX]}   differ in {d}")
    for p, q, d in [s for s in same if len(s[2]) == 1][:2]:
        print(f"        single-axis collapse: differ in {d} at "
              f"W={p['W']} F={p['F']} signed={p['signed']} policy={p['policy']}")
    print()
    return per_axis, same, multi

print("HONEST GRID: only axes the map reads, rounding held at one mode.")
pa1, s1, m1 = sweep([2,3,4], [0,1], [False,True], ["wrap","sat"], ["trunc"],
                    ("add","sub","mul"), [], "rounding fixed")
print("F1 (every carried axis has a witness) :",
      "PASS" if all(pa1[a] for a in ("W","F","signed","policy")) else "FAIL")
print("Q157-B verdict on this grid           :",
      "per-axis SUFFICIENT here (no multi-axis cancellation)" if not m1
      else f"per-axis INSUFFICIENT: {len(m1)} cancelling pairs")
print()

print("CONTROL F2: carry a spurious axis as well.")
pa2, s2, m2 = sweep([2,3,4], [0,1], [False,True], ["wrap","sat"], ["trunc","near","floor"],
                    ("add","sub","mul"), [], "rounding carried, no literal")
print("F2 (the per-pair check can fail)      :",
      "PASS" if s2 else "FAIL (nothing collapsed, so a zero above proves nothing)")
print(f"    of the {len(s2)} collapses, {len([x for x in s2 if len(x[2])>=2])} differ in >= 2 axes")
print()

print("WIDER GRID: three rounding modes AND a separating literal, so no axis is spurious.")
pa3, s3, m3 = sweep([2,3,4], [0,1], [False,True], ["wrap","sat"], ["trunc","near","floor"],
                    ("add","sub","mul","literal"), [Fraction(3,4), Fraction(-3,4), Fraction(3,2)],
                    "rounding carried, literal present")
print("Q157-B verdict on the wider grid      :",
      "per-axis SUFFICIENT here" if not m3 else f"per-axis INSUFFICIENT: {len(m3)} cancelling pairs")
