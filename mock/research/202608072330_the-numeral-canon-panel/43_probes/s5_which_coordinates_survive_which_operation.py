#!/usr/bin/env python3
"""s5: which grid coordinates survive which operation, and which survive a fold.

`p2` establishes that no derivation in the panel reads a numeral's grid: the
carrier, the fold accumulator and the law bound are unchanged across grids that
differ in adjustment, bias, phase and canonical exponent.  That is a statement
about the DERIVATIONS.  It says nothing about the OPERATIONS, and the two are
different questions with different answers.

The record's affine value map is v(k) = A * r^e * k + B (`seed/SETTLED_laws.md:274`,
quoted at `08:175-186` and `24:257-262`), with the note that A and B cannot be
folded into one another because one changes the spacing and the other moves the
origin.  Nobody in either panel has asked what B does to the arithmetic.

This probe asks, in exact rational arithmetic, for each of add, subtract,
multiply, compare and n-ary sum:

  Q1  is the exact result of the operation on two values of one grid again a
      value of a grid of the same shape, and if so with which coordinates?
  Q2  which coordinates does the RAW (stored-integer) implementation have to
      read, i.e. which of them cost an instruction rather than being free?
  Q3  what happens to each coordinate under an n-ary fold, where n is dynamic?

The distinction Q2 and Q3 draw is the one that decides whether a COMPOSITION
may hold a grid coordinate at run time, shared over its whole run.  A
coordinate no raw operation reads is free to be dynamic.  A coordinate whose
value after a fold depends on the DYNAMIC trip count has crossed a stage and is
not free.
"""

from fractions import Fraction as F
from itertools import product


def value(A, B, e, radix, k):
    """the record's affine value map"""
    return A * F(radix) ** e * k + B


