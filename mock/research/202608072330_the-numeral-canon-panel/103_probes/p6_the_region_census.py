#!/usr/bin/env python3
"""p6. The census over the right unit: committed REGIONS, not shared crates.

A "committed region" is one committed CSV: one bench name at one size, holding
the arms that ran against each other. That is the unit the claim is about
("every committed region is answer-equivalent"), and it is not the unit a
per-shared-crate census enumerates.

The difference is not cosmetic. A region's answer-discipline is decided by the
`Routine` bridge the driver registers for it in `src/main.rs`'s `routine_for_n`,
and one region's bridge is `ByteRoutine<N, 8, MAY_DIFFER>` declared inline in
that table with no shared crate anywhere. A census whose unit is
`variants/*-shared/` cannot see it, so it cannot see its flag either.

This probe joins three things that are all committed:

1. Every CSV in `mock/benches/`, giving the region name, size, and arm set.
2. `bench.toml`, giving the declared sizes and variant paths per bench.
3. `src/main.rs`'s `routine_for_n` table, giving the bridge and, where the
   bridge is `ByteRoutine`, the literal `MAY_DIFFER` const.

and reports, per region, whether anything in the repository requires that
region's arms to compute one value.

Three verdicts:

- **PINNED**: the region's routine has an exact-value validator (p5), so every
  arm that passes computes the reference value. This is decided by the code and
  it is INDEPENDENT of the consent flag: `satfold` both declares
  `outputs_may_differ = true` and pins every arm to a `u64` oracle, and it is
  answer-equivalent for the second reason regardless of the first. An earlier
  version of this probe let the flag override the oracle and mislabelled all 28
  satfold regions; the order below is the fix.
- **UNPINNED**: the routine's validator checks a property rather than a value,
  so arms passing it may compute different values.
- **NO ORACLE AT ALL**: the routine is the harness's generic `ByteRoutine`, so
  there is no `validate_output` anywhere and, at `MAY_DIFFER = true`, no
  cross-variant comparison either. Nothing in the repository requires these arms
  to compute one value.

The consent flag is reported as its own column rather than folded into the
verdict, because the two questions are independent and conflating them is what
produced the mislabelling above.

Run from anywhere; paths resolve relative to this file.
"""

import csv
import os
import re
import sys

# From p5, whose classification was cross-checked by hand against every
# `validate_output` body in the corpus.
ANSWER_PINNING = {
    "CarrierColumn": "bitpack-carrier-shared",
    "Contend": "bitpack-contend-shared",
    "FootprintColumn": "bitpack-footprint-shared",
    "PlanColumn": "bitpack-plan-shared",
    "MacColumn": "bitpack-plan-shared",
    "Column": "bitpack-shared",
    "Wide": "bitpack-wide-shared",
    "WriteContend": "bitpack-write-contend-shared",
    "SatFoldCase": "satfold-shared",
    "ClampCase": "warm-clamp-shared",
    "Case": "warm-container-shared",
    "WideCase": "wide-rung-shared",
}

PROPERTY_ONLY = {
    "AddSweep": "quantiser-fadd-shared",
    "RadixAdd": "quantiser-radix-shared",
}

# Routines whose declaration consents to arms differing, from the shared
# crates' own `fn outputs_may_differ` overrides (p5).
CONSENTS = {"SatFoldCase"}


