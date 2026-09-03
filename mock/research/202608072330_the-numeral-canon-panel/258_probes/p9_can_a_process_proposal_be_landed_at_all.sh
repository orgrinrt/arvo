#!/usr/bin/env bash
# Seat 258. Can a proposal about the corpus be landed, and under which spelling?
#
# Section 5 of the file this sits beside says a `proposal` row for what this
# sitting argued is blocked, because `a-region-agrees-with-the-sentence-kind`
# refuses an `argument` row carrying no region and `dimension.toml` declares no
# axis a claim about the canon can be stated over. That was reasoned from a lint's
# doc comment and a census of the 127 committed rows rather than from a refusal,
# and the file said so and called it the weak part.
#
# This plants the rows and finds out. Four spellings of one claim, each landed
# alone and removed in the same run.
#
#   A  sentence_kind = "argument", no predicate
#   B  sentence_kind = "argument", predicate naming an axis nothing declares
#   C  sentence_kind = "argument", predicate naming only `threads` and `toolchain`
#   D  sentence_kind = "normative", no predicate
#
# THE CASES THAT MUST FAIL, run as the whole content of this probe:
#   C0  With nothing planted, neither the registry section nor the lint errors may
#       hold anything but the one pre-existing tool-lock error, or this probe is
#       reporting somebody else's failure as its own.
#   C1  A must be refused, or section 5's premise is simply false.
#   C2  B must be refused, or the declared-axis check is not doing what its name
#       says and the axis set is decorative.
#   C4  At least one of the four must pass and at least one must be refused, or
#       the instrument says the same thing about every input.
set -u
cd "$(dirname "$0")/../../../.." || exit 1 # the repository root
fail() { echo "CONTROL FAILED: $1"; exit 2; }

PLANT=mock/registry/zz_258_proposal_control.toml
ID=zz_258_proposal_control
cleanup() { rm -f "$PLANT"; }
trap cleanup EXIT

echo "tree: $(git rev-parse HEAD)"
echo

# Everything the engine says about the registry plus every lint error.
verdict_section() {
  cargo mock --lint-only --strict 2>&1 \
    | sed -n '/^--- registry ---$/,$p' \
    | grep -E 'ERROR|\[error\]|schema check|registry check|lint check'
}

BASE_ERR='the-tool-locks-disagree'

# --- C0 ------------------------------------------------------------------------
base=$(verdict_section)
echo "C0 with nothing planted:"
printf '%s\n' "$base" | cut -c1-140 | sed 's/^/    /'
n=$(printf '%s\n' "$base" | grep -c 'ERROR' || true)
[ "$n" -eq 0 ] || fail "C0: the registry already reports an error with nothing planted"
printf '%s\n' "$base" | grep -q "$BASE_ERR" || fail "C0: the known pre-existing lint error is missing, so the run is not what it was"
echo

plant() {
  {
    printf '# Seat 258'"'"'s proposal control. Removed by the probe that wrote it.\n'
    printf '[[proposal]]\n'
    printf 'id = "%s"\n' "$ID"
    printf 'kind = "finding"\n'
    printf 'sentence_kind = "%s"\n' "$1"
    printf 'standing = "one_expert"\n'
    printf 'topic = "panel_conduct"\n'
    printf 'says = "A control plant that exists for the length of one lint run and claims nothing."\n'
    printf 'because = "It is planted to find out which spellings of a proposal about the corpus the gates accept."\n'
    if [ -n "$2" ]; then printf 'predicate = [%s]\n' "$2"; fi
    printf 'provenance = ["panel::202608072330_the-numeral-canon-panel::258_probes::p9_can_a_process_proposal_be_landed_at_all.sh::1"]\n'
    printf 'keywords = ["control", "plant", "proposal", "predicate"]\n'
  } > "$PLANT"
}

pass=0
refuse=0
try() {
  local label=$1 kind=$2 pred=$3
  plant "$kind" "$pred"
  local out
  out=$(verdict_section)
  rm -f "$PLANT"
  if printf '%s\n' "$out" | grep -E 'ERROR|\[error\]' | grep -q "$ID"; then
    echo "$label : REFUSED"
    printf '%s\n' "$out" | grep -E 'ERROR|\[error\]' | grep "$ID" | cut -c1-220 | sed 's/^/    /'
    refuse=$((refuse+1))
  else
    echo "$label : accepted"
    printf '%s\n' "$out" | grep -E 'schema check|registry check|lint check' | cut -c1-140 | sed 's/^/    /'
    pass=$((pass+1))
  fi
}

try "A argument, no predicate                    " "argument"  ""
echo
try "B argument, an axis nothing declares        " "argument"  '"corpus_tier: any"'
echo
try "C argument, threads and toolchain only      " "argument"  '"threads: threads = 1", "toolchain: toolchain = nightly-2026-05-28"'
echo
try "D normative, no predicate                   " "normative" ""
echo

[ -f "$PLANT" ] && fail "the control plant was not removed"
[ "$pass" -gt 0 ] && [ "$refuse" -gt 0 ] || fail "C4: the instrument said the same thing about every spelling"
echo "accepted: $pass   refused: $refuse"
echo
echo "VERDICT is in which spellings landed, above. Read A and B for whether section 5's"
echo "premise holds, and C for whether the block is a gate or a matter of honesty."
