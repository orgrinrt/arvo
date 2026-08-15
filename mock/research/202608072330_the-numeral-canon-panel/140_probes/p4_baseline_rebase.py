"""p4: what a per-arm baseline does to a comparison between strategies.

THIS IS NOT A BENCHMARK AND NOT A MEASUREMENT. It prices nothing, it times
nothing, and it must not be cited as though it did. It is an exhaustive
arithmetic enumeration over synthetic cost tuples, and the only thing it can
establish is a structural fact about how comparisons behave: whether a ranking
can invert. Under the workspace rule it is an ad-hoc quick spike with no
substance as far as any magnitude is concerned, and that is exactly what it is
called here.

The question. Section 6.3 argues that a strategy set needs one shared baseline
in the box, and that letting each strategy state its advantage against its own
naive version is the failure that emptied a literature of scaling claims. The
structural version of that claim is testable exhaustively:

  * SHARED baseline: every arm reports base / cost, for one common base.
  * PER-ARM baseline: every arm reports its own base_i / cost_i.

Claim A: under a shared baseline, the ranking by reported figure is ALWAYS the
ranking by absolute cost. So the reported numbers cannot mislead about which arm
to pick.

Claim B: under per-arm baselines, the ranking by reported figure can be any
permutation of the absolute ranking, including the exact reverse. So an arm can
report the best number while being the worst choice.

THE CASE THAT MUST FAIL, and it is mutual here. If the disagreement counter is
broken it reports zero everywhere, and Claim A would look confirmed for a bad
reason. So the two arms of the sweep control each other: the shared-baseline
count MUST be zero and the per-arm count MUST be non-zero. If both are zero the
counter is not counting, and the probe says so and exits non-zero.
"""

from itertools import product, permutations

# lower is better. these are unitless synthetic numbers, not times.
COSTS = [1, 2, 3, 4, 5, 6]
BASES = [1, 2, 3, 4, 6, 8, 12]


def rank(values, higher_is_better):
    """Return the ordering of indices, best first."""
    idx = list(range(len(values)))
    idx.sort(key=lambda i: (-values[i] if higher_is_better else values[i], i))
    return tuple(idx)


def main():
    n_arms = 3

    shared_disagreements = 0
    shared_total = 0
    perarm_disagreements = 0
    perarm_total = 0
    perarm_full_reversals = 0
    worst_reported_best = 0
    witness = None

    for costs in product(COSTS, repeat=n_arms):
        if len(set(costs)) != n_arms:
            continue  # ties make "the ranking" ambiguous; skip them
        truth = rank(costs, higher_is_better=False)

        # shared baseline: one common base for all arms
        for base in BASES:
            reported = [base / c for c in costs]
            shared_total += 1
            if rank(reported, higher_is_better=True) != truth:
                shared_disagreements += 1

        # per-arm baselines: each arm measured against its own naive version
        for bases in product(BASES, repeat=n_arms):
            reported = [bases[i] / costs[i] for i in range(n_arms)]
            if len(set(reported)) != n_arms:
                continue
            perarm_total += 1
            r = rank(reported, higher_is_better=True)
            if r != truth:
                perarm_disagreements += 1
            if r == tuple(reversed(truth)):
                perarm_full_reversals += 1
            # the sharpest form: the absolutely WORST arm reports the BEST figure
            if r[0] == truth[-1]:
                worst_reported_best += 1
                if witness is None:
                    witness = (costs, bases, reported, truth, r)

    print("p4: NOT A BENCHMARK. exhaustive arithmetic over synthetic cost tuples.")
    print("it prices nothing and times nothing.\n")
    print(f"arms per comparison: {n_arms}")
    print(f"cost values swept: {COSTS}")
    print(f"baseline values swept: {BASES}\n")

    print("SHARED baseline (one ruler in the box):")
    print(f"  {shared_total} comparisons, {shared_disagreements} where the reported")
    print(f"  ranking differs from the absolute-cost ranking")
    print(f"  Claim A (a shared baseline never misleads about the ranking): "
          f"{'CONFIRMED' if shared_disagreements == 0 else 'REFUTED'}\n")

    print("PER-ARM baselines (each arm against its own naive version):")
    print(f"  {perarm_total} comparisons, {perarm_disagreements} where the reported")
    pct = 100.0 * perarm_disagreements / perarm_total if perarm_total else 0.0
    print(f"  ranking differs from the absolute-cost ranking ({pct:.1f}%)")
    print(f"  of those, {perarm_full_reversals} are the exact reverse ordering")
    pct2 = 100.0 * worst_reported_best / perarm_total if perarm_total else 0.0
    print(f"  and in {worst_reported_best} ({pct2:.1f}%) the absolutely WORST arm")
    print(f"  reports the BEST figure")
    print(f"  Claim B (a per-arm baseline can invert the ranking): "
          f"{'CONFIRMED' if perarm_disagreements > 0 else 'REFUTED'}\n")

    if witness:
        costs, bases, reported, truth, r = witness
        print("a witness, spelled out:")
        for i in range(n_arms):
            print(f"  arm {i}: absolute cost {costs[i]}, its own baseline {bases[i]}, "
                  f"so it reports {reported[i]:.2f}x")
        print(f"  absolute ranking, best first: {truth}")
        print(f"  reported ranking, best first: {r}")
        print(f"  arm {r[0]} looks best and is the worst arm in the set.\n")

    print("=== mutual control ===")
    if shared_disagreements == 0 and perarm_disagreements == 0:
        print("BOTH counts are zero, so the counter is not counting and neither")
        print("claim is established. every number above is void.")
        raise SystemExit(1)
    print("the two arms disagree with each other, so the counter distinguishes")
    print("the two regimes. the zero on the shared-baseline side is a real zero.")


if __name__ == "__main__":
    main()
