#!/usr/bin/env nutshell
# Does the corpus reach the five plan-chain obligations, when the search covers
# every namespace rather than the three `187` read?
#
# `187_probes/obligation_reach.sh` searched the `says` field in `ruling.toml`,
# `proposal.toml` and `proposal-the-later-topics.toml`. Six namespaces were not
# searched at all, and `retirement` is one of them. A retirement is a sentence
# about a thing exactly as much as a proposal is: it records that the corpus
# considered the thing and rejected a way of doing it, which is the opposite of
# never reaching it.
#
# So this widens on two axes and reports the namespace per hit, because widening
# on both and printing one number cannot say which axis found what:
#
#   A. every registry file, not three.
#   B. every narrative field (says, claim, why, asks, instead, replacement,
#      note, states, need, defect, gap), not `says` alone.
#
# The net is the obligation's own `keywords`, as in `187`, minus generic keys
# that made that report 46 of 52 noise. Withholding keys is a choice, so the
# withheld list is printed and a reader can see exactly what was suppressed.
#
# CONTROL, and the run does not count without it. `--control` appends one
# synthetic record carrying phrases from the obligations `187` measured at zero.
# It must surface. An all-zero report from a grep that cannot match is
# indistinguishable from a corpus that says nothing, and that exact failure
# killed `187`'s first run.
#
# NOTE on nutshell: it does not set `$0` to the script path, ever, under either
# shebang invocation or `nutshell <file>`. `$0` is the interpreter. So the
# `here="$(dirname "$0")"` idiom other probes here use resolves into nutshell's
# own bin directory. Walk up for `mockspace.toml` instead.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
reg="$root/mock/registry"

drop='^(set|sets|cost|ordering|partition|primitive|format|container|conversion|denotation|contract|guarantee|composition|chain|fold|mask|difference|union|intersection|rank|hash)$'
echo "### generic keys withheld: $drop"
echo "### registry: $reg"
echo

scan=$(mktemp); trap 'rm -f "$scan"' EXIT

for f in "$reg"/*.toml; do
  awk -v file="$(basename "$f" .toml)" '
    /^id = / { line=$0; gsub(/^id = "|"$/, "", line); id=line; next }
    /^(says|claim|why|asks|instead|replacement|note|states|need|defect|gap) = / {
      print file "\t" id "\t" tolower($0)
    }
  ' "$f" >> "$scan"
done
echo "### scanned records: $(grep -c . "$scan")"
echo

if [ "${1:-}" = "--control" ]; then
  printf 'CONTROL\tPLANTED_ROW\tsays = "a fiedler eigenvector partition over a csr adjacency, topologically sorted"\n' >> "$scan"
  echo "### control row planted"
  echo
fi

awk '
  /^id = /       { line=$0; gsub(/^id = "|"$/, "", line); id=line }
  /^keywords = / { k=$0; gsub(/^keywords = \[|\]$/, "", k); gsub(/"/, "", k); print id "\t" k }
' "$reg/obligation.toml" | while IFS=$'\t' read -r ob keys; do
  hits=$(printf '%s' "$keys" | tr ',' '\n' | sed 's/^ *//; s/ *$//' | while read -r k; do
    [ ${#k} -ge 3 ] || continue
    if printf '%s' "$k" | grep -qiE "$drop"; then continue; fi
    { grep -i -- "$k" "$scan" || true; } \
      | awk -F'\t' -v k="$k" '{print $1 " :: " $2 "\t(" k ")"}'
  done | sort -u)
  n=$(printf '%s' "$hits" | grep -c . || true)
  ns=$(printf '%s' "$hits" | awk -F' :: ' 'NF>1{print $1}' | sort -u | tr '\n' ' ')
  echo "################ $ob"
  echo "    $n hits, namespaces: ${ns:-(none)}"
  [ "$n" -eq 0 ] || printf '%s\n' "$hits" | sed 's/^/      /'
  echo
done
