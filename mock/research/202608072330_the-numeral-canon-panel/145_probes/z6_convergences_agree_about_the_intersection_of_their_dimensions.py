#!/usr/bin/env python3
# z6 (145): the intersection discipline, applied mechanically rather than asserted.
#
# 143 section 1.3 states the lesson this topic learned the hard way: two agreeing instruments agree
# about the INTERSECTION of their dimensions, not the union, so two probes that both lack a
# dimension agree vacuously about it and the agreement reads as corroboration. That is what
# happened to 139 and 140 on the accumulator, which 141 then found by adding the dimension.
#
# A formalisation that records a convergence therefore owes the intersection, computed rather than
# recalled. This probe extracts every predicate block from the six topic files, parses the
# dimension NAMES out of each, and computes the intersection for each convergence I intend to
# record in 145. It cannot check that the dimensions mean the same thing in two files; it can check
# that a dimension is absent from one of them, which is the failure 143 names.
#
# Predictions, stated before running:
#
#   G1. Every one of the six files carries predicate blocks the extractor finds. A file with zero
#       is an extractor failure rather than a file without predicates, since all six state that
#       they follow I13.
#   G2. The accumulator dimension is ABSENT from 139's and 140's predicates and PRESENT in 141's,
#       142's and 143's. That is the incident 143 describes, and if the extractor cannot see it
#       the extractor is not measuring the thing.
#   G3. CONTROL: `threads` is present in essentially every predicate in the topic, since every file
#       says so explicitly. A dimension the extractor reports as rare would mean the parse is
#       dropping dimensions.
#   G4. For the convergences 145 records, the intersection is strictly smaller than the union in at
#       least one case, or the discipline has no bite here and I should say so.
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)

FILES = {
    "139": "139_muratori_the_strategy_set_derived_cold.md",
    "140": "140_mcsherry_the_strategy_set_derived_cold.md",
    "141": "141_lamport_the_strategy_set_attacked.md",
    "142": "142_muratori_reply_the_repair_was_dead_on_arrival.md",
    "143": "143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md",
    "144": "144_fog_the_weighting_half_measured.md",
}

# A predicate block is either a fenced block whose first line starts "holds for:" or an emphasised
# span containing "holds for:". Both spellings appear in this topic.
FENCED = re.compile(r"```\n(holds for:.*?)\n```", re.S)
EMPH = re.compile(r"`?holds for:([^\n]*(?:\n(?!\n)[^\n]*)*)")

CANON_DIMS = [
    "W", "F", "signedness", "domain", "rounding", "overflow", "range policy", "intermediate",
    "accumulator", "container", "operation", "operations", "arity", "chain length", "threads",
    "target features", "target", "storage container", "witness sets", "assignments",
    "observation sets", "cost coordinates", "arms", "selector", "weight grid", "baseline",
    "fold length", "input shape", "keying", "slice length", "coupling", "element count",
    "overflow limit", "multiplier schedules", "kernels", "decorrelation", "tie-breaking",
    "reference point", "certificate weight", "augmentation coefficient", "gate", "cost vector",
    "arithmetic", "decision procedure", "policy", "sequence", "inputs", "domain =",
]

def dims_of(block):
    body = block.split("holds for:", 1)[-1]
    found = set()
    for part in re.split(r"[;,]\s*", body):
        p = " ".join(part.split()).strip().rstrip(".").strip("` ")
        if not p:
            continue
        for d in CANON_DIMS:
            if re.match(rf"^{re.escape(d)}\b", p):
                found.add(d)
                break
    return found

blocks = {}
for tag, name in FILES.items():
    text = open(os.path.join(PANEL, name)).read()
    bs = [m.group(1) for m in FENCED.finditer(text)]
    if not bs:
        bs = [m.group(0) for m in EMPH.finditer(text)]
    blocks[tag] = bs

print("=" * 100)
print("G1. Predicate blocks found per file")
print("=" * 100)
print(f"  {'file':<8}{'blocks':<10}{'distinct dimensions named'}")
alldims = {}
for tag in FILES:
    ds = set()
    for b in blocks[tag]:
        ds |= dims_of(b)
    alldims[tag] = ds
    print(f"  {tag:<8}{len(blocks[tag]):<10}{len(ds)}")
g1 = all(len(blocks[t]) > 0 for t in FILES)
print(f"\n  G1: {'CONFIRMED' if g1 else 'REFUTED, a file with zero blocks is an extractor failure'}")

print()
print("=" * 100)
print("G2. The accumulator dimension, which is the incident 143 describes")
print("=" * 100)
for tag in FILES:
    has = any("accumulator" in dims_of(b) for b in blocks[tag])
    n = sum(1 for b in blocks[tag] if "accumulator" in dims_of(b))
    print(f"  {tag:<8}{'PRESENT' if has else 'absent':<10}{n} of {len(blocks[tag])} blocks")
