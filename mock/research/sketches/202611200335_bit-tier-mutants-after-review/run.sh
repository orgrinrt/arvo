#!/usr/bin/env bash
# One run of each `arvo_mutant_bits_*` flag of `arvo-bits` against the tests
# that hold its operation, from `mock/`, after round `202611200328`. Each output
# is the run's text less its passing lines, keeping each panic with the
# assertion it failed. Names given on the command line run only those outputs.
# Supersedes `../202611200230_bit-tier-mutants/run.sh`.
set -u
here=$(cd "$(dirname "$0")" && pwd)
cd "$here/../../.."
only=" $* "
wanted() {
  [ "$only" = "  " ] || [[ "$only" == *" $1 "* ]]
}
# The lines that say what ran and what failed: each test line that is not a
# pass, the summaries, and every panic with the three lines naming its assertion.
failed_lines() {
  awk '/ \.\.\. ok$/ { next }
       /panicked at/ { print; keep = 3; next }
       keep > 0 { print; keep--; next }
       /^(test |test result|     Running|error|failures:)/ { print }'
}
run() {
  local name=$1 out status; shift
  wanted "$name" || return 0
  out=$(RUSTFLAGS="--cfg $name" CARGO_INCREMENTAL=0 cargo test -p arvo-bits --no-fail-fast "$@" 2>&1)
  status=$?
  {
    echo "RUSTFLAGS=--cfg $name cargo test -p arvo-bits --no-fail-fast $*"
    printf '%s\n' "$out" | failed_lines
    echo "exit $status"
  } > "$here/outputs/$name.txt"
}
run arvo_mutant_bits_with_bit --test the_bit_access
run arvo_mutant_bits_not --test the_bit_logic --test the_bit_tier_in_a_constant
run arvo_mutant_bits_shift --test the_bit_sequence
run arvo_mutant_bits_rotate --test the_bit_sequence --test the_bit_tier_in_a_constant

# trybuild drops `RUSTFLAGS` from its own build, so `compile_fail.rs` cannot see
# `arvo_mutant_bits_index`. Its index cases are the examples of `index/`, built
# first as shipped and then under the flag; under the flag a case that builds is
# one `compile_fail.rs` would have failed on.
index_cases() {
  local flags=$1 e out status
  for e in $(grep -o '^name = "[a-z_0-9]*"' "$here/index/Cargo.toml" | cut -d'"' -f2); do
    echo "== $e, RUSTFLAGS='$flags'"
    out=$(cd "$here/index" && RUSTFLAGS="$flags" CARGO_INCREMENTAL=0 cargo build --example "$e" 2>&1)
    status=$?
    printf '%s\n' "$out" | grep -E '^error' | sort -u
    echo "exit $status"
  done
}
if wanted arvo_mutant_bits_index; then
  { index_cases ""; index_cases "--cfg arvo_mutant_bits_index"; } > "$here/outputs/arvo_mutant_bits_index.txt"
fi
