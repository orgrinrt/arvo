import re,sys
# count instructions per symbol in an aarch64 .s file
def parse(path):
    syms={}; cur=None
    for line in open(path):
        s=line.strip()
        if not s: continue
        m=re.match(r'^_?([A-Za-z_][A-Za-z0-9_]*):\s*(;.*)?$', s)
        if m and not s.startswith('.'):
            cur=m.group(1); syms.setdefault(cur,{'ins':0,'simd':0,'br':0,'ops':[]}); continue
        if s.startswith('.') or s.startswith(';') or s.startswith('//'): continue
        if cur is None: continue
        op=s.split()[0]
        if op.endswith(':'): continue
        syms[cur]['ins']+=1
        syms[cur]['ops'].append(op)
        if re.search(r'\.\d+[bhsdq]\b', s) or re.match(r'^(ld|st)[pr]?\s+q', s): syms[cur]['simd']+=1
        if op.startswith('b.') or op in ('b','cbz','cbnz','tbz','tbnz'): syms[cur]['br']+=1
    return syms
if __name__=='__main__':
    for p in sys.argv[1:]:
        print('==',p)
        for k,v in parse(p).items():
            if v['ins']==0: continue
            from collections import Counter
            c=Counter(v['ops'])
            print(f"  {k:28s} ins={v['ins']:4d} simd={v['simd']:3d} br={v['br']:2d}  {dict(c.most_common(6))}")
