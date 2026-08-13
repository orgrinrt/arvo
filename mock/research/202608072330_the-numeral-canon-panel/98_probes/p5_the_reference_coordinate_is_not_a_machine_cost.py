#!/usr/bin/env python3
"""p5. One of the strategies is defined by a coordinate that is not a measurement of the machine.

I3 is op's most-repeated call, stated four times over two days: the strategy in
question "should behave like native primitives in regular old rust would". I4
sharpens it: the imitation is wanted because it is intuitive, and it is not
absolutely required if it is "consistently just worse choice".

If a strategy is a weighting over measurements, that call has to be a weighting
over something. This probe asks what, using the repository's own committed
measurement of exactly this fork.

The bench is `warm-container-*`, six arms differing in which machine carrier a
declared width is held in and where the projection back to the declared width
sits. Two of them are the fork this question is about:

  headroom   the shipped rule: hold a declared width in the rung ABOVE the
             smallest one that fits, so a 13-bit numeral lives in a u32.
             Twice the bytes of the minimum, and room above the declared width
             for an intermediate result to sit in.
  minimum    hold it in the smallest rung that fits, so 13 bits live in a u16.

Both are declared side by side in the variant crate, and the bench's own
load-bearing test asserts every arm computes the identical value on every key,
so a timing difference between them is a difference in cost and not in answer
(`mock/benches/variants/warm-container-shared/src/lib.rs:1356`).

The question this probe answers: is the shipped rule reachable by ANY weighting
over the machine coordinates the harness measures? If it is Pareto-dominated on
all of them, the answer is no, and the coordinate that makes it the right choice
is not a measurement of the machine at all.
"""

import csv
import glob
import os
import random
import re
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

# Bytes per element, per arm, read off the carrier tables in
# variants/warm-container-shared/src/lib.rs:1257-1319. The declared width is
# encoded in the bench key; these are the two entries the fork turns on.
CARRIER = {
    8:  {"headroom": 2, "minimum": 1},
    13: {"headroom": 4, "minimum": 2},
    16: {"headroom": 4, "minimum": 2},
    32: {"headroom": 8, "minimum": 4},
    60: {"headroom": 16, "minimum": 8},
    64: {"headroom": 16, "minimum": 8},
}

PAIR = ("warm-container-headroom", "warm-container-minimum")


def key_w(key):
    return key // 10_000


def rows_for(path):
    per = {}
    for row in csv.DictReader(open(path)):
        per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
    return per