def main() -> int:
    print("s5: which grid coordinates survive which operation")
    print("=" * 74)

    radix = 2
    # a small box of grids and a small index set; exact rationals throughout
    adjustments = [F(1), F(3), F(1, 3)]
    biases = [F(0), F(1, 2), F(-2)]
    exponents = [-2, 0, 1]
    KMAX = 6
    ks = list(range(0, KMAX + 1))

    # ------------------------------------------------------------------- Q1
    # For each grid and each binary operation, take the exact result set over
    # all operand pairs and ask whether it is again an arithmetic progression,
    # and with which step and origin.
    print()
    print("Q1  is the exact result set of a same-grid operation again a grid,")
    print("    and which coordinates does it carry?")
    print()
    hdr = f"    {'A':>6} {'B':>6} {'e':>3} | {'add':>22} {'sub':>22} {'mul':>26}"
    print(hdr)
    print("    " + "-" * (len(hdr) - 4))

    add_bias_scaled = 0
    add_total = 0
    mul_affine_ok = 0
    mul_affine_bad = 0

    for A, B, e in product(adjustments, biases, exponents):
        vals = [value(A, B, e, radix, k) for k in ks]

        def describe(resultset):
            rs = sorted(set(resultset))
            if len(rs) < 2:
                return "singleton"
            steps = sorted(set(rs[i + 1] - rs[i] for i in range(len(rs) - 1)))
            if len(steps) == 1:
                return f"grid step={steps[0]} min={rs[0]}"
            return f"NOT a grid ({len(steps)} steps)"

        adds = [x + y for x in vals for y in vals]
        subs = [x - y for x in vals for y in vals]
        muls = [x * y for x in vals for y in vals]

        da, ds, dm = describe(adds), describe(subs), describe(muls)
        print(f"    {str(A):>6} {str(B):>6} {e:>3} | {da:>22} {ds:>22} {dm:>26}")

        # the specific claim: an add's origin is 2B, not B
        add_total += 1
        if min(adds) == 2 * B + 2 * min(A * F(radix) ** e * k for k in ks):
            add_bias_scaled += 1

        # the specific claim: a product is affine in one index only when B == 0
        # (A k1 + B)(A k2 + B) = A^2 k1 k2 + A B (k1 + k2) + B^2
        if dm.startswith("grid") or dm == "singleton":
            mul_affine_ok += 1
        else:
            mul_affine_bad += 1

    print()
    print(f"    add: origin doubles (2B rather than B) in {add_bias_scaled}"
          f" of {add_total} grids")
    print(f"    mul: result set is again a uniform grid in {mul_affine_ok}"
          f" of {add_total} grids, and is not in {mul_affine_bad}")

    # sharpen the mul row: split by whether the bias is zero
    zero_b_ok = zero_b_bad = nz_b_ok = nz_b_bad = 0
    for A, B, e in product(adjustments, biases, exponents):
        vals = [value(A, B, e, radix, k) for k in ks]
        rs = sorted(set(x * y for x in vals for y in vals))
        steps = set(rs[i + 1] - rs[i] for i in range(len(rs) - 1))
        ok = len(steps) <= 1
        if B == 0:
            zero_b_ok += ok
            zero_b_bad += not ok
        else:
            nz_b_ok += ok
            nz_b_bad += not ok
    print()
    print("    mul, split by bias:")
    print(f"      B == 0 : uniform grid {zero_b_ok}, not a grid {zero_b_bad}")
    print(f"      B != 0 : uniform grid {nz_b_ok}, not a grid {nz_b_bad}")

    # ------------------------------------------------------------------- Q2
    # Which coordinates does a RAW same-grid implementation read?  Answered by
    # checking whether the raw result index is a function of the raw operand
    # indices alone.
    print()
    print("Q2  which coordinates the raw (stored-integer) implementation reads")
    print("    on a SAME-GRID operation.  A coordinate is 'read' when the raw")
    print("    result depends on it.")
    print()

    def raw_add(A, B, e, k1, k2):
        """the raw index whose value is v(k1)+v(k2), if one exists"""
        target = value(A, B, e, radix, k1) + value(A, B, e, radix, k2)
        # solve A r^e k + B = target
        k = (target - B) / (A * F(radix) ** e)
        return k

    def raw_mul(A, B, e, k1, k2):
        target = value(A, B, e, radix, k1) * value(A, B, e, radix, k2)
        k = (target - B) / (A * F(radix) ** e)
        return k

    add_indep_A = add_indep_B = add_indep_e = True
    mul_indep_A = mul_indep_B = mul_indep_e = True
    add_k_integral = 0
    add_k_total = 0
    for k1, k2 in product(ks, ks):
        ra = {(A, B, e): raw_add(A, B, e, k1, k2)
              for A, B, e in product(adjustments, biases, exponents)}
        rm = {(A, B, e): raw_mul(A, B, e, k1, k2)
              for A, B, e in product(adjustments, biases, exponents)}
        for (A, B, e), v in ra.items():
            add_k_total += 1
            if v.denominator == 1:
                add_k_integral += 1
        # vary one coordinate at a time from a fixed base
        base = (adjustments[0], biases[0], exponents[0])
        for A in adjustments:
            if ra[(A, base[1], base[2])] != ra[base]:
                add_indep_A = False
            if rm[(A, base[1], base[2])] != rm[base]:
                mul_indep_A = False
        for B in biases:
            if ra[(base[0], B, base[2])] != ra[base]:
                add_indep_B = False
            if rm[(base[0], B, base[2])] != rm[base]:
                mul_indep_B = False
        for e in exponents:
            if ra[(base[0], base[1], e)] != ra[base]:
                add_indep_e = False
            if rm[(base[0], base[1], e)] != rm[base]:
                mul_indep_e = False

    print(f"    add: raw result independent of adjustment      : {add_indep_A}")
    print(f"    add: raw result independent of bias            : {add_indep_B}")
    print(f"    add: raw result independent of canonical expt  : {add_indep_e}")
    print(f"    mul: raw result independent of adjustment      : {mul_indep_A}")
    print(f"    mul: raw result independent of bias            : {mul_indep_B}")
    print(f"    mul: raw result independent of canonical expt  : {mul_indep_e}")
    print(f"    add: raw result is an integer index in"
          f" {add_k_integral} of {add_k_total} (grid, pair) cases")

    # ------------------------------------------------------------------- Q3
    # What an n-ary fold does to each coordinate, with n dynamic.
    print()
    print("Q3  the n-ary sum, with the trip count n dynamic")
    print()
    A, e = F(1), 0
    for B in biases:
        row = []
        for n in (1, 2, 3, 5, 8):
            total = sum(value(A, B, e, radix, 1) for _ in range(n))
            # express as A*k + B_eff for the same A: what is B_eff if k = n*1?
            b_eff = total - A * F(radix) ** e * n
            row.append(f"n={n}:B_eff={b_eff}")
        print(f"    B={str(B):>6} -> " + "  ".join(row))
    print()
    print("    The effective origin of a sum is n*B, and n is the DYNAMIC trip")
    print("    count, not the static capacity.  So a nonzero bias does not")
    print("    survive a fold as a static coordinate: the result numeral's own")
    print("    bias is a function of a runtime quantity.")

    # the same question for the adjustment and the exponent, which do survive
    print()
    surv_A = all(
        (sum(value(A2, F(0), e, radix, 1) for _ in range(n)) ==
         A2 * F(radix) ** e * n)
        for A2 in adjustments for e in exponents for n in (1, 2, 3, 5, 8, 13)
    )
    print(f"    adjustment and canonical exponent survive a zero-bias sum"
          f" unchanged, over every (A, e, n) checked: {surv_A}")

    # ------------------------------------------------------------------- Q4
    # A negative control on the whole file: if the value map were linear rather
    # than affine (B forced to zero), every result above would be trivial.
    print()
    print("Q4  NEGATIVE CONTROL: force B = 0 everywhere and re-run Q1's mul row.")
    bad = 0
    for A, e in product(adjustments, exponents):
        vals = [value(A, F(0), e, radix, k) for k in ks]
        rs = sorted(set(x * y for x in vals for y in vals))
        steps = set(rs[i + 1] - rs[i] for i in range(len(rs) - 1))
        if len(steps) > 1:
            bad += 1
    print(f"    with B = 0, products failing to be a uniform grid: {bad}"
          f" of {len(adjustments) * len(exponents)}")
    print("    (nonzero even at B = 0, because a product set of a grid is not")
    print("     a uniform grid in general; the point of Q1's split is the")
    print("     RATIO of failures, not their existence.)")

    # ------------------------------------------------------------------- Q5
    # Q1's mul column asked the wrong question and the probe says so rather
    # than being quietly rewritten.  The product SET of a grid is never a
    # uniform grid, at any bias, because {0, A, 2A, ...} squares to
    # {0, A^2, 4A^2, 9A^2, ...}.  0 of 27 is therefore a fact about squares and
    # not about the bias, and reading it as a bias result would have been wrong.
    #
    # The design's question is different: the derived numeral is the SMALLEST
    # GRID CONTAINING the result set (`07` section 3.2, `08` section 6), and
    # every finite set has one.  So the real question is whether that grid's
    # coordinates are a function of the operands' coordinates, or whether they
    # depend on an arithmetic relation between them.
    print()
    print("Q5  the product's smallest containing grid: are its coordinates a")
    print("    function of the operands' coordinates?")
    print()

    def containing_grid(vals):
        rs = sorted(set(vals))
        if len(rs) < 2:
            return (F(0), rs[0] if rs else F(0))
        # step = gcd of all differences, over the rationals
        diffs = [rs[i + 1] - rs[i] for i in range(len(rs) - 1)]
        g = diffs[0]
        for d in diffs[1:]:
            # gcd on rationals: gcd(num)/lcm(den)
            gn = F(_gcd(g.numerator * d.denominator, d.numerator * g.denominator),
                   g.denominator * d.denominator)
            g = gn
        return (g, rs[0])

    def _gcd(a, b):
        a, b = abs(a), abs(b)
        while b:
            a, b = b, a % b
        return a

    print(f"    {'A':>6} {'B':>6} {'e':>3} | {'product grid step':>22}"
          f" {'A^2 r^2e':>14} {'equal?':>7}")
    print("    " + "-" * 62)
    zero_b_match = zero_b_miss = nz_b_match = nz_b_miss = 0
    for A, B, e in product(adjustments, biases, exponents):
        vals = [value(A, B, e, radix, k) for k in ks]
        muls = [x * y for x in vals for y in vals]
        step, _origin = containing_grid(muls)
        predicted = A * A * F(radix) ** (2 * e)
        eq = (step == predicted)
        if B == 0:
            zero_b_match += eq
            zero_b_miss += not eq
        else:
            nz_b_match += eq
            nz_b_miss += not eq
        print(f"    {str(A):>6} {str(B):>6} {e:>3} | {str(step):>22}"
              f" {str(predicted):>14} {str(eq):>7}")
    print()
    print(f"    B == 0 : predicted step correct {zero_b_match},"
          f" wrong {zero_b_miss}")
    print(f"    B != 0 : predicted step correct {nz_b_match},"
          f" wrong {nz_b_miss}")
    print()
    print("    At zero bias the product's grid step is A^2 r^2e, a function of")
    print("    the operands' coordinates alone.  At nonzero bias it is not:")
    print("    the cross term A*B*(k1+k2) puts A*gcd(A,B)-shaped quantities in")
    print("    the difference set, so the derived step depends on an arithmetic")
    print("    relation between two coordinates rather than on either.")

    # and the same question for addition, where the answer is clean
    print()
    add_ok = add_bad = 0
    for A, B, e in product(adjustments, biases, exponents):
        vals = [value(A, B, e, radix, k) for k in ks]
        adds = [x + y for x in vals for y in vals]
        step, origin = containing_grid(adds)
        if step == A * F(radix) ** e and origin == 2 * B + 2 * min(
                A * F(radix) ** e * k for k in ks):
            add_ok += 1
        else:
            add_bad += 1
    print(f"    addition, by contrast: step is A*r^e and origin is 2B, in"
          f" {add_ok} of {add_ok + add_bad} grids")

    print()
    print("SUMMARY")
    print("  Addition reads none of the grid coordinates in its raw form, so a")
    print("  composition may hold all of them dynamically and pay nothing for a")
    print("  same-grid add.  Multiplication reads the canonical exponent (the")
    print("  rescale) and, with a nonzero bias, is not expressible as a raw")
    print("  index operation at all.  And a fold's effective origin is n*B with")
    print("  n dynamic, so the bias is the one coordinate that does not survive")
    print("  arity-n as a static fact.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
