"""p5: open every file:line this file cites and test that it says what I claim.

RULES.md:126 requires this, and records that one member found seven of its own
citations wrong by doing it. A citation that RESOLVES is not a citation that
SAYS what you claim; the failure mode is a line range that exists and contains
something else, which reads as rigour and is a lie.

Each entry is (path, first_line, last_line, substring_that_must_appear). The
substring is checked against the joined text of that line range, whitespace
normalised, so a claim survives rewrapping but not relocation.

THE CASE THAT MUST FAIL. A checker that always passes is worse than no checker.
So the table carries two deliberately-wrong entries at the end: one whose
substring is absent from the cited range, and one whose range is real but points
at the wrong section. Both MUST be reported as failures. If the checker reports
zero failures overall, it is not checking, and the script exits non-zero.
"""

import os
import re
import sys

# Resolved from this file's own location. Both were absolute, naming a checkout
# that still exists on this host, so they did not fail when the arc moved: they
# resolved against a different tree and said nothing. A checker verifying
# somebody else's clone reports clean and means nothing.
PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BENCH = os.path.normpath(os.path.join(PANEL, "../../benches"))

# (label, path, lo, hi, must_contain, is_deliberate_control_failure)
CITES = [
    ("I1 demoted to open", f"{PANEL}/INTENTS.md", 51, 61,
     "the strategy set is not closed at exactly four", False),
    ("I1 open to exploration", f"{PANEL}/INTENTS.md", 51, 61,
     "entirely open to discussion and exploration", False),
    ("I2 presets name intents", f"{PANEL}/INTENTS.md", 63, 72,
     "Each preset names a stated intent", False),
    ("I3 is ergonomics", f"{PANEL}/INTENTS.md", 88, 96,
     "Neither, it's ergonomics", False),
    ("I3 not about boundaries", f"{PANEL}/INTENTS.md", 88, 96,
     "not a statement about where arithmetic boundaries land", False),
    ("I3 restatements counted", f"{PANEL}/INTENTS.md", 85, 86,
     "Four statements of the same call", False),
    ("I4 intuitive best choice", f"{PANEL}/INTENTS.md", 102, 108,
     "intuitive best choice for most every use case", False),
    ("I4 escape clause", f"{PANEL}/INTENTS.md", 104, 108,
     "does not make it absolutely required", False),
    ("I5 sacrifice soundness", f"{PANEL}/INTENTS.md", 110, 117,
     "Hot *can* sacrifice soundness, that is its explicit purpose", False),
    ("I6 bitpacks", f"{PANEL}/INTENTS.md", 119, 127,
     "aggressively minimises and bitpacks", False),
    ("I6 can use Hot's paths", f"{PANEL}/INTENTS.md", 129, 133,
     "It can use the same paths Hot uses", False),
    ("I6 continuation narrow", f"{PANEL}/INTENTS.md", 131, 133,
     "same paths Hot uses", False),
    ("I7 within chains", f"{PANEL}/INTENTS.md", 136, 141,
     "especially within chains and ops, not only alone", False),
    ("I8 weigh differently", f"{PANEL}/INTENTS.md", 143, 153,
     "They weigh different measurements differently", False),
    ("I8 quote range", f"{PANEL}/INTENTS.md", 147, 153,
     "decided by measurement, just measuring different things", False),
    ("I9 changes correct answer", f"{PANEL}/INTENTS.md", 172, 177,
     "variables that change what the \"correct\" answer is", False),
    ("I9 quote range", f"{PANEL}/INTENTS.md", 175, 176,
     "strategies are the", False),
    ("I11 library not program", f"{PANEL}/INTENTS.md", 190, 197,
     "We are a library, not a program", False),
    ("I13 const time", f"{PANEL}/INTENTS.md", 248, 255,
     "collapses to whatever is available at const time", False),
    ("I17 four or seventeen", f"{PANEL}/INTENTS.md", 363, 377,
     "four or seventeen or a billion is besides the point", False),
    ("I17 quote range", f"{PANEL}/INTENTS.md", 369, 371,
     "besides the point of the intent", False),

    ("RULES counts are measurements", f"{PANEL}/RULES.md", 124, 124,
     "Counts are measurements", False),
    ("RULES check citations", f"{PANEL}/RULES.md", 126, 126,
     "Check your own citations before shipping", False),
    ("RULES two canon tests", f"{PANEL}/RULES.md", 79, 83,
     "Permanence", False),
    ("RULES option states closure", f"{PANEL}/RULES.md", 310, 312,
     "states what would close it", False),
    ("RULES predicate section", f"{PANEL}/RULES.md", 486, 541,
     "Every finding carries its predicate", False),

    ("satfold one-element control", f"{BENCH}/variants/satfold-shared/src/lib.rs", 1116, 1116,
     "the_one_element_defect_is_caught_up_to_1024_and_not_above_it", False),
    ("satfold wrong-operator control", f"{BENCH}/variants/satfold-shared/src/lib.rs", 1137, 1137,
     "a_wrong_operator_is_caught_at_every_length_and_both_ops", False),
    ("satfold dropped-lane control", f"{BENCH}/variants/satfold-shared/src/lib.rs", 1162, 1162,
     "a_dropped_lane_is_caught_wherever_a_sixteenth_lane_exists", False),
    ("satfold dropped-remainder", f"{BENCH}/variants/satfold-shared/src/lib.rs", 1181, 1181,
     "a_dropped_remainder_is_caught_wherever_one_exists", False),
    ("satfold false gate", f"{BENCH}/variants/satfold-shared/src/lib.rs", 1271, 1279,
     "so the false gate is not a control", False),
    ("satfold non-degenerate", f"{BENCH}/variants/satfold-shared/src/lib.rs", 1201, 1201,
     "the_workload_is_not_degenerate_at_any_length", False),
    ("warm-clamp noise floor", f"{BENCH}/variants/warm-clamp-shared/src/lib.rs", 1056, 1056,
     "the_noise_floor_controls_really_are_the_same_instantiation", False),
    ("warm-container diag spike", f"{BENCH}/variants/warm-container-shared/src/lib.rs", 1425, 1425,
     "diag_sat_lanes_actually_runs", False),
    ("warm-container shipped rule", f"{BENCH}/variants/warm-container-shared/src/lib.rs", 1506, 1506,
     "the_shipped_rule_widens_every_width_to_64", False),
    ("warm-container both regions", f"{BENCH}/variants/warm-container-shared/src/lib.rs", 1521, 1521,
     "both_regions_hold_the_same_column", False),

    ("Q51 location", f"{PANEL}/OPTIONS.md", 2425, 2461,
     "What a strategy is, after the pair was attacked", False),
    ("Q51 two-component", f"{PANEL}/OPTIONS.md", 2425, 2461,
     "It survives as a two-component object", False),
    ("Q51 denoted not computed", f"{PANEL}/OPTIONS.md", 2425, 2461,
     "component one fixes the denoted answer, not the computed one", False),
    ("Q51 fidelity constant", f"{PANEL}/OPTIONS.md", 2425, 2461,
     "a fidelity column measures a constant", False),
    ("Q51 observability of chain", f"{PANEL}/OPTIONS.md", 2425, 2461,
     "Observability is a property of the chain, not of the axis", False),
    ("Q51 0 vs 89.081", f"{PANEL}/OPTIONS.md", 2425, 2461,
     "0% against 89.081% depending on whether the limit is read", False),
    ("Q51 two-level rung", f"{PANEL}/OPTIONS.md", 2425, 2461,
     "two-level structure is **`40`'s at TWO EXPERTS**", False),

    # ---- the two deliberate control failures ----
    ("CONTROL: substring absent", f"{PANEL}/INTENTS.md", 51, 61,
     "this sentence does not appear anywhere in I1", True),
    ("CONTROL: right file wrong range", f"{PANEL}/INTENTS.md", 190, 197,
     "aggressively minimises and bitpacks", True),
]


