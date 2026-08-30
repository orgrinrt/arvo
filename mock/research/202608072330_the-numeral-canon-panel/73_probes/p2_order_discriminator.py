#!/usr/bin/env python3
"""p2. Can order, or magnitude, separate the kernel from what the narrow reading
wants to exclude?

HYPOTHESIS, written before the run.

    `OPTIONS.md` Q21 (`OPTIONS.md:1611-1617`) states the narrow reading of
    "number system" as "ordered value sets with a notion of magnitude", with a
    sibling term for the unordered case; the wording is `66:286-288`. `65:65-66`
    takes the broad reading and names the two-element Boolean algebra and
    GF(2)^n as systems "not about magnitude at all".

    So the narrow reading's discriminator is an order compatible with the
    operations. I predict it does not cut where the narrow reading needs it to:
    Z/2^n under wrapping addition, which is kernel item K1 and is demanded by
    I3 ("Warm behaves as a native Rust primitive would"), has no such order
    either, for the same structural reason GF(2)^n has none. Both are finite
    groups, and a finite group with more than one element admits no
    translation-invariant total order.

    If that holds, every order-based test puts a kernel item and the excluded
    candidate on the same side, and the discriminator is empty.

WHAT IS MEASURED.

  1. For each operation, at widths 2 and 3, EVERY total order on the carrier is
     enumerated and tested for compatibility (a <= b implies a.c <= b.c and
     c.a <= c.b for all c). 4! = 24 orders at w = 2, 8! = 40320 at w = 3. This
     is exhaustive over orders, not a sample.
  2. At width 4 the natural (unsigned representative) order only, exhaustively
     over the 4096 triples, because 16! is not enumerable.
  3. For the group operations, the structural witness is printed: the order of
     each non-identity element, which is what forbids a translation-invariant
     total order at EVERY width rather than only at the widths enumerated.

Nothing here is a bench. No magnitude is claimed. Counts only.
"""

from itertools import permutations


def ops_for(w):
    n = 1 << w
    top = n - 1
    return [
        ("wrap add  (Z/2^n)", lambda a, b: (a + b) % n, True),
        ("xor       (GF(2)^n)", lambda a, b: a ^ b, True),
        ("and       (lattice meet)", lambda a, b: a & b, False),
        ("or        (lattice join)", lambda a, b: a | b, False),
        ("min       (tropical add)", lambda a, b: min(a, b), False),
        ("sat add   (bounded chain)", lambda a, b: min(a + b, top), False),
    ]


def compatible(order, op, n):
    """order is a tuple listing the carrier from least to greatest."""
    rank = [0] * n
    for i, v in enumerate(order):
        rank[v] = i
    for a in range(n):
        for b in range(n):
            if rank[a] > rank[b]:
                continue
            for c in range(n):
                if rank[op(a, c)] > rank[op(b, c)]:
                    return False
                if rank[op(c, a)] > rank[op(c, b)]:
                    return False
    return True


def count_compatible_orders(op, n):
    good = 0
    first = None
    for order in permutations(range(n)):
        if compatible(order, op, n):
            good += 1
            if first is None:
                first = order
    return good, first


def natural_order_ok(op, n):
    bad = 0
    for a in range(n):
        for b in range(a, n):
            for c in range(n):
                if op(a, c) > op(b, c) or op(c, a) > op(c, b):
                    bad += 1
    return bad


def element_orders(op, n, identity):
    """for a group operation, the order of each element: the structural witness."""
    out = {}
    for a in range(n):
        if a == identity:
            continue
        x, k = a, 1
        while x != identity and k <= n:
            x = op(x, a)
            k += 1
        out[a] = k if x == identity else None
    return out


