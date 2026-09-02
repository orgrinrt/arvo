# p2. Is the preset table four values of one axis, or a product of two binary axes?
#
# Method: parse the strategy table straight out of arvo's own generated agent rule, so the
# input is the repository's text and not something I retyped. Split the Container cell into
# two independent choices (headroom, layout). Check whether the four names form a bijection
# with the 2x2 product. Then, separately, ask whether the Arithmetic cells all answer the
# same question.
#
# This is a spike. It proves one thing: the shape of that one table. It does not prove the
# axis list is complete.
import io, re, sys

RULE = "/Users/orgrinrt/Dev/clause-dev/arvo/.claude/rules/implementation.md"

s = io.open(RULE, encoding="utf-8").read()

# pull the markdown table whose header row mentions Strategy and Container
rows = []
for ln in s.splitlines():
    if not ln.strip().startswith("|"):
        continue
    cells = [c.strip() for c in ln.strip().strip("|").split("|")]
    if len(cells) == 4:
        rows.append(cells)

hdr = [r for r in rows if r[0].lower() == "strategy"]
if not hdr:
    print("FAIL: no strategy table found in %s" % RULE)
    sys.exit(1)
body = [r for r in rows if r[0].startswith("`")]

print("parsed %d data rows from %s" % (len(body), RULE))
for r in body:
    print("   %-22s | %-24s | %s" % (r[0], r[1], r[2]))
print()

# --- axis 1: headroom. does the container carry width beyond the numeral? ---
def headroom(container):
    if re.search(r"\b2x\b|doubl", container, re.I):
        return "doubled"
    if re.search(r"minimum", container, re.I):
        return "minimum"
    return "UNKNOWN"

# --- axis 2: layout. packed, or individually addressable? ---
def layout(container):
    if re.search(r"bitpack", container, re.I):
        return "bitpacked"
    if re.search(r"byte-aligned", container, re.I):
        return "byte-aligned"
    return "UNKNOWN"

grid = {}
for r in body:
    name = r[0].split()[0]          # drop the "(default)" annotation on Warm
    h, l = headroom(r[1]), layout(r[1])
    print("%-12s -> headroom=%-9s layout=%s" % (name, h, l))
    grid.setdefault((h, l), []).append(name)
print()

cells = [(h, l) for h in ("minimum", "doubled") for l in ("byte-aligned", "bitpacked")]
missing = [c for c in cells if c not in grid]
dupes = {c: v for c, v in grid.items() if len(v) > 1}
extra = [c for c in grid if c not in cells]

print("2x2 product cells        : %d" % len(cells))
print("cells occupied           : %d" % len([c for c in cells if c in grid]))
print("cells with >1 occupant   : %d %s" % (len(dupes), dupes if dupes else ""))
print("occupants outside the 2x2: %d %s" % (len(extra), extra if extra else ""))
bijection = (not missing) and (not dupes) and (not extra) and len(body) == 4
print()
print("BIJECTION between the four names and the 2x2 product: %s" % ("YES" if bijection else "NO"))
print()

# --- the arithmetic column: do all four cells answer the same question? ---
# "what happens when the result does not fit"  vs  "what precision does the intermediate carry"
OVERFLOW = r"wrap|satur|clamp"
INTERMEDIATE = r"widen|narrow"

print("arithmetic column, classified by which question the cell answers:")
ovf, itm, both, neither = [], [], [], []
for r in body:
    name = r[0].split()[0]
    a = r[2]
    o = bool(re.search(OVERFLOW, a, re.I))
    i = bool(re.search(INTERMEDIATE, a, re.I))
    tag = "overflow" if (o and not i) else "intermediate" if (i and not o) else "both" if (o and i) else "neither"
    (ovf if tag == "overflow" else itm if tag == "intermediate" else both if tag == "both" else neither).append(name)
    print("   %-12s %-34s -> answers: %s" % (name, a, tag))
print()
print("answers overflow only     : %s" % ovf)
print("answers intermediate only : %s" % itm)
print("answers both              : %s" % both)
print("answers neither           : %s" % neither)
print()
if ovf and itm:
    print("SPLIT CONFIRMED: the column contains cells answering two different questions.")
    print("  -> %s state an overflow policy and no intermediate precision." % ", ".join(ovf))
    print("  -> %s state an intermediate precision and no overflow policy." % ", ".join(itm))
else:
    print("NO SPLIT: every cell answers the same question.")
