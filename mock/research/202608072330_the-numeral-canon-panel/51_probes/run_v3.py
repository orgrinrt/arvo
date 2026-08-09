#!/usr/bin/env python3
"""Five-arm width matrix: loop shape per arm, and the erasure verdict per pair.

Reads assembly already emitted by `verify.sh` under one configuration, the same
configuration the link-and-call cross-check is built with, so what is read and
what is checked are the same artifact.

Prints, per width and arm: loop-block instructions, loads in the loop, number of
independent reduction accumulators, elements per iteration, and the loop-carried
dependent instructions per element. The last is the quantity a reduction is
bounded by and the one an opcode count cannot see.

Every number here is a count read off emitted assembly. Nothing is timed, no
bench harness has run, and what any of this costs in cycles is unpriced.

  python3 run_v3.py [--asmdir asm3]
"""

import argparse
import os

import loopshape
import oracle

ARMS = ["typed", "gather", "wide", "hand", "native"]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--asmdir", default="asm3")
    args = ap.parse_args()
    here = os.path.dirname(os.path.abspath(__file__))
    asmdir = os.path.join(here, args.asmdir)

    print("Each cell is  loop-instrs/loads/accumulators/elements-per-iter/chain-per-elem.")
    print("chain-per-elem is loop-carried dependent instructions per element: the quantity a")
    print("reduction is bounded by. Counts off emitted assembly. Nothing here is timed.")
    print()
    hdr = f"{'W':>3} {'acc':>3} |"
    for a in ARMS:
        hdr += f" {a:^22}|"
    print(hdr)
    print("-" * len(hdr))

    verdicts = {}
    for fn in sorted(os.listdir(asmdir)):
        if not fn.endswith(".s"):
            continue
        w = int(fn[1:-2])
        s = os.path.join(asmdir, fn)
        row = f"{w:>3} {(w + 14) // 8:>3} |"
        shapes = {}
        for a in ARMS:
            r = loopshape.analyse(s, f"w{w}_{a}", w)
            if r.get("error"):
                row += f" {'-':^22}|"
                continue
            shapes[a] = r
            ce = f"{r['chain'] / r['elems_per_iter']:.2f}" if r["elems_per_iter"] else "?"
            e = r["elems_per_iter"] if r["elems_per_iter"] else "?"
            row += f" {r['loop_instrs']:>3}/{r['loads']:>2}/{r['accumulators']:>1}/{e:>2}/{ce:>5} |"
        print(row)
        for a in ("typed", "gather", "wide"):
            v = oracle.verdict(oracle.compare(s, f"w{w}_{a}", f"w{w}_hand"))
            verdicts.setdefault(a, {})[w] = v

    print()
    print("Erasure verdict against the hand-written twin, by arm:")
    for a in ("typed", "gather", "wide"):
        ok = [w for w, v in verdicts[a].items() if v.startswith("ERASED")]
        bad = sorted(w for w, v in verdicts[a].items() if not v.startswith("ERASED"))
        print(f"  {a:<7}: ERASED at {len(ok)}/{len(verdicts[a])} widths."
              + (f" NOT at {bad}" if bad else ""))

    print()
    print("Where the typed arm's reduction serialises (one accumulator) and an attack recovers it:")
    for fn in sorted(os.listdir(asmdir)):
        if not fn.endswith(".s"):
            continue
        w = int(fn[1:-2])
        s = os.path.join(asmdir, fn)
        t = loopshape.analyse(s, f"w{w}_typed", w)
        g = loopshape.analyse(s, f"w{w}_gather", w)
        wd = loopshape.analyse(s, f"w{w}_wide", w)
        if t.get("error"):
            continue
        if t["accumulators"] <= 1 < max(g.get("accumulators", 0), wd.get("accumulators", 0)):
            print(f"  W={w:<3} typed accs={t['accumulators']} chain={t['chain']}  ->  "
                  f"gather accs={g['accumulators']} chain={g['chain']}, "
                  f"wide accs={wd['accumulators']} chain={wd['chain']}")


if __name__ == "__main__":
    main()
