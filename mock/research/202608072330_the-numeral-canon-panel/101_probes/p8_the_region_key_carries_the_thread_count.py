#!/usr/bin/env python3
"""What is actually in a region, and does the panel's predicate say so?

Every finding in this unit that reads the committed CSVs carries `threads = 1`.
`100`'s F-100-3b does, and it covers four families
(`100_xu_generating_the_table_attacked.md:498-500`). Three of those four are
declared `threaded = true` in the manifest, and their region key encodes the
thread count in its last digit.

The encoding is the crate's own and it is documented there:

    /// One row of the sweep: `KEY = N * 10 + T`.
    pub const N: usize = KEY / 10;
    /// Threads walking it.
    pub const T: usize = KEY % 10;
        (bitpack-contend-shared/src/routine.rs:11-24)

`bitpack-wide-shared/src/routine.rs:102` states it carries "the contention
crate's encoding unchanged".

So a section computed over `bitpack-contention`'s twelve regions is computed
across three different thread counts, and it cannot be reproduced from
single-threaded data at all. Under I13 a predicate lists the region where a
finding holds, and `threads = 1` names a region this class of finding does not
live in.

This probe decodes every committed region key against the manifest's own
`threaded` flag and reports, per family, which thread counts the data spans. It
is the same act `97` section 5 performed by hand for the arity sweep and `100`
section 10 audited mechanically for `warm-clamp`, applied to the dimension the
unit's predicates get wrong.

This reads committed artifacts. It is NOT a bench, no measurement was taken,
and no number here prices anything.

Run:  python3 p8_the_region_key_carries_the_thread_count.py
"""

import glob
import os
import re
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))


def threaded_sections():
    """Which manifest sections declare `threaded = true`."""
    out = set()
    cur = None
    for line in open(os.path.join(BENCH, "bench.toml")):
        m = re.match(r"\[bench\.([^\]]+)\]", line.strip())
        if m:
            cur = m.group(1)
        if line.strip().startswith("threaded") and "true" in line:
            out.add(cur)
    return out


def keys_by_family():
    fams = defaultdict(list)
    for path in sorted(glob.glob(os.path.join(BENCH, "*.csv"))):
        m = re.match(r"(.+)_n(\d+)\.csv$", os.path.basename(path))
        if m:
            fams[m.group(1)].append(int(m.group(2)))
    return fams


def main():
    th = threaded_sections()
    fams = keys_by_family()

    print("=" * 78)
    print("A. THE MANIFEST'S OWN THREADED FLAG")
    print("=" * 78)
    for f in sorted(th):
        print(f"  threaded = true   {f}")
    print(f"\n  {len(th)} of {len(fams)} committed families are threaded benches.")

    print()
    print("=" * 78)
    print("B. DECODING THE REGION KEY OF EVERY THREADED FAMILY")
    print("=" * 78)
    print("  KEY = N * 10 + T, per bitpack-contend-shared/src/routine.rs:11-24.")
    print()
    print(f"  {'family':28s} {'regions':>8s} {'element counts':>16s}   thread counts")
    for f in sorted(fams):
        if f not in th:
            continue
        ks = sorted(fams[f])
        ns = sorted({k // 10 for k in ks})
        ts = sorted({k % 10 for k in ks})
        print(f"  {f:28s} {len(ks):8d} {len(ns):16d}   {ts}")

    print()
    print("=" * 78)
    print("C. THE FAMILIES THE UNIT'S FINDINGS ACTUALLY COVER")
    print("=" * 78)
    print("  The four families carrying a byte-identical noise-floor control arm,")
    print("  which is the set `100`'s F-100-3b and this file's sections 3 and 5 use.")
    print()
    for f in [
        "bitpack-carrier-width",
        "bitpack-contend-decode",
        "bitpack-contention",
        "bitpack-wide",
    ]:
        ks = sorted(fams[f])
        if f in th:
            ts = sorted({k % 10 for k in ks})
            print(f"  {f:26s} threaded, threads in {ts}")
        else:
            print(f"  {f:26s} not threaded, threads = 1")
    print()
    print("  A section over bitpack-contention's twelve regions spans three thread")
    print("  counts, so it is not a threads = 1 object and cannot be rebuilt from")
    print("  single-threaded data. The correct predicate lists the set.")
    print()
    print("  And the shape is worth naming for its own sake: the thread count is a")
    print("  REGION dimension in this corpus, not a coordinate. That is the same")
    print("  answer p6 reaches for chain length, arrived at from the opposite")
    print("  direction: the corpus already does it for threads and does not yet for")
    print("  chains.")


if __name__ == "__main__":
    main()
