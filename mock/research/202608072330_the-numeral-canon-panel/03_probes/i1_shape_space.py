"""Instrument 1: brute force over materialised value sets.

Everything here is exact (fractions.Fraction) and everything is decided by ACTUAL SET
OPERATIONS on materialised value sets. No closed form is used anywhere in this file, so
that instrument 2 (closed form) has something independent to disagree with.

The questions, stated before any of them is computed:

  Q1  Does the four-condition inclusion predicate agree with true set inclusion, and if
      it disagrees, is the disagreement confined to sources carrying fewer than two
      values? (this is 02_carried section 1.6's claim, checked)

  Q2  Inside the unsigned fixed-point family at one radix with zero bias: is the meet
      total? is the join total? and does the answer move when the origin shape (I=F=0)
      and negative integer width are admitted or refused?

  Q3  The same two questions with two radices in one shape space.

  Q4  The same two questions with fixed-point and float in one shape space.

  Q5  Are there pairs of fixed-point numerals whose join EXISTS in the fixed-point-only
      space and STOPS existing once floats are added to the same space? (150's carried
      claim that adding floats removed a join that was already there)

  Q6  With a nonzero bias admitted, is the meet still total, and what does it need?

A note on the enumeration bound, because the prior panel had a headline count turn out to
be an artifact of one: every "no upper bound at all" answer is suspect under a bounded box,
since an upper bound may live outside it. Q2/Q3/Q4 therefore report the two failure kinds
SEPARATELY (no bound at all, versus bounds exist but none is least) and every count is run
at two box sizes so a reader can see whether it moved.
"""

from fractions import Fraction as F
from itertools import combinations

# ---------------------------------------------------------------- shape builders


def uf(r, I, Fw, bias=F(0)):
    """Unsigned fixed-point: r^(I+Fw) values, step r^-Fw, floor `bias`."""
    q = F(1, r**Fw) if Fw >= 0 else F(r ** (-Fw))
    n = r ** (I + Fw)
    if n < 1:
        return None
    return frozenset(bias + k * q for k in range(n))


def sf_sym(r, I, Fw):
    """Symmetric signed fixed-point: equal reach either way, one code spare at even radix."""
    q = F(1, r**Fw) if Fw >= 0 else F(r ** (-Fw))
    n = r ** (I + Fw)
    if n < 1:
        return None
    half = (n - 1) // 2
    return frozenset(k * q for k in range(-half, half + 1))


