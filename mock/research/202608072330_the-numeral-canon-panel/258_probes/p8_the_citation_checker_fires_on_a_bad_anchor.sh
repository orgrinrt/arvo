#!/usr/bin/env bash
# Seat 258. The rows this seat wrote pass the citation check, and the citation
# check is capable of failing.
#
# Twenty-one rows landed carrying provenance and `lives` citations into frozen
# panel files, most of them by line and one by a heading anchor into this seat's
# own file. `cargo mock --lint-only --strict` reports one error over the whole
# tree and it is the pre-existing tool-lock disagreement, so the citations pass.
#
# A pass means nothing until the checker has been made to fail. This plants three
# citations, one per failure mode, runs the check after each, and removes the
# plant.
#
# **The first version of this probe counted lint errors and got the answer
# backwards**, reporting the planted citations as NOT REPORTED because the count
# went from one to zero. The registry check runs before the lint pass and aborts
# it, so a planted bad citation removes the pre-existing lint error from the
# output rather than adding to it. Counting was the wrong operation and the
# instrument now reads the registry section by name. Kept in the header because
# a control that reports the opposite of the truth is the shape worth naming.
#
# THE CASES THAT MUST FAIL, run as the whole content of this probe:
#   C0  With nothing planted, the registry section must report no error, or the
#       tree was already failing and this probe would be hiding it.
#   C1  A citation into a panel file that does not exist must be reported, and the
#       report must name the planted row.
#   C2  A heading anchor that does not exist, in a file that does, must be
#       reported. This is the mode one of this seat's own citations uses.
#   C3  A citation naming a root and nothing in it must be reported.
set -u
cd "$(dirname "$0")/../../../.." || exit 1 # the repository root
fail() {
	echo "CONTROL FAILED: $1"
	exit 2
}

PLANT=mock/registry/zz_258_citation_control.toml
ID=zz_258_citation_control
cleanup() { rm -f "$PLANT"; }
trap cleanup EXIT

echo "tree: $(git rev-parse HEAD)"
echo

# The registry section of a strict lint run, which is where a citation is checked.
registry_section() {
	cargo mock --lint-only --strict 2>&1 | sed -n '/^--- registry ---$/,/^registry check\|^--- lint/p'
}

# --- C0 ------------------------------------------------------------------------
base=$(registry_section)
printf '%s\n' "$base" | sed 's/^/    /'
printf '%s\n' "$base" | grep -q 'ERROR' && fail "C0: the registry already reports an error with nothing planted"
echo "C0 registry clean with nothing planted : yes"
echo

plant() {
	cat >"$PLANT" <<EOF
# Seat 258's citation control. Removed by the probe that wrote it.
[[probe]]
id = "$ID"
establishes = "Nothing. A control plant that exists for the length of one lint run."
lives = ["$1"]
control = "This row is the control. It is planted to be refused and removed in the same run."
standing = "sound"
EOF
}

rc=0
check_plant() {
	local label=$1 cite=$2
	plant "$cite"
	local out
	out=$(registry_section)
	rm -f "$PLANT"
	if printf '%s\n' "$out" | grep -q "ERROR.*$ID"; then
		echo "$label : reported"
		printf '%s\n' "$out" | grep "ERROR" | cut -c1-200 | sed 's/^/    /'
	else
		echo "$label : NOT REPORTED"
		printf '%s\n' "$out" | sed 's/^/    /'
		rc=1
	fi
}

check_plant "C1 a panel file that does not exist  " \
	"panel::202608072330_the-numeral-canon-panel::999_no_such_seat_anywhere::1"
echo
check_plant "C2 a heading anchor that does not exist" \
	"panel::202608072330_the-numeral-canon-panel::258_orchard_what_the_admission_and_standing_sitting_settled::#no-such-heading-in-this-file"
echo
check_plant "C3 a root with nothing in it          " \
	"panel"
echo

[ -f "$PLANT" ] && fail "the control plant was not removed"
if [ "$rc" -ne 0 ]; then
	echo "VERDICT: at least one failure mode is not reported, so a clean run over this"
	echo "seat's citations does not establish that every one of them resolves. Which mode"
	echo "is unchecked is printed above and is the finding."
	exit 1
fi
echo "VERDICT: all three failure modes are reported and the plants are removed, so the"
echo "clean run over this seat's rows is a measurement rather than a silence."
