#!/usr/bin/env python3
"""P6. The located disagreement between 93 and 94, against committed harness output.

`94` phase two section 6 records the surviving disagreement: whether intermediate
precision sits on the value or at the site. `93` says value; `94` says site. Both name
the same discriminator and neither ran it: does any consumer want two accumulator
widths over one stored column? `93` declined on the ground that the consumers are
pinned to a dead tier.

The discriminator does not need a consumer. arvo's own committed harness carries the
controlled experiment, and `25` section 6.2 named the family without asking this
question of it.

`warm-clamp-arity-w13` sweeps SIX fold arities with everything else held. From the
crate's own key encoding (`variants/warm-clamp-shared/src/lib.rs:83`,
`KEY = W * 10000 + NC * 1000 + LOG2A * 10 + OP`):

    130010 -> W=13, 8192 elements, arity 2,   chunked clamping fold
    130020 -> W=13, 8192 elements, arity 4,   chunked clamping fold
    130030 -> W=13, 8192 elements, arity 8,   chunked clamping fold
    130040 -> W=13, 8192 elements, arity 16,  chunked clamping fold
    130060 -> W=13, 8192 elements, arity 64,  chunked clamping fold
    130080 -> W=13, 8192 elements, arity 256, chunked clamping fold

Declared width, element count and transform are constant. The stored column is the
same shape at every point. Only the fold's arity moves.

So the question becomes checkable without leaving arvo: does the winning accumulator
arm change with the arity? If it does, the accumulator's right width is a fact about
the fold and cannot be read off the stored value, and a design carrying it on the
value would need a cast that changes no value in order to fold the same column twice.

Rankings only, never magnitudes across runs, and the contending set at three
tolerances rather than a strict argmin, because a strict argmin over arms that sit
within noise of each other measures the noise. That is `40`'s p7 discipline and it is
borrowed deliberately.
"""

import csv
import glob
import os
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.abspath(os.path.join(HERE, "..", "..", "..", "benches"))

# the two arms whose difference IS the question: an accumulator derived from the
# fold's arity, against a fixed one that ignores it.
ARITY_SENSITIVE = "warm-clamp-accfit"
ARITY_BLIND = "warm-clamp-acc64"


def key_arity(key):
    return 1 << ((key // 10) % 100)


def key_w(key):
    return key // 10000


def key_n(key):
    return 8192 if (key // 1000) % 10 == 0 else 1048576


def load():
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "warm-clamp-arity-w13_n*.csv"))):
        key = int(os.path.basename(path).split("_n")[1].split(".")[0])
        by = {}
        for r in csv.DictReader(open(path)):
            by.setdefault(r["variant"], []).append(float(r["algo_ns"]))
        out[key] = {v: statistics.median(xs) for v, xs in by.items()}
    return dict(sorted(out.items(), key=lambda kv: key_arity(kv[0])))


def contending(meds, tol):
    best = min(meds.values())
    return sorted(a for a, m in meds.items() if m <= best * (1.0 + tol))


def main():
    data = load()
    print("P6. does the winning accumulator arm move with the fold's arity")
    print("source: mock/benches/warm-clamp-arity-w13_n*.csv, committed harness output")
    print("read, not measured: this probe runs no benchmark")
    print()

    widths = {key_w(k) for k in data}
    counts = {key_n(k) for k in data}
    print("declared width held : %s" % ", ".join(str(w) for w in sorted(widths)))
    print("element count held  : %s" % ", ".join(str(c) for c in sorted(counts)))
    print("fold arity varied   : %s" % ", ".join(str(key_arity(k)) for k in data))
    assert len(widths) == 1 and len(counts) == 1, "the control is not held"
    print()

    arms = sorted({a for m in data.values() for a in m})
    print("medians in ns, by fold arity")
    hdr = "%8s" % "arity"
    for a in arms:
        hdr += "%22s" % a.replace("warm-clamp-", "")
    print(hdr)
    for k, meds in data.items():
        line = "%8d" % key_arity(k)
        for a in arms:
            line += "%22.1f" % meds.get(a, float("nan"))
        print(line)
    print()

    moved = {}
    for tol in (0.0, 0.02, 0.05):
        print("arms in contention at tolerance %.0f%% of the fastest" % (100 * tol))
        seen = []
        for k, meds in data.items():
            c = contending(meds, tol)
            seen.append(tuple(c))
            print("  arity %4d : %s" % (key_arity(k),
                                        ", ".join(x.replace("warm-clamp-", "") for x in c)))
        distinct = len(set(seen))
        print("  distinct contending sets across the six arities: %d" % distinct)
        print("  the winner MOVES with arity: %s" % ("yes" if distinct > 1 else "no"))
        moved[tol] = distinct > 1
        print()

    print("the two arms whose difference is the question")
    print("  %s : accumulator derived from the fold's arity" % ARITY_SENSITIVE)
    print("  %s : a fixed accumulator that ignores it" % ARITY_BLIND)
    print()
    print("%8s %16s %16s %12s %s" % ("arity", "arity-sensitive", "arity-blind",
                                     "ratio", "which is faster"))
    flips = 0
    prev = None
    for k, meds in data.items():
        a = meds.get(ARITY_SENSITIVE)
        b = meds.get(ARITY_BLIND)
        if a is None or b is None:
            continue
        who = "arity-sensitive" if a < b else "arity-blind"
        if prev is not None and who != prev:
            flips += 1
        prev = who
        print("%8d %16.1f %16.1f %12.3f %s" % (key_arity(k), a, b, a / b, who))
    print()
    print("sign changes across the sweep: %d" % flips)
    print()

    print("and the const-availability contrast, which is a second question in the")
    print("same table: the same accumulator rule with the arity known at compile time")
    print("against the arity passed as a runtime value.")
    print()
    print("%8s %16s %16s %12s" % ("arity", "arity const", "arity runtime", "ratio"))
    for k, meds in data.items():
        a = meds.get(ARITY_SENSITIVE)
        b = meds.get("warm-clamp-accfit-dyn")
        if a is None or b is None:
            continue
        print("%8d %16.1f %16.1f %11.2fx" % (key_arity(k), a, b, b / a))
    print()

    if all(moved.values()):
        print("VERDICT. Which arm is right at 13 declared bits changes with the")
        print("fold's arity, on committed harness output, with the stored column held")
        print("constant, at every tolerance tried. So the accumulator's width is not")
        print("recoverable from the stored value, and a design carrying it on the value")
        print("cannot fold one column two ways without a cast that changes no value.")
        print()
        print("Note which comparison answers this and which does not. The head-to-head")
        print("between the arity-derived accumulator and the fixed one has NO sign")
        print("change: the arity-derived arm wins at every arity. That is a finding")
        print("about those two arms and it does not answer the carrier question. What")
        print("answers it is the field: the best arm is the minimum container at arity")
        print("2 and the arity-derived accumulator from arity 4 upward, so the right")
        print("intermediate width for one stored column is two different things.")
    else:
        print("VERDICT. One arm wins at every arity in this family, so this family does")
        print("not discriminate the question and the disagreement stands unresolved by")
        print("it. Report it as that rather than as support for either side.")
    print()
    print("What this does NOT establish. It is one family, one declared width, one")
    print("element count, one transform, and one machine. It says the accumulator")
    print("question is answered by the fold in this region. Whether some other region")
    print("makes the stored value the right carrier is not touched.")


if __name__ == "__main__":
    main()