def norm(s):
    """Normalise whitespace, and strip markdown blockquote markers.

    Without the second half, a quotation that WRAPS inside a blockquote reads as
    "... minimises and > bitpacks" and no honest substring matches it. Three of
    my citations failed on exactly that and all three were correct in the
    document, so the instrument was wrong rather than the file. Stripping the
    marker is not a loosening that lets a bad citation through: the controls
    below still have to fail, and they do.
    """
    s = re.sub(r"(?m)^\s*>\s?", " ", s)
    return re.sub(r"\s+", " ", s).strip()


def main():
    real_failures = []
    control_results = []
    checked = 0

    for label, path, lo, hi, needle, is_control in CITES:
        try:
            with open(path, encoding="utf-8") as fh:
                lines = fh.readlines()
        except OSError as e:
            print(f"FAIL [{label}] cannot open {path}: {e}")
            (control_results if is_control else real_failures).append(label)
            continue

        if hi > len(lines):
            msg = f"range {lo}-{hi} exceeds file length {len(lines)}"
            print(f"FAIL [{label}] {msg}")
            (control_results if is_control else real_failures).append(label)
            continue

        text = norm("".join(lines[lo - 1:hi]))
        ok = norm(needle) in text
        checked += 1
        if is_control:
            control_results.append((label, ok))
            print(f"{'CONTROL DID NOT FAIL' if ok else 'control failed as required'}: [{label}]")
        elif not ok:
            real_failures.append(label)
            print(f"FAIL [{label}] {path}:{lo}-{hi} does not contain {needle!r}")

    real_total = sum(1 for c in CITES if not c[5])
    print(f"\n{real_total} real citations checked by opening the range and testing its content.")
    print(f"{len(real_failures)} failed.")
    for f in real_failures:
        print(f"  - {f}")

    print("\n=== controls ===")
    bad_controls = [lbl for lbl, ok in control_results if ok]
    if bad_controls:
        print("the deliberately-wrong citations were NOT caught:")
        for b in bad_controls:
            print(f"  - {b}")
        print("the checker does not check. every verdict above is void.")
        sys.exit(1)
    print("both deliberately-wrong citations were caught, so the checker checks.")

    if real_failures:
        sys.exit(1)


if __name__ == "__main__":
    main()
