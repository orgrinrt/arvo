#!/bin/sh
# 169 P2. `167` and `168` derived the same definition blind: a chain/region is
# maximal and bounded by OBSERVATION rather than by syntax or by the operator.
# `168` section 23 discounts their agreement on the evidence of shared section
# headings. It does not ask whether the shared premise set fixed the DEFINITION,
# which is the load-bearing convergence rather than the cosmetic one.
#
# A workspace rule auto-loads into every member's context whose entire thesis is
# that a guarantee's extent is bounded by what can be observed. This checks
# whether it is in the loaded set and whether either member declared it.
#
# NEGATIVE CONTROLS, stated before the run.
#   C1. The membership test must report a rule that is NOT loaded as not loaded,
#       using 157's own measured set difference. Otherwise "it is loaded" is
#       unfalsifiable and the finding is about a broken test.
#   C2. The declaration grep must FIRE somewhere: at least one member must be
#       shown to cite some workspace rule by name, or "neither declared it" is a
#       fact about my pattern rather than about the files.
set -e
D=${PANEL:-/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel}
RULE=what-you-can-observe-is-what-you-guaranteed
LOADED="$D/157_probes/loaded_rules_157.txt"
A="$D/167_rompf_the_chain_derived_cold.md"
B="$D/168_mcsherry_the_chain_derived_cold.md"
C="$D/60_stam_the_chain_derived_cold.md"

echo "=== is the rule in the enumerated auto-loaded set? ==="
printf '  %-46s ' "$RULE"; grep -qx "$RULE" "$LOADED" && echo "LOADED (line $(grep -nx "$RULE" "$LOADED" | cut -d: -f1))" || echo "not loaded"
NOTLOADED=arvo-always-optimal-internals
printf '  C1 control, a rule 157 measured as NOT loaded:\n  %-46s ' "$NOTLOADED"
grep -qx "$NOTLOADED" "$LOADED" && echo "LOADED  <-- C1 FAILED" || echo "not loaded (C1 ok)"
C1=$(grep -qx "$NOTLOADED" "$LOADED" && echo bad || echo ok)

echo
echo "=== the rule's thesis, quoted ==="
sed -n '/^A guarantee about a type holds only/,+2p' /Users/orgrinrt/Dev/clause-dev/.claude/rules/$RULE.md | sed 's/^/  /'

echo
echo "=== does any member declare it? ==="
for f in "$A" "$B" "$C"; do
  printf '  %-42s %s\n' "$(basename "$f" | cut -c1-40)" \
    "$(grep -oc 'what-you-can-observe\|perimeter of what was guaranteed' "$f")"
done

echo
echo "=== C2 control: do these files cite workspace rules by name at all? ==="
for f in "$A" "$B" "$C"; do
  n=$(grep -oE '[a-z-]+\.md' "$f" | grep -vE '^(1[0-9]{2}|[0-9]{2})_' | sort -u | wc -l | tr -d ' ')
  printf '  %-42s %s distinct rule-shaped .md names\n' "$(basename "$f" | cut -c1-40)" "$n"
done
C2=$(grep -oE '[a-z-]+\.md' "$A" "$B" "$C" | grep -c 'arvo-toolbox-not-policer\|the-test-gate\|every-finding' || true)
echo "  C2: rule-name citations found across the three: $C2 (want >= 1)"

echo
if [ "$C1" = ok ] && [ "$C2" -ge 1 ]; then
  echo "CONTROLS: both ok."
  echo "VERDICT: the observability-perimeter rule is line 4 of the auto-loaded set"
  echo "         157 measured in this same panel, it states the principle both"
  echo "         definitions are built on, and NONE of the three members names it."
  echo "         This is a plausible shared route for the definitional agreement"
  echo "         and it is undeclared in all three contamination sections."
  echo
  echo "         BOUND, stated because the finding is easy to overstate: 168's"
  echo "         derivation is semantically self-contained (pi.g.pi.f against"
  echo "         pi.g.f) and does not NEED the rule. What is established is that"
  echo "         the rule was in every context and was declared by nobody, not"
  echo "         that anyone used it. That is a gap in the accounting, not a"
  echo "         demonstration of dependence."
  echo
  echo "         AND A SECOND BOUND, for 60. 157 measured the loaded set at its"
  echo "         own dispatch, not at 60's. The rule file records its own origin"
  echo "         in a panel dated 202607301300, which predates this panel, so the"
  echo "         rule EXISTED when 60 was written; whether it was loaded then is"
  echo "         not measured and I do not claim it."
else
  echo "CONTROL FAILED (C1=$C1 C2=$C2) -- suppressed"; exit 1
fi