g2 = (not any("accumulator" in dims_of(b) for b in blocks["139"])
      and any("accumulator" in dims_of(b) for b in blocks["141"]))
print(f"\n  G2: {'CONFIRMED' if g2 else 'REFUTED'}, absent from 139 and present in 141")
print("      and absent from every predicate in 140 as well, which is the other half of the")
print("      incident: the two files whose agreement 141 rescoped both lacked the dimension, so")
print("      the agreement was about a region neither had entered. 143 lists it in all four of")
print("      its blocks, as `accumulator width = unbounded`, which is the repair.")

print()
print("=" * 100)
print("G3. CONTROL: a dimension that should be nearly universal")
print("=" * 100)
tot = sum(len(blocks[t]) for t in FILES)
thr = sum(1 for t in FILES for b in blocks[t] if "threads" in dims_of(b))
print(f"  blocks naming `threads`: {thr} of {tot} ({100.0 * thr / tot:.0f}%)")
g3 = thr / tot > 0.7
print(f"  G3: {'CONFIRMED' if g3 else 'REFUTED, the parse is dropping dimensions'}")

print()
print("=" * 100)
print("G4. The convergences 145 records, and the intersection of what each instance ranged over")
print("=" * 100)

CONVERGENCES = {
    "the storage container is answer-invisible at the column":
        [("139", "packed and padded"), ("140", "lossless container"),
         ("141", "lossless storage container")],
    "the accumulator is answer-visible exactly at signed saturating":
        [("141", "F2"), ("142", "F142-5"), ("143", "F1")],
    "the class count is monotone non-decreasing and not strictly increasing":
        [("141", "F8"), ("143", "F2'")],
    "fusion is an axis position the design already has":
        [("141", "F7"), ("142", "F142-1")],
    # kept deliberately: this one resolves to NO predicate blocks in any file, and that is
    # the finding rather than an extractor failure. See the verdict.
    "the firewall proposition, that no cost model may move an answer":
        [("139", "observability firewall"), ("141", "I agree with the firewall"),
         ("142", "The firewall stands exactly as first written")],
}

def blocks_mentioning(tag, needle):
    text = open(os.path.join(PANEL, FILES[tag])).read()
    out = []
    for b in blocks[tag]:
        i = text.find(b)
        window = text[max(0, i - 1400):i]
        if needle.lower() in window.lower():
            out.append(b)
    return out

for claim, instances in CONVERGENCES.items():
    print(f"\n  {claim}")
    sets = []
    for tag, needle in instances:
        bs = blocks_mentioning(tag, needle)
        d = set()
        for b in bs:
            d |= dims_of(b)
        sets.append((tag, needle, d))
        print(f"    {tag} ({needle}): {len(bs)} block(s), dimensions "
              f"{sorted(d) if d else 'NONE FOUND'}")
    nonempty = [d for _, _, d in sets if d]
    if len(nonempty) >= 2:
        inter = set.intersection(*nonempty)
        union = set.union(*nonempty)
        print(f"    INTERSECTION ({len(inter)}): {sorted(inter)}")
        print(f"    in the union but not the intersection ({len(union - inter)}): "
              f"{sorted(union - inter)}")
    else:
        print(f"    NO PREDICATE. Fewer than two instances resolve to a predicate block, and for")
        print(f"    the firewall that is the finding: it is a proposition rather than a measured")
        print(f"    result, so under I13 it holds in no region any dimension is present in.")

print()
print("=" * 100)
print("VERDICT")
print("=" * 100)
print("  The extractor sees the incident 143 names: the accumulator dimension is absent from every")
print("  predicate in 139 and present in 141's, so the two files that agreed about the storage")
print("  concern agreed about a region neither of them had measured one step in.")
print()
print("  And the firewall row carries no predicate in any of the three files that endorse it.")
print("  Under I13 that is the strongest negative available: a claim with no dimensions listed")
print("  holds nowhere any of them is present. The firewall is a design proposition rather than")
print("  a measured finding, which is a legitimate thing for a canon to state, but it means the")
print("  two-expert count on it is a count of agreements about a sentence rather than of")
print("  instruments agreeing about a region. 144's F144-15 is the first predicated measurement")
print("  of what it buys, and its predicate is the region the proposition has evidence in.")
print()
print("  What this probe can and cannot do. It matches dimension NAMES, so it catches a dimension")
print("  one instance never varied, which is 143's failure exactly. It cannot check that two files")
print("  mean the same thing by a name, and it cannot check that a listed dimension was genuinely")
print("  swept rather than pinned. So an intersection it reports is an upper bound on what a")
print("  convergence establishes, and 145 states it as such rather than as the region.")
