#!/usr/bin/env python3
"""p7's reader: per-symbol census of the emitted assembly.

Counts, per exported symbol: assembly lines, calls into a failure path,
conditional branches, and SIMD register mentions.

INSTRUMENT DEFECT, kept on the record.  The first version of this script
matched failure paths with the pattern `panic|_failed|core9panicking|Unwind`
and reported ZERO for `BoundedRun::sum_via_slice`, which does call
`core::slice::index::slice_index_fail`.  `_failed` does not match `_fail`, so
the one arm whose failure path is a slice-index failure rather than a bounds
panic was counted clean, and it was the arm the conclusion turned on.  The
pattern below is the repaired one and it is listed explicitly rather than as a
regex fragment, so a reader can see exactly which symbols count.
"""
import re
import sys

FAIL_SYMBOLS = [
    "slice_index_fail",
    "panic_bounds_check",
    "slice_start_index_len_fail",
    "slice_end_index_len_fail",
    "unwrap_failed",
    "core9panicking",
    "5panic",
    "handle_alloc_error",
]

path = sys.argv[1] if len(sys.argv) > 1 else \
    "build/p7_does_the_capacity_bound_survive_lowering.s"
src = open(path).read().splitlines()

labels = []
for i, l in enumerate(src):
    m = re.match(r"^(__RNv[0-9A-Za-z_$.]+):$", l)
    if m:
        labels.append((i, m.group(1)))
labels.append((len(src), "END"))

NAMES = {
    "sum_run_iter": "sum_run_iter",
    "sum_run_clamped": "sum_run_clamped",
    "sum_run_unproven": "sum_run_unproven",
    "sum_full": "sum_full",
    "sum_slice": "sum_slice",
    "13sum_via_slice": "BoundedRun::sum_via_slice",
    "10BoundedRun3sum": "BoundedRun::sum",
}


def short(sym):
    for k, v in NAMES.items():
        if k in sym:
            return v
    return sym


rows = {}
for (a, name), (b, _) in zip(labels, labels[1:]):
    body = "\n".join(src[a + 1:b])
    fails = sum(body.count(s) for s in FAIL_SYMBOLS)
    branches = len(re.findall(r"^\s+(b\.[a-z]+|cb[nz]+|tb[nz]+|j[a-z]+)\s",
                              body, re.M))
    simd = len(re.findall(r"\bq\d+\b|\bv\d+\.\d+[a-z]\b", body))
    rows[short(name)] = (b - a - 1, fails, branches, simd)

print("p7: does a composition's len <= capacity invariant survive to the backend?")
print("=" * 78)
print("A QUALITATIVE assembly read, on one host and one toolchain.  It is not a")
print("bench, it prices nothing, and no timing figure appears in it.  What it")
print("can support is an existence claim: whether a failure path is emitted.")
print()
print("rustc +nightly-2026-05-28, -O, --emit asm, aarch64-apple-darwin")
print(f"failure-path symbols counted: {', '.join(FAIL_SYMBOLS)}")
print()
print(f"{'symbol':<28} {'asm lines':>9} {'fail calls':>11} {'branches':>9}"
      f" {'simd regs':>10}")
print("-" * 72)
order = ["sum_slice", "sum_run_unproven", "BoundedRun::sum", "sum_run_clamped",
         "BoundedRun::sum_via_slice", "sum_run_iter", "sum_full"]
for want in order:
    if want in rows:
        n, f, br, s = rows[want]
        print(f"{want:<28} {n:>9} {f:>11} {br:>9} {s:>10}")
print()
print("Read this way:")
print("  A nonzero 'fail calls' means the bounds check reached the backend and")
print("  was emitted.  Zero means the compiler proved it dead and removed it.")
