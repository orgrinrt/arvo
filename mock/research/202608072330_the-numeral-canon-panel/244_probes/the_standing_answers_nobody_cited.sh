#!/usr/bin/env bash
# Seat 244. Does the registry already carry an answer to each of the sitting's
# six questions, and did any of the three files place itself against one?
#
# The case that must fail: if the citation grep cannot find an id the files
# genuinely do cite, a zero from it means nothing. The positive control is
# `the_concept_is_closed_and_the_inventory_is_open`, which 241 and 242 both
# quote by id. It must come back nonzero or this probe reports nothing.
set -u
cd "$(dirname "$0")/../../../.." || exit 1   # repo root

REG=mock/registry
PANEL=mock/research/202608072330_the-numeral-canon-panel
FILES=(
  "$PANEL/241_kiselyov_admission_is_a_resolution_not_a_verdict.md"
  "$PANEL/242_what-admits-a-number-system.md"
  "$PANEL/243_seat242_the_resolution_has_no_second_arm.md"
)

SIX=(
  is_the_number_system_inventory_open
  is_admission_a_predicate_or_a_location
  is_number_system_broad_enough_for_non_magnitude
  are_set_valued_carriers_admitted
  one_word_or_two_for_is_a_number_system
  what_the_admission_contract_asks_a_candidate_to_expose
)

# Every row in any namespace whose `answers` list names a question id.
answers_index() {
  for f in proposal proposal-the-later-topics ruling law law-the-later-topics; do
    [ -f "$REG/$f.toml" ] || continue
    awk -v NS="$f" '
      /^\[\[/{ flush(); id="";ans="";st="";inb=0 }
      /^id *= /{ if(id==""){s=$0;sub(/^id *= *"/,"",s);sub(/"$/,"",s);id=s} }
      /^standing *= /{ s=$0;sub(/^standing *= *"/,"",s);sub(/"$/,"",s);st=s }
      /^rung *= /{ s=$0;sub(/^rung *= *"/,"",s);sub(/"$/,"",s);st="rung:" s }
      /^answers *= /{ ans=$0; if($0 ~ /\[[[:space:]]*$/) inb=1 }
      inb==1 && !/^answers/ { ans=ans $0; if(/\]/) inb=0 }
      END{ flush() }
      function flush(  n,a,i){
        if(id!=""&&ans!=""){ n=split(ans,a,/"/);
          for(i=1;i<=n;i++) if(a[i]~/^[a-z_]+$/)
            printf "%s\t%s::%s\t%s\n", a[i], NS, id, (st==""?"-":st) }
      }' "$REG/$f.toml"
  done
}

echo "======== 1. Which registry rows claim to answer the sitting's six questions"
IDX=$(answers_index)
declare -a ANSWERERS=()
for q in "${SIX[@]}"; do
  hits=$(printf '%s\n' "$IDX" | awk -F'\t' -v q="$q" '$1==q{print $2"  (standing="$3")"}')
  if [ -z "$hits" ]; then
    printf '  %-56s  NO STANDING ANSWER ROW\n' "$q"
  else
    printf '  %-56s\n' "$q"
    printf '%s\n' "$hits" | sed 's/^/      /'
    while IFS= read -r h; do
      ANSWERERS+=("$(printf '%s' "$h" | awk '{print $1}' | sed 's/^.*:://')")
    done <<<"$hits"
  fi
done

echo
echo "======== 2. CONTROL: an id the files demonstrably do cite"
CTRL=the_concept_is_closed_and_the_inventory_is_open
ctrl_total=0
for f in "${FILES[@]}"; do
  n=$(grep -c "$CTRL" "$f")
  ctrl_total=$((ctrl_total + n))
  printf '  %-58s %s : %d\n' "$CTRL" "$(basename "$f" | cut -c1-3)" "$n"
done
if [ "$ctrl_total" -eq 0 ]; then
  echo "  CONTROL FAILED: the citation grep finds nothing it should find."
  echo "  Every zero below would be a fact about this script. Stopping."
  exit 2
fi
echo "  control passes: $ctrl_total citations found, so a zero below is a fact about the files."

echo
echo "======== 3. Are the standing answer rows cited by any of the three files"
uniq_ans=$(printf '%s\n' "${ANSWERERS[@]}" | sort -u)
total=0
while IFS= read -r a; do
  [ -n "$a" ] || continue
  n=0
  for f in "${FILES[@]}"; do n=$((n + $(grep -c "$a" "$f"))); done
  total=$((total + n))
  printf '  %-92s cited %d times\n' "$a" "$n"
done <<<"$uniq_ans"

echo
echo "FINDING: standing answer rows cited across 241, 242 and 243 = $total"
echo "The control fired, so the count above is a fact about the three files."
