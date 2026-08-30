#!/usr/bin/env python3
"""p6: open every file:line citation in file 43 and test its CONTENT.

`RULES.md:126-133` records that a reference which resolves is not a reference
that says what the citing file claims, and that one member found seven of its
own citations wrong this way.  This probe opens each cited range and tests it
against a word or phrase the claim requires, so a citation that resolves to the
wrong text fails rather than passing.

It also cross-checks its own table against the citations actually present in the
document, so a citation added to the prose and not to this table is reported
rather than silently unchecked.

Run from the panel directory.
"""

import re
import sys

DOC = "43_rompf_what_a_composition_is.md"

# (citation as written in the document, file to open, line range, required text)
CHECKS = [
    ("archive/CANON_CANDIDATE.md:3386-3387", "archive/CANON_CANDIDATE.md",
     (3386, 3387), "Compositions are public and bindable by"),
    ("archive/CANON_CANDIDATE.md:802-805", "archive/CANON_CANDIDATE.md",
     (802, 805), "pub struct Number<N: Numeral"),
    ("32:73", "32_op_arvo_adapts_to_the_cores_it_finds.md",
     (73, 73), "compose to bigger units than just numerals alone"),
    ("32:82-83", "32_op_arvo_adapts_to_the_cores_it_finds.md",
     (82, 83), "first-class concern"),
    ("00_brief.md:8", "00_brief.md",
     (8, 8), "named compositions over one format"),
    ("RULES.md:207-208", "RULES.md",
     (207, 208), "writes its file to disk early"),
    ("RULES.md:116-118", "RULES.md",
     (116, 118), "One instance of evidence is never enough"),
    ("RULES.md:224-228", "RULES.md",
     (224, 228), "mock/benches/"),
    ("RULES.md:20-27", "RULES.md",
     (20, 27), "ACK"),
    ("35:41-43", "35_mcsherry_what_the_layers_above_need_from_the_numeral.md",
     (41, 43), "a quantity that lives in the composition"),
    ("35:113-115", "35_mcsherry_what_the_layers_above_need_from_the_numeral.md",
     (113, 115), "taxonomy classifies a site by which"),
    ("35:820-822", "35_mcsherry_what_the_layers_above_need_from_the_numeral.md",
     (820, 822), "I did not read vehje at"),
    ("35:96-97", "35_mcsherry_what_the_layers_above_need_from_the_numeral.md",
     (96, 97), "The boundary is the runtime trip count and nothing else"),
    ("08:287-288", "08_knuth_what_the_one_format_concept_covers.md",
     (287, 288), "no per-datum type can express a constraint that holds between"),
    ("08:292-294", "08_knuth_what_the_one_format_concept_covers.md",
     (292, 294), "the workload arvo exists"),
    # the WRONG range this probe caught on its first run, kept in the table so
    # the document's account of the failure is itself checked: it opens
    # mid-sentence and does not contain the phrase the claim needed.
    ("08:283-290", "08_knuth_what_the_one_format_concept_covers.md",
     (283, 290), "dictionary encodings and run-length encodings"),
    ("RULES.md:126-133", "RULES.md",
     (126, 133), "Check your own citations before shipping"),
    ("08:282-290", "08_knuth_what_the_one_format_concept_covers.md",
     (282, 290), "Block floating point"),
    ("08:287-290", "08_knuth_what_the_one_format_concept_covers.md",
     (287, 290), "the layer it names is storage"),
    ("08:292", "08_knuth_what_the_one_format_concept_covers.md",
     (292, 292), "the workload arvo exists"),
    ("35:97", "35_mcsherry_what_the_layers_above_need_from_the_numeral.md",
     (97, 97), "The boundary is the runtime trip count and nothing else"),
    ("40:409", "40_leijen_what_the_axes_actually_are.md",
     (409, 409), "reduction shape"),
    ("42:370-376", "42_willsey_the_law_layer.md",
     (370, 376), "Only arvo knows its own axis values"),
    ("42:460-484", "42_willsey_the_law_layer.md",
     (460, 484), "equality-saturation engine"),
    ("42:299-334", "42_willsey_the_law_layer.md",
     (299, 334), "reachab"),
    ("24:734-747", "24_amin_the_seam_between_two_vocabularies.md",
     (734, 747), "One definition, projected, rather than two definitions"),
    ("08:330", "08_knuth_what_the_one_format_concept_covers.md",
     (330, 330), "buildable above"),
    ("08:306", "08_knuth_what_the_one_format_concept_covers.md",
     (306, 306), "the only case the `Bias` axis earns"),
    ("08:556-560", "08_knuth_what_the_one_format_concept_covers.md",
     (556, 560), "A representation is a numeral when a datum denotes one rational"),
    ("24:262", "24_amin_the_seam_between_two_vocabularies.md",
     (262, 262), "varepsilon"),
    ("40:47-53", "40_leijen_what_the_axes_actually_are.md",
     (47, 53), "observable"),
    ("42:33-38", "42_willsey_the_law_layer.md",
     (33, 38), "The law layer is a vocabulary of derived facts"),
    ("seed/SETTLED_laws.md:274", "seed/SETTLED_laws.md",
     (274, 274), "Adjustment"),
    ("DROPLIST.md:400", "DROPLIST.md",
     (400, 400), "compositions public and bindable"),
]

CITE_RE = re.compile(
    r"`((?:[A-Za-z0-9_/.]+\.md|\d\d)):(\d+)(?:-(\d+))?`"
)


def read_range(path, lo, hi):
    with open(path, encoding="utf-8", errors="replace") as fh:
        lines = fh.readlines()
    if lo < 1 or hi > len(lines):
        return None
    return "".join(lines[lo - 1:hi])


def main() -> int:
    failures = 0
    print("p6: content check on every cited range")
    print("=" * 74)
    for cite, path, (lo, hi), needle in CHECKS:
        text = read_range(path, lo, hi)
        if text is None:
            print(f"  FAIL  {cite:<38} out of range in {path}")
            failures += 1
            continue
        flat = " ".join(text.split())
        flat = flat.replace("> ", "")
        ok = needle in flat
        print(f"  {'ok  ' if ok else 'FAIL'}  {cite:<38} "
              f"{'contains' if ok else 'MISSING'} {needle!r}")
        if not ok:
            print(f"        got: {flat[:160]}")
            failures += 1

    print()
    print(f"  checked: {len(CHECKS)}   failures: {failures}")

    # cross-check: every file:line citation in the document is in the table
    print()
    print("cross-check: citations present in the document but absent here")
    with open(DOC, encoding="utf-8") as fh:
        doc = fh.read()
    present = set()
    for m in CITE_RE.finditer(doc):
        f, a, b = m.group(1), m.group(2), m.group(3)
        present.add(f"{f}:{a}-{b}" if b else f"{f}:{a}")
    tabled = {c for c, _, _, _ in CHECKS}
    missing = sorted(present - tabled)
    extra = sorted(tabled - present)
    if missing:
        for c in missing:
            print(f"  UNCHECKED  {c}")
        failures += len(missing)
    else:
        print("  none")
    if extra:
        print()
        print("cross-check: entries in the table no longer cited in the document")
        for c in extra:
            print(f"  STALE  {c}")

    print()
    print(f"TOTAL failures (content + unchecked): {failures}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
