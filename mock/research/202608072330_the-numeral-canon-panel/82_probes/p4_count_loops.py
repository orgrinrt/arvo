#!/usr/bin/env python3
"""Counts the body of every BACKWARD-branching loop in the emitted assembly and
attributes each to the enclosing symbol. A backward branch is the definition of
a loop; picking 'the last label' (which an earlier version of this script did)
picks epilogues instead, which is why that version reported a 94-instruction
inner loop for an arm whose real loop is four instructions."""
import re, sys, collections

asm = open(sys.argv[1]).read().splitlines()
sym = None
sym_at = {}
label_at = {}
for i, l in enumerate(asm):
    s = l.strip()
    m = re.match(r'^(_[A-Za-z0-9_$.]+|__R[A-Za-z0-9_$.]+):$', s)
    if m:
        sym = m.group(1); sym_at[i] = sym
    m2 = re.match(r'^(LBB\d+_\d+):$', s)
    if m2:
        label_at[m2.group(1)] = (i, sym)

# find backward branches
loops = []
cur = None
for i, l in enumerate(asm):
    s = l.strip()
    if i in sym_at:
        cur = sym_at[i]
    m = re.match(r'^b(?:\.\w+)?\s+(LBB\d+_\d+)$|^cb\w+\s+\w+,\s*(LBB\d+_\d+)$|^tb\w+\s+\w+,\s*#?\d+,\s*(LBB\d+_\d+)$', s)
    if m:
        tgt = m.group(1) or m.group(2) or m.group(3)
        if tgt in label_at:
            ti, tsym = label_at[tgt]
            if ti < i:
                body = [x.strip() for x in asm[ti+1:i+1]
                        if x.strip() and not x.strip().startswith(('.', ';'))
                        and not re.match(r'^(LBB\d+_\d+|Lloh\d+):$', x.strip())]
                loops.append((cur, tgt, len(body), body))

by_sym = collections.OrderedDict()
for sym_, tgt, n, body in loops:
    by_sym.setdefault(sym_, []).append((tgt, n, body))

for sym_, ls in by_sym.items():
    print(f"{sym_}")
    for tgt, n, body in ls:
        vec = sum(1 for b in body if 'sqadd' in b or 'uqadd' in b)
        addv = sum(1 for b in body if re.match(r'^add\.\d+b', b))
        ld = [b for b in body if b.startswith(('ldr', 'ldp', 'ldrsb'))]
        # elements per iteration: infer from the loop decrement/increment
        step = None
        for b in body:
            m = re.match(r'^(?:subs|adds)\s+x\d+,\s*x\d+,\s*#(\d+)$', b)
            if m: step = int(m.group(1))
        print(f"   loop {tgt}: {n} instructions, "
              f"sqadd/uqadd {vec}, vector add {addv}, loads {len(ld)}, "
              f"step {step if step else '1 (no explicit stride)'}")
        if step:
            print(f"      -> {n/step:.3f} instructions per element")
        else:
            print(f"      -> {n:.3f} instructions per element (serial)")
    print()
