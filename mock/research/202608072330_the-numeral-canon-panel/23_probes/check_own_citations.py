import io, os, re, glob, sys
# Resolved from this file's own location. It was absolute, naming a checkout
# that still exists on this host, so it did not fail when the arc moved: it
# resolved against a different tree and said nothing.
D = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
F = os.path.join(D, "23_spj_the_sentences_a_canon_could_carry.md")
s = io.open(F, encoding="utf-8").read()

# map NN -> filename
files = {}
for fp in glob.glob(os.path.join(D, "*.md")):
    b = os.path.basename(fp)
    m = re.match(r"^(\d\d)_", b)
    if m:
        files[m.group(1)] = fp
# `SETTLED.md` was archived and prefixed after this probe was written. Pointing
# at where it went rather than where it was is the reading rule the archive
# convention states; the file's content is unchanged, so what this checks is.
files["SETTLED.md"] = os.path.join(D, "archive", "OLD_SETTLED.md")
files["RULES.md"] = os.path.join(D, "RULES.md")

cites = set()
for m in re.finditer(r"`(\d\d):(\d+)(?:-(\d+))?`", s):
    cites.add((m.group(1), int(m.group(2)), int(m.group(3) or m.group(2))))
for m in re.finditer(r"`(SETTLED\.md|RULES\.md):(\d+)(?:-(\d+))?`", s):
    cites.add((m.group(1), int(m.group(2)), int(m.group(3) or m.group(2))))

bad = []
ok = 0
for (k, a, b) in sorted(cites):
    fp = files.get(k)
    if fp is None:
        bad.append((k, a, b, "NO SUCH PANEL FILE"))
        continue
    n = len(io.open(fp, encoding="utf-8").readlines())
    if b > n:
        bad.append((k, a, b, "OUT OF RANGE, file has %d lines" % n))
    else:
        ok += 1

print("distinct citations: %d" % len(cites))
print("in range: %d" % ok)
print("out of range or unresolvable: %d" % len(bad))
for r in bad:
    print("   %s:%d-%d  %s" % r)
