#!/usr/bin/env python3
"""p10. Open every file:line `114` cites and test its CONTENT, not that it
resolves.

The instrument is `111_probes/p9`'s, reused rather than rebuilt, because that is
what the brief asks and because a checker written fresh for one's own file tends
to check the claims one remembers making. `RULES.md` records a member finding
seven of its own citations wrong this way, and a reference that resolves is not a
reference that says what you claim.

Each entry names the path, the line, the substring the claim depends on, and what
the claim is. Substrings match against the cited line and its two neighbours,
because a claim about a sentence can be about a sentence that wraps.

NEGATIVE CONTROL
----------------
Two deliberately wrong entries are included and marked. They must both FAIL, or
the checker is matching everything and its passes mean nothing.
"""

import os
import sys

PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ARVO = os.path.dirname(os.path.dirname(os.path.dirname(PANEL)))
BENCH = os.path.join(ARVO, "mock", "benches")

CITES = [
    # ---- op's intents, which govern
    (f"{PANEL}/INTENTS.md", 240,
     "unmeasured or unknown does not list in the predicate",
     "op's instruction that unmeasured goes unstated"),
    (f"{PANEL}/INTENTS.md", 252,
     "collapses to whatever is available at const time",
     "op's instruction on what a const predicate reaches"),
    (f"{PANEL}/INTENTS.md", 323,
     "We shouldn't police what kind of laws there are",
     "I16, the reason this file states arms rather than a winner"),
    (f"{PANEL}/INTENTS.md", 216,
     "RATIFIED",
     "I13 is the entry holding the RATIFIED rung"),

    # ---- the two offered statements' observable-axis clause
    (f"{PANEL}/106_giesen_consolidation_the_strategy_axis.md", 172,
     "assignment on the axes a consumer can observe",
     "106 section 1 defines component one by observability"),
    (f"{PANEL}/108_lamport_the_pair_attacked.md", 822,
     "assignment on the axes a consumer can observe",
     "108 section 7 does the same"),
    (f"{PANEL}/108_lamport_the_pair_attacked.md", 825,
     "An axis belongs here if there is **any** reachable chain",
     "108:825 is the membership criterion, which is what settled the refinement"),

    # ---- the sentence this file corrects
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 928,
     "Checking only the derivation's result rather than every node is unsound",
     "112 section 9 states it unqualified"),
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 1113,
     "A root-only range check is unsound and the per-node check is load-bearing",
     "F112-21 is the finding it came from"),
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 1116,
     "overflow policy = sat",
     "and F112-21's own predicate lists only the saturating policy"),

    # ---- the finding this file refutes at one policy
    (f"{PANEL}/111_jhala_the_primitive_attacked.md", 1385,
     "overflow policy in {sat, wrap}",
     "F111-15's predicate claims both overflow policies"),
    (f"{PANEL}/111_jhala_the_primitive_attacked.md", 1388,
     "Sufficient and not necessary",
     "and F111-15 states its own incompleteness without quantifying it"),

    # ---- the domination claim this file bounds
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 1127,
     "dominates both the interval rule",
     "F112-24 is the domination claim"),
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 1142,
     "Every declared extent in every sweep of this file is one-sided",
     "F112-23 is the limitation fourteen lines away that bounds it"),
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 1029,
     "exact only on left-nested",
     "F112-6's 'only', which 111 already refuted"),
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 1039,
     "which no node-wise rule reaches",
     "F112-7's annihilation claim"),

    # ---- the placement of the overflow policy in the declared semantics
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 1003,
     "zero for the overflow policy",
     "F112-2 counts zero directions for the overflow policy"),
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 937,
     "Zero means it is part of the declared semantics",
     "and 112 section 9 says what zero directions means"),
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 1009,
     "two assignments of an observable axis compute the same",
     "F112-3 measures the same placement from the consumer side"),

    # ---- the expressibility claim this file closes
    (f"{PANEL}/111_jhala_the_primitive_attacked.md", 1269,
     "Expressibility rests on `112` rather than on me",
     "111 section 19.2 declines to claim the predicate is writable"),

    # ---- the open item this file attacks
    (f"{PANEL}/111_jhala_the_primitive_attacked.md", 1478,
     "a harness rather than an argument closes",
     "111 section 26 carries the compile-time question as the open one"),
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 494,
     "not available as a const predicate at a real width",
     "112 section 6 says the enumerating oracle cannot be the predicate"),

    # ---- the state figure this file corrects
    (f"{PANEL}/111_jhala_the_primitive_attacked.md", 1402,
     "differ in state by the term's leaf count",
     "F111-18 is the state figure"),

    # ---- the alternative E that started the sitting
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 190,
     "I lean toward it being one of those axes",
     "111's alternative E, quoted by 112"),

    # ---- the bench surface the test gate read
    (f"{BENCH}/variants/warm-clamp-shared/src/lib.rs", 298,
     "for &x in chunk",
     "warm-clamp folds over a chunk, so its leaves are distinct"),
    (f"{BENCH}/variants/warm-clamp-shared/src/lib.rs", 299,
     "acc.wadd",
     "and the fold body is a single accumulating operation"),
    (f"{BENCH}/variants/satfold-shared/src/lib.rs", 1125,
     "assert_ne!",
     "satfold asserts its oracle CATCHES a deliberately broken kernel"),
    (f"{BENCH}/bitpack-carrier-width_n16384.csv", 1,
     "e2e_ns,algo_ns,bridge_ns",
     "the harness CSV schema is runtime fields"),

    (f"{BENCH}/variants/warm-clamp-shared/src/lib.rs", 291,
     "let safe = accumulator_bits_needed(W, ARITY) <= A::BITS",
     "the shipped kernel selects on a const predicate over width and arity"),
    (f"{BENCH}/variants/warm-clamp-shared/src/lib.rs", 301,
     "acc.min_with(limit)",
     "and its safe branch reduces once at the end of the chunk"),
    (f"{BENCH}/variants/warm-clamp-shared/src/lib.rs", 304,
     "sat_add",
     "while its other branch reduces at every node"),
    (f"{BENCH}/variants/warm-clamp-shared/src/lib.rs", 159,
     "w + ceil_log2(arity)",
     "and the predicate is the corner rule's root bound for a fold"),

    (f"{PANEL}/106_giesen_consolidation_the_strategy_axis.md", 281,
     "TWO EXPERTS requires two",
     "106 section 3 states the rung definitions this file uses"),

    # ---- NEGATIVE CONTROLS: both must FAIL
    (f"{PANEL}/112_leijen_where_the_refinement_lives.md", 928,
     "overflow policy = wrap",
     "CONTROL, MUST FAIL: 112's sentence does not carry a policy predicate"),
    (f"{PANEL}/111_jhala_the_primitive_attacked.md", 1385,
     "the corner rule is unsound",
     "CONTROL, MUST FAIL: F111-15 claims no such thing"),
]


