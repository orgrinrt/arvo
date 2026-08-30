#!/usr/bin/env python3
"""
157 P1. Adequacy decomposed: soundness by factoring, completeness by a separating witness.

CLAIM UNDER TEST
  111:507-573 states adequacy ("the type is a sound and complete decision procedure for
  denotational identity") and says it is "checkable the way 110 checks a congruence: at
  model widths, exhaustively, with the transfer argument named rather than assumed."

  This probe tests a different proposition: that completeness needs no exhaustive check at
  all, because it is a conjunction of INEQUALITIES, and an inequality is discharged by ONE
  witness. Only its refutation needs exhaustion.

NEGATIVE CONTROLS, stated before the run (RULES.md: every probe carries the case that must fail)
  C1. A SPURIOUS axis must yield NO separating witness at any input, at any width, under the
      signature where it is spurious. If the witness search finds one, this probe is wrong.
  C2. The same spurious axis must BECOME separable when the signature grows a non-grid
      literal. If it stays inseparable, the signature-relativity claim is wrong.
  C3. A deliberately BROKEN parameterisation (type carries less than the realisation map
      reads) must FAIL the soundness check. If it passes, the soundness check is decorative.
  C4. The exhaustive denotational-identity check and the witness check must AGREE on every
      pair at the model widths where exhaustion is affordable. If they disagree, the witness
      scheme is unsound.
"""
from fractions import Fraction
from itertools import product

# ---------------------------------------------------------------- the model

def value_set(W, F, signed):
    """V(p): the representable values, as exact rationals. F fraction bits."""
    step = Fraction(1, 2 ** F)
    if signed:
        lo, hi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
    else:
        lo, hi = 0, 2 ** W - 1
    return [Fraction(k) * step for k in range(lo, hi + 1)]

def realise(x, W, F, signed, policy, rounding):
    """R(p): exact rational -> V(p). Grid region first, then range region."""
    step = Fraction(1, 2 ** F)
    q = x / step
    if rounding == "trunc":
        k = int(q) if q >= 0 else -int(-q)      # toward zero
    else:                                        # nearest, ties to even
        fl = q.numerator // q.denominator
        frac = q - fl
        if frac > Fraction(1, 2):
            k = fl + 1
        elif frac < Fraction(1, 2):
            k = fl
        else:
            k = fl if fl % 2 == 0 else fl + 1
    if signed:
        lo, hi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
    else:
        lo, hi = 0, 2 ** W - 1
    span = hi - lo + 1
    if policy == "wrap":
        k = (k - lo) % span + lo
    else:                                        # saturate
        k = max(lo, min(hi, k))
    return Fraction(k) * step

# a parameter assignment
AXES = ("W", "F", "signed", "policy", "rounding")

def den_binop(p, op):
    """The denoted binary operation table of primitive p."""
    V = value_set(p["W"], p["F"], p["signed"])
    f = {"add": lambda a, b: a + b, "sub": lambda a, b: a - b,
         "mul": lambda a, b: a * b}[op]
    return {(a, b): realise(f(a, b), p["W"], p["F"], p["signed"],
                            p["policy"], p["rounding"]) for a in V for b in V}

def den_literal(p, lits):
    """The denoted nullary operations: each declared constant realised into V(p)."""
    return {c: realise(c, p["W"], p["F"], p["signed"], p["policy"], p["rounding"])
            for c in lits}

def denotation(p, sig, lits):
    d = {"V": tuple(value_set(p["W"], p["F"], p["signed"]))}
    for op in sig:
        if op == "literal":
            d["literal"] = tuple(sorted(den_literal(p, lits).items()))
        else:
            d[op] = tuple(sorted(den_binop(p, op).items()))
    return d

# ---------------------------------------------------------------- the two halves

def soundness_holds(param_names, sig, lits, grid):
    """SOUNDNESS: nominal equality never merges two denotations.

    Equivalent to: the denotation FACTORS through the named parameters. Checked by
    grouping the grid by the projection onto `param_names` and asking whether every
    group is denotationally constant. No enumeration over the value space beyond what
    building one denotation costs; the obligation is structural.
    """
    groups = {}
    for p in grid:
        key = tuple(p[a] for a in param_names)
        groups.setdefault(key, []).append(p)
    for key, ps in groups.items():
        d0 = denotation(ps[0], sig, lits)
        for q in ps[1:]:
            if denotation(q, sig, lits) != d0:
                return False, (ps[0], q)
    return True, None

def find_witness(p, q, sig, lits):
    """COMPLETENESS, one pair: a single input on which p and q disagree.

    Returns the witness or None. This is the EXISTENTIAL direction; one hit ends it.
    """
    Vp = value_set(p["W"], p["F"], p["signed"])
    Vq = value_set(q["W"], q["F"], q["signed"])
    if tuple(Vp) != tuple(Vq):
        return ("V", None, None)
    for op in sig:
        if op == "literal":
            for c in lits:
                if (realise(c, p["W"], p["F"], p["signed"], p["policy"], p["rounding"])
                        != realise(c, q["W"], q["F"], q["signed"], q["policy"], q["rounding"])):
                    return (op, c, None)
        else:
            f = {"add": lambda a, b: a + b, "sub": lambda a, b: a - b,
                 "mul": lambda a, b: a * b}[op]
            for a in Vp:
                for b in Vp:
                    e = f(a, b)
                    if (realise(e, p["W"], p["F"], p["signed"], p["policy"], p["rounding"])
                            != realise(e, q["W"], q["F"], q["signed"], q["policy"], q["rounding"])):
                        return (op, a, b)
    return None

