#!/usr/bin/env python3
"""
s1: do the two vocabularies denote the same value sets on the objects they both cover?

Vocabulary A, from 08:556-560 and 08:175-186 (the affine value map at
seed/SETTLED_laws.md:274):

    V(N) = { A * r^eps * k + B : k in K }

grouped by radix-binade, with the step inside binade e being A * r^{f(e)} for the
canonical exponent function f, all at one phase. So an A-numeral, written out in full,
is a five-tuple plus a reach:

    (radix r, adjustment A, bias B, phase p, canonical exponent f, reach R)

where R is the interval of magnitudes the numeral covers.

Vocabulary B, from 15:105-124: a numeral is keyed on (W, F), total width and fraction
width, both naturals, with I = W - F a derived view.

The hypothesis under test: B is A restricted to r=2, A=1, B=0, p=0, f constant, and
reach a full power-of-two interval anchored at zero. Under that restriction,
    F  = -f            (the constant canonical exponent, negated)
    W  = log_2 |K|     (the reach's cardinality exponent)
    I  = W - F         names the binade the reach stops at, exclusive.

Two things are checked, exactly, in Fraction arithmetic. No floats anywhere.

    Q1. Set equality. For every (W, F) in a box, the B-reading's value set equals the
        A-reading's value set built from (r=2, A=1, B=0, p=0, f == -F, R = [0, 2^{W-F})).

    Q2. 08's membership predicate (08:222-224), run at radix two over a DISCRIMINATING
        pool. The first version of this arm ran the predicate only over sets the same
        function had just generated as multiples of 2^{-F}, which made it incapable of
        failing. It now runs over a pool that contains sets which must be ACCEPTED and
        sets which must be REJECTED, and the arm fails if any verdict is wrong. The
        rejects are there to prove the predicate has teeth, not to say anything about
        those representations, which 08 classifies separately.

Run:  python3 s1_grid_reach_agreement.py
"""

from fractions import Fraction as Q

R = 2  # radix. B has no radix coordinate at all; this probe pins the one it assumes.
ADJ = Q(1)  # adjustment
BIAS = Q(0)  # bias
PHASE = Q(0)  # phase within the binade, as a multiple of the step


def b_value_set(w, f):
    """Vocabulary B: (W, F) unsigned. The stored integer k runs over [0, 2^W)."""
    step = Q(1, R**f)
    return {step * k for k in range(R**w)}


def a_value_set(radix, adj, bias, phase, fexp_const, reach_hi):
    """Vocabulary A, constant canonical exponent, reach [0, reach_hi).

    Inside every binade the step is adj * radix^{fexp}. With fexp constant the whole
    thing is one progression, so the set is every adj*radix^{fexp}*k + bias landing in
    [0, reach_hi). The phase is carried as an offset of that many steps.
    """
    step = adj * Q(radix) ** fexp_const
    out = set()
    k = 0
    while True:
        v = step * (k + phase) + bias
        if v >= reach_hi:
            break
        out.add(v)
        k += 1
    return out


def binade_of(v, radix):
    """e such that radix^e <= v < radix^{e+1}, for v > 0. Exact, no logs."""
    e = 0
    if v >= 1:
        while v >= Q(radix) ** (e + 1):
            e += 1
    else:
        while v < Q(radix) ** e:
            e -= 1
    return e


def is_power_of_radix(step, radix):
    """Is `step` equal to radix^q for some integer q? 08:557 requires the step to be
    'that radix raised to some power', and the first version of this function checked
    only that the step was CONSTANT, which let the decimal set through at radix two.
    The discriminating pool caught it; this clause is the repair."""
    if step <= 0:
        return False
    n, d = step.numerator, step.denominator
    if d == 1:
        v, q = 1, 0
        while v < n:
            v *= radix
            q += 1
        return v == n
    if n != 1:
        return False
    v, q = 1, 0
    while v < d:
        v *= radix
        q += 1
    return v == d


def check_binade_structure(values, radix, expect_step):
    """08's membership predicate (08:222-224): in each binade the denotable magnitudes
    are one arithmetic progression, at one phase, whose step is one adjustment times a
    power of the radix. Returns (ok, detail)."""
    if not is_power_of_radix(expect_step, radix):
        return False, f"step {expect_step} is not a power of radix {radix}"
    by_binade = {}
    for v in values:
        if v == 0:
            continue
        by_binade.setdefault(binade_of(v, radix), []).append(v)
    for e, vs in sorted(by_binade.items()):
        vs.sort()
        if len(vs) >= 2:
            steps = {vs[i + 1] - vs[i] for i in range(len(vs) - 1)}
            if len(steps) != 1:
                return False, f"binade {e}: {len(steps)} distinct steps {sorted(steps)}"
            (s,) = steps
            if not is_power_of_radix(s, radix):
                return False, f"binade {e}: step {s} is not a power of radix {radix}"
            if s != expect_step:
                return False, f"binade {e}: step {s}, expected {expect_step}"
        # phase: the progression, extended down, must hit an exact multiple of the step
        # offset by PHASE. With phase 0 that means every value is a multiple of the step.
        for v in vs:
            if (v / expect_step) % 1 != 0:
                return False, f"binade {e}: {v} is not on the step lattice"
    return True, f"{len(by_binade)} non-empty binades"


