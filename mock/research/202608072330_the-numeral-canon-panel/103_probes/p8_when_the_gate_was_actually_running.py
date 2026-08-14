#!/usr/bin/env python3
"""p8. For how much of the committed corpus was the cross-variant gate running?

`102` section 3.1 argues that every committed arm set is answer-equivalent, and
the load-bearing step is a claim about a mechanism: "Twelve crates take the
harness default, which is byte-exact cross-variant comparison."

p4 showed that a committed region takes that default and would be REFUSED by
that comparison at all four of its sizes on all 100 of the harness's own
validation seeds, which means the comparison cannot have run when that region's
CSVs were written. This probe asks the general form of that question rather than
the particular one: for how many committed regions was the comparison running at
all?

It is answerable from committed artifacts alone, with no inference:

1. Each `*.meta.json` records the `git_commit` of the tree that produced it.
2. `mock/benches/src/main.rs` gained its `harness::validate(...)` call in one
   commit, findable with `git log -S`.
3. So a region's producing commit either predates that commit or does not.

The driver's own comment beside that call states the consequence in its own
words, and this probe is a count of how far it reaches rather than a discovery
that it is true:

    `harness::run` does NOT do this: `run_orchestrator` never calls
    `validation::validate`, so without this call a variant computing a
    different answer from its peers is timed and reported like any other.
    ... which leaves the variant crate's own unit tests as the only fidelity
    check in the system.

The point is not that the pre-wiring regions are wrong. Most of them belong to
families whose unit tests assert cross-arm agreement against an independent
oracle, and all 123 of those tests pass. The point is which artifact the belief
rests on. For a region produced before the wiring, the harness mechanism is not
evidence of anything, and the evidence is the family's own tests.

Run from anywhere; paths resolve relative to this file.
"""

import collections
import glob
import json
import os
import re
import subprocess
import sys

# From p6, whose classification was cross-checked by hand against every
# `validate_output` body.
PINNED_BRIDGES = {
    "CarrierColumn",
    "Contend",
    "FootprintColumn",
    "PlanColumn",
    "MacColumn",
    "Column",
    "Wide",
    "WriteContend",
    "SatFoldCase",
    "ClampCase",
    "Case",
    "WideCase",
}


