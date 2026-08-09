#!/usr/bin/env python3
"""Five erasure oracles over one emitted assembly file, weakest first.

The panel has two oracles and `17_probes/t3_opcode_oracle.out` records them
disagreeing at the aggregate. Neither can see the thing that decides whether a
loop runs at the speed its instruction count suggests, so this adds three more
and states what each is blind to.

  O1 symbol identity     the assembler folded the two symbols into one body.
                         Sound when it fires. Reports NOT ERASED for two
                         bodies that differ only in register names, which is a
                         false negative and 17 measured it.
  O2 opcode multiset     same bag of opcodes in the same quantities. Blind to
                         order, to which operand feeds which, and therefore to
                         the whole dependency structure. Two bodies with the
                         same bag can compute different functions.
  O3 alpha sequence      identical instruction sequence after renaming
                         registers by first occurrence. Sees order. Because
                         the renaming is by first occurrence it is itself
                         order-sensitive, so it cannot see through a
                         reschedule; that is what O4 is for.
  O4 value-numbered      each instruction is keyed on its mnemonic, its
     dataflow            immediates, and the value numbers of its operands
                         rather than their register names. Two bodies match
                         when they compute the same dataflow graph, whatever
                         registers or order the allocator chose. Live-ins are
                         put into correspondence by their use signatures, not
                         by register name or first-use order. Where two
                         live-ins in one body share a signature the match is a
                         guess and the run says so. The bias is one-directional
                         by construction: O4 can fail to notice that two equal
                         bodies are equal, and cannot report two different
                         bodies as equal.
  O5 recurrence length   longest chain of dependent instructions through the
                         loop block. This is what a latency-bound loop is
                         limited by, and it is invisible to every oracle above.

What none of these is: a measurement of speed. O5 counts dependent
instructions, not cycles. It has no latency table, no port model, no issue
width and no memory model, so it cannot tell you that a body with a longer
recurrence is slower, only that it has more dependent work on its longest
chain. Where a magnitude is wanted, the bench harness at `mock/benches/` is the
only thing in this repository that can supply one, and nothing here has run on
it. Every number this script prints is a count.

Usage:  python3 oracle.py <file.s> <symA> <symB>
        python3 oracle.py <file.s> --pairs a1:b1,a2:b2
        python3 oracle.py <file.s> --pairs ... --terse
"""

import re
import sys
from collections import Counter

REG = re.compile(r"\b(?:([xw])(\d{1,2})|([vqsdhb])(\d{1,2})|(sp|xzr|wzr|lr|fp))\b")
INSTR = re.compile(r"^\s+([a-z][a-z0-9._]*)\s*(.*)$")
LABEL = re.compile(r"^(L[A-Za-z0-9_$.]+):")
SYMDEF = re.compile(r"^_([A-Za-z0-9_$.]+):")
SYMALIAS = re.compile(r"^_([A-Za-z0-9_$.]+)\s*=\s*_([A-Za-z0-9_$.]+)\s*$")
DIRECTIVE = re.compile(r"^\s*\.")
IMM = re.compile(r"#-?(?:0x[0-9a-fA-F]+|\d+)")


def parse(path):
    """Return (bodies, aliases). A body is a list of (label|None, mnemonic, operands)."""
    bodies, aliases = {}, {}
    cur, curlabel = None, None
    for line in open(path):
        line = line.rstrip("\n")
        m = SYMALIAS.match(line)
        if m:
            aliases[m.group(1)] = m.group(2)
            cur = None
            continue
        m = SYMDEF.match(line)
        if m:
            cur, curlabel = m.group(1), None
            bodies[cur] = []
            continue
        if cur is None:
            continue
        m = LABEL.match(line)
        if m:
            curlabel = m.group(1)
            continue
        if DIRECTIVE.match(line):
            if ".cfi_endproc" in line:
                cur = None
            continue
        m = INSTR.match(line)
        if m:
            bodies[cur].append((curlabel, m.group(1), m.group(2).strip()))
    return bodies, aliases


def resolve(bodies, aliases, sym):
    seen = set()
    while sym in aliases and sym not in seen:
        seen.add(sym)
        sym = aliases[sym]
    return bodies.get(sym), sym


def regs_in(ops):
    """Architectural register ids in operand order. x5 and w5 are the same register."""
    out = []
    for m in REG.finditer(ops):
        if m.group(2) is not None:
            out.append(("g", m.group(2)))
        elif m.group(4) is not None:
            out.append(("v", m.group(4)))
        else:
            out.append(("s", m.group(5)))
    return out


