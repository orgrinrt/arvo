#!/usr/bin/env python3
# p6: is the Precise fork a fork?
#
# 45:163-178 names two readings of I7 ("accurate... especially within chains and ops") and treats
# them as exclusive alternatives:
#
#   (a) best approximation: match the exact once-rounded chain answer for every input. 45's
#       pigeonhole argument shows this forces a wider carried intermediate.
#   (b) refuse on inexact: "This reading needs no extra compute width at all: refusing an inexact
#       result can be checked at the storage width plus one flag bit." (45:167-169)
#
# The fork is presented as a choice between two designs. This file measures what each one admits,
# exhaustively, as a function of chain length, which is the dimension I7 actually names and which
# nothing in the panel had measured.
#
# Domain: raw integers 0..2^F-1 denoting values in [0,1) at F fractional bits, so W == F. A product
# of two of them is exact at F bits iff the low F bits of the raw product are zero. The W > F case
# is NOT covered and the last section says why the difference matters.
#
# Three designs are distinguished, and the third is one 45 does not name:
#
#   D1  refuse the moment a step is inexact, never carry anything wider   (reading (b), per step)
#   D3  carry the chain exactly and refuse only if the final answer is inexact  (refusal AND widening)
#   (D2, round every step at F bits, is Warm. it admits everything and is wrong on some of it,
#    which is 45's pigeonhole result and is not re-measured here.)
#
# Two instruments, cross-checked: a brute force over every chain, and a dynamic program over 2-adic
# valuations. They agree everywhere the brute force is affordable.
#
#   python3 p6_precise_fork_is_not_a_fork.py

from itertools import product


def v2(n, cap):
    """2-adic valuation of a raw, capped. raw 0 is exactly representable under every multiply."""
    if n == 0:
        return cap
    v = 0
    while n % 2 == 0:
        n //= 2
        v += 1
    return min(v, cap)


# ---------------------------------------------------------------- instrument one: brute force

def brute(F, k):
    """returns (admitted_per_step, admitted_at_end, total) over every chain of k multiplies."""
    M = 1 << F
    per_step = 0
    at_end = 0
    total = 0
    for x in range(M):
        for mults in product(range(M), repeat=k):
            total += 1
            # design D1: refuse the moment a step is inexact, never widen
            v = x
            ok_steps = True
            for a in mults:
                p = v * a
                if p % M != 0:
                    ok_steps = False
                    break
                v = p // M
            if ok_steps:
                per_step += 1
            # design D3: carry the chain exactly, refuse only if the final answer is inexact
            num = x
            for a in mults:
                num *= a
            if num % (M ** k) == 0:
                at_end += 1
    return per_step, at_end, total


# -------------------------------------------- instrument two: a DP over 2-adic valuations only

def dp(F, k, nonzero=False):
    M = 1 << F
    cap = F * (k + 2) + 4
    counts = {}
    lo = 1 if nonzero else 0
    for r in range(lo, M):
        counts[v2(r, cap)] = counts.get(v2(r, cap), 0) + 1

    # D1: state is the valuation of the running value. a step is exact iff v + v(a) >= F, and the
    # result's valuation is v + v(a) - F.
    state = dict(counts)
    for _ in range(k):
        nxt = {}
        for v, n in state.items():
            for va, na in counts.items():
                if v + va >= F:
                    w = min(v + va - F, cap)
                    nxt[w] = nxt.get(w, 0) + n * na
        state = nxt
    per_step = sum(state.values())

    # D3: the final answer is exact iff the total valuation of x and every multiplier reaches F*k.
    tot = dict(counts)
    for _ in range(k):
        nxt = {}
        for v, n in tot.items():
            for va, na in counts.items():
                w = min(v + va, cap)
                nxt[w] = nxt.get(w, 0) + n * na
        tot = nxt
    at_end = sum(n for v, n in tot.items() if v >= F * k)
    total = (M - lo) ** (k + 1)
    return per_step, at_end, total


