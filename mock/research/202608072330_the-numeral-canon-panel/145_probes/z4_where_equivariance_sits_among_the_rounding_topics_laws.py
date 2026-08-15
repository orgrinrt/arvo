#!/usr/bin/env python3
# z4 (145): hole four. What the equivariance predicate does to the rounding topic, which is closed.
#
# 142 F142-2 partitions the rounding axis by translation equivariance: floor, ceiling and
# nearest-half-up carry it; toward-zero, away-from-zero and nearest-half-even do not. 142 section 4
# says this reaches the rounding topic and that naming the modes is "necessary and not sufficient",
# because the property an arm's predicate reads is not recoverable from a mode's name.
#
# The rounding topic is closed with its own candidate. Its statement of what the axis selects
# (132 section 5.3, replaced at 136 section 7.2) enumerates four exact laws: an exact one-sided
# order bound carried by the two adjoints, exact composition across precisions carried by the
# directed members, negation symmetry, and the additive law in expectation. It records one
# exclusivity: no member carries the order bound and negation symmetry at once, because negation
# exchanges the two adjoints.
#
# The question this probe answers, and it decides what 145 may say about a closed topic without
# editing it: is translation equivariance a NEW property, or is it a Boolean function of the laws
# that candidate already enumerates?
#
# Predictions, stated before running:
#
#   D1. The order bound IMPLIES equivariance. An exact one-sided bound forces an adjoint of the
#       grid inclusion, and both adjoints commute with integer translation.
#   D2. Negation symmetry EXCLUDES equivariance, and the argument is two lines rather than a
#       measurement: if both held, then rnd(-1/2) = -rnd(1/2) by symmetry and rnd(-1/2) =
#       rnd(1/2) - 1 by equivariance, so rnd(1/2) = 1/2, which is not an integer. So this is a
#       second exclusivity of exactly the shape the rounding topic already records, and it should
#       be reported as joining that structure rather than as an independent axis property.
#   D3. Among modes carrying NEITHER the order bound nor negation symmetry, equivariance is not
#       determined. Predicted witness: nearest-half-up carries neither and IS equivariant, so a
#       mode carrying neither and NOT equivariant settles independence. I expect to have to
#       construct one, and I mark it as constructed rather than natural.
#       *** THE FIRST CONSTRUCTED WITNESS FAILED AND IS KEPT BELOW. *** `half_floorpair` broke
#       ties on the parity of `k // 2`, which I expected to be sign-blind. It is not: it came out
#       negation symmetric, so it landed in the same signature group as toward_zero and settled
#       nothing. The replacement `mod3_bump` is not a tie rule at all, it perturbs floor on a
#       residue class, which is what actually breaks both symmetry and equivariance.
#       *** AND `mod3_bump` DID NOT SPLIT ANYTHING EITHER, FOR A SECOND REASON WORTH KEEPING. ***
#       It broke both properties as intended but landed alone in signature (F, T, F), so it had
#       nobody to disagree with. A witness for independence has to MATCH an existing mode's law
#       signature and differ on equivariance, which is a stronger requirement than merely lacking
#       the laws. The third attempt perturbs half_up, which already sits at (F, F, F) with
#       equivariance, so the perturbed mode lands in an occupied group.
#   D4. CONTROL: the law columns must not all agree, or the independence question is vacuous.
#   D5. CONTROL: the equivariance checker must return both answers over the mode set, or it is
#       measuring nothing.
from fractions import Fraction as Fr

# ---------------------------------------------------------------------------
# Rounding modes as maps from an exact rational to an integer.
# ---------------------------------------------------------------------------

def m_floor(x):
    return x.numerator // x.denominator

