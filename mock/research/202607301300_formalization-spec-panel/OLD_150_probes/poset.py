"""Exhaustive construction of the inclusion order on numeral value sets.

The point of this probe is NOT to count pairs. It is to make the model's
assumptions explicit switches, build the whole finite poset under each
setting of the switches, and ask directly whether binary meets and binary
joins exist. A count is only reported to locate the smallest witness.

A numeral is identified with its value set (a finite set of exact rationals).
Two shapes with the same value set are one element of the order.

Truncation hazard, handled explicitly: the universe is finite, so a join can
appear to be missing merely because the true least upper bound was not
enumerated. Every reported join failure is classified as either
  - REAL: two or more minimal upper bounds exist inside the universe and are
    pairwise incomparable, so no least one exists even if the universe grew, or
  - TRUNCATION: the upper-bound set is empty or entirely at the enumeration
    boundary, which says nothing.
Meets do not have the dual hazard as long as the universe is downward
adequate in shape space, which it is by construction here.
"""

from fractions import Fraction as Q
from itertools import combinations, product


# ---------------------------------------------------------------- numerals

def fixed(i_bits, f_bits, signed, offset=Q(0)):
    """Radix-2 fixed point. i_bits integer digits, f_bits fraction digits.

    Value set { (k + offset) * 2^-f : k in the container's integer range }.
    offset is the affine placement (a zero point). offset = 0 is the anchored
    case, in which every numeral contains the value zero.
    """
    w = i_bits + f_bits
    if w <= 0:
        return None
    step = Q(1, 2 ** f_bits) if f_bits >= 0 else Q(2 ** (-f_bits))
    if signed:
        lo, hi = -(2 ** (w - 1)), 2 ** (w - 1) - 1
    else:
        lo, hi = 0, 2 ** w - 1
    return frozenset((Q(k) + offset) * step for k in range(lo, hi + 1))


def flt(p, emin, emax, signed=True, subnormals=True):
    """Radix-2 binary float. p significand digits, exponent range [emin, emax].

    Value set is zero, the normals, and optionally the subnormals. This is a
    tapered grid: the spacing changes across binades, which is the property
    that matters for the order.
    """
    vals = {Q(0)}
    for e in range(emin, emax + 1):
        ulp = Q(2) ** (e - p + 1)
        for m in range(2 ** (p - 1), 2 ** p):
            vals.add(Q(m) * ulp)
    if subnormals:
        ulp = Q(2) ** (emin - p + 1)
        for m in range(1, 2 ** (p - 1)):
            vals.add(Q(m) * ulp)
    if signed:
        vals |= {-v for v in vals}
    return frozenset(vals)


# ------------------------------------------------------------------ universe

def universe_fixed_anchored(max_i, max_f, signs=(False, True)):
    u = {}
    for signed in signs:
        for i in range(0, max_i + 1):
            for f in range(0, max_f + 1):
                v = fixed(i, f, signed)
                if v:
                    u.setdefault(v, f"{'I' if signed else 'U'}{i}.{f}")
    return u


def universe_fixed_biased(max_i, max_f, offsets, signs=(False,)):
    u = {}
    for signed in signs:
        for i in range(0, max_i + 1):
            for f in range(0, max_f + 1):
                for o in offsets:
                    v = fixed(i, f, signed, o)
                    if v:
                        u.setdefault(v, f"{'I' if signed else 'U'}{i}.{f}@{o}")
    return u


def universe_mixed(max_i, max_f, p_range, e_range, signed=True):
    u = dict(universe_fixed_anchored(max_i, max_f,
                                     signs=(signed,) if signed else (False,)))
    for p in p_range:
        for emin, emax in e_range:
            v = flt(p, emin, emax, signed=signed)
            u.setdefault(v, f"F(p={p},e=[{emin},{emax}])")
    return u


# -------------------------------------------------------------------- order

