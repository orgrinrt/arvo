#!/usr/bin/env python3
"""w1. Two bookkeeping checks the candidate rests on.

PART ONE: the anchors.
`119_probes/r1` is the instrument, imported rather than rebuilt, so this
candidate and the preceding one are measured by one thing on one pattern. The
set difference is the check and the count is only the headline, per
`a-compression-is-checked-by-someone-else.md`; and any section headed as anchor
accounting is stripped before computing, because a candidate that lists the
anchors it dropped makes them present in its own text and disables the check.
The preceding candidate hit exactly that and its guard caught it, so the guard
is kept.

PART TWO: the commit ordering, which the ledger cites as evidence of blindness.
`128` says five convergences between `126` and `125` phase one "were reached
blind by both of us, with the commit ordering as the audit trail". A ledger
entry resting on an audit trail should show it rather than cite it, and should
say what it does and does not establish. It is checked here so the ledger can
state the weaker true thing rather than the stronger convenient one.

PREDICTIONS
-----------
P1. Both phase ones are committed as units, and `125`'s predictions land before
    its own probes, which is the within-file half of the blindness claim.
P2. The commit ordering does NOT by itself establish that the later-committed
    file had not read the earlier one, because the two commits are minutes
    apart. If so the ledger must rest blindness on the files' own coverage
    statements as well, and say so.

CONTROLS
--------
C1. The anchor extractor returns zero on prose with no addresses.
C2. The stripper's effect is reported either way, so a reader can see whether it
    fired rather than trusting that it would have.
"""

import importlib.util
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent
PANEL = HERE.parent
REPO = PANEL.parent.parent.parent

spec = importlib.util.spec_from_file_location(
    "r1", PANEL / "119_probes" / "r1_the_anchor_inventory_and_what_the_candidate_carries.py"
)
r1 = importlib.util.module_from_spec(spec)
sys.modules["r1"] = r1
spec.loader.exec_module(r1)

SOURCES = [
    "125_knuth_rounding_cold_derivation.md",
    "126_wronski_what_rounding_actually_selects.md",
    "127_dolan_attacking_the_convergence.md",
    "128_knuth_reply_the_boundary_moves_to_the_coupling.md",
    "129_wronski_reply_the_fork_is_decorrelation_not_determinism.md",
    "130_dolan_reply_one_axis_two_keyings.md",
    "131_leroy_formalising_the_rounding_axis.md",
]
CANDIDATE = "132_leroy_the_canon_candidate_for_the_rounding_axis.md"
CLASSES = ("finding", "probe_stem", "line_panel_norm", "theorem")

# `r1`'s finding pattern was written for a topic that numbered everything
# `F<file>-<n>`. This topic names half its results `T1` through `T9`, which that
# pattern does not see, so counting on `r1` alone would silently drop the class
# the check exists to protect. The theorem class is added here rather than by
# editing `r1`, which belongs to another file.
import re as _re
_RE_THM = _re.compile(r"\bT\d{1,2}\b")

# `r1`'s probe pattern also predates this stretch of the panel: it matches stems
# beginning p, q, r or s, and the probes here are named v and w. Counting on it
# alone reported zero probe references in a file that cites four, which is the
# same class of silent drop the theorem gap is. Widened to the whole letter
# range the panel has used.
_RE_PROBE = _re.compile(
    r"\b(?:\d+_probes/)?[a-z]\d+[a-z]?_[A-Za-z0-9_]*\.(?:py|txt|rs|s)\b"
    r"|\b\d+_probes/[a-z]\d+[a-z]?\b"
)


def _stems(refs):
    out = set()
    for r in refs:
        r = r.split("/")[-1]
        m = _re.match(r"([a-z]\d+[a-z]?)_", r)
        out.add(m.group(1) if m else r)
    return out


def anchors(text):
    a = dict(r1.anchors(text))
    a["theorem"] = set(_RE_THM.findall(text))
    a["probe_stem"] = _stems(set(_RE_PROBE.findall(text)))
    return a


def fmt(d):
    return f"{d[0]}:{d[1]}" if isinstance(d, tuple) else str(d)


