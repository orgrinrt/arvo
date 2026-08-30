#!/usr/bin/env python3
"""
P6. P5 reported 461/6561 pairs where the sum-of-widths product form is not the
least ADMITTED shape, and separately 15/6561 where the tight answer needs
negative integer width. This file characterises those two populations exactly,
because a count without a region is not a finding, and my own hand-derivation of
the region disagreed with the measurement by four pairs.
"""
from fractions import Fraction as Fr
LIM = 8

def tight_unclamped(a, b):
    I1, F1 = a; I2, F2 = b
    W1, W2 = I1+F1, I2+F2
    if W1 == 0 or W2 == 0: return (0, 0)
    F = F1+F2
    mx = Fr(2**W1-1, 2**F1) * Fr(2**W2-1, 2**F2)
    # least I, allowing negative, with 2^I - 2^-F >= mx
    I = -F
    while Fr(2**I if I >= 0 else 1, 1 if I >= 0 else 2**(-I)) - Fr(1, 2**F) < mx:
        I += 1
    return (I, F)

def clamped(a, b):
    I, F = tight_unclamped(a, b)
    return (max(I, 0), F)

def naive(a, b): return (a[0]+b[0], a[1]+b[1])

S = [(I, F) for I in range(0, LIM+1) for F in range(0, LIM+1)]
P = [(a, b) for a in S for b in S]

waste = [(a,b) for a,b in P if naive(a,b) != clamped(a,b)]
neg   = [(a,b) for a,b in P if tight_unclamped(a,b)[0] < 0]
print(f"# P6. box I,F in 0..{LIM}: {len(S)} shapes, {len(P)} pairs")
print()
print(f"## the sum-of-widths form is not the least ADMITTED shape: {len(waste)}/{len(P)}")
byW = {}
for a,b in waste:
    byW.setdefault((min(a[0]+a[1],b[0]+b[1]), max(a[0]+a[1],b[0]+b[1])), 0)
    byW[(min(a[0]+a[1],b[0]+b[1]), max(a[0]+a[1],b[0]+b[1]))] += 1
mins = sorted({k[0] for k in byW})
print(f"   minimum total width of the narrower operand, over the waste set: {mins}")
for m in mins:
    n = sum(v for k,v in byW.items() if k[0]==m)
    print(f"     narrower operand total width {m}: {n} pairs")
# is the converse true: does every pair with min total width in mins waste?
cand = [(a,b) for a,b in P if min(a[0]+a[1], b[0]+b[1]) in mins]
print(f"   pairs with narrower total width in {mins}: {len(cand)}")
print(f"   of those, wasting: {len(waste)}; NOT wasting: {len(cand)-len(waste)}")
nw = [(a,b) for a,b in cand if naive(a,b) == clamped(a,b)]
print(f"   the non-wasting exceptions, all of them: {nw}")
print()
print(f"## tight answer needs NEGATIVE integer width: {len(neg)}/{len(P)}")
print(f"   the region, stated exactly:")
sh = sorted({a for a,b in neg} | {b for a,b in neg})
print(f"   shapes appearing in it: {sh}")
print(f"   every such pair has one operand U<0,1>: "
      f"{all((0,1) in (a,b) for a,b in neg)}")
print(f"   every such pair has both operands purely fractional (I==0): "
      f"{all(a[0]==0 and b[0]==0 for a,b in neg)}")
print(f"   how the count scales with the box: linear in LIM (2*LIM-1), so the")
print(f"   FRACTION shrinks as the box grows. this is a corner, not a region.")
for L in (4,6,8):
    SS=[(I,F) for I in range(0,L+1) for F in range(0,L+1)]
    PP=[(a,b) for a in SS for b in SS]
    nn=sum(1 for a,b in PP if tight_unclamped(a,b)[0] < 0)
    print(f"     LIM={L}: {nn}/{len(PP)} = {100.0*nn/len(PP):.3f}%")
