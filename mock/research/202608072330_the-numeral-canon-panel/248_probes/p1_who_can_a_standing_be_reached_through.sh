#!/usr/bin/env nutshell
# p1. What a multi-arrival `standing` can actually be reached through.
#
# `mock/lints/a_standing_is_reachable_from_what_it_cites.rs` asks whether a row
# claiming several arrivals cites two distinct FILES. Its own doc comment says
# why: "Independence is between authors, and a numbered member file has one
# author". But `files_cited` counts any file, and three of the things a
# `provenance` entry can name are not authors at all: a living ledger, a probe
# artifact, and the topic consolidation the claim was compressed into, which
# `ruling::the_canon_is_written_once_at_the_end` says "has no standing beyond
# that" and `mock/lints/a_proposal_rests_on_more_than_a_consolidation.rs`
# already refuses as a sole source.
#
# So this re-runs the lint's own predicate and then re-runs it counting only
# what can be an independent author.
#
# THE CASES THAT MUST FAIL, STATED BEFORE THE RUN.
#
# C1. The lint arm must reproduce 29, the number `CEILING` in that lint was
#     measured at over this same committed corpus. Anything else means this
#     script's copy of `file_named` is not the lint's and every figure below is
#     about this script.
# C2. `membership_in_the_type_and_identity_are_two_criteria` cites file 161
#     twice, at two line anchors. It must come out at ONE file. If two anchors
#     into one file survive as two, the set-collapse is dead and the lint arm
#     passes for the wrong reason.
# C3. `the_concept_is_closed_and_the_inventory_is_open` cites 65 and 66, the two
#     cold number-system derivations, plus consolidation 74. The author arm must
#     report 2. If it reports 0 the classifier is eating member files and the
#     headline is vacuous.
# C4. `chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type`
#     cites consolidation 106 and `AGREEMENTS`, a living ledger. The author arm
#     must report 0. If a ledger counts as an author the whole distinction is
#     decorative.
#
# All four are asserted at the end and the script exits non-zero on any of them.
use log

ROOT="$(git rev-parse --show-toplevel)"
REG="$ROOT/mock/registry"

