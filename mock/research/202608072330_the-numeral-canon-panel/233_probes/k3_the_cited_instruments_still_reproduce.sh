#!/usr/bin/env bash
# k3. Do the four instruments I read the mode implementations out of still
# produce the output committed beside them?
#
# Seat 233 takes every mode from a probe's expression rather than its label, so
# every answer rests on a source file. A source nobody reran is weaker evidence
# than an output somebody reproduced: if the committed `.out` came from a
# different revision of the `.rs` or `.py` beside it, the label I rejected and
# the expression I trusted could both be wrong about what actually ran.
#
# `94_probes/c_retraction` and `149_probes/y2` are already pinned, by k2's
# controls R1, R2 and R3, which reproduce their published numbers from an
# independently written implementation. These four were not, and this closes it.
#
# Run from this directory. Exits nonzero if any instrument fails to reproduce.
set -uo pipefail
panel="$(cd "$(dirname "$0")/.." && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
fail=0

# p2 is run once per width and the committed file concatenates the three under
# `### W=n` headings, so the runner has to rebuild that shape to compare at all.
{ for w in 4 5 6; do echo "### W=$w"; python3 "$panel/97_probes/p2_congruence_predicts_the_laws.py" "$w"; done; } > "$work/p2.txt" 2>&1
if diff -q <(grep -v '^$' "$panel/97_probes/p2_congruence_predicts_the_laws.out") <(grep -v '^$' "$work/p2.txt") > /dev/null; then
    echo "p2_congruence_predicts_the_laws.py    REPRODUCES (blank-line-insensitive; the committed file carries two extra separators)"
else
    echo "p2_congruence_predicts_the_laws.py    DIFFERS"; fail=1
fi

run_rs() {
    local src="$1" out="$2" tag="$3"
    if ! rustc --edition 2021 -O -o "$work/$tag" "$panel/$src" 2>/dev/null; then
        rustc --edition 2024 -O -o "$work/$tag" "$panel/$src" 2>&1 | head -3
    fi
    "$work/$tag" > "$work/$tag.txt" 2>&1
    if diff -q "$panel/$out" "$work/$tag.txt" > /dev/null; then
        echo "$(basename "$src")    REPRODUCES EXACTLY"
    else
        echo "$(basename "$src")    DIFFERS"; diff "$panel/$out" "$work/$tag.txt" | head -8; fail=1
    fi
}

run_rs 60_probes/p_d_rescale_saving_is_adaptation_fusion.rs 60_probes/p_d.out pd
run_rs 62_probes/p4_signed_multiplicative_accumulator.rs     62_probes/p4_output.txt p4
run_rs 56_probes/q2_affine_membership.rs                     56_probes/q2_output.txt q2

# Control. The comparison must be able to fail, or four PASS lines mean nothing.
# Compare one instrument against another's output and require a difference.
if diff -q "$panel/60_probes/p_d.out" "$panel/62_probes/p4_output.txt" > /dev/null; then
    echo "CONTROL FAILED: two different instruments' outputs compare equal, so diff is not comparing"
    fail=1
else
    echo "control: a mismatched pair does diff as different, so the four passes above are real"
fi

exit $fail
