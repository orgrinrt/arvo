#!/usr/bin/env python3
"""u1. `120` section 5.1 reproduced on my own generator rather than on its.

THE DISSENT
-----------
`119:566` says the compile-side cost is "a recursion depth in the trait solver,
proportional to the derivation's size times the carrier's per-node state".
`120` says that attributes a multiplicative law to a depth, merging two
quantities the sitting kept apart: F114-18 measures a **cell count**, which is
`L(2L - 1)` against `2(2L - 1)` and is genuinely multiplicative, and F114-17
measures a **recursion wall**, whose explanation in `114` section 7.3 is that the
expensive tower's obligation chain is the spine depth **plus** the vector length.

`120`'s `t3` sweeps five recursion limits on `115`'s towers and reports the cheap
tower's wall at exactly the limit and the expensive tower's at exactly half of it,
mean absolute error 0.00 additive against 3.79 multiplicative.

A dissent about my own sentence is exactly the thing to reproduce rather than
accept, and reproducing it on `115`'s instrument would test my reading of their
file. So this runs the same question on **`114_probes/p9`'s own generator**, which
is mine, with only the recursion-limit attribute swept.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. The cheap tower's wall tracks the recursion limit linearly.
P2. The expensive tower's wall is about half the cheap tower's at every limit,
    because its obligation chain is the spine plus the vector rather than the
    spine alone.
P3. The additive model predicts the expensive wall with far smaller error than
    the multiplicative one, so `119:566` is wrong and `120` is right.
P4. The two towers' walls move together as the limit moves, which is the control:
    if the cheap tower's wall were fixed the instrument would be reading a
    property of the file rather than of the obligation chain.

NEGATIVE CONTROLS
-----------------
C1. P4 is the primary control: a fixed cheap wall means the sweep measures
    nothing.
C2. A variant naming neither tower must compile at every length at every limit,
    or the walls are about the generator's source size.
C3. The two models must give different predictions somewhere in the swept range,
    or scoring them against each other is vacuous.
"""

import importlib.util
import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).parent
P114 = HERE.parent / "114_probes"

spec = importlib.util.spec_from_file_location(
    "p9", P114 / "p9_pricing_the_two_spellings_without_a_clock.py"
)
p9 = importlib.util.module_from_spec(spec)
sys.modules["p9"] = p9
spec.loader.exec_module(p9)


def build(name, n, limit, tmp):
    """`114_probes/p9`'s own generator, with only the recursion limit varied."""
    body = p9.VARIANTS[name](n)
    if body.startswith("#!["):
        body = body.split("\n", 1)[1]
    src = tmp / f"{name}_{n}_{limit}.rs"
    src.write_text(f'#![recursion_limit = "{limit}"]\n' + p9.PRELUDE + body)
    r = subprocess.run(
        ["rustc", "--edition", "2021", "--crate-type", "lib", "-O", "--emit", "metadata",
         "-o", str(tmp / f"{name}_{n}_{limit}.rmeta"), str(src)],
        capture_output=True, text=True,
    )
    return r.returncode == 0, r.stderr


def wall(name, limit, tmp, lo=2, hi=400):
    """Largest fold length that still compiles, by bisection. Returns None if it
    compiles at `hi`, so a wall outside the search range is not reported as a
    number."""
    if build(name, hi, limit, tmp)[0]:
        return None
    if not build(name, lo, limit, tmp)[0]:
        return 0
    while lo + 1 < hi:
        mid = (lo + hi) // 2
        if build(name, mid, limit, tmp)[0]:
            lo = mid
        else:
            hi = mid
    return lo


def main():
    print("=" * 92)
    print("u1. Is the compile wall additive or multiplicative in the limit?")
    print("=" * 92)
    print()
    print(subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip())
    print()
    print("Generator: `114_probes/p9`, imported. Only the recursion-limit attribute")
    print("is varied. `corner` is the cheap tower, `affine` the expensive one.")
    print()

    limits = [16, 24, 32, 48, 64, 96]
    with tempfile.TemporaryDirectory() as td:
        tmp = Path(td)

        # ------------------------------------------------------------- C2
        print("C2. A variant forcing neither tower, at every limit.")
        neither = []
        for lim in limits:
            ok, _ = build("neither", 200, lim, tmp)
            neither.append("ok" if ok else "FAIL")
        print("    " + "  ".join(f"{l}:{s}" for l, s in zip(limits, neither)))
        print()

        # -------------------------------------------------------- P1 to P4
        print("P1 to P4. The wall for each tower at each limit.")
        print()
        print(f"  {'limit':>7} {'cheap wall':>11} {'expensive wall':>15} "
              f"{'cheap/limit':>12} {'expensive/limit':>16}")
        rows = []
        for lim in limits:
            c = wall("corner", lim, tmp)
            e = wall("affine", lim, tmp)
            rows.append((lim, c, e))
            cs = "none" if c is None else str(c)
            es = "none" if e is None else str(e)
            cr = "-" if c is None else f"{c / lim:.3f}"
            er = "-" if e is None else f"{e / lim:.3f}"
            print(f"  {lim:>7} {cs:>11} {es:>15} {cr:>12} {er:>16}")

        # ----------------------------------------------------- the two models
        print()
        print("-" * 92)
        print("P3. The two candidate laws scored against the measurement.")
        print()
        print("  additive       : the expensive chain is spine + vector, so a fold of L")
        print("                   leaves costs about 2L and the wall is about limit / 2.")
        print("  multiplicative : the chain is spine TIMES per-node state, so the wall")
        print("                   would fall as the square root of the limit, fitted at")
        print("                   the middle of the swept range.")
        print()
        usable = [(l, c, e) for l, c, e in rows if c is not None and e is not None]
        if len(usable) >= 2:
            mid = usable[len(usable) // 2]
            k_mul = mid[2] / (mid[0] ** 0.5)
            print(f"  {'limit':>7} {'measured':>9} {'additive':>10} {'multiplicative':>15}")
            ea = em = 0.0
            for lim, c, e in usable:
                pa = lim / 2
                pm = k_mul * (lim ** 0.5)
                ea += abs(e - pa)
                em += abs(e - pm)
                print(f"  {lim:>7} {e:>9} {pa:>10.1f} {pm:>15.1f}")
            n = len(usable)
            print()
            print(f"  mean absolute error, additive       : {ea / n:.2f}")
            print(f"  mean absolute error, multiplicative : {em / n:.2f}")
            print()
            preds = {round(k_mul * (l ** 0.5), 1) for l, _, _ in usable}
            adds = {round(l / 2, 1) for l, _, _ in usable}
            print(f"  C3: the two models give {len(preds | adds)} distinct predictions over")
            print(f"      {n} limits, so scoring them against each other is not vacuous.")
        else:
            print("  fewer than two usable rows; the sweep did not locate the walls")

    print()
    print("=" * 92)
    print(
        """
  READING IT

  P1 and P4 hold when the cheap wall moves across the limits rather than sitting
  at one value. A fixed cheap wall means the instrument is reading the file.

  P2 holds when the expensive wall is about half the cheap one at every limit.

  P3 holds when the additive error is far below the multiplicative one. If it is,
  `119:566` attributes the wrong law and `120` is right.
"""
    )


if __name__ == "__main__":
    main()
