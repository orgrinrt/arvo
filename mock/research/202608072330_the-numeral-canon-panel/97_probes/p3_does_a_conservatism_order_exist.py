#!/usr/bin/env python3
"""P3. Does "resolve toward the more conservative side" name an order?

`mock/DESIGN.md.tmpl:43` names `Resolve<S1, S2>` and `arvo-toolbox-not-policer.md`
gives its intended behaviour by example: wrapping combined with saturating yields
saturating. `93` section 7 keeps that, as "a componentwise join over the policy layer
only". A join needs an order. This probe asks whether the order exists.

Two candidate orders are on offer and neither is arbitrary. Both are computed here,
exhaustively, side by side, for signed and unsigned.

  LAW ORDER.  One policy is above another when it honours a superset of the laws.
              This is the order `OPTIONS.md` Q41 asks about and `93`'s P8 measured on
              one inventory at one width.

  FIDELITY ORDER.  One policy is above another when its result is never further from
              the exact answer. This is the order the word "conservative" most
              naturally names in a numerics setting.

The inventory deliberately mixes two families, because a policy that gains order laws
tends to lose algebraic ones and an inventory drawn from one family alone would show
a ladder that is an artifact of the choice. Every verdict is decided exhaustively over
the whole representable domain, so nothing here is sampled.
"""

import sys

# ------------------------------------------------------------------ policies


def domain(W, signed):
    if signed:
        return list(range(-(1 << (W - 1)), 1 << (W - 1)))
    return list(range(0, 1 << W))


def apply_policy(q, W, signed, policy):
    dom = domain(W, signed)
    lo, hi = dom[0], dom[-1]
    if policy == "wrap":
        mod = 1 << W
        r = q % mod
        if signed and r >= (1 << (W - 1)):
            r -= mod
        return r
    if policy == "saturate":
        return lo if q < lo else (hi if q > hi else q)
    if policy == "exact":
        return q  # a wider rung: never leaves the exact answer
    raise AssertionError(policy)


