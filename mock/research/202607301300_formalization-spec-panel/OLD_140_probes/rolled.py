import re,sys
# For each labelled function, find its loop block(s), the trip count from the
# `mov wN, #K` / `subs xN, xN, #S` pair, and compute dynamic instructions for 64 elements.
def blocks(path):
    txt=open(path).read().split('\n')
    cur=None; out={}; order=[]
    for line in txt:
        s=line.strip()
        m=re.match(r'^(_?[A-Za-z_][A-Za-z0-9_.]*):$', s)
        if m:
            cur=m.group(1); out[cur]=[]; order.append(cur); continue
        if cur and s and not s.startswith('.'):
            out[cur].append(s)
    return out,order
for path in sys.argv[1:]:
    b,order=blocks(path)
    print('==',path)
    for i,name in enumerate(order):
        body=b[name]
        if not any(x.startswith('b.') for x in body): continue
        if not name.startswith('LBB'): continue
        # find owning function = previous non-LBB label
        owner=[n for n in order[:i] if not n.startswith('LBB')]
        owner=owner[-1] if owner else '?'
        step=None
        for x in body:
            m=re.match(r'subs\s+\w+,\s*\w+,\s*#(\d+)',x)
            if m: step=int(m.group(1))
        n=len(body)
        pro=len(b[owner])
        if step:
            trips=64//step
            print(f"  {owner:22s} loop body={n} ins, step={step} elems/iter, trips={trips}, prologue={pro}"
                  f"  -> dynamic {pro + n*trips} ins for 64 elements")
        else:
            print(f"  {owner:22s} loop body={n} ins, step=? prologue={pro}")
