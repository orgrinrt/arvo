#!/usr/bin/env python3
"""r2. Open every file:line `119` cites and test its CONTENT.

Same instrument as `118_probes/q7`, which is `114_probes/p10`'s, which is
`111_probes/p9`'s. Reused rather than rebuilt.

A canon candidate keeps a line anchor only where the claim is about a document's
exact wording, so every anchor here is load-bearing by construction: op's
intents, the two sentences section 1.3 retires, the two places one file's prose
and predicate differ, and the concessions. If one of them does not say what the
candidate claims, a retirement or a concession is misattributed, which is the
worst class of error a compression can make.

Blockquote markers are stripped before joining lines, because a quotation
wrapped across two blockquote lines is still verbatim and that has produced a
false failure in three of this panel's four checkers.

NEGATIVE CONTROLS
-----------------
Three deliberately wrong entries, marked, which must all FAIL: a wrong
substring, a wrong line, and a wrong file. If any passes, the checker is
matching too loosely and its passes mean nothing.
"""

import os
import re
import sys

PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

I = f"{PANEL}/INTENTS.md"
F108 = f"{PANEL}/108_lamport_the_pair_attacked.md"
F112 = f"{PANEL}/112_leijen_where_the_refinement_lives.md"
F115 = f"{PANEL}/115_jhala_the_check_the_policy_selects.md"
F116 = f"{PANEL}/116_leijen_reply_what_the_homomorphism_opens.md"
F117 = f"{PANEL}/117_dispatcher_note_the_bench_profile_and_the_tests_that_cannot_run.md"
P9 = f"{PANEL}/114_probes/p9_pricing_the_two_spellings_without_a_clock.py"

CITES = [
    # ---- op's intents, which govern section 0.1 and the whole of section 4
    (I, 214, "I13. The work is predicated arms composed",
     "I13 is the entry section 4's shape is decided under"),
    (I, 240, "unmeasured or unknown does not list in the predicate",
     "op's instruction that unmeasured dimensions go unstated"),
    (I, 252, "collapses to whatever is available at const time",
     "op's instruction on what a const predicate reaches"),
    (I, 322, "We shouldn't police what kind of laws there are",
     "I16, why section 4 states arms rather than a winner"),

    # ---- the definition the placement measurements are read against
    (F108, 825, "An axis belongs here if there is **any** reachable chain",
     "108's membership criterion for the declared semantics"),

    # ---- the two placement measurements
    (F112, 1002, "F112-2. The direction count classifies an axis",
     "F112-2 places the overflow behaviour by direction count"),
    (F112, 1009, "F112-3. Under a discharged magnitude bound",
     "F112-3 places it from the consumer side"),

    # ---- the sentence section 1.3 retires, and its correctly predicated source
    (F112, 928,
     "Checking only the derivation's result rather than every node is unsound",
     "the unqualified sentence 1.3 retires"),
    (F112, 1116, "overflow policy = sat",
     "and F112-21's own predicate, which is the thing to cite instead"),

    # ---- the concessions
    (F115, 120,
     "F111-15 claimed soundness at `overflow policy in {sat, wrap}` and it does not hold at",
     "115 concedes the refutation of its own finding"),
    (F115, 183, "it discards the wrap half",
     "and declines 114's repair on that ground, which 1.3 retires as a framing"),
    (F116, 200, "beaten on none",
     "116 concedes its own domination claim"),
    (F116, 274, "Take the formula",
     "116 declares its own table superseded"),
    (F116, 142, "has no predicate at all",
     "116 concedes the unqualified sentence"),

    # ---- the two places 116's prose and its predicate differ, which is C2
    (F116, 455, "**W1b.** `F` any, operations in `{add, sub}`",
     "W1b's prose says any fraction width"),
    (F116, 462, "F in {0, 1, 2}",
     "while its predicate says three of them"),

    # ---- 116's theorem and its stated monotonicity hypothesis
    (F116, 315,
     "No realisation map onto a finite value set is both a ring homomorphism and monotone",
     "the theorem section 4.2 states"),
    (F116, 320, "monotone for some total order on `V`",
     "and its monotonicity hypothesis, quantified over some order"),

    # ---- the located precision at C1
    (F116, 360, "is conservative rather than load-bearing",
     "116 quotes the cost clause C1 is about"),
    (F116, 363, "the policy is not free",
     "and prices it across the design"),

    # ---- what would decide C4, in both files that depend on it
    (F115, 409, "DAGs rather than trees",
     "115 names the same untouched assumption 114 does"),

    # ---- 117's correction, which section 0.2 accepts
    (F117, 35, "false green",
     "117 names the defect as a false green rather than an untested surface"),
    (F117, 27, "That was wrong, and it was wrong in the direction that overstates",
     "and 117 corrects its own first version, which 1.3 records"),

    # ---- the provenance correction 1.3 carries
    (P9, 244, '"selection-assoc"',
     "114's p9 does define the route 115 reports as untried"),

    # ---- NEGATIVE CONTROLS: all three must FAIL
    (F116, 315, "except an additive one",
     "CONTROL, MUST FAIL: the theorem says except a constant one"),
    (F112, 1116, "overflow policy = wrap",
     "CONTROL, MUST FAIL: F112-21's predicate lists sat, not wrap"),
    (F115, 999999, "anything at all",
     "CONTROL, MUST FAIL: the line does not exist"),
]


def norm(s):
    return re.sub(r"\s+", " ", s).strip()


def unquote(line):
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
    print("r2. Every citation in `119`, opened")
    print("=" * 78)
    print()
    ok = bad = c_ok = c_bad = 0
    for path, line, needle, claim in CITES:
        good, ctx = check(path, line, needle)
        short = os.path.relpath(path, PANEL)
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
    print(f"  controls that correctly failed : {c_ok} of 3")
    print(f"  controls that wrongly passed   : {c_bad}")
    print()
    if c_ok != 3:
        print("  The controls did not all fail. The checker matches too loosely and")
        print("  every pass above is worth nothing.")
        return 2
    if bad:
        print("  Citations are wrong. Fix the candidate, not the checker.")
        return 1
    print("  Every citation opens and says what the claim depends on, and all three")
    print("  deliberately wrong entries were caught.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
