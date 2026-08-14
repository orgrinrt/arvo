#!/usr/bin/env python3
"""p5. What happens when the argmin ranges over arms that do not agree?

p1 establishes that every committed family's arm set is answer-equivalent: the arms
compute one value and the measurement compares their cost. The converged mechanism
of this unit, argmin of a weighting over a cost table, is exactly right there.

Op's intents are not there. I5 trades accuracy for speed, I7 buys accuracy with
speed, I3 asks for a particular answer. Each ranges over arms that DISAGREE.

So this probe asks what the mechanism does on the region it has never been run on,
and it turns out the answer is decided by a distinction the unit has not drawn:
whether a coordinate is MEASURED or COMPUTED.

  measured : `algo_ns`. Has a noise floor. `100` section 7.1 measures that floor
             between two byte-identical arms at a median of 0.273%, and section 7
             finds a fresh run moves the argmin in five of six configurations, up
             to 93.8%.
  computed : bits per element (declared, exact: `101` section 2.3). And, it turns
             out, error against an exact reference, which p3 computes with
             `fractions.Fraction` and which is reproducible to the bit.

Over answer-equivalent arms, a flip caused by noise costs a little speed and
nothing else. `100` prices exactly that: 0.045% to 3.83%.

Over answer-differing arms the same flip changes WHAT THE PROGRAM COMPUTES.

Three parts:

  1. an independent instance of the flip, bootstrapping the committed carrier
     samples, so the mechanism is not being taken on `100`'s word;
  2. the counterfactual, stated as one: p3's real error table for a real arm
     family, with a time coordinate whose noise is calibrated to the corpus's own
     measured floor, and a count of how often the SELECTED ARM'S ERROR moves;
  3. the same table ranked by computed coordinates only, which is stable by
     construction.

Part 2 is a counterfactual and is labelled that everywhere it appears. Its error
column is real and computed exactly; its time column is synthetic, because no
committed family has arms that disagree, which is the whole finding.

This is a spike.

Run:  python3 p5_a_measured_coordinate_over_answer_differing_arms.py
"""

import csv
import pathlib
import random
import statistics
import sys

HERE = pathlib.Path(__file__).resolve().parent
BENCHES = HERE.parents[2] / "benches"

RESAMPLES = 2000
random.seed(20260814)

# ---------------------------------------------------------------------------
# 1. The flip, on committed data, independently
# ---------------------------------------------------------------------------

print("1. THE ARGMIN FLIPS UNDER THE CORPUS'S OWN NOISE")
print()

files = sorted(BENCHES.glob("bitpack-carrier-width_n*.csv"))
if not files:
    sys.exit("no bitpack-carrier-width CSVs found")

# per file (region), per variant, the algo_ns samples
regions = []
for f in files:
    per_arm = {}
    with f.open(newline="") as fh:
        for row in csv.DictReader(fh):
            v = row.get("variant")
            try:
                t = float(row["algo_ns"])
            except (KeyError, TypeError, ValueError):
                continue
            if v:
                per_arm.setdefault(v, []).append(t)
    if per_arm:
        regions.append((f.name, per_arm))

print(f"regions (files) : {len(regions)}")
arms = sorted(set().union(*(set(p) for _, p in regions)))
print(f"arms            : {len(arms)}  {arms}")
print(f"samples per arm per region (first region): "
      f"{ {a: len(v) for a, v in sorted(regions[0][1].items())} }")
print()

flip_counts = []
for name, per_arm in regions:
    winners = {}
    for _ in range(RESAMPLES):
        best, best_t = None, None
        for a, samples in per_arm.items():
            boot = [samples[random.randrange(len(samples))] for _ in range(len(samples))]
            t = statistics.median(boot)
            if best_t is None or t < best_t:
                best, best_t = a, t
        winners[best] = winners.get(best, 0) + 1
    flip_counts.append((name, winners))
    top = max(winners.items(), key=lambda kv: kv[1])
    print(f"  {name:<44} distinct winners {len(winners):>2}   modal {top[0]} at {top[1]}/{RESAMPLES}")

n_unstable = sum(1 for _, w in flip_counts if len(w) > 1)
print()
print(f"regions whose pure-time argmin is NOT unique across {RESAMPLES} resamples:"
      f" {n_unstable} of {len(regions)}")
print()
print("That is `100`'s mechanism reproduced from a third implementation on the same")
print("committed samples. Over these arms it is harmless: they all compute the same")
print("value, so a flip buys or loses a little time and nothing else.")

# ---------------------------------------------------------------------------
# 2. The counterfactual: the same noise over arms that disagree
# ---------------------------------------------------------------------------

print()
print("2. COUNTERFACTUAL: THE SAME NOISE OVER ARMS THAT DISAGREE")
print()
print("   The error column is REAL, computed exactly by p3 at k = 16 against a")
print("   rational reference. The time column is SYNTHETIC, because no committed")
print("   family has arms that disagree. That absence is the finding, not a gap in")
print("   this probe. The noise magnitude is the corpus's own measured floor.")
print()

