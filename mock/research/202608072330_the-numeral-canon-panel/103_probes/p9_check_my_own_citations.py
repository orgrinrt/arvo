#!/usr/bin/env python3
"""p9. Open every citation in `103` and test its CONTENT, not its resolution.

`RULES.md` requires checking your own citations by opening them, and records
that one member found seven of its own wrong this way. A reference that resolves
is not a reference that says what you claim, so each check below asserts a
substring that must be present at the target, chosen so a rename or a move fails
loudly rather than silently pointing at different text.

Every entry is (what the file claims, where, what must be there). A failure
means my file is wrong, not that the check is fussy.

**Comparison is whitespace-normalised**, and that is a repair rather than a
loosening. The first version compared exact substrings and reported five
failures, all five of which were quotations spanning a line break in wrapped
markdown: `102`'s "SYNTHETIC, because no committed / family has arms that
disagree", op's I9, and three others. Every quotation was accurate and the
instrument was wrong, which is the failure mode this probe exists to catch
pointed at itself. Collapsing runs of whitespace to one space on both sides
still fails on a phrase that is absent, altered, or moved to a different file,
which is what the check is for.

Run from anywhere; paths resolve relative to this file.
"""

import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", ".."))
PANEL = os.path.abspath(os.path.join(HERE, ".."))
BENCH = os.path.join(REPO, "mock", "benches")
HARNESS = os.path.expanduser(
    "~/.cargo/git/checkouts/mockspace-d2db2c8fb6d9e932/bce17f6"
)

# (label, path, must-contain)
FILE_CHECKS = [
    (
        "102 states the claim in bold",
        os.path.join(PANEL, "102_torvalds_does_the_mechanism_serve_the_intents.md"),
        "Every arm set in the committed corpus is answer-equivalent",
    ),
    (
        "102's census unit is variants/*-shared",
        os.path.join(
            PANEL, "102_probes", "p1_the_corpus_compares_cost_at_a_fixed_answer.py"
        ),
        'd.name.endswith("-shared")',
    ),
    (
        "102's validator test is presence-only",
        os.path.join(
            PANEL, "102_probes", "p1_the_corpus_compares_cost_at_a_fixed_answer.py"
        ),
        'has_validate = "yes" if re.search(r"fn\\s+validate_output", src)',
    ),
    (
        "102's p5 says the time column is synthetic",
        os.path.join(
            PANEL,
            "102_probes",
            "p5_a_measured_coordinate_over_answer_differing_arms.out",
        ),
        "no committed family has arms that disagree",
    ),
    (
        "102's p5 states the constraint",
        os.path.join(
            PANEL,
            "102_probes",
            "p5_a_measured_coordinate_over_answer_differing_arms.out",
        ),
        "may include a MEASURED coordinate only where every arm",
    ),
    (
        "the ByteRoutine registration with MAY_DIFFER = true",
        os.path.join(BENCH, "src", "main.rs"),
        'routine_bridge!(ByteRoutine<64, 8, true>)',
    ),
    (
        "the driver's comment that run does not validate",
        os.path.join(BENCH, "src", "main.rs"),
        "never\n                // calls `validation::validate`",
    ),
    (
        "the driver's comment that unit tests are the only fidelity check",
        os.path.join(BENCH, "src", "main.rs"),
        "the only fidelity check in the system",
    ),
    (
        "bitpack-shared's validator compares against a computed reference",
        os.path.join(BENCH, "variants", "bitpack-shared", "src", "lib.rs"),
        "if output.value != expect {",
    ),
    (
        "quantiser-radix's validator is a range check and says radix-neutral",
        os.path.join(BENCH, "variants", "quantiser-radix-shared", "src", "lib.rs"),
        "// Radix-neutral",
    ),
    (
        "quantiser-radix's header states the differing-precision confound",
        os.path.join(BENCH, "variants", "quantiser-radix-shared", "src", "lib.rs"),
        "The confound that remains, stated rather than hidden",
    ),
    (
        "quantiser-radix's header states the grid-step exponent convention",
        os.path.join(BENCH, "variants", "quantiser-radix-shared", "src", "lib.rs"),
        "in grid steps, not in absolute magnitude",
    ),
    (
        "the radix-two-against-silicon test and its asserted check count",
        os.path.join(BENCH, "variants", "quantiser-radix-shared", "src", "lib.rs"),
        "assert_eq!(checked, 4 * 32 * N as u64);",
    ),
    (
        "the radix-ten nearest-grid-point test",
        os.path.join(BENCH, "variants", "quantiser-radix-shared", "src", "lib.rs"),
        "fn radix_ten_delivers_the_nearest_grid_point_ties_to_even",
    ),
    (
        "the redundant assertion I flagged",
        os.path.join(BENCH, "variants", "quantiser-radix-shared", "src", "lib.rs"),
        "assert!(p % 2 == 1);",
    ),
    (
        "quantiser-fadd's bit-exact test over 6*64*N checks",
        os.path.join(BENCH, "variants", "quantiser-fadd-shared", "src", "lib.rs"),
        "assert_eq!(checked, 6 * 64 * N as u64);",
    ),
    (
        "warm-container-shared's fast-arm-is-doing-less comment",
        os.path.join(BENCH, "variants", "warm-container-shared", "src", "lib.rs"),
        "is doing less.",
    ),
    (
        "fnv1a imports from the deleted crate tree",
        os.path.join(BENCH, "variants", "fnv1a", "src", "lib.rs"),
        "use arvo_hash::{ConstHash, Fnv1a};",
    ),
    (
        "xxhash3 imports from the deleted crate tree",
        os.path.join(BENCH, "variants", "xxhash3", "src", "lib.rs"),
        "use arvo_hash::{ConstHash, XxHash3};",
    ),
    (
        "the harness validation_plan, per_variant always true",
        os.path.join(HARNESS, "bench-harness", "src", "validation.rs"),
        "per_variant:   true,",
    ),
    (
        "the harness validation_plan, ByteExact as the no-consent default",
        os.path.join(HARNESS, "bench-harness", "src", "validation.rs"),
        "Some(CrossVariant::ByteExact)",
    ),
    (
        "the harness validation seed constant",
        os.path.join(HARNESS, "bench-harness", "src", "validation.rs"),
        "const VALIDATION_ROOT_SEED: u64 = 0xCAFE_BABE_DEAD_BEEF;",
    ),
    (
        "DEFAULT_VALIDATION_SEEDS is 100",
        os.path.join(HARNESS, "bench-harness", "src", "validation.rs"),
        "pub const DEFAULT_VALIDATION_SEEDS: usize = 100;",
    ),
    (
        "op's I13, ratified, rejecting a universal solution",
        os.path.join(PANEL, "INTENTS.md"),
        "We explicitly reject a universal solution",
    ),
    (
        "op's I9, the strategy is what makes an answer correct",
        os.path.join(PANEL, "INTENTS.md"),
        'strategies are the variables that change what the "correct" answer is',
    ),
    (
        "op's 95, a unit must end in agreement",
        os.path.join(PANEL, "95_op_the_panel_runs_to_ratification_and_units_must_converge.md"),
        "has to end with solutions and agreements at least with something",
    ),
    (
        "op's 95 names the never-called validator as inside the goal",
        os.path.join(PANEL, "95_op_the_panel_runs_to_ratification_and_units_must_converge.md"),
        "a `validate_output` the harness never calls",
    ),
    (
        "op's 88 section 4, no single one-fits-all solutions",
        os.path.join(PANEL, "88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md"),
        "No single one-fits-all solutions, it's impossible",
    ),
    (
        "Q49 in the register, carrying the claim I refute",
        os.path.join(PANEL, "OPTIONS.md"),
        "every committed region is\nanswer-equivalent: all arms compute one value",
    ),
]