def git(repo, *args):
    return subprocess.check_output(
        ["git", "-C", repo, *args], stderr=subprocess.DEVNULL
    ).decode()


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    repo = os.path.abspath(os.path.join(here, "..", "..", "..", ".."))
    benches = os.path.join(repo, "mock", "benches")
    driver = os.path.join(benches, "src", "main.rs")

    print("p8. for how much of the committed corpus was the cross-variant gate running?")
    print()
    print("repo  :", repo)
    print()

    # 1. Find the commit that wired validation into the driver.
    log = git(
        repo,
        "log",
        "--format=%H %ct %ad %s",
        "--date=short",
        "-S",
        "harness::validate",
        "--",
        "mock/benches/src/main.rs",
    ).strip().splitlines()
    if not log:
        print("could not find the wiring commit")
        return 1
    wiring = log[-1].split(None, 3)
    wire_sha, wire_ct, wire_date, wire_subj = wiring[0], int(wiring[1]), wiring[2], wiring[3]
    print("the driver gained its cross-variant validation call in:")
    print(f"  {wire_sha[:8]}  {wire_date}  {wire_subj}")
    print()

    # Confirm it is absent before and present after, rather than assuming.
    def has_call(sha):
        try:
            t = git(repo, "show", f"{sha}:mock/benches/src/main.rs")
        except subprocess.CalledProcessError:
            return None
        return "harness::validate" in t

    print("  present at that commit      :", has_call(wire_sha))
    print("  present at its parent       :", has_call(wire_sha + "~1"))
    print()

    # 2. Which bridge each region uses, so the pre-wiring set can be split.
    with open(driver) as fh:
        dtext = fh.read()
    bridge_of = {}
    for m in re.finditer(
        r'\(\s*"([^"]+)"\s*,\s*(\d+)\s*\)\s*=>\s*routine_bridge!\(\s*([A-Za-z_]\w*)\s*<',
        dtext,
    ):
        bridge_of[(m.group(1), int(m.group(2)))] = m.group(3)

    # 3. Classify every committed region.
    pre, post = [], []
    for f in sorted(glob.glob(os.path.join(benches, "*.meta.json"))):
        region = os.path.basename(f)[: -len(".meta.json")]
        meta = json.load(open(f))
        sha = meta["git_commit"].replace("-dirty", "")
        try:
            ct = int(git(repo, "log", "-1", "--format=%ct", sha).strip())
        except subprocess.CalledProcessError:
            print(f"  UNRESOLVABLE producing commit for {region}: {sha}")
            continue
        m = re.match(r"^(.*)_n(\d+)$", region)
        key = (m.group(1), int(m.group(2))) if m else None
        bridge = bridge_of.get(key, "-")
        (pre if ct < wire_ct else post).append((region, sha, bridge))

    print("REGIONS BY WHETHER THE GATE EXISTED WHEN THEY RAN")
    print(f"  produced BEFORE the wiring : {len(pre):>4} of {len(pre)+len(post)}")
    print(f"  produced AFTER  the wiring : {len(post):>4} of {len(pre)+len(post)}")
    print()

    def split(rows):
        p = [r for r in rows if r[2] in PINNED_BRIDGES]
        q = [r for r in rows if r[2] not in PINNED_BRIDGES]
        return p, q

    pre_p, pre_q = split(pre)
    post_p, post_q = split(post)

    print("SPLIT BY WHETHER THE REGION'S OWN ROUTINE PINS AN ANSWER")
    print(f"  before wiring, answer-pinning routine     : {len(pre_p):>4}")
    print(f"  before wiring, NOT answer-pinning         : {len(pre_q):>4}")
    print(f"  after  wiring, answer-pinning routine     : {len(post_p):>4}")
    print(f"  after  wiring, NOT answer-pinning         : {len(post_q):>4}")
    print()

    print("EVERY REGION WITH NEITHER THE GATE NOR AN ANSWER-PINNING ROUTINE")
    print("  (nothing in the repository required these arms to compute one value,")
    print("   and nothing checked whether they did)")
    for region, sha, bridge in pre_q:
        print(f"    {region:<44} produced at {sha:<10} bridge={bridge}")
    print(f"  count: {len(pre_q)}")
    print()

    if post_q:
        print("REGIONS RUN UNDER THE GATE WITHOUT AN ANSWER-PINNING ROUTINE")
        for region, sha, bridge in post_q:
            print(f"    {region:<44} produced at {sha:<10} bridge={bridge}")
        print(f"  count: {len(post_q)}")
    else:
        print("No region produced after the wiring lacks an answer-pinning routine.")
    print()

    print("PRODUCING COMMITS, MOST REGIONS FIRST")
    c = collections.Counter(sha for _, sha, _ in pre + post)
    for sha, n in c.most_common():
        side = "before" if any(s == sha for _, s, _ in pre) else "after"
        print(f"    {sha:<12} {n:>4} regions   ({side} the wiring)")
    print()

    print("READING")
    print("  The mechanism `102` cites as guaranteeing answer-equivalence is the")
    print("  harness's byte-exact cross-variant comparison. For every region in")
    print("  the BEFORE set, the driver never called it, so it is not evidence")
    print("  about those regions in either direction. What remains as evidence")
    print("  there is the family's own unit tests, which is what the driver's own")
    print("  comment says: the variant crate's unit tests are 'the only fidelity")
    print("  check in the system'. For the answer-pinning families those tests")
    print("  exist, assert agreement against an independent oracle, and pass. For")
    print("  the regions listed above they do not exist.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