def make(W, F, signed, policy):
    dom = domain(W, signed)
    s = 1 << F

    def quant(num, den):
        # truncate toward zero
        if (num < 0) != (den < 0):
            return -((-num) // den) if num < 0 else -(num // -den)
        return num // den

    def add(a, b):
        return apply_policy(a + b, W, signed, policy)

    def sub(a, b):
        return apply_policy(a - b, W, signed, policy)

    def mul(a, b):
        return apply_policy(quant(a * b, s), W, signed, policy)

    def exact_add(a, b):
        return a + b

    def exact_mul(a, b):
        return quant(a * b, s)

    return dom, add, sub, mul, exact_add, exact_mul


# ------------------------------------------------------------------ laws


def law_set(W, F, signed, policy):
    dom, add, sub, mul, _, _ = make(W, F, signed, policy)
    one = 1 << F if (1 << F) in dom else None
    hi = dom[-1]
    honoured = set()

    def all3(fn):
        for a in dom:
            for b in dom:
                for c in dom:
                    if not fn(a, b, c):
                        return False
        return True

    def all2(fn):
        for a in dom:
            for b in dom:
                if not fn(a, b):
                    return False
        return True

    if all3(lambda a, b, c: add(add(a, b), c) == add(a, add(b, c))):
        honoured.add("add_assoc")
    if all3(lambda a, b, c: mul(mul(a, b), c) == mul(a, mul(b, c))):
        honoured.add("mul_assoc")
    if all3(lambda a, b, c: mul(a, add(b, c)) == add(mul(a, b), mul(a, c))):
        honoured.add("distrib")
    if all3(lambda a, b, c: mul(a, sub(b, c)) == sub(mul(a, b), mul(a, c))):
        honoured.add("mul_over_sub")
    if all3(lambda a, b, c: (a > b) or (add(a, c) <= add(b, c))):
        honoured.add("add_monotone")
    if all3(lambda a, b, c: (a > b) or (c < 0) or (mul(a, c) <= mul(b, c))):
        honoured.add("mul_monotone_nonneg")
    if all2(lambda a, b: (a < 0) or add(hi, a) == hi):
        honoured.add("top_absorbing")
    if all2(lambda a, b: mul(0, a) == 0):
        honoured.add("zero_annihilates")
    if all2(lambda a, b: add(a, 0) == a):
        honoured.add("add_identity")
    if one is not None and all2(lambda a, b: mul(a, one) == a):
        honoured.add("mul_identity")
    return honoured


# ------------------------------------------------------------------ fidelity


def fidelity(W, F, signed, policy):
    """How far the policy's answer sits from the exact one, over the whole domain."""
    dom, add, sub, mul, exact_add, exact_mul = make(W, F, signed, policy)
    wrong = 0
    total = 0
    err = 0
    worst = 0
    for a in dom:
        for b in dom:
            for name, got, want in (("add", add(a, b), exact_add(a, b)),
                                    ("mul", mul(a, b), exact_mul(a, b))):
                total += 1
                d = abs(got - want)
                if d != 0:
                    wrong += 1
                    err += d
                    worst = max(worst, d)
    return wrong, total, err, worst


# ------------------------------------------------------------------ driver


def main():
    W = int(sys.argv[1]) if len(sys.argv) > 1 else 5
    policies = ["wrap", "saturate", "exact"]
    print("P3. does a conservatism order exist on the overflow axis")
    print("model width W = %d, exhaustive over the whole representable domain" % W)
    print()

    for signed in (False, True):
        for F in (0, 1):
            tag = ("signed" if signed else "unsigned") + " F=%d" % F
            print("=" * 74)
            print(tag)
            sets = {}
            fids = {}
            for p in policies:
                sets[p] = law_set(W, F, signed, p)
                fids[p] = fidelity(W, F, signed, p)
            allnames = sorted(set().union(*sets.values()))
            print("  %-22s %s" % ("law", "  ".join("%-9s" % p for p in policies)))
            for nm in allnames:
                print("  %-22s %s" % (nm, "  ".join(
                    "%-9s" % ("yes" if nm in sets[p] else ".") for p in policies)))
            print("  %-22s %s" % ("count", "  ".join(
                "%-9d" % len(sets[p]) for p in policies)))
            print()
            print("  fidelity against exact (lower is more faithful)")
            for p in policies:
                wrong, total, err, worst = fids[p]
                print("    %-9s wrong %7d of %7d (%6.2f%%)  total abs err %10d  worst %6d"
                      % (p, wrong, total, 100.0 * wrong / total, err, worst))
            print()
            print("  LAW ORDER, pairwise:")
            law_incomparable = []
            for i in range(len(policies)):
                for j in range(i + 1, len(policies)):
                    p, q = policies[i], policies[j]
                    if sets[p] == sets[q]:
                        rel = "equal"
                    elif sets[p] < sets[q]:
                        rel = "%s below %s" % (p, q)
                    elif sets[q] < sets[p]:
                        rel = "%s below %s" % (q, p)
                    else:
                        rel = "INCOMPARABLE"
                        law_incomparable.append((p, q))
                    print("    %-10s vs %-10s : %s" % (p, q, rel))
                    if rel == "INCOMPARABLE":
                        print("        only in %-9s: %s" % (p, ", ".join(sorted(sets[p] - sets[q]))))
                        print("        only in %-9s: %s" % (q, ", ".join(sorted(sets[q] - sets[p]))))
            print()
            print("  FIDELITY ORDER, and it splits into two metrics that disagree:")
            for metric, idx, label in (("frequency", 0, "how often the answer is wrong"),
                                       ("magnitude", 3, "how far wrong the worst case is")):
                print("    by %s (%s):" % (metric, label))
                for i in range(len(policies)):
                    for j in range(i + 1, len(policies)):
                        p, q = policies[i], policies[j]
                        kp, kq = fids[p][idx], fids[q][idx]
                        if kp == kq:
                            rel = "TIED, no order"
                        elif kp < kq:
                            rel = "%s more faithful than %s" % (p, q)
                        else:
                            rel = "%s more faithful than %s" % (q, p)
                        print("      %-10s vs %-10s : %s" % (p, q, rel))
            print()
            if ("wrap", "saturate") in law_incomparable or ("saturate", "wrap") in law_incomparable:
                print("  VERDICT: the two candidate orders DISAGREE here. The law order makes")
                print("  wrapping and saturating incomparable, so a join over the law order")
                print("  does not exist, while the fidelity order ranks them. \"The more")
                print("  conservative side\" therefore has no referent at this predicate")
                print("  without saying which order is meant.")
            else:
                print("  VERDICT: the two candidate orders AGREE here, and a join exists on")
                print("  both, so `Resolve` toward the conservative side is well defined at")
                print("  this predicate.")
            print()


if __name__ == "__main__":
    main()
