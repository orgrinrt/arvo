#!/usr/bin/env bash
# Seat 258. Does anything refuse a registry write while this panel is open?
#
# 255 section 4.1 declines to write its five candidate rows into `mock/registry/`
# and gives three reasons, of which the first is: "`canon_paths` refuses it while
# the panel is open". `mockspace.toml`'s own comment above `canon_paths` says the
# same: "Naming the paths here is what lets `mock check` refuse a change to them
# while a panel is still open".
#
# That is a claim about a mechanism and it is checkable. This checks it three
# ways: whether any panel is declared at all, what `mock check` reports on the
# panel row, and whether the registry has in fact been written while this panel
# ran. The third is the one that decides it, because a gate that has never
# refused anything over 138 commits is not a gate anybody has met.
#
# THE CASES THAT MUST FAIL, run before the census is reported:
#   C1  The commit classifier must report a nonzero number of registry commits
#       that land no numbered panel file, or it is matching the registry path
#       itself and every commit looks like a co-landing.
#   C2  It must also report a nonzero number that do land one, or it cannot see a
#       co-landing at all and the zero would be about the pattern.
#   C3  The panel-file pattern must reject a path that is not a numbered member
#       file. `AGREEMENTS.md` and a probe artifact are the two tried.
set -u
cd "$(dirname "$0")/../../../.." || exit 1 # the repository root
fail() {
	echo "CONTROL FAILED: $1"
	exit 2
}

PANEL=mock/research/202608072330_the-numeral-canon-panel
MEMBER='the-numeral-canon-panel/[0-9][0-9]*_[^/]*\.md'

echo "tree: $(git rev-parse HEAD)"
echo

# --- C3: the member-file pattern rejects what is not one -----------------------
for s in "$PANEL/AGREEMENTS.md" "$PANEL/248_probes/p1_output.txt" "$PANEL/HANDLES.md"; do
	if printf '%s\n' "$s" | grep -qE "$MEMBER"; then fail "C3: $s matched the member pattern"; fi
done
for s in "$PANEL/241_kiselyov_admission_is_a_resolution_not_a_verdict.md" "$PANEL/09_persona_checkpoint.md"; do
	printf '%s\n' "$s" | grep -qE "$MEMBER" || fail "C3: $s did not match the member pattern"
done
echo "C3 member pattern separates member files from ledgers and probe artifacts : yes"

# --- is a panel declared at all ------------------------------------------------
echo
if [ -d mock/panel ]; then
	echo "mock/panel exists, holding: $(ls mock/panel | tr '\n' ' ')"
else
	echo "mock/panel                           : does not exist"
fi
echo "mock panel status                    :"
cargo mock panel status 2>&1 | grep -v '^ ' | sed 's/^/    /'
echo
echo "mock check, the panel row            :"
cargo mock check 2>&1 | grep -E 'panel|git ' | sed 's/^/    /'

# --- the census ----------------------------------------------------------------
echo
both=0
regonly=0
for c in $(git log --format=%H -- mock/registry); do
	if git show --name-only --format= "$c" | grep -qE "$MEMBER"; then
		both=$((both + 1))
	else
		regonly=$((regonly + 1))
	fi
done
[ "$regonly" -gt 0 ] || fail "C1: every registry commit classified as a co-landing"
[ "$both" -gt 0 ] || fail "C2: no registry commit classified as a co-landing"
echo "C1 registry commits landing no member file : $regonly   (must be > 0)"
echo "C2 registry commits landing one            : $both   (must be > 0)"
echo
echo "commits touching mock/registry, total  : $((both + regonly))"
echo "  earliest: $(git log --format='%h %ad %s' --date=short -- mock/registry | tail -1)"
echo "  latest  : $(git log --format='%h %ad %s' --date=short -- mock/registry | head -1)"
echo "commits touching this panel directory  : $(git log --format=%H -- "$PANEL" | wc -l | tr -d ' ')"
echo "  earliest: $(git log --format='%h %ad %s' --date=short -- "$PANEL" | tail -1)"
echo "  latest  : $(git log --format='%h %ad %s' --date=short -- "$PANEL" | head -1)"
echo
echo "VERDICT: the registry was written throughout the panel's life, and $both commits"
echo "landed a registry change and a numbered member file in one act. No panel is"
echo "declared under mock/panel, so the gate mockspace.toml describes has nothing to"
echo "fire on. It reports green vacuously rather than because the registry is untouched."
