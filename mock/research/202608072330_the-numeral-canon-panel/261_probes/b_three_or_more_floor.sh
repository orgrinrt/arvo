#!/usr/bin/env bash
# B. Does the reachability floor scale with the arrival count it is checking?
#
# `mock/lints/a_standing_is_reachable_from_what_it_cites.rs` reasons per arrival:
# "Independence is between authors, and a numbered member file has one author,
# so one citation cannot exhibit two arrivals". Its predicate is
# `files.len() >= 2` for every member of
# `MULTI_ARRIVAL = ["two_experts", "three_or_more", "cross_topic"]`.
#
# Two is the right floor for two arrivals. This arm asks whether any committed
# row claims three or more arrivals while citing fewer than three files, which
# is the state the lint's own reasoning refuses and its predicate admits.
#
# The registry TOML is read directly rather than through the query renderer,
# because that renderer truncates a long `id` with an ellipsis and the first
# attempt at this arm silently saw one row of six. That is control B3 firing;
# the mechanism it tripped on is reproduced at `b_renderer_truncation.sh`, whose
# output shows five of the six ids rendered with an ellipsis.
#
# The second attempt read `provenance` as a single line and every row came back
# citing zero files. That is control B4 firing; its output is kept at
# `b_second_attempt.out`. `provenance` is a multi-line TOML array, so the reader
# below accumulates until the closing bracket.
#
# `file_named` is reimplemented here from `mock/lints/canon_citations.rs:66-77`:
# split on `::`, and where the last segment is an anchor take the one before it.
# An anchor is a heading beginning `#` or a bare line number.
#
# `proposal-the-later-topics.toml` is a second FILE of the one `proposal`
# namespace rather than a namespace of its own; the engine answers a query
# naming it with "does not resolve". So one engine query covers both files and
# the awk reader is pointed at both for the same reason.
#
# Controls, written before the run:
#   B1  the floor and the checked set are read out of the lint source, not
#       remembered. If the source does not say `>= 2` this arm is about a lint
#       that does not exist.
#   B2  the extractor must be able to report zero: a standing no row carries
#       must yield 0 rows.
#   B3  the extractor must see every row of the class. Its per-value counts must
#       match what the engine reports through `.count()`, which is a second
#       reader of the same field written by somebody else.
#   B4  the file counter must de-duplicate. `the_realisation_map_is_one_map_with
#       _two_regions` cites file `161` twice with two line anchors, so it must
#       count 2 citations and 1 file. Both numbers are asserted, so a reader
#       that finds nothing fails here rather than passing with a zero.
#   B5  the shipped floor's own population must reproduce the number the lint
#       carries as its measured ceiling, 29. A different number means this
#       reader and the lint disagree about what a citation is, and every count
#       above is then about this script rather than about the corpus.
set -uo pipefail
cd "$(dirname "$0")"
ROOT=../../../..
LINT=$ROOT/mock/lints/a_standing_is_reachable_from_what_it_cites.rs
REG=$ROOT/mock/registry

echo "### B1, the floor, the checked set and the ceiling, read out of the lint source"
grep -n 'MULTI_ARRIVAL: \|files.len() >=\|const CEILING' "$LINT"
echo

