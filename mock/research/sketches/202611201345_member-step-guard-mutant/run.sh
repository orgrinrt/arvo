#!/usr/bin/env bash
# The red runs of the format's three member-step flags against the member step's
# tests, and one run as shipped. Each output keeps every line that is not a pass,
# every panic with the three lines after it, and the exit status.
set -u
here=$(cd "$(dirname "$0")" && pwd)
mock="$here/../../.."
run() {
  local name=$1 flags=$2
  (cd "$mock" && RUSTFLAGS="$flags" CARGO_INCREMENTAL=0 \
    cargo test -q -p arvo-format --lib --no-fail-fast member::step 2>&1) \
    | grep -v '^test .* ok$' | grep -A3 -E 'panicked|FAILED|failed|error|test result' \
    > "$here/outputs/$name.txt"
  echo "exit ${PIPESTATUS[0]}" >> "$here/outputs/$name.txt"
}
run shipped ""
run arvo_mutant_member_step "--cfg arvo_mutant_member_step"
run arvo_mutant_member_step_edge "--cfg arvo_mutant_member_step_edge"
run arvo_mutant_member_step_guard "--cfg arvo_mutant_member_step_guard"
