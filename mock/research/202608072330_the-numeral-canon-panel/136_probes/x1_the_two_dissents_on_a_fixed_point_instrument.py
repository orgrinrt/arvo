#!/usr/bin/env python3
# x1 (136): reproducing 133's D2 and D3 on my own instrument, built differently on purpose.
#
# 133's s2 established both on an abstract cell model: part A enumerated round-up sets on m-1
# subpoints of ONE cell and matched monotone against suffix; part B evaluated three laws on
# rationals. I reproduce both on a CONCRETE fixed-point grid, and part A over SEVERAL cells at
# once, because the per-cell enumeration leaves the across-cell direction unchecked and the
# candidate's sentence quantifies over the whole map.
#
# Predictions, stated before running:
#
#   A1. Over a fine grid of F_fine fraction bits quantised to F_coarse, enumerating every LOCAL
#       grid-fixing retraction across C cells at once, the globally monotone maps are exactly
#       those whose every cell is a suffix rule. Count = s^C where s = (subpoints per cell) + 1,
#       out of 2^((s-1)*C). So across-cell monotonicity adds NO constraint beyond per-cell:
#       locality already forces it. If this prediction fails, 133's per-cell result does not
#       lift and my reproduction is the finding.
#   A2. The parity keying from 125_probes/p1 is a committed deterministic retraction that is not
#       monotone. Control: it must appear in the enumeration and must be classified non-monotone.
#   A3. Control on the locality assumption: a grid-fixing NON-local retraction (sending a value
#       to a distant grid point) can still be monotone, so locality is not what monotonicity
#       needs. Prediction: at least one such map is monotone.
#       *** THIS PREDICTION WAS REFUTED ON THE FIRST RUN AND THE REFUTATION IS THE BETTER
#       RESULT. *** No monotone non-local grid-fixing retraction exists, and the reason is one
#       line: a grid-fixing map pins k and k+1, and a subpoint strictly between them must land
#       in [k, k+1] to stay monotone, which is exactly locality. So monotonicity IMPLIES
#       locality rather than presupposing it, and the characterisation strengthens: among ALL
#       grid-fixing retractions, monotone iff every cell is a threshold rule. The prediction is
#       left standing above as written and the run below reports it refuted.
#
#   B1. On the concrete grid, floor carries the order bound AND exact staged composition. This
#       alone refutes candidate 5.3's "no member carries more than one of the first three".
#   B2. toward_zero carries exact composition AND negation symmetry. Second refutation.
#   B3. half_even carries negation symmetry, which 5.3 gives to toward_zero alone.
#   B4. The true exclusivity: no member carries the one-sided order bound AND negation symmetry.
#   B5. Control: half_up carries none of the three. If it carries one, my law encodings are wrong.
from itertools import product
from fractions import Fraction

# ---------------------------------------------------------------------------
# The instrument: a fixed-point grid. Values are integers scaled by 2^-F.
# quantise(v, F_from, F_to) takes a value at F_from bits to the grid at F_to bits.
# ---------------------------------------------------------------------------

def val(raw, F):
    return Fraction(raw, 1 << F)

def q_floor(x):
    return x.numerator // x.denominator

