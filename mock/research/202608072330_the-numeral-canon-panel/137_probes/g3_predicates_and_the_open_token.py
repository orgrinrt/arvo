#!/usr/bin/env python3
"""g3: the four vacuous predicates, checked two ways.

(1) Extract every predicate in 132 section 5 with my own pattern, independent of
    136's x4, and check the domain dimension myself. 136's x4 had four recorded
    defects, one of which made it miss a predicate, so its count is exactly the
    kind of thing to redo rather than accept.

(2) Check whether 136's own replacement predicates in section 7 obey op's
    instruction inside I13, which INTENTS.md records verbatim:
      "No adding 'unsure' into the predicate. Unsure or unmeasured etc
       explicitly go unstated and implicitly mean not true"
    and glosses as "It should write nothing there."
"""
import re, pathlib

H = pathlib.Path(__file__).resolve().parent.parent
t132 = (H / "132_leroy_the_canon_candidate_for_the_rounding_axis.md").read_text()
t136 = (H / "136_leroy_the_candidate_revised_against_three_signatures.md").read_text()

def section5(text):
    a = text.index("## 5. The statement")
    b = text.index("## 6. Three things that must stay visible")
    return text[a:b]

s5 = section5(t132)

# deliberately looser than x4's: accept "holds for" and "hold for", any lead-in,
# and take the span to the next blank line, so a plural subject cannot hide one.
preds = re.findall(r"\*?(?:The [^*\n]{0,60})?hold[s]? for:(.*?)(?:\n\n|\*\*Argument)", s5, re.S)
print(f"predicates found by my pattern in 132 section 5 : {len(preds)}   (136 x4 reports 11)")
print()
hdr = f"{'#':<3} {'domain':<9} {'W':<4} {'F':<4} {'signedness':<11} first 58 chars"
print(hdr); print("-"*len(hdr))
absent_domain = []
for i, p in enumerate(preds, 1):
    flat = " ".join(p.split())
    has = lambda k: "yes" if re.search(rf"\b{k}\b", flat) else "NO"
    if has("domain") == "NO":
        absent_domain.append(i)
    print(f"{i:<3} {has('domain'):<9} {has('W'):<4} {has('F'):<4} {has('signedness'):<11} {flat[:58]}")
print()
print(f"predicates with NO domain dimension: {len(absent_domain)} -> {absent_domain}")
print("136 section 5 reports four absences: 5.4 non-commutation, 5.6 variance,")
print("5.7 keying, 5.8 entropy.")

print()
print("=== (2) does 136 write a hedge token INTO a predicate? ===")
# a predicate span in 136 section 7
a = t136.index("## 7. The clauses that change")
b = t136.index("## 8. `134`'s addendum")
s7 = t136[a:b]
for m in re.finditer(r"\*?hold[s]? for:(.*?)\*\*Argument", s7, re.S):
    flat = " ".join(m.group(1).split())
    if "OPEN" in flat:
        print("  HEDGE TOKEN IN PREDICATE:")
        print(f"    {flat[:150]}")
print()
print("  occurrences of the token OPEN inside a predicate span in 136 sec 7:",
      sum(1 for m in re.finditer(r"\*?hold[s]? for:(.*?)\*\*Argument", s7, re.S) if "OPEN" in m.group(1)))
print()
print("  and 7.5's predicate, which has no **Argument** terminator:")
i = s7.index("### 7.5")
print("   ", " ".join(s7[i:i+700].split())[-220:])
