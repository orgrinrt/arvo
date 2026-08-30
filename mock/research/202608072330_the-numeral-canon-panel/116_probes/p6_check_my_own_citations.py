#!/usr/bin/env python3
"""
p6. Open every file:line this reply leans on and test its content.

`RULES.md`: a reference that resolves is not a reference that says what you
claim.  In `112` three of my citations were wrong on the first run of the
equivalent probe, so this is not ceremony.

Each row is (path, line, substring the claim depends on).  A row passes when
the substring appears on the cited line or within two lines either side, which
tolerates a one-line drift without tolerating a wrong file or a wrong section.
Three mutations at the bottom confirm the instrument fails when it should.
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

CITES = [
    # my own sentence, the one under attack
    ("112_leijen_where_the_refinement_lives.md", 928,
     "Checking only the derivation's result rather than every node is unsound"),
    # my own finding, correctly predicated, which is why nothing was mismeasured
    ("112_leijen_where_the_refinement_lives.md", 1116, "overflow policy = sat"),
    ("112_leijen_where_the_refinement_lives.md", 1113,
     "A root-only range check is unsound"),
    # my own findings the reply re-predicates
    ("112_leijen_where_the_refinement_lives.md", 1127,
     "dominates both the interval rule"),
    ("112_leijen_where_the_refinement_lives.md", 1142,
     "Every declared extent in every sweep of this file is one-sided"),
    ("112_leijen_where_the_refinement_lives.md", 1080,
     "The smallest sound transformer differs per construction"),
    ("112_leijen_where_the_refinement_lives.md", 1016,
     "switches off the completion and not the rounding"),
    ("112_leijen_where_the_refinement_lives.md", 1067,
     "discharges the interval construction's base predicate"),
    # 114, the file being answered
    ("114_leroy_formalising_the_primitive.md", 1010,
     "A wrapping realisation map is a ring homomorphism"),
    ("114_leroy_formalising_the_primitive.md", 283,
     "Checking only the derivation's result rather than every node is unsound"),
    ("114_leroy_formalising_the_primitive.md", 288,
     "own finding is correctly predicated"),
    ("114_leroy_formalising_the_primitive.md", 558,
     "domination is a fact about one-sided declarations"),
    ("114_leroy_formalising_the_primitive.md", 654,
     "it is a formula rather than a table"),
    ("114_leroy_formalising_the_primitive.md", 663,
     "L1 norm"),
    ("114_leroy_formalising_the_primitive.md", 343,
     "Under a homomorphism, check the root"),
    ("114_leroy_formalising_the_primitive.md", 588,
     "changes the propagation question qualitatively"),
    ("114_leroy_formalising_the_primitive.md", 702,
     "This covers **bilinear** constructions"),
    # 110, the composite finding the duality composes with
    ("110_willsey_the_primitive_derived_cold.md", 523,
     "closed exactly on monotone bases"),
    # 115, the sibling reply
    # `115` is live while this is written, so it is cited by SECTION rather
    # than by line in the reply itself; this row checks the heading exists.
    ("115_jhala_the_check_the_policy_selects.md", 57,
     "What this file concludes, stated first"),
    ("115_jhala_the_check_the_policy_selects.md", 62,
     "aim the predicate at the check the overflow policy"),
    # op
    ("113_op_steer_them_and_make_them_build_it_together.md", 1,
     "make them build it together rather than stopping at the refutation"),
    ("INTENTS.md", 214, "predicated arms composed"),
    ("INTENTS.md", 299, "Never a runtime check"),
]


def check(rows, quiet=False):
    fails = 0
    for path, line, sub in rows:
        p = ROOT / path
        if not p.exists():
            if not quiet:
                print(f"  FAIL  {path}:{line}  file does not exist")
            fails += 1
            continue
        text = p.read_text(errors="replace").splitlines()
        lo, hi = max(0, line - 3), min(len(text), line + 2)
        if sub in "\n".join(text[lo:hi]):
            if not quiet:
                print(f"  ok    {path}:{line}  {sub[:52]!r}")
        else:
            if not quiet:
                print(f"  FAIL  {path}:{line}  {sub[:52]!r} not within +/-2 lines")
            fails += 1
    return fails


def main():
    print("=" * 84)
    print("p6. Citation check")
    print("=" * 84)
    print()
    fails = check(CITES)
    print()
    print(f"  {len(CITES)} citations checked, {fails} failing")
    print()
    print("MUTATIONS (each must add exactly one failure)")
    a = list(CITES); a[0] = (a[0][0], a[0][1] + 70, a[0][2])
    b = list(CITES); b[1] = (b[1][0], b[1][1], "a phrase that is nowhere in the file")
    c = list(CITES); c[2] = ("110_willsey_the_primitive_derived_cold.md", c[2][1], c[2][2])
    n1, n2, n3 = check(a, True), check(b, True), check(c, True)
    print(f"  wrong line number : {n1} (expected {fails + 1})")
    print(f"  wrong substring   : {n2} (expected {fails + 1})")
    print(f"  wrong file        : {n3} (expected {fails + 1})")
    ok = n1 == fails + 1 and n2 == fails + 1 and n3 == fails + 1
    print()
    print(f"  the instrument fails when it should: {ok}")
    sys.exit(0 if fails == 0 and ok else 1)


if __name__ == "__main__":
    main()
