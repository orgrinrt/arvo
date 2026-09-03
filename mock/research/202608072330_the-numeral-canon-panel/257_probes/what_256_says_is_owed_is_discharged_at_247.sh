#!/usr/bin/env bash
# Seat 257. 256 section 4 lists two things as still owed by this sitting.
# Both name a reader that a Kiselyov seat cannot be. This asks whether 247,
# which 256 lists in its own table and did not open, already supplies them.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

echo "== What 256 section 4 says is still owed =="
sed -n '/^Still owed/,/^## 5/p' \
    256_giesen_the_sitting_was_consolidated_at_244_and_the_brief_did_not_know.md | head -8

echo
echo "== OWED 1: a second reader on 246 section 2.2 =="
echo "-- 247's own heading for it, and whether the reading was formed before accepting:"
grep -n "My own reading of 246 section 2.2" \
    247_leroy_the_cold_seats_answered_rows_73_wrote.md || echo "  (none)"
echo "-- 247's verdict lines on 2.2:"
grep -n "the containment .246. rests on holds\|Where I part from 246\|246's mechanism does not do the work" \
    247_leroy_the_cold_seats_answered_rows_73_wrote.md || echo "  (none)"

echo
echo "== OWED 2: a non-Kiselyov reading of 244's C3 demotion of the fixed-arity claim =="
grep -n "C3 demotion" 247_leroy_the_cold_seats_answered_rows_73_wrote.md || echo "  (none)"

echo
echo "== PERSONA CONTROL: which of the four files is Kiselyov-named =="
for f in 241_* 244_* 246_* 247_*; do
    case "$f" in
        *.md) printf '  %-62s ' "$f"
              case "$f" in *kiselyov*) echo "kiselyov";; *) echo "not kiselyov";; esac ;;
    esac
done

echo
echo "== NEGATIVE CONTROL: a phrase known absent from 247 must return none =="
grep -n "the second reader is still owed" 247_leroy_the_cold_seats_answered_rows_73_wrote.md \
    || echo "  (none, as required)"

echo
echo "== Registry provenance from seats 241 to 256, with 240 as the control =="
cd ../../.. || exit 1
printf '  rows citing seats 241-256 : '
grep -hoE "2(4[1-9]|5[0-6])_[a-z0-9_]+" mock/registry/*.toml | sort -u | wc -l | tr -d ' '
printf '  CONTROL, rows citing 240  : '
grep -hoE "240_[a-z0-9_]+" mock/registry/*.toml | sort -u | wc -l | tr -d ' '
echo "  (the control must be non-zero, or the grep proves nothing)"