STORE = re.compile(r"^st")
BRANCH = re.compile(r"^(b|cb|tb|ret)")
NO_DEST = {"cmp", "cmn", "tst", "ret", "nop"}


def defs_uses(mn, ops):
    """def/use split for the aarch64 subset these probes emit."""
    rs = regs_in(ops)
    base = mn.split(".")[0]
    if not rs:
        return [], []
    if base in NO_DEST or BRANCH.match(base) or STORE.match(base):
        return [], rs
    if base == "ldp":
        return rs[:2], rs[2:]
    return [rs[0]], rs[1:]


def hottest_block(body):
    """The block that is the target of a branch appearing inside itself: the loop."""
    blocks, order = {}, []
    for lab, mn, ops in body:
        if lab not in blocks:
            blocks[lab] = []
            order.append(lab)
        blocks[lab].append((mn, ops))
    for lab in order:
        if lab is None:
            continue
        for mn, ops in blocks[lab]:
            if BRANCH.match(mn) and lab in ops:
                return lab, blocks[lab]
    lab = max(order, key=lambda k: len(blocks[k]))
    return lab, blocks[lab]


def live_in_signature(instrs):
    """Signature of each live-in register: the sorted multiset of its use sites.

    Used to put the two bodies' live-ins into correspondence without assuming
    the allocator picked the same registers or the scheduler the same order.
    """
    defined, sig = set(), {}
    for mn, ops in instrs:
        d, u = defs_uses(mn, ops)
        for i, r in enumerate(u):
            if r not in defined:
                sig.setdefault(r, []).append((mn, i))
        for r in d:
            defined.add(r)
    return {r: tuple(sorted(v)) for r, v in sig.items()}


def value_number(instrs, seed, intern=None):
    """Canonical dataflow multiset for a straight-line block.

    `seed` maps a live-in register to a symbol shared with the other body, so
    the comparison never rests on the allocator having chosen the same
    registers or the scheduler the same order.

    Value numbers are interned integers rather than nested strings. The first
    version of this built each key by substituting its operands' keys inline,
    which grows exponentially in dependency-chain depth: on a fully unrolled
    64-element reduction it consumed the machine's memory and the process was
    killed. Interning makes the key size constant per instruction.

    `intern` must be SHARED between the two bodies being compared. Interning is
    order-dependent, so a table per body assigns different numbers to the same
    structure and every comparison reports a difference. Keys are built
    bottom-up from the shared seeds, so one shared table is deterministic.
    """
    vn, out, unseeded = {}, [], 0
    if intern is None:
        intern = {}

    def num(k):
        if k not in intern:
            intern[k] = len(intern)
        return intern[k]

    for r, sym in seed.items():
        vn[r] = num(("seed", sym))

    for mn, ops in instrs:
        d, u = defs_uses(mn, ops)
        keys = []
        for r in u:
            if r not in vn:
                vn[r] = num(("unseeded", unseeded))
                unseeded += 1
            keys.append(vn[r])
        imms = tuple(IMM.findall(ops))
        shifts = tuple(re.findall(r"\b(lsl|lsr|asr|ror|uxt[bhw]|sxt[bhw])\b", ops))
        key = (mn, tuple(keys), imms, shifts)
        out.append(key)
        for i, r in enumerate(d):
            vn[r] = num((key, i))
    return out


def paired_seeds(ia, ib):
    """Seed both blocks' live-ins with shared symbols, matched by signature.

    Returns (seed_a, seed_b, ambiguous). `ambiguous` is True when two live-ins
    in one body share a signature, in which case the correspondence is a guess
    and a reported difference may not be one.
    """
    sa, sb = live_in_signature(ia), live_in_signature(ib)
    groups_a, groups_b = {}, {}
    for r, s in sa.items():
        groups_a.setdefault(s, []).append(r)
    for r, s in sb.items():
        groups_b.setdefault(s, []).append(r)
    seed_a, seed_b, ambiguous = {}, {}, False
    for s in set(groups_a) | set(groups_b):
        ga, gb = groups_a.get(s, []), groups_b.get(s, [])
        if len(ga) > 1 or len(gb) > 1:
            ambiguous = True
        for i, r in enumerate(sorted(ga)):
            seed_a[r] = f"in[{s}]#{i}"
        for i, r in enumerate(sorted(gb)):
            seed_b[r] = f"in[{s}]#{i}"
    return seed_a, seed_b, ambiguous


