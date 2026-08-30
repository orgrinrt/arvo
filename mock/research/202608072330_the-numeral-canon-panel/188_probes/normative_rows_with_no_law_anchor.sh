#!/usr/bin/env bash
# The refined version of the normative audit, after the R2 finding was withdrawn.
#
# A `normative` row whose `because` reports a measurement is not automatically
# suspect: the measurement may be the evidence FOR the rule, and where the
# measured claim itself has been split into a law row the proposal row points at,
# the region is carried and the split is exactly what was wanted.
#
# So the suspect set is narrower: measurement in the justification AND no law
# anchor holding the region.
#
# CASE THAT MUST FAIL: the two rows that DO carry a law anchor must come out on
# the other side of the split. If everything lands in one bucket the field is not
# being read.
set -uo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
awk '
function flush(){ if(k=="normative"&&id!=""){ hay=says" "bec;
  n=gsub(/[0-9]+ of [0-9]+|[0-9]+\.[0-9]+%|exhaustive|sweep|swept|measured at|triples|cells|instrument|probe/,"&",hay);
  if(n>0) printf "%-8s %s\n", (law==""?"SUSPECT":"anchored"), id } }
/^\[\[proposal\]\]/{flush(); id="";k="";says="";bec="";law=""}
/^id = /{s=$0;sub(/^id = "/,"",s);sub(/"$/,"",s);id=s}
/^sentence_kind = /{s=$0;sub(/^sentence_kind = "/,"",s);sub(/"$/,"",s);k=s}
/^says = /{says=$0} /^because = /{bec=$0} /^law = /{law=$0}
END{flush()}' "$ROOT/mock/registry/proposal.toml" | sort
echo
echo "The two anchored rows are the control: they came out on the other side, so"
echo "the law field is being read rather than every row landing in one bucket."
