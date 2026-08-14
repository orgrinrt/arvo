#!/usr/bin/env python3
"""p2b. Why the strictly-positive rung caught none of p2's dropped-coordinate
generators, and the predicate under which it would.

p2 predicted, before running, that `G5 dropped` (a generator that never reads
one coordinate) would be caught by the strictly-positive rationalisability rung,
on the reasoning that a dropped coordinate IS a zero weight and `98` section 2.3
establishes that a zero weight is what admits an arm no strictly positive
weighting can reach.

The prediction was wrong: 0 of 152 and 0 of 160. This probe finds out why, and
the answer sharpens `98`'s finding rather than weakening it.

THE MECHANISM, stated as a hypothesis before measuring. The section a zero weight
produces is admitted by a CONE, and a cone with non-empty interior contains
strictly positive points arbitrarily close to any boundary point. Setting the
third weight to zero puts you on the boundary of the weight simplex, not on the
boundary of the admitting cone: as long as every difference-vector inequality is
satisfied STRICTLY at that section, a small positive third weight preserves them
all. So the zero-weight section is also strictly rationalisable, and D1s is
blind to it.

The inequalities become tight, and the hazard becomes real, exactly when two
arms carry the SAME VALUE on some coordinate, because then the difference vector
has a zero there and the zero weight is doing load-bearing work.

That is not a hypothetical shape. It is the committed carrier table:
`bitpack-carrier-packed` and `bitpack-carrier-packed-simd` both carry 13 bits per
element, so they are exactly tied on the footprint coordinate
(`mock/benches/variants/bitpack-carrier-packed/src/lib.rs` against
`.../bitpack-carrier-packed-simd/src/lib.rs`, both decoding the same 13-bit
column). `97` names the same pair for the same reason when reconciling 9 with 72.

THE TEST. Three model families, otherwise identical:

  independent   every cost drawn independently, so exact ties do not occur
  shared-coord  two arms share one coordinate exactly, mirroring the real table
  duplicate     two arms are identical on every coordinate, mirroring what a
                noise-floor control arm does to a model
  tied-except-dropped
                two arms tie on both coordinates the generator reads and differ
                only on the one it drops, which is the only shape in which a
                zero weight can reach a dominated arm

and for each, how often the strictly-positive rung catches a dropped
coordinate, plus how often the emitted section selects an arm that is
Pareto-dominated on the full coordinate set, which is the event the rung's
theorem is actually about.

Exact rational arithmetic. Not a bench, no measurement taken.

Run:  python3 p2b_why_the_strict_rung_caught_nothing.py
"""

import random
from fractions import Fraction

import cone

D = 3
NR, NA = 5, 5


def independent(rng):
    regions, arms = list(range(NR)), list(range(NA))
    costs = {
        r: {a: tuple(Fraction(rng.randrange(1, 400)) for _ in range(D)) for a in arms}
        for r in regions
    }
    return costs, regions, arms


def shared_coord(rng):
    """Two arms carry the same value on coordinate 1 at every region, the way
    two arms decoding the same packed column carry the same bits per element."""
    costs, regions, arms = independent(rng)
    for r in regions:
        c0, c1 = costs[r][arms[0]], costs[r][arms[1]]
        costs[r][arms[1]] = (c1[0], c0[1], c1[2])
    return costs, regions, arms


def duplicate(rng):
    """Two arms identical on every coordinate at every region."""
    costs, regions, arms = independent(rng)
    for r in regions:
        costs[r][arms[1]] = costs[r][arms[0]]
    return costs, regions, arms


def tied_except_dropped(rng):
    """Arms 0 and 1 tie on the two coordinates the generator DOES read, and arm
    0 is strictly worse on the one it drops. This is the only shape in which a
    zero weight can reach a dominated arm, and it is the shape the committed
    carrier table has: `bitpack-carrier-packed` and `-packed-simd` carry the
    same 13 bits per element and differ only in time, so a zero weight on time
    weakly admits the slower of the two, which is exactly `98` section 2.3's
    63 of 72.

    Arm 0 is made the worse one so the `first` tie-break lands on it; with the
    better one first the tie-break masks the hazard and the family would report
    clean for a reason that has nothing to do with the rung.
    """
    costs, regions, arms = independent(rng)
    for r in regions:
        c0, c1 = costs[r][arms[0]], costs[r][arms[1]]
        costs[r][arms[0]] = (c0[0], c0[1], c0[2] + 100)
        costs[r][arms[1]] = (c0[0], c0[1], c0[2])
    return costs, regions, arms


