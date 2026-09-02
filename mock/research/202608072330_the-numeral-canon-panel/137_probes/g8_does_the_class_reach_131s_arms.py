#!/usr/bin/env python3
"""g8: closing the gap I named rather than reporting it.

136's x4 sweeps 132 section 5. But 132 COMPRESSES 131 section 3's six arms, so
if a domain dimension is absent there it propagated rather than originated in
132, and the mechanism 136 names ('a predicate's dimensions read off the clause
above rather than off the argument underneath') would have a generation it does
not account for.

Control: 131 R0's predicate is known to carry 'domain any' explicitly, so a
sweep reporting every arm as absent is broken.
"""
import re, pathlib
H = pathlib.Path(__file__).resolve().parent.parent
t = (H / "131_leroy_formalising_the_rounding_axis.md").read_text()

a = t.index("## 3. The arms, stated exactly")
b = t.index("## 4. The bounds")
s3 = t[a:b]

arms = re.split(r"\n### (R[0-9][^\n]*)", s3)
rows = []
for i in range(1, len(arms), 2):
    name, body = arms[i], arms[i+1]
    m = re.search(r"\*holds for:(.*?)\*", body, re.S)
    pred = " ".join(m.group(1).split()) if m else ""
    rows.append((name.split(".")[0], pred))

print(f"arms found in 131 section 3: {len(rows)}   (131 states six: R0..R5 plus R6)")
print()
print(f"{'arm':<5} {'domain':<8} {'W':<4} {'F':<4} {'signedness':<11} predicate head")
print("-"*100)
absent = []
for name, pred in rows:
    has = lambda k: "yes" if re.search(rf"\b{k}\b", pred) else "NO"
    if has("domain") == "NO":
        absent.append(name)
    print(f"{name:<5} {has('domain'):<8} {has('W'):<4} {has('F'):<4} {has('signedness'):<11} {pred[:52]}")

print()
print(f"arms with NO domain dimension: {len(absent)} -> {absent}")
print()
print("CONTROL: R0 must show domain=yes, since 131 writes 'domain any' in it.")
r0 = dict(rows).get("R0", "")
print(f"  R0 predicate contains 'domain any': {'domain any' in r0}  (must be True)")
print()
print("Mapping to 132 section 5, which compresses these:")
print("  131 R3 (commutation)  -> 132 5.4   both predicates present in 131? see above")
print("  131 R5 (coupling)     -> 132 5.6 and 5.7")
print("  131 R6 (entropy-free) -> 132 5.8")
