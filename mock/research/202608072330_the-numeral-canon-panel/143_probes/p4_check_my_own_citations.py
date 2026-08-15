"""p4 (143): open every file:line this reply cites and test its content.

Same instrument as `140_probes/p5`, repointed. `RULES.md:126` requires it, and
in `140` the first run reported three failures which all turned out to be
defects in the checker rather than in the file, so the normaliser here already
strips markdown blockquote markers.

Each entry is (label, path, lo, hi, substring, is_deliberate_control_failure).

THE CASE THAT MUST FAIL. Two entries at the end are deliberately wrong: one
whose substring is absent from a real range, and one whose range is real but
points at the wrong section. Both MUST be reported. If neither is, the checker
is not checking and every verdict is void.
"""

import re
import sys

PANEL = ("/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/"
         "202608072330_the-numeral-canon-panel")
BENCH = "/Users/orgrinrt/Dev/clause-dev/arvo/mock/benches"

F141 = f"{PANEL}/141_lamport_the_strategy_set_attacked.md"
F140 = f"{PANEL}/140_mcsherry_the_strategy_set_derived_cold.md"

CITES = [
    # --- 141, the file I am replying to ---
    ("141 test gate 123", F141, 50, 62, "123 tests", False),
    ("141 test gate flag", F141, 50, 62, "--test-threads=1", False),
    ("141 neither file makes the claim", F141, 125, 127,
     "Neither file makes the claim", False),
    ("141 140 used it correctly", F141, 125, 127,
     "cites the same sentence and uses it correctly", False),
    ("141 scope refutation", F141, 216, 218,
     "composes with every assignment rather than competing with them", False),
    ("141 neither of two offered", F141, 220, 223,
     "the answer is neither of the two it offered", False),
    ("141 identically scoped", F141, 220, 223,
     "the two models are identically scoped", False),
    ("141 theorem not discovery", F141, 520, 523,
     "rather than presenting a theorem as a discovery", False),
    ("141 2532 pairs", F141, 520, 523,
     "2532 pairs, zero monotonicity violations", False),
    ("141 strictness is the operation", F141, 526, 527,
     "Strictness is a fact about the operation added", False),
    ("141 W cites 139", F141, 563, 567,
     "139`'s own table already contains the same pattern", False),
    ("141 G the quotient", F141, 598, 601,
     "quotiented by observational equality over an observation set", False),
    ("141 declines the baseline", F141, 776, 779,
     "I am not that read", False),
    ("141 carries closure asymmetry", F141, 780, 782,
     "the assignment space is closed and enumerable", False),
    ("141 declared-width rule", F141, 783, 785,
     "what makes F1 and F2 separable at all", False),
    ("141 asks for a second model", F141, 884, 890,
     "If a second reader builds a different accumulator model", False),
    ("141 rests on one instrument", F141, 884, 890,
     "it rests on one instrument", False),

    # --- 140, my own file ---
    ("140 uses I6 narrowly", F140, 263, 279,
     "the concerns are not mutually exclusive", False),
    ("140 the overreach", F140, 577, 579,
     "composes with every assignment rather than competing with them", False),

    # --- the intent catalogue ---
    ("I1 demoted", f"{PANEL}/INTENTS.md", 51, 61,
     "the strategy set is not closed at exactly four", False),
    ("I17 count beside the point", f"{PANEL}/INTENTS.md", 369, 371,
     "besides the point of the intent", False),
    ("RULES predicate section", f"{PANEL}/RULES.md", 486, 541,
     "Every finding carries its predicate", False),
    ("RULES check citations", f"{PANEL}/RULES.md", 126, 126,
     "Check your own citations before shipping", False),

    # --- the bench tree ---
    ("the 124th hit is prose", f"{BENCH}/variants/bitpack-write-contend-shared/src/stress.rs",
     66, 70, "runs every `#[test]` in", False),

    # --- the deliberate controls ---
    ("CONTROL: substring absent", F141, 50, 62,
     "this sentence appears nowhere in 141's gate section", True),
    ("CONTROL: right file wrong range", F141, 776, 779,
     "2532 pairs, zero monotonicity violations", True),
]


def norm(s):
    """Whitespace-normalise and strip markdown blockquote markers."""
    s = re.sub(r"(?m)^\s*>\s?", " ", s)
    return re.sub(r"\s+", " ", s).strip()


def main():
    real_failures = []
    control_results = []

    for label, path, lo, hi, needle, is_control in CITES:
        try:
            with open(path, encoding="utf-8") as fh:
                lines = fh.readlines()
        except OSError as e:
            print(f"FAIL [{label}] cannot open {path}: {e}")
            (control_results if is_control else real_failures).append(label)
            continue
        if hi > len(lines):
            print(f"FAIL [{label}] range {lo}-{hi} exceeds file length {len(lines)}")
            (control_results if is_control else real_failures).append(label)
            continue
        text = norm("".join(lines[lo - 1:hi]))
        ok = norm(needle) in text
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
    bad = [lbl for lbl, ok in control_results if ok]
    if bad:
        print("deliberately-wrong citations NOT caught:")
        for b in bad:
            print(f"  - {b}")
        print("the checker does not check. every verdict above is void.")
        sys.exit(1)
    print("both deliberately-wrong citations were caught, so the checker checks.")
    if real_failures:
        sys.exit(1)


if __name__ == "__main__":
    main()
