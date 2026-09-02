#!/usr/bin/env python3
"""p7 part two. Does `102`'s p5 hazard have a REAL instance in this corpus?

`102`'s p5 concludes with a constraint on the mechanism:

    A weighting may include a MEASURED coordinate only where every arm it
    ranges over computes the same answer. Where the arms disagree, every
    coordinate in the weighting must be computed or declared, because
    otherwise the program's output is a function of a benchmark's noise.

Its demonstration of that hazard is explicitly synthetic on the time column,
and its own probe output says why: "The time column is SYNTHETIC, because no
committed family has arms that disagree. That absence is the finding".

p2, p3, p4 and p6 establish that the absence is not real: eight committed
regions have arms that are not required to compute one value, and four of them
have arms measured to differ. So the hazard can be tested against real data,
which is what this probe does. Every number below comes from a committed
artifact or from exact arithmetic:

- **time**: the `algo_ns` samples in the committed CSVs for the two
  answer-differing regions, resampled with replacement. The noise is the
  corpus's own, not a chosen sigma.
- **error**: the exact mean relative error emitted by p7 part one, computed in
  rational arithmetic. Nothing synthetic, nothing chosen.

The question, per region and per exchange rate: over 2000 bootstrap resamples
of the committed timing samples, how many DISTINCT arms win the weighted
argmin, and does the winning arm's error change? A change in the winning arm's
error is exactly the failure `102` names: the same source, the same weighting
and the same inputs, and a different numeric answer, decided by a bench rerun.

The fadd region is the control. Its arms are answer-equivalent to twelve
printed digits, so a flip there costs time and nothing else, and the probe must
report zero error spread for it or the probe is wrong.

Requires `p7_errors.out` beside it, produced by p7 part one.
"""

import collections
import csv
import os
import random
import statistics
import sys

BOOTSTRAP = 2000
RATES = [0.0, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 10.0, 100.0]


def load_errors(path):
    errs = {}
    with open(path) as fh:
        for line in fh:
            line = line.rstrip("\n")
            if not line or line.startswith("#"):
                continue
            parts = line.split("\t")
            if len(parts) != 3:
                continue
            region, arm, val = parts
            errs.setdefault(region, {})[arm] = float(val)
    return errs


def load_times(benches, csv_name):
    per_arm = collections.defaultdict(list)
    with open(os.path.join(benches, csv_name), newline="") as fh:
        for row in csv.DictReader(fh):
            per_arm[row["variant"]].append(float(row["algo_ns"]))
    return per_arm


def run_region(label, per_arm, errs, rng):
    arms = sorted(per_arm)
    if len(arms) < 2:
        print(f"  {label}: fewer than two arms, skipped")
        return
    missing = [a for a in arms if a not in errs]
    if missing:
        print(f"  {label}: no error coordinate for {missing}, skipped")
        return

    means = {a: statistics.mean(per_arm[a]) for a in arms}
    # normalise both coordinates to the region's own best, so the exchange rate
    # is dimensionless and comparable across regions.
    tmin = min(means.values())
    emin = min(errs[a] for a in arms)

    print(f"  {label}")
    for a in arms:
        print(
            f"    {a:<24} samples={len(per_arm[a]):<4} mean_ns={means[a]:9.2f} "
            f"rel_time={means[a]/tmin:6.3f}  rel_error={(errs[a]/emin) if emin>0 else float('nan'):12.4f}"
        )

    print(f"    {'rate':>8} {'winners':>8} {'errors':>7} {'error spread':>14}  modal winner")
    for rate in RATES:
        winners = collections.Counter()
        for _ in range(BOOTSTRAP):
            best = None
            best_arm = None
            for a in arms:
                s = per_arm[a]
                t = sum(s[rng.randrange(len(s))] for _ in range(len(s))) / len(s)
                e = errs[a]
                # cost = rate * normalised_time + normalised_error.
                # rate = 0 is pure accuracy, large rate is pure speed.
                nt = t / tmin
                ne = (e / emin) if emin > 0 else 0.0
                c = rate * nt + ne
                if best is None or c < best:
                    best = c
                    best_arm = a
            winners[best_arm] += 1
        distinct = len(winners)
        errset = sorted({errs[a] for a in winners})
        spread = (max(errset) - min(errset)) / emin if emin > 0 and errset else 0.0
        modal, cnt = winners.most_common(1)[0]
        print(
            f"    {rate:>8} {distinct:>8} {len(errset):>7} {spread:>14.6f}"
            f"  {modal} at {cnt}/{BOOTSTRAP}"
        )
    print()


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    benches = os.path.abspath(os.path.join(here, "..", "..", "..", "benches"))
    errpath = os.path.join(here, "p7_errors.out")
    if not os.path.isfile(errpath):
        print("p7_errors.out not found; run p7 part one first")
        return 1

    errs = load_errors(errpath)
    rng = random.Random(0xA11CE)

    print("p7 part two. does 102's p5 hazard have a real instance in this corpus?")
    print()
    print(f"bootstrap resamples per rate : {BOOTSTRAP}")
    print("time coordinate              : committed algo_ns samples, resampled")
    print("error coordinate             : exact, from p7 part one")
    print("cost                         : rate * (time/time_min) + (error/error_min)")
    print("                               rate 0 is pure accuracy, large rate pure speed")
    print()

    print("SUBJECT: the answer-differing region")
    for n in [0, 2, 8, 20]:
        name = f"decimal-quantiser-radix-sweep_n{n}.csv"
        run_region(
            f"{name}",
            load_times(benches, name),
            errs["decimal-quantiser-radix-sweep"],
            rng,
        )

    print("CONTROL: the answer-equivalent region with a large time gap")
    for n in [0, 25, 50, 100]:
        name = f"quantiser-vs-fadd-subnormal-sweep_n{n}.csv"
        run_region(
            f"{name}",
            load_times(benches, name),
            errs[f"quantiser-vs-fadd-subnormal-sweep-n{n}"],
            rng,
        )

    print("READING")
    print("  `error spread` is the quantity that matters, in units of the region's")
    print("  own best error. Non-zero means the bootstrap picked arms with")
    print("  different errors at that exchange rate, so a rerun of the bench would")
    print("  change the numeric answer the shipped program produces. Zero means the")
    print("  hazard does not fire at that rate, whatever the time column did.")
    print()
    print("  The control must report zero spread at every rate: its two arms agree")
    print("  to twelve printed digits, so no flip between them can change an answer.")
    print("  A non-zero spread there would mean this probe is measuring wrong.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
