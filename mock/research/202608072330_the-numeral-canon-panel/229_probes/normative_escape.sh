#!/usr/bin/env bash
# How many rows escape the region obligation by being filed `normative` while
# their own `because` reads as a derivation from instruments?
#
# Two lints bound the predicate discipline. `a-region-agrees-with-the-sentence-
# kind` excuses `normative` and `definition` from carrying a region. `an-
# imposition-rests-on-no-instrument` catches a `normative` row that also carries
# an `evidence` field. Neither reaches a `normative` row that names its
# instruments in `because` prose and carries no `evidence` key, and that is
# where a claim with no expressible region lands: `proposal::the_topics_form_a_
# stack_a_frame_and_the_canons_own_machinery` says so in its own note, that it
# was "filed `normative` after being written `argument`" because no dimension
# could carry its region.
#
# Controls, outcomes written before the run:
#   N1  the two lints' own excused set must be exactly {normative, definition},
#       read out of the lint source rather than remembered.
#   N2  a row known to be in the class must be found by name:
#       `the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`.
#   N3  the arm must be able to report zero: run with the sentence_kind set to a
#       value nothing carries and the count must be 0.
#   N4  a row carrying `evidence` must NOT be counted, since the imposition lint
#       already reaches it.
set -euo pipefail
cd "$(dirname "$0")"
REG=../../../registry
LINTS=../../../lints

echo "### N1, the excused kinds, read out of the lint"
grep -oE '"(normative|definition|argument|measured|theorem|enumeration)"' "$LINTS/a_region_agrees_with_the_sentence_kind.rs" | sort -u | tr '\n' ' '; echo
echo

count_kind() {
  local kind="$1"
  awk -v kind="$kind" '
    /^\[\[/       { flush(); next }
    /^id = /      { id=$0; sub(/^id = "/,"",id); sub(/"$/,"",id); next }
    /^sentence_kind = / { sk=$0; next }
    /^evidence = /{ ev=1; next }
    /^because = / { inbec=1; bec=$0; next }
    inbec         { bec = bec " " $0; if ($0 ~ /^[a-z_]+ = / || $0 ~ /^"""$/) inbec=0 }
    END { flush() }
    function flush() {
      if (id != "" && sk ~ ("\"" kind "\"") && ev==0 && bec ~ /probe|measured|sweep|instrument|control|enumerat|F[0-9]|`[0-9]+`/)
        print "  " id
      id=""; sk=""; ev=0; bec=""; inbec=0
    }
  ' "$REG"/*.toml
}

echo "### rows filed as the excused kind, no \`evidence\` key, \`because\` naming instruments"
count_kind normative | sort -u > normative_escape_rows.txt
cat normative_escape_rows.txt
printf '  --- count: %s\n\n' "$(wc -l < normative_escape_rows.txt | tr -d ' ')"

echo "### N2, the row that says in its own note it is in this class"
grep -q 'the_topics_form_a_stack_a_frame_and_the_canons_own_machinery' normative_escape_rows.txt \
  && echo "  PASS, found" || echo "  FAIL, the arm cannot see the one row known to be here"

echo "### N3, the arm must be able to report zero"
n=$(count_kind zzz_no_such_kind | wc -l | tr -d ' ')
[ "$n" = "0" ] && echo "  PASS, 0 for a kind nothing carries" || echo "  FAIL, reports $n"

echo "### N4, a row carrying \`evidence\` must not be counted"
ev_rows=$(grep -c '^evidence = ' "$REG"/*.toml | awk -F: '{s+=$2} END{print s+0}')
echo "  rows carrying an \`evidence\` key anywhere: $ev_rows"
bad=0
while read -r r; do
  [ -z "$r" ] && continue
  # the row's own block must not contain an evidence key
  awk -v want="$(echo "$r" | tr -d ' ')" '
    /^\[\[/ {inrow=0}
    $0 == "id = \"" want "\"" {inrow=1}
    inrow && /^evidence = / {print "  LEAK " want}
  ' "$REG"/*.toml
done < normative_escape_rows.txt | tee n4.txt
[ ! -s n4.txt ] && echo "  PASS, none of the reported rows carries evidence" || echo "  FAIL"
