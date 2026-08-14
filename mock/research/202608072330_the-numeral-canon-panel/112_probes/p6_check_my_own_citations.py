#!/usr/bin/env python3
"""
p6. Open every file:line this file leans on and test its content.

`RULES.md`: "Check your own citations before shipping, by opening them.  A
reference that resolves is not a reference that says what you claim."  One
member found seven of its own citations wrong this way.

Each row is (path, line, substring the claim depends on).  A row passes when
the substring appears on the cited line or on either of its two neighbours,
which tolerates a one-line drift without tolerating a wrong file or a wrong
section.

Because a green run proves nothing on its own, three mutations at the bottom
confirm the instrument fails when it should: a wrong line number, a wrong
substring and a wrong file each produce exactly one failure.
"""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

CITES = [
    # 110, the contradiction and the rule it draws
    ("110_willsey_the_primitive_derived_cold.md", 282, "costs names and nothing else"),
    ("110_willsey_the_primitive_derived_cold.md", 357, "rather than a slow path"),
    ("110_willsey_the_primitive_derived_cold.md", 370, "And there is no repair"),
    (
        "110_willsey_the_primitive_derived_cold.md",
        395,
        "not be a type parameter",
    ),
    ("110_willsey_the_primitive_derived_cold.md", 540, "is a soundness statement"),
    # 110, the composite results nobody had touched
    ("110_willsey_the_primitive_derived_cold.md", 348, "the predicate is monotonicity"),
    ("110_willsey_the_primitive_derived_cold.md", 350, "I was not looking for it"),
    (
        "110_willsey_the_primitive_derived_cold.md",
        524,
        "The interval construction is closed exactly on monotone bases",
    ),
    (
        "110_willsey_the_primitive_derived_cold.md",
        519,
        "componentwise product preserves its base",
    ),
    # 111, the answer and the gap it left
    ("111_jhala_the_primitive_attacked.md", 728, "An axis nothing reads has no repair"),
    (
        "111_jhala_the_primitive_attacked.md",
        951,
        "Asking whether the refinement is a strategy",
    ),
    ("111_jhala_the_primitive_attacked.md", 626, "largest sound declared bound"),
    (
        "111_jhala_the_primitive_attacked.md",
        837,
        "extent closed under the operations",
    ),
    (
        "111_jhala_the_primitive_attacked.md",
        844,
        "predicts the completion merge boundary exactly",
    ),
    (
        "111_jhala_the_primitive_attacked.md",
        993,
        "composite results at all",
    ),
    # 108, the clause my answer composes with
    ("108_lamport_the_pair_attacked.md", 825, "must agree about it"),
    (
        "108_lamport_the_pair_attacked.md",
        827,
        "not a reclassification of the axis",
    ),
    ("108_lamport_the_pair_attacked.md", 442, "fails"),
    ("108_lamport_the_pair_attacked.md", 298, "| 729 | 65536 | 701 | 28 |"),
    # 106, the pair as consolidated
    (
        "106_giesen_consolidation_the_strategy_axis.md",
        174,
        "cannot recover it from the bits",
    ),
    # 109, the two results I build on
    (
        "109_bellard_the_primitive_derived_cold.md",
        376,
        "perimeter is the construction site",
    ),
    ("109_bellard_the_primitive_derived_cold.md", 424, "is not an endomorphism"),
    # the intents
    ("INTENTS.md", 110, "Hot may sacrifice soundness"),
    ("INTENTS.md", 214, "predicated arms composed"),
    ("INTENTS.md", 299, "Never a runtime check"),
    ("INTENTS.md", 363, "Cold is not to be deprioritised"),
    # op's own files
    (
        "88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md",
        1,
        "there is never a universal answer",
    ),
    (
        "104_op_the_imitation_is_ergonomic_and_i9_is_not_his_to_settle.md",
        1,
        "I9 is not his to settle",
    ),
    (
        "95_op_the_panel_runs_to_ratification_and_units_must_converge.md",
        1,
        "a unit has to end in agreement",
    ),
    # the shipped const census 111 F111-1 names
    (
        "../../benches/variants/satfold-shared/src/lib.rs",
        519,
        "saturating_add_is_associative_at",
    ),
    (
        "../../benches/variants/satfold-shared/src/lib.rs",
        547,
        "saturating_sub_is_associative_at",
    ),
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
        window = "\n".join(text[lo:hi])
        if sub in window:
            if not quiet:
                print(f"  ok    {path}:{line}  {sub[:50]!r}")
        else:
            if not quiet:
                print(f"  FAIL  {path}:{line}  {sub[:50]!r} not in +/-2 lines")
            fails += 1
    return fails


def main():
    print("=" * 78)
    print("p6. Citation check")
    print("=" * 78)
    print()
    fails = check(CITES)
    print()
    print(f"  {len(CITES)} citations checked, {fails} failing")

    print()
    print("MUTATIONS (each must produce exactly one failure)")
    print()
    bad_line = list(CITES)
    bad_line[0] = (bad_line[0][0], bad_line[0][1] + 60, bad_line[0][2])
    n1 = check(bad_line, quiet=True)
    print(f"  wrong line number  : {n1} failures (expected {fails + 1})")

    bad_sub = list(CITES)
    bad_sub[1] = (bad_sub[1][0], bad_sub[1][1], "a phrase that is not there at all")
    n2 = check(bad_sub, quiet=True)
    print(f"  wrong substring    : {n2} failures (expected {fails + 1})")

    bad_file = list(CITES)
    bad_file[2] = ("109_bellard_the_primitive_derived_cold.md", bad_file[2][1], bad_file[2][2])
    n3 = check(bad_file, quiet=True)
    print(f"  wrong file         : {n3} failures (expected {fails + 1})")

    ok = n1 == fails + 1 and n2 == fails + 1 and n3 == fails + 1
    print()
    print(f"  the instrument fails when it should: {ok}")
    sys.exit(0 if (fails == 0 and ok) else 1)


if __name__ == "__main__":
    main()
