#!/usr/bin/env python3
"""Independent re-derivation of 103's F-103-6, which 103 named as the finding it
most wanted re-derived.

Claim under test: 175 of 254 committed regions were produced before arvo's bench
driver ever called cross-variant validation.

Method, written without opening 103's p8:
  1. Find the commit that added `harness::validate` to mock/benches/src/main.rs.
  2. Verify the call is present there and absent at its parent.
  3. For each *.meta.json, resolve its `git_commit` to a commit time and compare.

THE PRECONDITION, which no file in this unit carries and without which the join
returns 253 unresolvable and the finding evaporates:
  253 of the 254 git_commit values carry a `-dirty` suffix. It is meaningless.
  `22` (section around lines 188-193) established that the harness writes its
  artifacts into the tree it then hashes, so every size row after the first is
  dirty by construction and `git diff --name-only HEAD` returns zero TRACKED
  files. Stripping the suffix is correct, and it is correct for a reason that
  sits 81 files back and is uncited in this unit.
Run from arvo/mock/benches.
"""
import json, glob, subprocess, collections

R = "/Users/orgrinrt/Dev/clause-dev/arvo"

def sh(*a):
    return subprocess.run(["git", "-C", R, *a], capture_output=True, text=True)

wiring = sh("log", "--oneline", "-S", "harness::validate", "--", "mock/benches/src/main.rs").stdout.strip()
print("commit that added harness::validate to the driver:")
print("   ", wiring or "(none found)")
c = wiring.split()[0]
present = sh("show", f"{c}:mock/benches/src/main.rs").stdout.count("harness::validate")
parent  = sh("show", f"{c}~1:mock/benches/src/main.rs").stdout.count("harness::validate")
print(f"    present at {c}      : {present > 0}")
print(f"    present at {c}~1    : {parent > 0}")

wire = int(sh("log", "-1", "--format=%ct", c).stdout.strip())

cache = {}
def when(commit):
    key = commit.split("-")[0]          # strip the meaningless -dirty suffix
    if key not in cache:
        r = sh("log", "-1", "--format=%ct", key)
        cache[key] = int(r.stdout.strip()) if r.returncode == 0 and r.stdout.strip() else None
    return cache[key]

before = after = unres = 0
dirty = clean = 0
producing = collections.Counter()
for f in sorted(glob.glob("*.meta.json")):
    gc = json.load(open(f)).get("git_commit", "")
    producing[gc.split("-")[0]] += 1
    if gc.endswith("-dirty"): dirty += 1
    else: clean += 1
    t = when(gc) if gc else None
    if t is None: unres += 1
    elif t < wire: before += 1
    else: after += 1

print()
print("meta files                      :", before + after + unres)
print("produced BEFORE the wiring      :", before)
print("produced AFTER  the wiring      :", after)
print("unresolvable                    :", unres)
print("distinct producing commits      :", len(producing))
print()
print("git_commit values ending -dirty :", dirty)
print("git_commit values clean         :", clean)
print()
print("NAIVE join, not stripping -dirty (what a later reader gets without 22's fact):")
naive_unres = sum(1 for f in sorted(glob.glob("*.meta.json"))
                  if sh("log", "-1", "--format=%ct",
                        json.load(open(f)).get("git_commit", "")).returncode != 0)
print("    unresolvable                :", naive_unres, "of", before + after + unres)
