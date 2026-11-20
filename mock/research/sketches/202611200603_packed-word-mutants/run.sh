#!/usr/bin/env bash
# Runs each packed word mutant flag against the word's law tests, from `mock/`,
# keeping every line that is not a pass and the exit status.
set -u
out="research/sketches/202611200603_packed-word-mutants/outputs"
for flag in arvo_mutant_bits_word_get arvo_mutant_bits_word_with; do
  RUSTFLAGS="--cfg $flag" CARGO_INCREMENTAL=0 cargo test -p arvo-bits --test the_packed_word --no-fail-fast \
    > "$out/$flag.full" 2>&1
  status=$?
  grep -v -e '^test .* ok$' -e '^ *Compiling' "$out/$flag.full" > "$out/$flag.txt"
  echo "exit $status" >> "$out/$flag.txt"
  rm "$out/$flag.full"
done
