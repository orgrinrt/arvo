#!/usr/bin/env bash
# Three of 229's claims, re-measured with a different reader.
#
# Why re-measure rather than read its output. 229 and I both report 527
# predicate entries and 19 named axes, from different scripts. That is worth
# something only if the two readers differ where it matters, so this one is
# `230_probes/entries.sh`, which recovers entries from arrays written on one
# line as well as from arrays spread over several. 229's reader is
# `axis_usage.txt`'s generator. If both give 527 the agreement is real; if only
# one handles the one-line shape then the agreement is a shared blind spot, and
# my own first reader had exactly that blind spot and reported 517.
#
# Claims under test:
#   V-b  zero of the 527 entries carry a warrant token (`swept`, `construction`,
#        `exhaustive`) on the values side.
#   V-c  `access_pattern`, `ambient_domain` and `occupancy` are declared and
#        named by no predicate entry.
#   V-d  nothing in `mock/lints/` or `mock/tools/` enforces append-only on
#        `dimension.toml`.
#
# Controls, outcomes written before the run:
#   P1  the entry total must be 527, which `ruling::the_warrant_is_a_token_and_
#       a_clause_on_the_values_side`'s own note reports from a third reader.
#   P2  the token matcher must find a planted token, or a zero means nothing.
#   P3  the unused-axis arm must report a used axis as used, or it reports
#       everything unused.
#   P4  the append grep must find a word that is there, or its zero is a fact
#       about the pipeline. `keywords` is the positive control.
set -uo pipefail
cd "$(dirname "$0")"
REG=../../../registry
bash ./entries.sh > verify_entries.tsv
n=$(grep -c . verify_entries.tsv)

echo "### V-b, warrant tokens on the values side"
echo "  entries: $n"
# The values side is everything after the first `<slug>: `. A token is one of
# three words in the position after a second colon, per the ratified form
# `<axis>: <span>: <token>, <clause>`.
vals() { grep -rhoE '"[a-z_]+: [^"]*"' "$REG"/*.toml; }
tot=$(vals | wc -l | tr -d ' ')
tok=$({ vals | grep -cE ': (swept|construction|exhaustive)\b' || true; })
echo "  values sides matched: $tot"
echo "  carrying a warrant token after a second colon: $tok"

echo "### P2, the matcher must find a planted token"
planted='"total_width: W in 3..=7: swept, the whole span"'
if printf '%s\n' "$planted" | grep -qE ': (swept|construction|exhaustive)\b'; then
  echo "  PASS, the planted token is found"
else
  echo "  FAIL, the matcher cannot see a token and its zero means nothing"
fi

echo
echo "### V-c, declared axes named by no predicate entry"
while read -r a; do
  c=$(awk -F'\t' -v a="$a" '$3==a{n++} END{print n+0}' verify_entries.tsv)
  [ "$c" = 0 ] && printf '  UNUSED   %s\n' "$a"
done < <(grep '^id = ' "$REG/dimension.toml" | sed 's/id = "//; s/"//')
echo "### P3, an axis in constant use must not be reported unused"
u=$(awk -F'\t' '$3=="total_width"{n++} END{print n+0}' verify_entries.tsv)
[ "$u" -gt 0 ] && echo "  PASS, total_width at $u" || echo "  FAIL"

echo
echo "### V-d, does anything enforce append-only on dimension.toml"
h=$({ grep -rn "append" ../../../lints/ ../../../tools/ 2>/dev/null || true; } | wc -l | tr -d ' ')
echo "  files/lines mentioning 'append' under mock/lints and mock/tools: $h"
echo "### P4, the same grep on a word that is there"
k=$({ grep -rln "keywords" ../../../lints/ 2>/dev/null || true; } | wc -l | tr -d ' ')
[ "$k" -gt 0 ] && echo "  PASS, 'keywords' found in $k files, so the grep works" \
               || echo "  FAIL, the grep finds nothing at all"

echo
echo "### P1, the entry total against a count from a third reader"
if grep -q "of 527 entries" "$REG/ruling.toml"; then
  [ "$n" = 527 ] && echo "  PASS, $n matches the ratified note's 527" \
                 || echo "  FAIL, $n against 527"
else
  echo "  INCONCLUSIVE, the note no longer carries a count"
fi

echo
echo "### V-d2, an independent route to the same question: which checks read the"
echo "###       dimension namespace at all, and does any of them compare the set"
echo "###       against a previous state. A grep for one word is one instrument;"
echo "###       this asks what the checks over that namespace actually do."
echo "  lints naming the dimension namespace:"
grep -rln '"dimension"' ../../../lints/ 2>/dev/null | sed 's|.*/|    |'
echo "  of those, any reading a stored or previous set (git, a baseline file, a"
echo "  snapshot), which is what an append-only check needs:"
h2=$({ grep -rlE 'git |baseline|snapshot|previous|was_declared|removed' \
       $(grep -rln '"dimension"' ../../../lints/ 2>/dev/null) 2>/dev/null || true; } | wc -l | tr -d ' ')
echo "    $h2"
echo "### P5, the same search on a lint that demonstrably reads history-like"
echo "###     state, so a zero above is not a fact about this pipeline."
if grep -rln 'ceiling\|ratchet' ../../../lints/ >/dev/null 2>&1; then
  echo "  PASS, ratchet-style state exists in the pack and the pattern finds it:"
  grep -rln 'ratchet' ../../../lints/ | sed 's|.*/|    |'
else
  echo "  FAIL, the pattern finds nothing anywhere and its zero means nothing"
fi