def parse_driver(path):
    """(bench_name, n) -> (bridge type name, may_differ literal or None)."""
    with open(path) as fh:
        text = fh.read()
    table = {}
    pat = re.compile(
        r'\(\s*"([^"]+)"\s*,\s*(\d+)\s*\)\s*=>\s*routine_bridge!\(\s*([A-Za-z_][A-Za-z0-9_]*)\s*<([^>]*)>\s*\)'
    )
    for m in pat.finditer(text):
        name, n, ty, args = m.group(1), int(m.group(2)), m.group(3), m.group(4)
        may = None
        if ty == "ByteRoutine":
            parts = [a.strip() for a in args.split(",")]
            if len(parts) == 3:
                may = parts[2]
        table[(name, n)] = (ty, may)
    return table


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    benches = os.path.abspath(os.path.join(here, "..", "..", "..", "benches"))
    driver = os.path.join(benches, "src", "main.rs")
    if not os.path.isdir(benches) or not os.path.isfile(driver):
        print("bench tree not found at", benches)
        return 1

    table = parse_driver(driver)
    print("p6. census over committed regions")
    print()
    print("driver:", driver)
    print("routine_for_n entries parsed:", len(table))
    print()

    # Region name is the CSV stem before the trailing `_n<digits>`.
    regions = []
    for fn in sorted(os.listdir(benches)):
        if not fn.endswith(".csv"):
            continue
        stem = fn[: -len(".csv")]
        m = re.match(r"^(.*)_n(\d+)$", stem)
        if not m:
            regions.append((fn, stem, None, set()))
            continue
        name, n = m.group(1), int(m.group(2))
        arms = set()
        with open(os.path.join(benches, fn), newline="") as fh:
            for row in csv.DictReader(fh):
                arms.add(row["variant"])
        regions.append((fn, name, n, arms))

    verdicts = {"PINNED": [], "NO ORACLE AT ALL": [], "UNPINNED": [], "NO BRIDGE": []}

    print(f"{'region':<46} {'n':>10} {'arms':>5} {'bridge':<16} {'verdict':<18} consent flag")
    print("-" * 120)
    for fn, name, n, arms in regions:
        key = (name, n)
        entry = table.get(key)
        if entry is None:
            v = "NO BRIDGE"
            bridge = "-"
            consent = "-"
        else:
            bridge, may = entry
            # The oracle question first. The consent flag is reported beside it
            # and never overrides it.
            if bridge in ANSWER_PINNING:
                v = "PINNED"
            elif bridge in PROPERTY_ONLY:
                v = "UNPINNED"
            else:
                v = "NO ORACLE AT ALL"
            if may is not None:
                consent = "may differ" if may == "true" else "must agree"
            elif bridge in CONSENTS:
                consent = "may differ"
            else:
                consent = "must agree"
        verdicts[v].append((fn, name, n, sorted(arms), consent))
        if v != "PINNED":
            print(f"{name:<46} {str(n):>10} {len(arms):>5} {bridge:<16} {v:<18} {consent}")

    print()
    print("(regions with verdict PINNED are omitted from the listing above)")
    print()
    print("TOTALS OVER COMMITTED REGIONS")
    total = sum(len(v) for v in verdicts.values())
    for k in ["PINNED", "UNPINNED", "NO ORACLE AT ALL", "NO BRIDGE"]:
        print(f"  {k:<22} {len(verdicts[k]):>4} of {total}")
    consented = sum(
        1 for v in verdicts.values() for row in v if row[4] == "may differ"
    )
    print(f"  {'(consent flag: may differ)':<22} {consented:>4} of {total}")
    pinned_and_consenting = sum(
        1 for row in verdicts["PINNED"] if row[4] == "may differ"
    )
    print(
        f"  {'  of which also PINNED':<22} {pinned_and_consenting:>4}"
        "   <- answer-equivalent by oracle despite the flag"
    )

    print()
    print("EVERY REGION NOT PINNED, WITH ITS ARMS")
    for k in ["UNPINNED", "NO ORACLE AT ALL", "NO BRIDGE"]:
        for fn, name, n, arms, consent in verdicts[k]:
            print(f"  [{k}] {name} n={n}  consent={consent}")
            print(f"      arms: {arms}")

    print()
    print("READING")
    print("  PINNED is the only verdict that supports 'this region compares cost")
    print("  at a fixed answer' from the region's own code. The consent flag is")
    print("  orthogonal: satfold consents AND pins, so it is answer-equivalent.")
    print("  UNPINNED and NO ORACLE AT ALL regions have arms whose agreement is")
    print("  asserted nowhere in this repository. Whether they in fact agree is a")
    print("  separate measurement, taken for two of them in p2, p3 and p4.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
