#!/usr/bin/env bash
# Is the ratified warrant marker used by any committed predicate entry?
#
# `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` is ratified
# and gives a predicate entry the form `<axis>: <span>: <token>, <clause>` with
# three tokens: `swept`, `construction`, `exhaustive`. `construction` is the one
# a compile-time result needs: an axis that cannot enter the argument at all.
#
# If no entry carries one, the mechanism the open item wants exists and is
# unused, which is a different finding from the mechanism being missing.
#
# Controls, outcomes written before the run:
#   W1  the grep must find the ruling itself, or it cannot see the file at all.
#   W2  a token nobody defined must find nothing (`phase_of_the_moon`).
#   W3  the entry matcher must find entries at all: count every predicate entry
#       first, and a zero there means the matcher is broken rather than the
#       corpus empty.
set -euo pipefail
cd "$(dirname "$0")"
REG=../../../registry

echo "### W1, the ruling is visible to this grep"
grep -c 'the_warrant_is_a_token_and_a_clause_on_the_values_side' "$REG/ruling.toml" | sed 's/^/  hits in ruling.toml: /'

echo
echo "### W3, predicate entries in the corpus at all"
# An entry is a quoted array element whose text starts `<slug>: `.
grep -hoE '"[a-z_]+: [^"]*"' "$REG"/proposal.toml "$REG"/proposal-the-later-topics.toml "$REG"/law.toml "$REG"/law-the-later-topics.toml > entries.txt || true
printf '  entries matched: %s\n' "$(wc -l < entries.txt | tr -d ' ')"
printf '  distinct axes named: %s\n' "$(sed -E 's/^"([a-z_]+): .*/\1/' entries.txt | sort -u | wc -l | tr -d ' ')"

echo
echo "### entries carrying a warrant token on the values side"
for tok in swept construction exhaustive; do
  n=$(grep -cE ": ($tok)(,|\")" entries.txt || true)
  printf '  %-14s %s\n' "$tok" "$n"
done
echo "  --- any token at all:"
grep -nE ": (swept|construction|exhaustive)(,|\")" entries.txt | head -20 || echo "    none"

echo
echo "### W2, a token nobody defined"
grep -cE ": phase_of_the_moon" entries.txt || echo "  PASS, 0"
