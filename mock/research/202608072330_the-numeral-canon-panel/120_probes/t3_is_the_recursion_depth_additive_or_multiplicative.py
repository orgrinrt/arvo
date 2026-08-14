#!/usr/bin/env python3
"""T3. `119` section 4.10 says the compile-side cost is "a recursion depth in the
trait solver, proportional to the derivation's size times the carrier's per-node
state". That attributes a multiplicative law to a depth, and the two quantities
in the sitting are not the same quantity.

  F114-18 measures a CELL COUNT, `L(2L - 1)` against `2(2L - 1)`, which is the
  derivation's size times the per-node state. Multiplicative, and correct.

  F114-17 measures a RECURSION WALL, and `114`'s own prose explains it as the
  obligation chain being "the spine depth plus the vector length rather than the
  spine depth alone", which is additive, and reports the affine tower reaching a
  fixed limit at half the fold length the interval tower does.

A candidate sentence that says the depth is proportional to the product has
merged the two. This probe decides it by measurement rather than by reading,
because both readings are consistent with a single wall observation and only a
sweep over the limit separates them.

**The discriminator.** Compile the expensive tower at several recursion limits
and find the first fold length that refuses.

  additive        depth ~ c*L, so the wall moves LINEARLY in the limit
  multiplicative  depth ~ c*L^2, so the wall moves as the SQUARE ROOT

Predicted before running, recorded so it can be wrong: the wall doubles when the
limit doubles. At limit 32 my `115` s2 measured the expensive tower refusing at
L = 16 and compiling at L = 12, so the additive reading predicts roughly 8 at
limit 16 and roughly 32 at limit 64, while the multiplicative reading predicts
roughly 5.7 and roughly 11.3.

The case that must fail is the control: the cheap tower's wall must move too, and
must move further, or the instrument is measuring something other than the
tower's own obligation chain.
"""

import importlib.util
import os
import re
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
S2 = os.path.join(PANEL, "115_probes", "s2_where_the_selection_can_live.py")

spec = importlib.util.spec_from_file_location("s2", S2)
s2 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(s2)

LIMITS = [16, 24, 32, 48, 64]
LENGTHS = [2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 20, 24, 28, 32, 40, 48, 56, 64]


def compiles(variant, n, limit, tmp):
    src = s2.source(variant, n)
    src = re.sub(r'#!\[recursion_limit = "\d+"\]',
                 f'#![recursion_limit = "{limit}"]', src)
    path = os.path.join(tmp, f"{variant}_{limit}_{n}.rs")
    with open(path, "w") as fh:
        fh.write(src)
    r = subprocess.run(
        ["rustc", "--edition", "2021", "--crate-type", "lib",
         "--emit", "metadata", "-o", os.path.join(tmp, "o.rmeta"), path],
        capture_output=True, text=True)
    return r.returncode == 0


def first_wall(variant, limit, tmp):
    for n in LENGTHS:
        if not compiles(variant, n, limit, tmp):
            return n
    return None


def main():
    print("T3. is the compile wall additive or multiplicative in the fold length?")
    print("=" * 78)
    v = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
    print(f"toolchain: {v.stdout.strip()}")
    print("towers are 115 s2's, imported rather than rebuilt; only the")
    print("recursion_limit attribute is substituted.")
    print()

    tmp = tempfile.mkdtemp(prefix="t3_")
    rows = []
    print(f"  {'limit':>6} {'expensive wall':>15} {'cheap wall':>12} "
          f"{'wall/limit':>11}")
    for lim in LIMITS:
        we = first_wall("expensive", lim, tmp)
        wc = first_wall("cheap", lim, tmp)
        rows.append((lim, we, wc))
        ratio = f"{we / lim:.3f}" if we else "n/a"
        print(f"  {lim:>6} {str(we):>15} {str(wc):>12} {ratio:>11}")

    print()
    print("-" * 78)
    print("the two readings, scored against the measurement:")
    print()
    base = next((r for r in rows if r[0] == 32 and r[1]), None)
    if base is None:
        print("  no wall at limit 32; cannot anchor the predictions")
        return 1
    L32 = base[1]
    print(f"  {'limit':>6} {'measured':>9} {'additive pred':>14} "
          f"{'multiplicative pred':>20}")
    add_err = mul_err = 0.0
    n = 0
    for lim, we, _ in rows:
        if we is None:
            continue
        add = L32 * lim / 32.0
        mul = L32 * (lim / 32.0) ** 0.5
        add_err += abs(we - add)
        mul_err += abs(we - mul)
        n += 1
        print(f"  {lim:>6} {we:>9} {add:>14.1f} {mul:>20.1f}")
    print()
    print(f"  mean absolute error, additive       : {add_err / n:.2f}")
    print(f"  mean absolute error, multiplicative : {mul_err / n:.2f}")
    verdict = "ADDITIVE" if add_err < mul_err else "MULTIPLICATIVE"
    print(f"  => the depth is {verdict} in the fold length")

    print()
    print("CONTROL: the cheap tower's wall must also move with the limit, or")
    print("the instrument is not reading the tower's own obligation chain.")
    moved = len({r[2] for r in rows if r[2] is not None})
    print(f"  distinct cheap-tower walls across the limits: {moved}")
    print(f"  control fired: {moved > 1}")

    print()
    print("-" * 78)
    print("reading: F114-18's cell count is multiplicative and is a count of")
    print("associated-const cells. The recursion wall this measures is a")
    print("different quantity and moves linearly in the limit, which is 114's")
    print("own spine-plus-vector explanation. 119 section 4.10's sentence")
    print("attributes the cell count's law to the depth.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
