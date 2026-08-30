#!/usr/bin/env python3
"""p3b. Does the cost-table encoding still compile at table sizes a real design
would carry, and where does const evaluation stop.

`p3` compiles the region-indexed cost table plus const argmin at 6 regions, 5
arms and 2 coordinates, which is the committed carrier family's shape. A design
carrying every region a strategy is defined over is larger than that, and the
question a canon needs answered before it can even name the encoding as
available is whether const evaluation reaches those sizes at all.

This is an EXISTENCE question, which an ad-hoc spike is licensed to answer: does
it compile, yes or no, and where does it stop. It is NOT a how-much question.
No compile-time figure is reported here and none should be inferred; the
compile-time COST of this encoding is UNPRICED, and pricing it needs the
mockspace bench harness, which measures runtime variants and has no compile-time
arm today.

Two distinct const-evaluation loads, and they scale differently, which is the
part worth having:

  PER CALL SITE   `resolve(S::W, I)` inside an inline `const { }` block runs
                  once per monomorphisation and costs A * D steps. It does not
                  depend on R.
  ONCE PER CRATE  the `AGREEMENT` differential runs R * A * D steps, once.

So a design with many regions pays the region dimension once at the check and
never at a call site, which is the opposite of what "the cost table is bigger"
suggests.

Run:  python3 p3b_const_eval_scaling.py
"""

import os
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))

TEMPLATE = """#![no_std]
#![crate_type = "lib"]

pub const R: usize = {R};
pub const A: usize = {A};
pub const D: usize = {D};

pub const COST: [[[u32; D]; A]; R] = [{table}];

pub const fn resolve(w: [u32; D], r: usize) -> usize {{
    let mut best = 0usize;
    let mut best_score = u64::MAX;
    let mut a = 0usize;
    while a < A {{
        let mut s = 0u64;
        let mut k = 0usize;
        while k < D {{
            s += (w[k] as u64) * (COST[r][a][k] as u64);
            k += 1;
        }}
        if s < best_score {{
            best_score = s;
            best = a;
        }}
        a += 1;
    }}
    best
}}

pub trait Weighting {{
    const W: [u32; D];
}}
pub struct S1;
impl Weighting for S1 {{
    const W: [u32; D] = [{w}];
}}

pub const WINNER: [usize; R] = [{winner}];

const AGREEMENT: () = {{
    let mut r = 0usize;
    while r < R {{
        assert!(WINNER[r] == resolve(S1::W, r), "table disagrees with weighting");
        r += 1;
    }}
}};
const _: () = AGREEMENT;

#[inline(never)]
pub fn arm(i: usize, xs: &[u64]) -> u64 {{
    xs.len() as u64 + i as u64
}}

#[inline(always)]
pub fn fold<S: Weighting, const I: usize>(xs: &[u64]) -> u64 {{
    arm(const {{ resolve(S::W, I) }}, xs)
}}

#[unsafe(no_mangle)]
pub fn entry(xs: &[u64]) -> u64 {{
    fold::<S1, 0>(xs)
}}
"""


def cost(r, a, k):
    """Deterministic, spread out enough that the argmin is not always arm 0."""
    return 100 + ((r * 7919 + a * 104729 + k * 1299709) % 9001)


def build(R, A, D):
    rows = []
    for r in range(R):
        arms = []
        for a in range(A):
            arms.append("[" + ", ".join(str(cost(r, a, k)) for k in range(D)) + "]")
        rows.append("[" + ", ".join(arms) + "]")
    w = [1 + (k * 3) % 7 for k in range(D)]
    winner = []
    for r in range(R):
        best, bs = 0, None
        for a in range(A):
            s = sum(w[k] * cost(r, a, k) for k in range(D))
            if bs is None or s < bs:
                bs, best = s, a
        winner.append(best)
    return TEMPLATE.format(
        R=R,
        A=A,
        D=D,
        table=",\n".join(rows),
        w=", ".join(str(x) for x in w),
        winner=", ".join(str(x) for x in winner),
    )


def try_compile(R, A, D, extra=()):
    src = build(R, A, D)
    with tempfile.TemporaryDirectory() as td:
        p = os.path.join(td, "t.rs")
        with open(p, "w") as fh:
            fh.write(src)
        cmd = [
            "rustc",
            "--edition",
            "2024",
            "-O",
            "-C",
            "panic=abort",
            *extra,
            "--emit",
            "asm",
            "-o",
            os.path.join(td, "t.s"),
            p,
        ]
        proc = subprocess.run(cmd, capture_output=True, text=True)
        ok = proc.returncode == 0
        err = ""
        if not ok:
            first = [l for l in proc.stderr.splitlines() if l.startswith("error")]
            err = first[0] if first else proc.stderr.splitlines()[0]
        return ok, err, len(src)


def main():
    print(__doc__.split("Run:")[0].strip())
    print()
    print(
        f"{'R':>6} {'A':>5} {'D':>4} {'cells':>9} {'agreement steps':>16} "
        f"{'src bytes':>10}  result"
    )
    shapes = [
        (6, 5, 2),
        (6, 5, 4),
        (16, 8, 3),
        (64, 16, 4),
        (128, 32, 4),
        (256, 64, 4),
        (512, 64, 4),
        (1024, 64, 4),
    ]
    last_ok = None
    for R, A, D in shapes:
        ok, err, n = try_compile(R, A, D)
        cells = R * A * D
        print(
            f"{R:>6} {A:>5} {D:>4} {cells:>9} {R * A * D:>16} {n:>10}  "
            f"{'compiles' if ok else 'FAILS: ' + err}"
        )
        if ok:
            last_ok = (R, A, D)
    print()
    print(f"largest shape that compiles on this pin: {last_ok}")
    print()
    print(
        """
READING

The const-evaluation step limit is the wall this encoding would hit, and where
it sits is a fact about the toolchain rather than about the design. Two things
follow whatever the number turns out to be.

The per-call-site load does not carry the region dimension. `resolve(S::W, I)`
runs over the arms and coordinates only, so adding regions costs nothing at any
call site and costs one linear pass at the crate-level differential.

And `long_running_const_eval` is a default refusal rather than an absolute one
(`unstable-features.md`), so a design that did hit the limit has a documented
lever before it has a redesign. Reaching for it is a separate decision and this
probe does not propose it.

The compile-time COST of any shape above is UNPRICED. This says which shapes
exist, not what they cost.
"""
    )


if __name__ == "__main__":
    main()
