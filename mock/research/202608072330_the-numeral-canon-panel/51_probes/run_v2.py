#!/usr/bin/env python3
"""Compile the three-arm width matrix and run the five oracles on two pairs.

  typed vs hand    the erasure question. Does the typestate cost an
                   instruction against the same walk written out by hand.
  typed vs native  the packing question, in shape only. What the packed walk
                   emits that an unpacked walk over a native carrier does not.
                   The SHAPE is answerable here. The MAGNITUDE is not: that
                   needs the bench harness at `mock/benches/`, which nothing in
                   this directory has run on, so the cost stays unpriced.

This is an assembly inspection. No timing loop runs in it and every number it
prints is a count.

  python3 run_v2.py [--opt 3] [--elems 1000] [--cgu N]
"""

import argparse
import os
import subprocess
import sys

import oracle

PIN = "nightly-2026-05-28"
HERE = os.path.dirname(os.path.abspath(__file__))


def sh(*a):
    return subprocess.run(a, capture_output=True, text=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--opt", default="3")
    ap.add_argument("--elems", type=int, default=1000)
    ap.add_argument("--cgu", default=None, help="codegen-units, default is rustc's")
    args = ap.parse_args()

    wdir = os.path.join(HERE, "widths2")
    tag = f"O{args.opt}_n{args.elems}" + (f"_cgu{args.cgu}" if args.cgu else "")
    outdir = os.path.join(HERE, f"asm2_{tag}")
    sh(sys.executable, os.path.join(HERE, "gen_v2.py"), wdir, str(args.elems))
    os.makedirs(outdir, exist_ok=True)

    print(f"toolchain : {sh('rustc', f'+{PIN}', '--version').stdout.strip()}")
    print(f"{[l for l in sh('rustc', f'+{PIN}', '-vV').stdout.splitlines() if l.startswith('host')][0]}")
    print(f"opt-level : {args.opt}   elements: {args.elems}   "
          f"codegen-units: {args.cgu or 'rustc default'}")
    gates = subprocess.run(f"grep -l '#!\\[feature' {wdir}/w*.rs | wc -l",
                           shell=True, capture_output=True, text=True).stdout.strip()
    print(f"feature gates in generated sources (expected 0): {gates}")
    print()
    print(f"{'W':>3} {'acc':>3} | {'typed vs hand: erasure':^46} | "
          f"{'typed vs native: packing shape':^40}")
    print(f"{'':>3} {'':>3} | {'t/h instr':>10} {'rec':>7} {'verdict':<26} | "
          f"{'t/n instr':>10} {'rec':>7} {'loads t/n':>12}")
    print("-" * 116)

    notgreen, recdiff = [], []
    for fn in sorted(os.listdir(wdir)):
        if not fn.startswith("w") or not fn.endswith(".rs"):
            continue
        w = int(fn[1:-3])
        r = sh(*(["rustc", f"+{PIN}", "--edition", "2024", "-C", f"opt-level={args.opt}"]
                 + (["-C", f"codegen-units={args.cgu}"] if args.cgu else [])
                 + ["--emit", "asm", "--crate-type", "lib",
                    os.path.join(wdir, fn), "--out-dir", outdir]))
        if r.returncode != 0:
            print(f"{w:>3}  BUILD FAILED: {(r.stderr.strip().splitlines() or ['?'])[0]}")
            notgreen.append((w, "build"))
            continue
        s = os.path.join(outdir, fn[:-3] + ".s")
        th = oracle.compare(s, f"w{w}_typed", f"w{w}_hand")
        tn = oracle.compare(s, f"w{w}_typed", f"w{w}_native")
        if th.get("error") or tn.get("error"):
            print(f"{w:>3}  ORACLE ERROR: {th.get('error') or tn.get('error')}")
            notgreen.append((w, "oracle"))
            continue
        v = oracle.verdict(th)
        bodies, aliases = oracle.parse(s)

        def loads(sym):
            b, _ = oracle.resolve(bodies, aliases, sym)
            _, ins = oracle.hottest_block(b)
            return sum(1 for mn, _ in ins if mn.startswith("ld"))

        lt, ln = loads(f"w{w}_typed"), loads(f"w{w}_native")
        acc = (w + 14) // 8
        print(f"{w:>3} {acc:>3} | {th['n_a']:>4}/{th['n_b']:<5} {th['O5_a']:>3}/{th['O5_b']:<3} "
              f"{v:<26} | {tn['n_a']:>4}/{tn['n_b']:<5} {tn['O5_a']:>3}/{tn['O5_b']:<3} "
              f"{lt:>5}/{ln:<6}")
        if not v.startswith("ERASED"):
            notgreen.append((w, v))
        if th["O5_a"] != th["O5_b"]:
            recdiff.append((w, th["O5_a"], th["O5_b"]))

    print()
    print(f"typed vs hand: {len(notgreen)} widths did not report ERASED")
    if notgreen:
        for w, v in notgreen:
            print(f"    W={w}: {v}")
    if recdiff:
        print(f"recurrence differs (typed, hand): {recdiff}")


if __name__ == "__main__":
    main()
