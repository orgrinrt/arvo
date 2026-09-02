#!/usr/bin/env python3
# q01. Independent re-derivation of 06's negative-integer-width count, and the
# question the count is actually being asked to answer.
#
# 06 section 7.2 claims: over the box of shapes U<I,F> with I,F in 0..8 (81
# shapes, 6561 ordered pairs), the tight product numeral has negative integer
# width at exactly 15 pairs, and the region is "one operand is U<0,1> and both
# operands are purely fractional". 06 also reports 7 of 625 at box 4 and 11 of
# 2401 at box 6.
#
# THREE INSTRUMENTS, because one number by one route is one instance.
#
#   A  enumerates the exact rational product set and reads the least containing
#      numeral straight off the definition. Correct by construction and
#      O(4^(W1+W2)), so it only reaches the small boxes.
#   B  is A with the enumeration removed: the product set's maximum and step in
#      closed form. It is NOT independent of A, it is A's closed form, and the
#      A-vs-B check tests the algebra rather than the definition.
#   C  is 06's own piecewise rule, quoted from 06 section 7.1. It is an
#      independent formula, so B-vs-C is a real second opinion on 06's number.
#
# Reproduce:  python3 q01_negative_width_recount.py
# Toolchain:  python3 only. Exact integer arithmetic, not a measurement.

from fractions import Fraction


def bits_for(units):
    """Least w with 2^w - 1 >= units. bit_length is exactly that: 2^w - 1 has
    w bits, and units needs bit_length(units) bits."""
    return units.bit_length()


# ---------- instrument A: straight from the value set -------------------------

def valueset(I, F):
    """U<I,F> denotes {k * 2^-F : 0 <= k <= 2^(I+F) - 1}."""
    return [Fraction(k, 2**F) for k in range(2 ** (I + F))]


def shape_A(a, b):
    prods = {x * y for x in valueset(*a) for y in valueset(*b)}
    f = 0
    for v in prods:
        need = v.denominator.bit_length() - 1  # denominators are powers of two
        f = max(f, need)
    units = int(max(prods) * 2**f)
    return (bits_for(units) - f, f)


# ---------- instrument B: A's closed form -------------------------------------

def shape_B(a, b):
    (I1, F1), (I2, F2) = a, b
    W1, W2 = I1 + F1, I2 + F2
    if W1 == 0 or W2 == 0:
        return (0, 0)  # one operand denotes only zero, so the product set is {0}
    top = (2**W1 - 1) * (2**W2 - 1)
    F = F1 + F2  # the unit product 1*1 is in the set, so the step is 2^-(F1+F2)
    return (bits_for(top) - F, F)


# ---------- instrument C: 06's stated piecewise rule ---------------------------

def shape_C(a, b):
    (I1, F1), (I2, F2) = a, b
    W1, W2 = I1 + F1, I2 + F2
    if W1 == 0 or W2 == 0:
        return (0, 0)
    if 2**W1 + 2**W2 - 2 >= 2 ** (W1 + W2 - 1):
        W = W1 + W2 - 1
    else:
        W = W1 + W2
    F = F1 + F2
    return (W - F, F)


def box(n):
    return [(I, F) for I in range(n) for F in range(n)]


def main():
    small = box(5)  # I, F in 0..4; the widest operand is 8 bits, A is feasible
    dAB = [(a, b) for a in small for b in small if shape_A(a, b) != shape_B(a, b)]
    print(f"A vs B over {len(small)**2} pairs of box 4: disagreements = {len(dAB)}")
    for a, b in dAB[:5]:
        print(f"   {a} {b}: A={shape_A(a,b)} B={shape_B(a,b)}")
    assert not dAB

    full = box(9)
    dBC = [(a, b) for a in full for b in full if shape_B(a, b) != shape_C(a, b)]
    print(f"B vs C over {len(full)**2} pairs of box 8: disagreements = {len(dBC)}")
    for a, b in dBC[:5]:
        print(f"   {a} {b}: B={shape_B(a,b)} C={shape_C(a,b)}")

    print()
    for n in (5, 7, 9):
        sh = box(n)
        c = sum(1 for a in sh for b in sh if shape_B(a, b)[0] < 0)
        print(f"box {n-1}: {c} of {len(sh)**2} pairs have I < 0")

    neg = [(a, b) + shape_B(a, b) for a in full for b in full if shape_B(a, b)[0] < 0]
    print()
    print("the exact region at box 8:")
    for a, b, I, F in neg:
        print(f"  U<{a[0]},{a[1]}> * U<{b[0]},{b[1]}>  ->  I={I} F={F}  W={I+F}")

    print()
    print("both operands purely fractional:",
          all(a[0] == 0 and b[0] == 0 for a, b, _, _ in neg))
    print("an operand equal to U<0,1>:     ",
          all(a == (0, 1) or b == (0, 1) for a, b, _, _ in neg))
    print("min I over the region:          ", min(I for _, _, I, _ in neg))

    # THE QUESTION THIS PROBE EXISTS FOR. The width encodings built this stretch
    # are binary NATURALS. I < 0 is unspellable in them. But (I, F) is only one
    # choice of coordinates. Ask whether the OTHER two coordinates ever go
    # negative: total width W = I + F, and fraction width F.
    allp = [shape_B(a, b) for a in full for b in full]
    print()
    print("product, over all 6561 pairs:")
    print("  pairs with I < 0:", sum(1 for I, _ in allp if I < 0))
    print("  pairs with F < 0:", sum(1 for _, F in allp if F < 0))
    print("  pairs with W < 0:", sum(1 for I, F in allp if I + F < 0))
    print("  min W:", min(I + F for I, F in allp), " min F:", min(F for _, F in allp))

    def add_shape(a, b):
        (I1, F1), (I2, F2) = a, b
        F = max(F1, F2)
        m = (2 ** (I1 + F1) - 1) * 2 ** (F - F1) + (2 ** (I2 + F2) - 1) * 2 ** (F - F2)
        return (bits_for(m) - F, F)

    adds = [add_shape(a, b) for a in full for b in full]
    print()
    print("addition, over the same 6561 pairs:")
    print("  pairs with I < 0:", sum(1 for I, _ in adds if I < 0))
    print("  pairs with W < 0:", sum(1 for I, F in adds if I + F < 0))
    print("  min W:", min(I + F for I, F in adds), " min F:", min(F for _, F in adds))

    # the coordinatewise join and meet, for completeness, since the record keeps
    # citing them even though 06 and 03 both report the meet has no caller
    joins = [(max(a[0], b[0]), max(a[1], b[1])) for a in full for b in full]
    meets = [(min(a[0], b[0]), min(a[1], b[1])) for a in full for b in full]
    print()
    print("join: min I", min(I for I, _ in joins), " min W", min(I + F for I, F in joins))
    print("meet: min I", min(I for I, _ in meets), " min W", min(I + F for I, F in meets))


if __name__ == "__main__":
    main()
