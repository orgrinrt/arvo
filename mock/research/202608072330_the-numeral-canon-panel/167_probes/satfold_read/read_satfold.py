#!/usr/bin/env python3
"""Read committed satfold harness findings into the crossover table used by 167 section 7.

Run from mock/benches/:  python3 <this>/read_satfold.py

NOTE ON A REAL BUG IN v1 OF THIS SCRIPT, kept because a plausible table is what it produced.
Splitting the file on '## Statistical comparison' and iterating the remainder let the later
'Bridge overhead per variant' table (whose values are single-digit ns) overwrite every median.
The fix is the '\n## ' stop below plus the first-match-wins guard.
"""
import re, glob, sys

L = [8, 15, 16, 17, 32, 63, 64, 65, 128, 256, 1024, 4096]
ARMS = ['satfold-seq', 'satfold-iterfold', 'satfold-nolaw', 'satfold-lanes4-idx',
        'satfold-lanes16', 'satfold-lanes16-constl', 'satfold-lanes64',
        'satfold-neon', 'satfold-neon8']

rows = {}
for f in sorted(glob.glob('satfold-length-l1_n*_findings.md')):
    key = int(re.search(r'_n(\d+)_findings', f).group(1))
    li, nc, al, op = key // 1000 - 1, (key // 100) % 10, (key // 10) % 10, key % 10
    if nc or al or op:
        continue
    sec = open(f).read().split('## Statistical comparison')[1].split('\n## ')[0]
    d = {}
    for line in sec.splitlines():
        m = re.match(r'\|\s*(satfold-[\w-]+)\s*\|\s*([\d.]+)ns', line)
        if m and m.group(1) not in d:
            d[m.group(1)] = float(m.group(2))
    rows[L[li]] = d

if not rows:
    sys.exit("no findings files found; run from mock/benches/")

print("Committed harness output, satfold-length-l1, 32 KiB column, aligned, saturating add.")
print("profile = release (harness loads variants from target/release); host per the meta.json.")
print()
print(f"{'L':>6} " + " ".join(f"{a.replace('satfold-',''):>12}" for a in ARMS) + f"  {'winner':>14} {'seq/win':>8}")
for l in L:
    if l not in rows:
        continue
    d = rows[l]
    present = {a: d[a] for a in ARMS if a in d}
    w = min(present, key=present.get)
    print(f"{l:>6} " + " ".join((f"{d[a]:>12.0f}" if a in d else f"{'-':>12}") for a in ARMS)
          + f"  {w.replace('satfold-',''):>14} {d['satfold-seq']/present[w]:>7.1f}x")

print()
better = sum(1 for l in rows if rows[l]['satfold-lanes16-constl'] < rows[l]['satfold-lanes16'])
print(f"lanes16-constl (fold length as a const generic) is faster at {better} of {len(rows)} lengths.")
