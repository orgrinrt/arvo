#!/usr/bin/env bash
# Seat 247. The L4 term census, taken with a tokeniser rather than with grep.
#
# 244 L4: archive/OLD_CANON_CANDIDATE.md "carries 61 uses of `sealed`, 8 of
# `value-unique`, 6 of `NonZero` and 6 of `AtLeastTwo`". 245 measured substring
# 61/8/13/6 and word-boundary NonZero 5. 246 measured four grep conventions and
# found no single one reproducing all four. This is a third instrument, built
# differently on purpose: it tokenises the file in awk (a token is a maximal run
# of [A-Za-z0-9_-]) and counts under four token relations plus the workspace's
# own word-frequency tool, whose tokeniser is a fifth. If 244 used any ordinary
# instrument, one of these should reproduce all four numbers.
#
# Conventions:
#   exact    token == term
#   suffix   token ends with term          (IsNonZero, unsealed)
#   prefix   token starts with term        (NonZeroCarrier, sealed-bound-...)
#   within   token contains term           (all of the above)
#   wf       .shared/scripts/word-frequency.sh, lowercased, hyphen-split
#
# THE CASES THAT MUST FAIL, run before the census is reported:
#   C1  A planted fixture with known counts and no embedding must reproduce its
#       claimed set under exact, suffix, prefix and within alike.
#   C1b The same fixture against a wrong claim set must reproduce under none.
#   C2  A term known absent returns zero under every convention.
#   C3  At least one term must differ across conventions on the real file, or
#       the four relations are one relation with four names.
set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory
fail() { echo "CONTROL FAILED: $1"; exit 2; }
TARGET=archive/OLD_CANON_CANDIDATE.md
TERMS=(sealed value-unique NonZero AtLeastTwo)
CLAIMED=(61 8 6 6)

count() {  # count <convention> <term> <file>
  case $1 in
    exact|suffix|prefix|within)
      awk -v conv="$1" -v term="$2" '
        { n=split($0, toks, /[^A-Za-z0-9_-]+/)
          for (i=1;i<=n;i++) { t=toks[i]; if (t=="") continue
            L=length(term)
            if (conv=="exact"  && t==term) c++
            if (conv=="suffix" && substr(t,length(t)-L+1)==term) c++
            if (conv=="prefix" && substr(t,1,L)==term) c++
            if (conv=="within" && index(t,term)>0) c++ } }
        END{print c+0}' "$3" ;;
  esac
}

reproducing() {  # reproducing <file> <c1> <c2> <c3> <c4>
  local file=$1; shift; local -a claims=("$@"); local out=""
  for conv in exact suffix prefix within; do
    local ok=1 i=0
    for t in "${TERMS[@]}"; do
      [ "$(count $conv "$t" "$file")" = "${claims[$i]}" ] || ok=0; i=$((i+1))
    done
    [ $ok -eq 1 ] && out="$out $conv"
  done
  echo "${out# }"
}

# --- C1 / C1b ---------------------------------------------------------------------------
FIX=$(mktemp -t s247fix); trap 'rm -f "$FIX"' EXIT
{ for i in 1 2 3; do echo "a sealed thing"; done
  for i in 1 2; do echo "a value-unique thing"; done
  echo "a NonZero thing"
  for i in 1 2 3 4; do echo "an AtLeastTwo thing"; done; } > "$FIX"
r=$(reproducing "$FIX" 3 2 1 4)
[ "$r" = "exact suffix prefix within" ] || fail "C1, on the planted fixture the reproducing set is '$r', expected all four"
echo "C1 passes: the planted fixture (3/2/1/4) reproduces under all four token relations."
[ -z "$(reproducing "$FIX" 3 2 1 5)" ] || fail "C1b, a wrong claim set reproduced on the fixture"
echo "C1b passes: a wrong claim set reproduces under none."

# --- C2 ---------------------------------------------------------------------------------
for conv in exact suffix prefix within; do
  [ "$(count $conv seat247_absent_term "$TARGET")" -eq 0 ] || fail "C2, an absent term counted nonzero under $conv"
done
echo "C2 passes: an absent term is zero under all four."

# --- C3 ---------------------------------------------------------------------------------
[ "$(count exact NonZero "$TARGET")" != "$(count within NonZero "$TARGET")" ] || fail "C3, exact and within agree on NonZero; the relations are not distinct here"
echo "C3 passes: exact and within disagree on NonZero, so the relations are distinct on this file."
echo

# --- the workspace tool, as a fifth tokeniser --------------------------------------------
WF=../../../../../.shared/scripts/word-frequency.sh
wf_count() {  # lowercased token from the workspace tool
  bash "$WF" --paths "$TARGET" --top 200000 --min-len 4 2>/dev/null | awk -v w="$1" -F'\t' '$2==w{print $1; f=1} END{if(!f)print 0}'
}

# --- the census -------------------------------------------------------------------------
echo "target : $TARGET  ($(wc -l < "$TARGET" | tr -d ' ') lines)"
echo "244 L4 claims: sealed=61 value-unique=8 NonZero=6 AtLeastTwo=6"
echo
printf '%-13s %6s %6s %6s %6s %6s %9s\n' term exact suffix prefix within wf '244 says'
i=0
for t in "${TERMS[@]}"; do
  lt=$(printf '%s' "$t" | tr 'A-Z' 'a-z')
  printf '%-13s %6s %6s %6s %6s %6s %9s\n' "$t" \
    "$(count exact "$t" "$TARGET")" "$(count suffix "$t" "$TARGET")" \
    "$(count prefix "$t" "$TARGET")" "$(count within "$t" "$TARGET")" \
    "$([ -x "$WF" ] || [ -f "$WF" ] && wf_count "$lt" || echo n/a)" "${CLAIMED[$i]}"
  i=$((i+1))
done
echo
echo "the tokens each term is embedded in:"
for t in "${TERMS[@]}"; do
  echo "  $t:"
  awk -v term="$t" '{ n=split($0, toks, /[^A-Za-z0-9_-]+/); for(i=1;i<=n;i++) if (index(toks[i],term)>0) print toks[i] }' "$TARGET" | sort | uniq -c | sed 's/^/    /'
done
echo
REP=$(reproducing "$TARGET" "${CLAIMED[@]}")
if [ -n "$REP" ]; then echo "VERDICT: 244's four numbers reproduce together under:$REP"
else
  echo "VERDICT: no single token relation reproduces all four of 244's numbers, and the workspace"
  echo "         word-frequency tool does not either (it lowercases and splits on hyphens)."
  echo "         Per term, the relations that give 244's number:"
  i=0
  for t in "${TERMS[@]}"; do
    hits=""; for conv in exact suffix prefix within; do [ "$(count $conv "$t" "$TARGET")" = "${CLAIMED[$i]}" ] && hits="$hits $conv"; done
    printf '           %-13s %s\n' "$t" "${hits:- none}"; i=$((i+1))
  done
fi
