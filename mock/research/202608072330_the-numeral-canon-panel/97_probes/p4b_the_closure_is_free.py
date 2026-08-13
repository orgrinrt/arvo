#!/usr/bin/env python3
"""P4b. `93`'s closure count is 2^d - 1, so it is generated and never enumerated.

`93`'s P1b Part B reports two numbers for a flat set of four markers each carrying one
demand, with resolution modelled as union of demands: 12 of 16 ordered pairs are
unresolvable, and the smallest set closed under the members' own resolution has 15
elements. `93` section 4 lists carrying that closure as response (a) and prices it as
"the space is larger than four".

This reproduces both numbers independently, then names the object: the closure is the
free join-semilattice on the d generators, whose carrier is the non-empty subsets, so
its size is 2^d - 1 for every d. Fifteen is not a number the design has to hold. It is
what four generators and one formal join produce.

The reproduction matters as much as the identification. `93`'s numbers came from a
probe this file has not read, so agreeing with them from an independently written
enumeration is a second instance rather than a citation.
"""

import itertools


def report(d, names):
    gens = [1 << i for i in range(d)]
    print("=" * 66)
    print("d = %d generators: %s" % (d, ", ".join(names)))

    unresolvable = 0
    pairs = 0
    for a in gens:
        for b in gens:
            pairs += 1
            u = a | b
            if u not in gens:
                unresolvable += 1
    print("  ordered pairs of generators                : %d" % pairs)
    print("  pairs whose union is NOT itself a generator: %d" % unresolvable)

    seen = set(gens)
    frontier = set(gens)
    while frontier:
        nxt = set()
        for a in seen:
            for b in seen:
                u = a | b
                if u not in seen:
                    nxt.add(u)
        if not nxt:
            break
        seen |= nxt
        frontier = nxt
    print("  closure under union                        : %d elements" % len(seen))
    print("  2^d - 1                                    : %d" % ((1 << d) - 1))
    print("  identical                                  : %s" % (len(seen) == (1 << d) - 1))

    un2 = sum(1 for a in seen for b in seen if (a | b) not in seen)
    print("  ordered pairs unresolvable INSIDE the closure: %d of %d"
          % (un2, len(seen) ** 2))

    print("  names a design must write down             : %d (the generators)" % d)
    print("  elements a consumer can name anyway        : %d (formal joins of them)"
          % len(seen))


def main():
    print("P4b. the closure of d one-demand generators under union")
    print()
    report(4, ["speed", "residency", "accuracy", "familiarity"])
    print()
    report(3, ["speed", "residency", "accuracy"])
    print()
    report(5, ["speed", "residency", "accuracy", "familiarity", "reproducibility"])
    print()
    print("=" * 66)
    print("growth: adding one generator doubles the carrier and adds ONE name.")
    for d in range(2, 9):
        print("  d = %d  carrier = %4d  names = %d" % (d, (1 << d) - 1, d))
    print()
    print("So `93` section 4's stated cost of response (a), that every question about")
    print("a strategy must be answered per coordinate rather than looked up per")
    print("marker, is the cost of the structure being GENERATED. That is the same cost")
    print("a formal union type carries in a type system with unions, and it is what")
    print("buys totality: zero unresolvable pairs against 12 of 16 on the flat set.")
    print()
    print("Note what this does NOT settle. It says the join is total and lawful on the")
    print("demand side. It says nothing about whether the demand side is where the")
    print("resolution question lives, which P3 attacks separately.")


if __name__ == "__main__":
    main()
