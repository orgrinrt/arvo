#!/usr/bin/env bash
# Seat 246. The anchor-set difference between 244's three sources and 244,
# rebuilt so it can see the anchor class 244's own instrument was blind to.
#
# WHAT WAS WRONG WITH v1 (244_probes/anchor_diff.sh), three defects:
#
#   1. Its pattern
#        [0-9]+_[a-z_]+\.md|[a-z_]+\.(rs|toml):[0-9]+|[A-Za-z0-9_]+\.(rs|toml)
#      cannot match a registry row slug at all, a slug being a namespace and an
#      identifier joined by a double colon. A slug is the
#      only way to address a canon row, and this panel's canon IS the registry,
#      so v1 was blind to the anchor class that matters most here. 245 found
#      this and built a slug-only diff beside it; this instrument merges the
#      classes into one accounting instead of leaving two.
#   2. Its filename alternative is unanchored, so `243_seat242_the_resolution_
#      has_no_second_arm.md` yields the phantom `242_the_resolution_has_no_
#      second_arm.md`, and `law-the-later-topics.toml` yields the phantom
#      `topics.toml`. 244 disclosed both and did not fix either.
#   3. Its filename alternative has no hyphen in the character class, so
#      `242_what-admits-a-number-system.md` is invisible in both directions.
#      Disclosed in v1's own trailing note and not in 244's section 7 prose.
#
# WHAT v2 DOES DIFFERENTLY:
#   - the slug namespace list is DERIVED from the registry's own table headers
#     rather than typed from memory, so it cannot go stale when a table is
#     added. 245's hand-written list held 8 of the 10 that exist.
#   - every pattern is anchored at a non-token character, which kills both
#     phantoms.
#   - hyphens are legal in a filename stem.
#
# THE CASES THAT MUST FAIL, planted and shown before any number is reported:
#   C1 both sides of both classes extract something.
#   C2 a synthetic anchor planted in the SOURCES ONLY must be reported LOST.
#   C3 a synthetic anchor planted in 244 ONLY must be reported NEW.
#   C4 a synthetic anchor planted in BOTH must be reported CARRIED and must
#      appear in neither difference. This is the one a "both sides nonempty"
#      control cannot catch: a classifier that calls everything lost passes
#      C1 and C2 and fails here.
#   C5 the two v1 phantoms must be absent from v2's extraction while v1 still
#      produces them, or the claim that they were fixed is not measured.
# The census is refused unless all five hold.

set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory

SRC1=241_kiselyov_admission_is_a_resolution_not_a_verdict.md
SRC2=242_what-admits-a-number-system.md
SRC3=243_seat242_the_resolution_has_no_second_arm.md
MINE=244_orchard_consolidation_admission_and_the_number_system.md
REG=../../registry

