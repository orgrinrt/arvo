#!/usr/bin/env python3
# x4 (136): sweeping every predicate in 132 for dimensions inherited from a neighbour rather
# than established for the claim they sit on.
#
# The class this looks for, now seen in both directions across two consecutive topics: the
# preceding topic OMITTED a domain dimension from nine predicates because the neighbouring
# clauses did not carry it, and this topic INHERITED one into two predicates because the
# neighbouring clauses did. Four instances, one mechanism: a predicate's dimensions copied from
# the clause above rather than read off the argument underneath.
#
# What this probe can and cannot do, stated up front because the distinction is the whole
# instrument. Inheritance leaves a mechanical fingerprint: a dimension whose value is identical
# to the adjacent clause's. That fingerprint is NECESSARY and NOT SUFFICIENT, because a genuinely
# established dimension can coincide with its neighbour's. So the probe flags candidates and the
# classification of each is a hand check against the source finding, recorded in 136 rather than
# here. What the probe adds over reading the file is that it cannot miss one.
#
# Predictions, stated before running:
#   R1. Every clause in 132 section 5 carries a predicate, and the extractor finds all of them.
#       Control: a deliberately malformed predicate must not be silently skipped.
#       *** THE FIRST VERSION OF THIS EXTRACTOR WAS WRONG AND THE CONTROL CAUGHT IT, AFTER
#       WHICH A HARDCODED VERDICT LINE HID THE CATCH. *** The key pattern was
#       `([A-Za-z_][A-Za-z_ ]*?)\s*(?:=|in|any)\b`, which matches the `in` INSIDE the word
#       `domain`, so every domain dimension parsed as a key named `doma` and the sweep
#       reported zero domain flags. That is a delimiter occurring in the content, and it
#       produced a confident False on the exact question the probe was built to answer. The
#       control returned False and the VERDICT block printed `True` because I had written the
#       literal rather than the variable, so the instrument reported itself sound while
#       failing. Both are fixed below; the operator now requires whitespace before it and the
#       verdict prints the measured value.
#       *** AND THE FIXED VERSION STILL MISSED A PREDICATE, CAUGHT THE SAME WAY. *** The span
#       pattern required the literal `holds for:`, and 132 section 5.6's uniqueness predicate
#       reads `The uniqueness and the impossibility hold for:`, a plural subject. So the sweep
#       reported nine predicates where there are ten, and the missing one is the single
#       predicate 135's dissent is actually about. Two misses in one extractor, both from a
#       pattern written against the shape I expected rather than the shape on the page, in the
#       probe built to catch exactly that. Fixed to `holds? for:` with a control below.
#   R2. The two 135 named (5.6 and 5.7's domain dimension) are flagged.
#       *** THE ADJACENT-NEIGHBOUR VERSION OF THIS FINGERPRINT FAILED TO FLAG EITHER. *** 5.6's
#       uniqueness predicate does carry `domain closed under negation`, but its immediate
#       neighbour 5.5 carries `domain any`, so a verbatim match against the PRECEDING clause
#       finds nothing: the inheritance is from 5.2 through 5.4, three clauses up. And 5.7
#       carries no domain dimension at all, so no match with anything could flag it. Widened
#       below to compare against every earlier predicate, and the absence case is reported
#       separately because a missing dimension cannot be caught by a matching heuristic at all.
#   R3. The flag set is strictly larger than two, because coincidence is common, which is why
#       the hand check is required and why a bare flag count is not a finding.
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
CAND = os.path.join(PANEL, "132_leroy_the_canon_candidate_for_the_rounding_axis.md")

text = open(CAND).read()

# A predicate is an emphasised span containing "holds for:" and a semicolon-separated list.
PRED = re.compile(r"\*([^*]*?holds? for:.*?)\*", re.S)
SECTION = re.compile(r"^### (5\.\d+)\s+(.*)$", re.M)

sections = [(m.group(1), m.group(2), m.start()) for m in SECTION.finditer(text)]

def section_of(pos):
    cur = ("?", "?")
    for num, title, start in sections:
        if start <= pos:
            cur = (num, title)
        else:
            break
    return cur

def dims(pred_text):
    body = re.split(r"holds? for:", pred_text, maxsplit=1)[1]
    out = {}
    for part in body.split(";"):
        part = part.strip().rstrip(".").strip()
        if not part:
            continue
        # the operator must be a separate word, or `in` matches inside `domain`
        m = re.match(r"(.+?)\s+(=|in|any)\b(.*)$", part, re.S)
        if m:
            key = " ".join(m.group(1).split())
            out[key] = " ".join(part[len(m.group(1)):].split())
        else:
            out[" ".join(part.split())] = "(unparsed)"
    return out

preds = []
for m in PRED.finditer(text):
    num, title = section_of(m.start())
    preds.append((num, title, m.group(1).strip(), dims(m.group(1))))

