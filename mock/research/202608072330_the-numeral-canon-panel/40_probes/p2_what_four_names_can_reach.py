#!/usr/bin/env python3
"""
p2. What fraction of the mechanism product can four names reach, and which
unreachable points have a consumer the panel has already named?

The axis list is taken from 25 section 4.2, which splits the preset table's
third column into two questions, and from 25 section 4.1, which splits its
second column into two. Each axis is listed with where the record gets it and
with the values the record actually exhibits, which is a lower bound on the
values each axis has rather than a claim that it has exactly these.

The point of the count is not "four is too few". It is that a name assigns a
POINT while a consumer may need any point, so the reachable set is bounded by
the number of names however the axes are cut, and the bound shrinks as a
fraction every time an axis is added. Whether that matters is a design
question; the count is arithmetic.

Two counts are reported and they differ, which is the interesting part:

  reachable-as-stated:  the union of the point SETS the four names denote,
                        where a name that is silent on an axis denotes every
                        value of it.
  reachable-determinate: the points a consumer can actually REQUEST, which
                        requires every coordinate to be named. A name silent on
                        an axis does not let a consumer pick that coordinate;
                        it lets the implementation pick it.
"""

from itertools import product

AXES = {
    "headroom": {
        "values": ("minimum", "doubled"),
        "source": "25 sec 4.1, read off the preset table's Container column "
                  "(25:127-132): 'minimum byte-aligned' against '2x logical'.",
        "observable": False,
        "why": "changes bytes and cycles, never the value computed.",
    },
    "layout": {
        "values": ("addressable", "packed"),
        "source": "25 sec 4.1, same column: 'byte-aligned' against 'bitpacked'.",
        "observable": False,
        "why": "changes bytes and cycles, never the value computed.",
    },
    "overflow": {
        "values": ("wrap", "saturate"),
        "source": "25 sec 4.2, splitting the Arithmetic column: wrapping and "
                  "saturating answer 'what happens when a result does not fit'.",
        "observable": True,
        "why": "wrapping and saturating are different functions on the same "
               "domain (25 sec 5.3); 35 sec 3.4 measures the top absorbing at "
               "63 of 63 cells under saturation and 0 of 63 under wrapping.",
    },
    "intermediate": {
        "values": ("same-width", "widened"),
        "source": "25 sec 4.2, same column: 'widen-op-narrow' answers 'what "
                  "precision does the intermediate carry'.",
        "observable": True,
        "why": "a widened intermediate changes which results are reachable at "
               "all; 35 sec 3.2 derives the accumulator width a fold needs and "
               "shows one bit narrower is insufficient.",
    },
}

# What the shipped preset table states, per name, per axis. None means the
# table is silent on that axis for that name. Read off 25:127-132.
TABLE = {
    "Hot":     {"headroom": "minimum", "layout": "addressable", "overflow": "wrap",     "intermediate": None},
    "Warm":    {"headroom": "doubled", "layout": "addressable", "overflow": "wrap",     "intermediate": None},
    "Cold":    {"headroom": "minimum", "layout": "packed",      "overflow": None,       "intermediate": "widened"},
    "Precise": {"headroom": "doubled", "layout": "packed",      "overflow": "saturate", "intermediate": None},
}

# Consumers named in the panel record for particular points, each with where.
# A consumer is recorded only where a panel file names a requirement that
# pins a coordinate, not where one could be imagined.
CONSUMERS = {
    ("minimum", "packed", "saturate", "same-width"):
        "a stored graph weight column: 35 sec 3.4 shows a min-plus fold needs "
        "an absorbing top, supplied by saturation at 63 of 63 cells and by "
        "wrapping at 0 of 63, and 35 sec 3.5 measures 48.9% wrong answers on "
        "in-range DAG shortest paths without it. 37 quoting op gives such a "
        "column 'minimum for memory or disk storage'. Packed, minimal, and "
        "saturating is what that consumer wants.",
    ("minimum", "addressable", "saturate", "same-width"):
        "a hot-path min-plus relaxation: same absorption requirement, on a "
        "column that is being iterated rather than stored (35 sec 3.4, 3.5).",
    ("minimum", "addressable", "wrap", "widened"):
        "a fold at high arity on a hot path: 35 sec 3.2 derives the "
        "accumulator as element width plus ceil(log2 capacity), and 20's "
        "committed 'accfit' arm, which sizes the accumulator by exactly that "
        "rule, is reported at or near best at every arity measured.",
}


