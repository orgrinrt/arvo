#!/usr/bin/env python3
"""p7: open every file:line citation in 141 and test its CONTENT, not that it resolves.

RULES.md:126 requires this and records that one member found seven of its own
citations wrong by doing it. A citation that resolves is not a citation that says
what the citing sentence claims, and the difference is invisible until somebody
opens it.

Each entry names a target range and a substring that must appear inside it. Two
entries are DELIBERATELY WRONG and must fail, because a checker that reports zero
failures without ever having failed is a checker nobody has tested.
"""

import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent


def norm(s):
    """Join wrapped lines and collapse whitespace, stripping blockquote markers,
    so a quotation that wraps inside a markdown blockquote still matches."""
    out = []
    for line in s.splitlines():
        t = line.strip()
        if t.startswith(">"):
            t = t[1:].strip()
        out.append(t)
    return " ".join(" ".join(out).split())


# (file, first line, last line, substring that must be present, must_pass)
CITES = [
    # INTENTS.md
    ("INTENTS.md", 51, 61, "the strategy set is not closed at exactly four", True),
    ("INTENTS.md", 110, 117, "Hot *can* sacrifice soundness, that is its explicit purpose", True),
    ("INTENTS.md", 119, 127, "it aggressively minimises and", True),
    ("INTENTS.md", 129, 133, "It can use the same paths Hot uses", True),
    ("INTENTS.md", 214, 235, "there's a nameable predicate for the sometimes", True),
    ("INTENTS.md", 214, 235, "We explicitly reject a universal solution", True),
    ("INTENTS.md", 317, 331, "We shouldn't police what kind of laws there are", True),
    ("INTENTS.md", 317, 331, "on a case by case basis", True),
    ("INTENTS.md", 363, 383, "besides the point of the intent", True),
    # RULES.md
    ("RULES.md", 124, 124, "Counts are measurements", True),
    ("RULES.md", 126, 126, "Check your own citations before shipping", True),
    ("RULES.md", 486, 541, "No finding in this panel states a result without stating the region", True),
    # 139
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        220,
        222,
        "hold everything else fixed",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        230,
        233,
        "Zero disagreements between packed and padded",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        244,
        249,
        "packing is answer-invisible",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        260,
        263,
        "two builds of one program produce different results",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        266,
        268,
        "The policy component determines the answer",
        True,
    ),
    ("139_muratori_the_strategy_set_derived_cold.md", 293, 302, "42.14%", True),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        338,
        338,
        "a policy specifies a set of acceptable answers rather than one",
        True,
    ),
    ("139_muratori_the_strategy_set_derived_cold.md", 347, 352, "32", True),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        356,
        357,
        "One unit in the last place buys fusion",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        497,
        500,
        "the count is not a property of the design at all",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        504,
        514,
        "An axis earns its place by a test, not by an argument",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        604,
        607,
        "a slack field on every policy and a conformance obligation on every arm",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        661,
        664,
        "the kind of argument that sounds obviously right",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        703,
        717,
        "livelock",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        750,
        767,
        "container width = declared width",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        769,
        776,
        "observability belongs to the chain",
        True,
    ),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        788,
        800,
        "keeps the property my firewall exists for",
        True,
    ),
    ("139_muratori_the_strategy_set_derived_cold.md", 166, 171, "12", True),
    # 140
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        160,
        169,
        "closed, finite, and arvo's to enumerate",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        263,
        279,
        "It can use the same paths Hot uses",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        564,
        575,
        "90 configs -> 24 classes",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        577,
        579,
        "The storage-minimising concern is a weighting, not an assignment",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        620,
        624,
        "every strategy's cost claim is stated against the same named arm",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        667,
        671,
        "strictly increasing function of the witness set",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        673,
        677,
        "A lossless container choice contributes zero distinguishable classes",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        702,
        706,
        "baseline argument",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        819,
        823,
        "the intermediate-width axis carries 10 to 12 of the 24",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        862,
        868,
        "read at the declared width, always",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        878,
        882,
        "F2 is not in Q51",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        921,
        923,
        "The count is not a property of the design",
        True,
    ),
    (
        "140_mcsherry_the_strategy_set_derived_cold.md",
        926,
        927,
        "one shared arm",
        True,
    ),
    # deliberately wrong, both must FAIL
    ("INTENTS.md", 1, 5, "this sentence is not in the intent catalogue", False),
    (
        "139_muratori_the_strategy_set_derived_cold.md",
        1,
        5,
        "the firewall is hereby withdrawn",
        False,
    ),
]

fails = []
unexpected_pass = []
for f, a, b, needle, must in CITES:
    p = BASE / f
    if not p.exists():
        fails.append((f, a, b, needle, "FILE MISSING"))
        continue
    lines = p.read_text().splitlines()
    if b > len(lines):
        fails.append((f, a, b, needle, f"RANGE PAST EOF ({len(lines)} lines)"))
        continue
    chunk = norm("\n".join(lines[a - 1 : b]))
    present = norm(needle) in chunk
    if must and not present:
        fails.append((f, a, b, needle, "NOT FOUND IN RANGE"))
    if not must and present:
        unexpected_pass.append((f, a, b, needle))

total = len([c for c in CITES if c[4]])
print(f"citations checked (must pass): {total}")
print(f"failures: {len(fails)}")
for x in fails:
    print(f"  {x[0]}:{x[1]}-{x[2]}  {x[4]}  <- {x[3][:60]!r}")
print(f"deliberately wrong entries that must fail: {len([c for c in CITES if not c[4]])}")
print(f"  of those, wrongly found: {len(unexpected_pass)} (must be 0)")
for x in unexpected_pass:
    print(f"  {x[0]}:{x[1]}-{x[2]} <- {x[3][:60]!r}")

sys.exit(1 if fails or unexpected_pass else 0)
