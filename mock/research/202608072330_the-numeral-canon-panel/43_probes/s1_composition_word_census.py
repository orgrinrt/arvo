#!/usr/bin/env python3
"""s1: how the panel and its inherited record use the word "composition".

The dispatch for file 43 assumes "composition" means an aggregate: a vector, a
matrix, a sparse structure, a graph weight column.  The panel's founding
sentence (00_brief.md) says "the primitives become named compositions over one
format concept".  This probe asks whether those are the same word.

Method: extract every occurrence of composit* / compose* / composing in the
panel directory and in the archived consolidation it inherited its vocabulary
from, with a window of context, and classify each by which sense the context
forces.  Every occurrence is printed so a reader can disagree with the
classification rather than trusting the counts.

Senses:
  BIND      a numeral kind bound to a strategy, i.e. Number<N, S>; the
            predecessor panel's sense.
  AGGREGATE a structure holding several values: vector, matrix, column, fold.
  DECOMP    "decomposition" of an axis space; unrelated, matched only because
            the substring is shared.  Excluded from the two counts.
  FUNC      function composition / composing two operations or mechanisms.
  UNCLEAR   context does not force a sense.

Run from the panel directory.
"""

import os
import re
import sys

PAT = re.compile(r"\b(compositions?|composes?|composing|composed|composable|composite)\b", re.I)
DECOMP = re.compile(r"\bdecompos", re.I)

BIND_HINTS = [
    "number<", "numeral", "strategy preset", "presets", "d52", "bindable",
    "warmfixed", "binary32", "policy", "lowering", "semantic name",
]
AGG_HINTS = [
    "vector", "matrix", "column", "fold", "aggregate", "capacity", "csr",
    "sparse", "array", "tensor", "run of value", "bigger unit", "element",
    "accumulator", "arity-n", "stride", "packed run",
]
FUNC_HINTS = [
    "mechanism", "bound", "signature", "law", "trait", "in one signature",
    "compose upward", "the two mechanisms", "operations", "interaction hazard",
]


def classify(window: str) -> str:
    w = window.lower()
    b = sum(1 for h in BIND_HINTS if h in w)
    a = sum(1 for h in AGG_HINTS if h in w)
    f = sum(1 for h in FUNC_HINTS if h in w)
    best = max(b, a, f)
    if best == 0:
        return "UNCLEAR"
    winners = [n for n, c in (("BIND", b), ("AGGREGATE", a), ("FUNC", f)) if c == best]
    if len(winners) > 1:
        return "UNCLEAR"
    return winners[0]


def scan(path):
    out = []
    with open(path, encoding="utf-8", errors="replace") as fh:
        lines = fh.readlines()
    for i, line in enumerate(lines, 1):
        for m in PAT.finditer(line):
            # a bare "decomposition" contains no standalone token match, but be safe
            if DECOMP.search(line[max(0, m.start() - 3):m.end()]):
                out.append((i, "DECOMP", line.strip()))
                continue
            lo = max(0, i - 3)
            hi = min(len(lines), i + 2)
            window = " ".join(x.strip() for x in lines[lo:hi])
            out.append((i, classify(window), line.strip()))
    return out


def main():
    root = "."
    groups = {
        "panel files 00-42": sorted(
            f for f in os.listdir(root)
            if re.match(r"^\d\d_.*\.md$", f)
        ),
        "panel instruments": [f for f in ("00_brief.md", "RULES.md", "OPTIONS.md",
                                          "INTENTS.md", "DROPLIST.md")
                              if os.path.exists(f)],
        "archived consolidation": [os.path.join("archive", f)
                                   for f in ("CANON_CANDIDATE.md", "SETTLED.md")
                                   if os.path.exists(os.path.join("archive", f))],
        "seed sweeps": [os.path.join("seed", f) for f in sorted(os.listdir("seed"))
                        if f.endswith(".md")] if os.path.isdir("seed") else [],
    }

    grand = {}
    for gname, files in groups.items():
        print(f"\n{'=' * 78}\n{gname}\n{'=' * 78}")
        tally = {}
        for path in files:
            hits = scan(path)
            if not hits:
                continue
            print(f"\n-- {path}  ({len(hits)} occurrences)")
            for lineno, sense, text in hits:
                tally[sense] = tally.get(sense, 0) + 1
                grand[sense] = grand.get(sense, 0) + 1
                snippet = text if len(text) <= 150 else text[:147] + "..."
                print(f"   {lineno:>5}  {sense:<9}  {snippet}")
        print(f"\n   subtotal {gname}: " +
              ", ".join(f"{k}={v}" for k, v in sorted(tally.items())))

    print(f"\n{'=' * 78}\nGRAND TOTAL\n{'=' * 78}")
    for k, v in sorted(grand.items()):
        print(f"  {k:<10} {v}")

    # The load-bearing sentence, quoted rather than counted.
    print("\nThe founding sentence, from 00_brief.md:")
    with open("00_brief.md", encoding="utf-8") as fh:
        for i, line in enumerate(fh, 1):
            if "named compositions over one format" in line:
                print(f"  00_brief.md:{i}: {line.strip()}")

    print("\nOp's sentence, from 32_op_arvo_adapts_to_the_cores_it_finds.md:")
    with open("32_op_arvo_adapts_to_the_cores_it_finds.md", encoding="utf-8") as fh:
        for i, line in enumerate(fh, 1):
            if "compose to bigger units" in line:
                print(f"  32:{i}: {line.strip()}")


if __name__ == "__main__":
    sys.exit(main())
