# Python pre-computations for file 99, run BEFORE the Rust probes were written.
# Exact integer arithmetic only (math.isqrt, integer powers, fractions).
from math import isqrt
from fractions import Fraction

def sqrt_sweep(P, F):
    """Same-numeral sqrt: operand and result grid k/2^F, indices 0..2^P-1.
    Correct rounding decided by exact integer comparison against squared midpoints.
    Returns (exact_hits, round_ups, ties, max_residue, max_widths, overflow_count, zero_flush_count)."""
    M = 2**P - 1
    exact = ups = ties = over = zflush = 0
    max_r = 0
    max_opw = 0
    for k in range(0, 2**P):
        X = k << F              # scaled operand: sqrt target is sqrt(X)/2^F on integer index scale... 
        # index-scale: true result index t = sqrt(k * 2^F). m = floor(t):
        m = isqrt(k << F)
        r = (k << F) - m*m
        max_r = max(max_r, r)
        max_opw = max(max_opw, (k << F).bit_length())
        if r == 0:
            exact += 1
            res = m
        else:
            # tie iff (2m+1)^2 == 4*(k<<F)  <=> 4r == 4m+1 (impossible, parity)
            if (2*m+1)**2 == 4*(k << F):
                ties += 1
            if r > m:
                ups += 1
                res = m + 1
            else:
                res = m
        if res > M:
            over += 1
        if k >= 1 and res == 0:
            zflush += 1
    return exact, ups, ties, max_r, max_opw, over, zflush

print("== sqrt sweeps (P, F) -> exact, ups, ties, max_r, max_operand_bits, overflow, zero_flush")
for (P, F) in [(2,2),(3,3),(4,2),(4,4),(6,6),(8,4),(8,8),(2,4),(3,6)]:
    print((P,F), sqrt_sweep(P,F))

# overflow-band emptiness criterion: empty iff M >= 2^F - 1
print("== criterion check: predicted-empty iff M >= 2^F - 1")
for (P, F) in [(2,2),(3,3),(4,2),(4,4),(6,6),(8,4),(8,8),(2,4),(3,6)]:
    M = 2**P - 1
    print((P,F), "predicted", "EMPTY" if M >= 2**F - 1 else "INHABITED")

# cube root, same shape: m = floor(X^(1/3)) with X = k * 2^(2F); decision vs (2m+1)^3 * ... 
def icbrt(n):
    if n == 0: return 0
    x = int(round(n ** (1.0/3))) + 2
    while x*x*x > n: x -= 1
    return x

def cbrt_sweep(P, F):
    exact = ups = ties = 0
    for k in range(0, 2**P):
        X = k << (2*F)          # true index t = (k * 2^{2F})^{1/3}
        m = icbrt(X)
        r = X - m**3
        if r == 0:
            exact += 1
        else:
            # tie iff (2m+1)^3 == 8X ; parity: LHS odd, RHS even -> impossible
            if (2*m+1)**3 == 8*X: ties += 1
            if 8*X > (2*m+1)**3: ups += 1
    return exact, ups, ties

print("== cbrt sweeps (P,F)=(4,2),(6,3):", cbrt_sweep(4,2), cbrt_sweep(6,3))

# exp2 exact hits and ties on a dyadic grid, exact integer arithmetic.
# x = k/2^F, result grid j/2^G. hit iff 2^k * 2^(G*2^F) == j^(2^F); tie iff with (2j+1)/2^(G+1).
def exp2_sweep(F, G, kmax):
    hits, ties, maxw = [], 0, 0
    Q = 2**F
    for k in range(0, kmax+1):
        lhs = 2**(k + G*Q)
        # search j near 2^(k/Q) * 2^G
        t = 2.0**(k/Q) * 2**G
        for j in range(max(1,int(t)-2), int(t)+3):
            w = (j**Q).bit_length(); maxw = max(maxw, w)
            if j**Q == lhs: hits.append((k, j))
        lhs_t = 2**(k + (G+1)*Q)
        for j in range(max(1,int(t)-2), int(t)+3):
            if (2*j+1)**Q == lhs_t: ties += 1
    return hits, ties, maxw

for (F,G,kmax) in [(2,4,8),(3,4,24)]:
    hits, ties, maxw = exp2_sweep(F,G,kmax)
    print(f"== exp2 F={F} G={G} k<= {kmax}: hits(k,j)={hits} ties={ties} max_comparison_bits={maxw}")

# transcendental hardness const at a model numeral: P=F=8, exp over x=k/256, k=1..255.
# decimal exp is correctly rounded to context precision; 60 digits >> needed for discrimination.
from decimal import Decimal, getcontext
getcontext().prec = 60
worst = (None, Decimal(1))
zero_dist = 0
for k in range(1, 256):
    x = Decimal(k) / 256
    t = x.exp() * 256           # result on the same 1/256 grid, index scale
    u = t % 1
    d = abs(u - Decimal("0.5"))  # distance to the nearest rounding boundary
    if d == 0: zero_dist += 1
    if d < worst[1]: worst = (k, d)
import math
k, d = worst
print(f"== exp hardness at P=F=8: ties={zero_dist}, hardest k={k}, boundary distance={float(d):.3e},"
      f" extra bits to decide rounding ~ {math.ceil(-math.log2(float(d)))}")
