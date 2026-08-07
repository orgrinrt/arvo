"""Instrument 2: the corrections to instrument 1, plus the questions it could not ask.

WHY THIS FILE EXISTS. Instrument 1's Q1 reported 1024 agreements and zero disagreements
between the four-condition predicate and true set inclusion, which would refute
02_carried section 1.6. It is wrong, and the reason is setup that helps: its shape list
was I in 0..3 and F in 0..3, in which the ONLY numeral carrying fewer than two values is
UF<0,0>, whose declared step is 1, the coarsest in the box. The disagreement 1.6 predicts
needs a singleton whose DECLARED step is finer than the target's, and no such shape was
offered to the predicate. The instrument never entered the path that breaks.

Corrected here, and the correction is the point: a singleton at a fine declared grid is
UF<-F, F>, one value, step r^-F.

What else this file adds, none of which instrument 1 could ask:

  Q7   the same four-condition check with singletons and empty shapes present
  Q8   meets in the unsigned family with the origin shape REFUSED (instrument 1 always
       had it in the space when I >= 0, so it never tested admitting it)
  Q9   the signed families, symmetric and asymmetric-low
  Q10  the coordinatewise meet on (F, -L, G) against the order-theoretic meet, which
       are different claims and are being conflated in the record I was given
  Q11  join failures split into box-truncation and structural, since a bounded
       enumeration cannot see an upper bound that lives above its own ceiling
"""

from fractions import Fraction as F
from itertools import combinations
from i1_shape_space import Space, uf, sf_sym, sf_asym, fl, shape_params, report


# ---------------------------------------------------------------- Q7


def declared(r, I, Fw, bias=F(0)):
    """(label, valueset, declared_step, declared_bias, n)."""
    q = F(1, r**Fw) if Fw >= 0 else F(r ** (-Fw))
    n = r ** (I + Fw) if I + Fw >= 0 else 0
    s = frozenset(bias + k * q for k in range(n))
    return (f"UF{r}<{I},{Fw}>" + (f"+{bias}" if bias else ""), s, q, bias, n)


def four_cond(src, tgt):
    """The four-condition predicate on DECLARED parameters: grid, phase, floor, ceiling."""
    _, s1, q1, b1, n1 = src
    _, s2, q2, b2, n2 = tgt
    if n1 == 0 or n2 == 0:
        return None  # endpoints undefined on an empty set; reported separately
    L1, G1 = b1, b1 + (n1 - 1) * q1
    L2, G2 = b2, b2 + (n2 - 1) * q2
    grid = (q1 / q2).denominator == 1
    phase = ((b1 - b2) / q2).denominator == 1
    return grid and phase and L2 <= L1 and G1 <= G2


def q7():
    print("Q7  four-condition predicate against true inclusion, WITH singletons at fine grids")
    rows = []
    for r in (2, 3):
        for I in range(-3, 4):
            for Fw in range(0, 4):
                if I + Fw < 0:
                    continue
                rows.append(declared(r, I, Fw))
    tot = agree = 0
    dis_lt2 = dis_ge2 = 0
    ws = []
    for a in rows:
        for b in rows:
            p = four_cond(a, b)
            if p is None:
                continue
            tot += 1
            truth = a[1] <= b[1]
            if truth == p:
                agree += 1
            elif a[4] < 2:
                dis_lt2 += 1
                if len(ws) < 3:
                    ws.append((a, b, truth, p))
            else:
                dis_ge2 += 1
                print(f"    UNEXPLAINED {a[0]} into {b[0]}: truth={truth} predicate={p}")
    print(f"    ordered pairs {tot}   agree {agree}   disagree {tot - agree}")
    print(f"      source carries fewer than two values: {dis_lt2}")
    print(f"      source carries two or more values:    {dis_ge2}")
    for a, b, t, p in ws:
        print(f"    witness: {a[0]} = {sorted(a[1])} into {b[0]}: really {t}, predicate {p}")
    print("    (a source of two or more values pins its own step, so the grid clause is")
    print("     forced; a singleton lies on every grid and the declared step is not)")
    print()


