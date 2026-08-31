#!/usr/bin/env nutshell
# How much measured fact sits in a namespace that cannot carry a region?
#
# WHY THIS RUNS. `191` section 2.1 reports that two of `35`'s fourteen figures
# reached the registry and concludes the canon "records what the surface above
# the numeral cannot be and says nothing about what it is". Its instrument was a
# substring search for `35`'s own figures. That measures whether a FIGURE
# travelled. It does not measure whether the FACT did, and the two come apart
# the moment a later seat re-established the same thing with its own instrument
# at its own widths, which is the ordinary shape here.
#
# So this asks a different question of the same corpus. `mock/checks` declares
# the fields that carry a region, in `src/predicate.rs`:
#
#     const PREDICATE_FIELDS = [(proposal, predicate), (law, holds), (law, fails)]
#
# Three fields, two namespaces. And `shape::predicate_disagrees_with_the_
# sentence_kind` iterates `reg.of("proposal")` alone. So a substantive measured
# sentence written into `retirement.why` carries no region, no sentence_kind, no
# evidence edge and no standing, and NO CHECK IN THE SUITE ASKS IT FOR ANY OF
# THOSE. The retirement namespace's schema has no such fields to ask for.
#
# The question this answers: how many retirement rows carry a quantified
# measurement inside `why`? Each one is a measured sentence living where the
# corpus's own I13 predicate discipline does not reach, and under that
# discipline an absent axis means the claim holds in no situation involving it,
# so each is a measurement that formally holds nowhere.
#
# THE CASE THAT MUST FAIL, and the run does not count without all four.
#   POS-REAL   `dl_interior_wrapping_with_a_reserved_absorbing_top` must match.
#              It is the row `191` names, and its `why` carries "12.6 percent",
#              "622 million" and "560 of 2176". If the matcher misses it the
#              matcher is broken and every zero below is meaningless.
#   POS-PLANT  a synthetic retirement whose why says "fails at 12 of 16 cells"
#              must match. Distinguishes "the matcher works" from "the matcher
#              happens to know that one row".
#   NEG-PLANT  a synthetic retirement whose why is a pure argument with no
#              figure must NOT match. Without this the arm could be reporting
#              every row and the count would look like a finding.
#   NEG-WORD   a synthetic retirement whose why says "wrong at every cell" and
#              "zero of sixteen" in WORDS must NOT match, because the matcher is
#              digit-based and must be honest that it undercounts. The real
#              store is therefore AT LEAST what this reports, never at most.
#
# CONTRAST ARM. The same matcher over `proposal.says` + `proposal.because`,
# where a measured sentence is REQUIRED to carry a predicate. Printing one side
# alone says nothing: the point is not that retirements contain numbers, it is
# that the same kind of sentence is disciplined in one namespace and not in the
# neighbouring one.
#
# NOTE ON NUTSHELL, from `191`: `$0` is the interpreter, never the script, so
# walk up for `mockspace.toml` rather than using `dirname "$0"`.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
[ -f "$root/mockspace.toml" ] || { echo "run me from inside the repository" >&2; exit 2; }
reg="$root/mock/registry"

# A quantified measurement, digit-anchored on purpose. Six shapes, each seen in
# the corpus. Words-as-numbers are deliberately out; see NEG-WORD.
figure='[0-9]+ of [0-9,]+|[0-9]+(\.[0-9]+)? percent|[0-9]+,[0-9]{3}|[0-9]+ (triples|cells|instances|configurations|failures|vectors|cases|placements|members|rows|files|lines|arms|sites)|at [0-9]+(\.[0-9]+)? |[0-9]+ (million|billion)'

echo "### root:   $root"
echo "### matcher: digit-anchored quantified figures, six shapes"
echo "### NOTE: words-as-numbers are NOT matched, so every count is a LOWER BOUND"
echo

