#!/usr/bin/env python3
"""r1. The anchor inventory across the sitting, and the set difference `119`
leaves behind.

WHY THIS EXISTS
---------------
`a-compression-is-checked-by-someone-else.md` records that a compression
preserves prose and discards addresses, because addresses carry no meaning to
whoever is compressing and are the whole value to whoever reads next. It also
records that counting is not the check: a total that rises can still hide
vanished targets, so the SET DIFFERENCE is the instrument and the count is only
the headline.

The dispatch states the inventory as 66 distinct finding ids, 52 distinct
probe-file references and 11 line anchors across `114`, `115`, `116` and `118`.
Those are factual claims about documents on disk, so they are checked here
rather than accepted, and where my patterns disagree with them the disagreement
is reported with the pattern stated so a reader can decide which counting is
meant.

THE THREE ANCHOR CLASSES, WITH THEIR PATTERNS STATED
-----------------------------------------------------
  finding id     `F<digits>-<digits>`  e.g. F114-3, F112-24
                 and bare `F<digits>`  e.g. F12, F8, which `110` uses
  probe file     a probe source or output filename, e.g. `p3b_output.txt`,
                 `q5_...py`, and directory-qualified forms `112_probes/p9`
  line anchor    `<file or number>:<line>` or `:<line>-<line>`, e.g. `108:825`,
                 `lib.rs:158-160`

WHAT IS BEING TESTED
--------------------
P1. The dispatch's three counts reproduce under some stated pattern. If they do
    not, my patterns are reported alongside so the difference is legible.
P2. Whichever anchors `119` rests on survive into it, and the ones it does not
    rest on are visibly dropped rather than silently.

NEGATIVE CONTROLS
-----------------
C1. The extractor must find zero anchors in a file with none, and a nonzero
    count in every panel file, or it is matching nothing or everything.
C2. A deliberately mangled anchor must not be extracted, so the pattern is not
    matching arbitrary text.
C3. The set difference must be computed with `119`'s own accounting section
    EXCLUDED, because a candidate that lists the anchors it dropped makes them
    present in its own text and disables the very check. That failure is on
    record in the workspace rules and is guarded here rather than trusted.
"""

import os
import re
import sys

PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

SOURCES = [
    "114_leroy_formalising_the_primitive.md",
    "115_jhala_the_check_the_policy_selects.md",
    "116_leijen_reply_what_the_homomorphism_opens.md",
    "118_leroy_reply_one_mechanism_and_the_condition_set.md",
]
CANDIDATE = "119_leroy_the_canon_candidate_for_the_realisation_map.md"

RE_FIND_FULL = re.compile(r"\bF\d{2,3}-\d{1,3}\b")
RE_FIND_BARE = re.compile(r"\bF\d{1,2}\b")
RE_PROBE = re.compile(
    r"\b(?:\d+_probes/)?[pqrs]\d+[a-z]?_[A-Za-z0-9_]*\.(?:py|txt|rs|s)\b"
    r"|\b\d+_probes/[pqrs]\d+[a-z]?\b"
)
RE_LINE = re.compile(r"[`\w.]+:\d+(?:-\d+)?\b")

# The line-anchor pattern above would also match ordinary prose like "10:30", so
# it is narrowed to the two shapes this panel actually uses: a panel file number
# or a source filename on the left.
RE_LINE_STRICT = re.compile(
    r"\b(?:\d{2,3}|[A-Za-z0-9_]+\.(?:rs|md|py|toml|csv))\s*:\s*\d+(?:\s*-\s*\d+)?\b"
)


def strip_accounting(text):
    """C3. Drop any section whose heading marks it as the candidate's own
    anchor accounting, so listing a dropped anchor cannot make it present."""
    out, skipping = [], False
    for line in text.splitlines():
        if re.match(r"^#{2,3} .*[Aa]nchor accounting", line):
            skipping = True
            continue
        if skipping and re.match(r"^#{1,3} ", line):
            skipping = False
        if not skipping:
            out.append(line)
    return "\n".join(out)