def m_ceil(x):
    return -((-x).numerator // (-x).denominator)

def m_tz(x):
    return m_floor(x) if x >= 0 else m_ceil(x)

def m_az(x):
    return m_ceil(x) if x >= 0 else m_floor(x)

def m_half_up(x):
    return m_floor(x + Fr(1, 2))

def m_half_down(x):
    return m_ceil(x - Fr(1, 2))

def _tie(x):
    return (x - m_floor(x)) == Fr(1, 2)

def m_half_even(x):
    k = m_floor(x)
    r = x - k
    if r < Fr(1, 2):
        return k
    if r > Fr(1, 2):
        return k + 1
    return k if k % 2 == 0 else k + 1

def m_half_odd(x):
    k = m_floor(x)
    r = x - k
    if r < Fr(1, 2):
        return k
    if r > Fr(1, 2):
        return k + 1
    return k if k % 2 != 0 else k + 1

def m_half_tz(x):
    k = m_floor(x)
    r = x - k
    if r < Fr(1, 2):
        return k
    if r > Fr(1, 2):
        return k + 1
    return m_tz(x + Fr(1, 2)) if x < 0 else k

def m_half_az(x):
    k = m_floor(x)
    r = x - k
    if r < Fr(1, 2):
        return k
    if r > Fr(1, 2):
        return k + 1
    return k if x < 0 else k + 1

def m_constructed_failed(x):
    """CONSTRUCTED AND FAILED, kept on the record. Ties by the parity of `k // 2`, which I
    expected to carry no negation symmetry. It does carry it, so it settled nothing."""
    k = m_floor(x)
    r = x - k
    if r < Fr(1, 2):
        return k
    if r > Fr(1, 2):
        return k + 1
    return k if (k // 2) % 2 == 0 else k + 1

def m_constructed(x):
    """CONSTRUCTED, not a mode anyone ships. Floor, bumped up on one residue class of the floor.
    Perturbing floor on a residue class rather than perturbing a tie rule is what breaks both
    negation symmetry and translation equivariance while leaving the order bound absent. It
    exists to settle D3 and nothing else."""
    k = m_floor(x)
    return k + 1 if k % 3 == 0 else k

def m_constructed_third(x):
    """CONSTRUCTED, third attempt, and the one built to land in an OCCUPIED signature group.
    half_up, bumped on one residue class of the floor. half_up already carries none of the three
    laws and is equivariant, so if this keeps the signature and loses equivariance it splits that
    group and settles D3."""
    k = m_floor(x)
    v = m_half_up(x)
    return v + 1 if k % 3 == 0 else v

MODES = {
    "floor": m_floor,
    "ceiling": m_ceil,
    "toward_zero": m_tz,
    "away_from_zero": m_az,
    "half_up": m_half_up,
    "half_down": m_half_down,
    "half_even": m_half_even,
    "half_odd": m_half_odd,
    "half_toward_zero": m_half_tz,
    "half_away_from_zero": m_half_az,
    "CONSTRUCTED half_floorpair (failed)": m_constructed_failed,
    "CONSTRUCTED mod3_bump (alone)": m_constructed,
    "CONSTRUCTED half_up_mod3": m_constructed_third,
}

DENOMS = (1, 2, 3, 4, 8)
NUMS = range(-40, 41)
GRID = [Fr(n, d) for d in DENOMS for n in NUMS]
SHIFTS = range(-6, 7)

# ---------------------------------------------------------------------------
# The properties. The three deterministic laws the rounding candidate enumerates, plus
# translation equivariance.
# ---------------------------------------------------------------------------

def order_bound(f):
    """An exact one-sided bound: f(x) <= x everywhere, or f(x) >= x everywhere."""
    below = all(Fr(f(x)) <= x for x in GRID)
    above = all(Fr(f(x)) >= x for x in GRID)
    return below or above

def staged_composition(f):
    """Narrowing in two stages equals narrowing directly, on a two-step grid chain."""
    for x in GRID:
        direct = f(x)
        mid = Fr(f(x * 4), 4)
        if f(mid) != direct:
            return False
    return True

def negation_symmetry(f):
    return all(f(-x) == -f(x) for x in GRID)

def equivariance(f):
    return all(f(x + c) == f(x) + c for x in GRID for c in SHIFTS)

PROPS = {
    "order bound": order_bound,
    "staged comp": staged_composition,
    "negation sym": negation_symmetry,
    "equivariant": equivariance,
}

print("=" * 100)
print("The table: the rounding candidate's three deterministic laws, plus equivariance")
print("=" * 100)
print(f"  {'mode':<30}{'order bound':<14}{'staged comp':<14}{'negation sym':<15}{'equivariant'}")
T = {}
for name, f in MODES.items():
    row = tuple(PROPS[p](f) for p in PROPS)
    T[name] = row
    print(f"  {name:<30}{str(row[0]):<14}{str(row[1]):<14}{str(row[2]):<15}{row[3]}")

print()
print("=" * 100)
print("D1. Does the order bound imply equivariance?")
print("=" * 100)
viol = [n for n, r in T.items() if r[0] and not r[3]]
print(f"  modes with the order bound and without equivariance: {viol} (must be empty)")
print(f"  D1: {'CONFIRMED' if not viol else 'REFUTED'}")

print()
print("=" * 100)
print("D2. Are negation symmetry and equivariance mutually exclusive?")
print("=" * 100)
both = [n for n, r in T.items() if r[2] and r[3]]
print(f"  modes carrying both: {both} (must be empty)")
print("  and the argument, which is two lines and does not depend on the sweep:")
print("    if rnd is negation symmetric and translation equivariant, then")
print("      rnd(-1/2) = -rnd(1/2)          by symmetry")
print("      rnd(-1/2) = rnd(1/2) - 1       by equivariance at c = -1")
print("    so 2*rnd(1/2) = 1 and rnd(1/2) = 1/2, which is not an integer.")
print(f"  D2: {'CONFIRMED' if not both else 'REFUTED'}")
print("  This is a SECOND exclusivity of exactly the shape 132 section 5.3 already records for")
print("  the order bound against negation symmetry, so equivariance joins that structure.")

print()
print("=" * 100)
print("D3. Among modes carrying neither the order bound nor negation symmetry,")
print("    is equivariance determined by the other laws?")
print("=" * 100)
groups = {}
for n, r in T.items():
    groups.setdefault(r[:3], []).append((n, r[3]))
print(f"  {'law signature (order, comp, negation)':<44}{'modes and their equivariance'}")
split = []
for sig, members in sorted(groups.items(), key=lambda kv: str(kv[0])):
    eq = {e for _, e in members}
    mark = "  <- SPLIT" if len(eq) > 1 else ""
    print(f"  {str(sig):<44}{', '.join(f'{n}={e}' for n, e in members)}{mark}")
    if len(eq) > 1:
        split.append((sig, members))
print(f"\n  law signatures whose members disagree on equivariance: {len(split)}")
print(f"  D3: {'CONFIRMED, equivariance is not a function of the three laws' if split else 'NOT ESTABLISHED on this mode set'}")
if split:
    natural = [(s, m) for s, m in split
               if not all(n.startswith("CONSTRUCTED") for n, _ in m if not _)]
    print(f"  and the witness is {'partly natural' if natural else 'CONSTRUCTED ONLY'}, which is")
    print(f"  stated because a constructed witness settles logical independence and says nothing")
    print(f"  about any mode a design would ship.")

print()
print("=" * 100)
print("D3b. And on the NATURAL modes alone, is the coincidence perfect?")
print("=" * 100)
nat = {n: r for n, r in T.items() if not n.startswith("CONSTRUCTED")}
coincide = all(r[3] == (not r[2]) for r in nat.values())
print(f"  modes considered: {len(nat)} natural")
print(f"  equivariance coincides with the ABSENCE of negation symmetry on all of them: {coincide}")
print(f"  and the constructed witness above shows the coincidence is not a theorem.")
print("  So a design reading the law table alone gets the right answer on every mode it would")
print("  plausibly ship and has no argument that it will keep getting it. That is the precise")
print("  content of 142's 'naming the modes is necessary and not sufficient'.")

print()
print("=" * 100)
print("D4/D5. The controls")
print("=" * 100)
cols = list(zip(*T.values()))
d4 = all(len(set(c)) == 2 for c in cols)
print(f"  every property column takes both values over the mode set: {d4} (must be True)")
for p, c in zip(PROPS, cols):
    print(f"    {p:<16}{sum(c)} true of {len(c)}")
print(f"  D4/D5: {'CONFIRMED' if d4 else 'REFUTED'}")

print()
print("=" * 100)
print("VERDICT, and what 145 may say about a closed topic without editing it")
print("=" * 100)
print("  Translation equivariance is not an independent fifth property sitting beside the")
print("  rounding candidate's four laws. It is bound to two of them:")
print()
print("    the order bound IMPLIES it, so both adjoints carry it;")
print("    negation symmetry EXCLUDES it, by a two-line argument of the same shape as the")
print("      exclusivity 132 section 5.3 already records;")
if split:
    print("    and among modes carrying neither it is UNDETERMINED, witnessed by")
    for sig, members in split:
        print(f"      signature {sig}: " + ", ".join(f"{n}={e}" for n, e in members))
else:
    print("    and whether it is determined among the rest is NOT ESTABLISHED here: every law")
    print("      signature in the mode set above agrees on equivariance, so on this evidence")
    print("      equivariance could be a function of the three laws and I have not separated it.")
print()
print("  So the rounding topic's statement is not wrong and is not complete for this use. It")
print("  enumerates which member carries which law; an arm relocating a rounding across an integer")
print("  addition reads a property that the enumeration determines in two of four cases and leaves")
print("  open in the rest. That is a gap in what the closed candidate happens to cover, reported")
print("  here rather than edited into it, and it is the strategy topic's finding rather than the")
print("  rounding topic's because the strategy topic is where an arm's predicate is being written.")
