#!/usr/bin/env bash
# The defect that killed attempt one of `b_three_or_more_floor.sh`, isolated.
#
# Attempt one read the row list out of `cargo mock query
# 'proposal.where(standing=three_or_more).select(id)'`. The table renderer
# truncates a long cell with an ellipsis, so a filter demanding a well-formed
# slug saw one row of six and printed nothing else. Nothing failed; the arm just
# measured a sixth of its population.
#
# Attempt one's own output is not kept as a file, because it was a terminal run
# rather than a redirect. This arm reproduces the mechanism instead, which is
# what the claim rests on.
#
# Control: the same rows read out of the TOML must be longer than what the
# renderer prints for at least one of them, or there is no truncation and this
# file is describing something that does not happen.
set -uo pipefail
cd "$(dirname "$0")"
REG=../../../registry

echo "### what the renderer prints"
cargo mock query 'proposal.where(standing=three_or_more).select(id)' 2>/dev/null | tail -n +2

echo
echo "### the same six ids, out of the TOML"
awk '
  /^\[\[proposal(-the-later-topics)?\]\]/ { id=""; st=""; next }
  /^id = /       { id=$0; sub(/^id = "/,"",id); sub(/"$/,"",id); next }
  /^standing = / { if ($0 ~ /"three_or_more"/) print id }
' "$REG"/proposal.toml "$REG"/proposal-the-later-topics.toml

echo
echo "### control: at least one printed cell must be shorter than its real id"
short=$(cargo mock query 'proposal.where(standing=three_or_more).select(id)' 2>/dev/null \
  | grep -c '…' || true)
echo "  rendered cells carrying an ellipsis: $short"
[ "$short" -gt 0 ] && echo "  PASS, the renderer truncates" \
  || echo "  FAIL, no truncation, so attempt one failed for some other reason"
