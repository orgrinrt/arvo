#!/usr/bin/env python3
"""Open every `file:line` this file cites and test its content, not its resolution.

Built after `25_probes/p3_verify_my_citations.py` and `26_probes/verify_my_citations.py`,
for the reason those exist: a reference that resolves is not a reference that says
what the citing document claims, and this panel has recorded five separate
instances of that failure, one of them at four wrong citations out of ten.

Two kinds of check:

  * source citations, where a named line must contain a named substring,
  * measured numbers, where a figure quoted in the prose must be reproducible
    from the committed csvs to within a stated tolerance.

The harness sources are a git dependency rather than repo content, so their
checkout is resolved from `mock/Cargo.lock` rather than hardcoded, which means
this probe fails loudly if the pin ever moves instead of checking a stale copy.

Run: python3 27_probes/verify_my_citations.py
"""

import csv
import os
import re
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ARVO = os.path.normpath(os.path.join(HERE, "..", "..", "..", ".."))
MOCK = os.path.join(ARVO, "mock")
BENCH = os.path.join(MOCK, "benches")

ok = 0
fail = []


def check_line(path, line, needle, why):
    global ok
    if not os.path.exists(path):
        fail.append(f"{why}: {path} does not exist")
        return
    lines = open(path, errors="replace").read().split("\n")
    if line > len(lines):
        fail.append(f"{why}: {path} has {len(lines)} lines, cited {line}")
        return
    if needle not in lines[line - 1]:
        fail.append(f"{why}: {path}:{line} does not contain {needle!r}; it is {lines[line-1].strip()!r}")
        return
    ok += 1


def harness_root():
    lock = open(os.path.join(MOCK, "Cargo.lock")).read()
    m = re.search(r'name = "mockspace-bench-harness".*?#([0-9a-f]{40})', lock, re.S)
    if not m:
        fail.append("could not read the mockspace revision out of mock/Cargo.lock")
        return None
    rev = m.group(1)[:7]
    root = os.path.expanduser(
        f"~/.cargo/git/checkouts/mockspace-d2db2c8fb6d9e932/{rev}"
    )
    if not os.path.isdir(root):
        fail.append(f"the pinned mockspace checkout {rev} is not on this machine at {root}")
        return None
    return root


