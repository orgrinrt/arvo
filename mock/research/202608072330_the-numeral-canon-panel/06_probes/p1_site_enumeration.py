#!/usr/bin/env python3
"""
P1. For every arithmetic site, is the result numeral

  (a) a total function of the operands' declared members (a FORMULA), and
  (b) equal to the LEAST admitted numeral containing the exact result set,
  (c) equal to the JOIN of the operands?

(a) is the option-H claim. (b) is the tightness claim: what a canon sentence
would have to establish if it wanted to call a derived numeral "the tightest
honest answer". (c) is what `03` section 7.3 denies for multiplication.

TWO METHODS, and the whole point is that they are cross-checked against each
other. A first version of this file did (b) by enumerating every value set and
searching a box of shapes for the minimal containers; it did not finish, and
`least_containing_slow` below is that method, kept and run at a small size as
the control on the fast one. `least_containing_fast` computes the required grid
and reach directly. If the two ever disagree the run says so and the numbers are
not to be used.

Family under test: unsigned fixed-point at radix 2, zero bias.
  U<I,F> = { k * 2^-F : 0 <= k < 2^(I+F) },  step 2^-F, low 0, high 2^I - 2^-F.

Within this family, inclusion is componentwise: U<I1,F1> is contained in
U<I2,F2> exactly when I2 >= I1 and F2 >= F1. That is derived, not assumed, and
`check_componentwise()` below verifies it by brute force before anything uses it.

Every count printed is produced by this file and is named in RUN.md.
"""

from fractions import Fraction as Fr
from math import gcd

LIM = 5          # operand box: I,F each 0..LIM
CTRL_LIM = 2     # control box for the slow method
CTRL_WIDE = 8    # control search box for the slow method


def vals(I, F):
    n = 2 ** (I + F)
    q = Fr(1, 2**F)
    return frozenset(Fr(k) * q for k in range(n))


SHAPES = [(I, F) for I in range(0, LIM + 1) for F in range(0, LIM + 1)]


# ---------------------------------------------------------------------------
# 0. Verify the componentwise inclusion characterisation before relying on it.
# ---------------------------------------------------------------------------

def check_componentwise():
    bad = []
    box = [(I, F) for I in range(0, 4) for F in range(0, 4)]
    v = {s: vals(*s) for s in box}
    for a in box:
        for b in box:
            by_sets = v[a] <= v[b]
            by_coords = (b[0] >= a[0]) and (b[1] >= a[1])
            if by_sets != by_coords:
                bad.append((a, b, by_sets, by_coords))
    return bad


# ---------------------------------------------------------------------------
# 1. Least containing shape, two ways.
# ---------------------------------------------------------------------------

def set_grid_and_max(s):
    """(required step, max element) of a finite set of non-negative rationals.

    The required step is the gcd of the set's elements, which is the coarsest
    uniform grid every element lies on.
    """
    s = [x for x in s]
    if not s:
        return None, None
    num = 0
    den = 1
    for x in s:
        num = gcd(num, x.numerator)
        den = den * x.denominator // gcd(den, x.denominator)
    if num == 0:
        return Fr(0), max(s)          # the set is {0}
    g = Fr(num, den)
    # normalise: gcd over rationals
    g = Fr(gcd(*[int(x / Fr(1, den)) for x in s]), den) if den else g
    return g, max(s)


def least_containing_fast(s):
    """Least U<I,F> whose value set contains s. Returns (I,F) or None."""
    g, mx = set_grid_and_max(s)
    if g is None:
        return None
    if g == 0:                     # s == {0}
        return (0, 0)
    # smallest F with 2^-F dividing every element, i.e. 2^-F <= g and 2^-F | g
    F = 0
    while True:
        q = Fr(1, 2**F)
        if all((x / q).denominator == 1 for x in s):
            break
        F += 1
        if F > 64:
            raise RuntimeError("grid not dyadic")
    # smallest I with 2^I - 2^-F >= mx
    I = 0
    while Fr(2**I) - Fr(1, 2**F) < mx:
        I += 1
        if I > 64:
            raise RuntimeError("reach runaway")
    return (I, F)


def least_containing_slow(s, wide):
    """Control. Brute-force minimal containers over an explicit shape box."""
    v = {t: vals(*t) for t in wide}
    ups = [t for t in wide if s <= v[t]]
    if not ups:
        return None
    minimal = [t for t in ups if not any(u != t and v[u] < v[t] for u in ups)]
    distinct = []
    for t in minimal:
        if not any(v[t] == v[u] for u in distinct):
            distinct.append(t)
    if len(distinct) != 1:
        return ("ANTICHAIN", distinct)
    return minimal[0]


# ---------------------------------------------------------------------------
# 2. The sites. Each gives (exact result set, closed-form candidate shape).
# ---------------------------------------------------------------------------

def site_mul_full(a, b):
    va, vb = vals(*a), vals(*b)
    return frozenset(x * y for x in va for y in vb), (a[0] + b[0], a[1] + b[1])


def site_add_mixed(a, b):
    va, vb = vals(*a), vals(*b)
    return (frozenset(x + y for x in va for y in vb),
            (max(a[0], b[0]) + 1, max(a[1], b[1])))


