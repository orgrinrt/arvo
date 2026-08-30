#!/usr/bin/env python3
"""u6. Every file:line `122` cites, opened, plus one check the earlier checkers
in this panel do not make.

The citation half is `119_probes/r2`'s instrument, which is `118`'s, which is
`114`'s, which is `111`'s. Reused.

The second half is new and exists because of what `u5` caught. This file is a
revision that restates some clauses and carries the rest by reference, and a
citation checker cannot see the failure that shape invites: a **restated** clause
whose evidence anchor was left behind in the document it supersedes. The first
draft had 22 of those and no checker in this panel would have reported one,
because every citation it did make was correct.

So the second half asserts, per restated clause, that the finding ids that clause
rests on are present somewhere in the file. It fails on an absence rather than on
a wrong reference, which is the failure this document's shape actually has.

NEGATIVE CONTROLS
-----------------
C1. Three deliberately wrong citations: a wrong substring, a wrong line, a wrong
    file. All three must fail.
C2. One deliberately impossible evidence requirement, naming a finding id that
    exists nowhere in the panel. It must be reported missing, or the second half
    is not checking anything.
"""

import os
import re
import sys

PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ARVO = os.path.dirname(os.path.dirname(os.path.dirname(PANEL)))
BENCH = os.path.join(ARVO, "mock", "benches")

I = f"{PANEL}/INTENTS.md"
F115 = f"{PANEL}/115_jhala_the_check_the_policy_selects.md"
F116 = f"{PANEL}/116_leijen_reply_what_the_homomorphism_opens.md"
F119 = f"{PANEL}/119_leroy_the_canon_candidate_for_the_realisation_map.md"
F120 = f"{PANEL}/120_jhala_partial_signature_on_the_candidate.md"
F121 = f"{PANEL}/121_leijen_partial_signature_the_domain_is_the_missing_dimension.md"
RULES = f"{PANEL}/RULES.md"
WC = f"{BENCH}/variants/warm-clamp-shared/src/lib.rs"
SELF = f"{PANEL}/122_leroy_the_candidate_revised_against_two_partial_signatures.md"

CITES = [
    (I, 214, "I13. The work is predicated arms composed",
     "I13 is the standard both dissents are made under"),

    (F119, 566, "proportional to the derivation's size",
     "the mechanism sentence 120 dissents on"),
    (F119, 559, "computable from the cheaper carrier and from the derivation's syntax",
     "the clause that dropped 115's hedge"),
    (F119, 308, "Framings retired",
     "where 119 filed the re-aiming, which is what 120's missing entry is about"),

    (F115, 322, "an inference from both conditions being syntactic, not a",
     "115 marks the syntax half as an inference"),
    (F116, 247, "arrived at S2 from the mechanism rather than from their file",
     "116's own claim about its reasoning, which 121 corrects against itself"),
    (RULES, 262, "agreement inherited by reading",
     "the standard 121 applies to A13"),

    (F120, 55, "attributes a multiplicative law to a recursion depth",
     "120's first dissent"),
    (F120, 108, "recursion limit",
     "120's second dissent, the dropped dimension"),
    (F120, 176, "Aimed at the check the character selects",
     "120's proposed ledger entry"),

    (F121, 69, "min(v, 15)",
     "121's counterexample to 4.2's predicate"),
    (F121, 137, "domain closed under negation",
     "121's replacement condition"),
    (F121, 191, "0/2116",
     "116's own table, which 121 read as a finding"),
    (F121, 243, "cannot carry an exact result",
     "121's replacement clause for 4.4"),
    (F121, 258, "semiring",
     "121 names the third structure"),

    (WC, 1105, "clamping_is_a_retraction_on_non_negative_addition_at_every_swept_width",
     "the shipped test the 4.4 dissent rests on"),
    (WC, 1113, "if s > l",
     "and its eager fold, read at source"),

    # ---- CONTROLS, all must fail
    (F121, 69, "min(v, 31)",
     "CONTROL, MUST FAIL: the counterexample clamps at 15"),
    (F116, 999999, "anything",
     "CONTROL, MUST FAIL: the line does not exist"),
    (I, 214, "I13. The work is a universal solution",
     "CONTROL, MUST FAIL: I13 rejects a universal"),
]

