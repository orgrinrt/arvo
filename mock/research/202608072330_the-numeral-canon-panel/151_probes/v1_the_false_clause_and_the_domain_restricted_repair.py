#!/usr/bin/env python3
# v1 (151): the clause 146 section 5.5 states, and the repair 149 proposes.
#
# 147 and 149 independently find 146:420-427 false: it claims fusing a multiply-add is free under
# unsigned range policies at every one of six rounding positions, and nearest-half-even is not free
# there. 147 traces the widening to its own sentence at 142:388-389; 149 traces it to the argument
# kind, a congruence covering the reduction relocation while the arm relocates the rounding too.
#
# Two files agreeing is not corroboration when neither read the other, and the candidate is mine, so
# I measure it rather than concede on two reports. I also test 149's repair, which is the part that
# decides whether the fix is one clause or two.
#
# Predictions, stated before running:
#
#   A1. Under unsigned, five of six rounding positions give zero fusion difference at every F, and
#       nearest-half-even does not. If all six are zero my instrument disagrees with two others and
#       that is the finding instead.
#   A2. The nonzero cell reproduces at 12.50% under wrapping at F = 1. An adjacent number is a
#       different measurement and I would report it as one.
#   A3. 149's repair: equivariance restricted to the non-negative domain moves toward-zero and
#       away-from-zero from the non-equivariant side to the equivariant side, giving five against
#       one under unsigned and three against three under signed.
#   A4. The restricted test predicts the measured fusion table at 12 of 12 (mode, signedness) cells
#       under wrapping; the unrestricted test mispredicts exactly 2, both under unsigned.
#   A5. CONTROL, non-vacuity: a mutant fused arm that drops the addend must differ from the stepwise
#       arm everywhere, or the comparison is not comparing.
#   A6. CONTROL, reach: at every F > 0 the shift must be inexact on some triples, or a zero means
#       the rounding never fired.
#   A7. CONTROL, cross-check: my signed wrapping rows must reproduce 142 F142-3, or my rounding is a
#       different rounding and the unsigned rows say nothing about the candidate's claim.
from fractions import Fraction as Fr

W = 6
FS = (0, 1, 2, 3, 4, 5)

def bounds(signed):
    return (-(1 << (W - 1)), (1 << (W - 1)) - 1) if signed else (0, (1 << W) - 1)

def reduce_(v, signed, ov):
    lo, hi = bounds(signed)
    if ov == "wrap":
        m = 1 << W
        v %= m
        return v - m if (signed and v > hi) else v
    return max(lo, min(hi, v))

# --------------------------------------------------------------------------
# Rounding modes, as maps from an exact rational to an integer.
# --------------------------------------------------------------------------
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

def m_half_even(x):
    k = m_floor(x)
    r = x - k
    if r < Fr(1, 2):
        return k
    if r > Fr(1, 2):
        return k + 1
    return k if k % 2 == 0 else k + 1

MODES = {
    "floor": m_floor,
    "ceiling": m_ceil,
    "toward_zero": m_tz,
    "away_from_zero": m_az,
    "half_up": m_half_up,
    "half_even": m_half_even,
}

# integer-only fast paths, checked against the rational forms below
def q(n, d, name):
    """round n/d to an integer, d > 0, exact integer arithmetic"""
    fl, r = divmod(n, d)
    if r == 0:
        return fl
    if name == "floor":
        return fl
    if name == "ceiling":
        return fl + 1
    if name == "toward_zero":
        return fl if n > 0 else fl + 1
    if name == "away_from_zero":
        return fl + 1 if n > 0 else fl
    twice = 2 * r
    if name == "half_up":
        return fl + 1 if twice >= d else fl
    if name == "half_even":
        if twice > d:
            return fl + 1
        if twice < d:
            return fl
        return fl if fl % 2 == 0 else fl + 1
    raise ValueError(name)

# agreement check between the two implementations, so the fast path is not its own model
_bad = [(n, d, nm) for nm in MODES for d in (1, 2, 4, 8)
        for n in range(-40, 41) if q(n, d, nm) != MODES[nm](Fr(n, d))]
print("=" * 96)
print("Instrument check: the integer fast path against the rational definitions")
print("=" * 96)
print(f"  disagreements over 6 modes x 4 denominators x 81 numerators: {len(_bad)} (must be 0)")
if _bad:
    print(f"    first: {_bad[:3]}")

def fusion_diff(signed, ov, mode, F, mutant=False):
    """Count triples where the fused and stepwise multiply-add disagree, plus reach."""
    lo, hi = bounds(signed)
    S = 1 << F
    diff = 0
    total = 0
    inexact = 0
    for a in range(lo, hi + 1):
        ab_base = a
        for b in range(lo, hi + 1):
            p = ab_base * b
            step_t = reduce_(q(p, S, mode), signed, ov)
            for c in range(lo, hi + 1):
                total += 1
                n = p + c * S
                if n % S:
                    inexact += 1
                fused_n = p if mutant else n
                fused = reduce_(q(fused_n, S, mode), signed, ov)
                step = reduce_(step_t + c, signed, ov)
                if fused != step:
                    diff += 1
    return diff, total, inexact

print()
print("=" * 96)
print("A1/A2. The unsigned half over six rounding positions, exhaustive at W = 6")
print("=" * 96)
tables = {}
for ov in ("wrap", "sat"):
    print(f"\n  unsigned, {ov}")
    print(f"  {'mode':<18}" + "".join(f"{'F=' + str(f):>10}" for f in FS))
    for mode in MODES:
        row = []
        for F in FS:
            d, t, _ = fusion_diff(False, ov, mode, F)
            row.append(100.0 * d / t)
        tables[(False, ov, mode)] = row
        print(f"  {mode:<18}" + "".join(f"{v:>9.2f}%" for v in row))

