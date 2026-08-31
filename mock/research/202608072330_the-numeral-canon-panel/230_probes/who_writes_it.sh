#!/usr/bin/env bash
# For each candidate phrase: how many distinct personas write it inside a
# predicate span, and in which files.
#
# Why author count. `183_probes`'s ranking sweeps twelve files, of which the
# eight that produce spans are all one persona's (`wide_census.out` establishes
# both numbers). A phrase's rank there is a count of one author's habits. What
# bears on whether a phrase names a real axis is how many separate authors
# reached for it, since independence is what the two-instance and
# three-instance rules are about.
#
# Span extraction is deliberately looser than `183_probes/axis_census.sh`'s,
# because the corpus writes spans in more than one dialect: italic paragraphs,
# blockquoted lines, and set notation with the unicode member sign. This one
# takes the whole `holds for:` paragraph and searches it as text, which cannot
# split keys correctly and does not need to: it counts authors, not keys.
#
# Two arms. The LOOSE arm searches the paragraph for the phrase anywhere, which
# over-counts any phrase that is also an ordinary English word. The KEYED arm
# requires the phrase to be followed by an operator, which is what a predicate
# key looks like. Both are printed because the difference between them is
# itself the reading: a phrase whose loose count is far above its keyed count
# is being read out of prose.
#
# Controls, outcomes written before the run:
#   A1  a declared axis every topic uses must come back with many personas.
#       `threads` is the check; one persona would mean the paragraph grab is
#       broken.
#   A2  a phrase nobody wrote must come back with zero. `phase_of_the_moon`.
#   A3  a phrase only one file writes must come back with one persona, so the
#       count is not saturating.
#   A4  the keyed arm must be a subset of the loose arm on every phrase. A
#       keyed count above a loose one means the two are not reading the same
#       text.
set -euo pipefail
cd "$(dirname "$0")"
PANEL=..
out=who_out; rm -rf "$out"; mkdir -p "$out"

# One line per holds-for paragraph, prefixed with its file. Built once.
for f in "$PANEL"/*.md; do
  b=$(basename "$f")
  awk 'BEGIN{RS="";ORS="\n"} {gsub(/\n/," "); print}' "$f" \
    | { grep -E 'holds? for:' || true; } \
    | sed -E 's/^.*holds? for: //' \
    | sed -E 's/\*\*Argument kind.*//' \
    | sed -E 's/\*\*//g; s/`//g; s/\*//g; s/^> *//; s/ > / /g' \
    | sed "s|^|$b\t|"
done > "$out/paras.tsv"
echo "### $(grep -c . "$out/paras.tsv") holds-for paragraphs, from $(cut -f1 "$out/paras.tsv" | sort -u | grep -c .) files"
echo

pf() {  # $1 = the matching lines' file column, on stdin
  local files; files=$(cut -f1 | sort -u)
  local np nf
  nf=$(printf '%s\n' "$files" | grep -c . || true)
  np=$(printf '%s\n' "$files" | sed -E 's/^[0-9]+_//; s/_.*//' | sort -u | grep -c . || true)
  printf '%s\t%s\t%s' "$np" "$nf" "$(printf '%s\n' "$files" | sed -E 's/^([0-9]+)_([a-z]+)_.*/\1 \2/' | tr '\n' ';')"
}
loose() { grep -F -- "$1" "$out/paras.tsv" | pf; }
keyed() { grep -E "	.*(^|	|; |, )$1s?( =| in | any|:)" "$out/paras.tsv" | pf; }

PHRASES=(threads arms "cost coordinates" selector coupling "feature gates" \
  "observation set" "assignment set" "term shape" declarations restrictions \
  "discharge check" "overflow limit read at the declared width" \
  "F_intermediate" radix occupancy "ambient domain" "operand window" \
  const compile phase_of_the_moon)

printf '%-4s %-4s %-4s %-4s  %-42s %s\n' lP lF kP kF phrase 'where (loose)'
for p in "${PHRASES[@]}"; do
  IFS=$'\t' read -r lp lf lw <<<"$(loose "$p")"
  IFS=$'\t' read -r kp kf _  <<<"$(keyed "$p")"
  printf '%-4s %-4s %-4s %-4s  %-42s %s\n' "$lp" "$lf" "$kp" "$kf" "$p" "$lw"
done

echo
IFS=$'\t' read -r a1 _ _ <<<"$(loose threads)"
echo "### A1, a declared axis every topic uses must show many personas"
[ "$a1" -ge 5 ] && echo "  PASS, threads at $a1 personas" || echo "  FAIL, threads at $a1"
IFS=$'\t' read -r a2 _ _ <<<"$(loose phase_of_the_moon)"
echo "### A2, a phrase nobody wrote must show zero"
[ "${a2:-0}" = 0 ] && echo "  PASS" || echo "  FAIL, $a2"
IFS=$'\t' read -r a3 _ _ <<<"$(loose 'discharge check')"
echo "### A3, a one-author phrase must not saturate"
[ "$a3" = 1 ] && echo "  PASS, discharge check at 1 persona" || echo "  FAIL, $a3"
echo "### A4, the keyed arm must be a subset of the loose arm on every phrase"
bad=0
for p in "${PHRASES[@]}"; do
  IFS=$'\t' read -r _ lf _ <<<"$(loose "$p")"
  IFS=$'\t' read -r _ kf _ <<<"$(keyed "$p")"
  [ "${kf:-0}" -gt "${lf:-0}" ] && { echo "  FAIL on $p: keyed $kf > loose $lf"; bad=1; }
done
[ "$bad" = 0 ] && echo "  PASS on all $((${#PHRASES[@]})) phrases"
