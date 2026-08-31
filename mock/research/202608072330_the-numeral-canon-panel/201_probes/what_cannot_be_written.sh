#!/usr/bin/env bash
# What a designer could not write from the canon, measured with the canon's own
# field rather than by reading it and forming an impression.
#
# Op's bar is that the canon be exhaustive enough that a full design and then a
# full implementation can be done from it. The `question` namespace carries a
# field for exactly the thing that bar turns on: `unblocks`, described in the
# schema as "What becomes writable once it is answered. This is what makes a
# question worth an interruption rather than a note, and a question that
# unblocks nothing is one to ask later or never."
#
# So an open question carrying an `unblocks` is a sentence, written by this
# corpus about itself, naming something that cannot be written yet. That is the
# measurement, and it needs no judgement from me about what a designer needs.
#
# A question is answered when a `ruling` or a `proposal` names it under
# `answers`, which is the same walk `obligation.rs` performs and the same tiers:
# a ruling is op and is an answer, a proposal is proposed and is not.
#
# Controls, both required:
#   POSITIVE  a planted ruling edge against an open question must move it.
#   NEGATIVE  a slug naming no question must not be counted as answering one.
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd)"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
R="$root/mock/registry"

edges=$(mktemp); trap 'rm -f "$edges"' EXIT
awk '
  /^\[\[/ { ns = substr($0, 3, length($0) - 4); next }
  /^answers = / {
    line = $0; gsub(/^answers = \[|\]$/, "", line); gsub(/"/, "", line)
    n = split(line, t, ",")
    for (i = 1; i <= n; i++) { gsub(/^ +| +$/, "", t[i]); if (t[i] != "") print ns "\t" t[i] }
  }
' "$R/ruling.toml" "$R"/proposal*.toml > "$edges"

if [ "${1:-}" = "--control" ]; then
  # Plant on a question that is currently OPEN. The first version of this
  # control took the first question in the file, which is already answered, so
  # the planted edge changed nothing and the run came out identical to the
  # baseline. That is indistinguishable from an instrument that reads no edges
  # at all, and it is the defect this suite exists to refuse: a control that
  # cannot move the number is not a control.
  open1=$(awk '/^id = /{gsub(/^id = "|"$/,"");print}' "$R/question.toml" \
    | while read -r q; do grep -q "	$q\$" "$edges" || { echo "$q"; break; }; done)
  printf 'ruling\t%s\n' "$open1" >> "$edges"
  printf 'ruling\tZZZ_NO_SUCH_QUESTION\n' >> "$edges"
  echo "  [control] planted a ruling edge at the first OPEN question, $open1,"
  echo "            and one at a slug naming no question at all"
fi

# id \t decider \t unblocks, with unblocks folded from its possibly-wrapped value
rows=$(awk '
  /^\[\[question\]\]/ { if (id != "") print id "\t" dec "\t" unb; id=""; dec=""; unb=""; inu=0; next }
  /^id = /       { gsub(/^id = "|"$/, ""); id = $0; next }
  /^decider = /  { gsub(/^decider = "|"$/, ""); dec = $0; next }
  # A single-line value already ends in its own quote. The first version set the
  # continuation flag unconditionally, so every `unblocks` swallowed the `note`
  # that followed it and the printed text was two fields joined. The counts were
  # unaffected, which is why it survived a run: it only shows when you read the
  # output, and reading the output was the point.
  /^unblocks = / {
    u = $0; sub(/^unblocks = "/, "", u)
    if (u ~ /"$/) { sub(/"$/, "", u); unb = u; inu = 0 } else { unb = u; inu = 1 }
    next
  }
  inu            { if ($0 ~ /"$/) { sub(/"$/, "", $0); inu = 0 } ; unb = unb " " $0; next }
  END            { if (id != "") print id "\t" dec "\t" unb }
' "$R/question.toml")

tier() {
  if grep -q "^ruling	$1\$"   "$edges"; then echo answered
  elif grep -q "^proposal	$1\$" "$edges"; then echo proposed
  else echo open; fi
}

echo "######## open questions that name what they block"
n=0
printf '%s\n' "$rows" | while IFS=$'\t' read -r id dec unb; do
  [ -n "$unb" ] || continue
  t=$(tier "$id")
  [ "$t" = open ] || continue
  printf '\n  %s   [decider: %s]\n' "$id" "$dec"
  printf '%s\n' "$unb" | fold -s -w 96 | sed 's/^/      /'
done

echo
echo "######## every question, by tier and by decider"
printf '%s\n' "$rows" | while IFS=$'\t' read -r id dec unb; do
  printf '%s\t%s\n' "$(tier "$id")" "$dec"
done | sort | uniq -c | sed 's/^/  /'

echo
echo "######## totals"
printf '%s\n' "$rows" | while IFS=$'\t' read -r id dec unb; do tier "$id"; done \
  | sort | uniq -c | sed 's/^/  /'
echo "  questions carrying an unblocks: $(printf '%s\n' "$rows" | awk -F'\t' '$3!=""' | wc -l | tr -d ' ')"
echo "  of those, still open:           $(printf '%s\n' "$rows" | while IFS=$'\t' read -r i d u; do [ -n "$u" ] && tier "$i"; done | grep -c open || true)"