def analyse(u, label, sample_limit=None, verbose_examples=3):
    """Build the order and test binary meets and joins over every pair."""
    els = sorted(u.keys(), key=lambda s: (len(s), sorted(s)))
    names = {e: u[e] for e in els}
    n = len(els)

    # maximal shapes in the universe, used to detect truncation on joins
    maximal = [e for e in els if not any(e < o for o in els)]

    stats = dict(pairs=0, comparable=0,
                 meet_ok=0, meet_exact=0, meet_fail=0,
                 join_ok=0, join_exact=0, join_fail_real=0, join_fail_trunc=0)
    meet_fails, join_fails = [], []

    for a, b in combinations(els, 2):
        stats['pairs'] += 1
        if a <= b or b <= a:
            stats['comparable'] += 1

        inter = a & b
        union = a | b

        lows = [c for c in els if c <= inter]
        if lows:
            tops = [c for c in lows if not any(c < d for d in lows)]
            if len(tops) == 1:
                stats['meet_ok'] += 1
                if tops[0] == inter:
                    stats['meet_exact'] += 1
            else:
                stats['meet_fail'] += 1
                meet_fails.append((names[a], names[b], [names[t] for t in tops]))
        else:
            stats['meet_fail'] += 1
            meet_fails.append((names[a], names[b], []))

        ups = [c for c in els if union <= c]
        if ups:
            bots = [c for c in ups if not any(d < c for d in ups)]
            if len(bots) == 1:
                stats['join_ok'] += 1
                if bots[0] == union:
                    stats['join_exact'] += 1
            else:
                # real only if at least two minimal upper bounds are not
                # themselves maximal elements of the truncated universe
                inner = [t for t in bots if t not in maximal]
                if len(inner) >= 2 or (len(inner) >= 1 and len(bots) >= 2):
                    stats['join_fail_real'] += 1
                    join_fails.append((names[a], names[b],
                                       [names[t] for t in bots], 'REAL'))
                else:
                    stats['join_fail_trunc'] += 1
                    join_fails.append((names[a], names[b],
                                       [names[t] for t in bots], 'TRUNC?'))
        else:
            stats['join_fail_trunc'] += 1
            join_fails.append((names[a], names[b], [], 'TRUNC'))

    print(f"\n=== {label} ===")
    print(f"elements (distinct value sets): {n}   pairs: {stats['pairs']}"
          f"   comparable: {stats['comparable']}")
    print(f"  meet: unique {stats['meet_ok']} (exact = intersection "
          f"{stats['meet_exact']}), FAILS {stats['meet_fail']}")
    print(f"  join: unique {stats['join_ok']} (exact = union "
          f"{stats['join_exact']}), FAILS real {stats['join_fail_real']}, "
          f"boundary/unknown {stats['join_fail_trunc']}")
    verdict = []
    verdict.append("meet-semilattice" if stats['meet_fail'] == 0 else "NOT a meet-semilattice")
    verdict.append("join-semilattice" if stats['join_fail_real'] == 0
                   and stats['join_fail_trunc'] == 0 else
                   ("join-semilattice modulo truncation"
                    if stats['join_fail_real'] == 0 else "NOT a join-semilattice"))
    print(f"  VERDICT: {' and '.join(verdict)}")

    for kind, fails in (("meet", meet_fails), ("join", join_fails)):
        for f in fails[:verbose_examples]:
            print(f"    {kind} failure: {f[0]} vs {f[1]} -> extremal {f[2]}"
                  + (f"  [{f[3]}]" if len(f) > 3 else ""))
    return stats


if __name__ == "__main__":
    # A. anchored fixed point, one sign. The model 145 describes.
    analyse(universe_fixed_anchored(4, 4, signs=(False,)),
            "A. anchored fixed point, unsigned, I,F <= 4")

    # B. anchored fixed point, both signs in one order.
    analyse(universe_fixed_anchored(3, 3, signs=(False, True)),
            "B. anchored fixed point, both signs, I,F <= 3")

    # C. fixed point with a free affine offset (a zero point).
    offs = [Q(0), Q(1, 2), Q(1, 4), Q(1), Q(3, 2)]
    analyse(universe_fixed_biased(2, 2, offs, signs=(False,)),
            "C. fixed point with free offset, unsigned, I,F <= 2")

    # D. floats alone.
    u = {}
    for p in (2, 3, 4):
        for emin, emax in ((0, 1), (0, 2), (-1, 1), (-1, 2), (0, 3)):
            u.setdefault(flt(p, emin, emax, signed=False), f"F(p={p},e=[{emin},{emax}])")
    analyse(u, "D. floats alone, unsigned")

    # E. fixed and float in one order. The whole numeral space.
    analyse(universe_mixed(3, 3, (2, 3), ((0, 1), (0, 2), (-1, 1)), signed=False),
            "E. fixed and float in one order, unsigned")
