#!/usr/bin/env nutshell
# Does the repaired gate actually catch a claim resting on an uncontrolled probe?
#
# `measurements_resting_on_an_unusable_instrument` reports a RAN_SOMETHING claim
# citing a probe whose own `control` admits none was run. It decides that by
# substring, against five phrases in `shape.rs`. Five rows in `probe.toml` open
# their control field with the word None; `p1` finds only ONE of them matched.
#
# That is an inference from the phrase list. This is the experiment: plant a
# `measured` proposal against each of the five in turn and see which the gate
# reports. Inferring from a matcher's source is how a matcher's behaviour gets
# described rather than measured.
#
# PREDICTION, recorded before the run: one reported, four silent. If four are
# reported the phrase list reaches further than reading it suggests and the
# finding evaporates, which is the better outcome.
#
#   CONTROL. A sixth arm citing a probe at `standing = "defective"`, which is a
#   different branch of the same check and MUST be reported. Without it, four
#   silences are indistinguishable from a check that is not running at all.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../.." && pwd)"
plant="$root/mock/registry/zzz_planted_control_reading.toml"
out="$here/p2_which_admissions_the_gate_reads.out"

cleanup() { rm -f "$plant"; }
trap cleanup EXIT

arm() {
  local slug="$1" why="$2"
  cat > "$plant" <<TOML
[[proposal]]
id = "planted_for_${slug}"
says = "a planted claim that exists to see whether the gate reads its instrument's control field."
because = "it does not; this row is the instrument."
topic = "panel_conduct"
kind = "answer"
sentence_kind = "measured"
standing = "one_expert"
predicate = ["threads: 1"]
evidence = ["${slug}"]
provenance = ["panel::202608072330_the-numeral-canon-panel::192_probes::p2_which_admissions_the_gate_reads.sh::1"]
keywords = ["planted", "control", "gate"]
TOML
  local r
  r=$( cd "$root/mock" && cargo test -p arvo-checks --test what_one_field_obliges_another_to_carry 2>&1 \
        | grep -oE 'measurement-rests-on-an-(uncontrolled|unusable)-instrument' | head -1 || true )
  printf '  %-58s %s   %s\n' "$slug" "${r:-SILENT}" "$why"
}

{
  printf '=== p2 which admissions the gate reads, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf 'prediction, before the run: one reported, four silent.\n\n'
  printf '%-60s %s\n' 'probe cited by a planted measured claim' 'what the gate did'
  arm chain_the_third_definition_is_not_observation_bounded      'control opens "None stated in the material read"'
  arm no_dependent_survives_the_rounding_units_defects           'control opens "None stated as a must-fail arm"'
  arm the_bench_tree_was_built_at_the_undocumented_profile       'control opens "None run, and the shape..."'
  arm the_debug_release_gap_that_retired_a_true_finding          'control opens "None, and none is needed"'
  arm an_equivalence_checker_that_skips_panics_certifies_a_definedness_difference \
                                                                 'control opens "None. It is a structural argument"'
  printf '\n'
  printf 'CONTROL, a different branch of the same check, MUST be reported:\n'
  arm the_width_invariance_control_was_toothless                 'standing = defective'
} 2>&1 | tee "$out"