# --- the namespace list, derived rather than remembered -----------------------
NS=$(grep -ho '^\[\[[a-z_]*\]\]' "$REG"/*.toml | tr -d '[]' | sort -u | paste -sd'|' -)
echo "registry namespaces, derived from the table headers of $REG/*.toml:"
echo "  $NS"
echo

SLUG="($NS)::[a-z0-9_]+"
# A filename or a file:line reference, anchored so a match cannot start inside
# another token. The leading boundary character is stripped after extraction.
BOUND='(^|[^A-Za-z0-9_.-])'
FILE="${BOUND}[0-9]+_[A-Za-z0-9_-]+\.md"
SREF="${BOUND}[A-Za-z0-9_.-]+\.(rs|toml)(:[0-9]+)?"

V1='[0-9]+_[a-z_]+\.md|[a-z_]+\.(rs|toml):[0-9]+|[A-Za-z0-9_]+\.(rs|toml)'

strip_bound() { sed 's/^[^A-Za-z0-9_]//'; }

slugs()  { grep -oE "$SLUG" | sort -u; }
files()  { grep -oE "$FILE|$SREF" | strip_bound | sort -u; }

# `mine_body` mirrors v1: section 7 is the accounting itself, so its own listing
# must not be able to make a dropped anchor present.
mine_body() { awk '/^## 7\. Accounting/{skip=1} /^## 8\. Coverage/{skip=0} !skip' "$MINE"; }
src_body()  { cat "$SRC1" "$SRC2" "$SRC3"; }

# ---------------------------------------------------------------------------
# CONTROLS
# ---------------------------------------------------------------------------
fail() { echo "CONTROL FAILED: $1"; exit 2; }

S_SLUG=$(src_body  | slugs);  M_SLUG=$(mine_body | slugs)
S_FILE=$(src_body  | files);  M_FILE=$(mine_body | files)

for pair in "sources/slug:$S_SLUG" "244/slug:$M_SLUG" "sources/file:$S_FILE" "244/file:$M_FILE"; do
  name=${pair%%:*}; val=${pair#*:}
  [ -n "$val" ] || fail "C1, $name extracted nothing, so any difference would be about the pattern."
done
echo "C1 passes: all four extractions are nonempty."

PLANT_L="ruling::c2_planted_in_the_sources_only"
PLANT_N="ruling::c3_planted_in_244_only"
PLANT_B="ruling::c4_planted_in_both"

s_ctl=$( { src_body;  printf '%s %s\n' "$PLANT_L" "$PLANT_B"; } | slugs )
m_ctl=$( { mine_body; printf '%s %s\n' "$PLANT_N" "$PLANT_B"; } | slugs )
lost_ctl=$(comm -23 <(printf '%s\n' "$s_ctl") <(printf '%s\n' "$m_ctl"))
new_ctl=$( comm -13 <(printf '%s\n' "$s_ctl") <(printf '%s\n' "$m_ctl"))
carr_ctl=$(comm -12 <(printf '%s\n' "$s_ctl") <(printf '%s\n' "$m_ctl"))

printf '%s\n' "$lost_ctl" | grep -qx "$PLANT_L" || fail "C2, a slug present only in the sources was not reported lost."
echo "C2 passes: the sources-only plant is reported lost."
printf '%s\n' "$new_ctl"  | grep -qx "$PLANT_N" || fail "C3, a slug present only in 244 was not reported new."
echo "C3 passes: the 244-only plant is reported new."
printf '%s\n' "$carr_ctl" | grep -qx "$PLANT_B" || fail "C4, a slug present on both sides was not reported carried."
printf '%s\n' "$lost_ctl" | grep -qx "$PLANT_B" && fail "C4, a slug present on both sides was also reported lost."
printf '%s\n' "$new_ctl"  | grep -qx "$PLANT_B" && fail "C4, a slug present on both sides was also reported new."
echo "C4 passes: the both-sides plant is carried and appears in neither difference."

v1_all=$( { src_body; mine_body; } | grep -oE "$V1" | sort -u )
v2_all=$(printf '%s\n%s\n%s\n%s\n' "$S_FILE" "$M_FILE" "$S_SLUG" "$M_SLUG" | sort -u)
for phantom in topics.toml 242_the_resolution_has_no_second_arm.md; do
  printf '%s\n' "$v1_all" | grep -qx "$phantom" || fail "C5, v1 does not produce the phantom '$phantom', so there is nothing to have fixed."
  printf '%s\n' "$v2_all" | grep -qx "$phantom" && fail "C5, v2 still produces the phantom '$phantom'."
done
echo "C5 passes: v1 produces both phantoms and v2 produces neither."
echo

# ---------------------------------------------------------------------------
# THE CENSUS
# ---------------------------------------------------------------------------
report() {
  local label=$1 s=$2 m=$3
  printf '%s\n' "--- $label ---"
  printf '  in the three sources : %d\n' "$(printf '%s\n' "$s" | grep -c .)"
  printf '  in 244               : %d (section 7 excluded)\n' "$(printf '%s\n' "$m" | grep -c .)"
  echo "  LOST (cited by a source, not by 244):"
  comm -23 <(printf '%s\n' "$s") <(printf '%s\n' "$m") | sed 's/^/    /'
  printf '    count = %d\n' "$(comm -23 <(printf '%s\n' "$s") <(printf '%s\n' "$m") | grep -c .)"
  echo "  new (cited by 244, not by a source):"
  comm -13 <(printf '%s\n' "$s") <(printf '%s\n' "$m") | sed 's/^/    /'
  printf '    count = %d\n' "$(comm -13 <(printf '%s\n' "$s") <(printf '%s\n' "$m") | grep -c .)"
  echo "  carried:"
  comm -12 <(printf '%s\n' "$s") <(printf '%s\n' "$m") | sed 's/^/    /'
  printf '    count = %d\n' "$(comm -12 <(printf '%s\n' "$s") <(printf '%s\n' "$m") | grep -c .)"
  echo
}

report "class: registry row slugs (invisible to v1 entirely)" "$S_SLUG" "$M_SLUG"
report "class: files and file:line (v1's class, phantoms removed, hyphens admitted)" "$S_FILE" "$M_FILE"

S_ALL=$(printf '%s\n%s\n' "$S_SLUG" "$S_FILE" | grep . | sort -u)
M_ALL=$(printf '%s\n%s\n' "$M_SLUG" "$M_FILE" | grep . | sort -u)
report "COMBINED, which is the corrected accounting" "$S_ALL" "$M_ALL"

echo "--- v1's own numbers, reproduced here for the comparison ---"
v1s=$(src_body  | grep -oE "$V1" | sort -u)
v1m=$(mine_body | grep -oE "$V1" | sort -u)
printf '  v1 sources : %d      v1 244 : %d\n' "$(printf '%s\n' "$v1s" | grep -c .)" "$(printf '%s\n' "$v1m" | grep -c .)"
printf '  v1 lost    : %d      v1 new : %d\n' \
  "$(comm -23 <(printf '%s\n' "$v1s") <(printf '%s\n' "$v1m") | grep -c .)" \
  "$(comm -13 <(printf '%s\n' "$v1s") <(printf '%s\n' "$v1m") | grep -c .)"