# p3's k = 16 family: (switch depth, mean |error| in declared ulp, rne steps).
# Copied from p3_the_crossing_is_an_arm_family.out, which computes them exactly.
K16 = [
    (0, 0.48948, 17),
    (1, 0.48948, 16),
    (2, 0.53599, 15),
    (3, 0.64924, 14),
    (4, 0.82263, 13),
    (5, 1.02983, 12),
    (6, 1.25932, 11),
    (7, 1.50680, 10),
    (8, 1.75680, 9),
    (9, 2.00131, 8),
    (10, 2.25228, 7),
    (11, 2.50326, 6),
    (12, 2.75131, 5),
    (13, 2.99642, 4),
    (14, 3.24740, 3),
    (15, 3.49838, 2),
    (16, 3.75033, 1),
    (17, 4.00521, 0),
]

# The corpus's own measured noise floor between two byte-identical arms, from
# `100` section 7.1: a median apparent gap of 0.273%.
NOISE = 0.00273

# A synthetic time cost proportional to the rounding work, so the arms are on a
# genuine trade rather than one dominating. Nominal units.
BASE = 100.0
PER_RNE = 0.30


def time_of(rne, noisy):
    t = BASE + PER_RNE * rne
    return t * (1.0 + random.gauss(0.0, NOISE)) if noisy else t


print(f"   noise sigma = {NOISE:.5f} (relative), the corpus's measured floor")
print(f"   arms = {len(K16)}, k = 16")
print()
print(f"   {'exchange rate':>14}  {'distinct arms':>14}  {'distinct errors':>16}  "
      f"{'error spread':>13}")

for rate in (0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0):
    picked = {}
    for _ in range(RESAMPLES):
        best, best_c = None, None
        for d, err, rne in K16:
            c = err + rate * (time_of(rne, True) - BASE)
            if best_c is None or c < best_c:
                best, best_c = (d, err), c
        picked[best] = picked.get(best, 0) + 1
    errs = sorted({e for _, e in picked})
    spread = (max(errs) - min(errs)) if errs else 0.0
    print(f"   {rate:>14.3f}  {len(picked):>14}  {len(errs):>16}  {spread:>13.5f}")

print()
print("   Read the last column as declared ulp. Where it is non-zero, a rerun of the")
print("   bench changes the numeric answer the shipped program produces, at the same")
print("   source, the same weighting and the same inputs.")

# ---------------------------------------------------------------------------
# 3. The same table, computed coordinates only
# ---------------------------------------------------------------------------

print()
print("3. THE SAME TABLE RANKED BY COMPUTED COORDINATES ONLY")
print()
print("   Both coordinates exact: error against a rational reference, and the")
print("   rne-step count, which is a declared static property of the arm exactly as")
print("   bits per element is in `97`'s model.")
print()
print(f"   {'exchange rate':>14}  {'distinct arms':>14}  {'distinct errors':>16}  "
      f"{'error spread':>13}")

for rate in (0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0):
    picked = {}
    for _ in range(RESAMPLES):
        best, best_c = None, None
        for d, err, rne in K16:
            c = err + rate * PER_RNE * rne  # no noise: nothing is measured
            if best_c is None or c < best_c:
                best, best_c = (d, err), c
        picked[best] = picked.get(best, 0) + 1
    errs = sorted({e for _, e in picked})
    spread = (max(errs) - min(errs)) if errs else 0.0
    print(f"   {rate:>14.3f}  {len(picked):>14}  {len(errs):>16}  {spread:>13.5f}")

print()
print("WHAT THIS ESTABLISHES")
print()
print("ONE. The unit's coordinates are two kinds and nothing has said so. A MEASURED")
print("coordinate carries a noise floor the corpus itself reports. A COMPUTED one")
print("does not: bits per element is declared and exact, and so is error against a")
print("rational reference. `101` noticed the first half of this and read it as a")
print("caution about applying a tolerance band uniformly. It is more than that.")
print()
print("TWO. Over answer-equivalent arms the distinction is a nuisance. `100` prices")
print("the whole cost of the flips at 0.045% to 3.83% of speed. Over answer-differing")
print("arms the same flip is a change in the value the program computes, and part 2")
print("shows it happening at six of the seven exchange rates swept. The seventh is")
print("not a reprieve: at that rate the weighting is close enough to pure error that")
print("the two lowest-error arms tie, and they tie because their errors are equal to")
print("five figures rather than because the noise stopped mattering.")
print()
print("THREE. So there is a predicate on the mechanism the unit converged on, and it")
print("is not a small one:")
print()
print("     A weighting may include a MEASURED coordinate only where every arm it")
print("     ranges over computes the same answer. Where the arms disagree, every")
print("     coordinate in the weighting must be computed or declared, because")
print("     otherwise the program's output is a function of a benchmark's noise.")
print()
print("FOUR. And that lands on `100` section 6.1's Arm C, the band. A band accepts a")
print("committed entry that is not the argmin. Over answer-equivalent arms that costs")
print("a little speed, which is what `100` measures. Over answer-differing arms it")
print("accepts a different ANSWER than the strategy names, which is not a cost to")
print("trade. Arm C carries a predicate its statement does not: `holds where the arms")
print("at the region compute the same value`.")
print()
print("FIVE. And the way out is not a restriction, it is a licence. Accuracy is")
print("exactly computable, which p3 does and which the harness's own `score_output`")
print("hook is shaped for. So the coordinate op's accuracy intent needs is a COMPUTED")
print("one by nature, and it lands on the same side as bits per element rather than")
print("on the timing side. The mechanism serves I7 and I5 once that is said out loud.")
