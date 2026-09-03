#!/usr/bin/env bash
# Seat 257. The trait-declared coordinate count and the tiers carrying an
# admission obligation, at four refs: 244's own commit, the respelling, 247's
# HEAD, and origin/dev now.
#
# Seat 247 section 5 established this first, at its own HEAD. This is a fourth
# arrival by a different extractor and it is NOT an independent instance for
# anything 247 concluded: I read 247 before running it. It is run because 247's
# HEAD was 607ca52f and origin/dev has moved since, so the question "does it
# still hold" is not one 247 could answer.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)" || exit 1

REFS="cc19b122 748c6004 607ca52f origin/dev"
TRAITS="ambient quantum slots format"

printf '%-12s %-14s %s\n' "ref" "coordinates" "traits carrying ADMITTED"
for r in $REFS; do
    tot=0; adm=""
    for f in $TRAITS; do
        n=$(git show "$r:mock/crates/arvo-format/src/$f.rs" 2>/dev/null |
            awk '/^pub trait /{t=1} t && /^    const [A-Z_]+/ && !/ADMITTED/{c++} /^}/{t=0} END{print c+0}')
        tot=$((tot + n))
        if git show "$r:mock/crates/arvo-format/src/$f.rs" 2>/dev/null | grep -q "    const ADMITTED"; then
            adm="$adm $f"
        fi
    done
    printf '%-12s %-14s %s\n' "$r" "$tot" "${adm:- (none)}"
done

echo
echo "== CONTROL A: the extractor must move across refs, or it sees nothing =="
echo "   satisfied above iff the coordinate column is not constant."
echo
echo "== CONTROL B: the ADMITTED exclusion must work, or the count is inflated =="
printf '   slots.rs consts at origin/dev including ADMITTED : '
git show origin/dev:mock/crates/arvo-format/src/slots.rs |
    awk '/^pub trait /{t=1} t && /^    const [A-Z_]+/{c++} /^}/{t=0} END{print c+0}'
printf '   slots.rs consts at origin/dev excluding ADMITTED : '
git show origin/dev:mock/crates/arvo-format/src/slots.rs |
    awk '/^pub trait /{t=1} t && /^    const [A-Z_]+/ && !/ADMITTED/{c++} /^}/{t=0} END{print c+0}'
echo "   the two must differ by exactly one."
echo
echo "== CONTROL C: a ref known to predate the traits must count zero =="
printf '   the panel-directory root commit : '
git show "$(git rev-list --max-parents=0 HEAD | tail -1):mock/crates/arvo-format/src/format.rs" \
    2>/dev/null | wc -l | tr -d ' '
echo "   (zero lines means the path does not exist there, as required)"
echo
echo "== What the ratified row says the count is =="
grep -o "three of the ten associated constants" mock/registry/ruling.toml | head -1
echo "   ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon"
