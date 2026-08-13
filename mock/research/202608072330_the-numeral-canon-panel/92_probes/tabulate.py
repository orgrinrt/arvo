#!/usr/bin/env python3
"""Derive file 92's tables from the harness's own committed findings files.

Reads only `mock/benches/<bench>_n<KEY>_findings.md`, specifically the
"Statistical comparison (algo, 95% bootstrap CI)" section, which is the
harness's own bootstrap median and confidence interval for the timed region
with the bridge subtracted. Nothing here re-times anything or recomputes a
statistic; it transposes what the harness wrote so a length sweep can be read
down a column.

Run from `mock/benches/`:

    python3 ../research/202608072330_the-numeral-canon-panel/92_probes/tabulate.py

Output is committed beside this file as `tables.txt`.
"""

import re
import sys

# KEY = (LI + 1) * 1000 + NC * 100 + AL * 10 + OP, LI indexing L_TABLE.
L_TABLE = [8, 15, 16, 17, 32, 63, 64, 65, 128, 256, 1024, 4096]
N_SMALL = 32_768
N_LARGE = 16_777_216

ARMS = [
    "satfold-seq",
    "satfold-iterfold",
    "satfold-nolaw",
    "satfold-lanes4-idx",
    "satfold-lanes16",
    "satfold-lanes16-3",
    "satfold-lanes16-constl",
    "satfold-lanes64",
    "satfold-neon",
    "satfold-neon8",
    "satfold-gate-true",
    "satfold-gate-false",
]

ROW = re.compile(
    r"\| (satfold-[a-z0-9-]+) \| ([0-9.]+)\s*(ns|us|ms) \| ([^|]*)\| ([^|]*)\| \[([^\]]*)\] \| ([^|]*)\|"
)
UNIT = {"ns": 1.0, "us": 1e3, "ms": 1e6}


def read(path):
    """Median, CI and significance per arm, from one findings file."""
    text = open(path).read()
    section = text.split("## Statistical comparison")[1].split("\n## ")[0]
    out = {}
    for line in section.splitlines():
        m = ROW.match(line)
        if m:
            out[m.group(1)] = {
                "median": float(m.group(2)) * UNIT[m.group(3)],
                "delta": m.group(4).strip(),
                "ci": m.group(6).strip(),
                "sig": m.group(7).strip(),
            }
    if not out:
        raise SystemExit(f"no statistical rows parsed from {path}")
    return out


def geom(key):
    li = key // 1000 - 1
    return (
        L_TABLE[li],
        N_SMALL if (key // 100) % 10 == 0 else N_LARGE,
        (key // 10) % 10,
        key % 10,
    )


def table(bench, keys, arms):
    present = []
    data = []
    for k in keys:
        d = read(f"{bench}_n{k}_findings.md")
        data.append((k, d))
        for a in arms:
            if a in d and a not in present:
                present.append(a)
    width = max(11, max(len(a) - 8 for a in present) + 2)
    head = "  L   |" + "|".join(a.replace("satfold-", "").rjust(width) for a in present)

    print(f"\n=== {bench} ===")
    print("median of the timed region, ns per call")
    print(head)
    print("-" * len(head))
    for k, d in data:
        l, _, _, _ = geom(k)
        print(
            f"{l:5} |"
            + "|".join(
                (f"{d[a]['median']:{width}.1f}" if a in d else " " * (width - 1) + "-")
                for a in present
            )
        )

    print("\nns per element")
    print(head)
    print("-" * len(head))
    for k, d in data:
        l, n, _, _ = geom(k)
        print(
            f"{l:5} |"
            + "|".join(
                (f"{d[a]['median'] / n:{width}.5f}" if a in d else " " * (width - 1) + "-")
                for a in present
            )
        )

    if "satfold-seq" in present:
        print("\nspeedup against the fold as written")
        print(head)
        print("-" * len(head))
        for k, d in data:
            l, _, _, _ = geom(k)
            base = d["satfold-seq"]["median"]
            print(
                f"{l:5} |"
                + "|".join(
                    (f"{base / d[a]['median']:{width}.2f}" if a in d else " " * (width - 1) + "-")
                    for a in present
                )
            )

    print("\n95% bootstrap CI of the median, ns")
    for k, d in data:
        l, _, _, _ = geom(k)
        print(f"  L={l}: " + "  ".join(f"{a.replace('satfold-', '')}=[{d[a]['ci']}]" for a in present if a in d))


def cross(bench_a, keys_a, bench_b, keys_b, label, arms):
    """Ratio between two benches at the same reduction lengths.

    A cross-row ratio, not a paired comparison: the harness pairs batches
    within one row and cannot pair across rows, so these carry no CI and are
    read against the per-row intervals printed above.
    """
    print(f"\n=== {label} ===")
    present = [a for a in arms]
    width = 12
    head = "  L   |" + "|".join(a.replace("satfold-", "").rjust(width) for a in present)
    print(head)
    print("-" * len(head))
    for ka, kb in zip(keys_a, keys_b):
        da = read(f"{bench_a}_n{ka}_findings.md")
        db = read(f"{bench_b}_n{kb}_findings.md")
        l, _, _, _ = geom(ka)
        print(
            f"{l:5} |"
            + "|".join(
                (
                    f"{da[a]['median'] / db[a]['median']:{width}.3f}"
                    if a in da and a in db
                    else " " * (width - 1) + "-"
                )
                for a in present
            )
        )


def main():
    l1 = [(i + 1) * 1000 for i in range(12)]
    wrap = [(i + 1) * 1000 + 1 for i in (0, 2, 6, 9, 11)]
    align = [(i + 1) * 1000 + 10 for i in (2, 6, 9, 11)]
    dram = [(i + 1) * 1000 + 100 for i in (2, 6)]
    dram_long = [12100]
    dram_wrap = [12101]
    gate = [7000, 10000, 12000]

    table("satfold-length-l1", l1, ARMS)
    table("satfold-length-l1-wrap", wrap, ARMS)
    table("satfold-align-l1", align, ARMS)
    table("satfold-length-dram", dram, ARMS)
    table("satfold-length-dram-long", dram_long, ARMS)
    table("satfold-length-dram-wrap", dram_wrap, ARMS)
    table("satfold-const-gate", gate, ARMS)

    cross(
        "satfold-align-l1",
        align,
        "satfold-length-l1",
        [(i + 1) * 1000 for i in (2, 6, 9, 11)],
        "offset start divided by aligned start, same reduction length",
        ARMS[:9],
    )
    cross(
        "satfold-length-l1",
        [(i + 1) * 1000 for i in (0, 2, 6, 9, 11)],
        "satfold-length-l1-wrap",
        wrap,
        "saturating divided by wrapping, same arm and reduction length",
        ARMS[:9],
    )


if __name__ == "__main__":
    sys.exit(main())