def main():
    print("=" * 92)
    print("w1. The anchor count, and the commit ordering the ledger cites")
    print("=" * 92)

    # -------------------------------------------------------------- PART ONE
    print()
    print("PART ONE. Anchors, on `119_probes/r1`'s patterns.")
    print()
    empty = anchors("ordinary prose carrying no addresses at all")
    print("  C1, extractor on prose with no anchors: "
          + ", ".join(f"{k} {len(empty[k])}" for k in CLASSES))
    print()
    union = {k: set() for k in CLASSES}
    print(f"  {'source':<52} {'findings':>9} {'probes':>7} {'lines':>6} {'theorems':>8}")
    for name in SOURCES:
        a = anchors((PANEL / name).read_text())
        for k in CLASSES:
            union[k] |= a[k]
        print(f"  {name:<52} {len(a['finding']):>9} {len(a['probe_stem']):>7} "
              f"{len(a['line_panel_norm']):>6} {len(a['theorem']):>8}")
    print()
    print(f"  {'UNION across the seven':<52} {len(union['finding']):>9} "
          f"{len(union['probe_stem']):>7} {len(union['line_panel_norm']):>6} "
          f"{len(union['theorem']):>8}")

    path = PANEL / CANDIDATE
    if not path.exists():
        print()
        print(f"  {CANDIDATE} does not exist yet. Inventory pass only; rerun after.")
    else:
        raw = path.read_text()
        a_raw = anchors(raw)
        a = anchors(r1.strip_accounting(raw))
        print()
        print(f"  {'class':<18} {'in the union':>13} {'in 132':>8} "
              f"{'incl. accounting':>17} {'not carried':>12}")
        for k in CLASSES:
            print(f"  {k:<18} {len(union[k]):>13} {len(a[k]):>8} "
                  f"{len(a_raw[k]):>17} {len(union[k] - a[k]):>12}")
        gap = {k: len(a_raw[k]) - len(a[k]) for k in CLASSES}
        print()
        print(f"  C2, the stripper: {'FIRED' if any(gap.values()) else 'did not fire'}"
              f"  ({', '.join(f'{k} +{gap[k]}' for k in CLASSES)})")
        print()
        for k in CLASSES:
            drop = sorted(union[k] - a[k], key=fmt)
            keep = sorted(union[k] & a[k], key=fmt)
            print(f"  {k}: {len(keep)} carried, {len(drop)} not")
            print(f"    not carried:")
            for i in range(0, len(drop), 6):
                print("      " + "  ".join(f"{fmt(d):<20}" for d in drop[i:i + 6]))
            print()

    # -------------------------------------------------------------- PART TWO
    print("-" * 92)
    print("PART TWO. The commit ordering `128` cites as the audit trail.")
    print()
    r = subprocess.run(
        ["git", "log", "--format=%h %ad %s", "--date=format:%m-%d %H:%M",
         "--diff-filter=A", "--name-only", "--reverse", "--",
         "mock/research/202608072330_the-numeral-canon-panel/125_probes",
         "mock/research/202608072330_the-numeral-canon-panel/126_probes",
         "mock/research/202608072330_the-numeral-canon-panel/125_knuth_rounding_cold_derivation.md",
         "mock/research/202608072330_the-numeral-canon-panel/126_wronski_what_rounding_actually_selects.md"],
        cwd=REPO, capture_output=True, text=True,
    )
    lines = [l for l in r.stdout.splitlines() if l and not l.startswith("mock/")]
    for l in lines:
        print("  " + l)
    print()
    print("  P1 holds if `125`'s derivation commit precedes its own probe commits,")
    print("  which is the within-file half of blindness: predictions before results.")
    print()
    print("  P2 is the half that does NOT hold. The two phase ones are committed")
    print("  minutes apart, and an ordering alone cannot establish that the later")
    print("  author had not read the earlier file in between. So the ledger rests")
    print("  blindness on the ordering AND on each file's own coverage statement,")
    print("  and says so rather than citing the ordering as though it settled it.")

    print()
    print("=" * 92)


if __name__ == "__main__":
    main()