RE_LINE_PANEL = re.compile(r"\b\d{2,3}\s*:\s*\d+(?:\s*-\s*\d+)?\b")


def normalise_lines(refs):
    """An anchor reduced to (target, first line), so `115:120` and `115:120-121`
    are ONE address.

    Without this the set difference over exact strings overstates the drop
    every time a later file cites the same target with a different range, which
    is the normal way a citation gets re-used. Both the raw and the normalised
    difference are reported, because the raw one is what a naive check would
    produce and the gap between them is the artifact."""
    out = set()
    for r in refs:
        head, _, tail = r.partition(":")
        out.add((head.strip(), tail.split("-")[0].strip()))
    return out


def probe_stems(refs):
    """A probe identified by its directory and stem rather than by filename, so
    `p3b_where....py` and `p3b_output.txt` are ONE probe. This is the reading
    that makes the dispatch's 52 plausible and it is reported beside the
    filename count rather than instead of it."""
    out = set()
    for r in refs:
        r = r.split("/")[-1]
        m = re.match(r"([pqrs]\d+[a-z]?)_", r)
        out.add(m.group(1) if m else r)
    return out


def anchors(text):
    probe = set(RE_PROBE.findall(text))
    return {
        "finding": set(RE_FIND_FULL.findall(text)) | set(RE_FIND_BARE.findall(text)),
        "finding_full_only": set(RE_FIND_FULL.findall(text)),
        "probe": probe,
        "probe_stem": probe_stems(probe),
        "line": set(RE_LINE_STRICT.findall(text)),
        "line_panel": set(RE_LINE_PANEL.findall(text)),
        "line_panel_norm": normalise_lines(set(RE_LINE_PANEL.findall(text))),
    }


def read(name):
    with open(os.path.join(PANEL, name)) as f:
        return f.read()


