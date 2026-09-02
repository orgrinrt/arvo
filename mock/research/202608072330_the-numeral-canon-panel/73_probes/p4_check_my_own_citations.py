#!/usr/bin/env python3
"""p4. Open every file:line this file cites and test its CONTENT, not its
resolution.

The instrument is `71_probes/p6`'s, adopted rather than reinvented, and the
reason to run it is `RULES.md:126-133`: one member found seven of its own
citations wrong by opening them. A reference that resolves is not a reference
that says what the citing sentence claims.

Each row below is (citation as written in file 73, target path, line range,
substring the claim depends on). The substring is matched against the joined
lines with internal whitespace normalised, because prose wraps and a raw
line-by-line comparison produced three false failures for `71_probes/p6`.

Run from the probe directory or the panel directory; paths are resolved
relative to the panel directory.
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
REPO = os.path.abspath(os.path.join(PANEL, "..", "..", ".."))

F = {
    "63": "63_spj_consolidation_the_format_concept.md",
    "65": "65_knuth_number_systems_derived_cold.md",
    "66": "66_dolan_number_systems_derived_cold.md",
    "67": "67_kiselyov_which_prefix_earns_the_word.md",
    "68": "68_leroy_what_the_pipeline_certifies.md",
    "69": "69_persona_checkpoint_six.md",
    "70": "70_lattner_levels_are_clients_not_facts.md",
    "71": "71_orchard_what_crosses_between_two_systems.md",
    "72": "72_kiselyov_reply.md",
    "OPTIONS": "OPTIONS.md",
    "DROPLIST": "DROPLIST.md",
    "RULES": "RULES.md",
    "INTENTS": "INTENTS.md",
}

# (citation, file key or repo-relative path, lo, hi, required substring)
ROWS = [
    ("71:464-469", "71", 464, 469, "membership test that does not enumerate"),
    ("71:466-469", "71", 466, 469, "two law verdicts are decidable"),
    ("71:675-679", "71", 675, 679, "A system exposes, for the purpose of crossing"),
    ("71:679", "71", 679, 679, "cannot be crossed into and composes with nothing"),
    ("71:341-344", "71", 341, 344, "both programs are right programs"),
    ("71:283", "71", 281, 283, "second reason to run it"),
    ("71:757-763", "71", 757, 763, "expensive to get wrong"),
    ("72:227-230", "72", 227, 230, "Q20 membership test: accepted"),
    ("72:232-255", "72", 232, 255, "this is where I think `71` over-reads"),
    ("72:236-247", "72", 236, 247, "OPTIONS.md:991-994"),
    ("67:440-446", "67", 440, 446, "is D's operation family fixed at (+, x), or is it a parameter?"),
    ("67:641-645", "67", 641, 645, "closure bounds the adaptation layer's subject"),
    ("67:48-59", "67", 48, 59, "None of that appears in `INTENTS.md`"),
    ("67:571-584", "67", 571, 584, "is the ambient domain's operation family fixed?"),
    ("63:625-631", "63", 625, 631, "A format is identified by its ambient domain and its representable set"),
    ("63:659-673", "63", 659, 673, "provisionally passes"),
    ("63:665-673", "63", 665, 673, "nobody has tried to break the frame"),
    ("63:219-222", "63", 219, 222, "All four combinations are inhabited"),
    ("63:216-219", "63", 216, 219, "distance-minimising"),
    ("63:230", "63", 230, 230, "952 associativity failures"),
    ("63:233-235", "63", 233, 235, "failed to break the ladder"),
    ("63:280-281", "63", 280, 281, "compositions over formats"),
    ("63:676-681", "63", 676, 681, "No multiplicative structure survives a nonzero fraction width"),
    ("65:65-66", "65", 65, 66, "number systems that are not about magnitude at all"),
    ("65:80-86", "65", 80, 86, "change the container width or field layout"),
    ("65:258-259", "65", 258, 259, "I3 demands Warm behave as native Rust primitives behave"),
    ("65:188-189", "65", 188, 189, "a redundant intermediate"),
    ("66:286-288", "66", 286, 288, "ordered value sets with a notion of magnitude"),
    ("66:455-457", "66", 455, 457, "signed\nsaturating multiplication at width 4"),
    ("68:314-337", "68", 314, 337, "ingest predicate"),
    ("68:335-337", "68", 335, 337, "writable as a pure function of (type parameters, bits)"),
    ("68:196-211", "68", 196, 211, "long_running_const_eval"),
    ("68:158-163", "68", 158, 163, "nearly a language tautology"),
    ("69:37-44", "69", 37, 44, "none of the following appears anywhere in"),
    ("69:125-135", "69", 125, 135, "That sentence as first written overclaimed"),
    ("69:133-134", "69", 133, 134, "removed outright"),
    ("70:369-384", "70", 369, 384, "the ownership key"),
    ("OPTIONS:1604-1609", "OPTIONS", 1604, 1609, "Is the inventory of number systems open or closed"),
    ("OPTIONS:1608-1609", "OPTIONS", 1608, 1609, "membership test that does not enumerate"),
    ("OPTIONS:1611-1617", "OPTIONS", 1611, 1617, "not about magnitude at all"),
    ("OPTIONS:991-994", "OPTIONS", 991, 994, "widen compute past storage"),
    ("DROPLIST:106-108", "DROPLIST", 106, 108, "section-retraction triple"),
    # op's own words, quoted in sections 4, 5 and 12
    ("INTENTS I11", "INTENTS", 154, 160,
     "our main selling point are the algo crates that hilavitkutin, vehje, pretty much"),
    ("INTENTS I3", "INTENTS", 67, 77,
     "It should behave like native primitives in regular old rust would"),
    # outside the panel directory, for section 9's second report
    ("cookbook.md:136-137", "REPO:.claude/rules/cookbook.md", 136, 137,
     "arvo-spectral::Laplacian"),
    ("cookbook.md.tmpl:119", "REPO:mock/agent/rules/cookbook.md.tmpl", 119, 119,
     "arvo-graph`: `topo_sort`"),
]


def norm(s):
    # strip leading blockquote markers before collapsing whitespace. Without this
    # a quotation of op's own words fails whenever the quote wraps a line, because
    # the "> " of the next line lands inside the substring. That is the second
    # defect this instrument found in its own author's citations and, like the
    # first, it was a defect in the checker rather than in the citation.
    s = re.sub(r"(?m)^[ \t]*>[ \t]?", "", s)
    return re.sub(r"\s+", " ", s).strip()


def resolve(key):
    if key.startswith("REPO:"):
        return os.path.join(REPO, key[5:])
    return os.path.join(PANEL, F[key])


fails = []
missing = []
for cite, key, lo, hi, needle in ROWS:
    path = resolve(key)
    if not os.path.exists(path):
        missing.append((cite, path))
        continue
    with open(path, encoding="utf-8") as fh:
        lines = fh.readlines()
    if hi > len(lines):
        fails.append((cite, "line %d past end of file (%d lines)" % (hi, len(lines))))
        continue
    chunk = norm("".join(lines[lo - 1 : hi]))
    if norm(needle) not in chunk:
        fails.append((cite, "substring absent: %r" % needle[:60]))

print("citations checked: %d" % len(ROWS))
print("files missing:     %d" % len(missing))
print("failures:          %d" % len(fails))
print()
for cite, path in missing:
    print("  MISSING FILE  %-22s %s" % (cite, path))
for cite, why in fails:
    print("  FAILED        %-22s %s" % (cite, why))

if fails or missing:
    sys.exit(1)
print("every citation opened, and every target contains the text the claim depends on.")