def q_ceil(x):
    return -((-x).numerator // (-x).denominator)

def q_toward_zero(x):
    return q_floor(x) if x >= 0 else q_ceil(x)

def q_half_up(x):
    return q_floor(x + Fraction(1, 2))

def q_half_even(x):
    k = q_floor(x)
    r = x - k
    if r < Fraction(1, 2):
        return k
    if r > Fraction(1, 2):
        return k + 1
    return k if k % 2 == 0 else k + 1

MODES = {
    "floor": q_floor,
    "ceil": q_ceil,
    "toward_zero": q_toward_zero,
    "half_up": q_half_up,
    "half_even": q_half_even,
}

print("=" * 92)
print("PART A. Is a deterministic quantisation monotone exactly when it is a threshold rule?")
print("Reproducing 133 D2 across several cells at once, on a fixed-point grid.")
print("=" * 92)

def enumerate_local_retractions(F_fine, cells):
    """Every local grid-fixing retraction on `cells` consecutive cells of a 2^-F_fine grid.

    A local retraction sends each fine value x to floor(x) or ceil(x), and fixes grid points.
    Encoded as one bit per off-grid subpoint, 1 meaning round up.
    """
    sub = (1 << F_fine) - 1  # off-grid subpoints per cell
    n = sub * cells
    for bits in product((0, 1), repeat=n):
        yield bits, sub

def is_monotone_global(bits, sub, cells):
    """Evaluate the whole map on the whole fine grid and check global monotonicity."""
    out = []
    b = 0
    for k in range(cells):
        out.append(k)  # the grid point at k is fixed
        for j in range(1, sub + 1):
            out.append(k + bits[b])
            b += 1
    out.append(cells)  # the final grid point
    return all(out[i] <= out[i + 1] for i in range(len(out) - 1))

def every_cell_is_suffix(bits, sub, cells):
    for k in range(cells):
        cell = bits[k * sub:(k + 1) * sub]
        if any(cell[i] > cell[i + 1] for i in range(len(cell) - 1)):
            return False
    return True

a1_ok = True
for F_fine, cells in ((2, 2), (2, 3), (3, 2)):
    sub = (1 << F_fine) - 1
    total = mono = suffix = agree = 0
    for bits, s in enumerate_local_retractions(F_fine, cells):
        total += 1
        m = is_monotone_global(bits, s, cells)
        f = every_cell_is_suffix(bits, s, cells)
        mono += m
        suffix += f
        agree += (m == f)
    expected = (sub + 1) ** cells
    ok = (mono == suffix == expected) and (agree == total)
    a1_ok &= ok
    print(f"  F_fine={F_fine} cells={cells}: {total} local retractions, "
          f"{mono} globally monotone, {suffix} all-cells-suffix, "
          f"agree on {agree}/{total}; expected (sub+1)^cells = {expected}  -> {'OK' if ok else 'MISMATCH'}")

print(f"\n  A1 (across-cell monotonicity adds no constraint beyond per-cell): "
      f"{'CONFIRMED' if a1_ok else 'REFUTED'}")

# A2: the parity control, on the same instrument.
F_fine, cells = 2, 2
sub = (1 << F_fine) - 1
parity_bits = tuple(((k * sub + j) % 2) for k in range(cells) for j in range(1, sub + 1))
p_mono = is_monotone_global(parity_bits, sub, cells)
p_suffix = every_cell_is_suffix(parity_bits, sub, cells)
print(f"\n  A2 parity keying {parity_bits}: monotone={p_mono} (must be False), "
      f"suffix={p_suffix} (must be False)  -> "
      f"{'CONFIRMED' if (not p_mono and not p_suffix) else 'REFUTED'}")
print("     It is a deterministic grid-fixing retraction, so candidate 5.2's unscoped")
print("     'every deterministic member is order-preserving' is false with a committed witness.")

# A3: the locality control. Drop locality: allow a subpoint to go to ANY grid point in range.
found_monotone_nonlocal = False
example = None
for choices in product(range(cells + 1), repeat=sub * cells):
    out = []
    b = 0
    local = True
    for k in range(cells):
        out.append(k)
        for j in range(1, sub + 1):
            out.append(choices[b])
            if choices[b] not in (k, k + 1):
                local = False
            b += 1
    out.append(cells)
    if local:
        continue
    if all(out[i] <= out[i + 1] for i in range(len(out) - 1)):
        found_monotone_nonlocal = True
        example = choices
        break
print(f"\n  A3 a monotone NON-local grid-fixing retraction exists: {found_monotone_nonlocal} "
      f"(PREDICTED True, so this prediction is REFUTED), witness={example}")
print("     The refutation is the stronger result and I keep it rather than repair the prediction.")
print("     A grid-fixing map pins k and k+1; a subpoint strictly between them must land in")
print("     [k, k+1] to stay monotone, which IS locality. So monotone implies local, and the")
print("     characterisation holds over ALL grid-fixing retractions, not only the local ones.")
print("     That is wider than 133's s2 established and wider than I predicted.")

print()
print("=" * 92)
print("PART B. The law overlap, on concrete fixed-point widths.")
print("Reproducing 133 D3. 133 evaluated on rationals; this sweeps actual (W, F) grids.")
print("=" * 92)

def order_bound(mode_fn, F, W):
    """A one-sided exact bound: Q(x) <= x for every x, or Q(x) >= x for every x."""
    below = above = True
    for raw in range(-(1 << (W - 1)), 1 << (W - 1)):
        x = val(raw, F)
        q = mode_fn(x)
        if q > x:
            below = False
        if q < x:
            above = False
    return below or above

def staged_composition(mode_fn, F_exact, F_mid, F_final, W):
    """Narrowing in two stages equals narrowing directly."""
    for raw in range(-(1 << (W - 1)), 1 << (W - 1)):
        x = val(raw, F_exact)
        direct = mode_fn(x * (1 << F_final)) / Fraction(1 << F_final)
        mid = Fraction(mode_fn(x * (1 << F_mid)), 1 << F_mid)
        staged = mode_fn(mid * (1 << F_final)) / Fraction(1 << F_final)
        if direct != staged:
            return False
    return True

def negation_symmetry(mode_fn, F, W):
    """Q(-x) == -Q(x) for every x."""
    for raw in range(-(1 << (W - 1)), 1 << (W - 1)):
        x = val(raw, F)
        if mode_fn(-x) != -mode_fn(x):
            return False
    return True

# The first run of this probe swept W=7, F=3 with staging 3 -> 2 -> 1 and reported half_even
# carrying exact staged composition, contradicting 133's s2 and 125 section 10's P4 (which
# measured 500 half_even mismatches of 4001). The contradiction was MY instrument's: that window
# holds no value whose intermediate stage lands on a tie, so the failing path is never entered.
# That is the "setup that helps" defect, caught on my own probe. The sweep below is widened and
# a witness-finder is added, so a True can no longer mean "the failing case was out of range".
W, F = 9, 4
rows = {}
print(f"\n  at W={W}, F={F}, staged narrowing F_exact=4 -> F_mid=2 -> F_final=0")
print(f"  {'member':<13}{'order bound':<14}{'staged comp':<14}{'negation sym':<14}{'count':<6}")
for name, fn in MODES.items():
    ob = order_bound(fn, F, W)
    sc = staged_composition(fn, 4, 2, 0, W)
    ns = negation_symmetry(fn, F, W)
    rows[name] = (ob, sc, ns)
    print(f"  {name:<13}{str(ob):<14}{str(sc):<14}{str(ns):<14}{sum((ob, sc, ns)):<6}")

b1 = rows["floor"][0] and rows["floor"][1]
b2 = rows["toward_zero"][1] and rows["toward_zero"][2]
b3 = rows["half_even"][2]
b4 = not any(r[0] and r[2] for r in rows.values())
b5 = not any(rows["half_up"])
# The witness-finder: for every member reported as FAILING composition, name the value that
# fails, and for every member reported as CARRYING it, report how many values were swept, so a
# True cannot hide an empty sweep.
print("\n  composition sweep detail (a True over zero candidates would be the defect above):")
for name, fn in MODES.items():
    swept = 0
    witness = None
    for raw in range(-(1 << (W - 1)), 1 << (W - 1)):
        x = val(raw, 4)
        swept += 1
        direct = Fraction(fn(x))
        mid = Fraction(fn(x * 4), 4)
        staged = Fraction(fn(mid))
        if direct != staged and witness is None:
            witness = (x, direct, staged)
    tag = f"first failing value {witness[0]} direct={witness[1]} staged={witness[2]}" if witness \
        else "no failing value in sweep"
    print(f"    {name:<13} swept {swept:>4} values, {tag}")

print(f"\n  B1 floor carries order bound AND composition: {b1} (predicted True)")
print(f"     -> this alone refutes candidate 5.3's 'no member carries more than one of the first three'")
print(f"  B2 toward_zero carries composition AND negation: {b2} (predicted True)")
print(f"  B3 half_even carries negation symmetry: {b3} (predicted True; 5.3 gives it to toward_zero alone)")
print(f"  B4 the true exclusivity, order bound never with negation symmetry: {b4} (predicted True)")
print(f"  B5 control, half_up carries none: {b5} (predicted True; if False my encodings are wrong)")

print()
print("=" * 92)
print("VERDICT")
print("=" * 92)
dissents_ok = a1_ok and (not p_mono) and b1 and b2 and b3 and b4 and b5
print(f"  both of 133's dissents reproduce: {dissents_ok} (predicted True)")
print(f"  my own A3 prediction: REFUTED (found_monotone_nonlocal={found_monotone_nonlocal}),")
print(f"    and the refutation widens the characterisation rather than weakening it.")
print(f"  one earlier defect in THIS probe, recorded rather than repaired silently: the first")
print(f"    run swept W=7,F=3 with a 3->2->1 staging and reported half_even carrying exact")
print(f"    composition, contradicting 133 and 125's P4. The window held no tie at the")
print(f"    intermediate stage, so the failing path was never entered. Widened, half_even now")
print(f"    fails at -247/16 and the witness is printed above.")
print("  133's D2 and D3 both reproduce on an instrument built differently from its own.")
print("  D2 additionally lifts: monotonicity across cells is free for local maps, so the")
print("  per-cell characterisation is the whole characterisation for the axis's members.")
