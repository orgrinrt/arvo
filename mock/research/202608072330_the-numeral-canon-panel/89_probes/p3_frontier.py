#!/usr/bin/env python3
"""PROBE p3 (file 89). The const-gate frontier for the saturating verdict,
in accept/refuse terms, separating the two costs.

86's F3 measured the RING grid procedure's frontier as a box budget. The
saturating criterion has a different cost shape and it splits in two:

  VERDICT cost   2*(D+1) term evaluations, each of C node steps, so ~2*D*C
                 MIR-level steps. Independent of width.
  CHECKER cost   the rung-0 implementation check, which sweeps exhaustively
                 at widths 1..=Wb over L laws: ~L * 2^Wb * C. Independent of
                 the gated law, and paid ONCE for the whole library rather
                 than per law.

This probe pushes each separately against the default long_running_const_eval
guard and against the guard with the allow, and reports accept/refuse.

Pins the toolchain explicitly (rustup resolves from the CWD).
"""
import subprocess, tempfile, os, sys, time, json

RUSTC = ["rustup", "run", "nightly-2026-05-28", "rustc"]

VERDICT_TMPL = r'''
#![no_std]
#![no_main]
{allow}
const OP_X: u8 = 0; const OP_C: u8 = 1; const OP_ADD: u8 = 2; const OP_MUL: u8 = 3;
#[derive(Clone, Copy)] struct Node {{ op: u8, imm: u64 }}
const MAXN: usize = {maxn}; const STACK: usize = 16;
#[derive(Clone, Copy)] struct Term {{ nodes: [Node; MAXN], len: usize }}
const fn pow_term(d: u32) -> Term {{
    let mut t = Term {{ nodes: [Node {{ op: OP_X, imm: 0 }}; MAXN], len: 1 }};
    let mut i = 1;
    while i < d {{ t.nodes[t.len] = Node {{ op: OP_X, imm: 0 }}; t.nodes[t.len+1] = Node {{ op: OP_MUL, imm: 0 }}; t.len += 2; i += 1; }}
    t
}}
const fn umax(w: u32) -> u128 {{ if w >= 64 {{ u64::MAX as u128 }} else {{ (1u128 << w) - 1 }} }}
const fn eval(t: &Term, x: u128, w: u32) -> u128 {{
    let m = umax(w); let mut st = [0u128; STACK]; let mut sp: usize = 0; let mut i: usize = 0;
    while i < t.len {{
        let nd = t.nodes[i];
        if nd.op == OP_X {{ st[sp] = x; sp += 1; }}
        else if nd.op == OP_C {{ let c = nd.imm as u128; st[sp] = if c > m {{ m }} else {{ c }}; sp += 1; }}
        else {{ let b = st[sp-1]; let a = st[sp-2]; sp -= 2;
                let v = if nd.op == OP_ADD {{ let s = a + b; if s > m {{ m }} else {{ s }} }} else {{ let s = a * b; if s > m {{ m }} else {{ s }} }};
                st[sp] = v; sp += 1; }}
        i += 1;
    }}
    st[0]
}}
const fn degree(t: &Term) -> u32 {{
    let mut st = [0u32; STACK]; let mut sp: usize = 0; let mut i: usize = 0;
    while i < t.len {{ let nd = t.nodes[i];
        if nd.op == OP_X {{ st[sp] = 1; sp += 1; }}
        else if nd.op == OP_C {{ st[sp] = 0; sp += 1; }}
        else {{ let b = st[sp-1]; let a = st[sp-2]; sp -= 2; st[sp] = if nd.op == OP_MUL {{ a + b }} else if a > b {{ a }} else {{ b }}; sp += 1; }}
        i += 1; }}
    st[0]
}}
const fn verdict(a: &Term, b: &Term, w: u32) -> bool {{
    let m = umax(w); let da = degree(a); let db = degree(b);
    let d = if da > db {{ da }} else {{ db }} as u128;
    let hi = if d < m {{ d }} else {{ m }};
    let mut x: u128 = 0;
    while x <= hi {{ if eval(a, x, w) != eval(b, x, w) {{ return false; }} x += 1; }}
    true
}}
// E_D at the gated width 64: TRUE for D >= 64, so nothing exits early.
const A: Term = pow_term({d});
const B: Term = pow_term({d1});
const V: bool = verdict(&A, &B, 64);
const _: () = assert!(V, "verdict came out false, so the probe exited early and measures nothing");
#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! {{ loop {{}} }}
'''

