#!/usr/bin/env python3
"""Compare two emitted function bodies for instruction identity.

FIRST VERSION OF THIS COMPARISON WAS WRONG and the wrongness is kept here rather
than quietly fixed: it compared raw text, so LBB0_3 against LBB1_3 read as a
difference and it reported the guarded fold as differing from the bare one when
the two are instruction-for-instruction the same.  Local label indices are
allocated per function and carry no content.  Normalising them is the fix.
"""
import re, sys

def bodies(path):
    out, cur, buf = {}, None, []
    for line in open(path):
        m = re.match(r'^(__RN[\w$.]*):\s*$', line)
        if m:
            if cur: out[cur] = buf
            cur, buf = m.group(1), []
            continue
        if cur is not None:
            if line.startswith('\t.cfi_endproc'):
                out[cur] = buf; cur, buf = None, []
                continue
            s = line.strip()
            if not s or s.startswith('.'):
                continue
            buf.append(re.sub(r'LBB\d+_', 'L_', re.sub(r'\s+', ' ', s)))
    if cur: out[cur] = buf
    return out

b = bodies(sys.argv[1])
keys = sorted(b)
print(f"{'symbol':<70} {'insns':>6}")
for k in keys:
    print(f"{k:<70} {len(b[k]):>6}")
base = [k for k in keys if k.endswith('9fold_bare')]
if base:
    ref = b[base[0]]
    print()
    for k in keys:
        if k == base[0]: continue
        print(f"identical to fold_bare after label normalisation: {b[k]==ref}  <- {k}")
print()
print("aliased symbols in the file (equal-sign directives):")
for line in open(sys.argv[1]):
    if ' = __RN' in line:
        print("  " + line.strip())
