#!/usr/bin/env bash
# p5. Does the registry engine actually RESOLVE a citation into the closed panel,
# or does it silently skip a path it cannot reach?
#
# Two probe rows were added to `probe.toml` whose `lives` points into
# `202607301300_formalization-spec-panel`. The schema check passed. That is not
# evidence: a resolver that skips what it does not recognise passes exactly the
# same way, and the whole value of putting a citation in a `ref[]` field is that
# something checks it.
#
# Before this run, `retirement.toml` was the only namespace citing that archive,
# always at one anchor. So "archive refs resolve" had never been tested against
# a path that could be wrong.
#
# THE CASE THAT MUST FAIL, stated before the run:
#   A. A planted `lives` naming a file that is NOT in the archive must make the
#      registry check REPORT. If it passes, `lives` is decorative for archive
#      paths and both new rows are citing nothing.
#   B. With the plant removed, the check must pass again. If it still reports,
#      the failure in A was something else and A proved nothing.
#
# The file is restored from a copy taken here, not from git, so a failure in the
# middle of the run cannot leave a mutation staged.

set -uo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
target="$repo/mock/registry/probe.toml"
backup="$(mktemp)"

cp "$target" "$backup"
restore() { cp "$backup" "$target"; rm -f "$backup"; }
trap restore EXIT

run_check() {
    ( cd "$repo" && cargo mock 2>&1 ) | grep -E 'schema check|rows across|unresolv|malformed|refus|error' | head -6
}

echo "== p5. does an archive citation actually resolve? =="
echo "target: mock/registry/probe.toml"
echo

echo "-- baseline, with the two new rows as committed --"
run_check
echo

echo "-- control A: plant a lives naming a file that is not in the archive --"
# Point one real citation at a filename that does not exist, leaving the rest of
# the row untouched, so anything reported is about the path and not the shape.
sed -i '' \
  's#panel::202607301300_formalization-spec-panel::OLD_76_probes::b1_structural_array.rs::1#panel::202607301300_formalization-spec-panel::OLD_76_probes::b1_THIS_FILE_IS_NOT_THERE.rs::1#' \
  "$target"
if grep -q 'b1_THIS_FILE_IS_NOT_THERE' "$target"; then
    echo "plant is in place."
else
    echo "CONTROL A FAILED: the plant did not apply, so nothing below is a test."; exit 1
fi
out_a=$(run_check)
echo "$out_a"
if echo "$out_a" | grep -qiE 'unresolv|malformed|error|refus'; then
    echo "  -> control A ok: the engine reports the missing archive file."
    a_ok=1
else
    echo "  -> CONTROL A FAILED: a citation naming nothing passed the check."
    echo "     `lives` is not resolved for archive paths, and both new rows cite"
    echo "     something nobody verifies."
    a_ok=0
fi
echo

echo "-- control B: restore and re-run --"
restore
cp "$target" "$backup"   # keep the trap's restore valid
out_b=$(run_check)
echo "$out_b"
if echo "$out_b" | grep -q 'schema check passed'; then
    echo "  -> control B ok: clean again, so control A's report was the plant."
    b_ok=1
else
    echo "  -> CONTROL B FAILED: still reporting with the plant gone."
    b_ok=0
fi
echo

if [ "${a_ok:-0}" -eq 1 ] && [ "${b_ok:-0}" -eq 1 ]; then
    echo "RESULT: archive citations are resolved, not skipped. A registry row can"
    echo "carry evidence out of the closed panel and the engine will hold it to"
    echo "the same standard as a citation into this one."
else
    echo "RESULT: the mechanism is NOT established. Do not rely on the two new rows'"
    echo "citations until this passes."
fi