# --- the discriminating pool for Q2 -------------------------------------------------
# Each entry is (name, value set, expected step, expected verdict at radix two).
# The rejects exist so the predicate can fail. Without them Q2 is asking whether
# arithmetic works.

def _hub(w, f):
    step = Q(1, R**f)
    return {step * (k + Q(1, 2)) for k in range(R**w)}


def _binary_float(p, emin, emax):
    out = {Q(0)}
    for e in range(emin, emax + 1):
        st = Q(2) ** (e - p + 1)
        for k in range(2 ** (p - 1)):
            out.add(Q(2) ** e + st * k)
    return out


def _decimal(digits, f10):
    return {Q(k, 10**f10) for k in range(10**digits)}


def _ragged():
    """not a progression in one binade: values 1, 5/4, 3/2, 2 has step 1/4 in binade 0
    and a lone point in binade 1, which is fine; this one breaks the progression by
    dropping a middle value."""
    return {Q(0), Q(1), Q(5, 4), Q(7, 4), Q(2)}


def q2_pool():
    pool = []
    # accepts: the B-reading's own objects
    for w, f in [(4, 2), (8, 0), (1, 4), (0, 0), (6, 6)]:
        pool.append((f"fixed W={w} F={f}", b_value_set(w, f), Q(1, R**f), True))
    # rejects: inside 08's concept but not at this step, or not one progression at
    # radix two at all
    pool.append(("HUB W=4 F=2 (phase half a step)", _hub(4, 2), Q(1, 4), False))
    pool.append(("float p=4 e=-3..3", _binary_float(4, -3, 3), Q(1, 64), False))
    pool.append(("decimal 2 digits F10=1", _decimal(2, 1), Q(1, 10), False))
    pool.append(("ragged progression", _ragged(), Q(1, 4), False))
    # a reject that differs from an accept only in the expected step, which catches a
    # predicate that ignores its step argument
    pool.append(("fixed W=4 F=2, wrong step asserted", b_value_set(4, 2), Q(1, 8), False))
    return pool


def main():
    WMAX, FMAX = 10, 10
    q1_ok = q1_total = 0
    q1_fail = []

    for w in range(0, WMAX + 1):
        for f in range(0, FMAX + 1):
            bset = b_value_set(w, f)
            reach_hi = Q(R) ** (w - f)  # 2^{W-F} = 2^I, exclusive top of the reach
            aset = a_value_set(R, ADJ, BIAS, PHASE, -f, reach_hi)
            q1_total += 1
            if aset == bset:
                q1_ok += 1
            else:
                q1_fail.append((w, f, len(aset), len(bset)))

    print("s1: grid-and-reach agreement between vocabulary A and vocabulary B")
    print(f"box: W in [0,{WMAX}], F in [0,{FMAX}]  ->  {q1_total} numerals")
    print()
    print(f"Q1 set equality      : {q1_ok}/{q1_total}")
    for row in q1_fail[:10]:
        print(f"    MISMATCH W={row[0]} F={row[1]} |A|={row[2]} |B|={row[3]}")
    print()

    print("Q2 08's predicate over a discriminating pool (accepts AND rejects):")
    q2_ok = q2_total = 0
    for name, values, step, want in q2_pool():
        got, detail = check_binade_structure(values, R, step)
        q2_total += 1
        agree = got == want
        if agree:
            q2_ok += 1
        verdict = "accept" if got else "reject"
        wanted = "accept" if want else "reject"
        mark = "ok " if agree else "XXX"
        print(f"    {mark} {name:<36} got {verdict:<7} want {wanted:<7} {detail if not got else ''}")
    print(f"    -> {q2_ok}/{q2_total} verdicts correct")
    print()

    print("I = W - F as the binade the reach stops at (exclusive):")
    for w, f in [(4, 0), (4, 2), (4, 4), (1, 1), (1, 8), (1, 32), (0, 0)]:
        bset = b_value_set(w, f) if w <= 12 else None
        hi = Q(2) ** (w - f)
        top = max(bset) if bset else None
        print(
            f"  W={w:<3} F={f:<3} I={w-f:<4} reach=[0,2^{w-f}) = [0,{hi})"
            + (f"   max value {top}" if top is not None else "")
        )
    print()
    print("negative I is the reach lying wholly below one, which needs no signed storage:")
    for w, f in [(1, 1), (1, 2), (1, 4), (1, 8), (1, 32)]:
        print(f"  W={w} F={f}: I={w-f}, values are exactly {{0, 2^-{f}}}, one bit")


if __name__ == "__main__":
    main()
