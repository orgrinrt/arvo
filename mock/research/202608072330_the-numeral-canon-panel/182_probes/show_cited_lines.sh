#!/usr/bin/env bash
# Prints the line each registry citation points at, so a reader can see whether
# the row's claim is actually there. The linter resolves that a line exists; it
# cannot resolve that the line says anything relevant, and that gap is where a
# mis-citation lives.
#
# Run from the repository root. Takes registry file paths as arguments.
set -uo pipefail
PANEL=mock/research
for f in "$@"; do
  grep -o 'panel::[A-Za-z0-9_.-]*::[A-Za-z0-9_.-]*::[0-9]\{1,\}' "$f" | sort -u | while IFS= read -r ref; do
    dir=$(printf '%s' "$ref" | cut -d: -f3)
    file=$(printf '%s' "$ref" | cut -d: -f5)
    line=$(printf '%s' "$ref" | cut -d: -f7)
    path=$(find "$PANEL/$dir" -maxdepth 1 -name "$file*" | head -1)
    printf '%-58s %s\n' "$file:$line" "$(sed -n "${line}p" "$path" | cut -c1-110)"
  done
done