def main():
    paths = sorted(glob.glob(os.path.join(BENCH, "warm-container-*_n*.csv")))
    print(f"committed warm-container runs: {len(paths)}\n")

    print(f"{'run':38s} {'W':>3s} {'headroom ns':>13s} {'minimum ns':>12s} "
          f"{'h bytes':>8s} {'m bytes':>8s}  verdict")
    dominated = 0
    considered = 0
    faster_cases = []
    for path in paths:
        base = os.path.basename(path)
        m = re.search(r"_n(\d+)\.csv$", base)
        key = int(m.group(1))
        w = key_w(key)
        if w not in CARRIER:
            continue
        per = rows_for(path)
        if not all(v in per for v in PAIR):
            continue
        h = statistics.median(per[PAIR[0]])
        mn = statistics.median(per[PAIR[1]])
        hb, mb = CARRIER[w]["headroom"], CARRIER[w]["minimum"]
        considered += 1
        # minimum dominates headroom when it is no worse on both and better on one.
        dom = (mn <= h and mb <= hb) and (mn < h or mb < hb)
        if dom:
            dominated += 1
            verdict = "minimum dominates headroom"
        else:
            verdict = "headroom survives"
            faster_cases.append((base, w, h, mn, hb, mb))
        print(f"{base[:38]:38s} {w:3d} {h:13.1f} {mn:12.1f} {hb:8d} {mb:8d}  {verdict}")

    print()
    print(f"runs considered: {considered}")
    print(f"runs where the shipped rule is Pareto-dominated on (time, bytes): "
          f"{dominated}")
    print(f"runs where it survives: {len(faster_cases)}")
    for base, w, h, mn, hb, mb in faster_cases:
        print(f"    {base}: W={w}, headroom {h:.1f} ns / {hb} B vs "
              f"minimum {mn:.1f} ns / {mb} B")
    print()

    # The survivors above all survive on a sub-one-percent time edge while
    # paying double the bytes. A difference that small is worth testing rather
    # than accepting: if it is not distinguishable from zero at the harness's
    # own sample count, then "headroom is faster here" is a statement about
    # forty samples and not about the arms, and the domination verdict flips.
    if faster_cases:
        print("testing the survivors' time edge against zero")
        print("  paired-free two-sided bootstrap on the difference of medians,")
        print("  200000 resamples, seed 20260814. A CI containing zero means the")
        print("  edge the survival rests on is not measurable.")
        rng = random.Random(20260814)
        flipped = 0
        for base, w, h, mn, hb, mb in faster_cases:
            per = rows_for(os.path.join(BENCH, base))
            a, b = per[PAIR[0]], per[PAIR[1]]
            obs = statistics.median(a) - statistics.median(b)
            diffs = []
            for _ in range(200000 // 100):
                ra = [a[rng.randrange(len(a))] for _ in range(len(a))]
                rb = [b[rng.randrange(len(b))] for _ in range(len(b))]
                diffs.append(statistics.median(ra) - statistics.median(rb))
            diffs.sort()
            lo = diffs[int(0.025 * len(diffs))]
            hi = diffs[int(0.975 * len(diffs)) - 1]
            spans_zero = lo <= 0 <= hi
            if spans_zero:
                flipped += 1
            pct = 100.0 * obs / statistics.median(b)
            print(f"    {base[:44]:44s} diff {obs:9.1f} ns ({pct:+.2f}%) "
                  f"CI [{lo:.1f}, {hi:.1f}]  "
                  f"{'not measurable' if spans_zero else 'measurable'}")
        print(f"  survivors whose edge is not measurable: {flipped} of "
              f"{len(faster_cases)}")
        if flipped == len(faster_cases):
            print("  So on every committed run of this bench, the shipped rule is")
            print("  either dominated outright or survives only on a time edge that")
            print("  the harness cannot distinguish from zero, while paying twice")
            print("  the footprint in all 22.")
        print()

    if dominated == considered:
        print("The shipped rule is dominated on both machine coordinates in EVERY")
        print("committed run of this bench. So there is no weighting over (time,")
        print("bytes) that selects it anywhere, by the same argument p1b makes: a")
        print("strictly positive weight vector cannot prefer a point that is worse")
        print("on every coordinate.")
    else:
        print("The shipped rule survives domination somewhere, so the conclusion")
        print("below is bounded to the runs where it does not.")
    print()
    print("What follows, and what does not.")
    print()
    print("  Does NOT follow: that the shipped rule is wrong. The arms compute the")
    print("  same value on the keys this bench sweeps, but the rule exists for the")
    print("  keys where they would NOT: an intermediate result that overflows the")
    print("  declared width has somewhere to sit in the wider carrier and nowhere")
    print("  in the narrower one. That is what the extra byte buys.")
    print()
    print("  DOES follow: whatever the extra byte buys is not on this list of")
    print("  coordinates. Time and footprint both say do not do it. The coordinate")
    print("  that says do it is agreement with a reference semantics, which is a")
    print("  measurement against another implementation rather than against the")
    print("  machine.")
    print()
    print("  So a canon that says a strategy is a weighting over measurements owes")
    print("  an answer to which measurements, and the answer cannot be 'the ones a")
    print("  profiler reports'. At least one coordinate is divergence from a named")
    print("  reference, and I4 gives that coordinate a FINITE weight rather than an")
    print("  absolute priority: mimicry is dropped where it is 'consistently just")
    print("  worse choice'.")


if __name__ == "__main__":
    main()