# `file_named` from mock/lints/canon_citations.rs, transcribed, plus a kind for
# each named file. The kinds are read off the citation grammar the panel uses:
#   ledger      one of LIVING_LEDGERS, edited after it is cited
#   probe       an artifact with an extension, or sitting under NNN_probes
#   consolid    a numbered member file with `consolidation` as a whole segment
#               and `entailment` absent, per is_a_consolidation
#   author      anything else, which is a numbered member file with one author
read -r -d '' AWKSRC <<'AWK' || true
function is_anchor(s,   i,c) {
  if (substr(s,1,1) == "#") return 1
  if (s == "") return 0
  for (i=1;i<=length(s);i++) { c=substr(s,i,1); if (c !~ /[0-9]/) return 0 }
  return 1
}
function file_named(cit,   n,p) {
  n = split(cit, p, "::")
  if (n < 2) return ""
  if (is_anchor(p[n]) && n >= 3) return p[n-1]
  return p[n]
}
function kind_of(f, cit,   stem,segs,i,nseg) {
  if (f ~ /^(AGREEMENTS|OPTIONS|DROPLIST|RULES|INTENTS|PRIOR_CALLS|HANDLES|PERSONA_CALLS|SEED_TALKING_POINTS)(\.md)?$/)
    return "ledger"
  if (f ~ /\./) return "probe"
  if (cit ~ /_probes::/) return "probe"
  stem = f
  nseg = split(stem, segs, "_")
  for (i=1;i<=nseg;i++) if (segs[i] == "entailment") return "author"
  for (i=1;i<=nseg;i++) if (segs[i] == "consolidation") return "consolid"
  return "author"
}
/^\[\[/ { if (id != "") emit(); id=""; standing=""; delete files; delete kinds; nf=0; inprov=0 }
/^id = / { v=$0; sub(/^id = "/,"",v); sub(/"$/,"",v); id=v; next }
/^standing = / { v=$0; sub(/^standing = "/,"",v); sub(/"$/,"",v); standing=v; next }
{
  if ($0 ~ /^provenance = /) inprov=1
  else if (inprov && $0 ~ /^[a-z_]+ = /) inprov=0
  if (inprov) {
    line=$0
    if ($0 ~ /^provenance = /) sub(/^provenance = /,"",line)
    while (match(line, /"[^"]*"/)) {
      cit=substr(line, RSTART+1, RLENGTH-2)
      line=substr(line, RSTART+RLENGTH)
      f=file_named(cit)
      if (f != "" && !(f in files)) { files[f]=1; kinds[f]=kind_of(f, cit); nf++ }
    }
    if ($0 ~ /\]$/ && $0 !~ /^provenance = \[$/) inprov=0
  }
}
END { if (id!="") emit() }
function emit(  k,na,s) {
  na=0; s=""
  for (k in files) { if (kinds[k]=="author") na++; s = s (s==""?"":" ") k "(" kinds[k] ")" }
  print id "\t" standing "\t" nf "\t" na "\t" s
}
AWK

awk "$AWKSRC" "$REG/proposal.toml" "$REG/proposal-the-later-topics.toml" > /tmp/p1_rows.tsv

MULTI='($2=="two_experts"||$2=="three_or_more"||$2=="cross_topic")'

echo "== every proposal row, standing x distinct-files x distinct-author-files =="
awk -F'\t' '{print $2"\t files="$3"\t authors="$4}' /tmp/p1_rows.tsv | sort | uniq -c | sort -k2,2
echo
TOTAL=$(wc -l < /tmp/p1_rows.tsv | tr -d ' ')
echo "rows read: $TOTAL"
echo

echo "== arm 1: the lint's own predicate, distinct files < 2 =="
LINT_N=$(awk -F'\t' "$MULTI && \$3<2" /tmp/p1_rows.tsv | wc -l | tr -d ' ')
echo "multi-arrival rows citing fewer than two distinct files: $LINT_N"
echo

echo "== arm 2: the same rows, counting only files that can be an author =="
MULTI_TOTAL=$(awk -F'\t' "$MULTI" /tmp/p1_rows.tsv | wc -l | tr -d ' ')
AUTH_N=$(awk -F'\t' "$MULTI && \$4<2" /tmp/p1_rows.tsv | wc -l | tr -d ' ')
echo "multi-arrival rows total:                              $MULTI_TOTAL"
echo "of those, reaching fewer than two independent authors:  $AUTH_N"
echo
echo "-- the rows that DO reach two authors --"
awk -F'\t' "$MULTI && \$4>=2 {print \"  \" \$1 \"  [\" \$2 \"]  \" \$5}" /tmp/p1_rows.tsv
echo
echo "-- the rows that reach two FILES but fewer than two authors --"
awk -F'\t' "$MULTI && \$3>=2 && \$4<2 {print \"  \" \$1 \"  [\" \$2 \"]  \" \$5}" /tmp/p1_rows.tsv
echo

fail=0
chk() { # name expected actual
  if [ "$2" = "$3" ]; then echo "  PASS $1 (= $3)"; else echo "  FAIL $1: expected $2, got $3"; fail=1; fi
}
echo "== the cases that had to fail =="
chk "C1 lint arm reproduces the measured CEILING" 29 "$LINT_N"
C2=$(awk -F'\t' '$1=="membership_in_the_type_and_identity_are_two_criteria"{print $3}' /tmp/p1_rows.tsv)
chk "C2 two anchors into file 161 collapse to one file" 1 "${C2:-MISSING}"
C3=$(awk -F'\t' '$1=="the_concept_is_closed_and_the_inventory_is_open"{print $4}' /tmp/p1_rows.tsv)
chk "C3 the 65/66 cold pair counts as two authors" 2 "${C3:-MISSING}"
C4=$(awk -F'\t' '$1=="chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type"{print $4}' /tmp/p1_rows.tsv)
chk "C4 a living ledger is not an author" 0 "${C4:-MISSING}"
exit $fail
