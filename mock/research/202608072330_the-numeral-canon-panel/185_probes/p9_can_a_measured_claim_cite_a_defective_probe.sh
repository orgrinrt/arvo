#!/usr/bin/env nutshell
# Now that the namespace has rows, does the gate check anything about them?
#
# The `measured` gate refused every measured proposal while `probe` was empty.
# That refusal was structural: it fired because the reference resolved to
# nothing. With rows present it resolves, and the question is what remains
# checked. Three things a reader would assume are checked, and each is planted:
#
#   A  a `measured` proposal citing a probe whose `standing` is `defective`
#   B  a `measured` proposal citing a probe whose `standing` is `withdrawn`
#   C  a `measured` proposal citing a probe whose `control` field says no control
#      was run
#
# Required outcomes, and I am recording a PREDICTION rather than a requirement
# because I do not know the answer: I expect all three to pass silently, because
# nothing in `mock/checks` reads a probe's `standing` and the engine's job stops
# at resolving the slug. If any of the three is reported, the prediction is wrong
# and that is the better outcome.
#
# RUN ONE ESTABLISHED NOTHING. It omitted `because` and `topic`, both required,
# and put a capital in arm D's id, so all four arms failed on the schema before
# the evidence was looked at and the control arm was indistinguishable from the
# three it was supposed to control. Kept as
# `p9_run1_schema_failures_before_the_evidence.out`.
#
#   D  the control. A `measured` proposal citing a probe row that does not exist
#      MUST still be reported. If it is not, the gate has stopped working
#      entirely and A, B and C say nothing.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../.." && pwd)"
plant="$root/mock/registry/zzz_planted_measured_control.toml"
out="$here/p9_can_a_measured_claim_cite_a_defective_probe.out"

cleanup() { rm -f "$plant"; }
trap cleanup EXIT

run_arm() {
  local name="$1" ev="$2" note="$3"
  cat > "$plant" <<TOML
[[proposal]]
id = "planted_${name}"
says = "a planted claim that exists only to see what the gate does with its evidence."
because = "it does not; this row is the instrument."
topic = "algebraic_laws"
kind = "answer"
sentence_kind = "measured"
standing = "one_expert"
predicate = ["threads: 1"]
evidence = ["${ev}"]
provenance = ["panel::202608072330_the-numeral-canon-panel::185_probes::p9_can_a_measured_claim_cite_a_defective_probe.sh::1"]
keywords = ["planted", "control", "gate"]
TOML
  printf '=== arm %s\n  evidence = %s\n  %s\n' "$name" "$ev" "$note"
  local lint
  lint=$( cd "$root" && cargo mock --lint-only 2>&1 | grep -E 'ERROR|rows across' | grep -v 'unknown-config-key' || true )
  printf '%s\n' "${lint:-  (lint silent)}" | sed 's/^/  /'
  local checks
  checks=$( cd "$root/mock" && cargo test -p arvo-checks 2>&1 | grep -E 'FAILED|test result: FAILED' || true )
  printf '  arvo-checks: %s\n\n' "${checks:-all green}"
}

{
  printf 'p9: what the measured gate checks once probe has rows\n'
  printf 'date: %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  run_arm a_cites_a_defective_probe "the_width_invariance_control_was_toothless" \
    'predicted: passes silently. The row says its zero is unearned and unusable.'

  run_arm b_cites_a_withdrawn_probe "the_pareto_arm_that_rests_on_one_size_point" \
    'predicted: passes silently. The row says its author withdrew the finding.'

  run_arm c_cites_a_probe_with_no_control "the_debug_release_gap_that_retired_a_true_finding" \
    'predicted: passes silently. The row says no control was run.'

  run_arm d_cites_a_probe_that_does_not_exist_control "no_such_probe_row_exists_anywhere" \
    'MUST be reported, or arms a, b and c say nothing at all.'
} 2>&1 | tee "$out"