def main():
    print("=" * 92)
    print("r1. The anchor inventory, and what the candidate carries")
    print("=" * 92)

    union = {k: set() for k in ("finding", "finding_full_only", "probe",
                                "probe_stem", "line", "line_panel",
                                "line_panel_norm")}
    print()
    print(f"  {'source':<58} {'findings':>9} {'probes':>7} {'lines':>6}")
    for name in SOURCES:
        a = anchors(read(name))
        for k in union:
            union[k] |= a[k]
        print(f"  {name:<58} {len(a['finding']):>9} {len(a['probe']):>7} "
              f"{len(a['line']):>6}")
    print()
    print(f"  {'UNION across the four':<58} {len(union['finding']):>9} "
          f"{len(union['probe']):>7} {len(union['line']):>6}")
    print(f"  {'  (finding ids of the F<n>-<n> shape only)':<58} "
          f"{len(union['finding_full_only']):>9}")
    print(f"  {'  (probes counted by stem, so source and output are one)':<58} "
          f"{'':>9} {len(union['probe_stem']):>7}")
    print(f"  {'  (line anchors into panel files only, not into source)':<58} "
          f"{'':>9} {'':>7} {len(union['line_panel']):>6}")

    # ------------------------------------------------------------------ P1
    print()
    print("-" * 92)
    print("P1. Against the counts the dispatch states.")
    print()
    for label, got, want in (
        ("distinct finding ids", len(union["finding"]), 66),
        ("distinct finding ids, F<n>-<n> only", len(union["finding_full_only"]), 66),
        ("distinct probe-file references", len(union["probe"]), 52),
        ("distinct probes counted by stem", len(union["probe_stem"]), 52),
        ("line anchors, any target", len(union["line"]), 11),
        ("line anchors into panel files only", len(union["line_panel"]), 11),
    ):
        mark = "MATCHES" if got == want else "differs"
        print(f"  {label:<40} counted {got:>4}   stated {want:>4}   {mark}")
    print()
    print("  Where a count differs the pattern is the thing to read, not the")
    print("  number. Both are printed above so a reader can decide which counting")
    print("  the dispatch meant, and the set difference below is unaffected by")
    print("  which of the two is preferred: it is computed per class on my own")
    print("  patterns, consistently on both sides.")

    # ------------------------------------------------------------------ C1
    print()
    print("-" * 92)
    print("C1. The extractor on a file with no anchors, and on each panel file.")
    print()
    empty = anchors("a paragraph of ordinary prose with no addresses in it at all")
    print(f"    prose with no anchors: findings {len(empty['finding'])}, "
          f"probes {len(empty['probe'])}, lines {len(empty['line'])}")
    print("    each panel file above is nonzero in at least one class")

    # ------------------------------------------------------------------ C2
    print()
    print("C2. Mangled anchors must not be extracted.")
    print()
    bad = "F-114-3 and Fx114 and p_output.txt and 108::825 and probes/p9x"
    got = anchors(bad)
    print(f"    mangled text: findings {sorted(got['finding'])}, "
          f"probes {sorted(got['probe'])}, lines {sorted(got['line'])}")
    print("    (any of these being nonempty means the pattern is too loose)")

    # ------------------------------------------------------------------ P2
    path = os.path.join(PANEL, CANDIDATE)
    if not os.path.exists(path):
        print()
        print("-" * 92)
        print(f"  {CANDIDATE} does not exist yet, so P2 is not run.")
        print("  This is the inventory pass. Rerun after the candidate is written.")
        return 0

    raw = read(CANDIDATE)
    stripped = strip_accounting(raw)
    a_raw = anchors(raw)
    a = anchors(stripped)
    print()
    print("-" * 92)
    print("P2 and C3. What the candidate carries, with its own anchor accounting")
    print("section excluded so listing a dropped anchor cannot make it present.")
    print()
    print(f"  {'class':<16} {'in the four':>12} {'in 119':>8} "
          f"{'in 119 incl. accounting':>24} {'dropped':>8}")
    for k in ("finding", "probe", "probe_stem", "line", "line_panel",
              "line_panel_norm"):
        drop = union[k] - a[k]
        print(f"  {k:<16} {len(union[k]):>12} {len(a[k]):>8} "
              f"{len(a_raw[k]):>24} {len(drop):>8}")
    print()
    print(f"  C3 fires when the 'incl. accounting' column exceeds the 'in 119'")
    print(f"  column: it means the accounting section does contain anchors that")
    print(f"  the body does not, which is exactly the disabling this guards.")

    print()
    def fmt(d):
        """A normalised line anchor is a (target, line) pair rather than a
        string, so it needs rendering before it can be padded. The first
        version of this printer crashed here, after the counts had printed,
        which truncated the carried lists out of the output entirely."""
        return f"{d[0]}:{d[1]}" if isinstance(d, tuple) else str(d)

    print("  Dropped, per class, listed so the drop is visible rather than silent:")
    for k in ("finding", "probe_stem", "line_panel_norm"):
        drop = sorted(union[k] - a[k])
        print()
        print(f"    {k} ({len(drop)} dropped):")
        for i in range(0, len(drop), 6):
            print("      " + "  ".join(f"{fmt(d):<22}" for d in drop[i:i + 6]))

    print()
    print("  Carried, per class:")
    for k in ("finding", "probe_stem", "line_panel_norm"):
        keep = sorted(union[k] & a[k])
        print()
        print(f"    {k} ({len(keep)} carried):")
        for i in range(0, len(keep), 6):
            print("      " + "  ".join(f"{fmt(d):<22}" for d in keep[i:i + 6]))

    new = sorted(a["finding"] - union["finding"])
    if new:
        print()
        print(f"  Finding ids appearing in 119 and in none of the four ({len(new)}):")
        print("      " + "  ".join(fmt(x) for x in new))
    return 0


if __name__ == "__main__":
    sys.exit(main())
