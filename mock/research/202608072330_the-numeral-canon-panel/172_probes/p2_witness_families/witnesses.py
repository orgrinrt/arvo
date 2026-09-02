#!/usr/bin/env python3
"""172 P2. Constructive witness families for the no-threshold double-rounding
theorem, verified at every (F, M) strictly inside, F = 3..12.

CLAIM UNDER TEST (the proof in 172 section 5)
  For every F >= 3 and every M with F < M < 2F, there is an operand pair whose
  double rounding (nearest-ties-even at M, then at F) disagrees with its single
  rounding (nearest-ties-even at F). Three explicit families:
    W1 at M = 2F-1        : (a, b) = (1, 2^(F-1) + 1)
    W2 at M = F+1         : (a, b) = (1, 3*2^(F-2) - 1)
    W3 at F+2 <= M <= 2F-2: (a, b) = (2^(2F-M-1), 2^(M-F) + 1)
  This closes the width question 167 4.1 left enumerated at F in {6, 8, 10} and
  169 extended to 4..10: with the families verified, the claim is constructive
  at F any (>= 3), M any strictly inside.

THE CASES THAT MUST FAIL, declared before the run
  CONTROL A: the family (1, 2^(F-1)) sits exactly on the F-level tie and must
  NOT disagree universally; if the checker reports it as a universal witness
  family, the checker cannot tell a witness from a non-witness.
  CONTROL B: full enumeration at F in {4, 6, 8, 10}, M = 2F-1 must reproduce
  169's disagreement count 2^(F-1) exactly; if not, this model is not the
  unit's model and nothing here transfers.
"""

def round_ne(x, g):
    # nearest multiple of g, ties to even multiple
    q, r = divmod(x, g)
    if 2 * r < g:
        return q * g
    if 2 * r > g:
        return (q + 1) * g
    return (q + (q % 2)) * g  # tie: pick even quotient

def double(p, F, M):
    return round_ne(round_ne(p, 1 << (2 * F - M)), 1 << F)

def single(p, F):
    return round_ne(p, 1 << F)

def witness(F, M):
    if M == 2 * F - 1:
        return (1, (1 << (F - 1)) + 1)
    if M == F + 1:
        return (1, 3 * (1 << (F - 2)) - 1)
    return (1 << (2 * F - M - 1), (1 << (M - F)) + 1)

def main():
    ok = True
    rows = 0
    for F in range(3, 13):
        for M in range(F + 1, 2 * F):
            a, b = witness(F, M)
            assert 0 <= a < (1 << F) and 0 <= b < (1 << F), (F, M, a, b)
            p = a * b
            d, s = double(p, F, M), single(p, F)
            rows += 1
            if d == s:
                ok = False
                print(f"FAMILY FAILS at F={F} M={M}: pair=({a},{b}) double={d} single={s}")
    print(f"witness families: {rows} (F, M) cells checked, {'ALL DISAGREE' if ok else 'FAILURE'}")

    # CONTROL A: the on-tie family must not be a universal witness family
    a_fails = 0
    for F in range(3, 13):
        M = 2 * F - 1
        a, b = 1, 1 << (F - 1)
        d, s = double(a * b, F, M), single(a * b, F)
        if d == s:
            a_fails += 1
    print(f"CONTROL A (on-tie family agrees somewhere): "
          f"{'PASS (fires, ' + str(a_fails) + ' of 10 cells agree)' if a_fails > 0 else 'FAIL'}")

    # CONTROL B: reproduce 169's exhaustive counts at M = 2F-1
    okB = True
    for F in (4, 6, 8, 10):
        M = 2 * F - 1
        n = sum(1 for a in range(1 << F) for b in range(1 << F)
                if double(a * b, F, M) != single(a * b, F))
        expect = 1 << (F - 1)
        print(f"CONTROL B F={F}: exhaustive disagreements at M=2F-1 = {n}, 2^(F-1) = {expect}, "
              f"{'match' if n == expect else 'MISMATCH'}")
        okB &= n == expect
    # cross-check: full enumeration at small F confirms every interior M nonzero
    okC = True
    for F in (3, 4, 5, 6):
        for M in range(F + 1, 2 * F):
            n = sum(1 for a in range(1 << F) for b in range(1 << F)
                    if double(a * b, F, M) != single(a * b, F))
            if n == 0:
                okC = False
                print(f"CROSS-CHECK FAILS: zero disagreements at F={F} M={M}")
    print(f"cross-check (exhaustive, F 3..6, all interior M nonzero): {'PASS' if okC else 'FAIL'}")
    print(f"VERDICT: {'PASS' if ok and a_fails > 0 and okB and okC else 'FAIL'}")

main()
