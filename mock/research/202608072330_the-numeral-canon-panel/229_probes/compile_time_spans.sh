#!/usr/bin/env bash
# Does a compile-time finding in the committed registry have a region to write?
#
# The open item says the notation has no region for a result that holds at
# compile time only, so such a finding is written in a form that, under the
# absence rule, says it holds nowhere. That is checkable rather than arguable:
# find the rows whose claim is about const evaluation or the compiler's staging,
# and print what their predicate actually says.
#
# Controls, written before the run:
#   T1  the search must find rows, or the corpus has no compile-time claims and
#       the open item is about a population of zero.
#   T2  a phrase nobody wrote must find none (`phase_of_the_moon`).
#   T3  the predicate field must be recovered for at least one hit, or the
#       extractor is reporting absence it cannot see.
set -euo pipefail
cd "$(dirname "$0")"
REG=../../../registry

pat='const time|const-time|const eval|const evaluation|compile time|compile-time|const fn|staging'

echo "### T2 control"
grep -c 'phase_of_the_moon' $REG/*.toml | grep -v ':0' && echo "  FAIL" || echo "  PASS, no file mentions it"
echo

echo "### rows whose says/note mentions const or compile-time staging, with their predicate"
awk -v pat="$pat" '
  /^\[\[/ { flush(); id=""; says=""; pred=""; holds=""; ns=$0; next }
  /^id = / { id=$0; sub(/^id = "/,"",id); sub(/"$/,"",id) }
  /^says = / { says=$0 }
  /^predicate = / { pred=$0 }
  /^holds = / { holds=$0 }
  { buf = buf $0 "\n" }
  END { flush() }
  function flush(  hit) {
    if (id == "") { buf=""; return }
    hit = (tolower(says) ~ tolower(pat))
    if (hit) {
      printf "--- %s  [%s]\n", id, ns
      printf "    predicate: %s\n", (pred=="" ? "(none)" : substr(pred,1,300))
      printf "    holds:     %s\n", (holds=="" ? "(none)" : substr(holds,1,300))
    }
    buf=""
  }
' $REG/law.toml $REG/proposal.toml $REG/ruling.toml $REG/law-the-later-topics.toml $REG/proposal-the-later-topics.toml