print("=" * 96)
print(f"R1. Predicates found in 132: {len(preds)}")
print("=" * 96)
for num, title, _, d in preds:
    print(f"  {num:<6} {title[:44]:<46} {len(d)} dimensions: {', '.join(sorted(d))}")

# control: a malformed predicate must not be skipped silently
ctrl = "holds for: W any; NOTADIMENSION"
ctrl_dims = dims(ctrl)
ctrl_ok = "NOTADIMENSION" in ctrl_dims and ctrl_dims["NOTADIMENSION"] == "(unparsed)"
print(f"\n  control, a malformed dimension is surfaced not dropped: {ctrl_ok} (must be True),"
      f" parsed as {ctrl_dims.get('NOTADIMENSION')}")
# second control, the one that caught the parser bug: `domain` must not split at its own `in`
ctrl2 = dims("holds for: domain closed under negation; W any")
ctrl2_ok = any(k.startswith("domain") for k in ctrl2) and "doma" not in ctrl2
print(f"  control, `domain` is not split at the `in` inside it: {ctrl2_ok} (must be True),"
      f" keys = {sorted(ctrl2)}")
# third control, the one that caught the second miss: a plural-subject predicate must be found
ctrl3 = PRED.findall("*The uniqueness and the impossibility hold for: domain any; W any.*")
ctrl3_ok = len(ctrl3) == 1
print(f"  control, a plural-subject `hold for:` predicate is found: {ctrl3_ok} (must be True)")

print()
print("=" * 96)
print("R2/R3. Dimensions identical to the preceding clause's, the inheritance fingerprint")
print("=" * 96)
print("  NECESSARY, NOT SUFFICIENT. Every flag below is a candidate for the hand check in 136,")
print("  not a finding. Coincidence between neighbours is normal and expected.")
print()
flagged = []
for i in range(1, len(preds)):
    num, title, _, d = preds[i]
    for j in range(i):
        pnum, _, _, pd = preds[j]
        shared = {k: v for k, v in d.items() if k in pd and pd[k] == v}
        if shared:
            flagged.append((pnum, num, sorted(shared)))
            adj = " (adjacent)" if j == i - 1 else f" ({i - j} clauses up)"
            print(f"  {num:<6} shares with {pnum:<6}{adj:<20}: {', '.join(sorted(shared))}")

domain_flags = [f for f in flagged if any("domain" in s for s in f[2])]
print(f"\n  total flagged clause pairs: {len(flagged)}")
print(f"  of those, flagged on the domain dimension specifically: {len(domain_flags)}")
for a, b, s in domain_flags:
    print(f"    {b} inherits domain from {a}")

print()
print("=" * 96)
print("The domain dimension across every predicate, for the hand check")
print("=" * 96)
print(f"  {'clause':<8}{'the domain dimension as written':<62}{'dims':<6}")
absent = []
for num, title, _, d in preds:
    key = next((k for k in d if "domain" in k), None)
    if key is None:
        dom = "(ABSENT: under I13 this claims nothing where a domain is present)"
        absent.append(num)
    else:
        dom = key if d[key] == "(unparsed)" else f"{key} {d[key]}"
    print(f"  {num:<8}{dom[:60]:<62}{len(d):<6}")
print(f"\n  domain present in {len(preds) - len(absent)} of {len(preds)} predicates, "
      f"absent in {len(absent)}: {', '.join(absent)}")
print("  An absent dimension is not a hedge. Per I13 it is the strongest negative available:")
print("  the claim does not hold in any situation where that dimension is present at all. For")
print("  a claim about quantising numbers a domain is always present, so each of those is")
print("  vacuous as written, which is the same class 133 names for the three unclassifiable")
print("  pins and a more severe defect than the over-narrow predicate 135 reports.")

print()
print("=" * 96)
print("VERDICT")
print("=" * 96)
print(f"  R1 predicates extracted: {len(preds)}, malformed-dimension control: {ctrl_ok}, "
      f"domain-not-split: {ctrl2_ok}, plural-subject: {ctrl3_ok} "
      f"(all three must be True or nothing below counts)")
print(f"  R2 5.6 flagged on domain once the comparison reaches past the adjacent clause: "
      f"{any(b == '5.6' for _, b, s in domain_flags)}")
print(f"  R2 5.7 flagged: False, and it cannot be. It carries no domain dimension, so there is")
print(f"     nothing to match. A matching heuristic is blind to omission by construction, which")
print(f"     is why the absence table above is a separate instrument and not a refinement.")
print(f"  R3 domain flags: {len(domain_flags)}, absences: {len(absent)}. Neither number is a")
print(f"     finding on its own; each of the {len(domain_flags) + len(absent)} is checked by hand")
print(f"     against its source argument in 136.")
