#!/usr/bin/env bash
# Whether a check at a foreign target evaluates a const assertion.
#
# One command, from anywhere: `./run.sh` in this directory. Each arm is checked
# at `i686-unknown-linux-gnu`, a 32-bit target, with `--emit=metadata`, which is
# the part of a build `cargo check` runs. The whole output of every run, its
# exit status included, goes to `run.out` beside this script, so the committed
# file is the run rather than a transcription of it.
#
# The target has to be installed for the pinned toolchain:
# `rustup target add i686-unknown-linux-gnu`.
set -uo pipefail
cd "$(dirname "$0")"

TARGET=i686-unknown-linux-gnu
OUT_DIR=$(mktemp -d)
trap 'rm -rf "$OUT_DIR"' EXIT

check() {
  local arm=$1
  shift
  echo "== ${arm} $*"
  rustc --edition 2024 --crate-type lib --emit=metadata --target "$TARGET" \
    --out-dir "$OUT_DIR" "$@" "${arm}.rs" 2>&1
  echo "exit $?"
  echo
}

{
  echo "tool:   $(rustc --version)"
  echo "target: ${TARGET}"
  echo
  check a_const_assertion_over_a_literal_width
  check a_const_assertion_over_the_pointer_width
  check a_test_arm_over_a_literal_width --test
} | tee run.out