# Per restated clause, the finding ids that clause rests on.
EVIDENCE = {
    "4.2 replaced": ["F116-4", "F121-1", "F122-2", "F122-3"],
    "4.3 amended": ["F116-5"],
    "4.4 replaced": ["F114-1", "F114-7", "F116-7", "F116-9", "F118-7",
                     "F121-4", "F122-4"],
    "4.5 amended": ["F116-6", "F122-6"],
    "4.6 amended": ["F114-19", "F118-11", "F118-12", "F118-13"],
    "4.7 amended": ["F114-4", "F114-5", "F114-6", "F118-1", "F122-5"],
    "4.8 stands": ["F114-11"],
    "4.9 stands": ["F114-12", "F116-10"],
    "4.10 amended": ["F114-17", "F114-18", "F115-4", "F115-5", "F122-1"],
    "A2 as confirmed": ["F112-21"],
    "A13 corrected": ["F112-24", "F114-10", "F114-11", "F116-3"],
    "A18 new": ["F115-1"],
    "A19 new": ["F121-4", "F121-5", "F122-5"],
    "A20 new": ["F122-6"],
    "section 7 withdrawals": ["F118-4", "F118-5", "F118-6"],
    "C2 CONTROL, must be missing": ["F999-1"],
}


def norm(s):
    return re.sub(r"\s+", " ", s).strip()


def unquote(line):
    return re.sub(r"^\s*>\s?", "", line)


def check(path, line, needle):
    try:
        lines = open(path).read().splitlines()
    except OSError as e:
        return False, f"cannot open: {e}"
    if line < 1 or line > len(lines):
        return False, f"line {line} out of range (file has {len(lines)})"
    lo, hi = max(0, line - 3), min(len(lines), line + 3)
    return norm(needle) in norm(" ".join(unquote(x) for x in lines[lo:hi])), ""


def main():
    print("u6. Every citation in `122`, and every restated clause's evidence")
    print("=" * 78)
    print()
    ok = bad = c_ok = c_bad = 0
    for path, line, needle, claim in CITES:
        good, _ = check(path, line, needle)
        short = os.path.relpath(path, PANEL if path.startswith(PANEL) else ARVO)
        if claim.startswith("CONTROL"):
            if good:
                c_bad += 1
                print(f"  CONTROL DID NOT FAIL  {short}:{line}  {claim}")
            else:
                c_ok += 1
        elif good:
            ok += 1
        else:
            bad += 1
            print(f"  FAIL  {short}:{line}")
            print(f"      claim:  {claim}")
            print(f"      wanted: {needle}")

    print(f"  citations checked : {ok + bad}")
    print(f"  passing           : {ok}")
    print(f"  failing           : {bad}")
    print(f"  controls that correctly failed : {c_ok} of 3")
    print()

    text = open(SELF).read()
    have = set(re.findall(r"\bF\d{2,3}-\d{1,3}\b", text)) | set(re.findall(r"\bF\d{1,2}\b", text))
    print("  Evidence present, per restated clause:")
    print()
    ev_bad = ctrl_bad = 0
    for clause, ids in EVIDENCE.items():
        missing = [i for i in ids if i not in have]
        is_ctrl = clause.startswith("C2 CONTROL")
        if is_ctrl:
            if not missing:
                ctrl_bad += 1
                print(f"    CONTROL DID NOT FIRE  {clause}: {ids} all present")
            else:
                print(f"    control fired as it must  {clause}: missing {missing}")
            continue
        if missing:
            ev_bad += 1
            print(f"    MISSING  {clause:<28} {missing}")
        else:
            print(f"    ok       {clause:<28} {len(ids)} anchors present")
    print()
    print(f"  restated clauses whose evidence is incomplete : {ev_bad}")
    print()
    if c_ok != 3 or ctrl_bad:
        print("  A control did not fail. The checker matches too loosely.")
        return 2
    if bad or ev_bad:
        print("  Fix the document, not the checker.")
        return 1
    print("  Every citation opens and says what the claim depends on, every restated")
    print("  clause carries the evidence it rests on, and every control failed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