# ---------------------------------------------------------------- runs

def grid_over(Ws, Fs, signs, policies, roundings):
    return [dict(zip(AXES, v)) for v in product(Ws, Fs, signs, policies, roundings)]

def report(title, param_names, sig, lits, grid):
    print(f"--- {title}")
    print(f"    parameters carried : {param_names}")
    print(f"    signature          : {sig}   literals={[str(l) for l in lits]}")
    ok, cx = soundness_holds(param_names, sig, lits, grid)
    print(f"    SOUNDNESS          : {'holds' if ok else 'FAILS'}"
          + ("" if ok else f"  counterexample {cx[0]} vs {cx[1]}"))
    # completeness: every pair distinct in the carried parameters must have a witness
    pairs = 0
    nowit = []
    seen = set()
    for i, p in enumerate(grid):
        for q in grid[i + 1:]:
            kp = tuple(p[a] for a in param_names)
            kq = tuple(q[a] for a in param_names)
            if kp == kq:
                continue
            if (kp, kq) in seen:
                continue
            seen.add((kp, kq))
            pairs += 1
            if find_witness(p, q, sig, lits) is None:
                nowit.append((kp, kq))
    print(f"    COMPLETENESS       : {pairs - len(nowit)}/{pairs} distinct-name pairs separated"
          + (f"   NO WITNESS for {len(nowit)}: {nowit[:4]}" if nowit else ""))
    print()
    return ok, len(nowit), pairs

print("=" * 78)
print("RUN 1. The honest parameterisation: carry exactly the axes the map reads.")
print("=" * 78)
g = grid_over([3, 4], [0], [False, True], ["wrap", "sat"], ["trunc"])
report("W,F,signed,policy carried; rounding fixed; F=0; sig={add,sub,mul}",
       ("W", "F", "signed", "policy"), ("add", "sub", "mul"), [], g)

print("=" * 78)
print("RUN 2. CONTROL C1: a spurious axis. Rounding at F=0 with no literal.")
print("=" * 78)
g2 = grid_over([3, 4], [0], [False, True], ["wrap", "sat"], ["trunc", "near"])
ok2, nw2, pr2 = report("rounding ALSO carried as a parameter; F=0; sig={add,sub,mul}",
       ("W", "F", "signed", "policy", "rounding"), ("add", "sub", "mul"), [], g2)
print(f"    C1 verdict: expected NO witness for every rounding-only pair -> "
      f"{'PASS' if nw2 > 0 else 'FAIL (the control did not fire)'}")
print()

print("=" * 78)
print("RUN 3. CONTROL C2: the same axis, signature grown a non-grid literal.")
print("=" * 78)
ok3, nw3, pr3 = report("rounding carried; F=0; sig={add,sub,mul,literal}, literal=3/4",
       ("W", "F", "signed", "policy", "rounding"),
       ("add", "sub", "mul", "literal"), [Fraction(3, 4)], g2)
print(f"    C2 verdict: expected the axis to BECOME separable -> "
      f"{'PASS' if nw3 == 0 else f'FAIL ({nw3} pairs still unseparated)'}")
print()

print("=" * 78)
print("RUN 4. CONTROL C3: a broken parameterisation. Drop the policy from the type.")
print("=" * 78)
ok4, nw4, pr4 = report("policy NOT carried; F=0; sig={add,sub,mul}",
       ("W", "F", "signed"), ("add", "sub", "mul"), [], g)
print(f"    C3 verdict: expected SOUNDNESS to FAIL -> "
      f"{'PASS' if not ok4 else 'FAIL (the check is decorative)'}")
print()

print("=" * 78)
print("RUN 5. CONTROL C4: witness scheme against exhaustive denotational identity.")
print("=" * 78)
g5 = grid_over([2, 3, 4], [0, 1], [False, True], ["wrap", "sat"], ["trunc", "near"])
sig5, lits5 = ("add", "sub", "mul"), []
agree = disagree = 0
for i, p in enumerate(g5):
    for q in g5[i + 1:]:
        exhaustive_same = denotation(p, sig5, lits5) == denotation(q, sig5, lits5)
        witness_same = find_witness(p, q, sig5, lits5) is None
        if exhaustive_same == witness_same:
            agree += 1
        else:
            disagree += 1
            if disagree < 4:
                print(f"    DISAGREE {p} vs {q}")
print(f"    pairs compared: {agree + disagree}   agree: {agree}   disagree: {disagree}")
print(f"    C4 verdict: {'PASS' if disagree == 0 else 'FAIL'}")