# id <TAB> standing <TAB> provenance, one line per proposal row, both files.
rows() {
  awk '
    /^\[\[proposal(-the-later-topics)?\]\]/ { flush(); inrow=1; next }
    /^\[\[/                                 { flush(); inrow=0; next }
    !inrow { next }
    inprov { pv = pv " " $0; if ($0 ~ /^\]/) inprov=0; next }
    /^id = /       { id=val(); next }
    /^standing = / { st=val(); next }
    /^provenance = \[$/ { inprov=1; pv=""; next }
    /^provenance = /    { pv=val(); next }
    END { flush() }
    function val(   s) { s=$0; sub(/^[a-z_]+ = /,"",s); gsub(/^"|"$/,"",s); return s }
    function flush() { if (id != "") printf "%s\t%s\t%s\n", id, st, pv; id=""; st=""; pv=""; inprov=0 }
  ' "$REG"/proposal.toml "$REG"/proposal-the-later-topics.toml
}

# Every citation in a provenance value, one per line, quotes and commas stripped.
cites_in() {
  printf '%s\n' "$1" | tr ',' '\n' \
    | sed -e 's/[][]//g' -e 's/"//g' -e 's/^ *//' -e 's/ *$//' \
    | grep -E '::' || true
}

# The lint's `file_named`, per citation, de-duplicated per row.
files_in() {
  cites_in "$1" | awk '
    {
      n = split($0, s, "::")
      if (n < 2) next
      last = s[n]
      if ((last ~ /^#/ || last ~ /^[0-9]+$/) && n >= 3) print s[n-1]
      else print last
    }' | sort -u | grep -v '^$' || true
}

echo "### B3, the extractor sees the same population the engine does"
for v in one_expert two_experts three_or_more cross_topic contested; do
  mine=$(rows | awk -F'\t' -v v="$v" '$2==v' | wc -l | tr -d ' ')
  tot=$( (cargo mock query "proposal.where(standing=$v).count()" 2>/dev/null || true) \
           | grep -oE '^[0-9]+$' | tail -1 )
  tot=${tot:-0}
  ok=FAIL; [ "$mine" = "$tot" ] && ok=PASS
  printf '  %-14s awk %-4s engine %-4s %s\n' "$v" "$mine" "$tot" "$ok"
done
echo

echo "### every three_or_more row, with the number of distinct files it cites"
under=0; n=0
while IFS=$'\t' read -r id st pv; do
  [ "$st" = "three_or_more" ] || continue
  c=$(files_in "$pv" | wc -l | tr -d ' ')
  n=$((n+1))
  flag=""
  if [ "$c" -lt 3 ]; then flag="  <-- claims 3+ arrivals, cites $c file(s)"; under=$((under+1)); fi
  printf '  %-3s %s%s\n' "$c" "$id" "$flag"
done < <(rows)
printf '  --- rows: %s, citing fewer than three files: %s\n\n' "$n" "$under"

echo "### the same question of cross_topic, the strongest count the panel produces"
cu=0
while IFS=$'\t' read -r id st pv; do
  [ "$st" = "cross_topic" ] || continue
  c=$(files_in "$pv" | wc -l | tr -d ' ')
  flag=""
  if [ "$c" -lt 3 ]; then flag="  <-- separate topics claimed, $c file(s) cited"; cu=$((cu+1)); fi
  printf '  %-3s %s%s\n' "$c" "$id" "$flag"
done < <(rows)
printf '  --- citing fewer than three files: %s\n\n' "$cu"

echo "### B5, the shipped >= 2 predicate's population against the lint's measured ceiling"
lintfind=0
while IFS=$'\t' read -r id st pv; do
  case "$st" in two_experts|three_or_more|cross_topic) ;; *) continue ;; esac
  c=$(files_in "$pv" | wc -l | tr -d ' ')
  [ "$c" -lt 2 ] && lintfind=$((lintfind+1))
done < <(rows)
ceil=$(grep -oE 'CEILING: usize = [0-9]+' "$LINT" | grep -oE '[0-9]+')
echo "  rows below the shipped floor of 2: $lintfind, the lint's measured ceiling: $ceil"
if [ "$lintfind" = "$ceil" ]; then echo "  PASS, this reader and the lint agree on the population"
else echo "  FAIL, the two disagree, so every count above is about this script"; fi
echo

echo "### B2, the extractor must be able to report zero"
z=$(rows | awk -F'\t' '$2=="zzz_no_such_standing"' | wc -l | tr -d ' ')
[ "$z" = "0" ] && echo "  PASS, 0 rows for a standing nothing carries" || echo "  FAIL, reports $z"

echo "### B4, two citations naming one file de-duplicate to one"
pv=$(rows | awk -F'\t' '$1=="the_realisation_map_is_one_map_with_two_regions"{print $3}')
raw=$(cites_in "$pv" | wc -l | tr -d ' ')
uq=$(files_in "$pv" | wc -l | tr -d ' ')
echo "  raw citations: $raw, distinct files: $uq"
if [ "$raw" = "2" ] && [ "$uq" = "1" ]; then echo "  PASS, 2 citations of one file counted once"
else echo "  FAIL, expected 2 citations and 1 file"; fi
