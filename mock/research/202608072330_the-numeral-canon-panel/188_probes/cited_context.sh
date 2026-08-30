#!/usr/bin/env bash
# For every citation in a registry file, print the owning row id, the field the
# citation sits in, and a window of the cited source around the cited line.
#
# The linter resolves that a line number exists. This prints what is on it, plus
# two lines either side, because a claim's support is usually a sentence and a
# sentence usually spans lines. Judging whether the window supports the row is
# the reader's job and no instrument does it.
#
# CASE THAT MUST FAIL: run with --control and every window is fetched from a
# line 400 past the cited one. If those windows read as supporting the rows just
# as well as the real ones do, the reader is not reading and the instrument is
# useless. The control also proves the fetch can move: a fetch that always
# returned the same bytes would show identical output under both modes.
set -uo pipefail
PANEL="$(cd "$(dirname "$0")/../.." && pwd)"
ROOT="$(cd "$PANEL/.." && pwd)"
OFF=0
if [ "${1:-}" = "--control" ]; then OFF=400; shift; fi
W=${W:-2}

for f in "$@"; do
  ns=$(basename "$f" .toml)
  id=""
  field=""
  while IFS= read -r ln; do
    case "$ln" in
      'id = '*) id=$(printf '%s' "$ln" | sed 's/^id = "//;s/"$//') ;;
    esac
    # remember the most recent field name seen, for citations inside arrays
    case "$ln" in
      [a-z_]*' = ['*|[a-z_]*' = "'*) field=$(printf '%s' "$ln" | sed 's/ =.*//') ;;
    esac
    case "$ln" in
      *panel::*)
        for ref in $(printf '%s\n' "$ln" | grep -oE 'panel::[A-Za-z0-9_.#-]+::[A-Za-z0-9_.#-]+(::[A-Za-z0-9_.#-]+)*'); do
          dir=$(printf '%s' "$ref" | awk -F'::' '{print $2}')
          file=$(printf '%s' "$ref" | awk -F'::' '{print $3}')
          loc=$(printf '%s' "$ref" | awk -F'::' '{print $4}')
          sub=$(printf '%s' "$ref" | awk -F'::' '{print $5}')
          if [ -n "$sub" ]; then base="$PANEL/$dir/$file"; loc="$sub"; else base="$PANEL/$dir"; fi
          path=$(find "$base" -maxdepth 1 -name "$file*" 2>/dev/null | head -1)
          [ -z "$path" ] && path=$(find "$base" -maxdepth 1 -name "$(printf '%s' "$loc" | sed 's/[0-9]*//')*" 2>/dev/null | head -1)
          case "$loc" in
            ''|*[!0-9]*)
              printf '\n%s::%s  [%s]\n  -> %s  anchor %s\n' "$ns" "$id" "$field" "$file" "$loc"
              continue ;;
          esac
          n=$((loc + OFF))
          lo=$((n - W)); [ "$lo" -lt 1 ] && lo=1
          hi=$((n + W))
          printf '\n%s::%s  [%s]\n  -> %s:%s%s\n' "$ns" "$id" "$field" "$file" "$loc" \
            "$( [ "$OFF" -ne 0 ] && printf '  (CONTROL: reading %s)' "$n" )"
          if [ -f "$path" ]; then
            sed -n "${lo},${hi}p" "$path" | sed "s/^/     | /" | cut -c1-140
          else
            printf '     | *** NO SUCH FILE: %s ***\n' "$base/$file"
          fi
        done ;;
    esac
  done < "$f"
done
