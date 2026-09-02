# Probe 3: the radix-power exponential is algebraic, its exact hits are decidable,
# and its comparison object is exponential-width.
#
# Claim under test: 2^x for x = k/2^F on a dyadic grid is an ALGEBRAIC number
# (a root of y^(2^F) = 2^k), so exact grid hits and rounding ties are decidable
# by exact integer comparison, with no table-maker's dilemma; but the comparison
# object j^(2^F) doubles in width per fractional bit of the operand, so the
# decidable carrier is exponential in F, the same growth class as division's
# lcm accumulator over a divisor numeral (file 43 probe 1), not sqrt's linear
# residue (file 99 probe 1).
#
# Sub-claims, each exhaustive over the stated range, exact integers only:
#   A: exact hits of 2^(k/2^F) on the result grid j/2^G occur EXACTLY at integer
#      exponents (k a multiple of 2^F).
#   B: rounding ties ((2j+1)/2^(G+1) hits) never occur.
#   C: the comparison object's width doubles per unit of F (measured).

def exp2_sweep(F, G, kmax):
    hits, ties, maxw = [], 0, 0
    Q = 2 ** F
    for k in range(0, kmax + 1):
        lhs = 2 ** (k + G * Q)          # 2^(k/Q) == j/2^G  <=>  j^Q == 2^(k + G*Q)
        t = 2.0 ** (k / Q) * 2 ** G     # float only locates the search window
        for j in range(max(1, int(t) - 2), int(t) + 3):
            w = (j ** Q).bit_length()
            maxw = max(maxw, w)
            if j ** Q == lhs:
                hits.append((k, j))
        lhs_t = 2 ** (k + (G + 1) * Q)  # tie: ((2j+1)/2^(G+1))^Q == 2^(k/Q)
        for j in range(max(1, int(t) - 2), int(t) + 3):
            if (2 * j + 1) ** Q == lhs_t:
                ties += 1
    return hits, ties, maxw

ok = True
for (F, G, kmax) in [(1, 4, 4), (2, 4, 8), (3, 4, 24), (4, 4, 48)]:
    hits, ties, maxw = exp2_sweep(F, G, kmax)
    integer_only = all(k % (2 ** F) == 0 for (k, _) in hits)
    n_expected = kmax // (2 ** F) + 1
    print(f"F={F} G={G} k<={kmax}: hits={hits} ties={ties} max_comparison_bits={maxw} "
          f"hits_at_integer_x_only={integer_only} expected_hit_count={n_expected}")
    ok &= integer_only and ties == 0 and len(hits) == n_expected

print("A, B hold on every sweep:", ok)
print("C: max comparison bits by F:", [exp2_sweep(F, 4, 8 * 2 ** (F - 1))[2] for F in (1, 2, 3, 4)])
