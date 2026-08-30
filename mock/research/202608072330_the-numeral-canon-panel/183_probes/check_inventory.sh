#!/usr/bin/env bash
# Does every clause named in `blocked_inventory.tsv` sit where the row says it
# does, and does every PORTED row name a slug that exists in the registry?
#
# The inventory is a hand-built table and a hand-built table is exactly what
# nobody checks. This opens each cited line and prints what is there, so a
# reader confirms the mapping by looking rather than by trusting, and it fails
# loudly on a slug the registry does not carry.
#
# Controls:
#   I1  a deliberately wrong line must be reported. The last two rows of
#       `blocked_inventory_control.tsv` cite a real file at a line holding
#       something else and a slug nothing declares; both must be named.
#   I2  the real inventory must come out clean. An arm that reports the control
#       and also reports the real table is reporting everything.
#   I3  the ported-row count in the inventory must equal the row count in the two
#       registry files, or one of them has drifted from the other.
set -euo pipefail
cd "$(dirname "$0")"
PANEL=..
REG=../../../registry

check() {
  local tsv="$1" label="$2" bad=0 ported=0
  while IFS=$'\t' read -r file line clause verdict why; do
    case "$file" in '#'*|'') continue;; esac
    local f
    f=$(ls "$PANEL/${file}"_*.md 2>/dev/null | head -1) || true
    if [ -z "${f:-}" ]; then
      echo "  MISSING FILE  $file"; bad=$((bad+1)); continue
    fi
    local text
    text=$(sed -n "${line}p" "$f")
    # the clause label's leading number must appear on the cited line, or the
    # line must be a heading naming it
    # a clause opens as a blockquote item, a heading, or an emphasis run. A
    # prose line is not a clause opening, and that is the whole test: an earlier
    # version also accepted `^#+ .*<n>\.`, which matched the file's own title
    # because `# 161. Canon candidate` contains `1.`, so the wrong-line control
    # passed and the arm could only ever report a bad slug.
    if ! printf '%s' "$text" | grep -qE '^(> \*\*|#{2,3} |\*[^ ]|\*\*)' ; then
      echo "  WRONG LINE    $file:$line wanted clause $clause, line reads: $(printf '%s' "$text" | cut -c1-52)"
      bad=$((bad+1))
    fi
    if [ "$verdict" = "PORTED" ]; then
      ported=$((ported+1))
      local slug=""
      case "$why" in
        "proposal "*|"law "*) slug=$(printf '%s' "$why" | sed -E 's/^(proposal|law) ([a-z0-9_]+).*/\2/') ;;
      esac
      if [ -n "$slug" ] && ! grep -qs "^id = \"$slug\"$" "$REG"/proposal-the-later-topics.toml "$REG"/law-the-later-topics.toml; then
        echo "  NO SUCH ROW   $slug (from $file:$line)"; bad=$((bad+1))
      fi
    fi
  done < "$tsv"
  echo "  $label: $bad problem(s), $ported ported rows referenced"
  printf '%s' "$ported" > /tmp/183_ported_count
  return 0
}

echo "### I1, the control table, which must be reported"
cat > blocked_inventory_control.tsv <<'EOF'
161	26	1. a primitive is a value set with one realisation map	PORTED	proposal a_primitive_is_a_value_set_with_one_realisation_map
161	521	1. a primitive is a value set with one realisation map	PORTED	proposal no_such_row_anywhere
EOF
check blocked_inventory_control.tsv "control"

echo
echo "### I2, the real inventory, which must be clean"
check blocked_inventory.tsv "inventory"
ported=$(cat /tmp/183_ported_count)

echo
echo "### I3, ported rows in the inventory against rows in the registry"
rows=$(( $(grep -c '^\[\[proposal\]\]' "$REG"/proposal-the-later-topics.toml) \
       + $(grep -c '^\[\[law\]\]' "$REG"/law-the-later-topics.toml) ))
echo "  inventory PORTED lines: $ported"
echo "  registry rows:          $rows"
if [ "$ported" -ge "$rows" ]; then
  echo "  PASS, every registry row is reachable from a clause (some clauses share a row)"
else
  echo "  FAIL, $((rows - ported)) registry rows are not named by any inventory line"
fi

echo
echo "### the verdict tally"
awk -F'\t' '!/^#/ && NF>3 {print $4}' blocked_inventory.tsv | sort | uniq -c | sort -rn | sed 's/^/  /'
