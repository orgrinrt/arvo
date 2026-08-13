#!/usr/bin/env python3
"""P2. One structural property predicts every law verdict, so the table is not needed.

The panel has been measuring law verdicts one law at a time, per policy, per fraction
width, per signedness, per arity: `93`'s P2/P2b/P2c/P7/P8 and `94`'s probe C are all
that shape. This probe tests whether a single property decides all of them at once.

The property. Write pi for the realisation map that takes an exact value to the
nearest representable one under the declared rounding and boundary policy. If pi
respects an operation, meaning applying the operation to already-realised operands
and realising once gives the same answer as realising the intermediate too, then the
representable set with the induced operations is a quotient of the exact structure.
Quotients inherit identities. So every identity true of exact arithmetic transfers,
at every arity, for free.

That is standard universal algebra and it is not new. What is new here, for this
panel, is the claim that it accounts for ALL of the measured verdicts, including the
ones two cold derivations reported as independent facts needing independent sweeps.

Prediction rule under test, stated before running:

    a law holds in the representable set
      IFF  it is an identity of exact arithmetic
      AND  pi respects every ordered nesting of operations the law contains

Two failure directions are reported separately because they mean different things.
A SOUNDNESS mismatch (predicted to hold, measured to fail) refutes the criterion.
A COMPLETENESS mismatch (predicted to fail, measured to hold) means the criterion is
conservative, which is safe for gating an arm and is still worth counting.

Two controls are in the battery on purpose. Subtraction's associativity and
commutativity are NOT identities of exact arithmetic, so the criterion predicts they
fail everywhere. `94`'s W4 cites wrapping subtraction retracting while failing to
associate as evidence that two permissions are independent; under this criterion that
is not a policy fact at all, it is subtraction not associating in the integers.

Arithmetic is exact integer arithmetic throughout. An exact value is carried scaled by
2^(3F), which is enough for the deepest expression in the battery (three factors), so
no rational arithmetic and no floating point appears anywhere.
"""

import itertools
import sys

# ---------------------------------------------------------------- realisation


def make_domain(W, signed, nonneg_window):
    if nonneg_window:
        return list(range(0, 1 << (W - 1)))
    if signed:
        return list(range(-(1 << (W - 1)), 1 << (W - 1)))
    return list(range(0, 1 << W))


