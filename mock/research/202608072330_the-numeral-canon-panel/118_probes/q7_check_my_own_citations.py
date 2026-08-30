#!/usr/bin/env python3
"""q7. Open every file:line `118` cites and test its CONTENT.

Same instrument as `114_probes/p10`, which is `111_probes/p9`'s, reused rather
than rebuilt. A reference that resolves is not a reference that says what you
claim, and this file makes a factual correction about another file's contents in
section 8.2, which is exactly the kind of claim that must be checkable.

Whitespace is normalised on both sides, because a quotation wrapped across two
source lines is still verbatim and `115` and `116` both hit that on their first
run.

NEGATIVE CONTROL
----------------
Two deliberately wrong entries, marked, which must both FAIL. If they pass the
checker is matching everything and its passes mean nothing.
"""

import os
import re
import sys

PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ARVO = os.path.dirname(os.path.dirname(os.path.dirname(PANEL)))
BENCH = os.path.join(ARVO, "mock", "benches")

F115 = f"{PANEL}/115_jhala_the_check_the_policy_selects.md"
F116 = f"{PANEL}/116_leijen_reply_what_the_homomorphism_opens.md"
F117 = f"{PANEL}/117_dispatcher_note_the_bench_profile_and_the_tests_that_cannot_run.md"
F114 = f"{PANEL}/114_leroy_formalising_the_primitive.md"
F112 = f"{PANEL}/112_leijen_where_the_refinement_lives.md"
F108 = f"{PANEL}/108_lamport_the_pair_attacked.md"
F106 = f"{PANEL}/106_giesen_consolidation_the_strategy_axis.md"
P9 = f"{PANEL}/114_probes/p9_pricing_the_two_spellings_without_a_clock.py"
WC = f"{BENCH}/variants/warm-clamp-shared/src/lib.rs"

CITES = [
    # ---- op's intents
    (f"{PANEL}/INTENTS.md", 214,
     "I13. The work is predicated arms composed",
     "I13 is the entry the repair shape is decided under"),

    # ---- 115: what it concluded, conceded and measured
    (F115, 62,
     "aim the predicate at the check the overflow policy selects",
     "115's repair, stated in its own section 1"),
    (F115, 120,
     "F111-15 claimed soundness at `overflow policy in {sat, wrap}` and it does not hold at",
     "115 concedes F114-3 outright"),
    (F115, 183,
     "it discards the wrap half",
     "and declines 114's repair on that ground"),
    (F115, 332,
     "F115-1. The structural predicate is sound as a certificate of the check the overflow policy",
     "F115-1 is the conjunction against the policy-selected check"),
    (F115, 339,
     "F115-2. Condition (b) is not load-bearing under wrap with the root check",
     "F115-2 is the half 118 section 2.3 folds into it"),
    (F115, 344,
     "F115-3. The root check is unsound at `sat`",
     "F115-3 is the control that makes the selection a requirement"),
    (F115, 349,
     "F115-4. A trait projection selecting the rule type does not escape",
     "F115-4 is the finding 118 section 8.2 corrects the provenance of"),
    (F115, 215,
     "including a route `114` did not try",
     "and that is the claim about 114 which is false"),
    (F115, 357,
     "F115-5. The certificate is computable from the cheap carrier alone",
     "F115-5 is the half 114 section 7 was missing"),
    (F115, 220,
     "a const gate choosing **which const is read**, and a selection choosing **which type is",
     "115 section 4.1's distinction, carried"),
    (F115, 409,
     "DAGs rather than trees",
     "115 depends on the same untouched assumption 114 does"),

    # ---- 116: the root, the consequences, the bounding
    (F116, 315,
     "No realisation map onto a finite value set is both a ring homomorphism and monotone",
     "F116-4's statement, the root of the convergence"),
    (F116, 555,
     "F116-4. No realisation map onto a finite value set is both",
     "and its finding entry"),
    (F116, 320,
     "monotone for some total order on `V`",
     "116 states monotonicity over SOME total order, which its probe does not test"),
    (F116, 563,
     "F116-5. The two licence families are disjoint per policy",
     "F116-5 closes 114 section 12's open question"),
    (F116, 570,
     "F116-6. A discharged declared extent restores both properties at once",
     "F116-6 is what corrects 114 section 6.4"),
    (F116, 575,
     "F116-7. At `F > 0` the map is a ring homomorphism for addition and subtraction",
     "F116-7 is the operation split, reproduced in q3"),
    (F116, 587,
     "F116-9. The multiplicative homomorphism at `F > 0` is restored exactly when the operands",
     "F116-9 is the unit-grid condition, reproduced in q3"),
    (F116, 546,
     "F116-3. My F112-24's domination holds over every declaration with a non-negative lower bound",
     "F116-3 is carried unreproduced"),
    (F116, 152,
     "an offered canon sentence is a finding and carries a",
     "116's sharper version of 114 section 3.2's lesson"),
    (F116, 359,
     "is conservative rather than load-bearing",
     "116 section 5.3 quotes arm W1's cost clause"),
    (F116, 363,
     "the policy is not free",
     "and prices it across the design"),
    (F116, 353,
     "It gives `115`'s conclusion its mechanism",
     "116 states it read 115 only after its own probe ran"),
    (F116, 455,
     "**W1b.** `F` any, operations in `{add, sub}`",
     "W1b's prose says F any"),
    (F116, 462,
     "F in {0, 1, 2}",
     "while its block predicate says F in {0, 1, 2}"),

    # ---- 117
    (F117, 35,
     "false green",
     "117 names the defect as a false green rather than an untested surface"),

    # ---- 114, including the route 115 says was not tried
    (P9, 244,
     '"selection-assoc"',
     "114's p9 defines the trait-projection variant"),
    (P9, 256,
     "as Pick>::Arm as Chk>::OK",
     "and its site reads through the projection"),
    (F114, 884,
     "selection-assoc",
     "114 reports it in the compile table"),
    (F114, 913,
     "routes the choice through an associated type",
     "and describes it in prose"),
    (F114, 1124,
     "selection-assoc",
     "and lists it in F114-17"),

    # ---- the pair's own definition, for the resolver-side precision
    (F108, 825,
     "An axis belongs here if there is **any** reachable chain",
     "108's membership criterion, which puts the policy in the declared semantics"),
    (F112, 1002,
     "F112-2. The direction count classifies an axis",
     "F112-2 measures the policy's placement"),
    (F112, 1009,
     "F112-3. Under a discharged magnitude bound",
     "and F112-3 measures it from the consumer side"),
    (F106, 281,
     "TWO EXPERTS requires two",
     "106 section 3 states the rung definitions used in section 4.2"),

    # ---- the shipped kernel
    (WC, 159,
     "w + ceil_log2(arity)",
     "the shipped guard's formula"),
    (WC, 291,
     "let safe = accumulator_bits_needed(W, ARITY) <= A::BITS",
     "and the const predicate that reads it"),
    (WC, 288,
     "pub fn fold_chunked",
     "in the kernel 114 section 6.4 names"),

    # ---- NEGATIVE CONTROLS: both must FAIL
    (F115, 332,
     "condition (a) alone",
     "CONTROL, MUST FAIL: F115-1 states the conjunction, not condition (a) alone"),
    (F116, 555,
     "except an additive one",
     "CONTROL, MUST FAIL: F116-4 says except a constant one"),
]


