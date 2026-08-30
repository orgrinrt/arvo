#!/usr/bin/env python3
# P3: commutation of rounding with the overflow policies (T6, T7).
# wrap: expected to commute with floor/ceil/half_up/half_even, and to FAIL with toward_zero
#   (the control that must fail). Wrap on an exact value translates by the span into the
#   representative window; wrap on a grid value is the usual modular reduction. Both are exact.
# saturate: expected to commute with all five deterministic modes; an artificial boundary-violating
#   mode (maps off-grid values just under the max ABOVE the range) is the control that must fail.
from fractions import Fraction

def q_floor(x): return x.numerator // x.denominator
def q_ceil(x): return -((-x).numerator // (-x).denominator)
def q_tz(x): return q_floor(x) if x >= 0 else q_ceil(x)
def q_half_up(x): return q_floor(x + Fraction(1, 2))
def q_half_even(x):
    k = q_floor(x); r = x - k
    if r < Fraction(1, 2): return k
    if r > Fraction(1, 2): return k + 1
    return k if k % 2 == 0 else k + 1

MODES = [("floor", q_floor), ("ceil", q_ceil), ("toward_zero", q_tz),
         ("half_up", q_half_up), ("half_even", q_half_even)]

E = 4  # subquanta per quantum = 2^E; exact points are u/2^E quanta, u integer
def pts(lo, hi):  # exact points in [lo, hi] quanta at subquantum resolution
    return [Fraction(u, 1 << E) for u in range(lo * (1 << E), hi * (1 << E) + 1)]

# --- wrap ---
for W in (3, 4, 5):
    span = 1 << W
    for signed in (True, False):
        lo_r, hi_r = (-(span // 2), span // 2 - 1) if signed else (0, span - 1)
        def wrap_exact(x):  # translate into [lo_r, lo_r + span)
            return x - span * ((x - lo_r) // span)
        def wrap_grid(k):
            return (k - lo_r) % span + lo_r
        window = pts(lo_r - 2 * span, hi_r + 2 * span)  # several spans both sides
        tag = "signed" if signed else "unsigned"
        for name, f in MODES:
            mism = sum(1 for x in window if wrap_grid(f(x)) != f(wrap_exact(x)))
            expect = "control: must be > 0" if name == "toward_zero" and signed else \
                     ("must be > 0 (negative exacts exist)" if name == "toward_zero" else "must be 0")
            # note: even the unsigned window includes negative exact values (lo_r - 2*span < 0),
            # so toward_zero is expected to fail there as well; the derivation says the failure
            # needs negative off-grid exacts, not a signed FORMAT.
            print(f"wrap W={W} {tag} x {name}: {mism} mismatches of {len(window)} ({expect})")

# --- saturate ---
W = 4
for signed in (True, False):
    m, M = (-(1 << (W - 1)), (1 << (W - 1)) - 1) if signed else (0, (1 << W) - 1)
    def sat_exact(x): return Fraction(max(m, min(M, x)))
    def sat_grid(k): return max(m, min(M, k))
    window = pts(m - 8, M + 8)
    tag = "signed" if signed else "unsigned"
    def boundary_broken(x):  # control: violates monotonicity at the upper boundary
        if M - 1 < x < M: return M + 1
        return q_floor(x)
    for name, f in MODES + [("boundary_broken(CONTROL)", boundary_broken)]:
        mism = sum(1 for x in window if sat_grid(f(x)) != f(sat_exact(x)))
        expect = "control: must be > 0" if "CONTROL" in name else "must be 0"
        print(f"saturate W={W} {tag} x {name}: {mism} mismatches of {len(window)} ({expect})")

# --- follow-up after run 1 refuted the representative-level prediction for ceil/half_up/half_even:
# test the corrected hypothesis. Wrap on exacts is a PIECEWISE translation, so the right statement
# is commutation in the quotient group Z/spanZ. Expect: floor/ceil/half_up/half_even agree mod span
# everywhere (0 quotient mismatches); toward_zero disagrees mod span (>0, the control); and at the
# representative level floor alone is exact, because floor never rounds upward out of the half-open
# representative window.
print()
print("--- quotient-level (mod span) commutation, the corrected hypothesis ---")
for W in (3, 4, 5):
    span = 1 << W
    for signed in (True, False):
        lo_r = -(span // 2) if signed else 0
        def wrap_exact(x): return x - span * ((x - lo_r) // span)
        def wrap_grid(k): return (k - lo_r) % span + lo_r
        window = pts(lo_r - 2 * span, lo_r + 3 * span)
        tag = "signed" if signed else "unsigned"
        for name, f in MODES:
            qm = sum(1 for x in window if (wrap_grid(f(x)) - f(wrap_exact(x))) % span != 0)
            rm = sum(1 for x in window if wrap_grid(f(x)) != f(wrap_exact(x)))
            expect = "control: must be > 0" if name == "toward_zero" else "must be 0"
            print(f"wrap-mod-span W={W} {tag} x {name}: {qm} quotient mismatches ({expect}); "
                  f"{rm} representative mismatches")
