#!/usr/bin/env bash
# Registry prose that says an axis is not declared, checked against the axes
# that are declared now.
#
# Why. `dimension.toml`'s header settles that declaring an axis does not reach
# backwards: a predicate's absence quantifies over the world rather than over
# this file's contents, so no written region changes meaning when the set
# grows. That is right about predicates and says nothing about PROSE. A `gap`
# or a `note` saying "because X is not a declared axis" is a claim about this
# file's contents, and it is exactly the kind of sentence declaring X falsifies.
# Nothing checks it, so this counts them.
#
# Controls, outcomes written before the run:
#   S1  a planted sentence naming a declared axis must be reported STALE.
#   S2  a planted sentence naming an axis nobody declares must be reported LIVE.
#       Without S2 the arm could be reporting everything stale.
#   S3  the pattern must match nothing in a file with no such sentence, checked
#       against `strategy.toml`, which carries no gap prose at all.
set -uo pipefail
cd "$(dirname "$0")"
REG=../../../registry
DECLARED=$(grep '^id = ' "$REG/dimension.toml" | sed 's/id = "//; s/"//')

# The axis a sentence names, matched by the declared rows' own keywords rather
# than by their slugs, because the prose writes "operand window" and the slug is
# "operand_window".
names_declared() {  # $1 = sentence
  local s; s=$(printf '%s' "$1" | tr 'A-Z' 'a-z')
  for d in $DECLARED; do
    local w; w=$(printf '%s' "$d" | tr '_' ' ')
    case "$s" in *"$w"*) echo "$d"; return 0 ;; esac
  done
  return 1
}

scan() {  # $1 = file to scan
  grep -nE "not a declared axis|no declared axis|nothing declares that axis|no axis existed|is not declared as an axis" "$1" 2>/dev/null \
  | while IFS= read -r line; do
      local n txt hit
      n=${line%%:*}; txt=${line#*:}
      if hit=$(names_declared "$txt"); then
        printf 'STALE  %s:%s  names `%s`, which dimension.toml declares\n' "$(basename "$1")" "$n" "$hit"
      else
        printf 'LIVE   %s:%s  names no declared axis\n' "$(basename "$1")" "$n"
      fi
    done
}

echo "### registry prose asserting an axis is undeclared"
for f in "$REG"/*.toml; do scan "$f"; done
echo

tmp=$(mktemp -d)
printf 'gap = "on neither list, because the operand window is not a declared axis."\n' > "$tmp/s1.toml"
printf 'gap = "on neither list, because the lunar phase is not a declared axis."\n'    > "$tmp/s2.toml"
echo "### S1, a planted sentence naming a declared axis must read STALE"
scan "$tmp/s1.toml" | sed 's/^/  /'
scan "$tmp/s1.toml" | grep -q '^STALE' && echo "  PASS" || echo "  FAIL"
echo "### S2, a planted sentence naming an undeclared axis must read LIVE"
scan "$tmp/s2.toml" | sed 's/^/  /'
scan "$tmp/s2.toml" | grep -q '^LIVE' && echo "  PASS" || echo "  FAIL"
echo "### S3, a file with no such sentence must produce nothing"
if [ -z "$(scan "$REG/strategy.toml")" ]; then echo "  PASS, strategy.toml silent"; else
  echo "  FAIL:"; scan "$REG/strategy.toml" | sed 's/^/    /'; fi
rm -rf "$tmp"

echo
echo "### S4, the tense arm, which is what actually separates the five hits."
echo "###     The attribution arm above is unreliable and its failure is left in"
echo "###     view: it reads the first declared axis name appearing anywhere on the"
echo "###     line, and a TOML value is one long line, so law.toml:170 is"
echo "###     attributed to strategy because its sentence also says 'the strategy"
echo "###     unit records'. Two of the five attributions are wrong. The verdict"
echo "###     column is not: what separates a false sentence from a correct"
echo "###     historical one is TENSE. \"no axis existed\" is a past-tense account of"
echo "###     why a row was written and stays true. \"is not a declared axis\" is a"
echo "###     present-tense claim about this file's contents and is false the"
echo "###     moment the axis is declared."
for f in "$REG"/*.toml; do
  grep -nE "not a declared axis|no declared axis|nothing declares that axis|no axis existed|is not declared as an axis" "$f" 2>/dev/null \
  | while IFS= read -r line; do
      n=${line%%:*}; txt=${line#*:}
      case "$txt" in
        *"no axis existed"*|*"could carry"*|*"neither could gate"*)
          printf '  PAST     %s:%s\n' "$(basename "$f")" "$n" ;;
        *)
          printf '  PRESENT  %s:%s  <- false against the current dimension set\n' "$(basename "$f")" "$n" ;;
      esac
    done
done
