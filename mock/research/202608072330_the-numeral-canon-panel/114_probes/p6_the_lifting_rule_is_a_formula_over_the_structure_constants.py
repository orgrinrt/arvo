#!/usr/bin/env python3
"""p6. Does the per-construction lifting rule generalise, or is it a table?

WHAT IS ESTABLISHED
-------------------
`112` F112-13: a construction's grade transformer is not its base's, and
borrowing the base's is unsound. Componentwise applied to complex multiplication
is unsound on 26 of 81 pairs.

`112` F112-14: the smallest sound transformer differs per construction and is a
joint fact with the base's signedness. Three constructions measured: `product2`
needs the componentwise rule, `dual` needs twice it, and `complex` needs twice it
over a signed base and has NO sound magnitude-only rule over an unsigned one.

`110` R6 reproduces the three unsoundness figures on an independent instrument
and adds the congruence-against-lifting split.

WHAT IS NOT ESTABLISHED
-----------------------
Whether "per construction" means "per construction, from a table someone writes"
or "per construction, from a formula anyone can evaluate". Three constructions is
three data points, and `112`'s own prose gestures at a derivation ("read off each
construction's multiplication and the rules are derivable rather than guessed")
without stating the derivation or testing it on a fourth.

That distinction decides what a canon can say. A table is a design obligation
that grows with every construction anyone adds. A formula is a sentence.

THE CANDIDATE FORMULA
---------------------
A bilinear construction on `d`-tuples over a base is fixed by its structure
constants: output component `i` is

    out_i = sum over j,k of  c[i][j][k] * a_j * b_k

Under a magnitude declaration `|component| <= m`, each product `a_j * b_k` lies
in `[-m^2, m^2]` over a signed base and in `[0, m^2]` over an unsigned one. So

    over a SIGNED base:    out_i lies in [-N_i * m^2, +N_i * m^2]
                           where N_i = sum over j,k of |c[i][j][k]|,
                           the L1 NORM of that output component's row.

    over an UNSIGNED base: out_i lies in [neg_i * m^2, pos_i * m^2]
                           where pos_i and neg_i are the sums of the positive
                           and negative constants. If neg_i is nonzero the lower
                           end is below zero, which an unsigned carrier cannot
                           hold, so NO magnitude-only rule discharges it at any
                           nonzero m.

So the transformer is predicted to be a function of ONE number per output
component, the L1 norm of its structure-constant row, together with one bit,
whether that row has a negative entry, read against the base's signedness.

Checked against `112`'s three:

    product2   rows (1), (1)            N = 1, no negatives  -> m^2 <= hi
    dual       rows (1), (1,1)          N = 1 and 2          -> 2 m^2 <= hi
    complex    rows (1,-1), (1,1)       N = 2, row 0 negative-> unsigned: none
                                                                signed: 2 m^2

which is `112` F112-14 exactly, arrived at without measuring it. That is a
prediction and this probe tests it on the whole family rather than on those
three.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. The formula is SOUND on every construction in the family: no cell where the
    formula says the declaration discharges and the arms in fact disagree.
P2. It is EXACT on most of the family and conservative on some, because the
    corner analysis ignores that `a_j` and `b_k` cannot independently attain
    their extremes when a component appears twice in one row with opposite sign.
P3. It reproduces `112`'s three verdicts, including the "no sound rule over an
    unsigned base" for complex.
P4. It holds at dimension 4 as well as 2, since nothing in the derivation
    mentions the dimension. Quaternion multiplication has L1 norm 4 on every
    row and a negative entry on every row, so it is predicted to need
    `4 m^2 <= bound` over a signed base and to be undischargeable over an
    unsigned one.

NEGATIVE CONTROLS
-----------------
C1. The componentwise rule (`m^2 <= hi`, `112`'s "borrow the base's") applied to
    every construction must be reported UNSOUND on some of them, or the
    soundness counter is a dead branch.
C2. A formula with the L1 norm replaced by 1 (ignoring the row) must be unsound
    on the constructions whose norm exceeds 1.
C3. The formula must be shown to REFUSE somewhere, or "sound" is being measured
    on a rule that discharges nothing.
"""

from fractions import Fraction
from itertools import product
import random
import sys


