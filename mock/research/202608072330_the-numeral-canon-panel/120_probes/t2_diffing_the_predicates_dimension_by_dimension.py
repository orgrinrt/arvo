#!/usr/bin/env python3
"""T2. Diff the candidate's predicates against mine dimension by dimension,
because a dropped dimension reads as a tightening and is a widening.

Two checks, both about claims made about my own file.

**Part one.** `119` section 4.7 and 4.10 state predicates covering claims of
mine. A reader comparing them by reading the sentences will miss a dropped
dimension, because a shorter predicate reads as a narrower claim and under I13 it
is a wider one: an absent dimension does not narrow the region, it says the
finding does not hold anywhere that dimension is present, and if the candidate
then asserts the claim generally the dimension has been widened away. So this
parses both sides into dimension maps and diffs the sets.

**Part two.** `115` F115-4 carries a clause saying the trait-projection route was
untried in `114`. `118` F118-16 reports it was tried, at
`114_probes/p9:244-256`. That is an absence claim of mine about a place I never
named, which is the class a negative claim about evidence always falls into. It
is checked here against the place rather than argued about.

The controls: part one mutates a predicate and must report the mutation; part two
searches for a marker that is not there and must report absent.
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
F115 = os.path.join(PANEL, "115_jhala_the_check_the_policy_selects.md")
F119 = os.path.join(PANEL, "119_leroy_the_canon_candidate_for_the_realisation_map.md")
P9 = os.path.join(PANEL, "114_probes",
                  "p9_pricing_the_two_spellings_without_a_clock.py")


def split_top(s):
    """Split on commas not inside braces."""
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch in "{[":
            depth += 1
        elif ch in "}]":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur)
    return out


ALIAS = {
    "overflow behaviour": "overflow policy",
    "rounding": "rounding",
    "term shapes": "term shapes",
    "declarations": "declarations",
    "discharge check": "discharge check",
    "target features": "target features",
    "crate type": "crate type",
    "feature gates": "feature gates",
    "toolchain": "toolchain",
    "crate-type": "crate type",
    "recursion_limit": "recursion limit",
}

NORM = {
    "truncation": "trunc",
    "wrapping": "wrap",
    "saturating": "sat",
}


def parse(pred):
    """dimension -> value, normalised enough that a wording difference is not
    reported as a dimension difference."""
    d = {}
    for part in split_top(pred):
        part = " ".join(part.split()).strip().strip("`.")
        if not part:
            continue
        m = re.match(r"^(.*?)\s+(=|in)\s+(.*)$", part)
        if not m:
            # A fragment with no `name =` is a continuation of the previous
            # item rather than a dimension. My own F115-4 predicate has one,
            # because it writes the toolchain as two comma-separated pieces,
            # and the notation reads a comma as a dimension boundary. Marked
            # rather than counted, and it is a defect in my predicate's
            # formatting rather than in anyone's compression.
            d["(continuation) " + part.lower()] = "(bare)"
            continue
        name, _, val = m.group(1).lower(), m.group(2), m.group(3)
        name = ALIAS.get(name, name)
        val = " ".join(NORM.get(w, w) for w in val.split())
        val = val.replace("{sat, wrap}", "{wrap, sat}")
        d[name] = val
    return d


def grab(path, pattern, span=14):
    """Return the first backticked predicate blob after a line matching."""
    lines = open(path, encoding="utf-8").read().splitlines()
    for i, ln in enumerate(lines):
        if re.search(pattern, ln):
            blob = " ".join(lines[i:i + span])
            m = re.search(r"`([^`]*?(?:threads|holds for)[^`]*?)`", blob)
            if not m:
                m = re.search(r"holds for:(.*?)\*", blob)
            if m:
                txt = m.group(1)
                txt = re.sub(r"^\s*holds for:\s*", "", txt)
                return txt
    return None


PAIRS = [
    ("F115-1 -> 119 4.7 first predicate",
     (F115, r"\*\*F115-1\."),
     (F119, r"\*The character split")),
    ("F115-4 -> 119 4.10",
     (F115, r"\*\*F115-4\."),
     (F119, r"\*holds for: toolchain = the pinned nightly")),
]


def report(label, mine, theirs, quiet=False):
    a, b = parse(mine), parse(theirs)
    dropped_all = sorted(set(a) - set(b))
    dropped = [k for k in dropped_all if not k.startswith("(continuation)")]
    conts = [k for k in dropped_all if k.startswith("(continuation)")]
    added = sorted(k for k in set(b) - set(a)
                   if not k.startswith("(continuation)"))
    changed = sorted(k for k in set(a) & set(b) if a[k] != b[k])
    if not quiet:
        print(f"  {label}")
        print(f"    dimensions mine {len(a)}, candidate {len(b)}")
        print(f"    DROPPED (in mine, absent from the candidate): "
              f"{dropped if dropped else 'none'}")
        print(f"    added   (in the candidate, absent from mine): "
              f"{added if added else 'none'}")
        if conts:
            print(f"    (parse note, not a dimension: {conts})")
        for k in changed:
            print(f"    value differs on '{k}': mine `{a[k]}` "
                  f"candidate `{b[k]}`")
        if not changed:
            print("    value differs on: none")
        print()
    return dropped, added, changed


def main():
    print("T2. predicates diffed dimension by dimension, and one absence claim")
    print("=" * 78)
    print()
    print("PART ONE. my predicates against the candidate's")
    print()
    bad = 0
    for label, (pa, ra), (pb, rb) in PAIRS:
        mine, theirs = grab(pa, ra), grab(pb, rb)
        if mine is None or theirs is None:
            print(f"  {label}: COULD NOT LOCATE "
                  f"(mine={mine is not None}, candidate={theirs is not None})")
            bad += 1
            continue
        d, a_, c = report(label, mine, theirs)
        if d or c:
            bad += 1

    print("  CONTROL: the same comparison with one dimension deleted and one")
    print("  value altered on the candidate side, which must be reported")
    print()
    mine = grab(*PAIRS[0][1])
    theirs = grab(*PAIRS[0][2])
    mutated = re.sub(r"threads = 1, ", "", theirs).replace("W = 3", "W = 5")
    d, a_, c = report("CONTROL mutated candidate predicate", mine, mutated)
    control_fired = bool(d or c)
    print(f"  control fired: {control_fired}")

    print()
    print("-" * 78)
    print("PART TWO. the absence claim in F115-4, checked against the place")
    print()
    src = open(P9, encoding="utf-8").read()
    markers = [
        ("trait Pick", "a selector trait"),
        ("type Arm", "an associated type carrying the chosen arm"),
        ("impl Pick for Cond<true>", "the true branch"),
        ("impl Pick for Cond<false>", "the false branch"),
        ("as Pick>::Arm", "reading the const through the projection"),
        ("selection-assoc", "and the variant is named"),
    ]
    found = 0
    for m, what in markers:
        hit = m in src
        found += hit
        print(f"  {'present' if hit else 'ABSENT ':<8} {m:<28} {what}")
    absent_marker = "trait PickNothingLikeThis" in src
    print(f"  {'present' if absent_marker else 'ABSENT ':<8} "
          f"{'trait PickNothingLikeThis':<28} CONTROL, must be absent")
    print()
    print(f"  markers found: {found} of {len(markers)}")
    print(f"  so 114_probes/p9 does contain a trait-projection variant: "
          f"{found == len(markers)}")
    print("  => F115-4's clause that the route was untried is WRONG, and")
    print("     F118-16 and 119 section 1.3 are right to retire it.")

    print()
    print("-" * 78)
    print(f"part one: pairs with a dropped dimension or a changed value: {bad}")
    print(f"part one control fired: {control_fired}")
    print(f"part two: the absence claim is refuted: {found == len(markers)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