def sf_asym(r, I, Fw):
    """Two's-complement shaped: one quantum further down than up."""
    q = F(1, r**Fw) if Fw >= 0 else F(r ** (-Fw))
    n = r ** (I + Fw)
    if n < 2:
        return None
    lo = -(n // 2)
    return frozenset(k * q for k in range(lo, lo + n))


def fl(p, emin, emax, signed=False):
    """Binary float, normals plus zero. No subnormals, no specials: those change the set
    but not the question, and leaving them out keeps the comparison with fixed-point
    honest about what is being compared."""
    vals = {F(0)}
    for e in range(emin, emax + 1):
        for m in range(2 ** (p - 1), 2**p):
            v = F(m) * F(2) ** (e - p + 1)
            vals.add(v)
            if signed:
                vals.add(-v)
    return frozenset(vals)


# ---------------------------------------------------------------- order machinery


class Space:
    """A shape space: labelled value sets, quotiented by equal value set (150:55 requires
    the quotient, since two numerals denoting the same values are one point of the order)."""

    def __init__(self, labelled):
        seen = {}
        self.collisions = 0
        for name, s in labelled:
            if s is None:
                continue
            if s in seen:
                seen[s].append(name)
                self.collisions += 1
            else:
                seen[s] = [name]
        self.sets = list(seen.keys())
        self.names = [" = ".join(seen[s]) for s in self.sets]
        self.n = len(self.sets)
        self.below = [0] * self.n  # bitmask of j with V_j subset of V_i
        self.above = [0] * self.n
        for i in range(self.n):
            for j in range(self.n):
                if self.sets[j] <= self.sets[i]:
                    self.below[i] |= 1 << j
                    self.above[j] |= 1 << i

    def bits(self, mask):
        while mask:
            b = mask & -mask
            yield b.bit_length() - 1
            mask ^= b

    def greatest(self, cand):
        """The unique greatest element of a candidate mask, or None."""
        for k in self.bits(cand):
            if cand & ~self.below[k] == 0:
                return k
        return None

    def least(self, cand):
        for k in self.bits(cand):
            if cand & ~self.above[k] == 0:
                return k
        return None

    def survey(self):
        """Per unordered pair: meet status and join status."""
        res = {
            "pairs": 0,
            "meet_ok": 0,
            "meet_none_at_all": 0,
            "meet_no_greatest": 0,
            "join_ok": 0,
            "join_none_at_all": 0,
            "join_no_least": 0,
            "meet_witness": None,
            "join_witness": None,
            "meet_exact": 0,
            "meet_undershoot": 0,
        }
        for i, j in combinations(range(self.n), 2):
            res["pairs"] += 1
            lc = self.below[i] & self.below[j]
            if lc == 0:
                res["meet_none_at_all"] += 1
                if res["meet_witness"] is None:
                    res["meet_witness"] = ("none", self.names[i], self.names[j])
            else:
                k = self.greatest(lc)
                if k is None:
                    res["meet_no_greatest"] += 1
                    if res["meet_witness"] is None:
                        res["meet_witness"] = ("no-greatest", self.names[i], self.names[j])
                else:
                    res["meet_ok"] += 1
                    if self.sets[k] == (self.sets[i] & self.sets[j]):
                        res["meet_exact"] += 1
                    else:
                        res["meet_undershoot"] += 1
            uc = self.above[i] & self.above[j]
            if uc == 0:
                res["join_none_at_all"] += 1
                if res["join_witness"] is None:
                    res["join_witness"] = ("none", self.names[i], self.names[j])
            else:
                k = self.least(uc)
                if k is None:
                    res["join_no_least"] += 1
                    if res["join_witness"] is None:
                        res["join_witness"] = ("no-least", self.names[i], self.names[j])
                else:
                    res["join_ok"] += 1
        return res


def report(title, sp):
    r = sp.survey()
    print(f"  {title}")
    print(f"    points {sp.n:>5} (label collisions folded: {sp.collisions})   pairs {r['pairs']:>6}")
    print(
        f"    meet   ok {r['meet_ok']:>6}  (exact {r['meet_exact']}, undershoot {r['meet_undershoot']})"
        f"   no-lower-bound {r['meet_none_at_all']:>5}   bounds-but-no-greatest {r['meet_no_greatest']:>5}"
    )
    print(
        f"    join   ok {r['join_ok']:>6}"
        f"{'':>36}   no-upper-bound {r['join_none_at_all']:>5}   bounds-but-no-least   {r['join_no_least']:>5}"
    )
    if r["meet_witness"]:
        print(f"    first meet failure: {r['meet_witness'][0]}  {r['meet_witness'][1]}  &  {r['meet_witness'][2]}")
    if r["join_witness"]:
        print(f"    first join failure: {r['join_witness'][0]}  {r['join_witness'][1]}  &  {r['join_witness'][2]}")
    return r


# ---------------------------------------------------------------- Q1


def shape_params(s):
    """(q, b, L, G) read off a materialised uniform set. None if not uniform."""
    v = sorted(s)
    if len(v) == 0:
        return None
    if len(v) == 1:
        return (None, v[0], v[0], v[0])
    q = v[1] - v[0]
    for a, b in zip(v, v[1:]):
        if b - a != q:
            return None
    return (q, v[0], v[0], v[-1])


def four_condition(p1, p2, declared_q1):
    """The four-condition predicate, read off DECLARED parameters. declared_q1 is the
    source's declared step, which for a singleton is not recoverable from its value set,
    which is the whole point of Q1."""
    q1, b1, L1, G1 = p1
    q2, b2, L2, G2 = p2
    if q2 is None:
        # a singleton target: it can only contain a singleton equal to it
        return L1 == G1 == L2
    grid = (declared_q1 / q2).denominator == 1
    phase = ((b1 - b2) / q2).denominator == 1
    return grid and phase and L2 <= L1 and G1 <= G2


def q1():
    print("Q1  four-condition predicate against true set inclusion")
    print("    shapes: unsigned fixed-point radix 2, I in 0..3, F in 0..3, plus the same")
    print("    at radix 3, all with declared step read from the DECLARATION not the set.")
    rows = []
    for r in (2, 3):
        for I in range(0, 4):
            for Fw in range(0, 4):
                s = uf(r, I, Fw)
                declared_q = F(1, r**Fw)
                rows.append((f"UF{r}<{I},{Fw}>", s, declared_q))
    tot = agree = 0
    dis_small = dis_big = 0
    witness = None
    for na, sa, qa in rows:
        for nb, sb, qb in rows:
            tot += 1
            truth = sa <= sb
            pa, pb = shape_params(sa), shape_params(sb)
            pred = four_condition(pa, pb, qa)
            if truth == pred:
                agree += 1
            elif len(sa) < 2:
                dis_small += 1
                if witness is None:
                    witness = (na, nb, truth, pred)
            else:
                dis_big += 1
                print(f"    UNEXPLAINED: {na} <= {nb}  truth={truth} pred={pred}")
    print(f"    ordered pairs {tot}   agree {agree}   disagree {tot - agree}")
    print(f"      of the disagreements, source carries fewer than two values: {dis_small}")
    print(f"      of the disagreements, source carries two or more values:    {dis_big}")
    if witness:
        na, nb, truth, pred = witness
        print(f"    witness: {na} into {nb}: really included = {truth}, predicate says {pred}")
    print()


# ---------------------------------------------------------------- Q2..Q6


def q2():
    print("Q2  unsigned fixed-point, radix 2, zero bias, one family")
    for lo_I, tag in ((1, "I>=1, F>=1: neither the origin nor negative width"),
                      (0, "I>=0, F>=0: the origin shape admitted")):
        for box in (5, 6):
            labelled = [
                (f"UF<{I},{Fw}>", uf(2, I, Fw))
                for I in range(lo_I, box + 1)
                for Fw in range(lo_I, box + 1)
                if I + Fw <= box + 2
            ]
            report(f"{tag}   box={box}", Space(labelled))
    for box in (5, 6):
        labelled = [
            (f"UF<{I},{Fw}>", uf(2, I, Fw))
            for I in range(-3, box + 1)
            for Fw in range(0, box + 1)
            if 0 <= I + Fw <= box + 2
        ]
        report(f"I>=-3, F>=0: negative integer width admitted   box={box}", Space(labelled))
    print()


def q3():
    print("Q3  two radices in one shape space (unsigned fixed-point, zero bias)")
    for box in (4, 5):
        labelled = [(f"UF2<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, box + 1) for Fw in range(0, box + 1) if I + Fw <= box + 1]
        labelled += [(f"UF3<{I},{Fw}>", uf(3, I, Fw)) for I in range(0, box) for Fw in range(0, box) if I + Fw <= 4]
        report(f"radix 2 and radix 3 together   box={box}", Space(labelled))
    print()


def q4():
    print("Q4  fixed-point and float in one shape space (nonnegative, zero bias)")
    for box in (4, 5):
        fx = [(f"UF<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, box + 1) for Fw in range(0, box + 1) if I + Fw <= box + 1]
        fls = [
            (f"FL<p{p},e{emin}..{emax}>", fl(p, emin, emax))
            for p in range(1, 4)
            for emin in range(-3, 1)
            for emax in range(0, 3)
            if emin <= emax
        ]
        report(f"fixed-point alone   box={box}", Space(fx))
        report(f"fixed-point and float together   box={box}", Space(fx + fls))
    print()


def q5():
    print("Q5  joins that exist among fixed-point alone and stop existing once floats join")
    fx = [(f"UF<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, 5) for Fw in range(0, 5) if I + Fw <= 5]
    fls = [
        (f"FL<p{p},e{emin}..{emax}>", fl(p, emin, emax))
        for p in range(1, 4)
        for emin in range(-3, 1)
        for emax in range(0, 3)
        if emin <= emax
    ]
    a = Space(fx)
    b = Space(fx + fls)
    idx = {s: i for i, s in enumerate(b.sets)}
    lost_join, lost_meet, shown = 0, 0, 0
    for i, j in combinations(range(a.n), 2):
        ka = a.least(a.above[i] & a.above[j])
        if ka is None:
            continue
        bi, bj = idx[a.sets[i]], idx[a.sets[j]]
        kb = b.least(b.above[bi] & b.above[bj])
        if kb is None:
            lost_join += 1
            if shown < 3:
                shown += 1
                print(f"    LOST JOIN  {a.names[i]}  v  {a.names[j]}")
                print(f"      was {a.names[ka]}; with floats present there is no least upper bound")
                ub = [b.names[k] for k in b.bits(b.above[bi] & b.above[bj])]
                mins = [
                    b.names[k]
                    for k in b.bits(b.above[bi] & b.above[bj])
                    if not any(
                        m != k and b.sets[m] < b.sets[k] and b.sets[bi] <= b.sets[m] and b.sets[bj] <= b.sets[m]
                        for m in b.bits(b.above[bi] & b.above[bj])
                    )
                ]
                print(f"      upper bounds present: {len(ub)}, minimal ones: {mins[:4]}")
    for i, j in combinations(range(a.n), 2):
        ka = a.greatest(a.below[i] & a.below[j])
        if ka is None:
            continue
        bi, bj = idx[a.sets[i]], idx[a.sets[j]]
        kb = b.greatest(b.below[bi] & b.below[bj])
        if kb is None:
            lost_meet += 1
    print(f"    joins present among fixed-point alone and absent once floats join: {lost_join}")
    print(f"    meets present among fixed-point alone and absent once floats join:  {lost_meet}")
    print()


def q6():
    print("Q6  nonzero bias")
    labelled = []
    for I in range(0, 4):
        for Fw in range(0, 4):
            if I + Fw > 4:
                continue
            for bias in (F(0), F(1, 2), F(1, 3), F(1)):
                s = uf(2, I, Fw, bias)
                labelled.append((f"UF<{I},{Fw}>+{bias}", s))
    sp = Space(labelled)
    r = report("unsigned fixed-point radix 2 with four biases", sp)
    disjoint = 0
    for i, j in combinations(range(sp.n), 2):
        if not (sp.sets[i] & sp.sets[j]):
            disjoint += 1
    print(f"    pairs whose value sets are disjoint (so the only lower bound is the empty set): {disjoint}")
    print(f"    of the {r['meet_none_at_all']} pairs with no lower bound at all, disjoint ones: {disjoint}")
    print()


if __name__ == "__main__":
    print("instrument 1: brute force over materialised value sets, exact rationals")
    print("=" * 78)
    q1()
    q2()
    q3()
    q4()
    q5()
    q6()