def p10(v):
    v = sorted(v)
    return v[max(0, (len(v) * 10) // 100)]


def measured(section, key, arm, mode="warm", stat=p10):
    n = key // 10
    path = os.path.join(BENCH, f"{section}_n{key}.csv")
    vals = []
    with open(path) as fh:
        for r in csv.DictReader(fh):
            if r["mode"] == mode and r["variant"].endswith(arm):
                vals.append(float(r["algo_ns"]))
    return stat(vals) * 1000.0 / n


def check_num(claimed, actual, tol, why):
    global ok
    if actual == 0 or abs(claimed - actual) / abs(actual) > tol:
        fail.append(f"{why}: prose says {claimed}, csvs say {actual:.2f}")
        return
    ok += 1


def main():
    root = harness_root()
    if root:
        check_line(
            os.path.join(root, "bench-harness/src/config.rs"), 109,
            "pub threaded:", "the manifest's threaded flag",
        )
        check_line(
            os.path.join(root, "bench-harness/src/config.rs"), 102,
            "Whether variants spawn their own threads", "what threaded means",
        )
        check_line(
            os.path.join(root, "bench-harness/src/harness.rs"), 160,
            "if !threaded {", "threaded skips the self-pin in the worker",
        )
        check_line(
            os.path.join(root, "bench-harness/src/harness.rs"), 621,
            "if config.threaded {", "threaded is passed through to each worker",
        )
        check_line(
            os.path.join(root, "bench-harness/src/harness.rs"), 801,
            "if !threaded {", "threaded skips the self-pin in the validate worker",
        )
        check_line(
            os.path.join(root, "bench-core/src/counter.rs"), 139,
            "pub fn pin_to_perf_cores()", "where the pin lives",
        )
        check_line(
            os.path.join(root, "bench-core/src/counter.rs"), 147,
            "pthread_set_qos_class_self_np(0x21, 0)", "the pin is a QoS call",
        )

    check_line(
        os.path.join(BENCH, "src/main.rs"), 151,
        "harness::validate(", "the driver validates before timing",
    )

    # every headline number in the prose, recomputed
    check_num(86.6, measured("bitpack-contention", 83886081, "d16"), 0.02,
              "section 7 warm p10 d16 at n=8388608 t=1")
    check_num(33.5, measured("bitpack-contention", 83886084, "d16"), 0.02,
              "section 7 warm p10 d16 at n=8388608 t=4")
    check_num(134.7, measured("bitpack-contention", 83886084, "d64"), 0.02,
              "section 7 warm p10 d64 at n=8388608 t=4")
    check_num(128.7, measured("bitpack-contention", 83886081, "d64"), 0.02,
              "section 7 warm p10 d64 at n=8388608 t=1")
    check_num(43.5, measured("bitpack-contention", 83886084, "packed-simd"), 0.02,
              "section 7 warm p10 packed-simd at n=8388608 t=4")
    check_num(42.4, measured("bitpack-contention", 83886084, "d16", "cold"), 0.02,
              "section 7.4 cold p10 d16 at n=8388608 t=4")
    check_num(43.4, measured("bitpack-contention", 83886084, "packed-simd", "cold"), 0.02,
              "section 7.4 cold p10 packed-simd at n=8388608 t=4")
    check_num(41.7, measured("bitpack-contention", 41943044, "packed-simd", "cold"), 0.02,
              "section 7.4 cold p10 packed-simd at n=4194304 t=4")
    check_num(43.8, measured("bitpack-contention", 41943044, "d16", "cold"), 0.02,
              "section 7.4 cold p10 d16 at n=4194304 t=4")

    check_num(117.6, measured("bitpack-contend-decode", 163841, "packed-simd"), 0.02,
              "section 8 warm p10 packed-simd at n=16384 t=1")
    check_num(70.1, measured("bitpack-contend-decode", 163841, "pipe2"), 0.02,
              "section 8 warm p10 pipe2 at n=16384 t=1")
    check_num(73.5, measured("bitpack-contend-decode", 83886081, "pipe4"), 0.02,
              "section 8 warm p10 pipe4 at n=8388608 t=1")

    check_num(32.5, measured("bitpack-contend-best", 41943041, "d16-padal"), 0.02,
              "section 9 warm p10 d16-padal at n=4194304 t=1")
    check_num(85.7, measured("bitpack-contend-best", 41943041, "d16"), 0.02,
              "section 9 warm p10 d16 at n=4194304 t=1")
    check_num(66.4, measured("bitpack-contend-best", 83886084, "d32"), 0.02,
              "section 9 warm p10 d32 at n=8388608 t=4")
    check_num(66.9, measured("bitpack-contend-best", 83886084, "d32-padal"), 0.02,
              "section 9 warm p10 d32-padal at n=8388608 t=4")
    check_num(26.4, measured("bitpack-contend-best", 83886084, "pipe4"), 0.02,
              "section 9 warm p10 pipe4 at n=8388608 t=4")
    check_num(27.1, measured("bitpack-contend-best", 83886084, "d16-padal"), 0.02,
              "section 9 warm p10 d16-padal at n=8388608 t=4")

    # 26's committed medians, which section 1 claims to reproduce
    def carrier_med(n, arm):
        path = os.path.join(BENCH, f"bitpack-carrier-width_n{n}.csv")
        vals = [
            float(r["algo_ns"])
            for r in csv.DictReader(open(path))
            if r["mode"] == "warm" and r["variant"].endswith(arm)
        ]
        return statistics.median(vals) * 1000.0 / n

    check_num(87.9, carrier_med(8388608, "d16"), 0.01, "section 1 carrier d16 at n=8388608")
    check_num(145.7, carrier_med(8388608, "d64"), 0.01, "section 1 carrier d64 at n=8388608")
    check_num(120.6, carrier_med(4194304, "packed-simd"), 0.01,
              "section 1 carrier packed-simd at n=4194304")


    # section 15, the wide sweep
    def wide(key, arm, mode="warm"):
        n = key // 10
        path = os.path.join(BENCH, f"bitpack-wide_n{key}.csv")
        vals = [
            float(r["algo_ns"])
            for r in csv.DictReader(open(path))
            if r["mode"] == mode and r["variant"].endswith(arm)
        ]
        return p10(vals) * 1000.0 / n

    check_num(86.8, wide(83886081, "d16"), 0.02, "section 15 wide d16 at n=8388608 t=1")
    check_num(33.8, wide(167772164, "d16"), 0.02, "section 15 wide d16 at n=16777216 t=4")
    check_num(34.1, wide(167772164, "d16-padal"), 0.02, "section 15 wide d16-padal at n=16777216 t=4")
    check_num(29.1, wide(167772164, "pipe4"), 0.02, "section 15 wide pipe4 at n=16777216 t=4")
    check_num(38.9, wide(167772161, "d16-padal"), 0.02, "section 15 wide d16-padal at n=16777216 t=1")
    check_num(89.0, wide(167772161, "d16"), 0.02, "section 15 wide d16 at n=16777216 t=1")
    check_num(33.3, wide(335544321, "d16-padal"), 0.02, "section 15 wide d16-padal at n=33554432 t=1")
    check_num(85.7, wide(335544321, "d16"), 0.02, "section 15 wide d16 at n=33554432 t=1")

    print(f"{ok} of {ok + len(fail)} citations and numbers check out")
    for f in fail:
        print("  FAIL " + f)
    return 1 if fail else 0


if __name__ == "__main__":
    sys.exit(main())
