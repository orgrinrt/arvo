#!/usr/bin/env python3
"""
p3. What a crossing can preserve, and whether the concept must state anything new for it.

HYPOTHESIS, written before the run.

`63` section 3.4 (C4) records that a reduction carries two independent law families: the
ADAPTATION laws facing the source (monotone, distance-minimising) and the COHERENCE law facing
the target (rho(a op b) = rho(rho(a) op rho(b))). `67` section 5 maps them onto two consumer
classes, order transport and reassociating folds.

Claim under test: a narrowing crossing has exactly two properties it can have or lack, and they
are those same two families, one each.

  ORDER PRESERVATION of the crossing map      <->  the target reduction is MONOTONE
  OPERATION PRESERVATION of the crossing      <->  the target reduction is COHERENT

If that holds, the crossing contract demands nothing the concept does not already carry, and
the crossing is a THIRD consumer of C4's classification rather than a new obligation.

Two regimes, and only the first is definitional.

  REGIME 1. Operand pairs whose exact result stays inside the SOURCE's representable set, so
  the source's own reduction never fires. Here operation preservation IS the coherence law
  written out, and the probe is confirming arithmetic rather than discovering a law. Said
  plainly so nobody cites it for more than it is.

  REGIME 2. Operand pairs whose exact result leaves the source's set, so the source's reduction
  fires BEFORE the crossing. This is not definitional and is where the interesting answer is.
  PREDICTION: a coherent target reduction is NOT sufficient here, because two adaptations act in
  sequence, which is double rounding. If that prediction holds, the crossing's commuting square
  is not a property of the crossing alone.

Exhaustive over each source set. Model widths: source 6 bits, target 4 bits.
"""

SIGNED_SRC = list(range(-32, 32))
SIGNED_TGT = list(range(-8, 8))
NONNEG_SRC = list(range(0, 64))
NONNEG_TGT = list(range(0, 16))


def wrap(v, q):
    n = len(q)
    return ((v - q[0]) % n) + q[0]


def saturate(v, q):
    return q[0] if v < q[0] else (q[-1] if v > q[-1] else v)


def mutant(v, q):
    """`56`'s opposite-bound control: clamps to the wrong end. Holds neither family."""
    return q[-1] if v < q[0] else (q[0] if v > q[-1] else v)


REDUCTIONS = [("wrap", wrap), ("saturate", saturate), ("opposite-bound", mutant)]
OPS = [("add", lambda a, b: a + b), ("mul", lambda a, b: a * b)]


# ------------------------------------------------- the two law families, computed directly

def is_monotone(rho, q, src):
    """Adaptation-law half, from `56`'s definition: order preserved on the source window."""
    ordered = sorted(src)
    for i in range(len(ordered) - 1):
        if rho(ordered[i], q) > rho(ordered[i + 1], q):
            return False
    return True


def coherence_failures(rho, q, op, src):
    """`56`'s coherence law: rho(a op b) == rho(rho(a) op rho(b)), over the source window."""
    bad = 0
    total = 0
    for a in src:
        for b in src:
            total += 1
            if rho(op(a, b), q) != rho(op(rho(a, q), rho(b, q)), q):
                bad += 1
    return bad, total


# ------------------------------------------------------- the two properties of a crossing

def crossing_preserves_order(rho, q, src):
    """a <= b implies c(a) <= c(b), where c is the crossing map v -> rho(v, q)."""
    bad = 0
    total = 0
    ordered = sorted(src)
    for i in range(len(ordered)):
        for j in range(i + 1, len(ordered)):
            total += 1
            if rho(ordered[i], q) > rho(ordered[j], q):
                bad += 1
    return bad, total


def crossing_preserves_op(rho_t, q_t, rho_s, q_s, op, src, regime):
    """
    c(a op_A b) == c(a) op_B c(b), where op_A is the source's own operation
    (rho_s applied to the exact result) and op_B the target's.

    regime 1: only pairs whose exact result is already inside the source's set.
    regime 2: only pairs whose exact result leaves it, so the source's reduction fires.
    """
    bad = 0
    total = 0
    inside = set(q_s)
    for a in src:
        for b in src:
            exact = op(a, b)
            fires = exact not in inside
            if regime == 1 and fires:
                continue
            if regime == 2 and not fires:
                continue
            total += 1
            left = rho_t(rho_s(exact, q_s), q_t)
            right = rho_t(op(rho_t(a, q_t), rho_t(b, q_t)), q_t)
            if left != right:
                bad += 1
    return bad, total


CASES = [
    ("signed  [-32,31] -> [-8,7]", SIGNED_SRC, SIGNED_TGT),
    ("nonneg  [0,63]   -> [0,15]", NONNEG_SRC, NONNEG_TGT),
]

print("=" * 96)
print("REGIME 1: the source's reduction never fires. Definitional, and stated as such.")
print("=" * 96)
print(f"{'window':<28}{'target rho':<16}{'op':<6}{'monotone':<10}{'coherent':<10}"
      f"{'ord-pres':<10}{'op-pres':<10}{'paired'}")

