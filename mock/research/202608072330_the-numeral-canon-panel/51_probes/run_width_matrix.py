#!/usr/bin/env python3
"""Compile every generated width crate and run the five oracles on each pair.

This is an assembly inspection, not a bench. It emits code and counts what is
in it. No timing loop runs and no magnitude is claimed anywhere in it. The
question it answers is whether the typestate-supplied layout coordinates lower
to the same code as the same coordinates written as literals, and it answers it
at every width rather than at one.

  python3 run_width_matrix.py [--opt 3] [--elems 1000]
"""

import argparse
import os
import subprocess
import sys

import oracle

PIN = "nightly-2026-05-28"
HERE = os.path.dirname(os.path.abspath(__file__))


def sh(*a, **kw):
    return subprocess.run(a, capture_output=True, text=True, **kw)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--opt", default="3")
    ap.add_argument("--elems", type=int, default=1000)
    ap.add_argument("--outdir", default=None)
    args = ap.parse_args()

    outdir = args.outdir or os.path.join(HERE, f"asm_O{args.opt}_n{args.elems}")
    wdir = os.path.join(HERE, "widths")
    sh(sys.executable, os.path.join(HERE, "gen_width_matrix.py"), wdir, str(args.elems))
    os.makedirs(outdir, exist_ok=True)

    ver = sh("rustc", f"+{PIN}", "--version").stdout.strip()
    host = [l for l in sh("rustc", f"+{PIN}", "-vV").stdout.splitlines() if l.startswith("host")]
    gates = sh("bash", "-c", f"grep -l '#!\\[feature' {wdir}/*.rs | wc -l").stdout.strip()
    print(f"toolchain : {ver}")
    print(f"{host[0] if host else 'host: unknown'}")
    print(f"opt-level : {args.opt}    elements: {args.elems}")
    print(f"feature gates in generated sources (expected 0): {gates}")
    print()
    print(f"{'W':>3} {'acc':>3} {'instr t/h':>11} {'loop t/h':>9} {'rec t/h':>8} "
          f"{'O1':>2} {'O2':>2} {'O3':>2} {'O4':>2}  verdict")
    print("-" * 96)

    rows, failures = [], []
    for fn in sorted(os.listdir(wdir)):
        if not fn.startswith("w") or not fn.endswith(".rs"):
            continue
        w = int(fn[1:-3])
        src = os.path.join(wdir, fn)
        r = sh("rustc", f"+{PIN}", "--edition", "2024", "-C", f"opt-level={args.opt}",
               "--emit", "asm", "--crate-type", "lib", src, "--out-dir", outdir)
        if r.returncode != 0:
            first = (r.stderr.strip().splitlines() or ["?"])[0]
            print(f"{w:>3}  BUILD FAILED: {first}")
            failures.append((w, first))
            continue
        s = os.path.join(outdir, fn[:-3] + ".s")
        res = oracle.compare(s, f"w{w}_typed", f"w{w}_hand")
        if res.get("error"):
            print(f"{w:>3}  ORACLE ERROR: {res['error']}")
            failures.append((w, res["error"]))
            continue
        v = oracle.verdict(res)
        acc = (w + 14) // 8
        print(f"{w:>3} {acc:>3} {res['n_a']:>5}/{res['n_b']:<5} "
              f"{res['loop_a']:>4}/{res['loop_b']:<4} {res['O5_a']:>3}/{res['O5_b']:<4} "
              f"{int(res['O1']):>2} {int(res['O2']):>2} {int(res['O3']):>2} {int(res['O4']):>2}"
              f"  {v}")
        rows.append((w, res, v))

    print()
    erased = [r for r in rows if r[2].startswith("ERASED")]
    print(f"summary: {len(rows)} widths built, {len(failures)} failed to build or parse")
    print(f"         {len(erased)} of {len(rows)} report ERASED under the strongest oracle "
          f"that fired")
    bad = [(w, v) for w, _, v in rows if not v.startswith("ERASED")]
    if bad:
        print(f"         widths NOT erased: {bad}")
    amb = [w for w, res, _ in rows if res.get("O4_ambiguous")]
    if amb:
        print(f"         widths where the live-in correspondence was a guess: {amb}")
    rec = [(w, res["O5_a"], res["O5_b"]) for w, res, _ in rows if res["O5_a"] != res["O5_b"]]
    if rec:
        print(f"         widths where the recurrence differs (typed, hand): {rec}")


if __name__ == "__main__":
    main()
