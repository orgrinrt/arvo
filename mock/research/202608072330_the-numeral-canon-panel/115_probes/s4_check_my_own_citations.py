#!/usr/bin/env python3
"""S4. Open every file:line `115` leans on and test the substring the claim
depends on, rather than testing whether the reference resolves.

Two deliberately wrong entries are included as controls. A run in which
everything passes and no control fails would mean the checker matches anything.
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
ARVO = os.path.dirname(os.path.dirname(os.path.dirname(PANEL)))
WC = os.path.join(ARVO, "mock", "benches", "variants", "warm-clamp-shared",
                  "src", "lib.rs")

F114 = os.path.join(PANEL, "114_leroy_formalising_the_primitive.md")
F112 = os.path.join(PANEL, "112_leijen_where_the_refinement_lives.md")
F111 = os.path.join(PANEL, "111_jhala_the_primitive_attacked.md")

CITES = [
    (WC, 288, "pub fn fold_chunked", "the shipped chunked fold"),
    (WC, 291, "let safe = accumulator_bits_needed(W, ARITY) <= A::BITS",
     "the const predicate selecting the two arms"),
    (WC, 296, "if safe {", "the wrapping branch"),
    (WC, 301, "acc = acc.min_with(limit);", "reduced once at the end of the chunk"),
    (WC, 158, "pub const fn accumulator_bits_needed(w: u32, arity: usize) -> u32",
     "the predicate 114 identifies as the corner rule at a fold's root"),
    (WC, 159, "w + ceil_log2(arity)", "and its body"),
    (F114, 916, "obligation is forced when the reading code is **defined**",
     "114's central mechanism, which s2 reproduces"),
    (F114, 921, "the predicate has to gate which type is constructed",
     "114's corrected form of my section 21"),
    (F114, 1021, "F114-3. `111` F111-15's structural predicate is unsound at",
     "the finding against me"),
    (F114, 1043, "F114-6. Under a homomorphic map the root-only check is sound",
     "the arm whose figures my repair reproduces"),
    (F114, 1121, "F114-17. The compile-side cost of the affine tower is trait-solving",
     "the compile-time refutation"),
    (F114, 1132, "F114-18. The affine tower's compile-side quantity is the leaf count",
     "the state-count correction"),
    (F114, 1107, "F114-15. The structural predicate and the policy-selected discharge",
     "114's own compiled predicate claim"),
    (F114, 1010, "F114-1. A wrapping realisation map is a ring homomorphism",
     "the mechanism the repair rests on"),
    (F114, 1069, "F114-10. `112` F112-24's domination is a fact about one-sided",
     "the bound on 112's rule, which I keep untested"),
    (F114, 276, "F111-15 restated at `overflow policy = sat` is",
     "the repair 114 offers and I decline"),
    (F114, 594, "If a real consumer's terms are not trees",
     "114's least-comfortable assumption, which my section 8 leans on"),
    (F114, 1237, "claim that the overflow policy is the right axis to carry it",
     "what 114 says it most wants a second read on"),
    (F112, 1002, "F112-2. The direction count classifies an axis",
     "where the overflow policy is placed by measurement"),
    (F112, 1113, "F112-21. A root-only range check is unsound",
     "the finding whose own predicate lists sat"),
    (F111, 1383, "rule's exactness.** Zero violations over twelve rows",
     "F111-15, the finding refuted at wrap"),
    (F111, 1386, "discharge check = per node",
     "and the dimension its predicate fixes"),
    (F111, 1402, "F111-18. The two propagation rules differ in state",
     "the finding whose figure is corrected"),
    # ---- controls, both expected to FAIL
    (F114, 100, "F114-3. `111` F111-15's structural predicate is unsound at",
     "CONTROL: right substring, wrong line"),
    (F111, 1383, "the corner rule is exact on every term shape whatsoever",
     "CONTROL: right line, substring that is not there"),
]

CONTROLS = 2


def main():
    fails = []
    for path, line, needle, claim in CITES:
        try:
            with open(path, encoding="utf-8") as fh:
                lines = fh.readlines()
        except OSError as e:
            fails.append(claim)
            print(f"FAIL  {os.path.basename(path)}:{line}  cannot open: {e}")
            continue
        lo, hi = max(0, line - 2), min(len(lines), line + 1)
        # Normalise whitespace on both sides, because a quotation wrapped
        # across two source lines is still verbatim. 114 records hitting
        # exactly this and so did this checker on its first run.
        window = " ".join("".join(lines[lo:hi]).split())
        ok = " ".join(needle.split()) in window
        if not ok:
            fails.append(claim)
        print(f"{'ok  ' if ok else 'FAIL'}  {os.path.basename(path):<42}:{line:<5} {claim}")

    real = len(CITES) - CONTROLS
    control_fails = sum(1 for c in fails if c.startswith("CONTROL"))
    real_fails = len(fails) - control_fails
    print()
    print(f"{real} real citations checked, {real_fails} failing")
    print(f"{CONTROLS} controls included, {control_fails} of them failed as they must")
    return 1 if (real_fails or control_fails != CONTROLS) else 0


if __name__ == "__main__":
    sys.exit(main())
