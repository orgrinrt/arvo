#!/usr/bin/env python3
"""157 citation checker. Opens every `file:line` and `file:a-b` in the deliverable and
prints the content, so a reference that RESOLVES can be checked against what it SAYS.

NEGATIVE CONTROL: a deliberately wrong citation is appended to the input list and must be
reported UNRESOLVED or as content that does not match. No basename fallback: an
unresolvable path is reported unresolvable rather than silently retargeted, which is the
defect 154's own checker found in itself.
"""
import re, sys, os

PANEL = os.path.dirname(os.path.abspath(__file__)) + "/.."
# Every root is resolved from this file's own location. The two workspace roots
# were absolute, naming a checkout that still exists on this host, so they did
# not fail when the arc moved: they resolved, against a different tree, and said
# nothing. A citation checker verifying somebody else's clone reports clean and
# means nothing.
WORKSPACE = PANEL + "/../../../.."
ROOTS = [PANEL, PANEL + "/../../..", PANEL + "/../../../mock/benches/variants",
         PANEL + "/../../../mock/benches", WORKSPACE + "/.claude/rules",
         WORKSPACE]

PAT = re.compile(r'`([A-Za-z0-9_./-]+\.(?:md|rs|py|toml|sh|err|out|txt)):(\d+)(?:-(\d+))?`')
BARE = re.compile(r'`(\d{2,3}):(\d+)(?:-(\d+))?`')

def resolve(name):
    for r in ROOTS:
        p = os.path.normpath(os.path.join(r, name))
        if os.path.isfile(p):
            return p
    return None

def resolve_bare(num):
    import glob
    hits = glob.glob(os.path.join(PANEL, f"{num}_*.md"))
    return hits[0] if len(hits) == 1 else None

def show(path, a, b):
    with open(path, errors="replace") as f:
        lines = f.readlines()
    b = b or a
    if a < 1 or b > len(lines):
        return None
    return "".join(lines[a-1:b])

def main(target):
    txt = open(target).read()
    cites = []
    for m in PAT.finditer(txt):
        cites.append((m.group(1), int(m.group(2)), int(m.group(3)) if m.group(3) else None, "path"))
    for m in BARE.finditer(txt):
        cites.append((m.group(1), int(m.group(2)), int(m.group(3)) if m.group(3) else None, "bare"))
    # CONTROL
    cites.append(("no_such_file_157.md", 1, None, "path"))
    cites.append(("109_bellard_the_primitive_derived_cold.md", 999999, None, "path"))
    bad = 0
    for name, a, b, kind in cites:
        p = resolve_bare(name) if kind == "bare" else resolve(name)
        if p is None:
            print(f"UNRESOLVED  {name}:{a}" + (f"-{b}" if b else ""))
            bad += 1
            continue
        body = show(p, a, b)
        if body is None:
            print(f"OUT OF RANGE {name}:{a}" + (f"-{b}" if b else ""))
            bad += 1
            continue
        head = body.strip().splitlines()
        print(f"OK  {name}:{a}" + (f"-{b}" if b else "") + f"   [{len(head)} lines]")
        for l in head[:3]:
            print("      " + l[:150])
    print(f"\ncitations checked: {len(cites)-2} plus 2 controls")
    print(f"failures: {bad}  (the 2 controls MUST be among them; "
          f"{'CONTROL PASS' if bad >= 2 else 'CONTROL FAIL'})")

main(sys.argv[1])
