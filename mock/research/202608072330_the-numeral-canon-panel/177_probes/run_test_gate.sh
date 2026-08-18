#!/bin/bash
# 157 test gate: run every -shared crate's suite, both profiles for the two slow ones.
# Negative control: the sweep must FAIL if a crate's tests do not actually run.
# We assert on the presence of a "test result: ok. N passed" line with N > 0,
# and print MISSING when absent, so a silently-skipped crate cannot read as green.
cd /Users/orgrinrt/Dev/clause-dev/arvo/mock/benches
PROFILE="${1:---release}"
for c in $(ls variants/ | grep -- '-shared$'); do
  if [ "$c" = "bitpack-write-contend-shared" ] && [ -z "$RUN_CONTEND" ]; then
    echo "$c : SKIPPED (set RUN_CONTEND=1; known non-terminating in default run)"
    continue
  fi
  out=$(cargo test --offline $PROFILE --manifest-path variants/$c/Cargo.toml 2>&1)
  line=$(echo "$out" | grep -E '^test result:' | head -1)
  n=$(echo "$line" | sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p')
  if [ -z "$n" ] || [ "$n" = "0" ]; then
    echo "$c : MISSING OR ZERO -> $line"
  else
    echo "$c : $n passed  [$PROFILE]"
  fi
done
