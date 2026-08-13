#!/usr/bin/env python3
"""PROBE p9. Attacking 80 section 4.2 inside the region this file's lifting names.

80 measured that at a shipped width the const evaluator produces only NEGATIVE
law verdicts: a false law exits at its first counterexample in 0.50s, and the
true verdict at the same width and arity is refused after 4.48s because the
whole domain has to be visited. Its conclusion, which the register carries as
Q38(a)'s cost: the mechanism produces exactly the verdict that licenses nothing.

That is measured over the FULL representable set. The lifting this file
constructs restricts the operand set to a sign-uniform half, and an exhaustive
arity-3 sweep over a half costs 2^(3(W-1)) rather than 2^(3W), which is a factor
of eight. So the question is whether one bit of restriction per operand moves a
positive verdict from refused to accepted, and at which width it stops.

For each width, three checks are emitted as top-level consts and compiled:

  full      the whole signed representable set, which is what 80 measured
  nonneg    the non-negative half, which is what a NonNeg declaration reaches
  nonpos    the non-positive half

Every check COUNTS violations rather than returning early, so the domain is
visited whatever the verdict is. That is the defect 80's own first run had, and
this probe is written to avoid it rather than to rediscover it.

Everything here is a compile-time acceptance measurement, not a benchmark. The
wall-clock seconds are an ad-hoc quick spike with no substance for any how-much
question; the accept/refuse outcome is the result.
"""
import subprocess, tempfile, os, time, json, sys

TEMPLATE = r'''
#![allow(dead_code)]

const fn sat_add(x: i64, y: i64, minv: i64, maxv: i64) -> i64 {{
    let s = x + y;
    if s > maxv {{ maxv }} else if s < minv {{ minv }} else {{ s }}
}}

// Counts, never returns early, so the whole domain is visited whatever the
// verdict turns out to be.
const fn violations(lo: i64, hi: i64, minv: i64, maxv: i64) -> u64 {{
    let mut n: u64 = 0;
    let mut a = lo;
    while a <= hi {{
        let mut b = lo;
        while b <= hi {{
            let mut c = lo;
            while c <= hi {{
                let l = sat_add(sat_add(a, b, minv, maxv), c, minv, maxv);
                let r = sat_add(a, sat_add(b, c, minv, maxv), minv, maxv);
                if l != r {{ n += 1; }}
                c += 1;
            }}
            b += 1;
        }}
        a += 1;
    }}
    n
}}

const W: u32 = {w};
const MAXV: i64 = (1i64 << (W - 1)) - 1;
const MINV: i64 = -(1i64 << (W - 1));

pub const RESULT: u64 = violations({lo}, {hi}, MINV, MAXV);

// Force the value to be observed so nothing can be skipped.
pub const _CHECK: () = {{
    assert!(RESULT != 0xFFFF_FFFF_FFFF_FFFF);
}};
'''

def run(w, kind, allow_guard):
    maxv = (1 << (w - 1)) - 1
    minv = -(1 << (w - 1))
    if kind == 'full':
        lo, hi = 'MINV', 'MAXV'
        n = maxv - minv + 1
    elif kind == 'nonneg':
        lo, hi = '0', 'MAXV'
        n = maxv + 1
    else:
        lo, hi = 'MINV', '0'
        n = -minv + 1
    src = TEMPLATE.format(w=w, lo=lo, hi=hi)
    if allow_guard:
        src = "#![allow(long_running_const_eval)]\n" + src
    with tempfile.TemporaryDirectory() as d:
        p = os.path.join(d, 'probe.rs')
        open(p, 'w').write(src)
        t0 = time.time()
        r = subprocess.run(
            ['rustc', '-O', '--crate-type=lib', '--emit=metadata',
             '-o', os.path.join(d, 'out.meta'), p],
            capture_output=True, text=True, timeout=300)
        dt = time.time() - t0
    ok = r.returncode == 0
    err = ''
    if not ok:
        for line in r.stderr.splitlines():
            if line.startswith('error'):
                err = line.strip()[:90]
                break
    return ok, dt, err, n ** 3


def main():
    print("p9: does restricting to a sign-uniform half move the POSITIVE verdict")
    print("    from refused to accepted at a shipped width\n")
    print(f"toolchain: {subprocess.run(['rustc','--version'],capture_output=True,text=True).stdout.strip()}")
    print("host:      aarch64-apple-darwin")
    print()
    print("The walk stops each (operand set, guard) combination at its first refusal,")
    print("because the domain grows monotonically with width and nothing wider can")
    print("accept once a narrower one has been refused. Per-compile cap is 300s: a")
    print("mechanism that needs longer than that per law is not a mechanism, and the")
    print("cap is recorded as part of the result rather than hidden.")
    print()
    rows = []
    dead = set()
    print(f"{'width':>5} {'operand set':>12} {'triples':>14} {'guard':>8} {'outcome':>10} {'wall s':>9}  diagnostic")
    for w in range(4, 13):
        any_alive = False
        for kind in ('full', 'nonneg', 'nonpos'):
            for allow in (False, True):
                key = (kind, allow)
                if key in dead:
                    continue
                any_alive = True
                try:
                    ok, dt, err, tuples = run(w, kind, allow)
                except subprocess.TimeoutExpired:
                    ok, dt, err = False, 300.0, 'TIMEOUT at the probe cap'
                    maxv = (1 << (w - 1)) - 1
                    n = (maxv + 1) if kind != 'full' else (2 * maxv + 2)
                    tuples = n ** 3
                rows.append(dict(width=w, set=kind, allow_guard=allow, tuples=tuples,
                                 accepted=ok, seconds=round(dt, 2), diagnostic=err))
                print(f"{w:>5} {kind:>12} {tuples:>14} "
                      f"{'allowed' if allow else 'default':>8} "
                      f"{'accept' if ok else 'REFUSE':>10} {dt:>9.2f}  {err}")
                if not ok:
                    dead.add(key)
        if not any_alive:
            break

    here = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(here, 'p9_output.json'), 'w') as f:
        json.dump(rows, f, indent=1)
    print("\nmachine-readable at p9_output.json")

    def widest(kind, allow):
        ws = [r['width'] for r in rows if r['set'] == kind and r['allow_guard'] == allow and r['accepted']]
        return max(ws) if ws else None

    print("\nwidest width at which the POSITIVE verdict is accepted:")
    print(f"{'operand set':>12} {'default guard':>15} {'guard allowed':>15}")
    for kind in ('full', 'nonneg', 'nonpos'):
        a = widest(kind, False); b = widest(kind, True)
        print(f"{kind:>12} {str(a):>15} {str(b):>15}")

    fw = widest('full', False); nw = widest('nonneg', False)
    if fw is not None and nw is not None:
        print(f"\nthe sign-uniform restriction moved the default-guard frontier from "
              f"width {fw} to width {nw}, which is {nw - fw} bit(s).")
    print("\nSeconds here are an ad-hoc quick spike with no substance for any how-much")
    print("question. The accept/refuse frontier is the result.")


if __name__ == '__main__':
    main()
