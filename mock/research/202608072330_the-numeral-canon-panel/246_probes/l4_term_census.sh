#!/usr/bin/env bash
# Seat 246. The term census 244's L4 states and does not commit.
#
# 244 L4 says of archive/OLD_CANON_CANDIDATE.md: "The same file carries 61 uses
# of `sealed`, 8 of `value-unique`, 6 of `NonZero` and 6 of `AtLeastTwo`."
#
# 245 measured plain-substring 61/8/13/6 and a word-boundary NonZero of 5, and
# concluded: "Three of four reproduce exactly under plain substring counting,
# which is strong evidence that is the method used", and "Neither reading gives
# 6."
#
# BOTH HALVES OF THAT ARE WRONG, and this instrument is what shows it. A count
# is not one number, it is a number and a convention, and there are four ordinary
# conventions, not two. This runs all four against all four terms and asks the
# only question worth asking: is there ONE convention under which all four of
# 244's numbers reproduce.
#
#   substr : occurrences of the substring, however embedded.   grep -oE 'T'
#   trailB : occurrences with a trailing token boundary only.  grep -oE 'T\b'
#   bothB  : occurrences as a free-standing token.             grep -oE '\bT\b'
#   lines  : lines carrying at least one occurrence.           grep -c 'T'
#
# THE CASE THAT MUST FAIL, planted and run before the census is reported:
# a fixture file is built in which all four terms appear a known number of times
# with no embedding and no repeated line, so every convention agrees and a
# single convention DOES reproduce a claimed set. If the instrument cannot
# report "reproduced" on that fixture, then its verdict of "no convention
# reproduces" on the real file is a fact about the instrument and not about the
# file. Two further controls: a term known absent must return zero under every
# convention, and at least one term must differ across conventions or the
# instrument is not distinguishing them at all.

set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory

TARGET=archive/OLD_CANON_CANDIDATE.md
TERMS=(sealed value-unique NonZero AtLeastTwo)
CLAIMED=(61 8 6 6)                   # 244 L4, in the order it states them

count() {  # count <convention> <term> <file>
  case $1 in
    substr) grep -oE -- "$2"        "$3" | wc -l | tr -d ' ' ;;
    trailB) grep -oE -- "$2\\b"     "$3" | wc -l | tr -d ' ' ;;
    bothB)  grep -oE -- "\\b$2\\b"  "$3" | wc -l | tr -d ' ' ;;
    lines)  grep -c  -- "$2"        "$3" | tr -d ' ' ;;
  esac
}

# which conventions reproduce every claimed number for a given file
reproducing() {
  local file=$1; shift
  local -a claims=("$@")
  local out=""
  for conv in substr trailB bothB lines; do
    local ok=1 i=0
    for t in "${TERMS[@]}"; do
      [ "$(count "$conv" "$t" "$file")" = "${claims[$i]}" ] || ok=0
      i=$((i+1))
    done
    [ $ok -eq 1 ] && out="$out $conv"
  done
  echo "${out# }"
}

fail() { echo "CONTROL FAILED: $1"; exit 2; }

# --- C1, the planted fixture where a convention must reproduce ---------------
FIX=$(mktemp -t s246fix)
trap 'rm -f "$FIX"' EXIT
{
  for i in 1 2 3; do echo "a sealed thing here"; done
  for i in 1 2;   do echo "a value-unique thing here"; done
  echo "a NonZero thing here"
  for i in 1 2 3 4; do echo "an AtLeastTwo thing here"; done
} > "$FIX"
FIXREP=$(reproducing "$FIX" 3 2 1 4)
[ -n "$FIXREP" ] || fail "C1, on a fixture built to make every convention agree, no convention reproduced. The verdict below would be about the instrument."
echo "C1 passes: on the planted fixture (3/2/1/4, no embedding, one per line) the reproducing conventions are:$FIXREP"

# and the same fixture against a set that is WRONG must reproduce under none
FIXBAD=$(reproducing "$FIX" 3 2 1 5)
[ -z "$FIXBAD" ] || fail "C1b, a deliberately wrong claim set reproduced under '$FIXBAD'. The instrument agrees with everything."
echo "C1b passes: a deliberately wrong claim set on the same fixture reproduces under no convention."

# --- C2, a term known absent returns zero everywhere -------------------------
for conv in substr trailB bothB lines; do
  [ "$(count "$conv" "AtLeastThreeNonZeroSealed" "$TARGET")" = "0" ] ||
    fail "C2, an absent term returned a nonzero count under $conv."
done
echo "C2 passes: an absent term returns zero under all four conventions."

# --- C3, the conventions must be able to disagree ----------------------------
if [ "$(count substr NonZero "$TARGET")" = "$(count bothB NonZero "$TARGET")" ]; then
  fail "C3, substr and bothB agree on NonZero, so the conventions are not being distinguished."
fi
echo "C3 passes: the conventions disagree on at least one term, so they are distinct instruments."
echo

# --- the census --------------------------------------------------------------
echo "target : $TARGET"
echo "244 L4 claims, in its own order: sealed=61 value-unique=8 NonZero=6 AtLeastTwo=6"
echo
printf '%-14s %8s %8s %8s %8s %10s\n' term substr trailB bothB lines '244 says'
i=0
for t in "${TERMS[@]}"; do
  printf '%-14s %8s %8s %8s %8s %10s\n' "$t" \
    "$(count substr "$t" "$TARGET")" "$(count trailB "$t" "$TARGET")" \
    "$(count bothB  "$t" "$TARGET")" "$(count lines  "$t" "$TARGET")" "${CLAIMED[$i]}"
  i=$((i+1))
done
echo
echo "why the conventions differ, by token:"
for t in "${TERMS[@]}"; do
  echo "  $t:"
  grep -oE -- "$t[A-Za-z0-9_-]*" "$TARGET" | sort | uniq -c | sed 's/^/    /'
  pre=$(grep -oE -- "[A-Za-z0-9_-]$t" "$TARGET" | sort | uniq -c)
  [ -n "$pre" ] && { echo "    preceded by a token character in:"; printf '%s\n' "$pre" | sed 's/^/      /'; }
done
echo
REP=$(reproducing "$TARGET" "${CLAIMED[@]}")
if [ -n "$REP" ]; then
  echo "VERDICT: 244's four numbers reproduce together under:$REP"
else
  echo "VERDICT: no single convention reproduces all four of 244's numbers."
  echo "         Per-term, the conventions that give 244's number are:"
  i=0
  for t in "${TERMS[@]}"; do
    hits=""
    for conv in substr trailB bothB lines; do
      [ "$(count "$conv" "$t" "$TARGET")" = "${CLAIMED[$i]}" ] && hits="$hits $conv"
    done
    printf '           %-14s %s\n' "$t" "${hits:- none}"
    i=$((i+1))
  done
fi
