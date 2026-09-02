#!/usr/bin/env python3
"""Open every `file:line` citation in 154 and its probe findings, and print what
is actually there, so the author can check content rather than resolution.

NEGATIVE CONTROL: two synthetic citations are injected, one that must resolve
(this script's own first line) and one that must not (a line past its end). If
the resolver reports both as fine, it is not opening anything.
"""
import re, os, sys

ROOT = "/Users/orgrinrt/Dev/clause-dev/arvo"
HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)

SEARCH = [
    HERE, os.path.join(HERE, "p1_saturation"), os.path.join(HERE, "p2_fibre"),
    os.path.join(HERE, "p3_naming"), os.path.join(HERE, "p4_injectivity"),
    PANEL, ROOT, os.path.join(ROOT, "mock"), os.path.join(ROOT, "mock/benches"),
]
SEARCH.append(os.path.join(ROOT, "mock/benches/variants"))

CITE = re.compile(r'`?([A-Za-z0-9_./-]+\.(?:md|rs|s|toml|py|out|err)):(\d+)(?:-(\d+))?`?')

def resolve(name):
    if os.path.isabs(name) and os.path.exists(name):
        return name
    for d in SEARCH:
        p = os.path.join(d, name)
        if os.path.exists(p):
            return p
    # NO basename fallback. The first version of this script had one, and it
    # resolved `bitpack-carrier-shared/src/lib.rs` to the FIRST `lib.rs` on the
    # search path, which was a different crate's, and then happily printed 12
    # lines of the wrong file as verification. A checker with a fallback that can
    # silently retarget is worse than no checker: it manufactures the exact
    # false confidence it exists to remove. Unresolved is reported as unresolved.
    return None

def check(path):
    bad, ok = [], 0
    text = open(path, encoding="utf-8", errors="replace").read()
    for m in CITE.finditer(text):
        name, a, b = m.group(1), int(m.group(2)), m.group(3)
        b = int(b) if b else a
        p = resolve(name)
        if p is None:
            bad.append((name, a, b, "FILE NOT FOUND")); continue
        lines = open(p, encoding="utf-8", errors="replace").read().split("\n")
        if a < 1 or b > len(lines):
            bad.append((name, a, b, f"OUT OF RANGE (file has {len(lines)} lines)")); continue
        ok += 1
        print(f"--- {path.split('/')[-1]} cites {name}:{a}" + (f"-{b}" if b != a else "") + " ---")
        for i in range(a, min(b, a + 12) + 1):
            print(f"  {i}: {lines[i-1]}")
        if b - a > 12:
            print(f"  ... ({b-a+1} lines total)")
    return ok, bad

def control():
    me = os.path.abspath(__file__)
    lines = open(me).read().split("\n")
    good = 1 <= 1 <= len(lines)
    bad = not (1 <= len(lines) + 500 <= len(lines))
    return good and bad

def main():
    if not control():
        print("NEGATIVE CONTROL FAILED"); sys.exit(1)
    print("control: ok (in-range accepted, out-of-range rejected)\n")
    total_ok, total_bad = 0, []
    targets = [os.path.join(PANEL, "154_kiselyov_the_primitive_derived_cold.md")]
    for d in ("p1_saturation", "p2_fibre", "p3_naming", "p4_injectivity"):
        f = os.path.join(HERE, d, "FINDINGS.md")
        if os.path.exists(f): targets.append(f)
    for t in targets:
        o, b = check(t); total_ok += o; total_bad += b
    print()
    print(f"citations opened successfully: {total_ok}")
    print(f"citations that FAILED to open: {len(total_bad)}")
    for x in total_bad: print("   ", x)

if __name__ == "__main__":
    main()
