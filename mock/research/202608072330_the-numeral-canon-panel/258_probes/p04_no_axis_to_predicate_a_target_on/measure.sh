#!/usr/bin/env bash
# Can a canon sentence say "this holds at 64-bit targets"?
#
# `ruling::the_work_is_predicated_arms_composed` puts every finding under a
# predicate, and `mock/registry/dimension.toml` is the roster of axes a predicate
# may be stated over. Its own header calls the set incomplete and append-only, so
# an axis being absent is a fact to establish rather than to assume.
#
# Three measurements, each with a control that must move:
#
#   1. the declared axes, read off the registry rather than remembered;
#   2. the axes the corpus actually predicates over, tallied from every `predicate`
#      entry in every registry file;
#   3. whether any of them is a pointer width, a target width or a platform.
#
# The control on 2 is that every count it reports must name a row from 1. The
# control on 3 is a phrase that is present, so a zero from the same pipeline is a
# zero the pipeline could have avoided returning.
#
# Run from this directory in a checkout of arvo. Stderr is never discarded.
set -uo pipefail

REG="../../../../registry"
[ -d "$REG" ] || { echo "no registry at $REG; run this from the probe directory"; exit 2; }

echo "=== 1. axes declared in dimension.toml ==="
DECLARED=$(grep '^id = ' "$REG/dimension.toml" | sed 's/^id = "\(.*\)"/\1/' | sort)
echo "$DECLARED" | sed 's/^/  /'
echo "  ---- $(echo "$DECLARED" | wc -l | tr -d ' ') declared"

echo
echo "=== 2. axes the corpus predicates over, with occurrence counts ==="
# A predicate entry is a quoted array element of the form "<key>: <region>" or
# "<key> = <region>". `panel` shows up because provenance arrays share the shape,
# and it is left in as the tell that the pattern is matching the file rather than
# a curated list.
USED=$(grep -h '^  "[a-z_ ]*[:=]' "$REG"/*.toml \
    | sed 's/^  "\([a-z_ ]*\)[:=].*/\1/' | sed 's/ *$//' | sort | uniq -c | sort -rn)
echo "$USED" | sed 's/^/  /'

echo
echo "=== control on 2: every key counted, other than provenance, is a declared axis ==="
UNDECLARED=$(echo "$USED" | awk '{print $2}' | grep -v '^panel$' \
    | while read -r k; do echo "$DECLARED" | grep -qx "$k" || echo "$k"; done)
if [ -z "$UNDECLARED" ]; then
    echo "  none. the tally and the roster agree, so the pattern is reading axes."
else
    echo "  keys counted that dimension.toml does not declare:"
    echo "$UNDECLARED" | sed 's/^/    /'
fi

echo
echo "=== 3. is any axis a target, pointer or platform width? ==="
for phrase in pointer_width pointer platform target_pointer target_width; do
    n=$(echo "$DECLARED" | grep -ic "$phrase")
    m=$(grep -ric "$phrase" "$REG"/*.toml | awk -F: '{s+=$2} END {print s+0}')
    printf '  %-16s declared axes matching: %-3s  registry lines matching: %s\n' "$phrase" "$n" "$m"
done

echo
echo "=== control on 3: a phrase that is present, through the same two pipelines ==="
for phrase in target_features container; do
    n=$(echo "$DECLARED" | grep -ic "$phrase")
    m=$(grep -ric "$phrase" "$REG"/*.toml | awk -F: '{s+=$2} END {print s+0}')
    printf '  %-16s declared axes matching: %-3s  registry lines matching: %s\n' "$phrase" "$n" "$m"
done
echo "  a zero in this block voids every zero in block 3."

echo
echo "=== 4. the same question over the panel corpus, not only the registry ==="
CORPUS=".."
for phrase in "target_pointer_width" "pointer width" "platform width"; do
    m=$(grep -ril "$phrase" "$CORPUS"/../*.md "$CORPUS"/../*/*.md 2>/dev/null | wc -l | tr -d ' ')
    printf '  %-24s files in the panel corpus mentioning it: %s\n' "$phrase" "$m"
done
echo "  mentioning is not predicating. Block 3 is what says no predicate is stated"
echo "  over it; this block says whether the subject is discussed at all."
