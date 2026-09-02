import io, os, glob, re
# Resolved from this file's own location. It was absolute, naming a checkout
# that still exists on this host, so it did not fail when the arc moved: it
# resolved against a different tree and said nothing.
D = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
files = {}
for fp in glob.glob(os.path.join(D, "*.md")):
    b = os.path.basename(fp)
    m = re.match(r"^(\d\d)_", b)
    if m: files[m.group(1)] = fp
files["RULES"] = os.path.join(D, "RULES.md")

checks = [
    ("06", 341, 348, "D0"),
    ("06", 718, 718, "six sites"),
    ("12", 130, 158, "UInt<5>"),
    ("12", 62, 78, "does not say"),
    ("12", 459, 481, "priced"),
    ("17", 377, 379, "ceil"),
    ("18", 193, 200, "42"),
    ("13", 654, 662, "alias-definition site"),
    ("15", 288, 306, "impl"),
    ("16", 384, 384, "28"),
    ("16", 639, 642, "23.1"),
    ("11", 42, 43, "1148"),
    ("20", 99, 104, "44"),
    ("22", 604, 612, "operation is"),
    ("RULES", 99, 101, "Keeping something"),
]
for (k, a, b, needle) in checks:
    fp = files[k]
    lines = io.open(fp, encoding="utf-8").readlines()
    blob = "".join(lines[a-1:b])
    hit = needle.lower() in blob.lower()
    print("%-6s %4d-%-4d  %-22s  %s" % (k, a, b, needle, "OK" if hit else "MISS"))
    if not hit:
        print("      >>> " + " / ".join(x.strip() for x in lines[a-1:b] if x.strip())[:300])
