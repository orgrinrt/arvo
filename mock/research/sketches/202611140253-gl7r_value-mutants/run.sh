#!/usr/bin/env bash
# One run of each `arvo_mutant_*` flag of `arvo-numeral` against the tests that
# name its operation, from `mock/`. Each output is the run's text less the
# passing lines. Names given on the command line run only those outputs.
set -u
here=$(cd "$(dirname "$0")" && pwd)
cd "$here/../../.."
only=" $* "
run() {
  local name=$1 flag=$2; shift 2
  [ "$only" = "  " ] || [[ "$only" == *" $name "* ]] || return 0
  {
    echo "RUSTFLAGS=--cfg $flag cargo test -p arvo-numeral --no-fail-fast $*"
    RUSTFLAGS="--cfg $flag" CARGO_INCREMENTAL=0 cargo test -p arvo-numeral --no-fail-fast "$@" 2>&1 \
      | grep -E '^(test |test result|     Running|error|failures:)' | grep -v ' \.\.\. ok$'
    echo "exit ${PIPESTATUS[0]}"
  } > "$here/outputs/$name.txt"
}
for flag in arvo_mutant_8 arvo_mutant_16 arvo_mutant_32 arvo_mutant_64 \
            arvo_mutant_sub arvo_mutant_neg arvo_mutant_mul; do
  run "$flag" "$flag" --lib
done
run arvo_mutant_under arvo_mutant_under --test the_crossing
run arvo_mutant_div arvo_mutant_div --test the_quotient
run arvo_mutant_convert arvo_mutant_convert --test the_conversion
run arvo_mutant_index arvo_mutant_index --test the_host_door
run arvo_mutant_count arvo_mutant_count --features hlist-const --test the_count
run arvo_mutant_count_without_const arvo_mutant_count --features hlist --test the_count_without_const
run arvo_mutant_convert_policy arvo_mutant_convert_policy \
  --test the_conversion_under_every_source_signature_past_the_range \
  --test the_conversion_under_every_source_signature \
  --test the_conversion_at_the_positive_edge_binary
