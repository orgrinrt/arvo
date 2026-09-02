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
# set, so the classification below is over a complete list rather than a sample.
#
# Governing set, stated so the scope of the census is checkable:
#   - every `ruling` row at `rung = "ratified"`
#   - every `proposal` row at `standing = "two_experts"`, which is the tier
#     `ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`
#     puts level with a stamp
#
# The control is a planted row carrying exactly the sentence that would refute the
# claim. Without it a zero here is a fact about the grep.
set -u
REG=${REG:-../../../../registry}
OUT=$(mktemp -d)

extract() {           # $1 = file, $2 = field, $3 = value
  awk -v f="$2" -v v="$3" '
    BEGIN { RS=""; }
    {
      if ($0 ~ (f " = \"" v "\"")) {
        id=""; says="";
        n=split($0, L, "\n");
        for (i=1;i<=n;i++) {
          if (L[i] ~ /^id = /)   { id = L[i]; sub(/^id = "/,"",id); sub(/"$/,"",id) }
          if (L[i] ~ /^says = /) { says = L[i] }
        }
        if (id != "") printf "%s\t%s\n", id, says;
      }
    }' "$1"
}

echo "======== the governing set"
extract "$REG/ruling.toml"   rung     ratified    > "$OUT/rows.tsv"
extract "$REG/proposal.toml" standing two_experts >> "$OUT/rows.tsv"
extract "$REG/proposal-the-later-topics.toml" standing two_experts >> "$OUT/rows.tsv"
echo "rows in the governing set: $(wc -l < "$OUT/rows.tsv" | tr -d ' ')"
echo "  ratified rulings          : $(extract "$REG/ruling.toml" rung ratified | wc -l | tr -d ' ')"
echo "  two-expert proposals      : $(( $(extract "$REG/proposal.toml" standing two_experts | wc -l) + $(extract "$REG/proposal-the-later-topics.toml" standing two_experts | wc -l) ))"
echo

PAT='reduction|adaptation|verdict|law inventory|encoding'
echo "======== rows whose statement names something outside (ambient, representable set)"
echo "pattern: $PAT"
grep -nE "$PAT" "$OUT/rows.tsv" | cut -f1 | sed 's/^/  /'
echo "  count: $(grep -cE "$PAT" "$OUT/rows.tsv")"
echo
echo "-------- their statements, in full, for classification by subject"
grep -E "$PAT" "$OUT/rows.tsv" | while IFS=$'\t' read -r id says; do
  echo "### $id"
  echo "$says" | sed 's/^says = //' | fold -s -w 96 | sed 's/^/    /'
  echo
done

echo "======== CONTROL: a planted row carrying the sentence that would refute the claim"
cp "$REG/ruling.toml" "$OUT/planted.toml"
cat >> "$OUT/planted.toml" <<'ROW'

[[ruling]]
id = "planted_control_a_candidate_exposes_its_reductions_verdicts"
kind = "ruling"
rung = "ratified"
topic = "the_number_system"
says = "A candidate number system exposes the verdicts of its selected reduction, and the admission contract asks for them."
ROW
echo "planted row flagged by the same instrument:"
extract "$OUT/planted.toml" rung ratified | grep -E "$PAT" | cut -f1 | grep planted | sed 's/^/  /'
echo "  (if this line is empty the census above proves nothing)"
rm -rf "$OUT"