emit_rows() {  # $1 = file, $2 = namespace marker, $3.. = fields to concatenate
  awk -v ns="$2" -v fields="$3" '
    BEGIN { split(fields, F, ","); for (i in F) want[F[i]] = 1 }
    $0 ~ "^\\[\\[" ns "\\]\\]" { if (id != "") print id "\t" buf; id=""; buf="" ; next }
    /^id = "/ { if (id == "") { line=$0; sub(/^id = "/, "", line); sub(/"$/, "", line); id=line } ; next }
    {
      key=$0; sub(/ =.*$/, "", key)
      if (key in want) { v=$0; sub(/^[a-z_]+ = "/, "", v); sub(/"$/, "", v); buf = buf " " v }
    }
    END { if (id != "") print id "\t" buf }
  ' "$1"
}

count_hits() {  # stdin: id<TAB>text ; prints matching ids
  grep -E -- "$figure" | cut -f1
}

echo "======== ARM 1. retirement.why, the namespace with no region field ========"
tot_ret=$(grep -c '^\[\[retirement\]\]' "$reg/retirement.toml")
hits_ret=$(emit_rows "$reg/retirement.toml" retirement why | count_hits)
n_ret=$(printf '%s\n' "$hits_ret" | grep -c . || true)
echo "retirement rows total:            $tot_ret"
echo "carrying a quantified figure:     $n_ret"
echo
echo "-- the rows, so a reader can check any of them --"
printf '%s\n' "$hits_ret" | sed 's/^/   /'
echo

echo "======== ARM 2. proposal says+because, where a region is required ========"
tot_pro=$(cat "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml" | grep -c '^\[\[proposal\]\]')
hits_pro=$( { emit_rows "$reg/proposal.toml" proposal says,because; emit_rows "$reg/proposal-the-later-topics.toml" proposal says,because; } | count_hits)
n_pro=$(printf '%s\n' "$hits_pro" | grep -c . || true)
echo "proposal rows total:              $tot_pro"
echo "carrying a quantified figure:     $n_pro"
echo
echo "-- of those, how many carry a predicate --"
withpred=0
nopred=0
for id in $hits_pro; do
  if awk -v want="$id" '
      /^\[\[proposal\]\]/ { inrow=0 }
      $0 == "id = \"" want "\"" { inrow=1 }
      inrow && /^predicate = / { found=1 }
      END { exit(found?0:1) }' "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml" 2>/dev/null; then
    withpred=$((withpred+1))
  else
    nopred=$((nopred+1))
  fi
done
echo "   with a predicate:              $withpred"
echo "   without:                       $nopred"
echo

echo "======== CONTROLS ========"
if printf '%s\n' "$hits_ret" | grep -qx "dl_interior_wrapping_with_a_reserved_absorbing_top"; then
  echo "POS-REAL   PASS  the row 191 names is matched"
else
  echo "POS-REAL   FAIL  *** the matcher misses the known-positive row; every count above is void ***"
  exit 3
fi

plant=$(mktemp)
cat > "$plant" <<'EOF'
[[retirement]]
id = "pos_plant_has_a_figure"
claim = "A planted claim."
why = "Closed by a probe: the property fails at 12 of 16 cells and the replacement does not."
kind = "wrong"

[[retirement]]
id = "neg_plant_pure_argument"
claim = "A planted claim."
why = "Closed because the reasoning does not survive contact with the intent, and no instrument was ever built for it."
kind = "wrong"

[[retirement]]
id = "neg_word_numbers_as_words"
claim = "A planted claim."
why = "Closed by a probe: wrong at every cell, at zero of sixteen failures, on both signednesses."
kind = "wrong"
EOF
ph=$(emit_rows "$plant" retirement why | count_hits)
for want in pos_plant_has_a_figure; do
  if printf '%s\n' "$ph" | grep -qx "$want"; then echo "POS-PLANT  PASS  $want matched"
  else echo "POS-PLANT  FAIL  *** $want not matched ***"; rm -f "$plant"; exit 3; fi
done
for want in neg_plant_pure_argument neg_word_numbers_as_words; do
  if printf '%s\n' "$ph" | grep -qx "$want"; then echo "NEG        FAIL  *** $want matched and must not ***"; rm -f "$plant"; exit 3
  else echo "NEG        PASS  $want correctly not matched"; fi
done
rm -f "$plant"
echo
echo "### NEG-WORD passing is the honest half: 'zero of sixteen failures' is a"
echo "### real measurement this matcher cannot see, and it sits in the very row"
echo "### POS-REAL matched. So ARM 1's count understates its own subject."