def main():
    print("=" * 78)
    print("p2. every total order, enumerated, against each candidate operation")
    print("=" * 78)
    print()

    results = {}
    for w in (2, 3):
        n = 1 << w
        print("--- width %d, carrier of %d elements, all %d total orders enumerated ---"
              % (w, n, __import__("math").factorial(n)))
        hdr = "%-28s %-18s %s" % ("operation", "compatible orders", "one witness")
        print(hdr)
        print("-" * len(hdr))
        for name, op, is_group in ops_for(w):
            good, first = count_compatible_orders(op, n)
            results[(w, name)] = good
            print("%-28s %-18s %s" % (name, "%d of %d" % (good, __import__("math").factorial(n)),
                                      "" if first is None else str(first)))
        print()

    w = 4
    n = 1 << w
    print("--- width %d, the natural (unsigned representative) order only ---" % w)
    hdr = "%-28s %s" % ("operation", "natural-order monotonicity failures")
    print(hdr)
    print("-" * len(hdr))
    nat = {}
    for name, op, _g in ops_for(w):
        bad = natural_order_ok(op, n)
        nat[name] = bad
        print("%-28s %d" % (name, bad))
    print()

    print("--- the structural witness for the two group operations ---")
    for w in (2, 3, 4):
        n = 1 << w
        for name, op, is_group in ops_for(w):
            if not is_group:
                continue
            orders = element_orders(op, n, 0)
            mx = max(v for v in orders.values() if v is not None)
            allfinite = all(v is not None for v in orders.values())
            print("w=%d %-28s every non-identity element has finite order: %s (max %d)"
                  % (w, name, allfinite, mx))
    print()
    print("A translation-invariant total order on a group forces a > e to give")
    print("a^k > e for every k. Finite order makes some a^k = e, a contradiction.")
    print("So the two rows above have no compatible total order at ANY width, and")
    print("the enumerations at widths 2 and 3 are that argument checked, not a sample.")
    print()

    print("-" * 78)
    print("ASSERTIONS")
    print("-" * 78)

    for w in (2, 3):
        assert results[(w, "wrap add  (Z/2^n)")] == 0, "wrap admitted an order at w=%d" % w
        assert results[(w, "xor       (GF(2)^n)")] == 0, "xor admitted an order at w=%d" % w
    print("  ok  wrap add and xor admit ZERO compatible total orders at widths 2 and 3")

    for w in (2, 3):
        assert results[(w, "min       (tropical add)")] > 0, "min admitted none at w=%d" % w
        assert results[(w, "sat add   (bounded chain)")] > 0, "sat add admitted none at w=%d" % w
    print("  ok  min and saturating add admit at least one, at widths 2 and 3")

    assert nat["wrap add  (Z/2^n)"] > 0 and nat["xor       (GF(2)^n)"] > 0
    assert nat["min       (tropical add)"] == 0 and nat["sat add   (bounded chain)"] == 0
    print("  ok  at width 4 the natural order agrees with the same split")

    assert results[(2, "wrap add  (Z/2^n)")] == results[(2, "xor       (GF(2)^n)")]
    assert results[(3, "wrap add  (Z/2^n)")] == results[(3, "xor       (GF(2)^n)")]
    assert nat["wrap add  (Z/2^n)"] > 0 and nat["xor       (GF(2)^n)"] > 0
    print("  ok  the kernel item (wrap, K1, demanded by I3) and the candidate the")
    print("      narrow reading excludes (xor, GF(2)^n) receive the SAME verdict")
    print("      from every order-based test run here")

    assert results[(3, "wrap add  (Z/2^n)")] != results[(3, "sat add   (bounded chain)")]
    print("  ok  and the same test SEPARATES two members of one kernel item:")
    print("      wrapping and saturating over the same window land on opposite sides")

    print()
    print("READING. An order-or-magnitude discriminator groups a kernel item with")
    print("the thing it was introduced to exclude, and splits a kernel item down")
    print("the middle. It is not a boundary of the concept; it is a property some")
    print("reductions have and others do not, which is `63` C4's adaptation-law")
    print("family under a different name.")


if __name__ == "__main__":
    main()
