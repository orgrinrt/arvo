#!/usr/bin/env python3
"""P9. Open every file:line this file cites and test its CONTENT, not that it
resolves.

`RULES.md` records that one member found seven of its own citations wrong this
way, and that a reference that resolves is not a reference that says what you
claim. Every entry below names the citation, the substring the claim depends on,
and whether it is there. Substrings are matched against the cited line and its
two neighbours, because a claim about a sentence can be about a sentence that
wraps.
"""

import os
import sys

PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ARVO = os.path.dirname(os.path.dirname(os.path.dirname(PANEL)))
BENCH = os.path.join(ARVO, "mock", "benches", "variants")

CITES = [
    # (path, line, substring the claim depends on, what the claim is)
    (f"{PANEL}/110_willsey_the_primitive_derived_cold.md", 282,
     "costs names and nothing else",
     "110 prices a split at nothing"),
    (f"{PANEL}/110_willsey_the_primitive_derived_cold.md", 357,
     "a missed merge is a wall",
     "110 prices the same act as a wall"),
    (f"{PANEL}/110_willsey_the_primitive_derived_cold.md", 370,
     "there is no repair",
     "110 says the wall has no in-language repair"),
    (f"{PANEL}/110_willsey_the_primitive_derived_cold.md", 540,
     "is a soundness statement, not a cost statement",
     "110 half-notices the tension itself"),
    (f"{PANEL}/110_probes/p2_laws_are_a_projection_not_a_coordinate.py", 192,
     '[3, 4], [0, 1, 2], [False, True], ["sat", "wrap"], ["near", "trunc"]',
     "the sweep is a product over exactly five axes"),
    (f"{PANEL}/110_probes/p2_laws_are_a_projection_not_a_coordinate.py", 52,
     "return (self.W, self.F, self.signed, self.policy, self.rounding, self.radix)",
     "the key is those five plus a constant radix"),
    (f"{PANEL}/110_probes/p5_definitional_versus_reachability_degeneracy.py", 99,
     'SIG_CLOSED = ["add", "sub", "mul", "neg"]',
     "110's grid-closed signature has no nullary operation"),
    (f"{PANEL}/110_probes/p5_definitional_versus_reachability_degeneracy.py", 102,
     'SIG_WIDE = ["add", "sub", "mul", "neg", "half", "recip", "fma"]',
     "nor does its widest one"),
    (f"{PANEL}/82_jhala_lifting_a_measured_region_into_a_declaration.md", 770,
     "on a declared operand window matches",
     "82 F6 is about a declared operand window"),
    (f"{PANEL}/82_jhala_lifting_a_measured_region_into_a_declaration.md", 899,
     "F6 separates two verdicts with",
     "82 says the window separates verdicts with everything else fixed"),
    (f"{PANEL}/18_jhala_the_denotation_clause.md", 363,
     "quantified over",
     "18 states soundness is quantified over an operation set"),
    (f"{PANEL}/35_mcsherry_what_the_layers_above_need_from_the_numeral.md", 63,
     "FromConstant",
     "35 records the old tree's algorithm crates bounding on FromConstant"),
    (f"{BENCH}/satfold-shared/src/lib.rs", 519,
     "pub const fn saturating_add_is_associative_at",
     "a shipped const fn computing a law with the operation inline"),
    (f"{BENCH}/satfold-shared/src/lib.rs", 547,
     "pub const fn saturating_sub_is_associative_at",
     "and its negative control, written out a second time"),
    (f"{BENCH}/satfold-shared/src/lib.rs", 1116,
     "the_one_element_defect_is_caught_up_to_1024_and_not_above_it",
     "a test asserting its own suite's sensitivity boundary"),
    (f"{BENCH}/quantiser-radix-shared/src/lib.rs", 370,
     "assert_eq!(p % 2, 1",
     "the first of the two redundant assertions"),
    (f"{BENCH}/quantiser-radix-shared/src/lib.rs", 372,
     "assert!(p % 2 == 1)",
     "the second, which restates it"),
    (f"{BENCH}/bitpack-write-contend-shared/src/stress.rs", 68,
     "#[test]",
     "the doc-comment occurrence that made two counts read 124"),
]


def main():
    width = max(len(os.path.basename(p)) for p, _, _, _ in CITES)
    fails = 0
    for path, line, needle, claim in CITES:
        try:
            with open(path, encoding="utf-8") as fh:
                lines = fh.readlines()
        except OSError as e:
            print(f"FAIL  {os.path.basename(path)}:{line}  cannot open: {e}")
            fails += 1
            continue
        lo = max(0, line - 2)
        hi = min(len(lines), line + 1)
        window = "".join(lines[lo:hi])
        ok = needle in window
        if not ok:
            fails += 1
        print(f"{'ok  ' if ok else 'FAIL'}  "
              f"{os.path.basename(path):<{width}}:{line:<5}  {claim}")
    print()
    print(f"{len(CITES)} citations checked, {fails} failing")
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
