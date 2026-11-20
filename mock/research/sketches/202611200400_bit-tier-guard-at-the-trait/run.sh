#!/usr/bin/env bash
# Builds every example of `reviewer/` and `guard/`, and runs the ones that
# build, recording the first error line or the run's output and the exit
# status per example in `outputs/<crate>.txt`.
set -u
here=$(cd "$(dirname "$0")" && pwd)
for crate in reviewer guard; do
  {
    for f in "$here/$crate/examples/"*.rs; do
      e=$(basename "$f" .rs)
      out=$(cd "$here/$crate" && CARGO_INCREMENTAL=0 cargo run -q --example "$e" 2>&1)
      status=$?
      line=$(printf '%s\n' "$out" | grep -E '^error(\[E[0-9]+\])?:' | grep -v 'aborting due to' | head -1)
      echo "== $e exit $status :: ${line:-$(printf '%s' "$out" | tail -1)}"
    done
  } > "$here/outputs/$crate.txt"
done
