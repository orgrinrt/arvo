#!/usr/bin/env python3
"""P5. Open every citation in 97 and test its CONTENT, not its resolution.

`RULES.md` records this as the cheapest correctness tool the panel has, after one
member found seven of its own citations wrong. A citation landing two lines from the
content still resolves; only reading the target and testing for an expected word
catches it. This is that instrument pointed at my own file.

Each row is (path, line or range, a word that must appear within the cited span).
The word is chosen to be the load-bearing one, so a citation that drifted onto a
heading fails rather than passing on a coincidence.
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.abspath(os.path.join(HERE, ".."))
WS = os.path.abspath(os.path.join(PANEL, "..", "..", "..", "..", ".claude", "rules"))
ARVO = os.path.abspath(os.path.join(PANEL, "..", "..", ".."))

P93 = "93_orchard_the_strategy_axis_derived_cold.md"
P94 = "94_wingo_the_strategy_axis_derived_cold.md"
P25 = "25_torvalds_what_a_strategy_is.md"
P40 = "40_leijen_what_the_axes_actually_are.md"
P88 = "88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md"

CITES = [
    (PANEL, "INTENTS.md", (56, 56), "not closed at exactly four"),
    (PANEL, "INTENTS.md", (327, 327), "besides the point of the intent"),
    (PANEL, P88, (20, 21), "Mostly option 1"),
    (PANEL, P25, (528, 537), "sections over a product of axes"),
    (PANEL, P40, (23, 28), "resolve"),
    (PANEL, P40, (209, 211), "objective is the primitive"),
    (PANEL, P40, (398, 398), "observable"),
    (PANEL, P40, (486, 492), "reachable-determinate"),
    (PANEL, P93, (354, 357), "Nobody asked and everybody pays"),
    (PANEL, P93, (384, 386), "the space is larger than four"),
    (PANEL, P93, (632, 633), "componentwise join over the **policy"),
    (PANEL, P93, (1042, 1044), "established rather than conjectured"),
    (PANEL, P93, (1178, 1180), "law inventory"),
    (PANEL, P94, (420, 421), "independent permissions"),
    (ARVO, "mock/DESIGN.md.tmpl", (43, 43), "Resolve<S1, S2>"),
    (WS, "arvo-toolbox-not-policer.md", (76, 76), "Hot wrapping + Precise saturating"),
    (WS, "arvo-always-optimal-internals.md", (55, 56), "hold\nexactly at `F == 0`"),
    (ARVO, "mock/benches/variants/warm-clamp-shared/src/lib.rs", (83, 83),
     "KEY = W * 10000 + NC * 1000 + LOG2A * 10 + OP"),
    (PANEL, "40_probes/p3_axes_presets_properties.rs", (270, 270),
     "pub fn armswap_consumer_observable"),
    (PANEL, "40_probes/p3_axes_presets_properties.rs", (274, 274),
     "pub fn armswap_consumer_unobservable"),
    (PANEL, "35_probes/p2b_laws_signed.rs", (72, 74),
     "Arithmetic shift right, which floors rather than truncating toward"),
]


def main():
    ok = 0
    bad = 0
    for root, rel, (lo, hi), want in CITES:
        path = os.path.join(root, rel)
        if not os.path.exists(path):
            print("MISSING FILE  %s" % path)
            bad += 1
            continue
        lines = open(path, encoding="utf-8").read().split("\n")
        span = "\n".join(lines[lo - 1:hi])
        flat = " ".join(span.split())
        wantflat = " ".join(want.split())
        if wantflat in flat:
            print("ok    %-58s %s" % ("%s:%d-%d" % (rel, lo, hi), wantflat[:44]))
            ok += 1
        else:
            print("FAIL  %-58s wanted [%s]" % ("%s:%d-%d" % (rel, lo, hi), wantflat))
            print("      got: %s" % flat[:150])
            bad += 1
    print()
    print("citations checked: %d   ok: %d   failed: %d" % (len(CITES), ok, bad))
    if bad:
        print("A failing row means the sentence citing it in 97 is wrong and must be")
        print("corrected against the file rather than from memory.")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
