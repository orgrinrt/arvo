"""p5. Testing the dispatch's third example, which turns out not to be a denotation question.

The dispatch offers three places the design might already denote sets: a saturating top, a
refusing conversion, and "an accumulator carrying more width than it needs denotes an interval
of achievable values". The first is real and p2/p2b measure it. This probe tests the third and
finds it is a different kind of statement wearing the same words, and then measures the thing it
actually is.

  THE CLAIM AS OFFERED. An accumulator with headroom denotes an interval.

  WHY IT DOES NOT HOLD. Denotation is a property of a DATUM. Every datum of an accumulator
  denotes exactly one rational, before and after any headroom is added. Widening the accumulator
  changes which data exist; it does not change what any datum means. The clause is untouched.

  WHAT IS ACTUALLY TRUE, AND IS A DIFFERENT STATEMENT. The accumulator's inhabitant set is
  larger than its REACHABLE set. After n additions of values from a source numeral, only a
  prefix of the accumulator's range can occur. That is a predicate on a value, of the shape
  {v : Acc | v <= n * max_source}, which is a refinement rather than a denotation.

  WHY THE DIFFERENCE MATTERS HERE. A denotation change would break the boundary clause. A
  refinement does not touch the boundary at all, and lives in a layer the design has not named.
  The two are worth keeping apart, because one is an attack on `08` section 5's first clause and
  the other is a separate gap.

DOMAIN, with every count. Source numeral U<3,3> unsigned: 64 values, ticks 0..63. Accumulators
are unsigned numerals of total width W ticks for W in the stated list. Trip counts n as stated.
Reachability is computed exactly by closure over addition from zero, so the counts are exact and
not sampled.

Run: python3 p5_headroom_is_a_refinement_not_a_denotation.py
"""

SRC_TICKS = 64  # source values are tick counts 0..63


def reachable(n, src_max_tick, acc_ticks):
    """Tick values reachable in the accumulator after exactly n additions from the source.

    Saturation is deliberately NOT applied: this probe is about the headroom case, which is
    the case where the accumulator was sized so no clamp occurs.
    """
    reach = {0}
    for _ in range(n):
        nxt = set()
        for r in reach:
            for s in range(src_max_tick + 1):
                t = r + s
                if t < acc_ticks:
                    nxt.add(t)
        reach = nxt
    return reach


def main():
    src_max = SRC_TICKS - 1
    print(f"# source numeral: {SRC_TICKS} values, top tick {src_max}")
    print()
    print("# inhabitants against reachable values, per accumulator width and trip count")
    print(f"{'acc bits':>9} {'inhabitants':>12} {'n':>4} {'reachable':>11} "
          f"{'unreachable':>12} {'share unreachable':>18}")
    for bits in (8, 10, 12, 14):
        acc = 1 << bits
        for n in (1, 2, 4, 8):
            r = reachable(n, src_max, acc)
            unreach = acc - len(r)
            print(f"{bits:>9} {acc:>12} {n:>4} {len(r):>11} {unreach:>12} "
                  f"{unreach / acc:>17.2%}")
    print()

    print("# the reachable set is an interval of the grid, not a scattered subset")
    acc = 1 << 12
    for n in (1, 2, 4, 8):
        r = sorted(reachable(n, src_max, acc))
        contiguous = (r == list(range(r[0], r[-1] + 1)))
        print(f"  n={n}: min tick {r[0]}, max tick {r[-1]}, contiguous={contiguous}, "
              f"predicate v <= {n * src_max}")
    print()

    print("# so the honest statement, and it is about the TYPE rather than about a datum")
    print("  Every datum of the accumulator denotes one rational. What headroom creates is a")
    print("  set of inhabitants no computation reaches, and the reachable set is exactly")
    print("  {v : v <= n * src_max}. That is a predicate on a value. It is the shape a")
    print("  refinement type carries and it is not a denotation, so `08` section 5's first")
    print("  clause is untouched by it.")
    print()

    print("# the second offered example, a refusing conversion, checked the same way")
    print("  A conversion that refuses is a PARTIAL map on data, not a set-valued map.")
    print("  It does become a set-valued map under one reading, where a refusal denotes the")
    print("  EMPTY set, and that reading is consistent with the containment order because")
    print("  the empty set is below everything. Consistency is not a reason to adopt it:")
    print("  the reading buys nothing unless the design wants a bottom element, and the")
    print("  record does not currently ask for one.")


if __name__ == "__main__":
    main()
