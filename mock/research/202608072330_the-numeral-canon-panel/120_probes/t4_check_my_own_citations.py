#!/usr/bin/env python3
"""T4. Open every file:line `120` leans on and test the substring the claim
depends on, with whitespace normalised because a quotation wrapped across two
source lines is still verbatim.

Two deliberately wrong entries are controls. A run in which nothing fails and no
control fails would mean the checker matches anything.
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
F119 = os.path.join(PANEL, "119_leroy_the_canon_candidate_for_the_realisation_map.md")
F115 = os.path.join(PANEL, "115_jhala_the_check_the_policy_selects.md")

CITES = [
    (F119, 505, "discharge check = root under a homomorphism and per node otherwise",
     "4.7's check phrase, wider than mine and bounded by its own predicate"),
    (F119, 684, "Section 4.7 states the certificate as two condition sets rather than one",
     "the note to me that 4.7 reshapes my finding"),
    (F119, 311, "Superseded by `115:182-187`'s shape",
     "where my re-aiming is credited, in the retirement list"),
    (F119, 566, "proportional to the derivation's size times the",
     "the mechanism sentence t3 refutes"),
    (F119, 568, "it is a configurable limit rather than a hard ceiling",
     "the prose that knows the limit matters while the predicate omits it"),
    (F119, 559, "computable from the cheaper carrier and from the derivation's syntax alone",
     "the clause that drops my hedge"),
    (F119, 446, "the only mechanism that makes both families available",
     "116's stronger sentence, raised and not pressed"),
    (F119, 698, "is where I am most likely to have flattered the sitting",
     "the candidate's own warning about the rung column"),
    (F119, 164, "measured it alone at wrapping, 9408 and 1200 firing at zero violations",
     "A9's record of my F115-2"),
    (F119, 92, "`115` F115-3 at 38 and 34",
     "A2's record of my F115-3 as a reproduction"),
    (F115, 322, "an inference from both conditions being syntactic",
     "my own hedge, which 119:559 states flat"),
    # ---- controls, both expected to FAIL
    (F119, 100, "proportional to the derivation's size times the",
     "CONTROL: right substring, wrong line"),
    (F115, 322, "measured on the bench harness at every fold length",
     "CONTROL: right line, substring that is not there"),
]

CONTROLS = 2


def main():
    fails = []
    for path, line, needle, claim in CITES:
        with open(path, encoding="utf-8") as fh:
            lines = fh.readlines()
        lo, hi = max(0, line - 2), min(len(lines), line + 1)
        window = " ".join("".join(lines[lo:hi]).split())
        ok = " ".join(needle.split()) in window
        if not ok:
            fails.append(claim)
        print(f"{'ok  ' if ok else 'FAIL'}  {os.path.basename(path):<52}:{line:<5} {claim}")

    control_fails = sum(1 for c in fails if c.startswith("CONTROL"))
    real_fails = len(fails) - control_fails
    print()
    print(f"{len(CITES) - CONTROLS} real citations checked, {real_fails} failing")
    print(f"{CONTROLS} controls included, {control_fails} failed as they must")
    return 1 if (real_fails or control_fails != CONTROLS) else 0


if __name__ == "__main__":
    sys.exit(main())
