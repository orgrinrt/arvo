#!/usr/bin/env python3
"""P5b. The characterisation, after P5's first hypothesis was refuted by its own
control.

P5 guessed 'p mod 2^F == 2^(F-1) - 1'. The set match came back False at every
width while the COUNT matched, which is the most dangerous near-miss available:
a wrong characterisation whose cardinality is right. Recorded rather than
repaired away.

This version reads the characterisation off the data at increasing moduli until
one determines the set exactly, then predicts at widths not used to find it.

CASES THAT MUST FAIL
  C-O  The predicting condition must FAIL to determine the set at some smaller
       modulus, else 'determined at 2^(F+k)' is vacuous.
  C-P  The condition must predict exactly at widths not used to derive it.
  C-Q  A wrong condition of the same shape must not predict exactly.
"""

def rte(v, frm, to):
    if to >= frm:
        return v << (to - frm)
    s = frm - to
    half = 1 << (s - 1)
    lo = v & ((1 << s) - 1)
    hi = v >> s
    if lo > half: return hi + 1
    if lo < half: return hi
    return hi + 1 if (hi & 1) else hi

def disagreeing(F):
    M = 2 * F - 1
    n = 1 << F
    return {(a, b) for a in range(n) for b in range(n)
            if rte(a*b, 2*F, F) != rte(rte(a*b, 2*F, M), M, F)}

F = 8
n = 1 << F
D = disagreeing(F)
print(f"F={F}: |disagreeing| = {len(D)} = 2^(F-1) = {1<<(F-1)}")
print()
print("--- find the smallest modulus 2^k on the exact product that determines the set ---")
determined_at = None
for k in range(F, 2*F + 1):
    m = 1 << k
    res = sorted({(a*b) % m for a, b in D})
    # does membership in that residue set predict D exactly?
    pred = {(a, b) for a in range(n) for b in range(n) if (a*b) % m in set(res)}
    ok = pred == D
    print(f"  modulus 2^{k:<2}  residues {res if len(res)<=6 else str(res[:6])+'...'}  predicts exactly: {ok}")
    if ok and determined_at is None:
        determined_at = (k, set(res))
print()
if determined_at is None:
    print("NO modulus in the swept range determines it. The characterisation is not a")
    print("congruence on the product alone, and P5's shape was wrong at the root.")
else:
    k, res = determined_at
    print(f"C-O  smaller moduli fail: {'yes' if k > F else 'NO, determined at the smallest tried'}")
    print(f"     determined at modulus 2^{k}, residues {sorted(res)}")
    print(f"     in units of 2^(F-1): {[r/(1<<(F-1)) for r in sorted(res)]}")
    print()
    print("--- C-P: predict at widths not used to derive it ---")
    for FF in (4, 5, 6, 7, 9):
        nn = 1 << FF
        DD = disagreeing(FF)
        kk = k - F + FF          # the modulus, scaled with the width
        mm = 1 << kk
        # scale the residues the same way
        rr = {r * (1 << (FF - F)) if FF >= F else r >> (F - FF) for r in res}
        pred = {(a, b) for a in range(nn) for b in range(nn) if (a*b) % mm in rr}
        print(f"  F={FF}  |D|={len(DD):>5}  2^(F-1)={1<<(FF-1):>5}  count ok {len(DD)==(1<<(FF-1))}  set ok {pred==DD}")
    print()
    print("--- C-Q: a wrong condition of the same shape ---")
    bad = {(r + 2) % (1 << k) for r in res}
    predbad = {(a, b) for a in range(n) for b in range(n) if (a*b) % (1 << k) in bad}
    print(f"  residues shifted by 2 predict exactly: {predbad == D}   (must be False)")
