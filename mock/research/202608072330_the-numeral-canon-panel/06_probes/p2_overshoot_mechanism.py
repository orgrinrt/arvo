#!/usr/bin/env python3
"""
P2. P1 found that the design's natural closed forms are NOT the least numeral
containing the exact result: mul_full is tight at 1099/1296, add at 1175/1296,
the subtraction candidate at 751/1296. This file finds the mechanism and then
attacks it, because "the formula overshoots" identifies a question rather than
answering one.

Three questions, in order:

  Q1  Is the overshoot confined to DEGENERATE operands, meaning a numeral whose
      value set is a single value? If so, the formulas are tight where it matters
      and the overshoot is the zero-width numeral's shadow rather than a defect.

  Q2  If overshoot survives on non-degenerate operands, what is the mechanism?

  Q3  Given the mechanism, is there a CORRECTED closed form that is tight
      everywhere? If there is, the canon can claim tightness. If there is not,
      the canon may only claim containment, and that difference is a sentence.

Independent of P1's counting path: this file recomputes the least containing
shape from the exact set, and re-derives the degeneracy split rather than
importing P1's classification.
"""

from fractions import Fraction as Fr

LIM = 5


def vals(I, F):
    n = 2 ** (I + F)
    q = Fr(1, 2**F)
    return frozenset(Fr(k) * q for k in range(n))


def least_containing(s):
    """Least U<I,F> containing s. Recomputed here, not imported."""
    if s == {Fr(0)}:
        return (0, 0)
    F = 0
    while not all((x * (2**F)).denominator == 1 for x in s):
        F += 1
        if F > 40:
            raise RuntimeError("non-dyadic")
    mx = max(s)
    I = 0
    while Fr(2**I) - Fr(1, 2**F) < mx:
        I += 1
        if I > 40:
            raise RuntimeError("runaway")
    return (I, F)


SHAPES = [(I, F) for I in range(0, LIM + 1) for F in range(0, LIM + 1)]
PAIRS = [(a, b) for a in SHAPES for b in SHAPES]


def degenerate(s):
    """A shape is degenerate when its value set holds one value: I+F == 0."""
    return s[0] + s[1] == 0


def mul_set(a, b):
    return frozenset(x * y for x in vals(*a) for y in vals(*b))


def add_set(a, b):
    return frozenset(x + y for x in vals(*a) for y in vals(*b))


def sub_set(a, b):
    return frozenset(x - y for x in vals(*a) for y in vals(*b) if x - y >= 0)


SITES = {
    "mul_full": (mul_set, lambda a, b: (a[0] + b[0], a[1] + b[1])),
    "add_mixed": (add_set, lambda a, b: (max(a[0], b[0]) + 1, max(a[1], b[1]))),
    "sub_mixed": (sub_set, lambda a, b: (max(a[0], b[0]), max(a[1], b[1]))),
}


