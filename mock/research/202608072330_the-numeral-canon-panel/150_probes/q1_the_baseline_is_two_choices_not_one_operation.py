"""q1: settling the conflict `145` found inside my own file.

NOT A BENCHMARK. Exact rational arithmetic over synthetic cost tables. It times
nothing and prices nothing.

`145` section 3.5 and `146` section 6.2 report that `144` sections 6 and 4.3
conflict, reproduce my 24.6% independently at 894 of 3640 cases, and offer a
dissolution: read the per-coordinate division as a declaration of the weighting's
units rather than as a transformation of the cost table. I am the author of both
sections and the coordinator asks which I meant.

**Both readings of the conflict are one dimension short.** The per-coordinate
form has two independent choices inside it and neither file names them
separately:

  UNITS. Is a weighting declared in absolute cost units, or in units of a named
  baseline arm? This is what my section 6 varied, on one target.

  BASELINE PROVENANCE. Is the baseline arm's cost vector fixed once, or
  re-measured on each target? This is what my section 4.3 varied, and it varied
  it while holding units at baseline-relative without naming that as a choice.

If those are orthogonal there is no conflict to dissolve: my two sections sit at
two different points of a two-by-two neither of them names, and the 24.6% and the
18.0% are measurements of two different axes.

PREDICTIONS, before running:
  AA1 under baseline-relative units on the simplex, the baseline arm's own figure
      is identically the weight sum, hence exactly 1. So the reporting
      normalisation `140`'s obligation asks for is already applied and there is
      no second operation to place. Derivation, checked rather than discovered.
  AA2 therefore the selector's ranking and the reported-figure ranking are the
      same ranking under relative units, at every case, because they are the same
      number.
  AA3 the cross-target invariance requires BOTH relative units AND a per-target
      baseline. Relative units with a baseline fixed once do NOT give it.
  AA4 the two-by-two is exactly: invariance under a pure per-coordinate rescale
      in one cell of four, and in no other.

CONTROLS, each must fire or the cell beside it is void:
  BB1 THE CASE THAT MUST FAIL. Under absolute units with the scalar reporting
      normalisation, the reported ranking must equal the raw ranking at every
      case. That is `140`'s theorem and my F144-13's 0.0%; a non-zero here means
      the reporting code is wrong and every other cell is suspect.
  BB2 THE SECOND CASE THAT MUST FAIL. Under relative units with a per-target
      baseline, an ARBITRARY target change must still move the selection. If it
      does not, the cell is not a change of units, it is a claim that targets do
      not matter. This reproduces `145` z5 E6 on my own instrument.
  BB3 THE THIRD CASE THAT MUST FAIL. Under relative units with a FIXED baseline,
      a pure rescale must move the selection. If it does not, AA3 is measuring
      nothing and the invariance was never about provenance.
  BB4 the identical-target control: every cell must report zero movement when the
      two targets are the same table.
  BB5 THE FOURTH CASE THAT MUST FAIL, and it caught a defect in my first run. Under
      ABSOLUTE units the baseline never enters the selection, so the two absolute
      rows must be IDENTICAL, digit for digit. My first version drew fresh tables
      per cell and reported 12.0% against 19.3%, which invites a reader to think
      provenance moves something it cannot touch. The draws are now shared across
      all four cells, making this a factorial rather than four samples, and the
      two absolute rows agreeing exactly is what says the sharing worked.
"""

from fractions import Fraction as F
import random

D = 3
RES = 12


def simplex(res, d=D):
    def rec(k, left):
        if k == 1:
            yield (F(left, res),)
            return
        for i in range(left + 1):
            for rest in rec(k - 1, left - i):
                yield (F(i, res),) + rest
    return [w for w in rec(d, res) if any(x > 0 for x in w)]


W = simplex(RES)


def wsum(c, w):
    return sum(w[k] * c[k] for k in range(D))


def argmin_of(vals):
    best, bi = None, None
    for i, v in enumerate(vals):
        if best is None or v < best:
            best, bi = v, i
    return bi


def table(rng, arms=7, hi=60):
    return [tuple(F(rng.randint(1, hi)) for _ in range(D)) for _ in range(arms)]


def relative(costs, b):
    return [tuple(c[k] / b[k] for k in range(D)) for c in costs]


def select(costs, w, units, b):
    """units in {absolute, relative}. b is the baseline cost vector in force."""
    if units == "absolute":
        return argmin_of([wsum(c, w) for c in costs])
    return argmin_of([wsum(c, w) for c in relative(costs, b)])


def reported(costs, w, units, b):
    """The figure each arm publishes as its cost claim against the named arm."""
    if units == "absolute":
        denom = wsum(b, w)
        return [wsum(c, w) / denom for c in costs]
    return [wsum(c, w) for c in relative(costs, b)]


fail = []
rng = random.Random(20260815 + 101)

print("=" * 78)
print("AA1/AA2. under relative units, is there a second operation left to place?")
print("=" * 78)
bad_one = bad_rank = cases = 0
for _ in range(60):
    C = table(rng)
    bi = rng.randrange(len(C))
    b = C[bi]
    for w in W:
        cases += 1
        figs = reported(C, w, "relative", b)
        if figs[bi] != sum(w):
            bad_one += 1
        sel = select(C, w, "relative", b)
        if argmin_of(figs) != sel:
            bad_rank += 1
print(f"  {cases} (arm set, weight) cases")
print(f"  AA1 the baseline's own reported figure differs from the weight sum: {bad_one} "
      f"-> {'CONFIRMED' if bad_one == 0 else 'REFUTED'}")
