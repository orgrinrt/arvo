#!/usr/bin/env python3
# P1: every deterministic mode is monotone; none is additive off-grid (T1, T1b, T2).
# Exact arithmetic throughout: values are integers u in subquantum units, one quantum = 2^E subunits.
# Negative control (must fail): a parity-broken mode fails monotonicity; the additivity checker
# passes on on-grid pairs (sanity) and each real mode fails it off-grid (the theorem's face).
E = 4
SUB = 1 << E  # subunits per quantum

def floor_m(u): return u >> E  # arithmetic shift: floor for negatives too
def ceil_m(u): return -((-u) >> E)
def toward_zero(u): return u // SUB if u >= 0 else -((-u) // SUB)
def half_up(u): return (u + SUB // 2) >> E
def half_even(u):
    k, r = floor_m(u), u - (floor_m(u) << E)
    if r * 2 < SUB: return k
    if r * 2 > SUB: return k + 1
    return k if k % 2 == 0 else k + 1
def parity_broken(u):  # control: deliberately non-monotone
    k = floor_m(u)
    return k + 1 if (u % 2) else k

MODES = [("floor", floor_m), ("ceil", ceil_m), ("toward_zero", toward_zero),
         ("half_up", half_up), ("half_even", half_even)]

N = 2000  # sweep window: u in [-N, N] subunits = [-125q, +125q], closed under negation
pts = range(-N, N + 1)

print(f"sweep: u in [-{N}, {N}] subunits, E={E} ({SUB} subunits per quantum), window [-{N//SUB}q, +{N//SUB}q]")

# monotonicity
for name, f in MODES + [("parity_broken(CONTROL)", parity_broken)]:
    bad = sum(1 for u in range(-N, N) if f(u) > f(u + 1))
    verdict = "MONOTONE" if bad == 0 else f"NOT MONOTONE ({bad} adjacent inversions)"
    print(f"monotone {name}: {verdict}")

# additivity: Q(x+y) = Q(x)+Q(y), off-grid pairs, negation-closed window and one-signed restriction
step = 33  # coprime-ish stride to sample pairs densely but affordably
pairs = [(x, y) for x in range(-N, N + 1, step) for y in range(-N, N + 1, step)]
for name, f in MODES:
    viol = sum(1 for x, y in pairs if f(x + y) != f(x) + f(y))
    viol_pos = sum(1 for x, y in pairs if x >= 0 and y >= 0 and f(x + y) != f(x) + f(y))
    ongrid = sum(1 for x, y in pairs if x % SUB == 0 and y % SUB == 0 and f(x + y) != f(x) + f(y))
    print(f"additivity {name}: {viol} violations of {len(pairs)} pairs; one-signed subdomain: "
          f"{viol_pos} violations; on-grid pairs: {ongrid} violations (must be 0)")
print("controls: parity mode must be NOT MONOTONE; every mode must have >0 additivity violations")
print("on both the negation-closed window and its one-signed restriction, and 0 on-grid violations.")
