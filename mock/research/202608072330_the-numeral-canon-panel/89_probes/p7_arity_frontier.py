#!/usr/bin/env python3
"""PROBE p7 (file 89). The arity frontier of the signed window gate: how far
n reaches before the const-eval guard refuses. The box for a sat_add fold is
2^n points, so this is the same box budget 86's F3 measured for multilinear
ring chain laws, on a different fragment and a cheaper per-point cost (n
saturating adds against a chain of multiplies).

Generated from p6's gate with N and the term builders parameterised. Nothing
exits early: the licensed law is TRUE, asserted."""
import subprocess, tempfile, os, time, json

RUSTC = ["rustup","run","nightly-2026-05-28","rustc"]
SRC = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "p6_signed_window_gate.rs")).read()

def variant(n, allow):
    s = SRC
    s = s.replace("const NV: usize = 8;", f"const NV: usize = {n};")
    s = s.replace("const MAXN: usize = 64;", f"const MAXN: usize = {4*n+16};")
    s = s.replace("const STACK: usize = 16;", f"const STACK: usize = {n+8};")
    s = s.replace("const N: usize = 8;", f"const N: usize = {n};")
    # the balanced tree is written for n = 8 only; the frontier uses left vs
    # right fold, which are defined at every n, so drop the tree assertion.
    s = s.replace("""        assert!(
            box_verdict(&LEFT, &TREE, N, T::WIDTH, WIN_LO, WIN_HI),
            "left fold and the balanced tree disagree at the gated width, decided there by the box criterion"
        );""", "")
    s = s.replace("const TREE: Term = tree_fold8();", "")
    if allow:
        s = s.replace("#![no_main]", "#![no_main]\n#![allow(long_running_const_eval)]")
    return s

def build(n, allow, cap=400.0):
    d = tempfile.mkdtemp(); p = os.path.join(d,"probe.rs"); open(p,"w").write(variant(n, allow))
    t0 = time.time()
    try:
        r = subprocess.run(RUSTC+["--edition","2021","-O","--crate-type=lib","--out-dir",d,p],
                           capture_output=True, text=True, timeout=cap)
        el = time.time()-t0
        if r.returncode == 0: return ("accept", el, "")
        if "long_running_const_eval" in r.stderr: return ("refuse-guard", el, "")
        first = r.stderr.strip().splitlines()[0][:80] if r.stderr.strip() else "?"
        return ("refuse", el, first)
    except subprocess.TimeoutExpired:
        return ("cap", cap, "")

print("ARITY FRONTIER of the signed window gate. law = left fold == right fold of")
print("signed saturating add over a declared non-negative window, gated width 64,")
print("box = 2^n points. Verdict TRUE, so nothing exits early.")
print(f"{'n':>4} {'box points':>12} {'default':>18} {'with allow':>18}")
rows=[]
for n in [8, 12, 14, 15, 16, 18, 20]:
    a = build(n, False)
    b = ("-",0.0,"")
    if a[0] == "refuse-guard":
        b = build(n, True)
    print(f"{n:>4} {1<<n:>12} {a[0]+' '+('%.1fs'%a[1]):>18} {(b[0]+' '+('%.1fs'%b[1]) if b[0]!='-' else '-'):>18}  {a[2]}{b[2]}")
    rows.append({"n":n,"box":1<<n,"default":a[0],"default_s":round(a[1],2),"allow":b[0],"allow_s":round(b[1],2),"note":a[2]+b[2]})
json.dump(rows, open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"p7_arity_frontier.json"),"w"), indent=1)
print()
print("Seconds are an ad-hoc quick spike with no substance. Accept and refuse are the results.")
