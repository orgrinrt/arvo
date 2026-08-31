#!/usr/bin/env bash
# p4: what `arvo_checks::load` does when the directory is not there.
#
# Found while pointing the spike at the registry from a nested manifest, which
# produced a clean run over zero rows. It matters beyond the spike: `canon()`
# derives the registry path from CARGO_MANIFEST_DIR, and its own doc comment
# says several worktrees of this repository are built at once, which is the
# arrangement where that path can resolve somewhere else.
#
# This probe reads the two sites. The behavioural half is measured rather than
# read, in `warrant_spike` section 5, which runs eight shipped arms over an
# empty registry and over the real one.
#
# Run from `mock/`.

set -uo pipefail
[ -d checks/src ] || { echo "run me from mock/" >&2; exit 2; }

echo "### the guard in walk()"
grep -n "is_dir" -A 2 checks/src/lib.rs | sed 's/^/  /'

echo
echo "### the expect() on canon()"
grep -n "mock/registry is readable" -B 3 checks/src/lib.rs | sed 's/^/  /'

echo
echo "### what that composes to"
echo "  walk() returns Ok(()) for a path that is not a directory, so load() returns"
echo "  Ok(empty) and the expect() never fires. A Registry with no rows makes every"
echo "  arm return an empty Vec, and every arm asserted empty passes."

echo
echo "### the shape of assertion that would still catch it"
echo "  a test asserting a finder NON-empty. Two exist:"
grep -rn "is what says the green means anything\|the canon declares at least one axis\|still_finds_the_stuck_rows" checks/tests/ | sed 's/^/    /'

echo
echo "### CONTROL. The reader must find a string known present and miss one known absent."
p=$(grep -c "is_dir" checks/src/lib.rs || true)
a=$(grep -c "zzqq_not_in_this_file" checks/src/lib.rs || true)
echo "  present(is_dir)=$p  absent(zzqq_not_in_this_file)=$a"
[ "$p" != "0" ] || { echo "  CONTROL FAILED: the reader cannot see a string that is there"; exit 1; }
[ "$a" = "0" ] || { echo "  CONTROL FAILED: the reader sees a string that is not there"; exit 1; }
echo "  controls held, so the two greps above mean what they print."
