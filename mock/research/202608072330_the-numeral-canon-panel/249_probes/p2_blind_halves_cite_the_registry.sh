#!/usr/bin/env bash
# p2. Do the panel's blind halves read the registry?
#
# A "blind" member file commits its derivation before opening the withheld
# material, then appends a reconciliation. If the blind half cites registry
# rows, the seat derived cold of its PEERS and warm of the CANON, which is a
# different claim from the one the file's heading makes.
#
# The instrument: for every member file carrying a reconciliation heading,
# split at that heading and count registry-slug citations on each side.
#
# The case that must fail, run first: a planted file whose citations sit
# entirely after the heading must report 0 before, and one whose citations sit
# entirely before must report 0 after. An instrument that cannot report zero on
# one side is reporting the total twice.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"

PANEL=mock/research/202608072330_the-numeral-canon-panel
SLUG='`(proposal|question|ruling|law|probe|retirement|obligation|dimension)::'
RECON='^#{1,4} .*[Rr]econcil'
T=$(mktemp -d)

# split <file> -> "<before> <after> <heading-line>"
split_at_recon() {
  local f=$1 h
  h=$(grep -nE "$RECON" "$f" | head -1 | cut -d: -f1)
  if [ -z "$h" ]; then echo "- - -"; return; fi
  local b a
  b=$(awk -v h="$h" 'NR<h' "$f" | grep -cE "$SLUG")
  a=$(awk -v h="$h" 'NR>=h' "$f" | grep -cE "$SLUG")
  echo "$b $a $h"
}

echo "=============================================================="
echo "CONTROLS"
echo "=============================================================="
cat > "$T/after_only.md" <<'MD'
# 000. a planted file
Some prose with no citation in it at all.
## 9. Reconciliation
This half cites `proposal::a_thing` and `ruling::another_thing`.
MD
cat > "$T/before_only.md" <<'MD'
# 000. a planted file
This half cites `proposal::a_thing`.
And here is `question::another_thing`.
## 9. Reconciliation
Nothing cited here.
MD
cat > "$T/no_recon.md" <<'MD'
# 000. a planted file with no such heading
It cites `proposal::a_thing`.
MD
echo "-- C1 citations only AFTER the heading -> before must be 0 --"
printf '   after_only  : '; split_at_recon "$T/after_only.md"
echo "-- C2 citations only BEFORE the heading -> after must be 0 --"
printf '   before_only : '; split_at_recon "$T/before_only.md"
echo "-- C3 no reconciliation heading -> excluded, reported as '-' --"
printf '   no_recon    : '; split_at_recon "$T/no_recon.md"
rm -rf "$T"

echo
echo "=============================================================="
echo "Member files >= 189 carrying a reconciliation heading"
echo "244 through 249 excluded: seat 249 had not read them when this ran."
echo "=============================================================="
printf '%-64s %6s %6s\n' file blind recon
tot=0; hot=0; blindcites=0
for f in "$PANEL"/[0-9]*_*.md; do
  b=$(basename "$f"); n=${b%%_*}; n=${n//[!0-9]/}
  [ -z "$n" ] && continue
  [ "$n" -lt 189 ] && continue
  case "$n" in 244|245|246|247|248|249) continue;; esac
  read -r before after _ <<<"$(split_at_recon "$f")"
  [ "$before" = "-" ] && continue
  tot=$((tot+1))
  if [ "$before" -gt 0 ]; then hot=$((hot+1)); blindcites=$((blindcites+before)); fi
  printf '%-64s %6s %6s\n' "${b%.md}" "$before" "$after"
done
echo
echo "files with a reconciliation heading:            $tot"
echo "of those, citing the registry in the BLIND half: $hot"
echo "registry citations inside blind halves:          $blindcites"

echo
echo "-- how many post-189 files ran a blind phase at all --"
have=0; none=0
for f in "$PANEL"/[0-9]*_*.md; do
  b=$(basename "$f"); n=${b%%_*}; n=${n//[!0-9]/}
  [ -z "$n" ] && continue; [ "$n" -lt 189 ] && continue
  case "$n" in 244|245|246|247|248|249) continue;; esac
  if grep -qE "$RECON" "$f"; then have=$((have+1)); else none=$((none+1)); fi
done
echo "with a reconciliation heading:    $have"
echo "without one:                      $none"

echo
echo "-- C4 the split regex must not fire on any file's TITLE line --"
t=0
for f in "$PANEL"/[0-9]*_*.md; do head -1 "$f" | grep -qE "$RECON" && t=$((t+1)); done
echo "files whose line 1 matches: $t   (non-zero would mean every split above is at line 1)"
