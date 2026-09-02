#!/usr/bin/env bash
# Every `panel::...::#anchor` in the catalogue names a heading that exists.
#
# The committed suite cannot do this for the closed formalization panel. Its two
# archive arms key on the literal `seed/`, which is the four-file archive inside
# the live panel, so a citation into the 203-file archive is invisible to them.
# See `archive_citation_gap.sh`. Until that is closed this script is the only
# thing checking the catalogue's own citations, which is exactly the situation
# the gap describes.
#
# CONTROL. A resolver that accepts everything passes trivially, so this plants a
# heading that does not exist and requires the matcher to reject it. If the
# planted anchor resolves, the matcher is not matching and the run is void.
set -uo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
research="$(cd "$panel/.." && pwd)"
cat="$panel/207_catalogue_op_material_in_the_closed_panel.toml"

# A heading slugged the way a markdown anchor is: lowercased, non-alphanumerics
# to hyphens, runs collapsed, ends trimmed.
slug() {
    printf '%s' "$1" |
        sed 's/^#\{1,6\} *//' |
        tr '[:upper:]' '[:lower:]' |
        sed 's/[^a-z0-9]\{1,\}/-/g; s/^-//; s/-$//'
}

resolves() {
    local dir="$1" file="$2" anchor="$3" line
    [ -f "$research/$dir/$file.md" ] || return 1
    while IFS= read -r line; do
        [ "$(slug "$line")" = "$anchor" ] && return 0
    done < <(grep '^#\{1,6\} ' "$research/$dir/$file.md")
    return 1
}

echo "=== catalogue citations ==="
bad=0
n=0
while IFS= read -r cite; do
    dir=$(echo "$cite" | cut -d: -f3)
    file=$(echo "$cite" | awk -F'::' '{print $3}')
    anchor=$(echo "$cite" | awk -F'::#' '{print $2}')
    dir=$(echo "$cite" | awk -F'::' '{print $2}')
    n=$((n + 1))
    if resolves "$dir" "$file" "$anchor"; then
        printf '  ok      %s\n' "$file#$anchor"
    else
        printf '  DANGLES %s\n' "$file#$anchor"
        bad=$((bad + 1))
    fi
done < <(grep -oE 'panel::[A-Za-z0-9_-]+::[A-Za-z0-9_]+::#[a-z0-9-]+' "$cat" | sort -u)

echo "checked: $n   dangling: $bad"

echo
echo "=== CONTROL: an anchor that does not exist must be rejected ==="
if resolves "202607301300_formalization-spec-panel" "OLD_143b_op_checkpoint_thirtysix" "this-heading-does-not-exist"; then
    echo "CONTROL FAILED: the matcher resolved a heading that is not there" >&2
    exit 1
fi
echo "  rejected, as it must be"

echo
echo "=== CONTROL: a real anchor in the same file must be accepted ==="
if resolves "202607301300_formalization-spec-panel" "OLD_143b_op_checkpoint_thirtysix" "the-ruling"; then
    echo "  accepted, so the matcher is not simply rejecting everything"
else
    echo "CONTROL FAILED: the matcher rejected a heading that is there" >&2
    exit 1
fi

[ "$bad" -eq 0 ] || exit 1
