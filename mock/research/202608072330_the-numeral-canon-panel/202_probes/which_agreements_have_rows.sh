#!/usr/bin/env nutshell
# For each multi-instance agreement in `AGREEMENTS.md` sections 2.1, 3.1, 4.1,
# 5.1 and 5.2, does a registry row carry it?
#
# Those five subsections are the ledger's own top tier: agreements the source
# consolidation itself calls independent. The lower tiers are different things
# and are handled in the deliverable rather than here, because a contested item
# is a question and a single-expert claim carried as if settled is a claim about
# a consolidation's conduct.
#
# METHOD, and its bound stated up front because two of my own earlier
# instruments were read past theirs. This is **row-aware**, not line-aware: it
# finds the row a phrase sits in anywhere, in any field, and prints that row's
# id. A first version filtered to matches on the `says =` line itself and
# reported five items absent that are present, because the phrase sat in
# `because` or `note`. Transcript: `which_agreements_have_rows_v1_line_aware.out`.
#
# It is still a net. A phrase is my choice, a miss is not proof of absence, and
# every verdict in the deliverable was reached by opening the row rather than by
# reading this table. What it is for is the other direction: an item with no hit
# under several independently chosen phrasings is one worth opening the corpus
# for, and the run is the record of which phrasings were tried.
#
# CONTROLS, three.
#   POSITIVE-A `a_law_is_a_fact_about_an_operation` must be found. It is the one
#     item I confirmed by hand before writing this, so a miss means the walker is
#     broken rather than the corpus empty.
#   POSITIVE-B a phrase from a row's `note` field must be found, since the
#     line-aware defect above was exactly a failure to read non-`says` fields.
#   NEGATIVE a phrase in no row must report nothing.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
reg="$root/mock/registry"

# One record per row: file, id, and the whole row's text flattened.
rows=$(mktemp); trap 'rm -f "$rows"' EXIT
for f in "$reg"/*.toml; do
  awk -v ns="$(basename "$f" .toml)" '
    function flush() { if (id != "") printf "%s\t%s\t%s\n", ns, id, tolower(txt); id=""; txt="" }
    /^\[\[/  { flush(); next }
    /^id = / { l=$0; gsub(/^id = "|"$/, "", l); id=l; next }
             { txt = txt " " $0 }
    END      { flush() }
  ' "$f" >> "$rows"
done
echo "### rows scanned: $(grep -c . "$rows")"
echo

look() {
  label="$1"; shift
  echo "######## $label"
  any=0
  for phrase in "$@"; do
    hits=$({ grep -i -- "$phrase" "$rows" || true; } | awk -F'\t' '{print "      " $1 " :: " $2}' | sort -u)
    n=$(printf '%s' "$hits" | grep -c . || true)
    printf "    %-58s %s\n" "\"$phrase\"" "$n"
    [ "$n" -eq 0 ] || { printf '%s\n' "$hits"; any=1; }
  done
  [ "$any" -eq 1 ] || echo "    -- NO ROW under any phrase tried"
  echo
}

echo "================ 2.1  the format concept"
look "C1 the standard model, computed = adapt(exact)" "adaptation of an exact" "adapt(exact)" "exact operation"
look "C2 identity is denotation rather than encoding" "denotation" "identity and" "membership in the type"
look "C3 the representable set is one affine predicate" "affine predicate" "representable set is one" "membership of the representable"
look "C5 the absorption criterion" "absorption decides" "absorbing"
look "scale-independence of the additive column" "scale independen" "scale-independen" "additive column"

echo "================ 3.1  the number-system concept"
look "the number-numeral distinction, laws as identity" "number system" "laws as identity" "numeral concept"
look "the crate-table cross-check is worth zero" "crate table" "crate tree" "removed crate"
look "strategy selects the correctness relation" "makes an answer correct" "correct answer is"

echo "================ 4.1  derived algebraic laws"
look "R1 a law is a fact about an operation" "fact about an operation"
look "the reproduction chain never broke" "reproduction chain" "rerun matched" "re-run"
look "the band-transfer defeat in both fragments" "band transfer" "band_transfer"

echo "================ 5.1  three or more instances"
look "the selection erases at compile time" "erases at compile time" "compile-time argmin" "compile time argmin" "symbol alias"
look "the rationalisability counts" "rationalisab"
look "distributivity at F = 0, and the signed re-measure" "distributivity" "47.72"
look "the corpus test suite is 123 tests across 13 crates" "123 across" "123 tests" "thirteen crates"

echo "================ 5.2  two experts"
look "a strategy is a preference over measurements" "preference over measurements"
look "chain accuracy needs a non-closed operator" "closed over its operand type"
look "the named strategies are points in a product" "points in a product"
look "175 of 254 bench regions predate the validation" "175 of 254" "cross-variant validation" "bench region"

echo "================ CONTROLS"
look "POSITIVE-A the item confirmed by hand first" "fact about an operation composed under"
look "POSITIVE-B a phrase from a note field only" "the work order for the porting seats"
look "NEGATIVE a phrase in no row" "zzz phrase that appears in no row anywhere"
