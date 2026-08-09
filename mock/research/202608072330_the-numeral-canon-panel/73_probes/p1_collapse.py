#!/usr/bin/env python3
"""p1. Does a system's own law verdicts survive it renaming its ambient domain?

HYPOTHESIS, written before the run.

    `71` section 7 proposes a membership test for the number-system concept: a
    candidate is a member when it can expose its first three telescope
    coordinates, that is an ambient domain D, a representable set Q that is a
    constant of the type, and a reduction rho onto Q "whose two law verdicts are
    decidable" (`71:466-469`).

    Every system has a second declaration of itself: take D' = (Q, f) where f is
    its own induced operation, and rho' = the identity on Q. That term satisfies
    the exposure list verbatim. It names an ambient domain, it names a
    representable set which is a constant of the type, and it names a total
    reduction onto it.

    I predict: (a) the collapsed term computes the identical function, which is
    definitional and is stated as such rather than reported as a discovery;
    (b) the collapsed term reports ZERO failures on BOTH law families for every
    declaration tested, including declarations that fail them under their honest
    form; and (c) the property that actually predicts an algorithm's answer,
    associativity of the induced operation, is unchanged by the collapse.

    If (b) and (c) both hold, then reading the law verdicts off the pair that a
    system exposes is not a membership test with content, because the verdicts
    are a fact about which ambient domain was named rather than about the
    arithmetic.

WHAT IS MEASURED, per declaration, all exhaustive at the 4-bit model width:

    AMB-ASSOC   is the ambient operation associative on the reachable set
    COH         failures of rho(x op_D y) == rho(x) f rho(y), over reachable x,y
    MONO        failures of monotonicity of rho on the ambient reachable set
    IND-ASSOC   failures of associativity of the induced f over Q^3
    SAME-FN     does the collapsed term compute the same f, over Q^2

Nothing here is a bench. No magnitude is claimed. Counts only.
"""

W = 4
N = 1 << W


def unsigned_window():
    return list(range(N))