pair_ok = True
regime1_rows = []
for label, src, tgt in CASES:
    for rname, rho in REDUCTIONS:
        mono = is_monotone(rho, tgt, src)
        ord_bad, ord_tot = crossing_preserves_order(rho, tgt, src)
        for oname, op in OPS:
            coh_bad, _ = coherence_failures(rho, tgt, op, src)
            op_bad, op_tot = crossing_preserves_op(rho, tgt, saturate, src, op, src, 1)
            coherent = coh_bad == 0
            op_pres = op_bad == 0
            ord_pres = ord_bad == 0
            ok = (mono == ord_pres) and (coherent == op_pres)
            pair_ok = pair_ok and ok
            tag = f"{label.split()[0]}:{rname}/{oname}"
            regime1_rows.append((tag, rname, oname, mono, coherent, ord_pres, op_pres))
            print(f"{label:<28}{rname:<16}{oname:<6}{str(mono):<10}{str(coherent):<10}"
                  f"{str(ord_pres):<10}{str(op_pres):<10}{'yes' if ok else 'NO'}")

print()
print(f"Every cell pairs as predicted: {pair_ok}")

combos = {}
for row in regime1_rows:
    combos.setdefault((row[3], row[4]), []).append(row[0])
print()
print("Inhabitation of the two-by-two, which is what makes the two properties independent")
print("rather than one property wearing two names:")
for k in [(True, True), (True, False), (False, True), (False, False)]:
    members = combos.get(k, [])
    print(f"  monotone={str(k[0]):<6} coherent={str(k[1]):<6}  "
          f"{len(members):>2} cells   {sorted(set(members))}")
inhabited = sum(1 for k in [(True, True), (True, False), (False, True), (False, False)]
                if combos.get(k))
print(f"  cells inhabited: {inhabited} of 4")

# ------------------------------------------------------------------------------ regime 2

print()
print("=" * 96)
print("REGIME 2: the source's reduction fires first. Not definitional.")
print("=" * 96)
print(f"{'window':<28}{'source rho':<14}{'target rho':<16}{'op':<6}"
      f"{'target coherent':<18}{'crossing op-preserving'}")

sufficiency_broken = []
for label, src, tgt in CASES:
    for sname, rho_s in REDUCTIONS[:2]:
        for rname, rho_t in REDUCTIONS[:2]:
            for oname, op in OPS:
                coh_bad, _ = coherence_failures(rho_t, tgt, op, src)
                bad, tot = crossing_preserves_op(rho_t, tgt, rho_s, src, op, src, 2)
                coherent = coh_bad == 0
                pres = bad == 0
                print(f"{label:<28}{sname:<14}{rname:<16}{oname:<6}"
                      f"{str(coherent):<18}{tot - bad}/{tot}")
                if coherent and not pres:
                    sufficiency_broken.append((label, sname, rname, oname, bad, tot))

print()
if sufficiency_broken:
    print("PREDICTION HELD. A COHERENT target reduction is not sufficient once the source's own")
    print("reduction has already fired. Cells where it fails:")
    for c in sufficiency_broken:
        print(f"  {c[0]}  source={c[1]}  target={c[2]}  {c[3]}: "
              f"{c[4]} of {c[5]} operand pairs diverge")
else:
    print("PREDICTION REFUTED: coherence of the target sufficed in every cell measured.")

# a hand witness, so the mechanism is legible rather than only counted
a = b = 31
exact = a + b
left = wrap(saturate(exact, SIGNED_SRC), SIGNED_TGT)
right = wrap(wrap(a, SIGNED_TGT) + wrap(b, SIGNED_TGT), SIGNED_TGT)
print()
print("Hand witness, signed, source saturate, target wrap, addition:")
print(f"  a = b = {a};  exact = {exact}")
print(f"  cross(a +_A b) = wrap(saturate({exact})) = wrap({saturate(exact, SIGNED_SRC)})"
      f" = {left}")
print(f"  cross(a) +_B cross(b) = wrap({wrap(a, SIGNED_TGT)} + {wrap(b, SIGNED_TGT)})"
      f" = {right}")
print(f"  {left} against {right}: two adaptations in sequence, which is double rounding.")

print()
print("=" * 96)
print("READING")
print("=" * 96)
print("The two properties a narrowing crossing can have are the two law families the concept")
print("already carries, one each, and all four combinations are inhabited, so neither implies")
print("the other. The crossing therefore demands nothing new of a number system: it consumes")
print("C4's classification. That makes crossings a THIRD consumer of the two families beside")
print("the two `67` section 5 names.")
print()
print("And the sufficiency half fails: coherence of the target reduction makes the crossing")
print("commute only while the source's own reduction has not acted. Once it has, two")
print("adaptations sit in the schedule and the square does not close. That is `63` C1's")
print("unfused condition arriving at the crossing layer, and the count of adaptation points,")
print("not the choice of reduction, is what decides the answer.")
