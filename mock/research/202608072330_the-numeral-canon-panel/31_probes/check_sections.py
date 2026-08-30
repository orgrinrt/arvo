# 31 p1. Mechanically check every `NN` section N.N citation in OPTIONS.md against the real
# heading structure of the file it cites. Run from the panel directory.
import re, glob, os

DIR = "."
FILES = {}
for fp in glob.glob(os.path.join(DIR, "[0-9][0-9]_*.md")):
    m = re.match(r"(\d\d)_", os.path.basename(fp))
    if m:
        FILES[m.group(1)] = fp

def headings(fp):
    out = set()
    for ln in open(fp, encoding="utf-8", errors="replace"):
        m = re.match(r"#{2,4}\s+([\d]+(?:\.[\d]+)*)\b", ln)
        if m:
            out.add(m.group(1))
    return out

HEAD = {k: headings(fp) for k, fp in FILES.items()}

s = open("OPTIONS.md", encoding="utf-8").read()
pat_sec = re.compile(r"\`(\d\d)\`(?: sections? ([\d.]+(?:, ?[\d.]+)*))")
secs = pat_sec.findall(s)

missing = []
checked = 0
for fnum, secstr in secs:
    if fnum not in FILES:
        missing.append((fnum, secstr, "FILE NOT FOUND"))
        continue
    for sec in [x.strip() for x in secstr.split(",")]:
        checked += 1
        if sec not in HEAD[fnum]:
            missing.append((fnum, sec, "HEADING NOT FOUND"))

print("total individual section refs checked:", checked)
print("missing:", len(missing))
for m in missing:
    print(m)
