#!/usr/bin/env python3
"""
t4. Open every file:line this signature leans on and test its content.

A dissent that misquotes the clause it dissents from is worse than no dissent.
Each row is (path, line, substring). A row passes when the substring appears
within two lines either side of the cited line. Three mutations confirm the
instrument fails when it should.
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

CITES = [
    # 119, the candidate
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 392,
     "both an additive homomorphism and order-preserving"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 397,
     "containing a complete residue system and the interval from zero"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 396,
     "operations including addition"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 401,
     "Multiplication is **not** required in the hypothesis"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 431,
     "A saturating map is a homomorphism for no operation"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 434,
     "overflow behaviour in {wrapping,"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 502,
     "The character split"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 105,
     "This is `116`'s claim and not a reproduction"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 202,
     "reaches the same repair from its own analysis"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 205,
     "one independent arrival without"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 690,
     "states your theorem without multiplication in the hypothesis"),
    ("119_leroy_the_canon_candidate_for_the_realisation_map.md", 697,
     "is where I am most likely to have flattered the sitting"),
    # 118, the ablation
    ("118_leroy_reply_one_mechanism_and_the_condition_set.md", 666,
     "Multiplication is not load-bearing"),
    ("118_leroy_reply_one_mechanism_and_the_condition_set.md", 671,
     "Addition is load-bearing, with witnesses"),
    ("118_leroy_reply_one_mechanism_and_the_condition_set.md", 677,
     "The domain's width is load-bearing"),
    ("118_leroy_reply_one_mechanism_and_the_condition_set.md", 657,
     "quantified over every total"),
    ("118_probes/q3_output.txt", 13, "720/2304"),
    # 116, mine
    ("116_leijen_reply_what_the_homomorphism_opens.md", 555,
     "both a ring homomorphism and monotone"),
    ("116_leijen_reply_what_the_homomorphism_opens.md", 247,
     "arrived at S2 from the mechanism rather than from their file"),
    ("116_probes/p4_output.txt", 13, "0/2116"),
    # the shipped test the clause contradicts
    ("../../benches/variants/warm-clamp-shared/src/lib.rs", 1105,
     "clamping_is_a_retraction_on_non_negative_addition"),
    # the rung standard
    ("RULES.md", 262, "agreement inherited by reading is not"),
    ("INTENTS.md", 214, "predicated arms composed"),
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
                print(f"  ok    {path}:{line}  {sub[:48]!r}")
        else:
            if not quiet:
                print(f"  FAIL  {path}:{line}  {sub[:48]!r} not within +/-2 lines")
            fails += 1
    return fails


def main():
    print("=" * 84)
    print("t4. Citation check")
    print("=" * 84)
    print()
    fails = check(CITES)
    print()
    print(f"  {len(CITES)} citations checked, {fails} failing")
    print()
    print("MUTATIONS (each must add exactly one failure)")
    a = list(CITES); a[0] = (a[0][0], a[0][1] + 80, a[0][2])
    b = list(CITES); b[1] = (b[1][0], b[1][1], "a phrase that is nowhere in the file")
    c = list(CITES); c[2] = ("RULES.md", c[2][1], c[2][2])
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
