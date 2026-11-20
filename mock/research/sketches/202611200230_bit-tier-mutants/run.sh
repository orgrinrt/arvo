#!/usr/bin/env bash
# One run of each `arvo_mutant_bits_*` flag of `arvo-bits` against the tests
# that hold its operation, from `mock/`. Each output is the run's text less the
# passing lines. Names given on the command line run only those outputs.
set -u
here=$(cd "$(dirname "$0")" && pwd)
cd "$here/../../.."
only=" $* "
wanted() {
  [ "$only" = "  " ] || [[ "$only" == *" $1 "* ]]
}
run() {
  local name=$1; shift
  wanted "$name" || return 0
  {
    echo "RUSTFLAGS=--cfg $name cargo test -p arvo-bits --no-fail-fast $*"
    RUSTFLAGS="--cfg $name" CARGO_INCREMENTAL=0 cargo test -p arvo-bits --no-fail-fast "$@" 2>&1 \
      | grep -E '^(test |test result|     Running|error|failures:)' | grep -v ' \.\.\. ok$'
    echo "exit ${PIPESTATUS[0]}"
  } > "$here/outputs/$name.txt"
}
run arvo_mutant_bits_with_bit --test the_bit_access
run arvo_mutant_bits_not --test the_bit_logic --test the_bit_tier_in_a_constant
run arvo_mutant_bits_shift --test the_bit_sequence
run arvo_mutant_bits_rotate --test the_bit_sequence --test the_bit_tier_in_a_constant

# trybuild drops `RUSTFLAGS` from its own build, so `compile_fail.rs` cannot see
# `arvo_mutant_bits_index`. Its refusal cases are built here as examples of
# `index/` instead, first as shipped and then under the flag; under the flag a
# case that builds is one `compile_fail.rs` would have failed on.
index_cases() {
  local flags=$1 e out status
  for e in a_bit_at_the_width_is_refused a_written_bit_at_the_width_is_refused \
           a_shift_by_the_width_is_refused a_rotation_by_the_width_is_refused \
           a_bit_at_the_width_through_the_trait_is_refused; do
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
