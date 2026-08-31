#!/usr/bin/env bash
# Every proposal whose `sentence_kind` says it ran something and whose
# `evidence` is empty, derived from the registry rather than read off anybody's
# table.
#
# This exists to be diffed against `192_probes/p4_coverage_map.tsv`. The map is
# a claim about which claims are uncovered, and a claim gets checked. Running
# the extraction independently is the check: if the two lists differ, one of
# them is describing a corpus that is not there any more.
#
# `RAN_SOMETHING` is `measured` and `enumeration`, copied from `shape.rs`, and
# it is the one thing here inherited rather than derived. A theorem owes its
# route and an argument claims no run, so neither is in scope.
#
# Control: `--control` plants a measured row with no evidence, which must
# appear, and a measured row with evidence, which must not.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

extra=""
if [ "${1:-}" = "--control" ]; then
  extra=$'\n[[proposal]]\nid = "PLANTED_UNCOVERED"\nsentence_kind = "measured"\n\n[[proposal]]\nid = "PLANTED_COVERED"\nsentence_kind = "measured"\nevidence = ["something"]\n'
fi

{ cat "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml"; printf '%s' "$extra"; } | awk '
  /^\[\[/ {
    if (id != "" && ran && !ev) print id
    id = ""; ran = 0; ev = 0; inblock = 0
    inrow = ($0 == "[[proposal]]"); next
  }
  !inrow { next }
  inblock { if ($0 ~ /^(\047\047\047|""")[ \t]*$/) inblock = 0; next }
  /^[a-z_]+ = (\047\047\047|""")[ \t]*$/ { inblock = 1; next }
  /^id = /            { gsub(/^id = "|"$/, ""); id = $0 }
  /^sentence_kind = / { gsub(/^sentence_kind = "|"$/, ""); if ($0 == "measured" || $0 == "enumeration") ran = 1 }
  /^evidence = /      { if ($0 !~ /^evidence = \[\][ \t]*$/) ev = 1 }
  END { if (id != "" && ran && !ev) print id }
' | sort