print(f"      on the simplex the weight sum is 1, so every arm's figure is already stated")
print(f"      against the named arm and the reporting normalisation is division by one.")
print(f"  AA2 the reported ranking differs from the selection: {bad_rank} "
      f"-> {'CONFIRMED' if bad_rank == 0 else 'REFUTED'}")
if bad_one:
    fail.append("AA1")
if bad_rank:
    fail.append("AA2")

print()
print("=" * 78)
print("BB1. the case that must fail: absolute units with the scalar reporting form")
print("=" * 78)
moved = tot = 0
for _ in range(60):
    C = table(rng)
    b = C[rng.randrange(len(C))]
    for w in W:
        if wsum(b, w) == 0:
            continue
        tot += 1
        if argmin_of(reported(C, w, "absolute", b)) != select(C, w, "absolute", b):
            moved += 1
print(f"  {tot} cases, reported ranking differs from raw ranking: {moved} "
      f"-> {'PASS' if moved == 0 else 'FAIL, the reporting code is wrong'}")
if moved:
    fail.append("BB1")

print()
print("=" * 78)
print("AA3/AA4/BB2/BB3/BB4. the two-by-two")
print("=" * 78)
print(f"  {'units':<10}{'baseline':<14}{'pure rescale':>14}{'arbitrary':>12}{'identical':>12}")

# one shared population, so the four cells are a factorial rather than four samples
DRAWS = []
for _ in range(40):
    C = table(rng)
    bi = rng.randrange(len(C))
    r = tuple(F(rng.randint(1, 8)) for _ in range(D))
    DRAWS.append((C, bi, r, [tuple(r[k] * c[k] for k in range(D)) for c in C], table(rng)))

grid = {}
for units in ("absolute", "relative"):
    for prov in ("fixed", "per-target"):
        resc = arb = ident = n_resc = n_arb = n_id = 0
        for C, bi, r, C_res, C_arb in DRAWS:
            b_fixed = C[bi]
            for w in W:
                b1 = C[bi]
                b2 = C_res[bi] if prov == "per-target" else b_fixed
                b3 = C_arb[bi] if prov == "per-target" else b_fixed
                if any(x == 0 for x in b1 + b2 + b3):
                    continue
                n_resc += 1
                n_arb += 1
                n_id += 1
                p0 = select(C, w, units, b1)
                if select(C_res, w, units, b2) != p0:
                    resc += 1
                if select(C_arb, w, units, b3) != p0:
                    arb += 1
                if select(C, w, units, b1) != p0:
                    ident += 1
        grid[(units, prov)] = (resc / n_resc, arb / n_arb, ident)
        print(f"  {units:<10}{prov:<14}{100.0 * resc / n_resc:>13.1f}%"
              f"{100.0 * arb / n_arb:>11.1f}%{ident:>12}")
        if ident:
            fail.append(f"BB4 {units}/{prov}")

inv = [k for k, v in grid.items() if v[0] == 0.0]
print()
print(f"  AA3/AA4 cells with exact invariance under a pure per-coordinate rescale: {inv}")
aa3 = inv == [("relative", "per-target")]
print(f"  AA3 the invariance needs BOTH relative units and a per-target baseline: "
      f"{'CONFIRMED' if aa3 else 'REFUTED'}")
if not aa3:
    fail.append("AA3")
bb2 = grid[("relative", "per-target")][1] > 0
print(f"  BB2 relative units with a per-target baseline still move under an arbitrary target "
      f"change: {'PASS' if bb2 else 'FAIL, it is not a change of units'}")
if not bb2:
    fail.append("BB2")
bb5 = grid[("absolute", "fixed")] == grid[("absolute", "per-target")]
print(f"  BB5 the two absolute rows are identical, as they must be since the baseline never")
print(f"      enters an absolute-units selection: {'PASS' if bb5 else 'FAIL, the draws are not shared'}")
if not bb5:
    fail.append("BB5")
bb3 = grid[("relative", "fixed")][0] > 0
print(f"  BB3 relative units with a FIXED baseline move under a pure rescale: "
      f"{'PASS' if bb3 else 'FAIL, AA3 measures nothing'}")
if not bb3:
    fail.append("BB3")

print()
print("=" * 78)
print("what this settles, in the author's own words")
print("=" * 78)
print("  My section 6 varied UNITS on one target and measured 24.6%.")
print("  My section 4.3 varied BASELINE PROVENANCE across two targets, and it did so while")
print("  holding units at baseline-relative, without naming either as a choice.")
print("  The two sections sit at two points of the table above and measure two different")
print("  axes. There is no operation that is required in one and forbidden in the other.")
print()
print("  What IS wrong in my file is one clause. Section 6's repair reads 'a per-coordinate")
print("  normalisation applied before the weighting is a change of weighting, and the arm a")
print("  consumer asked for is not the arm they get.' The second half presumes the consumer")
print("  asked in absolute units, which is exactly the undeclared choice. Under relative units")
print("  the per-coordinate figure IS the arm they asked for, and AA1 shows the reporting")
print("  normalisation is then division by one, so the two operations coincide rather than")
print("  competing.")
print()
print("  So the design owes two declarations rather than one: which units a weighting is in,")
print("  and whether the named baseline is fixed once or re-measured per target. Only the")
print("  conjunction buys portability, and the second declaration is what decides whether a")
print("  cost claim compares across targets, because a per-target baseline makes '0.8 of")
print("  baseline' two different absolute costs on two machines.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
