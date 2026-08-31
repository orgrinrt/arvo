#!/usr/bin/env bash
# Two archives were prefixed `OLD_`. One checker watches one of them.
#
# `no_living_ledger_cites_the_archive_by_its_dead_name` and its sibling
# `every_archive_citation_in_the_panel_names_a_file_that_is_there` both key on the
# literal string `seed/`, which is the four-file archive inside the live panel.
# The closed formalization panel is a second archive, 203 files, prefixed by the
# same commit, at a different address. Nothing watches it.
#
# CONTROL, and it is a mutation rather than an argument. A checker that reports
# nothing looks identical whether it is working over a clean corpus or blind. So
# this plants an unprefixed `seed/` citation in a living ledger, runs the arm, and
# requires it to FAIL. If it passes, the checker is not measuring what its name
# says and every number below is void. The ledger is copied aside first and
# restored from the copy, so no git operation touches the working tree.
set -uo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
mockdir="$(cd "$panel/../.." && pwd)"
dead="$mockdir/research/202607301300_formalization-spec-panel"

echo "=== 1. unprefixed citations into the closed panel, which no checker reads ==="
grep -rnoE 'formalization-spec-panel/[A-Za-z0-9_]+\.(md|rs)' \
    "$panel" "$mockdir/registry" 2>/dev/null |
    grep -v '/OLD_' | sed "s|$mockdir/||" | sort | tee "$here/dangling.txt"
n=$(wc -l <"$here/dangling.txt" | tr -d ' ')
echo "count: $n"

echo
echo "=== 2. each one resolves to nothing, and the OLD_ form exists ==="
sed 's/.*formalization-spec-panel\///' "$here/dangling.txt" | sort -u | while read -r f; do
    printf '  %-46s bare:%-3s OLD_:%s\n' "$f" \
        "$(test -e "$dead/$f" && echo yes || echo NO)" \
        "$(test -e "$dead/OLD_$f" && echo yes || echo no)"
done

echo
echo "=== 3. why the checker cannot see them: it keys on the literal seed/ ==="
grep -n 'seed/' "$mockdir/checks/src/corpus.rs" | head -4
echo "citations above containing the string 'seed/': $(grep -c 'seed/' "$here/dangling.txt")"

echo
echo "=== 4. CONTROL: plant the covered form and require the arm to fail ==="
ledger="$panel/AGREEMENTS.md"
if [ ! -f "$ledger" ]; then
    echo "CONTROL FAILED: no AGREEMENTS.md to plant in" >&2
    exit 1
fi
cp "$ledger" "$here/.agreements.bak"
printf '\nplanted by 207_probes control: see `seed/SETTLED_container.md` for the prior reading.\n' >>"$ledger"
out=$(cd "$mockdir" && cargo test -p arvo-checks --test the_archive_citations_resolve \
    no_living_ledger_cites_the_archive_by_its_dead_name 2>&1)
cp "$here/.agreements.bak" "$ledger"
rm -f "$here/.agreements.bak"

if echo "$out" | grep -q 'test result: FAILED'; then
    echo "  control fired: the arm catches an unprefixed citation into the archive it covers"
else
    echo "CONTROL FAILED: the arm stayed green on a planted violation of the class it names" >&2
    echo "$out" | tail -20 >&2
    exit 1
fi
echo "  ledger restored: $(cmp -s "$ledger" "$ledger" && echo ok)"
