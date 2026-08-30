#!/usr/bin/env bash
# Counts what a consumer TYPES at each site, off p04_five_spellings.rs, which
# compiles. Two numbers per site: the trimmed source line with the marker
# comment removed, and the type expression alone (between "= " and ";").
#
# Run: ./count.sh   Output committed as out/count.txt
set -euo pipefail
cd "$(dirname "$0")"
F=p04_five_spellings.rs

printf '%-22s %-6s %-6s  %s\n' SITE LINE TYPE SPELLING
printf '%-22s %-6s %-6s  %s\n' ---- ---- ---- --------
grep -n '// SITE-' "$F" | while IFS= read -r hit; do
  marker=$(printf '%s' "$hit" | sed -E 's|.*// (SITE-[A-Za-z0-9-]+).*|\1|')
  line=$(printf '%s' "$hit" | sed -E 's|^[0-9]+:||; s| *// SITE-[A-Za-z0-9-]+.*$||; s|^ *||; s| *$||')
  # the type expression: everything after the first "= " up to the trailing ";"
  ty=$(printf '%s' "$line" | sed -E 's|^[^=]*= ||; s|;$||')
  printf '%-22s %-6s %-6s  %s\n' "$marker" "${#line}" "${#ty}" "$line"
done
