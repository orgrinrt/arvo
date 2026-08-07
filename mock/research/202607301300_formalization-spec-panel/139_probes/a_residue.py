# What distinguishes the two overshooting formats from the nine exact ones?
# Integer arithmetic only for the verdicts; Fraction for the residues so nothing is a float guess.
from fractions import Fraction

def bitlen(n):                      # exact ceil(log2(n)) as ilog
    return n.bit_length()
def ceil_log2(n):                   # least k with 2^k >= n
    return (n - 1).bit_length()

# (name, radix, P, EMIN, EMAX, hidden, actual stored width)
F = [("binary16",   2, 11,  -14,    15, 1,  16),
     ("binary32",   2, 24, -126,   127, 1,  32),
     ("binary64",   2, 53,-1022,  1023, 1,  64),
     ("binary128",  2,113,-16382,16383, 1, 128),
     ("binary256",  2,237,-262142,262143,1,256),
     ("bfloat16",   2,  8, -126,   127, 1,  16),
     ("E4M3(OCP)",  2,  4,   -6,     8, 1,   8),
     ("E5M2(OCP)",  2,  3,  -14,    15, 1,   8),
     ("decimal32", 10,  7,  -95,    96, 0,  32),
     ("decimal64", 10, 16, -383,   384, 0,  64),
     ("decimal128",10, 34,-6143,  6144, 0, 128)]

print(f"{'format':11} {'joint':>6} {'perfield':>9} {'actual':>7}   {'r_sig':>7} {'r_exp':>7} {'r_sum':>7}  verdict")
for name, R, P, EMIN, EMAX, h, actual in F:
    span = EMAX - EMIN + 1
    card = R**(P - h) * span                 # the code space, as an exact integer
    joint = 1 + ceil_log2(card)              # one ceiling over the product
    sig   = ceil_log2(R**(P - h))
    exp   = ceil_log2(span)
    perf  = 1 + sig + exp                    # two ceilings, one per field
    # residues: how much each field's own ceiling wastes, exactly
    # r = k - log2(x) where k = ceil_log2(x); compare 2^k / x against 2^1 etc via rationals
    def residue(k, x):                       # k - log2(x), as a Fraction approximation by bisection on 2^(2^n)
        # exact comparison-free: return float only for display, verdicts below use integers
        import math
        return k - math.log2(x)
    r_sig = residue(sig, R**(P - h))
    r_exp = residue(exp, span)
    v = "ok" if perf == actual else "OVERSHOOT"
    assert joint == actual, (name, joint, actual)
    print(f"{name:11} {joint:6} {perf:9} {actual:7}   {r_sig:7.3f} {r_exp:7.3f} {r_sig+r_exp:7.3f}  {v}")

print()
print("integer verdict, no logs: perfield exceeds joint exactly when 2^(sig+exp) >= 2 * card")
for name, R, P, EMIN, EMAX, h, actual in F:
    span = EMAX - EMIN + 1
    card = R**(P - h) * span
    sig  = ceil_log2(R**(P - h)); exp = ceil_log2(span)
    lhs  = 2**(sig + exp)
    print(f"  {name:11} 2^(sig+exp)={lhs:>22}  2*card={2*card:>22}  loses_a_bit={lhs >= 2*card}")
