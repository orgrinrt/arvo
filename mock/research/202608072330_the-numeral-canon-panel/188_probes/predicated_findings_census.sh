#!/usr/bin/env bash
# 182 section 1 measures region statements in the four CONSOLIDATIONS and finds
# eleven. Section 5.2 then corrects itself: the panel knows the regions, they are
# in the instruments, and the compression is where they were lost.
#
# This measures the population 182 never measured: every `holds for:` line in
# every MEMBER file of the four topics, which is where a fully predicated,
# already-portable finding would sit.
#
# CASE THAT MUST FAIL: control 1 runs the same pattern over a file type that
# provably carries none (the registry TOML, which writes predicates as TOML
# arrays and never as `holds for:`). Control 2 runs a pattern that must find
# something in every member file, so a zero above is about the pattern and not
# about the grep.
set -uo pipefail
cd "$(dirname "$0")/.."

# The member files of the four ported topics, by the ranges each consolidation
# declares it compresses. Read off the consolidations' own coverage sections.
MEMBERS_FORMAT="55 55b 56 57 57b 58 60 61 62"
MEMBERS_NUMSYS="65 66 67 68 70 71 72 73"
MEMBERS_LAWS="76 77 79 80 81 82 84 86 89"
MEMBERS_STRAT="93 94 97 98 100 101 102 103"

total=0
echo "=== \`holds for:\` lines per member file ==="
for grp in FORMAT:"$MEMBERS_FORMAT" NUMSYS:"$MEMBERS_NUMSYS" LAWS:"$MEMBERS_LAWS" STRAT:"$MEMBERS_STRAT"; do
  name=${grp%%:*}; list=${grp#*:}
  sub=0
  printf '\n-- %s --\n' "$name"
  for n in $list; do
    f=$(ls ${n}_*.md 2>/dev/null | head -1)
    [ -z "$f" ] && { printf '  %-6s (no file)\n' "$n"; continue; }
    c=$(grep -c 'holds for:' "$f" || true)
    sub=$((sub + c)); total=$((total + c))
    printf '  %-6s %-58s %3s\n' "$n" "$f" "$c"
  done
  printf '  %-65s %3s\n' "subtotal" "$sub"
done
echo
echo "=== TOTAL \`holds for:\` lines across the 34 member files: $total ==="
echo "=== against 182 section 1's count over the four consolidations:  11 ==="

echo
echo "=== CONTROL 1: the same pattern over the registry, which must be zero ==="
printf '  proposal.toml %s   law.toml %s\n' \
  "$(grep -c 'holds for:' ../../registry/proposal.toml || true)" \
  "$(grep -c 'holds for:' ../../registry/law.toml || true)"

echo
echo "=== CONTROL 2: a pattern that must be non-zero in every member file ==="
for n in 55 65 76 93; do
  f=$(ls ${n}_*.md 2>/dev/null | head -1)
  printf '  %-58s lines=%s\n' "$f" "$(wc -l < "$f" | tr -d ' ')"
done
