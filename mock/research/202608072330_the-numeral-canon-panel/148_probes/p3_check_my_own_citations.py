"""p3 (148): open every file:line this signature cites and test its content.

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
F146 = f"{PANEL}/146_leroy_the_canon_candidate_for_the_strategy_object.md"
F139 = f"{PANEL}/139_muratori_the_strategy_set_derived_cold.md"
F140 = f"{PANEL}/140_mcsherry_the_strategy_set_derived_cold.md"

CITES = [
    # --- 146, the candidate I am signing ---
    ("146 test gate inherited", F146, 21, 24, "123 across 13", False),
    ("146 ordering runs wrong way", F146, 37, 43, "two minutes later and after", False),
    ("146 rests on 140:6", F146, 37, 43, "140:6`'s specific disclaimer", False),
    ("146 shared input declared", F146, 45, 50, "load automatically", False),
    ("146 contamination scoping", F146, 52, 56, "140`'s own refuted P3", False),
    ("146 not a refutation", F146, 89, 93, "This is not a refutation", False),
    ("146 names 140's overreach", F146, 89, 93, "composes with every assignment", False),
    ("146 F2 as worded", F146, 225, 226, "The content survives as F2'", False),
    ("146 one-expert list", F146, 156, 162, "carries it forward and says its own count function", False),
    ("146 declared-width one expert", F146, 160, 162, "load-bearing and still one expert", False),
    ("146 6.2 units reading", F146, 578, 583, "declaration of the weighting's units", False),
    ("146 6.2 asks me", F146, 586, 587, "are the signatures that decide", False),
    ("146 6.3 the reason", F146, 596, 602, "the operation set is not named", False),
    ("146 6.3 visibility saturates", F146, 593, 595, "therefore saturates", False),
    ("146 5.3 component one", F146, 358, 361, "is component one and cutting it is forbidden", False),
    ("146 5.5 container predicate", F146, 447, 450, "operations in {add, subtract, multiply}", False),
    ("146 1.6 union only", F146, 176, 182, "including `signedness`", False),
    ("146 1.6 upper bound", F146, 184, 186, "The intersection is an upper bound", False),
    ("146 1.8 slack retired on measure", F146, 214, 220, "The residue the mechanism served is", False),
    ("146 section 8 op decides", F146, 656, 658, "unpredicated proposition at all", False),
    ("146 6.1 firewall unpredicated", F146, 552, 556, "carries no predicate in any of the three files", False),

    # --- 139, whose merge mechanism I separate from mine ---
    ("139 intermediate merged", F139, 172, 180, "The two saturating intermediate values merged", False),
    ("139 congruence mechanism", F139, 172, 180, "one-sided clipping of a monotone operation is a congruence", False),
    ("139 corroboration not discovery", F139, 172, 180, "corroboration and not discovery", False),

    # --- 141, for the signedness span and what it carries ---
    ("141 p4 spans unsigned", F141, 147, 152, "W=4 F=0 unsigned", False),
    ("141 p4 spans signed", F141, 147, 152, "W=4 F=0 signed", False),
    ("141 not a refutation", F141, 209, 223, "true of what they measured", False),
    ("141 carries closure asymmetry", F141, 780, 782, "consistent with the asymmetry rather than evidence for it", False),

    # --- 140 and 143, my own ---
    ("140 disclaimer line 5", F140, 5, 5, "139_probes/` exists and did not open it", False),
    ("140 full disclaimer", F140, 3, 7, "no probes of anyone else's", False),
    ("140 the overreach", F140, 577, 579, "composes with every assignment rather than competing with them", False),

    # --- the governing documents ---
    ("I1 demoted", f"{PANEL}/INTENTS.md", 51, 61, "the strategy set is not closed at exactly four", False),
    ("I17 count beside the point", f"{PANEL}/INTENTS.md", 369, 371, "besides the point of the intent", False),
    ("RULES absence is strongest", f"{PANEL}/RULES.md", 519, 520, "does not hold in any situation involving that dimension", False),

    # --- the deliberate controls ---
    ("CONTROL: substring absent", F146, 21, 24, "this sentence appears nowhere in the gate section", True),
    ("CONTROL: right file wrong range", F146, 21, 24, "the operation set is not named", True),
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
