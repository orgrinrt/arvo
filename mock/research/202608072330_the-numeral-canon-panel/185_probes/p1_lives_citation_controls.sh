#!/usr/bin/env nutshell
# Can the committed checks fail on a probe row's `lives` field?
#
# Written and its required outcomes recorded BEFORE the run. A pass from an
# instrument that has never produced a fail is a claim about the instrument.
#
# Six arms, planted one at a time into a throwaway registry file that is removed
# afterwards. Every arm carries a schema-legal id, because run one of this
# instrument used capitals and every arm failed on the id regex instead of on
# the thing it was pointed at, which is a fail that proves nothing.
#
#   A  lives names a file that is not in the tree        MUST be reported
#   B  lives names a real committed probe artifact       MUST be silent  <- control
#   C  lives is a line number into a living ledger       MUST be reported (arvo-checks)
#   D  lives is a bare root with no path                 MUST be reported
#   E  lives ends at a filename with no line or anchor   outcome recorded, not predicted
#   F  lives names a real file at a line past its end    outcome recorded, not predicted
#
# Arm B is the one that matters. A, C, D would all fire on an instrument that
# reports everything; only B distinguishes a checker from a shouter.
#
# E and F are questions rather than controls: run one established that the
# engine refuses a citation whose last segment is neither a line number nor a
# heading anchor, and both arms A and B were refused for that reason before
# either reached resolution. So the terminal form has to be fixed before the
# existence arm says anything, and whether a past-the-end line is caught decides
# how much a line citation into a probe artifact is worth.
set -euo pipefail

root="$(cd "$(dirname "$0")/../../../.." && pwd)"
plant="$root/mock/registry/zzz_planted_probe_control.toml"
out="$(dirname "$0")/p1_lives_citation_controls.out"

cleanup() { rm -f "$plant"; }
trap cleanup EXIT

run_arm() {
  local name="$1" lives="$2"
  cat > "$plant" <<TOML
[[probe]]
id = "planted_${name}"
establishes = "nothing; this row exists to make a check fail."
lives = ["${lives}"]
control = "this row IS the control."
standing = "sound"
TOML
  printf '=== arm %s\n  lives = %s\n' "$name" "$lives"
  ( cd "$root" && cargo mock --lint-only 2>&1 ) \
    | grep -Ei 'ERROR|warning|unresolv|unknown|no such|cannot|planted_|rows across' \
    | grep -v 'unknown-config-key' \
    || printf '  (nothing matched)\n'
  printf '\n'
}

{
  printf 'p1: can a probe row lives citation fail?\n'
  printf 'date: %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  run_arm a_missing_file \
    'panel::202608072330_the-numeral-canon-panel::185_probes::this_file_does_not_exist::1'

  run_arm b_real_file_control \
    'panel::202608072330_the-numeral-canon-panel::147_probes::r1_the_unsigned_row_and_the_congruence_isolated::1'

  run_arm c_line_into_living_ledger \
    'panel::202608072330_the-numeral-canon-panel::AGREEMENTS::12'

  run_arm d_bare_root \
    'panel'

  run_arm e_filename_with_no_terminal_anchor \
    'panel::202608072330_the-numeral-canon-panel::147_probes::r1_the_unsigned_row_and_the_congruence_isolated'

  run_arm f_line_past_the_end \
    'panel::202608072330_the-numeral-canon-panel::147_probes::r1_the_unsigned_row_and_the_congruence_isolated::999999'
} 2>&1 | tee "$out"

# Arm C's second half, and run two of this instrument got it wrong.
#
# The ledger guard is in `arvo-checks`, not in the lint run, so arm C above is
# silent by construction and says nothing. Run two then ran the test suite AFTER
# the loop, by which point the planted file held arm F rather than arm C, so the
# suite passed on material that was never a ledger citation and the arm proved
# nothing while printing eight greens. The plant is re-laid here, deliberately,
# and the required outcome is a FAILING test.
cat > "$plant" <<'TOML'
[[probe]]
id = "planted_c_line_into_living_ledger"
establishes = "nothing; this row exists to make a check fail."
lives = ["panel::202608072330_the-numeral-canon-panel::AGREEMENTS::12"]
control = "this row IS the control."
standing = "sound"
TOML
{
  printf '\n=== arm c second half: ledger guard, with the arm-c plant actually in place\n'
  printf '    required outcome: the_committed_canon_cites_no_moving_line FAILS\n'
  ( cd "$root/mock" && cargo test -p arvo-checks --test no_line_citation_into_a_living_ledger 2>&1 \
      | grep -E '^test |test result|line-citation|AGREEMENTS' | head -20 )
} | tee -a "$out"