def points_for(name):
    """The set of product points a name denotes, silence meaning 'any'."""
    axes = list(AXES)
    choices = []
    for a in axes:
        v = TABLE[name][a]
        choices.append(AXES[a]["values"] if v is None else (v,))
    return {p for p in product(*choices)}


def determinate_point_for(name):
    """The point a consumer can request by writing this name, or None."""
    axes = list(AXES)
    vals = [TABLE[name][a] for a in axes]
    return tuple(vals) if all(v is not None for v in vals) else None


def main():
    axes = list(AXES)
    total = 1
    for a in axes:
        total *= len(AXES[a]["values"])

    print("AXES, with where the record gets each and whether it is observable")
    print()
    for a in axes:
        d = AXES[a]
        print(f"  {a} = {d['values']}")
        print(f"      source: {d['source']}")
        print(f"      observable by the consumer: {d['observable']}. {d['why']}")
    print()
    print(f"  product size with these values: {total}")
    print()

    print("WHAT THE FOUR NAMES REACH")
    print()
    reachable = set()
    for name in TABLE:
        pts = points_for(name)
        reachable |= pts
        det = determinate_point_for(name)
        silent = [a for a in axes if TABLE[name][a] is None]
        print(f"  {name:<8} denotes {len(pts)} point(s); silent on {silent if silent else 'nothing'}; "
              f"consumer-requestable point: {det if det else 'NONE (silence is the implementation''s choice)'}")
    print()
    print(f"  reachable-as-stated:   {len(reachable)} of {total}  "
          f"({100.0*len(reachable)/total:.1f}%)")
    det_pts = {determinate_point_for(n) for n in TABLE}
    det_pts.discard(None)
    print(f"  reachable-determinate: {len(det_pts)} of {total}  "
          f"({100.0*len(det_pts)/total:.1f}%)")
    print()
    print("  Every one of the four is silent on exactly one axis, so the")
    print("  consumer-requestable count is zero: not one of the four names")
    print("  pins a point in the product it is a name for.")
    print()

    print("UNREACHABLE POINTS WITH A CONSUMER NAMED IN THE PANEL RECORD")
    print()
    n_named = 0
    for pt in sorted(product(*[AXES[a]["values"] for a in axes])):
        if pt in reachable:
            continue
        if pt in CONSUMERS:
            n_named += 1
            print(f"  {pt}")
            print(f"      {CONSUMERS[pt]}")
    for pt, why in CONSUMERS.items():
        if pt in reachable:
            print(f"  (reachable-as-stated, so not counted) {pt}")
    print()
    print(f"  unreachable points with a named consumer: {n_named}")
    print()

    print("HOW THE FRACTION MOVES AS AXES ARE ADDED")
    print("  Candidate further axes named in the record but not counted above:")
    print("    rounding mode  (25 sec 9, absent from arvo entirely per the")
    print("                    prior-art memory it cites; 35 sec 3.10 measures")
    print("                    round-to-nearest dropping one downstream")
    print("                    invariant's failure rate from 87.5% to 12.5%)")
    print("    sign domain    (35 sec 3.6: unsigned saturating addition is")
    print("                    exactly reassociable and signed saturating is")
    print("                    not, at 70.1% of 16.7M vectors)")
    print("    reduction shape (35 sec 6, its Q12: specified tree against")
    print("                    detected split)")
    print()
    print("  {:>5}  {:>12}  {:>26}".format("axes", "product size", "fraction 4 names can pin"))
    for k in range(2, 8):
        size = 2 ** k
        print("  {:>5}  {:>12}  {:>25.1f}%".format(k, size, 100.0 * min(4, size) / size))
    print()
    print("  The bound is 4/2^k however the axes are cut, because a name is a")
    print("  point and there are four names. That is the whole content of the")
    print("  count: it is a statement about naming, not about arvo's axes.")


if __name__ == "__main__":
    main()
