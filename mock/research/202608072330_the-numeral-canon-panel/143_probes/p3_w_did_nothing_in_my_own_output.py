"""p3 (143): W does nothing, and the evidence was already in my own committed output.

`141` section 5.5 reports that neither cold derivation noticed the integer width
does not move the class count. It cites `139`'s table. It does not cite mine,
and mine contains the same pattern: `140_probes/p1_out.txt` sweeps three widths
and every (F, operation) group that appears at more than one W reports an
identical count.

This does not re-run anything. It parses the output file I committed at
`a60f1a47` and asks whether any group disagrees across W. The point is that the
concession is checkable rather than eyeballed, and that the miss was mine to
catch from an artifact I already had.

THE CASE THAT MUST FAIL. A checker that reports "no disagreement" because it
never grouped anything, or because its parse produced nothing, looks identical
to a real negative. So it carries two controls and exits non-zero if either
misbehaves:
  (a) a POSITIVE control: an injected row that disagrees across W must be
      reported as a disagreement;
  (b) a NON-VACUITY control: the number of groups actually spanning two or more
      widths must be greater than zero, and is printed, because a zero there
      would mean the file has nothing to say about W and the negative is empty.
"""

import re
import sys
from collections import defaultdict

SRC = ("/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/"
       "202608072330_the-numeral-canon-panel/140_probes/p1_out.txt")

ROW = re.compile(r"^W=\s*(\d+)\s+F=(\d+)\s+(\S.*?)\s{2,}\d+ assignments -> (\d+) distinguishable")


def parse(lines):
    """(F, op) -> {W: count}"""
    groups = defaultdict(dict)
    for line in lines:
        m = ROW.match(line)
        if m:
            w, f, op, n = int(m.group(1)), int(m.group(2)), m.group(3).strip(), int(m.group(4))
            groups[(f, op)][w] = n
    return groups


def disagreements(groups):
    out = []
    for (f, op), by_w in sorted(groups.items()):
        if len(by_w) < 2:
            continue
        counts = set(by_w.values())
        if len(counts) > 1:
            out.append((f, op, dict(sorted(by_w.items()))))
    return out


def spanning(groups):
    return [(f, op, dict(sorted(by_w.items())))
            for (f, op), by_w in sorted(groups.items()) if len(by_w) >= 2]


def main():
    with open(SRC, encoding="utf-8") as fh:
        lines = fh.readlines()

    groups = parse(lines)
    span = spanning(groups)
    dis = disagreements(groups)

    print("p3 (143): does the integer width move the class count in 140's own output?")
    print(f"source: {SRC}")
    print(f"rows parsed: {sum(len(v) for v in groups.values())}")
    print(f"(F, operation) groups: {len(groups)}")
    print(f"groups appearing at two or more widths: {len(span)}\n")

    for f, op, by_w in span:
        vals = ", ".join(f"W={w}:{n}" for w, n in by_w.items())
        agree = "agree" if len(set(by_w.values())) == 1 else "DISAGREE"
        print(f"  F={f} {op:<12} {vals}   {agree}")

    print(f"\ngroups disagreeing across W: {len(dis)}")
    if dis:
        for f, op, by_w in dis:
            print(f"  F={f} {op}: {by_w}")

    print("\n=== controls ===")

    # (b) non-vacuity
    if len(span) == 0:
        print("!! CONTROL FAIL: no group spans two widths, so this file says nothing")
        print("about W and the negative above is empty.")
        sys.exit(1)
    print(f"non-vacuity control: {len(span)} groups span two or more widths, so there")
    print("was something for a disagreement to show up in.")

    # (a) positive control: inject a row that must be caught
    injected = list(lines) + [
        "W= 9 F=0 add           30 assignments -> 99 distinguishable classes\n"
    ]
    inj_groups = parse(injected)
    inj_dis = disagreements(inj_groups)
    caught = any(f == 0 and op == "add" for f, op, _ in inj_dis)
    if not caught:
        print("!! CONTROL FAIL: an injected row that disagrees across W was NOT reported.")
        print("the checker cannot detect a disagreement, so its zero is worthless.")
        sys.exit(1)
    print("positive control: an injected disagreeing row IS reported, so the checker")
    print("can detect a disagreement when one exists.")

    print()
    if not dis:
        print("VERDICT: zero disagreements. The class count does not move with W anywhere")
        print("in 140's own swept region, and 140 did not name it. 141 is right that the")
        print("miss was there to be caught, and it was catchable from the committed file.")
    else:
        print("VERDICT: W moves the count somewhere, contrary to 141.")
        sys.exit(1)


if __name__ == "__main__":
    main()
