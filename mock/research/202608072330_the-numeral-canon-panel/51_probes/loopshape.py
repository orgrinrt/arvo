#!/usr/bin/env python3
"""Loop-carried structure of an emitted reduction loop.

The oracles in `oracle.py` compare two bodies. This asks a different question
about one body, and it is the question a reduction actually turns on: how much
serial dependent work stands between one element and the next.

For a sum over N elements the binding constraint is usually not the instruction
count. It is the loop-carried chain: each accumulate depends on the previous
one, so a loop with a single accumulator cannot retire faster than one add
latency per element however wide the machine is. Splitting the reduction across
k independent accumulators divides that bound by k, which is why compilers
unroll reductions and why an unrolled loop and a rolled one with the same
instruction count are not the same program.

What this reports per symbol:

  loop instrs      instructions in the loop block.
  carried          registers live across the back edge: used in the block
                   before the block defines them, and defined in it.
  accumulators     carried registers whose defining instruction is an add-like
                   op taking a value derived from a load. These are the
                   reduction chains.
  chain            longest loop-carried dependency chain, in dependent
                   instructions per iteration.
  step             the induction increment where one is identifiable, which
                   with the declared width gives elements per iteration.

Not a cycle count. No latency table, no port model, no issue width, no memory
model. `chain / elements-per-iteration` is a count of dependent instructions
per element and is a lower bound on nothing in particular until a machine model
is attached to it. Where a magnitude is wanted the bench harness is the only
thing that can supply one.

  python3 loopshape.py <file.s> <sym> [<sym> ...] [--width N]
"""

import re
import sys

import oracle

ADDLIKE = {"add", "adds", "sub", "subs", "orr", "eor", "madd", "mla", "fadd"}


def analyse(path, sym, width=None):
    bodies, aliases = oracle.parse(path)
    body, resolved = oracle.resolve(bodies, aliases, sym)
    if body is None:
        return {"sym": sym, "error": "not found"}
    lab, ins = oracle.hottest_block(body)

    defined, carried = set(), []
    for mn, ops in ins:
        d, u = oracle.defs_uses(mn, ops)
        for r in u:
            if r not in defined and r not in carried:
                carried.append(r)
        for r in d:
            defined.add(r)
    carried = [r for r in carried if r in defined]

    # which carried registers are reduction accumulators, and which are induction
    accs, defsite = [], {}
    for mn, ops in ins:
        d, u = oracle.defs_uses(mn, ops)
        for r in d:
            defsite[r] = (mn, ops)
    loaded = set()
    for mn, ops in ins:
        d, _ = oracle.defs_uses(mn, ops)
        if mn.startswith("ld"):
            loaded.update(d)
    # propagate "derived from a load" forward
    changed = True
    while changed:
        changed = False
        for mn, ops in ins:
            d, u = oracle.defs_uses(mn, ops)
            if any(r in loaded for r in u):
                for r in d:
                    if r not in loaded:
                        loaded.add(r)
                        changed = True
    for r in carried:
        mn, ops = defsite.get(r, ("", ""))
        base = mn.split(".")[0]
        if base in ADDLIKE:
            _, u = oracle.defs_uses(mn, ops)
            if any(x in loaded for x in u):
                accs.append(r)

    # induction step: a carried register redefined by add/sub with an immediate
    steps = []
    for r in carried:
        mn, ops = defsite.get(r, ("", ""))
        if mn.split(".")[0] in ("add", "sub", "adds", "subs"):
            imm = re.findall(r"#(\d+)", ops)
            _, u = oracle.defs_uses(mn, ops)
            if imm and r in u and r not in accs:
                steps.append(int(imm[0]))

    chain = oracle.recurrence(ins)
    elems = None
    if width:
        cands = [s // width for s in steps if s % width == 0 and s // width > 0]
        if cands:
            elems = min(cands)

    return {
        "sym": sym, "resolved": resolved, "loop_instrs": len(ins),
        "carried": len(carried), "accumulators": len(accs),
        "chain": chain, "steps": sorted(set(steps)), "elems_per_iter": elems,
        "loads": sum(1 for mn, _ in ins if mn.startswith("ld")),
    }


def main():
    args = [a for a in sys.argv[1:]]
    width = None
    if "--width" in args:
        i = args.index("--width")
        width = int(args[i + 1])
        del args[i:i + 2]
    path, syms = args[0], args[1:]
    print(f"{'symbol':<16} {'loop':>5} {'lds':>4} {'carr':>5} {'accs':>5} "
          f"{'chain':>6} {'e/it':>5} {'chain/elem':>11}  steps")
    for s in syms:
        r = analyse(path, s, width)
        if r.get("error"):
            print(f"{s:<16} {r['error']}")
            continue
        ce = f"{r['chain'] / r['elems_per_iter']:.2f}" if r["elems_per_iter"] else "?"
        print(f"{r['sym']:<16} {r['loop_instrs']:>5} {r['loads']:>4} {r['carried']:>5} "
              f"{r['accumulators']:>5} {r['chain']:>6} "
              f"{r['elems_per_iter'] if r['elems_per_iter'] else '?':>5} {ce:>11}  {r['steps']}")


if __name__ == "__main__":
    main()
