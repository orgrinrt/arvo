#!/usr/bin/env bash
# Seat 253. The instrument that attacks my own central claim.
#
# The claim: what admission asks a candidate to expose is fixed by the ratified
# identity clause, so it is the ambient domain and the representable set, and
# nothing that names a reduction, an adaptation, a verdict or a law inventory is
# asked of a *candidate*.
#
# The refutation would be a governing sentence whose subject is a candidate and
# which reads something outside that pair. This extracts every such sentence and
# flags the ones that name one of those words, exhaustively over the governing
# set, so the classification in `classification.md` is over a complete list
# rather than a sample.
#
# Governing set, stated so the scope of the census is checkable:
#   - every `ruling` row at `rung = "ratified"`
#   - every `proposal` row at `standing = "two_experts"`, which is the tier
#     `ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`
#     puts level with a stamp
#
# **The extractor is a state machine over `[[table]]` headers, not paragraph
# mode.** A first version used `RS=""` and lost exactly one ratified row,
# `warms_objective_is_the_intuitive_best_choice`, whose `quote` block contains a
# blank line and therefore splits into two paragraphs, the second of which
# carries `rung` and no `id`. The count control below is what caught it, and it
# is why the count control exists: a census that cannot say how many rows it saw
# cannot say it saw all of them.
#
# Two controls. A count, cross-checked against `cargo mock query`, so a missed
# row is visible. And a planted row carrying exactly the sentence that would
# refute the claim, so a zero is a fact about the corpus rather than the grep.
set -u
REG=${REG:-../../../../registry}
OUT=$(mktemp -d)

extract() {           # $1 = file, $2 = table name, $3 = field, $4 = value
  awk -v tbl="[[$2]]" -v f="$3" -v v="$4" '
    $0 == tbl { flush(); id=""; says=""; hit=0; next }
    /^id = / { id = $0; sub(/^id = "/,"",id); sub(/"$/,"",id) }
    /^says = / { says = $0 }
    $0 == (f " = \"" v "\"") { hit = 1 }
    END { flush() }
    function flush() { if (id != "" && hit) printf "%s\t%s\n", id, says }
  ' "$1"
}

echo "======== the governing set"
nr=$(extract "$REG/ruling.toml"   ruling   rung     ratified    | tee "$OUT/rows.tsv" | wc -l | tr -d ' ')
np=$( { extract "$REG/proposal.toml" proposal standing two_experts
        extract "$REG/proposal-the-later-topics.toml" proposal standing two_experts
      } | tee "$OUT/p.tsv" | wc -l | tr -d ' ')
cat "$OUT/p.tsv" >> "$OUT/rows.tsv"
echo "  ratified rulings     : $nr"
echo "  two-expert proposals : $np"
echo "  rows in the set      : $(wc -l < "$OUT/rows.tsv" | tr -d ' ')"
echo
echo "-------- COUNT CONTROL: the same two counts from a second instrument"
echo "  grep 'rung = \"ratified\"' in ruling.toml   : $(grep -c 'rung = "ratified"' "$REG/ruling.toml")"
echo "  grep 'standing = \"two_experts\"' in both   : $(( $(grep -c 'standing = "two_experts"' "$REG/proposal.toml") + $(grep -c 'standing = "two_experts"' "$REG/proposal-the-later-topics.toml") ))"
echo "  (if either disagrees with the counts above, the extractor is lossy)"
echo

PAT='reduction|adaptation|verdict|law inventory|encoding'
echo "======== rows whose STATEMENT names something outside (ambient, representable set)"
echo "pattern, matched against the says field only: $PAT"
cut -f2 "$OUT/rows.tsv" > "$OUT/says.txt"
paste -d'\t' <(cut -f1 "$OUT/rows.tsv") "$OUT/says.txt" \
  | awk -F'\t' -v p="$PAT" '$2 ~ p {print "  " $1}'
echo "  count: $(awk -F'\t' -v p="$PAT" '$2 ~ p' "$OUT/rows.tsv" | wc -l | tr -d ' ')"
echo
echo "-------- their statements, in full, for classification by subject"
awk -F'\t' -v p="$PAT" '$2 ~ p {print $1 "\t" $2}' "$OUT/rows.tsv" | while IFS=$'\t' read -r id says; do
  echo "### $id"
  echo "$says" | sed 's/^says = //' | fold -s -w 96 | sed 's/^/    /'
  echo
done

echo "======== PLANTED CONTROL: a row carrying the sentence that would refute the claim"
cp "$REG/ruling.toml" "$OUT/planted.toml"
cat >> "$OUT/planted.toml" <<'ROW'

[[ruling]]
id = "planted_control_a_candidate_exposes_its_reductions_verdicts"
kind = "ruling"
rung = "ratified"
topic = "the_number_system"
says = "A candidate number system exposes the verdicts of its selected reduction, and the admission contract asks for them."
ROW
echo "  planted row flagged by the same pipeline:"
extract "$OUT/planted.toml" ruling rung ratified | awk -F'\t' -v p="$PAT" '$2 ~ p && $1 ~ /planted/ {print "    " $1}'
echo "  (if that line is empty the census above proves nothing)"
rm -rf "$OUT"
