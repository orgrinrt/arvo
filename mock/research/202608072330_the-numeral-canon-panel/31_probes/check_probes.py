# 31 p2. Every `NN_probes/xxx` citation in OPTIONS.md, checked against disk.
import re, glob

s = open("OPTIONS.md", encoding="utf-8").read()
probe_refs = re.findall(r"\`(\d\d)_probes/([\w./-]+)\`", s)
print("total probe citations:", len(probe_refs))
missing = 0
for fnum, name in probe_refs:
    d = fnum + "_probes"
    name_clean = name.rstrip(".,")
    candidates = glob.glob(d + "/" + name_clean + "*")
    status = "OK" if candidates else "MISSING"
    if status == "MISSING":
        missing += 1
    print("%-10s %-40s %s" % (d, name_clean, status))
print()
print("missing:", missing, "of", len(probe_refs))
