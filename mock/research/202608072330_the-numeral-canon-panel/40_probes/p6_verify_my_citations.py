#!/usr/bin/env python3
"""
p6. Open every file:line this file cites and test its CONTENT, not its
resolution.

RULES.md records this as the cheapest correctness tool the panel has, after one
member found seven of its own citations wrong. A citation that lands two lines
off a heading still resolves; only a content test catches it.

Each row is (label, path, first line, last line, a string that must appear
somewhere in that range). The string is chosen to be what the citation is FOR,
not merely something nearby.
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.abspath(os.path.join(HERE, ".."))

P25 = "25_torvalds_what_a_strategy_is.md"
P34 = "34_op_hot_may_sacrifice_soundness_for_proven_gain.md"
P35 = "35_mcsherry_what_the_layers_above_need_from_the_numeral.md"
P36 = "36_op_the_intent_behind_each_strategy.md"
P37 = "37_op_warm_imitates_rust_and_strategy_is_not_orthogonal.md"
P38 = "38_op_the_strategies_weigh_measurements_differently.md"
P39 = "39_op_the_strategy_set_is_not_closed.md"
RULES = "RULES.md"

CITES = [
    ("25:132 the Precise row of the preset table", P25, 132, 132, "bitpacked"),
    ("25:186-189 op: the table is one arm", P25, 186, 189, "one evaluation of the strategies"),
    ("25:238 the finding that settles the question", P25, 238, 238, "settles the question"),
    ("25:275-277 four cells covering two axes", P25, 275, 277, "four cells covering two axes"),
    ("25:277-279 Cold has no stated overflow policy", P25, 277, 279, "no stated overflow policy"),
    ("25:341-349 the relation form", P25, 341, 349, "headroom"),
    ("25:351-355 the strategy axis is the wrong phrase", P25, 351, 355, "wrong phrase"),
    ("25:429-430 does not change which values", P25, 429, 430, "does not change which values a numeral denotes"),
    ("25:528-537 the proposed canon sentence", P25, 528, 537, "named\nsections over a product of axes"),
    ("34:16-18 Hot may sacrifice soundness", P34, 16, 18, "provable meaningful gains"),
    ("35:52 three of four reassociable", P35, 52, 53, "70.1%"),
    ("35:201-202 absorption 63 of 63 against 0 of 63", P35, 201, 203, "63 of 63"),
    ("35:222-223 min-plus wrong at 48.9%", P35, 222, 224, "48.9%"),
    ("36:42-43 Precise throws out cold-axis optimisations", P36, 42, 43, "cold axis optimisations"),
    ("36:42-45 Precise throws out both", P36, 42, 45, "hot and the cold axis optimisations"),
    ("36:44-45 within chains and ops", P36, 44, 45, "within chains and ops, not only alone"),
    ("37:43-44 the call kept failing to stick", P37, 43, 45, "failing to stick"),
    ("37:106-107 Cold is for cold paths", P37, 106, 108, "memory or disk storage"),
    ("37:109-110 throughput is for Hot", P37, 109, 111, "that is for Hot"),
    ("38:16-17 they weigh different measurements differently", P38, 16, 18, "weigh different measurements differently"),
    ("39:58-62 the two-by-two carried forward", P39, 58, 62, "exact bijection with a two-by-two"),
    ("39:60-62 evidence about what the axes actually are", P39, 58, 63, "what the axes actually\nare"),
    ("RULES.md:79-83 the permanence test", RULES, 79, 83, "Permanence"),
    ("RULES.md:224-228 mock/benches is the evidence", RULES, 224, 232, "committed harness output"),
]


def main():
    failures = 0
    for label, path, lo, hi, needle in CITES:
        full = os.path.join(PANEL, path)
        if not os.path.exists(full):
            print(f"FAIL  {label}: file does not exist ({path})")
            failures += 1
            continue
        with open(full) as fh:
            lines = fh.readlines()
        if hi > len(lines):
            print(f"FAIL  {label}: file has {len(lines)} lines, cite needs {hi}")
            failures += 1
            continue
        body = "".join(lines[lo - 1:hi])
        # Compare with newlines collapsed and blockquote markers stripped, so a
        # needle spanning a wrapped line inside a quote still matches the text
        # rather than the layout. Without the strip, a quoted sentence that
        # wraps reads as "named > sections" and a correct citation fails.
        unquoted = "\n".join(
            ln[2:] if ln.startswith("> ") else (ln[1:] if ln.startswith(">") else ln)
            for ln in body.splitlines()
        )
        flat_body = " ".join(unquoted.split())
        flat_needle = " ".join(needle.split())
        if flat_needle in flat_body:
            print(f"ok    {label}")
        else:
            print(f"FAIL  {label}: {needle!r} not found in {path}:{lo}-{hi}")
            print(f"        got: {flat_body[:160]}")
            failures += 1
    print()
    print(f"citations checked: {len(CITES)}, failures: {failures}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
