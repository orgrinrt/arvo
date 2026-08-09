#!/usr/bin/env python3
"""Probe 4. Does geometry's arithmetic compose freely over the settled operation
surface, and is D10's rotor component count right?

Exact integer arithmetic and exact Clifford algebra over the integers. No float
enters any load-bearing comparison. Blades are bitmasks over the axes, the
product is the standard sign-counting geometric product, Euclidean signature.

Two questions:

  (1) Every geometric operation is a composition of operations `91` already
      settles (mul_full at 2p, the fold at p + log n, division and roots as
      `91` section 1.13 and file 99 give them). Does the composition stay in
      one of the settled growth classes, or does it produce a new one?

  (2) D10 (`202607281220`, op) states a rotor "carries a scalar plus n(n-1)/2
      bivector components, which is exactly the degrees of freedom a rotation
      has", and rejects the matrix for "carrying N squared components for
      n(n-1)/2 degrees of freedom". Degrees of freedom and storage are two
      different counts. This computes both.
"""

from fractions import Fraction
from itertools import combinations


# --- exact Clifford algebra over the integers, Euclidean signature ----------

def blade_product(a, b):
    """Geometric product of two basis blades given as axis bitmasks.
    Returns (sign, blade). Euclidean: e_i e_i = +1."""
    # count transpositions to sort the concatenation a then b
    swaps = 0
    for i in range(64):
        if not (b >> i) & 1:
            continue
        # move e_i left past every axis of a strictly greater than i
        higher = a >> (i + 1)
        swaps += bin(higher).count("1")
    sign = -1 if swaps & 1 else 1
    return sign, a ^ b


def grade(blade):
    return bin(blade).count("1")


def gp(x, y, n):
    """Geometric product of two multivectors (dict blade -> Fraction)."""
    out = {}
    for ba, va in x.items():
        for bb, vb in y.items():
            s, bc = blade_product(ba, bb)
            out[bc] = out.get(bc, Fraction(0)) + s * va * vb
    return {b: v for b, v in out.items() if v != 0}