def dominated_at(costs, r, arms, a):
    ca = costs[r][a]
    for b in arms:
        if b == a:
            continue
        cb = costs[r][b]
        if all(x <= y for x, y in zip(cb, ca)) and any(x < y for x, y in zip(cb, ca)):
            return True
    return False


def run(name, maker, n, rng):
    differs = caught_strict = caught_nonneg = picks_dominated = 0
    tight = 0
    for _ in range(n):
        costs, regions, arms = maker(rng)
        w = tuple(Fraction(rng.randrange(1, 60)) for _ in range(D))
        ref = cone.argmin_section(w, costs, regions, arms, "first")
        w0 = (w[0], w[1], Fraction(0))
        sec = cone.argmin_section(w0, costs, regions, arms, "first")
        if all(sec[r] == ref[r] for r in regions):
            continue
        differs += 1
        if not cone.nonempty(sec, costs, regions, arms, D, strict=True):
            caught_strict += 1
        if not cone.nonempty(sec, costs, regions, arms, D, strict=False):
            caught_nonneg += 1
        if any(dominated_at(costs, r, arms, sec[r]) for r in regions):
            picks_dominated += 1
        # how often the zero weight is load bearing: some difference vector has
        # a zero in the dropped coordinate at the chosen arm
        if any(
            costs[r][a][2] == costs[r][sec[r]][2]
            for r in regions
            for a in arms
            if a != sec[r]
        ):
            tight += 1
    pct = lambda x: f"{x:>4} ({100.0 * x / differs:5.1f}%)" if differs else "   0  (n/a)"
    print(
        f"  {name:<14} differs {differs:>4}   "
        f"strict rung caught {pct(caught_strict)}   "
        f"selects a dominated arm {pct(picks_dominated)}   "
        f"zero weight load bearing {pct(tight)}"
    )
    return differs, caught_strict, picks_dominated


def main():
    print(__doc__.split("Run:")[0].strip())
    print()
    rng = random.Random(20260814)
    n = 300
    print(f"{n} models per family, {NR} regions, {NA} arms, {D} coordinates")
    print("generator: coordinate 2 never read, which is a zero weight on it")
    print()
    run("independent", independent, n, rng)
    run("shared-coord", shared_coord, n, rng)
    run("duplicate", duplicate, n, rng)
    run("tied-except-dropped", tied_except_dropped, n, rng)
    print()
    print("=" * 78)
    print("READING")
    print("=" * 78)
    print(
        """
The strictly-positive rung is a detector for exactly one event: the section
selects an arm that is Pareto-dominated on the full coordinate set. The two
columns measuring those are identical in every family above, 0 and 0 three times
and 230 and 230 in the fourth, which is what "detector for exactly one event"
looks like when it is true rather than asserted.

It is NOT a detector for a generator reading the wrong thing, and p2's prediction
that it would be was wrong. A zero weight sits on the boundary of the weight
simplex, not on the boundary of the admitting cone, so as long as every
difference-vector inequality holds strictly the section stays strictly
rationalisable and the rung sees nothing.

Whether the event is reachable at all is a property of the ARM SET rather than of
the weighting, and the predicate is exact: it needs a pair of arms that tie on
every coordinate the effective weighting reads and differ on one it does not.
Arms drawn independently never satisfy it, 0 of 230. Arms sharing one coordinate
while differing on the rest do not satisfy it either, 0 of 241, which refutes the
looser form of the hypothesis this probe opened with. Two arms identical
everywhere do not satisfy it, 0 of 234, because identical arms do not dominate
each other. Only the exact shape satisfies it, 230 of 285.

That exact shape is what the committed carrier table has. `bitpack-carrier-packed`
and `-packed-simd` carry the same 13 bits per element and differ in time, so a
zero weight on TIME reaches the slower of the two, which is `98` section 2.3's 63
of 72 rather than an abstract possibility.

So `98`'s constraint stands and its predicate is sharper than the general
statement suggests. Strict positivity buys the no-dominated-arm guarantee; the
guarantee is reachable exactly where the arm set contains such a pair, which is a
property of the arm set checkable directly and once; and it buys nothing whatever
against a generator computing the wrong argmin.
"""
    )


if __name__ == "__main__":
    main()