def recurrence(instrs):
    """Longest chain of dependent instructions in the block, in instruction count.

    A lower bound on issue-slot depth. Not a cycle count: no latency table, no
    port model, no memory model.
    """
    depth, best = {}, 0
    for mn, ops in instrs:
        d, u = defs_uses(mn, ops)
        mine = max([depth.get(r, 0) for r in u], default=0) + 1
        for r in d:
            depth[r] = mine
        best = max(best, mine)
    return best


def alpha_seq(body):
    rmap, lmap, out = {}, {}, []
    for lab, mn, ops in body:
        if lab is not None and lab not in lmap:
            lmap[lab] = f"L{len(lmap)}"
        toks = []
        for r in regs_in(ops):
            if r not in rmap:
                rmap[r] = f"r{len(rmap)}"
            toks.append(rmap[r])
        imms = tuple(IMM.findall(ops))
        out.append((mn, tuple(toks), imms))
    return out


def compare(path, a, b):
    bodies, aliases = parse(path)
    ba, ra = resolve(bodies, aliases, a)
    bb, rb = resolve(bodies, aliases, b)
    if ba is None or bb is None:
        return {"pair": f"{a} vs {b}", "error": f"symbol not found ({a if ba is None else b})"}

    ca, cb = Counter(m for _, m, _ in ba), Counter(m for _, m, _ in bb)
    la, ia = hottest_block(ba)
    lb, ib = hottest_block(bb)
    seed_a, seed_b, ambiguous = paired_seeds(ia, ib)
    intern = {}
    va, vb = value_number(ia, seed_a, intern), value_number(ib, seed_b, intern)
    reca, recb = recurrence(ia), recurrence(ib)

    return {
        "pair": f"{a} vs {b}",
        "n_a": len(ba), "n_b": len(bb),
        "O1": ra == rb,
        "O2": ca == cb,
        "O3": alpha_seq(ba) == alpha_seq(bb),
        "O4": Counter(va) == Counter(vb),
        "O4_seq": va == vb, "O4_ambiguous": ambiguous,
        "O5_a": reca, "O5_b": recb, "O5_eq": reca == recb,
        "loop_a": len(ia), "loop_b": len(ib),
        "delta": {k: (ca.get(k, 0), cb.get(k, 0))
                  for k in sorted(set(ca) | set(cb)) if ca.get(k, 0) != cb.get(k, 0)},
    }


def verdict(r):
    if r.get("error"):
        return "ERROR"
    if r["O1"]:
        return "ERASED: folded to one body"
    if r["O3"]:
        return "ERASED: same sequence, different registers"
    if r["O4_seq"]:
        return "ERASED: same dataflow in the same order"
    if r["O4"] and r["O5_eq"]:
        return "ERASED: same dataflow, rescheduled, same recurrence"
    if r["O4"]:
        return "RESCHEDULED: same dataflow, recurrence differs"
    if r["O2"]:
        return "SAME BAG, DIFFERENT DATAFLOW"
    return "NOT ERASED"


def main():
    argv = [a for a in sys.argv[1:] if a != "--terse"]
    terse = "--terse" in sys.argv
    path = argv[0]
    pairs = ([p.split(":") for p in argv[2].split(",")]
             if argv[1] == "--pairs" else [(argv[1], argv[2])])
    for a, b in pairs:
        r = compare(path, a, b)
        if terse:
            if r.get("error"):
                print(f"{r['pair']:44s} ERROR {r['error']}")
            else:
                print(f"{r['pair']:44s} {r['n_a']:4d}/{r['n_b']:4d}  "
                      f"O1={int(r['O1'])} O2={int(r['O2'])} O3={int(r['O3'])} "
                      f"O4={int(r['O4'])} rec={r['O5_a']}/{r['O5_b']}  {verdict(r)}")
            continue
        print(r["pair"])
        if r.get("error"):
            print(f"  {r['error']}\n")
            continue
        print(f"  instructions        : {r['n_a']} vs {r['n_b']}")
        print(f"  O1 symbol identity  : {r['O1']}")
        print(f"  O2 opcode multiset  : {r['O2']}")
        print(f"  O3 alpha sequence   : {r['O3']}")
        print(f"  O4 value-numbered   : {r['O4']}  (in order: {r['O4_seq']}{', live-in match ambiguous' if r['O4_ambiguous'] else ''})")
        print(f"  O5 recurrence       : {r['O5_a']} vs {r['O5_b']}"
              f"  (loop block {r['loop_a']} vs {r['loop_b']} instrs)")
        if r["delta"]:
            print(f"  opcode delta        : {r['delta']}")
        print(f"  VERDICT             : {verdict(r)}\n")


if __name__ == "__main__":
    main()
