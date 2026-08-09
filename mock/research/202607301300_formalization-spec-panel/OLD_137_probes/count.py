import re,sys
txt=open(sys.argv[1]).read().splitlines()
cur=None; counts={}
for l in txt:
    m=re.match(r'^_([A-Za-z0-9_]+):',l)
    if m:
        cur=m.group(1); counts[cur]=0; continue
    if cur is None: continue
    s=l.strip()
    if not s or s.startswith('.') or s.startswith(';') or s.endswith(':'): continue
    counts[cur]+=1
for k in sorted(counts): print(f"{k:24s} {counts[k]}")
print("--- aliases ---")
for l in txt:
    if re.match(r'^_[A-Za-z0-9_]+ = ', l): print(l.strip())
