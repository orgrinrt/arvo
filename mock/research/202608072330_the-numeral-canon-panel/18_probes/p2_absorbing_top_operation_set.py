"""p2. The absorbing reading of a saturating top, and which operations it survives.

`07` section 4.2 measures that an n-step saturating FOLD (addition) is sound under the reading
"the top denotes [top, infinity)" and unsound under "a datum denotes its own value", and that the
design's algorithm crates already rely on the absorbing reading without the record stating it.

This probe is an independent instance arrived at differently, and it goes past the question `07`
asked. `07` measured one operation. Soundness of an abstraction is quantified over the operation
set, not over one operation, so the question this probe asks is:

  For which operations does the absorbing reading stay sound?

Soundness here is the standard containment statement: the exact value computed over the
rationals must lie inside the denotation of the abstract result.

  point reading      gamma(d) = {d}                  so sound means abstract == exact
  absorbing reading  gamma(top) = [top, infinity)    so sound means exact == abstract,
                                                     or abstract is top and exact >= top

DOMAIN, stated with every count below. The numeral is U<3,3> unsigned: 64 values, k/8 for
k in 0..63, top 63/8, saturating on overflow and on underflow at zero. Every operation used is
EXACT in range, so no rounding enters and the only source of abstraction is the saturation.
Chains are enumerated exhaustively over the stated operand alphabet at the stated length; the
counts are over that enumeration and over nothing else.

Run: python3 p2_absorbing_top_operation_set.py
"""

from fractions import Fraction as F
from itertools import product

FRAC = 3
INT = 3
STEP = F(1, 2 ** FRAC)
N = 2 ** (INT + FRAC)
TOP = (N - 1) * STEP
BOT = F(0)


def sat(x):
    """Saturating placement of an exact rational onto the grid. Exact in range by construction."""
    if x >= TOP:
        return TOP
    if x <= BOT:
        return BOT
    assert x % STEP == 0, f"operand alphabet made an off-grid value {x}"
    return x


def sound_point(abstract, exact):
    return abstract == exact


def sound_absorbing(abstract, exact):
    if abstract == TOP:
        return exact >= TOP
    return abstract == exact


def run(name, start_values, ops, steps):
    """Enumerate every chain of `steps` operations drawn from `ops`, from every start value."""
    bad_point = 0
    bad_absorb = 0
    total = 0
    witness = None
    for start in start_values:
        for chain in product(ops, repeat=steps):
            exact = start
            abstract = start
            for op in chain:
                exact = op(exact)
                abstract = sat(op(abstract))
            total += 1
            if not sound_point(abstract, exact):
                bad_point += 1
            if not sound_absorbing(abstract, exact):
                bad_absorb += 1
                if witness is None:
                    witness = (start, chain, abstract, exact)
    print(f"{name:34s} chains={total:7d}  unsound point={bad_point:7d}  "
          f"unsound absorbing={bad_absorb:7d}")
    return witness


def main():
    print(f"# numeral U<{INT},{FRAC}> unsigned: {N} values, step {STEP}, top {TOP}, "
          f"saturating both ends")
    print()

    starts = [k * STEP for k in range(N)]

    add1 = lambda x: x + 1
    add2 = lambda x: x + 2
    sub1 = lambda x: x - 1
    mul0 = lambda x: x * 0
    mul1 = lambda x: x * 1
    mul2 = lambda x: x * 2
    halve = lambda x: x / 2 if (x / 2) % STEP == 0 else x

    print("# A. monotone non-decreasing operations only, which is what 07 measured")
    run("add-only, 4 steps", starts, [add1, add2], 4)
    run("add-only, 6 steps", starts, [add1, add2], 6)
    run("scale up by 2, 4 steps", starts, [mul2], 4)
    print()

    print("# B. an operation set that can decrease")
    w1 = run("add and subtract, 4 steps", starts, [add1, add2, sub1], 4)
    w2 = run("add and multiply by zero, 3 steps", starts, [add2, mul0], 3)
    w3 = run("add and multiply by one, 4 steps", starts, [add2, mul1], 4)
    print()

    print("# witnesses, first found, for the absorbing failures")
    for label, w in (("add/sub", w1), ("add/mul0", w2), ("add/mul1", w3)):
        if w is None:
            print(f"{label:10s} none: absorbing stayed sound over the whole enumeration")
        else:
            start, chain, abstract, exact = w
            names = " ".join(op.__name__ if op.__name__ != "<lambda>" else "op" for op in chain)
            print(f"{label:10s} start={start} chain-length={len(chain)} "
                  f"abstract={abstract} exact={exact}")
    print()

    print("# C. the single step that breaks it, stated minimally")
    x = TOP
    print(f"  start at the top, {TOP}, whose absorbing denotation is [{TOP}, inf)")
    print(f"  one saturating add of 2 gives abstract {sat(x + 2)}, denotation [{TOP}, inf)  SOUND")
    y = sat(x + 2)
    print(f"  then one subtract of 1 gives abstract {sat(y - 1)}, denotation "
          f"{{{sat(y - 1)}}}, a point")
    print(f"  but the exact set was [{TOP}, inf) shifted down by 1, which is "
          f"[{TOP - 1}, inf), not a point")
    print("  so the abstraction has claimed a point where it holds an unbounded set")
    print()

    print("# D. does a second absorbing bottom repair it")
    print("  no, and the reason is structural rather than measured: the bottom absorbs")
    print("  [-inf, 0], and the failure above is at the top coming DOWN into the")
    print("  representable range, which neither absorbing end covers.")


if __name__ == "__main__":
    main()
