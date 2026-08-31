#!/usr/bin/env nutshell
# The corpus does reason about the algorithm surface. Where does it file the
# result?
#
# `187` measured that nothing reaches five obligations and read that as the
# panel never having touched the subject. This asks the narrower question its
# instrument could not: are there rows whose *content* is about computing an
# algorithm over arvo numerals, and if so, what `topic` do they carry?
#
# The hypothesis under test is that such rows exist and are filed under numeral
# topics, so every instrument pointed at the demand side is blind to them by
# construction. That is a claim about `topic` values, and it is falsifiable:
# if any of these rows carries `topic = "arvo_identity"` or a topic naming the
# surface above the numeral, the hypothesis is wrong for that row.
#
# The terms are chosen for the *algebra an algorithm computes in* rather than
# for the algorithm's name, because the algorithm names are the deleted crate
# tree's and `184` is right that reasoning from them is reattaching a dead tier.
# What survives a rename is the algebra: min-plus, monotonicity, absorption,
# shortest path, spectral, a total order.
#
# CONTROL, two of them, and the run does not count without both.
#   1. A term that must hit: `saturat`, which is everywhere. If it reports zero
#      the field extractor is broken.
#   2. A term that must miss: `zzz_no_such_term`. If it reports hits the matcher
#      is matching something other than what it is given.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
reg="$root/mock/registry"

rows=$(mktemp); trap 'rm -f "$rows"' EXIT

# One record per row: namespace, id, topic, and the row's whole narrative text
# joined. Topic is captured wherever it appears in the row, since the field
# order is not fixed.
for f in "$reg"/*.toml; do
  awk -v ns="$(basename "$f" .toml)" '
    function flush() {
      if (id != "") printf "%s\t%s\t%s\t%s\n", ns, id, (tp == "" ? "(none)" : tp), tolower(txt)
      id=""; tp=""; txt=""
    }
    /^\[\[/                { flush(); next }
    /^id = /               { l=$0; gsub(/^id = "|"$/, "", l); id=l; next }
    /^topic = /            { l=$0; gsub(/^topic = "|"$/, "", l); tp=l; next }
    /^(says|claim|why|asks|instead|replacement|note|states|statement|witness|need|defect|gap|because) = / {
                             txt = txt " " $0; next }
    /^ *"/                 { txt = txt " " $0; next }   # options / holds / fails entries
    END                    { flush() }
  ' "$f" >> "$rows"
done
echo "### rows with narrative text: $(grep -c . "$rows")"
echo

for term in "min-plus" "tropical" "shortest path" "monotonic" "absorb" "spectral" "total order" "laplacian" "eigen" "critical path" "saturat" "zzz_no_such_term"; do
  echo "######## $term"
  { grep -i -- "$term" "$rows" || true; } | awk -F'\t' '{printf "    %-28s %-46s topic=%s\n", $1, $2, $3}' | sort -u
  n=$({ grep -ic -- "$term" "$rows" || true; })
  echo "    -- $n rows"
  echo
done
