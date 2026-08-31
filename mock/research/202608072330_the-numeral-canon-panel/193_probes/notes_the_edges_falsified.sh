#!/usr/bin/env bash
# Rows whose `note` says `evidence` is empty, and now carries one.
#
# Several rows narrate the absence of the thing this pass added: "`evidence` is
# empty and the measured-implies-evidence check is red on this row". That was
# true when written. Adding the edge makes the sentence false, and a false
# sentence in a `note` is worse than a missing one, because a `note` is what a
# reader trusts when the fields disagree with their memory.
#
# `note` is out of scope for this dispatch by instruction, so this counts them
# rather than fixing them.
#
# Control: --control plants a row whose note says the same thing and has no
# evidence, which must NOT be reported, since that note is still true.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"
extra=""
[ "${1:-}" = "--control" ] && extra=$'\n[[proposal]]\nid = "PLANTED_STILL_TRUE"\nnote = "`evidence` is empty and the measured-implies-evidence check is red on this row."\n'

{ cat "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml"; printf '%s' "$extra"; } | awk '
  /^\[\[/ { if (id && stale && ev) print id; id=""; stale=0; ev=0; inrow = ($0 == "[[proposal]]"); next }
  !inrow { next }
  /^id = /       { gsub(/^id = "|"$/, ""); id = $0 }
  /^evidence = / { if ($0 !~ /^evidence = \[\]/) ev = 1 }
  /evidence` is empty|`evidence` is empty/ { stale = 1 }
  END { if (id && stale && ev) print id }
'
