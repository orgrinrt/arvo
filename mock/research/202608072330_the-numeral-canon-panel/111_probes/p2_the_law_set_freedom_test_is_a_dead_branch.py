#!/usr/bin/env python3
"""P2. `110`'s TEST 3 cannot report anything but zero, and the reason is the
enumeration rather than the mathematics.

`110` F3's third bullet is the one the dispatch brief names as the measurement:
"0 of 48 configurations can vary the law set with the others held fixed". Its
instrument is `110_probes/p2_laws_are_a_projection_not_a_coordinate.py`, TEST 3:

    seen = {}
    free = 0
    for p in configs:
        k = p.key()
        if k in seen and law_set(seen[k]) != law_set(p):
            free += 1
        seen[k] = p

and `configs` is built at that file's line 191 by

    product([3,4], [0,1,2], [False,True], ["sat","wrap"], ["near","trunc"])

while `key()` at line 51 returns exactly those five components plus a constant
radix. So every key in the sweep is distinct, `k in seen` is never true, and the
statement that increments `free` never executes.

This probe establishes that mechanically rather than by reading, and then
mutation-tests the instrument: it reruns the identical test with the law set
made into a genuinely free coordinate, the exact condition TEST 3 exists to
detect, and shows the verdict does not move.

The conclusion `110` draws is not under attack here and I believe it. What is
under attack is citing this as a measurement, because a test that cannot fail
has measured nothing and the rung it is cited at should reflect that.
"""

from itertools import product

# The sweep and the key, transcribed from 110_probes/p2 lines 190-194 and 51-52.
AXES = ([3, 4], [0, 1, 2], [False, True], ["sat", "wrap"], ["near", "trunc"])
RADIX = 2


def key(cfg):
    W, F, signed, policy, rounding = cfg
    return (W, F, signed, policy, rounding, RADIX)


def freedom_test(configs, law_set_of):
    """TEST 3, transcribed, with the branch instrumented."""
    seen = {}
    free = 0
    key_hits = 0          # how often `k in seen` was true
    comparisons = 0       # how often the law sets were actually compared
    for cfg in configs:
        k = key(cfg)
        if k in seen:
            key_hits += 1
            comparisons += 1
            if law_set_of(seen[k]) != law_set_of(cfg):
                free += 1
        seen[k] = cfg
    return free, key_hits, comparisons


def main():
    configs = list(product(*AXES))
    print("P2. can 110's TEST 3 report a nonzero answer?")
    print("=" * 74)
    print(f"configurations swept        : {len(configs)}")
    print(f"distinct keys among them    : {len({key(c) for c in configs})}")
    print()

    # ---- arm 1: the law set as 110 computes it, a function of the tables.
    # Stubbed here as any function of the configuration, because the point is
    # about the loop and not about the arithmetic.
    def law_set_real(cfg):
        W, F, signed, policy, rounding = cfg
        s = {"add_comm", "add_zero", "mul_one"}
        if F == 0:
            s |= {"mul_assoc", "distrib_add"}
        if policy == "wrap":
            s |= {"neg_involutive"}
        else:
            s |= {"add_monotone"}
        return frozenset(s)

    free, hits, comps = freedom_test(configs, law_set_real)
    print("arm 1: law set computed from the configuration (110's shape)")
    print(f"  times `k in seen` was true   : {hits}")
    print(f"  law-set comparisons performed: {comps}")
    print(f"  free variations found        : {free}")
    print(f"  verdict printed by TEST 3    : "
          f"{'FREE (a coordinate)' if free else 'DETERMINED (a projection)'}")
    print()

    # ---- arm 2: the MUTATION. The law set is now a genuinely free coordinate,
    # declared per configuration and reading nothing about the algebra. This is
    # exactly the condition TEST 3 claims to detect. A live test moves here.
    declared = {}
    for i, cfg in enumerate(configs):
        declared[cfg] = frozenset({f"declared_law_{i % 3}"})

    free_m, hits_m, comps_m = freedom_test(configs, lambda c: declared[c])
    print("arm 2: MUTATION, law set declared freely per configuration")
    print("       (three different declared law sets across the 48 configs)")
    print(f"  distinct declared law sets   : {len(set(declared.values()))}")
    print(f"  times `k in seen` was true   : {hits_m}")
    print(f"  law-set comparisons performed: {comps_m}")
    print(f"  free variations found        : {free_m}")
    print(f"  verdict printed by TEST 3    : "
          f"{'FREE (a coordinate)' if free_m else 'DETERMINED (a projection)'}")
    print()

    print("-" * 74)
    same = (free == free_m == 0) and (comps == comps_m == 0)
    print(f"the mutation does not move the verdict : {same}")
    print("so the test does not distinguish a projection from a free coordinate.")
    print()

    # ---- arm 3: what a live version of the same test looks like. The sweep has
    # to contain two configurations agreeing on the key, which means the key has
    # to be a projection of the sweep rather than a transcription of it.
    print("arm 3: the same test over a sweep whose key is coarser than its axes")
    print("       (radix added to the sweep, absent from the key, as it would be")
    print("        if somebody asked whether radix is a free coordinate)")
    wide = [(W, F, s, p, r, radix)
            for (W, F, s, p, r) in configs for radix in (2, 3)]

    def key6_dropping_radix(cfg):
        W, F, s, p, r, _radix = cfg
        return (W, F, s, p, r)

    def observable_at_f0(cfg):
        # A stand-in reading that depends on radix only when F > 0, which is
        # 110's own F5. At F = 0 the two radices agree and the test finds
        # nothing; at F > 0 they differ and the test finds it.
        W, F, s, p, r, radix = cfg
        return frozenset({f"step_1_over_{radix ** F}"})

    seen, free3, hits3 = {}, 0, 0
    for cfg in wide:
        k = key6_dropping_radix(cfg)
        if k in seen:
            hits3 += 1
            if observable_at_f0(seen[k]) != observable_at_f0(cfg):
                free3 += 1
        seen[k] = cfg
    print(f"  configurations               : {len(wide)}")
    print(f"  times `k in seen` was true   : {hits3}")
    print(f"  free variations found        : {free3}")
    print("  verdict: the branch is reachable and the count is informative")
    print(f"  (the {free3} are exactly the F > 0 cells, which is 110's F5 from")
    print("   the other side: at F = 0 the radix is not free, above it is)")


if __name__ == "__main__":
    main()
