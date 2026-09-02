#!/usr/bin/env python3
"""Does the finding survive a change of optimisation level, element count and
target-cpu, or is it an artifact of one configuration?

`17_probes/verify.sh` found the panel's existing erasure instrument producing a
false negative below full optimisation, because the symbol folding it depends
on is itself an optimisation pass. So a result read at one -C opt-level is a
result about that level until it is swept.

Reports, per configuration, how many of the 36 widths report ERASED against the
hand-written twin, and at how many the reduction collapses onto a single
accumulator.

Counts off emitted assembly. Nothing here is timed.
"""

import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

import loopshape
import oracle

PIN = "nightly-2026-05-28"
HERE = os.path.dirname(os.path.abspath(__file__))
WIDTHS = list(range(1, 33)) + [33, 40, 47, 48]
ARMS = ("typed", "gather", "wide")


def build(args):
    src, outdir, opt, cpu = args
    cmd = ["rustc", f"+{PIN}", "--edition", "2024", "-C", f"opt-level={opt}",
           "-C", "panic=abort", "--cfg", "link_check"]
    if cpu:
        cmd += ["-C", f"target-cpu={cpu}"]
    cmd += ["--emit", "asm", "--crate-type", "lib", src, "--out-dir", outdir]
    return subprocess.run(cmd, capture_output=True, text=True).returncode


def sweep(opt, elems, cpu):
    tag = f"O{opt}_n{elems}" + (f"_{cpu}" if cpu else "")
    wdir, adir = f"/tmp/51w_{tag}", f"/tmp/51a_{tag}"
    subprocess.run([sys.executable, os.path.join(HERE, "gen_v3.py"), wdir, str(elems)],
                   capture_output=True)
    os.makedirs(adir, exist_ok=True)
    with ThreadPoolExecutor(max_workers=1) as ex:
        list(ex.map(build, [(os.path.join(wdir, f"w{w:02d}.rs"), adir, opt, cpu)
                            for w in WIDTHS]))
    bad, collapse = {}, {}
    for a in ARMS:
        bad[a], collapse[a] = [], []
        for w in WIDTHS:
            s = os.path.join(adir, f"w{w:02d}.s")
            if not os.path.exists(s):
                bad[a].append(w)
                continue
            if not oracle.verdict(
                    oracle.compare(s, f"w{w}_{a}", f"w{w}_hand")).startswith("ERASED"):
                bad[a].append(w)
            r = loopshape.analyse(s, f"w{w}_{a}", w)
            if not r.get("error") and r["accumulators"] <= 1:
                collapse[a].append(w)
    return bad, collapse


def one(opt, elems, cpu):
    bad, collapse = sweep(opt, elems, cpu)
    head = f"opt={opt:<2} n={elems:<5} cpu={cpu or 'default':<8}"
    for a in ARMS:
        print(f"  {head} {a:<7} ERASED {len(WIDTHS) - len(bad[a]):>2}/{len(WIDTHS)}"
              f"   single-accumulator at {len(collapse[a]):>2} widths"
              + (f"   not-erased: {bad[a]}" if a != "wide" and bad[a] else ""), flush=True)


def main():
    if len(sys.argv) > 1:
        opt, elems, cpu = sys.argv[1], int(sys.argv[2]), (sys.argv[3] or None)
        one(opt, elems, cpu if cpu != "-" else None)
        return
    ver = subprocess.run(["rustc", f"+{PIN}", "--version"],
                         capture_output=True, text=True).stdout.strip()
    print(f"toolchain: {ver}")
    print(f"{len(WIDTHS)} widths per configuration. Counts off emitted assembly; nothing timed.")
    print(flush=True)
    configs = [(o, n, None) for o in ("2", "3", "s") for n in (64, 1000, 4096)]
    configs += [("3", 1000, "native")]
    for opt, elems, cpu in configs:
        bad, collapse = sweep(opt, elems, cpu)
        head = f"opt={opt:<2} n={elems:<5} cpu={cpu or 'default':<8}"
        for a in ARMS:
            print(f"  {head} {a:<7} ERASED {len(WIDTHS) - len(bad[a]):>2}/{len(WIDTHS)}"
                  f"   single-accumulator at {len(collapse[a]):>2} widths"
                  + (f"   not-erased: {bad[a]}" if a != "wide" and bad[a] else ""),
                  flush=True)
        print(flush=True)


if __name__ == "__main__":
    main()
