# p9: exhaustive check. Which functions from the two-lane truth algebra to the
# one-lane one preserve the Boolean-algebra structure? n = 2 and n = 3, all
# 2^(2^n) candidate functions enumerated, no sampling.
import itertools
def check(n):
    pts = list(itertools.product([0,1], repeat=n))
    homs, all_f, any_f = [], None, None
    for bits in itertools.product([0,1], repeat=len(pts)):
        f = dict(zip(pts, bits))
        AND = lambda a,b: tuple(x&y for x,y in zip(a,b))
        OR  = lambda a,b: tuple(x|y for x,y in zip(a,b))
        NOT = lambda a: tuple(1-x for x in a)
        TOP, BOT = tuple([1]*n), tuple([0]*n)
        ok = (f[TOP]==1 and f[BOT]==0
              and all(f[AND(a,b)] == (f[a]&f[b]) for a in pts for b in pts)
              and all(f[OR(a,b)]  == (f[a]|f[b]) for a in pts for b in pts)
              and all(f[NOT(a)]   == 1-f[a]      for a in pts))
        if ok: homs.append(f)
        if all(f[p]==min(p) for p in pts): all_f = f
        if all(f[p]==max(p) for p in pts): any_f = f
    projections = [ {p: p[i] for p in pts} for i in range(n) ]
    print(f"n={n}: candidate functions {2**len(pts)}, homomorphisms {len(homs)}")
    print(f"  homomorphisms == the {n} coordinate projections: {sorted(map(str,homs))==sorted(map(str,projections))}")
    print(f"  'all' is a homomorphism: {all_f in homs}")
    print(f"  'any' is a homomorphism: {any_f in homs}")
check(2); check(3)
