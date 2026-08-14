#!/usr/bin/env python3
"""p11. Open every citation in 98 and test its content, not its resolution.

`25` section 9 built this instrument and `RULES.md:126-134` records that five
instances of the same failure went by across two panels before anybody counted,
and two more in `97`. A citation that lands two lines from its content still
resolves, still looks right in a terminal, and is still wrong. Only reading the
target and testing for an expected word catches it.

Each row below is (path, line range, a word or phrase that must appear in that
range). The expectation is written from what the citing sentence claims, so a
citation that has drifted onto a neighbouring paragraph fails even though the
file and the line both exist.
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.normpath(os.path.join(HERE, ".."))
ARVO = os.path.normpath(os.path.join(HERE, "..", "..", "..", ".."))

CITATIONS = [
    ("INTENTS.md", 56, 56, "not closed at exactly four"),
    ("INTENTS.md", 81, 81, "native primitives in regular old rust"),
    ("INTENTS.md", 92, 94, "absolutely required"),
    ("INTENTS.md", 102, 103, "provable meaningful gains"),
    ("INTENTS.md", 109, 119, "more leeway to do things"),
    ("INTENTS.md", 125, 127, "as makes sense"),
    ("INTENTS.md", 309, 311, "police what kind of laws"),
    ("88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md", 20, 21,
     "Mostly option 1"),
    ("25_torvalds_what_a_strategy_is.md", 533, 534,
     "sections over a product of axes rather than values of a single axis"),
    ("93_orchard_the_strategy_axis_derived_cold.md", 172, 176,
     "Divergence from a reference semantics"),
    ("93_orchard_the_strategy_axis_derived_cold.md", 180, 188,
     "Reproducibility across targets and builds"),
]

SOURCE_CITATIONS = [
    ("mock/benches/variants/bitpack-carrier-shared/src/lib.rs", 513, 516,
     "disagrees with ground truth"),
    ("mock/benches/variants/warm-container-shared/src/lib.rs", 1356, 1356,
     "all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key"),
    ("mock/benches/variants/bitpack-write-contend-shared/src/stress.rs", 68, 68,
     "#[test]"),
]


# One citation-shaped string in 98 is deliberately NOT checked: section 14
# quotes `25:534-537` as the wrong citation it corrected, so a checker that
# tested it would fail on a string whose whole purpose is to be wrong. Named
# here so a later reader can see it was excluded on purpose rather than missed.
EXCLUDED = ["25_torvalds_what_a_strategy_is.md:534-537 (quoted in section 14 as the "
            "citation that was corrected)"]


def check(root, path, lo, hi, expect):
    full = os.path.join(root, path)
    if not os.path.exists(full):
        return False, "file does not exist"
    lines = open(full).read().splitlines()
    if hi > len(lines):
        return False, f"file has {len(lines)} lines, citation reaches {hi}"
    window = "\n".join(lines[lo - 1:hi])
    if expect.replace("\n", " ") in " ".join(window.split()):
        return True, ""
    return False, f"expected text absent from lines {lo}-{hi}"


def main():
    ok = bad = 0
    print("panel citations\n")
    for path, lo, hi, expect in CITATIONS:
        good, why = check(PANEL, path, lo, hi, expect)
        label = f"{path}:{lo}" + (f"-{hi}" if hi != lo else "")
        print(f"  {'ok  ' if good else 'FAIL'}  {label:70s} {'' if good else why}")
        ok += good
        bad += not good

    print("\nrepository citations\n")
    for path, lo, hi, expect in SOURCE_CITATIONS:
        good, why = check(ARVO, path, lo, hi, expect)
        label = f"{path}:{lo}" + (f"-{hi}" if hi != lo else "")
        print(f"  {'ok  ' if good else 'FAIL'}  {label:70s} {'' if good else why}")
        ok += good
        bad += not good

    print("\ndeliberately excluded")
    for e in EXCLUDED:
        print(f"  {e}")
    print(f"\ncitations checked: {ok + bad}   ok: {ok}   failed: {bad}")
    return 0 if bad == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