def main():
    print("cross-check: brute force against the valuation DP, where brute force is affordable")
    print("  F  k   D1 brute      D1 dp         D3 brute      D3 dp         agree")
    for F in (3, 4):
        for k in (1, 2, 3):
            if (1 << F) ** (k + 1) > 3_000_000:
                continue
            b = brute(F, k)
            d = dp(F, k)
            agree = (b[0] == d[0]) and (b[1] == d[1]) and (b[2] == d[2])
            print("  %-2d %-3d %-13d %-13d %-13d %-13d %s"
                  % (F, k, b[0], d[0], b[1], d[1], "yes" if agree else "NO"))
    print()

    print("admitted fraction of the input space, by design and chain length")
    print()
    print("  D1  refuse per step, never widen        (45's reading (b), applied per operation)")
    print("  D3  carry the chain exactly, refuse only at the end   (refusal AND widening)")
    print()
    print("   F   k     D1 admitted     D3 admitted   D3/D1     D1 nonzero   D3 nonzero")
    for F in (4, 6, 8, 10):
        for k in (1, 2, 3, 4, 6, 8):
            p, e, t = dp(F, k)
            pn, en, tn = dp(F, k, nonzero=True)
            ratio = (e / p) if p else float("inf")
            print("  %2d  %2d  %12.6f%%  %12.6f%%  %6.1fx  %11.6f%%  %11.6f%%"
                  % (F, k, 100.0 * p / t, 100.0 * e / t, ratio,
                     100.0 * pn / tn, 100.0 * en / tn))
        print()

    # the equivalence, checked rather than asserted
    print("D1 and D3 admit the SAME chains once zero operands are excluded. checked:")
    bad = []
    for F in (3, 4, 5, 6, 8, 10):
        for k in (1, 2, 3, 4, 5, 6, 8):
            pn, en, _ = dp(F, k, nonzero=True)
            if pn != en:
                bad.append((F, k, pn, en))
    print("  cells checked: 42.  cells where they differ: %d" % len(bad))
    print("  and it is a theorem, not a coincidence. write T_i for the total 2-adic valuation of")
    print("  the start and the first i multipliers. D1 admits iff T_i >= i*F for every i; D3 admits")
    print("  iff T_k >= k*F. a nonzero raw below 2^F has valuation at most F-1, so if T_i <= i*F - 1")
    print("  then T_k <= i*F - 1 + (k-i)(F-1) = k*F - 1 - (k-i) < k*F. contrapositive: T_k >= k*F")
    print("  forces every T_i >= i*F. the hypothesis is exactly W == F, which is this file's domain.")
    print()

    print("what the numbers say, stated as what they are and no further:")
    print("  * refusal on inexact is not a viable design for chains, and the number is not close.")
    print("    at F=8 with nonzero operands it admits 1.18% of single multiplies, 0.0062% of pairs,")
    print("    and 0.000018% of triples, which is eighteen chains in a hundred million. a strategy")
    print("    whose intent is to be accurate WITHIN CHAINS cannot be one that refuses all of them.")
    print("  * where the two refusal designs were presented as a choice, they are the same policy.")
    print("    the 1.5x to 4.5x gap in the with-zero columns is entirely chains containing a zero")
    print("    operand, which are exact by accident.")
    print("  * so the fork 45 names is not two live designs. one arm is measurable and is not a")
    print("    design anyone ships. what survives as a real question is narrower and is about the")
    print("    container derivation directly: whether the wide value a multiply forms internally is")
    print("    CARRIED between operations, or consumed inside one and discarded. every fixed-point")
    print("    multiply forms a 2W-bit product whatever the strategy, so forming it is not what")
    print("    distinguishes Precise; carrying it is.")
    print()
    print("what this file does not establish:")
    print("  * the W > F case. the equivalence theorem above needs every operand's valuation to be")
    print("    at most F-1, which holds exactly when W == F. with integer bits present a raw can")
    print("    carry more trailing zeros than F, the implication breaks, and D1 and D3 may separate.")
    print("    a design keyed on total-and-fraction width would want that measured; I did not.")
    print("  * 18 section 3.4's 4.60% to 55.56% is a different domain (in-range multiplications and")
    print("    divisions over mixed integer and fraction widths). these numbers do not replace it")
    print("    and are not comparable to it row for row.")
    print("  * what any of the three designs costs in cycles or code size. no bench in this panel")
    print("    has priced it, so it is unpriced.")


if __name__ == "__main__":
    main()
