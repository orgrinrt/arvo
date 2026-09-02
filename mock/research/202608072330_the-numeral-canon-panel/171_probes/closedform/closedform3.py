#!/usr/bin/env python3
"""P5c. The characterisation as a FORMULA in F, after P5b determined it at one width.

P5b found the set is determined by the exact product's residue modulo 2^(F+1),
with residues {2^(F-1)+1, 3*2^(F-1)-1} at F = 8. P5b's cross-width check failed
because its residue-scaling rule was ad hoc, not because the characterisation was
wrong. This version states the formula and tests it.

  H:  a*b mod 2^(F+1)  in  { 2^(F-1) + 1 ,  3*2^(F-1) - 1 }

CASES THAT MUST FAIL
  C-R  H must predict the set exactly at EVERY width tested, including the four
       169 did not report.
  C-S  Each of the two residues alone must FAIL, else the pair is not needed and
       the formula is over-specified.
  C-T  The two residues' neighbours (+/-2) must fail, else the formula is not
       locating anything in particular.
"""

def rte(v, frm, to):
    if to >= frm: return v << (to - frm)
    s = frm - to; half = 1 << (s - 1)
    lo = v & ((1 << s) - 1); hi = v >> s
    if lo > half: return hi + 1
    if lo < half: return hi
    return hi + 1 if (hi & 1) else hi

def disagreeing(F):
    M = 2*F - 1; n = 1 << F
    return {(a, b) for a in range(n) for b in range(n)
            if rte(a*b, 2*F, F) != rte(rte(a*b, 2*F, M), M, F)}

def predict(F, residues):
    n = 1 << F; m = 1 << (F + 1)
    return {(a, b) for a in range(n) for b in range(n) if (a*b) % m in residues}

print("H:  a*b mod 2^(F+1)  in  { 2^(F-1)+1 , 3*2^(F-1)-1 }")
print()
print(f"{'F':>3} {'|D|':>7} {'2^(F-1)':>9} {'count ok':>9} {'H exact':>9} {'r1 only':>9} {'r2 only':>9} {'shifted':>9}")
allok = True
for F in range(4, 11):
    D = disagreeing(F)
    h = 1 << (F - 1)
    r1, r2 = h + 1, 3*h - 1
    m = 1 << (F + 1)
    ok  = predict(F, {r1, r2}) == D
    o1  = predict(F, {r1}) == D
    o2  = predict(F, {r2}) == D
    sh  = predict(F, {(r1 + 2) % m, (r2 + 2) % m}) == D
    allok &= ok
    print(f"{F:>3} {len(D):>7} {h:>9} {str(len(D)==h):>9} {str(ok):>9} {str(o1):>9} {str(o2):>9} {str(sh):>9}")

print()
print(f"C-R  H predicts exactly at every width 4..10: {allok}   (must be True)")
print("C-S  the 'r1 only' and 'r2 only' columns must be all False")
print("C-T  the 'shifted' column must be all False")
print()
print("READING. 2^(F-1) and 3*2^(F-1) are the two ODD multiples of 2^(F-1) modulo")
print("2^(F+1): the tie points of the F-level rounding. The disagreeing products sit")
print("exactly one unit from a tie, on the side the single discarded bit rounds ONTO")
print("it, where ties-to-even then breaks the other way from the direct rounding.")
print("The count is 2^(F-1) because that is how many products land there.")
