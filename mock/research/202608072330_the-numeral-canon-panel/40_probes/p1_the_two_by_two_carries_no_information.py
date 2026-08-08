#!/usr/bin/env python3
"""
p1. How much information does "the four names are an exact bijection with a
two-by-two, zero cells spare" actually carry?

`25` section 4.1 reports that the four strategy names fill the two-by-two of
(headroom x layout) exactly, and reads that as evidence about the decomposition:
"Four names filling a two-by-two exactly is not what four values of one axis
looks like."

This probe tests that reading two ways.

  Part A. Enumerate every assignment of four distinct labels to the four cells
  of a two-by-two, and check how many of them are "an exact bijection with every
  cell filled and none repeated". If the answer is "all of them", the property
  is an identity (4 = 2*2) rather than a measurement, and observing it in arvo's
  table tells you nothing about arvo.

  Part B. The bijection would carry information if the PLACEMENT were forced by
  something independent of the table. Op has now stated per-strategy intents in
  his own words. Encode those as constraints on the placement and count the
  survivors. Three outcomes are informative and they are different:
    - exactly one survivor, and it is the shipped table  -> the placement is
      determined by intent and the table records it.
    - several survivors                                  -> the placement is
      underdetermined by intent; the table picked one.
    - zero survivors                                     -> the intents cannot
      be tiled onto the two-by-two at all, and the bijection is an artifact of
      insisting on a grid.

Every constraint below is MY READING of a verbatim quote, the quote is carried
next to it, and each is switchable so a reader who rejects one can re-run
without it. C1 is the most literal and is always on; the subset sweep shows
which constraint does the work.

Sources for the quotes, all in this panel directory, all frozen:
  36_op_the_intent_behind_each_strategy.md
  37_op_warm_imitates_rust_and_strategy_is_not_orthogonal.md
  38_op_the_strategies_weigh_measurements_differently.md
The shipped table is quoted at 25_torvalds_what_a_strategy_is.md:127-132, which
reproduces arvo/.claude/rules/implementation.md:52-58.
"""

from itertools import permutations
from itertools import combinations

HEADROOM = ("minimum", "doubled")
LAYOUT = ("addressable", "packed")
CELLS = [(h, l) for h in HEADROOM for l in LAYOUT]
NAMES = ("Hot", "Warm", "Cold", "Precise")

# The placement the shipped preset table records, read off 25:127-132.
SHIPPED = {
    "Hot": ("minimum", "addressable"),
    "Warm": ("doubled", "addressable"),
    "Cold": ("minimum", "packed"),
    "Precise": ("doubled", "packed"),
}

QUOTES = {
    "C1": (
        "Cold is packed.",
        "36: \"Cold is optimised for cold paths and cold storage, which means, "
        "it aggressively minimises and bitpacks\". Literal: the word bitpacks "
        "is op's.",
    ),
    "C2": (
        "Cold is minimum headroom.",
        "36: \"it aggressively minimises\"; 37: \"it should remain small for "
        "memory or disk storage, because it's just sitting basically\". "
        "Reading: minimising storage excludes carrying headroom beyond the "
        "numeral.",
    ),
    "C3": (
        "Precise is not packed.",
        "36: Precise is \"throwing out all cold or hot axis optimisations to "
        "be accurate and precise\". Two-step reading: C1 establishes in op's "
        "own words that bitpacking is Cold's mechanism, so bitpacking is a "
        "cold-axis optimisation, so Precise throws it out.",
    ),
    "C4": (
        "Warm is not packed.",
        "37 quoting op's standing call: Warm \"should behave like native "
        "primitives in regular old rust would\". Reading: a native Rust "
        "primitive is individually addressable, not bit-packed.",
    ),
}

# Hot is deliberately left unconstrained on both coordinates. Op's words give
# Hot an objective (performance) and no mechanism, and under 38 the mechanism
# is decided by measurement, which means it may differ by regime. 27 measures
# packing winning against 2- and 4-byte carriers at four cores and losing at
# one, so a packed Hot is not excluded by anything op has said.


