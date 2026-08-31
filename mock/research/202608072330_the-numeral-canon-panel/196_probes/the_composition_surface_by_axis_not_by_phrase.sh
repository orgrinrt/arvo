#!/usr/bin/env nutshell
# Is the fold-and-chain reconciliation absent from the registry, or was it
# searched for in the wrong vocabulary?
#
# WHY THIS RUNS. `191` section 2.3 states: "The registry contains two results
# about composition and they contradict each other on the operator, and nothing
# reconciles them." It then reports its search: "I searched for the positive
# statement of this. It exists in the registry as keywords only: `trip count`
# appears in question.toml:222 (Q11's keyword list), in retirement.toml:114 and
# in one unrelated row. No says, no claim, no statement carries it."
#
# That search is a PHRASE search for the vocabulary of `191`'s own hypothesis.
# `191` hypothesised the axis is the trip count and then looked for the words
# "trip count". A reconciliation written over a DIFFERENT axis is invisible to
# it, and cannot be distinguished from a reconciliation that was never written.
#
# The corpus declares its axes. `dimension.toml:135` declares `chain_length`.
# So the same question has a mechanical form that does not depend on guessing
# anybody's phrasing: which rows carry a predicate over `chain_length`, and what
# do they say about the operator?
#
# ARM 1 reproduces `191`'s phrase search, so this file agrees with its data
#       before disagreeing with its conclusion. If ARM 1 does not reproduce the
#       three hits `191` reports, one of us is reading a different tree.
# ARM 2 asks the axis question instead.
# ARM 3 splits ARM 2's rows by the `operation` entry in the same predicate,
#       which is what decides whether the two arms of ARM 2 are a contradiction
#       or a partition.
#
# THE CASE THAT MUST FAIL, and the run does not count without all three.
#   CTRL-REPRO  ARM 1 must return exactly the hits `191` reports, in `says`,
#               `claim` and `statement`: zero. If it returns some, `191`'s
#               search was fine and this file's diagnosis is wrong.
#   CTRL-POS    a planted predicate entry `chain_length: chain length = 7` in a
#               planted file must be found by ARM 2's matcher. Without it a zero
#               from ARM 2 would be indistinguishable from a broken matcher, and
#               ARM 2 returning MORE than ARM 1 is the whole claim.
#   CTRL-NEG    a planted `chain_lengthy: nonsense` must NOT be found, or the
#               matcher is a substring net and its count means nothing.
#
# NUTSHELL: `$0` is the interpreter, never the script. Walk up for mockspace.toml.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
reg="$root/mock/registry"
echo "### root: $root"
echo

