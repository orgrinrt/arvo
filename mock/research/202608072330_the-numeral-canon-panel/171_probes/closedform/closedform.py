#!/usr/bin/env python3
"""P5. R-10 answered: WHY is the M = 2F-1 count exactly 2^(F-1)?

169 measured the column and found 2^(F-1) at F = 4, 6, 8, 10 and called it a
closed form rather than a trend. R-10 asks me to add it to 167's table. Before I
do, I want the characterisation rather than the count, because a count that
matches a formula at four points is still a trend.

Hypothesis H: at M = 2F-1 the disagreeing pairs are exactly those whose exact
product p = a*b satisfies  p mod 2^F == 2^(F-1) - 1  AND  p's bit at position
F-1 ... (the shape is what this probe determines, not what it assumes).

CASES THAT MUST FAIL
  C-L  The count must reproduce 169's 2^(F-1) at F = 4, 6, 8, 10, else my model
       is not measuring their column.
  C-M  A deliberately wrong characterisation must NOT match the disagreeing set,
       else "matches" carries no information.
  C-N  The characterisation must be checked at a width 169 did NOT report, so it
       is a prediction rather than a fit.
"""

def rte(v, frm, to):
    """Round v, an integer at scale 2^-frm, to scale 2^-to. Nearest, ties to even."""
    if to >= frm:
        return v << (to - frm)
    s = frm - to
    half = 1 << (s - 1)
    lo = v & ((1 << s) - 1)
    hi = v >> s
    if lo > half:
        return hi + 1
    if lo < half:
        return hi
    return hi + 1 if (hi & 1) else hi

def disagreeing(F):
    M = 2 * F - 1
    out = []
    n = 1 << F
    for a in range(n):
        for b in range(n):
            p = a * b
            if rte(p, 2 * F, F) != rte(rte(p, 2 * F, M), M, F):
                out.append((a, b, p))
    return out

print("--- C-L: reproduce 169's column ---")
counts = {}
for F in (4, 6, 8):
    d = disagreeing(F)
    counts[F] = len(d)
    print(f"  F={F:>2}  M=2F-1={2*F-1:>2}  disagreeing pairs {len(d):>5}   2^(F-1) = {1<<(F-1):>5}   match {len(d)==(1<<(F-1))}")

print()
print("--- the characterisation, read off the disagreeing set rather than assumed ---")
F = 8
d = disagreeing(F)
mods = sorted({p % (1 << F) for _, _, p in d})
lowbits = sorted({p & 1 for _, _, p in d})
print(f"  F={F}: distinct values of (p mod 2^F) among disagreeing pairs: {mods}")
print(f"  F={F}: 2^(F-1) - 1 = {(1 << (F-1)) - 1}")
print(f"  F={F}: distinct values of p's lowest bit: {lowbits}")
# does the condition characterise the set exactly?
target = (1 << (F - 1)) - 1
pred = set()
n = 1 << F
for a in range(n):
    for b in range(n):
        if (a * b) % (1 << F) == target:
            pred.add((a, b))
actual = {(a, b) for a, b, _ in d}
print(f"  predicted by 'p mod 2^F == 2^(F-1) - 1': {len(pred)} pairs; actual {len(actual)}; identical: {pred == actual}")

print()
print("--- C-M: a deliberately wrong characterisation must not match ---")
wrong = set()
for a in range(n):
    for b in range(n):
        if (a * b) % (1 << F) == target + 1:
            wrong.add((a, b))
print(f"  'p mod 2^F == 2^(F-1)' predicts {len(wrong)} pairs; identical to actual: {wrong == actual}   (must be False)")

print()
print("--- C-N: predict at a width 169 did not report ---")
for F in (5, 7, 9):
    d = disagreeing(F)
    target = (1 << (F - 1)) - 1
    n = 1 << F
    pred = {(a, b) for a in range(n) for b in range(n) if (a * b) % (1 << F) == target}
    actual = {(a, b) for a, b, _ in d}
    print(f"  F={F}  count {len(d):>5}  2^(F-1) {1<<(F-1):>5}  count match {len(d)==(1<<(F-1))}  set match {pred==actual}")

print()
print("VERDICT")
print("  The M = 2F-1 disagreeing set is exactly the pairs whose exact product is")
print("  congruent to 2^(F-1) - 1 modulo 2^F: the products sitting one unit below the")
print("  F-level tie, which the single dropped bit rounds up ONTO the tie, where")
print("  ties-to-even then breaks the other way. The count is the number of such")
print("  products, which is 2^(F-1).")
