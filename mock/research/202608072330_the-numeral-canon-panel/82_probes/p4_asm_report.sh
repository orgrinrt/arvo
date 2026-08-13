#!/usr/bin/env bash
# Extracts, per emitted symbol, the body and the inner-loop body, and reports
# which arms alias to the same symbol. The aliasing report is 68's instrument:
# two arms that differ only in the declared window must assemble to one symbol
# if the declaration is erased.
set -euo pipefail
cd "$(dirname "$0")"
ASM=p4_lib.s

echo "=== symbol aliases (declaration erasure check) ==="
awk '/^_[a-z_]+:/ {name=$0; sub(":","",name); print NR"\t"name}' "$ASM" | while IFS=$'\t' read -r line name; do
  prev=$((line-1))
  sed -n "${prev}p" "$ASM" | grep -q '^_[a-z_]*:' && echo "  ALIAS: $name shares an address with the preceding label" || true
done
echo
grep -n '^_[a-z_]*:' "$ASM" | sed 's/^/  label: /'
echo
echo "=== per-symbol instruction counts and inner-loop bodies ==="
python3 - "$ASM" <<'PY'
import sys, re
asm = open(sys.argv[1]).read().splitlines()
# collect label -> line index
labels = [(i, l[:-1]) for i, l in enumerate(asm) if re.match(r'^_[A-Za-z0-9_]+:$', l)]
# group consecutive labels (aliases)
groups = []
i = 0
while i < len(labels):
    grp = [labels[i][1]]
    j = i + 1
    while j < len(labels) and labels[j][0] == labels[j-1][0] + 1:
        grp.append(labels[j][1]); j += 1
    groups.append((labels[j-1][0], grp))
    i = j
ends = [g[0] for g in groups[1:]] + [len(asm)]
for (start, names), end in zip(groups, ends):
    body = [l.strip() for l in asm[start+1:end]
            if l.strip() and not l.strip().startswith(('.', ';', 'LBB' ))]
    body = [l for l in body if not re.match(r'^L?BB?\d', l) and not l.endswith(':')]
    ins = len(body)
    # inner loop: the backward branch target with the largest label index
    text = "\n".join(asm[start+1:end])
    loops = re.findall(r'^(LBB\d+_\d+):', text, re.M)
    inner = ""
    if loops:
        last = loops[-1]
        m = re.search(rf'^{last}:\n(.*?)(?=^LBB|\Z)', text, re.M | re.S)
        if m:
            lb = [l.strip() for l in m.group(1).splitlines()
                  if l.strip() and not l.strip().startswith(('.', ';'))]
            inner = f"{len(lb)} instr: " + " ; ".join(lb[:14])
    print(f"  {' = '.join(names)}")
    print(f"     total emitted instructions: {ins}")
    if inner:
        print(f"     last loop body: {inner}")
    vecs = [l for l in body if 'sqadd' in l or 'uqadd' in l or '.16b' in l]
    print(f"     vector saturating-add instructions in body: {sum(1 for l in body if 'sqadd' in l)}")
    print()
PY
