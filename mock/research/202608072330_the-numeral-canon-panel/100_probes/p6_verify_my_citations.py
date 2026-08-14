#!/usr/bin/env python3
"""p6. Open every `file:line` this file cites and test its CONTENT, not its
resolution.

`RULES.md` records this as the cheapest correctness tool the panel has, and that
five instances of the failure went by across two panels before anyone counted. A
citation landing two lines from its content still resolves and still looks right
in a terminal; only reading the target and testing for an expected word catches
it.

Each row below is (path, line spec, a word or phrase that must appear in the
cited span). The phrase is chosen to be the thing the citation is FOR, so a
citation that drifts onto a neighbouring heading or a blank line fails rather
than passing on a coincidence.

Run:  python3 p6_verify_my_citations.py
"""

import glob
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.normpath(os.path.join(HERE, ".."))
ARVO = os.path.normpath(os.path.join(HERE, "..", "..", "..", ".."))

CITATIONS = [
    # op's intents
    ("mock/research/202608072330_the-numeral-canon-panel/INTENTS.md", (56, 56),
     "not closed at exactly four"),
    ("mock/research/202608072330_the-numeral-canon-panel/INTENTS.md", (290, 292),
     "Never any runtime checks"),
    # the register
    ("mock/research/202608072330_the-numeral-canon-panel/OPTIONS.md", (2059, 2060),
     "written as a binary and that was the dispatcher's error"),
    ("mock/research/202608072330_the-numeral-canon-panel/OPTIONS.md", (2089, 2089),
     "The measurement that motivates (b)"),
    ("mock/research/202608072330_the-numeral-canon-panel/OPTIONS.md", (2096, 2096),
     "Standing: ONE EXPERT"),
    # predecessors
    ("mock/research/202608072330_the-numeral-canon-panel/93_orchard_the_strategy_axis_derived_cold.md",
     (966, 973), "Two encodings exist"),
    ("mock/research/202608072330_the-numeral-canon-panel/97_dolan_the_strategy_space_attacked.md",
     (939, 942), "explicable by one statement of what matters"),
    ("mock/research/202608072330_the-numeral-canon-panel/98_spj_what_the_strategy_axis_settles.md",
     (398, 402), "true by construction"),
    ("mock/research/202608072330_the-numeral-canon-panel/98_spj_what_the_strategy_axis_settles.md",
     (473, 477), "generation happens once, by a tool"),
    ("mock/research/202608072330_the-numeral-canon-panel/98_spj_what_the_strategy_axis_settles.md",
     (285, 286), "insensitive to the arm count"),
    ("mock/research/202608072330_the-numeral-canon-panel/99_persona_checkpoint_eight.md",
     (60, 63), "not stable across a rerun"),
    # the probe outputs the central finding reads
    ("mock/research/202608072330_the-numeral-canon-panel/98_probes/p10_is_the_table_stable_enough_to_be_an_object.out",
     (8, 13), "d16-control"),
    ("mock/research/202608072330_the-numeral-canon-panel/93_probes/p4_preference_erases.rs",
     (47, 47), "ARM_COST"),
    # the repository
    ("mock/benches/variants/bitpack-carrier-d16-control/src/lib.rs", (1, 8),
     "Noise-floor control"),
    ("mock/benches/variants/bitpack-carrier-shared/src/lib.rs", (373, 388),
     "all_four_transforms_agree"),
    ("mock/benches/variants/bitpack-carrier-shared/src/lib.rs", (394, 411),
     "validate_output_rejects_a_wrong_sum"),
    ("mock/benches/variants/bitpack-write-contend-shared/src/stress.rs", (68, 68),
     "#[test]"),
    ("mock/benches/variants/bitpack-write-contend-shared/src/stress.rs", (66, 72),
     "must\n/// agree on one thread count"),
    ("mock/research/202608072330_the-numeral-canon-panel/100_probes/p10_the_slow_crate_is_serial_only.out",
     None, "has been running for over 60 seconds"),
    ("mock/benches/variants/warm-clamp-shared/src/lib.rs", (83, 83),
     "W * 10000 + NC * 1000"),
    # the pinned bench harness checkout, resolved from arvo's own lockfile
    ("@HARNESS@/bench-harness/src/harness.rs", (752, 752), "algo_ns,bridge_ns"),
    ("@HARNESS@/bench-harness/src/env.rs", (105, 106), "rustc"),
    # workspace rules
    (".claude/rules/arvo-toolbox-not-policer.md", None, "ship sharp tools"),
    (".claude/rules/arvo-compile-time-last.md", None, "compile time last"),
]


def harness_dir():
    """The pinned mockspace checkout, read from arvo's own bench lockfile so the
    citation cannot drift onto a different revision."""
    lock = os.path.join(ARVO, "mock", "benches", "Cargo.lock")
    rev = None
    with open(lock) as fh:
        txt = fh.read()
    m = re.search(r'name = "mockspace-bench-harness".*?#([0-9a-f]{40})', txt, re.S)
    if m:
        rev = m.group(1)[:7]
    if not rev:
        return None
    base = os.path.expanduser("~/.cargo/git/checkouts")
    for d in glob.glob(os.path.join(base, "mockspace-*", rev)):
        return d
    return None


def main():
    ok = failed = 0
    hd = harness_dir()
    for rel, span, want in CITATIONS:
        if rel.startswith("@HARNESS@"):
            if hd is None:
                print(f"  FAIL  {rel}  <- pinned harness checkout not resolvable")
                failed += 1
                continue
            rel = rel.replace("@HARNESS@", hd)
        path = rel if os.path.isabs(rel) else os.path.join(ARVO, rel)
        if not os.path.exists(path):
            # workspace rules live above the repo
            alt = os.path.normpath(os.path.join(ARVO, "..", rel))
            path = alt if os.path.exists(alt) else path
        if not os.path.exists(path):
            print(f"  FAIL  {rel}  <- file does not exist")
            failed += 1
            continue
        with open(path, errors="replace") as fh:
            lines = fh.read().split("\n")
        if span is None:
            text = "\n".join(lines)
            where = "(whole file)"
        else:
            lo, hi = span
            text = "\n".join(lines[lo - 1 : hi])
            where = f"{lo}-{hi}" if lo != hi else str(lo)
        if want.lower() in text.lower():
            print(f"  ok    {rel}:{where}")
            ok += 1
        else:
            print(f"  FAIL  {rel}:{where}  <- expected {want!r}")
            print(f"        got: {text[:160]!r}")
            failed += 1
    print()
    print(f"citations checked: {ok + failed}   ok: {ok}   failed: {failed}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
