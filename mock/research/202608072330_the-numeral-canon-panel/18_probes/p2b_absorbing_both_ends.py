"""p2b. The absorbing reading with BOTH ends absorbing, and where it still fails.

p2 found that the absorbing reading survives an operation set that cannot decrease and fails on
one that can, and its first witness turned out to be the BOTTOM clamp rather than the top. `07`
section 4.2 modelled the top only. This probe fixes the model and asks the sharper question:

  If both saturating endpoints are read as absorbing, is the reading sound?

Three readings compared:

  point       gamma(d) = {d} for every datum
  top         gamma(top) = [top, inf), else {d}          the reading 07 measured
  both        gamma(top) = [top, inf), gamma(bot) = (-inf, bot], else {d}

and every failure is classified by which clamp produced it, so the two mechanisms are not
conflated the way p2's first witness conflated them.

DOMAIN, with every count. Numeral U<3,3> unsigned, 64 values k/8 for k in 0..63, top 63/8,
bottom 0, saturating at both ends. All operations exact in range, so saturation is the only
abstraction. Chains are enumerated exhaustively from every one of the 64 start values over the
operand alphabet at the stated length. Counts are over that enumeration only.

Run: python3 p2b_absorbing_both_ends.py
"""

from fractions import Fraction as F
from itertools import product

STEP = F(1, 8)
N = 64
TOP = (N - 1) * STEP
BOT = F(0)


def sat(x):
    if x >= TOP:
        return TOP, "top"
    if x <= BOT:
        return BOT, "bot"
    return x, None


def main():
    print(f"# U<3,3> unsigned: {N} values, step {STEP}, bottom {BOT}, top {TOP}")
    print()

    starts = [k * STEP for k in range(N)]
    ops = [("+1", lambda x: x + 1), ("+2", lambda x: x + 2), ("-1", lambda x: x - 1)]

    for steps in (3, 4, 5):
        bad = {"point": 0, "top": 0, "both": 0}
        blame = {"top-then-down": 0, "bot-then-up": 0, "never-clamped": 0, "other": 0}
        total = 0
        witness_both = None
        for start in starts:
            for chain in product(ops, repeat=steps):
                exact = start
                abstract = start
                clamps = []
                for _, op in chain:
                    exact = op(exact)
                    abstract, which = sat(op(abstract))
                    clamps.append(which)
                total += 1

                ok_point = abstract == exact
                ok_top = (exact >= TOP) if abstract == TOP else (abstract == exact)
                if abstract == TOP:
                    ok_both = exact >= TOP
                elif abstract == BOT:
                    ok_both = exact <= BOT
                else:
                    ok_both = abstract == exact

                if not ok_point:
                    bad["point"] += 1
                if not ok_top:
                    bad["top"] += 1
                if not ok_both:
                    bad["both"] += 1
                    hit_top = "top" in clamps
                    hit_bot = "bot" in clamps
                    if hit_top and clamps[-1] is None:
                        blame["top-then-down"] += 1
                    elif hit_bot and clamps[-1] is None:
                        blame["bot-then-up"] += 1
                    elif not hit_top and not hit_bot:
                        blame["never-clamped"] += 1
                    else:
                        blame["other"] += 1
                    if witness_both is None:
                        witness_both = (start, [n for n, _ in chain], abstract, exact, clamps)

        print(f"steps={steps}  chains={total:7d}   unsound: "
              f"point={bad['point']:6d}  top-absorbing={bad['top']:6d}  "
              f"both-absorbing={bad['both']:6d}")
        print(f"          both-absorbing failures by cause: {blame}")
        if witness_both:
            s, c, a, e, cl = witness_both
            print(f"          first witness: start={s} chain={c} abstract={a} exact={e} "
                  f"clamps={cl}")
        print()

    print("# the structural statement the numbers are pointing at")
    print("  An absorbing endpoint is sound exactly while the computation STAYS at it.")
    print("  The instant an operation moves off the endpoint into the interior, the")
    print("  interior datum's point denotation is asserted, and it is false, because the")
    print("  information that was absorbed cannot be recovered.")
    print()
    print("  So 'the top absorbs' is not a denotation for the numeral. It is a denotation")
    print("  for the numeral RESTRICTED to operations that cannot decrease, and the")
    print("  restriction is the part nobody has written down.")


if __name__ == "__main__":
    main()