CHECKER_TMPL = r'''
#![no_std]
#![no_main]
{allow}
const OP_X: u8 = 0; const OP_C: u8 = 1; const OP_ADD: u8 = 2; const OP_MUL: u8 = 3;
#[derive(Clone, Copy)] struct Node {{ op: u8, imm: u64 }}
const MAXN: usize = 64; const STACK: usize = 16;
#[derive(Clone, Copy)] struct Term {{ nodes: [Node; MAXN], len: usize }}
const fn pow_term(d: u32) -> Term {{
    let mut t = Term {{ nodes: [Node {{ op: OP_X, imm: 0 }}; MAXN], len: 1 }};
    let mut i = 1;
    while i < d {{ t.nodes[t.len] = Node {{ op: OP_X, imm: 0 }}; t.nodes[t.len+1] = Node {{ op: OP_MUL, imm: 0 }}; t.len += 2; i += 1; }}
    t
}}
const fn umax(w: u32) -> u128 {{ if w >= 64 {{ u64::MAX as u128 }} else {{ (1u128 << w) - 1 }} }}
const fn eval(t: &Term, x: u128, w: u32) -> u128 {{
    let m = umax(w); let mut st = [0u128; STACK]; let mut sp: usize = 0; let mut i: usize = 0;
    while i < t.len {{
        let nd = t.nodes[i];
        if nd.op == OP_X {{ st[sp] = x; sp += 1; }}
        else if nd.op == OP_C {{ let c = nd.imm as u128; st[sp] = if c > m {{ m }} else {{ c }}; sp += 1; }}
        else {{ let b = st[sp-1]; let a = st[sp-2]; sp -= 2;
                let v = if nd.op == OP_ADD {{ let s = a + b; if s > m {{ m }} else {{ s }} }} else {{ let s = a * b; if s > m {{ m }} else {{ s }} }};
                st[sp] = v; sp += 1; }}
        i += 1;
    }}
    st[0]
}}
const fn degree(t: &Term) -> u32 {{
    let mut st = [0u32; STACK]; let mut sp: usize = 0; let mut i: usize = 0;
    while i < t.len {{ let nd = t.nodes[i];
        if nd.op == OP_X {{ st[sp] = 1; sp += 1; }}
        else if nd.op == OP_C {{ st[sp] = 0; sp += 1; }}
        else {{ let b = st[sp-1]; let a = st[sp-2]; sp -= 2; st[sp] = if nd.op == OP_MUL {{ a + b }} else if a > b {{ a }} else {{ b }}; sp += 1; }}
        i += 1; }}
    st[0]
}}
const fn verdict(a: &Term, b: &Term, w: u32) -> bool {{
    let m = umax(w); let da = degree(a); let db = degree(b);
    let d = if da > db {{ da }} else {{ db }} as u128;
    let hi = if d < m {{ d }} else {{ m }};
    let mut x: u128 = 0;
    while x <= hi {{ if eval(a, x, w) != eval(b, x, w) {{ return false; }} x += 1; }}
    true
}}
const fn sweep(a: &Term, b: &Term, w: u32) -> bool {{
    let m = umax(w); let mut x: u128 = 0;
    while x <= m {{ if eval(a, x, w) != eval(b, x, w) {{ return false; }} x += 1; }}
    true
}}
// the rung-0 implementation check: L laws, widths 1..=WB
const IMPL_CHECK: bool = {{
    let mut ok = true; let mut d: u32 = 2;
    while d <= {lmax} {{
        let a = pow_term(d); let b = pow_term(d + 1);
        let mut w: u32 = 1;
        while w <= {wb} {{ if verdict(&a, &b, w) != sweep(&a, &b, w) {{ ok = false; }} w += 1; }}
        d += 1;
    }}
    ok
}};
const _: () = assert!(IMPL_CHECK, "implementation check failed");
#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! {{ loop {{}} }}
'''

