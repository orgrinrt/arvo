#!/usr/bin/env bash
# The counts this pass reports, computed rather than remembered.
#
# A number in an accounting paragraph that nobody ran is a belief about one's
# own document wearing a measurement's authority. Everything reported in `182`
# section 2 comes out of here.
#
# Run from the repository root.
set -uo pipefail
P=mock/registry/proposal.toml
L=mock/registry/law.toml

echo "=== rows per namespace, whole registry ==="
for f in mock/registry/*.toml; do
  n=$(basename "$f" .toml)
  printf '%-14s %s\n' "$n" "$(grep -c "^\[\[$n\]\]" "$f")"
done

echo
echo "=== proposal rows written by this pass ==="
grep -c '^\[\[proposal\]\]' "$P"

echo
echo "=== proposals by sentence_kind ==="
grep '^sentence_kind = ' "$P" | sort | uniq -c | sort -rn

echo
echo "=== proposals by standing ==="
grep '^standing = ' "$P" | sort | uniq -c | sort -rn

echo
echo "=== proposals by kind ==="
grep '^kind = ' "$P" | sort | uniq -c | sort -rn

echo
echo "=== proposals by topic ==="
grep '^topic = ' "$P" | sort | uniq -c | sort -rn

echo
echo "=== predicate carried, against not carried ==="
# A row's predicate block opens with `predicate = [`. Counting those against the
# row count gives the split directly, because a row has at most one.
withpred=$(grep -c '^predicate = \[' "$P")
rows=$(grep -c '^\[\[proposal\]\]' "$P")
echo "rows with a predicate:    $withpred"
echo "rows without a predicate: $((rows - withpred))"
echo "(every row without one is normative; the check enforces the biconditional,"
echo " so these two numbers are the normative and non-normative counts as well.)"
echo "normative rows: $(grep -c 'sentence_kind = "normative"' "$P")"

echo
echo "=== predicate entries, by axis, across proposal and law ==="
grep -hoE '^  "[a-z_]+: ' "$P" "$L" | tr -d ' "' | tr -d ':' | sort | uniq -c | sort -rn

echo
echo "=== declared axes never used by any row this pass wrote ==="
declared=$(grep '^id = ' mock/registry/dimension.toml | sed 's/id = "//; s/"//')
used=$(grep -hoE '^  "[a-z_]+: ' "$P" "$L" | tr -d ' "' | tr -d ':' | sort -u)
for d in $declared; do
  printf '%s\n' "$used" | grep -qx "$d" || echo "  $d"
done

echo
echo "=== law rows ==="
echo "rows:            $(grep -c '^\[\[law\]\]' "$L")"
echo "with holds:      $(grep -c '^holds = \[' "$L")"
echo "with fails:      $(grep -c '^fails = \[' "$L")"
echo "with a witness:  $(grep -c '^witness = ' "$L")"
echo "with a gap:      $(grep -c '^gap = ' "$L")"

echo
echo "=== measured rows with no evidence, which is the red ==="
( cd mock && cargo test -p arvo-checks --test what_one_field_obliges_another_to_carry 2>&1 \
  | grep -oE 'proposal::[a-z_]+' | sort -u )

echo
echo "=== citations, by shape ==="
echo "line citations into numbered panel files: $(grep -hoE 'panel::[A-Za-z0-9_.-]*::[A-Za-z0-9_.-]*::[0-9]+' "$P" "$L" | wc -l | tr -d ' ')"
echo "heading anchors into living ledgers:      $(grep -hoE 'panel::[A-Za-z0-9_.-]*::[A-Z]+::#[a-z0-9-]+' "$P" "$L" | wc -l | tr -d ' ')"
echo "distinct source files cited:              $(grep -hoE 'panel::[A-Za-z0-9_.-]*::[A-Za-z0-9_.-]*::' "$P" "$L" | cut -d: -f5 | sort -u | wc -l | tr -d ' ')"
grep -hoE 'panel::[A-Za-z0-9_.-]*::[A-Za-z0-9_.-]*::' "$P" "$L" | cut -d: -f5 | sort | uniq -c | sort -rn | sed 's/^/  /'