def reverse(x):
    out = {}
    for b, v in x.items():
        g = grade(b)
        out[b] = v * (-1 if (g * (g - 1) // 2) % 2 else 1)
    return out


def even_basis(n):
    return [b for b in range(1 << n) if grade(b) % 2 == 0]


def simple_rotor(i, j, c, s):
    """cos(t/2) + sin(t/2) e_i e_j, with c, s exact rationals on the unit
    circle so the rotor is exactly unit without any irrational."""
    return {0: Fraction(c), (1 << i) | (1 << j): Fraction(s)}


print("=" * 78)
print("CLAIM A. Storage: a general rotor occupies the whole even subalgebra,")
print("2^(n-1) components, not 1 + n(n-1)/2.")
print("=" * 78)
print(f"{'n':>3} {'dof of SO(n)':>13} {'D10 count':>10} {'even subalg':>12} "
      f"{'matrix n^2':>11} {'rotor > matrix?':>16}")
crossover = None
for n in range(2, 11):
    dof = n * (n - 1) // 2
    d10 = 1 + dof
    ev = 1 << (n - 1)
    m = n * n
    worse = ev > m
    if worse and crossover is None:
        crossover = n
    print(f"{n:3} {dof:13} {d10:10} {ev:12} {m:11} {str(worse):>16}")
print(f"\n  D10's count agrees with the even subalgebra only at n = 2 and n = 3")
print(f"  (2 = 2, 4 = 4). It diverges from n = 4 on (7 against 8).")
print(f"  Rotor storage first exceeds matrix storage at n = {crossover}.")

print()
print("=" * 78)
print("CLAIM B. The divergence is not bookkeeping: a product of two simple")
print("rotors in 4D has a NONZERO grade-4 part, so the 1 + n(n-1)/2 slots")
print("cannot hold it. Exhibited with exact rational unit rotors.")
print("=" * 78)
# (3,4,5) and (5,12,13) Pythagorean, so cos and sin are exact rationals.
n = 4
R1 = simple_rotor(0, 1, Fraction(3, 5), Fraction(4, 5))
R2 = simple_rotor(2, 3, Fraction(5, 13), Fraction(12, 13))
R = gp(R1, R2, n)
print("  R1 = 3/5 + 4/5 e01,  R2 = 5/13 + 12/13 e23")
for b in sorted(R, key=lambda b: (grade(b), b)):
    axes = "".join(str(i) for i in range(n) if (b >> i) & 1)
    print(f"    grade {grade(b)}  e{axes or 'scalar':<6} = {R[b]}")
g4 = [b for b in R if grade(b) == 4]
print(f"  grade-4 components present: {len(g4)}, value {R[g4[0]] if g4 else 0}")
assert g4 and R[g4[0]] != 0, "the pseudoscalar part must be nonzero"
# and it is a unit rotor: R R~ = 1
RRr = gp(R, reverse(R), n)
print(f"  R R~ = {RRr}  (unit, so this is a legitimate rotor, not a stray)")
assert RRr == {0: Fraction(1)}

print()
print("  Sanity, same construction at n = 3, where D10's count is correct:")
R3 = gp(simple_rotor(0, 1, Fraction(3, 5), Fraction(4, 5)),
        simple_rotor(0, 2, Fraction(5, 13), Fraction(12, 13)), 3)
print(f"    grades present: {sorted({grade(b) for b in R3})}, "
      f"components {len(R3)} (even subalgebra of Cl(3) is 4, the quaternions)")

print()
print("=" * 78)
print("CLAIM C. Term counts for the operations, exact, by enumeration.")
print("=" * 78)


def gp_terms(basis_a, basis_b):
    """Number of scalar multiplications in a dense product of two multivectors
    supported on the given basis sets."""
    return len(basis_a) * len(basis_b)


def sandwich_terms(n):
    """R x R~ with R over the even subalgebra and x a grade-1 vector."""
    ev = even_basis(n)
    vec = [1 << i for i in range(n)]
    # first product R*x lands on odd blades; second product with R~
    first = gp_terms(ev, vec)
    odd = [b for b in range(1 << n) if grade(b) % 2 == 1]
    second = gp_terms(odd, ev)
    return first + second


print(f"{'n':>3} {'rotor store':>12} {'compose R1R2':>13} {'apply RxR~':>12} "
      f"{'mat compose':>12} {'mat apply':>10}")
for n in range(2, 9):
    ev = 1 << (n - 1)
    print(f"{n:3} {ev:12} {ev*ev:13} {sandwich_terms(n):12} "
          f"{n**3:12} {n*n:10}")

print()
print("  Both families are polynomial in the term COUNT and identical in the")
print("  DEGREE of the products: compose is degree 2, apply is degree 2 for a")
print("  matrix and degree 3 for a rotor sandwich.")

print()
print("=" * 78)
print("CLAIM D. Accumulator widths. Every geometric operation lands in a class")
print("`91` already has. p is the numeral's precision in bits.")
print("=" * 78)


def clog2(k):
    """ceil(log2(k)), exact, for k >= 1."""
    assert k >= 1
    return (k - 1).bit_length()


rows = []
for n in range(2, 9):
    ev = 1 << (n - 1)
    rows.append((
        f"dot product, rank {n}",
        f"2p + {clog2(n)}",
        "mul_full then the fold: settled classes only",
    ))
for n in (2, 3, 4):
    ev = 1 << (n - 1)
    rows.append((
        f"rotor compose, rank {n}",
        f"2p + {clog2(ev)}",
        "geometric product is bilinear: one mul_full layer, one fold",
    ))
    rows.append((
        f"rotor sandwich, rank {n}",
        f"3p + {clog2(sandwich_terms(n))}",
        "two product layers: 3p, not 2p",
    ))
    rows.append((
        f"affine apply, rank {n}",
        f"max(2p + {clog2(n)}, p + F) + 1",
        "a dot product plus a translation at the operand's own quantum",
    ))
for label, width, why in rows:
    print(f"  {label:26} exact width {width:22} {why}")

print()
print("  Not in a settled class, and there is exactly one: renormalisation.")
print("  |R| needs a square root (file 99: the root-residue pair, LINEAR, and")
print("  ties are impossible by parity) and then a reciprocal, which is the")
print("  design's one exponential class (`91` section 1.13). The exponential")
print("  width is the width of the exact QUOTIENT as a numeral; normalisation")
print("  never materialises one, it quantises, so the class is reached in the")
print("  statement and never in the storage.")

print()
print("=" * 78)
print("CLAIM E. Exact composition depth, which IS exponential, for both forms.")
print("=" * 78)
p = 16
print(f"  starting at p = {p} bits, composing k transforms with no quantiser:")
for k in range(0, 7):
    w_rot = (2 ** k) * p + sum(clog2(1 << (3 - 1)) for _ in range(k))
    w_mat = (2 ** k) * p + sum(clog2(3) for _ in range(k))
    print(f"    k = {k}: rotor {w_rot:6} bits, matrix {w_mat:6} bits")
print("  So an exact transform chain is exponential in DEPTH for every")
print("  representation. The design's own answer is already written: a")
print("  quantiser fires per composition, and the site count is a function of")
print("  the monomorphised type (`91` section 1.14). Depth is a type-level")
print("  fact when the chain is typed, which is the only way the count is")
print("  computable at compile time.")

print()
print("=" * 78)
print("CLAIM F. The closed-form exponential, which is what makes a rotor cheap")
print("to interpolate, survives only to rank 4.")
print("=" * 78)
print("  exp(B) closes in the algebra when B splits into commuting SIMPLE")
print("  bivectors. The split always exists; FINDING it is the cost. For")
print("  floor(n/2) invariant planes the split is the root problem of the")
print("  bivector's own characteristic polynomial, degree floor(n/2).")
for n in range(2, 12):
    planes = n // 2
    if planes == 1:
        note = "B is simple, B^2 is a negative scalar, exp is cos + sin"
    elif n == 4:
        note = "2 planes, split closed-form by the dual: B+- = (B +- B~)/2"
    elif planes <= 4:
        note = f"degree-{planes} root problem, closed form exists (deg <= 4)"
    else:
        note = f"degree-{planes} root problem, NO closed form (Abel-Ruffini)"
    print(f"  n = {n:2}: dof {n*(n-1)//2:3}, planes {planes}  {note}")
print()
print("  So the rotor's cheap interpolation is a rank 2 through 4 property,")
print("  not a general one, and the same Abel-Ruffini ceiling the round")
print("  already recorded for exact distance to a cubic Bezier reappears here")
print("  at rank 10.")
print()
print("  Checked: a general 4D bivector is NOT simple.")
B = {0b0011: Fraction(1), 0b1100: Fraction(1)}  # e01 + e23
wedge = gp(B, B, 4)
g4 = {b: v for b, v in wedge.items() if grade(b) == 4}
print(f"    (e01 + e23) squared has grade-4 part {g4}, nonzero, so B^B != 0")
assert g4
B2 = {0b0011: Fraction(1)}
w2 = {b: v for b, v in gp(B2, B2, 4).items() if grade(b) == 4}
print(f"    e01 squared has grade-4 part {w2}, empty, so e01 IS simple")
assert not w2
print()
print("  NOTE on CLAIM C's counts: they are DENSE, every basis pair multiplied.")
print("  A real sandwich exploits that R x R~ is grade 1 and cancels most of")
print("  it (the quaternion form is ~15 multiplies against 28 dense at n=3).")
print("  The dense count is the upper bound and the ORDER is what the")
print("  comparison rests on, not the constant.")
