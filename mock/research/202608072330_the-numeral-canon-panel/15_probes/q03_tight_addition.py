#!/usr/bin/env python3
# q03. q02 found the classic addition rule overshoots at 289 of 6561 pairs, in
# (I,F) and in (W,F) alike, because the two are the same rule in two coordinate
# systems. 06 section 7.1 measured the product form's overshoot at 461 and
# nobody measured addition's. This probe asks whether addition has a tight form
# reachable with natural comparison alone, the way the product's did.
#
# Derivation, so the reader can check it rather than take the result:
#   align to F = max(F1, F2); d_i = F - F_i; A_i = W_i + d_i = I_i + F.
#   operand i's maximum in units of 2^-F is 2^A_i - 2^d_i.
#   with A1 >= A2 the sum is 2^A1 + 2^A2 - 2^d1 - 2^d2, which needs A1 + 1 bits
#   exactly when 2^A2 >= 2^d1 + 2^d2.
#   let M = max(d1, d2). then 2^d1 + 2^d2 lies in (2^M, 2^(M+1)], so
#     A2 > M   =>  2^A2 >= 2^(M+1) >= 2^d1 + 2^d2   carry
#     A2 <= M  =>  2^A2 <= 2^M     <  2^d1 + 2^d2   no carry
#   so the carry predicate is  min(A1, A2) > max(d1, d2), a natural comparison.
#
# Reproduce:  python3 q03_tight_addition.py
# Toolchain:  python3 only. Exact integer arithmetic, not a measurement.

BOX = 9


def bits_for(u):
    return u.bit_length()


def box(n):
    return [(I, F) for I in range(n) for F in range(n)]


def to_WF(IF):
    return (IF[0] + IF[1], IF[1])


def add_exact(a, b):
    """Ground truth: enumerate nothing, but compute the true maximum."""
    (I1, F1), (I2, F2) = a, b
    F = max(F1, F2)
    m = (2 ** (I1 + F1) - 1) * 2 ** (F - F1) + (2 ** (I2 + F2) - 1) * 2 ** (F - F2)
    return (bits_for(m), F)  # (W, F)


def add_classic(a, b):
    (I1, F1), (I2, F2) = a, b
    return to_WF((max(I1, I2) + 1, max(F1, F2)))


def add_tight(a, b):
    """Naturals only: two subtractions whose subtrahends are provably smaller,
    two maxima, one minimum, one comparison, one conditional successor."""
    (W1, F1), (W2, F2) = to_WF(a), to_WF(b)
    F = max(F1, F2)
    d1, d2 = F - F1, F - F2
    A1, A2 = W1 + d1, W2 + d2
    carry = 1 if min(A1, A2) > max(d1, d2) else 0
    return (max(A1, A2) + carry, F)


def main():
    shapes = box(BOX)
    pairs = [(a, b) for a in shapes for b in shapes]

    cbad = [(a, b) for a, b in pairs if add_classic(a, b) != add_exact(a, b)]
    tbad = [(a, b) for a, b in pairs if add_tight(a, b) != add_exact(a, b)]
    print(f"pairs: {len(pairs)}")
    print(f"classic rule wrong at: {len(cbad)}")
    print(f"tight  rule wrong at: {len(tbad)}")
    for a, b in tbad[:10]:
        print(f"   U<{a[0]},{a[1]}> + U<{b[0]},{b[1]}>: "
              f"exact {add_exact(a,b)} tight {add_tight(a,b)}")

    # what the classic rule costs, in the currency the erasure gate cares about:
    # a wasted bit is only visible when it crosses a container rung
    rungs = (8, 16, 32, 64, 128)
    cross = [(a, b) for a, b in cbad
             if any(add_exact(a, b)[0] <= r < add_classic(a, b)[0] for r in rungs)]
    print(f"of the classic rule's overshoots, container-rung crossings: {len(cross)}")
    for a, b in cross[:10]:
        print(f"   U<{a[0]},{a[1]}> + U<{b[0]},{b[1]}>: "
              f"exact W={add_exact(a,b)[0]} classic W={add_classic(a,b)[0]}")

    # and the same accounting for the product, for comparison with 06's 38
    def prod_exact(a, b):
        (I1, F1), (I2, F2) = a, b
        W1, W2 = I1 + F1, I2 + F2
        if W1 == 0 or W2 == 0:
            return (0, 0)
        return (bits_for((2**W1 - 1) * (2**W2 - 1)), F1 + F2)

    def prod_naive(a, b):
        (I1, F1), (I2, F2) = a, b
        return to_WF((I1 + I2, F1 + F2))

    pbad = [(a, b) for a, b in pairs if prod_naive(a, b) != prod_exact(a, b)]
    pcross = [(a, b) for a, b in pbad
              if any(prod_exact(a, b)[0] <= r < prod_naive(a, b)[0] for r in rungs)]
    print()
    print(f"product: sum-of-widths rule wrong at {len(pbad)}, "
          f"rung crossings {len(pcross)}")

    # the tight rules composed: does repeated self-addition diverge?
    print()
    print("repeated self-addition of U<3,5>, the width each rule reaches:")
    c = t = (3, 5)
    for k in range(6):
        print(f"  step {k}: classic W={to_WF(c)[0]}   tight W={to_WF(t)[0]}   F={t[1]}")
        cw, cf = add_classic(c, c)
        c = (cw - cf, cf)
        tw, tf = add_tight(t, t)
        t = (tw - tf, tf)


if __name__ == "__main__":
    main()
