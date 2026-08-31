#!/usr/bin/env python3
"""Every file:line and every measured number file 26 cites, opened and checked.

The panel has caught a member citing its own sources wrongly five times, and
file 25 built the tool that stops it. This is that tool applied to file 26.

A citation that resolves is not a citation that says what the citing file
claims, so each entry carries the text expected at the target. The expectation
is the test; the path is only the sieve.

Two of these entries exist because this file got them wrong first. The
decoder-shape numbers were quoted as medians when they are that report's means,
and the sequential-sum aligned median was written as 5574 when the report says
5570. Both were corrected before shipping and both are checked here so a later
edit cannot quietly reintroduce them.

Run from anywhere. Exit 0 = all checks pass.
"""

import os
import sys

# Resolved from this file's own location. It was absolute, naming a checkout
# that still exists on this host, so it did not fail when the arc moved: it
# resolved against a different tree and said nothing.
ARVO = os.path.normpath(
    os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "../../..")
)
BENCH = ARVO + "/mock/benches"
VAR = BENCH + "/variants"

# (path, first_line, last_line, substring that must appear in that span)
CITES = [
    # section 1.2: the head-to-head that exists is capped at MAX_N = 16384
    (VAR + "/bitpack-shared/src/lib.rs", 62, 62, "MAX_N: usize = 16384"),
    (VAR + "/bitpack-shared/src/lib.rs", 207, 211, "aligned: [u8; MAX_ALIGNED_BYTES]"),
    (VAR + "/bitpack-shared/src/lib.rs", 207, 211, "logical: [u16; MAX_N]"),
    # section 1.2: the footprint crate's own concession about L1
    (VAR + "/bitpack-footprint-shared/src/lib.rs", 5, 8,
     "never left this host's"),
    # section 1.3: the combined layout and its offsets
    (VAR + "/bitpack-footprint-shared/src/lib.rs", 101, 115,
     "MAX_N: usize = 33_554_432"),
    (VAR + "/bitpack-footprint-shared/src/lib.rs", 101, 115, "PACKED_OFFSET"),
    # section 1.3: build_input_bytes is a real override, not a convenience one
    (VAR + "/bitpack-footprint-shared/src/lib.rs", 36, 42,
     "removes the practical ceiling"),
    # section 3: the --bench filter, and why it matters
    (BENCH + "/src/main.rs", 25, 31, "Six consecutive panel files declined to bench"),
    # section 6.1: the harness does not validate on its own
    (BENCH + "/src/main.rs", 136, 137, "run_orchestrator` never"),
    (BENCH + "/src/main.rs", 136, 145, "400 rows of"),
]

# (findings file, substring that must appear) for every measured number quoted
NUMBERS = [
    # section 2 and 3.1: prior committed results, medians from the CI tables
    ("bitpack-sequential-sum_n16384_findings.md", "| bitpack-aligned-seq | 5570ns |"),
    ("bitpack-sequential-sum_n16384_findings.md", "| bitpack-native-seq | 1667ns |"),
    ("bitpack-sequential-sum_n16384_findings.md", "| bitpack-zeropad-seq | 7679ns |"),
    ("bitpack-decoder-shape_n262144_findings.md", "| bitpack-plan-simd | 55191ns |"),
    ("bitpack-decoder-shape_n262144_findings.md", "| bitpack-plan-windowed | 43779ns |"),
    ("bitpack-decoder-shape_n262144_findings.md", "| bitpack-plan-native | 29212ns |"),
    ("bitpack-decoder-shape_n262144_findings.md", "| bitpack-plan-naive | 133839ns |"),
    # section 5: the head-to-head this file ran, means table
    ("bitpack-footprint-headtohead_n7000000_findings.md", "625428ns"),
    ("bitpack-footprint-headtohead_n7000000_findings.md", "918099ns"),
    ("bitpack-footprint-headtohead_n4194304_findings.md", "366307ns"),
    ("bitpack-footprint-headtohead_n4194304_findings.md", "538446ns"),
    ("bitpack-footprint-headtohead_n1048576_findings.md", "89608ns"),
    ("bitpack-footprint-headtohead_n1048576_findings.md", "133680ns"),
    ("bitpack-footprint-headtohead_n65536_findings.md", "5603ns"),
    ("bitpack-footprint-headtohead_n16384_findings.md", "1418ns"),
    # section 7: the carrier sweep, means table
    ("bitpack-carrier-width_n8388608_findings.md", "1241938ns"),
    ("bitpack-carrier-width_n8388608_findings.md", "1028422ns"),
    ("bitpack-carrier-width_n8388608_findings.md", "766061ns"),
    ("bitpack-carrier-width_n8388608_findings.md", "746274ns"),
    ("bitpack-carrier-width_n4194304_findings.md", "641737ns"),
    ("bitpack-carrier-width_n4194304_findings.md", "507039ns"),
    ("bitpack-carrier-width_n4194304_findings.md", "410984ns"),
    ("bitpack-carrier-width_n2097152_findings.md", "289652ns"),
    ("bitpack-carrier-width_n2097152_findings.md", "251548ns"),
    ("bitpack-carrier-width_n1048576_findings.md", "118684ns"),
    ("bitpack-carrier-width_n1048576_findings.md", "126334ns"),
    ("bitpack-carrier-width_n131072_findings.md", "12543ns"),
    ("bitpack-carrier-width_n16384_findings.md", "2077ns"),
]

fails = []

print("file:line citations")
for path, a, b, want in CITES:
    if not os.path.exists(path):
        fails.append("missing file: %s" % path)
        print("  MISSING  %s" % path)
        continue
    lines = open(path).read().split("\n")
    span = "\n".join(lines[a - 1:b])
    ok = want in span
    print("  %-7s %s:%d-%d  expects %r" %
          ("ok" if ok else "WRONG", os.path.basename(path), a, b, want[:44]))
    if not ok:
        fails.append("%s:%d-%d does not contain %r" % (path, a, b, want))

print()
print("measured numbers, each against the findings file it came from")
for fn, want in NUMBERS:
    path = os.path.join(BENCH, fn)
    if not os.path.exists(path):
        fails.append("missing findings file: %s" % fn)
        print("  MISSING  %s" % fn)
        continue
    ok = want in open(path).read()
    print("  %-7s %-52s %r" % ("ok" if ok else "WRONG", fn, want[:34]))
    if not ok:
        fails.append("%s does not contain %r" % (fn, want))

print()
if fails:
    print("%d FAILED:" % len(fails))
    for f in fails:
        print("  - " + f)
    sys.exit(1)
print("all %d citations and numbers check out" % (len(CITES) + len(NUMBERS)))