echo "======== ARM 1. 191's search, reproduced: the phrase 'trip count' ========"
echo "-- every occurrence, with its field --"
grep -rn -i "trip count" "$reg"/*.toml | sed "s|$reg/||" | cut -c1-150 | sed 's/^/   /'
echo
narr=$(grep -rn -i "trip count" "$reg"/*.toml | grep -Ec '^[^:]+:[0-9]+:(says|claim|statement|asks|why|because|establishes) ' || true)
echo "   occurrences in a claim-bearing field (says/claim/statement/asks/why/because/establishes): $narr"
if [ "$narr" -eq 0 ]; then
  echo "   CTRL-REPRO  PASS  reproduces 191: the phrase carries no claim anywhere"
else
  echo "   CTRL-REPRO  FAIL  *** 191's search result does not reproduce; diagnosis void ***"
  exit 3
fi
echo

echo "======== ARM 2. the same question by declared axis: chain_length ========"
if grep -q '^id = "chain_length"' "$reg/dimension.toml"; then
  echo "   dimension.toml declares chain_length: yes"
else
  echo "   *** chain_length is not a declared axis; ARM 2 is meaningless ***"; exit 3
fi
echo
# A predicate entry is `"<slug>: <values>",` on its own line inside a list.
rows() {
  awk '
    /^\[\[(proposal|law)\]\]/ { id=""; kind=""; standing=""; op=""; cl=""; ans=""; next }
    /^id = "/          { if (id=="") { l=$0; sub(/^id = "/,"",l); sub(/"$/,"",l); id=l } ; next }
    /^sentence_kind = /{ l=$0; sub(/^sentence_kind = "/,"",l); sub(/"$/,"",l); kind=l; next }
    /^standing = /     { l=$0; sub(/^standing = "/,"",l); sub(/"$/,"",l); standing=l; next }
    /^answers = /      { ans="yes"; next }
    /"chain_length: /  { l=$0; sub(/^ *"chain_length: /,"",l); sub(/",?$/,"",l); cl=l; next }
    /"operation: /     { l=$0; sub(/^ *"operation: /,"",l); sub(/",?$/,"",l); op=l; next }
    /^keywords = /     { if (cl != "") printf "%-72s | %-9s | %-11s | %-28s | %-24s | answers:%s\n", id, kind, standing, cl, op, (ans==""?"no":ans); id=""; cl=""; op=""; kind=""; standing=""; ans="" }
  ' "$1"
}
found=$( { rows "$reg/proposal.toml"; rows "$reg/proposal-the-later-topics.toml"; rows "$reg/law.toml"; rows "$reg/law-the-later-topics.toml"; } )
printf '%s\n' "$found" | sed 's/^/   /'
n=$(printf '%s\n' "$found" | grep -c . || true)
echo
echo "   rows carrying a chain_length predicate: $n"
echo

echo "======== ARM 3. the split ARM 2's rows make, by operation ========"
# v1 of this arm ran `grep -i mul` over the WHOLE LINE and matched every row
# whose id contains "accumulator". Transcript kept beside this file as
# `_v1_accumulator_matched_mul.out`. The operation is column 5; split on it.
opcol() { printf '%s\n' "$found" | awk -F'|' -v pat="$1" 'tolower($5) ~ pat'; }
echo "-- operation names multiplication --"
opcol "mul" | sed 's/^/   /'
echo "-- operation names addition and not multiplication --"
printf '%s\n' "$found" | awk -F'|' 'tolower($5) ~ /add/ && tolower($5) !~ /mul/' | sed 's/^/   /'
echo "-- both --"
printf '%s\n' "$found" | awk -F'|' 'tolower($5) ~ /add/ && tolower($5) ~ /mul/' | sed 's/^/   /'
echo "-- neither named --"
printf '%s\n' "$found" | awk -F'|' 'tolower($5) !~ /add/ && tolower($5) !~ /mul/' | sed 's/^/   /'
echo
echo "   CTRL-COL  the id a_coherent_reduction_needs_no_accumulator must NOT"
echo "   appear under multiplication. It contains the letters m-u-l inside"
echo "   'accumulator' and v1 put it there."
if opcol "mul" | grep -q "a_coherent_reduction_needs_no_accumulator"; then
  echo "   CTRL-COL  FAIL  *** the v1 defect is still live ***"; exit 3
else
  echo "   CTRL-COL  PASS"
fi
echo

echo "======== ARM 4. how many rows are about composition at all ========"
for ns in proposal law question retirement; do
  for f2 in "$reg/$ns.toml" "$reg/$ns-the-later-topics.toml"; do
    [ -f "$f2" ] || continue
    printf "   %-34s %3s rows with topic = the_chain\n" "$(basename "$f2")" \
      "$(grep -c '^topic = "the_chain"' "$f2" || true)"
  done
done
echo
echo "   191 section 2.3: \"The registry contains two results about composition\"."
echo

echo "======== CONTROLS ========"
plant=$(mktemp -d)
cat > "$plant/planted.toml" <<'EOF'
[[proposal]]
id = "pos_plant_carries_chain_length"
sentence_kind = "measured"
standing = "one_expert"
predicate = [
  "chain_length: chain length = 7",
  "operation: operation = add",
]
keywords = ["planted"]

[[proposal]]
id = "neg_plant_lookalike_axis"
sentence_kind = "measured"
standing = "one_expert"
predicate = [
  "chain_lengthy: nonsense",
]
keywords = ["planted"]
EOF
ph=$(rows "$plant/planted.toml")
if printf '%s\n' "$ph" | grep -q "pos_plant_carries_chain_length"; then
  echo "CTRL-POS  PASS  a planted chain_length predicate is found"
else
  echo "CTRL-POS  FAIL  *** matcher misses a planted row; ARM 2's count is void ***"; rm -rf "$plant"; exit 3
fi
if printf '%s\n' "$ph" | grep -q "neg_plant_lookalike_axis"; then
  echo "CTRL-NEG  FAIL  *** matcher is a substring net ***"; rm -rf "$plant"; exit 3
else
  echo "CTRL-NEG  PASS  a lookalike axis slug is not matched"
fi
rm -rf "$plant"
