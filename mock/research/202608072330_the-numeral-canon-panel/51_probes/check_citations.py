#!/usr/bin/env python3
"""Open every citation in 51 and test its CONTENT, not its resolution.

`RULES.md:126-133` records that one member found seven of its own citations
wrong by opening them, and that a reference which resolves is not a reference
that says what you claim. This opens each `file:line` and `file:a-b` cited in
`51_fog_the_packed_sequence_erasure_arm.md`, prints the line, and flags any
that does not exist. Judging whether the line supports the claim is still a
reading job; this makes the reading possible.

  python3 check_citations.py
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
DOC = os.path.join(PANEL, "51_fog_the_packed_sequence_erasure_arm.md")

CITE = re.compile(r"`([A-Za-z0-9_./-]+\.(?:md|rs|toml|sh|out|s))(?::(\d+)(?:-(\d+))?)?`")


def resolve(name):
    bases = (PANEL, HERE, os.path.dirname(PANEL),
             os.path.join(PANEL, "..", "..", ".."),
             os.path.expanduser("~/Dev/clause-dev/.claude/rules"),
             os.path.expanduser("~/Dev/clause-dev/arvo/.claude/rules"))
    for base in bases:
        p = os.path.normpath(os.path.join(base, name))
        if os.path.isfile(p):
            return p
    return None


def main():
    text = open(DOC).read()
    seen, missing, linefail, ok = set(), [], [], 0
    for m in CITE.finditer(text):
        name, a, b = m.group(1), m.group(2), m.group(3)
        key = (name, a, b)
        if key in seen:
            continue
        seen.add(key)
        path = resolve(name)
        if path is None:
            missing.append(name)
            continue
        if a is None:
            ok += 1
            print(f"OK   {name}  (file exists, no line cited)")
            continue
        lines = open(path, errors="replace").read().splitlines()
        lo, hi = int(a), int(b or a)
        if hi > len(lines):
            linefail.append(f"{name}:{a}" + (f"-{b}" if b else ""))
            print(f"FAIL {name}:{a}-{b or a}  file has only {len(lines)} lines")
            continue
        ok += 1
        snippet = " / ".join(x.strip() for x in lines[lo - 1:hi])[:150]
        print(f"OK   {name}:{a}" + (f"-{b}" if b else "") + f"  -> {snippet}")

    print()
    print(f"{ok} citations resolved and opened, {len(missing)} name a file that does not exist, "
          f"{len(linefail)} name a line past end of file")
    if missing:
        print(f"  missing files: {sorted(set(missing))}")
    if linefail:
        print(f"  bad lines: {linefail}")
    return 1 if (missing or linefail) else 0


if __name__ == "__main__":
    sys.exit(main())
