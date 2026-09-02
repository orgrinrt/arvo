#!/usr/bin/env python3
"""y5: open every file:line citation in 149 and test its CONTENT.

RULES.md:126 requires it. In my previous dispatch this instrument found three of my
own citations wrong, all three off by a couple of lines, all three of which would
have resolved and read as rigorous.

Two entries are deliberately wrong and must fail. And the normaliser strips markdown
before matching, because `145` z7 and `146` w2 both report the same class of false
failure: a quotation that is verbatim correct and unmatchable because the source's
own backticks and emphasis sit inside the quoted phrase. That is now five instances
of the class in this panel, so it is designed in rather than discovered again.
"""

import re
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent


def norm(s):
    out = []
    for line in s.splitlines():
        t = line.strip()
        if t.startswith(">"):
            t = t[1:].strip()
        out.append(t)
    j = " ".join(" ".join(out).split())
    j = j.replace("`", "").replace("**", "").replace("*", "")
    return " ".join(j.split())


# (file, first, last, substring that must be inside, must_pass)
CITES = [
    ("INTENTS.md", 51, 61, "the strategy set is not closed at exactly four", True),
    ("INTENTS.md", 214, 235, "We explicitly reject a universal solution", True),
    ("INTENTS.md", 317, 331, "We shouldn't police what kind of laws there are", True),
    ("INTENTS.md", 363, 383, "besides the point of the intent", True),
    ("RULES.md", 486, 541, "No finding in this panel states a result without stating the region", True),
    ("137_probes/g0_test_gate.out", 16, 16, "123 passed across 13 crates, 0 failed", True),
    # 146
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        420,
        427,
        "Under unsigned range policies that is every rounding position",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        416,
        416,
        "The proposition itself is unpredicated",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        557,
        560,
        "the claim is not the kind of thing a sweep establishes",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        632,
        632,
        "does not require reopening the rounding topic",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        609,
        610,
        "Nothing here edits it and nothing here asks",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        163,
        164,
        "reproduced the invariance half on its own committed",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        128,
        129,
        "third instance of a proof rather than a third measurement",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        111,
        119,
        "it does not earn the two-expert rung",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        45,
        56,
        "the two cold derivations are one instance wearing two hats",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        729,
        732,
        "A trailing apostrophe is a real part of an id here",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        222,
        223,
        "shape -> count is a well-defined function",
        True,
    ),
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        228,
        231,
        "replaced by B-prime",
        True,
    ),
    # 145
    (
        "145_leroy_formalising_the_strategy_object.md",
        336,
        339,
        "signedness = unsigned; overflow in {wrap, saturating}",
        True,
    ),
    (
        "145_leroy_formalising_the_strategy_object.md",
        345,
        348,
        "closure for the unsigned half",
        True,
    ),
    (
        "145_leroy_formalising_the_strategy_object.md",
        39,
        40,
        "thirteen release builds would take most of it",
        True,
    ),
    # 142
    (
        "142_muratori_reply_the_repair_was_dead_on_arrival.md",
        215,
        216,
        "Calling that a spelling is the same move",
        True,
    ),
    (
        "142_muratori_reply_the_repair_was_dead_on_arrival.md",
        225,
        228,
        "Ship floor as a rounding position and let the consumer select it",
        True,
    ),
    (
        "142_muratori_reply_the_repair_was_dead_on_arrival.md",
        122,
        126,
        "a re-derivation and a re-measurement, not a blind instance",
        True,
    ),
    (
        "142_muratori_reply_the_repair_was_dead_on_arrival.md",
        266,
        269,
        "The canon should also record, per mode, whether it is translation",
        True,
    ),
    # the probe source, which is where the signed-only claim is established
    (
        "142_probes/q2_equivariance_partitions_the_rounding_axis.rs",
        230,
        240,
        "wrap(rnd(a * b, f, m), w, true)",
        True,
    ),
    # deliberately wrong, both must FAIL
    (
        "146_leroy_the_canon_candidate_for_the_strategy_object.md",
        1,
        5,
        "the firewall is withdrawn",
        False,
    ),
    ("INTENTS.md", 1, 5, "nearest-half-even is not equivariant", False),
]

fails = []
unexpected = []
for f, a, b, needle, must in CITES:
    p = BASE / f
    if not p.exists():
        fails.append((f, a, b, needle, "FILE MISSING"))
        continue
    lines = p.read_text().splitlines()
    if b > len(lines):
        fails.append((f, a, b, needle, f"PAST EOF ({len(lines)} lines)"))
        continue
    chunk = norm("\n".join(lines[a - 1 : b]))
    present = norm(needle) in chunk
    if must and not present:
        fails.append((f, a, b, needle, "NOT IN RANGE"))
    if not must and present:
        unexpected.append((f, a, b, needle))

must_pass = len([c for c in CITES if c[4]])
print(f"citations checked (must pass): {must_pass}")
print(f"failures: {len(fails)}")
for x in fails:
    print(f"  {x[0]}:{x[1]}-{x[2]}  {x[4]}  <- {x[3][:58]!r}")
print(f"deliberately wrong entries: {len([c for c in CITES if not c[4]])}")
print(f"  of those, wrongly found: {len(unexpected)} (must be 0)")
for x in unexpected:
    print(f"  {x[0]}:{x[1]}-{x[2]} <- {x[3][:58]!r}")

# two prose claims in 149 that a grep settles, checked here so they are not recalled
c146 = (BASE / "146_leroy_the_canon_candidate_for_the_strategy_object.md").read_text()
c145 = (BASE / "145_leroy_formalising_the_strategy_object.md").read_text()
print()
print(f"claim: 'tolerance' absent from 146 and 145 -> 146:{c146.count('tolerance')} 145:{c145.count('tolerance')} (both must be 0)")
print(f"claim: 'congruence' occurrences in 146 -> {c146.count('congruence')}")
for i, line in enumerate(c146.splitlines(), 1):
    if "congruence" in line:
        print(f"    line {i}")

sys.exit(1 if fails or unexpected else 0)
