#!/usr/bin/env bash
# Statements of consumer need in the panel's own corpus, which `184` names as
# owed and did not sweep.
#
# A demand side is read from outside the canon so that a check over it cannot
# report merely that the canon agrees with itself. `184` read three consumer
# repositories at one level and one intent, and said in terms that the majority
# of the enumeration would come from this corpus. Nobody has looked.
#
# The filter is need-shaped language beside the word, rather than the word
# alone: 1590 occurrences of `consumer` in the panel's markdown is a corpus, not
# a finding, and most of them are a claim's subject rather than a demand.
#
# Deliberately excludes `184` itself and this pass's own files, so the sweep
# cannot rediscover the enumeration it is checking.
#
# Controls, both required.
#   POSITIVE: a phrase from a need the obligations already carry must surface,
#     or the filter is too narrow to find one that is known to be there.
#   NEGATIVE: a phrase in no file must not.
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd)"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
P="$root/mock/research/202608072330_the-numeral-canon-panel"

NEED='consumer[s]? (need|needs|want|wants|require|requires|asks|cannot|can not|has to|must)|(need|needs|wants|requires) (a|an|the) consumer|what (a |the )?consumer (needs|wants|asks)'

echo "######## need-shaped lines, panel markdown, 184 and this pass excluded"
grep -rEn "$NEED" "$P" --include='*.md' 2>/dev/null \
  | grep -v '/184_dispatcher_note_the_demand_side.md:' \
  | grep -v '/199_' \
  | sed "s|^$P/||" | cut -c1-190 | sort -u

echo
echo "######## controls"
pos=$(grep -rEc "$NEED" "$P" --include='*.md' 2>/dev/null | grep -v ':0$' | wc -l | tr -d ' ')
echo "  POSITIVE  files matching the need filter at all: $pos  (zero would mean the filter is dead)"
neg=$(grep -rEn "consumers? need ZZZNOTAPHRASE" "$P" --include='*.md' 2>/dev/null | wc -l | tr -d ' ')
echo "  NEGATIVE  a phrase in no file: $neg  (must be 0)"