nonzero_modes = sorted({m for (s, o, m), r in tables.items()
                        if s is False and any(v > 0 for v in r)})
a1 = nonzero_modes == ["half_even"]
print(f"\n  A1 (exactly one unsigned mode is nonzero): "
      f"{'CONFIRMED' if a1 else 'REFUTED'}, nonzero modes = {nonzero_modes}")
a2 = abs(tables[(False, 'wrap', 'half_even')][1] - 12.50) < 0.005
print(f"  A2 (unsigned wrapping half_even at F=1 is 12.50%): "
      f"{'CONFIRMED' if a2 else 'REFUTED'}, measured "
      f"{tables[(False, 'wrap', 'half_even')][1]:.2f}%")

print()
print("=" * 96)
print("A7. CONTROL: the signed wrapping rows must reproduce 142 F142-3")
print("=" * 96)
print(f"  {'mode':<18}" + "".join(f"{'F=' + str(f):>10}" for f in FS))
for mode in MODES:
    row = [100.0 * fusion_diff(True, "wrap", mode, F)[0] / fusion_diff(True, "wrap", mode, F)[1]
           for F in FS]
    tables[(True, "wrap", mode)] = row
    print(f"  {mode:<18}" + "".join(f"{v:>9.2f}%" for v in row))
tz = tables[(True, "wrap", "toward_zero")]
he = tables[(True, "wrap", "half_even")]
a7 = (all(abs(a - b) < 0.005 for a, b in zip(tz[1:], [1.64, 5.54, 12.34, 22.22, 33.40]))
      and all(abs(a - b) < 0.005 for a, b in zip(he[1:], [12.50, 12.50, 9.38, 6.25, 3.91])))
print(f"\n  A7: {'CONFIRMED' if a7 else 'REFUTED'}, toward_zero and half_even both match 142's table")

print()
print("=" * 96)
print("A5/A6. The remaining controls")
print("=" * 96)
md, mt, _ = fusion_diff(False, "wrap", "floor", 2, mutant=True)
print(f"  A5 mutant arm dropping the addend differs at {md} of {mt} "
      f"({'CONFIRMED' if md > 0 else 'REFUTED'})")
reach_ok = True
for F in FS[1:]:
    _, _, inex = fusion_diff(False, "wrap", "floor", F)
    reach_ok &= inex > 0
    print(f"  A6 F={F}: inexact shifts present at {inex} triples")
print(f"  A6: {'CONFIRMED' if reach_ok else 'REFUTED'}")

print()
print("=" * 96)
print("A3/A4. 149's repair: equivariance on the domain the cell reaches")
print("=" * 96)

def equivariant(name, restricted):
    """rnd(x + c) == rnd(x) + c for integer c, over a swept window.

    restricted: only non-negative x and only shifts keeping x + c non-negative, which is the
    domain an unsigned cell reaches.
    """
    for d in (2, 4, 8, 16):
        for n in range(0 if restricted else -60, 61):
            x = Fr(n, d)
            for c in range(0 if restricted else -6, 7):
                if restricted and n + c * d < 0:
                    continue
                if q(n + c * d, d, name) != q(n, d, name) + c:
                    return False
    return True

print(f"  {'mode':<18}{'unrestricted':<16}{'non-negative domain':<22}")
eq_un, eq_re = {}, {}
for mode in MODES:
    eq_un[mode] = equivariant(mode, False)
    eq_re[mode] = equivariant(mode, True)
    print(f"  {mode:<18}{str(eq_un[mode]):<16}{str(eq_re[mode]):<22}")
a3 = (sum(eq_re.values()) == 5 and sum(eq_un.values()) == 3
      and eq_re["toward_zero"] and eq_re["away_from_zero"])
print(f"\n  A3 (restricted gives five, unrestricted gives three): "
      f"{'CONFIRMED' if a3 else 'REFUTED'}")

print()
print(f"  {'mode':<18}{'signedness':<12}{'free (measured)':<18}{'restricted':<14}{'unrestricted':<14}")
mis_re = mis_un = 0
for mode in MODES:
    for signed in (False, True):
        free = all(v == 0.0 for v in tables[(signed, "wrap", mode)])
        pred_re = eq_re[mode] if not signed else eq_un[mode]
        pred_un = eq_un[mode]
        mis_re += (pred_re != free)
        mis_un += (pred_un != free)
        mark = ""
        if pred_un != free:
            mark = "  <- unrestricted mispredicts"
        print(f"  {mode:<18}{'signed' if signed else 'unsigned':<12}{str(free):<18}"
              f"{str(pred_re):<14}{str(pred_un):<14}{mark}")
print(f"\n  restricted mispredictions: {mis_re} of 12")
print(f"  unrestricted mispredictions: {mis_un} of 12")
print(f"  A4: {'CONFIRMED' if (mis_re == 0 and mis_un == 2) else 'REFUTED'}")

print()
print("=" * 96)
print("VERDICT")
print("=" * 96)
print(f"  146 section 5.5's first predicate block is FALSE: {a1 and a2}")
print(f"  149's domain-restricted repair predicts the table where the unrestricted one does not:")
print(f"    {mis_re} against {mis_un} mispredictions of 12")
print()
print("  The repair is one clause rather than two, and it is not merely narrower: it explains why")
print("  toward-zero and away-from-zero are free under unsigned, which the enumerated form has to")
print("  record as a coincidence. Signedness stops being a case split and becomes what determines")
print("  the domain the rounding is read on.")
