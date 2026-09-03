#!/usr/bin/env bash
# D. Can a reasoned claim about the canon's own contents state a region, and
#    what does the shipped checker say to each candidate spelling?
#
# Every other arm here reads the corpus. This one asks the thing that decides:
# the lints. Six candidate rows in `d_plant.toml` are appended to
# `mock/registry/proposal.toml`, `cargo mock --lint-only` is run over the real
# registry with them in it, and the file is truncated back to its original
# length. Nothing is committed into the registry and the tree is checked clean
# at the end.
#
# The six, with the verdict each is predicted to draw, written before the run:
#
#   D1 argument, no region      -> an-established-claim-carries-no-region
#   D2 argument, structural axis-> axis-not-declared, twice
#   D3 argument, every declared axis at `any`, tokenless -> SILENT
#   D4 normative, with evidence -> an-imposition-rests-on-no-instrument
#   D5 normative, with a region -> an-imposed-proposition-carries-a-region
#   D6 control, ordinary numeric-> SILENT
#
# D3 is the arm the question turns on. If it is silent, the notation already
# admits a region for a structural claim and the region is the whole space on
# every declared axis; the question's premise is then about what is idiomatic
# rather than about what is expressible. If it is reported, the premise holds
# mechanically and the residue is a real gap.
#
# Controls, written before the run:
#   L1  the baseline must be clean. `cargo mock --lint-only` over the untouched
#       registry must report no finding from the three lints under test, or
#       every verdict below is contaminated by a pre-existing one.
#   L2  D6 must be silent. A planted row that is correctly formed and reported
#       anyway means the arm is measuring the lints and not the rows.
#   L3  at least one planted row must be REPORTED. A run in which nothing fires
#       is indistinguishable from a run in which the plant never reached the
#       file.
#   L4  the file must be back to its original length at the end, and
#       `git status --porcelain mock/registry` must be empty.
set -uo pipefail
cd "$(dirname "$0")"
ROOT=$(cd ../../../.. && pwd)
TARGET=$ROOT/mock/registry/proposal.toml
PLANT=$PWD/d_plant.toml

LINTS='a-region-agrees-with-the-sentence-kind|every-predicate-names-a-declared-axis|an-imposition-rests-on-no-instrument'

orig=$(wc -l < "$TARGET" | tr -d ' ')
echo "### the target before planting: $orig lines"

restore() {
  head -n "$orig" "$TARGET" > "$TARGET.tmp" && mv "$TARGET.tmp" "$TARGET" && touch "$TARGET"
}
trap restore EXIT

echo
echo "### L1, the baseline, before anything is planted"
( cd "$ROOT" && cargo mock --lint-only 2>&1 ) | grep -E "$LINTS" > l1_baseline.txt || true
n1=$(wc -l < l1_baseline.txt | tr -d ' ')
echo "  findings from the three lints under test on the untouched registry: $n1"
[ "$n1" = "0" ] && echo "  PASS, the baseline is clean" || { echo "  FAIL, baseline is dirty:"; cat l1_baseline.txt; }

echo
echo "### planting six candidate rows"
cat "$PLANT" >> "$TARGET"
planted=$(wc -l < "$TARGET" | tr -d ' ')
echo "  target is now $planted lines"

echo
echo "### the run"
( cd "$ROOT" && cargo mock --lint-only 2>&1 ) > d_run_full.txt || true
grep -E "$LINTS" d_run_full.txt > d_findings.txt || true
sed -e 's/^  //' d_findings.txt

echo
echo "### verdict per planted row"
verdict() {
  local id="$1" want="$2"
  local hits
  hits=$(grep -c "$id" d_findings.txt || true)
  if [ "$want" = "SILENT" ]; then
    if [ "$hits" = "0" ]; then printf '  %-46s SILENT   as predicted\n' "$id"
    else printf '  %-46s REPORTED against a prediction of SILENT\n' "$id"; fi
  else
    if grep -q "$id" d_findings.txt && grep "$id" d_findings.txt | grep -q "$want"; then
      printf '  %-46s REPORTED %s, as predicted\n' "$id" "$want"
    else
      printf '  %-46s did NOT draw %s (hits: %s)\n' "$id" "$want" "$hits"
    fi
  fi
}
verdict zz_probe_argument_with_no_region              an-established-claim-carries-no-region
verdict zz_probe_argument_with_a_structural_axis      axis-not-declared
verdict zz_probe_argument_with_universal_on_every_axis SILENT
verdict zz_probe_normative_carrying_its_instrument    an-imposition-rests-on-no-instrument
verdict zz_probe_normative_carrying_a_region          an-imposed-proposition-carries-a-region
verdict zz_probe_control_an_ordinary_numeric_row      SILENT

echo
echo "### L3, at least one planted row must be reported"
fired=$(grep -c 'zz_probe_' d_findings.txt || true)
[ "$fired" -gt 0 ] && echo "  PASS, $fired findings name a planted row" \
  || echo "  FAIL, nothing fired, so the plant may never have been read"

echo
echo "### restoring"
restore
trap - EXIT
back=$(wc -l < "$TARGET" | tr -d ' ')
echo "  target is $back lines, was $orig"
[ "$back" = "$orig" ] && echo "  L4a PASS, length restored" || echo "  L4a FAIL"
dirty=$( cd "$ROOT" && git status --porcelain mock/registry | wc -l | tr -d ' ' )
echo "  git reports $dirty modified path(s) under mock/registry"
[ "$dirty" = "0" ] && echo "  L4b PASS, the registry is untouched" || echo "  L4b FAIL, the registry is dirty"