def make_pi(W, F, signed, policy, rounding, nonneg_window):
    """pi maps an exact value (scaled by 2^(3F)) to a representable raw integer."""
    dom = make_domain(W, signed, nonneg_window)
    lo, hi = dom[0], dom[-1]
    shift = 1 << (2 * F)
    mod = 1 << W

    def quantise(e):
        # e is value * 2^(3F); the raw grid value is e / 2^(2F).
        if rounding == "truncate":
            # toward zero
            if e >= 0:
                return e // shift
            return -((-e) // shift)
        # nearest, ties away from zero
        if e >= 0:
            return (2 * e + shift) // (2 * shift)
        return -((2 * (-e) + shift) // (2 * shift))

    def pi(e):
        q = quantise(e)
        if policy == "wrap":
            if nonneg_window:
                # a window is a declaration, not a container; wrapping inside one
                # is not a thing the design offers, so this combination is skipped
                # by the caller rather than given a meaning here.
                raise AssertionError("wrap on a declared window is not modelled")
            r = q % mod
            if signed and r >= (1 << (W - 1)):
                r -= mod
            return r
        if policy == "saturate":
            return lo if q < lo else (hi if q > hi else q)
        raise AssertionError(policy)

    return pi, dom


def make_ops(F):
    """Exact operations on values scaled by 2^(3F)."""
    s3 = 1 << (3 * F)

    def lift(r):
        # raw r means value r / 2^F, so scaled by 2^(3F) it is r * 2^(2F)
        return r * (1 << (2 * F))

    def add(a, b):
        return a + b

    def sub(a, b):
        return a - b

    def mul(a, b):
        # a = v1 * s3, b = v2 * s3, want v1*v2*s3
        return (a * b) // s3

    return lift, {"add": add, "sub": sub, "mul": mul}


# ---------------------------------------------------------------- retraction


def retraction_verdicts(W, F, signed, policy, rounding, nonneg_window):
    """For each ordered pair (f, g): does realising the inner result change the answer?

    Tests both argument positions of the outer operation, since subtraction is not
    symmetric and the position is exactly where saturation's escape lives.
    """
    pi, dom = make_pi(W, F, signed, policy, rounding, nonneg_window)
    lift, ops = make_ops(F)
    out = {}
    lifted = {r: lift(r) for r in dom}
    for fname, f in ops.items():
        for gname, g in ops.items():
            bad = 0
            total = 0
            for a in dom:
                la = lifted[a]
                for b in dom:
                    inner = f(la, lifted[b])
                    inner_r = lift(pi(inner))
                    for c in dom:
                        lc = lifted[c]
                        total += 1
                        if pi(g(inner, lc)) != pi(g(inner_r, lc)):
                            bad += 1
                            continue
                        if pi(g(lc, inner)) != pi(g(lc, inner_r)):
                            bad += 1
            out[(fname, gname)] = (total, bad)
    return out


# ---------------------------------------------------------------- laws

# name -> (arity, is an identity of exact arithmetic, nesting pairs it contains)
LAWS = {
    "add_comm":     (2, True,  []),
    "mul_comm":     (2, True,  []),
    "add_assoc":    (3, True,  [("add", "add")]),
    "mul_assoc":    (3, True,  [("mul", "mul")]),
    "distrib":      (3, True,  [("add", "mul"), ("mul", "add")]),
    "mul_over_sub": (3, True,  [("sub", "mul"), ("mul", "sub")]),
    "sub_assoc":    (3, False, [("sub", "sub")]),
    "sub_comm":     (2, False, []),
}


def law_verdicts(W, F, signed, policy, rounding, nonneg_window):
    pi, dom = make_pi(W, F, signed, policy, rounding, nonneg_window)
    lift, ops = make_ops(F)
    add, sub, mul = ops["add"], ops["sub"], ops["mul"]
    L = {r: lift(r) for r in dom}

    def R(x):
        return lift(pi(x))

    out = {}
    for name in LAWS:
        arity, _, _ = LAWS[name]
        bad = 0
        total = 0
        if arity == 2:
            for a in dom:
                la = L[a]
                for b in dom:
                    lb = L[b]
                    total += 1
                    if name == "add_comm":
                        ok = pi(add(la, lb)) == pi(add(lb, la))
                    elif name == "mul_comm":
                        ok = pi(mul(la, lb)) == pi(mul(lb, la))
                    elif name == "sub_comm":
                        ok = pi(sub(la, lb)) == pi(sub(lb, la))
                    else:
                        raise AssertionError(name)
                    if not ok:
                        bad += 1
        else:
            for a in dom:
                la = L[a]
                for b in dom:
                    lb = L[b]
                    for c in dom:
                        lc = L[c]
                        total += 1
                        if name == "add_assoc":
                            ok = pi(add(R(add(la, lb)), lc)) == pi(add(la, R(add(lb, lc))))
                        elif name == "mul_assoc":
                            ok = pi(mul(R(mul(la, lb)), lc)) == pi(mul(la, R(mul(lb, lc))))
                        elif name == "distrib":
                            lhs = pi(mul(la, R(add(lb, lc))))
                            rhs = pi(add(R(mul(la, lb)), R(mul(la, lc))))
                            ok = lhs == rhs
                        elif name == "mul_over_sub":
                            lhs = pi(mul(la, R(sub(lb, lc))))
                            rhs = pi(sub(R(mul(la, lb)), R(mul(la, lc))))
                            ok = lhs == rhs
                        elif name == "sub_assoc":
                            ok = pi(sub(R(sub(la, lb)), lc)) == pi(sub(la, R(sub(lb, lc))))
                        else:
                            raise AssertionError(name)
                        if not ok:
                            bad += 1
        out[name] = (total, bad)
    return out


# ---------------------------------------------------------------- driver


def configs():
    for signed in (False, True):
        for policy in ("wrap", "saturate"):
            for F in (0, 1, 2):
                for rounding in ("truncate", "nearest"):
                    if F == 0 and rounding == "nearest":
                        continue  # no rounding happens at F = 0; one pass is enough
                    yield dict(signed=signed, policy=policy, F=F, rounding=rounding,
                               nonneg_window=False)
    # the predicted recovery arm: a signed type whose declared operand window does
    # not straddle zero, saturating. If one-sidedness is what makes the clamp a
    # congruence, this recovers every law that two-sided saturation loses.
    for F in (0, 1):
        for rounding in ("truncate", "nearest"):
            if F == 0 and rounding == "nearest":
                continue
            yield dict(signed=True, policy="saturate", F=F, rounding=rounding,
                       nonneg_window=True)


def main():
    W = int(sys.argv[1]) if len(sys.argv) > 1 else 5
    print("P2. does one property predict every law verdict")
    print("model width W = %d, exhaustive over every pair and every triple" % W)
    print()

    sound_mismatch = 0
    complete_mismatch = 0
    cells = 0

    for cfg in configs():
        tag = "%s %s F=%d %s%s" % (
            "signed  " if cfg["signed"] else "unsigned",
            cfg["policy"].ljust(8),
            cfg["F"],
            cfg["rounding"].ljust(8),
            " [nonneg window]" if cfg["nonneg_window"] else "",
        )
        retr = retraction_verdicts(W, **cfg)
        laws = law_verdicts(W, **cfg)

        respects = {k: (v[1] == 0) for k, v in retr.items()}
        print("=" * 78)
        print(tag)
        print("  pi respects: " + ", ".join(
            "%s>%s" % k for k in sorted(respects) if respects[k]) or "  pi respects: (nothing)")
        print("  pi breaks  : " + (", ".join(
            "%s>%s(%.2f%%)" % (k[0], k[1], 100.0 * retr[k][1] / retr[k][0])
            for k in sorted(respects) if not respects[k]) or "(nothing)"))
        print("  %-14s %-10s %-10s %s" % ("law", "predicted", "measured", "verdict"))
        for name, (arity, exact_identity, pairs) in LAWS.items():
            predicted = exact_identity and all(respects[p] for p in pairs)
            total, bad = laws[name]
            measured = (bad == 0)
            cells += 1
            if predicted and not measured:
                verdict = "SOUNDNESS MISMATCH"
                sound_mismatch += 1
            elif measured and not predicted:
                verdict = "conservative (predicted fail, holds)"
                complete_mismatch += 1
            else:
                verdict = "agree"
            print("  %-14s %-10s %-10s %s%s" % (
                name,
                "holds" if predicted else "fails",
                "holds" if measured else "fails",
                verdict,
                "" if measured else "  (%.2f%% of %d)" % (100.0 * bad / total, total),
            ))

    print()
    print("=" * 78)
    print("cells                              : %d" % cells)
    print("SOUNDNESS mismatches (refute)      : %d" % sound_mismatch)
    print("conservative mismatches (safe)     : %d" % complete_mismatch)
    print()
    if sound_mismatch == 0:
        print("Every law the criterion predicted to hold, holds, in every configuration")
        print("swept. So the law table is derivable from the retraction verdicts and")
        print("does not need to be measured law by law.")
    else:
        print("The criterion is refuted at %d cells. It is not the rule." % sound_mismatch)


if __name__ == "__main__":
    main()