# ---------------------------------------------------------------- Q8, Q9


def box_ok_uf(r, I, Fw, lim):
    return 0 <= I + Fw <= lim


def q8():
    print("Q8  does the meet need the origin shape? (unsigned fixed-point, radix 2, zero bias)")
    for lim in (6, 7):
        full = [(f"UF<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
        cut = [(n, s) for (n, s) in full if s != frozenset({F(0)})]
        rf = report(f"origin ADMITTED    lim={lim}", Space(full))
        rc = report(f"origin REFUSED     lim={lim}", Space(cut))
        print(f"    meets lost by refusing the origin: {rc['meet_none_at_all'] + rc['meet_no_greatest'] - (rf['meet_none_at_all'] + rf['meet_no_greatest'])}")
    print()


def q9():
    print("Q9  the signed families")
    for lim in (5, 6):
        sym = [(f"S<{I},{Fw}>", sf_sym(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
        asy = [(f"A<{I},{Fw}>", sf_asym(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
        uns = [(f"U<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
        report(f"symmetric alone            lim={lim}", Space(sym))
        report(f"asymmetric-low alone       lim={lim}", Space(asy))
        report(f"unsigned + symmetric       lim={lim}", Space(uns + sym))
        report(f"unsigned + sym + asym      lim={lim}", Space(uns + sym + asy))
    print()


# ---------------------------------------------------------------- Q10


def coords(s):
    """(step, -L, G) for a uniform set, the frame 150 is reported to use."""
    p = shape_params(s)
    if p is None:
        return None
    q, b, L, G = p
    return (q, -L, G)


def log2_of(q):
    """F with q = 2**-F, for a step that is a power of two."""
    return (q.denominator.bit_length() - 1) - (q.numerator.bit_length() - 1)


def q10():
    print("Q10 coordinatewise meet on (step, -L, G) against the order-theoretic meet")
    print("    these are DIFFERENT claims: one asks whether a formula's output is a shape,")
    print("    the other asks whether a greatest lower bound exists at all.")
    lim = 6
    labelled = [(f"UF<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
    sp = Space(labelled)
    realised = coord_not_shape = skipped = 0
    ws = []
    allcoords = [coords(s) for s in sp.sets]
    for i, j in combinations(range(sp.n), 2):
        ci, cj = allcoords[i], allcoords[j]
        if ci[0] is None or cj[0] is None:
            skipped += 1
            continue
        target = (max(ci[0], cj[0]), min(ci[1], cj[1]), min(ci[2], cj[2]))
        if any(c == target for c in allcoords):
            realised += 1
        else:
            coord_not_shape += 1
            if len(ws) < 4:
                k = sp.greatest(sp.below[i] & sp.below[j])
                ws.append((sp.names[i], sp.names[j], target, sp.names[k] if k is not None else "NONE"))
    print(f"    pairs compared {realised + coord_not_shape}   (singleton sources skipped: {skipped})")
    print(f"      componentwise triple IS a shape in the family:     {realised}")
    print(f"      componentwise triple is NOT a shape in the family: {coord_not_shape}")
    for a, b, t, m in ws:
        print(f"      {a} & {b}: componentwise (step,-L,G)={t} names no shape; the actual meet is {m}")
    print("    the order-theoretic meet existed for every pair (Q8, origin admitted), so a")
    print("    closure condition stated for the FORMULA is not a condition on the meet.")
    print()


def q11():
    print("Q11 join failures split: box truncation against structural")
    lim = 6
    fx = [(f"UF<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
    sp = Space(fx)
    allc = [coords(s) for s in sp.sets]
    artifact = structural = ok = nolst = 0
    for i, j in combinations(range(sp.n), 2):
        uc = sp.above[i] & sp.above[j]
        if uc:
            if sp.least(uc) is None:
                nolst += 1
            else:
                ok += 1
            continue
        ci, cj = allc[i], allc[j]
        if ci[0] is None or cj[0] is None:
            structural += 1
            continue
        Fn = max(log2_of(ci[0]), log2_of(cj[0]))
        Gn = max(ci[2], cj[2])
        In = 0
        while F(2) ** In - F(2) ** (-Fn) < Gn:
            In += 1
        if In + Fn > lim:
            artifact += 1
        else:
            structural += 1
            print(f"    STRUCTURAL no-upper-bound inside the box: {sp.names[i]} & {sp.names[j]}")
    print(f"    pairs {ok + nolst + artifact + structural}")
    print(f"      join present                          {ok}")
    print(f"      upper bounds present, none least      {nolst}")
    print(f"      no upper bound, join shape OUTSIDE box {artifact}   <- artifact of the bound")
    print(f"      no upper bound, join shape INSIDE box  {structural}")
    print()


def q12():
    print("Q12 cross-radix joins: structural or artifact?")
    lim = 4
    r2 = [(f"UF2<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
    r3 = [(f"UF3<{I},{Fw}>", uf(3, I, Fw)) for I in range(0, lim) for Fw in range(0, lim) if box_ok_uf(3, I, Fw, 3)]
    sp = Space(r2 + r3)
    allc = [coords(s) for s in sp.sets]
    cross = nobound = nobound_incommensurable = 0
    for i, j in combinations(range(sp.n), 2):
        ni, nj = sp.names[i], sp.names[j]
        is2i, is3i = "UF2" in ni, "UF3" in ni
        is2j, is3j = "UF2" in nj, "UF3" in nj
        if not ((is2i and is3j) or (is3i and is2j)):
            continue
        cross += 1
        if sp.above[i] & sp.above[j]:
            continue
        nobound += 1
        qi, qj = allc[i][0], allc[j][0]
        if qi is None or qj is None:
            continue
        # a common upper bound needs a step dividing both. the admitted steps are
        # 2^-a and 3^-b. a step dividing a radix-2 step and a radix-3 step must divide
        # their gcd, and gcd(2^-a, 3^-b) has both 2 and 3 in its denominator unless
        # one of them is an integer step.
        g = F(1, (qi.denominator * qj.denominator) // __import__("math").gcd(qi.denominator, qj.denominator))
        d = g.denominator
        two_only = (d & (d - 1)) == 0
        three_only = all(x == 3 for x in _factor(d)) if d > 1 else True
        if not (two_only or three_only):
            nobound_incommensurable += 1
    print(f"    cross-radix pairs {cross}   with no common upper bound in the space {nobound}")
    print(f"      of those, the required step is neither a power of 1/2 nor of 1/3: {nobound_incommensurable}")
    print("      that count is structural: no enlargement of either radix's box supplies")
    print("      a step with both 2 and 3 in its denominator.")
    print()


def _factor(n):
    fs = []
    d = 2
    while d * d <= n:
        while n % d == 0:
            fs.append(d)
            n //= d
        d += 1
    if n > 1:
        fs.append(n)
    return fs



def q13():
    print("Q13 the meet stops being EXACT once two sign domains share one order")
    lim = 5
    uns = [(f"U<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
    sym = [(f"S<{I},{Fw}>", sf_sym(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if box_ok_uf(2, I, Fw, lim)]
    for tag, sp in (("unsigned alone", Space(uns)), ("unsigned + symmetric", Space(uns + sym))):
        exact = under = 0
        ws = []
        for i, j in combinations(range(sp.n), 2):
            k = sp.greatest(sp.below[i] & sp.below[j])
            if k is None:
                continue
            inter = sp.sets[i] & sp.sets[j]
            if sp.sets[k] == inter:
                exact += 1
            else:
                under += 1
                if len(ws) < 3:
                    ws.append((sp.names[i], sp.names[j], sorted(inter), sorted(sp.sets[k])))
        print(f"    {tag}: meets exact {exact}, meets strictly undershooting the intersection {under}")
        for a, b, inter, got in ws:
            print(f"      {a} & {b}")
            print(f"        set intersection      {inter}")
            print(f"        greatest lower bound  {got}   <- a proper subset of it")
    print()


def q14():
    print("Q14 what the meet needs once a bias is admitted")
    biases = (F(0), F(1, 2), F(1, 4), F(1, 3))
    labelled = []
    for I in range(0, 4):
        for Fw in range(0, 4):
            if not box_ok_uf(2, I, Fw, 4):
                continue
            for b in biases:
                labelled.append((f"U<{I},{Fw}>+{b}", uf(2, I, Fw, b)))
    sp = Space(labelled)
    disjoint = nonempty_no_glb = 0
    ws = []
    for i, j in combinations(range(sp.n), 2):
        inter = sp.sets[i] & sp.sets[j]
        k = sp.greatest(sp.below[i] & sp.below[j])
        if k is not None:
            continue
        if not inter:
            disjoint += 1
        else:
            nonempty_no_glb += 1
            if len(ws) < 3:
                ws.append((sp.names[i], sp.names[j], sorted(inter)))
    print(f"    pairs {sp.n * (sp.n - 1) // 2}")
    print(f"      no greatest lower bound because the intersection is EMPTY:    {disjoint}")
    print(f"      no greatest lower bound though the intersection is NONEMPTY:  {nonempty_no_glb}")
    for a, b, inter in ws:
        print(f"      {a} & {b}: intersection {inter} names no shape in the space")
    print("    so a biased space needs an empty numeral for the first count and a singleton")
    print("    at every reachable phase for the second. They are two separate admissions.")
    print()


def q15():
    print("Q15 float family alone: are meet and join total there?")
    for pmax, elo, ehi in ((3, -3, 2), (4, -3, 2)):
        fls = [
            (f"FL<p{p},e{a}..{b}>", fl(p, a, b))
            for p in range(1, pmax + 1)
            for a in range(elo, 1)
            for b in range(0, ehi + 1)
            if a <= b
        ]
        report(f"float alone, p<= {pmax}, e in [{elo},{ehi}]", Space(fls))
    print()



def q16():
    print("Q16 does negative integer width restore the exact meet across two sign domains?")
    lim = 5
    for lo in (0, -3):
        uns = [(f"U<{I},{Fw}>", uf(2, I, Fw)) for I in range(lo, lim + 1) for Fw in range(0, lim + 1) if 0 <= I + Fw <= lim]
        sym = [(f"S<{I},{Fw}>", sf_sym(2, I, Fw)) for I in range(lo, lim + 1) for Fw in range(0, lim + 1) if 0 <= I + Fw <= lim]
        sp = Space(uns + sym)
        exact = under = nogl = 0
        ws = []
        for i, j in combinations(range(sp.n), 2):
            k = sp.greatest(sp.below[i] & sp.below[j])
            if k is None:
                nogl += 1
                continue
            if sp.sets[k] == (sp.sets[i] & sp.sets[j]):
                exact += 1
            else:
                under += 1
                if len(ws) < 2:
                    ws.append((sp.names[i], sp.names[j], sorted(sp.sets[i] & sp.sets[j]), sorted(sp.sets[k])))
        tag = "I >= 0" if lo == 0 else "I >= -3 (negative integer width admitted)"
        print(f"    unsigned + symmetric, {tag}: points {sp.n}, meets exact {exact}, undershoot {under}, no glb {nogl}")
        for a, b, inter, got in ws:
            print(f"      residual undershoot {a} & {b}: intersection {inter}, glb {got}")
    print()


if __name__ == "__main__":
    print("instrument 2: instrument 1's corrections, and the questions it could not ask")
    print("=" * 78)
    q7()
    q8()
    q9()
    q10()
    q11()
    q12()
    q13()
    q14()
    q15()
    q16()
