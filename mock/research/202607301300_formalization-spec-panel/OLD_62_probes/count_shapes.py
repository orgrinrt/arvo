# File 62. Whole-function instruction and branch counts for the five delivery
# shapes in the rebuilt union crate's e_codegen asm. The count is the whole
# function body between the label and .cfi_endproc, which is a coarser measure
# than file 08's "loop body only"; the branch count is comparable directly.
import re
import sys

asm = open(sys.argv[1] if len(sys.argv) > 1 else "/tmp/union-rebuild-62/e.s").read().splitlines()
funcs = {}
cur = None
for line in asm:
    m = re.match(r"^_(u_\w+):", line)
    if m:
        cur = m.group(1)
        funcs[cur] = {"instr": 0, "branch": 0}
        continue
    if cur and re.match(r"^\t\.cfi_endproc", line):
        cur = None
        continue
    if cur and re.match(r"^\t[a-z]", line):
        funcs[cur]["instr"] += 1
        if re.match(r"^\t(b|b\.\w+|cbz|cbnz|tbz|tbnz)\t", line):
            funcs[cur]["branch"] += 1
for name, counts in sorted(funcs.items()):
    print(name, counts)