class Base:
    def __init__(self, W, signed, policy):
        self.W, self.signed, self.policy = W, signed, policy
        n = 1 << W
        self.lo = -(n // 2) if signed else 0
        self.hi = (n // 2 - 1) if signed else n - 1

    def __repr__(self):
        return f"{'i' if self.signed else 'u'}W{self.W}/{self.policy}"

    def values(self):
        return list(range(self.lo, self.hi + 1))

    def R(self, v):
        if self.lo <= v <= self.hi:
            return v
        if self.policy == "sat":
            return self.hi if v > self.hi else self.lo
        span = self.hi - self.lo + 1
        return ((v - self.lo) % span) + self.lo


# ------------------------------------------------------- bilinear constructions
# c[i][j][k]: the coefficient of a_j * b_k in output component i.


def mul_general(B, c, x, y, general):
    d = len(c)
    out = []
    for i in range(d):
        acc = 0
        for j in range(d):
            for k in range(d):
                w = c[i][j][k]
                if w == 0:
                    continue
                term = B.R(x[j] * y[k]) if general else x[j] * y[k]
                acc = B.R(acc + w * term) if general else acc + w * term
        out.append(acc)
    return tuple(out)


def norms(c):
    d = len(c)
    out = []
    for i in range(d):
        n1 = sum(abs(c[i][j][k]) for j in range(d) for k in range(d))
        neg = any(c[i][j][k] < 0 for j in range(d) for k in range(d))
        out.append((n1, neg))
    return out


def formula_discharges(B, c, m):
    """The candidate rule. `m` is the declared magnitude bound on every
    component: [0, m] over an unsigned base, [-m, m] over a signed one."""
    for n1, neg in norms(c):
        if B.signed:
            if n1 * m * m > min(B.hi, -B.lo):
                return False
        else:
            if neg and m > 0:
                return False
            if n1 * m * m > B.hi:
                return False
    return True


def componentwise_discharges(B, c, m):
    """C1: `112`'s 'borrow the base's rule', which ignores the construction."""
    return (m * m <= B.hi) if not B.signed else (m * m <= min(B.hi, -B.lo))


def unit_norm_discharges(B, c, m):
    """C2: the formula with every L1 norm forced to 1."""
    for _n1, neg in norms(c):
        if B.signed:
            if m * m > min(B.hi, -B.lo):
                return False
        else:
            if neg and m > 0:
                return False
            if m * m > B.hi:
                return False
    return True


def arms_agree(B, c, m):
    """Do the exact and the clamping composites agree on every pair inside the
    declaration? This is the ground truth the rules are predicting."""
    d = len(c)
    dom = [v for v in B.values() if (0 <= v <= m if not B.signed else -m <= v <= m)]
    if not dom:
        return True
    for x in product(dom, repeat=d):
        for y in product(dom, repeat=d):
            if mul_general(B, c, x, y, False) != mul_general(B, c, x, y, True):
                return False
    return True


# ------------------------------------------------------------------ the family


def family_dim2():
    """Every bilinear product on pairs with structure constants in {-1, 0, 1},
    restricted to rows with at most two nonzero entries, which keeps the sweep
    finite and covers every named two-dimensional algebra."""
    cells = [(j, k) for j in range(2) for k in range(2)]
    rows = [((0, 0, 0, 0))]
    rows = []
    for a in cells:
        for wa in (1, -1):
            r = [[0, 0], [0, 0]]
            r[a[0]][a[1]] = wa
            rows.append(tuple(tuple(x) for x in r))
    for a in cells:
        for b in cells:
            if a >= b:
                continue
            for wa in (1, -1):
                for wb in (1, -1):
                    r = [[0, 0], [0, 0]]
                    r[a[0]][a[1]] = wa
                    r[b[0]][b[1]] = wb
                    rows.append(tuple(tuple(x) for x in r))
    out = []
    for r0 in rows:
        for r1 in rows:
            out.append((r0, r1))
    return out


NAMED = {
    "product2": (((1, 0), (0, 0)), ((0, 0), (0, 1))),
    "dual": (((1, 0), (0, 0)), ((0, 1), (1, 0))),
    "complex": (((1, 0), (0, -1)), ((0, 1), (1, 0))),
    "split-complex": (((1, 0), (0, 1)), ((0, 1), (1, 0))),
}


def quaternion():
    """Hamilton's product as structure constants, to test P4 at dimension 4."""
    d = 4
    c = [[[0] * d for _ in range(d)] for _ in range(d)]
    # 1, i, j, k with i^2 = j^2 = k^2 = ijk = -1
    tab = {
        (0, 0): (0, 1), (0, 1): (1, 1), (0, 2): (2, 1), (0, 3): (3, 1),
        (1, 0): (1, 1), (1, 1): (0, -1), (1, 2): (3, 1), (1, 3): (2, -1),
        (2, 0): (2, 1), (2, 1): (3, -1), (2, 2): (0, -1), (2, 3): (1, 1),
        (3, 0): (3, 1), (3, 1): (2, 1), (3, 2): (1, -1), (3, 3): (0, -1),
    }
    for (j, k), (i, w) in tab.items():
        c[i][j][k] = w
    return tuple(tuple(tuple(r) for r in comp) for comp in c)


def main():
    print("=" * 78)
    print("p6. Is the lifting rule a formula or a table?")
    print("=" * 78)

    # ------------------------------------------------- P3: the named three
    print()
    print("P3. The formula against `112` F112-14's three verdicts, and one more.")
    print()
    print(f"  {'construction':<16} {'L1 norms':<14} {'has a negative':<15} "
          f"{'formula says':<28}")
    for name, c in list(NAMED.items()) + [("quaternion", quaternion())]:
        ns = norms(c)
        n1s = ", ".join(str(n) for n, _ in ns)
        negs = ", ".join("yes" if g else "no" for _, g in ns)
        Bs = Base(4, True, "sat")
        Bu = Base(4, False, "sat")
        ms = max([m for m in range(0, 9) if formula_discharges(Bu, c, m)] or [-1])
        mss = max([m for m in range(0, 9) if formula_discharges(Bs, c, m)] or [-1])
        print(
            f"  {name:<16} {n1s:<14} {negs:<15} "
            f"unsigned largest m {ms}, signed largest m {mss}"
        )
    print()
    print("  `112` F112-14 reads: product2 componentwise, dual twice it, complex")
    print("  twice it over a signed base and nothing over an unsigned one. The")
    print("  unsigned column above must show complex at m = 0 for that to match.")

    # --------------------------------------------- P1, P2, C1, C2, C3 at dim 2
    print()
    print("-" * 78)
    print("P1, P2 and the controls, over the whole two-dimensional family.")
    print()
    fam = family_dim2()
    print(f"  constructions in the family: {len(fam)}")
    for B in (Base(3, False, "sat"), Base(3, True, "sat"), Base(3, False, "wrap")):
        f_uns = f_cons = f_fires = 0
        c1_uns = c1_fires = 0
        c2_uns = c2_fires = 0
        cells = 0
        agree_cells = 0
        for c in fam:
            for m in range(0, B.hi + 1):
                cells += 1
                ag = arms_agree(B, c, m)
                agree_cells += ag
                f = formula_discharges(B, c, m)
                c1 = componentwise_discharges(B, c, m)
                c2 = unit_norm_discharges(B, c, m)
                f_fires += f
                c1_fires += c1
                c2_fires += c2
                if f and not ag:
                    f_uns += 1
                if ag and not f:
                    f_cons += 1
                if c1 and not ag:
                    c1_uns += 1
                if c2 and not ag:
                    c2_uns += 1
        print()
        print(f"  base {B}   cells {cells}   arms agree on {agree_cells}")
        print(
            f"    FORMULA          discharges {f_fires:>6}   unsound {f_uns:>5}   "
            f"conservative {f_cons:>6}"
        )
        print(
            f"    C1 componentwise discharges {c1_fires:>6}   unsound {c1_uns:>5}"
        )
        print(
            f"    C2 unit norm     discharges {c2_fires:>6}   unsound {c2_uns:>5}"
        )
        print(
            f"    C3: the formula refuses on {cells - f_fires} of {cells} cells"
        )

    # ---- A wider SIGNED base, because at iW3 the container is too narrow for
    # ---- the L1 norm to matter: every rule collapses to m <= 1 and the
    # ---- controls stop discriminating, which is a defect in the sweep above
    # ---- rather than evidence for the formula.
    print()
    print("-" * 78)
    print("A wider signed base, so the signed controls can fire. The family is")
    print("sampled here because the exhaustive pair enumeration grows as m^8.")
    print()
    rng = random.Random(20260814)
    fam_s = rng.sample(fam, 150)
    for B in (Base(5, True, "sat"), Base(6, True, "sat")):
        f_uns = f_cons = f_fires = c1_uns = c1_fires = c2_uns = c2_fires = 0
        cells = agree_cells = 0
        for c in fam_s:
            for m in range(0, 5):
                cells += 1
                ag = arms_agree(B, c, m)
                agree_cells += ag
                f = formula_discharges(B, c, m)
                c1 = componentwise_discharges(B, c, m)
                c2 = unit_norm_discharges(B, c, m)
                f_fires += f
                c1_fires += c1
                c2_fires += c2
                if f and not ag:
                    f_uns += 1
                if ag and not f:
                    f_cons += 1
                if c1 and not ag:
                    c1_uns += 1
                if c2 and not ag:
                    c2_uns += 1
        print(f"  base {B}   cells {cells} (150 sampled constructions, m in 0..4)"
              f"   arms agree on {agree_cells}")
        print(f"    FORMULA          discharges {f_fires:>6}   unsound {f_uns:>5}"
              f"   conservative {f_cons:>6}")
        print(f"    C1 componentwise discharges {c1_fires:>6}   unsound {c1_uns:>5}")
        print(f"    C2 unit norm     discharges {c2_fires:>6}   unsound {c2_uns:>5}")

    # ------------------------------------------------------ P4: dimension four
    print()
    print("-" * 78)
    print("P4. Dimension four, where nothing in the derivation changes.")
    print()
    q = quaternion()
    for B in (Base(3, True, "sat"), Base(4, True, "sat"), Base(3, False, "sat")):
        rows = []
        for m in range(0, min(3, B.hi) + 1):
            ag = arms_agree(B, q, m)
            f = formula_discharges(B, q, m)
            c1 = componentwise_discharges(B, q, m)
            rows.append((m, ag, f, c1))
        print(f"  base {B}")
        print(f"    {'m':>3} {'arms agree':>11} {'formula':>9} {'C1 componentwise':>18}")
        for m, ag, f, c1 in rows:
            flag = ""
            if f and not ag:
                flag = "   <- FORMULA UNSOUND"
            if c1 and not ag:
                flag += "   <- C1 unsound"
            print(f"    {m:>3} {str(ag):>11} {str(f):>9} {str(c1):>18}{flag}")

    # -------------------------------------------- where the formula is not exact
    print()
    print("-" * 78)
    print("P2. Where the formula is conservative, named rather than counted.")
    print()
    B = Base(3, True, "sat")
    shown = 0
    for name, c in NAMED.items():
        for m in range(0, B.hi + 1):
            ag = arms_agree(B, c, m)
            f = formula_discharges(B, c, m)
            if ag and not f and shown < 10:
                ns = ", ".join(str(n) for n, _ in norms(c))
                print(
                    f"  {name:<16} m = {m}: the arms agree and the formula "
                    f"refuses (L1 norms {ns}, container [{B.lo}, {B.hi}])"
                )
                shown += 1
    if shown == 0:
        print("  none among the named constructions at this base")

    print()
    print("=" * 78)
    print(
        """
  The formula is a sentence rather than a table when its unsound count is zero
  across the family AND both controls are nonzero somewhere. If C1 and C2 never
  go unsound, the family contains no construction that distinguishes them and
  the sweep proves nothing about the formula.

  SCOPE, stated because it is narrower than it looks. This covers BILINEAR
  constructions on tuples, which is what `product2`, `dual`, `complex` and the
  quaternions are. It does NOT cover `interval`, whose product is a hull rather
  than a bilinear form, and whose transformer `110` F12 and `112` F112-12 handle
  by a monotonicity predicate instead. So the canon sentence is about bilinear
  constructions, and a non-bilinear construction remains its own obligation.
"""
    )


if __name__ == "__main__":
    main()
