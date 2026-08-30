#!/usr/bin/env python3
"""PROBE p3b (file 89). The corrected checker-cost frontier.

p3's checker rows measured nothing above WB = 9 because the swept laws were
false there and both the verdict and the sweep exited at the first witness
(NOTE_p3_checker_half_first_run.md). Here every law in the band is TRUE at
every width swept, enforced by an assert inside the generated crate, so both
run to completion.

The rung-0 implementation check is the model band demoted to its one licensed
job (84 section 4): it validates the CHECKER's implementation, not any law's
verdict. Its cost is therefore paid once for a library rather than per law,
which is the point the frontier is being measured to establish.
"""
import subprocess, tempfile, os, time, json

RUSTC = ["rustup", "run", "nightly-2026-05-28", "rustc"]

TMPL = r'''
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
const fn sweep(a: &Term, b: &Term, w: u32) -> bool {{
    let m = umax(w); let mut x: u128 = 0;
    while x <= m {{ if eval(a, x, w) != eval(b, x, w) {{ return false; }} x += 1; }}
    true
}}
// L laws, all TRUE at every width in 1..=WB, so no sweep and no verdict can
// exit early. E_d has truth set 1..=d, so d starts at WB.
const IMPL_CHECK: bool = {{
    let mut ok = true; let mut n: u32 = 0;
    while n < {lcount} {{
        let d = {wb} + n;
        let a = pow_term(d); let b = pow_term(d + 1);
        let mut w: u32 = 1;
        while w <= {wb} {{
            let v = verdict(&a, &b, w);
            let s = sweep(&a, &b, w);
            if v != s {{ ok = false; }}
            if !s {{ ok = false; }}   // a false law would mean the row exits early and measures nothing
            w += 1;
        }}
        n += 1;
    }}
    ok
}};
const _: () = assert!(IMPL_CHECK, "check failed, or a swept law was false so the row measures nothing");
#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! {{ loop {{}} }}
'''

def build(lcount, wb, allow, cap=400.0):
    maxn = 2 * (wb + lcount + 2) + 4
    src = TMPL.format(allow="#![allow(long_running_const_eval)]" if allow else "",
                      maxn=maxn, lcount=lcount, wb=wb)
    d = tempfile.mkdtemp(); p = os.path.join(d, "probe.rs"); open(p,"w").write(src)
    t0 = time.time()
    try:
        r = subprocess.run(RUSTC + ["--edition","2021","-O","--crate-type=lib","--out-dir",d,p],
                           capture_output=True, text=True, timeout=cap)
        el = time.time()-t0
        if r.returncode == 0: return ("accept", el)
        if "long_running_const_eval" in r.stderr: return ("refuse-guard", el)
        return ("refuse:"+ (r.stderr.strip().splitlines()[0][:60] if r.stderr.strip() else "?"), el)
    except subprocess.TimeoutExpired:
        return ("cap", cap)

print("CHECKER COST, corrected. L laws (E_d, d = WB..WB+L-1), all TRUE at widths 1..=WB,")
print("so every sweep runs to completion. Cost model: L * 2^WB * (term size).")
print("This is the rung-0 implementation check, paid ONCE for a library, not per law.")
print(f"{'L':>4} {'WB':>4} {'sweep points':>14} {'default':>18} {'with allow':>18}")
rows = []
for (l, wb) in [(4,8),(4,10),(4,12),(4,14),(4,15),(4,16),(4,17),(8,14),(16,12),(1,18),(1,20)]:
    a = build(l, wb, False)
    b = ("-", 0.0)
    if a[0].startswith("refuse-guard"):
        b = build(l, wb, True)
    pts = l * (1 << wb)
    print(f"{l:>4} {wb:>4} {pts:>14} {a[0]+' '+('%.1fs'%a[1]):>18} {(b[0]+' '+('%.1fs'%b[1]) if b[0]!='-' else '-'):>18}")
    rows.append({"L":l,"WB":wb,"points":pts,"default":a[0],"default_s":round(a[1],2),"allow":b[0],"allow_s":round(b[1],2)})
json.dump(rows, open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"p3b_checker.json"),"w"), indent=1)
print()
print("Seconds are an ad-hoc quick spike with no substance. Accept and refuse are the results.")
