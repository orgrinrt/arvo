#!/usr/bin/env bash
# Builds and runs each example of `variance/`, recording the first error line or
# the run's last line, and the exit status, in `outputs/variance.txt`.
set -u
here=$(cd "$(dirname "$0")" && pwd)
{
  for f in "$here/variance/examples/"*.rs; do
    e=$(basename "$f" .rs)
    out=$(cd "$here/variance" && CARGO_INCREMENTAL=0 cargo run -q --example "$e" 2>&1)
    status=$?
    line=$(printf '%s\n' "$out" | grep -E '^error(\[E[0-9]+\])?:|panicked' | grep -v 'aborting due to' | head -1)
    echo "== $e exit $status :: ${line:-$(printf '%s' "$out" | tail -1)}"
  done
} > "$here/outputs/variance.txt"
