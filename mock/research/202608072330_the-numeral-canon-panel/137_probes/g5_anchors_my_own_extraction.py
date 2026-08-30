#!/usr/bin/env python3
"""g5: the anchor diff on my own patterns, both directions, with the
accounting sections excluded from BOTH sides.

136 reports 25/28 findings, 16/17 probe stems, 9/9 theorems, and a line row it
says needs reading. This rebuilds the sets rather than rerunning 136's runner,
because a shared instrument makes agreement uninformative.
"""
import re, pathlib
H = pathlib.Path(__file__).resolve().parent.parent

UNION = [f for f in sorted(H.glob("1[23][0-9]_*.md"))
         if re.match(r"1(2[5-9]|3[0-5])_", f.name)]
CAND = next(H.glob("136_*.md"))

def strip_acct(t):
    # drop any section whose heading names anchor accounting, to the next ## heading
    return re.sub(r"\n#{2,3} [^\n]*[Aa]nchor accounting.*?(?=\n## |\Z)", "\n", t, flags=re.S)

def anchors(t):
    return {
      "finding":    set(re.findall(r"\bF1[0-9]{2}-[0-9]+\b", t)) | set(re.findall(r"\bF12[0-9]-[0-9]+\b", t)),
      "theorem":    set(re.findall(r"\bT[0-9]b?\b", t)),
      "probe_stem": set(re.findall(r"\b([pqrsvwxyz][0-9]+[a-z]?)\b", t)),
      "line_panel": set(re.findall(r"\b(1[0-3][0-9]):([0-9]+)", t)),
    }

u = {}
for f in UNION:
    for k, v in anchors(strip_acct(f.read_text())).items():
        u.setdefault(k, set()).update(v)
c = anchors(strip_acct(CAND.read_text()))
c_raw = anchors(CAND.read_text())

print(f"union files: {len(UNION)}  ({UNION[0].name[:3]}..{UNION[-1].name[:3]})")
print("136 prose says 'the five preceding files of this topic plus the three")
print("signatures'; its table header says eleven-file union.")
print()
print(f"{'class':<12} {'union':>6} {'in 136':>7} {'carried':>8} {'dropped':>8} {'new in 136':>11}")
for k in ["finding", "theorem", "probe_stem", "line_panel"]:
    U, C = u.get(k, set()), c.get(k, set())
    print(f"{k:<12} {len(U):>6} {len(C):>7} {len(U&C):>8} {len(U-C):>8} {len(C-U):>11}")
print()
for k in ["finding", "theorem", "probe_stem"]:
    d = sorted(u.get(k, set()) - c.get(k, set()))
    print(f"DROPPED {k}: {d}")
print()
lp = sorted(u.get("line_panel", set()) - c.get("line_panel", set()))
print(f"DROPPED line_panel ({len(lp)}):")
for a, b in lp:
    origin = "PRECEDING topic" if int(a) < 125 else "THIS topic"
    print(f"   {a}:{b:<10} {origin}")
print()
print("=== the stripper guard: does excluding the accounting section change the count? ===")
for k in ["finding", "theorem", "probe_stem", "line_panel"]:
    print(f"  {k:<12} stripped={len(c.get(k,set())):>3}  unstripped={len(c_raw.get(k,set())):>3}"
          f"  delta={len(c_raw.get(k,set()))-len(c.get(k,set())):+d}")
print("  A positive delta is the guard's designed case: the accounting section")
print("  names the anchors it dropped, which makes them present.")