def norm(s):
    return re.sub(r"\s+", " ", s).strip()


def unquote(line):
    """Strip a markdown blockquote marker so a quotation wrapped across two
    blockquote lines joins into the text it actually is. Without this a needle
    spanning a line break sees a stray '>' in the middle of it, which is how
    both of this checker's first-run failures happened."""
    return re.sub(r"^\s*>\s?", "", line)


def check(path, line, needle):
    try:
        with open(path) as f:
            lines = f.read().splitlines()
    except OSError as e:
        return False, f"cannot open: {e}"
    if line < 1 or line > len(lines):
        return False, f"line {line} out of range (file has {len(lines)})"
    lo, hi = max(0, line - 2), min(len(lines), line + 2)
    window = norm(" ".join(unquote(x) for x in lines[lo:hi]))
    return norm(needle) in window, window[:140]


def main():
    print("q7. Every citation in `118`, opened")
    print("=" * 78)
    print()
    ok = bad = c_ok = c_bad = 0
    for path, line, needle, claim in CITES:
        good, ctx = check(path, line, needle)
        short = os.path.relpath(path, PANEL if path.startswith(PANEL) else ARVO)
        if claim.startswith("CONTROL"):
            if good:
                c_bad += 1
                print(f"  CONTROL DID NOT FAIL  {short}:{line}")
                print(f"      {claim}")
            else:
                c_ok += 1
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
    print(f"  controls that correctly failed : {c_ok} of 2")
    print(f"  controls that wrongly passed   : {c_bad}")
    print()
    if c_ok != 2:
        print("  The controls did not both fail. The checker matches too loosely and")
        print("  every pass above is worth nothing.")
        return 2
    if bad:
        print("  Citations are wrong. Fix the file, not the checker.")
        return 1
    print("  Every citation opens and says what the claim depends on, and both")
    print("  deliberately wrong entries were caught.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