def main():
    print("# P2. the overshoot mechanism, and whether a tight formula exists")
    print()

    print("## Q1. is the overshoot confined to degenerate operands?")
    print()
    least_cache = {}
    for name, (setf, form) in SITES.items():
        tight_nd = over_nd = tight_d = over_d = 0
        first_nd = None
        for a, b in PAIRS:
            s = setf(a, b)
            L = least_containing(s)
            least_cache[(name, a, b)] = L
            C = form(a, b)
            deg = degenerate(a) or degenerate(b)
            if C == L:
                if deg:
                    tight_d += 1
                else:
                    tight_nd += 1
            else:
                if deg:
                    over_d += 1
                else:
                    over_nd += 1
                    if first_nd is None:
                        first_nd = (a, b, C, L)
        print(f"### {name}")
        print(f"   NON-degenerate operands : tight {tight_nd}, overshoot {over_nd}")
        print(f"   at least one degenerate : tight {tight_d}, overshoot {over_d}")
        if first_nd:
            a, b, C, L = first_nd
            print(f"   first NON-degenerate overshoot: U{a} , U{b} -> "
                  f"formula U{C}, least U{L}")
        else:
            print("   no non-degenerate overshoot: the formula is TIGHT on every")
            print("   pair where neither operand denotes a single value.")
        print()

    print("## Q2. the mechanism, for whichever site still overshoots")
    print()
    # characterise the non-degenerate overshoots of each site
    for name, (setf, form) in SITES.items():
        bad = [(a, b, form(a, b), least_cache[(name, a, b)])
               for a, b in PAIRS
               if not (degenerate(a) or degenerate(b))
               and form(a, b) != least_cache[(name, a, b)]]
        if not bad:
            continue
        print(f"### {name}: {len(bad)} non-degenerate overshoots")
        # which coordinate is wrong, and by how much
        dI = sorted({c[0] - l[0] for _, _, c, l in bad})
        dF = sorted({c[1] - l[1] for _, _, c, l in bad})
        print(f"   integer-width excess values : {dI}")
        print(f"   fraction-width excess values: {dF}")
        # is the excess explained by one relation between the operands?
        expl = all(l == (a[0], max(a[1], b[1])) for a, b, c, l in bad)
        print(f"   every least answer equals (I1, max(F1,F2)) : {expl}")
        for a, b, c, l in bad[:3]:
            print(f"   e.g. U{a} , U{b} -> formula U{c}, least U{l}")
        print()

    print("## Q3. the corrected closed forms, and whether they are tight everywhere")
    print()
    # candidate corrections, each still a total function of the four members
    CORRECTED = {
        "mul_full": lambda a, b: (0, 0) if (a[0] + a[1] == 0 or b[0] + b[1] == 0)
        else (a[0] + b[0], a[1] + b[1]),
        "add_mixed": lambda a, b: least_add(a, b),
        "sub_mixed": lambda a, b: (0, 0) if (a[0] + a[1] == 0) else (a[0], max(a[1], b[1])),
    }
    for name, (setf, _) in SITES.items():
        f = CORRECTED[name]
        tight = sum(1 for a, b in PAIRS if f(a, b) == least_cache[(name, a, b)])
        contains = sum(1 for a, b in PAIRS
                       if f(a, b)[0] >= least_cache[(name, a, b)][0]
                       and f(a, b)[1] >= least_cache[(name, a, b)][1])
        bad = [(a, b, f(a, b), least_cache[(name, a, b)]) for a, b in PAIRS
               if f(a, b) != least_cache[(name, a, b)]]
        print(f"   {name:10s}: corrected form tight {tight}/{len(PAIRS)}, "
              f"contains {contains}/{len(PAIRS)}")
        if bad:
            print(f"              first miss: U{bad[0][0]} , U{bad[0][1]} -> "
                  f"U{bad[0][2]} against least U{bad[0][3]}")
    print()

    print("## Q4. is the LEAST-CONTAINING map itself a coordinatewise function?")
    print("   (if it is, then 'take the least upper bound' is itself a formula,")
    print("    and the formula-versus-extremum distinction is not structural)")
    for name in SITES:
        # a coordinatewise function would mean: the I of the answer depends only
        # on coordinates, which it does by construction. The real question is
        # whether it FACTORS: I_out depends only on (I1,I2) and F_out only on
        # (F1,F2). Test that.
        fac_I = {}
        fac_F = {}
        ok_I = ok_F = True
        for a, b in PAIRS:
            L = least_cache[(name, a, b)]
            kI = (a[0], b[0])
            kF = (a[1], b[1])
            if kI in fac_I and fac_I[kI] != L[0]:
                ok_I = False
            fac_I[kI] = L[0]
            if kF in fac_F and fac_F[kF] != L[1]:
                ok_F = False
            fac_F[kF] = L[1]
        print(f"   {name:10s}: I_out factors through (I1,I2) alone: {ok_I} ; "
              f"F_out factors through (F1,F2) alone: {ok_F}")


def least_add(a, b):
    """Closed form for the least numeral containing every sum.

    max sum = (2^I1 - 2^-F1) + (2^I2 - 2^-F2). The least reach above that is
    2^(max(I1,I2)+1) unless one operand contributes nothing, so the +1 is real
    except in the degenerate corner. The grid is max(F1,F2) exactly, since the
    finer operand's own step is a sum (with 0 from the other side).
    """
    if a[0] + a[1] == 0 and b[0] + b[1] == 0:
        return (0, 0)
    if a[0] + a[1] == 0:
        return b
    if b[0] + b[1] == 0:
        return a
    F = max(a[1], b[1])
    mx = (Fr(2**a[0]) - Fr(1, 2**a[1])) + (Fr(2**b[0]) - Fr(1, 2**b[1]))
    I = 0
    while Fr(2**I) - Fr(1, 2**F) < mx:
        I += 1
    return (I, F)


if __name__ == "__main__":
    main()
