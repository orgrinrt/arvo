#!/usr/bin/env python3
"""
p6. Opens every file:line this file cites and tests that the target CONTAINS the substring the
citation is being used for, rather than merely resolving. `RULES.md` records one member finding
seven of its own citations wrong this way and calls the instrument the cheapest correctness tool
the panel has. This is that instrument, rebuilt for this file's citations.

A reference that resolves is not a reference that says what is claimed, so every row below pairs
a location with the text the claim depends on.
"""

import os
import re
import sys

# Resolve relative to this file rather than the caller's working directory, so the check gives
# the same answer wherever it is run from. It did not, and reported all 37 citations missing.
PANEL = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")


def norm(s):
    """A citation's target is prose, and prose wraps. Compare on normalised text so a claim
    spanning a line break is not reported as absent, and so emphasis markers do not decide
    whether a citation resolves."""
    return re.sub(r"\s+", " ", s.replace("*", "").replace("`", "")).strip()

CHECKS = [
    # (file, first line, last line, substring the claim depends on)
    ("67_kiselyov_which_prefix_earns_the_word.md", 252, 252,
     "No crossing preserves operations at 100%"),
    ("67_kiselyov_which_prefix_earns_the_word.md", 143, 143,
     "Reduce(D, Q)"),
    ("67_kiselyov_which_prefix_earns_the_word.md", 123, 124,
     "It has a shape problem"),
    ("67_kiselyov_which_prefix_earns_the_word.md", 48, 59,
     "None of that appears in `INTENTS.md`"),
    ("67_kiselyov_which_prefix_earns_the_word.md", 236, 274,
     "192 of 256 operand pairs for addition"),
    ("67_kiselyov_which_prefix_earns_the_word.md", 236, 274,
     "with xor on 108 of 256 pairs and with min on 1 of 256"),
    ("67_kiselyov_which_prefix_earns_the_word.md", 564, 570,
     "are **three** crossings, not two"),
    ("67_kiselyov_which_prefix_earns_the_word.md", 634, 640,
     "A crossing may have the first and lack the second"),
    ("67_kiselyov_which_prefix_earns_the_word.md", 276, 276,
     "two law families are two consumer classes"),
    ("66_dolan_number_systems_derived_cold.md", 310, 345,
     "two separable questions rather than one"),
    ("66_dolan_number_systems_derived_cold.md", 327, 335,
     "This needs a rule, not"),
    ("65_knuth_number_systems_derived_cold.md", 185, 196,
     "keyed by role"),
    ("65_knuth_number_systems_derived_cold.md", 185, 196,
     "Cold's intent (I6) is a statement about the storage role"),
    ("65_knuth_number_systems_derived_cold.md", 533, 536,
     "possibly chain-extent as a"),
    ("65_knuth_number_systems_derived_cold.md", 240, 250,
     "the chain has its own compute representation"),
    ("68_leroy_what_the_pipeline_certifies.md", 283, 296,
     "stored bits are not self-describing"),
    ("68_leroy_what_the_pipeline_certifies.md", 347, 362,
     "Both hierarchies are value-centric"),
    ("68_leroy_what_the_pipeline_certifies.md", 196, 215,
     "long_running_const_eval"),
    ("63_spj_consolidation_the_format_concept.md", 205, 243,
     "The adaptation laws face the source"),
    ("63_spj_consolidation_the_format_concept.md", 616, 624,
     "an operation that fuses one invisibly"),
    ("63_spj_consolidation_the_format_concept.md", 692, 706,
     "the schedule is part of the function's meaning"),
    ("63_spj_consolidation_the_format_concept.md", 778, 784,
     "Q3, mixed-numeral addition"),
    ("63_spj_consolidation_the_format_concept.md", 665, 673,
     "the attack this file most wants made next"),
    ("63_spj_consolidation_the_format_concept.md", 676, 681,
     "No multiplicative structure survives a nonzero fraction width"),
    ("63_spj_consolidation_the_format_concept.md", 258, 262,
     "Redundant encodings are wholly unexamined"),
    ("OPTIONS.md", 1010, 1053, "the inclusion order's own predicate"),
    ("OPTIONS.md", 1010, 1053, "188 disagreements"),
    ("OPTIONS.md", 1053, 1053, "no second read has run"),
    ("OPTIONS.md", 1604, 1609, "a membership test that does not enumerate"),
    ("OPTIONS.md", 1611, 1617, "one instance decides nothing"),
    ("OPTIONS.md", 143, 161, "Is there a mixed-numeral addition?"),
    ("OPTIONS.md", 143, 161, "It exists only through an explicit conversion"),
    ("OPTIONS.md", 638, 640, "cross once, at literals, in one direction"),
    ("OPTIONS.md", 1626, 1631, "Is the role set closed"),
    ("DROPLIST.md", 106, 108, "section-retraction triple"),
    ("00_brief.md", 130, 163, "then validate, and erase"),
    ("INTENTS.md", 154, 160, "contracts for things that compose to bigger units"),
]

fails = []
for path, lo, hi, needle in CHECKS:
    try:
        text = open(f"{PANEL}/{path}", encoding="utf-8").read().splitlines()
    except OSError as e:
        fails.append((path, lo, hi, needle, f"cannot open: {e}"))
        continue
    if hi > len(text):
        fails.append((path, lo, hi, needle, f"file has only {len(text)} lines"))
        continue
    window = norm("\n".join(text[lo - 1:hi]))
    if norm(needle) not in window:
        whole = norm("\n".join(text))
        elsewhere = norm(needle) in whole
        fails.append((path, lo, hi, needle,
                      "not in the cited window; it IS elsewhere in the file"
                      if elsewhere else "not anywhere in the file"))

print(f"citations checked: {len(CHECKS)}")
print(f"failures:          {len(fails)}")
for f in fails:
    print()
    print(f"  FAIL {f[0]}:{f[1]}-{f[2]}")
    print(f"       looking for: {f[3]!r}")
    print(f"       {f[4]}")

sys.exit(1 if fails else 0)