def check(path, line, needle, claim):
    try:
        with open(path) as f:
            lines = f.read().splitlines()
    except OSError as e:
        return False, f"cannot open: {e}"
    if line < 1 or line > len(lines):
        return False, f"line {line} out of range (file has {len(lines)})"
    window = "\n".join(lines[max(0, line - 2):min(len(lines), line + 1)])
    return needle in window, window.splitlines()[min(1, line - 1)][:88]


def main():
    print("p10. Every citation in `114`, opened")
    print("=" * 78)
    print()
    ok = bad = 0
    controls_ok = controls_bad = 0
    for path, line, needle, claim in CITES:
        good, ctx = check(path, line, needle, claim)
        is_control = claim.startswith("CONTROL")
        short = os.path.relpath(path, PANEL if path.startswith(PANEL) else ARVO)
        if is_control:
            if good:
                controls_bad += 1
                print(f"  CONTROL DID NOT FAIL  {short}:{line}")
                print(f"      {claim}")
            else:
                controls_ok += 1
        elif good:
            ok += 1
        else:
            bad += 1
            print(f"  FAIL  {short}:{line}")
            print(f"      claim:  {claim}")
            print(f"      wanted: {needle}")
            print(f"      got:    {ctx}")
    print()
    print(f"  citations checked : {ok + bad}")
    print(f"  passing           : {ok}")
    print(f"  failing           : {bad}")
    print(f"  controls that correctly failed : {controls_ok} of 2")
    print(f"  controls that wrongly passed   : {controls_bad}")
    print()
    if controls_ok != 2:
        print("  The controls did not both fail. The checker is matching too")
        print("  loosely and every pass above is worth nothing.")
        return 2
    if bad:
        print("  Citations are wrong. Fix the file, not the checker.")
        return 1
    print("  Every citation opens and says what the claim depends on, and both")
    print("  deliberately wrong entries were caught.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