# (label, command, must-appear-in-output)
CMD_CHECKS = [
    (
        "94 variant crates",
        ["bash", "-c", f"ls -d {BENCH}/variants/*/ | wc -l"],
        "94",
    ),
    (
        "zero crates implement score_output or score_dimensions",
        [
            "bash",
            "-c",
            f"grep -rl 'fn score_output\\|fn score_dimensions' {BENCH}/variants/*/src/ 2>/dev/null | wc -l",
        ],
        "0",
    ),
    (
        "254 committed CSV files",
        ["bash", "-c", f"ls {BENCH}/*.csv | wc -l"],
        "254",
    ),
    (
        "102 never mentions the hash bench or ByteRoutine",
        [
            "bash",
            "-c",
            f"grep -c 'fnv1a\\|xxhash\\|ByteRoutine' "
            f"'{PANEL}/102_torvalds_does_the_mechanism_serve_the_intents.md' || true",
        ],
        "0",
    ),
    (
        "the validation call landed in 9db33f8c",
        [
            "bash",
            "-c",
            f"git -C {REPO} log --format=%h -S'harness::validate' -- mock/benches/src/main.rs | tail -1",
        ],
        "9db33f8c",
    ),
    (
        "the four fnv1a-vs-xxhash3 CSVs each hold both arms",
        [
            "bash",
            "-c",
            f"cd {BENCH} && for f in fnv1a-vs-xxhash3_n*.csv; do "
            f"cut -d, -f5 $f | tail -n +2 | sort -u | tr '\\n' ' '; done | tr -s ' '",
        ],
        "fnv1a xxhash3 fnv1a xxhash3 fnv1a xxhash3 fnv1a xxhash3",
    ),
]


def norm(s):
    """Strip blockquote markers, then collapse whitespace to single spaces.

    Two markdown artifacts break an exact substring test on an accurate
    quotation. Wrapping puts a newline inside a sentence, and a blockquote puts
    a `>` at the start of every continuation line, so a quotation of op's words
    spanning two lines contains a `>` in the source that is syntax rather than
    anything he said. Both are removed here and neither removal can make an
    absent phrase appear.
    """
    s = re.sub(r"(?m)^\s*>\s?", " ", s)
    return re.sub(r"\s+", " ", s).strip()


def main():
    ok = 0
    bad = []

    print("p9. checking 103's own citations by opening them")
    print()

    for label, path, needle in FILE_CHECKS:
        if not os.path.isfile(path):
            bad.append((label, path, "FILE DOES NOT EXIST"))
            continue
        with open(path, errors="replace") as fh:
            text = norm(fh.read())
        if norm(needle) in text:
            ok += 1
        else:
            bad.append((label, path, f"substring absent: {needle!r}"))

    for label, cmd, needle in CMD_CHECKS:
        try:
            out = subprocess.check_output(cmd, stderr=subprocess.DEVNULL).decode()
        except subprocess.CalledProcessError as e:
            bad.append((label, " ".join(cmd[-1:]), f"command failed: {e}"))
            continue
        if norm(needle) in norm(out) or norm(needle) == norm(out):
            ok += 1
        else:
            bad.append((label, " ".join(cmd[-1:]), f"expected {needle!r}, got {out.strip()!r}"))

    total = len(FILE_CHECKS) + len(CMD_CHECKS)
    print(f"citations checked : {total}")
    print(f"passing           : {ok}")
    print(f"failing           : {len(bad)}")
    print()
    if bad:
        print("FAILURES, each of which is a defect in 103 rather than in this probe:")
        for label, where, why in bad:
            print(f"  {label}")
            print(f"    at   {where}")
            print(f"    why  {why}")
        return 1
    print("Every citation resolves AND says what 103 claims it says.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
