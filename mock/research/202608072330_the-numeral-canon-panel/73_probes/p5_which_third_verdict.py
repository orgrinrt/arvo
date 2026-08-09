#!/usr/bin/env python3
"""p5. Is the retraction the right third verdict, or a corollary of a better one?

This closes an item p1d left open rather than reporting it.

`63:216-218` names the adaptation laws as a pair, "monotone, distance-minimising",
and `71` X3 compresses the whole family into one verdict. p1d found that a third
clause is needed, because a reduction adapting every ambient value to zero passes
monotonicity and coherence and computes nothing, and proposed the RETRACTION:
the reduction moves no value already representable.

HYPOTHESIS, written before the run.

    Distance-minimising is the stronger clause and implies retraction, since a
    representable value's nearest representable neighbour is itself. So the
    obvious move is to use the stronger one and get the retraction for free.

    I predict that move is wrong for the same reason section 4's order
    discriminator is wrong: distance-minimising EXCLUDES wrapping, which is
    kernel item K1 and which `65:258-259` derives from op's I3. If that holds,
    then the third verdict must be the weaker clause precisely because the
    stronger one is not a membership condition at all, and the panel's habit of
    reaching for the strongest available law is what would have got it wrong.

WHAT IS MEASURED, exhaustively at the 4-bit model width over the ambient
reachable set:

    RETRACT   rho(q) == q for every q in the representable set
    DIST-MIN  rho(x) is a nearest element of the representable set to x
    MONOTONE  x <= y implies rho(x) <= rho(y)
    COHERENT  rho is a homomorphism onto the induced operation

Not a bench. Counts only.
"""

W = 4
SQ = list(range(-(1 << (W - 1)), 1 << (W - 1)))
UQ = list(range(1 << W))


def clamp(Q):
    lo, hi = min(Q), max(Q)
    return lambda e: lo if e < lo else (hi if e > hi else e)


def wrap(Q):
    lo, hi = min(Q), max(Q)
    span = hi - lo + 1
    return lambda e: (e - lo) % span + lo


def const_zero(_Q):
    return lambda _e: 0


def opposite(Q):
    lo, hi = min(Q), max(Q)
    return lambda e: hi if e < lo else (lo if e > hi else e)


def parity(Q):
    c = clamp(Q)
    return lambda e: c(e) if e % 2 == 0 else c(-e)


def ident(_Q):
    return lambda e: e


ADD = lambda a, b: a + b
XOR = lambda a, b: a ^ b


def build(name, Q, amb, rf, kernel):
    rho = rf(Q)
    reach = sorted({amb(a, b) for a in Q for b in Q} | set(Q))
    return dict(name=name, Q=Q, amb=amb, rho=rho, reach=reach, kernel=kernel)


def c_retract(d):
    return all(d["rho"](q) == q for q in d["Q"])


def c_distmin(d):
    S = d["Q"]
    for x in d["reach"]:
        best = min(abs(x - q) for q in S)
        if abs(x - d["rho"](x)) != best:
            return False
    return True


def c_monotone(d):
    R = d["reach"]
    return all(d["rho"](x) <= d["rho"](y) for x in R for y in R if x <= y)


def c_coherent(d):
    f = lambda a, b: d["rho"](d["amb"](a, b))
    R = d["reach"]
    return all(
        d["rho"](d["amb"](x, y)) == f(d["rho"](x), d["rho"](y)) for x in R for y in R
    )


SHAPES = [
    # kernel = is this demanded by a quoted intent, per `65` section 5
    build("u4 wrap add (K1)", UQ, ADD, wrap, True),
    build("i4 wrap add (K1)", SQ, ADD, wrap, True),
    build("u4 saturate add (K1)", UQ, ADD, clamp, True),
    build("i4 saturate add (K1)", SQ, ADD, clamp, True),
    build("gf(2)^4 xor (K5)", UQ, XOR, ident, True),
    build("u4 constant zero", UQ, ADD, const_zero, False),
    build("i4 opposite bound", SQ, ADD, opposite, False),
    build("i4 parity scramble", SQ, ADD, parity, False),
]

print("=" * 84)
print("p5. which clause is the right third verdict")
print("=" * 84)
print()
hdr = "%-24s %-7s %-9s %-9s %-9s %-9s" % (
    "declaration", "kernel", "RETRACT", "DIST-MIN", "MONOTONE", "COHERENT"
)
print(hdr)
print("-" * len(hdr))
rows = []
for d in SHAPES:
    r = (d["name"], d["kernel"], c_retract(d), c_distmin(d), c_monotone(d), c_coherent(d))
    rows.append(r)
    print("%-24s %-7s %-9s %-9s %-9s %-9s" % r)

print()
print("-" * 84)
print("ASSERTIONS")
print("-" * 84)

# 1. distance-minimising implies retraction, over everything measured.
imp = [r for r in rows if r[3] and not r[2]]
assert not imp, "a shape was distance-minimising and not a retraction: %s" % imp
print("  ok  distance-minimising implies retraction in every row measured")

# 2. and it is STRICTLY stronger: something retracts and does not minimise.
strict = [r[0] for r in rows if r[2] and not r[3]]
assert strict, "the two clauses coincided; there is nothing to choose between"
print("  ok  and strictly so: %s retract without minimising" % ", ".join(strict))

# 3. the strictly stronger clause excludes a KERNEL item.
excluded = [r[0] for r in rows if r[1] and not r[3]]
assert excluded, "distance-minimising excluded no kernel item"
print("  ok  the stronger clause EXCLUDES kernel items: %s" % ", ".join(excluded))

# 4. the weaker clause excludes none of them.
kernel_fail = [r[0] for r in rows if r[1] and not r[2]]
assert not kernel_fail, "retraction excluded a kernel item: %s" % kernel_fail
print("  ok  the weaker clause excludes no kernel item")

# 5. and it still does the job it was introduced for.
zero = [r for r in rows if r[0] == "u4 constant zero"][0]
assert not zero[2] and zero[4] and zero[5], "constant zero no longer isolates the clause"
print("  ok  and still excludes the reduction that computes nothing, which passes")
print("      both of the proposed verdicts")

print()
print("READING. The third verdict is the RETRACTION and not the distance law, and")
print("the reason is the same one section 4 gives for the order discriminator: the")
print("stronger, more natural-sounding clause throws out a kernel item op's I3")
print("demands. Reaching for the strongest available law is exactly the move that")
print("would have got this wrong, twice, in one unit.")