def site_sub_mixed(a, b):
    """Unsigned subtraction: the representable part of the difference set."""
    va, vb = vals(*a), vals(*b)
    return (frozenset(x - y for x in va for y in vb if x - y >= 0),
            (max(a[0], b[0]), max(a[1], b[1])))


def site_join(a, b):
    return vals(*a) | vals(*b), (max(a[0], b[0]), max(a[1], b[1]))


SITES = [("mul_full", site_mul_full),
         ("add_mixed", site_add_mixed),
         ("sub_mixed", site_sub_mixed),
         ("join", site_join)]


def main():
    print("# P1. sites, formulas, tightness, and whether any of them is the join")
    print()

    bad = check_componentwise()
    print("## Q0. is inclusion in this family componentwise on (I,F)?")
    print(f"   disagreements between set inclusion and coordinate order: {len(bad)}")
    if bad:
        print(f"   FIRST: {bad[0]}   -- everything below is void, stop here")
        return
    print("   so the coordinate order IS the inclusion order here, and a")
    print("   coordinatewise formula is a statement about value sets.")
    print()

    # cross-check the two least-containing methods on the control box
    ctrl = [(I, F) for I in range(0, CTRL_LIM + 1) for F in range(0, CTRL_LIM + 1)]
    wide = [(I, F) for I in range(0, CTRL_WIDE + 1) for F in range(0, CTRL_WIDE + 1)]
    dis = 0
    checked = 0
    anti = 0
    for name, fn in SITES:
        for a in ctrl:
            for b in ctrl:
                s, _ = fn(a, b)
                slow = least_containing_slow(s, wide)
                fast = least_containing_fast(s)
                if isinstance(slow, tuple) and slow and slow[0] == "ANTICHAIN":
                    anti += 1
                    continue
                checked += 1
                if slow != fast:
                    dis += 1
                    if dis == 1:
                        print(f"   FIRST DISAGREEMENT {name} U{a} U{b}: slow={slow} fast={fast}")
    print("## Q1. do the two least-containing methods agree?")
    print(f"   pairs cross-checked : {checked}")
    print(f"   disagreements       : {dis}")
    print(f"   antichains found by the brute-force method : {anti}")
    if dis:
        print("   NUMBERS BELOW ARE VOID")
        return
    print()

    pairs = [(a, b) for a in SHAPES for b in SHAPES]
    print(f"## Q2. per site, over {len(pairs)} operand pairs (I,F in 0..{LIM})")
    print()
    for name, fn in SITES:
        tight = overshoot = wrong = 0
        eq_join = 0
        over_ex = None
        wrong_ex = None
        for a, b in pairs:
            s, cand = fn(a, b)
            least = least_containing_fast(s)
            # does the formula's shape contain the exact result?
            if not (cand[0] >= least[0] and cand[1] >= least[1]):
                wrong += 1
                if wrong_ex is None:
                    wrong_ex = (a, b, cand, least)
                continue
            if cand == least:
                tight += 1
            else:
                overshoot += 1
                if over_ex is None:
                    over_ex = (a, b, cand, least)
            if cand == (max(a[0], b[0]), max(a[1], b[1])):
                eq_join += 1
        n = len(pairs)
        print(f"### {name}")
        print(f"   formula contains the exact result   : {tight+overshoot}/{n}")
        print(f"   formula IS the least such (tight)   : {tight}/{n}")
        print(f"   formula overshoots                  : {overshoot}/{n}")
        print(f"   formula does NOT contain (would be a bug) : {wrong}/{n}")
        print(f"   formula == join of the operands     : {eq_join}/{n}")
        if over_ex:
            a, b, c, l = over_ex
            print(f"   first overshoot : U{a} x U{b} -> formula U{c}, least U{l}")
        if wrong_ex:
            a, b, c, l = wrong_ex
            print(f"   first WRONG     : U{a} x U{b} -> formula U{c}, least U{l}")
        print()

    print("## Q3. the claim under test: is the product numeral the join?")
    same = sum(1 for a, b in pairs
               if (a[0] + b[0], a[1] + b[1]) == (max(a[0], b[0]), max(a[1], b[1])))
    print(f"   product shape == join shape : {same}/{len(pairs)}")
    print(f"   product shape != join shape : {len(pairs)-same}/{len(pairs)}")
    coin = [(a, b) for a, b in pairs
            if (a[0] + b[0], a[1] + b[1]) == (max(a[0], b[0]), max(a[1], b[1]))]
    print(f"   every coincidence has min(I1,I2)==0 and min(F1,F2)==0 : "
          f"{all(min(a[0],b[0])==0 and min(a[1],b[1])==0 for a,b in coin)}")
    print("   so the two agree only where one operand contributes nothing on")
    print("   each coordinate, which is the degenerate corner rather than a region.")
    print()

    print("## Q4. does any site's formula coincide with the join, over the whole box?")
    for name, fn in SITES:
        k = sum(1 for a, b in pairs
                if fn(a, b)[1] == (max(a[0], b[0]), max(a[1], b[1])))
        print(f"   {name:10s} : {k}/{len(pairs)}")
    print()

    print("## Q5. where the JOIN itself is the site, is it tight?")
    print("   (this is the one site whose formula IS a lattice operation, and it")
    print("    is measured above as the 'join' row; a tight count equal to the")
    print("    pair count means the coordinatewise answer and the least upper")
    print("    bound coincide everywhere in this family)")


if __name__ == "__main__":
    main()
