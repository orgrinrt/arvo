#!/usr/bin/env nutshell
# Every claim item in `AGREEMENTS.md`, with the section and tier it sits under.
#
# The ledger is tiered and the tier decides whether an item is a candidate row
# at all. Its own opening: "Ranking follows the brief: op's own words;
# multi-instance agreements the source consolidation itself calls independent;
# single-expert claims a consolidation carried forward as if settled; contested
# or located disagreements; and explicitly closed or retired material." Four of
# those five are different things and only some are rows, so the extractor keeps
# the tier rather than flattening to a list of claims.
#
# Two item shapes, because the ledger has two. Sections 1 through 5 and 9
# through 13 use `- **Label**` bullets. Section 6 uses bolded paragraph leads
# with no bullet, which is why a bullet-only count reports section 6 as empty.
#
# CONTROLS, three.
#   POSITIVE-A section 1 must report 24 items. Counted independently by a bare
#     `grep -c` before this script existed, so a disagreement means the walker
#     is dropping or duplicating.
#   POSITIVE-B section 7 must report zero bullets and be present in the section
#     list, because it says "None found" in prose. An extractor that omits an
#     empty section cannot distinguish "no items" from "not walked".
#   NEGATIVE a heading that does not exist must not appear.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
led="$root/mock/research/202608072330_the-numeral-canon-panel/AGREEMENTS.md"
[ -f "$led" ] || { echo "ledger not found" >&2; exit 2; }
echo "### ledger: $(wc -l < "$led") lines"
echo

awk '
  /^## /  { s=$0; gsub(/^## /,"",s); t="(no subsection)"; seen[s]=1; order[++k]=s; next }
  /^### / { t=$0; gsub(/^### /,"",t); next }
  # bullet items
  /^- \*\*/ {
      line=$0; gsub(/^- \*\*/,"",line); sub(/\*\*.*/,"",line)
      printf "%-46s | %-44s | BULLET | %s\n", substr(s,1,44), substr(t,1,42), substr(line,1,74)
      c[s]++; next
  }
  # section 6 shape: a paragraph opening with a bolded lead and no bullet
  /^\*\*/ {
      line=$0; gsub(/^\*\*/,"",line); sub(/\*\*.*/,"",line)
      if (length(line) > 25) {
        printf "%-46s | %-44s | LEAD   | %s\n", substr(s,1,44), substr(t,1,42), substr(line,1,74)
        c[s]++
      }
      next
  }
  END {
      printf "\n######## items per section, in document order\n"
      for (i=1; i<=k; i++) printf "  %-58s %s\n", substr(order[i],1,56), (order[i] in c ? c[order[i]] : 0)
      tot=0; for (x in c) tot+=c[x]
      printf "  %-58s %s\n", "TOTAL", tot
  }
' "$led"

echo
echo "######## CONTROLS"
n1=$(awk '/^## 1\./{f=1;next} /^## 2\./{f=0} f&&/^- \*\*/{n++} END{print n+0}' "$led")
echo "  POSITIVE-A  section 1 bullets: $n1   (must be 24)"
n7=$(awk '/^## 7\./{f=1;next} /^## 8\./{f=0} f&&/^- \*\*/{n++} END{print n+0}' "$led")
s7=$({ grep -c '^## 7\.' "$led" || true; })
echo "  POSITIVE-B  section 7 present: $s7, bullets: $n7   (must be 1 and 0)"
nz=$({ grep -c '^## 99\. no such section' "$led" || true; })
echo "  NEGATIVE    a heading that does not exist: $nz   (must be 0)"