def build(src, cap=420.0):
    d = tempfile.mkdtemp()
    p = os.path.join(d, "probe.rs")
    open(p, "w").write(src)
    t0 = time.time()
    try:
        r = subprocess.run(RUSTC + ["--edition", "2021", "-O", "--crate-type=lib",
                                    "--out-dir", d, p],
                           capture_output=True, text=True, timeout=cap)
        el = time.time() - t0
        err = r.stderr
        if r.returncode == 0:
            return ("accept", el, "")
        if "long_running_const_eval" in err or "long-running" in err:
            return ("refuse-guard", el, "long_running_const_eval")
        return ("refuse", el, err.strip().splitlines()[0] if err.strip() else "")
    except subprocess.TimeoutExpired:
        return ("cap", cap, "probe cap %.0fs" % cap)

rows = []
print("VERDICT COST. law = E_D at gated width 64, verdict TRUE so nothing exits early.")
print("cost model: 2*(D+1) evaluations of a ~D-node term, ~2*D^2 node steps.")
print(f"{'D':>6} {'~node steps':>12} {'default':>14} {'with allow':>14}")
for d in [64, 128, 256, 512, 1024, 2048, 4096]:
    src = VERDICT_TMPL.format(allow="", maxn=2*(d+1)+4, d=d, d1=d+1)
    a = build(src)
    b = ("skipped", 0.0, "")
    if a[0].startswith("refuse-guard"):
        srcb = VERDICT_TMPL.format(allow="#![allow(long_running_const_eval)]", maxn=2*(d+1)+4, d=d, d1=d+1)
        b = build(srcb)
    steps = 2*(d+1)*(2*d)
    print(f"{d:>6} {steps:>12} {a[0]+' '+('%.1fs'%a[1]):>14} {(b[0] if b[0]!='skipped' else '-'):>14}")
    rows.append({"kind":"verdict","D":d,"steps":steps,"default":a[0],"default_s":round(a[1],2),"allow":b[0],"allow_s":round(b[1],2)})

print()
print("CHECKER COST. the rung-0 implementation check: E_d for d in 2..=L, widths 1..=WB.")
print("cost model: L * 2^WB * (term size), paid once for the library, not per law.")
print(f"{'L':>4} {'WB':>4} {'default':>16} {'with allow':>16}")
for (lmax, wb) in [(9,8),(9,10),(9,12),(9,14),(9,16),(16,12),(16,14),(32,12)]:
    src = CHECKER_TMPL.format(allow="", lmax=lmax, wb=wb)
    a = build(src)
    b = ("skipped", 0.0, "")
    if a[0].startswith("refuse-guard"):
        srcb = CHECKER_TMPL.format(allow="#![allow(long_running_const_eval)]", lmax=lmax, wb=wb)
        b = build(srcb)
    print(f"{lmax:>4} {wb:>4} {a[0]+' '+('%.1fs'%a[1]):>16} {(b[0]+' '+('%.1fs'%b[1]) if b[0]!='skipped' else '-'):>16}")
    rows.append({"kind":"checker","L":lmax,"WB":wb,"default":a[0],"default_s":round(a[1],2),"allow":b[0],"allow_s":round(b[1],2)})

json.dump(rows, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "p3_frontier.json"), "w"), indent=1)
print()
print("Seconds are an ad-hoc quick spike with no substance. Accept and refuse are the results.")
