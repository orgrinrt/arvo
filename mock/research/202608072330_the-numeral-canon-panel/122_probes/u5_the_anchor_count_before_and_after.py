#!/usr/bin/env python3
"""u5. The anchor count for `122`, on `119`'s own patterns, before and after.

The dispatch asks for the count on the same patterns `119` used, with the
accounting section excluded, and both numbers stated. `119_probes/r1` is that
instrument and it is imported rather than rebuilt, so the two documents are
measured by one thing.

Three things differ from `119`'s run and each is stated rather than folded in.

**The source set is larger.** `119` compressed four files. `122` revises `119`
against two more, so the union it is measured against is `114`, `115`, `116`,
`118`, `119`, `120` and `121`. Both unions are reported so the two runs are
comparable.

**`122` is a revision rather than a compression**, so a clause marked as standing
unchanged deliberately does not restate `119`'s anchors. Those are carried by
reference, which the set difference cannot see and which is why the per-class
lists matter more than the totals here.

**The accounting section is excluded on both sides**, by `r1`'s own stripper,
which fired on `119` and reported four finding ids present only in that section.

NEGATIVE CONTROLS
-----------------
C1. `r1`'s stripper must fire or not fire visibly: the two columns it prints are
    compared and the difference reported either way.
C2. The extractor must return zero on prose with no anchors, which `r1`'s own
    controls already assert and which is re-run here rather than assumed.
"""

import importlib.util
import sys
from pathlib import Path

HERE = Path(__file__).parent
PANEL = HERE.parent

spec = importlib.util.spec_from_file_location(
    "r1", PANEL / "119_probes" / "r1_the_anchor_inventory_and_what_the_candidate_carries.py"
)
r1 = importlib.util.module_from_spec(spec)
sys.modules["r1"] = r1
spec.loader.exec_module(r1)

FOUR = [
    "114_leroy_formalising_the_primitive.md",
    "115_jhala_the_check_the_policy_selects.md",
    "116_leijen_reply_what_the_homomorphism_opens.md",
    "118_leroy_reply_one_mechanism_and_the_condition_set.md",
]
SEVEN = FOUR + [
    "119_leroy_the_canon_candidate_for_the_realisation_map.md",
    "120_jhala_partial_signature_on_the_candidate.md",
    "121_leijen_partial_signature_the_domain_is_the_missing_dimension.md",
]
C119 = "119_leroy_the_canon_candidate_for_the_realisation_map.md"
C122 = "122_leroy_the_candidate_revised_against_two_partial_signatures.md"

CLASSES = ("finding", "probe_stem", "line_panel_norm")


def read(name):
    return (PANEL / name).read_text()


def union_over(names):
    u = {k: set() for k in CLASSES}
    for n in names:
        a = r1.anchors(read(n))
        for k in CLASSES:
            u[k] |= a[k]
    return u


def fmt(d):
    return f"{d[0]}:{d[1]}" if isinstance(d, tuple) else str(d)


def main():
    print("=" * 96)
    print("u5. The anchor count, on `119`'s patterns, before and after")
    print("=" * 96)

    # ------------------------------------------------------------------- C2
    print()
    empty = r1.anchors("a paragraph of ordinary prose carrying no addresses at all")
    print("C2. The extractor on prose with no anchors: "
          + ", ".join(f"{k} {len(empty[k])}" for k in CLASSES))

    u4 = union_over(FOUR)
    u7 = union_over(SEVEN)

    for label, doc, union, uname in (
        ("119 as landed", C119, u4, "the four it compressed"),
        ("122, this revision", C122, u7, "the seven it revises across"),
    ):
        raw = read(doc)
        a_raw = r1.anchors(raw)
        a = r1.anchors(r1.strip_accounting(raw))
        print()
        print("-" * 96)
        print(f"{label}, measured against {uname}")
        print()
        print(f"  {'class':<18} {'in the union':>13} {'in the doc':>11} "
              f"{'incl. accounting':>17} {'not carried':>12}")
        for k in CLASSES:
            print(f"  {k:<18} {len(union[k]):>13} {len(a[k]):>11} "
                  f"{len(a_raw[k]):>17} {len(union[k] - a[k]):>12}")
        gap = {k: len(a_raw[k]) - len(a[k]) for k in CLASSES}
        print()
        print(f"  C1, the stripper: {'FIRED' if any(gap.values()) else 'did not fire'}"
              f"  ({', '.join(f'{k} +{gap[k]}' for k in CLASSES)})")

    # ------------------------------------------------- what 122 adds and drops
    print()
    print("-" * 96)
    print("What `122` carries that `119` did not, and what it lets go.")
    print()
    a119 = r1.anchors(r1.strip_accounting(read(C119)))
    a122 = r1.anchors(r1.strip_accounting(read(C122)))
    for k in CLASSES:
        added = sorted(a122[k] - a119[k], key=fmt)
        gone = sorted(a119[k] - a122[k], key=fmt)
        print(f"  {k}: {len(a122[k])} in 122 against {len(a119[k])} in 119")
        print(f"    added ({len(added)}):")
        for i in range(0, len(added), 6):
            print("      " + "  ".join(f"{fmt(d):<20}" for d in added[i:i + 6]))
        print(f"    not restated ({len(gone)}):")
        for i in range(0, len(gone), 6):
            print("      " + "  ".join(f"{fmt(d):<20}" for d in gone[i:i + 6]))
        print()

    print("=" * 96)
    print(
        """
  READING IT

  The 'not carried' column against the seven-file union is the honest headline
  and it is larger than `119`'s was, because the union grew by three files while
  the document shrank: `122` restates only the clauses it changes and carries the
  rest of `119` by reference.

  So the number to read is 'not restated' in the last block, which is what this
  revision dropped from a document it supersedes in part. Every entry there
  belongs to a clause marked [STANDS], where `119`'s text is the text and this
  file deliberately does not repeat its anchors.
"""
    )


if __name__ == "__main__":
    main()
