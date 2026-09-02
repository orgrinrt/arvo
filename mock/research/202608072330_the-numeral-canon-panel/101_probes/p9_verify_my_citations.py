#!/usr/bin/env python3
"""Open every citation in `101` and test its CONTENT, not its resolution.

`RULES.md` records that a reference which resolves is not a reference that says
what the citing file claims, that five instances of the failure went by before
anyone counted, and that `100` found two of its own this way. Each row below
carries the phrase the citation is FOR, so a citation drifting onto a
neighbouring line fails rather than passing on a coincidence.

Fourteen of thirty-seven failed on the first run and every one was mine. Eight
were line citations into `100`, which grew by 46 lines while this dispatch ran
because its author is still committing to the same branch, so every line I read
from it moved; the numbers below are pinned to `100` at commit `cad7a505`. Four
were a phrase that spans a line break, which the check joins with a newline and
therefore misses. One named `bitpack-wide-shared/src/routine.rs`, a file that
does not exist, for a doc comment that lives in that crate's `lib.rs`. One was a
doc comment cited two lines below where it starts. The first output is kept at
`p9_first_run_fourteen_of_mine_failed.out`.

Run:  python3 p9_verify_my_citations.py
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.normpath(os.path.join(HERE, ".."))
ARVO = os.path.normpath(os.path.join(PANEL, "..", "..", ".."))
PIN = os.path.expanduser(
    "~/.cargo/git/checkouts/mockspace-d2db2c8fb6d9e932/bce17f6"
)
VAR = os.path.join(ARVO, "mock", "benches", "variants")

# (path, line range, a phrase that MUST appear inside that range)
CITES = [
    # op's intents
    (f"{PANEL}/INTENTS.md", (137, 139), "weigh different measurements differently"),
    (f"{PANEL}/INTENTS.md", (155, 156), "ordinary empirical question about"),
    (f"{PANEL}/INTENTS.md", (100, 103), "even at the cost of accuracy or soundness"),
    (f"{PANEL}/INTENTS.md", (102, 103), "provable meaningful gains"),
    (f"{PANEL}/INTENTS.md", (112, 113), "remain small for memory or"),
    (f"{PANEL}/INTENTS.md", (125, 127), "especially within chains and ops"),
    (f"{PANEL}/INTENTS.md", (126, 127), "not only alone"),
    (f"{PANEL}/INTENTS.md", (81, 83), "behave like native primitives in regular old rust"),
    (f"{PANEL}/INTENTS.md", (92, 94), "if mimicking is"),
    # 100 and its probes
    (f"{PANEL}/100_xu_generating_the_table_attacked.md", (520, 520), "threads = 1"),
    (f"{PANEL}/100_xu_generating_the_table_attacked.md", (803, 812), "iqr "),
    (f"{PANEL}/100_xu_generating_the_table_attacked.md", (812, 812), "Strictly better on both"),
    (f"{PANEL}/100_xu_generating_the_table_attacked.md", (841, 841), "a fact about the interquartile range"),
    (f"{PANEL}/100_xu_generating_the_table_attacked.md", (866, 866), "achievable objective range"),
    (f"{PANEL}/100_xu_generating_the_table_attacked.md", (925, 927), "declared constants"),
    (f"{PANEL}/100_xu_generating_the_table_attacked.md", (1090, 1092), "Normalise per region"),
    (f"{PANEL}/100_xu_generating_the_table_attacked.md", (1102, 1104), "per-coordinate tolerance band"),
    (
        f"{PANEL}/100_probes/p1_what_the_instability_is_made_of.py",
        (78, 87),
        "A declared property of the arm, not a measurement",
    ),
    (
        f"{PANEL}/100_probes/p1_what_the_instability_is_made_of.py",
        (118, 128),
        "median time, bytes, IQR",
    ),
    (
        f"{PANEL}/98_probes/p6_reproduce_the_predecessors_count_and_rung_it.out",
        (1, 1),
        "2 coordinates (ns per record, bits per element)",
    ),
    # the pinned harness and core
    (f"{PIN}/bench-harness/src/harness.rs", (752, 752), "run,pass,cooldown_ms,mode,variant"),
    (f"{PIN}/bench-harness/src/harness.rs", (228, 231), "iff the routine declared a score label"),
    (f"{PIN}/bench-harness/src/perf.rs", (9, 14), "Gated behind the `perf-counters` feature"),
    (f"{PIN}/bench-harness/src/perf.rs", (16, 21), "takes exclusive control of the PMU"),
    (f"{PIN}/bench-core/src/lib.rs", (98, 107), "Score an output for quality comparison"),
    (f"{PIN}/bench-core/src/lib.rs", (182, 193), "Multi-dimensional quality scores for Pareto analysis"),
    (f"{PIN}/bench-core/src/lib.rs", (421, 438), "populated by the matrix scaffold"),
    (f"{PIN}/bench-core/src/lib.rs", (427, 429), "(S_b - S_a) / (I_a - I_b)"),
    (f"{PIN}/bench-core/src/lib.rs", (432, 435), "reps-invariant fidelity witness"),
    # the corpus
    (f"{ARVO}/mock/benches/bench.toml", (312, 312), "Layout::Dense footprint"),
    (f"{ARVO}/mock/benches/bench.toml", (359, 359), "Layout::Bitpacked footprint"),
    (f"{VAR}/warm-clamp-shared/src/lib.rs", (83, 89), "KEY = W * 10000 + NC * 1000 + LOG2A * 10 + OP"),
    (f"{VAR}/bitpack-contend-shared/src/routine.rs", (11, 24), "KEY = N * 10 + T"),
    (f"{VAR}/bitpack-wide-shared/src/lib.rs", (102, 102), "contention crate's encoding unchanged"),
    (f"{VAR}/bitpack-carrier-d16-control/src/lib.rs", (1, 8), "differ only in the exported symbol name"),
    (f"{VAR}/bitpack-contend-d16-control/src/lib.rs", (1, 5), "byte-identical to `bitpack-contend-d16`"),
    (f"{VAR}/bitpack-wide-d16-control/src/lib.rs", (1, 6), "byte-identical to `bitpack-wide-d16`"),
]


def main():
    ok = bad = 0
    for path, (lo, hi), phrase in CITES:
        short = path.replace(ARVO + "/", "").replace(PANEL + "/", "").replace(PIN + "/", "")
        if not os.path.exists(path):
            print(f"  MISSING FILE  {short}:{lo}-{hi}")
            bad += 1
            continue
        lines = open(path, encoding="utf-8", errors="replace").read().splitlines()
        span = "\n".join(lines[lo - 1 : hi])
        if phrase in span:
            ok += 1
        else:
            bad += 1
            print(f"  FAILED  {short}:{lo}-{hi}")
            print(f"          wanted: {phrase!r}")
            print(f"          got:    {span[:200]!r}")
    print()
    print(f"citations checked: {ok + bad}   ok: {ok}   failed: {bad}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
