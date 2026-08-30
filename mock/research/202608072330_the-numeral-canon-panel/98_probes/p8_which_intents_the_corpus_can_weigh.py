#!/usr/bin/env python3
"""p8. Which of op's stated intents the committed measurement corpus can weigh at all.

If a strategy is a weighting over measurements, then a strategy can only be
distinguished from another where the corpus varies a coordinate the two weigh
differently. That is not a philosophical point, it is a fact about what has been
measured, and it is checkable.

Op states four intents over four different things. Speed (I5). Storage (I6).
Accuracy, especially across chains (I7). Agreement with what a native Rust
primitive does (I3, I4).

A bench family measures time and, where the arms differ in carrier, footprint.
It measures accuracy only if its arms are allowed to give different answers.
This probe checks whether any of them are, by reading the cross-arm agreement
assertions the variant crates carry.

The prediction, stated before running: none of them are. A bench whose arms
disagree on the answer is measuring two different computations and its timing
comparison is meaningless, so every family has a test forcing agreement, and
those tests are exactly right. The consequence is not that the benches are
wrong. It is that the corpus has no accuracy coordinate anywhere, so two of the
four intents currently have nothing in it to weigh.

The prediction was too sweeping and the probe found the counterexample rather
than assuming it away: `bitpack-write-unsound` is an arm that may compute a
different answer, deliberately. It is recorded below with what it does and does
not change, which is that the difference it carries is a corruption rate from a
data race rather than a quantisation error, and that the rate lives in a stress
test rather than in a bench column. The narrower claim survives and is the one
stated.
"""

import os
import re
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
VARIANTS = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches", "variants"))

# Phrases that indicate a test forcing the arms of a family to compute the same
# value, or to match an independently computed reference.
AGREEMENT = re.compile(
    r"(arms?_agree|agrees?_with|all_.*_arms_agree|agrees|_oracle|ground.?truth|"
    r"reference|identical value|same value|cross.?valid)", re.I)


def crates():
    out = []
    for name in sorted(os.listdir(VARIANTS)):
        d = os.path.join(VARIANTS, name)
        if os.path.isdir(d) and name.endswith("-shared"):
            out.append((name, d))
    return out


def test_names(d):
    names = []
    for root, _, files in os.walk(os.path.join(d, "src")):
        for f in files:
            if not f.endswith(".rs"):
                continue
            path = os.path.join(root, f)
            src = open(path).read()
            for m in re.finditer(r"#\[test\]\s*(?:\n\s*)*fn\s+(\w+)", src):
                names.append((os.path.relpath(path, d), m.group(1)))
    return names


def main():
    print("cross-arm agreement assertions in the committed bench corpus\n")
    total = 0
    with_agreement = 0
    for name, d in crates():
        names = test_names(d)
        total += 1
        hits = [(f, n) for f, n in names if AGREEMENT.search(n)]
        # Also look inside the crate for an oracle or ground-truth comparison,
        # since a test may assert agreement without saying so in its name.
        body_hit = False
        for root, _, files in os.walk(os.path.join(d, "src")):
            for f in files:
                if f.endswith(".rs"):
                    src = open(os.path.join(root, f)).read()
                    if re.search(r"(oracle|ground truth|truth\b|reference)", src, re.I):
                        body_hit = True
        flag = bool(hits) or body_hit
        if flag:
            with_agreement += 1
        print(f"  {name:34s} tests {len(names):3d}   "
              f"{'FORCES AGREEMENT' if flag else 'no agreement assertion found'}")
        for f, n in hits[:3]:
            print(f"      {f}: {n}")
    print()
    print(f"  crates: {total}, with an agreement or oracle assertion: {with_agreement}")
    print()

    # One arm in the corpus is exempt from the agreement requirement, and it was
    # found by looking rather than assumed away. Recording it because the
    # sweeping form of the claim below would be false without it.
    unsound = os.path.join(VARIANTS, "bitpack-write-unsound", "src", "lib.rs")
    print("the one exception, and what it is and is not")
    if os.path.exists(unsound):
        head = open(unsound).read().splitlines()[:17]
        for i, line in enumerate(head):
            if line.startswith("//!"):
                print(f"    {i + 1:3d}  {line}")
    print()
    print("  So one arm may compute a different answer, on purpose, and its own")
    print("  header calls it a demonstration arm rather than a candidate. That is")
    print("  a CORRECTNESS difference from a data race, not an accuracy difference")
    print("  from quantisation, and its magnitude is measured in the crate's stress")
    print("  test as a corruption RATE over trials rather than as a bench column.")
    print("  A rate of wrong answers is exactly the shape of an accuracy")
    print("  coordinate, and it sits in a test rather than in the CSV, so no")
    print("  weighting can read it.")
    print()

    print("what follows")
    print("  A family whose arms are required to agree cannot measure an accuracy")
    print("  difference between them, because there is none to measure. That is")
    print("  correct bench design and it is the right requirement: without it the")
    print("  fast arm is fast because it is doing less.")
    print()
    print("  It does mean that no committed bench family carries a column for how")
    print("  wrong an arm is, nor one for how far an arm departs from a reference")
    print("  semantics. So of the four intents op has stated, two have coordinates")
    print("  in the committed corpus (time and footprint) and two do not. A")
    print("  weighting cannot distinguish strategies along a coordinate nobody has")
    print("  varied, so on this corpus the accuracy-first and the")
    print("  imitate-the-native-primitive intents have no expression at all.")
    print()
    print("  This is a gap in the corpus rather than in the design, and it is the")
    print("  kind op named as inside the panel's scope at `95`: a harness question")
    print("  that blocks pricing. What is missing is a family whose arms compute")
    print("  DIFFERENT answers on purpose, with the difference measured against an")
    print("  exact reference and reported as a column. `35`'s law probes measure")
    print("  exactly that quantity and are not bench arms; the write-unsound arm")
    print("  has exactly that property and reports no such column; nothing joins")
    print("  the two.")
    print()

    print("and the same question asked of the arms themselves")
    print("  In the carrier family used by p1 and p6, every arm decodes the same")
    print("  13-bit column and the crate asserts the decode against an independent")
    print("  ground truth, so the five or six arms differ in time and footprint and")
    print("  in nothing else. Any weighting over that family is a weighting over")
    print("  two coordinates, whatever the strategy claims to weigh.")

    # Show the concrete assertion in the carrier crate, since p1 and p6 rest on it.
    path = os.path.join(VARIANTS, "bitpack-carrier-shared", "src", "lib.rs")
    if os.path.exists(path):
        src = open(path).read().splitlines()
        for i, line in enumerate(src):
            if "disagrees with ground truth" in line:
                lo = max(0, i - 6)
                print(f"\n  {os.path.relpath(path, VARIANTS)}:{i + 1}")
                for j in range(lo, min(len(src), i + 2)):
                    print(f"    {j + 1:5d}  {src[j]}")
                break


if __name__ == "__main__":
    main()
