#!/usr/bin/env bash
# Emits the heading anchor slugs mockspace's ref checker accepts, for one markdown file.
# The rule was established by control run rather than read off a spec: every run of
# non-alphanumeric characters collapses to one hyphen, then leading and trailing hyphens go.
# Note this is NOT "punctuation dropped": an apostrophe becomes a hyphen, so
# "the derivation's outputs" is "the-derivation-s-outputs".
set -euo pipefail
grep -h '^#\{1,6\} ' "$1" | sed 's/^#* //' | while IFS= read -r h; do
  printf '%s\n' "$h" | tr 'A-Z' 'a-z' | sed 's/[^a-z0-9]\{1,\}/-/g; s/^-//; s/-$//'
done