def signed_window():
    return list(range(-(N // 2), N // 2))


def wrap(lo, hi):
    span = hi - lo + 1
    return lambda e: (e - lo) % span + lo


def saturate(lo, hi):
    return lambda e: lo if e < lo else (hi if e > hi else e)


def identity_reduce(_lo, _hi):
    return lambda e: e


# A declaration is (name, Q, ambient combine on ints, rho, ambient-reachable set).
# The ambient reachable set is every exact result of combining two members of Q,
# which is what the coherence and monotonicity questions are actually asked over
# (`63` C5: quantified over the values the format can hold).


def decl(name, Q, amb_op, rho_factory):
    lo, hi = min(Q), max(Q)
    rho = rho_factory(lo, hi)
    reach = sorted({amb_op(a, b) for a in Q for b in Q} | set(Q))
    return dict(name=name, Q=Q, amb=amb_op, rho=rho, reach=reach)


def induced(d):
    return lambda a, b: d["rho"](d["amb"](a, b))


def amb_assoc_failures(d):
    """associativity of the AMBIENT operation, over the ambient reachable set."""
    bad = 0
    R = d["reach"]
    for a in R:
        for b in R:
            for c in R:
                if d["amb"](d["amb"](a, b), c) != d["amb"](a, d["amb"](b, c)):
                    bad += 1
    return bad, len(R) ** 3


def coherence_failures(d):
    """rho a homomorphism from the ambient onto the induced operation."""
    f = induced(d)
    bad = 0
    R = d["reach"]
    for x in R:
        for y in R:
            if d["rho"](d["amb"](x, y)) != f(d["rho"](x), d["rho"](y)):
                bad += 1
    return bad, len(R) ** 2


def monotone_failures(d):
    """x <= y implies rho(x) <= rho(y), over the ambient reachable set."""
    bad = 0
    R = d["reach"]
    for x in R:
        for y in R:
            if x <= y and d["rho"](x) > d["rho"](y):
                bad += 1
    return bad, len(R) ** 2


def induced_assoc_failures(d):
    f = induced(d)
    bad = 0
    for a in d["Q"]:
        for b in d["Q"]:
            for c in d["Q"]:
                if f(f(a, b), c) != f(a, f(b, c)):
                    bad += 1
    return bad, len(d["Q"]) ** 3


def collapse(d):
    """D' = (Q, f), rho' = identity. The exposure list of `71` X3, satisfied."""
    f = induced(d)
    return dict(
        name=d["name"] + " [collapsed]",
        Q=d["Q"],
        amb=f,
        rho=lambda e: e,
        reach=sorted(set(d["Q"])),
    )


def main():
    U = unsigned_window()
    S = signed_window()
    add = lambda a, b: a + b
    mul = lambda a, b: a * b
    xor = lambda a, b: a ^ b

    decls = [
        decl("u4 wrap add", U, add, wrap),
        decl("u4 saturate add", U, add, saturate),
        decl("i4 saturate add", S, add, saturate),
        decl("i4 wrap add", S, add, wrap),
        decl("u4 saturate mul", U, mul, saturate),
        decl("gf(2)^4 xor", U, xor, identity_reduce),
    ]

    print("=" * 78)
    print("p1. the collapsed declaration, measured against the honest one")
    print("W = %d, Q sizes %s" % (W, sorted({len(d["Q"]) for d in decls})))
    print("=" * 78)
    print()
    hdr = "%-22s %-10s %-12s %-12s %-12s" % (
        "declaration",
        "AMB-ASSOC",
        "COH fails",
        "MONO fails",
        "IND-ASSOC",
    )
    print(hdr)
    print("-" * len(hdr))

    same_fn_all = True
    rows = []
    for d in decls:
        c = collapse(d)
        aa, aat = amb_assoc_failures(d)
        cf, cft = coherence_failures(d)
        mf, mft = monotone_failures(d)
        ia, iat = induced_assoc_failures(d)

        caa, caat = amb_assoc_failures(c)
        ccf, ccft = coherence_failures(c)
        cmf, cmft = monotone_failures(c)
        cia, ciat = induced_assoc_failures(c)

        f, g = induced(d), induced(c)
        same = all(f(a, b) == g(a, b) for a in d["Q"] for b in d["Q"])
        same_fn_all = same_fn_all and same

        print(
            "%-22s %-10s %-12s %-12s %-12s"
            % (
                d["name"],
                "%d/%d" % (aa, aat),
                "%d/%d" % (cf, cft),
                "%d/%d" % (mf, mft),
                "%d/%d" % (ia, iat),
            )
        )
        print(
            "%-22s %-10s %-12s %-12s %-12s   same fn: %s"
            % (
                "  [collapsed]",
                "%d/%d" % (caa, caat),
                "%d/%d" % (ccf, ccft),
                "%d/%d" % (cmf, cmft),
                "%d/%d" % (cia, ciat),
                same,
            )
        )
        rows.append((d["name"], aa, cf, mf, ia, caa, ccf, cmf, cia, same))

    print()
    print("-" * 78)
    print("ASSERTIONS (the probe crashes rather than inviting the reader to eyeball)")
    print("-" * 78)

    # 1. the collapse never changes the computed function.
    assert same_fn_all, "a collapsed term computed a different function"
    print("  ok  every collapsed term computes the identical induced operation")

    # 2. the collapse reports zero failures on both law families, always.
    for (nm, aa, cf, mf, ia, caa, ccf, cmf, cia, _s) in rows:
        assert ccf == 0, "collapsed coherence nonzero for %s" % nm
        assert cmf == 0, "collapsed monotonicity nonzero for %s" % nm
    print("  ok  every collapsed term reports 0 coherence and 0 monotonicity failures")

    # 3. at least one declaration fails a law family honestly. otherwise the
    #    comparison is vacuous and the probe is measuring nothing.
    assert any(cf > 0 for (_n, _a, cf, _m, _i, _x, _y, _z, _w, _s) in rows), (
        "no declaration failed coherence honestly; the contrast is vacuous"
    )
    assert any(mf > 0 for (_n, _a, _c, mf, _i, _x, _y, _z, _w, _s) in rows), (
        "no declaration failed monotonicity honestly; the contrast is vacuous"
    )
    print("  ok  at least one honest declaration fails each law family")

    # 4. the induced operation's associativity is unchanged by the collapse.
    for (nm, _aa, _cf, _mf, ia, _caa, _ccf, _cmf, cia, _s) in rows:
        assert ia == cia, "induced associativity moved under collapse for %s" % nm
    print("  ok  induced associativity is identical under both declarations")

    # 5. the collapse destroys the ambient half wherever the induced algebra is
    #    not associative: the antecedent that made a coherence verdict useful.
    broken = [
        nm
        for (nm, aa, _cf, _mf, ia, caa, _c2, _m2, _i2, _s) in rows
        if aa == 0 and ia > 0 and caa > 0
    ]
    assert broken, "no declaration exhibits the ambient-half collapse"
    print(
        "  ok  ambient associativity holds honestly and fails collapsed for: %s"
        % ", ".join(broken)
    )

    print()
    print("READING. A test that asks a candidate to name (D, Q, rho) and reads the")
    print("two law verdicts off what it named cannot distinguish an honest signed")
    print("saturating numeral from the same numeral naming its own computed")
    print("algebra as its ambient domain. The second reports both families clean.")
    print("The verdicts are informative only jointly with the ambient domain's own")
    print("laws, and the collapse satisfies the second conjunct by voiding the first.")


if __name__ == "__main__":
    main()