def satisfies(assignment, active):
    if "C1" in active and assignment["Cold"][1] != "packed":
        return False
    if "C2" in active and assignment["Cold"][0] != "minimum":
        return False
    if "C3" in active and assignment["Precise"][1] == "packed":
        return False
    if "C4" in active and assignment["Warm"][1] == "packed":
        return False
    return True


def is_exact_bijection(assignment):
    occupied = list(assignment.values())
    return len(set(occupied)) == 4 and set(occupied) == set(CELLS)


def main():
    all_assignments = [dict(zip(NAMES, p)) for p in permutations(CELLS)]

    print("PART A: is 'an exact bijection, zero cells spare' a measurement?")
    print(f"  placements of 4 distinct labels into a 2x2 grid: {len(all_assignments)}")
    n_bij = sum(1 for a in all_assignments if is_exact_bijection(a))
    print(f"  of those, exact bijections with every cell filled and none repeated: {n_bij}")
    print(f"  fraction: {n_bij}/{len(all_assignments)}")
    print("  -> the property holds of every placement. It is 4 = 2*2 restated,")
    print("     not a fact about arvo. Observing it in the preset table")
    print("     distinguishes that table from no other placement.")
    print()

    print("PART B: does op's stated intent determine the placement?")
    print()
    for key, (claim, src) in QUOTES.items():
        print(f"  {key}: {claim}")
        print(f"      {src}")
    print()

    optional = ["C2", "C3", "C4"]
    print("  survivors by constraint subset (C1 always on):")
    print("  {:<22} {:>9}  {:>16}  {}".format("active", "survivors", "shipped survives?", "if unique, which"))
    for r in range(0, len(optional) + 1):
        for combo in combinations(optional, r):
            active = {"C1"} | set(combo)
            survivors = [a for a in all_assignments if satisfies(a, active)]
            shipped_ok = SHIPPED in survivors
            label = "+".join(sorted(active))
            uniq = ""
            if len(survivors) == 1:
                s = survivors[0]
                uniq = " ".join(f"{n}={s[n][0][:3]}/{s[n][1][:4]}" for n in NAMES)
            print("  {:<22} {:>9}  {:>16}  {}".format(label, len(survivors), "yes" if shipped_ok else "NO", uniq))
    print()

    full = {"C1", "C2", "C3", "C4"}
    survivors = [a for a in all_assignments if satisfies(a, full)]
    print(f"  under all four constraints: {len(survivors)} survivor(s)")
    for s in survivors:
        print("    " + ", ".join(f"{n}=({s[n][0]}, {s[n][1]})" for n in NAMES))
    print(f"  shipped table among them: {'yes' if SHIPPED in survivors else 'NO'}")
    print()

    # Which cell is the problem, stated as a count rather than as an opinion.
    print("  cell occupancy across the survivors of the full constraint set:")
    for cell in CELLS:
        who = sorted({n for s in survivors for n in NAMES if s[n] == cell})
        print(f"    {cell}: {who if who else '(unreachable)'}")
    print()

    # And the same question asked of the intents alone, without insisting on a
    # bijection: how many of the four cells does any intent-respecting name want?
    print("PART C: drop the bijection requirement and ask what each name wants.")
    wants = {}
    for name in NAMES:
        ok = []
        for cell in CELLS:
            probe = {n: (cell if n == name else None) for n in NAMES}
            bad = False
            if name == "Cold" and (cell[1] != "packed" or cell[0] != "minimum"):
                bad = True
            if name == "Precise" and cell[1] == "packed":
                bad = True
            if name == "Warm" and cell[1] == "packed":
                bad = True
            if not bad:
                ok.append(cell)
        wants[name] = ok
        print(f"    {name:<8} admissible cells: {ok}")
    unwanted = [c for c in CELLS if all(c not in wants[n] for n in NAMES if n != "Hot")]
    print(f"    cells no name but Hot may occupy: {unwanted}")
    print("  -> a bijection forces some name into a cell its intent excludes.")
    print("     The tiling is a property of the grid, not of the intents.")


if __name__ == "__main__":
    main()
